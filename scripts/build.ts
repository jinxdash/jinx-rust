import { AST_NODE_TYPES as NT, simpleTraverse, TSESTree } from "@typescript-eslint/typescript-estree";
import { build, Options as tsup_Options } from "tsup";
import { assert, color, ColorFn, exit, getLineStarts, pretty_timespan, time, time_since, urlAt } from "../src/utils/common";
import { overrideDefaultError } from "../src/utils/debug";
import { createStripPlugin } from "./utils/build";
import { JSONstringify, remove_unknown_files } from "./utils/common";
import { read_file, update_file, write_file } from "./utils/fs";
import {
	getIdentifierName,
	getParsedNodesFile,
	insert_at,
	ParsedNodesFile,
	prepend_to,
	remove_node,
	replace_node,
	TSEdit,
	tsParse,
	ts_edit_code,
} from "./utils/typescript";

const MAIN_OUT_DIR = "dist";
const MAIN_OUT_PATH = MAIN_OUT_DIR + "/index";

const UTILS_OUT_DIR = "utils";
const UTILS_OUT_PATH = UTILS_OUT_DIR + "/index";

const known_files = new Set<string>();
function addKnownFile(filepath: string) {
	known_files.add(filepath);
	return filepath;
}

overrideDefaultError();

function createBundle(entry: string, options: Partial<tsup_Options>) {
	return build({
		entry: [entry],
		format: ["esm", "cjs"],
		tsconfig: "tsconfig.build.json",
		plugins: [createStripPlugin({ labels: ["__DEV__"], functionCalls: ["devonly"] })],
		treeshake: {
			preset: "smallest",
			moduleSideEffects: false,
			propertyReadSideEffects: false,
			tryCatchDeoptimization: false,
			unknownGlobalSideEffects: false,
		},
		...options,
	});
}

function update_bundle_file(filepath: string, content: string | ((prev: string) => string)): Promise<boolean> {
	return update_file(addKnownFile(filepath), content, { sync: false, prettier: true });
}
function format_bundle_file(filepath: string): Promise<boolean> {
	return update_bundle_file(filepath, (v) => v);
}

await build_steps({
	codegen: () => import("./parser.codegen"),
	bundle: () => [
		createBundle("src/index.ts", { dts: { resolve: true } }),
		createBundle("src/utils/ast/index.ts", { outDir: UTILS_OUT_DIR }),
		createBundle("src/utils/ast/index.ts", { outDir: UTILS_OUT_DIR, dts: { only: true }, external: [/\/parser\/?/] }),
		write_file(
			addKnownFile(`${UTILS_OUT_DIR}/package.json`),
			JSONstringify({
				type: "module",
				main: "./index.cjs",
				module: "./index.js",
				types: "./index.d.ts",
			}),
			{ sync: false }
		),
	],
	transform: () => {
		const Spec = getParsedNodesFile();
		return [
			update_bundle_file(MAIN_OUT_PATH + ".js", (prev) => transform_parser(Spec, prev)),
			update_bundle_file(MAIN_OUT_PATH + ".cjs", (prev) => transform_parser(Spec, prev)),
			update_bundle_file(MAIN_OUT_PATH + ".d.ts", (prev) =>
				ts_edit_code(prev, function* (body) {
					for (const node of body) {
						if (node.type === NT.ClassDeclaration) {
							const target = Spec.Nodes.find((Node) => Node.name === node.id.name);
							const l = target?.comments.length;
							if (l) {
								yield prepend_to(
									node,
									(l === 1 ? "/** " : "/**\n * ") +
										target.comments.map((c) => c.replace(/\*\//g, "* /").trim()).join("  \n * ") +
										(l === 1 ? " */\n" : "\n */\n")
								);
							}
						}
					}
				}).replaceAll("declare const enum", "declare enum")
			),
			format_bundle_file(UTILS_OUT_PATH + ".js"),
			format_bundle_file(UTILS_OUT_PATH + ".cjs"),
			update_bundle_file(UTILS_OUT_PATH + ".d.ts", (prev) =>
				prev.replaceAll("../../parser/nodes", `../` + MAIN_OUT_PATH).replace(/\{\n/g, "{ ")
			),
		];
	},
	test: () => import("../tests/test.build"),
	copy: function* () {
		for (const [inpath, outname] of [
			[MAIN_OUT_PATH, "index"],
			[UTILS_OUT_PATH, "utils"],
		]) {
			for (const ext of ["js", "d.ts"]) {
				const esmCode = read_file(inpath + "." + ext);
				const target = "tests/bundle/" + outname + ".";

				yield write_file(target + ext, esmCode, { sync: false });

				if (ext === "js") {
					assert(/export {[\w\s,$]*};?\s*$/.test(esmCode));
					const cjs = read_file(inpath + "." + "cjs");
					yield write_file(
						target + "cjs",
						cjs.slice(0, cjs.indexOf("//")) +
							trim_same_start(
								cjs.slice(cjs.indexOf("//")),
								esmCode.slice(esmCode.indexOf("//")).trimEnd() //
							).trim(),
						{ sync: false }
					);
				}
			}
		}
	},
	cleanup: () => remove_unknown_files(["dist", "utils"], known_files),
});

async function build_steps(obj: { [step: string]: () => Promise<any> | Iterable<Promise<any>> }) {
	const keys = Object.keys(obj);
	const length = keys.length;
	const name = process.env.npm_package_name ?? "unknown";
	const times: { [step: string]: number } = {};
	const b = `Build script ${color.white(name)}`;
	console.log(padEndSection(boldYellow(b), ":", color.grey));
	const INIT_TIME = time();
	for (let i = 0; i < keys.length; i++) {
		const step = keys[i];
		console.log(
			padEndSection(boldYellow(`${color.grey(tr(i, color.yellow))} ${x(i)} ${color.white(step.toUpperCase())}`), ":", color.grey)
		);
		const START_TIME = time();
		const v = obj[keys[i]]();
		if (Symbol.iterator in v) await Promise.all([...v]);
		else await v;
		await new Promise((r) => setImmediate(() => setTimeout(r, 17)));
		times[step] = time_since(START_TIME);
		if (times[step] > 1000) console.log(color.grey(`Completed ${step.toUpperCase()} in ${pretty_timespan(times[step])}`));
	}

	console.log(
		padEndSection(boldYellow(tr(0) + ` ${x(length)} Completed build in ${pretty_timespan(time_since(INIT_TIME))}`), ":", color.grey)
	);

	function padEndSection(msg: string, dash = "- ", dashColor: ColorFn = (str) => str) {
		const length = color.unstyledLength(msg);
		return (
			msg +
			" ".repeat(dash.length - (length % dash.length)) +
			dashColor(dash.repeat(Math.max(0, Math.round((50 - length) / dash.length))))
		);
	}

	function boldYellow(str: string) {
		return color.yellow(color.bold(str));
	}
	function tr(l: number, color: ColorFn = (str) => str) {
		const c = ">";
		return c.repeat(l) + color(c) + c.repeat(length - (l + 1));
	}
	function x(i: number) {
		return `(${i}/${length})`;
	}
}

function trim_same_start(code: string, comparator: string): string {
	var i = 0,
		j = 0;
	while (i < code.length && j < comparator.length && code.charCodeAt(i++) === comparator.charCodeAt(j++));
	return code.slice(code.lastIndexOf("\n", i));
}
function trim_same_ending(code: string, comparator: string): string {
	var i = code.length,
		j = comparator.length;
	while (i-- > 0 && j-- > 0 && code.charCodeAt(i) === comparator.charCodeAt(j));
	return code.slice(0, 1 + i);
}

function transform_parser(Spec: ParsedNodesFile, code: string) {
	// const { ast } = tsParse(code, { range: false });
	// update_file("bundle.original.temp.json", JSONstringify(ast.body), { force: true, prettier: true });
	// return;
	const ARGUMENTS = "ARGUMENTS";
	const READ_NODE = "READ_NODE";
	const mixinNodeReader = `mixinNodeReader`;

	const NodeTypes = new Set(Spec.Nodes.map((Node) => Node.name));
	const toNodeType = createMapper(Spec.Nodes.map(({ name, nodeType }) => [name, nodeType]));
	const readers = createMapper<{ mixin: string; parameters: string[]; body: string }>();
	const mixins = new Map<string, { readLoc: string; readNode: string }>();

	{
		const faulty = [...NodeTypes].filter((Node) => Node.includes("$"));
		assert(!faulty.length, "", faulty);
	}

	code = code.replace(/var ([^=;{<]+) = class /g, "class $1 "); // undo esbuild quirk

	// transform
	code = ts_edit_code(code, function* (body) {
		for (const node of body) {
			if (is_NodeReader(node)) yield eat_NodeReader(node);
			else if (is_Mixin(node)) yield eat_Mixin(node);
			else if (is_SuperMixin(node)) yield remove_node(node);
		}
		for (const node of body) {
			if (is_NodeDeclaration(node)) {
				yield* ES2022_strip_InitProps(node);
				yield replace_node(node.id, NodeType(node.id));
				yield insert_at(node.body.range[1] - 1, inline_ctor_from_reader(node));
			}
		}
	});

	// minify
	code = ts_edit_code(code, function (body) {
		const edits: TSEdit[] = [];
		for (const node of body) {
			simpleTraverse(node, {
				enter(node, parent) {
					if (is_Node_read_call(node)) {
						edits.push(replace_node(node.callee, `new ${NodeType(node.callee.object)}`));
					} else if (is_NodeType_enumGet(node)) {
						edits.push(replace_node(node, "" + toNodeType.get(node.property)));
					} else if (
						parent &&
						parent.type !== NT.VariableDeclarator &&
						node.type === NT.Identifier &&
						toNodeType.has(dedupeId(node.name)) &&
						dedupeId(node.name) !== node.name &&
						!edits.some((edit) => edit.start === node.range[0])
					) {
						edits.push(replace_node(node, NodeType(node)));
					}
				},
			});
		}
		return edits;

		// assertAt(body[0], false);
	});

	code = code
		.replace(/\/\/ prettier-ignore/g, "")
		.replace(/\n\s+/g, "\n")
		.replace(/\/\*[^]*?\*\//g, ""); // remove enum comments (unfortunately, bundling removes actual comments)

	return code;

	// "class { foo: 0; }" (.ts input)
	// "class { foo;    }" (.js output at target >ES2022)  => strip "foo;"
	function* ES2022_strip_InitProps(node: TSESTree.ClassDeclarationWithName & { superClass: TSESTree.Identifier }) {
		for (const prop of node.body.body) {
			if (prop.type === NT.PropertyDefinition && prop.value === null && !prop.static) {
				yield remove_node(prop);
			}
		}
	}
	function is_SuperMixin(
		node: TSESTree.ProgramStatement
	): node is TSESTree.FunctionDeclarationWithName & { id: { name: typeof mixinNodeReader } } {
		return node.type === NT.FunctionDeclaration && node.id.name === mixinNodeReader;
	}
	function is_Mixin(
		node: TSESTree.ProgramStatement
	): node is TSESTree.FunctionDeclarationWithName & { id: { name: `FileLoc${string}` } } {
		return node.type === NT.FunctionDeclaration && node.id.name.includes("FileLoc");
	}
	function eat_Mixin(node: TSESTree.FunctionDeclarationWithName) {
		const stmts = node.body.body;
		assertAt(node.body, stmts.length === 1 && stmts[0].type === NT.ReturnStatement && stmts[0].argument?.type === NT.CallExpression);
		const call = stmts[0].argument;
		const [specClass, getLoc, readNode] = call.arguments;
		assertAt(
			call,
			call.arguments.length === 3 &&
				specClass.type === NT.Identifier &&
				getLoc.type === NT.ArrowFunctionExpression &&
				readNode.type === NT.FunctionExpression
		);
		assertAt(getLoc, getLoc.params.length === 0 || (getLoc.params.length === 1 && is_rest_ARGUMENTS(getLoc.params[0])));
		assertAt(
			readNode,
			readNode.params.length >= 1 &&
				readNode.params[0].type === NT.Identifier &&
				readNode.params[0].name === READ_NODE &&
				(readNode.params.length === 1 || (readNode.params.length === 2 && is_rest_ARGUMENTS(readNode.params[1])))
		);
		mixins.set(getIdentifierName(node.id), { readLoc: sliceBody(getLoc), readNode: sliceBody(readNode) });
		return remove_node(node);
	}

	function eat_NodeReader(node: NodeReader) {
		const read = node.body.body[0];
		readers.set(node.id, {
			mixin: getIdentifierName(node.superClass.callee),
			parameters: read.value.params.map((node) => getIdentifierName(node)),
			body: sliceBody(read.value),
		});
		return remove_node(node);
	}

	function inline_ctor_from_reader(node: TSESTree.ClassDeclarationWithName & { superClass: TSESTree.Identifier }) {
		const { mixin, parameters, body } = readers.get(node.id);
		const { readLoc, readNode } = mixins.get(mixin)! ?? {};
		assertAt(node.id, !!mixin, `"class ... extends FileLoc_x(${node.id.name})" has the wrong name`);
		assertAt(node.id, !!readLoc, `Could not find "${mixin}"`);
		return `
			constructor(${parameters}) {
				super(${toNodeType.get(node.id)}, ${withParamNames(parameters, readLoc)});
				${withParamNames(parameters, readNode).replace(`${READ_NODE}();`, body)}
			}`;

		function withParamNames(parameters: string[], str: string) {
			return str.replace(new RegExp(String.raw`${ARGUMENTS}\[([0-9]+)\]`, "g"), (_, i) => parameters[i]);
		}
	}

	function createMapper<T>(init?: Iterable<readonly [string, T]>) {
		const map = new Map<string, T>(init);
		return {
			get(id: TSESTree.Identifier | string): T {
				if (typeof id === "string") {
					assert(map.has(id), `Could not find '${id}'`);
					return map.get(id)!;
				} else {
					const name = NodeType(id);
					assertAt(id, map.has(name), name, [...map.keys()].includes(name), NodeTypes.has(name));
					return map.get(name)!;
				}
			},
			set(id: TSESTree.Identifier, data: T) {
				const name = NodeType(id);
				assertAt(id, !map.has(name));
				assertAt(id, NodeTypes.has(name));
				map.set(name, data);
			},
			has(name: string) {
				return map.has(name);
			},
		};
	}
	function NodeType(node: TSESTree.Identifier) {
		const name = dedupeId(getIdentifierName(node));
		assertAt(node, NodeTypes.has(name) || name.endsWith("_NODE") /* e.g. PATH_NODE (see CCPATH_read) */, node.name);
		return name;
	}
	function dedupeId(name: string) {
		return name.replace(/\$?[0-9]$/, "");
	}
	function is_rest_ARGUMENTS(param: TSESTree.Parameter) {
		return param.type === NT.RestElement && param.argument.type === NT.Identifier && param.argument.name === ARGUMENTS;
	}

	type NodeReader = TSESTree.ClassDeclarationWithName & {
		superClass: TSESTree.CallExpression;
		body: { body: [{ value: TSESTree.FunctionExpression }] };
	};

	function is_NodeReader(stmt: TSESTree.ProgramStatement): stmt is NodeReader {
		if (
			// class Leet extends FileLoc(Nodes.Leet) { read(...) {...} }
			stmt.type === NT.ClassDeclaration &&
			stmt.superClass?.type === NT.CallExpression &&
			stmt.superClass.arguments.length === 1 &&
			stmt.superClass.arguments[0].type === NT.Identifier &&
			stmt.superClass.callee.type === NT.Identifier &&
			stmt.superClass.callee.name.includes("FileLoc")
		) {
			const read = stmt.body.body[0];
			assertAt(
				read,
				stmt.body.body.length === 1 &&
					read.type === NT.MethodDefinition &&
					getIdentifierName(read.key) === "read" &&
					read.value.type === NT.FunctionExpression
			);
			return true;
		}
		return false;
	}
	function is_NodeDeclaration(
		stmt: TSESTree.ProgramStatement
	): stmt is TSESTree.ClassDeclarationWithName & { superClass: TSESTree.Identifier } {
		if (
			stmt.type === NT.ClassDeclaration && //
			stmt.superClass?.type === NT.Identifier &&
			stmt.superClass.name.includes("BaseNode")
		) {
			const read = stmt.body.body[0];
			assertAt(
				read,
				!(
					stmt.body.body.length === 1 &&
					read.type === NT.MethodDefinition &&
					getIdentifierName(read.key) === "read" &&
					read.value.type === NT.FunctionExpression
				)
			);
			return true;
		}
		return false;
	}
	function sliceBody(node: { type: any; body: { range: [number, number] } }) {
		const { body } = node;
		const dx = node.type === NT.ArrowFunctionExpression ? 0 : 1;
		return code.slice(body.range[0] + dx, body.range[1] - dx);
	}
	function assertAt(node: { range: [number, number] }, predicate: boolean, ...ctx: any[]): asserts predicate {
		if (!predicate) {
			write_file(temp_filepath("index.js"), code);
			write_file(temp_filepath("ast.temp.json"), JSONstringify(tsParse(code, { range: false }).ast.body), {
				prettier: true,
			});
			exit("", urlAt(temp_filepath("index.js"), getLineStarts(code), node.range[0]), ...ctx);
		}
		function temp_filepath(name: string): string {
			return `scripts/dist.temp/${name}`;
		}
	}
	function stripNodeTypeEnum(text: string) {
		// const start = indexOf(/todo/, text);
		const start = text.indexOf("var NodeType;");
		const s = "})(NodeType || (NodeType = {}));";
		const end = text.indexOf(s, start) + s.length;
		assert(start !== -1);
		assert(text.indexOf(s, start) !== -1);
		return text.slice(0, start) + text.slice(end);
		// /var NodeType;[^]*?\(NodeType \|\| \(NodeType = \{\}\)\);/,
		(""); // Added when merging nodes.cjs into index.js // `const NodeType = [${Array.from(node_nodeType.keys(), (name) => `"${name}"`)}]`
	}
	function is_NodeType_enumGet(
		node: TSESTree.Node
	): node is TSESTree.MemberExpression & { object: { name: "NodeType" }; property: TSESTree.Identifier } {
		return (
			node.type === NT.MemberExpression &&
			!node.computed &&
			node.object.type === NT.Identifier &&
			node.property.type === NT.Identifier &&
			node.object.name === "NodeType"
		);
	}

	function is_Node_read_call(
		node: TSESTree.Node
	): node is TSESTree.CallExpression & { callee: { object: TSESTree.Identifier; property: { name: "read" } } } {
		return (
			node.type === NT.CallExpression &&
			node.callee.type === NT.MemberExpression &&
			node.callee.object.type === NT.Identifier &&
			node.callee.property.type === NT.Identifier &&
			node.callee.property.name === "read"
		);
	}
}
