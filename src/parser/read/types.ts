import { Identifier, read_ccPath_or_Identifier, read_maybe_missing } from ".";
import { is_BareTypeTraitBound, is_Lifetime, is_TypeTraitBound } from "../../utils/ast";
import { CharCode } from "../../utils/CharCode";
import { exit } from "../error";
import * as Nodes from "../nodes";
import { DelimKind, Feature } from "../nodes";
import {
	CCPATH_read,
	check_ahead,
	currChar,
	EDGECASE_STEPBACK_TO,
	FG_property,
	match,
	match_keyword,
	maybe_read,
	maybe_read_2,
	maybe_read_keyword,
	maybe_skip_1_read_2,
	Mc_ctx_isReadingTokens,
	not_match,
	peek_keyword,
	peek_match,
	read,
	readLocatedArrayDelim,
	read_ahead,
	read_between,
	read_cached_keyword,
	read_keyword,
	read_sequence,
	safe_skip,
	safe_skip_keyword,
	safe_step_over,
	sequence_hasTrailingComma,
	TY_ctx_allowMultipleBounds,
	TY_withContext,
	will_match_charLiteral_not_lt,
	with_outerAttributes_fromParentContext,
	__inherit_startPos,
} from "../state";
import {
	escapeParens,
	FileLoc,
	FileLocLifetime,
	FileLoc_COPY,
	FileLoc_FromChild,
	FileLoc_FromChildElseReadAhead,
	FileLoc_ReadAhead,
	withEscapedParens,
} from "../state/constructor";
import { Keyword } from "../state/scanner";
import { BlockExpression, ExpressionTypeSelector, MinusExpression, read_expression_between } from "./expressions";
import { Literal } from "./literals";
import { MacroInvocation } from "./macro";
import { read_with_extern_specifier, read_with_move_modifier, read_with_unsafe_modifier } from "./specifiers";
import { FunctionSpread } from "./statements";

//#region ================================================[        Types        ]===========================================================``--.

class LtIdentifier extends FileLocLifetime(Nodes.LtIdentifier) {
	read() {
		// 'x••••••••••••••••
		//  ^- You are here
		read_cached_keyword();
	}
}

class LtElided extends FileLocLifetime(Nodes.LtElided) {
	read() {
		// '_••••••••••••••••
		//  ^- You are here
		safe_skip_keyword(Keyword.Underscore);
	}
}

class LtStatic extends FileLocLifetime(Nodes.LtStatic) {
	read() {
		// 'static•••••••••••
		//  ^- You are here
		safe_skip_keyword(Keyword.static);
	}
}

// prettier-ignore
function read_Lifetime(): Nodes.Lifetime {
	// '••••••••••••••••
	// ^- You are here
	safe_step_over(CharCode["'"]);
	switch (peek_keyword()) {
		case Keyword.static: return Mc_ctx_isReadingTokens() ? LtIdentifier.read() : LtStatic.read();
		case Keyword.Underscore: return Mc_ctx_isReadingTokens() ? LtIdentifier.read() : LtElided.read();
		default: return LtIdentifier.read();
		case Keyword.NotAWord: case Keyword.StringLiteral: exit.expected("Identifier");
	}
}

export function maybe_read_lifetime() {
	return match(CharCode["'"]) ? read_Lifetime() : undefined;
}

export function read_charLiteral_or_lifetime() {
	return will_match_charLiteral_not_lt() ? Literal.read() : read_Lifetime();
}

//#-------------------------------------------------------+        Bounds        +----------------------------------------------------------.

/**/ class GenericTypeParameterDeclaration extends FileLoc(Nodes.GenericTypeParameterDeclaration) {
	read() {
		// < ..., •••••••••••••••••
		//        ^- You are here
		this.id = Identifier.read();
		if (maybe_read(CharCode[":"])) {
			switch (currChar()) {
				case CharCode[","]:
				case CharCode[">"]:
					// 'A: ,' is allowed
					this.typeBounds = [];
					break;
				default:
					this.typeBounds = read_typeBounds();
					break;
			}
		} else {
			// '<>' is allowed
			this.typeBounds = undefined;
		}
		this.typeDefault = maybe_read(CharCode["="]) ? read_type(true) : undefined;
	}
}

/**/ class ConstTypeParameterDeclaration extends FileLoc(Nodes.ConstTypeParameterDeclaration) {
	read() {
		// < ..., const••••••••••••
		//        ^- You are here
		safe_skip_keyword(Keyword.const);
		this.id = Identifier.read();
		this.typeAnnotation = read_maybe_missing(() => maybe_read_typeAnnotation());
		this.typeDefault = FG_property(Feature["const_generics_defaults"], this, "typeDefault", () =>
			maybe_read(CharCode["="]) ? read_FG_typeDefault() : undefined
		);
	}
}

function read_FG_typeDefault(): Nodes.ConstTypeParamDefault {
	// prettier-ignore
	switch (currChar()) {
		case CharCode["-"]: return MinusExpression.read(() => Literal.read()) as Nodes.MinusExpression<Nodes.Literal>;
		case CharCode["{"]: return BlockExpression.read();
		case CharCode["'"]: return read_charLiteral_or_lifetime() as Nodes.Literal;
		case CharCode['"']:
		case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]:
		case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]:
			return Literal.read();
		default:
			switch (peek_keyword()) {
				case Keyword.StringLiteral: case Keyword.false: case Keyword.true: return Literal.read();
				default: return read_type(true);
			}
	}
}

class GenericLtParameterDeclaration extends FileLoc(Nodes.GenericLtParameterDeclaration) {
	read() {
		// < ..., '••••••••••••••••
		//        ^- You are here
		if (not_match(CharCode["'"])) exit.expected("Lifetime");
		this.id = read_Lifetime() as Nodes.LtIdentifier;
		this.ltBounds = maybe_read_ltBounds();
	}
}

/**/ class WhereTypeBoundDeclaration extends FileLoc(Nodes.WhereTypeBoundDeclaration) {
	read() {
		// where ..., •••••••••••••••••
		//            ^- You are here
		this.ltParameters = maybe_read_forLtParameters();
		this.typeTarget = read_type(true);
		read(CharCode[":"]);
		// 'where A: ,' is allowed
		switch (currChar()) {
			case CharCode[","]:
			case CharCode["="]:
			case CharCode["{"]:
			case CharCode[";"]:
				this.typeBounds = [];
				break;
			default:
				this.typeBounds = read_typeBounds();
				break;
		}
	}
}

/**/ class WhereLtBoundDeclaration extends FileLoc(Nodes.WhereLtBoundDeclaration) {
	read() {
		// where ..., '••••••••••••••••
		//            ^- You are here
		this.ltTarget = read_Lifetime();
		this.ltBounds = maybe_read_ltBounds()!;
	}
}

function maybe_read_ltBounds(): Nodes.Lifetime[] | undefined {
	if (maybe_read(CharCode[":"])) {
		const ltBounds: Nodes.Lifetime[] = [];
		if (match(CharCode["'"])) {
			do ltBounds.push(read_Lifetime());
			while (maybe_read(CharCode["+"]) && match(CharCode["'"]));
		}
		return ltBounds;
	}
	return undefined;
}

function read_TypeBound(): Nodes.TypeBound {
	switch (currChar()) {
		case CharCode["("]:
			return escapeParens(
				TypeParenthesized.read(
					readLocatedArrayDelim<Nodes.TypeTraitBound, 1>(DelimKind["()"], (arr) => {
						arr[0] = read_between(CharCode["("], () => read_typeTraitBound(true), CharCode[")"]);
					})
				) as Nodes.TypeParenthesized<Nodes.TypeTraitBound>
			);
		case CharCode["'"]:
			return read_Lifetime();
		default:
			return read_typeTraitBound();
	}
}

export function read_typeBounds() {
	const typeBounds: Nodes.TypeBound[] = [];
	do typeBounds.push(read_TypeBound());
	while (maybe_read_boundContinue());
	return typeBounds;
}

export function maybe_read_colon_typeBounds(): Nodes.TypeBound[] | undefined {
	return maybe_read(CharCode[":"]) ? read_typeBounds() : undefined;
}

//#-------------------------------------------------------+        Common        +----------------------------------------------------------.

class TypePath extends FileLoc_FromChildElseReadAhead(Nodes.TypePath) {
	read(namespace: this["namespace"]) {
		// namespace::•••••••••••••••••
		//            ^- You are here
		this.namespace = namespace;
		this.segment = Identifier.read();
	}
}

class TypeCall extends FileLoc_FromChild(Nodes.TypeCall) {
	read(callee: this["typeCallee"]) {
		// callee<••••••••••••••••
		//       ^- You are here
		this.typeCallee = callee;
		this.typeArguments = read_TypeArguments();
	}
}

export function read_TypeArguments() {
	return read_sequence(DelimKind["<>"], () => read_TypeArgument());
}

function read_TypeArgument(): Nodes.TypeCallArgument {
	// prettier-ignore
	switch (currChar()) {
		case CharCode["-"]: return MinusExpression.read(() => Literal.read()) as Nodes.MinusExpression<Nodes.Literal>;
		case CharCode["{"]: return BlockExpression.read();
		case CharCode["'"]: return read_charLiteral_or_lifetime() as Nodes.Lifetime;
		case CharCode['"']:
		case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]:
		case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]:
			return Literal.read();
		default:
			switch (peek_keyword()) {
				case Keyword.StringLiteral: case Keyword.false: case Keyword.true: return Literal.read();
				default: {
					const lhs = read_type(true);
					switch (currChar()) {
						case CharCode["="]: return TypeCallNamedArgument.read(lhs as Nodes.Identifier);
						case CharCode[":"]: return TypeCallNamedBound.read(lhs);
						default: return lhs;
					}
				}
			}
	}
}

/**/ class TypeCallNamedArgument extends FileLoc_FromChild(Nodes.TypeCallNamedArgument) {
	read(id: this["target"]) {
		// id =••••••••••••••••
		//    ^- You are here
		this.target = id;
		safe_skip(CharCode["="]);
		this.typeExpression = read_type(true);
	}
}

/**/ class TypeCallNamedBound extends FileLoc_FromChild(Nodes.TypeCallNamedBound) {
	read(typeTarget: this["typeTarget"]) {
		// typeTarget :••••••••••••••••
		//            ^- You are here
		this.typeTarget = typeTarget;
		safe_skip(CharCode[":"]);
		this.typeBounds = read_typeBounds();
	}
}

//#-------------------------------------------------------+        Forms        +-----------------------------------------------------------.

class TypeFnPointer extends FileLoc_ReadAhead(Nodes.TypeFnPointer) {
	read(ltParameters: this["ltParameters"]) {
		// for<...ltParameters>? fn(••••••••••••••
		//                       ^- You are here
		this.ltParameters = ltParameters;
		safe_skip_keyword(Keyword.fn);
		this.parameters = read_sequence(DelimKind["()"], () =>
			with_outerAttributes_fromParentContext(() => (match(CharCode["."]) ? FunctionSpread.read() : TypeFnPointerParameter.read()))
		);
		this.returnType = maybe_read_ReturnType(false);
	}
}

/**/ class TypeFnPointerParameter extends FileLoc(Nodes.TypeFnPointerParameter) {
	read() {
		const lhs = read_type(true);
		if (maybe_read(CharCode[":"])) {
			this.id = lhs as Nodes.Identifier;
			this.typeAnnotation = read_type(true);
		} else {
			this.id = undefined;
			this.typeAnnotation = lhs;
		}
	}
}

class TypeFunction extends FileLoc_FromChild(Nodes.TypeFunction) {
	read(callee: this["callee"]) {
		// callee(••••••••••••••••
		//       ^- You are here
		this.callee = callee;
		this.parameters = read_sequence(DelimKind["()"], () => read_type(true));
		this.returnType = maybe_read_ReturnType(false);
	}
}

class TypeSizedArray extends FileLoc_ReadAhead(Nodes.TypeSizedArray) {
	read(typeExpression: this["typeExpression"]) {
		// [typeExpression;••••••••••••••••
		//                ^- You are here
		this.typeExpression = typeExpression;
		this.sizeExpression = read_expression_between(CharCode[";"], CharCode["]"]);
	}
}

class TypeSlice extends FileLoc_ReadAhead(Nodes.TypeSlice) {
	read(typeExpression: this["typeExpression"]) {
		// [typeExpression]
		//                ^- You are here
		this.typeExpression = typeExpression;
		safe_skip(CharCode["]"]);
	}
}

class TypeTuple extends FileLoc_COPY(Nodes.TypeTuple) {
	read(items: this["items"]) {
		this.items = items;
	}
}

//#-------------------------------------------------------+        Bounds        +----------------------------------------------------------.

class TypeDynBounds extends FileLoc_ReadAhead(Nodes.TypeDynBounds) {
	read(dyn: this["dyn"], typeBound: Nodes.TypeBound) {
		this.dyn = dyn;
		this.typeBounds = read_standalone_bounds(typeBound);
	}
}

function asTypeDynBounds(typeBound: Nodes.TypeDynBounds["typeBounds"][number]) {
	return __inherit_startPos(TypeDynBounds.read(false, typeBound), typeBound);
}

function asTypeTraitBound(typeExpression: Nodes.TypeNamespaceTargetNoSelector) {
	return __inherit_startPos(TypeTraitBound.read(false, false, undefined, typeExpression), typeExpression);
}

function read_standalone_bounds(first: Nodes.TypeBound) {
	const typeBounds = [first];
	if (TY_ctx_allowMultipleBounds() || is_Lifetime(first) || (is_TypeTraitBound(first) && !is_BareTypeTraitBound(first))) {
		while (maybe_read_boundContinue()) {
			typeBounds.push(read_TypeBound());
		}
	}
	return typeBounds;
}

class TypeImplBounds extends FileLoc(Nodes.TypeImplBounds) {
	read() {
		// impl•••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.impl);
		this.typeBounds = read_standalone_bounds(read_TypeBound());
	}
}

/**/ class TypeTraitBound extends FileLoc_ReadAhead(Nodes.TypeTraitBound) {
	read(maybeConst: boolean, optional: this["optional"], ltParameters: this["ltParameters"], typeExpression: this["typeExpression"]) {
		this.maybeConst = FG_property(Feature["const_trait_impl"], this, "maybeConst", () => maybeConst);
		this.optional = optional;
		this.ltParameters = ltParameters;
		this.typeExpression = typeExpression;
	}
}

function read_typeTraitBound(allowMultipleBounds = false): Nodes.TypeTraitBound {
	return read_ahead(() =>
		TypeTraitBound.read(
			maybe_read(CharCode["~"]) ? (read_keyword(Keyword.const), true) : false,
			maybe_read(CharCode["?"]),
			maybe_read_forLtParameters(),
			read_type(allowMultipleBounds) as Nodes.TypeNamespaceTargetNoSelector // read_TypeNamespaceTargetNoSelector();
		)
	);
}

//#-------------------------------------------------------+        Memory        +----------------------------------------------------------.

class TypeReference extends FileLoc(Nodes.TypeReference) {
	read() {
		// &••••••••••••••••
		// ^- You are here
		safe_skip(CharCode["&"]);
		this.lt = maybe_read_lifetime();
		this.mut = maybe_read_keyword(Keyword.mut);
		this.typeExpression = read_type(false);
	}
}

class TypeDereferenceConst extends FileLoc_ReadAhead(Nodes.TypeDereferenceConst) {
	read() {
		// *const•••••••••••••••••
		//  ^- You are here
		safe_skip_keyword(Keyword.const);
		this.typeExpression = read_type(false);
	}
}

class TypeDereferenceMut extends FileLoc_ReadAhead(Nodes.TypeDereferenceMut) {
	read() {
		// *mut•••••••••••••••••
		//  ^- You are here
		safe_skip_keyword(Keyword.mut);
		this.typeExpression = read_type(false);
	}
}

//#---------------------------------------------------+        Miscellaneous        +-------------------------------------------------------.

class TypeParenthesized extends FileLoc_COPY(Nodes.TypeParenthesized) {
	read(items: Nodes.LocArray<this["typeExpression"], "()">) {
		this.typeExpression = items[0];
	}
}

class TypeNever extends FileLoc(Nodes.TypeNever) {
	read() {
		// !••••••••••••••••
		// ^- You are here
		safe_skip(CharCode["!"]);
	}
}

class TypeInferred extends FileLoc(Nodes.TypeInferred) {
	read() {
		// _••••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.Underscore);
	}
}

//#endregion ===============================================================================================================================..--'

function read_TypeNamespaceTarget(init: Nodes._PathBase | Nodes.TypePath): Nodes.TypeNamespaceTarget | Nodes.MacroInvocation {
	let lhs = init as Nodes.TypeNamespaceTarget;
	loop: while (true) {
		switch (currChar()) {
			case CharCode["!"]:
				if (peek_match(1, CharCode["="])) break loop;
				lhs = MacroInvocation.read(lhs as any) as any;
				break loop;
			case CharCode[":"]:
				if (maybe_skip_1_read_2(CharCode[":"])) {
					switch (currChar()) {
						case CharCode[":"]:
							EDGECASE_STEPBACK_TO(lhs);
							break loop;
						case CharCode["<"]:
							lhs = TypeCall.read(lhs);
							continue loop;
						default:
							lhs = TypePath.read(lhs);
							continue loop;
					}
				}
				break loop;
			case CharCode["("]:
				lhs = TypeFunction.read(lhs);
				break;
			case CharCode["<"]:
				if (peek_match(1, CharCode["="])) break loop;
				lhs = TypeCall.read(lhs);
				break;
			default:
				break loop;
		}
	}
	return lhs;
}

export function read_TypeNamespaceTargetNoSelector(): Nodes.TypeNamespaceTargetNoSelector {
	return read_TypeNamespaceTarget(read_ccPath_or_Identifier(TypePath)) as Nodes.TypeNamespaceTargetNoSelector;
}

export function maybe_read_generics() {
	return match(CharCode["<"])
		? read_sequence(DelimKind["<>"], () =>
				with_outerAttributes_fromParentContext(() =>
					match(CharCode["'"])
						? GenericLtParameterDeclaration.read()
						: match_keyword(Keyword.const)
						? ConstTypeParameterDeclaration.read()
						: GenericTypeParameterDeclaration.read()
				)
		  )
		: undefined;
}

function read_WhereBoundDeclaration() {
	return match(CharCode["'"])
		? WhereLtBoundDeclaration.read() // 'a: 'b
		: WhereTypeBoundDeclaration.read(); // A: B
}

function match_boundEnd() {
	switch (currChar()) {
		case CharCode[":"]:
			return check_ahead(() => {
				if (maybe_skip_1_read_2(CharCode[":"])) {
					// where A +: ::••••••••••••••••
					// where A + ::•••••••••••••••••
					//             ^- You are here
					return match(CharCode[":"]);
				} else {
					// where A +: •••••••••••••••••
					//            ^- You are here
					return true;
				}
			});
		case CharCode[","]:
		case CharCode[";"]:
		case CharCode["="]:
		case CharCode[">"]:
		case CharCode[")"]:
		case CharCode["{"]:
			return true;
		default:
			return false;
	}
}

function maybe_read_boundContinue() {
	return maybe_read(CharCode["+"]) && !match_boundEnd();
}

export function maybe_read_whereBounds() {
	return match_keyword(Keyword.where)
		? readLocatedArrayDelim<Nodes.WhereBoundDeclaration, DelimKind.None>(DelimKind.None, (whereBounds) => {
				safe_skip_keyword(Keyword.where);
				if (!match_boundEnd()) {
					do whereBounds.push(read_WhereBoundDeclaration());
					while (maybe_read(CharCode[","]) && !match_boundEnd());
				}
		  })
		: undefined;
}

function read_type_lhs(): Nodes.TypeNode {
	switch (currChar()) {
		case CharCode["<"]:
			return read_TypeNamespaceTarget(ExpressionTypeSelector.read());
		case CharCode["["]:
			return read_ahead(() => {
				safe_skip(CharCode["["]);
				const typeExpression = read_type(true);
				return match(CharCode["]"])
					? TypeSlice.read(typeExpression) // [ty]
					: TypeSizedArray.read(typeExpression); // [ty; expr]
			});
		case CharCode["("]: {
			const items = read_sequence(DelimKind["()"], () => read_type(true));
			return items.length !== 1 || sequence_hasTrailingComma(items)
				? TypeTuple.read(items)
				: (TypeParenthesized.read(items) as Nodes.TypeParenthesized<Nodes.TypeNode>);
		}
		case CharCode[":"]:
			return read_TypeNamespaceTarget(CCPATH_read(TypePath)); // ::Foo
		case CharCode["!"]:
			return TypeNever.read(); // !
		case CharCode["&"]:
			return TypeReference.read(); // &Bar
		case CharCode["*"]:
			// prettier-ignore
			return read_ahead(() => {
				safe_skip(CharCode["*"]);
				switch (peek_keyword()) {
					case Keyword.const: return TypeDereferenceConst.read(); // *const Foo
					case Keyword.mut: return TypeDereferenceMut.read(); // *mut Bar
					default: exit.expected(["const", "mut"]);
				}
			});
		case CharCode["'"]:
			return asTypeDynBounds(read_Lifetime()); // 'a + ...
		case CharCode["?"]:
		case CharCode["~"]:
			return asTypeDynBounds(read_typeTraitBound()); // ?Sized | ~const ...
		default:
			switch (peek_keyword()) {
				case Keyword.for: {
					// for<...> unsafe extern "abi" fn(...) -> ...
					// for<...> ...
					// prettier-ignore
					const ty = read_ahead(() => {
						const ltParameters = read_forltParameters();
						return (function r(): Nodes.TypeFnPointer | Nodes.TypeTraitBound {
							switch (peek_keyword()) {
								case Keyword.unsafe: return read_with_unsafe_modifier( r as () => Nodes.TypeFnPointer); // for<...> unsafe fn(...) -> ...
								case Keyword.extern: return read_with_extern_specifier(r as () => Nodes.TypeFnPointer); // for<...> extern "abi" fn(...) -> ...
								case Keyword.fn: return TypeFnPointer.read(ltParameters); // for<...> fn(...) -> ...
								default: return TypeTraitBound.read(false, false, ltParameters, read_TypeNamespaceTargetNoSelector());
							}
						})();
					});
					return is_TypeTraitBound(ty) ? asTypeDynBounds(ty) : ty;
				}
				case Keyword.unsafe:
					return read_with_unsafe_modifier(() => read_type_lhs() as Extract<Nodes.TypeNode, Nodes.UnsafeModifier>);
				case Keyword.extern:
					return read_with_extern_specifier(() => read_type_lhs() as Extract<Nodes.TypeNode, Nodes.ExternTarget>);
				case Keyword.move:
					exit.never();
					return read_with_move_modifier(() => read_type_lhs() as Extract<Nodes.TypeNode, Nodes.MoveModifier>);

				case Keyword.dyn: // dyn ...
					return read_ahead(() => {
						safe_skip_keyword(Keyword.dyn);
						return TY_withContext(() => TypeDynBounds.read(true, read_TypeBound()), true);
					});
				case Keyword.impl: // impl Foo
					return TY_withContext(() => TypeImplBounds.read(), true);

				case Keyword.Underscore:
					return TypeInferred.read(); // _
				case Keyword.fn:
					return read_ahead(() => TypeFnPointer.read(undefined));
				// #valid_identifier_keywords
				case Keyword.auto:
				case Keyword.self:
				case Keyword.Self:
				case Keyword.union:
				case Keyword.super:
				case Keyword.crate:
				case Keyword.RawIdentifier:
				case Keyword.NotKeyword:
				case Keyword["macro_rules!"]:
					return read_TypeNamespaceTarget(Identifier.read());
				default:
					exit.expected("TypeNode");
			}
	}
}

function read_forltParameters() {
	safe_skip_keyword(Keyword.for);
	return read_sequence(DelimKind["<>"], () => with_outerAttributes_fromParentContext(() => GenericLtParameterDeclaration.read()));
}

function maybe_read_forLtParameters() {
	return match_keyword(Keyword.for) ? read_forltParameters() : undefined;
}

export function read_type(allowMultipleBounds: boolean): Nodes.TypeNode {
	return TY_withContext(
		() =>
			withEscapedParens(read_type_lhs(), (ty) =>
				TY_ctx_allowMultipleBounds() && match(CharCode["+"])
					? asTypeDynBounds(asTypeTraitBound(ty as Nodes.TypeNamespaceTargetNoSelector))
					: ty
			),
		allowMultipleBounds
	);
}

export function maybe_read_ReturnType(allowMultipleBounds: boolean) {
	return maybe_read_2(CharCode["-"], CharCode[">"]) ? read_type(allowMultipleBounds) : undefined;
}

export function maybe_read_typeAnnotation() {
	return maybe_read(CharCode[":"]) ? read_type(true) : undefined;
}
