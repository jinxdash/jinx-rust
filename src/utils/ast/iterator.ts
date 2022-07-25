import { is_LocArray, is_Node } from ".";
import { FunctionDeclarationParameters, Node } from "../../parser/nodes";
import { assert, exit, is_array, is_defined } from "../common";
import { insertNode, insertNodes } from "./transform";

interface AstItFn<R = void> {
	(child: Node, key: string, index?: number | "self"): R;
}

interface AstItEachFn<R = void> {
	(child: Node, parent: Node, key: string, index?: number | "self"): R;
}

type NodeValues<T> = T extends FunctionDeclarationParameters ? T[number] | T["self"] : T extends ReadonlyArray<any> ? T[number] : T;
export type NodeChildTypes<T> = T extends never ? never : Extract<NodeValues<T[keyof T]>, { nodeType: any }>;

export type PickProps<T, F> = T extends never
	? never
	: { [K in keyof T as T[K] & {} extends infer U ? (U extends F ? K : never) : never]: T[K] };
export type BoolProps<T> = PickProps<T, boolean>;
export type NodeProps<T> = PickProps<T, { nodeType: number }>;
export type ArrayProps<T> = PickProps<T, { nodeType: number }[]>;

function _it_arr(fn: AstItFn, array: Node[], key: string): void {
	for (var i = 0; i < array.length; i++) fn(array[i], key, i);
}

// <generated>
function iterate_nodes(node: Node, fn: AstItFn): void {
	__DEV__: assert.isNode(node);
	if ("attributes" in node) _it_arr(fn, node.attributes, "attributes");
	switch (node.nodeType) {
		case 0:
		case 2:
		case 5:
		case 6:
		case 7:
		case 8:
		case 9:
		case 12:
		case 15:
		case 41:
		case 74:
		case 94:
		case 104:
		case 105:
		case 120:
		case 121:
		case 122:
		case 123:
		case 124:
			break;
		case 1:
			if ("shebang" in node) fn(node.shebang, "shebang");
			fn(node.program, "program");
			break;
		case 3:
			_it_arr(fn, node.ast, "ast");
			_it_arr(fn, node.danglingAttributes, "danglingAttributes");
			_it_arr(fn, node.comments, "comments");
			break;
		case 4:
			if (is_LocArray(node.ast)) _it_arr(fn, node.ast, "ast");
			else fn(node.ast, "ast");
			_it_arr(fn, node.danglingAttributes, "danglingAttributes");
			_it_arr(fn, node.comments, "comments");
			break;
		case 10:
			if ("suffix" in node) fn(node.suffix, "suffix");
			break;
		case 11:
		case 58:
		case 116:
			if (undefined !== node.namespace) fn(node.namespace, "namespace");
			fn(node.segment, "segment");
			break;
		case 13:
		case 14:
			_it_arr(fn, node.segments, "segments");
			break;
		case 16:
			fn(node.callee, "callee");
			_it_arr(fn, node.segments, "segments");
			break;
		case 17:
			fn(node.id, "id");
			_it_arr(fn, node.rules, "rules");
			break;
		case 18:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (is_LocArray(node.rules)) _it_arr(fn, node.rules, "rules");
			else fn(node.rules, "rules");
			break;
		case 19:
		case 20:
			_it_arr(fn, node.match, "match");
			_it_arr(fn, node.transform, "transform");
			break;
		case 21:
			_it_arr(fn, node.segments, "segments");
			if (undefined !== node.sep) fn(node.sep, "sep");
			break;
		case 22:
			fn(node.id, "id");
			fn(node.ty, "ty");
			break;
		case 23:
			if (undefined !== node.location) fn(node.location, "location");
			break;
		case 24:
			if (undefined !== node.abi) fn(node.abi, "abi");
			break;
		case 25:
		case 60:
		case 63:
			if (undefined !== node.expression) fn(node.expression, "expression");
			break;
		case 26:
		case 31:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.import, "import");
			break;
		case 27:
			fn(node.source, "source");
			if (undefined !== node.local) fn(node.local, "local");
			break;
		case 28:
			if (undefined !== node.source) fn(node.source, "source");
			break;
		case 29:
			fn(node.source, "source");
			break;
		case 30:
			if (undefined !== node.source) fn(node.source, "source");
			_it_arr(fn, node.specifiers, "specifiers");
			break;
		case 32:
			if ("pub" in node) fn(node.pub, "pub");
			if (undefined !== node.abi) fn(node.abi, "abi");
			_it_arr(fn, node.body, "body");
			break;
		case 33:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			if (undefined !== node.typeBounds) _it_arr(fn, node.typeBounds, "typeBounds");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			if (undefined !== node.typeExpression) fn(node.typeExpression, "typeExpression");
			break;
		case 34:
		case 35:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.pattern, "pattern");
			fn(node.typeAnnotation, "typeAnnotation");
			if (undefined !== node.expression) fn(node.expression, "expression");
			break;
		case 36:
			fn(node.pattern, "pattern");
			if (undefined !== node.typeAnnotation) fn(node.typeAnnotation, "typeAnnotation");
			if (undefined !== node.expression) fn(node.expression, "expression");
			if (undefined !== node.else) fn(node.else, "else");
			break;
		case 37:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (undefined !== node.body) _it_arr(fn, node.body, "body");
			break;
		case 38:
			if ("pub" in node) fn(node.pub, "pub");
			if ("extern" in node) fn(node.extern, "extern");
			fn(node.id, "id");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			if (undefined !== node.parameters.self) fn(node.parameters.self, "parameters", "self");
			_it_arr(fn, node.parameters, "parameters");
			if (undefined !== node.returnType) fn(node.returnType, "returnType");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			if (undefined !== node.body) _it_arr(fn, node.body, "body");
			break;
		case 39:
			if (undefined !== node.lt) fn(node.lt, "lt");
			if (undefined !== node.typeAnnotation) fn(node.typeAnnotation, "typeAnnotation");
			break;
		case 40:
			fn(node.pattern, "pattern");
			fn(node.typeAnnotation, "typeAnnotation");
			break;
		case 42:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			if (undefined !== node.properties) _it_arr(fn, node.properties, "properties");
			break;
		case 43:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			_it_arr(fn, node.items, "items");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			break;
		case 44:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			fn(node.typeAnnotation, "typeAnnotation");
			break;
		case 45:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.typeAnnotation, "typeAnnotation");
			break;
		case 46:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			_it_arr(fn, node.properties, "properties");
			break;
		case 47:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			_it_arr(fn, node.members, "members");
			break;
		case 48:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (undefined !== node.value) fn(node.value, "value");
			break;
		case 49:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			_it_arr(fn, node.items, "items");
			if (undefined !== node.value) fn(node.value, "value");
			break;
		case 50:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			_it_arr(fn, node.properties, "properties");
			if (undefined !== node.value) fn(node.value, "value");
			break;
		case 51:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			if (undefined !== node.typeBounds) _it_arr(fn, node.typeBounds, "typeBounds");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			_it_arr(fn, node.body, "body");
			break;
		case 52:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			break;
		case 53:
			if ("pub" in node) fn(node.pub, "pub");
			fn(node.id, "id");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			_it_arr(fn, node.typeBounds, "typeBounds");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			break;
		case 54:
			if ("pub" in node) fn(node.pub, "pub");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			if (undefined !== node.trait) fn(node.trait, "trait");
			fn(node.typeTarget, "typeTarget");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			_it_arr(fn, node.body, "body");
			break;
		case 55:
			if ("pub" in node) fn(node.pub, "pub");
			if (undefined !== node.generics) _it_arr(fn, node.generics, "generics");
			fn(node.trait, "trait");
			fn(node.typeTarget, "typeTarget");
			if (undefined !== node.whereBounds) _it_arr(fn, node.whereBounds, "whereBounds");
			break;
		case 56:
			fn(node.typeTarget, "typeTarget");
			if (undefined !== node.typeExpression) fn(node.typeExpression, "typeExpression");
			break;
		case 57:
		case 117:
			fn(node.typeCallee, "typeCallee");
			_it_arr(fn, node.typeArguments, "typeArguments");
			break;
		case 59:
			fn(node.expression, "expression");
			fn(node.typeExpression, "typeExpression");
			break;
		case 61:
			if (undefined !== node.label) fn(node.label, "label");
			if (undefined !== node.expression) fn(node.expression, "expression");
			break;
		case 62:
			if (undefined !== node.label) fn(node.label, "label");
			break;
		case 64:
			fn(node.callee, "callee");
			if (undefined !== node.method) fn(node.method, "method");
			if (undefined !== node.typeArguments) _it_arr(fn, node.typeArguments, "typeArguments");
			_it_arr(fn, node.arguments, "arguments");
			break;
		case 65:
			fn(node.expression, "expression");
			fn(node.property, "property");
			break;
		case 66:
		case 67:
		case 68:
		case 69:
		case 70:
		case 93:
		case 98:
		case 99:
		case 100:
		case 101:
			fn(node.expression, "expression");
			break;
		case 71:
		case 72:
		case 73:
		case 75:
		case 76:
		case 77:
			fn(node.left, "left");
			fn(node.right, "right");
			break;
		case 78:
			fn(node.pattern, "pattern");
			fn(node.expression, "expression");
			break;
		case 79:
			_it_arr(fn, node.parameters, "parameters");
			if (undefined !== node.returnType) fn(node.returnType, "returnType");
			fn(node.expression, "expression");
			break;
		case 80:
			fn(node.pattern, "pattern");
			if (undefined !== node.typeAnnotation) fn(node.typeAnnotation, "typeAnnotation");
			break;
		case 81:
		case 82:
		case 85:
			if (undefined !== node.label) fn(node.label, "label");
			_it_arr(fn, node.body, "body");
			break;
		case 83:
			if (undefined !== node.label) fn(node.label, "label");
			fn(node.condition, "condition");
			_it_arr(fn, node.body, "body");
			break;
		case 84:
			if (undefined !== node.label) fn(node.label, "label");
			fn(node.pattern, "pattern");
			fn(node.expression, "expression");
			_it_arr(fn, node.body, "body");
			break;
		case 86:
			if (undefined !== node.label) fn(node.label, "label");
			fn(node.condition, "condition");
			_it_arr(fn, node.body, "body");
			if (undefined !== node.else) fn(node.else, "else");
			break;
		case 87:
			if (undefined !== node.label) fn(node.label, "label");
			fn(node.expression, "expression");
			_it_arr(fn, node.cases, "cases");
			break;
		case 88:
			fn(node.pattern, "pattern");
			if (undefined !== node.condition) fn(node.condition, "condition");
			fn(node.expression, "expression");
			break;
		case 89:
		case 115:
			if (undefined !== node.lower) fn(node.lower, "lower");
			if (undefined !== node.upper) fn(node.upper, "upper");
			break;
		case 90:
		case 107:
			fn(node.struct, "struct");
			_it_arr(fn, node.properties, "properties");
			break;
		case 91:
			fn(node.key, "key");
			fn(node.value, "value");
			break;
		case 92:
			fn(node.value, "value");
			break;
		case 95:
		case 96:
		case 111:
		case 136:
			_it_arr(fn, node.items, "items");
			break;
		case 97:
			fn(node.initExpression, "initExpression");
			fn(node.sizeExpression, "sizeExpression");
			break;
		case 102:
			_it_arr(fn, node.patterns, "patterns");
			break;
		case 103:
		case 112:
		case 113:
		case 114:
			fn(node.pattern, "pattern");
			break;
		case 106:
			fn(node.id, "id");
			if (undefined !== node.pattern) fn(node.pattern, "pattern");
			break;
		case 108:
			fn(node.key, "key");
			fn(node.pattern, "pattern");
			break;
		case 109:
			fn(node.id, "id");
			break;
		case 110:
			if (undefined !== node.struct) fn(node.struct, "struct");
			_it_arr(fn, node.items, "items");
			break;
		case 118:
			fn(node.target, "target");
			fn(node.typeExpression, "typeExpression");
			break;
		case 119:
			fn(node.typeTarget, "typeTarget");
			_it_arr(fn, node.typeBounds, "typeBounds");
			break;
		case 125:
			fn(node.id, "id");
			if (undefined !== node.typeBounds) _it_arr(fn, node.typeBounds, "typeBounds");
			if (undefined !== node.typeDefault) fn(node.typeDefault, "typeDefault");
			break;
		case 126:
			fn(node.id, "id");
			fn(node.typeAnnotation, "typeAnnotation");
			if (undefined !== node.typeDefault) fn(node.typeDefault, "typeDefault");
			break;
		case 127:
			fn(node.id, "id");
			if (undefined !== node.ltBounds) _it_arr(fn, node.ltBounds, "ltBounds");
			break;
		case 128:
			if (undefined !== node.ltParameters) _it_arr(fn, node.ltParameters, "ltParameters");
			fn(node.typeTarget, "typeTarget");
			_it_arr(fn, node.typeBounds, "typeBounds");
			break;
		case 129:
			fn(node.ltTarget, "ltTarget");
			_it_arr(fn, node.ltBounds, "ltBounds");
			break;
		case 130:
			if (undefined !== node.ltParameters) _it_arr(fn, node.ltParameters, "ltParameters");
			fn(node.typeExpression, "typeExpression");
			break;
		case 131:
		case 132:
			_it_arr(fn, node.typeBounds, "typeBounds");
			break;
		case 133:
			if ("extern" in node) fn(node.extern, "extern");
			if (undefined !== node.ltParameters) _it_arr(fn, node.ltParameters, "ltParameters");
			_it_arr(fn, node.parameters, "parameters");
			if (undefined !== node.returnType) fn(node.returnType, "returnType");
			break;
		case 134:
			if (undefined !== node.id) fn(node.id, "id");
			fn(node.typeAnnotation, "typeAnnotation");
			break;
		case 135:
			fn(node.callee, "callee");
			_it_arr(fn, node.parameters, "parameters");
			if (undefined !== node.returnType) fn(node.returnType, "returnType");
			break;
		case 137:
			fn(node.typeExpression, "typeExpression");
			fn(node.sizeExpression, "sizeExpression");
			break;
		case 138:
		case 140:
		case 141:
		case 142:
			fn(node.typeExpression, "typeExpression");
			break;
		case 139:
			if (undefined !== node.lt) fn(node.lt, "lt");
			fn(node.typeExpression, "typeExpression");
			break;
		default:
			exit.never(node);
	}
}
// </generated>

/** Iterate childNodes */
export function each_childNode(node: Node, callback: AstItEachFn) {
	iterate_nodes(node, function f(child, key, index) {
		callback(child, node, key, index);
	});
}

/** Iterate nodes in tree, bottom up, excluding itself */
export function each_node(ast: Node, callback: AstItEachFn) {
	let i = 0;
	const ancestry: Node[] = [ast];
	iterate_nodes(ast, function f(child, key, index) {
		iterate_nodes((ancestry[++i] = child), f);
		callback(child, ancestry[--i], key, index);
	});
}

export function getNodeChildren<T extends Node>(node: T): NodeChildTypes<T>[] {
	const children: Node[] = [];
	iterate_nodes(node, function f(child) {
		insertNode(children, child);
	});
	return children as any;
}

function getChildParentMap(node: Node): WeakMap<Node, Node> {
	const ChildToParentMap = new WeakMap<Node, Node>();
	each_node(node.loc.src, function f(child, parent) {
		ChildToParentMap.set(child, parent);
	});
	return ChildToParentMap;
}

export function hasChildren(node: Node) {
	try {
		iterate_nodes(node, () => {
			throw 0;
		});
	} catch (e) {
		if (0 === e) return true;
		throw e;
	}
	return false;
}

function findNodeParentFromProgram(node: Node) {
	let r: Node | undefined = undefined;
	(function f(n) {
		if (n === node) r = n;
		else r ?? iterate_nodes(n, f);
	})(node.loc.src);
	return r;
}
/** Returns every node between origin down to and including target */
function getAncestryNodesDownTo(origin: Node, target: Node) {
	let j = 0;
	const a: Node[] = [];

	try {
		(function r(parent) {
			if (parent === target) throw 0;
			iterate_nodes(parent, (n) => ((a[j++] = n), r(n), j--));
		})(origin);
	} catch (e) {
		if (0 === e) return (a.length = j), a;
		throw e;
	}
	exit("Could not find AstPath", { origin, target });
}

/** returns the keys needed to access target from origin */
export function getAstPath(parent: Node, target: Node): (string | number)[] {
	let j = 0;
	const p: (string | number)[] = [];
	try {
		(function r(parent) {
			if (parent === target) throw 0;
			iterate_nodes(parent, (n, k, i) => {
				if (undefined === i) (p[j++] = k), r(n), j--;
				else (p[j++] = k), (p[j++] = i), r(n), (j -= 2);
			});
		})(parent);
	} catch (e) {
		if (0 === e) return (p.length = j), p;
		throw e;
	}
	exit(`Could not find target ${target.type} in parent ${parent.type}`, { parent, target });
}

/** returns the keys needed to access a child from its parent */
export function getOwnChildAstPath(parent: Node, child: Node): [string] | [string, number | "self"] {
	let r;
	try {
		iterate_nodes(parent, (n, ...p) => {
			if (n === child) {
				r = p;
				throw 0;
			}
		});
	} catch (e) {
		if (0 === e) return r;
		throw e;
	}
	exit(`Could not find child ${child.type} in parent ${parent.type}`, { parent, child });
}

function EXPENSIVE_getParent(target: Node, origin: Node = target.loc.src) {
	let p: Node | undefined;
	try {
		(function r(child: Node, parent?: Node) {
			if (child === target) throw ((p = parent), 0);
			iterate_nodes(child, (nth_child) => r(nth_child, child));
		})(origin);
	} catch (e) {
		if (0 === e) return p;
		throw e;
	}
	exit("Could not find AstPath" /* { origin, target } */);
}

function some_childNode(node: Node, test: AstItFn<boolean>) {
	try {
		iterate_nodes(node, (...args) => {
			if (test(...args)) throw 0;
		});
	} catch (e) {
		if (0 === e) return true;
		throw e;
	}
	return false;
}

export function reassignNodeProperty(value: any, parent: Node, key: string, index?: number | "self" | undefined) {
	if (typeof index === "undefined") parent[key] = value;
	else parent[key][index] = value;
}

function _hasSelf(value: any[]): value is FunctionDeclarationParameters & { self: {} } {
	return "self" in value && is_defined((value as FunctionDeclarationParameters).self);
}

/** Iterate Object.keys to find childNodes, as opposed to just getting known properties of that NodeType */
export function getActualNodeChildren(node: Node) {
	const children: Node[] = [];
	for (var key in node) {
		var value = node[key];
		if (is_array(value)) {
			if (_hasSelf(value)) insertNode(children, value.self);
			insertNodes(children, value);
		} else if (is_Node(value)) {
			insertNode(children, value);
		}
	}
	return children;
}

export function countActualNodeChildren(node: Node) {
	var l = 0;
	for (var key in node) {
		var value = node[key];
		if (is_array(value)) {
			if (_hasSelf(value)) l += 1;
			l += value.length;
		} else if (is_Node(value)) {
			l += 1;
		}
	}
	return l;
}
