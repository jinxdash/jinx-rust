import { Identifier } from ".";
import { CharCode } from "../../utils/CharCode";
import { is_bin, is_hex, is_number, is_oct, is_XID_Start } from "../../utils/enum";
import { assert, exit } from "../error";
import * as Nodes from "../nodes";
import { LiteralKind } from "../nodes";
import {
	ASSERT_WILL_MATCH_CHARLITERAL_NOT_LT,
	currChar,
	match,
	maybe_step_over,
	peek,
	peek_keyword,
	read,
	safe_skip_keyword,
	safe_step_eol,
	safe_step_over,
	safe_step_over_2,
	step,
	step_over,
	step_until_match,
} from "../state";
import { FileLoc } from "../state/constructor";
import { Keyword } from "../state/scanner";

const safe_step_over_number_us = (): void => {
	__DEV__: assert(is_number(currChar()) || match(CharCode["_"]));
	step();
};

export class Literal extends FileLoc(Nodes.Literal) {
	read() {
		this.kind = read_literal();
		switch (this.kind) {
			case LiteralKind.Binary:
			case LiteralKind.Hex:
			case LiteralKind.Octal:
			case LiteralKind.Integer:
			case LiteralKind.Float:
				if (is_XID_Start(currChar())) {
					this.suffix = Identifier.read() as any;
				}
		}
	}
}

function read_literal(): LiteralKind {
	switch (peek_keyword()) {
		// #literal_keywords
		case Keyword.true: // true
			safe_skip_keyword(Keyword.true);
			return LiteralKind.True;

		case Keyword.false: // false
			safe_skip_keyword(Keyword.false);
			return LiteralKind.False;

		case Keyword.StringLiteral:
			switch (currChar()) {
				case CharCode["b"]: {
					switch (peek(1)) {
						case CharCode["'"]: // b'a'
							read_bChar();
							return LiteralKind.bChar;

						case CharCode['"']: // b"foo"
							read_bString();
							return LiteralKind.bString;

						case CharCode["r"]: // br#"foo"#
							read_brString();
							return LiteralKind.brString;
					}
					exit.never();
				}
				case CharCode["r"]: // r#"foo"#
					read_rString();
					return LiteralKind.rString;
			}
			exit.never();

		case Keyword.NotAWord:
			switch (currChar()) {
				case CharCode['"']: // "foo"
					read_String();
					return LiteralKind.String;
				case CharCode["'"]: // 'a'
					read_Char();
					return LiteralKind.Char;
				case CharCode["0"]:
					switch (peek(1)) {
						case CharCode["b"]: // 0b1010
							safe_step_over_2("0b");
							while (is_bin(currChar()) || match(CharCode["_"])) step();
							return LiteralKind.Binary;
						case CharCode["o"]: // 0o777
							safe_step_over_2("0o");
							while (is_oct(currChar()) || match(CharCode["_"])) step();
							return LiteralKind.Octal;
						case CharCode["x"]: // 0xffff
							safe_step_over_2("0x");
							while (is_hex(currChar()) || match(CharCode["_"])) step();
							return LiteralKind.Hex;
						default:
							return read_number();
					}
				case CharCode["1"]:
				case CharCode["2"]:
				case CharCode["3"]:
				case CharCode["4"]:
				case CharCode["5"]:
				case CharCode["6"]:
				case CharCode["7"]:
				case CharCode["8"]:
				case CharCode["9"]:
					return read_number();
			}
	}
	exit.never();
}

// prettier-ignore
function read_number(): LiteralKind.Integer | LiteralKind.Float {
	__DEV__: assert(is_number(currChar()));
	step();
	while (true) {
		switch (currChar()) {
			case CharCode["_"]: // 42_0
			case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]: 
			case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]: // 420
				safe_step_over_number_us();
				break;
			case CharCode["e"]: case CharCode["E"]:  // 42e+1
				read_exponent();
				return LiteralKind.Float;
			case CharCode["."]: 
				switch (peek(1)) {
					case CharCode["."]:
						return LiteralKind.Integer; // 42..abc
					case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]: 
					case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]:
						read_decimals();
						return LiteralKind.Float; // 42.3
					default:
						if (is_XID_Start(peek(1))) {
							return LiteralKind.Integer; // 42.method()
						} else {
							safe_step_over(CharCode["."]);
							return LiteralKind.Float; // 42.
						}
				}
			default:
				return LiteralKind.Integer;
		}
	}
}

// prettier-ignore
function read_decimals() {
	safe_step_over(CharCode["."]);
	__DEV__: assert(is_number(currChar()));
	step();
	while (true) {
		switch (currChar()) {
			case CharCode["_"]:
			case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]: 
			case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]:
				safe_step_over_number_us();
				break;
			case CharCode["e"]: case CharCode["E"]:
				read_exponent();
				return;
			default:
				return;
		}
	}
}
// prettier-ignore
function read_exponent() {
	__DEV__: assert(match(CharCode["e"]) || match(CharCode["E"]));
	step();
	switch (currChar()) { case CharCode["+"]: case CharCode["-"]: step(); }
	loop: while (true) {
		switch (currChar()) {
			case CharCode["_"]:
			case CharCode["0"]: case CharCode["1"]: case CharCode["2"]: case CharCode["3"]: case CharCode["4"]: 
			case CharCode["5"]: case CharCode["6"]: case CharCode["7"]: case CharCode["8"]: case CharCode["9"]:
				safe_step_over_number_us();
				break;
			default:
				break loop;
		}
	}
}

function read_escaped(kind: LiteralKind) {
	safe_step_over(CharCode["\\"]);
	switch (currChar()) {
		case CharCode["\r"]:
		case CharCode["\n"]: // "STRING_CONTINUE \
			break;
		case CharCode['"']: //   \"
		case CharCode["'"]: //   \'
		case CharCode["0"]: //   \0
		case CharCode["\\"]: //  \\
		case CharCode["n"]: //   \n
		case CharCode["r"]: //   \r
		case CharCode["t"]: //   \t
			step();
			break;
		case CharCode["x"]: // \x
			switch (kind) {
				case LiteralKind.String:
				case LiteralKind.Char:
					safe_step_over(CharCode["x"]), read_OCT(), read_HEX();
					break;
				default:
					safe_step_over(CharCode["x"]), read_HEX(), read_HEX();
					break;
			}
			break;
		case CharCode["u"]: // \u{...}
			safe_step_over(CharCode["u"]);
			step_over(CharCode["{"]);
			while (is_hex(currChar()) || match(CharCode["_"])) step();
			step_over(CharCode["}"]);
			break;
		default:
			exit("Invalid escape");
			break;
	}
}

function read_String() {
	safe_step_over(CharCode['"']);
	loop: while (true) {
		switch (currChar()) {
			case CharCode["\n"]:
				safe_step_eol();
				break;
			case CharCode['"']:
				break loop;
			case CharCode["\\"]:
				read_escaped(LiteralKind.String);
				break;
			default:
				step();
				break;
		}
	}
	safe_step_over(CharCode['"']);
}

function read_rString() {
	safe_step_over(CharCode["r"]);
	if (maybe_step_over(CharCode["#"])) {
		let i = 0;
		let ht = 1;
		while (maybe_step_over(CharCode["#"])) ++ht;
		safe_step_over(CharCode['"']);
		while (true) {
			step_until_match(CharCode['"']), safe_step_over(CharCode['"']);
			while (maybe_step_over(CharCode["#"])) if (++i === ht) return;
			i = 0;
		}
	} else {
		safe_step_over(CharCode['"']);
		step_until_match(CharCode['"']);
		safe_step_over(CharCode['"']);
	}
}

function read_brString() {
	safe_step_over(CharCode["b"]);
	read_rString();
}

// prettier-ignore
function read_Char() {
	__DEV__: ASSERT_WILL_MATCH_CHARLITERAL_NOT_LT(0);
	safe_step_over(CharCode["'"]);
	switch (currChar()) {
		case CharCode["'"]: case CharCode["\n"]: case CharCode["\r"]: case CharCode["\t"]: exit.expected("character");
		case CharCode["\\"]: read_escaped(LiteralKind.Char); break;
		default:
			for (let i = 0; i < 8; i++) {
				step();
				if (match(CharCode["'"])) break;
			}
			break;
	}
	read(CharCode["'"]);
}

// prettier-ignore
function read_bChar() {
	__DEV__: ASSERT_WILL_MATCH_CHARLITERAL_NOT_LT(1);
	safe_step_over_2("b'");
	switch (currChar()) {
		case CharCode["'"]: case CharCode["\n"]: case CharCode["\r"]: case CharCode["\t"]: exit.expected("character");
		case CharCode["\\"]: read_escaped(LiteralKind.bChar); break;
		default: step(); break;
	}
	read(CharCode["'"]);
}

// prettier-ignore
function read_bString() {
	safe_step_over_2('b"');
	loop: while (true) {
		switch (currChar()) {
			case CharCode["\n"]: safe_step_eol(); break;
			case CharCode["\\"]: read_escaped(LiteralKind.bString); break;
			case CharCode['"']: break loop;
			default: step(); break;
		}
	}
	safe_step_over(CharCode['"']);
}

const read_OCT = () => {
	if (is_oct(currChar())) {
		step();
	} else {
		exit.expected("oct digit /0-7/");
	}
};
const read_HEX = () => {
	if (is_hex(currChar())) {
		step();
	} else {
		exit.expected("hex digit /0-9A-Fa-f/");
	}
};
