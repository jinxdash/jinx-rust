import { end, ownStart, start } from "../utils/ast";
import { AssertTypesEq, getLineChar, getLineIndex, urlAt } from "../utils/common";
import { createLocArray, mockNode } from "./helpers";
import { ParserOptions } from "./options";

//#region ================================================[        Types        ]===========================================================``--.

// <generated>
// prettier-ignore
export enum NodeType {
	MissingNode, SourceFile, Shebang, Program, Snippet, Comment, Identifier, Index, LbIdentifier, McIdentifier, Literal, ItemPath,
	PunctuationToken, DelimGroup, Attribute, DocCommentAttribute, MacroInvocation, MacroRulesDeclaration, MacroDeclaration,
	MacroRuleDeclaration, MacroInlineRuleDeclaration, MacroGroup, MacroParameterDeclaration, PubSpecifier, ExternSpecifier,
	ExpressionStatement, UseStatement, NamedImport, AmbientImport, AnonymousImport, DestructuredImport, ExternCrateStatement,
	ExternBlockDeclaration, TypeAliasDeclaration, ConstVariableDeclaration, StaticVariableDeclaration, LetVariableDeclaration,
	ModuleDeclaration, FunctionDeclaration, FunctionSelfParameterDeclaration, FunctionParameterDeclaration, FunctionSpread,
	StructDeclaration, TupleStructDeclaration, StructPropertyDeclaration, TupleStructItemDeclaration, UnionDeclaration, EnumDeclaration,
	EnumMemberDeclaration, EnumMemberTupleDeclaration, EnumMemberStructDeclaration, TraitDeclaration, AutoTraitDeclaration,
	TraitAliasDeclaration, ImplDeclaration, NegativeImplDeclaration, ExpressionTypeSelector, ExpressionTypeCast, ExpressionPath,
	ExpressionAsTypeCast, ReturnExpression, BreakExpression, ContinueExpression, YieldExpression, CallExpression, MemberExpression,
	AwaitExpression, UnwrapExpression, ParenthesizedExpression, MinusExpression, NotExpression, OrExpression, AndExpression,
	ReassignmentExpression, UnassignedExpression, OperationExpression, ReassignmentOperationExpression, ComparisonExpression, LetScrutinee,
	ClosureFunctionExpression, ClosureFunctionParameterDeclaration, BlockExpression, LoopBlockExpression, WhileBlockExpression,
	ForInBlockExpression, TryBlockExpression, IfBlockExpression, MatchExpression, MatchExpressionCase, RangeLiteral, StructLiteral,
	StructLiteralProperty, StructLiteralPropertyShorthand, StructLiteralPropertySpread, StructLiteralRestUnassigned, TupleLiteral,
	ArrayLiteral, SizedArrayLiteral, ReferenceExpression, RawReferenceExpression, DereferenceExpression, BoxExpression, UnionPattern,
	ParenthesizedPattern, RestPattern, WildcardPattern, PatternVariableDeclaration, StructPattern, StructPatternPropertyDestructured,
	StructPatternPropertyShorthand, TuplePattern, ArrayPattern, ReferencePattern, BoxPattern, MinusPattern, RangePattern, TypePath,
	TypeCall, TypeCallNamedArgument, TypeCallNamedBound, LtIdentifier, LtElided, LtStatic, TypeNever, TypeInferred,
	GenericTypeParameterDeclaration, ConstTypeParameterDeclaration, GenericLtParameterDeclaration, WhereTypeBoundDeclaration,
	WhereLtBoundDeclaration, TypeTraitBound, TypeDynBounds, TypeImplBounds, TypeFnPointer, TypeFnPointerParameter, TypeFunction, TypeTuple,
	TypeSizedArray, TypeSlice, TypeReference, TypeDereferenceConst, TypeDereferenceMut, TypeParenthesized
}
// prettier-ignore
export type Node = MissingNode | SourceFile | Shebang | Program | Snippet | Comment | Identifier | Index | LbIdentifier | McIdentifier |
	Literal | ItemPath | PunctuationToken | DelimGroup | Attribute | DocCommentAttribute | MacroInvocation | MacroRulesDeclaration |
	MacroDeclaration | MacroRuleDeclaration | MacroInlineRuleDeclaration | MacroGroup | MacroParameterDeclaration | PubSpecifier |
	ExternSpecifier | ExpressionStatement | UseStatement | NamedImport | AmbientImport | AnonymousImport | DestructuredImport |
	ExternCrateStatement | ExternBlockDeclaration | TypeAliasDeclaration | ConstVariableDeclaration | StaticVariableDeclaration |
	LetVariableDeclaration | ModuleDeclaration | FunctionDeclaration | FunctionSelfParameterDeclaration | FunctionParameterDeclaration |
	FunctionSpread | StructDeclaration | TupleStructDeclaration | StructPropertyDeclaration | TupleStructItemDeclaration | UnionDeclaration |
	EnumDeclaration | EnumMemberDeclaration | EnumMemberTupleDeclaration | EnumMemberStructDeclaration | TraitDeclaration |
	AutoTraitDeclaration | TraitAliasDeclaration | ImplDeclaration | NegativeImplDeclaration | ExpressionTypeSelector | ExpressionTypeCast |
	ExpressionPath | ExpressionAsTypeCast | ReturnExpression | BreakExpression | ContinueExpression | YieldExpression | CallExpression |
	MemberExpression | AwaitExpression | UnwrapExpression | ParenthesizedExpression | MinusExpression | NotExpression | OrExpression |
	AndExpression | ReassignmentExpression | UnassignedExpression | OperationExpression | ReassignmentOperationExpression |
	ComparisonExpression | LetScrutinee | ClosureFunctionExpression | ClosureFunctionParameterDeclaration | BlockExpression |
	LoopBlockExpression | WhileBlockExpression | ForInBlockExpression | TryBlockExpression | IfBlockExpression | MatchExpression |
	MatchExpressionCase | RangeLiteral | StructLiteral | StructLiteralProperty | StructLiteralPropertyShorthand |
	StructLiteralPropertySpread | StructLiteralRestUnassigned | TupleLiteral | ArrayLiteral | SizedArrayLiteral | ReferenceExpression |
	RawReferenceExpression | DereferenceExpression | BoxExpression | UnionPattern | ParenthesizedPattern | RestPattern | WildcardPattern |
	PatternVariableDeclaration | StructPattern | StructPatternPropertyDestructured | StructPatternPropertyShorthand | TuplePattern |
	ArrayPattern | ReferencePattern | BoxPattern | MinusPattern | RangePattern | TypePath | TypeCall | TypeCallNamedArgument |
	TypeCallNamedBound | LtIdentifier | LtElided | LtStatic | TypeNever | TypeInferred | GenericTypeParameterDeclaration |
	ConstTypeParameterDeclaration | GenericLtParameterDeclaration | WhereTypeBoundDeclaration | WhereLtBoundDeclaration | TypeTraitBound |
	TypeDynBounds | TypeImplBounds | TypeFnPointer | TypeFnPointerParameter | TypeFunction | TypeTuple | TypeSizedArray | TypeSlice |
	TypeReference | TypeDereferenceConst | TypeDereferenceMut | TypeParenthesized;
// prettier-ignore
export interface NTMap {
	0: MissingNode; 1: SourceFile; 2: Shebang; 3: Program; 4: Snippet; 5: Comment; 6: Identifier; 7: Index; 8: LbIdentifier;
	9: McIdentifier; 10: Literal; 11: ItemPath; 12: PunctuationToken; 13: DelimGroup; 14: Attribute; 15: DocCommentAttribute;
	16: MacroInvocation; 17: MacroRulesDeclaration; 18: MacroDeclaration; 19: MacroRuleDeclaration; 20: MacroInlineRuleDeclaration;
	21: MacroGroup; 22: MacroParameterDeclaration; 23: PubSpecifier; 24: ExternSpecifier; 25: ExpressionStatement; 26: UseStatement;
	27: NamedImport; 28: AmbientImport; 29: AnonymousImport; 30: DestructuredImport; 31: ExternCrateStatement; 32: ExternBlockDeclaration;
	33: TypeAliasDeclaration; 34: ConstVariableDeclaration; 35: StaticVariableDeclaration; 36: LetVariableDeclaration;
	37: ModuleDeclaration; 38: FunctionDeclaration; 39: FunctionSelfParameterDeclaration; 40: FunctionParameterDeclaration;
	41: FunctionSpread; 42: StructDeclaration; 43: TupleStructDeclaration; 44: StructPropertyDeclaration; 45: TupleStructItemDeclaration;
	46: UnionDeclaration; 47: EnumDeclaration; 48: EnumMemberDeclaration; 49: EnumMemberTupleDeclaration; 50: EnumMemberStructDeclaration;
	51: TraitDeclaration; 52: AutoTraitDeclaration; 53: TraitAliasDeclaration; 54: ImplDeclaration; 55: NegativeImplDeclaration;
	56: ExpressionTypeSelector; 57: ExpressionTypeCast; 58: ExpressionPath; 59: ExpressionAsTypeCast; 60: ReturnExpression;
	61: BreakExpression; 62: ContinueExpression; 63: YieldExpression; 64: CallExpression; 65: MemberExpression; 66: AwaitExpression;
	67: UnwrapExpression; 68: ParenthesizedExpression; 69: MinusExpression; 70: NotExpression; 71: OrExpression; 72: AndExpression;
	73: ReassignmentExpression; 74: UnassignedExpression; 75: OperationExpression; 76: ReassignmentOperationExpression;
	77: ComparisonExpression; 78: LetScrutinee; 79: ClosureFunctionExpression; 80: ClosureFunctionParameterDeclaration;
	81: BlockExpression; 82: LoopBlockExpression; 83: WhileBlockExpression; 84: ForInBlockExpression; 85: TryBlockExpression;
	86: IfBlockExpression; 87: MatchExpression; 88: MatchExpressionCase; 89: RangeLiteral; 90: StructLiteral; 91: StructLiteralProperty;
	92: StructLiteralPropertyShorthand; 93: StructLiteralPropertySpread; 94: StructLiteralRestUnassigned; 95: TupleLiteral;
	96: ArrayLiteral; 97: SizedArrayLiteral; 98: ReferenceExpression; 99: RawReferenceExpression; 100: DereferenceExpression;
	101: BoxExpression; 102: UnionPattern; 103: ParenthesizedPattern; 104: RestPattern; 105: WildcardPattern;
	106: PatternVariableDeclaration; 107: StructPattern; 108: StructPatternPropertyDestructured; 109: StructPatternPropertyShorthand;
	110: TuplePattern; 111: ArrayPattern; 112: ReferencePattern; 113: BoxPattern; 114: MinusPattern; 115: RangePattern; 116: TypePath;
	117: TypeCall; 118: TypeCallNamedArgument; 119: TypeCallNamedBound; 120: LtIdentifier; 121: LtElided; 122: LtStatic; 123: TypeNever;
	124: TypeInferred; 125: GenericTypeParameterDeclaration; 126: ConstTypeParameterDeclaration; 127: GenericLtParameterDeclaration;
	128: WhereTypeBoundDeclaration; 129: WhereLtBoundDeclaration; 130: TypeTraitBound; 131: TypeDynBounds; 132: TypeImplBounds;
	133: TypeFnPointer; 134: TypeFnPointerParameter; 135: TypeFunction; 136: TypeTuple; 137: TypeSizedArray; 138: TypeSlice;
	139: TypeReference; 140: TypeDereferenceConst; 141: TypeDereferenceMut; 142: TypeParenthesized;
}
// </generated>

//#---------------------------------------------------+        Feature Gated        +-------------------------------------------------------.

// prettier-ignore
export const enum Feature {
	/** `crate ...`										*/ crate_visibility_modifier = "crate_visibility_modifier",

	/** `macro foo(...) { ... }` 						*/ decl_macro = "decl_macro",
	
	/** `impl<...> const ... for ...` | `~const ...` 	*/ const_trait_impl = "const_trait_impl",
	/** `impl<...> 		!... for ... {}` 				*/ negative_impls = "negative_impls",
	
	/** `auto trait ... {}` 							*/ auto_traits = "auto_traits",
	/** `trait ... = ...` 								*/ trait_alias = "trait_alias",

	/** `enum ... { Foo(...) = discriminant }` 			*/ arbitrary_enum_discriminant = "arbitrary_enum_discriminant",
	
	/** `<const ... = ...>` 							*/ const_generics_defaults = "const_generics_defaults",
	/** `Foo<...: typeBounds>` 							*/ associated_type_bounds = "associated_type_bounds",

	/** `static || { ... }` | `yield ...` 				*/ generators = "generators",
	/** `async || { ... }` 								*/ async_closure = "async_closure",

	/** `try { ... }` 									*/ try_blocks = "try_blocks",
	
	/** `min..max` 										*/ exclusive_range_pattern = "exclusive_range_pattern",
	/** `..max` | `..=max` | `...max` 					*/ half_open_range_patterns = "half_open_range_patterns",
	
	/** `Color { r: _, g, .. } = ...` 					*/ destructuring_assignment = "destructuring_assignment",
	
	/** `let ... = ... else { ... }` 					*/ let_else = "let_else",
	/** `let ... && let ...` 							*/ let_chains = "let_chains",
	/** `match ... { ... if let ... = ... => ... }` 	*/ if_let_guard = "if_let_guard",

	/** `&raw mut ...` | `&raw const ...` 				*/ raw_ref_op = "raw_ref_op",

	/** `const { ... }`									*/ inline_const = "inline_const",
	/** `const { ... }`									*/ inline_const_pat = "inline_const_pat",

	/** `box ...`										*/ box_patterns = "box_patterns",
	/** `box ...` 										*/ box_syntax = "box_syntax",
}

// compiler\rustc_feature\src\active.rs

// specialization = "specialization",
// const_extern_fn = "const_extern_fn",
// ellipsis_inclusive_range_patterns = "ellipsis_inclusive_range_patterns",
// illegal_floating_point_literal_pattern = "illegal_floating_point_literal_pattern",
// more_qualified_paths = "more_qualified_paths",
// type_ascription = "type_ascription",

export interface FG_Map {
	[Feature.crate_visibility_modifier]: PubSpecifier;

	[Feature.decl_macro]: MacroDeclaration | MacroInlineRuleDeclaration;

	[Feature.const_trait_impl]: TypeTraitBound | ImplDeclaration;
	[Feature.negative_impls]: NegativeImplDeclaration;

	[Feature.auto_traits]: AutoTraitDeclaration;
	[Feature.trait_alias]: TraitAliasDeclaration;

	[Feature.arbitrary_enum_discriminant]: EnumMemberStructDeclaration | EnumMemberTupleDeclaration;

	[Feature.const_generics_defaults]: ConstTypeParameterDeclaration;
	[Feature.associated_type_bounds]: TypeCallNamedBound;

	[Feature.generators]: YieldExpression | ClosureFunctionExpression;
	[Feature.async_closure]: ClosureFunctionExpression;

	[Feature.try_blocks]: TryBlockExpression;

	[Feature.exclusive_range_pattern]: RangePattern;
	[Feature.half_open_range_patterns]: RangePattern;

	[Feature.destructuring_assignment]: ReassignmentExpression | StructLiteralRestUnassigned | UnassignedExpression;

	[Feature.let_else]: LetVariableDeclaration;
	[Feature.let_chains]: NodeWithCondition;
	[Feature.if_let_guard]: MatchExpressionCase;

	[Feature.raw_ref_op]: RawReferenceExpression;

	[Feature.inline_const]: BlockExpression;
	[Feature.inline_const_pat]: BlockExpression;

	[Feature.box_patterns]: BoxPattern | StructPatternPropertyShorthand;
	[Feature.box_syntax]: BoxExpression;
}

export const enum PRCD {
	ScrutineeDefault,
	"Scrutinee ||",
	"Scrutinee &&",
	Default,
	"=",
	"..",
	"||",
	"&&",
	"==", // different in Javascript
	"|",
	"^",
	"&",
	// Javascript:
	// ["==", "!="],
	// ["<", ">", "<=", ">="],
	">>",
	"+-",
	"*/%",
	"as",
	Unary,
	Top,
}

//#--------------------------------------------------+        Contextual Types        +-----------------------------------------------------.

export type missing = MissingNode;

export type Segment = AttrSegment | MacroInvokeSegment | MacroMatchSegment | MacroTransformSegment;
export type AttrSegment = TokenNode | DelimGroup<AttrSegment>;
export type MacroInvokeSegment = TokenNode | DelimGroup<MacroInvokeSegment>;
export type MacroMatchSegment = TokenNode | DelimGroup<MacroMatchSegment> | MacroGroup<MacroMatchSegment> | MacroParameterDeclaration;
export type MacroTransformSegment = TokenNode | DelimGroup<MacroTransformSegment> | MacroGroup<MacroTransformSegment> | McIdentifier;
/**/ export type TokenNode = PunctuationToken | Literal | Identifier | LtIdentifier;
/**/ export type MacroSeparator = TokenNode;

export type ImportNode =
	| NamedImport //
	| AmbientImport
	| AnonymousImport
	| DestructuredImport;

export type StatementNode =
	// expressions
	| ExpressionStatement
	// macro
	| MacroRulesDeclaration
	| MacroDeclaration
	// import
	| UseStatement
	// extern
	| ExternCrateStatement
	| ExternBlockDeclaration
	// mod
	| ModuleDeclaration
	// var
	| TypeAliasDeclaration
	| LetVariableDeclaration
	| ConstVariableDeclaration
	| StaticVariableDeclaration
	// function
	| FunctionDeclaration
	// struct
	| StructDeclaration
	| TupleStructDeclaration
	// trait
	| TraitDeclaration
	| AutoTraitDeclaration
	| TraitAliasDeclaration
	// impl
	| ImplDeclaration
	| NegativeImplDeclaration
	// enum
	| EnumDeclaration
	| UnionDeclaration;

export type ExpressionNode =
	// common
	| MacroInvocation
	| Identifier
	| Literal
	// path
	| ExpressionPath
	| ExpressionTypeCast
	| ExpressionTypeSelector
	| ExpressionAsTypeCast
	// flow control
	| ReturnExpression
	| BreakExpression
	| ContinueExpression
	| YieldExpression
	// derivative
	| CallExpression
	| MemberExpression
	| MemberExpression
	| AwaitExpression
	| UnwrapExpression
	// unary
	| MinusExpression
	| NotExpression
	// closure
	| ClosureFunctionExpression
	// block like
	| BlockExpression
	| LoopBlockExpression
	| WhileBlockExpression
	| ForInBlockExpression
	| TryBlockExpression
	| IfBlockExpression
	| MatchExpression
	// op like
	| OrExpression<ExpressionNode, ExpressionNode>
	| AndExpression<ExpressionNode, ExpressionNode>
	| ReassignmentExpression
	| OperationExpression
	| ReassignmentOperationExpression
	| ComparisonExpression
	// literal
	| RangeLiteral
	| StructLiteral
	| TupleLiteral
	| ArrayLiteral
	| SizedArrayLiteral
	// memory
	| ReferenceExpression
	| RawReferenceExpression
	| DereferenceExpression
	| BoxExpression
	// misc
	| ParenthesizedExpression<ExpressionNode>;

export type PatternNode =
	// common
	| MacroInvocation
	| Identifier
	| Literal
	// path
	| ExpressionPath
	| ExpressionTypeCast
	| ExpressionTypeSelector
	// value
	| MinusPattern<Literal>
	| RangePattern
	| BlockExpression // const { ... } #[feature(inline_const_pat)]
	// variable
	| PatternVariableDeclaration
	// destructuring
	| StructPattern
	| TuplePattern
	| ArrayPattern
	// wildcard
	| RestPattern
	| WildcardPattern
	// memory
	| ReferencePattern
	| BoxPattern
	// misc
	| UnionPattern
	| ParenthesizedPattern<PatternNode>;

export type TypeNode =
	// common
	| MacroInvocation
	| Identifier
	// path
	| TypePath
	| TypeCall
	| ExpressionTypeSelector
	// special
	| TypeNever
	| TypeInferred
	// bounds
	| TypeDynBounds
	| TypeImplBounds
	// functions
	| TypeFnPointer
	| TypeFunction
	// literals
	| TypeSizedArray
	| TypeSlice
	| TypeTuple
	// memory
	| TypeReference
	| TypeDereferenceMut
	| TypeDereferenceConst
	// misc
	| TypeParenthesized<TypeNode>;

//#------------------------------------------------+        Contextual Subtypes        +----------------------------------------------------.

export type AttributeOrComment = Comment | Attribute | DocCommentAttribute;
export type AttributeOrDocComment = Attribute | DocCommentAttribute;
export type CommentOrDocComment = Comment | DocCommentAttribute;

export type IdentifierOrIndex = Identifier | Index;
export type IdentifierOrItemPath = Identifier | ItemPath;

//#--------------------------------------------------+        Rust Ref aliases        +-----------------------------------------------------.

/**/ export type _PathBase = Identifier | ExpressionTypeSelector;

/** What can be on the left of 'PathExprSegment' */
/**/ export type _ExprPathSource<T extends _PathBase> = T | ExpressionPath<T> | ExpressionTypeCast<T>;
/** What can be on the left of 'TypePathSegment' */
/**/ export type _TypePathSource<T extends _PathBase> = T | TypePath<T> | TypeCall<T> | TypeFunction<T>;

/** 'PathInExpression' */
export type ExpressionNamespaceTargetNoSelector = _ExprPathSource<Identifier>;
/** 'TypePath' */
export type TypeNamespaceTargetNoSelector = _TypePathSource<Identifier>;

/** 'PathExpression' === ('PathInExpression' | 'QualifiedPathInExpression')  === 'PathPattern' */
export type ExpressionNamespaceTarget = _ExprPathSource<_PathBase>;
/** 'TypePath' | 'QualifiedTypeInExpression' */
export type TypeNamespaceTarget = _TypePathSource<_PathBase>;

/** 'PatternNoTopAlt */
export type PatternNoUnion = Exclude<PatternNode, UnionPattern>;
/** 'PatternWithoutRange' */
export type PatternNoUnionNoRange = Exclude<PatternNoUnion, RangePattern>;

//#----------------------------------------------------+        Type helpers        +-------------------------------------------------------.

export type MaybePubNode = Extract<Node, PubTarget>;
export type MaybeExternNode = Extract<Node, ExternTarget>;
export type MaybeConstNode = Extract<Node, ConstModifier>;
export type MaybeAsyncNode = Extract<Node, AsyncModifier>;
export type MaybeMoveNode = Extract<Node, MoveModifier>;
export type MaybeUnsafeNode = Extract<Node, UnsafeModifier>;
export type MaybeStaticNode = Extract<Node, StaticModifier>;

export type PathNode = ItemPath | ExpressionPath | TypePath;
export type RangeNode = RangeLiteral | RangePattern;
export type FunctionNode = FunctionDeclaration | ClosureFunctionExpression;
export type TypeFunctionNode = TypeFunction | TypeFnPointer;
export type ParenthesizedNode = ParenthesizedExpression | ParenthesizedPattern | TypeParenthesized;
export type ObjectNode = Extract<Node, MaybeObjectLike>;
export type ArrayLikeNode = Extract<Node, ArrayLike>;
export type ArrayOrTupleLiteral = ArrayLiteral | TupleLiteral;
export type TupleNode = TupleStructDeclaration | EnumMemberTupleDeclaration | TupleLiteral | TuplePattern | TypeTuple;
__DEV__: AssertTypesEq<TupleNode, Extract<Node, TupleLike>>();

export type DeclarationNode = Extract<Node, DeclarationLike>;
export type TraitDeclarationNode = TraitDeclaration | AutoTraitDeclaration | TraitAliasDeclaration;
export type ImplDeclarationNode = ImplDeclaration | NegativeImplDeclaration;

export type NodeWithSegments = Extract<Node, DelimitedSequence<any, any>>;

export type NodeWithBody = Extract<Node, MaybeBlockBody>;
export type NodeWithBodyOrCases = NodeWithBody | MatchExpression;
export type NodeWithBodyNoBody = AutoTraitDeclaration | NegativeImplDeclaration;
export type NodeWithCondition = Extract<Node, MaybeConditionBody>;

export type ImplicitReturnAbleNode = FunctionDeclaration | BlockExpression | IfBlockExpression | MatchExpression | TryBlockExpression;

export type ExpressionWithBody = Extract<ExpressionNode, MaybeBlockBody>;
export type ExpressionWithBodyOrCases = ExpressionWithBody | MatchExpression;

export type LogicalExpression = OrExpression | AndExpression;
export type LeftRightExpression = Extract<Node, LeftRightLike>;
export type FlowControlExpression = ReturnExpression | ContinueExpression | BreakExpression | YieldExpression;

// prettier-ignore
export type UnaryExpression = NotExpression | MinusExpression | ReferenceExpression | DereferenceExpression | RawReferenceExpression | BoxExpression;
export type UnaryPattern = ReferencePattern | BoxPattern | MinusPattern;
export type UnaryType = TypeReference | TypeDereferenceMut | TypeDereferenceConst;

export type PostfixExpression = UnwrapExpression | AwaitExpression;

export type VariableDeclarationNode = LetScrutinee | LetVariableDeclaration | ConstVariableDeclaration | StaticVariableDeclaration;
export type ReassignmentNode = ReassignmentExpression | ReassignmentOperationExpression;

export type NodeWithTypeBounds = Extract<Node, TypeBoundsConstaint>;
export type TypeBoundsStandaloneNode = Extract<TypeNode, TypeBoundsStandalone>;

export type FunctionLikeNode = Extract<Node, FunctionLike>;
export type FunctionParameterNode =
	| FunctionSelfParameterDeclaration
	| FunctionParameterDeclaration
	| ClosureFunctionParameterDeclaration
	| TypeFnPointerParameter
	| TypeNode
	| FunctionSpread;

__DEV__: AssertTypesEq<FunctionParameterNode, FunctionLikeNode["parameters"][number] | FunctionSelfParameterDeclaration>();
// prettier-ignore
export type NodeWithMaybePatternNoUnionBody = LetVariableDeclaration | ConstVariableDeclaration | StaticVariableDeclaration |
	FunctionParameterDeclaration | ClosureFunctionParameterDeclaration | PatternVariableDeclaration | UnaryPattern;
__DEV__: AssertTypesEq<NodeWithMaybePatternNoUnionBody, Extract<Node, MaybePatternNoUnionBody>>();

//#endregion ===============================================================================================================================..--'

//#region ==============================================[        Interfaces        ]========================================================``--.

//#--------------------------------------------------------+        Loc        +------------------------------------------------------------.

// prettier-ignore
export interface Located { loc: Loc; }
// prettier-ignore
export interface Delimited<K extends keyof DelimKindMap = DelimKind> { dk: DelimKindMap[K]; }
// prettier-ignore
export interface LocArray<T extends Node = Node, K extends keyof DelimKindMap = keyof DelimKindMap> extends Array<T>, Located, Delimited<K> {}
// prettier-ignore
export interface DelimitedSequence<T extends Segment, TK extends keyof DelimKindMap = "()" | "[]" | "{}"> { segments: LocArray<T, TK>; }

//#------------------------------------------------------+        Program        +----------------------------------------------------------.

// prettier-ignore
export interface __DevonlyObject { stats: { [statsName: string]: string | number }; }

export interface ProgramLike {
	ast: Node | LocArray<any, any>;
	danglingAttributes: AttributeOrDocComment[];
	comments: Comment[];
	__devonly?: __DevonlyObject;
}

//#-----------------------------------------------------+        Identifier        +--------------------------------------------------------.

export interface IdentifierLike {
	name: string;
}
export interface MaybeIdentifiable<T extends IdentifierLike = Identifier> {
	id: T | undefined;
}
export interface Identifiable<T extends IdentifierLike = Identifier> extends MaybeIdentifiable<T> {
	id: T;
}
export interface LtIdentifiable {
	id: LtIdentifier;
}
export interface PathLike {
	namespace: Identifier | unknown | undefined;
	segment: Identifier;
}

//#------------------------------------------------------+        Optional        +---------------------------------------------------------.

export interface AttributeTarget {
	attributes?: AttributeOrDocComment[];
}
export interface PubTarget {
	pub?: PubSpecifier;
}
export interface ExternTarget {
	extern?: ExternSpecifier;
}
export interface ConstModifier {
	const?: true;
}
export interface AsyncModifier {
	async?: true;
}
export interface MoveModifier {
	move?: true;
}
export interface UnsafeModifier {
	unsafe?: true;
}
export interface StaticModifier {
	static?: true;
}

//#-------------------------------------------------------+        Memory        +----------------------------------------------------------.

// prettier-ignore
export interface RefAble { ref: boolean; }
// prettier-ignore
export interface MutAble { mut: boolean; }
// prettier-ignore
export interface LtAble { lt: Lifetime | undefined; }
// prettier-ignore
export interface TargetsLifetime { ltTarget: Lifetime; }
// prettier-ignore
export interface MaybeHasLtBounds { ltBounds: Lifetime[] | undefined; }
// prettier-ignore
export interface HasLtBounds extends MaybeHasLtBounds { ltBounds: Lifetime[]; }

export interface ForLtParametersBody {
	// for<...ltParameters>? ...
	ltParameters: LocArray<GenericLtParameterDeclaration, "<>"> | undefined;
}

//#-------------------------------------------------------+        Common        +----------------------------------------------------------.

export interface CommentLike {
	line: boolean;
	value: string;
}
export interface AttributeLike extends DelimitedSequence<AttrSegment, "None" | "[]"> {
	inner: boolean;
	segments: LocArray<AttrSegment, "None" | "[]">;
}
export interface AbiTarget {
	abi: Literal | undefined;
}

//#-------------------------------------------------+        Expression Context        +----------------------------------------------------.

export interface Callable {
	callee: Node;
	arguments: LocArray<Node, "()">;
}

export interface MaybeLabelTarget {
	label: LbIdentifier | undefined;
}

export interface MaybeBlockBody {
	body: LocArray<StatementNode, "None" | "{}"> | undefined;
}
export interface BlockBody extends MaybeBlockBody {
	body: LocArray<StatementNode, "None" | "{}">;
}
export interface BlockLike extends MaybeLabelTarget, BlockBody {}

export interface MaybeConditionBody {
	condition: ConditionExpressionXS | undefined;
}
export interface ConditionBody extends MaybeConditionBody {
	condition: ConditionExpressionXS;
}

export interface MaybeExpressionBody {
	expression: ExpressionNode | undefined;
}
export interface ExpressionBody extends MaybeExpressionBody {
	expression: ExpressionNode;
}

export interface LeftRightLike {
	kind: keyof typeof TK;
	left: Node | undefined;
	tk: TK;
	right: Node | undefined;
}
export interface RangeLike {
	lower: Node | undefined;
	last: boolean;
	upper: Node | undefined;
}

export interface SizeExpressionBody {
	sizeExpression: ExpressionNode;
}

export interface ArrayLike {
	items: LocArray<Node, "()" | "[]">;
}
export interface TupleLike extends ArrayLike {
	items: LocArray<Node, "()">;
}

export interface MaybeObjectLike {
	properties: LocArray<Node, "{}"> | undefined;
}
export interface ObjectLike extends MaybeObjectLike {
	properties: LocArray<Node, "{}">;
}

// prettier-ignore
export interface PropertyLike { key: IdentifierOrIndex; }

//#----------------------------------------------------+        Type Context        +-------------------------------------------------------.

export interface MaybeReturnTypeConstraint {
	returnType: TypeNode | undefined;
}

export interface FunctionLike extends MaybeReturnTypeConstraint {
	// (...parameters) -> returnType?
	// |...parameters| -> returnType?
	parameters: LocArray<Node, "()" | "||">;
	returnType: TypeNode | undefined;
}

export interface MaybeGenericArgsTarget {
	typeArguments: LocArray<TypeCallArgument, "<>"> | undefined;
}

export interface GenericArgsTarget extends MaybeGenericArgsTarget {
	// ...::<...typeArguments>
	//   ...<...typeArguments>
	typeArguments: LocArray<TypeCallArgument, "<>">;
}

export interface TypeCallable<T = TypeNode> extends GenericArgsTarget {
	// typeCallee<...typeArguments>
	typeCallee: T;
}

/**/ export interface MaybeGenericsDeclaration {
	// <...generics>?
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
}

/**/ export interface MaybeWhereBoundsDeclaration {
	// where ...whereBounds?
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
}

export interface DeclarationLike extends MaybeWhereBoundsDeclaration, MaybeWhereBoundsDeclaration {
	// ...<...generics>? ... where ...whereBounds?
}
export interface TypeTargetConstraint {
	typeTarget: TypeNode;
}

export interface MaybeTypeExpressionBody<T = TypeNode> {
	typeExpression: T | undefined;
}
export interface TypeExpressionBody<T = TypeNode> extends MaybeTypeExpressionBody<T> {
	typeExpression: T;
}

export interface MaybeTypeAnnotationTarget<T = TypeNode> {
	typeAnnotation: T | undefined;
}
export interface TypeAnnotationTarget<T = TypeNode> extends MaybeTypeAnnotationTarget<T> {
	typeAnnotation: T;
}

export interface TypeBoundsConstaint {
	// ...: ...typeBounds?
	// ...= ...typeBounds?
	typeBounds: TypeBound[] | undefined; // allows trailing "+", allows empty value
}

export interface TypeBoundsStandalone extends TypeBoundsConstaint {
	typeBounds: TypeBound[]; // allows trailing "+"
}

//#--------------------------------------------------+        Pattern Context        +------------------------------------------------------.

export interface MaybePatternBody {
	pattern: PatternNode | undefined;
}
export interface PatternBody extends MaybePatternBody {
	pattern: PatternNode;
}
export interface MaybePatternNoUnionBody extends MaybePatternBody {
	pattern: PatternNoUnion | undefined;
}
export interface PatternNoUnionBody extends MaybePatternBody {
	pattern: PatternNoUnion;
}

//#endregion ===============================================================================================================================..--'

//#region ================================================[        Enums        ]===========================================================``--.

// prettier-ignore
export const enum TK { None, ".", "&&", "||", "=", "+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>", "==", "!=", ">", ">=", "<", "<=", "+=", 
"-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "$", "@", "_", "..", "...", "..=", ",", ";", ":", "::", "#", "?", "!", "=>", "->", "~" }
// prettier-ignore
const str_TK = ["", ".", "&&", "||", "=", "+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>", "==", "!=", ">", ">=", "<", "<=", "+=", "-=", 
"*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "$", "@", "_", "..", "...", "..=", ",", ";", ":", "::", "#", "?", "!", "=>", "->", "~"] as const;
// prettier-ignore
type ReaOpExpr = typeof TK["+="] | typeof TK["-="] | typeof TK["*="] | typeof TK["/="] | typeof TK["%="] | typeof TK["&="] | typeof TK["|="] | typeof TK["^="] | typeof TK["<<="] | typeof TK[">>="];
// prettier-ignore
type OpExpr = typeof TK["+"] | typeof TK["-"] | typeof TK["*"] | typeof TK["/"] | typeof TK["%"] | typeof TK["&"] | typeof TK["|"] | typeof TK["^"] | typeof TK["<<"] | typeof TK[">>"];
type CompExpr = typeof TK["=="] | typeof TK["!="] | typeof TK[">"] | typeof TK[">="] | typeof TK["<"] | typeof TK["<="];
// prettier-ignore
/**/ export interface DelimKindMap { "None": 0; "()": 1; "[]": 2; "{}": 3; "<>": 4; "||": 5; 0: 0; 1: 1; 2: 2; 3: 3; 4: 4; 5: 5; }
// prettier-ignore
/**/ export interface DelimNameMap { 0: "None"; 1: "()"; 2: "[]"; 3: "{}"; 4: "<>"; 5: "||"; }
// prettier-ignore
/**/ export const enum DelimKind { None, "()", "[]", "{}", "<>", "||" }

export const enum TyMacroMatch {
	/** undefined										*/ None,
	/** anything										*/ "tt",
	/** Attribute										*/ "meta",
	/** PubSpecifier									*/ "vis",
	/** Identifier										*/ "ident",
	/** LtIdentifier									*/ "lifetime",
	/** Literal | MinusExpression<Literal>				*/ "literal",
	/** StatementNode									*/ "stmt",
	/** StatementNode subset valid in Toplevel			*/ "item",
	/** ExpressionNode									*/ "expr",
	/** ExpressionNode & BlockLike						*/ "block",
	/** PatternNode (PatternNoUnion pre edition 2021)	*/ "pat",
	/** PatternNoUnion									*/ "pat_param",
	/** TypeNode										*/ "ty",
	/** TypeNamespaceTargetNoSelector					*/ "path",
}

export const enum LiteralKind {
	/** i.e. `false`						*/ False,
	/** i.e. `true`							*/ True,
	/** e.g. `'A'`							*/ Char,
	/** e.g. `b'A'`							*/ bChar,
	/** e.g. `b"ABCDE"`						*/ bString,
	/** e.g. `br#"AAAA"#` 					*/ brString,
	/** e.g. `r#"hello"#` 					*/ rString,
	/** e.g. `"hello"` 						*/ String,
	/** e.g. `0b1111_0000` 	(int suffix) 	*/ Binary,
	/** e.g. `0xff` 		(int suffix)	*/ Hex,
	/** e.g. `0o77` 		(int suffix)	*/ Octal,
	/** e.g. `123` 			(int suffix)	*/ Integer,
	/** e.g. `7.6e+54` 		(float suffix) 	*/ Float,
}

//#endregion ===============================================================================================================================..--'

//#region =================================================[        Base        ]===========================================================``--.

//#--------------------------------------------------------+        Loc        +------------------------------------------------------------.

// prettier-ignore
export class Loc {
	declare readonly src: SourceFile;
	protected declare readonly 0: number;
	protected declare readonly 1: number;
	protected declare readonly 2?: number;

	constructor(src: SourceFile, start: number, end: number) {
		this.src = src;
		this[0] = start;
		this[1] = end;
	}

	len() { return this[1] - this[0]; }

	isBefore(target: Located) { return this[1] <= start(target); }
	isAfter(target: Located) { return this[0] >= end(target); }
	contains(target: Located) { return this[0] <= start(target) && this[1] >= end(target); }
	ownContains(target: Located) { return (2 in this ? this[2]! : this[0]) <= start(target) && this[1] >= end(target); }
	isBetween(left: Located, right: Located) { return this[0] >= end(left) && this[1] <= start(right); }

	url() { return this.src.url(this[0]); }
	
	getText() { return this.src.code.slice(this[0], this[1]); }
	getOwnText() { return this.src.code.slice(2 in this ? this[2]! : this[0], this[1]); }
	sliceText(startIndex?: number, endIndex?: number) { return this.src.code.slice(this[0], this[1]).slice(startIndex, endIndex); }

	clone() { return new Loc(this.src, 2 in this ? this[2]! : this[0], this[1]); }
	cloneFrom(startPos: number) { return new Loc(this.src, startPos, this[1]); }

	[Symbol.for("nodejs.util.inspect.custom")]() { return this.url(); }
	toJSON() { const { src, ...rest } = this; return rest; }
}

//#------------------------------------------------------+        BaseNode        +---------------------------------------------------------.

// prettier-ignore
export class BaseNode<NT extends NodeType = NodeType> implements Located {
	get type() { return NodeType[this.nodeType]; }
	declare readonly nodeType: NT;
	declare readonly loc: Loc;
	constructor(nodeType: NT, loc: Loc) { this.nodeType = nodeType; this.loc = loc; }
	toJSON() { 
		var obj: any = { type: NodeType[this.nodeType], ...(this as any) };
		if ("name" in this) obj.name = (this as any).name;
		if ("value" in this) obj.value = (this as any).value;
		if ("kind" in this) obj.kind = (this as any).kind;
		if ("token" in this) obj.token = (this as any).token;
		return obj;
	}
}

// prettier-ignore
export class MissingNode extends BaseNode<0> implements IdentifierLike {
	get name(): "" { return ""; }
}

//#-----------------------------------------------------+        SourceFile        +--------------------------------------------------------.

// prettier-ignore
export class SourceFile extends BaseNode<1> {
	code: string;
	filepath: string | undefined;

	parserOptions: ParserOptions;
	
	lineStarts: number[];
	program: Program;

	declare UTF8BOM?: true;
	declare shebang?: Shebang;
	l(index: number) { return getLineIndex(this.lineStarts, index); }
	lc(index: number) { return getLineChar(this.lineStarts, index); }
	url(index?: number) { return urlAt(this.filepath ?? "undefined", this.lineStarts, index); }
}

// prettier-ignore
/**/ export class Shebang extends BaseNode<2> {
	// #!value (e.g. "#!/usr/bin/env rustx")
	get value() { return this.loc.sliceText(2).trim(); }
}

//#------------------------------------------------------+        Program        +----------------------------------------------------------.

// const enum SnippetContext { Statements, Arguments, Expression, Type, Pattern, }
// prettier-ignore
export class Program extends BaseNode<3> implements ProgramLike, BlockBody, AttributeTarget {
	// ...body
	get body() { return this.ast; }
	ast: LocArray<StatementNode, "None">;
	danglingAttributes: AttributeOrDocComment[];
	comments: Comment[];
	declare attributes?: AttributeOrDocComment[];
	declare __devonly?: __DevonlyObject;
}

// prettier-ignore
export class Snippet<T extends Node | LocArray = Node | LocArray> extends BaseNode<4> implements ProgramLike, AttributeTarget {
	ast: T;
	danglingAttributes: AttributeOrDocComment[];
	comments: Comment[];
	declare attributes?: AttributeOrDocComment[]; // only when reading statements
	declare __devonly?: __DevonlyObject;
}

//#endregion ===============================================================================================================================..--'

//#region ===============================================[        Comments        ]=========================================================``--.

// prettier-ignore
export class Comment extends BaseNode<5> implements CommentLike {
	line: boolean;
	get value() { return this.loc.sliceText(2, this.line ? undefined : -2); }
}

//#endregion ===============================================================================================================================..--'

//#region =========================================[        Identifier & Literal        ]===================================================``--.

// prettier-ignore
export class Identifier extends BaseNode<6> implements IdentifierLike {
	get name() { return this.loc.getOwnText() as string | `r#${string}`; }
}

// prettier-ignore
export class Index extends BaseNode<7> implements IdentifierLike {
	get name() { return this.loc.getOwnText() as `${number}`; }
}

// prettier-ignore
export class LbIdentifier extends BaseNode<8> implements IdentifierLike {
	get name() { return this.loc.getOwnText() as `'${string}`; }
}

// prettier-ignore
export class McIdentifier extends BaseNode<9> implements IdentifierLike {
	get name() { return this.loc.getOwnText() as `$${string}`; }
}

// prettier-ignore
export class Literal extends BaseNode<10> {
	kind: LiteralKind;
	get value() { return "suffix" in this ? this.loc.src.code.slice(ownStart(this), start(this.suffix)) : this.loc.getOwnText(); }
	declare suffix?: Identifier;
}

//#endregion ===============================================================================================================================..--'

//#region ================================================[        Macro        ]===========================================================``--.

//#-------------------------------------------------------+        Common        +----------------------------------------------------------.

export class ItemPath extends BaseNode<11> implements PathLike {
	// namespace?::segment
	namespace: IdentifierOrItemPath | undefined;
	segment: Identifier;
}

// prettier-ignore
/**/ export class PunctuationToken extends BaseNode<12> {
	// token
	tk: TK;
	get token() { return str_TK[this.tk] as typeof str_TK[number]; }
}

/**/ export class DelimGroup<T extends Segment = Segment> extends BaseNode<13> implements DelimitedSequence<T> {
	segments: LocArray<T, "()" | "[]" | "{}">;
}

//#-----------------------------------------------------+        Invocation        +--------------------------------------------------------.

// prettier-ignore
export class Attribute extends BaseNode<14> implements AttributeLike, CommentLike {
	// #!?[expression] ...
	inner: boolean;
	segments: LocArray<AttrSegment, "[]">;
	get value() { return this.segments.loc.sliceText(1, -1); }
	get line() { return this.inner; }
}

// prettier-ignore
export class DocCommentAttribute extends BaseNode<15> implements DelimitedSequence<AttrSegment, "None">, AttributeLike, CommentLike {
	// /// ...
	// /** ... */
	// //! ...	  (inner)
	// /*! ... */ (inner)
	inner: boolean;
	line: boolean;
	get value() { return this.loc.sliceText(3, this.line ? undefined : -2); }
	get segments() {
		const loc = new Loc(this.loc.src, start(this) + 3, end(this) + (this.line ? 0 : -2));
		return createLocArray(
			DelimKind.None,
			loc, //
			[mockNode(NodeType.Literal, loc.clone(), { kind: LiteralKind.String, value: this.value })]
		);
	}
}

export class MacroInvocation extends BaseNode<16> implements DelimitedSequence<MacroInvokeSegment> {
	// callee!(...segments);?
	// callee![...segments];?
	// callee! { ...segments }
	callee: Identifier | PathNode;
	segments: LocArray<MacroInvokeSegment, "()" | "[]" | "{}">;
}

//#----------------------------------------------------+        Declaration        +--------------------------------------------------------.

export class MacroRulesDeclaration extends BaseNode<17> implements Identifiable, AttributeTarget {
	// macro_rules! id { ...rules }
	id: Identifier;
	rules: LocArray<MacroRuleDeclaration, "()" | "[]" | "{}">;
	declare attributes?: AttributeOrDocComment[];
}

export class MacroDeclaration extends BaseNode<18> implements Identifiable, AttributeTarget, PubTarget {
	// macro id ...rules
	id: Identifier;
	rules: LocArray<MacroRuleDeclaration, "{}"> | MacroInlineRuleDeclaration;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

/**/ export class MacroRuleDeclaration extends BaseNode<19> {
	// (...match) => (...transform);
	match: LocArray<MacroMatchSegment, "()" | "[]" | "{}">;
	transform: LocArray<MacroTransformSegment, "()" | "[]" | "{}">;
}

/**/ export class MacroInlineRuleDeclaration extends BaseNode<20> {
	// (...match) { ...transform }
	match: LocArray<MacroMatchSegment, "()">;
	transform: LocArray<MacroTransformSegment, "{}">;
}

//#---------------------------------------------------+        Rule Matching        +-------------------------------------------------------.

/**/ export class MacroGroup<T extends Segment = Segment> extends BaseNode<21> implements DelimitedSequence<T> {
	// $(...segments)sep?kind
	segments: LocArray<T, "()">;
	sep: MacroSeparator | undefined;
	kind: "*" | "+" | "?";
}

//#-----------------------------------------------------+        Match $AST        +--------------------------------------------------------.

/**/ export class MacroParameterDeclaration extends BaseNode<22> implements Identifiable<McIdentifier> {
	// id: ty
	id: McIdentifier;
	ty: Identifier | missing;
}

//#endregion ===============================================================================================================================..--'

//#region ==============================================[        Specifiers        ]========================================================``--.

export class PubSpecifier extends BaseNode<23> {
	// pub(in? location)? ...
	// {@link Feature.crate_visibility_modifier} `crate ...` (location is Identifier<"crate">)
	location: IdentifierOrItemPath | undefined; // crate | self | super | in ... (must start with 'crate' | 'self' | 'super' from edition 2018)
}

export class ExternSpecifier extends BaseNode<24> {
	// extern abi? ...
	abi: Literal | undefined; // "Application binary interface" [e.g. "C", "win64", "aapcs" (ARM), ...]
}

//#endregion ===============================================================================================================================..--'

//#region ==============================================[        Statements        ]========================================================``--.

//#------------------------------------------------+        ExpressionStatement        +----------------------------------------------------.

export class ExpressionStatement<T extends ExpressionNode | undefined = ExpressionNode | undefined>
	extends BaseNode<25>
	implements MaybeExpressionBody, AttributeTarget
{
	// expression?;?
	expression: T;
	semi: boolean;
	declare attributes?: AttributeOrDocComment[];
}

//#----------------------------------------------------+        UseStatement        +-------------------------------------------------------.

export class UseStatement extends BaseNode<26> implements AttributeTarget, PubTarget {
	// use import;
	import: ImportNode;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

/**/ export class NamedImport extends BaseNode<27> {
	// source as local?
	source: IdentifierOrItemPath; // (Identifier in ExternCrateStatement)
	local: Identifier | undefined;
}

/**/ export class AmbientImport extends BaseNode<28> {
	// source::?*
	source: IdentifierOrItemPath | undefined;
}

/**/ export class AnonymousImport extends BaseNode<29> {
	// source as _
	source: IdentifierOrItemPath; // (Identifier in ExternCrateStatement)
}

/**/ export class DestructuredImport extends BaseNode<30> {
	// source?::{ ...specifiers }
	source: IdentifierOrItemPath | undefined;
	specifiers: LocArray<ImportNode, "{}">;
}

//#-------------------------------------------------------+        Extern        +----------------------------------------------------------.

export class ExternCrateStatement extends BaseNode<31> implements AttributeTarget, PubTarget {
	// extern crate import as local?;
	import: AnonymousImport | NamedImport;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

export class ExternBlockDeclaration extends BaseNode<32> implements BlockBody, AttributeTarget, PubTarget, UnsafeModifier {
	// extern abi? { ...body }?
	abi: Literal | undefined;
	body: LocArray<StatementNode, "{}">;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
	declare unsafe?: true;
}

//#--------------------------------------------------------+        Var        +------------------------------------------------------------.

export class TypeAliasDeclaration
	extends BaseNode<33>
	implements Identifiable, DeclarationLike, TypeBoundsConstaint, MaybeTypeExpressionBody, AttributeTarget, PubTarget
{
	// type id<...generics>?: ...typeBounds? where ...whereBounds? = typeExpression?;
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	typeBounds: TypeBound[] | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	typeExpression: TypeNode | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

export class ConstVariableDeclaration
	extends BaseNode<34>
	implements PatternNoUnionBody, TypeAnnotationTarget<TypeNode | missing>, MaybeExpressionBody, AttributeTarget, PubTarget
{
	// const pattern: typeAnnotation = expression?;
	pattern: Identifier;
	typeAnnotation: TypeNode | missing;
	expression: ExpressionNode | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

export class StaticVariableDeclaration
	extends BaseNode<35>
	implements PatternNoUnionBody, TypeAnnotationTarget<TypeNode | missing>, MaybeExpressionBody, AttributeTarget, PubTarget
{
	// static pattern: typeAnnotation = expression?;
	pattern: Identifier | PatternVariableDeclaration /* & { ref: false; mut: true; id: Identifier; pattern: undefined; } */;
	typeAnnotation: TypeNode | missing;
	expression: ExpressionNode | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

export class LetVariableDeclaration
	extends BaseNode<36>
	implements PatternNoUnionBody, MaybeTypeAnnotationTarget, MaybeExpressionBody, AttributeTarget
{
	// let pattern: typeAnnotation? = expression?;
	pattern: PatternNoUnion;
	typeAnnotation: TypeNode | undefined;
	expression: ExpressionNode | undefined;
	/** {@link Feature.let_else} */ else: BlockExpression | undefined;
	declare attributes?: AttributeOrDocComment[];
}

//#-------------------------------------------------+        ModuleDeclaration        +-----------------------------------------------------.

export class ModuleDeclaration extends BaseNode<37> implements Identifiable, MaybeBlockBody, AttributeTarget, PubTarget, UnsafeModifier {
	// mod id { ...body }?
	id: Identifier;
	body: LocArray<StatementNode, "{}"> | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
	declare unsafe?: true; // (Allowed but ignored)
}

//#------------------------------------------------+        FunctionDeclaration        +----------------------------------------------------.

export class FunctionDeclaration
	extends BaseNode<38>
	implements
		Identifiable,
		FunctionLike,
		MaybeBlockBody,
		DeclarationLike,
		AttributeTarget,
		PubTarget,
		ExternTarget,
		ConstModifier,
		AsyncModifier,
		UnsafeModifier
{
	// fn id<...generics>?(...parameters) -> returnType? where ...whereBounds? { ...body }?
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	parameters: FunctionDeclarationParameters;
	returnType: TypeNode | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	body: LocArray<StatementNode, "{}"> | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
	declare const?: true;
	declare async?: true;
	declare unsafe?: true;
	declare extern?: ExternSpecifier;
	toJSON() {
		return { "parameters.self": this.parameters.self, ...this };
	}
}

/**/ export interface FunctionDeclarationParameters extends LocArray<FunctionParameterDeclaration | FunctionSpread, "()"> {
	self: FunctionSelfParameterDeclaration | undefined;
}

/**/ export class FunctionSelfParameterDeclaration
	extends BaseNode<39>
	implements RefAble, LtAble, MutAble, MaybeTypeAnnotationTarget, AttributeTarget
{
	// &?lt? mut? self: typeAnnotation?
	ref: boolean;
	lt: Lifetime | undefined;
	mut: boolean;
	typeAnnotation: TypeNode | undefined;
	declare attributes?: AttributeOrDocComment[];
}

/**/ export class FunctionParameterDeclaration
	extends BaseNode<40>
	implements PatternNoUnionBody, TypeAnnotationTarget<TypeNode | FunctionSpread | missing>, AttributeTarget
{
	// pattern: typeAnnotation
	pattern: PatternNoUnion;
	typeAnnotation: TypeNode | FunctionSpread | missing;
	declare attributes?: AttributeOrDocComment[];
}

/**/ export class FunctionSpread extends BaseNode<41> implements AttributeTarget {
	// ...
	declare attributes?: AttributeOrDocComment[];
}

//#-------------------------------------------------+        StructDeclaration        +-----------------------------------------------------.

export class StructDeclaration extends BaseNode<42> implements Identifiable, DeclarationLike, MaybeObjectLike, AttributeTarget, PubTarget {
	// struct id<...generics>? where ...whereBounds? { ...properties }?
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	properties: LocArray<StructPropertyDeclaration, "{}"> | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

export class TupleStructDeclaration extends BaseNode<43> implements Identifiable, DeclarationLike, ArrayLike, AttributeTarget, PubTarget {
	// struct id<...generics>?(...items) where ...whereBounds?;
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	items: LocArray<TupleStructItemDeclaration, "()">;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

/**/ export class StructPropertyDeclaration
	extends BaseNode<44>
	implements Identifiable, TypeAnnotationTarget<TypeNode | missing>, AttributeTarget, PubTarget
{
	// id: typeAnnotation
	id: Identifier;
	typeAnnotation: TypeNode | missing;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

/**/ export class TupleStructItemDeclaration extends BaseNode<45> implements TypeAnnotationTarget, AttributeTarget, PubTarget {
	typeAnnotation: TypeNode;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

//#--------------------------------------------------+        UnionDeclaration        +-----------------------------------------------------.

export class UnionDeclaration extends BaseNode<46> implements Identifiable, DeclarationLike, ObjectLike, AttributeTarget, PubTarget {
	// union id<...generics>? where ...whereBounds? { ...properties }
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	properties: LocArray<StructPropertyDeclaration, "{}">;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

//#--------------------------------------------------+        EnumDeclaration        +------------------------------------------------------.

export class EnumDeclaration extends BaseNode<47> implements Identifiable, DeclarationLike, AttributeTarget, PubTarget {
	// enum id<...generics>? where ...whereBounds? { ...members }
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	members: LocArray<EnumDeclarationMember, "{}">;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

/**/ export type EnumDeclarationMember = EnumMemberDeclaration | EnumMemberTupleDeclaration | EnumMemberStructDeclaration;

/**/ export class EnumMemberDeclaration extends BaseNode<48> implements Identifiable, AttributeTarget, PubTarget {
	// id = value?
	id: Identifier;
	value: ExpressionNode | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier; // (Allowed but ignored)
}

/**/ export class EnumMemberTupleDeclaration extends BaseNode<49> implements Identifiable, ArrayLike, AttributeTarget, PubTarget {
	// id(...items) = value?
	id: Identifier;
	items: LocArray<TupleStructItemDeclaration, "()">;
	/** `= ...` {@link Feature.arbitrary_enum_discriminant} */ value: ExpressionNode | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier; // (Allowed but ignored)
}

/**/ export class EnumMemberStructDeclaration extends BaseNode<50> implements Identifiable, ObjectLike, AttributeTarget, PubTarget {
	// id {...properties} = value?
	id: Identifier;
	properties: LocArray<StructPropertyDeclaration, "{}">;
	/** `= ...` {@link Feature.arbitrary_enum_discriminant} */ value: ExpressionNode | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier; // (Allowed but ignored)
}

//#--------------------------------------------------+        TraitDeclaration        +-----------------------------------------------------.

export class TraitDeclaration
	extends BaseNode<51>
	implements Identifiable, BlockBody, DeclarationLike, TypeBoundsConstaint, AttributeTarget, PubTarget, UnsafeModifier
{
	// trait id<...generics>?: ...typeBounds? where ...whereBounds? { ...body }
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	typeBounds: TypeBound[] | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	body: LocArray<StatementNode, "{}">;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
	declare unsafe?: true;
}

export class AutoTraitDeclaration extends BaseNode<52> implements Identifiable, AttributeTarget, PubTarget, UnsafeModifier {
	// auto trait id {}
	id: Identifier;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
	declare unsafe?: true;
}

export class TraitAliasDeclaration
	extends BaseNode<53>
	implements Identifiable, DeclarationLike, TypeBoundsConstaint, AttributeTarget, PubTarget, UnsafeModifier
{
	// trait id<...generics>? = ...typeBounds where ...whereBounds?;
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	typeBounds: TypeBound[];
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
	declare unsafe?: true;
}

//#--------------------------------------------------+        ImplDeclaration        +------------------------------------------------------.

export class ImplDeclaration
	extends BaseNode<54>
	implements BlockBody, TypeTargetConstraint, DeclarationLike, AttributeTarget, PubTarget, UnsafeModifier
{
	// impl<...generics>? const? trait for typeTarget where ...whereBounds? { ...body }
	// impl<...generics>?                  typeTarget where ...whereBounds? { ...body }
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	/** `impl<...> const ... for ...` {@link Feature.const_trait_impl} 	*/ const: boolean;
	trait: TypeNamespaceTargetNoSelector | undefined;
	typeTarget: TypeNode;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	body: LocArray<StatementNode, "{}">;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
	declare unsafe?: true;
}

export class NegativeImplDeclaration extends BaseNode<55> implements TypeTargetConstraint, DeclarationLike, AttributeTarget, PubTarget {
	// impl<...generics>? !trait for typeTarget where ...whereBounds? {}
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	trait: TypeNamespaceTargetNoSelector;
	typeTarget: TypeNode;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	declare attributes?: AttributeOrDocComment[];
	declare pub?: PubSpecifier;
}

//#endregion ===============================================================================================================================..--'

//#region =============================================[        Expressions        ]========================================================``--.

//#--------------------------------------------------------+        Path        +-----------------------------------------------------------.

/**/ export class ExpressionTypeSelector extends BaseNode<56> implements TypeTargetConstraint, MaybeTypeExpressionBody {
	// <typeTarget as typeExpression?>
	typeTarget: TypeNode;
	typeExpression: TypeNamespaceTargetNoSelector | undefined;
}

export class ExpressionTypeCast<T extends _PathBase = _PathBase> extends BaseNode<57> implements TypeCallable<_ExprPathSource<T>> {
	// typeCallee::<...typeArguments>
	typeCallee: _ExprPathSource<T>;
	typeArguments: LocArray<TypeCallArgument, "<>">;
}

export class ExpressionPath<T extends _PathBase = _PathBase> extends BaseNode<58> implements PathLike {
	// namespace::segment
	namespace: _ExprPathSource<T> | undefined;
	segment: Identifier;
}
/** 'TypeCastExpression' */
export class ExpressionAsTypeCast extends BaseNode<59> implements ExpressionBody, TypeExpressionBody {
	// expression as typeExpression
	expression: ExpressionNode;
	typeExpression: TypeNode;
}

//#----------------------------------------------------+        Flow Control        +-------------------------------------------------------.

export class ReturnExpression extends BaseNode<60> implements MaybeExpressionBody {
	// return expression?
	expression: ExpressionNode | undefined;
}

export class BreakExpression extends BaseNode<61> implements MaybeExpressionBody, MaybeLabelTarget {
	// break label? expression?
	label: LbIdentifier | undefined;
	expression: ExpressionNode | undefined;
}

export class ContinueExpression extends BaseNode<62> implements MaybeLabelTarget {
	// continue label?
	label: LbIdentifier | undefined;
}

export class YieldExpression extends BaseNode<63> implements MaybeExpressionBody {
	// yield expression
	expression: ExpressionNode | undefined;
}

//#-----------------------------------------------------+        Derivative        +--------------------------------------------------------.

export class CallExpression extends BaseNode<64> implements MaybeGenericArgsTarget {
	// callee.method?::<...typeArguments>?(...arguments)
	callee: ExpressionNode;
	method: Identifier | undefined;
	typeArguments: LocArray<TypeCallArgument, "<>"> | undefined;
	arguments: LocArray<ExpressionNode, "()">;
}

export class MemberExpression extends BaseNode<65> implements ExpressionBody {
	// expression.property
	// expression[property] // computed
	expression: ExpressionNode;
	computed: boolean;
	property: IdentifierOrIndex | ExpressionNode;
}

export class AwaitExpression extends BaseNode<66> implements ExpressionBody {
	// expression.await
	expression: ExpressionNode;
}

export class UnwrapExpression extends BaseNode<67> implements ExpressionBody {
	// expression?
	expression: ExpressionNode;
}

export class ParenthesizedExpression<T extends ExpressionNode = ExpressionNode> extends BaseNode<68> implements ExpressionBody {
	// (expression)
	expression: T;
}

export class MinusExpression<T extends ExpressionNode = ExpressionNode> extends BaseNode<69> implements ExpressionBody {
	// -expression
	expression: T;
}

export class NotExpression extends BaseNode<70> implements ExpressionBody {
	// !expression
	expression: ExpressionNode;
}

//#-----------------------------------------------------+        LeftRight        +---------------------------------------------------------.

// prettier-ignore
export class OrExpression<L extends ConditionExpression = ConditionExpression, R extends ConditionExpression = ConditionExpression> extends BaseNode<71> implements LeftRightLike {
	// left || right
	get kind() { return str_TK[this.tk] as "||"; }
	left: L;
	tk: TK;
	right: R;
}

// prettier-ignore
export class AndExpression<L extends ConditionExpression = ConditionExpression, R extends ConditionExpression = ConditionExpression> extends BaseNode<72> implements LeftRightLike {
	// left && right
	get kind() { return str_TK[this.tk] as "&&"; }
	left: L;
	tk: TK;
	right: R;
}

// prettier-ignore
export class ReassignmentExpression extends BaseNode<73> implements LeftRightLike {
	// left = right
	get kind() { return str_TK[this.tk] as "="; }
	left: ExpressionNode | UnassignedExpression /* | ExpressionNode with nested UnassignedExpression */;
	tk: TK;
	right: ExpressionNode;
}

/**/ export class UnassignedExpression extends BaseNode<74> {
	// _
}

// prettier-ignore
export class OperationExpression extends BaseNode<75> implements LeftRightLike {
	// e.g. left + right
	get kind() { return str_TK[this.tk]; }
	left: ExpressionNode;
	tk: TK & OpExpr;
	right: ExpressionNode;
}

// prettier-ignore
export class ReassignmentOperationExpression extends BaseNode<76> implements LeftRightLike {
	// e.g. left += right
	get kind() { return str_TK[this.tk]; }
	left: ExpressionNode;
	tk: TK & ReaOpExpr;
	right: ExpressionNode;
}

// prettier-ignore
export class ComparisonExpression extends BaseNode<77> implements LeftRightLike {
	// e.g. left == right
	get kind() { return str_TK[this.tk] }
	left: ExpressionNode;
	tk: TK & CompExpr;
	right: ExpressionNode;
}

//#----------------------------------------------------+        LetScrutinee        +-------------------------------------------------------.

/**/ export class LetScrutinee extends BaseNode<78> implements PatternBody {
	// let pattern = expression
	pattern: PatternNode;
	expression: ScrutineeExpression;
}

/**/ export type ScrutineeExpression = Exclude<ExpressionNode, OrExpression | AndExpression>;
/**/ export type ExpressionNodeXS = Exclude<ExpressionNode, StructLiteral>;
/**/ export type ConditionExpression =
	| ExpressionNode
	| LetScrutinee
	| AndExpression<ConditionExpression, ConditionExpression>
	| OrExpression<ConditionExpression, ConditionExpression>;
/**/ export type ConditionExpressionXS =
	| ExpressionNodeXS
	| LetScrutinee
	| AndExpression<ConditionExpressionXS, ConditionExpressionXS>
	| OrExpression<ConditionExpressionXS, ConditionExpressionXS>;

//#--------------------------------------------------+        ClosureFunction        +------------------------------------------------------.

export class ClosureFunctionExpression
	extends BaseNode<79>
	implements FunctionLike, ExpressionBody, MoveModifier, StaticModifier, AsyncModifier
{
	// |...| -> returnType? expression
	parameters: LocArray<ClosureFunctionParameterDeclaration, "||">;
	returnType: TypeNode | undefined;
	expression: ExpressionNode;
	declare move?: true;
	/** `static |...| ...` {@link Feature.generators}    */ declare static?: true;
	/** ` async |...| ...` {@link Feature.async_closure} */ declare async?: true;
}

/**/ export class ClosureFunctionParameterDeclaration
	extends BaseNode<80>
	implements PatternNoUnionBody, MaybeTypeAnnotationTarget, AttributeTarget
{
	// pattern: typeAnnotation?
	pattern: PatternNoUnion;
	typeAnnotation: TypeNode | undefined;
	declare attributes?: AttributeOrDocComment[];
}

//#-----------------------------------------------------+        BlockLike        +---------------------------------------------------------.

export class BlockExpression
	extends BaseNode<81>
	implements BlockLike, AttributeTarget, AsyncModifier, MoveModifier, UnsafeModifier, ConstModifier
{
	// label?: { ...body }
	/** jinx-rust only */ label: LbIdentifier | undefined;
	body: LocArray<StatementNode, "{}">;
	declare attributes?: AttributeOrDocComment[];
	declare async?: true;
	declare move?: true;
	declare unsafe?: true;
	/** `const { ... }` {@link Feature.inline_const} */ declare const?: true;
}

export class LoopBlockExpression extends BaseNode<82> implements BlockLike, AttributeTarget {
	// label?: loop { ...body }
	label: LbIdentifier | undefined;
	body: LocArray<StatementNode, "{}">;
	declare attributes?: AttributeOrDocComment[];
}

export class WhileBlockExpression extends BaseNode<83> implements BlockLike, ConditionBody, AttributeTarget {
	// label?: while condition { ...body }
	label: LbIdentifier | undefined;
	condition: ConditionExpressionXS;
	body: LocArray<StatementNode, "{}">;
	declare attributes?: AttributeOrDocComment[];
}

export class ForInBlockExpression extends BaseNode<84> implements BlockLike, AttributeTarget {
	// label?: for pattern in expression { ...body }
	label: LbIdentifier | undefined;
	pattern: PatternNode;
	expression: ExpressionNodeXS;
	body: LocArray<StatementNode, "{}">;
	declare attributes?: AttributeOrDocComment[];
}

export class TryBlockExpression extends BaseNode<85> implements BlockLike, AttributeTarget {
	// try { ...body }
	/** jinx-rust only */ label: LbIdentifier | undefined;
	body: LocArray<StatementNode, "{}">;
	declare attributes?: AttributeOrDocComment[];
}

export class IfBlockExpression extends BaseNode<86> implements BlockLike, ConditionBody, AttributeTarget {
	// label?: if condition { ...body } else?
	/** jinx-rust only */ label: LbIdentifier | undefined;
	condition: ConditionExpressionXS;
	body: LocArray<StatementNode, "{}">;
	else: IfBlockExpression | BlockExpression | undefined;
	declare attributes?: AttributeOrDocComment[];
}

export class MatchExpression extends BaseNode<87> implements MaybeLabelTarget, ExpressionBody, AttributeTarget {
	// match expression { ...cases }
	/** jinx-rust only */ label: LbIdentifier | undefined;
	expression: ExpressionNodeXS;
	cases: LocArray<MatchExpressionCase, "{}">;
	declare attributes?: AttributeOrDocComment[];
}

/**/ export class MatchExpressionCase extends BaseNode<88> implements PatternBody, MaybeConditionBody, ExpressionBody, AttributeTarget {
	// pattern if condition? => expression
	pattern: PatternNode;
	/** {@link Feature.if_let_guard} */ condition: ConditionExpressionXS | undefined;
	expression: ExpressionNode;
	declare attributes?: AttributeOrDocComment[];
}

//#------------------------------------------------------+        Literal        +----------------------------------------------------------.

export class RangeLiteral extends BaseNode<89> implements RangeLike {
	// .. | min.. | min..max | min..=max | ..max | ..=max
	lower: ExpressionNode | undefined;
	last: boolean;
	upper: ExpressionNode | undefined;
}

export class StructLiteral extends BaseNode<90> implements ObjectLike {
	// struct { ...properties }
	// Note: Struct(...) is parsed as a CallExpression
	struct: ExpressionNamespaceTargetNoSelector;
	properties: LocArray<StructProperty, "{}">;
}

/**/ export type StructProperty =
	| StructLiteralProperty
	| StructLiteralPropertyShorthand
	| StructLiteralPropertySpread
	| StructLiteralRestUnassigned;

/**/ export class StructLiteralProperty extends BaseNode<91> implements PropertyLike, AttributeTarget {
	// key: value
	key: IdentifierOrIndex;
	value: ExpressionNode;
	declare attributes?: AttributeOrDocComment[];
}

/**/ export class StructLiteralPropertyShorthand extends BaseNode<92> implements AttributeTarget {
	// value
	value: Identifier;
	declare attributes?: AttributeOrDocComment[];
}

/**/ export class StructLiteralPropertySpread extends BaseNode<93> implements ExpressionBody {
	// ..expression
	expression: ExpressionNode;
}

/**/ export class StructLiteralRestUnassigned extends BaseNode<94> {
	// ..
}

export class TupleLiteral extends BaseNode<95> implements ArrayLike {
	// (...items) | (item,)
	items: LocArray<ExpressionNode, "()">;
}

export class ArrayLiteral extends BaseNode<96> implements ArrayLike {
	// [...items]
	items: LocArray<ExpressionNode, "[]">;
}

export class SizedArrayLiteral extends BaseNode<97> implements SizeExpressionBody {
	// [initExpression; sizeExpression]
	initExpression: ExpressionNode;
	sizeExpression: ExpressionNode;
}

//#-------------------------------------------------------+        Memory        +----------------------------------------------------------.

export class ReferenceExpression extends BaseNode<98> implements MutAble, ExpressionBody {
	// &mut? expression
	mut: boolean;
	expression: ExpressionNode;
}

export class RawReferenceExpression extends BaseNode<99> implements ExpressionBody {
	// &raw kind expression
	kind: "mut" | "const";
	expression: ExpressionNode;
}

export class DereferenceExpression extends BaseNode<100> implements ExpressionBody {
	// *expression
	expression: ExpressionNode;
}

export class BoxExpression extends BaseNode<101> implements ExpressionBody {
	// box expression
	expression: ExpressionNode;
}

//#endregion ===============================================================================================================================..--'

//#region ===============================================[        Patterns        ]=========================================================``--.

//#------------------------------------------------------+        Grouping        +---------------------------------------------------------.

export class UnionPattern extends BaseNode<102> {
	// ...patterns
	patterns: PatternNode[]; // (allows leading "|")
}

export class ParenthesizedPattern<T extends PatternNode = PatternNode> extends BaseNode<103> implements PatternBody {
	// (pattern)
	pattern: T;
}

//#------------------------------------------------------+        Wildcard        +---------------------------------------------------------.

export class RestPattern extends BaseNode<104> {
	// ..
	// (attributes are parsed but ignored when property of StructPattern)
}

export class WildcardPattern extends BaseNode<105> {
	// _
}

//#-----------------------------------------------------+        Variables        +---------------------------------------------------------.

export class PatternVariableDeclaration extends BaseNode<106> implements Identifiable, RefAble, MutAble, MaybePatternNoUnionBody {
	// ref? mut? id @ pattern?
	ref: boolean;
	mut: boolean;
	id: Identifier;
	pattern: PatternNoUnion | undefined;
}

//#------------------------------------------------------+        Literals        +---------------------------------------------------------.

export class StructPattern extends BaseNode<107> implements ObjectLike {
	// struct { ...properties }
	struct: ExpressionNamespaceTargetNoSelector;
	properties: LocArray<StructPatternProperty, "{}">;
}

/**/ export type StructPatternProperty =
	| StructPatternPropertyDestructured //
	| StructPatternPropertyShorthand
	| RestPattern;

/**/ export class StructPatternPropertyDestructured extends BaseNode<108> implements PropertyLike, PatternBody, AttributeTarget {
	// key: pattern
	key: IdentifierOrIndex;
	pattern: PatternNode;
	declare attributes?: AttributeOrDocComment[];
}

/**/ export class StructPatternPropertyShorthand extends BaseNode<109> implements Identifiable, AttributeTarget {
	// box? ref? mut? id
	/** `box ...` {@link Feature.box_patterns} */ box: boolean;
	ref: boolean;
	mut: boolean;
	id: Identifier;
	declare attributes?: AttributeOrDocComment[];
}

export class TuplePattern extends BaseNode<110> implements ArrayLike {
	// struct?(...items) | (item,) | (..)
	struct: ExpressionNamespaceTargetNoSelector | undefined;
	items: LocArray<PatternNode, "()">; // if !!struct, items may have attributes
}

export class ArrayPattern extends BaseNode<111> implements ArrayLike {
	// [...items]
	items: LocArray<PatternNode, "[]">;
}

//#-------------------------------------------------------+        Memory        +----------------------------------------------------------.

export class ReferencePattern extends BaseNode<112> implements MutAble, PatternBody {
	// &mut? pattern
	mut: boolean;
	pattern: PatternNoUnionNoRange;
}

export class BoxPattern extends BaseNode<113> implements PatternBody {
	// box pattern
	pattern: PatternNoUnionNoRange;
}

//#---------------------------------------------------+        Miscellaneous        +-------------------------------------------------------.

export class MinusPattern<T extends Literal = Literal> extends BaseNode<114> implements PatternBody {
	// -pattern
	pattern: T; // only LiteralKind.Float or LiteralKind.Integer
}

export class RangePattern extends BaseNode<115> implements RangeLike {
	// min.. | min...max | min..=max
	// exclusive_range_pattern  | min..max
	// half_open_range_patterns | ..max | ...max | ..=max
	lower: RangePatternBound | undefined;
	last: boolean; // '..=' (or '...', but errors from edition 2021)
	upper: RangePatternBound | undefined;
}

/**/ export type RangePatternBound =
	| BlockExpression // const { ... } (#![feature(inline_const_pat)])
	| ExpressionNamespaceTarget
	| MinusPattern<Literal>
	| Literal;

//#endregion ===============================================================================================================================..--'

//#region ================================================[        Types        ]===========================================================``--.

//#--------------------------------------------------------+        Path        +-----------------------------------------------------------.

export class TypePath<T extends _PathBase = _PathBase> extends BaseNode<116> implements PathLike {
	// namespace?::segment
	namespace: _TypePathSource<T> | undefined;
	segment: Identifier;
}

export class TypeCall<T extends _PathBase = _PathBase> extends BaseNode<117> implements TypeCallable<_TypePathSource<T>> {
	// typeCallee::<...typeArguments>
	// typeCallee<...typeArguments>
	typeCallee: _TypePathSource<T>;
	typeArguments: LocArray<TypeCallArgument, "<>">;
}

/**/ export type TypeCallArgument =
	| Literal
	| Lifetime
	| MinusExpression<Literal>
	| BlockExpression
	| TypeCallNamedArgument
	| TypeCallNamedBound
	| TypeNode;

/**/ export class TypeCallNamedArgument extends BaseNode<118> implements TypeExpressionBody {
	// target = typeExpression
	target: Identifier;
	typeExpression: TypeNode;
}

/**/ export class TypeCallNamedBound extends BaseNode<119> implements TypeTargetConstraint, TypeBoundsConstaint {
	// typeTarget: ...typeBounds
	typeTarget: TypeNode;
	typeBounds: TypeBound[];
}

//#------------------------------------------------------+        Lifetime        +---------------------------------------------------------.

export type Lifetime = LtIdentifier | LtElided | LtStatic;

// prettier-ignore
export class LtIdentifier extends BaseNode<120> implements IdentifierLike {
	get name() { return this.loc.getOwnText() as `'${string}`; }
}

export class LtElided extends BaseNode<121> {
	// '_
}

export class LtStatic extends BaseNode<122> {
	// 'static
}

//#------------------------------------------------------+        Special        +----------------------------------------------------------.

export class TypeNever extends BaseNode<123> {
	// !
}

export class TypeInferred extends BaseNode<124> {
	// _
}

//#------------------------------------------------------+        Generics        +---------------------------------------------------------.

/**/ export type GenericParameterDeclaration =
	| GenericTypeParameterDeclaration
	| ConstTypeParameterDeclaration
	| GenericLtParameterDeclaration;

/**/ export class GenericTypeParameterDeclaration extends BaseNode<125> implements Identifiable, TypeBoundsConstaint, AttributeTarget {
	// id: ...typeBounds? = typeDefault?
	id: Identifier;
	typeBounds: TypeBound[] | undefined;
	typeDefault: TypeNode | undefined;
	declare attributes?: AttributeOrDocComment[];
}

/**/ export class ConstTypeParameterDeclaration
	extends BaseNode<126>
	implements Identifiable, TypeAnnotationTarget<TypeNode | missing>, AttributeTarget
{
	// const id: typeAnnotation = typeDefault?
	id: Identifier;
	typeAnnotation: TypeNode | missing;
	/** `const T = ...` {@link Feature.const_generics_defaults} */ typeDefault: ConstTypeParamDefault | undefined;
	declare attributes?: AttributeOrDocComment[];
}

/**/ export type ConstTypeParamDefault =
	| Literal //
	| MinusExpression<Literal>
	| BlockExpression
	| TypeNode;

/** 'LifetimeParam' */
/**/ export class GenericLtParameterDeclaration extends BaseNode<127> implements LtIdentifiable, MaybeHasLtBounds, AttributeTarget {
	// id: ltBounds?
	id: LtIdentifier;
	ltBounds: Lifetime[] | undefined;
	declare attributes?: AttributeOrDocComment[];
}

//#-------------------------------------------------------+        Where        +-----------------------------------------------------------.

/** 'WhereClauseItem' */
/**/ export type WhereBoundDeclaration = WhereTypeBoundDeclaration | WhereLtBoundDeclaration;

/** 'TypeBoundWhereClauseItem' */
/**/ export class WhereTypeBoundDeclaration
	extends BaseNode<128>
	implements ForLtParametersBody, TypeTargetConstraint, TypeBoundsConstaint
{
	// for<...ltParameters>? typeTarget: ...typeBounds?
	ltParameters: LocArray<GenericLtParameterDeclaration, "<>"> | undefined;
	typeTarget: TypeNode;
	typeBounds: TypeBound[];
}

/** 'LifetimeWhereClauseItem' */
/**/ export class WhereLtBoundDeclaration extends BaseNode<129> implements TargetsLifetime, HasLtBounds {
	// ltTarget: ltBounds
	ltTarget: Lifetime;
	ltBounds: Lifetime[];
}

//#-------------------------------------------------------+        Bounds        +----------------------------------------------------------.

/** 'TypeParamBound' */
/**/ export type TypeBound = TypeTraitBound | Lifetime | TypeParenthesized<TypeTraitBound>;

/**/ export class TypeTraitBound extends BaseNode<130> implements ForLtParametersBody, TypeExpressionBody {
	// ~const? ??for<...ltParameters>? typeExpression
	/** `~const` {@link Feature.const_trait_impl} */ maybeConst: boolean;
	/** e.g. `?Sized`                             */ optional: boolean;
	ltParameters: LocArray<GenericLtParameterDeclaration, "<>"> | undefined;
	typeExpression: TypeNamespaceTargetNoSelector;
}

export class TypeDynBounds extends BaseNode<131> implements TypeBoundsStandalone {
	// dyn? ...typeBounds
	dyn: boolean;
	typeBounds: TypeBound[];
}

export class TypeImplBounds extends BaseNode<132> implements TypeBoundsStandalone {
	// impl ...typeBounds
	typeBounds: TypeBound[];
}

//#------------------------------------------------------+        Function        +---------------------------------------------------------.

export class TypeFnPointer extends BaseNode<133> implements ForLtParametersBody, FunctionLike, UnsafeModifier, ExternTarget {
	// for<...ltParameters>? fn(...parameters) -> returnType?
	ltParameters: LocArray<GenericLtParameterDeclaration, "<>"> | undefined;
	parameters: LocArray<TypeFnPointerParameter | FunctionSpread, "()">;
	returnType: TypeNode | undefined;
	declare unsafe?: true;
	declare extern?: ExternSpecifier;
}

/**/ export class TypeFnPointerParameter extends BaseNode<134> implements MaybeIdentifiable, TypeAnnotationTarget, AttributeTarget {
	// id?: typeAnnotation
	id: Identifier | undefined;
	typeAnnotation: TypeNode;
	declare attributes?: AttributeOrDocComment[];
}

export class TypeFunction<T extends _PathBase = _PathBase> extends BaseNode<135> implements FunctionLike {
	// callee(...parameters) -> returnType?
	callee: _TypePathSource<T>;
	parameters: LocArray<TypeNode, "()">;
	returnType: TypeNode | undefined;
}

//#------------------------------------------------------+        Literal        +----------------------------------------------------------.

export class TypeTuple extends BaseNode<136> implements ArrayLike {
	// (...items) | (item,)
	items: LocArray<TypeNode, "()">;
}

export class TypeSizedArray extends BaseNode<137> implements TypeExpressionBody, SizeExpressionBody {
	// [typeExpression; sizeExpression]
	typeExpression: TypeNode;
	sizeExpression: ExpressionNode;
}

export class TypeSlice extends BaseNode<138> implements TypeExpressionBody {
	// [typeExpression]
	typeExpression: TypeNode;
}

//#-------------------------------------------------------+        Memory        +----------------------------------------------------------.

export class TypeReference extends BaseNode<139> implements LtAble, MutAble, TypeExpressionBody {
	// &lt? mut? typeExpression
	lt: Lifetime | undefined;
	mut: boolean;
	typeExpression: TypeNode;
}

export class TypeDereferenceConst extends BaseNode<140> implements TypeExpressionBody {
	// *const typeExpression
	typeExpression: TypeNode;
}

export class TypeDereferenceMut extends BaseNode<141> implements TypeExpressionBody {
	// *mut typeExpression
	typeExpression: TypeNode;
}

//#---------------------------------------------------+        Miscellaneous        +-------------------------------------------------------.

export class TypeParenthesized<T extends TypeNode | TypeTraitBound = TypeNode | TypeTraitBound>
	extends BaseNode<142>
	implements TypeExpressionBody<T>
{
	// (typeExpression)
	typeExpression: T;
}

//#endregion ===============================================================================================================================..--'
