import { AttributeOrDocComment, Literal, LiteralKind, Node, Program, SourceFile } from "../parser/nodes";
import {
	countRawLiteralHashtags,
	each_node,
	end,
	is_AttributeOrDocComment,
	is_Identifier,
	is_LineCommentNode,
	is_Literal,
	is_LiteralStringLike,
	is_MissingNode,
	is_Program,
	start,
} from "../utils/ast";
import { assert, binarySearch, each, find_last, getLineStarts, get_extra_indent, last_of, print_string } from "../utils/common";

function tab_aware_index(str: string, index: number) {
	return index + get_extra_indent(str.slice(0, index + 1));
}

function is_within(node: pNode, start: number, end: number) {
	return node.start >= start && node.end <= end;
}

function repeat(char: string, length: number) {
	return char.repeat(length);
}

type Segment = { start: number; text: string };
function edit_string(str: string, { start, text }: Segment, replace: boolean) {
	if (str.length === start) return str + text;
	else if (str.length < start) return str.padEnd(start) + text;
	else return str.slice(0, start) + text + str.slice(start + (replace ? text.length : 0));
}

const MAX_LENGTH = 140;

class pNode {
	get name() {
		return this.node.type;
	}
	get start() {
		return start(this.node);
	}
	get end() {
		return end(this.node);
	}
	get parentNode() {
		return this.file.ChildToParentMap.get(this.node)!;
	}
	lines: pLine[] = [];
	constructor(public file: pFile, public node: Node) {}
	is_multiline() {
		return this.lines.length > 1;
	}
	startsAtLine(line: pLine) {
		return this.lines[0] === line;
	}
	endsAtLine(line: pLine) {
		return last_of(this.lines) === line;
	}
	contains(child: pNode) {
		return is_within(child, this.start, this.end);
	}
	getFirstLine() {
		return this.lines[0];
	}
	getLastLine() {
		return this.lines[this.lines.length - 1];
	}
}

class pLine {
	nodes: pNode[] = [];
	length: number;

	constructor(public file: pFile, public start: number, public end: number, public index: number) {
		this.length = end - start;
	}

	hasNodeRef(node: Node) {
		return this.nodes.some((p) => p.node === node);
	}

	content() {
		return this.file.code.slice(this.start, this.end - (this.index === this.file.lines.length - 1 ? 0 : 1));
	}
	sliceNode(node: pNode) {
		return this.file.code.slice(Math.max(this.start, node.start), Math.min(this.end, node.end));
	}
	getInnerMostNodeAt(i: number) {
		assert(i < this.length);
		let current = this.nodes[0];
		for (let j = 1; j < this.nodes.length; j++) {
			const node = this.nodes[j];
			if (
				node.start <= this.start + j && //
				node.end >= this.start + j &&
				current.contains(node)
			) {
				current = node;
			}
		}
		return current;
	}
	print() {
		const code = this.content().trimEnd();
		if (this.nodes.length === 0 || code.length === 0) {
			return code + "\n";
		}
		const has_LineComment = this.nodes.some((p) => is_LineCommentNode(p.node));
		const StringStart = find_last(this.nodes, (p) => is_LiteralStringLike(p.node) && p.is_multiline() && !p.endsAtLine(this))
			?.node as Literal;

		this.nodes.sort((a, b) => (a.contains(b) ? -1 : b.contains(a) ? 1 : a.start < b.start ? -1 : a.end < b.end ? -1 : 1));

		const printed_nodes = this.nodes.filter((p) =>
			is_Identifier(p.node)
				? is_Literal(p.parentNode) || p.node.loc.len() !== p.node.name.trim().length
				: p.startsAtLine(this) || p.endsAtLine(this)
		);
		if (printed_nodes.length === 0) return code + "\n";
		const lines = Array.from(printed_nodes, () => "");
		function edit_line(i: number, { start, text }: Segment) {
			lines[i] = edit_string(lines[i], { start, text }, true);
		}
		each(printed_nodes, (pnode, i) => {
			const text = is_MissingNode(pnode.node) ? "^".repeat(pnode.node.loc.len() + 1) : this.sliceNode(pnode);
			const start = tab_aware_index(code, (pnode.startsAtLine(this) ? pnode.start : this.start) - this.start);
			edit_line(i, { start, text: print_string(text) });
		});
		const right = Math.max(...lines.map((text) => text.length), code.length) + 4;

		const toName = (pnode: pNode) => {
			let name = pnode.name;
			if (
				is_AttributeOrDocComment(pnode.node) &&
				this.file.program.danglingAttributes?.includes(pnode.node as AttributeOrDocComment)
			) {
				name += " (dangling)";
			}
			if (pnode.is_multiline()) {
				if (pnode.startsAtLine(this)) return `<${name}>`;
				else return `</${name}>`;
			}
			return name;
		};
		const names = printed_nodes.map(toName);
		each([...printed_nodes].reverse(), (node) => {
			const i = printed_nodes.indexOf(node);
			const content = (lines[i] || "").trimEnd();
			const prev_content = (lines[i - 1] || "").trimEnd();
			if (content === prev_content) {
				names[i] = "";
				names[i - 1] = `${names[i - 1]}, ${toName(node)}`;
			}
		});
		each(printed_nodes, (node, i) => {
			const name = names[i];
			edit_line(i, { start: right, text: name });
		});

		{
			const plines = lines.filter((line, i) => !!names[i]);
			if (has_LineComment && plines.length === 1 && /^\s*\/\//.test(plines[0])) {
				/**
				 *  // foo
				 *  //•foo		Comment
				 */
				return code + "\n" + plines[0] + "\n";
			}
			const last = plines.pop()!;
			const h = StringStart ? "#".repeat(countRawLiteralHashtags(StringStart)) : "";
			const s = StringStart
				? StringStart.kind === LiteralKind.brString
					? `br${h}`
					: StringStart.kind === LiteralKind.rString
					? `r${h}`
					: ""
				: "";

			return [
				has_LineComment //
					? code + "\n" + " ".repeat(MAX_LENGTH - 2) + "/*"
					: StringStart
					? code.padEnd(MAX_LENGTH - (get_extra_indent(code) + 3 + h.length)) + `"${h}/*`
					: code.padEnd(MAX_LENGTH - (get_extra_indent(code) + 2)) + "/*", //
				...plines,
				StringStart
					? last.padEnd(MAX_LENGTH - (3 + s.length)) + `*/${s}"` //
					: last.padEnd(MAX_LENGTH - 2) + "*/",
				"",
			].join("\n");
		}
	}
}

class pFile {
	readonly lines: pLine[];
	readonly lineStarts: number[];
	readonly code: string;
	readonly program: Program;
	readonly ChildToParentMap: WeakMap<Node, Node>;
	constructor(file: SourceFile) {
		this.code = file.program.loc.src.code;
		this.program = file.program;
		this.lineStarts = getLineStarts(this.code);
		this.lines = this.lineStarts.map((start, i) => new pLine(this, start, this.lineStarts[i + 1] ?? this.code.length, i));
		this.ChildToParentMap = new WeakMap();
		each_node(file, (node, parent) => {
			this.ChildToParentMap.set(node, parent);
			if (!is_Program(node)) {
				const p = new pNode(this, node);
				var i = binarySearch(this.lineStarts, p.start);
				var line = this.lineAt(i);
				addNode(line, p);
				while (p.end > line.end) {
					addNode((line = this.lineAt(++i)), p);
				}
			}
		});
		function addNode(line: pLine, node: pNode) {
			line.nodes.push(node);
			node.lines.push(line);
		}
	}
	lineAtIndex(charIndex: number) {
		return this.lineAt(binarySearch(this.lineStarts, charIndex));
	}
	lineAt(lineIndex: number) {
		return this.lines[lineIndex];
	}
	print() {
		return this.lines.map((line) => line.print()).join("");
	}
	toString() {
		return this.print();
	}
}

// export function astDebugPrinter(file: SourceFile) {
// 	return new pFile(file);
// }

export function ast_debug_print(file: SourceFile) {
	return new pFile(file).print();
}
