import { Identifier, read_identifier_token, read_maybe_missing } from ".";
import { start } from "../../utils/ast";
import { CharCode } from "../../utils/CharCode";
import { getDelimEndCharCode, getDelimStartCharCode, getGroupDelimKind, GroupDelimKind, is_XID_Start } from "../../utils/enum";
import { exit } from "../error";
import * as Nodes from "../nodes";
import {
	AttrSegment,
	DelimKind,
	DelimNameMap,
	LocArray,
	MacroInvokeSegment,
	MacroMatchSegment,
	MacroSeparator,
	MacroTransformSegment,
	Segment,
} from "../nodes";
import {
	currChar,
	edgecase_stepback,
	ENDS_WITH,
	Env,
	FOR_ANY_BETWEEN,
	getEnv,
	GET_POSITION,
	invalidate_kw,
	isReadingEnv,
	match,
	maybe_read,
	maybe_read_2,
	Mc_ctx_isReadingTokens,
	MC_ctx_isReadingTokens_end,
	MC_ctx_isReadingTokens_start,
	peek_keyword,
	read,
	readLocatedArrayDelim,
	readLocatedArrayNoDelim,
	read_XID_CONTINUE,
	safe_skip,
	safe_skip_keyword,
	safe_step_over,
	setEnv,
	step_over,
} from "../state";
import { FileLoc, FileLocAttr, FileLocDollar, FileLoc_FromChild } from "../state/constructor";
import { Keyword, ptTree } from "../state/scanner";
import { __EXPORT_ATTR_CTORS } from "../state/whitespace";
import { Literal } from "./literals";
import { read_charLiteral_or_lifetime } from "./types";

//#region ==============================================[        Attribute        ]=========================================================``--.

export class Attribute extends FileLocAttr(Nodes.Attribute) {
	read() {
		// #••••••••••••••••
		// ^- You are here
		safe_skip(CharCode["#"]);
		this.inner = maybe_read(CharCode["!"]);
		this.segments = read_segments_withEnv(Env.Attribute, DelimKind["[]"]);
	}
}

__EXPORT_ATTR_CTORS(Attribute);

//#endregion ===============================================================================================================================..--'

//#region ================================================[        Macro        ]===========================================================``--.

class McIdentifier extends FileLocDollar(Nodes.McIdentifier) {
	read() {
		// $x••••••••••••••••
		//  ^- You are here (x:XID_START)
		read_XID_CONTINUE();
	}
}

export class MacroInvocation extends FileLoc_FromChild(Nodes.MacroInvocation) {
	read(callee: this["callee"]) {
		// callee!(••••••••••••••••
		//       ^- You are here
		this.callee = callee;
		safe_skip(CharCode["!"]);
		this.segments = read_segments_withEnv<MacroInvokeSegment>(Env.MacroInvocation);
		// this.semi = false; // see read_expr_or_macroInvocation_stmt under class ExpressionStatement
	}
}

export class MacroRulesDeclaration extends FileLoc(Nodes.MacroRulesDeclaration) {
	read() {
		// macro_rules!••••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword["macro_rules!"]), safe_skip(CharCode["!"]);
		this.id = read_identifier_token();
		switch (currChar()) {
			case CharCode["{"]:
				this.rules = read_rules(DelimKind["{}"]);
				break;
			case CharCode["("]:
				this.rules = read_rules(DelimKind["()"]);
				maybe_read(CharCode[";"]);
				break;
			case CharCode["["]:
				this.rules = read_rules(DelimKind["[]"]);
				maybe_read(CharCode[";"]);
				break;
			default:
				exit.expected([CharCode["{"], CharCode["("], CharCode["["]]);
		}
	}
}

/**/ class MacroRuleDeclaration extends FileLoc(Nodes.MacroRuleDeclaration) {
	read() {
		read_rule(this);
	}
}

export class MacroDeclaration extends FileLoc(Nodes.MacroDeclaration) {
	read() {
		// macro••••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.macro);
		this.id = Identifier.read();
		switch (currChar()) {
			case CharCode["("]:
				this.rules = MacroInlineRuleDeclaration.read();
				break;
			case CharCode["{"]:
				this.rules = read_rules(DelimKind["{}"]);
				break;
			default:
				exit.expected([CharCode["("], CharCode["{"]]);
		}
	}
}

/**/ class MacroInlineRuleDeclaration extends FileLoc(Nodes.MacroInlineRuleDeclaration) {
	read() {
		read_rule(this);
	}
}

/**/ class DelimGroup extends FileLoc(Nodes.DelimGroup) {
	read() {
		this.segments = read_segments();
	}
}

function read_rules<K extends typeof DelimKind["()"] | typeof DelimKind["[]"] | typeof DelimKind["{}"]>(delim: K) {
	return readLocatedArrayDelim<Nodes.MacroRuleDeclaration, K>(delim, (rules) => {
		FOR_ANY_BETWEEN(
			getDelimStartCharCode(rules.dk),
			() => {
				rules.push(MacroRuleDeclaration.read());
				while (maybe_read(CharCode[";"]) || maybe_read(CharCode[","]));
			},
			getDelimEndCharCode(rules.dk)
		);
	});
}

function read_rule(rule: MacroRuleDeclaration | MacroInlineRuleDeclaration) {
	rule.match = read_segments_withEnv<MacroMatchSegment>(Env.MacroMatch);
	maybe_read_2(CharCode["="], CharCode[">"]);
	rule.transform = read_segments_withEnv<MacroTransformSegment>(Env.MacroTransform);
}

//#endregion ===============================================================================================================================..--'

class PunctuationToken extends FileLoc(Nodes.PunctuationToken) {
	read() {
		this.tk = ptTree();
		__DEV__: if (!ENDS_WITH(this.token)) exit.at(this, `Attempted to read '${this.token}'`);
		__DEV__: if (GET_POSITION() - this.token.length !== start(this))
			exit.at(this, `Attempted to start token '${this.token}' from here`);
	}
}

// prettier-ignore
/**/ class MacroGroup extends FileLocDollar(Nodes.MacroGroup) {
	read() {
		// $(••••••••••••••••
		//  ^- You are here
		this.segments = read_segments(DelimKind["()"]);
		this.sep = maybe_read_sep();
		switch (currChar()) {
			case CharCode["*"]: this.kind = "*"; safe_skip(CharCode["*"]); break;
			case CharCode["+"]: this.kind = "+"; safe_skip(CharCode["+"]); break;
			case CharCode["?"]: this.kind = "?"; safe_skip(CharCode["?"]); break;
			default: exit.expected([CharCode["*"], CharCode["+"], CharCode["?"]]);
		}
	}
}

/**/ class MacroParameterDeclaration extends FileLocDollar(Nodes.MacroParameterDeclaration) {
	read() {
		// $•••••••••••••••••
		//  ^- You are here
		this.id = McIdentifier.read();
		this.ty = read_maybe_missing(() => (maybe_read(CharCode[":"]) ? read_identifier_token() : undefined));
	}
}
// prettier-ignore
function read_segments<T extends Segment, K extends GroupDelimKind>(tk = getGroupDelimKind(currChar()) as K): LocArray<T, DelimNameMap[K]> {
	return readLocatedArrayDelim<Segment, K>(tk, (tokens) => {
		MC_ctx_isReadingTokens_start();
		safe_skip(getDelimStartCharCode(tokens.dk));

		loop: while (true) {
			switch (currChar()) {
				case CharCode[")"]: case CharCode["]"]: case CharCode["}"]: break loop;
				case CharCode["("]: case CharCode["["]: case CharCode["{"]: tokens.push(DelimGroup.read() as T); break;
				case CharCode["'"]: tokens.push(read_charLiteral_or_lifetime() as Nodes.Literal | Nodes.LtIdentifier); break;
				case CharCode['"']:
				case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]:
				case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]: 
					tokens.push(Literal.read()); break;
				case CharCode["$"]: tokens.push(read_dollar() as T); break;
				default:
					switch (peek_keyword()) {
						case Keyword.true: case Keyword.false: case Keyword.StringLiteral: tokens.push(Literal.read()); break;
						case Keyword.Underscore: tokens.push(read_underscore()); break;
						case Keyword.NotAWord: tokens.push(PunctuationToken.read()); break;
						default: tokens.push(read_identifier_token()); break;
					}
					break;
			}
		}

		MC_ctx_isReadingTokens_end();

		// Attribute delim must end without skipping whitespace
		if (isReadingEnv(Env.Attribute) && !Mc_ctx_isReadingTokens()) {
			step_over(getDelimEndCharCode(tokens.dk));
		} else {
			read(getDelimEndCharCode(tokens.dk));
		}
		
	}) as LocArray<T, any>;
}

function read_segments_withEnv<T extends Segment, K extends GroupDelimKind = GroupDelimKind>(
	env: Env,
	tk = getGroupDelimKind(currChar()) as K
): LocArray<T, any> {
	setEnv(env);
	return read_segments<T, any>(tk);
}

// prettier-ignore
function maybe_read_sep(): MacroSeparator | undefined {
	switch (currChar()) {
		case CharCode["$"]:
		case CharCode[")"]: case CharCode["]"]: case CharCode["}"]:
		case CharCode["("]: case CharCode["["]: case CharCode["{"]: exit.unexpected();
		case CharCode["*"]: case CharCode["+"]: case CharCode["?"]: return undefined;
		case CharCode["'"]: return read_charLiteral_or_lifetime() as Nodes.Literal | Nodes.LtIdentifier;
		case CharCode['"']:
		case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]: 
		case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]:
			return Literal.read();
		default:
			switch (peek_keyword()) {
				case Keyword.true: case Keyword.false: case Keyword.StringLiteral: return Literal.read();
				case Keyword.Underscore: return read_underscore();
				case Keyword.NotAWord: return PunctuationToken.read();
				default: return read_identifier_token();
			}
	}
}

function read_dollar(): Nodes.MacroGroup | Nodes.MacroParameterDeclaration | Nodes.McIdentifier | Nodes.PunctuationToken {
	switch (getEnv()) {
		case Env.Attribute: // #[ ... $
		case Env.MacroInvocation: // foo!( ... $
			return PunctuationToken.read();
	}

	safe_step_over(CharCode["$"]);

	if (match(CharCode["("])) {
		return MacroGroup.read(); // ... $(
	}

	if (is_XID_Start(currChar())) {
		return isReadingEnv(Env.MacroMatch)
			? MacroParameterDeclaration.read() // ... $id: ty
			: McIdentifier.read(); // ... $id
	}

	edgecase_stepback();
	return PunctuationToken.read(); // ... $
}

function read_underscore() {
	invalidate_kw();
	return PunctuationToken.read();
}

// prettier-ignore
export function read_tokens_until(endPos: number): LocArray<AttrSegment, "None"> {
	setEnv(Env.Attribute);
	return readLocatedArrayNoDelim((tokens) => {
		MC_ctx_isReadingTokens_start();

		loop: while (GET_POSITION() < endPos) {
			switch (currChar()) {
				case CharCode[")"]: case CharCode["]"]: case CharCode["}"]: exit.unexpected();
				case CharCode["("]: case CharCode["["]: case CharCode["{"]: tokens.push(DelimGroup.read() as AttrSegment); break;
				case CharCode["'"]: tokens.push(read_charLiteral_or_lifetime() as Nodes.Literal | Nodes.LtIdentifier); break;
				case CharCode['"']:
				case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]:
				case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]: 
					tokens.push(Literal.read()); break;
				case CharCode["$"]: tokens.push(read_dollar() as AttrSegment); break;
				default:
					switch (peek_keyword()) {
						case Keyword.true: case Keyword.false: case Keyword.StringLiteral: tokens.push(Literal.read()); break;
						case Keyword.Underscore: tokens.push(read_underscore()); break;
						case Keyword.NotAWord: tokens.push(PunctuationToken.read()); break;
						default: tokens.push(read_identifier_token()); break;
					}
					break;
			}
		}

		MC_ctx_isReadingTokens_end();
	});
}
