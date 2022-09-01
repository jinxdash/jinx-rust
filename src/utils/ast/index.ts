import type { Loc, LocArray, Located, Node } from "../../parser/nodes";
import { AssertTypesEq, exit, is_object } from "../common";

export * from "./helpers";
export * from "./iterator";
export * from "./nodetype";
export * from "./transform";

// meta types

function assert_located(obj: Located) {
	__DEV__: if (!is_Located(obj)) exit("Expected Object.loc", obj);
}

export function is_Loc(data: any): data is Loc {
	return is_object(data) && "src" in data;
}

export function is_Located(data: any): data is Located {
	return is_object(data) && "loc" in data;
}

export function is_Node(node: any): node is Node {
	return is_object(node) && "nodeType" in node;
}

__DEV__: AssertTypesEq<never, Extract<Node, { length: any }>>();
export function is_LocArray(data: any): data is LocArray<Node> {
	return is_Located(data) && "length" in data;
}

// loc helpers

export function start(node: Located): number {
	__DEV__: assert_located(node);
	return node.loc[0];
}

export function end(node: Located): number {
	__DEV__: assert_located(node);
	return node.loc[1];
}

export function hasOwnStart(node: Located): boolean {
	__DEV__: assert_located(node);
	return 2 in node.loc;
}

export function ownStart(node: Located): number {
	__DEV__: assert_located(node);
	return 2 in node.loc ? node.loc[2]! : start(node);
}

export function isLocEqual(a: Loc, b: Loc): boolean {
	__DEV__: (is_Loc(a) && is_Loc(b)) || exit("Expected Loc", a, b);
	return (
		(2 in a ? 2 in b && a[2] === b[2] : !(2 in b)) &&
		a[0] === b[0] && //
		a[1] === b[1] &&
		a.src === b.src
	);
}
