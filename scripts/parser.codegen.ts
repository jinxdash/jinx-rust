import { TSESTree } from "@typescript-eslint/typescript-estree";
import {
	array_filter_map_join,
	assert,
	color,
	each_match,
	every,
	exit,
	itfn,
	joinln,
	join_lines,
	last_of,
	Map_add,
	vIterable,
} from "../src/utils/common";
import { overrideDefaultError } from "../src/utils/debug";
import { format_headers, for_each_ts_file, join_wrap, update_file_section } from "./utils/common";
import { read_file, update_file } from "./utils/fs";
import {
	edit_nodes,
	FormattedNode,
	getParsedNodesFile,
	ParsedNodesFile,
	replace_node,
	slice_node,
	TSNodeType,
	tsParse,
	ts_edit_code,
} from "./utils/typescript";

overrideDefaultError();

const on_match_RawIdentifier = `
	if (is_XID_Start(uc_next())) {
		read_XID_CONTINUE();
		return ${Keyword("RawIdentifier")};
	} 
	edgecase_stepback();
	if (will_actually_read_rString()) { 
		return ${Keyword("StringLiteral")}; 
	}
	return ${Keyword("NotAWord")};
	`;

const KEYWORD_SCANNER_SPEC = [
	/** NotAWord, */
	/** NotKeyword, */

	// # Not keywords
	{ name: "Underscore", match: "_" },
	{ name: "RawIdentifier", match: "r#", eval: on_match_RawIdentifier },
	{ name: "macro_rules!", match: "macro_rules", eval: `if (uc_next_match(${CharCode(33)})) { return ${Keyword("macro_rules!")}; }` },

	{ name: "StringLiteral", match: "b", eval: ` if (will_actually_read_bString()) { return ${Keyword("StringLiteral")}; }` },
	{ name: "StringLiteral", match: "br", eval: `if (will_actually_read_rString()) { return ${Keyword("StringLiteral")}; }` },
	{ name: "StringLiteral", match: "r", eval: ` if (will_actually_read_rString()) { return ${Keyword("StringLiteral")}; }` },

	"auto",

	// # Actual keywords
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
	"async", // (2018)
	"extern",
	"move",
	"pub",

	"as",
	"in",
	"dyn", // (2015)
	"ref",
	"for",
	"mut",
	"where",

	"await", // (2018)
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

	// # Feature gated

	"box",
	"try", // (2018)
	"yield",

	// # Reserved unused

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
] as readonly (string | UniformKeyword)[];

const KEYWORDS = KEYWORD_SCANNER_SPEC.map((kw) => (typeof kw === "string" ? { name: kw, match: kw } : kw));
const KEYWORD_NAMES = ["NotAWord", "NotKeyword", ...new Set(KEYWORDS.map((k) => k.name))];

type UniformKeyword = { name: string; match: string; eval?: string };
const matchSymbol = Symbol();
const endsSymbol = Symbol();
const depthSymbol = Symbol();

type MatchToken = UniformKeyword;
type Tree = { [charcode: number]: Branch; [depthSymbol]: number };
type Branch = { [charcode: number]: Branch; [depthSymbol]: number; [matchSymbol]?: MatchToken; [endsSymbol]: MatchToken[] };

function Some<T>(v: T | undefined): T {
	assert(v !== undefined);
	return v;
}

function withExistCheck<T extends vIterable>(v: T, check: itfn<T, boolean>) {
	assert.every(v, check, "Some identifiers were renamed");
	return v;
}

const NodesSpec: ParsedNodesFile = getParsedNodesFile();
const NodeType: Map<string, number> = new Map(NodesSpec.Nodes.map((node) => [node.name, node.nodeType]));
const NodeNames: string[] = NodesSpec.Nodes.map((node) => node.name);

(function main() {
	for (const node of NodesSpec.Nodes) {
		for (const [Name, attr] of [
			["AttributeTarget", "attributes"],
			["PubTarget", "pub"],
			["ExternTarget", "extern"],
			["ConstModifier", "const"],
			["AsyncModifier", "async"],
			["MoveModifier", "move"],
			["UnsafeModifier", "unsafe"],
		]) {
			if (node.implements.includes(Name)) {
				if (!hasProperty(node, attr)) {
					log(`${color.magenta(node.name)} implements ${Name} but does not declare "${attr}"`);
				}
			}
			if (hasProperty(node, attr)) {
				if (!node.implements.includes(Name)) {
					log(`${color.magenta(node.name)} declares "${attr}" but does not implement ${Name}`);
				}
			}
			function log(msg: string) {
				console.log(color.cyan(msg));
			}
			function hasProperty(Node: FormattedNode, name: string) {
				return Node.properties.some((Property) => Property.name === name);
			}
		}
	}

	edit_nodes((node) => ({
		extends: (superName) => `${superName}<${Some(NodeType.get(node.declaration.id.name))}>`,
	}));

	const generated: { [path: string]: () => Generator<string> } = {};

	generated["src/parser/nodes.ts"] = function* () {
		yield join_wrap({
			start: "export enum NodeType {\n",
			content: NodeNames,
			sep: ", ",
			end: "\n}",
		});
		yield join_wrap({
			start: "export type Node = ",
			content: NodeNames,
			sep: " | ",
			end: ";",
		});
		yield join_wrap({
			start: "export interface NTMap {\n",
			content: NodeNames.map((name) => `${NodeType.get(name)}: ${name};`),
			sep: " ",
			end: "\n}",
		});
	};

	generated["src/utils/ast/nodetype.ts"] = function* () {
		yield* NodesSpec.Nodes.map(
			({ name }) => `export function is_${name}(node: Node): node is ${name} {
				return nis(node, ${NodeType.get(name)});
			}`
		);

		const IgnoredTypes = withExistCheck(
			[
				"missing",
				"_PathBase",
				"_ExprPathSource",
				"_TypePathSource",

				"Node",
				"TokenNode",

				"ExpressionNodeXS",
				"ScrutineeExpression",
				"ConditionExpression",
				"ConditionExpressionXS",

				"MaybeConstNode",

				"MacroSeparator",
				"Segment",
				"AttrSegment",
				"MacroInvokeSegment",
				"MacroMatchSegment",
				"MacroTransformSegment",

				"TypeCallArgument",
				"ConstTypeParamDefault",
			],
			(typeName) => NodesSpec.Types.some((Type) => Type.name === typeName)
		);

		const ExpressionNamespaceTargetNoSelector = "ExpressionNamespaceTargetNoSelector";
		const TypeNamespaceTargetNoSelector = "TypeNamespaceTargetNoSelector";

		const TypesFix = withExistCheck(
			{
				[ExpressionNamespaceTargetNoSelector]: {
					Identifier: "true",
					ExpressionPath: `undefined === node.namespace || is_${ExpressionNamespaceTargetNoSelector}(node.namespace)`,
					ExpressionTypeCast: `is_${ExpressionNamespaceTargetNoSelector}(node.typeCallee)`,
				},
				[TypeNamespaceTargetNoSelector]: {
					Identifier: "true",
					TypePath: `undefined === node.namespace || is_${TypeNamespaceTargetNoSelector}(node.namespace)`,
					TypeCall: `is_${TypeNamespaceTargetNoSelector}(node.typeCallee)`,
					TypeFunction: `is_${TypeNamespaceTargetNoSelector}(node.callee)`,
				},
			},
			(obj, typeName) =>
				NodesSpec.Types.some(({ name, types }) => name === typeName && every(obj, (code, nodeName) => types.includes(nodeName)))
		);

		const AssertTypesFix = withExistCheck(
			{
				[ExpressionNamespaceTargetNoSelector]: {
					ExpressionPath: "ExpressionPath<Identifier>",
					ExpressionTypeCast: "ExpressionTypeCast<Identifier>",
				},
				[TypeNamespaceTargetNoSelector]: {
					TypePath: "TypePath<Identifier>",
					TypeCall: "TypeCall<Identifier>",
					TypeFunction: "TypeFunction<Identifier>",
				},
				TypeBound: {
					TypeParenthesized: "TypeParenthesized<TypeTraitBound>",
				},
				TypeNode: {
					TypeParenthesized: "TypeParenthesized<TypeNode>",
				},
				FunctionParameterNode: {
					TypeParenthesized: "TypeParenthesized<TypeNode>",
				},
				ExpressionNode: {
					OrExpression: "OrExpression<ExpressionNode, ExpressionNode>",
					AndExpression: "AndExpression<ExpressionNode, ExpressionNode>",
				},
			},
			(obj, typeName) =>
				NodesSpec.Types.some(
					(Type) => Type.name === typeName && every(obj, (code, subTypeName) => Type.types.includes(subTypeName))
				)
		);

		yield* NodesSpec.Types.filter((Type) => !IgnoredTypes.includes(Type.name)).map(
			({ name, types }) => `export function is_${name}(node: Node): node is ${name} {
				${join_wrap({
					start: `AssertTypesEq<${name}, `,
					content: types.map((typeName) =>
						name in AssertTypesFix && typeName in AssertTypesFix[name] ? AssertTypesFix[name][typeName] : typeName
					),
					sep: " | ",
					indent: "\t\t",
					end: ">();",
				})}
				__DEV__: assert.isNode(node);
				${create_switch_return("node", "false", function* () {
					yield* types.map((name) => [name, "true"]);
					if (name in TypesFix) yield* Object.entries(TypesFix[name]);
				})}
			}`
		);

		const stmt_expr_attributes = withExistCheck(
			{
				LetVariableDeclaration: "stmt_expr_attributes && node === parent.expression",
				CallExpression: "parent.arguments.includes(node as any)",

				StructLiteral: "parent.properties.includes(node as any)",
				ArrayLiteral: "parent.items.includes(node as any)",
				TupleLiteral: "parent.items.includes(node as any)",

				TuplePattern: "parent.items.includes(node as any)",
				StructPattern: "parent.properties.includes(node as any)",
			},
			(value, key) => NodeNames.includes(key)
		);

		yield `export function can_have_OuterAttributes(node: Node, parent: Node | undefined, stmt_expr_attributes: boolean): boolean {
			__DEV__: assert.isNode(node);
			if (undefined !== parent) {
				${create_switch_return("parent", null, () => Object.entries(stmt_expr_attributes))}
			}
			${create_switch_return("node", "false", function* () {
				for (const { name, properties } of NodesSpec.Nodes) {
					if (properties.some((Property) => Property.name === "attributes")) {
						yield [name, "true"];
					}
				}
			})}
		}`;
	};

	generated["src/utils/ast/iterator.ts"] = function* () {
		yield `function iterate_nodes(node: Node, fn: AstItFn): void {
		__DEV__: assert.isNode(node);
		if ("attributes" in node) _it_arr(fn, node.attributes, "attributes");
		${create_switch("node", "exit.never(node)", function* () {
			const Nodes_Properties_Fix = {
				FunctionDeclaration: {
					parameters: joinln(
						`if (undefined !== node.parameters.self) fn(node.parameters.self, "parameters", "self");`,
						`_it_arr(fn, node.parameters, "parameters");`
					),
				},
			};

			for (const Node of NodesSpec.Nodes) {
				const code = array_filter_map_join(
					/* Node.name === "Program" ? Node.properties : */ Node.properties.sort((a, b) => +b.declare - +a.declare),
					(Property) =>
						Property.name !== "attributes" &&
						!Property.getter &&
						Property.types.length > 0 &&
						!Property.types.every((Type) => Type === "primitive" || Type === "undefined"),
					(Property) => {
						assert(!Property.types.includes("primitive"), "", Property);

						if (Node.name in Nodes_Properties_Fix && Property.name in Nodes_Properties_Fix[Node.name]) {
							return Nodes_Properties_Fix[Node.name][Property.name];
						}

						let str = "";

						const { name } = Property;
						let value = `node.${name}`;

						if (Property.optional) {
							// value += "!";
							str += `if ("${name}" in node) `;
						}

						if (Property.types.includes("undefined")) {
							str += `if (undefined !== ${value}) `;
						}

						if (Property.types.includes("Array")) {
							return Property.types.includes("Node")
								? str + `if (is_LocArray(${value})) _it_arr(fn, ${value}, "${name}");` + `else fn(${value}, "${name}");`
								: str + `_it_arr(fn, ${value}, "${name}");`;
						}

						return str + `fn(${value}, "${name}");`;
					},
					"\n"
				);

				yield [Node.name, code + "break;"];
			}
		})}
	}`;
	};

	generated["src/parser/state/scanner.ts"] = function* () {
		const ROOT_TREE = (function () {
			const tree: Tree = { [depthSymbol]: 0 };
			for (const kw of KEYWORDS) {
				let current = tree;
				for (let i = 0; i < kw.match.length; i++) {
					current = current[kw.match.charCodeAt(i)] ??= { [endsSymbol]: [], [depthSymbol]: i };
					current[endsSymbol].push(kw);
				}
				assert(!(matchSymbol in current), `Duplicate keyword ${kw.name} (${kw.match})`);
				current[matchSymbol] = kw;
			}
			return tree;
		})();

		yield `
			export const enum Keyword {
				${KEYWORD_NAMES.map((name) => kwToEnumName(name))}
			}
	
			export const ToKeyword = ${JSON.stringify(KEYWORD_NAMES)} as const;
	
			export function kwTree(): Keyword {
				switch (currChar()) {
					${getCharCodes(ROOT_TREE)
						.map(
							(char) =>
								`case ${CharCode(char)}:` +
								print_branch(
									ROOT_TREE[char],
									(char) => `uc_next_match(${CharCode(char)})`,
									() => `uc_next()`,
									(kw: UniformKeyword, failed: boolean) =>
										"eval" in kw
											? kw.eval
											: kw.name === "NotKeyword"
											? kw.match.length === 1 && !failed
												? `read_XID_CONTINUE(); return ${Keyword(kw.name)};`
												: "break;"
											: failed
											? `return kw_resolve_failed(${Keyword(kw.name)});`
											: `return kw_resolve(${Keyword(kw.name)});`
								) +
								maybe_fallback(ROOT_TREE[char], () => "break;")
						)
						.join("")}
	
					${(function () {
						const extra = new Set<number>();
						const kwStarts = new Set(KEYWORDS.map((kw) => kw.match[0]));
						const add = (match: string) => !kwStarts.has(match[0]) && extra.add(match.charCodeAt(0));
						add("_");
						for (const l of "abcdefghijklmnopqrstuvwxyz") add(l), add(l.toUpperCase());
						return [...extra]
							.sort((a, b) => (a > b ? 1 : -1))
							.map((charcode) => `case ${CharCode(charcode)}:`)
							.join("");
					})()}
						read_XID_CONTINUE();
						return ${Keyword("NotKeyword")};
	
					default:
						if (128 < currChar() && is_UNICODE_XID_Start(currChar())) {
							read_XID_CONTINUE();
							return ${Keyword("NotKeyword")};
						}
						return ${Keyword("NotAWord")};
				}
	
				if (is_XID_Continue(currChar())) {
					read_XID_CONTINUE();
				}
	
				return ${Keyword("NotKeyword")};
			}`;
	};

	for (const filepath in generated) {
		update_file_section(
			filepath,
			"\n// <generated>\n",
			join_lines(generated[filepath]), //
			"\n// </generated>\n",
			{ prettier: true }
		);
	}

	for_each_ts_file("src", (file) => {
		format_headers(file.path);
		for (const m of each_match(/class\s*([^ ]+)(?:\s*extends\s*)FileLoc[^\(]*\(([^,\)]+)/g, file.content)) {
			const classname = m[1];
			const superclass = last_of(m[2].split("."));

			if (classname !== superclass) {
				console.log(color.red(`${file.cmd}: Misnamed class '${classname}'. Expected '${superclass}'.`));
			}
		}
	});

	update_file("src/parser/state/scanner.ts", (code) =>
		ts_edit_code(code, function* (body) {
			const RHS = getEnum(body, "RHS");
			const TK = new Map(
				getEnum(tsParse(read_file("src/parser/nodes.ts")).ast.body, "TK").members.map((member, i) => [
					getEnumId(member),
					`/* TK['${getEnumId(member)}'] */ ${i}`,
				])
			);
			let disc = 100;
			for (const member of RHS.members) {
				// let content = "" + (TK.get(getEnumId(member)) ?? disc++);
				yield replace_node(member, `${slice_node(code, member.id)} = ${TK.get(getEnumId(member)) ?? disc++}`);
			}
		})
	);
	// write_localTempFile("scripts/scanner.temp.json", tsParse(read_file("src/parser/state/scanner.ts")));
})();

// const literal_suffix = ["u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32", "f64"];
function getEnumId({ id }: TSESTree.TSEnumMember) {
	return id.type === TSNodeType.Identifier ? id.name : "" + (id as TSESTree.Literal).value;
}
function getEnum(stmts: TSESTree.ProgramStatement[], name: string) {
	for (const x of stmts) {
		if (
			x.type === TSNodeType.ExportNamedDeclaration &&
			x.declaration?.type === TSNodeType.TSEnumDeclaration &&
			x.declaration.id.name === name
		) {
			return x.declaration;
		}
	}
	exit(`Could not find enum ${name}`);
}

function EnumGet(name: string, property: string | number) {
	return (
		name +
		(typeof property === "number"
			? `[${property === 34 ? `'"'` : `"${String.fromCharCode(property)}"`}]`
			: !/^["'0-9]/.test(property)
			? `.${property}`
			: `[${property}]`)
	);
}

function kwToEnumName(token: string) {
	switch (token) {
		case "macro_rules!":
			return `"${token}"`;
		default:
			return token;
	}
}

function Keyword(keyword: string) {
	return EnumGet("Keyword", kwToEnumName(keyword));
}
function CharCode(charcode: number) {
	return EnumGet("CharCode", charcode);
}

function print_branch(
	tree: Branch | Tree,
	match_next_char: (char: number, tree: Branch) => string,
	switch_next_char: (chars: number[], tree: Branch) => string,
	print_return: (match: MatchToken, failed: boolean) => string,
	format_branch: (res: string, branch: Branch) => string = (res) => res
) {
	function print_wrapped(branch: Branch) {
		return format_branch(print_each(branch), branch);
	}
	function print_each(tree: Branch) {
		const matched_item = matchSymbol in tree ? tree[matchSymbol]! : "";
		const keys = getCharCodes(tree);
		if (keys.length === 0) {
			assert(!!matched_item);
			return print_return(matched_item, false);
		}
		if (keys.length !== 1) {
			return `switch (${switch_next_char(keys, tree)}) {
					${keys.map((key) => `case ${CharCode(key)}:` + print_wrapped(tree[key]) + maybe_fallback(tree[key], () => "break;")).join("")}
					${matched_item && `default: ${print_return(matched_item, true)}`}
				}`;
		}
		const key = keys[0];
		if (matched_item) {
			return `if (${match_next_char(key, tree)}) { 
				${print_wrapped(tree[key])} 
			} else { 
				${print_return(matched_item, true)}; 
			}`;
		} else {
			return `if (${match_next_char(key, tree)} ${(function chain(SUB_BRANCH: Branch) {
				const SUB_KEYS = getCharCodes(SUB_BRANCH);
				return SUB_KEYS.length === 1 && !(matchSymbol in SUB_BRANCH)
					? `&& ${match_next_char(SUB_KEYS[0], SUB_BRANCH)} ${chain(SUB_BRANCH[SUB_KEYS[0]])}`
					: `) { ${print_wrapped(SUB_BRANCH)} }`;
			})(tree[key])}`;
		}
	}
	return print_wrapped(tree as Branch);
}
function getCharCodes(tree: Branch | Tree) {
	return Object.keys(tree)
		.map((v) => +v)
		.sort((a, b) => (a > b ? 1 : -1));
}

function maybe_fallback(branch: Branch, res: () => string) {
	function doesnt_need_fallback(branch: Branch) {
		const keys = Object.keys(branch);
		return matchSymbol in branch && (keys.length === 0 || keys.every((key) => doesnt_need_fallback(branch[key])));
	}
	if (!doesnt_need_fallback(branch)) return res();
	else return "";
}

function create_switch_return(target: string, defaultReturn: string | null, NameToReturnMap: () => Iterable<[string, string]>) {
	return create_switch(target, defaultReturn && `return ${defaultReturn};`, function* () {
		for (const [name, ret] of NameToReturnMap()) yield [name, `return ${ret};`];
	});
}

function create_switch(target: string, defaultRes: string | null, NameToReturnMap: () => Iterable<[string, string]>) {
	const names = new Set();
	const cases = new Map<string, string[]>();
	for (const [name, ret] of new Map(NameToReturnMap())) {
		Map_add(cases, ret, name);
		names.add(name);
	}
	if (names.size === 1) {
		const name = [...names][0];
		const success_value = [...cases.keys()][0];
		if (success_value === "return true;" && defaultRes === "return false;") {
			return `return is_${name}(${target});`;
		}
		if (success_value === "return false;" && defaultRes === "return true;") {
			return `return !is_${name}(${target});`;
		}
	}

	return `switch (${target}.nodeType) {
		${join_lines(function* () {
			const names_default_res = cases.get(defaultRes!) ?? [];
			cases.delete(defaultRes!);

			for (const [res, names] of cases) {
				for (const name of sort_names(names)) {
					yield `case ${get(name)}:`;
				}
				yield res;
			}
			if (defaultRes) {
				for (const name of sort_names(names_default_res)) {
					yield `case ${get(name)}:`;
				}
				yield `default: ${defaultRes}`;
			}
		})}
	}`;
	function sort_names(names: string[]) {
		return names.sort((a, b) => get(a) - get(b));
	}
	function get(name: string) {
		return NodeType.get(name) ?? 999;
	}
}
