import { Identifier, ItemPath, read_maybe_missing } from ".";
import { is_ItemPath } from "../../utils/ast";
import { CharCode } from "../../utils/CharCode";
import { is_UpperCase, is_XID_Continue, is_XID_Start } from "../../utils/enum";
import { exit } from "../error";
import * as Nodes from "../nodes";
import { DelimKind, Feature, Node, NodeWithBody, NodeWithBodyNoBody } from "../nodes";
import {
	check_ahead,
	currChar,
	FG_property,
	FG_with_outerAttributes_fromParentContext,
	FOR_EACH_UNTIL,
	match,
	match_keyword,
	maybe_read,
	maybe_read_keyword,
	maybe_skip_1_read_2,
	not_match,
	not_match_keyword,
	peek,
	peek_keyword,
	peek_match,
	read,
	readLocatedArrayDelim,
	read_ahead,
	read_ahead_either,
	read_ahead_maybe_extern,
	read_group,
	read_group_noDelim,
	read_group_noGroup,
	read_keyword,
	read_sequence,
	safe_skip,
	safe_skip_1_read_2,
	safe_skip_keyword,
	step,
	step_over_3,
	with_outerAttributes_fromParentContext,
} from "../state";
import { FileLoc, FileLoc_FromChild, FileLoc_FromChildElseReadAhead, FileLoc_ReadAhead } from "../state/constructor";
import { Keyword } from "../state/scanner";
import { BlockExpression, read_contained_expr_in_stmt, read_stmt_expression } from "./expressions";
import { MacroDeclaration, MacroRulesDeclaration } from "./macro";
import { read_PatternNoUnion_unstrict } from "./patterns";
import {
	apply_const_modifier,
	ExternSpecifier,
	maybe_read_abi,
	maybe_read_with_pub_specifier,
	read_with_async_modifier,
	read_with_pub_specifier,
	read_with_unsafe_modifier,
	will_match_crate_specifier,
} from "./specifiers";
import {
	maybe_read_colon_typeBounds,
	maybe_read_generics,
	maybe_read_lifetime,
	maybe_read_ReturnType,
	maybe_read_typeAnnotation,
	maybe_read_whereBounds,
	read_type,
	read_typeBounds,
	read_TypeNamespaceTargetNoSelector,
} from "./types";

//#region ==============================================[        Statements        ]========================================================``--.

//#------------------------------------------------+        ExpressionStatement        +----------------------------------------------------.

class ExpressionStatement extends FileLoc(Nodes.ExpressionStatement) {
	read() {
		if (maybe_read(CharCode[";"])) {
			this.expression = undefined;
			this.semi = true;
		} else {
			this.expression = read_stmt_expression();
			this.semi = maybe_read(CharCode[";"]);
		}
	}
}

function read_expr_or_macroInvocation_stmt() {
	return ExpressionStatement.read();
	// if (match(CharCode[";"])) {
	// 	return read_ahead(() => ExpressionStatement.read(undefined));
	// } else {
	// 	const expr = read_stmt_expression();
	// 	if (is_MacroInvocation(expr)) {
	// 		switch (expr.segments.tk) {
	// 			case DelimKind["()"]:
	// 			case DelimKind["[]"]:
	// 				if (match(CharCode[";"])) {
	// 					__set_endPos_eat(expr, CharCode[";"]);
	// 					expr.semi = true;
	// 					return expr;
	// 				}
	// 				break;
	// 			case DelimKind["{}"]:
	// 				if (match(CharCode["}"])) {
	// 					break;
	// 				} else {
	// 					return expr;
	// 				}
	// 		}
	// 	}
	// 	return __inherit_startPos(ExpressionStatement.read(expr), expr);
	// }
}

function safe_read_expr_stmt() {
	return ExpressionStatement.read();
	// __DEV__: assert(!MATCH(CharCode[";"]));
	// const expr = read_stmt_expression();
	// __DEV__: assert(!is_MacroInvocation(expr));
	// return __inherit_startPos(ExpressionStatement.read(expr), expr);
}

//#----------------------------------------------------+        UseStatement        +-------------------------------------------------------.

class UseStatement extends FileLoc(Nodes.UseStatement) {
	read() {
		// use••••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.use);
		this.import = read_import();
		maybe_read(CharCode[";"]);
	}
}

/**/ class DestructuredImport extends FileLoc_FromChildElseReadAhead(Nodes.DestructuredImport) {
	read(source: this["source"]) {
		// source::{••••••••••••••••
		//         ^- You are here
		this.source = source;
		this.specifiers = read_sequence(DelimKind["{}"], () => read_import());
	}
}

/**/ class AmbientImport extends FileLoc_FromChildElseReadAhead(Nodes.AmbientImport) {
	read(source: this["source"]) {
		this.source = source;
		safe_skip(CharCode["*"]);
	}
}

/**/ class AnonymousImport extends FileLoc_FromChild(Nodes.AnonymousImport) {
	read(source: this["source"]) {
		this.source = source;
		safe_skip_keyword(Keyword.Underscore);
	}
}

/**/ class NamedImport extends FileLoc_FromChild(Nodes.NamedImport) {
	read(source: this["source"], local: this["local"]) {
		this.source = source;
		this.local = local;
	}
}

function read_import(): Nodes.ImportNode {
	let lhs: Nodes.Identifier | Nodes.ItemPath;
	switch (currChar()) {
		case CharCode["{"]:
			return read_ahead(() => DestructuredImport.read(undefined));
		case CharCode["*"]:
			return read_ahead(() => AmbientImport.read(undefined));
		case CharCode[":"]: {
			const res = read_ahead(() => {
				safe_skip_1_read_2(CharCode[":"], CharCode[":"]);
				switch (currChar()) {
					case CharCode["{"]:
						return DestructuredImport.read(undefined);
					case CharCode["*"]:
						return AmbientImport.read(undefined);
					default:
						return ItemPath.read(undefined);
				}
			});
			if (is_ItemPath(res)) {
				lhs = res;
				break;
			} else {
				return res;
			}
		}
		default:
			lhs = Identifier.read();
			break;
	}
	while (true) {
		if (match(CharCode[":"])) {
			safe_skip_1_read_2(CharCode[":"], CharCode[":"]);
			switch (currChar()) {
				case CharCode["{"]:
					return DestructuredImport.read(lhs);
				case CharCode["*"]:
					return AmbientImport.read(lhs);
				default:
					lhs = ItemPath.read(lhs);
					break;
			}
		} else {
			return maybe_read_keyword(Keyword.as)
				? match_keyword(Keyword.Underscore)
					? AnonymousImport.read(lhs)
					: NamedImport.read(lhs, Identifier.read())
				: NamedImport.read(lhs, undefined);
		}
	}
}

//#-----------------------------------------------+        ExternCrateDeclaration        +--------------------------------------------------.

class ExternCrateStatement extends FileLoc(Nodes.ExternCrateStatement) {
	read() {
		// extern crate•••••
		//        ^- You are here
		safe_skip_keyword(Keyword.crate);
		this.import = read_import() as Nodes.AnonymousImport | Nodes.NamedImport;
		maybe_read(CharCode[";"]);
	}
}

//#------------------------------------------------+        TypeAliasDeclaration        +---------------------------------------------------.

class TypeAliasDeclaration extends FileLoc(Nodes.TypeAliasDeclaration) {
	read() {
		// type•••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.type);
		this.id = Identifier.read();
		this.generics = maybe_read_generics();
		this.typeBounds = maybe_read_colon_typeBounds();
		this.whereBounds = maybe_read_whereBounds();
		this.typeExpression = maybe_read(CharCode["="]) ? read_type(true) : undefined;
		maybe_read(CharCode[";"]);
	}
}

//#------------------------------------------------+        VariableDeclaration        +----------------------------------------------------.

class LetVariableDeclaration extends FileLoc(Nodes.LetVariableDeclaration) {
	read() {
		// let••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.let);
		this.pattern = read_PatternNoUnion_unstrict();
		this.typeAnnotation = maybe_read_typeAnnotation();
		if (maybe_read(CharCode["="])) {
			this.expression = FG_with_outerAttributes_fromParentContext(() => read_contained_expr_in_stmt());
			this.else = FG_property(Feature["let_else"], this, "else", () =>
				maybe_read_keyword(Keyword.else) ? BlockExpression.read() : undefined
			);
		} else {
			this.expression = undefined;
		}
		maybe_read(CharCode[";"]);
	}
}

export class ConstVariableDeclaration extends FileLoc_ReadAhead(Nodes.ConstVariableDeclaration) {
	read() {
		// const•••••••••••••••••
		//      ^- You are here
		this.pattern = read_PatternNoUnion_unstrict() as Nodes.Identifier;
		this.typeAnnotation = read_maybe_missing(() => maybe_read_typeAnnotation());
		this.expression = maybe_read(CharCode["="]) ? read_contained_expr_in_stmt() : undefined;
		maybe_read(CharCode[";"]);
	}
}

class StaticVariableDeclaration extends FileLoc(Nodes.StaticVariableDeclaration) {
	read() {
		// static•••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.static);
		this.pattern = read_PatternNoUnion_unstrict() as Nodes.Identifier | Nodes.PatternVariableDeclaration;
		this.typeAnnotation = read_maybe_missing(() => maybe_read_typeAnnotation());
		this.expression = maybe_read(CharCode["="]) ? read_contained_expr_in_stmt() : undefined;
		maybe_read(CharCode[";"]);
	}
}

//#-------------------------------------------------+        ModuleDeclaration        +-----------------------------------------------------.

class ModuleDeclaration extends FileLoc(Nodes.ModuleDeclaration) {
	read() {
		// mod•••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.mod);
		this.id = Identifier.read();
		maybe_read_body(this);
	}
}

//#-------------------------------------------------+        ModuleDeclaration        +-----------------------------------------------------.

class ExternBlockDeclaration extends FileLoc_ReadAhead(Nodes.ExternBlockDeclaration) {
	read(abi: this["abi"]) {
		this.abi = abi;
		read_body(this);
	}
}

//#------------------------------------------------+        FunctionDeclaration        +----------------------------------------------------.

class FunctionDeclaration extends FileLoc(Nodes.FunctionDeclaration) {
	read() {
		// fn•••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.fn);

		this.id = Identifier.read();
		this.generics = maybe_read_generics();

		this.parameters = readLocatedArrayDelim(DelimKind["()"], (parameters: Nodes.FunctionDeclarationParameters) => {
			read(CharCode["("]);
			if (maybe_read(CharCode[")"])) {
				parameters.self = undefined;
			} else {
				with_outerAttributes_fromParentContext(() =>
					read_ahead_either(
						() => {
							const ref = maybe_read(CharCode["&"]);
							const lt = ref ? maybe_read_lifetime() : undefined;
							const mut = maybe_read_keyword(Keyword.mut);
							return (parameters.self = match_keyword(Keyword.self)
								? FunctionSelfParameterDeclaration.read(ref, lt, mut)
								: undefined);
						},
						() => (parameters[0] = read_function_parameter())
					)
				);
				FOR_EACH_UNTIL(
					() => {
						parameters.push(with_outerAttributes_fromParentContext(() => read_function_parameter()));
					},
					CharCode[","],
					CharCode[")"]
				);
			}
		}) as Nodes.FunctionDeclarationParameters;
		this.returnType = maybe_read_ReturnType(true);
		this.whereBounds = maybe_read_whereBounds();
		maybe_read_body(this);
	}
}

/**/ class FunctionSelfParameterDeclaration extends FileLoc_ReadAhead(Nodes.FunctionSelfParameterDeclaration) {
	read(ref: this["ref"], lt: this["lt"], mut: this["mut"]) {
		// self
		// ^- You are here
		safe_skip_keyword(Keyword.self);
		this.ref = ref;
		this.lt = lt;
		this.mut = mut;
		this.typeAnnotation = maybe_read_typeAnnotation();
	}
}

/**/ class FunctionParameterDeclaration extends FileLoc(Nodes.FunctionParameterDeclaration) {
	read() {
		this.pattern = read_PatternNoUnion_unstrict() as Nodes.PatternNoUnion;
		this.typeAnnotation = read_maybe_missing(() =>
			maybe_read(CharCode[":"])
				? match(CharCode["."])
					? FunctionSpread.read() //
					: read_type(true)
				: undefined
		);
	}
}

/**/ export class FunctionSpread extends FileLoc(Nodes.FunctionSpread) {
	read() {
		// ...
		// ^- You are here
		step_over_3(CharCode["."], CharCode["."], CharCode["."]);
	}
}

function read_function_parameter() {
	return match(CharCode["."]) ? FunctionSpread.read() : FunctionParameterDeclaration.read();
}

//#-------------------------------------------------+        StructDeclaration        +-----------------------------------------------------.

class StructDeclaration extends FileLoc_ReadAhead(Nodes.StructDeclaration) {
	read(id: this["id"], generics: this["generics"]) {
		// struct id<...generics?> where ...whereBounds? {••••••••••••••••
		//                                               ^- You are here
		this.id = id;
		this.generics = generics;
		this.whereBounds = maybe_read_whereBounds();
		this.properties = match(CharCode["{"]) ? read_struct_properties_declaration() : (maybe_read(CharCode[";"]), undefined);
	}
}

function read_struct_properties_declaration() {
	return read_sequence(DelimKind["{}"], () => maybe_read_with_attr_pub(() => StructPropertyDeclaration.read()));
}

/**/ class StructPropertyDeclaration extends FileLoc(Nodes.StructPropertyDeclaration) {
	read() {
		this.id = Identifier.read();
		this.typeAnnotation = read_maybe_missing(() => maybe_read_typeAnnotation());
	}
}

class TupleStructDeclaration extends FileLoc_ReadAhead(Nodes.TupleStructDeclaration) {
	read(id: this["id"], generics: this["generics"]) {
		// struct id<...generics?>(
		//                              ^- You are here
		this.id = id;
		this.generics = generics;
		this.items = read_struct_items_declaration();
		this.whereBounds = maybe_read_whereBounds();
		maybe_read(CharCode[";"]);
	}
}

/**/ class TupleStructItemDeclaration extends FileLoc(Nodes.TupleStructItemDeclaration) {
	read() {
		this.typeAnnotation = read_type(true);
	}
}

function read_struct_items_declaration() {
	return read_sequence(DelimKind["()"], () => maybe_read_with_attr_pub(() => TupleStructItemDeclaration.read()));
}

function read_struct_declaration() {
	return read_ahead(() => {
		safe_skip_keyword(Keyword.struct);
		const id = Identifier.read();
		const generics = maybe_read_generics();
		return match(CharCode["("]) //
			? TupleStructDeclaration.read(id, generics)
			: StructDeclaration.read(id, generics);
	});
}

//#--------------------------------------------------+        UnionDeclaration        +-----------------------------------------------------.

class UnionDeclaration extends FileLoc(Nodes.UnionDeclaration) {
	read() {
		// union••••••••••••
		// ^- you are here
		safe_skip_keyword(Keyword.union);
		this.id = Identifier.read();
		this.generics = maybe_read_generics();
		this.whereBounds = maybe_read_whereBounds();
		this.properties = read_struct_properties_declaration();
	}
}

//#--------------------------------------------------+        EnumDeclaration        +------------------------------------------------------.

class EnumDeclaration extends FileLoc(Nodes.EnumDeclaration) {
	read() {
		// enum•••••••••••••
		// ^- you are here
		safe_skip_keyword(Keyword.enum);
		this.id = Identifier.read();
		this.generics = maybe_read_generics();
		this.whereBounds = maybe_read_whereBounds();
		// prettier-ignore
		this.members = read_sequence(DelimKind["{}"], () =>
			maybe_read_with_attr_pub(() =>
				read_ahead(() => {
					const id = Identifier.read();
					switch (currChar()) {
						case CharCode["("]: return EnumMemberTupleDeclaration.read(id);
						case CharCode["{"]: return EnumMemberStructDeclaration.read(id);
						default: return EnumMemberDeclaration.read(id);
					}
				})
			)
		);
	}
}

/**/ class EnumMemberDeclaration extends FileLoc_FromChild(Nodes.EnumMemberDeclaration) {
	read(id: this["id"]) {
		this.id = id;
		this.value = maybe_read(CharCode["="]) ? read_contained_expr_in_stmt() : undefined;
	}
}

/**/ class EnumMemberTupleDeclaration extends FileLoc_FromChild(Nodes.EnumMemberTupleDeclaration) {
	read(id: this["id"]) {
		// id(••••••••••••••••
		//   ^- you are here
		this.id = id;
		this.items = read_struct_items_declaration();
		this.value = FG_property(Feature["arbitrary_enum_discriminant"], this, "value", () =>
			maybe_read(CharCode["="]) ? read_contained_expr_in_stmt() : undefined
		);
	}
}

/**/ class EnumMemberStructDeclaration extends FileLoc_FromChild(Nodes.EnumMemberStructDeclaration) {
	read(id: this["id"]) {
		this.id = id;
		this.properties = read_struct_properties_declaration();
		this.value = FG_property(Feature["arbitrary_enum_discriminant"], this, "value", () =>
			maybe_read(CharCode["="]) ? read_contained_expr_in_stmt() : undefined
		);
	}
}

//#--------------------------------------------------+        TraitDeclaration        +-----------------------------------------------------.

class TraitDeclaration extends FileLoc_ReadAhead(Nodes.TraitDeclaration) {
	read(id: this["id"], generics: this["generics"]) {
		// trait id<...generics?>••••••••••••
		//                       ^- you are here
		this.id = id;
		this.generics = generics;
		this.typeBounds = maybe_read_colon_typeBounds();
		this.whereBounds = maybe_read_whereBounds();
		read_body(this);
	}
}

class AutoTraitDeclaration extends FileLoc(Nodes.AutoTraitDeclaration) {
	read() {
		// auto trait•••••••
		// ^- you are here
		safe_skip_keyword(Keyword.auto);
		read_keyword(Keyword.trait);
		this.id = Identifier.read();
		read_body_noBody(this);
	}
}

class TraitAliasDeclaration extends FileLoc_ReadAhead(Nodes.TraitAliasDeclaration) {
	read(id: this["id"], generics: this["generics"]) {
		// trait id =••••••••••••••••
		//          ^- you are here
		this.id = id;
		this.generics = generics;
		safe_skip(CharCode["="]);
		this.typeBounds = match_keyword(Keyword.where) || match(CharCode[";"]) ? [] : read_typeBounds();
		this.whereBounds = maybe_read_whereBounds();
	}
}

function read_TraitDeclaration() {
	return read_ahead(() => {
		safe_skip_keyword(Keyword.trait);
		const id = Identifier.read();
		const generics = maybe_read_generics();
		return match(CharCode["="]) //
			? TraitAliasDeclaration.read(id, generics)
			: TraitDeclaration.read(id, generics);
	});
}

//#--------------------------------------------------+        ImplDeclaration        +------------------------------------------------------.

let _impl_startsWith_selector = false;

class ImplDeclaration extends FileLoc(Nodes.ImplDeclaration) {
	read(generics: this["generics"]) {
		// impl•••••••••••••
		// ^- you are here

		if (_impl_startsWith_selector) {
			// impl <A as B>::C •••••
			//      ^- you are here
			this.generics = undefined;
			this.const = false;
			this.trait = undefined;
			this.typeTarget = read_type(true);
		} else {
			this.generics = generics;
			if ((this.const = FG_property(Feature["const_trait_impl"], this, "const", () => maybe_read_keyword(Keyword.const)))) {
				this.trait = read_TypeNamespaceTargetNoSelector();
				this.typeTarget = (read_keyword(Keyword.for), read_type(true));
			} else {
				const ty = read_type(true);
				if (maybe_read_keyword(Keyword.for)) {
					this.trait = ty as Nodes.TypeNamespaceTargetNoSelector;
					this.typeTarget = read_type(true);
				} else {
					this.trait = undefined;
					this.typeTarget = ty;
				}
			}
		}

		this.whereBounds = maybe_read_whereBounds();
		read_body(this);
	}
}

class NegativeImplDeclaration extends FileLoc(Nodes.NegativeImplDeclaration) {
	read(generics: this["generics"]) {
		this.generics = generics;
		safe_skip(CharCode["!"]);
		this.trait = read_TypeNamespaceTargetNoSelector();
		this.typeTarget = (read_keyword(Keyword.for), read_type(true));
		this.whereBounds = maybe_read_whereBounds();
		read_body_noBody(this);
	}
}

function read_impl() {
	return read_ahead(() => {
		// impl•••••••••••••
		// ^- you are here
		safe_skip_keyword(Keyword.impl);

		if ((_impl_startsWith_selector = impl_match_selector_not_generics())) {
			return ImplDeclaration.read(undefined);
		}

		const generics = maybe_read_generics();
		return match(CharCode["!"]) &&
			check_ahead(() => (safe_skip(CharCode["!"]), not_match(CharCode["{"]) && not_match_keyword(Keyword.where)))
			? NegativeImplDeclaration.read(generics)
			: ImplDeclaration.read(generics);
	});
}

function impl_match_selector_not_generics(): boolean {
	// compiler/rustc_parse/src/parser/generics.rs:325
	return (
		match(CharCode["<"]) &&
		check_ahead(() => {
			safe_skip(CharCode["<"]);
			switch (currChar()) {
				case CharCode["c"]:
					if (match_keyword(Keyword.const)) return false; // <const T: u32>
					break;
				case CharCode[">"]: // <>
					return false;
				case CharCode["'"]:
					safe_skip(CharCode["'"]);
					break;
			}
			if (is_XID_Start(currChar())) {
				while (is_XID_Continue(peek(1))) step();
				safe_skip(currChar());
				switch (currChar()) {
					case CharCode[">"]: // <T>
					case CharCode[","]: // <T,...>
					case CharCode["="]: // <T=...>
						return false;
					case CharCode[":"]:
						if (maybe_skip_1_read_2(CharCode[":"])) {
							return not_match(CharCode[":"]); // <T::X>
						}
						return false; // <T: X> | <T: ::X>
				}
			}
			return true;
		})
	);
}

//#endregion ===============================================================================================================================..--'

function maybe_read_with_attr_pub<T extends Node>(fn: () => T): T {
	return with_outerAttributes_fromParentContext(() => maybe_read_with_pub_specifier(fn as any) as any);
}

export function read_statement(): Nodes.StatementNode {
	switch (peek_keyword()) {
		case Keyword.extern:
			return read_ahead_maybe_extern(() => {
				safe_skip_keyword(Keyword.extern);

				if (match_keyword(Keyword.crate)) {
					// extern crate ... ;
					return ExternCrateStatement.read();
				}

				const abi = maybe_read_abi();
				const unsafe = maybe_read_keyword(Keyword.unsafe);

				if (match(CharCode["{"])) {
					// extern ... { ... }
					const stmt = ExternBlockDeclaration.read(abi);
					if (unsafe) stmt.unsafe = true;
					return stmt;
				}

				if (match_keyword(Keyword.fn)) {
					// extern ... fn ...
					const extern = ExternSpecifier.read(abi);
					const stmt = FunctionDeclaration.read();
					stmt.extern = extern;
					if (unsafe) stmt.unsafe = true;
					return stmt;
				}

				exit("'extern' cannot be used here");
			});

		case Keyword.pub: // pub(crate) ...
			return read_with_pub_specifier(() => read_statement() as Extract<Nodes.StatementNode, Nodes.PubTarget>);

		case Keyword.crate:
			return will_match_crate_specifier()
				? read_with_pub_specifier(() => read_statement() as Extract<Nodes.StatementNode, Nodes.PubTarget>) // crate ... ('pub(crate)' alias)
				: read_expr_or_macroInvocation_stmt(); // crate::path::foo

		// Modifier
		case Keyword.async: // async ...
			return peek_match("async".length + 1, CharCode["{"]) ||
				check_ahead(
					() => (safe_skip_keyword(Keyword.async), maybe_read_keyword(Keyword.move), match(CharCode["{"]) || match(CharCode["|"]))
				)
				? safe_read_expr_stmt()
				: read_with_async_modifier(() => read_statement() as Nodes.FunctionDeclaration);

		case Keyword.move: // move ...
			return safe_read_expr_stmt();

		case Keyword.unsafe: // unsafe ...
			return peek_match(7, CharCode["{"]) || check_ahead(() => (safe_skip_keyword(Keyword.unsafe), match(CharCode["{"])))
				? safe_read_expr_stmt()
				: read_with_unsafe_modifier(() => read_statement() as Extract<Nodes.StatementNode, Nodes.UnsafeModifier>);

		// Declarations
		case Keyword.use: // use ...;
			return UseStatement.read();

		case Keyword["macro_rules!"]:
			switch (check_ahead(() => (safe_skip_keyword(Keyword["macro_rules!"]), safe_skip(CharCode["!"]), currChar()))) {
				case CharCode["("]:
				case CharCode["["]:
				case CharCode["{"]:
					return read_expr_or_macroInvocation_stmt(); // macro_rules! { ... }
				default:
					return MacroRulesDeclaration.read(); // macro_rules! ... { ... };
			}

		case Keyword.macro:
			return MacroDeclaration.read(); // macro ...(...) {...}

		case Keyword.type:
			return TypeAliasDeclaration.read(); // type ...: ... = ...;

		case Keyword.static:
			return !is_UpperCase(peek(7)) &&
				check_ahead(() => {
					safe_skip_keyword(Keyword.static);
					switch (peek_keyword()) {
						case Keyword.async:
						case Keyword.move:
						case Keyword.static:
							return true;
						default:
							return match(CharCode["|"]);
					}
				})
				? safe_read_expr_stmt() // static || ...
				: StaticVariableDeclaration.read(); // static ...: ... = ...;

		case Keyword.const:
			return read_ahead(() => {
				safe_skip_keyword(Keyword.const);
				switch (peek_keyword()) {
					case Keyword.NotAWord:
						return match(CharCode["{"])
							? apply_const_modifier(safe_read_expr_stmt() as Nodes.ExpressionStatement<Nodes.BlockExpression>) // const { ... } [feature(inline_const)]
							: ConstVariableDeclaration.read(); // const pattern: ... = ...
					case Keyword.unsafe:
					case Keyword.extern:
					case Keyword.async:
					case Keyword.fn:
						return apply_const_modifier(
							read_statement() as Nodes.FunctionDeclaration | Nodes.ExpressionStatement<Nodes.BlockExpression>
						);
					default:
						return ConstVariableDeclaration.read();
				}
			});

		case Keyword.let:
			return LetVariableDeclaration.read(); // let ... = ...;

		case Keyword.mod:
			return ModuleDeclaration.read(); // mod ... { ... }
		case Keyword.fn:
			return FunctionDeclaration.read(); // fn ...(...) -> ... { ... }

		case Keyword.struct:
			return read_struct_declaration(); // struct ...;

		case Keyword.union:
			return check_ahead(() => (safe_skip_keyword(Keyword.union), !match_keyword(Keyword.NotAWord) && !match_keyword(Keyword.as)))
				? UnionDeclaration.read() // union ... { ... }
				: read_expr_or_macroInvocation_stmt(); // 'union' identifier

		case Keyword.enum:
			return EnumDeclaration.read(); // enum ... { ... }

		case Keyword.auto:
			return check_ahead(() => (safe_skip_keyword(Keyword.auto), match_keyword(Keyword.trait)))
				? AutoTraitDeclaration.read()
				: read_expr_or_macroInvocation_stmt();

		case Keyword.trait:
			return read_TraitDeclaration(); // trait Growling<T>: Sound { ... }

		case Keyword.impl:
			return read_impl(); // impl<T> Growling<T> for Wolf {}

		default:
			return read_expr_or_macroInvocation_stmt(); // ...;
	}
}

// type NodeWithBody = Extract<Node, { body: LocArray<StatementNode, "{}"> | undefined }>;

export function read_top_statements(target: Nodes.Program) {
	return read_group_noDelim(target, () => read_statement());
}

export function read_body(target: Exclude<NodeWithBody, Nodes.Program>) {
	read_group(target, DelimKind["{}"], () => read_statement());
}

function read_body_noBody(target: NodeWithBodyNoBody) {
	read_group_noGroup(target);
}

function maybe_read_body(target: ModuleDeclaration | FunctionDeclaration) {
	if (match(CharCode["{"])) read_body(target);
	else maybe_read(CharCode[";"]), (target.body = undefined);
}
