import { DelimKind, Literal, LiteralKind, LocArray, Located, Node, NodeType, SourceFile, TK } from "../parser/nodes";
import {
	ChildNodeIndex,
	countRawLiteralHashtags,
	each_node,
	end,
	getAstPath,
	hasOwnStart,
	isLocEqual,
	is_CommentOrDocComment,
	is_FunctionDeclaration,
	is_Literal,
	is_LiteralStringLike,
	is_LocArray,
	is_Node,
	is_Program,
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
	iLast,
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

	abstract shouldOmit(): boolean;

	customPrint?(): string | undefined;
}

function labelNode(node: Node) {
	switch (node.nodeType) {
		case NodeType.Attribute:
		case NodeType.DocCommentAttribute:
			return (
				(node.loc.src.program.danglingAttributes?.includes(node) ? "(dangling) " : "") + //
				// (node.inner ? "#!" : "#") +
				node.type
			);
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

	shouldOmit() {
		switch (this.item.nodeType) {
			case NodeType.Identifier:
				return (
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
	shouldOmit(): boolean {
		return false;
	}
	// customPrint(): string | undefined {
	// 	return "^";
	// }
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
}

class pLinedItem {
	readonly name: string;
	readonly content: string;
	readonly printWidth: number;
	readonly relativeStartIndex: number;
	readonly relativeEndIndex: number;
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
	}
	getDepth() {
		return getAstPath(this.item.item.loc.src, this.item.parentNode).length - +is_LocArray(this.item.item);
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

// this.length = end - start;
// this.actualContent = this.f.code.slice(start, end);
// this.content = this.actualContent.replace(/[\r\n]*$/, "");
// this.blankContent = this.actualContent.replace(/[^\t]/g, " ");
// this.lineWidth = calcLineWidth(this.content);
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
		(p) => (p.firstLine === LINE || p.lastLine === LINE) && !p.shouldOmit(),
		(a, b) => (a.item.loc.contains(b.item) ? -1 : b.item.loc.contains(a.item) ? 1 : a.start < b.start ? -1 : 1),
		(p) => new pLinedItem(LINE, p)
	);

	if (ITEMS.length === 0) {
		return LINE.content;
	}

	const start_col2 = Math.max(...ITEMS.map((p) => p.printWidth)) + 4;

	function isForeignChildArray(a: pLinedItem, b: pLinedItem) {
		return a.item instanceof pLocArray && a.item.parentNode !== b.item.item;
	}
	function isOwnChild(child: pLinedItem, b: pLinedItem) {
		return child.item.parentNode === b.item.item;
	}

	let has_multiline_ends = false;
	const lines = combineItems(
		ITEMS,
		(prev, next) =>
			prev.content === next.content &&
			!next.item.isMultiline &&
			!(next instanceof pNodeOwnStart) &&
			!isForeignChildArray(prev, next) &&
			!isForeignChildArray(next, prev),
		(chunk) => {
			if (someMultilineEnd(chunk)) {
				has_multiline_ends = true;
				chunk.reverse();
			}
			return chunk;
		},
		(chunks) => {
			if (has_multiline_ends) {
				// let element;
				// for (var i = 0; i < chunks.length; i++) {
				// 	if (!someMultiline((element = chunks[i]))) break;
				// }
				// reorders </multilines>
				x: if ((chunks.every((chunk) => someMultilineEnd(chunk)), false)) {
					// chunks.reverse();
				} else {
					// chunks.sort((a, b) =>
					// 	someMultilineEnd(a) && someMultilineEnd(b)
					// 		? last_of(b).getDepth() - last_of(a).getDepth() /* || b[0].item.start - a[0].item.start */
					// 		: (someMultilineEnd(a) ? a[0].relativeEndIndex - 1 : a[0].relativeStartIndex) -
					// 		  (someMultilineEnd(b) ? b[0].relativeEndIndex - 1 : b[0].relativeStartIndex)
					// );
					// break x;
					// chunks.sort(
					// 	(a, b) =>
					// 		-(
					// 			last_of(a).relativeEndIndex - last_of(b).relativeEndIndex ||
					// 			last_of(a).relativeStartIndex - last_of(b).relativeStartIndex
					// 		)
					// );
					let i = 0;
					const r: pLinedItem[][] = [];
					while (i < chunks.length) {
						// const pre = i;
						// while (i < chunks.length && !someMultilineEnd(chunks[i])) i++;
						// if (pre !== i) r.push(...chunks.slice(pre, i));
						// // start ---> multilines | --->
						// if (i >= chunks.length) break;
						const start = i++;
						while (i < chunks.length && someMultilineEnd(chunks[i])) i++;
						// const end = max(chunks[i - 1][0].relativeEndIndex, last_of(chunks[i - 1]).relativeEndIndex);
						let j = i;
						while (j < chunks.length && !someMultilineEnd(chunks[i]) /* chunks[j][0].relativeEndIndex <= end */) j++;
						// assert(i !== 0);
						// const post = chunks.splice(j + 1);
						/* if (j === i) chunks.reverse();
					else */
						r.push(...chunks.slice(i, j), ...chunks.slice(start, i).reverse());
						// chunks.splice(j - 1, 0, ...chunks.splice(start, i - start).reverse());
						i = j;
						// while (j < chunks.length && !someMultilineEnd(chunks[j])) j++;
						// r.push(...chunks.slice(i, j));
						// i = j;
					}
					i = 0;
					// chunks=r;
					// r.push(...chunks.slice(i));
					// assert(chunks.length === r.length);
					const d: typeof r = [];
					while (i < r.length - 1) {
						const start = i;
						const prev = r[i][0];
						while (++i < r.length) {
							if (r[i][0].item.end < prev.item.start) {
								let j = i;
								while (++j < r.length && r[j][0].item.end < prev.item.start);
								// r.splice(start, 0, ...r.splice(i, j));
								d.push(...r.slice(i, j), ...r.slice(start, i));
								i = j;
							}
						}
					}
					chunks = [...d, ...r.slice(d.length)].sort(
						(a, b) =>
							(someMultilineEnd(a) ? a[0].relativeEndIndex - 1 : a[0].relativeStartIndex) -
							(someMultilineEnd(b) ? b[0].relativeEndIndex - 1 : b[0].relativeStartIndex)
					);
					// for (let i = 0; i < chunks.length - 1; i++) {
					// 	const prev = chunks[i][0];
					// 	const next = chunks[i + 1][0];
					// 	if (next.relativeEndIndex < prev.relativeStartIndex) {
					// 	}
					// }
					// chunks.push(...chunks.splice(0, i).reverse(), ...post);
					// chunks.reverse();
				}
			}
			return chunks.map((chunk) => {
				const { content } = chunk[0];
				return (
					content +
					" ".repeat(start_col2 - calcLineWidth(content)) +
					chunk
						.map((item) => item.name)
						.join(
							chunk.some(({ item }) => item.isMultiline) //
								? " "
								: ", "
						)
				);
			});
		}
	);
	function someMultilineEnd(chunk: pLinedItem[]) {
		return chunk.some(({ item }) => item.isMultiline && LINE === item.lastLine);
	}

	{
		let has_LineComment = false;
		let has_blockComment = false;

		let in_blockComment = false;
		let stops_blockComment = false;

		for (let i = 0; i < LINE.items.length; i++) {
			const p = LINE.items[i];
			const node = p.item;
			if (is_Node(node) && is_CommentOrDocComment(node)) {
				has_LineComment ||= node.line;
				has_blockComment ||= !node.line;
				in_blockComment ||= !node.line && p.isMultiline && LINE === p.firstLine;
				stops_blockComment ||= !node.line && p.isMultiline && LINE === p.lastLine;
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
		let last_last = "";

		while (has_LineComment && /^\s*\/\//.test(last) && lines.length !== 0) {
			last_last = "\n" + last + last_last;
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
			padBetween(last, C1 + (StringStart ? `${s}"` : ""), MAX_LENGTH) + last_last,
		].join("\n");
		/* (StringStart
					? LINE.content.padEnd(MAX_LENGTH - (getTabAddedWidth(LINE.content) + (1 + h.length) + C0.length)) + `"${h}` + C0
					: LINE.content.padEnd(MAX_LENGTH - (getTabAddedWidth(LINE.content) + C0.length)) + C0
			).trimEnd(), // */
		// StringStart
		// 	? last + " ".repeat(max(0, MAX_LENGTH - (calcLineWidth(last) + C1.length + (s.length + 1)))) + C1 + `${s}"` //
		// 	: last + " ".repeat(max(0, MAX_LENGTH - (calcLineWidth(last) + C1.length))) + C1,
		function padBetween(left: string, right: string, targetWidth: number) {
			return left.padEnd(max(0, targetWidth - getTabAddedWidth(left) - right.length)) + right;
		}
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
			item.isMultiline = false;
			item.lastLine = item.firstLine;
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
