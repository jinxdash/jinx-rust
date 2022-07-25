import { CharCode } from "../../utils/CharCode";
import { is_number } from "../../utils/enum";
import { assert, exit } from "../error";
import * as Nodes from "../nodes";
import {
	CCPATH_read,
	currChar,
	GET_SOURCEFILEPATH,
	match,
	MATCH_ANY,
	peek_keyword,
	read_cached_keyword,
	safe_skip_1_read_2,
	step,
} from "../state";
import { FileLoc, FileLoc_FromChildElseReadAhead, FileLoc_MissingNode } from "../state/constructor";
import { Keyword } from "../state/scanner";

class MissingNode extends FileLoc_MissingNode(Nodes.MissingNode) {
	read() {}
}

export function read_maybe_missing<T extends Nodes.Node>(fn: () => T | undefined): T | Nodes.MissingNode {
	return fn() ?? MissingNode.read();
}

let __nocheck = false;
function is_valid_identifier(kw: Keyword) {
	if (__nocheck) {
		__nocheck = false;
		return true;
	}
	switch (kw) {
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
			return true;
		case Keyword.try:
			return GET_SOURCEFILEPATH().includes("serde") || MATCH_ANY("try!(", "try(", "try;", "try::");
		case Keyword.fn:
			return MATCH_ANY("fn(", "fn::");
		default:
			return false;
	}
}

export function read_identifier_token() {
	__nocheck = true;
	return Identifier.read();
}

export class Identifier extends FileLoc(Nodes.Identifier) {
	read() {
		__DEV__: assert(is_valid_identifier(peek_keyword()), "Attempted to read Identifier");
		switch (peek_keyword()) {
			// #invalid_identifier_keywords
			case Keyword.NotAWord:
			case Keyword.Underscore:
			case Keyword.StringLiteral:
				exit.expected("Identifier");
		}
		read_cached_keyword();
	}
}

export class Index extends FileLoc(Nodes.Index) {
	read() {
		// foo.x••••••••••••••••
		//     ^- You are here (x:0-9)
		__DEV__: assert(is_number(currChar()));
		do step();
		while (is_number(currChar()));
	}
}
export function read_Index() {
	assert.expect(is_number(currChar()));
	return Index.read();
}

export class ItemPath extends FileLoc_FromChildElseReadAhead(Nodes.ItemPath) {
	read(namespace: this["namespace"]) {
		// namespace::••••••••••••••••
		//            ^- You are here
		this.namespace = namespace;
		this.segment = Identifier.read();
	}
}

export function read_Identifier_or_Index() {
	return is_number(currChar()) ? Index.read() : Identifier.read();
}

export function read_ccPath_or_Identifier<T extends Parameters<typeof CCPATH_read>[0]>(PATH_NODE: T) {
	return match(CharCode[":"]) ? CCPATH_read(PATH_NODE) : Identifier.read();
}
export function read_Identifier_or_ItemPath_unbound(): Nodes.IdentifierOrItemPath {
	let lhs = read_ccPath_or_Identifier(ItemPath) as Nodes.Identifier | Nodes.ItemPath;
	while (match(CharCode[":"])) {
		safe_skip_1_read_2(CharCode[":"], CharCode[":"]);
		lhs = ItemPath.read(lhs);
	}
	return lhs;
}
