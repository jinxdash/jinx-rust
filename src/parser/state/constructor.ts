import {
	FG_with_outerAttributes_fromParentContext,
	getProgramEndPos,
	GET_LENGTH,
	GET_POSITION,
	GET_SOURCE,
	shouldPreserveParens,
	skip_whitespace_getProgramStartPos,
	SNIPPET_endAt,
	withParserState,
} from ".";
import {
	can_have_InnerAttributes,
	can_have_OuterAttributes,
	each_node,
	end,
	getParenthesizedNodeContent,
	hasAttributes,
	hasInnerAttributes,
	hasOuterAttributes,
	is_AttributeOrDocComment,
	is_Comment,
	is_LetVariableDeclaration,
	is_ParenthesizedNode,
	ownStart,
	setRangeEnd,
	start,
	unsafe_setRangeStart,
	unsafe_set_nodeType,
} from "../../utils/ast";
import { Array_splice, color } from "../../utils/common";
import { d } from "../../utils/debug";
import { assert, exit } from "../error";
import {
	Attribute,
	AttributeTarget,
	BaseNode,
	Comment,
	DocCommentAttribute,
	Loc,
	Located,
	MissingNode,
	Node,
	NodeType,
	ParenthesizedNode,
	Program,
	Snippet,
	SourceFile,
} from "../nodes";
import { ParserOptions } from "../options";
import { devonly } from "./debug";
import {
	didJustSkipWhitespace,
	getPreWhitespaceSkipPosition,
	setPreSkipWhitespace,
	skip_whitespace,
	_get_ws_preskip_pos,
} from "./whitespace";

//#region ===============================================[        FileLoc        ]==========================================================``--.

type SpecClass = new (nodeType: any, loc: Loc) => BaseNode<any>;

export function FileLocSourceFile<T extends typeof SourceFile>(specClass: T) {
	return mixinNodeReader(
		specClass,
		(...ARGUMENTS: [code: string, parserOptions: ParserOptions]) => new Loc(undefined!, 0, ARGUMENTS[0].length),
		function (READ_NODE, ...ARGUMENTS: [code: string, parserOptions: ParserOptions]) {
			// @ts-expect-error
			this.loc.src = this;

			this.code = ARGUMENTS[0];
			this.filepath = ARGUMENTS[1].filepath;

			this.parserOptions = ARGUMENTS[1];

			this.lineStarts = [0];

			withParserState(this, 0, () => {
				READ_NODE();
				return this.program;
			});
		}
	);
}

export function FileLocProgram<T extends typeof Program>(specClass: T) {
	return mixinNodeReader(
		specClass,
		() => new Loc(GET_SOURCE(), skip_whitespace_getProgramStartPos(), 0),
		function (READ_NODE) {
			READ_NODE();
			setRangeEnd(this, getProgramEndPos(this));
			__DEV__: assert(GET_POSITION() === GET_LENGTH());
		}
	);
}

export function FileLocSnippet<T extends typeof Snippet>(specClass: T) {
	return mixinNodeReader(
		specClass,
		(...ARGUMENTS) => (ARGUMENTS[0] as Located).loc.clone(),
		function (READ_NODE) {
			READ_NODE();
			SNIPPET_endAt(end(this));
		}
	);
}

export function FileLocCommentOrDocComment<T extends typeof Comment | typeof DocCommentAttribute>(specClass: T) {
	return mixinNodeReader(
		specClass,
		() => new Loc(GET_SOURCE(), GET_POSITION(), 0),
		function (READ_NODE) {
			READ_NODE();
			setRangeEnd(this, GET_POSITION());
		}
	);
}

export function FileLocAttr<T extends typeof Attribute | typeof DocCommentAttribute>(specClass: T) {
	return mixinNodeReader(
		specClass,
		() => new Loc(GET_SOURCE(), GET_POSITION(), 0),
		function (READ_NODE) {
			const ws_preskip = _get_ws_preskip_pos();
			READ_NODE();
			setRangeEnd(this, GET_POSITION());
			setPreSkipWhitespace(ws_preskip);
		}
	);
}

function __endRange(target: Located) {
	if (didJustSkipWhitespace()) {
		setRangeEnd(target, _get_ws_preskip_pos());
	} else {
		setRangeEnd(target, GET_POSITION());
		skip_whitespace();
	}
}

export function FileLoc_ReadAhead<T extends SpecClass>(specClass: T) {
	return mixinNodeReader(
		specClass,
		() => new Loc(GET_SOURCE(), 0, 0),
		function (READ_NODE) {
			READ_NODE();
			__endRange(this);
		}
	);
}

export function FileLoc_FromChild<T extends SpecClass>(specClass: T) {
	return mixinNodeReader(
		specClass,
		(...ARGUMENTS) => new Loc(GET_SOURCE(), start(ARGUMENTS[0]), 0),
		function (READ_NODE) {
			READ_NODE();
			__endRange(this);
		}
	);
}

export function FileLoc_FromChildElsePos<T extends SpecClass>(specClass: T) {
	return mixinNodeReader(
		specClass,
		(...ARGUMENTS) => new Loc(GET_SOURCE(), undefined === ARGUMENTS[0] ? GET_POSITION() : start(ARGUMENTS[0]), 0),
		function (READ_NODE) {
			READ_NODE();
			__endRange(this);
		}
	);
}

export function FileLoc_FromChildElseReadAhead<T extends SpecClass>(specClass: T) {
	return mixinNodeReader(
		specClass,
		(...ARGUMENTS) => new Loc(GET_SOURCE(), undefined === ARGUMENTS[0] ? GET_POSITION() : start(ARGUMENTS[0]), 0),
		function (READ_NODE) {
			READ_NODE();
			__endRange(this);
		}
	);
}

export function FileLocLifetime<T extends SpecClass>(specClass: T) {
	return mixinNodeReader(
		specClass,
		() => new Loc(GET_SOURCE(), GET_POSITION() - 1, 0),
		function (READ_NODE) {
			READ_NODE();
			__endRange(this);
		}
	);
}

export function FileLocDollar<T extends SpecClass>(specClass: T) {
	return mixinNodeReader(
		specClass,
		() => new Loc(GET_SOURCE(), GET_POSITION() - 1, 0),
		function (READ_NODE) {
			READ_NODE();
			__endRange(this);
		}
	);
}

export function FileLoc_MissingNode<T extends typeof MissingNode>(specClass: T) {
	return mixinNodeReader(
		specClass,
		() => new Loc(GET_SOURCE(), getPreWhitespaceSkipPosition(), getPreWhitespaceSkipPosition()),
		function (READ_NODE) {
			READ_NODE();
		}
	);
}

export function FileLoc_DEFINES_OWN_RANGE<T extends SpecClass>(specClass: T) {
	return mixinNodeReader(
		specClass,
		() => new Loc(GET_SOURCE(), 0, 0),
		function (READ_NODE) {
			READ_NODE();
			__DEV__: assert(start(this) !== 0 && end(this) !== 0);
		}
	);
}

export function FileLoc_COPY<T extends SpecClass>(specClass: T) {
	return mixinNodeReader(
		specClass,
		(...ARGUMENTS) => new Loc(GET_SOURCE(), start(ARGUMENTS[0]), end(ARGUMENTS[0])),
		function (READ_NODE) {
			READ_NODE();
		}
	);
}

export function FileLoc<T extends SpecClass>(specClass: T) {
	return mixinNodeReader(
		specClass,
		() => new Loc(GET_SOURCE(), GET_POSITION(), 0),
		function (READ_NODE) {
			READ_NODE();
			__endRange(this);
		}
	);
}

// prettier-ignore
function mixinNodeReader<T extends SpecClass>(SPEC_CLASS: T, CREATE_LOC: (...ARGUMENTS: any[]) => Loc, READ_WRAPPER: (this: InstanceType<T>, READ_NODE: () => void, ...ARGUMENTS: any[]) => void) {
	__DEV__: assert(SPEC_CLASS.name in NodeType, `Could not find ${SPEC_CLASS.name} in NodeType[...]`);
	const NODETYPE = NodeType[SPEC_CLASS.name]; // @ts-expect-error
	abstract class LocatedNode extends SPEC_CLASS {
		protected constructor(...ARGUMENTS: any[]) {
			super(NODETYPE, CREATE_LOC(...ARGUMENTS));
			__DEV__: DevAST.on_Node_read(this);
			READ_WRAPPER.call(this as any, () => this.read(...ARGUMENTS), ...ARGUMENTS);
		}
		abstract read(...args: unknown[]): void; // @ts-expect-error
		static read<T2>(this: T2, ...args: Parameters<InstanceType<T2>["read"]>): InstanceType<T> { return new this(...args); }
	}
	return LocatedNode;
}

export function dev_discardNode(node: Node) {
	__DEV__: DevAST.discardNode(node);
}

export function withEscapedParens<T extends Node>(startNode: T, read: (node: T) => T): T {
	if (is_ParenthesizedNode(startNode)) {
		const inner = getParenthesizedNodeContent(startNode);
		if (shouldPreserveParens()) {
			// EXPERIMENTAL
			const nodeType = startNode.nodeType;
			unsafe_set_nodeType(startNode, inner.nodeType);
			const res = read(startNode as any);
			unsafe_set_nodeType(startNode, nodeType);
			return res;
		} else {
			__DEV__: DevAST.escape(startNode);
			const innerStart = start(inner);
			unsafe_setRangeStart(inner, start(startNode));
			const res = read(inner as any);
			unsafe_setRangeStart(inner, innerStart);
			return res;
		}
	} else {
		return read(startNode);
	}
}

export function escapeParens<T extends ParenthesizedNode>(startNode: T) {
	if (shouldPreserveParens()) {
		return startNode;
	} else {
		__DEV__: DevAST.escape(startNode);
		return getParenthesizedNodeContent(startNode);
	}
}

const DevAST = devonly(() => {
	const DEBUG_NODES_SHOWN = 15;
	const ALL_PROGRAM_NODES: Node[] = [];
	const DISCARDED_NODES: Node[] = [];
	const PARENS_NODES = new Map<Node, ParenthesizedNode>();

	function __discard(node: Node) {
		Array_splice(ALL_PROGRAM_NODES, node);
		DISCARDED_NODES.push(node);
		if (PARENS_NODES.has(node)) {
			exit.never();
			PARENS_NODES.delete(node);
		}
	}

	return {
		debug: () => {
			const nodes = ALL_PROGRAM_NODES.slice(-DEBUG_NODES_SHOWN).map(
				(n) =>
					color.unstyle(d`${n}`) +
					(end(n) === 0 //
						? " (...parsing)"
						: ` (-${GET_POSITION() - end(n)})`)
			);
			if (ALL_PROGRAM_NODES.length > DEBUG_NODES_SHOWN) {
				nodes.unshift(`... [${ALL_PROGRAM_NODES.length - DEBUG_NODES_SHOWN} hidden nodes] ...`);
			}
			const discarded_nodes = DISCARDED_NODES.filter((n) => end(n) === 0 || ownStart(n) >= GET_POSITION());
			return { nodes, discarded_nodes };
		},
		stats: () => ({ "Parsed Nodes": ALL_PROGRAM_NODES.length, "Discarded Nodes": DISCARDED_NODES.length }),
		check(ast) {
			assert(ALL_PROGRAM_NODES[0] === ast);
			const UFO = new Set(ALL_PROGRAM_NODES);
			UFO.delete(ast);
			each_node(ast, (node, parent) => {
				if (!parent.loc.contains(node)) {
					const diff0 = start(parent) - start(node);
					const diff1 = end(node) - end(parent);
					if (diff0 > 0) exit.at(node, d`${node} starts ${diff0 + "cs"} before its parent ${parent}`);
					if (diff1 > 0) exit.at(end(node), d`${node} ends ${diff1 + "cs"} after its parent ${parent}`);
				}
				UFO.delete(node);
				if (hasAttributes(node)) {
					if (hasOuterAttributes(node))
						can_have_OuterAttributes(node, parent, true) ||
							exit.at(node, d`Attempted to assign OuterAttribute to ${node} (in ${parent})`);
					if (hasInnerAttributes(node))
						can_have_InnerAttributes(node) || exit.at(node, d`Attempted to assign InnerAttribute to ${node}`);
				}

				if (shouldPreserveParens()) {
				} else {
					if (is_ParenthesizedNode(node)) {
						exit.never();
					}

					if (PARENS_NODES.has(node)) {
						const pchild = node;
						const parens = PARENS_NODES.get(pchild)!;
						if (is_LetVariableDeclaration(parent) && parent.expression === pchild && hasOuterAttributes(pchild)) {
							// see:
							FG_with_outerAttributes_fromParentContext;
						} else {
							assert.at(ownStart(pchild), start(parens) < ownStart(pchild));
							assert.at(end(pchild), end(pchild) < end(parens));
							assert.at(ownStart(parent), ownStart(parent) <= start(parens));
							assert.at(end(parent), end(parens) <= end(parent));
						}
					}
				}
			});
			if (UFO.size)
				exit.at([...UFO][0], d`${[...UFO][0]} is unassigned ${UFO.size > 1 ? `(+${UFO.size - 1} others)` : ""}`, [...UFO]);
		},

		on_Node_read(instance: Node) {
			ALL_PROGRAM_NODES.push(instance);
		},

		discardNode(node: Node) {
			assert(is_Comment(node) || is_AttributeOrDocComment(node));

			__discard(node);
			each_node(node, (child) => __discard(child));
		},

		escape(parens: ParenthesizedNode) {
			assert(is_ParenthesizedNode(parens));
			assert(!shouldPreserveParens());

			const child = getParenthesizedNodeContent(parens);

			assert(!hasAttributes(parens) && !!child);

			assert(start(parens) < start(child));
			assert(end(child) < end(parens));

			__discard(parens);
			PARENS_NODES.set(child, parens);
		},
	};

	function can_have_outer_attributes(node: Node & AttributeTarget, parent: Node & AttributeTarget) {
		can_have_OuterAttributes(node, parent, true) || exit.at(node, d`Attempted to assign OuterAttribute to ${node} (in ${parent})`);
	}

	function can_have_inner_attributes(target: AttributeTarget & Node) {
		can_have_InnerAttributes(target) || exit.at(target, d`Attempted to assign InnerAttribute to ${target}`);
	}
});
