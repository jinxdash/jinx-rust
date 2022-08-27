import { is_ExpressionNamespaceTarget, is_ExpressionWithBodyOrCases, is_ParenthesizedExpression, is_RangeLiteral } from "../../utils/ast";
import { CharCode } from "../../utils/CharCode";
import { is_whitespaceOrSlash } from "../../utils/enum";
import { assert, exit } from "../error";
import * as Nodes from "../nodes";
import { DelimKind, ExpressionNode, ExpressionNodeXS, NodeType, PRCD, TK } from "../nodes";
import {
	CCPATH_read,
	check_ahead,
	currChar,
	EDGECASE_STEPBACK_TO,
	ES_consume_optional_read,
	ES_ctx_exceptStructFormExpression,
	ES_ctx_insideScrutinee,
	ES_signal_optional_read,
	ES_withContext,
	ES_withPrecedence,
	FOR_EACH_UNTIL,
	match,
	match_3,
	match_keyword,
	maybe_read,
	maybe_read_2,
	maybe_read_keyword,
	maybe_skip_1_read_2,
	not_match,
	peek,
	peek_keyword,
	peek_match,
	peek_not_match,
	read,
	readLocatedArrayDelim,
	read_2,
	read_ahead,
	read_ahead_either,
	read_group,
	read_identifier_with,
	read_keyword,
	read_sequence,
	safe_skip,
	safe_skip_1_read_2,
	safe_skip_keyword,
	safe_skip_word,
	sequence_hasTrailingComma,
	will_match_charLiteral_not_lt,
	will_match_lt,
	withEnd,
	withStart,
	with_outerAttributes_fromParentContext,
	with_outerAttributes_fromParentContext_if_test,
} from "../state";
import {
	FileLoc,
	FileLoc_COPY,
	FileLoc_FromChild,
	FileLoc_FromChildElseReadAhead,
	FileLoc_ReadAhead,
	withEscapedParens,
} from "../state/constructor";
import { Keyword, RHS, rhsTree } from "../state/scanner";
import { Identifier, read_Identifier_or_Index, read_Index } from "./index";
import { Literal } from "./literals";
import { MacroInvocation } from "./macro";
import { read_pattern, read_PatternNoUnion_unstrict } from "./patterns";
import {
	read_with_async_modifier,
	read_with_const_modifier,
	read_with_move_modifier,
	read_with_static_modifier,
	read_with_unsafe_modifier,
} from "./specifiers";
import { read_body } from "./statements";
import { maybe_read_ReturnType, maybe_read_typeAnnotation, read_type, read_TypeArguments } from "./types";

//#region =============================================[        Expressions        ]========================================================``--.

class LbIdentifier extends FileLoc(Nodes.LbIdentifier) {
	read() {
		// 'x•••••••••••••••
		// ^- You are here (x:XID_START)
		read_identifier_with(CharCode["'"]);
	}
}

//#--------------------------------------------------------+        Path        +-----------------------------------------------------------.

export class ExpressionTypeSelector extends FileLoc(Nodes.ExpressionTypeSelector) {
	read() {
		// <••••••••••••••••
		// ^- You are here
		safe_skip(CharCode["<"]);
		this.typeTarget = read_type(true);
		this.typeExpression = maybe_read_keyword(Keyword.as) ? (read_type(true) as any) : undefined;
		read(CharCode[">"]);
	}
}

export class ExpressionTypeCast extends FileLoc_FromChild(Nodes.ExpressionTypeCast) {
	read(typeCallee: this["typeCallee"], typeArguments: this["typeArguments"]) {
		// namespace::<...typeArguments>••••••••••••••••
		// 			                   ^- You are here
		this.typeCallee = typeCallee;
		this.typeArguments = typeArguments;
	}
}

export class ExpressionPath extends FileLoc_FromChildElseReadAhead(Nodes.ExpressionPath) {
	read(namespace: this["namespace"]) {
		// namespace::••••••••••••••••
		//            ^- You are here
		this.namespace = namespace;
		this.segment = Identifier.read();
	}
}

class ExpressionAsTypeCast extends FileLoc_FromChild(Nodes.ExpressionAsTypeCast) {
	read(expression: this["expression"]) {
		// expression as••••••••••••••••
		//              ^- You are here
		this.expression = expression;
		this.typeExpression = read_type(false);
	}
}

//#----------------------------------------------------+        Flow Control        +-------------------------------------------------------.

class ReturnExpression extends FileLoc(Nodes.ReturnExpression) {
	read() {
		// return•••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.return);
		this.expression = maybe_read_expression_rhs();
	}
}

class BreakExpression extends FileLoc(Nodes.BreakExpression) {
	read() {
		// break••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.break);
		if (will_match_lt()) {
			const label = LbIdentifier.read();
			if (match(CharCode[":"]) && peek_not_match(1, CharCode[":"])) {
				this.label = undefined;
				this.expression = read_labelled_block(label);
			} else {
				this.label = label;
				this.expression = maybe_read_expression_rhs();
			}
		} else {
			this.label = undefined;
			this.expression = maybe_read_expression_rhs();
		}
	}
}

class ContinueExpression extends FileLoc(Nodes.ContinueExpression) {
	read() {
		// continue•••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.continue);
		this.label = match(CharCode["'"]) ? LbIdentifier.read() : undefined;
	}
}

class YieldExpression extends FileLoc(Nodes.YieldExpression) {
	read() {
		// yield••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.yield);
		this.expression = maybe_read_expression_rhs();
	}
}

//#------------------------------------------------------+        Literal        +----------------------------------------------------------.

class RangeLiteral extends FileLoc_FromChildElseReadAhead(Nodes.RangeLiteral) {
	read(lower: this["lower"], last: this["last"]) {
		// lower?..=?•••••••••••••••••
		//           ^- You are here
		this.lower = lower;
		this.last = last;
		this.upper = maybe_read_expression_rhs();
	}
}

//#-----------------------------------------------------+        Derivative        +--------------------------------------------------------.

class CallExpression extends FileLoc_FromChild(Nodes.CallExpression) {
	read(callee: this["callee"], method: this["method"], typeArguments: this["typeArguments"]) {
		// callee.method?<...typearguments>?(••••••••••••••••
		//                                  ^- You are here
		this.callee = callee;
		this.method = method;
		this.typeArguments = typeArguments;
		this.arguments = read_sequence(DelimKind["()"], () =>
			with_outerAttributes_fromParentContext(() => read_contained_expression(false))
		);
	}
}

class MemberExpression extends FileLoc_FromChild(Nodes.MemberExpression) {
	read(expression: this["expression"], computed: boolean, property: this["property"]) {
		this.expression = expression;
		this.computed = computed;
		this.property = property;
	}
}

class AwaitExpression extends FileLoc_FromChild(Nodes.AwaitExpression) {
	read(expression: this["expression"]) {
		// expression.await•••••••••••
		//            ^- You are here
		this.expression = expression;
		safe_skip_keyword(Keyword.await);
	}
}

function read_property_or_method(expression: CallExpression["callee"] | MemberExpression["expression"]) {
	safe_skip(CharCode["."]);
	switch (peek_keyword()) {
		case Keyword.await:
			return AwaitExpression.read(expression);
		// #valid_identifier_keywords
		case Keyword.auto:
		case Keyword.self:
		case Keyword.Self:
		case Keyword.union:
		case Keyword.super:
		case Keyword.crate:
		case Keyword.RawIdentifier:
		case Keyword.NotKeyword:
		case Keyword["macro_rules!"]: {
			const property = Identifier.read();
			switch (currChar()) {
				case CharCode[":"]:
					// lhs.property::••••••••••••••••
					//             ^ You are here
					if (maybe_skip_1_read_2(CharCode[":"]) && match(CharCode["<"])) {
						const typeArguments = read_TypeArguments();
						if (match(CharCode["("])) {
							// lhs.property::<...typeArguments>(
							return CallExpression.read(expression, property, typeArguments);
						} else {
							// lhs.property::<...typeArguments> // syntax error
							return ExpressionTypeCast.read(
								withEnd(property, MemberExpression.read(expression, false, property)) as any,
								typeArguments
							);
						}
					} else {
						// lhs.property::foo // syntax error
						EDGECASE_STEPBACK_TO(property);
						return MemberExpression.read(expression, false, property);
					}
				case CharCode["("]:
					// lhs.property(
					return CallExpression.read(expression, property, undefined);
				default:
					return MemberExpression.read(expression, false, property);
			}
		}
		default:
			return MemberExpression.read(expression, false, read_Index());
	}
}

function read_computed_property(lhs: MemberExpression["expression"]) {
	return MemberExpression.read(lhs, true, read_expression_between(CharCode["["], CharCode["]"]));
}

class UnwrapExpression extends FileLoc_FromChild(Nodes.UnwrapExpression) {
	read(expression: this["expression"]) {
		// expression?••••••••••••••••
		//           ^- You are here
		this.expression = expression;
		safe_skip(CharCode["?"]);
	}
}

class ParenthesizedExpression extends FileLoc_COPY(Nodes.ParenthesizedExpression) {
	read(items: Nodes.LocArray<this["expression"], any>) {
		this.expression = items[0];
	}
}

export class MinusExpression extends FileLoc(Nodes.MinusExpression) {
	read(read_expression: () => this["expression"]) {
		safe_skip(CharCode["-"]);
		this.expression = read_expression();
	}
}

class NotExpression extends FileLoc(Nodes.NotExpression) {
	read() {
		// !•••••••••••••••
		// ^- You are here
		safe_skip(CharCode["!"]);
		this.expression = read_unary_rhs();
	}
}

//#-----------------------------------------------------+        LeftRight        +---------------------------------------------------------.

class OrExpression extends FileLoc_FromChild(Nodes.OrExpression) {
	read(left: this["left"]) {
		// left ||••••••••••••••••
		//        ^- You are here
		this.left = left;
		this.tk = TK["||"];
		this.right = read_expression_rhs();
	}
}

class AndExpression extends FileLoc_FromChild(Nodes.AndExpression) {
	read(left: this["left"]) {
		// left &&••••••••••••••••
		//        ^- You are here
		this.left = left;
		this.tk = TK["&&"];
		this.right = read_expression_rhs();
	}
}

class ReassignmentExpression extends FileLoc_FromChild(Nodes.ReassignmentExpression) {
	read(left: this["left"]) {
		// left =•••••••••••••••••
		//       ^- You are here
		this.left = left;
		this.tk = TK["="];
		this.right = read_expression_rhs();
	}
}

/**/ class UnassignedExpression extends FileLoc(Nodes.UnassignedExpression) {
	read() {
		safe_skip_keyword(Keyword.Underscore);
	}
}

class OperationExpression extends FileLoc_FromChild(Nodes.OperationExpression) {
	read(left: this["left"], tk: this["tk"]) {
		// left +••••••••••••••••
		//       ^- You are here ( "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^" | "<<" | ">>")
		this.left = left;
		this.tk = tk;
		this.right = read_expression_rhs();
	}
}

class ReassignmentOperationExpression extends FileLoc_FromChild(Nodes.ReassignmentOperationExpression) {
	read(left: this["left"], tk: this["tk"]) {
		// left +=•••••••••••••••
		//        ^- You are here ( "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=")
		this.left = left;
		this.tk = tk;
		this.right = read_expression_rhs();
	}
}

class ComparisonExpression extends FileLoc_FromChild(Nodes.ComparisonExpression) {
	read(left: this["left"], tk: this["tk"]) {
		// left ==•••••••••••••••••
		//        ^- You are here ( "==" | "!=" | ">" | ">=" | "<" | "<=")
		this.left = left;
		this.tk = tk;
		this.right = read_expression_rhs();
	}
}

//#----------------------------------------------------+        LetScrutinee        +-------------------------------------------------------.

/**/ class LetScrutinee extends FileLoc(Nodes.LetScrutinee) {
	read() {
		// let •••••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.let);
		this.pattern = read_PatternNoUnion_unstrict();
		read(CharCode["="]);
		this.expression = read_scrutinee_rhs() as Nodes.ScrutineeExpression;
	}
}

function read_ConditionExpressionNode() {
	return read_contained_expression(true, true);
}

//#--------------------------------------------------+        ClosureFunction        +------------------------------------------------------.

class ClosureFunctionExpression extends FileLoc(Nodes.ClosureFunctionExpression) {
	read() {
		// |•••••••••••••••
		// ^- You are here
		this.parameters = read_sequence(DelimKind["||"], () =>
			with_outerAttributes_fromParentContext(() => ClosureFunctionParameterDeclaration.read())
		);
		this.returnType = maybe_read_ReturnType(false);
		this.expression = read_closure_rhs();
	}
}

/**/ class ClosureFunctionParameterDeclaration extends FileLoc(Nodes.ClosureFunctionParameterDeclaration) {
	read() {
		this.pattern = read_pattern(false) as Nodes.PatternNoUnion;
		this.typeAnnotation = maybe_read_typeAnnotation();
	}
}

//#-----------------------------------------------------+        BlockLike        +---------------------------------------------------------.

export class BlockExpression extends FileLoc(Nodes.BlockExpression) {
	read() {
		// {••••••••••••••••
		// ^- You are here
		this.label = undefined;
		read_body(this);
	}
}

class LoopBlockExpression extends FileLoc(Nodes.LoopBlockExpression) {
	read() {
		// loop•••••••••••••
		// ^- You are here
		this.label = undefined;
		safe_skip_keyword(Keyword.loop);
		read_body(this);
	}
}

class WhileBlockExpression extends FileLoc(Nodes.WhileBlockExpression) {
	read() {
		// while••••••••••••
		// ^- You are here
		this.label = undefined;
		safe_skip_keyword(Keyword.while);
		this.condition = read_ConditionExpressionNode();
		read_body(this);
	}
}

class ForInBlockExpression extends FileLoc(Nodes.ForInBlockExpression) {
	read() {
		// for••••••••••••••
		// ^- You are here
		this.label = undefined;
		safe_skip_keyword(Keyword.for);
		this.pattern = read_pattern(true);
		read_keyword(Keyword.in);
		this.expression = read_contained_expression(true);
		read_body(this);
	}
}

class TryBlockExpression extends FileLoc(Nodes.TryBlockExpression) {
	read() {
		// try••••••••••••••
		//    ^- You are here
		safe_skip_keyword(Keyword.try);
		read_body(this);
	}
}

class IfBlockExpression extends FileLoc(Nodes.IfBlockExpression) {
	read() {
		// if condition {••••••••••••••••
		// ^- You are here
		this.label = undefined;
		safe_skip_keyword(Keyword.if);
		this.condition = read_ConditionExpressionNode();
		read_body(this);
		this.else = maybe_read_keyword(Keyword.else)
			? match_keyword(Keyword.if)
				? IfBlockExpression.read()
				: BlockExpression.read()
			: undefined;
	}
}

class MatchExpression extends FileLoc(Nodes.MatchExpression) {
	read() {
		// match••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.match);
		this.expression = read_contained_expression(true);
		read_group(this, DelimKind["{}"], () => {
			const match_case = MatchExpressionCase.read();
			maybe_read(CharCode[","]);
			return match_case;
		});
	}
}

/**/ class MatchExpressionCase extends FileLoc(Nodes.MatchExpressionCase) {
	read() {
		this.pattern = read_pattern(true);
		this.condition = maybe_read_keyword(Keyword.if)
			? read_contained_expression(
					// issue 93817 unnecessarily forbids struct literals
					false as true,
					true
			  )
			: undefined;
		read_2(CharCode["="], CharCode[">"]);
		this.expression = read_stmt_expression();
	}
}

//#--------------------------------------------------+        Init Expressions        +-----------------------------------------------------.

class StructLiteral extends FileLoc_FromChild(Nodes.StructLiteral) {
	read(struct: this["struct"]) {
		// struct {••••••••••••••••
		//        ^- You are here
		this.struct = struct;
		this.properties = read_sequence(DelimKind["{}"], () =>
			with_outerAttributes_fromParentContext_if_test(
				() => {
					if (match(CharCode["."])) {
						return read_ahead(() => {
							safe_skip_1_read_2(CharCode["."], CharCode["."]);
							return might_read_expression() //
								? StructLiteralPropertySpread.read()
								: StructLiteralRestUnassigned.read();
						});
					} else {
						const id = read_Identifier_or_Index();
						return match(CharCode[":"]) //
							? StructLiteralProperty.read(id)
							: StructLiteralPropertyShorthand.read(id as Nodes.Identifier);
					}
				},
				(node) => {
					switch (node.nodeType) {
						// can have outer attrs
						case NodeType.StructLiteralProperty:
						case NodeType.StructLiteralPropertyShorthand:
							return true;
						// cannot have outer attrs
						case NodeType.StructLiteralPropertySpread:
						case NodeType.StructLiteralRestUnassigned:
							return false;
					}
					__DEV__: exit.never();
				}
			)
		);
	}
}

/**/ class StructLiteralPropertyShorthand extends FileLoc_FromChild(Nodes.StructLiteralPropertyShorthand) {
	read(value: this["value"]) {
		this.value = value;
	}
}

/**/ class StructLiteralProperty extends FileLoc_FromChild(Nodes.StructLiteralProperty) {
	read(key: this["key"]) {
		// { ..., key:••••••••••••••••
		//           ^- You are here
		this.key = key;
		safe_skip(CharCode[":"]);
		this.value = read_contained_expression(false);
	}
}

/**/ class StructLiteralRestUnassigned extends FileLoc_ReadAhead(Nodes.StructLiteralRestUnassigned) {
	read() {
		// ..•••••••••••••••••
		//   ^- You are here
	}
}

/**/ class StructLiteralPropertySpread extends FileLoc_ReadAhead(Nodes.StructLiteralPropertySpread) {
	read() {
		// struct { ..., ..•••••••••••••••••
		//                 ^- You are here
		this.expression = read_contained_expression(false);
	}
}

class TupleLiteral extends FileLoc_COPY(Nodes.TupleLiteral) {
	read(items: this["items"]) {
		this.items = items;
	}
}

class ArrayLiteral extends FileLoc_COPY(Nodes.ArrayLiteral) {
	read(items: this["items"]) {
		this.items = items;
	}
}

class SizedArrayLiteral extends FileLoc_COPY(Nodes.SizedArrayLiteral) {
	read(items: ArrayLiteral["items"]) {
		this.initExpression = items[0];
		this.sizeExpression = items[1];
	}
}

//#-------------------------------------------------------+        Memory        +----------------------------------------------------------.

class ReferenceExpression extends FileLoc_ReadAhead(Nodes.ReferenceExpression) {
	read() {
		// &•••••••••••••••••
		//  ^- You are here
		this.mut = maybe_read_keyword(Keyword.mut);
		this.expression = read_unary_rhs();
	}
}

class RawReferenceExpression extends FileLoc_ReadAhead(Nodes.RawReferenceExpression) {
	read(kind: this["kind"]) {
		// &raw const••••••••••••
		// &raw mut  ••••••••••••
		//           ^- You are here
		this.kind = kind;
		this.expression = read_unary_rhs();
	}
}

class DereferenceExpression extends FileLoc(Nodes.DereferenceExpression) {
	read() {
		// *••••••••••••••••
		// ^- You are here
		safe_skip(CharCode["*"]);
		this.expression = read_unary_rhs();
	}
}

class BoxExpression extends FileLoc(Nodes.BoxExpression) {
	read() {
		// box••••••••••••••
		// ^- You are here
		safe_skip_keyword(Keyword.box);
		this.expression = read_unary_rhs();
	}
}

//#endregion ===============================================================================================================================..--'

function read_turbofish(namespace: CallExpression["callee"] | ExpressionTypeCast["typeCallee"]) {
	// ::<••••••••••••••
	//   ^- You are here
	const typeArguments = read_TypeArguments();
	return match(CharCode["("])
		? CallExpression.read(namespace as CallExpression["callee"], undefined, typeArguments)
		: ExpressionTypeCast.read(namespace as ExpressionTypeCast["typeCallee"], typeArguments);
}

function read_ExpressionNamespaceTarget(namespace: Nodes.ExpressionNamespaceTarget | Nodes.ExpressionTypeSelector) {
	let lhs = namespace as any;
	loop: while (maybe_read_2(CharCode[":"], CharCode[":"])) {
		switch (currChar()) {
			case CharCode[":"]:
				EDGECASE_STEPBACK_TO(lhs);
				break loop;
			case CharCode["<"]:
				lhs = read_turbofish(lhs); // ::<
				break;
			default:
				lhs = ExpressionPath.read(lhs);
				break;
		}
	}
	return lhs as Nodes.ExpressionNamespaceTarget;
}

function read_expression_lhs(): ExpressionNode {
	switch (currChar()) {
		case CharCode[":"]: // ::foo
			return read_ExpressionNamespaceTarget(CCPATH_read(ExpressionPath));
		case CharCode["<"]: // <A as B>::C
			return read_ExpressionNamespaceTarget(ExpressionTypeSelector.read());

		case CharCode["."]:
			return read_ahead(() => {
				safe_skip_1_read_2(CharCode["."], CharCode["."]);
				return RangeLiteral.read(undefined, maybe_read(CharCode["="])); // ..foo?
			});

		case CharCode["{"]: // { ...body }
			return BlockExpression.read();

		case CharCode["("]: {
			const items = read_sequence(DelimKind["()"], (SEQUENCE) =>
				with_outerAttributes_fromParentContext_if_test(
					() => read_contained_expression(false, ES_ctx_insideScrutinee() && 0 === SEQUENCE.length),
					() => SEQUENCE.length > 0 || match(CharCode[","])
				)
			);
			return items.length !== 1 ||
				sequence_hasTrailingComma(items) ||
				(is_RangeLiteral(items[0]) && match(CharCode["="]) && !peek_match(1, CharCode["="])) // (..) = ...
				? TupleLiteral.read(items)
				: ParenthesizedExpression.read(items);
		}

		case CharCode["["]: {
			let is_sized = false;

			const items = readLocatedArrayDelim<Nodes.ExpressionNode, typeof DelimKind["[]"]>(DelimKind["[]"], (items) => {
				safe_skip(CharCode["["]);
				if (!maybe_read(CharCode["]"])) {
					items[0] = read_item();
					if ((is_sized = maybe_read(CharCode[";"]))) {
						(items[1] = read_item()), read(CharCode["]"]);
					} else {
						FOR_EACH_UNTIL(CharCode[","], () => items.push(read_item()), CharCode["]"]);
					}
				}
				function read_item() {
					return with_outerAttributes_fromParentContext(() => read_contained_expression(false));
				}
			});

			return is_sized ? SizedArrayLiteral.read(items) : ArrayLiteral.read(items);
		}

		case CharCode["|"]: // |arg_0, arg_1| ...
			return ClosureFunctionExpression.read();

		case CharCode["-"]: // -foo
			return MinusExpression.read(() => read_unary_rhs());
		case CharCode["!"]: // !foo
			return NotExpression.read();

		case CharCode["&"]: // &foo
			return read_ahead(() => {
				safe_skip(CharCode["&"]);
				return match_3(CharCode["r"], CharCode["a"], CharCode["w"]) && is_whitespaceOrSlash(peek(3))
					? read_ahead_either(
							() => {
								safe_skip_word("raw");
								switch (peek_keyword()) {
									case Keyword.mut:
										safe_skip_keyword(Keyword.mut);
										return RawReferenceExpression.read("mut");
									case Keyword.const:
										safe_skip_keyword(Keyword.const);
										return RawReferenceExpression.read("const");
									default:
										return undefined;
								}
							},
							() => ReferenceExpression.read()
					  )
					: ReferenceExpression.read();
			});
		case CharCode["*"]: // *foo
			return DereferenceExpression.read();

		case CharCode["'"]:
			return will_match_charLiteral_not_lt() //
				? Literal.read()
				: read_labelled_block(LbIdentifier.read());

		case CharCode['"']:
		case CharCode["0"]:
		case CharCode["1"]:
		case CharCode["2"]:
		case CharCode["3"]:
		case CharCode["4"]:
		case CharCode["5"]:
		case CharCode["6"]:
		case CharCode["7"]:
		case CharCode["8"]:
		case CharCode["9"]:
			return Literal.read();

		default:
			switch (peek_keyword()) {
				case Keyword.StringLiteral:
				case Keyword.false:
				case Keyword.true:
					return Literal.read();

				case Keyword.extern:
				case Keyword.pub:
					exit.unexpected();

				case Keyword.static:
					return read_with_static_modifier(read_expression_lhs as () => Extract<Nodes.ExpressionNode, Nodes.StaticModifier>);
				case Keyword.const:
					return read_with_const_modifier(read_expression_lhs as () => Extract<Nodes.ExpressionNode, Nodes.ConstModifier>);
				case Keyword.async:
					return read_with_async_modifier(read_expression_lhs as () => Extract<Nodes.ExpressionNode, Nodes.AsyncModifier>);
				case Keyword.move:
					return read_with_move_modifier(read_expression_lhs as () => Extract<Nodes.ExpressionNode, Nodes.MoveModifier>);
				case Keyword.unsafe:
					return read_with_unsafe_modifier(read_expression_lhs as () => Extract<Nodes.ExpressionNode, Nodes.UnsafeModifier>);

				case Keyword.return:
					return ReturnExpression.read();
				case Keyword.break:
					return BreakExpression.read();
				case Keyword.continue:
					return ContinueExpression.read();
				case Keyword.yield:
					return YieldExpression.read();

				case Keyword.match:
					return MatchExpression.read();

				case Keyword.loop:
					return LoopBlockExpression.read();
				case Keyword.while:
					return WhileBlockExpression.read();
				case Keyword.for:
					return ForInBlockExpression.read();
				case Keyword.if:
					return IfBlockExpression.read();

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
					return read_ExpressionNamespaceTarget(Identifier.read());

				case Keyword.Underscore:
					return UnassignedExpression.read() as any;

				case Keyword.box:
					return BoxExpression.read();

				case Keyword.try:
					switch (check_ahead(() => (safe_skip_keyword(Keyword.try), currChar()))) {
						case CharCode["!"]:
						case CharCode["("]:
							return Identifier.read(); // try!() | try()
						case CharCode[":"]:
							return read_ExpressionNamespaceTarget(Identifier.read()); // try::
						case CharCode["{"]:
							return TryBlockExpression.read();
						default:
							exit.unexpected();
					}
				case Keyword.let:
					assert(ES_ctx_insideScrutinee(), "let variable declarations are not allowed in this context");
					return LetScrutinee.read() as any;

				default:
					if (ES_consume_optional_read()) {
						return undefined!;
					}
					exit.expected("Expression");
			}
	}
}

export function read_expression() {
	return read_contained_expression(false);
}

function read_contained_expression<T extends boolean>(exceptStructFormExpression: T, insideScrutinee = false) {
	return ES_withContext(exceptStructFormExpression, insideScrutinee, () => read_expression_rhs()) as T extends true
		? ExpressionNodeXS
		: ExpressionNode;
}

function read_labelled_block(label: Nodes.LbIdentifier) {
	read(CharCode[":"]);
	const lhs = read_expression_lhs();
	assert.at(lhs, is_ExpressionWithBodyOrCases(lhs), `Expected ExpressionWithBodyOrCases, found ${lhs.type}`);
	return maybe_combine_expression_block(withStart((lhs.label = label), lhs));
}

export function read_stmt_expression() {
	return ES_withContext(false, false, () => {
		let lhs = read_expression_lhs();

		if (!is_ParenthesizedExpression(lhs)) {
			if (is_ExpressionWithBodyOrCases(lhs)) {
				return maybe_combine_expression_block(lhs);
			}

			if (match(CharCode["!"]) && !peek_match(1, CharCode["="])) {
				lhs = MacroInvocation.read(lhs as any);
				if (lhs.segments.dk === DelimKind["{}"]) {
					return maybe_combine_expression_block(lhs);
				}
			}
		}

		return maybe_combine_expression(lhs);
	});
}

function maybe_combine_expression_block<T extends Nodes.ExpressionWithBodyOrCases | Nodes.MacroInvocation>(lhs: T) {
	return should_combine_blockstmt()
		? (maybe_combine_expression(lhs) as Nodes.UnwrapExpression | Nodes.MemberExpression | Nodes.AwaitExpression)
		: lhs;
	function should_combine_blockstmt() {
		switch (currChar()) {
			default:
				return false;
			/** parser/stmt.rs:165 */
			case CharCode["?"]:
				return true;
			case CharCode["."]:
				return peek_not_match(1, CharCode["."]);
			/** jinx-rust only */
			case CharCode["+"]:
				return true;
			case CharCode["a"]:
				return match_keyword(Keyword.as);
		}
	}
}

function might_read_expression() {
	switch (currChar()) {
		case CharCode[")"]:
		case CharCode["}"]:
		case CharCode["]"]:
		case CharCode[";"]:
		case CharCode[","]:
			return false;
		case CharCode["{"]:
			return !ES_ctx_exceptStructFormExpression();
		default:
			return true;
	}
}

function maybe_read_expression_rhs() {
	if (might_read_expression()) {
		ES_signal_optional_read();
		const lhs = read_expression_lhs();
		if (undefined !== lhs) {
			return maybe_combine_expression(lhs);
		}
	}
	return undefined;
}

function read_expression_rhs() {
	return maybe_combine_expression(read_expression_lhs());
}

function read_unary_rhs() {
	return ES_withPrecedence(PRCD.Unary, () => read_expression_rhs());
}

function read_closure_rhs() {
	return ES_withPrecedence(PRCD.Default, () => read_expression_rhs());
}

function read_scrutinee_rhs() {
	return ES_withPrecedence(PRCD.Default, () => read_expression_rhs());
}

export function read_expression_between(charStart: CharCode, charEnd: CharCode) {
	safe_skip(charStart);
	const res = read_contained_expression(false);
	read(charEnd);
	return res;
}

export function read_contained_expr_in_stmt() {
	return read_contained_expression(false);
}

/**
 * # Example of precedence handling
 *
 * ```
 * [breakdown]:
 *
 * 		> "A && B || C"
 * 		= read_expression_lhs():
 * 			> "A && B || C"
 * 			< 'A'
 * 		= maybe_combine_expression('A'):
 * 			> "&& B || C"
 * 			: ...
 * 			= AndExpression.right = read_expression():
 * 				> "B || C"
 * 				= read_expression_lhs();
 * 					> "B || C"
 * 					< 'B'
 * 				= maybe_combine_expression('B'):
 * 					> "|| C"
 * 					: Stop because prev precedence '&&' (4) >= next precedence '||' (3)
 * 					< 'B'
 * 				< AndExpression 'A && B'
 * 			: "|| C"
 * 			: ...
 * 			> OrExpression.right = read_expression():
 * 				> "C"
 * 				: ...
 * 				< 'C'
 * 			< ...
 * 		< OrExpression 'A && B || C' {
 * 			left: AndExpression 'A && B',
 * 			right: 'C'
 * 		}
 *
 * [order]:
 *
 * 	A && B || C
 * 	^-~~   ~~-x
 * 	   ^---xx
 * ```
 */

function maybe_combine_expression(startNode: ReturnType<typeof read_expression_lhs>): ExpressionNode {
	return ES_withPrecedence(PRCD.Default, () =>
		withEscapedParens(startNode, (startNode) => {
			let lhs = startNode;
			let rhs = RHS.None;
			loop: while (true) {
				switch ((rhs = rhsTree())) {
					case RHS["::"]:
						lhs = match(CharCode["<"])
							? read_turbofish(lhs) //
							: ExpressionPath.read(lhs as any); // syntax error
						break;

					case RHS["!"]:
						lhs = MacroInvocation.read(lhs as any);
						break;
					case RHS["("]:
						lhs = CallExpression.read(lhs, undefined, undefined);
						break;

					case RHS["{"]:
						if (!is_ExpressionNamespaceTarget(lhs)) break loop;
						lhs = StructLiteral.read(lhs as Nodes.ExpressionNamespaceTargetNoSelector);
						break;

					case RHS["."]:
						lhs = read_property_or_method(lhs);
						break;
					case RHS["?"]:
						lhs = UnwrapExpression.read(lhs);
						break;
					case RHS["["]:
						lhs = read_computed_property(lhs);
						break;

					case RHS["as"]:
						lhs = ExpressionAsTypeCast.read(lhs);
						break;

					case RHS[".."]:
						lhs = RangeLiteral.read(lhs, false);
						break;
					case RHS["..="]:
					case RHS["..."]:
						lhs = RangeLiteral.read(lhs, true);
						break;

					case RHS["&&"]:
						lhs = AndExpression.read(lhs) as Nodes.AndExpression<ExpressionNode, ExpressionNode>;
						break;
					case RHS["||"]:
						lhs = OrExpression.read(lhs) as Nodes.OrExpression<ExpressionNode, ExpressionNode>;
						break;

					case RHS["="]:
						lhs = ReassignmentExpression.read(lhs);
						break;

					case RHS["=="]: // @ts-expect-error fallthrough
					case RHS["!="]:
						maybe_read(CharCode["="]); // allow '===' and '!=='
					case RHS[">"]:
					case RHS[">="]:
					case RHS["<"]:
					case RHS["<="]:
						lhs = ComparisonExpression.read(lhs, rhs as any);
						break;

					case RHS["+"]:
					case RHS["-"]:
					case RHS["*"]:
					case RHS["/"]:
					case RHS["%"]:
					case RHS["&"]:
					case RHS["|"]:
					case RHS["^"]:
					case RHS["<<"]:
					case RHS[">>"]:
						lhs = OperationExpression.read(lhs, rhs as any);
						break;

					case RHS["+="]:
					case RHS["-="]:
					case RHS["*="]:
					case RHS["/="]:
					case RHS["%="]:
					case RHS["&="]:
					case RHS["|="]:
					case RHS["^="]:
					case RHS["<<="]:
					case RHS[">>="]:
						lhs = ReassignmentOperationExpression.read(lhs, rhs as any);
						break;

					case RHS.None:
						break loop;

					default:
						__DEV__: exit.never();
				}
			}
			return lhs;
		})
	);
}
