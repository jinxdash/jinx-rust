import { hasOwnStart, ownStart, start } from ".";
import { AttributeOrDocComment, AttributeTarget, Located, Node, NodeType } from "../../parser/nodes";
import { assert, binaryInsertEach, binaryInsertIn, exit, is_non_empty_array, Narrow } from "../common";
import { hasAttributes } from "./helpers";
import { isOuter } from "./nodetype";

// function isSortedChildNodeArray(parent: Located, childNodes: Located[]): boolean {
// 	let i = childNodes.length;
// 	if (0 === i) return true;
// 	if (end(parent) < end(childNodes[--i])) return false;
// 	while (0 !== i) if (start(childNodes[i]) < end(childNodes[--i])) return false;
// 	if (start(childNodes[0]) < start(parent)) return false;
// 	return true;
// }

// function assert_sorted(parent: Located, childNodes: Located[]) {
// 	assert(isSortedChildNodeArray(parent, childNodes), "Unexpected: unordered node Array", { parent, childNodes });
// }

function assert_realPos(target: Located, pos: number) {
	if (typeof pos !== "number" || pos < 0 || (target.loc.src && pos > target.loc.src.code.length)) {
		exit("Attempted to set range to non-existing position", { target, pos, code_length: target.loc.src.code.length });
	}
}

export function unsafe_set_nodeType(node: Node, nodeType: NodeType) {
	// @ts-expect-error readonly
	node.nodeType = nodeType;
}

export function unsafe_setRangeStart(target: Located, startPos: number) {
	__DEV__: assert_realPos(target, startPos);
	// @ts-expect-error readonly
	target.loc[0] = startPos;
}

export function unsafe_setRangeEnd(target: Located, endPos: number) {
	__DEV__: assert_realPos(target, endPos);
	// @ts-expect-error readonly
	target.loc[1] = endPos;
}

function internal_saveOwnStart(target: Located) {
	__DEV__: assert(!hasOwnStart(target)), assert_realPos(target, start(target)); // @ts-expect-error readonly
	target.loc[2] = start(target);
}

// prettier-ignore
export function setRangeStart(target: Located, startPos: number) {
	// __DEV__: assert(range_start >= 0 && (start(target) === 0 || range_start < start(target)), d`Attempted to setRangeStart(${target}, ${start(target)} -> ${range_start})`);
	unsafe_setRangeStart(target, startPos);
}

// prettier-ignore
export function setRangeEnd(target: Located, endPos: number) {
	// __DEV__: assert(range_end > 0 && (end(target) === 0 || range_end > end(target)), d`Attempted to setRangeEnd(${target}, ${end(target)} -> ${range_end})`);
	unsafe_setRangeEnd(target, endPos);
}

export function setRange(target: Located, startPos: number, endPos: number) {
	setRangeStart(target, startPos);
	setRangeEnd(target, endPos);
}

export function deleteAttributes(target: Node) {
	__DEV__: assert("attributes" in target);
	if (hasOwnStart(target)) {
		unsafe_setRangeStart(target, ownStart(target));
		// @ts-expect-error readonly
		delete target.loc[2];
	}
	delete target.attributes;
}

export function assignAttributes(target: Node, attrs: AttributeOrDocComment[]) {
	__DEV__: Narrow<AttributeTarget>(target), assert(is_non_empty_array(attrs));

	if (hasAttributes(target)) insertNodes(target.attributes, attrs);
	else target.attributes = [...attrs];

	if (isOuter(attrs[0])) {
		if (!hasOwnStart(target)) internal_saveOwnStart(target);
		setRangeStart(target, start(attrs[0]));
	}

	// __DEV__: assert_sorted(target, target.attributes!);
}

export function transferAttributes(from: Node, to: Node) {
	if (hasAttributes(from)) {
		assignAttributes(to, from.attributes);
		deleteAttributes(from);
	}
}

/** Insert node in array sorted by start loc */
export function insertNode<T extends Node[]>(array: T, node: T[number]) {
	binaryInsertIn(array, node, start);
}

export function insertNodes<T extends Node[]>(array: T, nodes: T[number][]) {
	binaryInsertEach(array, nodes, start);
}
