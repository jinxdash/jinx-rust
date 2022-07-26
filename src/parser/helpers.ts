import { Narrow } from "../utils/common";
import { BaseNode, DelimKind, DelimNameMap, Loc, LocArray, Node, NodeType, NTMap } from "./nodes";

export function mockNode<NT extends NodeType>(nodeType: NT, loc: Loc, obj: Omit<NTMap[NT], keyof BaseNode>): NTMap[NT] {
	var node = new BaseNode(nodeType, loc) as any;
	for (var key in obj) node[key] = obj[key];
	return node;
}

export function createLocArray<T extends Node, D extends DelimKind>(dk: D, loc: Loc, array: T[] = []): LocArray<T, DelimNameMap[D]> {
	__DEV__: Narrow<LocArray<T, any>>(array);
	array.loc = loc;
	array.dk = dk;
	return array;
}
