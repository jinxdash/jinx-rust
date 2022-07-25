import { start } from "../utils/ast";
import { CharCode } from "../utils/CharCode";
import {
	color,
	ColorFn,
	getLineChar,
	getLineStarts,
	getTerminalWidth,
	last_of,
	pretty_join,
	print_string,
	strChar,
	urlAt,
} from "../utils/common";
import { createCustomError } from "../utils/debug";
import { Located, Node, NodeType, NTMap } from "./nodes";
import { check_ahead, currChar, GET_KEYWORD_NAME, GET_LENGTH, GET_POSITION, GET_SOURCEFILEPATH, GET_SOURCETEXT, nStep } from "./state";
import { devonly_getDebugState } from "./state/debug";
import { Keyword, kwTree } from "./state/scanner";

type Expectation = string | CharCode | (string | CharCode)[];
type AtTarget = number | Located;

// prettier-ignore
function format_expectation(data: Expectation) {
	switch (typeof data) {
		case "number":
			switch (data) {
				case CharCode['"']: return `'"'`;
				case CharCode["'"]: return `"'"`;
				default: return `"${print_string(String.fromCharCode(data))}"`;
			}
		case "string": return data;
		case "object":
			return pretty_join(
				data.map(format_expectation).filter(Boolean),
				{ tail: "or", empty:"undefined" }
			);
	}
}
function name_thing(pos: number) {
	if (pos >= GET_LENGTH() - 1) return "End Of File";
	if (GET_POSITION() !== pos) nStep(pos - GET_POSITION());
	return check_ahead(() => {
		const kw = kwTree();
		switch (kw) {
			case Keyword.NotAWord:
				return `token '${strChar(currChar())}'`;
			case Keyword.NotKeyword:
				return `Identifier`;
			default:
				const name = GET_KEYWORD_NAME(kw);
				if (name[0].toUpperCase() === name[0]) return name;
				return `keyword '${name}'`;
		}
	});
}

interface CFLoc {
	line: number;
	column: number;
}

interface CFLocSpan {
	start: CFLoc;
	end?: CFLoc;
}

export interface ParserError extends Error {
	loc: CFLocSpan;
	url: string;
	ctx?: any[];
	parserState?: { [state_name: string]: unknown };
}

const THIS_FILE_NAME = "error.ts";

export function exit(err: string, ...ctx: any[]): never {
	exit.at(GET_POSITION(), err, ...ctx);
}

export namespace exit {
	export function at(pos: number | Located, msg: string, ...ctx: any[]): never {
		const code = GET_SOURCETEXT();

		const { text, loc, url } = printCodeError(msg, code, pos, GET_SOURCEFILEPATH());

		const error = createCustomError({
			message: msg,
			editStack(stack) {
				__DEV__: {
					if (!last_of(stack).filepath.includes(THIS_FILE_NAME))
						stack[0].hideWhileTrue((line) => line.filepath.includes(THIS_FILE_NAME));
					stack.forEach((line) => {
						if (line.i > 3) {
							if (
								line.callee === "new LocatedNode" && //
								line.at(+1)?.raw.includes("at new ") &&
								line.at(+2)?.raw.includes("at Function.read")
							) {
								line.hideNext(3);
							}
							if (line.callee === "read_ahead") {
								line.hideNext(2);
							}
						}
					});
				}
			},
			style: {
				callee: (callee) => (callee.endsWith(".read") || callee.startsWith("new ") ? color.blue : color.cyan),
			},
		}) as ParserError;

		error.url = url;
		error.loc = loc;
		error.ctx = ctx;
		error.toString = function () {
			return text;
		};

		__DEV__: error.parserState = devonly_getDebugState();

		throw error;
	}
	export function never(): never {
		exit("Reached unreachable code");
	}
	export function infinite(): never {
		exit("Maximum call stack size exceeded");
	}
	export function expected(expected: Expectation = "", pos = GET_POSITION()): never {
		at(pos, `Unexpected ${name_thing(pos)}` + (expected && `, expected ${format_expectation(expected)}`));
	}
	export function unexpected(): never {
		expected();
	}
}

// prettier-ignore
export function assert(predicate: boolean, err?: string, ...ctx: any[]): asserts predicate {
	if (false === predicate) exit(err || "Assertion failed", ...ctx);
}

export namespace assert {
	// prettier-ignore
	export function at(pos: AtTarget, predicate: boolean, err?: string, ...ctx: any[]): asserts predicate {
		if (false === predicate) exit.at(pos, err || "Assertion failed", ...ctx);
	}

	export function nis<K extends NodeType>(node: Node, nodeType: K): asserts node is NTMap[K] {
		if (nodeType !== node.nodeType) exit.at(node, `Expected ${NodeType[nodeType]}, found ${NodeType[node.nodeType]}`);
	}

	export function match(data: CharCode) {
		expect(data === currChar());
	}

	export function expect(predicate: boolean, expected?: Expectation) {
		if (false === predicate) exit.expected(expected);
	}
}

function printCodeError(message: string, code: string, pos: Located | number, filepath?: string) {
	const startPos = typeof pos === "number" ? pos : start(pos);
	const lineStarts = getLineStarts(code);
	const { line, char } = safe_getLineChar(code, lineStarts, startPos);
	const SEPARATOR = "\n" + "-".repeat(getTerminalWidth()) + "\n";
	const url = urlAt(filepath ?? "undefined", lineStarts, startPos);
	return {
		text:
			color.grey(
				"" +
					SEPARATOR +
					stringifyLines(
						line - 2,
						line - 1,
						...(hasIndex(code, startPos)
							? [
									{ line, color: color.yellow },
									...(char + 2 + color.unstyledLength(message) > getTerminalWidth() - 8
										? [" ".repeat(char) + "^", message, ""]
										: [" ".repeat(char) + "^ " + message]
									).map((str) => color.red(str)),
							  ]
							: [line, color.red(`x---- ${message}`)]),
						line + 1,
						line + 2
					) +
					SEPARATOR
			) + `ParserError at ${color.blue(color.underline(url))}\n`,
		loc: {
			start: {
				line: line + 1,
				column: char + 1,
			},
		},
		url,
	};

	function hasIndex(data: { length: number }, i: number) {
		return 0 <= i && data.length > i;
	}
	function safe_getLineChar(code: string, lineStarts: number[], pos: number): { line: number; char: number } {
		return hasIndex(code, pos) ? getLineChar(lineStarts, pos) : { line: pos < 0 ? -1 : lineStarts.length, char: 0 };
	}
	function stringifyLines(...plines: (number | string | { line: number; color: ColorFn })[]): string {
		const dlines: [number, string][] = plines.map((data) =>
			typeof data === "number"
				? [data, printLine(data)]
				: typeof data === "string"
				? [-1, data]
				: [data.line, data.color(printLine(data.line))]
		);
		const pre = padEndUniform(dlines.map((data) => "" + (1 + data[0] || "")));
		return dlines
			.map((data, i) => {
				let str = `  ${pre[i]} ${/[^ ]/.test(pre[i]) && !!data[1] ? "|" : " "} ${data[1] ?? ""}`;
				return data[0] !== -1 && data[0] !== line && color.unstyledLength(str) > getTerminalWidth()
					? str.slice(0, getTerminalWidth() - 4) + "..."
					: str;
			})
			.join("\n");
		function padEndUniform(arr: string[]) {
			const actualLengths = arr.map(color.unstyledLength);
			const topLength = Math.max(...actualLengths);
			return arr.map((str, i) => str + " ".repeat(topLength - actualLengths[i]));
		}
		function printLine(i: number): string {
			return hasIndex(lineStarts, i) ? print_string(sliceLine(code, lineStarts, i)) : "";
		}
		function sliceLine(str: string, lineStarts: number[], i: number): string {
			return hasIndex(lineStarts, i) ? str.slice(lineStarts[i], i + 1 === lineStarts.length ? str.length : lineStarts[i + 1]) : "";
		}
	}
}
