import {
	check_ahead,
	currChar,
	edgecase_stepback,
	ES_ctx_exceptStructFormExpression,
	ES_ctx_insideScrutinee,
	kw_resolve,
	kw_resolve_failed,
	match,
	match_keyword,
	maybe_skip_1_read_2,
	maybe_step_over,
	peek,
	PEEK_MATCH,
	peek_match,
	read_XID_CONTINUE,
	rhs_resolve,
	step,
	uc_eat,
	uc_next,
	uc_next_match,
	will_match_charLiteral_not_lt,
} from ".";
import { CharCode } from "../../utils/CharCode";
import { is_XID_Continue, is_XID_Start } from "../../utils/enum";
import { is_UNICODE_XID_Start } from "../../utils/unicode";
import { assert, exit } from "../error";
import { PRCD, TK } from "../nodes";
import { skip_whitespace } from "./whitespace";

// <codegen> edits discriminants
export const enum RHS {
	None = /* TK['None'] */ 0,

	"::" = /* TK['::'] */ 40,

	"!" = /* TK['!'] */ 43,
	"(" = 100,

	"{" = 101,

	"." = /* TK['.'] */ 1,
	"?" = /* TK['?'] */ 42,
	"[" = 102,

	"as" = 103,

	".." = /* TK['..'] */ 34,
	"..=" = /* TK['..='] */ 36,
	"..." = /* TK['...'] */ 35,

	"&&" = /* TK['&&'] */ 2,
	"||" = /* TK['||'] */ 3,

	"=" = /* TK['='] */ 4,

	"==" = /* TK['=='] */ 15,
	"!=" = /* TK['!='] */ 16,
	">" = /* TK['>'] */ 17,
	">=" = /* TK['>='] */ 18,
	"<" = /* TK['<'] */ 19,
	"<=" = /* TK['<='] */ 20,
	"<<" = /* TK['<<'] */ 13,
	">>" = /* TK['>>'] */ 14,

	"*" = /* TK['*'] */ 7,
	"/" = /* TK['/'] */ 8,
	"%" = /* TK['%'] */ 9,
	"+" = /* TK['+'] */ 5,
	"-" = /* TK['-'] */ 6,
	"&" = /* TK['&'] */ 10,
	"|" = /* TK['|'] */ 11,
	"^" = /* TK['^'] */ 12,

	"+=" = /* TK['+='] */ 21,
	"-=" = /* TK['-='] */ 22,
	"*=" = /* TK['*='] */ 23,
	"/=" = /* TK['/='] */ 24,
	"%=" = /* TK['%='] */ 25,
	"&=" = /* TK['&='] */ 26,
	"|=" = /* TK['|='] */ 27,
	"^=" = /* TK['^='] */ 28,
	"<<=" = /* TK['<<='] */ 29,
	">>=" = /* TK['>>='] */ 30,
}

export function rhsTree(): RHS {
	switch (currChar()) {
		case CharCode["!"]:
			return peek_match(1, CharCode["="]) //
				? rhs_resolve(RHS["!="], PRCD["=="], 2)
				: RHS["!"];
		case CharCode["%"]:
			return peek_match(1, CharCode["="]) //
				? rhs_resolve(RHS["%="], PRCD["=="], 2)
				: rhs_resolve(RHS["%"], PRCD["*/%"], 1);
		case CharCode["&"]:
			switch (peek(1)) {
				case CharCode["&"]:
					return rhs_resolve(
						RHS["&&"],
						ES_ctx_insideScrutinee() //
							? PRCD["Scrutinee &&"]
							: PRCD["&&"],
						2
					);
				case CharCode["="]:
					return rhs_resolve(RHS["&="], PRCD["="], 2);
				default:
					return rhs_resolve(RHS["&"], PRCD["&"], 1);
			}
		case CharCode["("]:
			return RHS["("];
		case CharCode["*"]:
			return peek_match(1, CharCode["="]) //
				? rhs_resolve(RHS["*="], PRCD["="], 2)
				: rhs_resolve(RHS["*"], PRCD["*/%"], 1);
		case CharCode["+"]:
			return peek_match(1, CharCode["="]) //
				? rhs_resolve(RHS["+="], PRCD["="], 2)
				: rhs_resolve(RHS["+"], PRCD["+-"], 1);
		case CharCode["-"]:
			return peek_match(1, CharCode["="]) //
				? rhs_resolve(RHS["-="], PRCD["="], 2)
				: rhs_resolve(RHS["-"], PRCD["+-"], 1);
		case CharCode["."]:
			if (peek_match(1, CharCode["."])) {
				switch (peek(2)) {
					case CharCode["="]:
						return rhs_resolve(RHS["..="], PRCD[".."], 3);
					case CharCode["."]:
						return rhs_resolve(RHS["..."], PRCD[".."], 3);
					default:
						return rhs_resolve(RHS[".."], PRCD[".."], 2);
				}
			} else {
				return RHS["."];
			}
		case CharCode["/"]:
			return peek_match(1, CharCode["="]) //
				? rhs_resolve(RHS["/="], PRCD["="], 2)
				: rhs_resolve(RHS["/"], PRCD["*/%"], 1);
		case CharCode[":"]:
			return maybe_skip_1_read_2(CharCode[":"]) //
				? RHS["::"]
				: RHS.None;
		case CharCode["<"]:
			switch (peek(1)) {
				case CharCode["<"]:
					return peek_match(2, CharCode["="]) //
						? rhs_resolve(RHS["<<="], PRCD["="], 3)
						: rhs_resolve(RHS["<<"], PRCD[">>"], 2);
				case CharCode["="]:
					return rhs_resolve(RHS["<="], PRCD["=="], 2);
				default:
					return rhs_resolve(RHS["<"], PRCD["=="], 1);
			}
		case CharCode["="]:
			switch (peek(1)) {
				case CharCode["="]:
					return rhs_resolve(RHS["=="], PRCD["=="], 2);
				case CharCode[">"]:
					// match ... { ... if expression => ... }••••••••
					//                               ^ You are here
					return RHS.None;
				default:
					return rhs_resolve(RHS["="], PRCD["="], 1);
			}
		case CharCode[">"]:
			switch (peek(1)) {
				case CharCode["="]:
					return rhs_resolve(RHS[">="], PRCD["=="], 2);
				case CharCode[">"]:
					return peek_match(2, CharCode["="]) //
						? rhs_resolve(RHS[">>="], PRCD["="], 3)
						: rhs_resolve(RHS[">>"], PRCD[">>"], 2);
				default:
					return rhs_resolve(RHS[">"], PRCD["=="], 1);
			}
		case CharCode["?"]:
			return RHS["?"];
		case CharCode["["]:
			return RHS["["];
		case CharCode["^"]:
			return peek_match(1, CharCode["="]) //
				? rhs_resolve(RHS["^="], PRCD["="], 2)
				: rhs_resolve(RHS["^"], PRCD["^"], 1);
		case CharCode["a"]:
			return match_keyword(Keyword.as) //
				? rhs_resolve(RHS["as"], PRCD["as"], 2)
				: RHS.None;
		case CharCode["{"]:
			return !ES_ctx_exceptStructFormExpression() //
				? RHS["{"]
				: RHS.None;
		case CharCode["|"]:
			switch (peek(1)) {
				case CharCode["="]:
					return rhs_resolve(RHS["|="], PRCD["="], 2);
				case CharCode["|"]:
					return rhs_resolve(
						RHS["||"],
						ES_ctx_insideScrutinee() //
							? PRCD["Scrutinee ||"]
							: PRCD["||"],
						2
					);
				default:
					return rhs_resolve(RHS["|"], PRCD["|"], 1);
			}
		default:
			return RHS.None;
	}
}

function will_actually_read_rString() {
	__DEV__: assert(PEEK_MATCH(-1, CharCode["r"]));
	// br•••••••••••••••••
	// •r•••••••••••••••••
	//   ^- You are here
	switch (currChar()) {
		case CharCode["#"]:
			while (true)
				switch (uc_next()) {
					case CharCode["#"]:
						continue;
					case CharCode['"']:
						return true; // r#" | br#"
					default:
						do edgecase_stepback();
						while (match(CharCode["#"]));
						return false;
				}
		case CharCode['"']:
			return true; // r" | br"
		default:
			return false; // bread
	}
}

function will_actually_read_bString() {
	__DEV__: assert(PEEK_MATCH(-1, CharCode["b"]));
	// b"••••••••••••••••
	// b'••••••••••••••••
	//  ^- You are here
	switch (currChar()) {
		case CharCode['"']:
			return true; // b"
		case CharCode["'"]:
			return will_match_charLiteral_not_lt(); // b'o
		default:
			return false; // boop
	}
}

function will_actually_read_macro_rules() {
	// macro_rules••••••••••••••••
	//           ^- You are here

	return uc_next_match(CharCode["!"]) || check_ahead(() => (skip_whitespace(), match(CharCode["!"])));
}

// #valid_identifier_keywords
// #invalid_identifier_keywords
// #literal_keywords

// rustc_span/src/symbol.rs
// <generated>

export const enum Keyword {
	NotAWord,
	NotKeyword,
	Underscore,
	RawIdentifier,
	"macro_rules!",
	StringLiteral,
	auto,
	true,
	false,
	fn,
	mod,
	use,
	struct,
	trait,
	union,
	enum,
	impl,
	type,
	let,
	static,
	const,
	unsafe,
	async,
	extern,
	move,
	pub,
	as,
	in,
	dyn,
	ref,
	for,
	mut,
	where,
	await,
	return,
	break,
	continue,
	if,
	else,
	match,
	loop,
	while,
	super,
	self,
	Self,
	crate,
	box,
	try,
	yield,
	abstract,
	become,
	do,
	final,
	macro,
	override,
	priv,
	typeof,
	unsized,
	virtual,
}

export const ToKeyword = [
	"NotAWord",
	"NotKeyword",
	"Underscore",
	"RawIdentifier",
	"macro_rules!",
	"StringLiteral",
	"auto",
	"true",
	"false",
	"fn",
	"mod",
	"use",
	"struct",
	"trait",
	"union",
	"enum",
	"impl",
	"type",
	"let",
	"static",
	"const",
	"unsafe",
	"async",
	"extern",
	"move",
	"pub",
	"as",
	"in",
	"dyn",
	"ref",
	"for",
	"mut",
	"where",
	"await",
	"return",
	"break",
	"continue",
	"if",
	"else",
	"match",
	"loop",
	"while",
	"super",
	"self",
	"Self",
	"crate",
	"box",
	"try",
	"yield",
	"abstract",
	"become",
	"do",
	"final",
	"macro",
	"override",
	"priv",
	"typeof",
	"unsized",
	"virtual",
] as const;

export function kwTree(): Keyword {
	switch (currChar()) {
		case CharCode["S"]:
			if (uc_next_match(CharCode["e"]) && uc_next_match(CharCode["l"]) && uc_next_match(CharCode["f"])) {
				return kw_resolve(Keyword.Self);
			}
			break;
		case CharCode["_"]:
			return kw_resolve(Keyword.Underscore);
		case CharCode["a"]:
			switch (uc_next()) {
				case CharCode["b"]:
					if (
						uc_next_match(CharCode["s"]) &&
						uc_next_match(CharCode["t"]) &&
						uc_next_match(CharCode["r"]) &&
						uc_next_match(CharCode["a"]) &&
						uc_next_match(CharCode["c"]) &&
						uc_next_match(CharCode["t"])
					) {
						return kw_resolve(Keyword.abstract);
					}
					break;
				case CharCode["s"]:
					if (uc_next_match(CharCode["y"])) {
						if (uc_next_match(CharCode["n"]) && uc_next_match(CharCode["c"])) {
							return kw_resolve(Keyword.async);
						}
					} else {
						return kw_resolve_failed(Keyword.as);
					}
					break;
				case CharCode["u"]:
					if (uc_next_match(CharCode["t"]) && uc_next_match(CharCode["o"])) {
						return kw_resolve(Keyword.auto);
					}
					break;
				case CharCode["w"]:
					if (uc_next_match(CharCode["a"]) && uc_next_match(CharCode["i"]) && uc_next_match(CharCode["t"])) {
						return kw_resolve(Keyword.await);
					}
					break;
			}
			break;
		case CharCode["b"]:
			switch (uc_next()) {
				case CharCode["e"]:
					if (
						uc_next_match(CharCode["c"]) &&
						uc_next_match(CharCode["o"]) &&
						uc_next_match(CharCode["m"]) &&
						uc_next_match(CharCode["e"])
					) {
						return kw_resolve(Keyword.become);
					}
					break;
				case CharCode["o"]:
					if (uc_next_match(CharCode["x"])) {
						return kw_resolve(Keyword.box);
					}
					break;
				case CharCode["r"]:
					if (uc_next_match(CharCode["e"])) {
						if (uc_next_match(CharCode["a"]) && uc_next_match(CharCode["k"])) {
							return kw_resolve(Keyword.break);
						}
					} else {
						if (will_actually_read_rString()) {
							return Keyword.StringLiteral;
						}
					}
					break;
				default:
					if (will_actually_read_bString()) {
						return Keyword.StringLiteral;
					}
			}
			break;
		case CharCode["c"]:
			switch (uc_next()) {
				case CharCode["o"]:
					if (uc_next_match(CharCode["n"])) {
						switch (uc_next()) {
							case CharCode["s"]:
								if (uc_next_match(CharCode["t"])) {
									return kw_resolve(Keyword.const);
								}
								break;
							case CharCode["t"]:
								if (
									uc_next_match(CharCode["i"]) &&
									uc_next_match(CharCode["n"]) &&
									uc_next_match(CharCode["u"]) &&
									uc_next_match(CharCode["e"])
								) {
									return kw_resolve(Keyword.continue);
								}
								break;
						}
					}
					break;
				case CharCode["r"]:
					if (uc_next_match(CharCode["a"]) && uc_next_match(CharCode["t"]) && uc_next_match(CharCode["e"])) {
						return kw_resolve(Keyword.crate);
					}
					break;
			}
			break;
		case CharCode["d"]:
			switch (uc_next()) {
				case CharCode["o"]:
					return kw_resolve(Keyword.do);
				case CharCode["y"]:
					if (uc_next_match(CharCode["n"])) {
						return kw_resolve(Keyword.dyn);
					}
					break;
			}
			break;
		case CharCode["e"]:
			switch (uc_next()) {
				case CharCode["l"]:
					if (uc_next_match(CharCode["s"]) && uc_next_match(CharCode["e"])) {
						return kw_resolve(Keyword.else);
					}
					break;
				case CharCode["n"]:
					if (uc_next_match(CharCode["u"]) && uc_next_match(CharCode["m"])) {
						return kw_resolve(Keyword.enum);
					}
					break;
				case CharCode["x"]:
					if (
						uc_next_match(CharCode["t"]) &&
						uc_next_match(CharCode["e"]) &&
						uc_next_match(CharCode["r"]) &&
						uc_next_match(CharCode["n"])
					) {
						return kw_resolve(Keyword.extern);
					}
					break;
			}
			break;
		case CharCode["f"]:
			switch (uc_next()) {
				case CharCode["a"]:
					if (uc_next_match(CharCode["l"]) && uc_next_match(CharCode["s"]) && uc_next_match(CharCode["e"])) {
						return kw_resolve(Keyword.false);
					}
					break;
				case CharCode["i"]:
					if (uc_next_match(CharCode["n"]) && uc_next_match(CharCode["a"]) && uc_next_match(CharCode["l"])) {
						return kw_resolve(Keyword.final);
					}
					break;
				case CharCode["n"]:
					return kw_resolve(Keyword.fn);
				case CharCode["o"]:
					if (uc_next_match(CharCode["r"])) {
						return kw_resolve(Keyword.for);
					}
					break;
			}
			break;
		case CharCode["i"]:
			switch (uc_next()) {
				case CharCode["f"]:
					return kw_resolve(Keyword.if);
				case CharCode["m"]:
					if (uc_next_match(CharCode["p"]) && uc_next_match(CharCode["l"])) {
						return kw_resolve(Keyword.impl);
					}
					break;
				case CharCode["n"]:
					return kw_resolve(Keyword.in);
			}
			break;
		case CharCode["l"]:
			switch (uc_next()) {
				case CharCode["e"]:
					if (uc_next_match(CharCode["t"])) {
						return kw_resolve(Keyword.let);
					}
					break;
				case CharCode["o"]:
					if (uc_next_match(CharCode["o"]) && uc_next_match(CharCode["p"])) {
						return kw_resolve(Keyword.loop);
					}
					break;
			}
			break;
		case CharCode["m"]:
			switch (uc_next()) {
				case CharCode["a"]:
					switch (uc_next()) {
						case CharCode["c"]:
							if (uc_next_match(CharCode["r"]) && uc_next_match(CharCode["o"])) {
								if (uc_next_match(CharCode["_"])) {
									if (
										uc_next_match(CharCode["r"]) &&
										uc_next_match(CharCode["u"]) &&
										uc_next_match(CharCode["l"]) &&
										uc_next_match(CharCode["e"]) &&
										uc_next_match(CharCode["s"])
									) {
										if (will_actually_read_macro_rules()) {
											return Keyword["macro_rules!"];
										}
									}
								} else {
									return kw_resolve_failed(Keyword.macro);
								}
							}
							break;
						case CharCode["t"]:
							if (uc_next_match(CharCode["c"]) && uc_next_match(CharCode["h"])) {
								return kw_resolve(Keyword.match);
							}
							break;
					}
					break;
				case CharCode["o"]:
					switch (uc_next()) {
						case CharCode["d"]:
							return kw_resolve(Keyword.mod);
						case CharCode["v"]:
							if (uc_next_match(CharCode["e"])) {
								return kw_resolve(Keyword.move);
							}
							break;
					}
					break;
				case CharCode["u"]:
					if (uc_next_match(CharCode["t"])) {
						return kw_resolve(Keyword.mut);
					}
					break;
			}
			break;
		case CharCode["o"]:
			if (
				uc_next_match(CharCode["v"]) &&
				uc_next_match(CharCode["e"]) &&
				uc_next_match(CharCode["r"]) &&
				uc_next_match(CharCode["r"]) &&
				uc_next_match(CharCode["i"]) &&
				uc_next_match(CharCode["d"]) &&
				uc_next_match(CharCode["e"])
			) {
				return kw_resolve(Keyword.override);
			}
			break;
		case CharCode["p"]:
			switch (uc_next()) {
				case CharCode["r"]:
					if (uc_next_match(CharCode["i"]) && uc_next_match(CharCode["v"])) {
						return kw_resolve(Keyword.priv);
					}
					break;
				case CharCode["u"]:
					if (uc_next_match(CharCode["b"])) {
						return kw_resolve(Keyword.pub);
					}
					break;
			}
			break;
		case CharCode["r"]:
			switch (uc_next()) {
				case CharCode["#"]:
					if (is_XID_Start(uc_next())) {
						read_XID_CONTINUE();
						return Keyword.RawIdentifier;
					}
					edgecase_stepback();
					if (will_actually_read_rString()) {
						return Keyword.StringLiteral;
					}
					return Keyword.NotAWord;
				case CharCode["e"]:
					switch (uc_next()) {
						case CharCode["f"]:
							return kw_resolve(Keyword.ref);
						case CharCode["t"]:
							if (uc_next_match(CharCode["u"]) && uc_next_match(CharCode["r"]) && uc_next_match(CharCode["n"])) {
								return kw_resolve(Keyword.return);
							}
							break;
					}
					break;
				default:
					if (will_actually_read_rString()) {
						return Keyword.StringLiteral;
					}
			}
			break;
		case CharCode["s"]:
			switch (uc_next()) {
				case CharCode["e"]:
					if (uc_next_match(CharCode["l"]) && uc_next_match(CharCode["f"])) {
						return kw_resolve(Keyword.self);
					}
					break;
				case CharCode["t"]:
					switch (uc_next()) {
						case CharCode["a"]:
							if (uc_next_match(CharCode["t"]) && uc_next_match(CharCode["i"]) && uc_next_match(CharCode["c"])) {
								return kw_resolve(Keyword.static);
							}
							break;
						case CharCode["r"]:
							if (uc_next_match(CharCode["u"]) && uc_next_match(CharCode["c"]) && uc_next_match(CharCode["t"])) {
								return kw_resolve(Keyword.struct);
							}
							break;
					}
					break;
				case CharCode["u"]:
					if (uc_next_match(CharCode["p"]) && uc_next_match(CharCode["e"]) && uc_next_match(CharCode["r"])) {
						return kw_resolve(Keyword.super);
					}
					break;
			}
			break;
		case CharCode["t"]:
			switch (uc_next()) {
				case CharCode["r"]:
					switch (uc_next()) {
						case CharCode["a"]:
							if (uc_next_match(CharCode["i"]) && uc_next_match(CharCode["t"])) {
								return kw_resolve(Keyword.trait);
							}
							break;
						case CharCode["u"]:
							if (uc_next_match(CharCode["e"])) {
								return kw_resolve(Keyword.true);
							}
							break;
						case CharCode["y"]:
							return kw_resolve(Keyword.try);
					}
					break;
				case CharCode["y"]:
					if (uc_next_match(CharCode["p"]) && uc_next_match(CharCode["e"])) {
						if (uc_next_match(CharCode["o"])) {
							if (uc_next_match(CharCode["f"])) {
								return kw_resolve(Keyword.typeof);
							}
						} else {
							return kw_resolve_failed(Keyword.type);
						}
					}
					break;
			}
			break;
		case CharCode["u"]:
			switch (uc_next()) {
				case CharCode["n"]:
					switch (uc_next()) {
						case CharCode["i"]:
							if (uc_next_match(CharCode["o"]) && uc_next_match(CharCode["n"])) {
								return kw_resolve(Keyword.union);
							}
							break;
						case CharCode["s"]:
							switch (uc_next()) {
								case CharCode["a"]:
									if (uc_next_match(CharCode["f"]) && uc_next_match(CharCode["e"])) {
										return kw_resolve(Keyword.unsafe);
									}
									break;
								case CharCode["i"]:
									if (uc_next_match(CharCode["z"]) && uc_next_match(CharCode["e"]) && uc_next_match(CharCode["d"])) {
										return kw_resolve(Keyword.unsized);
									}
									break;
							}
							break;
					}
					break;
				case CharCode["s"]:
					if (uc_next_match(CharCode["e"])) {
						return kw_resolve(Keyword.use);
					}
					break;
			}
			break;
		case CharCode["v"]:
			if (
				uc_next_match(CharCode["i"]) &&
				uc_next_match(CharCode["r"]) &&
				uc_next_match(CharCode["t"]) &&
				uc_next_match(CharCode["u"]) &&
				uc_next_match(CharCode["a"]) &&
				uc_next_match(CharCode["l"])
			) {
				return kw_resolve(Keyword.virtual);
			}
			break;
		case CharCode["w"]:
			if (uc_next_match(CharCode["h"])) {
				switch (uc_next()) {
					case CharCode["e"]:
						if (uc_next_match(CharCode["r"]) && uc_next_match(CharCode["e"])) {
							return kw_resolve(Keyword.where);
						}
						break;
					case CharCode["i"]:
						if (uc_next_match(CharCode["l"]) && uc_next_match(CharCode["e"])) {
							return kw_resolve(Keyword.while);
						}
						break;
				}
			}
			break;
		case CharCode["y"]:
			if (
				uc_next_match(CharCode["i"]) &&
				uc_next_match(CharCode["e"]) &&
				uc_next_match(CharCode["l"]) &&
				uc_next_match(CharCode["d"])
			) {
				return kw_resolve(Keyword.yield);
			}
			break;

		case CharCode["A"]:
		case CharCode["B"]:
		case CharCode["C"]:
		case CharCode["D"]:
		case CharCode["E"]:
		case CharCode["F"]:
		case CharCode["G"]:
		case CharCode["H"]:
		case CharCode["I"]:
		case CharCode["J"]:
		case CharCode["K"]:
		case CharCode["L"]:
		case CharCode["M"]:
		case CharCode["N"]:
		case CharCode["O"]:
		case CharCode["P"]:
		case CharCode["Q"]:
		case CharCode["R"]:
		case CharCode["T"]:
		case CharCode["U"]:
		case CharCode["V"]:
		case CharCode["W"]:
		case CharCode["X"]:
		case CharCode["Y"]:
		case CharCode["Z"]:
		case CharCode["g"]:
		case CharCode["h"]:
		case CharCode["j"]:
		case CharCode["k"]:
		case CharCode["n"]:
		case CharCode["q"]:
		case CharCode["x"]:
		case CharCode["z"]:
			read_XID_CONTINUE();
			return Keyword.NotKeyword;

		default:
			if (128 < currChar() && is_UNICODE_XID_Start(currChar())) {
				read_XID_CONTINUE();
				return Keyword.NotKeyword;
			}
			return Keyword.NotAWord;
	}

	if (is_XID_Continue(currChar())) {
		read_XID_CONTINUE();
	}

	return Keyword.NotKeyword;
}
// </generated>

// prettier-ignore
export function ptTree(): TK {
	switch (uc_eat()) {
		case CharCode["!"]: return maybe_step_over(CharCode["="]) ? TK["!="] : TK["!"];
		case CharCode["#"]: return TK["#"];
		case CharCode["$"]: return TK["$"];
		case CharCode["%"]: return maybe_step_over(CharCode["="]) ? TK["%="] : TK["%"];
		case CharCode["&"]:
			switch (currChar()) {
				case CharCode["&"]: return step(), TK["&&"];
				case CharCode["="]: return step(), TK["&="];
				default: return TK["&"];
			}
		case CharCode["*"]: return maybe_step_over(CharCode["="]) ? TK["*="] : TK["*"];
		case CharCode["+"]: return maybe_step_over(CharCode["="]) ? TK["+="] : TK["+"];
		case CharCode[","]: return TK[","];
		case CharCode["-"]:
			switch (currChar()) {
				case CharCode["="]: return step(), TK["-="];
				case CharCode[">"]: return step(), TK["->"];
				default: return TK["-"];
			}
		case CharCode["."]:
			if (maybe_step_over(CharCode["."])) {
				switch (currChar()) {
					case CharCode["."]: return step(), TK["..."];
					case CharCode["="]: return step(), TK["..="];
					default: return TK[".."];
				}
			} else {
				return TK["."];
			}
		case CharCode["/"]: return maybe_step_over(CharCode["="]) ? TK["/="] : TK["/"];
		case CharCode[":"]: return maybe_step_over(CharCode[":"]) ? TK["::"] : TK[":"];
		case CharCode[";"]: return TK[";"];
		case CharCode["<"]:
			switch (currChar()) {
				case CharCode["<"]: return step(), maybe_step_over(CharCode["="]) ? TK["<<="] : TK["<<"];
				case CharCode["="]: return step(), TK["<="];
				default: return TK["<"];
			}
		case CharCode["="]:
			switch (currChar()) {
				case CharCode["="]: return step(), TK["=="];
				case CharCode[">"]: return step(), TK["=>"];
				default: return TK["="];
			}
		case CharCode[">"]:
			switch (currChar()) {
				case CharCode["="]: return step(), TK[">="];
				case CharCode[">"]: return step(), maybe_step_over(CharCode["="]) ? TK[">>="] : TK[">>"];
				default: return TK[">"];
			}
		case CharCode["?"]: return TK["?"];
		case CharCode["@"]: return TK["@"];
		case CharCode["^"]: return maybe_step_over(CharCode["="]) ? TK["^="] : TK["^"];
		case CharCode["_"]: return TK["_"];
		case CharCode["|"]:
			switch (currChar()) {
				case CharCode["="]: return step(), TK["|="];
				case CharCode["|"]: return step(), TK["||"];
				default: return TK["|"];
			}
		case CharCode["~"]: return TK["~"];
		default: edgecase_stepback(); exit.unexpected();
	}
}
