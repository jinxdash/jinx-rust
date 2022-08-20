import {
	AmbientImport,
	AndExpression,
	AnonymousImport,
	ArrayLikeNode,
	ArrayLiteral,
	ArrayOrTupleLiteral,
	ArrayPattern,
	Attribute,
	AttributeOrComment,
	AttributeOrDocComment,
	AutoTraitDeclaration,
	AwaitExpression,
	BaseNode,
	BlockExpression,
	BoxExpression,
	BoxPattern,
	BreakExpression,
	CallExpression,
	ClosureFunctionExpression,
	ClosureFunctionParameterDeclaration,
	Comment,
	CommentOrDocComment,
	ComparisonExpression,
	ConditionExpression,
	ConstTypeParameterDeclaration,
	ConstVariableDeclaration,
	ContinueExpression,
	DeclarationNode,
	DelimGroup,
	DelimitedSequence,
	DelimKind,
	DereferenceExpression,
	DestructuredImport,
	DocCommentAttribute,
	EnumDeclaration,
	EnumDeclarationMember,
	EnumMemberDeclaration,
	EnumMemberStructDeclaration,
	EnumMemberTupleDeclaration,
	ExpressionAsTypeCast,
	ExpressionNamespaceTarget,
	ExpressionNamespaceTargetNoSelector,
	ExpressionNode,
	ExpressionPath,
	ExpressionStatement,
	ExpressionTypeCast,
	ExpressionTypeSelector,
	ExpressionWithBody,
	ExpressionWithBodyOrCases,
	ExternBlockDeclaration,
	ExternCrateStatement,
	ExternSpecifier,
	FlowControlExpression,
	ForInBlockExpression,
	FunctionDeclaration,
	FunctionLikeNode,
	FunctionNode,
	FunctionParameterDeclaration,
	FunctionParameterNode,
	FunctionSelfParameterDeclaration,
	FunctionSpread,
	GenericLtParameterDeclaration,
	GenericParameterDeclaration,
	GenericTypeParameterDeclaration,
	Identifier,
	IdentifierOrIndex,
	IdentifierOrItemPath,
	IfBlockExpression,
	ImplDeclaration,
	ImplDeclarationNode,
	ImplicitReturnAbleNode,
	ImportNode,
	Index,
	ItemPath,
	LbIdentifier,
	LeftRightExpression,
	LetScrutinee,
	LetVariableDeclaration,
	Lifetime,
	Literal,
	LiteralKind,
	LogicalExpression,
	LoopBlockExpression,
	LtElided,
	LtIdentifier,
	LtStatic,
	MacroDeclaration,
	MacroGroup,
	MacroInlineRuleDeclaration,
	MacroInvocation,
	MacroParameterDeclaration,
	MacroRuleDeclaration,
	MacroRulesDeclaration,
	MatchExpression,
	MatchExpressionCase,
	MaybeAsyncNode,
	MaybeExternNode,
	MaybeMoveNode,
	MaybePubNode,
	MaybeStaticNode,
	MaybeUnsafeNode,
	McIdentifier,
	MemberExpression,
	MinusExpression,
	MinusPattern,
	MissingNode,
	ModuleDeclaration,
	NamedImport,
	NegativeImplDeclaration,
	Node,
	NodeType,
	NodeWithBody,
	NodeWithBodyNoBody,
	NodeWithBodyOrCases,
	NodeWithCondition,
	NodeWithMaybePatternNoUnionBody,
	NodeWithSegments,
	NodeWithTypeBounds,
	NotExpression,
	NTMap,
	ObjectNode,
	OperationExpression,
	OrExpression,
	ParenthesizedExpression,
	ParenthesizedNode,
	ParenthesizedPattern,
	PathNode,
	PatternNode,
	PatternNoUnion,
	PatternNoUnionNoRange,
	PatternVariableDeclaration,
	PostfixExpression,
	PRCD,
	Program,
	PubSpecifier,
	PunctuationToken,
	RangeLiteral,
	RangeNode,
	RangePattern,
	RangePatternBound,
	RawReferenceExpression,
	ReassignmentExpression,
	ReassignmentNode,
	ReassignmentOperationExpression,
	ReferenceExpression,
	ReferencePattern,
	RestPattern,
	ReturnExpression,
	Shebang,
	SizedArrayLiteral,
	Snippet,
	SourceFile,
	StatementNode,
	StaticVariableDeclaration,
	StructDeclaration,
	StructLiteral,
	StructLiteralProperty,
	StructLiteralPropertyShorthand,
	StructLiteralPropertySpread,
	StructLiteralRestUnassigned,
	StructPattern,
	StructPatternProperty,
	StructPatternPropertyDestructured,
	StructPatternPropertyShorthand,
	StructProperty,
	StructPropertyDeclaration,
	TK,
	TraitAliasDeclaration,
	TraitDeclaration,
	TraitDeclarationNode,
	TryBlockExpression,
	TupleLiteral,
	TupleNode,
	TuplePattern,
	TupleStructDeclaration,
	TupleStructItemDeclaration,
	TypeAliasDeclaration,
	TypeBound,
	TypeBoundsStandaloneNode,
	TypeCall,
	TypeCallNamedArgument,
	TypeCallNamedBound,
	TypeDereferenceConst,
	TypeDereferenceMut,
	TypeDynBounds,
	TypeFnPointer,
	TypeFnPointerParameter,
	TypeFunction,
	TypeFunctionNode,
	TypeImplBounds,
	TypeInferred,
	TypeNamespaceTarget,
	TypeNamespaceTargetNoSelector,
	TypeNever,
	TypeNode,
	TypeParenthesized,
	TypePath,
	TypeReference,
	TypeSizedArray,
	TypeSlice,
	TypeTraitBound,
	TypeTuple,
	UnaryExpression,
	UnaryPattern,
	UnaryType,
	UnassignedExpression,
	UnionDeclaration,
	UnionPattern,
	UnwrapExpression,
	UseStatement,
	VariableDeclarationNode,
	WhereBoundDeclaration,
	WhereLtBoundDeclaration,
	WhereTypeBoundDeclaration,
	WhileBlockExpression,
	WildcardPattern,
	YieldExpression,
} from "../../parser/nodes";
import { assert, AssertTypesEq, exit } from "../common";
import { hasAttributes } from "./helpers";

function nis<T extends NodeType>(node: BaseNode, nodeType: T): node is NTMap[T] {
	__DEV__: assert.isNode(node);
	return nodeType === node.nodeType;
}

export function nisAnyOf<T extends NodeType[]>(node: BaseNode, nodeTypes: T): node is NTMap[T[number]] {
	__DEV__: assert.isNode(node);
	return nodeTypes.includes(node.nodeType);
}

export function isTK(node: Node, tk: TK): node is PunctuationToken & { tk: TK } {
	return is_PunctuationToken(node) && node.tk === tk;
}

export function includesTK(node: DelimitedSequence<any>, tk: TK) {
	return node.segments.some((segment) => isTK(segment, tk));
}

// <generated>
export function is_MissingNode(node: Node): node is MissingNode {
	return nis(node, 0);
}
export function is_SourceFile(node: Node): node is SourceFile {
	return nis(node, 1);
}
export function is_Shebang(node: Node): node is Shebang {
	return nis(node, 2);
}
export function is_Program(node: Node): node is Program {
	return nis(node, 3);
}
export function is_Snippet(node: Node): node is Snippet {
	return nis(node, 4);
}
export function is_Comment(node: Node): node is Comment {
	return nis(node, 5);
}
export function is_Identifier(node: Node): node is Identifier {
	return nis(node, 6);
}
export function is_Index(node: Node): node is Index {
	return nis(node, 7);
}
export function is_LbIdentifier(node: Node): node is LbIdentifier {
	return nis(node, 8);
}
export function is_McIdentifier(node: Node): node is McIdentifier {
	return nis(node, 9);
}
export function is_Literal(node: Node): node is Literal {
	return nis(node, 10);
}
export function is_ItemPath(node: Node): node is ItemPath {
	return nis(node, 11);
}
export function is_PunctuationToken(node: Node): node is PunctuationToken {
	return nis(node, 12);
}
export function is_DelimGroup(node: Node): node is DelimGroup {
	return nis(node, 13);
}
export function is_Attribute(node: Node): node is Attribute {
	return nis(node, 14);
}
export function is_DocCommentAttribute(node: Node): node is DocCommentAttribute {
	return nis(node, 15);
}
export function is_MacroInvocation(node: Node): node is MacroInvocation {
	return nis(node, 16);
}
export function is_MacroRulesDeclaration(node: Node): node is MacroRulesDeclaration {
	return nis(node, 17);
}
export function is_MacroDeclaration(node: Node): node is MacroDeclaration {
	return nis(node, 18);
}
export function is_MacroRuleDeclaration(node: Node): node is MacroRuleDeclaration {
	return nis(node, 19);
}
export function is_MacroInlineRuleDeclaration(node: Node): node is MacroInlineRuleDeclaration {
	return nis(node, 20);
}
export function is_MacroGroup(node: Node): node is MacroGroup {
	return nis(node, 21);
}
export function is_MacroParameterDeclaration(node: Node): node is MacroParameterDeclaration {
	return nis(node, 22);
}
export function is_PubSpecifier(node: Node): node is PubSpecifier {
	return nis(node, 23);
}
export function is_ExternSpecifier(node: Node): node is ExternSpecifier {
	return nis(node, 24);
}
export function is_ExpressionStatement(node: Node): node is ExpressionStatement {
	return nis(node, 25);
}
export function is_UseStatement(node: Node): node is UseStatement {
	return nis(node, 26);
}
export function is_NamedImport(node: Node): node is NamedImport {
	return nis(node, 27);
}
export function is_AmbientImport(node: Node): node is AmbientImport {
	return nis(node, 28);
}
export function is_AnonymousImport(node: Node): node is AnonymousImport {
	return nis(node, 29);
}
export function is_DestructuredImport(node: Node): node is DestructuredImport {
	return nis(node, 30);
}
export function is_ExternCrateStatement(node: Node): node is ExternCrateStatement {
	return nis(node, 31);
}
export function is_ExternBlockDeclaration(node: Node): node is ExternBlockDeclaration {
	return nis(node, 32);
}
export function is_TypeAliasDeclaration(node: Node): node is TypeAliasDeclaration {
	return nis(node, 33);
}
export function is_ConstVariableDeclaration(node: Node): node is ConstVariableDeclaration {
	return nis(node, 34);
}
export function is_StaticVariableDeclaration(node: Node): node is StaticVariableDeclaration {
	return nis(node, 35);
}
export function is_LetVariableDeclaration(node: Node): node is LetVariableDeclaration {
	return nis(node, 36);
}
export function is_ModuleDeclaration(node: Node): node is ModuleDeclaration {
	return nis(node, 37);
}
export function is_FunctionDeclaration(node: Node): node is FunctionDeclaration {
	return nis(node, 38);
}
export function is_FunctionSelfParameterDeclaration(node: Node): node is FunctionSelfParameterDeclaration {
	return nis(node, 39);
}
export function is_FunctionParameterDeclaration(node: Node): node is FunctionParameterDeclaration {
	return nis(node, 40);
}
export function is_FunctionSpread(node: Node): node is FunctionSpread {
	return nis(node, 41);
}
export function is_StructDeclaration(node: Node): node is StructDeclaration {
	return nis(node, 42);
}
export function is_TupleStructDeclaration(node: Node): node is TupleStructDeclaration {
	return nis(node, 43);
}
export function is_StructPropertyDeclaration(node: Node): node is StructPropertyDeclaration {
	return nis(node, 44);
}
export function is_TupleStructItemDeclaration(node: Node): node is TupleStructItemDeclaration {
	return nis(node, 45);
}
export function is_UnionDeclaration(node: Node): node is UnionDeclaration {
	return nis(node, 46);
}
export function is_EnumDeclaration(node: Node): node is EnumDeclaration {
	return nis(node, 47);
}
export function is_EnumMemberDeclaration(node: Node): node is EnumMemberDeclaration {
	return nis(node, 48);
}
export function is_EnumMemberTupleDeclaration(node: Node): node is EnumMemberTupleDeclaration {
	return nis(node, 49);
}
export function is_EnumMemberStructDeclaration(node: Node): node is EnumMemberStructDeclaration {
	return nis(node, 50);
}
export function is_TraitDeclaration(node: Node): node is TraitDeclaration {
	return nis(node, 51);
}
export function is_AutoTraitDeclaration(node: Node): node is AutoTraitDeclaration {
	return nis(node, 52);
}
export function is_TraitAliasDeclaration(node: Node): node is TraitAliasDeclaration {
	return nis(node, 53);
}
export function is_ImplDeclaration(node: Node): node is ImplDeclaration {
	return nis(node, 54);
}
export function is_NegativeImplDeclaration(node: Node): node is NegativeImplDeclaration {
	return nis(node, 55);
}
export function is_ExpressionTypeSelector(node: Node): node is ExpressionTypeSelector {
	return nis(node, 56);
}
export function is_ExpressionTypeCast(node: Node): node is ExpressionTypeCast {
	return nis(node, 57);
}
export function is_ExpressionPath(node: Node): node is ExpressionPath {
	return nis(node, 58);
}
export function is_ExpressionAsTypeCast(node: Node): node is ExpressionAsTypeCast {
	return nis(node, 59);
}
export function is_ReturnExpression(node: Node): node is ReturnExpression {
	return nis(node, 60);
}
export function is_BreakExpression(node: Node): node is BreakExpression {
	return nis(node, 61);
}
export function is_ContinueExpression(node: Node): node is ContinueExpression {
	return nis(node, 62);
}
export function is_YieldExpression(node: Node): node is YieldExpression {
	return nis(node, 63);
}
export function is_CallExpression(node: Node): node is CallExpression {
	return nis(node, 64);
}
export function is_MemberExpression(node: Node): node is MemberExpression {
	return nis(node, 65);
}
export function is_AwaitExpression(node: Node): node is AwaitExpression {
	return nis(node, 66);
}
export function is_UnwrapExpression(node: Node): node is UnwrapExpression {
	return nis(node, 67);
}
export function is_ParenthesizedExpression(node: Node): node is ParenthesizedExpression {
	return nis(node, 68);
}
export function is_MinusExpression(node: Node): node is MinusExpression {
	return nis(node, 69);
}
export function is_NotExpression(node: Node): node is NotExpression {
	return nis(node, 70);
}
export function is_OrExpression(node: Node): node is OrExpression {
	return nis(node, 71);
}
export function is_AndExpression(node: Node): node is AndExpression {
	return nis(node, 72);
}
export function is_ReassignmentExpression(node: Node): node is ReassignmentExpression {
	return nis(node, 73);
}
export function is_UnassignedExpression(node: Node): node is UnassignedExpression {
	return nis(node, 74);
}
export function is_OperationExpression(node: Node): node is OperationExpression {
	return nis(node, 75);
}
export function is_ReassignmentOperationExpression(node: Node): node is ReassignmentOperationExpression {
	return nis(node, 76);
}
export function is_ComparisonExpression(node: Node): node is ComparisonExpression {
	return nis(node, 77);
}
export function is_LetScrutinee(node: Node): node is LetScrutinee {
	return nis(node, 78);
}
export function is_ClosureFunctionExpression(node: Node): node is ClosureFunctionExpression {
	return nis(node, 79);
}
export function is_ClosureFunctionParameterDeclaration(node: Node): node is ClosureFunctionParameterDeclaration {
	return nis(node, 80);
}
export function is_BlockExpression(node: Node): node is BlockExpression {
	return nis(node, 81);
}
export function is_LoopBlockExpression(node: Node): node is LoopBlockExpression {
	return nis(node, 82);
}
export function is_WhileBlockExpression(node: Node): node is WhileBlockExpression {
	return nis(node, 83);
}
export function is_ForInBlockExpression(node: Node): node is ForInBlockExpression {
	return nis(node, 84);
}
export function is_TryBlockExpression(node: Node): node is TryBlockExpression {
	return nis(node, 85);
}
export function is_IfBlockExpression(node: Node): node is IfBlockExpression {
	return nis(node, 86);
}
export function is_MatchExpression(node: Node): node is MatchExpression {
	return nis(node, 87);
}
export function is_MatchExpressionCase(node: Node): node is MatchExpressionCase {
	return nis(node, 88);
}
export function is_RangeLiteral(node: Node): node is RangeLiteral {
	return nis(node, 89);
}
export function is_StructLiteral(node: Node): node is StructLiteral {
	return nis(node, 90);
}
export function is_StructLiteralProperty(node: Node): node is StructLiteralProperty {
	return nis(node, 91);
}
export function is_StructLiteralPropertyShorthand(node: Node): node is StructLiteralPropertyShorthand {
	return nis(node, 92);
}
export function is_StructLiteralPropertySpread(node: Node): node is StructLiteralPropertySpread {
	return nis(node, 93);
}
export function is_StructLiteralRestUnassigned(node: Node): node is StructLiteralRestUnassigned {
	return nis(node, 94);
}
export function is_TupleLiteral(node: Node): node is TupleLiteral {
	return nis(node, 95);
}
export function is_ArrayLiteral(node: Node): node is ArrayLiteral {
	return nis(node, 96);
}
export function is_SizedArrayLiteral(node: Node): node is SizedArrayLiteral {
	return nis(node, 97);
}
export function is_ReferenceExpression(node: Node): node is ReferenceExpression {
	return nis(node, 98);
}
export function is_RawReferenceExpression(node: Node): node is RawReferenceExpression {
	return nis(node, 99);
}
export function is_DereferenceExpression(node: Node): node is DereferenceExpression {
	return nis(node, 100);
}
export function is_BoxExpression(node: Node): node is BoxExpression {
	return nis(node, 101);
}
export function is_UnionPattern(node: Node): node is UnionPattern {
	return nis(node, 102);
}
export function is_ParenthesizedPattern(node: Node): node is ParenthesizedPattern {
	return nis(node, 103);
}
export function is_RestPattern(node: Node): node is RestPattern {
	return nis(node, 104);
}
export function is_WildcardPattern(node: Node): node is WildcardPattern {
	return nis(node, 105);
}
export function is_PatternVariableDeclaration(node: Node): node is PatternVariableDeclaration {
	return nis(node, 106);
}
export function is_StructPattern(node: Node): node is StructPattern {
	return nis(node, 107);
}
export function is_StructPatternPropertyDestructured(node: Node): node is StructPatternPropertyDestructured {
	return nis(node, 108);
}
export function is_StructPatternPropertyShorthand(node: Node): node is StructPatternPropertyShorthand {
	return nis(node, 109);
}
export function is_TuplePattern(node: Node): node is TuplePattern {
	return nis(node, 110);
}
export function is_ArrayPattern(node: Node): node is ArrayPattern {
	return nis(node, 111);
}
export function is_ReferencePattern(node: Node): node is ReferencePattern {
	return nis(node, 112);
}
export function is_BoxPattern(node: Node): node is BoxPattern {
	return nis(node, 113);
}
export function is_MinusPattern(node: Node): node is MinusPattern {
	return nis(node, 114);
}
export function is_RangePattern(node: Node): node is RangePattern {
	return nis(node, 115);
}
export function is_TypePath(node: Node): node is TypePath {
	return nis(node, 116);
}
export function is_TypeCall(node: Node): node is TypeCall {
	return nis(node, 117);
}
export function is_TypeCallNamedArgument(node: Node): node is TypeCallNamedArgument {
	return nis(node, 118);
}
export function is_TypeCallNamedBound(node: Node): node is TypeCallNamedBound {
	return nis(node, 119);
}
export function is_LtIdentifier(node: Node): node is LtIdentifier {
	return nis(node, 120);
}
export function is_LtElided(node: Node): node is LtElided {
	return nis(node, 121);
}
export function is_LtStatic(node: Node): node is LtStatic {
	return nis(node, 122);
}
export function is_TypeNever(node: Node): node is TypeNever {
	return nis(node, 123);
}
export function is_TypeInferred(node: Node): node is TypeInferred {
	return nis(node, 124);
}
export function is_GenericTypeParameterDeclaration(node: Node): node is GenericTypeParameterDeclaration {
	return nis(node, 125);
}
export function is_ConstTypeParameterDeclaration(node: Node): node is ConstTypeParameterDeclaration {
	return nis(node, 126);
}
export function is_GenericLtParameterDeclaration(node: Node): node is GenericLtParameterDeclaration {
	return nis(node, 127);
}
export function is_WhereTypeBoundDeclaration(node: Node): node is WhereTypeBoundDeclaration {
	return nis(node, 128);
}
export function is_WhereLtBoundDeclaration(node: Node): node is WhereLtBoundDeclaration {
	return nis(node, 129);
}
export function is_TypeTraitBound(node: Node): node is TypeTraitBound {
	return nis(node, 130);
}
export function is_TypeDynBounds(node: Node): node is TypeDynBounds {
	return nis(node, 131);
}
export function is_TypeImplBounds(node: Node): node is TypeImplBounds {
	return nis(node, 132);
}
export function is_TypeFnPointer(node: Node): node is TypeFnPointer {
	return nis(node, 133);
}
export function is_TypeFnPointerParameter(node: Node): node is TypeFnPointerParameter {
	return nis(node, 134);
}
export function is_TypeFunction(node: Node): node is TypeFunction {
	return nis(node, 135);
}
export function is_TypeTuple(node: Node): node is TypeTuple {
	return nis(node, 136);
}
export function is_TypeSizedArray(node: Node): node is TypeSizedArray {
	return nis(node, 137);
}
export function is_TypeSlice(node: Node): node is TypeSlice {
	return nis(node, 138);
}
export function is_TypeReference(node: Node): node is TypeReference {
	return nis(node, 139);
}
export function is_TypeDereferenceConst(node: Node): node is TypeDereferenceConst {
	return nis(node, 140);
}
export function is_TypeDereferenceMut(node: Node): node is TypeDereferenceMut {
	return nis(node, 141);
}
export function is_TypeParenthesized(node: Node): node is TypeParenthesized {
	return nis(node, 142);
}
export function is_ImportNode(node: Node): node is ImportNode {
	// prettier-ignore
	AssertTypesEq<ImportNode, NamedImport | AmbientImport | AnonymousImport | DestructuredImport>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 27:
		case 28:
		case 29:
		case 30:
			return true;
		default:
			return false;
	}
}
export function is_StatementNode(node: Node): node is StatementNode {
	// prettier-ignore
	AssertTypesEq<StatementNode, ExpressionStatement | MacroRulesDeclaration | MacroDeclaration | UseStatement | ExternCrateStatement |
		ExternBlockDeclaration | ModuleDeclaration | TypeAliasDeclaration | LetVariableDeclaration | ConstVariableDeclaration |
		StaticVariableDeclaration | FunctionDeclaration | StructDeclaration | TupleStructDeclaration | TraitDeclaration |
		AutoTraitDeclaration | TraitAliasDeclaration | ImplDeclaration | NegativeImplDeclaration | EnumDeclaration | UnionDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 17:
		case 18:
		case 25:
		case 26:
		case 31:
		case 32:
		case 33:
		case 34:
		case 35:
		case 36:
		case 37:
		case 38:
		case 42:
		case 43:
		case 46:
		case 47:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
			return true;
		default:
			return false;
	}
}
export function is_ExpressionNode(node: Node): node is ExpressionNode {
	// prettier-ignore
	AssertTypesEq<ExpressionNode, MacroInvocation | Identifier | Literal | ExpressionPath | ExpressionTypeCast | ExpressionTypeSelector |
		ExpressionAsTypeCast | ReturnExpression | BreakExpression | ContinueExpression | YieldExpression | CallExpression |
		MemberExpression | AwaitExpression | UnwrapExpression | MinusExpression | NotExpression | ClosureFunctionExpression |
		BlockExpression | LoopBlockExpression | WhileBlockExpression | ForInBlockExpression | TryBlockExpression | IfBlockExpression |
		MatchExpression | OrExpression<ExpressionNode, ExpressionNode> | AndExpression<ExpressionNode, ExpressionNode> |
		ReassignmentExpression | OperationExpression | ReassignmentOperationExpression | ComparisonExpression | RangeLiteral |
		StructLiteral | TupleLiteral | ArrayLiteral | SizedArrayLiteral | ReferenceExpression | RawReferenceExpression |
		DereferenceExpression | BoxExpression | ParenthesizedExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 10:
		case 16:
		case 56:
		case 57:
		case 58:
		case 59:
		case 60:
		case 61:
		case 62:
		case 63:
		case 64:
		case 65:
		case 66:
		case 67:
		case 68:
		case 69:
		case 70:
		case 71:
		case 72:
		case 73:
		case 75:
		case 76:
		case 77:
		case 79:
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
		case 87:
		case 89:
		case 90:
		case 95:
		case 96:
		case 97:
		case 98:
		case 99:
		case 100:
		case 101:
			return true;
		default:
			return false;
	}
}
export function is_PatternNode(node: Node): node is PatternNode {
	// prettier-ignore
	AssertTypesEq<PatternNode, MacroInvocation | Identifier | Literal | ExpressionPath | ExpressionTypeCast | ExpressionTypeSelector |
		MinusPattern | RangePattern | BlockExpression | PatternVariableDeclaration | StructPattern | TuplePattern | ArrayPattern |
		RestPattern | WildcardPattern | ReferencePattern | BoxPattern | UnionPattern | ParenthesizedPattern>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 10:
		case 16:
		case 56:
		case 57:
		case 58:
		case 81:
		case 102:
		case 103:
		case 104:
		case 105:
		case 106:
		case 107:
		case 110:
		case 111:
		case 112:
		case 113:
		case 114:
		case 115:
			return true;
		default:
			return false;
	}
}
export function is_TypeNode(node: Node): node is TypeNode {
	// prettier-ignore
	AssertTypesEq<TypeNode, MacroInvocation | Identifier | TypePath | TypeCall | ExpressionTypeSelector | TypeNever | TypeInferred |
		TypeDynBounds | TypeImplBounds | TypeFnPointer | TypeFunction | TypeSizedArray | TypeSlice | TypeTuple | TypeReference |
		TypeDereferenceMut | TypeDereferenceConst | TypeParenthesized<TypeNode>>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 16:
		case 56:
		case 116:
		case 117:
		case 123:
		case 124:
		case 131:
		case 132:
		case 133:
		case 135:
		case 136:
		case 137:
		case 138:
		case 139:
		case 140:
		case 141:
		case 142:
			return true;
		default:
			return false;
	}
}
export function is_AttributeOrComment(node: Node): node is AttributeOrComment {
	// prettier-ignore
	AssertTypesEq<AttributeOrComment, Comment | Attribute | DocCommentAttribute>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 5:
		case 14:
		case 15:
			return true;
		default:
			return false;
	}
}
export function is_AttributeOrDocComment(node: Node): node is AttributeOrDocComment {
	// prettier-ignore
	AssertTypesEq<AttributeOrDocComment, Attribute | DocCommentAttribute>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 14:
		case 15:
			return true;
		default:
			return false;
	}
}
export function is_CommentOrDocComment(node: Node): node is CommentOrDocComment {
	// prettier-ignore
	AssertTypesEq<CommentOrDocComment, Comment | DocCommentAttribute>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 5:
		case 15:
			return true;
		default:
			return false;
	}
}
export function is_IdentifierOrIndex(node: Node): node is IdentifierOrIndex {
	// prettier-ignore
	AssertTypesEq<IdentifierOrIndex, Identifier | Index>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 7:
			return true;
		default:
			return false;
	}
}
export function is_IdentifierOrItemPath(node: Node): node is IdentifierOrItemPath {
	// prettier-ignore
	AssertTypesEq<IdentifierOrItemPath, Identifier | ItemPath>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 11:
			return true;
		default:
			return false;
	}
}
export function is_ExpressionNamespaceTargetNoSelector(node: Node): node is ExpressionNamespaceTargetNoSelector {
	// prettier-ignore
	AssertTypesEq<ExpressionNamespaceTargetNoSelector, Identifier | ExpressionPath<Identifier> | ExpressionTypeCast<Identifier>>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
			return true;
		case 58:
			return undefined === node.namespace || is_ExpressionNamespaceTargetNoSelector(node.namespace);
		case 57:
			return is_ExpressionNamespaceTargetNoSelector(node.typeCallee);
		default:
			return false;
	}
}
export function is_TypeNamespaceTargetNoSelector(node: Node): node is TypeNamespaceTargetNoSelector {
	// prettier-ignore
	AssertTypesEq<TypeNamespaceTargetNoSelector, Identifier | TypePath<Identifier> | TypeCall<Identifier> | TypeFunction<Identifier>>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
			return true;
		case 116:
			return undefined === node.namespace || is_TypeNamespaceTargetNoSelector(node.namespace);
		case 117:
			return is_TypeNamespaceTargetNoSelector(node.typeCallee);
		case 135:
			return is_TypeNamespaceTargetNoSelector(node.callee);
		default:
			return false;
	}
}
export function is_ExpressionNamespaceTarget(node: Node): node is ExpressionNamespaceTarget {
	// prettier-ignore
	AssertTypesEq<ExpressionNamespaceTarget, Identifier | ExpressionTypeSelector | ExpressionPath | ExpressionTypeCast>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 56:
		case 57:
		case 58:
			return true;
		default:
			return false;
	}
}
export function is_TypeNamespaceTarget(node: Node): node is TypeNamespaceTarget {
	// prettier-ignore
	AssertTypesEq<TypeNamespaceTarget, Identifier | ExpressionTypeSelector | TypePath | TypeCall | TypeFunction>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 56:
		case 116:
		case 117:
		case 135:
			return true;
		default:
			return false;
	}
}
export function is_PatternNoUnion(node: Node): node is PatternNoUnion {
	// prettier-ignore
	AssertTypesEq<PatternNoUnion, MacroInvocation | Identifier | Literal | ExpressionPath | ExpressionTypeCast | ExpressionTypeSelector |
		MinusPattern | RangePattern | BlockExpression | PatternVariableDeclaration | StructPattern | TuplePattern | ArrayPattern |
		RestPattern | WildcardPattern | ReferencePattern | BoxPattern | ParenthesizedPattern>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 10:
		case 16:
		case 56:
		case 57:
		case 58:
		case 81:
		case 103:
		case 104:
		case 105:
		case 106:
		case 107:
		case 110:
		case 111:
		case 112:
		case 113:
		case 114:
		case 115:
			return true;
		default:
			return false;
	}
}
export function is_PatternNoUnionNoRange(node: Node): node is PatternNoUnionNoRange {
	// prettier-ignore
	AssertTypesEq<PatternNoUnionNoRange, MacroInvocation | Identifier | Literal | ExpressionPath | ExpressionTypeCast |
		ExpressionTypeSelector | MinusPattern | BlockExpression | PatternVariableDeclaration | StructPattern | TuplePattern | ArrayPattern |
		RestPattern | WildcardPattern | ReferencePattern | BoxPattern | ParenthesizedPattern>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 10:
		case 16:
		case 56:
		case 57:
		case 58:
		case 81:
		case 103:
		case 104:
		case 105:
		case 106:
		case 107:
		case 110:
		case 111:
		case 112:
		case 113:
		case 114:
			return true;
		default:
			return false;
	}
}
export function is_MaybePubNode(node: Node): node is MaybePubNode {
	// prettier-ignore
	AssertTypesEq<MaybePubNode, MacroDeclaration | UseStatement | ExternCrateStatement | ExternBlockDeclaration | TypeAliasDeclaration |
		ConstVariableDeclaration | StaticVariableDeclaration | ModuleDeclaration | FunctionDeclaration | StructDeclaration |
		TupleStructDeclaration | StructPropertyDeclaration | TupleStructItemDeclaration | UnionDeclaration | EnumDeclaration |
		EnumMemberDeclaration | EnumMemberTupleDeclaration | EnumMemberStructDeclaration | TraitDeclaration | AutoTraitDeclaration |
		TraitAliasDeclaration | ImplDeclaration | NegativeImplDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 18:
		case 26:
		case 31:
		case 32:
		case 33:
		case 34:
		case 35:
		case 37:
		case 38:
		case 42:
		case 43:
		case 44:
		case 45:
		case 46:
		case 47:
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
			return true;
		default:
			return false;
	}
}
export function is_MaybeExternNode(node: Node): node is MaybeExternNode {
	// prettier-ignore
	AssertTypesEq<MaybeExternNode, FunctionDeclaration | TypeFnPointer>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 38:
		case 133:
			return true;
		default:
			return false;
	}
}
export function is_MaybeAsyncNode(node: Node): node is MaybeAsyncNode {
	// prettier-ignore
	AssertTypesEq<MaybeAsyncNode, FunctionDeclaration | ClosureFunctionExpression | BlockExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 38:
		case 79:
		case 81:
			return true;
		default:
			return false;
	}
}
export function is_MaybeMoveNode(node: Node): node is MaybeMoveNode {
	// prettier-ignore
	AssertTypesEq<MaybeMoveNode, ClosureFunctionExpression | BlockExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 79:
		case 81:
			return true;
		default:
			return false;
	}
}
export function is_MaybeUnsafeNode(node: Node): node is MaybeUnsafeNode {
	// prettier-ignore
	AssertTypesEq<MaybeUnsafeNode, ExternBlockDeclaration | ModuleDeclaration | FunctionDeclaration | TraitDeclaration |
		AutoTraitDeclaration | TraitAliasDeclaration | ImplDeclaration | BlockExpression | TypeFnPointer>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 32:
		case 37:
		case 38:
		case 51:
		case 52:
		case 53:
		case 54:
		case 81:
		case 133:
			return true;
		default:
			return false;
	}
}
export function is_MaybeStaticNode(node: Node): node is MaybeStaticNode {
	// prettier-ignore
	AssertTypesEq<MaybeStaticNode, ClosureFunctionExpression>();
	__DEV__: assert.isNode(node);
	return is_ClosureFunctionExpression(node);
}
export function is_PathNode(node: Node): node is PathNode {
	// prettier-ignore
	AssertTypesEq<PathNode, ItemPath | ExpressionPath | TypePath>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 11:
		case 58:
		case 116:
			return true;
		default:
			return false;
	}
}
export function is_RangeNode(node: Node): node is RangeNode {
	// prettier-ignore
	AssertTypesEq<RangeNode, RangeLiteral | RangePattern>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 89:
		case 115:
			return true;
		default:
			return false;
	}
}
export function is_FunctionNode(node: Node): node is FunctionNode {
	// prettier-ignore
	AssertTypesEq<FunctionNode, FunctionDeclaration | ClosureFunctionExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 38:
		case 79:
			return true;
		default:
			return false;
	}
}
export function is_TypeFunctionNode(node: Node): node is TypeFunctionNode {
	// prettier-ignore
	AssertTypesEq<TypeFunctionNode, TypeFunction | TypeFnPointer>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 133:
		case 135:
			return true;
		default:
			return false;
	}
}
export function is_ParenthesizedNode(node: Node): node is ParenthesizedNode {
	// prettier-ignore
	AssertTypesEq<ParenthesizedNode, ParenthesizedExpression | ParenthesizedPattern | TypeParenthesized>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 68:
		case 103:
		case 142:
			return true;
		default:
			return false;
	}
}
export function is_ObjectNode(node: Node): node is ObjectNode {
	// prettier-ignore
	AssertTypesEq<ObjectNode, StructDeclaration | UnionDeclaration | EnumMemberStructDeclaration | StructLiteral | StructPattern>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 42:
		case 46:
		case 50:
		case 90:
		case 107:
			return true;
		default:
			return false;
	}
}
export function is_ArrayLikeNode(node: Node): node is ArrayLikeNode {
	// prettier-ignore
	AssertTypesEq<ArrayLikeNode, TupleStructDeclaration | EnumMemberTupleDeclaration | TupleLiteral | ArrayLiteral | TuplePattern |
		ArrayPattern | TypeTuple>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 43:
		case 49:
		case 95:
		case 96:
		case 110:
		case 111:
		case 136:
			return true;
		default:
			return false;
	}
}
export function is_ArrayOrTupleLiteral(node: Node): node is ArrayOrTupleLiteral {
	// prettier-ignore
	AssertTypesEq<ArrayOrTupleLiteral, ArrayLiteral | TupleLiteral>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 95:
		case 96:
			return true;
		default:
			return false;
	}
}
export function is_TupleNode(node: Node): node is TupleNode {
	// prettier-ignore
	AssertTypesEq<TupleNode, TupleStructDeclaration | EnumMemberTupleDeclaration | TupleLiteral | TuplePattern | TypeTuple>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 43:
		case 49:
		case 95:
		case 110:
		case 136:
			return true;
		default:
			return false;
	}
}
export function is_DeclarationNode(node: Node): node is DeclarationNode {
	// prettier-ignore
	AssertTypesEq<DeclarationNode, TypeAliasDeclaration | FunctionDeclaration | StructDeclaration | TupleStructDeclaration |
		UnionDeclaration | EnumDeclaration | TraitDeclaration | TraitAliasDeclaration | ImplDeclaration | NegativeImplDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 33:
		case 38:
		case 42:
		case 43:
		case 46:
		case 47:
		case 51:
		case 53:
		case 54:
		case 55:
			return true;
		default:
			return false;
	}
}
export function is_TraitDeclarationNode(node: Node): node is TraitDeclarationNode {
	// prettier-ignore
	AssertTypesEq<TraitDeclarationNode, TraitDeclaration | AutoTraitDeclaration | TraitAliasDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 51:
		case 52:
		case 53:
			return true;
		default:
			return false;
	}
}
export function is_ImplDeclarationNode(node: Node): node is ImplDeclarationNode {
	// prettier-ignore
	AssertTypesEq<ImplDeclarationNode, ImplDeclaration | NegativeImplDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 54:
		case 55:
			return true;
		default:
			return false;
	}
}
export function is_NodeWithSegments(node: Node): node is NodeWithSegments {
	// prettier-ignore
	AssertTypesEq<NodeWithSegments, DelimGroup | Attribute | DocCommentAttribute | MacroInvocation | MacroGroup>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 13:
		case 14:
		case 15:
		case 16:
		case 21:
			return true;
		default:
			return false;
	}
}
export function is_NodeWithBody(node: Node): node is NodeWithBody {
	// prettier-ignore
	AssertTypesEq<NodeWithBody, Program | ExternBlockDeclaration | ModuleDeclaration | FunctionDeclaration | TraitDeclaration |
		ImplDeclaration | BlockExpression | LoopBlockExpression | WhileBlockExpression | ForInBlockExpression | TryBlockExpression |
		IfBlockExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 3:
		case 32:
		case 37:
		case 38:
		case 51:
		case 54:
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
			return true;
		default:
			return false;
	}
}
export function is_NodeWithBodyOrCases(node: Node): node is NodeWithBodyOrCases {
	// prettier-ignore
	AssertTypesEq<NodeWithBodyOrCases, Program | ExternBlockDeclaration | ModuleDeclaration | FunctionDeclaration | TraitDeclaration |
		ImplDeclaration | BlockExpression | LoopBlockExpression | WhileBlockExpression | ForInBlockExpression | TryBlockExpression |
		IfBlockExpression | MatchExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 3:
		case 32:
		case 37:
		case 38:
		case 51:
		case 54:
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
		case 87:
			return true;
		default:
			return false;
	}
}
export function is_NodeWithBodyNoBody(node: Node): node is NodeWithBodyNoBody {
	// prettier-ignore
	AssertTypesEq<NodeWithBodyNoBody, AutoTraitDeclaration | NegativeImplDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 52:
		case 55:
			return true;
		default:
			return false;
	}
}
export function is_NodeWithCondition(node: Node): node is NodeWithCondition {
	// prettier-ignore
	AssertTypesEq<NodeWithCondition, WhileBlockExpression | IfBlockExpression | MatchExpressionCase>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 83:
		case 86:
		case 88:
			return true;
		default:
			return false;
	}
}
export function is_ImplicitReturnAbleNode(node: Node): node is ImplicitReturnAbleNode {
	// prettier-ignore
	AssertTypesEq<ImplicitReturnAbleNode, FunctionDeclaration | BlockExpression | IfBlockExpression | MatchExpression |
		TryBlockExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 38:
		case 81:
		case 85:
		case 86:
		case 87:
			return true;
		default:
			return false;
	}
}
export function is_ExpressionWithBody(node: Node): node is ExpressionWithBody {
	// prettier-ignore
	AssertTypesEq<ExpressionWithBody, BlockExpression | LoopBlockExpression | WhileBlockExpression | ForInBlockExpression |
		TryBlockExpression | IfBlockExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
			return true;
		default:
			return false;
	}
}
export function is_ExpressionWithBodyOrCases(node: Node): node is ExpressionWithBodyOrCases {
	// prettier-ignore
	AssertTypesEq<ExpressionWithBodyOrCases, BlockExpression | LoopBlockExpression | WhileBlockExpression | ForInBlockExpression |
		TryBlockExpression | IfBlockExpression | MatchExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
		case 87:
			return true;
		default:
			return false;
	}
}
export function is_LogicalExpression(node: Node): node is LogicalExpression {
	// prettier-ignore
	AssertTypesEq<LogicalExpression, OrExpression | AndExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 71:
		case 72:
			return true;
		default:
			return false;
	}
}
export function is_LeftRightExpression(node: Node): node is LeftRightExpression {
	// prettier-ignore
	AssertTypesEq<LeftRightExpression, OrExpression | AndExpression | ReassignmentExpression | OperationExpression |
		ReassignmentOperationExpression | ComparisonExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 71:
		case 72:
		case 73:
		case 75:
		case 76:
		case 77:
			return true;
		default:
			return false;
	}
}
export function is_FlowControlExpression(node: Node): node is FlowControlExpression {
	// prettier-ignore
	AssertTypesEq<FlowControlExpression, ReturnExpression | ContinueExpression | BreakExpression | YieldExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 60:
		case 61:
		case 62:
		case 63:
			return true;
		default:
			return false;
	}
}
export function is_UnaryExpression(node: Node): node is UnaryExpression {
	// prettier-ignore
	AssertTypesEq<UnaryExpression, NotExpression | MinusExpression | ReferenceExpression | DereferenceExpression | RawReferenceExpression |
		BoxExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 69:
		case 70:
		case 98:
		case 99:
		case 100:
		case 101:
			return true;
		default:
			return false;
	}
}
export function is_UnaryPattern(node: Node): node is UnaryPattern {
	// prettier-ignore
	AssertTypesEq<UnaryPattern, ReferencePattern | BoxPattern | MinusPattern>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 112:
		case 113:
		case 114:
			return true;
		default:
			return false;
	}
}
export function is_UnaryType(node: Node): node is UnaryType {
	// prettier-ignore
	AssertTypesEq<UnaryType, TypeReference | TypeDereferenceMut | TypeDereferenceConst>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 139:
		case 140:
		case 141:
			return true;
		default:
			return false;
	}
}
export function is_PostfixExpression(node: Node): node is PostfixExpression {
	// prettier-ignore
	AssertTypesEq<PostfixExpression, UnwrapExpression | AwaitExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 66:
		case 67:
			return true;
		default:
			return false;
	}
}
export function is_VariableDeclarationNode(node: Node): node is VariableDeclarationNode {
	// prettier-ignore
	AssertTypesEq<VariableDeclarationNode, LetScrutinee | LetVariableDeclaration | ConstVariableDeclaration | StaticVariableDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 34:
		case 35:
		case 36:
		case 78:
			return true;
		default:
			return false;
	}
}
export function is_ReassignmentNode(node: Node): node is ReassignmentNode {
	// prettier-ignore
	AssertTypesEq<ReassignmentNode, ReassignmentExpression | ReassignmentOperationExpression>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 73:
		case 76:
			return true;
		default:
			return false;
	}
}
export function is_NodeWithTypeBounds(node: Node): node is NodeWithTypeBounds {
	// prettier-ignore
	AssertTypesEq<NodeWithTypeBounds, TypeAliasDeclaration | TraitDeclaration | TraitAliasDeclaration | TypeCallNamedBound |
		GenericTypeParameterDeclaration | WhereTypeBoundDeclaration | TypeDynBounds | TypeImplBounds>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 33:
		case 51:
		case 53:
		case 119:
		case 125:
		case 128:
		case 131:
		case 132:
			return true;
		default:
			return false;
	}
}
export function is_TypeBoundsStandaloneNode(node: Node): node is TypeBoundsStandaloneNode {
	// prettier-ignore
	AssertTypesEq<TypeBoundsStandaloneNode, TypeDynBounds | TypeImplBounds>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 131:
		case 132:
			return true;
		default:
			return false;
	}
}
export function is_FunctionLikeNode(node: Node): node is FunctionLikeNode {
	// prettier-ignore
	AssertTypesEq<FunctionLikeNode, FunctionDeclaration | ClosureFunctionExpression | TypeFnPointer | TypeFunction>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 38:
		case 79:
		case 133:
		case 135:
			return true;
		default:
			return false;
	}
}
export function is_FunctionParameterNode(node: Node): node is FunctionParameterNode {
	// prettier-ignore
	AssertTypesEq<FunctionParameterNode, FunctionSelfParameterDeclaration | FunctionParameterDeclaration |
		ClosureFunctionParameterDeclaration | TypeFnPointerParameter | MacroInvocation | Identifier | TypePath | TypeCall |
		ExpressionTypeSelector | TypeNever | TypeInferred | TypeDynBounds | TypeImplBounds | TypeFnPointer | TypeFunction | TypeSizedArray |
		TypeSlice | TypeTuple | TypeReference | TypeDereferenceMut | TypeDereferenceConst | TypeParenthesized<TypeNode> | FunctionSpread>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 16:
		case 39:
		case 40:
		case 41:
		case 56:
		case 80:
		case 116:
		case 117:
		case 123:
		case 124:
		case 131:
		case 132:
		case 133:
		case 134:
		case 135:
		case 136:
		case 137:
		case 138:
		case 139:
		case 140:
		case 141:
		case 142:
			return true;
		default:
			return false;
	}
}
export function is_NodeWithMaybePatternNoUnionBody(node: Node): node is NodeWithMaybePatternNoUnionBody {
	// prettier-ignore
	AssertTypesEq<NodeWithMaybePatternNoUnionBody, LetVariableDeclaration | ConstVariableDeclaration | StaticVariableDeclaration |
		FunctionParameterDeclaration | ClosureFunctionParameterDeclaration | PatternVariableDeclaration | ReferencePattern | BoxPattern |
		MinusPattern>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 34:
		case 35:
		case 36:
		case 40:
		case 80:
		case 106:
		case 112:
		case 113:
		case 114:
			return true;
		default:
			return false;
	}
}
export function is_EnumDeclarationMember(node: Node): node is EnumDeclarationMember {
	// prettier-ignore
	AssertTypesEq<EnumDeclarationMember, EnumMemberDeclaration | EnumMemberTupleDeclaration | EnumMemberStructDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 48:
		case 49:
		case 50:
			return true;
		default:
			return false;
	}
}
export function is_StructProperty(node: Node): node is StructProperty {
	// prettier-ignore
	AssertTypesEq<StructProperty, StructLiteralProperty | StructLiteralPropertyShorthand | StructLiteralPropertySpread |
		StructLiteralRestUnassigned>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 91:
		case 92:
		case 93:
		case 94:
			return true;
		default:
			return false;
	}
}
export function is_StructPatternProperty(node: Node): node is StructPatternProperty {
	// prettier-ignore
	AssertTypesEq<StructPatternProperty, StructPatternPropertyDestructured | StructPatternPropertyShorthand | RestPattern>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 104:
		case 108:
		case 109:
			return true;
		default:
			return false;
	}
}
export function is_RangePatternBound(node: Node): node is RangePatternBound {
	// prettier-ignore
	AssertTypesEq<RangePatternBound, BlockExpression | Identifier | ExpressionTypeSelector | ExpressionPath | ExpressionTypeCast |
		MinusPattern | Literal>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 6:
		case 10:
		case 56:
		case 57:
		case 58:
		case 81:
		case 114:
			return true;
		default:
			return false;
	}
}
export function is_Lifetime(node: Node): node is Lifetime {
	// prettier-ignore
	AssertTypesEq<Lifetime, LtIdentifier | LtElided | LtStatic>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 120:
		case 121:
		case 122:
			return true;
		default:
			return false;
	}
}
export function is_GenericParameterDeclaration(node: Node): node is GenericParameterDeclaration {
	// prettier-ignore
	AssertTypesEq<GenericParameterDeclaration, GenericTypeParameterDeclaration | ConstTypeParameterDeclaration |
		GenericLtParameterDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 125:
		case 126:
		case 127:
			return true;
		default:
			return false;
	}
}
export function is_WhereBoundDeclaration(node: Node): node is WhereBoundDeclaration {
	// prettier-ignore
	AssertTypesEq<WhereBoundDeclaration, WhereTypeBoundDeclaration | WhereLtBoundDeclaration>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 128:
		case 129:
			return true;
		default:
			return false;
	}
}
export function is_TypeBound(node: Node): node is TypeBound {
	// prettier-ignore
	AssertTypesEq<TypeBound, TypeTraitBound | LtIdentifier | LtElided | LtStatic | TypeParenthesized<TypeTraitBound>>();
	__DEV__: assert.isNode(node);
	switch (node.nodeType) {
		case 120:
		case 121:
		case 122:
		case 130:
		case 142:
			return true;
		default:
			return false;
	}
}
export function can_have_OuterAttributes(node: Node, parent: Node | undefined, stmt_expr_attributes: boolean): boolean {
	__DEV__: assert.isNode(node);
	if (undefined !== parent) {
		switch (parent.nodeType) {
			case 36:
				return stmt_expr_attributes && node === parent.expression;
			case 64:
				return parent.arguments.includes(node as any);
			case 90:
			case 107:
				return parent.properties.includes(node as any);
			case 95:
			case 96:
			case 110:
				return parent.items.includes(node as any);
		}
	}
	switch (node.nodeType) {
		case 3:
		case 4:
		case 17:
		case 18:
		case 25:
		case 26:
		case 31:
		case 32:
		case 33:
		case 34:
		case 35:
		case 36:
		case 37:
		case 38:
		case 39:
		case 40:
		case 41:
		case 42:
		case 43:
		case 44:
		case 45:
		case 46:
		case 47:
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 80:
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
		case 87:
		case 88:
		case 91:
		case 92:
		case 108:
		case 109:
		case 125:
		case 126:
		case 127:
		case 134:
			return true;
		default:
			return false;
	}
}
// </generated>

export function can_have_InnerAttributes(node: Node): node is NodeWithBodyOrCases {
	return is_NodeWithBodyOrCases(node) || is_NodeWithBodyNoBody(node);
}

export function can_have_Attributes(node: Node, parent: Node | undefined, stmt_expr_attributes: boolean): boolean {
	return can_have_InnerAttributes(node) || can_have_OuterAttributes(node, parent, stmt_expr_attributes);
}

export function getPrecedence(node: ExpressionNode | ConditionExpression, insideScrutinee: boolean): PRCD {
	switch (node.nodeType) {
		case NodeType.MacroInvocation:
		case NodeType.ExpressionPath:
		case NodeType.CallExpression:
		case NodeType.MemberExpression:
		case NodeType.AwaitExpression:
		case NodeType.UnwrapExpression:
		case NodeType.StructLiteral:
			return PRCD.Top;
		case NodeType.MinusExpression:
		case NodeType.NotExpression:
		case NodeType.ReferenceExpression:
		case NodeType.RawReferenceExpression:
		case NodeType.DereferenceExpression:
		case NodeType.BoxExpression:
			return PRCD.Unary;
		case NodeType.ExpressionAsTypeCast:
			return PRCD["as"];
		case NodeType.OperationExpression:
			switch (node.tk) {
				case TK["*"]:
				case TK["/"]:
				case TK["%"]:
					return PRCD["*/%"];
				case TK["+"]:
				case TK["-"]:
					return PRCD["+-"];
				case TK["<<"]:
				case TK[">>"]:
					return PRCD[">>"];
				case TK["&"]:
					return PRCD["&"];
				case TK["^"]:
					return PRCD["^"];
				case TK["|"]:
					return PRCD["|"];
			}
			exit.never(node);
		case NodeType.ComparisonExpression:
			return PRCD["=="];
		case NodeType.AndExpression:
			return insideScrutinee ? PRCD["Scrutinee &&"] : PRCD["&&"];
		case NodeType.OrExpression:
			return insideScrutinee ? PRCD["Scrutinee ||"] : PRCD["||"];
		case NodeType.RangeLiteral:
			return PRCD[".."];
		case NodeType.ReassignmentExpression:
		case NodeType.ReassignmentOperationExpression:
			return PRCD["="];
		default:
			return insideScrutinee ? PRCD.ScrutineeDefault : PRCD.Default;
	}
}

export function is_MacroInvocation_BlockLike(node: Node): node is MacroInvocation {
	return is_MacroInvocation(node) && node.segments.dk === DelimKind["{}"];
}
export function is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation(node: Node) {
	return is_ExpressionWithBodyOrCases(node) || is_MacroInvocation_BlockLike(node);
}

export function isInner(node: AttributeOrDocComment): node is AttributeOrDocComment & { inner: true } {
	return node.inner;
}

export function isOuter(node: AttributeOrDocComment): node is AttributeOrDocComment & { inner: false } {
	return !node.inner;
}

export function is_ElseBlock(node: Node, parent: Node): parent is IfBlockExpression {
	return is_IfBlockExpression(parent) && parent.else === node;
}

export function is_CaseBlock(node: Node, parent: Node): parent is MatchExpressionCase {
	return is_MatchExpressionCase(parent) && parent.expression === node;
}

export function is_ClosureBlock(node: Node, parent: Node): parent is ClosureFunctionExpression {
	return is_ClosureFunctionExpression(parent) && parent.expression === node;
}

export function is_FlowControlMaybeValueExpression(node: Node): node is Exclude<FlowControlExpression, ContinueExpression> {
	return is_FlowControlExpression(node) && !is_ContinueExpression(node);
}

export function is_MacroRule(node: Node): node is MacroRuleDeclaration | MacroInlineRuleDeclaration {
	return is_MacroRuleDeclaration(node) || is_MacroInlineRuleDeclaration(node);
}

export function is_LiteralStringLike(
	node: Node
): node is Literal & { kind: LiteralKind.String | LiteralKind.rString | LiteralKind.bString | LiteralKind.brString } {
	switch (is_Literal(node) ? node.kind : LiteralKind.False) {
		case LiteralKind.String:
		case LiteralKind.rString:
		case LiteralKind.bString:
		case LiteralKind.brString:
			return true;
		default:
			return false;
	}
}

export function is_LiteralRawStringLike(node: Node): node is Literal & { kind: LiteralKind.rString | LiteralKind.brString } {
	switch (is_Literal(node) ? node.kind : LiteralKind.False) {
		case LiteralKind.rString:
		case LiteralKind.brString:
			return true;
		default:
			return false;
	}
}

export function is_LiteralNumberLike(
	node: Node
): node is Literal & { kind: LiteralKind.Integer | LiteralKind.Hex | LiteralKind.Octal | LiteralKind.Binary | LiteralKind.Float } {
	switch (is_Literal(node) ? node.kind : LiteralKind.False) {
		case LiteralKind.Integer:
		case LiteralKind.Hex:
		case LiteralKind.Octal:
		case LiteralKind.Binary:
		case LiteralKind.Float:
			return true;
		default:
			return false;
	}
}

export function is_LiteralBooleanLike(node: Node): node is Literal & { kind: LiteralKind.False | LiteralKind.True } {
	switch (is_Literal(node) ? node.kind : LiteralKind.Char) {
		case LiteralKind.False:
		case LiteralKind.True:
			return true;
		default:
			return false;
	}
}

export function is_BlockCommentKind(node: AttributeOrComment): node is CommentOrDocComment & { line: false } {
	return !node.line;
}

export function is_LineCommentKind(node: AttributeOrComment): node is CommentOrDocComment & { line: true } {
	return node.line;
}

export function is_BlockCommentNode(node: Node): node is CommentOrDocComment & { line: false } {
	return is_CommentOrDocComment(node) && is_BlockCommentKind(node);
}

export function is_LineCommentNode(node: Node): node is CommentOrDocComment & { line: true } {
	return is_CommentOrDocComment(node) && is_LineCommentKind(node);
}

export function is_BareTypeTraitBound(
	node: TypeTraitBound
): node is TypeTraitBound & { maybeConst: false; optional: false; ltParameters: undefined } {
	__DEV__: assert(is_TypeTraitBound(node));
	return !hasAttributes(node) && !node.maybeConst && !node.optional && undefined === node.ltParameters;
}

// prettier-ignore
export function is_multiplicativeOperator(tk: number): boolean {
	switch (tk) {
		case TK["%"]: case TK["*"]: case TK["/"]: return true;
		default: return false;
	}
}

// prettier-ignore
export function is_bitshiftOperator(tk: number): boolean {
	switch (tk) {
		case TK["<<"]: case TK[">>"]: return true;
		default: return false;
	}
}

// prettier-ignore
export function is_BitwiseOperator(tk: number): boolean {
	switch (tk) {
		case TK["<<"]: case TK[">>"]: case TK["|"]: case TK["^"]: case TK["&"]: return true;
		default: return false;
	}
}

// prettier-ignore
export function is_LargerLesserOperator(tk: number): boolean {
	switch (tk) {
		case TK[">"]: case TK["<"]: case TK["<="]: case TK[">="]: return true;
		default: return false;
	}
}

// prettier-ignore
export function is_EqualityOperator(tk: number): boolean {
	switch (tk) {
		case TK["=="]: case TK["!="]:return true;
		default: return false;
	}
}
