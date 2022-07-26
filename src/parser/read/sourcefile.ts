import { end, ownStart } from "../../utils/ast";
import { CharCode } from "../../utils/CharCode";
import { clamp } from "../../utils/common";
import { getDelimKind, getGroupDelimKind } from "../../utils/enum";
import { CharCode_UTF8BOM } from "../../utils/unicode";
import * as Nodes from "../nodes";
import { AttrSegment, DelimKind, DelimNameMap, ExpressionNode, LocArray, Located, Node, StatementNode } from "../nodes";
import { checkOptions, ParserOptions } from "../options";
import {
	currChar,
	GET_LENGTH,
	match_2,
	maybe_read,
	peek_not_match,
	read_group,
	read_group_noDelim,
	read_sequence,
	safe_step_over_2,
	step_until_ln,
	withParserState,
} from "../state";
import { FileLoc, FileLocProgram, FileLocSnippet, FileLocSourceFile } from "../state/constructor";
import { read_expression } from "./expressions";
import { read_tokens_until } from "./macro";
import { read_statement, read_top_statements } from "./statements";

/**/ class Shebang extends FileLoc(Nodes.Shebang) {
	read() {
		// #!•••••••••••••••
		// ^- you are here
		safe_step_over_2("#!");
		step_until_ln();
	}
}

class SourceFile extends FileLocSourceFile(Nodes.SourceFile) {
	read(_code: string, _options: ParserOptions) {
		if (maybe_read(CharCode_UTF8BOM)) {
			this.UTF8BOM = true;
		}

		if (match_2(CharCode["#"], CharCode["!"]) && peek_not_match(2, CharCode["["])) {
			this.shebang = Shebang.read();
		}

		this.program = Program.read();
	}
}

class Program extends FileLocProgram(Nodes.Program) {
	read() {
		read_top_statements(this);
	}
}

class Snippet extends FileLocSnippet(Nodes.Snippet) {
	read(target: Located, READ_SNIPPET: (snippet: Nodes.Snippet<any>) => void) {
		READ_SNIPPET(this);
	}
	declare static read: <T2, T3 extends Node | LocArray>(
		this: T2,
		...args: [Located, (snippet: Nodes.Snippet<T3>) => void] | unknown[]
	) => Nodes.Snippet<T3>;
}

function toSnippet<T extends Node | LocArray>(target: Located, READ_SNIPPET: (snippet: Nodes.Snippet<T>) => void): Nodes.Snippet<T> {
	return withParserState(target.loc.src, ownStart(target), () => Snippet.read(target, READ_SNIPPET));
}

export function parseFile(code: string, options: ParserOptions = {}): Nodes.SourceFile {
	checkOptions(options);
	return SourceFile.read(code, options);
}

export function toExpression(tokens: LocArray<any, DelimNameMap[Exclude<DelimKind, DelimKind.None>]>): Nodes.Snippet<ExpressionNode> {
	return toSnippet(tokens, (snippet) => {
		snippet.ast = read_expression();
	});
}

export function toCallExpressionArguments<T extends LocArray>(tokens: T): Nodes.Snippet<LocArray<ExpressionNode, DelimNameMap[T["dk"]]>> {
	return toSnippet(tokens, (snippet) => {
		snippet.ast = read_sequence(getGroupDelimKind(currChar()) as T["dk"], () => read_expression());
	});
}

export function toBlockBody<T extends LocArray>(tokens: T): Nodes.Snippet<LocArray<StatementNode, DelimNameMap[T["dk"]]>> {
	return toSnippet(tokens, (snippet) => {
		const tk = getDelimKind(currChar()) as T["dk"];
		if (DelimKind.None === tk) read_group_noDelim(snippet, () => read_statement());
		else read_group(snippet, tk, () => read_statement());
	});
}

export function toTokens(node: Node): Nodes.Snippet<LocArray<AttrSegment, "None">> {
	return toSnippet(node, (snippet) => {
		snippet.ast = read_tokens_until(clamp(0, GET_LENGTH(), end(node) - 1));
	});
}
