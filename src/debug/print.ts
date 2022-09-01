import { DelimKind, Literal, LiteralKind, LocArray, Located, Node, NodeType, SourceFile, TK } from "../parser/nodes";
import {
	ChildNodeIndex,
	countRawLiteralHashtags,
	each_node,
	end,
	getAstPath,
	hasOwnStart,
	isDangling,
	isLocEqual,
	is_CommentOrDocComment,
	is_FunctionDeclaration,
	is_Literal,
	is_LiteralStringLike,
	is_LocArray,
	is_Node,
	ownStart,
	start,
} from "../utils/ast";
import {
	assert,
	binarySearch,
	calcLineWidth,
	count_occurences,
	find_last,
	getTabAddedWidth,
	is_array,
	last_of,
	max,
	min,
	print_string,
} from "../utils/common";

function edit_string(str: string, start: number, text: string) {
	return str.slice(0, start).padEnd(start) + text + str.slice(text.length + start);
}
const MAX_LENGTH = 140;
const FILTER_KEYS = new Set(["loc", "nodeType", "__devonly"]);
abstract class pItem<T extends Located = Located> {
	abstract get label(): string;
	props(): string[] {
		return Object.keys(this.item).filter((key) => !FILTER_KEYS.has(key));
	}
	get start() {
		return start(this.item);
	}
	get end() {
		return end(this.item);
	}
	declare firstLine: pLine;
	declare lastLine: pLine;
	declare isMultiline: boolean;
	constructor(readonly item: T, readonly parentNode: Node, readonly key: string, readonly index?: ChildNodeIndex) {}

	abstract shouldOmit(line: pLine): boolean;

	canInlineWith(item: pItem): boolean {
		return true;
	}

	customPrint?(): string | undefined;
}

class pNode extends pItem<Node> {
	get label() {
		switch (this.item.nodeType) {
			case NodeType.MissingNode:
				return `MissingNode (${labelNode(this.parentNode)}${composeKey(this.key, this.index)})`;
		}
		return labelNode(this.item);
	}
	props() {
		return super.props().filter((key) => !is_Node(this.item[key]) && !is_array(this.item[key]));
	}

	shouldOmit(line: pLine) {
		switch (this.item.nodeType) {
			case NodeType.Identifier:
				return (
					!this.isMultiline &&
					this.item.loc.len() === this.item.name.trim().length && //
					!is_Literal(this.parentNode)
				);
			default:
				return false;
		}
	}

	customPrint() {
		switch (this.item.nodeType) {
			case NodeType.MissingNode:
				return "^".repeat(this.item.loc.len() + 1);
		}
	}

	canInlineWith(): boolean {
		switch (this.item.nodeType) {
			case NodeType.Attribute:
			case NodeType.DocCommentAttribute:
				return !isDangling(this.item);
			default:
				return true;
		}
	}
}

class pNodeOwnStart extends pNode {
	get start() {
		return ownStart(this.item);
	}
	get label(): string {
		return `${super.label}~ownStart`;
	}
	props(): string[] {
		return [];
	}
	shouldOmit(line: pLine): boolean {
		return this.isMultiline && this.lastLine === line;
	}
	canInlineWith(): boolean {
		return false;
	}
}

class pLocArray extends pItem<LocArray> {
	get label() {
		return labelNode(this.parentNode) + composeKey(this.key, this.index);
	}
	props() {
		const props = super.props().slice(this.item.length);
		if (is_FunctionDeclaration(this.parentNode) && this.key === "parameters") {
			return props.filter((key) => key !== "self");
		}
		return props;
	}

	shouldOmit() {
		switch (this.parentNode.nodeType) {
			case NodeType.SourceFile:
			case NodeType.Program:
			case NodeType.Snippet:
				return false;
			default:
				return isLocEqual(this.item.loc, this.parentNode.loc);
		}
	}
	canInlineWith(p: pItem<Located>): boolean {
		return this.parentNode === p.item;
	}
}

class pLinedItem {
	readonly name: string;
	readonly content: string;
	readonly printWidth: number;
	readonly relativeStartIndex: number;
	readonly relativeEndIndex: number;
	readonly endsLine: boolean;
	constructor(line: pLine, readonly item: pItem) {
		this.name = item.isMultiline
			? line === item.firstLine
				? `<${item.label}${composeProps(item.item, item.props())}>`
				: `</${item.label}>`
			: item.label + composeProps(item.item, item.props());
		this.content = edit_string(
			line.blankContent,
			(this.relativeStartIndex = line === item.firstLine ? item.start - line.start : 0),
			print_string(item.customPrint?.() ?? sliceAtLine(item, line))
		)
			.trimEnd()
			.replace(/^╚+/g, (s) => " ".repeat(s.length * 3) + s);
		this.printWidth = calcLineWidth(this.content);
		this.relativeEndIndex = min(line.end, item.end) - line.start;
		this.endsLine = this.item.isMultiline && this.item.lastLine === line;
	}
	getDepth() {
		return getAstPath(this.item.item.loc.src, this.item.parentNode).length - +is_LocArray(this.item.item);
	}
}

class pLine {
	constructor(
		private readonly f: pFile, //
		readonly start: number,
		readonly end: number
	) {}

	readonly items: pItem[] = [];
	readonly content = this.f.file.code.slice(this.start, this.end).replace(/[\r\n]*$/, "");
	readonly blankContent = this.f.file.code.slice(this.start, this.end).replace(/[^\t]/g, " ");
}

function print(FILE: pFile, LINE: pLine) {
	const ITEMS = filterSortMap(
		LINE.items,
		(p) => (p.firstLine === LINE || p.lastLine === LINE) && !p.shouldOmit(LINE),
		(a, b) => (a.item.loc.contains(b.item) ? -1 : b.item.loc.contains(a.item) ? 1 : a.start < b.start ? -1 : 1),
		(p) => new pLinedItem(LINE, p)
	);

	if (ITEMS.length === 0) {
		return LINE.content;
	}

	const start_col2 = Math.max(...ITEMS.map((p) => p.printWidth)) + 4;

	const lines = combineItems(
		ITEMS,
		(prev, next) =>
			!next.item.isMultiline &&
			prev.content === next.content &&
			prev.item.canInlineWith(next.item) &&
			next.item.canInlineWith(prev.item),
		(cs) => (someMultilineEnd(cs) ? cs.reverse() : cs),
		(cs) =>
			(cs.some((c) => someMultilineEnd(c))
				? (function reorderChunks(r: pLinedItem[][]) {
						for (var i = 0, j = 0, start = 0; i < r.length - 1; i = j) {
							for (start = i++; i < r.length && someMultilineEnd(r[i]); i++);
							for (j = i + 1; j < r.length && !someMultilineEnd(r[j]); j++);
							r.splice(start + (j - i), 0, ...r.splice(start, i - start).reverse());
						}
						for (var i = 0, j = 0, start = 0; i < r.length - 1; i = j) {
							for (start = i++; i < r.length && directionalPos(r[i]) >= directionalPos(r[start]); i++);
							for (j = i + 1; j < r.length && directionalPos(r[j]) < directionalPos(r[start]); j++);
							r.splice(start + (j - i), 0, ...r.splice(start, i - start));
						}
						return r.sort((a, b) => directionalPos(a) - directionalPos(b));
				  })(cs)
				: cs
			).map(
				(cs) =>
					cs[0].content +
					" ".repeat(start_col2 - calcLineWidth(cs[0].content)) +
					cs.map((item) => item.name).join(cs.some(({ item }) => item.isMultiline) ? " " : ", ")
			)
	);
	function directionalPos(c: pLinedItem[]) {
		return someMultilineEnd(c) ? c[0].item.end - 1 : c[0].item.start;
	}
	function someMultilineEnd(c: pLinedItem[]) {
		return c.some((c) => c.endsLine);
	}

	{
		let has_LineComment = false;
		let has_blockComment = false;

		for (let i = 0; i < LINE.items.length; i++) {
			const p = LINE.items[i];
			const node = p.item;
			if (is_Node(node) && is_CommentOrDocComment(node)) {
				has_LineComment ||= node.line;
				has_blockComment ||= !node.line;
			}
		}

		let safe_LineComment = false;
		if (has_LineComment) {
			if (lines.every((line) => /^\s*\/\//.test(line))) {
				return LINE.content + "\n" + lines.join("\n");
			}

			if (lines[0].startsWith("  ")) {
				safe_LineComment = true;
				lines[0] = "/*" + lines[0].slice(2);
			} else if (lines[0].startsWith("\t")) {
				safe_LineComment = true;
				lines[0] = "/*\t" + lines[0].slice(1);
			}
		} else if (LINE.start === 0 && LINE.start === LINE.end) {
			// empty file
			safe_LineComment = true;
			lines[0] = "/*" + lines[0].slice(2);
		}

		let multilineStarts = 1;
		let multilineEnds = 1;
		if (!safe_LineComment && has_blockComment) {
			for (let i = 0; i < lines.length; i++) {
				const x = count_occurences(lines[i], /\/\*/g);
				const x2 = count_occurences(lines[i], /\*\//g);
				const diff = x - x2;
				if (diff !== 0) {
					multilineStarts += x2;
					multilineEnds += x;
				}
			}
		}

		let last = lines.pop()!;
		let postfix = "";

		while (has_LineComment && /^\s*\/\//.test(last) && lines.length !== 0) {
			postfix = "\n" + last + postfix;
			last = lines.pop()!;
		}

		const StringStart = find_last(
			LINE.items,
			(p) => is_Node(p.item) && is_LiteralStringLike(p.item) && p.isMultiline && LINE !== p.lastLine
		)?.item as Literal | undefined;

		const h = StringStart ? "#".repeat(countRawLiteralHashtags(StringStart)) : "";
		const s = StringStart
			? StringStart.kind === LiteralKind.brString
				? `br${h}`
				: StringStart.kind === LiteralKind.rString
				? `r${h}`
				: ""
			: "";

		const C0 = "/*".repeat(multilineStarts);
		const C1 = "*/".repeat(multilineEnds);
		return [
			safe_LineComment
				? LINE.content
				: has_LineComment //
				? LINE.content + "\n" + C0.padStart(MAX_LENGTH)
				: padBetween(LINE.content, (StringStart ? `"${h}` : "") + C0, MAX_LENGTH),
			...lines,
			padBetween(last, C1 + (StringStart ? `${s}"` : ""), MAX_LENGTH) + postfix,
		].join("\n");
	}
}

function padBetween(left: string, right: string, targetWidth: number) {
	return left.padEnd(max(0, targetWidth - getTabAddedWidth(left) - right.length)) + right;
}

function labelNode(node: Node) {
	switch (node.nodeType) {
		case NodeType.Attribute:
		case NodeType.DocCommentAttribute:
			return node.type + (isDangling(node) ? "#DANGLING" : "");
		default:
			return node.type;
	}
}

function composeKey(key: string, index: ChildNodeIndex) {
	switch (typeof index) {
		case "undefined":
			return `.${key}`;
		case "number":
			return `.${key}[${index}]`;
		default:
			return `.${key}.${index}`;
	}
}

function composeProps(item: Located, keys: string[]) {
	return printObject();

	function printObject() {
		const pairs: string[] = [];
		for (const key of keys) {
			var value = item[key];
			if (value == null) continue;
			pairs.push(printValue(item, key, value));
		}
		if (pairs.length === 0) return "";
		return `{${pairs.join(", ")}}`;
	}

	function printValue(item: Located, key: string, value: any) {
		switch (value) {
			case true:
				return key;
			case false:
				return "!" + key;
		}

		switch (key) {
			case "tk":
				// @ts-expect-error const enum
				return `tk: "${TK[item.tk]}"`; // return `tk: TK["${TK[item.tk]}"]`;
			case "dk":
				// @ts-expect-error const enum
				return `dk: "${DelimKind[item.dk]}"`; // return `dk: DelimKind["${DelimKind[item.dk]}"]`;
		}

		if (is_Node(item)) {
			switch (item.nodeType) {
				case NodeType.Literal:
					if (key === "kind") {
						// @ts-expect-error const enum
						return `kind: ${LiteralKind[item.kind]}`; // return `kind: LiteralKind.${LiteralKind[item.kind]}`;
					}
					break;
			}
		}
		return `${key}: ${JSON.stringify(value)}`;
	}
}

function sliceAtLine(p: pItem, line: pLine) {
	return p.item.loc.src.code.slice(
		max(line.start, p.start), //
		min(line.end, p.end)
	);
}

/**
 * given: "{ 0 }"
 * line items: [Block, Stmt, Lit]
 * returns: [[Block], [Stmt, Lit]]
 */
function combineItems<T, R>(
	ITEMS: T[],
	include: (prevItem: T, nextItem: T, currentChunk: T[]) => boolean,
	modify: (chunk: T[]) => T[],
	finalize: (chunks: T[][]) => R
): R {
	const chunks: T[][] = [];
	const addChunk = (chunk: T[]) => chunks.push(modify(chunk));
	if (ITEMS.length !== 0) {
		for (var current = [ITEMS[0]], i = 1; i < ITEMS.length; i++) {
			var next = ITEMS[i];
			if (include(next, last_of(current), current)) {
				current.push(next);
			} else {
				addChunk(current);
				current = [next];
			}
		}
		addChunk(current);
	}
	return finalize(chunks);
}

function filterSortMap<T, R>(items: T[], filter: (item: T) => boolean, sort: (a: T, b: T) => number, map: (item: T) => R): R[] {
	const arr: T[] = [];
	for (let i = 0; i < items.length; i++) if (filter(items[i])) arr.push(items[i]);
	return arr.sort((a, b) => sort(a, b)).map((item) => map(item));
}

class pFile {
	declare lines: pLine[];
	constructor(readonly file: SourceFile) {}
	print() {
		// const maxLineLength = Math.max(...this.lines.map((p) => p.lineWidth)) + 4;
		// return this.lines.map((line) => line.printLine(maxLineLength)).join(""); //.replace(/\t/g, "    ");
		return this.lines.map((line) => print(this, line)).join("\n");
	}
	toString() {
		return this.print();
	}
}

function create(file: SourceFile) {
	const FILE = new pFile(file);
	const { lineStarts, code } = file;

	const LINES: pLine[] = (FILE.lines = lineStarts.map(
		(start, i, a) =>
			new pLine(
				FILE, //
				start,
				i === a.length - 1 ? code.length : a[i + 1]
			)
	));

	const visited = new Set<LocArray>();
	each_node(file, (...args) => {
		const { 0: node /* , 1: parent, 2: key */ } = args;
		// const target = parent[key];
		// maybeCreateArr(target, parent, key);

		for (const key in node) {
			maybeCreateArr(node[key], node, key);
		}

		if (hasOwnStart(node)) {
			const item = createItem(pNodeOwnStart, ...args);
			// item.isMultiline = false;
			// item.lastLine = item.firstLine;
		}
		createItem(pNode, ...args);
	});

	function maybeCreateArr(target: any, parent: Node, key: string) {
		if (is_LocArray(target) && !visited.has(target)) {
			visited.add(target);
			createItem(pLocArray, target, parent, key);
		}
	}

	function createItem<T extends typeof pNode | typeof pLocArray>(ctor: T, ...args: ConstructorParameters<T>) {
		// @ts-expect-error
		const ITEM: pNode | pLocArray = new ctor(...args);
		let i = binarySearch(lineStarts, ITEM.start);
		ITEM.firstLine = LINES[i];
		do (ITEM.lastLine = LINES[i]).items.push(ITEM);
		while (LINES[i++].end < ITEM.end);
		ITEM.isMultiline = ITEM.firstLine !== ITEM.lastLine;
		return ITEM;
	}

	return FILE;
}

export function ast_debug_print(file: SourceFile) {
	return create(file).print();
}
