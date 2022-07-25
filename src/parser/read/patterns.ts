import { Identifier, read_Index } from ".";
import { is_RestPattern } from "../../utils/ast";
import { CharCode } from "../../utils/CharCode";
import * as Nodes from "../nodes";
import { DelimKind, Feature } from "../nodes";
import {
	CCPATH_read,
	currChar,
	EDGECASE_STEPBACK_TO,
	FG_property_true,
	match,
	maybe_read,
	maybe_read_2,
	maybe_read_keyword,
	peek_keyword,
	peek_match,
	peek_not_match,
	read_ahead,
	read_between,
	read_sequence,
	safe_skip,
	safe_skip_1_read_2,
	safe_skip_keyword,
	sequence_hasTrailingComma,
	with_outerAttributes_fromParentContext,
	with_outerAttributes_fromParentContext_if_test,
	__inherit_endPos,
	__inherit_startPos,
} from "../state";
import {
	FileLoc,
	FileLoc_COPY,
	FileLoc_DEFINES_OWN_RANGE,
	FileLoc_FromChild,
	FileLoc_FromChildElsePos,
	FileLoc_FromChildElseReadAhead,
	withEscapedParens,
} from "../state/constructor";
import { Keyword } from "../state/scanner";
import { BlockExpression, ExpressionPath, ExpressionTypeCast, ExpressionTypeSelector } from "./expressions";
import { Literal } from "./literals";
import { MacroInvocation } from "./macro";
import { read_with_const_modifier } from "./specifiers";
import { read_TypeArguments } from "./types";

//#region ===============================================[        Patterns        ]=========================================================``--.

//#------------------------------------------------------+        Grouping        +---------------------------------------------------------.

class UnionPattern extends FileLoc_FromChildElseReadAhead(Nodes.UnionPattern) {
	read(pattern: this["patterns"][number] | undefined) {
		// item |••••••••••••••••
		//      ^- You are here
		this.patterns = undefined === pattern ? [] : [pattern];
		while (maybe_read(CharCode["|"])) this.patterns.push(read_pattern(false));
	}
}

class ParenthesizedPattern extends FileLoc_COPY(Nodes.ParenthesizedPattern) {
	read(items: Nodes.LocArray<this["pattern"], any>) {
		// (pattern)•••••••••••••••••
		//          ^- You are here
		this.pattern = items[0];
	}
}

//#------------------------------------------------------+        Wildcard        +---------------------------------------------------------.

class RestPattern extends FileLoc(Nodes.RestPattern) {
	read() {
		// ..•••••••••••••••••
		//   ^- You are here
	}
}

class WildcardPattern extends FileLoc(Nodes.WildcardPattern) {
	read() {
		// _••••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.Underscore);
	}
}

//#-----------------------------------------------------+        Variables        +---------------------------------------------------------.

class PatternVariableDeclaration extends FileLoc_FromChildElsePos(Nodes.PatternVariableDeclaration) {
	read(id: this["id"] | undefined) {
		this.ref = false;
		this.mut = false;
		if (id === undefined) {
			loop: while (true) {
				// prettier-ignore
				switch (peek_keyword()) {
					case Keyword.ref: safe_skip_keyword(Keyword.ref); this.ref = true; break;
					case Keyword.mut: safe_skip_keyword(Keyword.mut); this.mut = true; break;
					default: break loop;
				}
			}
			this.id = Identifier.read();
			this.pattern = maybe_read(CharCode["@"]) ? read_pattern(false) : undefined;
		} else {
			this.id = id;
			this.pattern = (safe_skip(CharCode["@"]), read_pattern(false));
		}
	}
}

//#-------------------------------------------------------+        Struct        +----------------------------------------------------------.

class StructPattern extends FileLoc_FromChild(Nodes.StructPattern) {
	read(struct: this["struct"]) {
		// struct {••••••••••••••••
		//        ^- You are here
		this.struct = struct;
		this.properties = read_sequence(DelimKind["{}"], () =>
			with_outerAttributes_fromParentContext((): Nodes.StructPatternProperty => {
				switch (peek_keyword()) {
					case Keyword.box:
					case Keyword.ref:
					case Keyword.mut:
						return StructPatternPropertyShorthand.read(undefined);
					case Keyword.RawIdentifier:
					case Keyword.NotKeyword:
					case Keyword.union: {
						const id = Identifier.read();
						return match(CharCode[":"]) //
							? StructPatternPropertyDestructured.read(id)
							: StructPatternPropertyShorthand.read(id);
					}
				}
				if (match(CharCode["."])) {
					return read_ahead(() => {
						safe_skip_1_read_2(CharCode["."], CharCode["."]);
						return RestPattern.read();
					});
				} else {
					return StructPatternPropertyDestructured.read(read_Index());
				}
			})
		);
	}
}

class StructPatternPropertyDestructured extends FileLoc_FromChild(Nodes.StructPatternPropertyDestructured) {
	read(key: this["key"]) {
		this.key = key;
		safe_skip(CharCode[":"]);
		this.pattern = read_PatternNoUnion_unstrict();
	}
}

class StructPatternPropertyShorthand extends FileLoc_FromChildElsePos(Nodes.StructPatternPropertyShorthand) {
	read(id: this["id"] | undefined) {
		this.box = false;
		this.ref = false;
		this.mut = false;
		if (id === undefined) {
			loop: while (true) {
				// prettier-ignore
				switch (peek_keyword()) {
					case Keyword.box: safe_skip_keyword(Keyword.box); this.box = FG_property_true(Feature.box_patterns, this, "box"); break;
					case Keyword.ref: safe_skip_keyword(Keyword.ref); this.ref = true; break;
					case Keyword.mut: safe_skip_keyword(Keyword.mut); this.mut = true; break;
					default: break loop;
				}
			}
			this.id = Identifier.read();
		} else {
			this.id = id;
		}
	}
}

class TuplePattern extends FileLoc_DEFINES_OWN_RANGE(Nodes.TuplePattern) {
	read(struct: this["struct"], items: this["items"]) {
		this.struct = struct;
		this.items = items;
		__inherit_startPos(this, struct ?? items);
		__inherit_endPos(this, items);
	}
}

class ArrayPattern extends FileLoc(Nodes.ArrayPattern) {
	read() {
		// [••••••••••••••••
		// ^- You are here
		this.items = read_sequence(DelimKind["[]"], () => read_pattern(true));
	}
}

//#-------------------------------------------------------+        Memory        +----------------------------------------------------------.

class ReferencePattern extends FileLoc(Nodes.ReferencePattern) {
	read() {
		// &••••••••••••••••
		// ^- You are here
		safe_skip(CharCode["&"]);
		this.mut = maybe_read_keyword(Keyword.mut);
		this.pattern = read_pattern(false) as Nodes.PatternNoUnionNoRange;
	}
}

class BoxPattern extends FileLoc(Nodes.BoxPattern) {
	read() {
		// box••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.box);
		this.pattern = read_pattern(false) as Nodes.PatternNoUnionNoRange;
	}
}

//#---------------------------------------------------+        Miscellaneous        +-------------------------------------------------------.

class MinusPattern extends FileLoc(Nodes.MinusPattern) {
	read(read_pattern: () => this["pattern"]) {
		// -••••••••••••••••
		// ^- You are here
		safe_skip(CharCode["-"]);
		this.pattern = read_pattern();
	}
}

class RangePattern extends FileLoc_FromChildElseReadAhead(Nodes.RangePattern) {
	read(lower: this["lower"]) {
		this.lower = lower;
		switch (currChar()) {
			case CharCode["="]:
				if (peek_not_match(-1, CharCode["."]) || peek_match(1, CharCode[">"])) {
					// let lower.. =  _ ••••••••••••
					//     lower.. => _ ••••••••••••
					//             ^- You are here
					this.last = false;
					this.upper = undefined;
					break;
				}
				// lower..=upper•••••••••••
				//        ^- You are here
				safe_skip(CharCode["="]);
				this.last = true;
				this.upper = maybe_read_RangePatternBoundEnd();
				break;
			case CharCode["."]:
				// lower...upper•••••••••••
				//        ^- You are here
				safe_skip(CharCode["."]);
				this.last = true;
				this.upper = maybe_read_RangePatternBoundEnd();
				break;
			default:
				// lower..•••••••••••••••••
				//        ^- You are here
				this.last = false;
				this.upper = maybe_read_RangePatternBoundEnd();
				break;
		}
		// __DEV__: assert(!!this.lower || !!this.upper);
	}
}

// prettier-ignore
function maybe_read_RangePatternBoundEnd(): Nodes.RangePatternBound | undefined {
	switch (currChar()) {
		case CharCode["("]: // jinx-rust only
			return read_between(CharCode["("], () => maybe_read_RangePatternBoundEnd(), CharCode[")"])
		case CharCode[":"]: // ::foo
			return read_PatternNamespaceTarget(CCPATH_read(ExpressionPath));
		case CharCode["<"]: // <A as B>::C
			return read_PatternNamespaceTarget(ExpressionTypeSelector.read());
		case CharCode["-"]: // -5
			return MinusPattern.read(() => Literal.read());
		case CharCode['"']: case CharCode["'"]: 
		case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]: 
		case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]:
			return Literal.read();
	}
	switch (peek_keyword()) {
		case Keyword.StringLiteral: case Keyword.false: case Keyword.true: return Literal.read();
		case Keyword.const: return read_with_const_modifier(() => BlockExpression.read());
		// #valid_identifier_keywords
		case Keyword.auto: case Keyword.self: case Keyword.Self: case Keyword.union: case Keyword.super: 
		case Keyword.crate: case Keyword.RawIdentifier: case Keyword.NotKeyword: /* case Keyword["macro_rules!"]: */ return read_PatternNamespaceTarget(Identifier.read());
		default: return undefined;
	}
}

//#endregion ===============================================================================================================================..--'

function read_PatternNamespaceTarget(namespace: Nodes.ExpressionNamespaceTarget | Nodes.ExpressionTypeSelector) {
	let lhs = namespace as any;
	loop: while (maybe_read_2(CharCode[":"], CharCode[":"])) {
		switch (currChar()) {
			case CharCode[":"]:
				EDGECASE_STEPBACK_TO(lhs);
				break loop;
			case CharCode["<"]:
				lhs = ExpressionTypeCast.read(lhs, read_TypeArguments());
				break;
			default:
				lhs = ExpressionPath.read(lhs);
				break;
		}
	}
	return lhs as Nodes.ExpressionNamespaceTarget;
}

// prettier-ignore
function read_pattern_lhs() {
	switch (currChar()) {
		case CharCode[":"]: // ::foo
			return read_PatternNamespaceTarget(CCPATH_read(ExpressionPath));
		case CharCode["<"]: // <A as B>::C
			return read_PatternNamespaceTarget(ExpressionTypeSelector.read());

		case CharCode["&"]: // &mut foo
			return ReferencePattern.read();

		case CharCode["("]: { // (...items) or (pattern)
			const items = read_sequence(DelimKind["()"], (SEQUENCE) => 
				with_outerAttributes_fromParentContext_if_test(
					() => read_pattern(true),
					() => SEQUENCE.length > 0 || match(CharCode[","])
				)
			);
			return items.length !== 1 || sequence_hasTrailingComma(items) || is_RestPattern(items[0])
				? TuplePattern.read(undefined, items)
				: ParenthesizedPattern.read(items);
		}
		case CharCode["["]: // [foo, bar, ..]
			return ArrayPattern.read();

		case CharCode["."]:
			return read_ahead(() => {
				safe_skip_1_read_2(CharCode["."], CharCode["."]);
				switch (currChar()) {
					case CharCode[","]:
					case CharCode[")"]:
					case CharCode["]"]:
						return RestPattern.read();
					// case CharCode["="]:
					// 	if (peek_match(1, CharCode[">"]) || peek_not_match(-1, CharCode["."])) {
					// 		return RestPattern.read(); // ..=> _ | .. => _ | let .. = _
					// 	}
				}
				return RangePattern.read(undefined);
			})

		case CharCode["|"]:
			return read_ahead(() => UnionPattern.read(undefined));

		case CharCode["-"]: // -5
			return MinusPattern.read(() => Literal.read());

		case CharCode['"']: case CharCode["'"]:
		case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]:
		case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]:
			return Literal.read();

		default: 
			switch (peek_keyword()) {
				// #lit_keywords
				case Keyword.StringLiteral: case Keyword.false: case Keyword.true:
					return Literal.read();
				case Keyword.const:
					return read_with_const_modifier(() => BlockExpression.read());
				case Keyword.ref:
				case Keyword.mut: 
					return PatternVariableDeclaration.read(undefined);
				case Keyword.box:
					return BoxPattern.read();
				case Keyword.Underscore:
					return WildcardPattern.read();
				default: {
					const id = Identifier.read();
					return match(CharCode["@"])
						? PatternVariableDeclaration.read(id)
						: read_PatternNamespaceTarget(id);
				}
			}
	}
}

export function read_PatternNoUnion_unstrict() {
	return read_pattern(true) as Nodes.PatternNoUnion;
}

export function read_pattern<T extends boolean>(allowUnionPattern: T): true extends T ? Nodes.PatternNode : Nodes.PatternNoUnion {
	return withEscapedParens(read_pattern_lhs() as any, (startNode) => {
		let lhs = startNode;
		loop: while (true) {
			switch (currChar()) {
				case CharCode["!"]:
					lhs = MacroInvocation.read(lhs);
					break;
				case CharCode["{"]:
					lhs = StructPattern.read(lhs);
					break;
				case CharCode["("]:
					lhs = TuplePattern.read(
						lhs,
						read_sequence(DelimKind["()"], () => with_outerAttributes_fromParentContext(() => read_pattern(true)))
					);
					break;
				case CharCode["."]:
					safe_skip_1_read_2(CharCode["."], CharCode["."]);
					lhs = RangePattern.read(lhs);
					if (allowUnionPattern && match(CharCode["|"])) {
						lhs = UnionPattern.read(lhs);
					}
					break loop;
				case CharCode["|"]:
					if (allowUnionPattern) {
						lhs = UnionPattern.read(lhs);
					}
					break loop;
				default:
					break loop;
			}
		}
		return lhs;
	});
}
