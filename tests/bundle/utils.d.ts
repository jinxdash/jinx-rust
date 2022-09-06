import {
	Node,
	FunctionDeclarationParameters,
	AttributeOrDocComment,
	FunctionLikeNode,
	FunctionDeclaration,
	FunctionSelfParameterDeclaration,
	Literal,
	NodeWithCondition,
	ObjectNode,
	NodeWithBody,
	ArrayLikeNode,
	DeclarationNode,
	CallExpression,
	Identifier,
	NodeWithTypeBounds,
	MacroInvocation,
	ConditionExpression,
	ScrutineeExpression,
	ParenthesizedNode,
	Delimited,
	MaybeExpressionBody,
	NodeType,
	BaseNode,
	NTMap,
	TK,
	PunctuationToken,
	DelimitedSequence,
	MissingNode,
	SourceFile,
	Shebang,
	Program,
	Snippet,
	Comment,
	Index as Index$1,
	LbIdentifier,
	McIdentifier,
	ItemPath,
	DelimGroup,
	Attribute,
	DocCommentAttribute,
	MacroRulesDeclaration,
	MacroDeclaration,
	MacroRuleDeclaration,
	MacroInlineRuleDeclaration,
	MacroGroup,
	MacroParameterDeclaration,
	PubSpecifier,
	ExternSpecifier,
	ExpressionStatement,
	UseStatement,
	NamedImport,
	AmbientImport,
	AnonymousImport,
	DestructuredImport,
	ExternCrateStatement,
	ExternBlockDeclaration,
	TypeAliasDeclaration,
	ConstVariableDeclaration,
	StaticVariableDeclaration,
	LetVariableDeclaration,
	ModuleDeclaration,
	FunctionParameterDeclaration,
	FunctionSpread,
	StructDeclaration,
	TupleStructDeclaration,
	StructPropertyDeclaration,
	TupleStructItemDeclaration,
	UnionDeclaration,
	EnumDeclaration,
	EnumMemberDeclaration,
	EnumMemberTupleDeclaration,
	EnumMemberStructDeclaration,
	TraitDeclaration,
	AutoTraitDeclaration,
	TraitAliasDeclaration,
	ImplDeclaration,
	NegativeImplDeclaration,
	ExpressionTypeSelector,
	ExpressionTypeCast,
	ExpressionPath,
	ExpressionAsTypeCast,
	ReturnExpression,
	BreakExpression,
	ContinueExpression,
	YieldExpression,
	MemberExpression,
	AwaitExpression,
	UnwrapExpression,
	ParenthesizedExpression,
	MinusExpression,
	NotExpression,
	OrExpression,
	AndExpression,
	ReassignmentExpression,
	UnassignedExpression,
	OperationExpression,
	ReassignmentOperationExpression,
	ComparisonExpression,
	LetScrutinee,
	ClosureFunctionExpression,
	ClosureFunctionParameterDeclaration,
	BlockExpression,
	LoopBlockExpression,
	WhileBlockExpression,
	ForInBlockExpression,
	TryBlockExpression,
	IfBlockExpression,
	MatchExpression,
	MatchExpressionCase,
	RangeLiteral,
	StructLiteral,
	StructLiteralProperty,
	StructLiteralPropertyShorthand,
	StructLiteralPropertySpread,
	StructLiteralRestUnassigned,
	TupleLiteral,
	ArrayLiteral,
	SizedArrayLiteral,
	ReferenceExpression,
	RawReferenceExpression,
	DereferenceExpression,
	BoxExpression,
	UnionPattern,
	ParenthesizedPattern,
	RestPattern,
	WildcardPattern,
	PatternVariableDeclaration,
	StructPattern,
	StructPatternPropertyDestructured,
	StructPatternPropertyShorthand,
	TuplePattern,
	ArrayPattern,
	ReferencePattern,
	BoxPattern,
	MinusPattern,
	RangePattern,
	TypePath,
	TypeCall,
	TypeCallNamedArgument,
	TypeCallNamedBound,
	LtIdentifier,
	LtElided,
	LtStatic,
	TypeNever,
	TypeInferred,
	GenericTypeParameterDeclaration,
	ConstTypeParameterDeclaration,
	GenericLtParameterDeclaration,
	WhereTypeBoundDeclaration,
	WhereLtBoundDeclaration,
	TypeTraitBound,
	TypeDynBounds,
	TypeImplBounds,
	TypeFnPointer,
	TypeFnPointerParameter,
	TypeFunction,
	TypeTuple,
	TypeSizedArray,
	TypeSlice,
	TypeReference,
	TypeDereferenceConst,
	TypeDereferenceMut,
	TypeParenthesized,
	ImportNode,
	StatementNode,
	ExpressionNode,
	PatternNode,
	TypeNode,
	AttributeOrComment,
	CommentOrDocComment,
	IdentifierOrIndex,
	IdentifierOrItemPath,
	ExpressionNamespaceTargetNoSelector,
	TypeNamespaceTargetNoSelector,
	ExpressionNamespaceTarget,
	TypeNamespaceTarget,
	PatternNoUnion,
	PatternNoUnionNoRange,
	MaybePubNode,
	MaybeExternNode,
	MaybeAsyncNode,
	MaybeMoveNode,
	MaybeUnsafeNode,
	MaybeStaticNode,
	MacroRule,
	PathNode,
	RangeNode,
	FunctionNode,
	TypeFunctionNode,
	ArrayOrTupleLiteral,
	TupleNode,
	TraitDeclarationNode,
	ImplDeclarationNode,
	NodeWithSegments,
	NodeWithBodyOrCases,
	NodeWithBodyNoBody,
	ImplicitReturnAbleNode,
	ExpressionWithBody,
	ExpressionWithBodyOrCases,
	LogicalExpression,
	LeftRightExpression,
	FlowControlExpression,
	UnaryExpression,
	UnaryPattern,
	UnaryType,
	PostfixExpression,
	VariableDeclarationNode,
	ReassignmentNode,
	TypeBoundsStandaloneNode,
	FunctionParameterNode,
	NodeWithMaybePatternNoUnionBody,
	EnumDeclarationMember,
	StructProperty,
	StructPatternProperty,
	RangePatternBound,
	Lifetime,
	GenericParameterDeclaration,
	WhereBoundDeclaration,
	TypeBound,
	PRCD,
	LiteralKind,
	Located,
	Loc,
	LocArray,
} from "../dist/index";

declare type Index = number | "self";
declare type ChildNodeIndex = Index | undefined;
interface AstItEachFn<R = void> {
	(child: Node, parent: Node, key: string, index?: Index): R;
}
declare type NodeValues<T> = T extends FunctionDeclarationParameters ? T[Index] : T extends ReadonlyArray<any> ? T[number] : T;
declare type NodeChildTypes<T> = T extends never ? never : Extract<NodeValues<T[keyof T]>, { nodeType: any }>;
declare type PickProps<T, F> = T extends never
	? never
	: {
			[K in keyof T as T[K] & {} extends infer U ? (U extends F ? K : never) : never]: T[K];
	  };
declare type BoolProps<T> = PickProps<T, boolean>;
declare type NodeProps<T> = PickProps<T, { nodeType: number }>;
declare type ArrayProps<T> = PickProps<T, { nodeType: number }[]>;
/** Iterate only direct childNodes */
declare function each_childNode(target: Node, callback: AstItEachFn): void;
/** Iterate all nodes in target. Bottom up, excluding itself. */
declare function each_node(target: Node, callback: AstItEachFn): void;
declare function getNodeChildren<T extends Node>(node: T): NodeChildTypes<T>[];
declare function hasChildren(node: Node): boolean;
/** returns the keys needed to access target from parent */
declare function getAstPath(parent: Node, target: Node): (string | number)[];
/** returns the key or the key + index needed to access a childNode from its direct parent */
declare function getOwnChildAstPath(parent: Node, child: Node): [string] | [string, number | "self"];
declare function reassignNodeProperty(value: any, parent: Node, key: string, index?: Index): void;
/** Iterate Object.keys to find childNodes, as opposed to known properties of that NodeType */
declare function getActualNodeChildren(node: Node): Node[];
declare function countActualNodeChildren(node: Node): number;

declare type WithAttributes<T extends Node> = T & { attributes: AttributeOrDocComment[] };
declare type ParameterOf<T extends FunctionLikeNode> =
	| T["parameters"]
	| (T extends FunctionDeclaration ? FunctionSelfParameterDeclaration : never);
declare function hasAttributes<T extends Node>(node: T): node is WithAttributes<T>;
declare function hasOuterAttributes<T extends Node>(node: T): node is WithAttributes<T>;
declare function hasInnerAttributes<T extends Node>(node: T): node is WithAttributes<T>;
declare function hasSuffix(node: Node): node is Literal & { suffix: {} };
declare function hasCondition(node: Node): node is NodeWithCondition & { condition: {} };
declare function hasLetScrutineeCondition(node: Node): node is NodeWithCondition & { condition: {} };
declare function hasSemiNoProperties(node: Node): node is ObjectNode & { properties: undefined };
declare function hasSemiNoBody(node: Node): node is NodeWithBody & { body: undefined };
declare function hasProperties(node: Node): node is ObjectNode & { properties: {} };
declare function hasItems(node: Node): node is ArrayLikeNode & { items: {} };
declare function hasBody(node: Node): node is NodeWithBody & { body: {} };
declare function hasGenerics(node: Node): node is DeclarationNode & { generics: {} };
declare function hasSelfParameter(
	node: FunctionLikeNode
): node is FunctionLikeNode & { parameters: { self: FunctionSelfParameterDeclaration } };
declare function hasParameters(node: FunctionLikeNode): boolean;
declare function hasMethod(node: CallExpression): node is CallExpression & { method: Identifier };
declare function hasTypeBounds(node: Node): node is NodeWithTypeBounds & { typeBounds: {} };
declare type NodeWithMaybeExpressionProperty = Extract<Node, MaybeExpressionBody>;
declare function hasExpression(node: NodeWithMaybeExpressionProperty): node is NodeWithMaybeExpressionProperty & { expression: {} };
declare function getParameters<T extends FunctionLikeNode>(
	node: T
): [...T["parameters"]] | (T extends FunctionDeclaration ? [FunctionSelfParameterDeclaration, ...T["parameters"]] : never);
declare function getFirstParameter<T extends FunctionLikeNode>(node: T): ParameterOf<T> | undefined;
declare function getLastParameter<T extends FunctionLikeNode>(node: T): ParameterOf<T> | undefined;
declare function getBodyOrCases<T extends { body: any } | { cases: any }>(
	node: T
): T extends { body: infer U } ? U : T extends { cases: infer U } ? U : never;
declare function getMacroName(node: MacroInvocation): string;
declare function getLeftMostCondition(node: ConditionExpression): ScrutineeExpression;
declare function getParenthesizedNodeContent<T extends ParenthesizedNode>(node: T): NodeChildTypes<T>;
declare function countRawLiteralHashtags(node: Literal): number;
declare type DelimChars = { left: string; right: string };
declare function getDelimChars(node: Delimited): DelimChars;

declare function nisAnyOf<T extends NodeType[]>(node: BaseNode, nodeTypes: T): node is NTMap[T[number]];
declare function isTK(node: Node, tk: TK): node is PunctuationToken & { tk: TK };
declare function includesTK(node: DelimitedSequence<any>, tk: TK): boolean;
declare function is_MissingNode(node: Node): node is MissingNode;
declare function is_SourceFile(node: Node): node is SourceFile;
declare function is_Shebang(node: Node): node is Shebang;
declare function is_Program(node: Node): node is Program;
declare function is_Snippet(node: Node): node is Snippet;
declare function is_Comment(node: Node): node is Comment;
declare function is_Identifier(node: Node): node is Identifier;
declare function is_Index(node: Node): node is Index$1;
declare function is_LbIdentifier(node: Node): node is LbIdentifier;
declare function is_McIdentifier(node: Node): node is McIdentifier;
declare function is_Literal(node: Node): node is Literal;
declare function is_ItemPath(node: Node): node is ItemPath;
declare function is_PunctuationToken(node: Node): node is PunctuationToken;
declare function is_DelimGroup(node: Node): node is DelimGroup;
declare function is_Attribute(node: Node): node is Attribute;
declare function is_DocCommentAttribute(node: Node): node is DocCommentAttribute;
declare function is_MacroInvocation(node: Node): node is MacroInvocation;
declare function is_MacroRulesDeclaration(node: Node): node is MacroRulesDeclaration;
declare function is_MacroDeclaration(node: Node): node is MacroDeclaration;
declare function is_MacroRuleDeclaration(node: Node): node is MacroRuleDeclaration;
declare function is_MacroInlineRuleDeclaration(node: Node): node is MacroInlineRuleDeclaration;
declare function is_MacroGroup(node: Node): node is MacroGroup;
declare function is_MacroParameterDeclaration(node: Node): node is MacroParameterDeclaration;
declare function is_PubSpecifier(node: Node): node is PubSpecifier;
declare function is_ExternSpecifier(node: Node): node is ExternSpecifier;
declare function is_ExpressionStatement(node: Node): node is ExpressionStatement;
declare function is_UseStatement(node: Node): node is UseStatement;
declare function is_NamedImport(node: Node): node is NamedImport;
declare function is_AmbientImport(node: Node): node is AmbientImport;
declare function is_AnonymousImport(node: Node): node is AnonymousImport;
declare function is_DestructuredImport(node: Node): node is DestructuredImport;
declare function is_ExternCrateStatement(node: Node): node is ExternCrateStatement;
declare function is_ExternBlockDeclaration(node: Node): node is ExternBlockDeclaration;
declare function is_TypeAliasDeclaration(node: Node): node is TypeAliasDeclaration;
declare function is_ConstVariableDeclaration(node: Node): node is ConstVariableDeclaration;
declare function is_StaticVariableDeclaration(node: Node): node is StaticVariableDeclaration;
declare function is_LetVariableDeclaration(node: Node): node is LetVariableDeclaration;
declare function is_ModuleDeclaration(node: Node): node is ModuleDeclaration;
declare function is_FunctionDeclaration(node: Node): node is FunctionDeclaration;
declare function is_FunctionSelfParameterDeclaration(node: Node): node is FunctionSelfParameterDeclaration;
declare function is_FunctionParameterDeclaration(node: Node): node is FunctionParameterDeclaration;
declare function is_FunctionSpread(node: Node): node is FunctionSpread;
declare function is_StructDeclaration(node: Node): node is StructDeclaration;
declare function is_TupleStructDeclaration(node: Node): node is TupleStructDeclaration;
declare function is_StructPropertyDeclaration(node: Node): node is StructPropertyDeclaration;
declare function is_TupleStructItemDeclaration(node: Node): node is TupleStructItemDeclaration;
declare function is_UnionDeclaration(node: Node): node is UnionDeclaration;
declare function is_EnumDeclaration(node: Node): node is EnumDeclaration;
declare function is_EnumMemberDeclaration(node: Node): node is EnumMemberDeclaration;
declare function is_EnumMemberTupleDeclaration(node: Node): node is EnumMemberTupleDeclaration;
declare function is_EnumMemberStructDeclaration(node: Node): node is EnumMemberStructDeclaration;
declare function is_TraitDeclaration(node: Node): node is TraitDeclaration;
declare function is_AutoTraitDeclaration(node: Node): node is AutoTraitDeclaration;
declare function is_TraitAliasDeclaration(node: Node): node is TraitAliasDeclaration;
declare function is_ImplDeclaration(node: Node): node is ImplDeclaration;
declare function is_NegativeImplDeclaration(node: Node): node is NegativeImplDeclaration;
declare function is_ExpressionTypeSelector(node: Node): node is ExpressionTypeSelector;
declare function is_ExpressionTypeCast(node: Node): node is ExpressionTypeCast;
declare function is_ExpressionPath(node: Node): node is ExpressionPath;
declare function is_ExpressionAsTypeCast(node: Node): node is ExpressionAsTypeCast;
declare function is_ReturnExpression(node: Node): node is ReturnExpression;
declare function is_BreakExpression(node: Node): node is BreakExpression;
declare function is_ContinueExpression(node: Node): node is ContinueExpression;
declare function is_YieldExpression(node: Node): node is YieldExpression;
declare function is_CallExpression(node: Node): node is CallExpression;
declare function is_MemberExpression(node: Node): node is MemberExpression;
declare function is_AwaitExpression(node: Node): node is AwaitExpression;
declare function is_UnwrapExpression(node: Node): node is UnwrapExpression;
declare function is_ParenthesizedExpression(node: Node): node is ParenthesizedExpression;
declare function is_MinusExpression(node: Node): node is MinusExpression;
declare function is_NotExpression(node: Node): node is NotExpression;
declare function is_OrExpression(node: Node): node is OrExpression;
declare function is_AndExpression(node: Node): node is AndExpression;
declare function is_ReassignmentExpression(node: Node): node is ReassignmentExpression;
declare function is_UnassignedExpression(node: Node): node is UnassignedExpression;
declare function is_OperationExpression(node: Node): node is OperationExpression;
declare function is_ReassignmentOperationExpression(node: Node): node is ReassignmentOperationExpression;
declare function is_ComparisonExpression(node: Node): node is ComparisonExpression;
declare function is_LetScrutinee(node: Node): node is LetScrutinee;
declare function is_ClosureFunctionExpression(node: Node): node is ClosureFunctionExpression;
declare function is_ClosureFunctionParameterDeclaration(node: Node): node is ClosureFunctionParameterDeclaration;
declare function is_BlockExpression(node: Node): node is BlockExpression;
declare function is_LoopBlockExpression(node: Node): node is LoopBlockExpression;
declare function is_WhileBlockExpression(node: Node): node is WhileBlockExpression;
declare function is_ForInBlockExpression(node: Node): node is ForInBlockExpression;
declare function is_TryBlockExpression(node: Node): node is TryBlockExpression;
declare function is_IfBlockExpression(node: Node): node is IfBlockExpression;
declare function is_MatchExpression(node: Node): node is MatchExpression;
declare function is_MatchExpressionCase(node: Node): node is MatchExpressionCase;
declare function is_RangeLiteral(node: Node): node is RangeLiteral;
declare function is_StructLiteral(node: Node): node is StructLiteral;
declare function is_StructLiteralProperty(node: Node): node is StructLiteralProperty;
declare function is_StructLiteralPropertyShorthand(node: Node): node is StructLiteralPropertyShorthand;
declare function is_StructLiteralPropertySpread(node: Node): node is StructLiteralPropertySpread;
declare function is_StructLiteralRestUnassigned(node: Node): node is StructLiteralRestUnassigned;
declare function is_TupleLiteral(node: Node): node is TupleLiteral;
declare function is_ArrayLiteral(node: Node): node is ArrayLiteral;
declare function is_SizedArrayLiteral(node: Node): node is SizedArrayLiteral;
declare function is_ReferenceExpression(node: Node): node is ReferenceExpression;
declare function is_RawReferenceExpression(node: Node): node is RawReferenceExpression;
declare function is_DereferenceExpression(node: Node): node is DereferenceExpression;
declare function is_BoxExpression(node: Node): node is BoxExpression;
declare function is_UnionPattern(node: Node): node is UnionPattern;
declare function is_ParenthesizedPattern(node: Node): node is ParenthesizedPattern;
declare function is_RestPattern(node: Node): node is RestPattern;
declare function is_WildcardPattern(node: Node): node is WildcardPattern;
declare function is_PatternVariableDeclaration(node: Node): node is PatternVariableDeclaration;
declare function is_StructPattern(node: Node): node is StructPattern;
declare function is_StructPatternPropertyDestructured(node: Node): node is StructPatternPropertyDestructured;
declare function is_StructPatternPropertyShorthand(node: Node): node is StructPatternPropertyShorthand;
declare function is_TuplePattern(node: Node): node is TuplePattern;
declare function is_ArrayPattern(node: Node): node is ArrayPattern;
declare function is_ReferencePattern(node: Node): node is ReferencePattern;
declare function is_BoxPattern(node: Node): node is BoxPattern;
declare function is_MinusPattern(node: Node): node is MinusPattern;
declare function is_RangePattern(node: Node): node is RangePattern;
declare function is_TypePath(node: Node): node is TypePath;
declare function is_TypeCall(node: Node): node is TypeCall;
declare function is_TypeCallNamedArgument(node: Node): node is TypeCallNamedArgument;
declare function is_TypeCallNamedBound(node: Node): node is TypeCallNamedBound;
declare function is_LtIdentifier(node: Node): node is LtIdentifier;
declare function is_LtElided(node: Node): node is LtElided;
declare function is_LtStatic(node: Node): node is LtStatic;
declare function is_TypeNever(node: Node): node is TypeNever;
declare function is_TypeInferred(node: Node): node is TypeInferred;
declare function is_GenericTypeParameterDeclaration(node: Node): node is GenericTypeParameterDeclaration;
declare function is_ConstTypeParameterDeclaration(node: Node): node is ConstTypeParameterDeclaration;
declare function is_GenericLtParameterDeclaration(node: Node): node is GenericLtParameterDeclaration;
declare function is_WhereTypeBoundDeclaration(node: Node): node is WhereTypeBoundDeclaration;
declare function is_WhereLtBoundDeclaration(node: Node): node is WhereLtBoundDeclaration;
declare function is_TypeTraitBound(node: Node): node is TypeTraitBound;
declare function is_TypeDynBounds(node: Node): node is TypeDynBounds;
declare function is_TypeImplBounds(node: Node): node is TypeImplBounds;
declare function is_TypeFnPointer(node: Node): node is TypeFnPointer;
declare function is_TypeFnPointerParameter(node: Node): node is TypeFnPointerParameter;
declare function is_TypeFunction(node: Node): node is TypeFunction;
declare function is_TypeTuple(node: Node): node is TypeTuple;
declare function is_TypeSizedArray(node: Node): node is TypeSizedArray;
declare function is_TypeSlice(node: Node): node is TypeSlice;
declare function is_TypeReference(node: Node): node is TypeReference;
declare function is_TypeDereferenceConst(node: Node): node is TypeDereferenceConst;
declare function is_TypeDereferenceMut(node: Node): node is TypeDereferenceMut;
declare function is_TypeParenthesized(node: Node): node is TypeParenthesized;
declare function is_ImportNode(node: Node): node is ImportNode;
declare function is_StatementNode(node: Node): node is StatementNode;
declare function is_ExpressionNode(node: Node): node is ExpressionNode;
declare function is_PatternNode(node: Node): node is PatternNode;
declare function is_TypeNode(node: Node): node is TypeNode;
declare function is_AttributeOrComment(node: Node): node is AttributeOrComment;
declare function is_AttributeOrDocComment(node: Node): node is AttributeOrDocComment;
declare function is_CommentOrDocComment(node: Node): node is CommentOrDocComment;
declare function is_IdentifierOrIndex(node: Node): node is IdentifierOrIndex;
declare function is_IdentifierOrItemPath(node: Node): node is IdentifierOrItemPath;
declare function is_ExpressionNamespaceTargetNoSelector(node: Node): node is ExpressionNamespaceTargetNoSelector;
declare function is_TypeNamespaceTargetNoSelector(node: Node): node is TypeNamespaceTargetNoSelector;
declare function is_ExpressionNamespaceTarget(node: Node): node is ExpressionNamespaceTarget;
declare function is_TypeNamespaceTarget(node: Node): node is TypeNamespaceTarget;
declare function is_PatternNoUnion(node: Node): node is PatternNoUnion;
declare function is_PatternNoUnionNoRange(node: Node): node is PatternNoUnionNoRange;
declare function is_MaybePubNode(node: Node): node is MaybePubNode;
declare function is_MaybeExternNode(node: Node): node is MaybeExternNode;
declare function is_MaybeAsyncNode(node: Node): node is MaybeAsyncNode;
declare function is_MaybeMoveNode(node: Node): node is MaybeMoveNode;
declare function is_MaybeUnsafeNode(node: Node): node is MaybeUnsafeNode;
declare function is_MaybeStaticNode(node: Node): node is MaybeStaticNode;
declare function is_MacroRule(node: Node): node is MacroRule;
declare function is_PathNode(node: Node): node is PathNode;
declare function is_RangeNode(node: Node): node is RangeNode;
declare function is_FunctionNode(node: Node): node is FunctionNode;
declare function is_TypeFunctionNode(node: Node): node is TypeFunctionNode;
declare function is_ParenthesizedNode(node: Node): node is ParenthesizedNode;
declare function is_ObjectNode(node: Node): node is ObjectNode;
declare function is_ArrayLikeNode(node: Node): node is ArrayLikeNode;
declare function is_ArrayOrTupleLiteral(node: Node): node is ArrayOrTupleLiteral;
declare function is_TupleNode(node: Node): node is TupleNode;
declare function is_DeclarationNode(node: Node): node is DeclarationNode;
declare function is_TraitDeclarationNode(node: Node): node is TraitDeclarationNode;
declare function is_ImplDeclarationNode(node: Node): node is ImplDeclarationNode;
declare function is_NodeWithSegments(node: Node): node is NodeWithSegments;
declare function is_NodeWithBody(node: Node): node is NodeWithBody;
declare function is_NodeWithBodyOrCases(node: Node): node is NodeWithBodyOrCases;
declare function is_NodeWithBodyNoBody(node: Node): node is NodeWithBodyNoBody;
declare function is_NodeWithCondition(node: Node): node is NodeWithCondition;
declare function is_ImplicitReturnAbleNode(node: Node): node is ImplicitReturnAbleNode;
declare function is_ExpressionWithBody(node: Node): node is ExpressionWithBody;
declare function is_ExpressionWithBodyOrCases(node: Node): node is ExpressionWithBodyOrCases;
declare function is_LogicalExpression(node: Node): node is LogicalExpression;
declare function is_LeftRightExpression(node: Node): node is LeftRightExpression;
declare function is_FlowControlExpression(node: Node): node is FlowControlExpression;
declare function is_UnaryExpression(node: Node): node is UnaryExpression;
declare function is_UnaryPattern(node: Node): node is UnaryPattern;
declare function is_UnaryType(node: Node): node is UnaryType;
declare function is_PostfixExpression(node: Node): node is PostfixExpression;
declare function is_VariableDeclarationNode(node: Node): node is VariableDeclarationNode;
declare function is_ReassignmentNode(node: Node): node is ReassignmentNode;
declare function is_NodeWithTypeBounds(node: Node): node is NodeWithTypeBounds;
declare function is_TypeBoundsStandaloneNode(node: Node): node is TypeBoundsStandaloneNode;
declare function is_FunctionLikeNode(node: Node): node is FunctionLikeNode;
declare function is_FunctionParameterNode(node: Node): node is FunctionParameterNode;
declare function is_NodeWithMaybePatternNoUnionBody(node: Node): node is NodeWithMaybePatternNoUnionBody;
declare function is_EnumDeclarationMember(node: Node): node is EnumDeclarationMember;
declare function is_StructProperty(node: Node): node is StructProperty;
declare function is_StructPatternProperty(node: Node): node is StructPatternProperty;
declare function is_RangePatternBound(node: Node): node is RangePatternBound;
declare function is_Lifetime(node: Node): node is Lifetime;
declare function is_GenericParameterDeclaration(node: Node): node is GenericParameterDeclaration;
declare function is_WhereBoundDeclaration(node: Node): node is WhereBoundDeclaration;
declare function is_TypeBound(node: Node): node is TypeBound;
declare function can_have_OuterAttributes(node: Node, parent: Node | undefined, stmt_expr_attributes: boolean): boolean;
declare function getPrecedence(node: ExpressionNode | ConditionExpression, insideScrutinee: boolean): PRCD;
declare function is_MacroInvocation_BlockLike(node: Node): node is MacroInvocation;
declare function is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation(node: Node): boolean;
declare function is_ElseBlock(node: Node, parent: Node): parent is IfBlockExpression;
declare function is_CaseBlock(node: Node, parent: Node): parent is MatchExpressionCase;
declare function is_ClosureBlock(node: Node, parent: Node): parent is ClosureFunctionExpression;
declare function is_FlowControlMaybeValueExpression(node: Node): node is Exclude<FlowControlExpression, ContinueExpression>;
declare function is_BareTypeTraitBound(
	node: TypeTraitBound
): node is TypeTraitBound & { maybeConst: false; optional: false; ltParameters: undefined };
declare function is_LiteralStringLike(
	node: Node
): node is Literal & { kind: LiteralKind.bString | LiteralKind.brString | LiteralKind.rString | LiteralKind.String };
declare function is_LiteralRawStringLike(node: Node): node is Literal & { kind: LiteralKind.brString | LiteralKind.rString };
declare function is_LiteralNumberLike(
	node: Node
): node is Literal & { kind: LiteralKind.Binary | LiteralKind.Hex | LiteralKind.Octal | LiteralKind.Integer | LiteralKind.Float };
declare function is_LiteralBooleanLike(node: Node): node is Literal & { kind: LiteralKind.False | LiteralKind.True };
declare function isInner(node: AttributeOrDocComment): node is AttributeOrDocComment & { inner: true };
declare function isOuter(node: AttributeOrDocComment): node is AttributeOrDocComment & { inner: false };
declare function isDangling(node: AttributeOrDocComment): boolean;
declare function is_LineCommentKind(node: AttributeOrComment): node is CommentOrDocComment & { line: true };
declare function is_BlockCommentKind(node: AttributeOrComment): node is CommentOrDocComment & { line: false };
declare function is_BlockCommentNode(node: Node): node is CommentOrDocComment & { line: false };
declare function is_LineCommentNode(node: Node): node is CommentOrDocComment & { line: true };
declare function can_have_InnerAttributes(node: Node): node is NodeWithBodyOrCases;
declare function can_have_Attributes(node: Node, parent: Node | undefined, stmt_expr_attributes: boolean): boolean;
declare function is_multiplicativeOperator(tk: number): boolean;
declare function is_bitshiftOperator(tk: number): boolean;
declare function is_BitwiseOperator(tk: number): boolean;
declare function is_LargerLesserOperator(tk: number): boolean;
declare function is_EqualityOperator(tk: number): boolean;

declare function unsafe_set_nodeType(node: Node, nodeType: NodeType): void;
declare function unsafe_setRangeStart(target: Located, startPos: number): void;
declare function unsafe_setRangeEnd(target: Located, endPos: number): void;
declare function setRangeStart(target: Located, startPos: number): void;
declare function setRangeEnd(target: Located, endPos: number): void;
declare function setRange(target: Located, startPos: number, endPos: number): void;
declare function deleteAttributes(target: Node): void;
declare function assignAttributes(target: Node, attrs: AttributeOrDocComment[]): void;
declare function transferAttributes(from: Node, to: Node): void;
/** Insert node in array sorted by start loc */
declare function insertNode<T extends Node[]>(array: T, node: T[number]): void;
declare function insertNodes<T extends Node[]>(array: T, nodes: T[number][]): void;

declare function is_Loc(data: any): data is Loc;
declare function is_Located(data: any): data is Located;
declare function is_Node(node: any): node is Node;
declare function is_LocArray(data: any): data is LocArray<Node>;
declare function start(node: Located): number;
declare function end(node: Located): number;
declare function hasOwnStart(node: Located): boolean;
declare function ownStart(node: Located): number;
declare function isLocEqual(a: Loc, b: Loc): boolean;

export {
	ArrayProps,
	BoolProps,
	ChildNodeIndex,
	DelimChars,
	NodeChildTypes,
	NodeProps,
	ParameterOf,
	PickProps,
	WithAttributes,
	assignAttributes,
	can_have_Attributes,
	can_have_InnerAttributes,
	can_have_OuterAttributes,
	countActualNodeChildren,
	countRawLiteralHashtags,
	deleteAttributes,
	each_childNode,
	each_node,
	end,
	getActualNodeChildren,
	getAstPath,
	getBodyOrCases,
	getDelimChars,
	getFirstParameter,
	getLastParameter,
	getLeftMostCondition,
	getMacroName,
	getNodeChildren,
	getOwnChildAstPath,
	getParameters,
	getParenthesizedNodeContent,
	getPrecedence,
	hasAttributes,
	hasBody,
	hasChildren,
	hasCondition,
	hasExpression,
	hasGenerics,
	hasInnerAttributes,
	hasItems,
	hasLetScrutineeCondition,
	hasMethod,
	hasOuterAttributes,
	hasOwnStart,
	hasParameters,
	hasProperties,
	hasSelfParameter,
	hasSemiNoBody,
	hasSemiNoProperties,
	hasSuffix,
	hasTypeBounds,
	includesTK,
	insertNode,
	insertNodes,
	isDangling,
	isInner,
	isLocEqual,
	isOuter,
	isTK,
	is_AmbientImport,
	is_AndExpression,
	is_AnonymousImport,
	is_ArrayLikeNode,
	is_ArrayLiteral,
	is_ArrayOrTupleLiteral,
	is_ArrayPattern,
	is_Attribute,
	is_AttributeOrComment,
	is_AttributeOrDocComment,
	is_AutoTraitDeclaration,
	is_AwaitExpression,
	is_BareTypeTraitBound,
	is_BitwiseOperator,
	is_BlockCommentKind,
	is_BlockCommentNode,
	is_BlockExpression,
	is_BoxExpression,
	is_BoxPattern,
	is_BreakExpression,
	is_CallExpression,
	is_CaseBlock,
	is_ClosureBlock,
	is_ClosureFunctionExpression,
	is_ClosureFunctionParameterDeclaration,
	is_Comment,
	is_CommentOrDocComment,
	is_ComparisonExpression,
	is_ConstTypeParameterDeclaration,
	is_ConstVariableDeclaration,
	is_ContinueExpression,
	is_DeclarationNode,
	is_DelimGroup,
	is_DereferenceExpression,
	is_DestructuredImport,
	is_DocCommentAttribute,
	is_ElseBlock,
	is_EnumDeclaration,
	is_EnumDeclarationMember,
	is_EnumMemberDeclaration,
	is_EnumMemberStructDeclaration,
	is_EnumMemberTupleDeclaration,
	is_EqualityOperator,
	is_ExpressionAsTypeCast,
	is_ExpressionNamespaceTarget,
	is_ExpressionNamespaceTargetNoSelector,
	is_ExpressionNode,
	is_ExpressionPath,
	is_ExpressionStatement,
	is_ExpressionTypeCast,
	is_ExpressionTypeSelector,
	is_ExpressionWithBody,
	is_ExpressionWithBodyOrCases,
	is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation,
	is_ExternBlockDeclaration,
	is_ExternCrateStatement,
	is_ExternSpecifier,
	is_FlowControlExpression,
	is_FlowControlMaybeValueExpression,
	is_ForInBlockExpression,
	is_FunctionDeclaration,
	is_FunctionLikeNode,
	is_FunctionNode,
	is_FunctionParameterDeclaration,
	is_FunctionParameterNode,
	is_FunctionSelfParameterDeclaration,
	is_FunctionSpread,
	is_GenericLtParameterDeclaration,
	is_GenericParameterDeclaration,
	is_GenericTypeParameterDeclaration,
	is_Identifier,
	is_IdentifierOrIndex,
	is_IdentifierOrItemPath,
	is_IfBlockExpression,
	is_ImplDeclaration,
	is_ImplDeclarationNode,
	is_ImplicitReturnAbleNode,
	is_ImportNode,
	is_Index,
	is_ItemPath,
	is_LargerLesserOperator,
	is_LbIdentifier,
	is_LeftRightExpression,
	is_LetScrutinee,
	is_LetVariableDeclaration,
	is_Lifetime,
	is_LineCommentKind,
	is_LineCommentNode,
	is_Literal,
	is_LiteralBooleanLike,
	is_LiteralNumberLike,
	is_LiteralRawStringLike,
	is_LiteralStringLike,
	is_Loc,
	is_LocArray,
	is_Located,
	is_LogicalExpression,
	is_LoopBlockExpression,
	is_LtElided,
	is_LtIdentifier,
	is_LtStatic,
	is_MacroDeclaration,
	is_MacroGroup,
	is_MacroInlineRuleDeclaration,
	is_MacroInvocation,
	is_MacroInvocation_BlockLike,
	is_MacroParameterDeclaration,
	is_MacroRule,
	is_MacroRuleDeclaration,
	is_MacroRulesDeclaration,
	is_MatchExpression,
	is_MatchExpressionCase,
	is_MaybeAsyncNode,
	is_MaybeExternNode,
	is_MaybeMoveNode,
	is_MaybePubNode,
	is_MaybeStaticNode,
	is_MaybeUnsafeNode,
	is_McIdentifier,
	is_MemberExpression,
	is_MinusExpression,
	is_MinusPattern,
	is_MissingNode,
	is_ModuleDeclaration,
	is_NamedImport,
	is_NegativeImplDeclaration,
	is_Node,
	is_NodeWithBody,
	is_NodeWithBodyNoBody,
	is_NodeWithBodyOrCases,
	is_NodeWithCondition,
	is_NodeWithMaybePatternNoUnionBody,
	is_NodeWithSegments,
	is_NodeWithTypeBounds,
	is_NotExpression,
	is_ObjectNode,
	is_OperationExpression,
	is_OrExpression,
	is_ParenthesizedExpression,
	is_ParenthesizedNode,
	is_ParenthesizedPattern,
	is_PathNode,
	is_PatternNoUnion,
	is_PatternNoUnionNoRange,
	is_PatternNode,
	is_PatternVariableDeclaration,
	is_PostfixExpression,
	is_Program,
	is_PubSpecifier,
	is_PunctuationToken,
	is_RangeLiteral,
	is_RangeNode,
	is_RangePattern,
	is_RangePatternBound,
	is_RawReferenceExpression,
	is_ReassignmentExpression,
	is_ReassignmentNode,
	is_ReassignmentOperationExpression,
	is_ReferenceExpression,
	is_ReferencePattern,
	is_RestPattern,
	is_ReturnExpression,
	is_Shebang,
	is_SizedArrayLiteral,
	is_Snippet,
	is_SourceFile,
	is_StatementNode,
	is_StaticVariableDeclaration,
	is_StructDeclaration,
	is_StructLiteral,
	is_StructLiteralProperty,
	is_StructLiteralPropertyShorthand,
	is_StructLiteralPropertySpread,
	is_StructLiteralRestUnassigned,
	is_StructPattern,
	is_StructPatternProperty,
	is_StructPatternPropertyDestructured,
	is_StructPatternPropertyShorthand,
	is_StructProperty,
	is_StructPropertyDeclaration,
	is_TraitAliasDeclaration,
	is_TraitDeclaration,
	is_TraitDeclarationNode,
	is_TryBlockExpression,
	is_TupleLiteral,
	is_TupleNode,
	is_TuplePattern,
	is_TupleStructDeclaration,
	is_TupleStructItemDeclaration,
	is_TypeAliasDeclaration,
	is_TypeBound,
	is_TypeBoundsStandaloneNode,
	is_TypeCall,
	is_TypeCallNamedArgument,
	is_TypeCallNamedBound,
	is_TypeDereferenceConst,
	is_TypeDereferenceMut,
	is_TypeDynBounds,
	is_TypeFnPointer,
	is_TypeFnPointerParameter,
	is_TypeFunction,
	is_TypeFunctionNode,
	is_TypeImplBounds,
	is_TypeInferred,
	is_TypeNamespaceTarget,
	is_TypeNamespaceTargetNoSelector,
	is_TypeNever,
	is_TypeNode,
	is_TypeParenthesized,
	is_TypePath,
	is_TypeReference,
	is_TypeSizedArray,
	is_TypeSlice,
	is_TypeTraitBound,
	is_TypeTuple,
	is_UnaryExpression,
	is_UnaryPattern,
	is_UnaryType,
	is_UnassignedExpression,
	is_UnionDeclaration,
	is_UnionPattern,
	is_UnwrapExpression,
	is_UseStatement,
	is_VariableDeclarationNode,
	is_WhereBoundDeclaration,
	is_WhereLtBoundDeclaration,
	is_WhereTypeBoundDeclaration,
	is_WhileBlockExpression,
	is_WildcardPattern,
	is_YieldExpression,
	is_bitshiftOperator,
	is_multiplicativeOperator,
	nisAnyOf,
	ownStart,
	reassignNodeProperty,
	setRange,
	setRangeEnd,
	setRangeStart,
	start,
	transferAttributes,
	unsafe_setRangeEnd,
	unsafe_setRangeStart,
	unsafe_set_nodeType,
};
