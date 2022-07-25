import {
	assignAttributes,
	end,
	insertNode,
	insertNodes,
	is_MaybeExternNode,
	is_Program,
	setRange,
	setRangeEnd,
	setRangeStart,
	start,
} from "../../utils/ast";
import { CharCode } from "../../utils/CharCode";
import { binarySearch, getLineStarts, last_of, Narrow, spliceAll, strChar, strToken } from "../../utils/common";
import {
	getDelimEndCharCode,
	getDelimStartCharCode,
	is_whitespaceOrSlash,
	is_XID_Continue,
	is_XID_Start,
	SomeDelimKind,
} from "../../utils/enum";
import { CharcodeOrUnicodeWhitespace } from "../../utils/unicode";
import { assert, exit } from "../error";
import { createLocArray } from "../helpers";
import {
	AttributeOrDocComment,
	AttributeTarget,
	Comment,
	DelimKind,
	DelimNameMap,
	ExternTarget,
	Feature,
	FG_Map,
	Loc,
	LocArray,
	Located,
	MatchExpression,
	Node,
	NodeType,
	NodeWithBodyNoBody,
	NodeWithBodyOrCases,
	PRCD,
	Program,
	ProgramLike,
	Snippet,
	SourceFile,
	StatementNode,
} from "../nodes";
import { dev_discardNode } from "./constructor";
import { devonly, devonly_check, devonly_cleanup, devonly_getDevonlyObject } from "./debug";
import { Keyword, kwTree, RHS, ToKeyword } from "./scanner";
import {
	didJustSkipWhitespace,
	getPreWhitespaceSkipPosition,
	setPostSkipWhitespace,
	setPreSkipWhitespace,
	skip_whitespace,
	_get_ws_postskip_pos,
	_get_ws_preskip_pos,
} from "./whitespace";

//#region ================================================[        COMMON        ]==========================================================``--.

let src: SourceFile = undefined!;
let src_text = "";

let pos = 0;
let line = 0;

const CharCode_INIT = 0;
const CharCode_EOF = 1;
let _cc: CharCode = CharCode_INIT;

export const IS_PARSING = () => CharCode_INIT !== _cc;

export const GET_SOURCE = () => src;
export const GET_LENGTH = () => src.code.length;
export const GET_SOURCETEXT = () => src?.code ?? "";
export const GET_SOURCEFILEPATH = () => src?.filepath ?? "";
export const GET_POSITION = () => pos;

// devonly
const PEEK = (i: number) => src_text.charCodeAt(pos + i);
export const PEEK_MATCH = (i: number, char: CharCode) => char === PEEK(i);
export const MATCH_ANY = (...seqs: string[]) => seqs.some((seq) => src_text.startsWith(seq, pos));
export const ENDS_WITH = (seq: string) => src_text.endsWith(seq, getPreWhitespaceSkipPosition());
export const GET_KEYWORD_NAME = (kw: Keyword) => ToKeyword[kw];
export const KW_IS = (target: Keyword) => kw_endpos > pos && target === kw;

const DevCC = devonly(() => {
	const INFINITE_LOOP_THRESHOLD = 256;
	let cc_reads = 0;
	let top_pos_queried = -1;
	let back_pos_queries = 0;
	return {
		debug: () => (pos === top_pos_queried ? {} : { pos, top_pos_queried }),
		stats: () => ({
			"Total '.charCodeAt()' calls":
				cc_reads > GET_LENGTH()
					? `${cc_reads} (${Math.round(100 * ((cc_reads - GET_LENGTH()) / GET_LENGTH()))}% re-reads)`
					: "unknown",
		}),
		charCodeAt(target: number) {
			if (target < 0 || target >= src_text.length) {
				exit(`Attempted out of bounds read (at pos + ${pos - target})`);
			}
			if (target > top_pos_queried) {
				top_pos_queried = target;
				back_pos_queries = 0;
			} else {
				if (++back_pos_queries > INFINITE_LOOP_THRESHOLD) {
					exit.infinite();
				}
			}

			++cc_reads;
		},
		match(data: string | CharCode | CharCode[]) {
			if (typeof data === "number" ? !match(data) : !src_text.startsWith(strChar(data), pos))
				exit(`Attempted to read '${strChar(data)}'`);
		},
	};
});

function ccAt(index: number) {
	__DEV__: DevCC.charCodeAt(index);
	return src_text.charCodeAt(index);
}

const _ccAt = (index: number): CharCode => {
	__DEV__: if (pos !== index || _cc === CharCode_EOF) exit.never();
	return (_cc = ccAt(index));
};
const _ccSet = (pos: number): void => {
	_ccAt(pos);
};
const _setPos = (npos: number): void => {
	_ccAt((pos = npos));
};
export const currChar = (): CharcodeOrUnicodeWhitespace => {
	__DEV__: if (!IS_PARSING()) exit("Stale cache read");
	return _cc;
};

//#---------------------------------------------------+        State Snapshot        +------------------------------------------------------.

type StateSnapshot = ReturnType<typeof __create_save_state>;
function __create_save_state() {
	return [pos, line, kw, kw_endpos, _get_ws_preskip_pos(), _get_ws_postskip_pos()] as const;
}
function __restore_init_state() {
	_cc = CharCode_INIT;
	pos = 0;
	line = 0;
	kw = Keyword.NotAWord;
	kw_endpos = -1;
	setPreSkipWhitespace(0);
	setPostSkipWhitespace(0);
	ATTRIBUTES_INNER.length = 0;
	ATTRIBUTES_OUTER.length = 0;
	ATTRIBUTES_DANGLING.length = 0;
	COMMENTS.length = 0;
}
function __restore_save_state(state: StateSnapshot) {
	if (pos !== state[0]) {
		___restorePos(state[0]);
		line = state[1];
		kw = state[2];
		kw_endpos = state[3];
		setPreSkipWhitespace(state[4]);
		setPostSkipWhitespace(state[5]);
	}
}
function __restore_state_to_pos(target: number) {
	__DEV__: assert(target < pos);
	___restorePos(target);
	if (pos > kw_endpos) kw_endpos = -1;
	while (src.lineStarts[line] > pos) --line;
	__DEV__: assert(!is_whitespaceOrSlash(PEEK(-1)), "Attempted to walk state back to a position post whitespace skip");
}

function ___restorePos(target: number) {
	__DEV__: DevStateRollback.set(target);
	_setPos(target);
	__discardAfterPos(target);
}

function __discardAfterPos(target: number) {
	discard(COMMENTS);
	discard(ATTRIBUTES_DANGLING);
	discard(ATTRIBUTES_INNER);
	discard(ATTRIBUTES_OUTER);
	// fails to discard devtools ANCESTRY for e.g. self parameter lifetime
	function discard(arr: AttributeOrDocComment[] | Comment[]) {
		while (0 !== arr.length && start(last_of(arr)) >= target) {
			__DEV__: dev_discardNode(last_of(arr));
			arr.pop();
		}
	}
}

const DevStateRollback = devonly(function () {
	let last_state_restore = -1;
	let state_rollbacks = 0;
	return {
		debug: () => (last_state_restore === -1 ? {} : { last_state_restore: `pos - ${pos - last_state_restore}` }),
		stats: () => ({ state_rollbacks }),
		set(target: number) {
			assert(target >= 0 && target < pos);
			last_state_restore = target;
			++state_rollbacks;
		},
	};
});

export const EDGECASE_STEPBACK_TO = (target: Located): void => {
	if (pos !== end(target)) {
		__restore_state_to_pos(end(target));
		skip_whitespace();
	}
};

export const EDGECASE_STEPBACK_TO_POS = (target: number): void => {
	if (pos !== target) {
		__restore_state_to_pos(target);
		skip_whitespace();
	}
};

export const SNIPPET_endAt = (endpos: number): void => {
	if (pos !== endpos) {
		if (endpos !== getPreWhitespaceSkipPosition()) {
			exit.at(getPreWhitespaceSkipPosition(), "Snippet failed to parse all tokens");
		}
		// __restore_state_to_pos(endpos);
		__discardAfterPos(endpos);
	}
};

//#------------------------------------------------------+        helpers        +----------------------------------------------------------.

export const match = (char: CharCode): boolean => char === currChar();
export const match_2 = (char_0: CharCode, char_1: CharCode): boolean => match(char_0) && peek_match(1, char_1);
export const match_3 = (char_0: CharCode, char_1: CharCode, char_2: CharCode): boolean => match_2(char_0, char_1) && peek_match(2, char_2);
export const not_match = (char: CharCode): boolean => char !== currChar();
export const peek = (n: number): CharcodeOrUnicodeWhitespace => ccAt(n + pos);
export const peek_match = (n: number, char_n: CharCode): boolean => char_n === peek(n);
export const peek_not_match = (n: number, char_n: CharCode): boolean => char_n !== peek(n);
export const step = (): void => _ccSet(++pos);
export const nStep = (n: number): void => _ccSet((pos += n));
const step_ws = (): void => (step(), skip_whitespace());
const nStep_ws = (n: number): void => (nStep(n), skip_whitespace());
// prettier-ignore
export const step_until_match = (char: Exclude<CharCode, 10>): void => {
	if (match(CharCode_EOF)) exit.unexpected();
	loop: while (true) {
		switch (currChar()) {
			case char: break loop;
			case CharCode["\n"]: safe_step_eol(); break;
			default: step(); break;
		}
	}
};
export const step_until_ln = (): void => {
	if (match(CharCode_EOF)) exit.unexpected();
	while (not_match(CharCode["\n"])) step();
};
export const step_over = (char: CharCode): void => {
	if (match(char)) step();
	else exit.expected(char);
};
export const step_over_3 = (char_0: CharCode, char_1: CharCode, char_2: CharCode): void => {
	if (match_3(char_0, char_1, char_2)) nStep(3);
	else exit.expected(strToken([char_0, char_1, char_2]));
};
export const safe_step_over = (char: CharCode): void => {
	__DEV__: DevCC.match(char);
	step();
};
export const safe_step_over_2 = (word: string): void => {
	__DEV__: assert(word.length === 2), DevCC.match(word);
	nStep(2);
};
export const maybe_step_over = (char: CharCode): boolean => match(char) && (step(), true);
export const read = (char: CharCode): void => {
	if (match(char)) step_ws();
	else exit.expected(char);
};
export const read_2 = (char_0: CharCode, char_1: CharCode): void => {
	if (!maybe_read_2(char_0, char_1)) exit.expected(strToken([char_0, char_1]));
};
export const maybe_read = (char_0: CharCode): boolean => match(char_0) && (step_ws(), true);
export const maybe_read_2 = (char_0: CharCode, char_1: CharCode): boolean => match(char_0) && maybe_skip_1_read_2(char_1);
// prettier-ignore
export function maybe_skip_1_read_2(char_1: CharCode): boolean {
	switch (peek(1)) {
		case char_1:
			nStep_ws(2);
			return true;
		// #ws
		case 9: case 10: case 11: case 12: case 13: case 32: case 133: case 8206: case 8207: case 8232: case 8233:
		case 35:
		case 47: {
			const state = __create_save_state();
			step_ws();
			if (match(char_1)) {
				step_ws();
				return true;
			} else {
				__restore_save_state(state);
				return false;
			}
		}
		default:
			return false;
	}
}

export const match_keyword = (keyword: Keyword): boolean => keyword === peek_keyword();
export const not_match_keyword = (keyword: Keyword): boolean => keyword !== peek_keyword();
export const read_keyword = (kw: Keyword): void => {
	if (match_keyword(kw)) safe_skip_keyword(kw);
	else exit.expected(`Keyword '${GET_KEYWORD_NAME(kw)}'`);
};
export const maybe_read_keyword = (kw: Keyword): boolean => match_keyword(kw) && (safe_skip_keyword(kw), true);
export const safe_skip = (char_0: CharCode): void => {
	safe_step_over(char_0);
	skip_whitespace();
};
export const safe_skip_word = (WORD: string): void => {
	__DEV__: DevCC.match(WORD), assert(!is_XID_Continue(PEEK(WORD.length)));
	nStep_ws(WORD.length);
};
export const safe_skip_1_read_2 = (char_0: CharCode, char_1: CharCode): void => {
	__DEV__: DevCC.match(char_0);
	if (peek_match(1, char_1)) {
		nStep_ws(2);
	} else {
		safe_skip(char_0);
		if (match(char_1)) safe_skip(char_1);
		else exit.expected(strToken([char_0, char_1]), getPreWhitespaceSkipPosition() - 1);
	}
};
export function read_between<T>(char_start: number, read_content: () => T, char_end: number): T {
	safe_skip(char_start);
	const res = read_content();
	read(char_end);
	return res;
}

export const will_match_charLiteral_not_lt = (): boolean => {
	__DEV__: DevCC.match(CharCode["'"]);
	return peek_match(2, CharCode["'"]) || peek_match(1, CharCode["\\"]) || !is_XID_Start(peek(1));
};
export const ASSERT_WILL_MATCH_CHARLITERAL_NOT_LT = (d: number) =>
	assert(
		PEEK_MATCH(d, CharCode["'"]) &&
			(PEEK_MATCH(d + 2, CharCode["'"]) || PEEK_MATCH(d + 1, CharCode["\\"]) || !is_XID_Start(PEEK(d + 1)))
	);

export const will_match_lt = (): boolean => match(CharCode["'"]) && !will_match_charLiteral_not_lt();

//#-----------------------------------------------------+        Sequences        +---------------------------------------------------------.

// prettier-ignore
export const FOR_EACH_UNTIL = (__RUN__: () => void, CHAR_SEP: CharCode, CHAR_END: CharCode) => {
	loop: while (match(CHAR_SEP)) {
		safe_skip(CHAR_SEP);
		switch (currChar()) { case CHAR_END: case CharCode_EOF: break loop; }
		__RUN__();
	}
	if (match(CHAR_END)) safe_skip(CHAR_END);
	else exit.expected([CHAR_SEP, CHAR_END]);
};

// prettier-ignore
export const FOR_ANY_BETWEEN = (CHAR_START: CharCode, __RUN__: () => void, CHAR_END: CharCode) => {
	safe_skip(CHAR_START);
	loop: while (true) {
		switch (currChar()) { case CHAR_END: break loop; case CharCode_EOF: exit.expected(CHAR_END); }
		__RUN__();
	}
	safe_skip(CHAR_END);
};

export function readLocatedArrayNoDelim<T extends Node>(fn: (arr: LocArray<T, "None">) => void): LocArray<T, "None"> {
	const __dev_startPos = pos;
	const ARRAY = createLocArray<T, DelimKind.None>(DelimKind.None, new Loc(src, 0, 0));
	fn(ARRAY);
	if (ARRAY.length === 0) {
		__DEV__: assert(__dev_startPos === pos);
		setRange(ARRAY, pos, pos);
	} else {
		__DEV__: assert(start(ARRAY[0]) <= __dev_startPos), assert(end(last_of(ARRAY)) === getPreWhitespaceSkipPosition());
		setRange(ARRAY, start(ARRAY[0]), getPreWhitespaceSkipPosition());
	}
	return ARRAY;
}

// prettier-ignore
export function readLocatedArrayDelim<T extends Node, K extends DelimKind>(tk: K, fn: (arr: LocArray<T, DelimNameMap[K]>) => void): LocArray<T, DelimNameMap[K]> {
	const ARRAY = createLocArray<T, K>(tk, new Loc(src, pos, 0));
	fn(ARRAY);
	__DEV__: assert(ARRAY.length === 0 ? pos !== start(ARRAY) : start(ARRAY) <= start(ARRAY[0]));
	setRangeEnd(ARRAY, getPreWhitespaceSkipPosition());
	return ARRAY;
}

// prettier-ignore
export const read_sequence = <T extends Node, K extends SomeDelimKind>(TK: K, EACH: (SEQUENCE: unknown[]) => T): LocArray<T, DelimNameMap[K]> => {
	__DEV__: DEV_lastSequence.set(null);

	const SEQUENCE = createLocArray<T, K>(TK, new Loc(src, pos, 0));

	safe_skip(getDelimStartCharCode(TK));
	
	const END = getDelimEndCharCode(TK);

	LOOP: {
		while (not_match(END)) {
			SEQUENCE.push(EACH(SEQUENCE));
			switch (currChar()) {
				default: exit.expected([COMMA, END]);
				case END: __read_sequence_lastHadTrailingSeparator = false; break LOOP;
				case COMMA: step_ws(); break;
			}
		}
		__read_sequence_lastHadTrailingSeparator = true;
	}

	__set_endPos_eat(SEQUENCE, END);

	__DEV__: DEV_lastSequence.set(SEQUENCE);

	return SEQUENCE;
};

// prettier-ignore
export const read_group = <T extends AttributeTarget & Node, K extends SomeDelimKind>(SELF: Exclude<NodeWithBodyOrCases, Program> | Snippet, TK: K, EACH: () => T) => {
	const SEQUENCE = createLocArray<T, K>(TK, new Loc(src, pos, 0));
	switch (SELF.nodeType) {
		case NodeType.Snippet: SELF.ast = SEQUENCE as any; break;
		case NodeType.MatchExpression: SELF.cases = SEQUENCE as any; break;
		default: 
			Narrow<Exclude<NodeWithBodyOrCases, Program | MatchExpression>>(SELF);
			SELF.body = SEQUENCE as any; break;
	}
	const END = getDelimEndCharCode(TK);
	read(getDelimStartCharCode(TK));
	LOOP: while (true) {
		maybe_read_inner_attributes(SELF);
		switch (currChar()) { case END: break LOOP; case CharCode_EOF: exit.expected(END); }
		SEQUENCE.push(with_outerAttributes_fromStatementContext(() => EACH()));
	}
	__set_endPos_eat(SEQUENCE, END);
};

export const read_group_noGroup = (SELF: NodeWithBodyNoBody) => {
	read(CharCode["{"]);
	maybe_read_inner_attributes(SELF);
	if (!maybe_read(CharCode["}"])) {
		if (match(CharCode_EOF)) exit.expected(CharCode["}"]);
		else exit(`${SELF.type} cannot define items, expected '}'`);
	}
};

export const read_group_noDelim = (SELF: Program | Snippet, EACH: () => StatementNode) => {
	const SEQUENCE = (SELF.ast = createLocArray<StatementNode, DelimKind.None>(DelimKind.None, new Loc(src, pos, pos)));

	maybe_read_inner_attributes(SELF);

	if (not_match(CharCode_EOF)) {
		do {
			SEQUENCE.push(with_outerAttributes_fromStatementContext(() => EACH()));
			maybe_read_inner_attributes(SELF);
		} while (not_match(CharCode_EOF));
		__DEV__: assert(start(SEQUENCE[0]) <= start(SEQUENCE)), assert(end(last_of(SEQUENCE)) === getPreWhitespaceSkipPosition());
		setRange(SEQUENCE, start(SEQUENCE[0]), getPreWhitespaceSkipPosition());
	}
};

export const sequence_hasTrailingComma = (sequence: LocArray<any, any>) => {
	__DEV__: DEV_lastSequence.is(sequence);
	return __read_sequence_lastHadTrailingSeparator;
};

const COMMA = CharCode[","];
let __read_sequence_lastHadTrailingSeparator = false;
const DEV_lastSequence = devonly(() => {
	let l: LocArray | null;
	const n = sequence_hasTrailingComma.name;
	return {
		set: (arr: LocArray<any, any> | null) => (l = arr),
		is: (s: LocArray) => s === l || exit(`'${n}(<sequence>)' expects last read sequence`, { expected: l, got: s }),
	};
});

//#endregion ===============================================================================================================================..--'

//#region ========================================[        KEYWORDS / IDENTIFIERS        ]==================================================``--.

let kw: Keyword = 0;
let kw_endpos: number = -1;

export function peek_keyword() {
	if (pos > kw_endpos) {
		kw_endpos = pos;

		kw = kwTree();

		if (kw_endpos !== pos) {
			const kw_startpos = kw_endpos;
			kw_endpos = pos;
			_setPos(kw_startpos);
		}
	}
	return kw;
}

export const invalidate_kw = () => {
	__DEV__: assert(pos === kw_endpos || (kw === Keyword.Underscore && kw_endpos === pos + 1));
	kw_endpos = -1;
};

export function read_identifier_with(char: CharCode) {
	__DEV__: assert(kw_endpos <= pos);
	safe_step_over(char);
	if (!is_XID_Start(currChar())) exit.expected("Identifier");
	read_XID_CONTINUE();
}

export function read_XID_CONTINUE() {
	__DEV__: assert(is_XID_Continue(currChar()));
	while (is_XID_Continue(uc_next()));
}

export function safe_skip_keyword(keyword: Keyword) {
	__DEV__: {
		assert(kw_endpos > pos, `Attempted safe_skip_keyword(Keyword.${GET_KEYWORD_NAME(keyword)}) before reading keyword`);
		assert(kw === keyword, `Attempted safe_skip_keyword(Keyword.${GET_KEYWORD_NAME(keyword)}) at Keyword.${GET_KEYWORD_NAME(kw)}`);
		switch (kw) {
			case Keyword["macro_rules!"]:
				assert(kw_endpos === pos + "macro_rules".length);
				break;
			case Keyword.StringLiteral:
			case Keyword.NotAWord:
				exit(`Attempted to read identifier`);
			case Keyword.NotKeyword:
				DevCC.match(GET_SOURCETEXT()!.slice(pos, kw_endpos));
				break;
			case Keyword.Underscore:
				DevCC.match("_");
				assert(kw_endpos === pos + 1);
				break;
			case Keyword.RawIdentifier:
				DevCC.match("r#");
				break;
			default:
				DevCC.match(GET_KEYWORD_NAME(kw));
				break;
		}
	}

	_setPos(kw_endpos);
	invalidate_kw();
	skip_whitespace();
}

export const read_cached_keyword = () => safe_skip_keyword(kw);

//#endregion ===============================================================================================================================..--'

//#region ==========================================[        Position patching        ]=====================================================``--.

export const read_ahead = <T extends Node>(fn: () => T): T => __patch_startPos(pos, fn());
export const CCPATH_read = <T extends { new (...args: any[]): any; read(namespace: undefined): any }>(PATH_NODE: T): InstanceType<T> =>
	__patch_startPos(pos, (safe_skip_1_read_2(CharCode[":"], CharCode[":"]), PATH_NODE.read(undefined)));

export const __patch_startPos = <T extends Located>(startPos: number, node: T): T => (setRangeStart(node, startPos), node);
export const __patch_endPos = <T extends Located>(endPos: number, node: T): T => (setRangeEnd(node, endPos), node);

export const __inherit_startPos = <T extends Located>(to: T, from: Located): T => __patch_startPos(start(from), to);
export const __inherit_endPos = <T extends Located>(to: T, from: Located): T => __patch_endPos(end(from), to);
export const __inherit_range = <T extends Located>(to: T, from: Located): T => {
	__patch_startPos(start(from), to);
	__patch_endPos(end(from), to);
	return to;
};

export const __set_endPos_eat = (target: Located, char: CharCode) => {
	setRangeEnd(target, pos + 1);
	safe_skip(char);
};

export function read_ahead_extern<T extends Extract<Node, ExternTarget>>(fn: () => T): T {
	const startPos = pos;
	const node = fn();
	if (!is_MaybeExternNode(node)) exit.at(node, "Expected `extern` target");
	__DEV__: assert(!!node.extern);
	setRangeStart(node.extern, startPos);
	setRangeStart(node, startPos);
	return node;
}

export function read_ahead_maybe_extern<T extends Node>(fn: () => T): T {
	const startPos = pos;
	const node = fn();
	if ("extern" in node) {
		__DEV__: assert(!!node.extern);
		setRangeStart(node.extern, startPos);
		setRangeEnd(node.extern, node.extern.abi ? end(node.extern.abi) : startPos + 6);
	}
	setRangeStart(node, startPos);
	return node;
}

export function check_ahead<T>(check: () => T): T {
	const state = __create_save_state();
	const res = check();
	__restore_save_state(state);
	return res;
}

export function maybe_read_if_check(char: CharCode, check: () => boolean) {
	if (match(char)) {
		const state = __create_save_state();
		safe_skip(char);
		if (true === check()) return true;
		__restore_save_state(state);
	}
	return false;
}

export function read_ahead_either<A extends Node, B extends Node>(fn_0: () => A | undefined, fn_1: () => B): A | B {
	const state = __create_save_state();
	return read_ahead(() => fn_0() ?? (__restore_save_state(state), fn_1()));
}

//#endregion ===============================================================================================================================..--'

//#region ===============================================[        Features        ]=========================================================``--.

// prettier-ignore
export function FG_property<F extends Feature, N extends FG_Map[F], K extends keyof N>(feature: F, node: N, key: K, read_value: () => N[K]): N[K] {
	return read_value();
}

export function FG_property_true<F extends Feature, N extends FG_Map[F], K extends keyof N>(feature: F, node: N, key: K): true {
	return true;
}

//#endregion ===============================================================================================================================..--'

//#region ===============================================[        Program        ]==========================================================``--.

export function withParserState<R extends Extract<Node, ProgramLike>>(SOURCE: SourceFile, START_POS: number, READ_AST: () => R): R {
	__DEV__: assert(_cc === CharCode_INIT), assert(src === undefined);

	try {
		src = SOURCE;

		if (START_POS >= src.code.length) {
			_setEOF();
		} else {
			src_text = withLF(src.code);
			_setPos(START_POS);
		}

		if (0 !== START_POS) {
			if (1 === src.lineStarts.length) src.lineStarts = getLineStarts(src.code);
			line = binarySearch(src.lineStarts, START_POS);
		}

		const PROGRAM = READ_AST();

		if (0 !== ATTRIBUTES_INNER.length) insertNodes(ATTRIBUTES_DANGLING, ATTRIBUTES_INNER);
		if (0 !== ATTRIBUTES_OUTER.length) insertNodes(ATTRIBUTES_DANGLING, ATTRIBUTES_OUTER);

		PROGRAM.danglingAttributes = [...ATTRIBUTES_DANGLING];
		PROGRAM.comments = [...COMMENTS];
		__DEV__: PROGRAM.__devonly = devonly_getDevonlyObject();

		__DEV__: devonly_check(is_Program(PROGRAM) ? PROGRAM.loc.src : PROGRAM);

		return PROGRAM;
	} finally {
		src = undefined!;
		src_text = "";

		__restore_init_state();

		__es_optional_start = -1;

		__ctx_ES_PRCD_i = 0;
		__ctx_ES_i = 0;
		__ctx_TY_i = 0;
		__ctx_MC_i = 0;

		__DEV__: devonly_cleanup();
	}
}

function withLF(code: string): string {
	return 10 === code.charCodeAt(code.length - 1) ? code : code + "\n";
}

//#endregion ===============================================================================================================================..--'

//#region =============================================[        Parser State        ]=======================================================``--.

devonly(() => {
	return {
		debug: () => ({
			__ctx_Precedence: __ctx_Precedence.slice(0, __ctx_ES_PRCD_i + 1),
			max_Precedence_depth: __ctx_Precedence.length,
			max_ES_ctx_depth: __ctx_exceptStructFormExpression.length,
			max_TY_depth: __ctx_allowMultipleBounds.length,

			__es_optional_start,
			__ctx_ES_i,
			__ctx_ES_PRCD_i,
			__ctx_TY_i,
			__ctx_MC_i,
		}),
	};
});

//#-------------------------------------------------+        Expression Context        +----------------------------------------------------.

/** Whether reading an expression is allowed to fail */
let __es_optional_start: number = -1;
export const ES_signal_optional_read = () => (__es_optional_start = pos);
export const ES_consume_optional_read = () => __es_optional_start === ((__es_optional_start = -1), pos);

let __ctx_ES_i = 0;
let __ctx_ES_PRCD_i = 0;
const __ctx_Precedence: PRCD[] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const __ctx_exceptStructFormExpression: boolean[] = [false, false, false, false, false, false, false, false, false, false, false, false];
const __ctx_insideScrutinee: boolean[] = [false, false, false, false, false, false, false, false, false, false, false, false];
const ES_ctx_precedence_pop = () => --__ctx_ES_PRCD_i;
const ES_ctx_precedence_push = (prcd: PRCD) => (__ctx_Precedence[++__ctx_ES_PRCD_i] = prcd);
const ES_ctx_setCurrentPrecedence = (prcd: PRCD) => (__ctx_Precedence[__ctx_ES_PRCD_i] = prcd);
const ES_ctx_hasHigherPrecedence = (prcd: PRCD) => prcd > __ctx_Precedence[__ctx_ES_PRCD_i - 1];

/** Whether StructLiteral is forbidden in context */
export const ES_ctx_exceptStructFormExpression = () => __ctx_exceptStructFormExpression[__ctx_ES_i];
/** Whether `&&` and `||` precedence should be different (let scrutinees) */
export const ES_ctx_insideScrutinee = () => __ctx_insideScrutinee[__ctx_ES_i];

export const ES_withContext = <T>(exceptStructFormExpression: boolean, insideScrutinee: boolean, fn: () => T): T => {
	__ctx_exceptStructFormExpression[++__ctx_ES_i] = exceptStructFormExpression;
	__ctx_insideScrutinee[__ctx_ES_i] = insideScrutinee;
	const res = ES_withPrecedence(insideScrutinee ? PRCD.ScrutineeDefault : PRCD.Default, fn);
	--__ctx_ES_i;
	return res;
};

export const ES_withPrecedence = <T>(prcd: PRCD, fn: () => T): T => {
	ES_ctx_precedence_push(prcd);
	const res = fn();
	ES_ctx_precedence_pop();
	return res;
};

export function rhs_resolve(RHS_next: RHS, PRCD_next: PRCD, RHS_length: number) {
	if (ES_ctx_hasHigherPrecedence(PRCD_next)) {
		ES_ctx_setCurrentPrecedence(PRCD["="] === PRCD_next ? PRCD["="] - 1 : PRCD_next);
		nStep_ws(RHS_length);
		return RHS_next;
	}
	return RHS.None;
}

//#----------------------------------------------------+        Type Context        +-------------------------------------------------------.

let __ctx_TY_i = 0;
const __ctx_allowMultipleBounds: boolean[] = [false, false, false, false, false, false, false, false, false, false, false, false];

/** Whether should read '+' bounds */
export const TY_ctx_allowMultipleBounds = () => __ctx_allowMultipleBounds[__ctx_TY_i];
const TY_ctx_allowMultipleBounds_set = (mB: boolean) => (__ctx_allowMultipleBounds[++__ctx_TY_i] = mB);
const TY_ctx_allowMultipleBounds_pop = () => --__ctx_TY_i;

export const TY_withContext = <T>(fn: () => T, allowMultipleBounds: boolean): T => {
	TY_ctx_allowMultipleBounds_set(allowMultipleBounds);
	const res = fn();
	TY_ctx_allowMultipleBounds_pop();
	return res;
};

//#---------------------------------------------------+        Macro Context        +-------------------------------------------------------.

export const enum Env {
	None,
	Attribute,
	MacroInvocation,
	MacroMatch,
	MacroTransform,
}

let _env: Env = Env.None;
export const getEnv = () => _env;
export const setEnv = (env: Env) => (_env = env);
export const isReadingEnv = (env: Env) => env === _env;

/**
 * Reading TokenTrees parses some nodes differently:
 * 	- [meta] Attribute -> Tokens
 * 	- [meta] DocCommentAttribute -> Comment
 * 	- [ty]   LtStatic -> LtIdentifier
 * 	- [ty]   LtElided -> LtIdentifier
 */

let __ctx_MC_i = 0;
export const Mc_ctx_isReadingTokens = () => 0 !== __ctx_MC_i;
export const MC_ctx_isReadingTokens_start = () => ++__ctx_MC_i;
export const MC_ctx_isReadingTokens_end = () => --__ctx_MC_i;

//#endregion ===============================================================================================================================..--'

//#region ===========================================[        Scanner helpers        ]======================================================``--.

export const uc_eat = () => {
	const cc = _cc;
	return step(), cc;
};
export const uc_next = () => _ccAt(++pos);
export const uc_next_match = (char: number) => char === uc_next();
export const uc_next_eat = (char: number) => uc_next_match(char) && (step(), true);

export function kw_resolve<T extends Keyword>(kw: T) {
	// kw••••••••••••••••
	//  ^- You are here
	if (is_XID_Continue(uc_next())) {
		read_XID_CONTINUE();
		return Keyword.NotKeyword;
	}
	return kw;
}

export function kw_resolve_failed<T extends Keyword>(kw: T) {
	// kw••••••••••••••••
	//  ^- You are here
	if (is_XID_Continue(currChar())) {
		read_XID_CONTINUE();
		return Keyword.NotKeyword;
	}
	return kw;
}

export const edgecase_stepback = () => _ccSet(--pos);

//#endregion ===============================================================================================================================..--'

//#region ==============================================[        Whitespace        ]========================================================``--.

const COMMENTS: Comment[] = [];
const ATTRIBUTES_DANGLING: AttributeOrDocComment[] = [];
const ATTRIBUTES_INNER: AttributeOrDocComment[] = [];
const ATTRIBUTES_OUTER: AttributeOrDocComment[] = [];

const _setEOF = () => {
	src_text = src.code;
	pos = src.code.length;
	_cc = CharCode_EOF;
};

export const step_eol = () => {
	__DEV__: assert(match(CharCode["\n"]), "Attempted to read linebreak");
	if (++pos === src_text.length) _setEOF();
	else {
		__DEV__: if (src.lineStarts.length > line + 1) assert(pos === src.lineStarts[line + 1]);
		_ccSet((src.lineStarts[++line] = pos));
	}
};

export const safe_step_eol = () => {
	step_eol();
	if (match(CharCode_EOF)) exit.unexpected();
};

export const register_comment = (attr: Comment) => COMMENTS.push(attr);
export const register_attribute = (attr: AttributeOrDocComment) => (attr.inner ? ATTRIBUTES_INNER : ATTRIBUTES_OUTER).push(attr);

function __may_claim_attributes(attrs: AttributeOrDocComment[]) {
	if (0 !== attrs.length && didJustSkipWhitespace()) {
		while (start(attrs[0]) < _get_ws_preskip_pos()) {
			insertNode(ATTRIBUTES_DANGLING, attrs.shift()!);
			if (0 === attrs.length) return false;
		}
		__DEV__: assert(!attrs.some((a, i) => (i > 0 && start(a) < start(attrs[i - 1])) || start(a) < _get_ws_preskip_pos()));
		return true;
	}
	return false;
}

export function skip_whitespace_getProgramStartPos() {
	skip_whitespace();
	return Math.min(
		0 === ATTRIBUTES_INNER.length ? pos : start(ATTRIBUTES_INNER[0]), //
		0 === COMMENTS.length ? pos : start(COMMENTS[0]),
		pos
	);
}

export function getProgramEndPos() {
	return Math.max(
		0 === ATTRIBUTES_INNER.length ? 0 : end(last_of(ATTRIBUTES_INNER)), //
		0 === ATTRIBUTES_OUTER.length ? 0 : end(last_of(ATTRIBUTES_OUTER)), //
		0 === ATTRIBUTES_DANGLING.length ? 0 : end(last_of(ATTRIBUTES_DANGLING)), //
		0 === COMMENTS.length ? 0 : end(last_of(COMMENTS)),
		getPreWhitespaceSkipPosition()
	);
}

export function maybe_read_inner_attributes(target: AttributeTarget & Node) {
	if (__may_claim_attributes(ATTRIBUTES_INNER)) {
		assignAttributes(target, ATTRIBUTES_INNER);
		ATTRIBUTES_INNER.length = 0;
	}
}

function with_outerAttributes<T extends Node>(fn: () => T): T {
	if (__may_claim_attributes(ATTRIBUTES_OUTER)) {
		const attributes = spliceAll(ATTRIBUTES_OUTER);
		const target = fn() as Extract<T, AttributeTarget>;
		assignAttributes(target, attributes);
		return target;
	}
	return fn();
}

export function with_outerAttributes_fromParentContext_if_test<T extends Node>(fn: () => T, accepts_args: (node: T) => boolean): T {
	if (__may_claim_attributes(ATTRIBUTES_OUTER)) {
		const attributes = spliceAll(ATTRIBUTES_OUTER);
		const target = fn() as Extract<T, AttributeTarget>;
		if (accepts_args(target)) {
			assignAttributes(target, attributes);
		} else {
			insertNodes(ATTRIBUTES_DANGLING, attributes);
		}
		return target;
	}
	return fn();
}

export function with_outerAttributes_fromStatementContext<T extends AttributeTarget & Node>(fn: () => T): T {
	return with_outerAttributes(fn);
}

export function with_outerAttributes_fromParentContext<T extends Node>(fn: () => T): T {
	return with_outerAttributes(fn);
}

export function FG_with_outerAttributes_fromParentContext<T extends Node>(fn: () => T): T {
	if (__may_claim_attributes(ATTRIBUTES_OUTER)) {
		const attributes = spliceAll(ATTRIBUTES_OUTER);
		const target = fn() as Extract<T, AttributeTarget>;
		assignAttributes(target, attributes);
		{
			// let a = #[b] (target);
			//               ^^^^^^   own range
			//         ^^^^^^^^^^^^   with attribute
			//         ^^^^^^^^^^^^^  with this line
			// __patch_endPos(getPreWhitespaceSkipPosition(), target);
		}
		return target;
	}
	return fn();
}

//#endregion ===============================================================================================================================..--'

export function shouldPreserveParens() {
	return false;
	// return src.parserOptions.preserveParens;
}

export function shouldCollectComments() {
	return true;
	// return src.parserOptions.collectComments;
}
