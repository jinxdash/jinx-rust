import { AST_NODE_TYPES, parseAndGenerateServices, TSESTree, TSESTreeOptions } from "@typescript-eslint/typescript-estree";
import {
	assert,
	color,
	dedupe,
	each,
	edit_string,
	exit,
	flat,
	getLineStarts,
	is_object,
	Narrow,
	toArray,
	toArrayFlat,
	urlAt,
} from "../../src/utils/common";
import { read_file, update_file, write_file_temp } from "./fs";

export const TSNodeType = AST_NODE_TYPES;

export function tsParse<T extends TSESTreeOptions>(code: string, options: T = {} as T) {
	try {
		return parseAndGenerateServices(code, {
			errorOnTypeScriptSyntacticAndSemanticIssues: false,
			errorOnUnknownASTType: false,
			ecmaVersion: "latest",
			loc: false,
			comment: false,
			range: true,
			tokens: false,
			...options,
		});
	} catch (e) {
		const url = "scripts/tsParser.error.ts.temp";
		write_file_temp(url, code);
		const cmd = is_object(e) && "lineNumber" in e && "column" in e ? `${url}:${e.lineNumber}:${e.column}` : url;
		exit(
			color.red("Error occured while parsing typescript file:\n") + //
				color.red(`  ${(e as any).message}\n`) +
				color.magenta(`  at ${color.underline(cmd)}`)
		);
	}
}

type Program = TSESTree.Program;
type Node = TSESTree.Node;
type ExportedNode = TSESTree.ExportNamedDeclaration & { declaration: TSESTree.ClassDeclaration & { id: TSESTree.Identifier } };
type ExportedInterface = TSESTree.ExportNamedDeclaration & { declaration: TSESTree.TSInterfaceDeclaration };
type ExportedType = TSESTree.ExportNamedDeclaration & {
	declaration: TSESTree.TSTypeAliasDeclaration & { typeAnnotation: _TypeUnion | _TypeRef };
};
type _TypeReference = TSESTree.TSTypeReference & { typeName: TSESTree.Identifier };
type _TypeUnion = TSESTree.TSUnionType & { types: _TypeRef[] };
type _TypeRef = _TypeReference | TSESTree.TSArrayType | TSESTree.TSUndefinedKeyword;
type _InterfaceProperty = TSESTree.TSPropertySignature & { key: TSESTree.Identifier };

export const NODES_FILEPATH = "src/parser/nodes.ts";

export function getParsedNodesFile(): ParsedNodesFile {
	const DEBUG_NODESFILES = false;

	const { ast } = tsParse(read_file(NODES_FILEPATH), { filePath: NODES_FILEPATH });

	if (DEBUG_NODESFILES) write_file_temp("scripts/nodes.tsAst", ast);

	const classes = getAllNodeClasses(ast);
	const NodeNames = classes.map(getDeclarationName);

	const parsedNodesFile = {} as ParsedNodesFile;

	assert(
		ast.body.some(
			(node) =>
				node.type === TSNodeType.ExportNamedDeclaration &&
				node.declaration?.type === TSNodeType.TSInterfaceDeclaration &&
				node.declaration.id.name === "LocArray"
		),
		`Could not find interface 'LocArray'`
	);

	parsedNodesFile.Interfaces = createResolver<ExportedInterface, FormattedInterface>(
		ast,
		(node: ExportedInterface): node is ExportedInterface =>
			node.type === TSNodeType.ExportNamedDeclaration &&
			node.declaration?.type === TSNodeType.TSInterfaceDeclaration &&
			node.declaration.id.name !== "LocArray" &&
			node.declaration.id.name !== "FunctionDeclarationParameters" &&
			getInterfaceProperties(node).every(
				(property) => property.type === TSNodeType.TSPropertySignature && property.key.type === TSNodeType.Identifier
			),
		(node) => getDeclarationName(node),
		(node, name, createRefQuery) => [
			...getInterfaceProperties(node).map((property) => property.key.name),
			...(node.declaration.extends
				?.filter((d) => d.type === TSNodeType.TSInterfaceHeritage && d.expression.type === TSNodeType.Identifier)
				.map((node) => createRefQuery((node.expression as TSESTree.Identifier).name, node)) ?? []),
		],
		(node, name, properties) => ({ name, properties })
	);

	function is_ExportedType(node: Node): node is ExportedType {
		return node.type === TSNodeType.ExportNamedDeclaration && node.declaration?.type === TSNodeType.TSTypeAliasDeclaration;
	}
	parsedNodesFile.Types = createResolver<ExportedType, FormattedType>(
		ast,
		is_ExportedType,
		(node) => getDeclarationName(node),
		function (node, name, createRefQuery) {
			return resolveType(node, createRefQuery, createGenericConstraintLookup(node));

			function resolveType(node: ExportedType, createRefQuery: (name: string, info?: any) => RawRef, lookupName: LookupFn): RawRef {
				return getTypes(node.declaration.typeAnnotation, lookupName).map(function TypesResolve(ty) {
					assert_is_is_TypeReference(ty);
					const typeName = getTypeReferenceName(ty);

					if (ty.typeParameters?.params.length) {
						const { params } = ty.typeParameters;
						if (typeName === "Extract") {
							const [left, right] = params;
							const shape = findInterfaceFromName(getTypeReferenceName(right));
							return resolveQuery(left).filter((name) => Node_extends_Interface(name, shape));
						}

						if (typeName === "Exclude") {
							const [left, right] = params;
							const rightResolved: ReturnType<typeof findEntity>[] = flat(
								getTypes(right, lookupName).map((type) => query(type as any))
							).map((name) => findEntity(name));
							return resolveQuery(left).filter(
								(name) =>
									!rightResolved.some((shape) =>
										typeof shape === "string" ? shape === name : Node_extends_Interface(name, shape)
									)
							);
						}

						const localType = ast.body.find(
							(node): node is ExportedType => is_ExportedType(node) && getDeclarationName(node) === typeName
						);
						if (localType && localType.declaration.typeParameters?.params.length) {
							return resolveType(
								localType,
								createRefQuery,
								(name) =>
									params[
										localType.declaration.typeParameters?.params.findIndex((param) => param.name.name === name) ?? -1
									] as _TypeRef
							);
						}
					}

					return query(typeName);

					function query(name: string | Node): RawRef {
						const target = typeof name === "string" ? name : getTypeReferenceName(name);
						return NodeNames.some((name) => name === target) ? target : createRefQuery(target);
					}
					function resolveQuery(name: string | Node) {
						return toArrayFlat(query(name));
					}
				});
			}
		},
		(node, name, types) => ({ name, types })
	);

	const allNames = [...[...parsedNodesFile.Types, ...parsedNodesFile.Interfaces].map((Thing) => Thing.name), ...NodeNames];

	parsedNodesFile.Nodes = classes.map((node, i) => {
		assert(node.type === TSNodeType.ExportNamedDeclaration);
		assert(node.declaration.type === TSNodeType.ClassDeclaration);
		assert(node.declaration.id.type === TSNodeType.Identifier);

		type P = (TSESTree.PropertyDefinition | TSESTree.MethodDefinition | TSESTree.TSArrayType) & { key: TSESTree.Identifier };
		return {
			name: getDeclarationName(node),
			nodeType: i,
			implements: node.declaration.implements?.map((node) => getIdentifierName(node.expression)) ?? [],
			properties: node.declaration.body.body.map((Property) => {
				Narrow<P>(Property);
				const name = getIdentifierName(Property.key);
				const types = getTypes(
					(Property as TSESTree.PropertyDefinition).typeAnnotation?.typeAnnotation,
					createGenericConstraintLookup(node)
				).map((Type) => {
					if (Type.type === TSNodeType.TSTypeReference) {
						const typeName = getTypeReferenceName(Type);
						if (typeName === "__DevonlyObject") {
							return "primitive";
						}
						if (typeName === "LocArray" || typeName === "FunctionDeclarationParameters") {
							return "Array";
						}
						if (allNames.includes(typeName)) {
							return "Node";
						}
					}
					if (Type.type === TSNodeType.TSArrayType && Type.elementType.type === TSNodeType.TSTypeReference) {
						return "Array";
					}
					if (Type.type === TSNodeType.TSUndefinedKeyword) {
						return "undefined";
					}
					return "primitive";
				});
				return {
					name,
					types: dedupe(flat(types)),
					declare: "declare" in Property && Property.declare === true,
					optional: "optional" in Property && Property.optional === true,
					getter: "kind" in Property && Property.kind === "get",
				};
			}),
		};
	});

	if (DEBUG_NODESFILES) write_file_temp("scripts/nodes.processed", parsedNodesFile);

	return parsedNodesFile;

	function Node_extends_Interface(nodeName: string, shape: FormattedInterface): unknown {
		const node = classes[NodeNames.indexOf(nodeName)];
		assert(!!node);
		return shape.properties.every((prop) => Node_has_property(node, prop));
		function Node_has_property(node: ExportedNode, name: string) {
			return node.declaration.body.body.some((Property) => "key" in Property && getIdentifierName(Property.key) === name);
		}
	}

	function findNodeFromName(name: string): string {
		const res = NodeNames.find((x) => x === name);
		assert(!!res, `Could not find Node '${name}'`);
		return res;
	}

	function findInterfaceFromName(name: string): FormattedInterface {
		const res = parsedNodesFile.Interfaces.find((Interface) => Interface.name === name);
		assert(!!res, `Could not find Interface '${name}'`);
		return res;
	}

	function findEntity(name: string): string | FormattedInterface {
		const res = NodeNames.find((x) => x === name) ?? parsedNodesFile.Interfaces.find((Interface) => name === Interface.name);
		assert(!!res, `Could not find a Node or an Interface named '${name}'`);
		return res;
	}
}

export interface ParsedNodesFile {
	Interfaces: readonly FormattedInterface[];
	Types: readonly FormattedType[];
	Nodes: readonly FormattedNode[];
}

function getDeclarationName(node: { declaration: { id: { name: string } } }) {
	return node.declaration.id.name;
}

export interface FormattedNode {
	name: string;
	nodeType: number;
	implements: string[];
	properties: FormattedNodeProperty[];
}
interface FormattedNodeProperty {
	name: string;
	types: ("Array" | "Node" | "undefined" | "primitive")[];
	declare: boolean;
	optional: boolean;
	getter: boolean;
}

interface FormattedType {
	name: string;
	types: string[];
}

interface FormattedInterface {
	name: string;
	properties: string[];
}

type ResolvedRef = string[] | string;
type UnresolvedRef = (DeferredRef | ResolvedRef)[];
type DeferredRef = () => string | UnresolvedRef | undefined;
type RawRef = string | RawRef[];
function createResolver<T extends Node, F extends { name: string }>(
	program: Program,
	isTarget: (node: Node) => node is T,
	getName: (node: T) => string,
	getRefs: (node: T, name: string, createRefQuery: (name: string, info?: any) => RawRef) => RawRef,
	format: (node: T, name: string, refs: string[]) => F
) {
	type Entry = { node: T; name: string; refs: RawRef[] };
	const Mappings = new Map<string, Entry>();
	const entries: Entry[] = [];

	for (const node of program.body) {
		if (isTarget(node)) {
			const name = getName(node);
			const entry: Entry = { node, name, refs: [] };
			assert(!Mappings.has(name));
			entries.push(entry);
			Mappings.set(name, entry);
		}
	}

	each(entries, (entry) => {
		const refs = getRefs(entry.node, entry.name, (name: string, info?: any) => {
			assert(Mappings.has(name), `Failed to resolve '${name}'`, info);
			return Mappings.get(name)!.refs;
		});
		each(toArray(refs), (ref, i) => {
			entry.refs[i] = ref;
		});
	});

	return entries.map((entry) => format(entry.node, entry.name, dedupe(flat(entry.refs))));
}

function getAllNodeClasses(program: Program) {
	return program.body.filter(function (node: ExportedNode) {
		return (
			node.type === TSNodeType.ExportNamedDeclaration && //
			node.declaration?.type === TSNodeType.ClassDeclaration &&
			!node.declaration.abstract &&
			node.declaration.superClass?.type === TSNodeType.Identifier &&
			node.declaration.superClass?.name?.endsWith("BaseNode")
		);
	}) as ExportedNode[];
}
function is_TypeReference(node: Node): node is _TypeReference {
	return node.type === TSNodeType.TSTypeReference && node.typeName.type === TSNodeType.Identifier;
}

function assert_is_is_TypeReference(node: Node): asserts node is _TypeReference {
	assert(is_TypeReference(node), "Expected TypeReference Identifier", node);
}
function getTypeReferenceName(node: Node) {
	assert_is_is_TypeReference(node);
	return getIdentifierName(node.typeName);
}
function getTypes(typeAnnotation: TSESTree.TypeNode | undefined, lookupName: LookupFn): _TypeRef[] {
	return flat(
		(function ExtractTypes(node?: TSESTree.TypeNode): _TypeRef[] {
			if (!node) return [];
			if (node?.type === TSNodeType.TSUnionType) return node.types.map(ExtractTypes) as any[];
			if (node?.type === TSNodeType.TSTypeReference) {
				const name = getTypeReferenceName(node);
				const resolvedType = lookupName(name);
				if (resolvedType) return ExtractTypes(resolvedType);
			}
			return [node as _TypeRef];
		})(typeAnnotation)
	);
}
type LookupFn = (name: string) => _TypeRef | undefined;
function createGenericConstraintLookup(parent: ExportedNode | ExportedType | ExportedInterface): LookupFn {
	return function lookup(name: string) {
		return parent.declaration.typeParameters?.params.find((param) => param.name.name === name)?.constraint as _TypeRef | undefined;
	};
}
function getInterfaceProperties(node: ExportedInterface): _InterfaceProperty[] {
	return node.declaration.body.body as _InterfaceProperty[];
}
export function getIdentifierName(node: Node) {
	return asIdentifier(node).name;
}
function asIdentifier(node: Node | null): TSESTree.Identifier {
	assert(node?.type === TSNodeType.Identifier, "Expected Identifier", node);
	return node;
}

export type TSEdit = { start: number; end: number; content: string };
interface NodeEditor {
	extends: (superName: string, typeArguments: (string | number)[]) => string;
}
interface InitNodeEditor {
	(node: ExportedNode): NodeEditor;
}

export function edit_nodes(initEditor: InitNodeEditor) {
	const code = read_file(NODES_FILEPATH)!;
	const { ast } = tsParse(code, { filePath: NODES_FILEPATH });
	const edits: TSEdit[] = [];

	each(getAllNodeClasses(ast), (Node) => {
		const editor = initEditor(Node);
		const superClass = asIdentifier(Node.declaration.superClass);
		const superTypeParameters = Node.declaration.superTypeParameters;
		const start = superClass.range[0];
		const end = (superTypeParameters ? superTypeParameters : superClass).range[1];
		const args =
			superTypeParameters?.params?.map(function (arg) {
				const data = ntext(arg);
				return isNaN(+data) ? data : +data;
			}) ?? [];

		addEdit({ start, end, content: editor.extends(superClass.name, args) });
	});

	if (edits.length > 0) {
		edits.sort((a, b) => b.start - a.start);
		let res = code;
		each(edits, (edit) => {
			res = edit_string(res, edit.start, edit.content, edit.end);
		});
		update_file(NODES_FILEPATH, res);
	}

	function addEdit(edit: TSEdit) {
		if (code.slice(edit.start, edit.end) !== edit.content) edits.push(edit);
	}

	function ntext(node: Node) {
		return code.slice(node.range[0], node.range[1]);
	}
}

export function ts_edit_code(code: string, getEdits: (body: Program["body"]) => Iterable<TSEdit>): string {
	let res = "";
	let prev: TSEdit = { start: code.length, end: code.length, content: "" };
	for (const next of [...getEdits(tsParse(code).ast.body)].sort((a, b) => b.end - a.end)) {
		if (prev.start < next.end) {
			if (prev.start === next.start && prev.end === next.end && next.content.includes(prev.content)) continue;
			const at = urlAt.bind(null, write_file_temp("scripts/overlap.temp.ts", code), getLineStarts(code));
			exit("Edits overlap:", {
				prev: { ...prev, start: at(prev.start), end: prev.start },
				next: { ...next, start: at(next.start), end: next.start },
			});
		}
		res = next.content + code.slice(next.end, prev.start) + res;
		prev = next;
	}
	return code.slice(0, prev.start) + res;
}

export function remove_node(node: { range: [number, number] }): TSEdit {
	return { content: "", start: node.range[0], end: node.range[1] };
}
export function replace_node(node: { range: [number, number] }, content: string): TSEdit {
	return { content, start: node.range[0], end: node.range[1] };
}
export function insert_at(pos: number, content: string): TSEdit {
	return { content, start: pos, end: pos };
}
export function append_to(node: { range: [number, number] }, content: string): TSEdit {
	return { content, start: node.range[1], end: node.range[1] };
}
export function slice_node(code: string, node: { range: [number, number] }) {
	return code.slice(node.range[0], node.range[1]);
}
