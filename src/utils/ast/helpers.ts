import {
	ArrayLikeNode,
	AttributeOrDocComment,
	CallExpression,
	ConditionExpression,
	DeclarationNode,
	Delimited,
	DelimKind,
	FunctionDeclaration,
	FunctionLikeNode,
	FunctionSelfParameterDeclaration,
	Identifier,
	Literal,
	MacroInvocation,
	MaybeExpressionBody,
	Node,
	NodeType,
	NodeWithBody,
	NodeWithCondition,
	NodeWithTypeBounds,
	ObjectNode,
	ParenthesizedNode,
	ScrutineeExpression,
} from "../../parser/nodes";
import {
	assert,
	exit,
	has_key,
	has_key_defined,
	has_key_non_empty_array,
	has_key_undefined,
	isEmpty,
	is_defined,
	is_non_empty_array,
	last_of,
} from "../common";
import { NodeChildTypes } from "./iterator";
import {
	isInner,
	isOuter,
	is_DeclarationNode,
	is_FunctionDeclaration,
	is_FunctionLikeNode,
	is_Identifier,
	is_LetScrutinee,
	is_LiteralRawStringLike,
	is_LogicalExpression,
	is_NodeWithCondition,
	is_NodeWithTypeBounds,
	is_PathNode,
} from "./nodetype";

export type WithAttributes<T extends Node> = T & { attributes: AttributeOrDocComment[] };
export type ParameterOf<T extends FunctionLikeNode> =
	| T["parameters"]
	| (T extends FunctionDeclaration ? FunctionSelfParameterDeclaration : never);

export function hasAttributes<T extends Node>(node: T): node is WithAttributes<T> {
	__DEV__: assert.isNode(node), "attributes" in node && assert(is_non_empty_array(node.attributes!));
	return has_key(node as Node, "attributes");
}

export function hasOuterAttributes<T extends Node>(node: T): node is WithAttributes<T> {
	return hasAttributes(node) && isOuter(node.attributes[0]);
}

export function hasInnerAttributes<T extends Node>(node: T): node is WithAttributes<T> {
	return hasAttributes(node) && isInner(last_of(node.attributes));
}

export function hasSuffix(node: Node): node is Literal & { suffix: {} } {
	return has_key(node, "suffix");
}

export function hasCondition(node: Node): node is NodeWithCondition & { condition: {} } {
	return is_NodeWithCondition(node) && is_defined(node.condition);
}

export function hasLetScrutineeCondition(node: Node): node is NodeWithCondition & { condition: {} } {
	return hasCondition(node) && r(node.condition);
	function r(node: ConditionExpression) {
		return is_LogicalExpression(node) ? r(node.left) || r(node.right) : is_LetScrutinee(node);
	}
}

export function hasSemiNoProperties(node: Node): node is ObjectNode & { properties: undefined } {
	return has_key_undefined(node, "properties");
}
export function hasSemiNoBody(node: Node): node is NodeWithBody & { body: undefined } {
	return has_key_undefined(node, "body");
}

export function hasProperties(node: Node): node is ObjectNode & { properties: {} } {
	return has_key_non_empty_array(node, "properties");
}
export function hasItems(node: Node): node is ArrayLikeNode & { items: {} } {
	return has_key_non_empty_array(node, "items");
}
export function hasBody(node: Node): node is NodeWithBody & { body: {} } {
	return has_key_non_empty_array(node, "body");
}

export function hasGenerics(node: Node): node is DeclarationNode & { generics: {} } {
	return is_DeclarationNode(node) && has_key_non_empty_array(node, "generics");
}
export function hasSelfParameter(
	node: FunctionLikeNode
): node is FunctionLikeNode & { parameters: { self: FunctionSelfParameterDeclaration } } {
	return is_FunctionDeclaration(node) && is_defined(node.parameters.self);
}
export function hasParameters(node: FunctionLikeNode): boolean {
	return is_FunctionLikeNode(node) && (node.parameters.length > 0 || hasSelfParameter(node));
}

export function hasMethod(node: CallExpression): node is CallExpression & { method: Identifier } {
	return is_defined(node.method);
}

export function hasTypeBounds(node: Node): node is NodeWithTypeBounds & { typeBounds: {} } {
	return is_NodeWithTypeBounds(node) && is_defined(node.typeBounds);
}
type NodeWithMaybeExpressionProperty = Extract<Node, MaybeExpressionBody>;
export function hasExpression(node: NodeWithMaybeExpressionProperty): node is NodeWithMaybeExpressionProperty & { expression: {} } {
	return has_key_defined(node, "expression");
}

export function getParameters<T extends FunctionLikeNode>(node: T) {
	return (hasSelfParameter(node) ? [node.parameters.self, ...node.parameters] : node.parameters) as
		| [...T["parameters"]]
		| (T extends FunctionDeclaration ? [FunctionSelfParameterDeclaration, ...T["parameters"]] : never);
}
export function getFirstParameter<T extends FunctionLikeNode>(node: T): ParameterOf<T> | undefined {
	return (hasSelfParameter(node) ? node.parameters.self : node.parameters.length > 0 ? node.parameters[0] : undefined) as any;
}
export function getLastParameter<T extends FunctionLikeNode>(node: T): ParameterOf<T> | undefined {
	return (isEmpty(node.parameters) ? (hasSelfParameter(node) ? node.parameters.self : undefined) : last_of(node.parameters)) as any;
}

export function getBodyOrCases<T extends { body: any } | { cases: any }>(
	node: T
): T extends { body: infer U } ? U : T extends { cases: infer U } ? U : never {
	__DEV__: assert("body" in node || "cases" in node, "", node);
	return "body" in node ? node.body : node.cases;
}

export function getMacroName(node: MacroInvocation) {
	const callee = node.callee;
	if (is_Identifier(callee)) return callee.name;
	if (is_PathNode(callee)) return callee.segment.name;
	exit("Expected Identifier | PathNode", node);
}

export function countRawLiteralHashtags(node: Literal) {
	return is_LiteralRawStringLike(node) ? node.loc.len() - (1 + node.value.lastIndexOf('"')) : 0;
}

export type DelimChars = { left: string; right: string };

// prettier-ignore
export function getDelimChars(node: Delimited): DelimChars {
	switch (node.dk) {
		case DelimKind["||"]: return { left: "|", right: "|" };
		case DelimKind["()"]: return { left: "(", right: ")" };
		case DelimKind["[]"]: return { left: "[", right: "]" };
		case DelimKind["{}"]: return { left: "{", right: "}" };
		case DelimKind["<>"]: return { left: "<", right: ">" };
	}
	exit.never(node);
}

export function getLeftMostCondition(node: ConditionExpression): ScrutineeExpression {
	for (var target = node; is_LogicalExpression(target); target = target.left);
	return target as any;
}

// prettier-ignore
export function getParenthesizedNodeContent<T extends ParenthesizedNode>(node: T): NodeChildTypes<T> {
	switch (node.nodeType) {
		case NodeType.ParenthesizedExpression: return node.expression as any;
		case NodeType.ParenthesizedPattern: return node.pattern as any;
		case NodeType.TypeParenthesized: return node.typeExpression as any;
	}
	exit.never(node);
}
