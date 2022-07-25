import { Identifier, read_Identifier_or_ItemPath_unbound } from ".";
import {
	is_MaybeAsyncNode,
	is_MaybeExternNode,
	is_MaybeMoveNode,
	is_MaybePubNode,
	is_MaybeStaticNode,
	is_MaybeUnsafeNode,
	start,
} from "../../utils/ast";
import { CharCode } from "../../utils/CharCode";
import { d } from "../../utils/debug";
import { assert, exit } from "../error";
import * as Nodes from "../nodes";
import {
	ExpressionNode,
	Feature,
	MaybeAsyncNode,
	MaybeConstNode,
	MaybeMoveNode,
	MaybePubNode,
	MaybeStaticNode,
	MaybeUnsafeNode,
	NodeType,
} from "../nodes";
import {
	EDGECASE_STEPBACK_TO_POS,
	FG_property_true,
	KW_IS,
	match,
	match_keyword,
	maybe_read,
	peek_keyword,
	peek_not_match,
	read,
	read_ahead,
	read_ahead_extern,
	safe_skip_keyword,
} from "../state";
import { FileLoc, FileLoc_ReadAhead } from "../state/constructor";
import { Keyword } from "../state/scanner";
import { Literal } from "./literals";

class PubSpecifier extends FileLoc(Nodes.PubSpecifier) {
	read() {
		switch (peek_keyword()) {
			case Keyword.pub:
				safe_skip_keyword(Keyword.pub);
				if (maybe_read(CharCode["("])) {
					switch (peek_keyword()) {
						case Keyword.in:
							safe_skip_keyword(Keyword.in);
							this.location = read_Identifier_or_ItemPath_unbound();
							read(CharCode[")"]);
							break;
						case Keyword.self:
						case Keyword.super:
						case Keyword.crate:
							this.location = Identifier.read();
							read(CharCode[")"]);
							break;
						default:
							EDGECASE_STEPBACK_TO_POS(start(this) + "pub".length);
							this.location = undefined;
							break;
					}
				} else {
					this.location = undefined;
				}
				break;
			case Keyword.crate:
				this.location = Identifier.read();
				break;
			default:
				__DEV__: exit.never();
		}
	}
}

export function read_with_pub_specifier<T extends MaybePubNode>(fn: () => T): T {
	return read_ahead(() => {
		const pub = PubSpecifier.read();
		const node = fn();
		if (!is_MaybePubNode(node)) exit.at(node, d`Expected 'pub' target, not ${node}`);
		node.pub = pub;
		return node;
	});
}

export function will_match_crate_specifier() {
	__DEV__: assert(KW_IS(Keyword.crate));
	// crate••••••••••••
	// crate::foo•••••••
	// ^- You are here
	return peek_not_match("crate".length, CharCode[":"]);
}

export function maybe_read_with_pub_specifier<T extends MaybePubNode>(fn: () => T): T {
	switch (peek_keyword()) {
		case Keyword.pub:
			return read_with_pub_specifier(fn);
		case Keyword.crate:
			return will_match_crate_specifier() ? read_with_pub_specifier(fn) : fn();
		default:
			return fn();
	}
}

export class ExternSpecifier extends FileLoc_ReadAhead(Nodes.ExternSpecifier) {
	read(abi: this["abi"]) {
		this.abi = abi;
	}
}

export function maybe_read_abi() {
	return match(CharCode['"']) || match_keyword(Keyword.StringLiteral) ? Literal.read() : undefined;
}

export function read_with_extern_specifier<R extends Extract<Nodes.Node, Nodes.ExternTarget>>(fn: () => R) {
	return read_ahead_extern(() => {
		safe_skip_keyword(Keyword.extern);
		const extern = ExternSpecifier.read(maybe_read_abi());
		const node = fn();
		if (!is_MaybeExternNode(node)) exit.at(node, d`'extern' cannot be used with ${node}`);
		node.extern = extern;
		return node;
	});
}

export function apply_const_modifier<T extends MaybeConstNode | Nodes.ExpressionStatement<MaybeConstNode & ExpressionNode>>(node: T): T {
	switch (node.nodeType) {
		case NodeType.ExpressionStatement:
			node.expression.const = FG_property_true(Feature["inline_const"], node.expression, "const");
			break;
		case NodeType.BlockExpression:
			node.const = FG_property_true(Feature["inline_const_pat"] /** or Feature["inline_const"] */, node, "const");
			break;
		case NodeType.FunctionDeclaration:
			node.const = true;
			break;
		default:
			exit.at(node, d`'const' cannot be used with ${node}`);
	}
	return node;
}

export function read_with_const_modifier<T extends MaybeConstNode>(read: () => T): T {
	return read_ahead(() => (safe_skip_keyword(Keyword.const), apply_const_modifier(read())));
}

export function read_with_async_modifier<T extends MaybeAsyncNode>(read: () => T): T {
	return read_ahead(() => {
		safe_skip_keyword(Keyword.async);
		const node = read();
		if (!is_MaybeAsyncNode(node)) exit.at(node, d`Expected 'async' target, found ${node}`);
		node.async = true;
		return node;
	});
}

export function read_with_move_modifier<T extends MaybeMoveNode>(read: () => T): T {
	return read_ahead(() => {
		safe_skip_keyword(Keyword.move);
		const node = read();
		if (!is_MaybeMoveNode(node)) exit.at(node, d`Expected 'move' target, found ${node}`);
		node.move = true;
		return node;
	});
}

export function read_with_unsafe_modifier<T extends MaybeUnsafeNode>(read: () => T): T {
	return read_ahead(() => {
		safe_skip_keyword(Keyword.unsafe);
		const node = read();
		if (!is_MaybeUnsafeNode(node)) exit.at(node, d`Expected 'unsafe' target, found ${node}`);
		node.unsafe = true;
		return node;
	});
}

export function read_with_static_modifier<T extends MaybeStaticNode>(read: () => T): T {
	return read_ahead(() => {
		safe_skip_keyword(Keyword.static);
		const expr = read();
		if (!is_MaybeStaticNode(expr)) exit.at(expr, d`Expected 'static' target, found ${expr}`);
		expr.static = FG_property_true(Feature["generators"], expr, "static");
		return expr;
	});
}
