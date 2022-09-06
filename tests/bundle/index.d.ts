interface ParserOptions {
	filepath?: string | undefined;
}

declare enum NodeType {
	MissingNode = 0,
	SourceFile = 1,
	Shebang = 2,
	Program = 3,
	Snippet = 4,
	Comment = 5,
	Identifier = 6,
	Index = 7,
	LbIdentifier = 8,
	McIdentifier = 9,
	Literal = 10,
	ItemPath = 11,
	PunctuationToken = 12,
	DelimGroup = 13,
	Attribute = 14,
	DocCommentAttribute = 15,
	MacroInvocation = 16,
	MacroRulesDeclaration = 17,
	MacroDeclaration = 18,
	MacroRuleDeclaration = 19,
	MacroInlineRuleDeclaration = 20,
	MacroGroup = 21,
	MacroParameterDeclaration = 22,
	PubSpecifier = 23,
	ExternSpecifier = 24,
	ExpressionStatement = 25,
	UseStatement = 26,
	NamedImport = 27,
	AmbientImport = 28,
	AnonymousImport = 29,
	DestructuredImport = 30,
	ExternCrateStatement = 31,
	ExternBlockDeclaration = 32,
	TypeAliasDeclaration = 33,
	ConstVariableDeclaration = 34,
	StaticVariableDeclaration = 35,
	LetVariableDeclaration = 36,
	ModuleDeclaration = 37,
	FunctionDeclaration = 38,
	FunctionSelfParameterDeclaration = 39,
	FunctionParameterDeclaration = 40,
	FunctionSpread = 41,
	StructDeclaration = 42,
	TupleStructDeclaration = 43,
	StructPropertyDeclaration = 44,
	TupleStructItemDeclaration = 45,
	UnionDeclaration = 46,
	EnumDeclaration = 47,
	EnumMemberDeclaration = 48,
	EnumMemberTupleDeclaration = 49,
	EnumMemberStructDeclaration = 50,
	TraitDeclaration = 51,
	AutoTraitDeclaration = 52,
	TraitAliasDeclaration = 53,
	ImplDeclaration = 54,
	NegativeImplDeclaration = 55,
	ExpressionTypeSelector = 56,
	ExpressionTypeCast = 57,
	ExpressionPath = 58,
	ExpressionAsTypeCast = 59,
	ReturnExpression = 60,
	BreakExpression = 61,
	ContinueExpression = 62,
	YieldExpression = 63,
	CallExpression = 64,
	MemberExpression = 65,
	AwaitExpression = 66,
	UnwrapExpression = 67,
	ParenthesizedExpression = 68,
	MinusExpression = 69,
	NotExpression = 70,
	OrExpression = 71,
	AndExpression = 72,
	ReassignmentExpression = 73,
	UnassignedExpression = 74,
	OperationExpression = 75,
	ReassignmentOperationExpression = 76,
	ComparisonExpression = 77,
	LetScrutinee = 78,
	ClosureFunctionExpression = 79,
	ClosureFunctionParameterDeclaration = 80,
	BlockExpression = 81,
	LoopBlockExpression = 82,
	WhileBlockExpression = 83,
	ForInBlockExpression = 84,
	TryBlockExpression = 85,
	IfBlockExpression = 86,
	MatchExpression = 87,
	MatchExpressionCase = 88,
	RangeLiteral = 89,
	StructLiteral = 90,
	StructLiteralProperty = 91,
	StructLiteralPropertyShorthand = 92,
	StructLiteralPropertySpread = 93,
	StructLiteralRestUnassigned = 94,
	TupleLiteral = 95,
	ArrayLiteral = 96,
	SizedArrayLiteral = 97,
	ReferenceExpression = 98,
	RawReferenceExpression = 99,
	DereferenceExpression = 100,
	BoxExpression = 101,
	UnionPattern = 102,
	ParenthesizedPattern = 103,
	RestPattern = 104,
	WildcardPattern = 105,
	PatternVariableDeclaration = 106,
	StructPattern = 107,
	StructPatternPropertyDestructured = 108,
	StructPatternPropertyShorthand = 109,
	TuplePattern = 110,
	ArrayPattern = 111,
	ReferencePattern = 112,
	BoxPattern = 113,
	MinusPattern = 114,
	RangePattern = 115,
	TypePath = 116,
	TypeCall = 117,
	TypeCallNamedArgument = 118,
	TypeCallNamedBound = 119,
	LtIdentifier = 120,
	LtElided = 121,
	LtStatic = 122,
	TypeNever = 123,
	TypeInferred = 124,
	GenericTypeParameterDeclaration = 125,
	ConstTypeParameterDeclaration = 126,
	GenericLtParameterDeclaration = 127,
	WhereTypeBoundDeclaration = 128,
	WhereLtBoundDeclaration = 129,
	TypeTraitBound = 130,
	TypeDynBounds = 131,
	TypeImplBounds = 132,
	TypeFnPointer = 133,
	TypeFnPointerParameter = 134,
	TypeFunction = 135,
	TypeTuple = 136,
	TypeSizedArray = 137,
	TypeSlice = 138,
	TypeReference = 139,
	TypeDereferenceConst = 140,
	TypeDereferenceMut = 141,
	TypeParenthesized = 142,
}
declare type Node =
	| MissingNode
	| SourceFile
	| Shebang
	| Program
	| Snippet
	| Comment
	| Identifier
	| Index
	| LbIdentifier
	| McIdentifier
	| Literal
	| ItemPath
	| PunctuationToken
	| DelimGroup
	| Attribute
	| DocCommentAttribute
	| MacroInvocation
	| MacroRulesDeclaration
	| MacroDeclaration
	| MacroRuleDeclaration
	| MacroInlineRuleDeclaration
	| MacroGroup
	| MacroParameterDeclaration
	| PubSpecifier
	| ExternSpecifier
	| ExpressionStatement
	| UseStatement
	| NamedImport
	| AmbientImport
	| AnonymousImport
	| DestructuredImport
	| ExternCrateStatement
	| ExternBlockDeclaration
	| TypeAliasDeclaration
	| ConstVariableDeclaration
	| StaticVariableDeclaration
	| LetVariableDeclaration
	| ModuleDeclaration
	| FunctionDeclaration
	| FunctionSelfParameterDeclaration
	| FunctionParameterDeclaration
	| FunctionSpread
	| StructDeclaration
	| TupleStructDeclaration
	| StructPropertyDeclaration
	| TupleStructItemDeclaration
	| UnionDeclaration
	| EnumDeclaration
	| EnumMemberDeclaration
	| EnumMemberTupleDeclaration
	| EnumMemberStructDeclaration
	| TraitDeclaration
	| AutoTraitDeclaration
	| TraitAliasDeclaration
	| ImplDeclaration
	| NegativeImplDeclaration
	| ExpressionTypeSelector
	| ExpressionTypeCast
	| ExpressionPath
	| ExpressionAsTypeCast
	| ReturnExpression
	| BreakExpression
	| ContinueExpression
	| YieldExpression
	| CallExpression
	| MemberExpression
	| AwaitExpression
	| UnwrapExpression
	| ParenthesizedExpression
	| MinusExpression
	| NotExpression
	| OrExpression
	| AndExpression
	| ReassignmentExpression
	| UnassignedExpression
	| OperationExpression
	| ReassignmentOperationExpression
	| ComparisonExpression
	| LetScrutinee
	| ClosureFunctionExpression
	| ClosureFunctionParameterDeclaration
	| BlockExpression
	| LoopBlockExpression
	| WhileBlockExpression
	| ForInBlockExpression
	| TryBlockExpression
	| IfBlockExpression
	| MatchExpression
	| MatchExpressionCase
	| RangeLiteral
	| StructLiteral
	| StructLiteralProperty
	| StructLiteralPropertyShorthand
	| StructLiteralPropertySpread
	| StructLiteralRestUnassigned
	| TupleLiteral
	| ArrayLiteral
	| SizedArrayLiteral
	| ReferenceExpression
	| RawReferenceExpression
	| DereferenceExpression
	| BoxExpression
	| UnionPattern
	| ParenthesizedPattern
	| RestPattern
	| WildcardPattern
	| PatternVariableDeclaration
	| StructPattern
	| StructPatternPropertyDestructured
	| StructPatternPropertyShorthand
	| TuplePattern
	| ArrayPattern
	| ReferencePattern
	| BoxPattern
	| MinusPattern
	| RangePattern
	| TypePath
	| TypeCall
	| TypeCallNamedArgument
	| TypeCallNamedBound
	| LtIdentifier
	| LtElided
	| LtStatic
	| TypeNever
	| TypeInferred
	| GenericTypeParameterDeclaration
	| ConstTypeParameterDeclaration
	| GenericLtParameterDeclaration
	| WhereTypeBoundDeclaration
	| WhereLtBoundDeclaration
	| TypeTraitBound
	| TypeDynBounds
	| TypeImplBounds
	| TypeFnPointer
	| TypeFnPointerParameter
	| TypeFunction
	| TypeTuple
	| TypeSizedArray
	| TypeSlice
	| TypeReference
	| TypeDereferenceConst
	| TypeDereferenceMut
	| TypeParenthesized;
interface NTMap {
	0: MissingNode;
	1: SourceFile;
	2: Shebang;
	3: Program;
	4: Snippet;
	5: Comment;
	6: Identifier;
	7: Index;
	8: LbIdentifier;
	9: McIdentifier;
	10: Literal;
	11: ItemPath;
	12: PunctuationToken;
	13: DelimGroup;
	14: Attribute;
	15: DocCommentAttribute;
	16: MacroInvocation;
	17: MacroRulesDeclaration;
	18: MacroDeclaration;
	19: MacroRuleDeclaration;
	20: MacroInlineRuleDeclaration;
	21: MacroGroup;
	22: MacroParameterDeclaration;
	23: PubSpecifier;
	24: ExternSpecifier;
	25: ExpressionStatement;
	26: UseStatement;
	27: NamedImport;
	28: AmbientImport;
	29: AnonymousImport;
	30: DestructuredImport;
	31: ExternCrateStatement;
	32: ExternBlockDeclaration;
	33: TypeAliasDeclaration;
	34: ConstVariableDeclaration;
	35: StaticVariableDeclaration;
	36: LetVariableDeclaration;
	37: ModuleDeclaration;
	38: FunctionDeclaration;
	39: FunctionSelfParameterDeclaration;
	40: FunctionParameterDeclaration;
	41: FunctionSpread;
	42: StructDeclaration;
	43: TupleStructDeclaration;
	44: StructPropertyDeclaration;
	45: TupleStructItemDeclaration;
	46: UnionDeclaration;
	47: EnumDeclaration;
	48: EnumMemberDeclaration;
	49: EnumMemberTupleDeclaration;
	50: EnumMemberStructDeclaration;
	51: TraitDeclaration;
	52: AutoTraitDeclaration;
	53: TraitAliasDeclaration;
	54: ImplDeclaration;
	55: NegativeImplDeclaration;
	56: ExpressionTypeSelector;
	57: ExpressionTypeCast;
	58: ExpressionPath;
	59: ExpressionAsTypeCast;
	60: ReturnExpression;
	61: BreakExpression;
	62: ContinueExpression;
	63: YieldExpression;
	64: CallExpression;
	65: MemberExpression;
	66: AwaitExpression;
	67: UnwrapExpression;
	68: ParenthesizedExpression;
	69: MinusExpression;
	70: NotExpression;
	71: OrExpression;
	72: AndExpression;
	73: ReassignmentExpression;
	74: UnassignedExpression;
	75: OperationExpression;
	76: ReassignmentOperationExpression;
	77: ComparisonExpression;
	78: LetScrutinee;
	79: ClosureFunctionExpression;
	80: ClosureFunctionParameterDeclaration;
	81: BlockExpression;
	82: LoopBlockExpression;
	83: WhileBlockExpression;
	84: ForInBlockExpression;
	85: TryBlockExpression;
	86: IfBlockExpression;
	87: MatchExpression;
	88: MatchExpressionCase;
	89: RangeLiteral;
	90: StructLiteral;
	91: StructLiteralProperty;
	92: StructLiteralPropertyShorthand;
	93: StructLiteralPropertySpread;
	94: StructLiteralRestUnassigned;
	95: TupleLiteral;
	96: ArrayLiteral;
	97: SizedArrayLiteral;
	98: ReferenceExpression;
	99: RawReferenceExpression;
	100: DereferenceExpression;
	101: BoxExpression;
	102: UnionPattern;
	103: ParenthesizedPattern;
	104: RestPattern;
	105: WildcardPattern;
	106: PatternVariableDeclaration;
	107: StructPattern;
	108: StructPatternPropertyDestructured;
	109: StructPatternPropertyShorthand;
	110: TuplePattern;
	111: ArrayPattern;
	112: ReferencePattern;
	113: BoxPattern;
	114: MinusPattern;
	115: RangePattern;
	116: TypePath;
	117: TypeCall;
	118: TypeCallNamedArgument;
	119: TypeCallNamedBound;
	120: LtIdentifier;
	121: LtElided;
	122: LtStatic;
	123: TypeNever;
	124: TypeInferred;
	125: GenericTypeParameterDeclaration;
	126: ConstTypeParameterDeclaration;
	127: GenericLtParameterDeclaration;
	128: WhereTypeBoundDeclaration;
	129: WhereLtBoundDeclaration;
	130: TypeTraitBound;
	131: TypeDynBounds;
	132: TypeImplBounds;
	133: TypeFnPointer;
	134: TypeFnPointerParameter;
	135: TypeFunction;
	136: TypeTuple;
	137: TypeSizedArray;
	138: TypeSlice;
	139: TypeReference;
	140: TypeDereferenceConst;
	141: TypeDereferenceMut;
	142: TypeParenthesized;
}
declare enum Feature {
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
interface FG_Map {
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
declare enum PRCD {
	ScrutineeDefault = 0,
	"Scrutinee ||" = 1,
	"Scrutinee &&" = 2,
	Default = 3,
	"=" = 4,
	".." = 5,
	"||" = 6,
	"&&" = 7,
	"==" = 8,
	"|" = 9,
	"^" = 10,
	"&" = 11,
	">>" = 12,
	"+-" = 13,
	"*/%" = 14,
	"as" = 15,
	Unary = 16,
	Top = 17,
}
declare type missing = MissingNode;
declare type Segment = AttrSegment | MacroInvokeSegment | MacroMatchSegment | MacroTransformSegment;
declare type AttrSegment = TokenNode | DelimGroup<AttrSegment>;
declare type MacroInvokeSegment = TokenNode | DelimGroup<MacroInvokeSegment>;
declare type MacroMatchSegment = TokenNode | DelimGroup<MacroMatchSegment> | MacroGroup<MacroMatchSegment> | MacroParameterDeclaration;
declare type MacroTransformSegment = TokenNode | DelimGroup<MacroTransformSegment> | MacroGroup<MacroTransformSegment> | McIdentifier;
declare type TokenNode = PunctuationToken | Literal | Identifier | LtIdentifier;
declare type MacroSeparator = TokenNode;
declare type ImportNode = NamedImport | AmbientImport | AnonymousImport | DestructuredImport;
declare type StatementNode =
	| ExpressionStatement
	| MacroRulesDeclaration
	| MacroDeclaration
	| UseStatement
	| ExternCrateStatement
	| ExternBlockDeclaration
	| ModuleDeclaration
	| TypeAliasDeclaration
	| LetVariableDeclaration
	| ConstVariableDeclaration
	| StaticVariableDeclaration
	| FunctionDeclaration
	| StructDeclaration
	| TupleStructDeclaration
	| TraitDeclaration
	| AutoTraitDeclaration
	| TraitAliasDeclaration
	| ImplDeclaration
	| NegativeImplDeclaration
	| EnumDeclaration
	| UnionDeclaration;
declare type ExpressionNode =
	| MacroInvocation
	| Identifier
	| Literal
	| ExpressionPath
	| ExpressionTypeCast
	| ExpressionTypeSelector
	| ExpressionAsTypeCast
	| ReturnExpression
	| BreakExpression
	| ContinueExpression
	| YieldExpression
	| CallExpression
	| MemberExpression
	| MemberExpression
	| AwaitExpression
	| UnwrapExpression
	| MinusExpression
	| NotExpression
	| ClosureFunctionExpression
	| BlockExpression
	| LoopBlockExpression
	| WhileBlockExpression
	| ForInBlockExpression
	| TryBlockExpression
	| IfBlockExpression
	| MatchExpression
	| OrExpression<ExpressionNode, ExpressionNode>
	| AndExpression<ExpressionNode, ExpressionNode>
	| ReassignmentExpression
	| OperationExpression
	| ReassignmentOperationExpression
	| ComparisonExpression
	| RangeLiteral
	| StructLiteral
	| TupleLiteral
	| ArrayLiteral
	| SizedArrayLiteral
	| ReferenceExpression
	| RawReferenceExpression
	| DereferenceExpression
	| BoxExpression
	| ParenthesizedExpression<ExpressionNode>;
declare type PatternNode =
	| MacroInvocation
	| Identifier
	| Literal
	| ExpressionPath
	| ExpressionTypeCast
	| ExpressionTypeSelector
	| MinusPattern<Literal>
	| RangePattern
	| BlockExpression
	| PatternVariableDeclaration
	| StructPattern
	| TuplePattern
	| ArrayPattern
	| RestPattern
	| WildcardPattern
	| ReferencePattern
	| BoxPattern
	| UnionPattern
	| ParenthesizedPattern<PatternNode>;
declare type TypeNode =
	| MacroInvocation
	| Identifier
	| TypePath
	| TypeCall
	| ExpressionTypeSelector
	| TypeNever
	| TypeInferred
	| TypeDynBounds
	| TypeImplBounds
	| TypeFnPointer
	| TypeFunction
	| TypeSizedArray
	| TypeSlice
	| TypeTuple
	| TypeReference
	| TypeDereferenceMut
	| TypeDereferenceConst
	| TypeParenthesized<TypeNode>;
declare type AttributeOrComment = Comment | Attribute | DocCommentAttribute;
declare type AttributeOrDocComment = Attribute | DocCommentAttribute;
declare type CommentOrDocComment = Comment | DocCommentAttribute;
declare type IdentifierOrIndex = Identifier | Index;
declare type IdentifierOrItemPath = Identifier | ItemPath;
declare type _PathBase = Identifier | ExpressionTypeSelector;
/** What can be on the left of 'PathExprSegment' */
declare type _ExprPathSource<T extends _PathBase> = T | ExpressionPath<T> | ExpressionTypeCast<T>;
/** What can be on the left of 'TypePathSegment' */
declare type _TypePathSource<T extends _PathBase> = T | TypePath<T> | TypeCall<T> | TypeFunction<T>;
/** 'PathInExpression' */
declare type ExpressionNamespaceTargetNoSelector = _ExprPathSource<Identifier>;
/** 'TypePath' */
declare type TypeNamespaceTargetNoSelector = _TypePathSource<Identifier>;
/** 'PathExpression' === ('PathInExpression' | 'QualifiedPathInExpression')  === 'PathPattern' */
declare type ExpressionNamespaceTarget = _ExprPathSource<_PathBase>;
/** 'TypePath' | 'QualifiedTypeInExpression' */
declare type TypeNamespaceTarget = _TypePathSource<_PathBase>;
/** 'PatternNoTopAlt */
declare type PatternNoUnion = Exclude<PatternNode, UnionPattern>;
/** 'PatternWithoutRange' */
declare type PatternNoUnionNoRange = Exclude<PatternNoUnion, RangePattern>;
declare type MaybePubNode = Extract<Node, PubTarget>;
declare type MaybeExternNode = Extract<Node, ExternTarget>;
declare type MaybeConstNode = Extract<Node, ConstModifier>;
declare type MaybeAsyncNode = Extract<Node, AsyncModifier>;
declare type MaybeMoveNode = Extract<Node, MoveModifier>;
declare type MaybeUnsafeNode = Extract<Node, UnsafeModifier>;
declare type MaybeStaticNode = Extract<Node, StaticModifier>;
declare type MacroRule = MacroRuleDeclaration | MacroInlineRuleDeclaration;
declare type PathNode = ItemPath | ExpressionPath | TypePath;
declare type RangeNode = RangeLiteral | RangePattern;
declare type FunctionNode = FunctionDeclaration | ClosureFunctionExpression;
declare type TypeFunctionNode = TypeFunction | TypeFnPointer;
declare type ParenthesizedNode = ParenthesizedExpression | ParenthesizedPattern | TypeParenthesized;
declare type ObjectNode = Extract<Node, MaybeObjectLike>;
declare type ArrayLikeNode = Extract<Node, ArrayLike>;
declare type ArrayOrTupleLiteral = ArrayLiteral | TupleLiteral;
declare type TupleNode = TupleStructDeclaration | EnumMemberTupleDeclaration | TupleLiteral | TuplePattern | TypeTuple;
declare type DeclarationNode = Extract<Node, DeclarationLike>;
declare type TraitDeclarationNode = TraitDeclaration | AutoTraitDeclaration | TraitAliasDeclaration;
declare type ImplDeclarationNode = ImplDeclaration | NegativeImplDeclaration;
declare type NodeWithSegments = Extract<Node, DelimitedSequence<any, any>>;
declare type NodeWithBody = Extract<Node, MaybeBlockBody>;
declare type NodeWithBodyOrCases = NodeWithBody | MatchExpression;
declare type NodeWithBodyNoBody = AutoTraitDeclaration | NegativeImplDeclaration;
declare type NodeWithCondition = Extract<Node, MaybeConditionBody>;
declare type ImplicitReturnAbleNode = FunctionDeclaration | BlockExpression | IfBlockExpression | MatchExpression | TryBlockExpression;
declare type ExpressionWithBody = Extract<ExpressionNode, MaybeBlockBody>;
declare type ExpressionWithBodyOrCases = ExpressionWithBody | MatchExpression;
declare type LogicalExpression = OrExpression | AndExpression;
declare type LeftRightExpression = Extract<Node, LeftRightLike>;
declare type FlowControlExpression = ReturnExpression | ContinueExpression | BreakExpression | YieldExpression;
declare type UnaryExpression =
	| NotExpression
	| MinusExpression
	| ReferenceExpression
	| DereferenceExpression
	| RawReferenceExpression
	| BoxExpression;
declare type UnaryPattern = ReferencePattern | BoxPattern | MinusPattern;
declare type UnaryType = TypeReference | TypeDereferenceMut | TypeDereferenceConst;
declare type PostfixExpression = UnwrapExpression | AwaitExpression;
declare type VariableDeclarationNode = LetScrutinee | LetVariableDeclaration | ConstVariableDeclaration | StaticVariableDeclaration;
declare type ReassignmentNode = ReassignmentExpression | ReassignmentOperationExpression;
declare type NodeWithTypeBounds = Extract<Node, TypeBoundsConstaint>;
declare type TypeBoundsStandaloneNode = Extract<TypeNode, TypeBoundsStandalone>;
declare type FunctionLikeNode = Extract<Node, FunctionLike>;
declare type FunctionParameterNode =
	| FunctionSelfParameterDeclaration
	| FunctionParameterDeclaration
	| ClosureFunctionParameterDeclaration
	| TypeFnPointerParameter
	| TypeNode
	| FunctionSpread;
declare type NodeWithMaybePatternNoUnionBody =
	| LetVariableDeclaration
	| ConstVariableDeclaration
	| StaticVariableDeclaration
	| FunctionParameterDeclaration
	| ClosureFunctionParameterDeclaration
	| PatternVariableDeclaration
	| UnaryPattern;
interface Located {
	loc: Loc;
}
interface Delimited<K extends keyof DelimKindMap = DelimKind> {
	dk: DelimKindMap[K];
}
interface LocArray<T extends Node = Node, K extends keyof DelimKindMap = keyof DelimKindMap> extends Array<T>, Located, Delimited<K> {}
interface DelimitedSequence<T extends Segment, TK extends keyof DelimKindMap = "()" | "[]" | "{}"> {
	segments: LocArray<T, TK>;
}
interface __DevonlyObject {
	stats: {
		[statsName: string]: string | number;
	};
}
interface ProgramLike {
	ast: Node | LocArray<any, any>;
	danglingAttributes: AttributeOrDocComment[];
	comments: Comment[];
	__devonly?: __DevonlyObject;
}
interface IdentifierLike {
	name: string;
}
interface MaybeIdentifiable<T extends IdentifierLike = Identifier> {
	id: T | undefined;
}
interface Identifiable<T extends IdentifierLike = Identifier> extends MaybeIdentifiable<T> {
	id: T;
}
interface LtIdentifiable {
	id: LtIdentifier;
}
interface PathLike {
	namespace: Identifier | unknown | undefined;
	segment: Identifier;
}
interface AttributeTarget {
	attributes?: AttributeOrDocComment[];
}
interface PubTarget {
	pub?: PubSpecifier;
}
interface ExternTarget {
	extern?: ExternSpecifier;
}
interface ConstModifier {
	const?: true;
}
interface AsyncModifier {
	async?: true;
}
interface MoveModifier {
	move?: true;
}
interface UnsafeModifier {
	unsafe?: true;
}
interface StaticModifier {
	static?: true;
}
interface RefAble {
	ref: boolean;
}
interface MutAble {
	mut: boolean;
}
interface LtAble {
	lt: Lifetime | undefined;
}
interface TargetsLifetime {
	ltTarget: Lifetime;
}
interface MaybeHasLtBounds {
	ltBounds: Lifetime[] | undefined;
}
interface HasLtBounds extends MaybeHasLtBounds {
	ltBounds: Lifetime[];
}
interface ForLtParametersBody {
	ltParameters: LocArray<GenericLtParameterDeclaration, "<>"> | undefined;
}
interface CommentLike {
	line: boolean;
	value: string;
}
interface AttributeLike extends DelimitedSequence<AttrSegment, "None" | "[]"> {
	inner: boolean;
	segments: LocArray<AttrSegment, "None" | "[]">;
}
interface AbiTarget {
	abi: Literal | undefined;
}
interface Callable {
	callee: Node;
	arguments: LocArray<Node, "()">;
}
interface MaybeLabelTarget {
	label: LbIdentifier | undefined;
}
interface MaybeBlockBody {
	body: LocArray<StatementNode, "None" | "{}"> | undefined;
}
interface BlockBody extends MaybeBlockBody {
	body: LocArray<StatementNode, "None" | "{}">;
}
interface BlockLike extends MaybeLabelTarget, BlockBody {}
interface MaybeConditionBody {
	condition: ConditionExpressionXS | undefined;
}
interface ConditionBody extends MaybeConditionBody {
	condition: ConditionExpressionXS;
}
interface MaybeExpressionBody {
	expression: ExpressionNode | undefined;
}
interface ExpressionBody extends MaybeExpressionBody {
	expression: ExpressionNode;
}
interface LeftRightLike {
	kind: keyof typeof TK;
	left: Node | undefined;
	tk: TK;
	right: Node | undefined;
}
interface RangeLike {
	lower: Node | undefined;
	last: boolean;
	upper: Node | undefined;
}
interface SizeExpressionBody {
	sizeExpression: ExpressionNode;
}
interface ArrayLike {
	items: LocArray<Node, "()" | "[]">;
}
interface TupleLike extends ArrayLike {
	items: LocArray<Node, "()">;
}
interface MaybeObjectLike {
	properties: LocArray<Node, "{}"> | undefined;
}
interface ObjectLike extends MaybeObjectLike {
	properties: LocArray<Node, "{}">;
}
interface PropertyLike {
	key: IdentifierOrIndex;
}
interface MaybeReturnTypeConstraint {
	returnType: TypeNode | undefined;
}
interface FunctionLike extends MaybeReturnTypeConstraint {
	parameters: LocArray<Node, "()" | "||">;
	returnType: TypeNode | undefined;
}
interface MaybeGenericArgsTarget {
	typeArguments: LocArray<TypeCallArgument, "<>"> | undefined;
}
interface GenericArgsTarget extends MaybeGenericArgsTarget {
	typeArguments: LocArray<TypeCallArgument, "<>">;
}
interface TypeCallable<T = TypeNode> extends GenericArgsTarget {
	typeCallee: T;
}
interface MaybeGenericsDeclaration {
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
}
interface MaybeWhereBoundsDeclaration {
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
}
interface DeclarationLike extends MaybeWhereBoundsDeclaration, MaybeWhereBoundsDeclaration {}
interface TypeTargetConstraint {
	typeTarget: TypeNode;
}
interface MaybeTypeExpressionBody<T = TypeNode> {
	typeExpression: T | undefined;
}
interface TypeExpressionBody<T = TypeNode> extends MaybeTypeExpressionBody<T> {
	typeExpression: T;
}
interface MaybeTypeAnnotationTarget<T = TypeNode> {
	typeAnnotation: T | undefined;
}
interface TypeAnnotationTarget<T = TypeNode> extends MaybeTypeAnnotationTarget<T> {
	typeAnnotation: T;
}
interface TypeBoundsConstaint {
	typeBounds: TypeBound[] | undefined;
}
interface TypeBoundsStandalone extends TypeBoundsConstaint {
	typeBounds: TypeBound[];
}
interface MaybePatternBody {
	pattern: PatternNode | undefined;
}
interface PatternBody extends MaybePatternBody {
	pattern: PatternNode;
}
interface MaybePatternNoUnionBody extends MaybePatternBody {
	pattern: PatternNoUnion | undefined;
}
interface PatternNoUnionBody extends MaybePatternBody {
	pattern: PatternNoUnion;
}
declare enum TK {
	None = 0,
	"." = 1,
	"&&" = 2,
	"||" = 3,
	"=" = 4,
	"+" = 5,
	"-" = 6,
	"*" = 7,
	"/" = 8,
	"%" = 9,
	"&" = 10,
	"|" = 11,
	"^" = 12,
	"<<" = 13,
	">>" = 14,
	"==" = 15,
	"!=" = 16,
	">" = 17,
	">=" = 18,
	"<" = 19,
	"<=" = 20,
	"+=" = 21,
	"-=" = 22,
	"*=" = 23,
	"/=" = 24,
	"%=" = 25,
	"&=" = 26,
	"|=" = 27,
	"^=" = 28,
	"<<=" = 29,
	">>=" = 30,
	"$" = 31,
	"@" = 32,
	"_" = 33,
	".." = 34,
	"..." = 35,
	"..=" = 36,
	"," = 37,
	";" = 38,
	":" = 39,
	"::" = 40,
	"#" = 41,
	"?" = 42,
	"!" = 43,
	"=>" = 44,
	"->" = 45,
	"~" = 46,
}
declare type ReaOpExpr =
	| typeof TK["+="]
	| typeof TK["-="]
	| typeof TK["*="]
	| typeof TK["/="]
	| typeof TK["%="]
	| typeof TK["&="]
	| typeof TK["|="]
	| typeof TK["^="]
	| typeof TK["<<="]
	| typeof TK[">>="];
declare type OpExpr =
	| typeof TK["+"]
	| typeof TK["-"]
	| typeof TK["*"]
	| typeof TK["/"]
	| typeof TK["%"]
	| typeof TK["&"]
	| typeof TK["|"]
	| typeof TK["^"]
	| typeof TK["<<"]
	| typeof TK[">>"];
declare type CompExpr = typeof TK["=="] | typeof TK["!="] | typeof TK[">"] | typeof TK[">="] | typeof TK["<"] | typeof TK["<="];
interface DelimKindMap {
	None: 0;
	"()": 1;
	"[]": 2;
	"{}": 3;
	"<>": 4;
	"||": 5;
	0: 0;
	1: 1;
	2: 2;
	3: 3;
	4: 4;
	5: 5;
}
interface DelimNameMap {
	0: "None";
	1: "()";
	2: "[]";
	3: "{}";
	4: "<>";
	5: "||";
}
declare enum DelimKind {
	None = 0,
	"()" = 1,
	"[]" = 2,
	"{}" = 3,
	"<>" = 4,
	"||" = 5,
}
declare enum TyMacroMatch {
	/** undefined										*/ None = 0,
	/** anything										*/ "tt" = 1,
	/** Attribute										*/ "meta" = 2,
	/** PubSpecifier									*/ "vis" = 3,
	/** Identifier										*/ "ident" = 4,
	/** LtIdentifier									*/ "lifetime" = 5,
	/** Literal | MinusExpression<Literal>				*/ "literal" = 6,
	/** StatementNode									*/ "stmt" = 7,
	/** StatementNode subset valid in Toplevel			*/ "item" = 8,
	/** ExpressionNode									*/ "expr" = 9,
	/** ExpressionNode & BlockLike						*/ "block" = 10,
	/** PatternNode (PatternNoUnion pre edition 2021)	*/ "pat" = 11,
	/** PatternNoUnion									*/ "pat_param" = 12,
	/** TypeNode										*/ "ty" = 13,
	/** TypeNamespaceTargetNoSelector					*/ "path" = 14,
}
declare enum LiteralKind {
	/** i.e. `false`						*/ False = 0,
	/** i.e. `true`							*/ True = 1,
	/** e.g. `'A'`							*/ Char = 2,
	/** e.g. `b'A'`							*/ bChar = 3,
	/** e.g. `b"ABCDE"`						*/ bString = 4,
	/** e.g. `br#"AAAA"#` 					*/ brString = 5,
	/** e.g. `r#"hello"#` 					*/ rString = 6,
	/** e.g. `"hello"` 						*/ String = 7,
	/** e.g. `0b1111_0000` 	(int suffix) 	*/ Binary = 8,
	/** e.g. `0xff` 		(int suffix)	*/ Hex = 9,
	/** e.g. `0o77` 		(int suffix)	*/ Octal = 10,
	/** e.g. `123` 			(int suffix)	*/ Integer = 11,
	/** e.g. `7.6e+54` 		(float suffix) 	*/ Float = 12,
}
declare class Loc {
	readonly src: SourceFile;
	protected readonly 0: number;
	protected readonly 1: number;
	protected readonly 2?: number;
	constructor(src: SourceFile, start: number, end: number);
	len(): number;
	isBefore(target: Located): boolean;
	isAfter(target: Located): boolean;
	contains(target: Located): boolean;
	ownContains(target: Located): boolean;
	isBetween(left: Located, right: Located): boolean;
	url(): string;
	getText(): string;
	getOwnText(): string;
	sliceText(startIndex?: number, endIndex?: number): string;
	clone(): Loc;
	cloneFrom(startPos: number): Loc;
	toJSON(): Omit<
		this,
		| "toJSON"
		| "url"
		| "src"
		| "len"
		| "isBefore"
		| "isAfter"
		| "contains"
		| "ownContains"
		| "isBetween"
		| "getText"
		| "getOwnText"
		| "sliceText"
		| "clone"
		| "cloneFrom"
	>;
}
declare class BaseNode<NT extends NodeType = NodeType> implements Located {
	get type(): typeof NodeType[NT];
	readonly nodeType: NT;
	readonly loc: Loc;
	constructor(nodeType: NT, loc: Loc);
	toJSON(): any;
}
declare class MissingNode extends BaseNode<0> implements IdentifierLike {
	get name(): "";
}
declare class SourceFile extends BaseNode<1> {
	code: string;
	filepath: string | undefined;
	parserOptions: ParserOptions;
	lineStarts: number[];
	program: Program;
	UTF8BOM?: true;
	shebang?: Shebang;
	l(index: number): number;
	lc(index: number): {
		line: number;
		char: number;
	};
	url(index?: number): string;
}
/** #!value (e.g. "#!/usr/bin/env rustx") */
declare class Shebang extends BaseNode<2> {
	get value(): string;
}
/** ...body */
declare class Program extends BaseNode<3> implements ProgramLike, BlockBody, AttributeTarget {
	get body(): LocArray<StatementNode, "None">;
	ast: LocArray<StatementNode, "None">;
	danglingAttributes: AttributeOrDocComment[];
	comments: Comment[];
	attributes?: AttributeOrDocComment[];
	__devonly?: __DevonlyObject;
}
declare class Snippet<T extends Node | LocArray = Node | LocArray> extends BaseNode<4> implements ProgramLike, AttributeTarget {
	ast: T;
	danglingAttributes: AttributeOrDocComment[];
	comments: Comment[];
	attributes?: AttributeOrDocComment[];
	__devonly?: __DevonlyObject;
}
declare class Comment extends BaseNode<5> implements CommentLike {
	line: boolean;
	get value(): string;
}
declare class Identifier extends BaseNode<6> implements IdentifierLike {
	get name(): string;
}
declare class Index extends BaseNode<7> implements IdentifierLike {
	get name(): `${number}`;
}
declare class LbIdentifier extends BaseNode<8> implements IdentifierLike {
	get name(): `'${string}`;
}
declare class McIdentifier extends BaseNode<9> implements IdentifierLike {
	get name(): `$${string}`;
}
declare class Literal extends BaseNode<10> {
	kind: LiteralKind;
	get value(): string;
	suffix?: Identifier;
}
/** namespace?::segment */
declare class ItemPath extends BaseNode<11> implements PathLike {
	namespace: IdentifierOrItemPath | undefined;
	segment: Identifier;
}
/** token */
declare class PunctuationToken extends BaseNode<12> {
	tk: TK;
	get token():
		| ""
		| "/"
		| "||"
		| "."
		| "&&"
		| "="
		| "+"
		| "-"
		| "*"
		| "%"
		| "&"
		| "|"
		| "^"
		| "<<"
		| ">>"
		| "=="
		| "!="
		| ">"
		| ">="
		| "<"
		| "<="
		| "+="
		| "-="
		| "*="
		| "/="
		| "%="
		| "&="
		| "|="
		| "^="
		| "<<="
		| ">>="
		| "$"
		| "@"
		| "_"
		| ".."
		| "..."
		| "..="
		| ","
		| ";"
		| ":"
		| "::"
		| "#"
		| "?"
		| "!"
		| "=>"
		| "->"
		| "~";
}
declare class DelimGroup<T extends Segment = Segment> extends BaseNode<13> implements DelimitedSequence<T> {
	segments: LocArray<T, "()" | "[]" | "{}">;
}
/** #!?[expression] ... */
declare class Attribute extends BaseNode<14> implements AttributeLike, CommentLike {
	inner: boolean;
	segments: LocArray<AttrSegment, "[]">;
	get value(): string;
	get line(): boolean;
}
/**
 * /// ...
 * /** ... * /
 * //! ...	  (inner)
 * /*! ... * / (inner)
 */
declare class DocCommentAttribute extends BaseNode<15> implements DelimitedSequence<AttrSegment, "None">, AttributeLike, CommentLike {
	inner: boolean;
	line: boolean;
	get value(): string;
	get segments(): LocArray<Literal, "None">;
}
/**
 * callee!(...segments);?
 * callee![...segments];?
 * callee! { ...segments }
 */
declare class MacroInvocation extends BaseNode<16> implements DelimitedSequence<MacroInvokeSegment> {
	callee: Identifier | PathNode;
	segments: LocArray<MacroInvokeSegment, "()" | "[]" | "{}">;
}
/** macro_rules! id { ...rules } */
declare class MacroRulesDeclaration extends BaseNode<17> implements Identifiable, AttributeTarget {
	id: Identifier;
	rules: LocArray<MacroRuleDeclaration, "()" | "[]" | "{}">;
	attributes?: AttributeOrDocComment[];
}
/** macro id ...rules */
declare class MacroDeclaration extends BaseNode<18> implements Identifiable, AttributeTarget, PubTarget {
	id: Identifier;
	rules: LocArray<MacroRuleDeclaration, "{}"> | MacroInlineRuleDeclaration;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** (...match) => (...transform); */
declare class MacroRuleDeclaration extends BaseNode<19> {
	match: LocArray<MacroMatchSegment, "()" | "[]" | "{}">;
	transform: LocArray<MacroTransformSegment, "()" | "[]" | "{}">;
}
/** (...match) { ...transform } */
declare class MacroInlineRuleDeclaration extends BaseNode<20> {
	match: LocArray<MacroMatchSegment, "()">;
	transform: LocArray<MacroTransformSegment, "{}">;
}
/** $(...segments)sep?kind */
declare class MacroGroup<T extends Segment = Segment> extends BaseNode<21> implements DelimitedSequence<T> {
	segments: LocArray<T, "()">;
	sep: MacroSeparator | undefined;
	kind: "*" | "+" | "?";
}
/** id: ty */
declare class MacroParameterDeclaration extends BaseNode<22> implements Identifiable<McIdentifier> {
	id: McIdentifier;
	ty: Identifier | missing;
}
/**
 * pub(in? location)? ...
 * {@link Feature.crate_visibility_modifier} `crate ...` (location is Identifier<"crate">)
 */
declare class PubSpecifier extends BaseNode<23> {
	location: IdentifierOrItemPath | undefined;
}
/** extern abi? ... */
declare class ExternSpecifier extends BaseNode<24> {
	abi: Literal | undefined;
}
/** expression?;? */
declare class ExpressionStatement<T extends ExpressionNode | undefined = ExpressionNode | undefined>
	extends BaseNode<25>
	implements MaybeExpressionBody, AttributeTarget
{
	expression: T;
	semi: boolean;
	attributes?: AttributeOrDocComment[];
}
/** use import; */
declare class UseStatement extends BaseNode<26> implements AttributeTarget, PubTarget {
	import: ImportNode;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** source as local? */
declare class NamedImport extends BaseNode<27> {
	source: IdentifierOrItemPath;
	local: Identifier | undefined;
}
/** source::?* */
declare class AmbientImport extends BaseNode<28> {
	source: IdentifierOrItemPath | undefined;
}
/** source as _ */
declare class AnonymousImport extends BaseNode<29> {
	source: IdentifierOrItemPath;
}
/** source?::{ ...specifiers } */
declare class DestructuredImport extends BaseNode<30> {
	source: IdentifierOrItemPath | undefined;
	specifiers: LocArray<ImportNode, "{}">;
}
/** extern crate import as local?; */
declare class ExternCrateStatement extends BaseNode<31> implements AttributeTarget, PubTarget {
	import: AnonymousImport | NamedImport;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** extern abi? { ...body }? */
declare class ExternBlockDeclaration extends BaseNode<32> implements BlockBody, AttributeTarget, PubTarget, UnsafeModifier {
	abi: Literal | undefined;
	body: LocArray<StatementNode, "{}">;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
	unsafe?: true;
}
/** type id<...generics>?: ...typeBounds? where ...whereBounds? = typeExpression?; */
declare class TypeAliasDeclaration
	extends BaseNode<33>
	implements Identifiable, DeclarationLike, TypeBoundsConstaint, MaybeTypeExpressionBody, AttributeTarget, PubTarget
{
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	typeBounds: TypeBound[] | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	typeExpression: TypeNode | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** const pattern: typeAnnotation = expression?; */
declare class ConstVariableDeclaration
	extends BaseNode<34>
	implements PatternNoUnionBody, TypeAnnotationTarget<TypeNode | missing>, MaybeExpressionBody, AttributeTarget, PubTarget
{
	pattern: Identifier;
	typeAnnotation: TypeNode | missing;
	expression: ExpressionNode | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** static pattern: typeAnnotation = expression?; */
declare class StaticVariableDeclaration
	extends BaseNode<35>
	implements PatternNoUnionBody, TypeAnnotationTarget<TypeNode | missing>, MaybeExpressionBody, AttributeTarget, PubTarget
{
	pattern: Identifier | PatternVariableDeclaration;
	typeAnnotation: TypeNode | missing;
	expression: ExpressionNode | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** let pattern: typeAnnotation? = expression?; */
declare class LetVariableDeclaration
	extends BaseNode<36>
	implements PatternNoUnionBody, MaybeTypeAnnotationTarget, MaybeExpressionBody, AttributeTarget
{
	pattern: PatternNoUnion;
	typeAnnotation: TypeNode | undefined;
	expression: ExpressionNode | undefined;
	/** {@link Feature.let_else} */ else: BlockExpression | undefined;
	attributes?: AttributeOrDocComment[];
}
/** mod id { ...body }? */
declare class ModuleDeclaration extends BaseNode<37> implements Identifiable, MaybeBlockBody, AttributeTarget, PubTarget, UnsafeModifier {
	id: Identifier;
	body: LocArray<StatementNode, "{}"> | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
	unsafe?: true;
}
/** fn id<...generics>?(...parameters) -> returnType? where ...whereBounds? { ...body }? */
declare class FunctionDeclaration
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
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	parameters: FunctionDeclarationParameters;
	returnType: TypeNode | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	body: LocArray<StatementNode, "{}"> | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
	const?: true;
	async?: true;
	unsafe?: true;
	extern?: ExternSpecifier;
	toJSON(): {
		"parameters.self": FunctionSelfParameterDeclaration | undefined;
	} & this;
}
interface FunctionDeclarationParameters extends LocArray<FunctionParameterDeclaration | FunctionSpread, "()"> {
	self: FunctionSelfParameterDeclaration | undefined;
}
/** &?lt? mut? self: typeAnnotation? */
declare class FunctionSelfParameterDeclaration
	extends BaseNode<39>
	implements RefAble, LtAble, MutAble, MaybeTypeAnnotationTarget, AttributeTarget
{
	ref: boolean;
	lt: Lifetime | undefined;
	mut: boolean;
	typeAnnotation: TypeNode | undefined;
	attributes?: AttributeOrDocComment[];
}
/** pattern: typeAnnotation */
declare class FunctionParameterDeclaration
	extends BaseNode<40>
	implements PatternNoUnionBody, TypeAnnotationTarget<TypeNode | FunctionSpread | missing>, AttributeTarget
{
	pattern: PatternNoUnion;
	typeAnnotation: TypeNode | FunctionSpread | missing;
	attributes?: AttributeOrDocComment[];
}
/** ... */
declare class FunctionSpread extends BaseNode<41> implements AttributeTarget {
	attributes?: AttributeOrDocComment[];
}
/** struct id<...generics>? where ...whereBounds? { ...properties }? */
declare class StructDeclaration extends BaseNode<42> implements Identifiable, DeclarationLike, MaybeObjectLike, AttributeTarget, PubTarget {
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	properties: LocArray<StructPropertyDeclaration, "{}"> | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** struct id<...generics>?(...items) where ...whereBounds?; */
declare class TupleStructDeclaration extends BaseNode<43> implements Identifiable, DeclarationLike, ArrayLike, AttributeTarget, PubTarget {
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	items: LocArray<TupleStructItemDeclaration, "()">;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** id: typeAnnotation */
declare class StructPropertyDeclaration
	extends BaseNode<44>
	implements Identifiable, TypeAnnotationTarget<TypeNode | missing>, AttributeTarget, PubTarget
{
	id: Identifier;
	typeAnnotation: TypeNode | missing;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
declare class TupleStructItemDeclaration extends BaseNode<45> implements TypeAnnotationTarget, AttributeTarget, PubTarget {
	typeAnnotation: TypeNode;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** union id<...generics>? where ...whereBounds? { ...properties } */
declare class UnionDeclaration extends BaseNode<46> implements Identifiable, DeclarationLike, ObjectLike, AttributeTarget, PubTarget {
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	properties: LocArray<StructPropertyDeclaration, "{}">;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** enum id<...generics>? where ...whereBounds? { ...members } */
declare class EnumDeclaration extends BaseNode<47> implements Identifiable, DeclarationLike, AttributeTarget, PubTarget {
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	members: LocArray<EnumDeclarationMember, "{}">;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
declare type EnumDeclarationMember = EnumMemberDeclaration | EnumMemberTupleDeclaration | EnumMemberStructDeclaration;
/** id = value? */
declare class EnumMemberDeclaration extends BaseNode<48> implements Identifiable, AttributeTarget, PubTarget {
	id: Identifier;
	value: ExpressionNode | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** id(...items) = value? */
declare class EnumMemberTupleDeclaration extends BaseNode<49> implements Identifiable, ArrayLike, AttributeTarget, PubTarget {
	id: Identifier;
	items: LocArray<TupleStructItemDeclaration, "()">;
	/** `= ...` {@link Feature.arbitrary_enum_discriminant} */ value: ExpressionNode | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** id {...properties} = value? */
declare class EnumMemberStructDeclaration extends BaseNode<50> implements Identifiable, ObjectLike, AttributeTarget, PubTarget {
	id: Identifier;
	properties: LocArray<StructPropertyDeclaration, "{}">;
	/** `= ...` {@link Feature.arbitrary_enum_discriminant} */ value: ExpressionNode | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** trait id<...generics>?: ...typeBounds? where ...whereBounds? { ...body } */
declare class TraitDeclaration
	extends BaseNode<51>
	implements Identifiable, BlockBody, DeclarationLike, TypeBoundsConstaint, AttributeTarget, PubTarget, UnsafeModifier
{
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	typeBounds: TypeBound[] | undefined;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	body: LocArray<StatementNode, "{}">;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
	unsafe?: true;
}
/** auto trait id {} */
declare class AutoTraitDeclaration extends BaseNode<52> implements Identifiable, AttributeTarget, PubTarget, UnsafeModifier {
	id: Identifier;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
	unsafe?: true;
}
/** trait id<...generics>? = ...typeBounds where ...whereBounds?; */
declare class TraitAliasDeclaration
	extends BaseNode<53>
	implements Identifiable, DeclarationLike, TypeBoundsConstaint, AttributeTarget, PubTarget, UnsafeModifier
{
	id: Identifier;
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	typeBounds: TypeBound[];
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
	unsafe?: true;
}
/**
 * impl<...generics>? const? trait for typeTarget where ...whereBounds? { ...body }
 * impl<...generics>?                  typeTarget where ...whereBounds? { ...body }
 */
declare class ImplDeclaration
	extends BaseNode<54>
	implements BlockBody, TypeTargetConstraint, DeclarationLike, AttributeTarget, PubTarget, UnsafeModifier
{
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	/** `impl<...> const ... for ...` {@link Feature.const_trait_impl} 	*/ const: boolean;
	trait: TypeNamespaceTargetNoSelector | undefined;
	typeTarget: TypeNode;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	body: LocArray<StatementNode, "{}">;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
	unsafe?: true;
}
/** impl<...generics>? !trait for typeTarget where ...whereBounds? {} */
declare class NegativeImplDeclaration extends BaseNode<55> implements TypeTargetConstraint, DeclarationLike, AttributeTarget, PubTarget {
	generics: LocArray<GenericParameterDeclaration, "<>"> | undefined;
	trait: TypeNamespaceTargetNoSelector;
	typeTarget: TypeNode;
	whereBounds: LocArray<WhereBoundDeclaration, "None"> | undefined;
	attributes?: AttributeOrDocComment[];
	pub?: PubSpecifier;
}
/** <typeTarget as typeExpression?> */
declare class ExpressionTypeSelector extends BaseNode<56> implements TypeTargetConstraint, MaybeTypeExpressionBody {
	typeTarget: TypeNode;
	typeExpression: TypeNamespaceTargetNoSelector | undefined;
}
/** typeCallee::<...typeArguments> */
declare class ExpressionTypeCast<T extends _PathBase = _PathBase> extends BaseNode<57> implements TypeCallable<_ExprPathSource<T>> {
	typeCallee: _ExprPathSource<T>;
	typeArguments: LocArray<TypeCallArgument, "<>">;
}
/** namespace::segment */
declare class ExpressionPath<T extends _PathBase = _PathBase> extends BaseNode<58> implements PathLike {
	namespace: _ExprPathSource<T> | undefined;
	segment: Identifier;
}
/** 'TypeCastExpression' */
/** expression as typeExpression */
declare class ExpressionAsTypeCast extends BaseNode<59> implements ExpressionBody, TypeExpressionBody {
	expression: ExpressionNode;
	typeExpression: TypeNode;
}
/** return expression? */
declare class ReturnExpression extends BaseNode<60> implements MaybeExpressionBody {
	expression: ExpressionNode | undefined;
}
/** break label? expression? */
declare class BreakExpression extends BaseNode<61> implements MaybeExpressionBody, MaybeLabelTarget {
	label: LbIdentifier | undefined;
	expression: ExpressionNode | undefined;
}
/** continue label? */
declare class ContinueExpression extends BaseNode<62> implements MaybeLabelTarget {
	label: LbIdentifier | undefined;
}
/** yield expression */
declare class YieldExpression extends BaseNode<63> implements MaybeExpressionBody {
	expression: ExpressionNode | undefined;
}
/** callee.method?::<...typeArguments>?(...arguments) */
declare class CallExpression extends BaseNode<64> implements MaybeGenericArgsTarget {
	callee: ExpressionNode;
	method: Identifier | undefined;
	typeArguments: LocArray<TypeCallArgument, "<>"> | undefined;
	arguments: LocArray<ExpressionNode, "()">;
}
/**
 * expression.property
 * expression[property] // computed
 */
declare class MemberExpression extends BaseNode<65> implements ExpressionBody {
	expression: ExpressionNode;
	computed: boolean;
	property: IdentifierOrIndex | ExpressionNode;
}
/** expression.await */
declare class AwaitExpression extends BaseNode<66> implements ExpressionBody {
	expression: ExpressionNode;
}
/** expression? */
declare class UnwrapExpression extends BaseNode<67> implements ExpressionBody {
	expression: ExpressionNode;
}
/** (expression) */
declare class ParenthesizedExpression<T extends ExpressionNode = ExpressionNode> extends BaseNode<68> implements ExpressionBody {
	expression: T;
}
/** -expression */
declare class MinusExpression<T extends ExpressionNode = ExpressionNode> extends BaseNode<69> implements ExpressionBody {
	expression: T;
}
/** !expression */
declare class NotExpression extends BaseNode<70> implements ExpressionBody {
	expression: ExpressionNode;
}
/** left || right */
declare class OrExpression<L extends ConditionExpression = ConditionExpression, R extends ConditionExpression = ConditionExpression>
	extends BaseNode<71>
	implements LeftRightLike
{
	get kind(): "||";
	left: L;
	tk: TK;
	right: R;
}
/** left && right */
declare class AndExpression<L extends ConditionExpression = ConditionExpression, R extends ConditionExpression = ConditionExpression>
	extends BaseNode<72>
	implements LeftRightLike
{
	get kind(): "&&";
	left: L;
	tk: TK;
	right: R;
}
/** left = right */
declare class ReassignmentExpression extends BaseNode<73> implements LeftRightLike {
	get kind(): "=";
	left: ExpressionNode | UnassignedExpression;
	tk: TK;
	right: ExpressionNode;
}
/** _ */
declare class UnassignedExpression extends BaseNode<74> {}
/** e.g. left + right */
declare class OperationExpression extends BaseNode<75> implements LeftRightLike {
	get kind(): "/" | "+" | "-" | "*" | "%" | "&" | "|" | "^" | "<<" | ">>";
	left: ExpressionNode;
	tk: TK & OpExpr;
	right: ExpressionNode;
}
/** e.g. left += right */
declare class ReassignmentOperationExpression extends BaseNode<76> implements LeftRightLike {
	get kind(): "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=";
	left: ExpressionNode;
	tk: TK & ReaOpExpr;
	right: ExpressionNode;
}
/** e.g. left == right */
declare class ComparisonExpression extends BaseNode<77> implements LeftRightLike {
	get kind(): "==" | "!=" | ">" | ">=" | "<" | "<=";
	left: ExpressionNode;
	tk: TK & CompExpr;
	right: ExpressionNode;
}
/** let pattern = expression */
declare class LetScrutinee extends BaseNode<78> implements PatternBody {
	pattern: PatternNode;
	expression: ScrutineeExpression;
}
declare type ScrutineeExpression = Exclude<ExpressionNode, OrExpression | AndExpression>;
declare type ExpressionNodeXS = Exclude<ExpressionNode, StructLiteral>;
declare type ConditionExpression =
	| ExpressionNode
	| LetScrutinee
	| AndExpression<ConditionExpression, ConditionExpression>
	| OrExpression<ConditionExpression, ConditionExpression>;
declare type ConditionExpressionXS =
	| ExpressionNodeXS
	| LetScrutinee
	| AndExpression<ConditionExpressionXS, ConditionExpressionXS>
	| OrExpression<ConditionExpressionXS, ConditionExpressionXS>;
/** |...| -> returnType? expression */
declare class ClosureFunctionExpression
	extends BaseNode<79>
	implements FunctionLike, ExpressionBody, MoveModifier, StaticModifier, AsyncModifier
{
	parameters: LocArray<ClosureFunctionParameterDeclaration, "||">;
	returnType: TypeNode | undefined;
	expression: ExpressionNode;
	move?: true;
	/** `static |...| ...` {@link Feature.generators}    */ static?: true;
	/** ` async |...| ...` {@link Feature.async_closure} */ async?: true;
}
/** pattern: typeAnnotation? */
declare class ClosureFunctionParameterDeclaration
	extends BaseNode<80>
	implements PatternNoUnionBody, MaybeTypeAnnotationTarget, AttributeTarget
{
	pattern: PatternNoUnion;
	typeAnnotation: TypeNode | undefined;
	attributes?: AttributeOrDocComment[];
}
/** label?: { ...body } */
declare class BlockExpression
	extends BaseNode<81>
	implements BlockLike, AttributeTarget, AsyncModifier, MoveModifier, UnsafeModifier, ConstModifier
{
	/** jinx-rust only */ label: LbIdentifier | undefined;
	body: LocArray<StatementNode, "{}">;
	attributes?: AttributeOrDocComment[];
	async?: true;
	move?: true;
	unsafe?: true;
	/** `const { ... }` {@link Feature.inline_const} */ const?: true;
}
/** label?: loop { ...body } */
declare class LoopBlockExpression extends BaseNode<82> implements BlockLike, AttributeTarget {
	label: LbIdentifier | undefined;
	body: LocArray<StatementNode, "{}">;
	attributes?: AttributeOrDocComment[];
}
/** label?: while condition { ...body } */
declare class WhileBlockExpression extends BaseNode<83> implements BlockLike, ConditionBody, AttributeTarget {
	label: LbIdentifier | undefined;
	condition: ConditionExpressionXS;
	body: LocArray<StatementNode, "{}">;
	attributes?: AttributeOrDocComment[];
}
/** label?: for pattern in expression { ...body } */
declare class ForInBlockExpression extends BaseNode<84> implements BlockLike, AttributeTarget {
	label: LbIdentifier | undefined;
	pattern: PatternNode;
	expression: ExpressionNodeXS;
	body: LocArray<StatementNode, "{}">;
	attributes?: AttributeOrDocComment[];
}
/** try { ...body } */
declare class TryBlockExpression extends BaseNode<85> implements BlockLike, AttributeTarget {
	/** jinx-rust only */ label: LbIdentifier | undefined;
	body: LocArray<StatementNode, "{}">;
	attributes?: AttributeOrDocComment[];
}
/** label?: if condition { ...body } else? */
declare class IfBlockExpression extends BaseNode<86> implements BlockLike, ConditionBody, AttributeTarget {
	/** jinx-rust only */ label: LbIdentifier | undefined;
	condition: ConditionExpressionXS;
	body: LocArray<StatementNode, "{}">;
	else: IfBlockExpression | BlockExpression | undefined;
	attributes?: AttributeOrDocComment[];
}
/** match expression { ...cases } */
declare class MatchExpression extends BaseNode<87> implements MaybeLabelTarget, ExpressionBody, AttributeTarget {
	/** jinx-rust only */ label: LbIdentifier | undefined;
	expression: ExpressionNodeXS;
	cases: LocArray<MatchExpressionCase, "{}">;
	attributes?: AttributeOrDocComment[];
}
/** pattern if condition? => expression */
declare class MatchExpressionCase extends BaseNode<88> implements PatternBody, MaybeConditionBody, ExpressionBody, AttributeTarget {
	pattern: PatternNode;
	/** {@link Feature.if_let_guard} */ condition: ConditionExpressionXS | undefined;
	expression: ExpressionNode;
	attributes?: AttributeOrDocComment[];
}
/** .. | min.. | min..max | min..=max | ..max | ..=max */
declare class RangeLiteral extends BaseNode<89> implements RangeLike {
	lower: ExpressionNode | undefined;
	last: boolean;
	upper: ExpressionNode | undefined;
}
/**
 * struct { ...properties }
 * Note: Struct(...) is parsed as a CallExpression
 */
declare class StructLiteral extends BaseNode<90> implements ObjectLike {
	struct: ExpressionNamespaceTargetNoSelector;
	properties: LocArray<StructProperty, "{}">;
}
declare type StructProperty =
	| StructLiteralProperty
	| StructLiteralPropertyShorthand
	| StructLiteralPropertySpread
	| StructLiteralRestUnassigned;
/** key: value */
declare class StructLiteralProperty extends BaseNode<91> implements PropertyLike, AttributeTarget {
	key: IdentifierOrIndex;
	value: ExpressionNode;
	attributes?: AttributeOrDocComment[];
}
/** value */
declare class StructLiteralPropertyShorthand extends BaseNode<92> implements AttributeTarget {
	value: Identifier;
	attributes?: AttributeOrDocComment[];
}
/** ..expression */
declare class StructLiteralPropertySpread extends BaseNode<93> implements ExpressionBody {
	expression: ExpressionNode;
}
/** .. */
declare class StructLiteralRestUnassigned extends BaseNode<94> {}
/** (...items) | (item,) */
declare class TupleLiteral extends BaseNode<95> implements ArrayLike {
	items: LocArray<ExpressionNode, "()">;
}
/** [...items] */
declare class ArrayLiteral extends BaseNode<96> implements ArrayLike {
	items: LocArray<ExpressionNode, "[]">;
}
/** [initExpression; sizeExpression] */
declare class SizedArrayLiteral extends BaseNode<97> implements SizeExpressionBody {
	initExpression: ExpressionNode;
	sizeExpression: ExpressionNode;
}
/** &mut? expression */
declare class ReferenceExpression extends BaseNode<98> implements MutAble, ExpressionBody {
	mut: boolean;
	expression: ExpressionNode;
}
/** &raw kind expression */
declare class RawReferenceExpression extends BaseNode<99> implements ExpressionBody {
	kind: "mut" | "const";
	expression: ExpressionNode;
}
/** *expression */
declare class DereferenceExpression extends BaseNode<100> implements ExpressionBody {
	expression: ExpressionNode;
}
/** box expression */
declare class BoxExpression extends BaseNode<101> implements ExpressionBody {
	expression: ExpressionNode;
}
/** ...patterns */
declare class UnionPattern extends BaseNode<102> {
	patterns: PatternNode[];
}
/** (pattern) */
declare class ParenthesizedPattern<T extends PatternNode = PatternNode> extends BaseNode<103> implements PatternBody {
	pattern: T;
}
/**
 * ..
 * (attributes are parsed but ignored when property of StructPattern)
 */
declare class RestPattern extends BaseNode<104> {}
/** _ */
declare class WildcardPattern extends BaseNode<105> {}
/** ref? mut? id @ pattern? */
declare class PatternVariableDeclaration extends BaseNode<106> implements Identifiable, RefAble, MutAble, MaybePatternNoUnionBody {
	ref: boolean;
	mut: boolean;
	id: Identifier;
	pattern: PatternNoUnion | undefined;
}
/** struct { ...properties } */
declare class StructPattern extends BaseNode<107> implements ObjectLike {
	struct: ExpressionNamespaceTargetNoSelector;
	properties: LocArray<StructPatternProperty, "{}">;
}
declare type StructPatternProperty = StructPatternPropertyDestructured | StructPatternPropertyShorthand | RestPattern;
/** key: pattern */
declare class StructPatternPropertyDestructured extends BaseNode<108> implements PropertyLike, PatternBody, AttributeTarget {
	key: IdentifierOrIndex;
	pattern: PatternNode;
	attributes?: AttributeOrDocComment[];
}
/** box? ref? mut? id */
declare class StructPatternPropertyShorthand extends BaseNode<109> implements Identifiable, AttributeTarget {
	/** `box ...` {@link Feature.box_patterns} */ box: boolean;
	ref: boolean;
	mut: boolean;
	id: Identifier;
	attributes?: AttributeOrDocComment[];
}
/** struct?(...items) | (item,) | (..) */
declare class TuplePattern extends BaseNode<110> implements ArrayLike {
	struct: ExpressionNamespaceTargetNoSelector | undefined;
	items: LocArray<PatternNode, "()">;
}
/** [...items] */
declare class ArrayPattern extends BaseNode<111> implements ArrayLike {
	items: LocArray<PatternNode, "[]">;
}
/** &mut? pattern */
declare class ReferencePattern extends BaseNode<112> implements MutAble, PatternBody {
	mut: boolean;
	pattern: PatternNoUnionNoRange;
}
/** box pattern */
declare class BoxPattern extends BaseNode<113> implements PatternBody {
	pattern: PatternNoUnionNoRange;
}
/** -pattern */
declare class MinusPattern<T extends Literal = Literal> extends BaseNode<114> implements PatternBody {
	pattern: T;
}
/**
 * min.. | min...max | min..=max
 * exclusive_range_pattern  | min..max
 * half_open_range_patterns | ..max | ...max | ..=max
 */
declare class RangePattern extends BaseNode<115> implements RangeLike {
	lower: RangePatternBound | undefined;
	last: boolean;
	upper: RangePatternBound | undefined;
}
declare type RangePatternBound = BlockExpression | ExpressionNamespaceTarget | MinusPattern<Literal> | Literal;
/** namespace?::segment */
declare class TypePath<T extends _PathBase = _PathBase> extends BaseNode<116> implements PathLike {
	namespace: _TypePathSource<T> | undefined;
	segment: Identifier;
}
/**
 * typeCallee::<...typeArguments>
 * typeCallee<...typeArguments>
 */
declare class TypeCall<T extends _PathBase = _PathBase> extends BaseNode<117> implements TypeCallable<_TypePathSource<T>> {
	typeCallee: _TypePathSource<T>;
	typeArguments: LocArray<TypeCallArgument, "<>">;
}
declare type TypeCallArgument =
	| Literal
	| Lifetime
	| MinusExpression<Literal>
	| BlockExpression
	| TypeCallNamedArgument
	| TypeCallNamedBound
	| TypeNode;
/** target = typeExpression */
declare class TypeCallNamedArgument extends BaseNode<118> implements TypeExpressionBody {
	target: Identifier;
	typeExpression: TypeNode;
}
/** typeTarget: ...typeBounds */
declare class TypeCallNamedBound extends BaseNode<119> implements TypeTargetConstraint, TypeBoundsConstaint {
	typeTarget: TypeNode;
	typeBounds: TypeBound[];
}
declare type Lifetime = LtIdentifier | LtElided | LtStatic;
declare class LtIdentifier extends BaseNode<120> implements IdentifierLike {
	get name(): `'${string}`;
}
/** '_ */
declare class LtElided extends BaseNode<121> {}
/** 'static */
declare class LtStatic extends BaseNode<122> {}
/** ! */
declare class TypeNever extends BaseNode<123> {}
/** _ */
declare class TypeInferred extends BaseNode<124> {}
declare type GenericParameterDeclaration = GenericTypeParameterDeclaration | ConstTypeParameterDeclaration | GenericLtParameterDeclaration;
/** id: ...typeBounds? = typeDefault? */
declare class GenericTypeParameterDeclaration extends BaseNode<125> implements Identifiable, TypeBoundsConstaint, AttributeTarget {
	id: Identifier;
	typeBounds: TypeBound[] | undefined;
	typeDefault: TypeNode | undefined;
	attributes?: AttributeOrDocComment[];
}
/** const id: typeAnnotation = typeDefault? */
declare class ConstTypeParameterDeclaration
	extends BaseNode<126>
	implements Identifiable, TypeAnnotationTarget<TypeNode | missing>, AttributeTarget
{
	id: Identifier;
	typeAnnotation: TypeNode | missing;
	/** `const T = ...` {@link Feature.const_generics_defaults} */ typeDefault: ConstTypeParamDefault | undefined;
	attributes?: AttributeOrDocComment[];
}
declare type ConstTypeParamDefault = Literal | MinusExpression<Literal> | BlockExpression | TypeNode;
/** 'LifetimeParam' */
/** id: ltBounds? */
declare class GenericLtParameterDeclaration extends BaseNode<127> implements LtIdentifiable, MaybeHasLtBounds, AttributeTarget {
	id: LtIdentifier;
	ltBounds: Lifetime[] | undefined;
	attributes?: AttributeOrDocComment[];
}
/** 'WhereClauseItem' */
declare type WhereBoundDeclaration = WhereTypeBoundDeclaration | WhereLtBoundDeclaration;
/** 'TypeBoundWhereClauseItem' */
/** for<...ltParameters>? typeTarget: ...typeBounds? */
declare class WhereTypeBoundDeclaration extends BaseNode<128> implements ForLtParametersBody, TypeTargetConstraint, TypeBoundsConstaint {
	ltParameters: LocArray<GenericLtParameterDeclaration, "<>"> | undefined;
	typeTarget: TypeNode;
	typeBounds: TypeBound[];
}
/** 'LifetimeWhereClauseItem' */
/** ltTarget: ltBounds */
declare class WhereLtBoundDeclaration extends BaseNode<129> implements TargetsLifetime, HasLtBounds {
	ltTarget: Lifetime;
	ltBounds: Lifetime[];
}
/** 'TypeParamBound' */
declare type TypeBound = TypeTraitBound | Lifetime | TypeParenthesized<TypeTraitBound>;
/** ~const? ??for<...ltParameters>? typeExpression */
declare class TypeTraitBound extends BaseNode<130> implements ForLtParametersBody, TypeExpressionBody {
	/** `~const` {@link Feature.const_trait_impl} */ maybeConst: boolean;
	/** e.g. `?Sized`                             */ optional: boolean;
	ltParameters: LocArray<GenericLtParameterDeclaration, "<>"> | undefined;
	typeExpression: TypeNamespaceTargetNoSelector;
}
/** dyn? ...typeBounds */
declare class TypeDynBounds extends BaseNode<131> implements TypeBoundsStandalone {
	dyn: boolean;
	typeBounds: TypeBound[];
}
/** impl ...typeBounds */
declare class TypeImplBounds extends BaseNode<132> implements TypeBoundsStandalone {
	typeBounds: TypeBound[];
}
/** for<...ltParameters>? fn(...parameters) -> returnType? */
declare class TypeFnPointer extends BaseNode<133> implements ForLtParametersBody, FunctionLike, UnsafeModifier, ExternTarget {
	ltParameters: LocArray<GenericLtParameterDeclaration, "<>"> | undefined;
	parameters: LocArray<TypeFnPointerParameter | FunctionSpread, "()">;
	returnType: TypeNode | undefined;
	unsafe?: true;
	extern?: ExternSpecifier;
}
/** id?: typeAnnotation */
declare class TypeFnPointerParameter extends BaseNode<134> implements MaybeIdentifiable, TypeAnnotationTarget, AttributeTarget {
	id: Identifier | undefined;
	typeAnnotation: TypeNode;
	attributes?: AttributeOrDocComment[];
}
/** callee(...parameters) -> returnType? */
declare class TypeFunction<T extends _PathBase = _PathBase> extends BaseNode<135> implements FunctionLike {
	callee: _TypePathSource<T>;
	parameters: LocArray<TypeNode, "()">;
	returnType: TypeNode | undefined;
}
/** (...items) | (item,) */
declare class TypeTuple extends BaseNode<136> implements ArrayLike {
	items: LocArray<TypeNode, "()">;
}
/** [typeExpression; sizeExpression] */
declare class TypeSizedArray extends BaseNode<137> implements TypeExpressionBody, SizeExpressionBody {
	typeExpression: TypeNode;
	sizeExpression: ExpressionNode;
}
/** [typeExpression] */
declare class TypeSlice extends BaseNode<138> implements TypeExpressionBody {
	typeExpression: TypeNode;
}
/** &lt? mut? typeExpression */
declare class TypeReference extends BaseNode<139> implements LtAble, MutAble, TypeExpressionBody {
	lt: Lifetime | undefined;
	mut: boolean;
	typeExpression: TypeNode;
}
/** *const typeExpression */
declare class TypeDereferenceConst extends BaseNode<140> implements TypeExpressionBody {
	typeExpression: TypeNode;
}
/** *mut typeExpression */
declare class TypeDereferenceMut extends BaseNode<141> implements TypeExpressionBody {
	typeExpression: TypeNode;
}
/** (typeExpression) */
declare class TypeParenthesized<T extends TypeNode | TypeTraitBound = TypeNode | TypeTraitBound>
	extends BaseNode<142>
	implements TypeExpressionBody<T>
{
	typeExpression: T;
}

interface CFLoc {
	line: number;
	column: number;
}
interface CFLocSpan {
	url: string;
	start: CFLoc;
	end?: CFLoc;
}
interface ParserError extends Error {
	loc: CFLocSpan;
	ctx?: any[];
	/** devonly */ parserState?: {
		[state_name: string]: unknown;
	};
}

declare function mockNode<NT extends NodeType>(nodeType: NT, loc: Loc, obj: Omit<NTMap[NT], keyof BaseNode>): NTMap[NT];
declare function createLocArray<T extends Node, D extends DelimKind>(dk: D, loc: Loc, array?: T[]): LocArray<T, DelimNameMap[D]>;

declare function parseFile(code: string, options?: ParserOptions): SourceFile;
declare function toExpression(tokens: LocArray<any, DelimNameMap[Exclude<DelimKind, DelimKind.None>]>): Snippet<ExpressionNode>;
declare function toCallExpressionArguments<T extends LocArray>(tokens: T): Snippet<LocArray<ExpressionNode, DelimNameMap[T["dk"]]>>;
declare function toBlockBody<T extends LocArray>(tokens: T): Snippet<LocArray<StatementNode, DelimNameMap[T["dk"]]>>;
declare function toTokens(node: Node): Snippet<LocArray<AttrSegment, "None">>;

type index_ParserError = ParserError;
type index_ParserOptions = ParserOptions;
declare const index_parseFile: typeof parseFile;
declare const index_toBlockBody: typeof toBlockBody;
declare const index_toCallExpressionArguments: typeof toCallExpressionArguments;
declare const index_toExpression: typeof toExpression;
declare const index_toTokens: typeof toTokens;
declare const index_mockNode: typeof mockNode;
declare const index_createLocArray: typeof createLocArray;
declare namespace index {
	export {
		index_ParserError as ParserError,
		index_ParserOptions as ParserOptions,
		index_parseFile as parseFile,
		index_toBlockBody as toBlockBody,
		index_toCallExpressionArguments as toCallExpressionArguments,
		index_toExpression as toExpression,
		index_toTokens as toTokens,
		index_mockNode as mockNode,
		index_createLocArray as createLocArray,
	};
}

export {
	AbiTarget,
	AmbientImport,
	AndExpression,
	AnonymousImport,
	ArrayLike,
	ArrayLikeNode,
	ArrayLiteral,
	ArrayOrTupleLiteral,
	ArrayPattern,
	AsyncModifier,
	AttrSegment,
	Attribute,
	AttributeLike,
	AttributeOrComment,
	AttributeOrDocComment,
	AttributeTarget,
	AutoTraitDeclaration,
	AwaitExpression,
	BaseNode,
	BlockBody,
	BlockExpression,
	BlockLike,
	BoxExpression,
	BoxPattern,
	BreakExpression,
	CallExpression,
	Callable,
	ClosureFunctionExpression,
	ClosureFunctionParameterDeclaration,
	Comment,
	CommentLike,
	CommentOrDocComment,
	ComparisonExpression,
	ConditionBody,
	ConditionExpression,
	ConditionExpressionXS,
	ConstModifier,
	ConstTypeParamDefault,
	ConstTypeParameterDeclaration,
	ConstVariableDeclaration,
	ContinueExpression,
	DeclarationLike,
	DeclarationNode,
	DelimGroup,
	DelimKind,
	DelimKindMap,
	DelimNameMap,
	Delimited,
	DelimitedSequence,
	DereferenceExpression,
	DestructuredImport,
	DocCommentAttribute,
	EnumDeclaration,
	EnumDeclarationMember,
	EnumMemberDeclaration,
	EnumMemberStructDeclaration,
	EnumMemberTupleDeclaration,
	ExpressionAsTypeCast,
	ExpressionBody,
	ExpressionNamespaceTarget,
	ExpressionNamespaceTargetNoSelector,
	ExpressionNode,
	ExpressionNodeXS,
	ExpressionPath,
	ExpressionStatement,
	ExpressionTypeCast,
	ExpressionTypeSelector,
	ExpressionWithBody,
	ExpressionWithBodyOrCases,
	ExternBlockDeclaration,
	ExternCrateStatement,
	ExternSpecifier,
	ExternTarget,
	FG_Map,
	Feature,
	FlowControlExpression,
	ForInBlockExpression,
	ForLtParametersBody,
	FunctionDeclaration,
	FunctionDeclarationParameters,
	FunctionLike,
	FunctionLikeNode,
	FunctionNode,
	FunctionParameterDeclaration,
	FunctionParameterNode,
	FunctionSelfParameterDeclaration,
	FunctionSpread,
	GenericArgsTarget,
	GenericLtParameterDeclaration,
	GenericParameterDeclaration,
	GenericTypeParameterDeclaration,
	HasLtBounds,
	Identifiable,
	Identifier,
	IdentifierLike,
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
	LeftRightLike,
	LetScrutinee,
	LetVariableDeclaration,
	Lifetime,
	Literal,
	LiteralKind,
	Loc,
	LocArray,
	Located,
	LogicalExpression,
	LoopBlockExpression,
	LtAble,
	LtElided,
	LtIdentifiable,
	LtIdentifier,
	LtStatic,
	MacroDeclaration,
	MacroGroup,
	MacroInlineRuleDeclaration,
	MacroInvocation,
	MacroInvokeSegment,
	MacroMatchSegment,
	MacroParameterDeclaration,
	MacroRule,
	MacroRuleDeclaration,
	MacroRulesDeclaration,
	MacroSeparator,
	MacroTransformSegment,
	MatchExpression,
	MatchExpressionCase,
	MaybeAsyncNode,
	MaybeBlockBody,
	MaybeConditionBody,
	MaybeConstNode,
	MaybeExpressionBody,
	MaybeExternNode,
	MaybeGenericArgsTarget,
	MaybeGenericsDeclaration,
	MaybeHasLtBounds,
	MaybeIdentifiable,
	MaybeLabelTarget,
	MaybeMoveNode,
	MaybeObjectLike,
	MaybePatternBody,
	MaybePatternNoUnionBody,
	MaybePubNode,
	MaybeReturnTypeConstraint,
	MaybeStaticNode,
	MaybeTypeAnnotationTarget,
	MaybeTypeExpressionBody,
	MaybeUnsafeNode,
	MaybeWhereBoundsDeclaration,
	McIdentifier,
	MemberExpression,
	MinusExpression,
	MinusPattern,
	MissingNode,
	ModuleDeclaration,
	MoveModifier,
	MutAble,
	NTMap,
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
	ObjectLike,
	ObjectNode,
	OperationExpression,
	OrExpression,
	PRCD,
	ParenthesizedExpression,
	ParenthesizedNode,
	ParenthesizedPattern,
	PathLike,
	PathNode,
	PatternBody,
	PatternNoUnion,
	PatternNoUnionBody,
	PatternNoUnionNoRange,
	PatternNode,
	PatternVariableDeclaration,
	PostfixExpression,
	Program,
	ProgramLike,
	PropertyLike,
	PubSpecifier,
	PubTarget,
	PunctuationToken,
	RangeLike,
	RangeLiteral,
	RangeNode,
	RangePattern,
	RangePatternBound,
	RawReferenceExpression,
	ReassignmentExpression,
	ReassignmentNode,
	ReassignmentOperationExpression,
	RefAble,
	ReferenceExpression,
	ReferencePattern,
	RestPattern,
	ReturnExpression,
	ScrutineeExpression,
	Segment,
	Shebang,
	SizeExpressionBody,
	SizedArrayLiteral,
	Snippet,
	SourceFile,
	StatementNode,
	StaticModifier,
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
	TargetsLifetime,
	TokenNode,
	TraitAliasDeclaration,
	TraitDeclaration,
	TraitDeclarationNode,
	TryBlockExpression,
	TupleLike,
	TupleLiteral,
	TupleNode,
	TuplePattern,
	TupleStructDeclaration,
	TupleStructItemDeclaration,
	TyMacroMatch,
	TypeAliasDeclaration,
	TypeAnnotationTarget,
	TypeBound,
	TypeBoundsConstaint,
	TypeBoundsStandalone,
	TypeBoundsStandaloneNode,
	TypeCall,
	TypeCallArgument,
	TypeCallNamedArgument,
	TypeCallNamedBound,
	TypeCallable,
	TypeDereferenceConst,
	TypeDereferenceMut,
	TypeDynBounds,
	TypeExpressionBody,
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
	TypeTargetConstraint,
	TypeTraitBound,
	TypeTuple,
	UnaryExpression,
	UnaryPattern,
	UnaryType,
	UnassignedExpression,
	UnionDeclaration,
	UnionPattern,
	UnsafeModifier,
	UnwrapExpression,
	UseStatement,
	VariableDeclarationNode,
	WhereBoundDeclaration,
	WhereLtBoundDeclaration,
	WhereTypeBoundDeclaration,
	WhileBlockExpression,
	WildcardPattern,
	YieldExpression,
	_ExprPathSource,
	_PathBase,
	_TypePathSource,
	__DevonlyObject,
	missing,
	index as rs,
};
