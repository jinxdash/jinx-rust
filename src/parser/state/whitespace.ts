import {
	currChar,
	GET_POSITION,
	match,
	MATCH_ANY,
	Mc_ctx_isReadingTokens,
	nStep,
	peek,
	peek_match,
	peek_not_match,
	register_attribute,
	register_comment,
	safe_step_over,
	shouldCollectComments,
	step,
	step_eol,
	step_until_ln,
	step_until_match,
} from ".";
import { CharCode } from "../../utils/CharCode";
import { assert } from "../error";
import * as Nodes from "../nodes";
import { FileLocCommentOrDocComment } from "./constructor";
import { devonly } from "./debug";

var Attribute: typeof import("../read/macro").Attribute;
export function __EXPORT_ATTR_CTORS(_Attribute: typeof Attribute) {
	Attribute = _Attribute;
}

const DEV_skip = devonly(() => {
	let last_ws = -1;
	let unne_ws = 0;

	return {
		stats: () => ({ "Unnecessary 'skip_whitespace()' calls": unne_ws }),
		skip: () => {
			if (last_ws === (last_ws = GET_POSITION())) unne_ws++;
		},
	};
});

// #ws
export function skip_whitespace(): void {
	__DEV__: DEV_skip.skip();
	// case CharCode["\t"]: case CharCode["\v"]: case CharCode["\f"]: case CharCode["\r"]: case CharCode[" "]: case 8206: case 8207: case 8232: case 8233:
	switch (currChar()) {
		case 9:
		case 11:
		case 12:
		case 13:
		case 32:
		case 133:
		case 8206:
		case 8207:
		case 8232:
		case 8233:
			savePreSkipPos();
			step();
			break;
		case 10:
			savePreSkipPos();
			step_eol();
			break;
		case 35:
			if (Mc_ctx_isReadingTokens()) return;
			savePreSkipPos();
			register_attribute(Attribute.read());
			break;
		case 47:
			if (!will_read_CommentNode()) return;
			savePreSkipPos();
			read_CommentNode();
			break;
		default:
			return;
	}

	while (true) {
		switch (currChar()) {
			case 9:
			case 11:
			case 12:
			case 13:
			case 32:
			case 133:
			case 8206:
			case 8207:
			case 8232:
			case 8233:
				step();
				break;
			case 10:
				step_eol();
				break;
			case 35:
				if (Mc_ctx_isReadingTokens()) {
					savePostSkipPos();
					return;
				}
				register_attribute(Attribute.read());
				break;
			case 47:
				if (!will_read_CommentNode()) {
					savePostSkipPos();
					return;
				}
				read_CommentNode();
				break;
			default:
				savePostSkipPos();
				return;
		}
	}
}

let _ws_preskip_pos = 0;
let _ws_postskip_pos = 0;

let __is_line = false;
let __is_attr = false;
let __is_inner = false;

export const _get_ws_preskip_pos = () => _ws_preskip_pos;
export const _get_ws_postskip_pos = () => _ws_postskip_pos;

export const setPreSkipWhitespace = (pos: number) => (_ws_preskip_pos = pos);
export const setPostSkipWhitespace = (pos: number) => (_ws_postskip_pos = pos);

const savePreSkipPos = () => setPreSkipWhitespace(GET_POSITION());
const savePostSkipPos = () => setPostSkipWhitespace(GET_POSITION());

export const didJustSkipWhitespace = () => GET_POSITION() === _get_ws_postskip_pos();
export const getPreWhitespaceSkipPosition = () => (didJustSkipWhitespace() ? _get_ws_preskip_pos() : GET_POSITION());

class Comment extends FileLocCommentOrDocComment(Nodes.Comment) {
	read() {
		this.line = __is_line;
		read_comment();
	}
}

class DocCommentAttribute extends FileLocCommentOrDocComment(Nodes.DocCommentAttribute) {
	read() {
		this.inner = __is_inner;
		this.line = __is_line;
		read_comment();
	}
}

// prettier-ignore
function will_read_CommentNode() {
	switch (peek(1)) {
		case CharCode["/"]: return __is_line = true;
		case CharCode["*"]: return (__is_line = false), true;
		default: return false;
	}
}

/** MUST CALL {@link will_read_CommentNode} FIRST */
function read_CommentNode(): void {
	// ////	Comment
	// ///	DocCommentAttribute
	// //!	DocCommentAttribute
	// //	Comment

	// /***	Comment
	// /**	DocCommentAttribute
	// /*!	DocCommentAttribute
	// /*	Comment

	switch (Mc_ctx_isReadingTokens() ? 0 : peek(2)) {
		case CharCode["!"]:
			__is_attr = __is_inner = true;
			break;
		case CharCode["*"]:
			if ((__is_attr = !__is_line))
				switch (peek(3)) {
					case CharCode["*"]:
					case CharCode["/"]:
						__is_attr = false;
				}
			__is_inner = false;
			break;
		case CharCode["/"]:
			__is_attr = __is_line && peek_not_match(3, CharCode["/"]);
			__is_inner = false;
			break;
		default:
			__is_attr = __is_inner = false;
			break;
	}

	if (__is_attr) register_attribute(DocCommentAttribute.read());
	else if (shouldCollectComments()) register_comment(Comment.read());
	else read_comment();
}

function read_comment(): void {
	__DEV__: assert(
		Mc_ctx_isReadingTokens() || MATCH_ANY("////", "/**/", "/***")
			? !__is_attr
			: MATCH_ANY("///", "//!", "/**", "/*!")
			? __is_attr
			: !__is_attr,
		`Attempted to read ${__is_line ? "Line" : "Block"} ${__is_attr ? "DocCommentAttribute" : "Comment"}`
	);

	if (__is_line) {
		nStep(__is_attr ? 3 : 2);
		step_until_ln();
	} else {
		let depth = 1;
		let lpos = GET_POSITION();
		nStep(__is_attr ? 3 : 2);
		while (true) {
			step_until_match(CharCode["/"]);
			// /*...*/••••••••••••••••
			// ••••/*/*•••••••••••••••
			// •/*.../*•••••••••••••••
			//       ^- You are here
			if (lpos !== GET_POSITION() - 2 && peek_match(-1, CharCode["*"])) {
				safe_step_over(CharCode["/"]);
				if (--depth === 0) break;
			} else {
				safe_step_over(CharCode["/"]);
				if (match(CharCode["*"])) {
					safe_step_over(CharCode["*"]);
					lpos = GET_POSITION() - 2;
					depth++;
				}
			}
		}
	}
}
