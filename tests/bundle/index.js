var __defProp = Object.defineProperty;
var __export = (target, all) => {
	for (var name in all) __defProp(target, name, { get: all[name], enumerable: true });
};
// src/parser/index.ts
var parser_exports = {};
__export(parser_exports, {
	createLocArray: () => createLocArray,
	mockNode: () => mockNode,
	parseFile: () => parseFile,
	toBlockBody: () => toBlockBody,
	toCallExpressionArguments: () => toCallExpressionArguments,
	toExpression: () => toExpression,
	toTokens: () => toTokens,
});
// src/utils/debug.ts
var cwd =
	typeof process === "object" && typeof (process == null ? void 0 : process.cwd) === "function" ? normPath(process.cwd() ?? "") : "";
function normPath_strip_cwd(filepath) {
	let normFilePath = normPath(filepath);
	return normFilePath.startsWith(cwd) ? normFilePath.slice(cwd.length + 1) : normFilePath;
}
class StackLine {
	constructor(raw) {
		({
			1: this.callee = "",
			2: this.filepath = "",
			3: this.line = "",
			4: this.col = "",
			5: this.other = "",
		} = (this.raw = raw).match(/at (?:(.+?)\s+\()?(?:(.+?):([0-9]+)(?::([0-9]+))?|([^)]+))\)?/) ?? ["", "", "", "", "", ""]);
		this.url = this.filepath
			? normPath_strip_cwd(this.filepath) + (this.line && this.col && `:${this.line}:${this.col}`)
			: this.other === "native"
			? "<native>"
			: "";
	}
}
function getPrintWidth() {
	return clamp(0, getTerminalWidth(128), 200) - 4;
}
class StackItem extends StackLine {
	constructor(stack, i, raw) {
		super(raw);
		this.stack = stack;
		this.i = i;
		this.hidden = false;
	}
	hide() {
		this.hidden = true;
		return this;
	}
	hideNext(n) {
		var _a;
		for (let i = 0; i < n; i++) (_a = this.at(i)) == null ? void 0 : _a.hide();
	}
	hideWhileTrue(test) {
		let line2 = this;
		while (line2 && test(line2)) line2 = line2.hide().next();
	}
	at(relIndex) {
		return this.i + relIndex >= this.stack.length || this.i + relIndex < 0 ? void 0 : this.stack[this.i + relIndex];
	}
	next() {
		return this.at(1);
	}
	toString() {
		var _a, _b, _c, _d;
		const url = this.url;
		const calleeColor =
			((_b = (_a = this.stack.style) == null ? void 0 : _a.callee) == null ? void 0 : _b.call(_a, this.callee, this)) ?? color.cyan;
		const urlColor = ((_d = (_c = this.stack.style) == null ? void 0 : _c.url) == null ? void 0 : _d.call(_c, url, this)) ?? color.grey;
		return compose2Cols("    at " + calleeColor(this.callee), urlColor(url), getPrintWidth());
	}
}
function createStack(message, Error_stack, style) {
	for (var STACK = [], i = 0, stack = Error_stack.split("\n").slice(1); i < stack.length; i++)
		STACK[i] = new StackItem(STACK, i, stack[i]);
	return (STACK.message = message), (STACK.style = style), STACK;
}
function composeStack(stack) {
	var hidden = 0;
	var str = stack.message;
	for (var item of stack) item.hidden ? ++hidden : (str += "\n" + item.toString());
	return str + (hidden > 0 ? "\n" + color.grey(compose2Cols("", `...filtered ${hidden} lines`, getPrintWidth())) : "");
}
function createCustomError({
	ctor = createCustomError,
	message = "Unknown Error",
	editStack = (stack) => {},
	style = void 0,
	stackTraceLimit = 20,
}) {
	const _stackTraceLimit = Error.stackTraceLimit;
	const _prepareStackTrace = Error.prepareStackTrace;
	Error.stackTraceLimit = stackTraceLimit;
	const _ctx = {};
	Error.captureStackTrace(_ctx, ctor);
	const stack = createStack(message, _ctx.stack, style);
	Error.prepareStackTrace = function (err2, calls) {
		editStack(stack);
		return composeStack(stack);
	};
	const err = new Error(message);
	err.stack = err.stack;
	Error.stackTraceLimit = _stackTraceLimit;
	Error.prepareStackTrace = _prepareStackTrace;
	return err;
}
function d(...args) {
	return reduce_tagged_template(args, (data) =>
		null == data
			? color.magenta("" + data)
			: is_Node(data)
			? "name" in data
				? TY(REF(data.type), VAL(`"${data.name}"`))
				: "token" in data
				? TY(REF(data.type), VAL(`"${data.token}"`))
				: REF(data.type)
			: is_LocArray(data)
			? REF("LocArray")
			: typeof data === "function"
			? REF(data.name)
			: VAL(data)
	);
	function REF(data) {
		return color.cyan("" + data);
	}
	function TY(thing, data) {
		return color.grey(`${thing}<${data}>`);
	}
	function VAL(data) {
		return color.yellow("" + data);
	}
}
function compose2Cols(left, right, len = 64, min2 = 1) {
	return left + " ".repeat(clamp(min2, len, len - (color.unstyledLength(left) + color.unstyledLength(right)))) + right;
}
var __parser_maybe_exit;
function __SET_PARSER_ERROR_MNGR(_maybe_exit) {
	__parser_maybe_exit = _maybe_exit;
}
function exit(message, ...ctx) {
	__parser_maybe_exit == null ? void 0 : __parser_maybe_exit(message, ctx);
	if (ctx.length > 0) console.log("Error context:", { ...ctx });
	throw createCustomError({ message, ctor: exit });
}
exit.never = function never(...ctx) {
	exit("Reached unreachable code", ...ctx);
};
function pretty_join(ARR, { quote = false, tail = "and", empty = "" } = {}) {
	if (ARR.length === 0) return empty;
	const MAP = quote
		? function wrap_str(str) {
				return typeof str === "string" ? (str.includes('"') ? `'${str}'` : `"${str}"`) : "" + str;
		  }
		: Identity;
	const DEDUPED = dedupe(ARR);
	var res = "" + MAP(ARR[0]);
	if (DEDUPED.length === 1) return res;
	for (var i = 1; i < DEDUPED.length - 1; ++i) res += ", " + MAP(ARR[i]);
	return res + " " + tail + " " + MAP(last_of(ARR));
}
function Identity(v) {
	return v;
}
function last_of(arr) {
	return arr[arr.length - 1];
}
function normPath(filepath) {
	return filepath.replace(/^file:\/\/\//, "").replace(/\\\\?/g, "/");
}
function strChar(sequence) {
	switch (typeof sequence) {
		case "number":
			return print_string(String.fromCharCode(sequence));
		case "object":
			return print_string(String.fromCharCode(...sequence));
		default:
			return print_string(sequence);
	}
}
function strToken(sequence) {
	return sequence.includes(39) ? `\`${strChar(sequence)}\`` : `'${strChar(sequence)}'`;
}
function print_string(str) {
	return /[\u0000-\u0020]/.test(str)
		? str
				.replace(/ /g, "\u2022")
				.replace(/\n/g, "\u21B2")
				.replace(/\t/g, "\u255A")
				.replace(/[\u0000-\u0020]/g, "\u25A1")
		: str;
}
function getLineStarts(str) {
	for (var arr = [0], i = 0, j = 0; i < str.length - 1; i++) if (10 === str.charCodeAt(i)) arr[++j] = i + 1;
	return arr;
}
function urlAt(filepath, lineStarts, index) {
	const line2 = void 0 === lineStarts || void 0 === index || index < 0 ? -1 : getLineIndex(lineStarts, index);
	return normPath(filepath) + (-1 === line2 ? "" : `:${1 + line2}:${1 + (index - lineStarts[line2])}`);
}
function binarySearch(array, target) {
	if (isEmpty(array)) return -1;
	let i = 0;
	let low = 0;
	let high = array.length - 1;
	if (target >= array[high]) return high;
	else high--;
	while (low <= high) {
		i = low + ((high - low) >> 1);
		if (target === array[i]) return i;
		if (target > array[i]) low = i + 1;
		else high = i - 1;
	}
	return low - 1;
}
function binarySearchIn(array, target, toValue) {
	if (isEmpty(array)) return -1;
	let i = 0;
	let low = 0;
	let high = array.length - 1;
	let value = toValue(array[high]);
	if (target >= value) return high;
	else high--;
	while (low <= high) {
		i = low + ((high - low) >> 1);
		value = toValue(array[i]);
		if (target === value) return i;
		if (target > value) low = i + 1;
		else high = i - 1;
	}
	return low - 1;
}
function binaryInsertIn(array, item, toValue) {
	if (0 === array.length || toValue(item) >= toValue(array[array.length - 1])) array[array.length] = item;
	else array.splice(1 + binarySearchIn(array, toValue(item), toValue), 0, item);
}
function binaryInsertEach(array, items, toValue) {
	for (var i = 0; i !== items.length; i++) binaryInsertIn(array, items[i], toValue);
}
function getLineIndex(lineStarts, index) {
	return binarySearch(lineStarts, index);
}
function getLineChar(lineStarts, index) {
	const line2 = getLineIndex(lineStarts, index);
	return { line: line2, char: index - lineStarts[line2] };
}
function getTerminalWidth(fallbackWidth = 200) {
	return 100;
}
var isBrowser = typeof window !== "undefined" && typeof window.document !== "undefined";
var color = ((cfn, mfn) => ({
	black: cfn(30),
	red: cfn(31),
	green: cfn(32),
	yellow: cfn(33),
	blue: cfn(34),
	magenta: cfn(35),
	cyan: cfn(36),
	white: cfn(37),
	grey: cfn(90),
	bold: mfn(1, 22),
	italic: mfn(3, 23),
	underline: mfn(4, 24),
	hidden: mfn(8, 28),
	hiddenCursor: (str) => `\x1B[?25l${str}\x1B[?25h`,
	unstyle: (str) => str.replace(/\x1B\[[0-9][0-9]?m/g, ""),
	unstyledLength: (str) => str.replace(/\x1B\[[0-9][0-9]?m/g, "").length,
	link: (str) => color.underline(color.blue(str)),
}))(
	(c1) => (isBrowser ? Identity : (str) => `\x1B[${c1}m${str.replace(/\x1B\[39m/g, `\x1B[${c1}m`)}\x1B[39m`),
	(c1, c2) => (isBrowser ? Identity : (str) => `\x1B[${c1}m${str}\x1B[${c2}m`)
);
function dedupe(arr) {
	return [...new Set(arr)];
}
function isEmpty(array) {
	return 0 === array.length;
}
function has_key(o, k) {
	return k in o;
}
function is_object(data) {
	return "object" === typeof data && null !== data;
}
function clamp(min2, max, value) {
	return value > min2 ? (value < max ? value : max) : min2;
}
function reduce_tagged_template(args, map) {
	for (var str = "" + args[0][0], i = 1; i < args.length; i++) str += map(args[i]) + args[0][i];
	return str;
}
function spliceAll(array) {
	const r = [...array];
	array.length = 0;
	return r;
}
// src/utils/ast/nodetype.ts
function nis(node, nodeType) {
	return nodeType === node.nodeType;
}
function is_ItemPath(node) {
	return nis(node, 11);
}
function is_ParenthesizedExpression(node) {
	return nis(node, 68);
}
function is_ClosureFunctionExpression(node) {
	return nis(node, 79);
}
function is_RangeLiteral(node) {
	return nis(node, 89);
}
function is_RestPattern(node) {
	return nis(node, 104);
}
function is_TypeTraitBound(node) {
	return nis(node, 130);
}
function is_ExpressionNamespaceTarget(node) {
	switch (node.nodeType) {
		case 6:
		case 56:
		case 57:
		case 58:
			return true;
		default:
			return false;
	}
}
function is_MaybePubNode(node) {
	switch (node.nodeType) {
		case 18:
		case 26:
		case 31:
		case 32:
		case 33:
		case 34:
		case 35:
		case 37:
		case 38:
		case 42:
		case 43:
		case 44:
		case 45:
		case 46:
		case 47:
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
			return true;
		default:
			return false;
	}
}
function is_MaybeExternNode(node) {
	switch (node.nodeType) {
		case 38:
		case 133:
			return true;
		default:
			return false;
	}
}
function is_MaybeAsyncNode(node) {
	switch (node.nodeType) {
		case 38:
		case 79:
		case 81:
			return true;
		default:
			return false;
	}
}
function is_MaybeMoveNode(node) {
	switch (node.nodeType) {
		case 79:
		case 81:
			return true;
		default:
			return false;
	}
}
function is_MaybeUnsafeNode(node) {
	switch (node.nodeType) {
		case 32:
		case 37:
		case 38:
		case 51:
		case 52:
		case 53:
		case 54:
		case 81:
		case 133:
			return true;
		default:
			return false;
	}
}
function is_MaybeStaticNode(node) {
	return is_ClosureFunctionExpression(node);
}
function is_ParenthesizedNode(node) {
	switch (node.nodeType) {
		case 68:
		case 103:
		case 142:
			return true;
		default:
			return false;
	}
}
function is_ExpressionWithBodyOrCases(node) {
	switch (node.nodeType) {
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
		case 87:
			return true;
		default:
			return false;
	}
}
function is_Lifetime(node) {
	switch (node.nodeType) {
		case 120:
		case 121:
		case 122:
			return true;
		default:
			return false;
	}
}
function is_BareTypeTraitBound(node) {
	return !hasAttributes(node) && !node.maybeConst && !node.optional && void 0 === node.ltParameters;
}
function isOuter(node) {
	return !node.inner;
}
// src/utils/ast/helpers.ts
function hasAttributes(node) {
	return has_key(node, "attributes");
}
function getParenthesizedNodeContent(node) {
	switch (node.nodeType) {
		case 68:
			return node.expression;
		case 103:
			return node.pattern;
		case 142:
			return node.typeExpression;
	}
	exit.never(node);
}
function unsafe_setRangeStart(target, startPos) {
	target.loc[0] = startPos;
}
function unsafe_setRangeEnd(target, endPos) {
	target.loc[1] = endPos;
}
function internal_saveOwnStart(target) {
	target.loc[2] = start(target);
}
function setRangeStart(target, startPos) {
	unsafe_setRangeStart(target, startPos);
}
function setRangeEnd(target, endPos) {
	unsafe_setRangeEnd(target, endPos);
}
function setRange(target, startPos, endPos) {
	setRangeStart(target, startPos);
	setRangeEnd(target, endPos);
}
function assignAttributes(target, attrs) {
	if (hasAttributes(target)) insertNodes(target.attributes, attrs);
	else target.attributes = [...attrs];
	if (isOuter(attrs[0])) {
		if (!hasOwnStart(target)) internal_saveOwnStart(target);
		setRangeStart(target, start(attrs[0]));
	}
}
function insertNode(array, node) {
	binaryInsertIn(array, node, start);
}
function insertNodes(array, nodes) {
	binaryInsertEach(array, nodes, start);
}
// src/utils/ast/index.ts
function is_Located(data) {
	return is_object(data) && "loc" in data;
}
function is_Node(node) {
	return is_object(node) && "nodeType" in node;
}
function is_LocArray(data) {
	return is_Located(data) && "length" in data;
}
function start(node) {
	return node.loc[0];
}
function end(node) {
	return node.loc[1];
}
function hasOwnStart(node) {
	return 2 in node.loc;
}
function ownStart(node) {
	return 2 in node.loc ? node.loc[2] : start(node);
}
// src/parser/nodes.ts
var NodeType = ((NodeType3) => {
	NodeType3[(NodeType3["MissingNode"] = 0)] = "MissingNode";
	NodeType3[(NodeType3["SourceFile"] = 1)] = "SourceFile";
	NodeType3[(NodeType3["Shebang"] = 2)] = "Shebang";
	NodeType3[(NodeType3["Program"] = 3)] = "Program";
	NodeType3[(NodeType3["Snippet"] = 4)] = "Snippet";
	NodeType3[(NodeType3["Comment"] = 5)] = "Comment";
	NodeType3[(NodeType3["Identifier"] = 6)] = "Identifier";
	NodeType3[(NodeType3["Index"] = 7)] = "Index";
	NodeType3[(NodeType3["LbIdentifier"] = 8)] = "LbIdentifier";
	NodeType3[(NodeType3["McIdentifier"] = 9)] = "McIdentifier";
	NodeType3[(NodeType3["Literal"] = 10)] = "Literal";
	NodeType3[(NodeType3["ItemPath"] = 11)] = "ItemPath";
	NodeType3[(NodeType3["PunctuationToken"] = 12)] = "PunctuationToken";
	NodeType3[(NodeType3["DelimGroup"] = 13)] = "DelimGroup";
	NodeType3[(NodeType3["Attribute"] = 14)] = "Attribute";
	NodeType3[(NodeType3["DocCommentAttribute"] = 15)] = "DocCommentAttribute";
	NodeType3[(NodeType3["MacroInvocation"] = 16)] = "MacroInvocation";
	NodeType3[(NodeType3["MacroRulesDeclaration"] = 17)] = "MacroRulesDeclaration";
	NodeType3[(NodeType3["MacroDeclaration"] = 18)] = "MacroDeclaration";
	NodeType3[(NodeType3["MacroRuleDeclaration"] = 19)] = "MacroRuleDeclaration";
	NodeType3[(NodeType3["MacroInlineRuleDeclaration"] = 20)] = "MacroInlineRuleDeclaration";
	NodeType3[(NodeType3["MacroGroup"] = 21)] = "MacroGroup";
	NodeType3[(NodeType3["MacroParameterDeclaration"] = 22)] = "MacroParameterDeclaration";
	NodeType3[(NodeType3["PubSpecifier"] = 23)] = "PubSpecifier";
	NodeType3[(NodeType3["ExternSpecifier"] = 24)] = "ExternSpecifier";
	NodeType3[(NodeType3["ExpressionStatement"] = 25)] = "ExpressionStatement";
	NodeType3[(NodeType3["UseStatement"] = 26)] = "UseStatement";
	NodeType3[(NodeType3["NamedImport"] = 27)] = "NamedImport";
	NodeType3[(NodeType3["AmbientImport"] = 28)] = "AmbientImport";
	NodeType3[(NodeType3["AnonymousImport"] = 29)] = "AnonymousImport";
	NodeType3[(NodeType3["DestructuredImport"] = 30)] = "DestructuredImport";
	NodeType3[(NodeType3["ExternCrateStatement"] = 31)] = "ExternCrateStatement";
	NodeType3[(NodeType3["ExternBlockDeclaration"] = 32)] = "ExternBlockDeclaration";
	NodeType3[(NodeType3["TypeAliasDeclaration"] = 33)] = "TypeAliasDeclaration";
	NodeType3[(NodeType3["ConstVariableDeclaration"] = 34)] = "ConstVariableDeclaration";
	NodeType3[(NodeType3["StaticVariableDeclaration"] = 35)] = "StaticVariableDeclaration";
	NodeType3[(NodeType3["LetVariableDeclaration"] = 36)] = "LetVariableDeclaration";
	NodeType3[(NodeType3["ModuleDeclaration"] = 37)] = "ModuleDeclaration";
	NodeType3[(NodeType3["FunctionDeclaration"] = 38)] = "FunctionDeclaration";
	NodeType3[(NodeType3["FunctionSelfParameterDeclaration"] = 39)] = "FunctionSelfParameterDeclaration";
	NodeType3[(NodeType3["FunctionParameterDeclaration"] = 40)] = "FunctionParameterDeclaration";
	NodeType3[(NodeType3["FunctionSpread"] = 41)] = "FunctionSpread";
	NodeType3[(NodeType3["StructDeclaration"] = 42)] = "StructDeclaration";
	NodeType3[(NodeType3["TupleStructDeclaration"] = 43)] = "TupleStructDeclaration";
	NodeType3[(NodeType3["StructPropertyDeclaration"] = 44)] = "StructPropertyDeclaration";
	NodeType3[(NodeType3["TupleStructItemDeclaration"] = 45)] = "TupleStructItemDeclaration";
	NodeType3[(NodeType3["UnionDeclaration"] = 46)] = "UnionDeclaration";
	NodeType3[(NodeType3["EnumDeclaration"] = 47)] = "EnumDeclaration";
	NodeType3[(NodeType3["EnumMemberDeclaration"] = 48)] = "EnumMemberDeclaration";
	NodeType3[(NodeType3["EnumMemberTupleDeclaration"] = 49)] = "EnumMemberTupleDeclaration";
	NodeType3[(NodeType3["EnumMemberStructDeclaration"] = 50)] = "EnumMemberStructDeclaration";
	NodeType3[(NodeType3["TraitDeclaration"] = 51)] = "TraitDeclaration";
	NodeType3[(NodeType3["AutoTraitDeclaration"] = 52)] = "AutoTraitDeclaration";
	NodeType3[(NodeType3["TraitAliasDeclaration"] = 53)] = "TraitAliasDeclaration";
	NodeType3[(NodeType3["ImplDeclaration"] = 54)] = "ImplDeclaration";
	NodeType3[(NodeType3["NegativeImplDeclaration"] = 55)] = "NegativeImplDeclaration";
	NodeType3[(NodeType3["ExpressionTypeSelector"] = 56)] = "ExpressionTypeSelector";
	NodeType3[(NodeType3["ExpressionTypeCast"] = 57)] = "ExpressionTypeCast";
	NodeType3[(NodeType3["ExpressionPath"] = 58)] = "ExpressionPath";
	NodeType3[(NodeType3["ExpressionAsTypeCast"] = 59)] = "ExpressionAsTypeCast";
	NodeType3[(NodeType3["ReturnExpression"] = 60)] = "ReturnExpression";
	NodeType3[(NodeType3["BreakExpression"] = 61)] = "BreakExpression";
	NodeType3[(NodeType3["ContinueExpression"] = 62)] = "ContinueExpression";
	NodeType3[(NodeType3["YieldExpression"] = 63)] = "YieldExpression";
	NodeType3[(NodeType3["CallExpression"] = 64)] = "CallExpression";
	NodeType3[(NodeType3["MemberExpression"] = 65)] = "MemberExpression";
	NodeType3[(NodeType3["AwaitExpression"] = 66)] = "AwaitExpression";
	NodeType3[(NodeType3["UnwrapExpression"] = 67)] = "UnwrapExpression";
	NodeType3[(NodeType3["ParenthesizedExpression"] = 68)] = "ParenthesizedExpression";
	NodeType3[(NodeType3["MinusExpression"] = 69)] = "MinusExpression";
	NodeType3[(NodeType3["NotExpression"] = 70)] = "NotExpression";
	NodeType3[(NodeType3["OrExpression"] = 71)] = "OrExpression";
	NodeType3[(NodeType3["AndExpression"] = 72)] = "AndExpression";
	NodeType3[(NodeType3["ReassignmentExpression"] = 73)] = "ReassignmentExpression";
	NodeType3[(NodeType3["UnassignedExpression"] = 74)] = "UnassignedExpression";
	NodeType3[(NodeType3["OperationExpression"] = 75)] = "OperationExpression";
	NodeType3[(NodeType3["ReassignmentOperationExpression"] = 76)] = "ReassignmentOperationExpression";
	NodeType3[(NodeType3["ComparisonExpression"] = 77)] = "ComparisonExpression";
	NodeType3[(NodeType3["LetScrutinee"] = 78)] = "LetScrutinee";
	NodeType3[(NodeType3["ClosureFunctionExpression"] = 79)] = "ClosureFunctionExpression";
	NodeType3[(NodeType3["ClosureFunctionParameterDeclaration"] = 80)] = "ClosureFunctionParameterDeclaration";
	NodeType3[(NodeType3["BlockExpression"] = 81)] = "BlockExpression";
	NodeType3[(NodeType3["LoopBlockExpression"] = 82)] = "LoopBlockExpression";
	NodeType3[(NodeType3["WhileBlockExpression"] = 83)] = "WhileBlockExpression";
	NodeType3[(NodeType3["ForInBlockExpression"] = 84)] = "ForInBlockExpression";
	NodeType3[(NodeType3["TryBlockExpression"] = 85)] = "TryBlockExpression";
	NodeType3[(NodeType3["IfBlockExpression"] = 86)] = "IfBlockExpression";
	NodeType3[(NodeType3["MatchExpression"] = 87)] = "MatchExpression";
	NodeType3[(NodeType3["MatchExpressionCase"] = 88)] = "MatchExpressionCase";
	NodeType3[(NodeType3["RangeLiteral"] = 89)] = "RangeLiteral";
	NodeType3[(NodeType3["StructLiteral"] = 90)] = "StructLiteral";
	NodeType3[(NodeType3["StructLiteralProperty"] = 91)] = "StructLiteralProperty";
	NodeType3[(NodeType3["StructLiteralPropertyShorthand"] = 92)] = "StructLiteralPropertyShorthand";
	NodeType3[(NodeType3["StructLiteralPropertySpread"] = 93)] = "StructLiteralPropertySpread";
	NodeType3[(NodeType3["StructLiteralRestUnassigned"] = 94)] = "StructLiteralRestUnassigned";
	NodeType3[(NodeType3["TupleLiteral"] = 95)] = "TupleLiteral";
	NodeType3[(NodeType3["ArrayLiteral"] = 96)] = "ArrayLiteral";
	NodeType3[(NodeType3["SizedArrayLiteral"] = 97)] = "SizedArrayLiteral";
	NodeType3[(NodeType3["ReferenceExpression"] = 98)] = "ReferenceExpression";
	NodeType3[(NodeType3["RawReferenceExpression"] = 99)] = "RawReferenceExpression";
	NodeType3[(NodeType3["DereferenceExpression"] = 100)] = "DereferenceExpression";
	NodeType3[(NodeType3["BoxExpression"] = 101)] = "BoxExpression";
	NodeType3[(NodeType3["UnionPattern"] = 102)] = "UnionPattern";
	NodeType3[(NodeType3["ParenthesizedPattern"] = 103)] = "ParenthesizedPattern";
	NodeType3[(NodeType3["RestPattern"] = 104)] = "RestPattern";
	NodeType3[(NodeType3["WildcardPattern"] = 105)] = "WildcardPattern";
	NodeType3[(NodeType3["PatternVariableDeclaration"] = 106)] = "PatternVariableDeclaration";
	NodeType3[(NodeType3["StructPattern"] = 107)] = "StructPattern";
	NodeType3[(NodeType3["StructPatternPropertyDestructured"] = 108)] = "StructPatternPropertyDestructured";
	NodeType3[(NodeType3["StructPatternPropertyShorthand"] = 109)] = "StructPatternPropertyShorthand";
	NodeType3[(NodeType3["TuplePattern"] = 110)] = "TuplePattern";
	NodeType3[(NodeType3["ArrayPattern"] = 111)] = "ArrayPattern";
	NodeType3[(NodeType3["ReferencePattern"] = 112)] = "ReferencePattern";
	NodeType3[(NodeType3["BoxPattern"] = 113)] = "BoxPattern";
	NodeType3[(NodeType3["MinusPattern"] = 114)] = "MinusPattern";
	NodeType3[(NodeType3["RangePattern"] = 115)] = "RangePattern";
	NodeType3[(NodeType3["TypePath"] = 116)] = "TypePath";
	NodeType3[(NodeType3["TypeCall"] = 117)] = "TypeCall";
	NodeType3[(NodeType3["TypeCallNamedArgument"] = 118)] = "TypeCallNamedArgument";
	NodeType3[(NodeType3["TypeCallNamedBound"] = 119)] = "TypeCallNamedBound";
	NodeType3[(NodeType3["LtIdentifier"] = 120)] = "LtIdentifier";
	NodeType3[(NodeType3["LtElided"] = 121)] = "LtElided";
	NodeType3[(NodeType3["LtStatic"] = 122)] = "LtStatic";
	NodeType3[(NodeType3["TypeNever"] = 123)] = "TypeNever";
	NodeType3[(NodeType3["TypeInferred"] = 124)] = "TypeInferred";
	NodeType3[(NodeType3["GenericTypeParameterDeclaration"] = 125)] = "GenericTypeParameterDeclaration";
	NodeType3[(NodeType3["ConstTypeParameterDeclaration"] = 126)] = "ConstTypeParameterDeclaration";
	NodeType3[(NodeType3["GenericLtParameterDeclaration"] = 127)] = "GenericLtParameterDeclaration";
	NodeType3[(NodeType3["WhereTypeBoundDeclaration"] = 128)] = "WhereTypeBoundDeclaration";
	NodeType3[(NodeType3["WhereLtBoundDeclaration"] = 129)] = "WhereLtBoundDeclaration";
	NodeType3[(NodeType3["TypeTraitBound"] = 130)] = "TypeTraitBound";
	NodeType3[(NodeType3["TypeDynBounds"] = 131)] = "TypeDynBounds";
	NodeType3[(NodeType3["TypeImplBounds"] = 132)] = "TypeImplBounds";
	NodeType3[(NodeType3["TypeFnPointer"] = 133)] = "TypeFnPointer";
	NodeType3[(NodeType3["TypeFnPointerParameter"] = 134)] = "TypeFnPointerParameter";
	NodeType3[(NodeType3["TypeFunction"] = 135)] = "TypeFunction";
	NodeType3[(NodeType3["TypeTuple"] = 136)] = "TypeTuple";
	NodeType3[(NodeType3["TypeSizedArray"] = 137)] = "TypeSizedArray";
	NodeType3[(NodeType3["TypeSlice"] = 138)] = "TypeSlice";
	NodeType3[(NodeType3["TypeReference"] = 139)] = "TypeReference";
	NodeType3[(NodeType3["TypeDereferenceConst"] = 140)] = "TypeDereferenceConst";
	NodeType3[(NodeType3["TypeDereferenceMut"] = 141)] = "TypeDereferenceMut";
	NodeType3[(NodeType3["TypeParenthesized"] = 142)] = "TypeParenthesized";
	return NodeType3;
})(NodeType || {});
var Feature = ((Feature3) => {
	Feature3["crate_visibility_modifier"] = "crate_visibility_modifier";
	Feature3["decl_macro"] = "decl_macro";
	Feature3["const_trait_impl"] = "const_trait_impl";
	Feature3["negative_impls"] = "negative_impls";
	Feature3["auto_traits"] = "auto_traits";
	Feature3["trait_alias"] = "trait_alias";
	Feature3["arbitrary_enum_discriminant"] = "arbitrary_enum_discriminant";
	Feature3["const_generics_defaults"] = "const_generics_defaults";
	Feature3["associated_type_bounds"] = "associated_type_bounds";
	Feature3["generators"] = "generators";
	Feature3["async_closure"] = "async_closure";
	Feature3["try_blocks"] = "try_blocks";
	Feature3["exclusive_range_pattern"] = "exclusive_range_pattern";
	Feature3["half_open_range_patterns"] = "half_open_range_patterns";
	Feature3["destructuring_assignment"] = "destructuring_assignment";
	Feature3["let_else"] = "let_else";
	Feature3["let_chains"] = "let_chains";
	Feature3["if_let_guard"] = "if_let_guard";
	Feature3["raw_ref_op"] = "raw_ref_op";
	Feature3["inline_const"] = "inline_const";
	Feature3["inline_const_pat"] = "inline_const_pat";
	Feature3["box_patterns"] = "box_patterns";
	Feature3["box_syntax"] = "box_syntax";
	return Feature3;
})(Feature || {});
var PRCD = ((PRCD2) => {
	PRCD2[(PRCD2["ScrutineeDefault"] = 0)] = "ScrutineeDefault";
	PRCD2[(PRCD2["Scrutinee ||"] = 1)] = "Scrutinee ||";
	PRCD2[(PRCD2["Scrutinee &&"] = 2)] = "Scrutinee &&";
	PRCD2[(PRCD2["Default"] = 3)] = "Default";
	PRCD2[(PRCD2["="] = 4)] = "=";
	PRCD2[(PRCD2[".."] = 5)] = "..";
	PRCD2[(PRCD2["||"] = 6)] = "||";
	PRCD2[(PRCD2["&&"] = 7)] = "&&";
	PRCD2[(PRCD2["=="] = 8)] = "==";
	PRCD2[(PRCD2["|"] = 9)] = "|";
	PRCD2[(PRCD2["^"] = 10)] = "^";
	PRCD2[(PRCD2["&"] = 11)] = "&";
	PRCD2[(PRCD2[">>"] = 12)] = ">>";
	PRCD2[(PRCD2["+-"] = 13)] = "+-";
	PRCD2[(PRCD2["*/%"] = 14)] = "*/%";
	PRCD2[(PRCD2["as"] = 15)] = "as";
	PRCD2[(PRCD2["Unary"] = 16)] = "Unary";
	PRCD2[(PRCD2["Top"] = 17)] = "Top";
	return PRCD2;
})(PRCD || {});
var TK = ((TK2) => {
	TK2[(TK2["None"] = 0)] = "None";
	TK2[(TK2["."] = 1)] = ".";
	TK2[(TK2["&&"] = 2)] = "&&";
	TK2[(TK2["||"] = 3)] = "||";
	TK2[(TK2["="] = 4)] = "=";
	TK2[(TK2["+"] = 5)] = "+";
	TK2[(TK2["-"] = 6)] = "-";
	TK2[(TK2["*"] = 7)] = "*";
	TK2[(TK2["/"] = 8)] = "/";
	TK2[(TK2["%"] = 9)] = "%";
	TK2[(TK2["&"] = 10)] = "&";
	TK2[(TK2["|"] = 11)] = "|";
	TK2[(TK2["^"] = 12)] = "^";
	TK2[(TK2["<<"] = 13)] = "<<";
	TK2[(TK2[">>"] = 14)] = ">>";
	TK2[(TK2["=="] = 15)] = "==";
	TK2[(TK2["!="] = 16)] = "!=";
	TK2[(TK2[">"] = 17)] = ">";
	TK2[(TK2[">="] = 18)] = ">=";
	TK2[(TK2["<"] = 19)] = "<";
	TK2[(TK2["<="] = 20)] = "<=";
	TK2[(TK2["+="] = 21)] = "+=";
	TK2[(TK2["-="] = 22)] = "-=";
	TK2[(TK2["*="] = 23)] = "*=";
	TK2[(TK2["/="] = 24)] = "/=";
	TK2[(TK2["%="] = 25)] = "%=";
	TK2[(TK2["&="] = 26)] = "&=";
	TK2[(TK2["|="] = 27)] = "|=";
	TK2[(TK2["^="] = 28)] = "^=";
	TK2[(TK2["<<="] = 29)] = "<<=";
	TK2[(TK2[">>="] = 30)] = ">>=";
	TK2[(TK2["$"] = 31)] = "$";
	TK2[(TK2["@"] = 32)] = "@";
	TK2[(TK2["_"] = 33)] = "_";
	TK2[(TK2[".."] = 34)] = "..";
	TK2[(TK2["..."] = 35)] = "...";
	TK2[(TK2["..="] = 36)] = "..=";
	TK2[(TK2[","] = 37)] = ",";
	TK2[(TK2[";"] = 38)] = ";";
	TK2[(TK2[":"] = 39)] = ":";
	TK2[(TK2["::"] = 40)] = "::";
	TK2[(TK2["#"] = 41)] = "#";
	TK2[(TK2["?"] = 42)] = "?";
	TK2[(TK2["!"] = 43)] = "!";
	TK2[(TK2["=>"] = 44)] = "=>";
	TK2[(TK2["->"] = 45)] = "->";
	TK2[(TK2["~"] = 46)] = "~";
	return TK2;
})(TK || {});
var str_TK = [
	"",
	".",
	"&&",
	"||",
	"=",
	"+",
	"-",
	"*",
	"/",
	"%",
	"&",
	"|",
	"^",
	"<<",
	">>",
	"==",
	"!=",
	">",
	">=",
	"<",
	"<=",
	"+=",
	"-=",
	"*=",
	"/=",
	"%=",
	"&=",
	"|=",
	"^=",
	"<<=",
	">>=",
	"$",
	"@",
	"_",
	"..",
	"...",
	"..=",
	",",
	";",
	":",
	"::",
	"#",
	"?",
	"!",
	"=>",
	"->",
	"~",
];
var DelimKind = ((DelimKind3) => {
	DelimKind3[(DelimKind3["None"] = 0)] = "None";
	DelimKind3[(DelimKind3["()"] = 1)] = "()";
	DelimKind3[(DelimKind3["[]"] = 2)] = "[]";
	DelimKind3[(DelimKind3["{}"] = 3)] = "{}";
	DelimKind3[(DelimKind3["<>"] = 4)] = "<>";
	DelimKind3[(DelimKind3["||"] = 5)] = "||";
	return DelimKind3;
})(DelimKind || {});
var TyMacroMatch = ((TyMacroMatch2) => {
	TyMacroMatch2[(TyMacroMatch2["None"] = 0)] = "None";
	TyMacroMatch2[(TyMacroMatch2["tt"] = 1)] = "tt";
	TyMacroMatch2[(TyMacroMatch2["meta"] = 2)] = "meta";
	TyMacroMatch2[(TyMacroMatch2["vis"] = 3)] = "vis";
	TyMacroMatch2[(TyMacroMatch2["ident"] = 4)] = "ident";
	TyMacroMatch2[(TyMacroMatch2["lifetime"] = 5)] = "lifetime";
	TyMacroMatch2[(TyMacroMatch2["literal"] = 6)] = "literal";
	TyMacroMatch2[(TyMacroMatch2["stmt"] = 7)] = "stmt";
	TyMacroMatch2[(TyMacroMatch2["item"] = 8)] = "item";
	TyMacroMatch2[(TyMacroMatch2["expr"] = 9)] = "expr";
	TyMacroMatch2[(TyMacroMatch2["block"] = 10)] = "block";
	TyMacroMatch2[(TyMacroMatch2["pat"] = 11)] = "pat";
	TyMacroMatch2[(TyMacroMatch2["pat_param"] = 12)] = "pat_param";
	TyMacroMatch2[(TyMacroMatch2["ty"] = 13)] = "ty";
	TyMacroMatch2[(TyMacroMatch2["path"] = 14)] = "path";
	return TyMacroMatch2;
})(TyMacroMatch || {});
var LiteralKind = ((LiteralKind2) => {
	LiteralKind2[(LiteralKind2["False"] = 0)] = "False";
	LiteralKind2[(LiteralKind2["True"] = 1)] = "True";
	LiteralKind2[(LiteralKind2["Char"] = 2)] = "Char";
	LiteralKind2[(LiteralKind2["bChar"] = 3)] = "bChar";
	LiteralKind2[(LiteralKind2["bString"] = 4)] = "bString";
	LiteralKind2[(LiteralKind2["brString"] = 5)] = "brString";
	LiteralKind2[(LiteralKind2["rString"] = 6)] = "rString";
	LiteralKind2[(LiteralKind2["String"] = 7)] = "String";
	LiteralKind2[(LiteralKind2["Binary"] = 8)] = "Binary";
	LiteralKind2[(LiteralKind2["Hex"] = 9)] = "Hex";
	LiteralKind2[(LiteralKind2["Octal"] = 10)] = "Octal";
	LiteralKind2[(LiteralKind2["Integer"] = 11)] = "Integer";
	LiteralKind2[(LiteralKind2["Float"] = 12)] = "Float";
	return LiteralKind2;
})(LiteralKind || {});
class Loc {
	constructor(src2, start2, end2) {
		this.src = src2;
		this[0] = start2;
		this[1] = end2;
	}
	len() {
		return this[1] - this[0];
	}
	isBefore(target) {
		return this[1] <= start(target);
	}
	isAfter(target) {
		return this[0] >= end(target);
	}
	contains(target) {
		return this[0] <= start(target) && this[1] >= end(target);
	}
	ownContains(target) {
		return (2 in this ? this[2] : this[0]) <= start(target) && this[1] >= end(target);
	}
	isBetween(left, right) {
		return this[0] >= end(left) && this[1] <= start(right);
	}
	url() {
		return this.src.url(this[0]);
	}
	getText() {
		return this.src.code.slice(this[0], this[1]);
	}
	getOwnText() {
		return this.src.code.slice(2 in this ? this[2] : this[0], this[1]);
	}
	sliceText(startIndex, endIndex) {
		return this.src.code.slice(this[0], this[1]).slice(startIndex, endIndex);
	}
	clone() {
		return new Loc(this.src, 2 in this ? this[2] : this[0], this[1]);
	}
	cloneFrom(startPos) {
		return new Loc(this.src, startPos, this[1]);
	}
	[Symbol.for("nodejs.util.inspect.custom")]() {
		return this.url();
	}
	toJSON() {
		const { src: src2, ...rest } = this;
		return rest;
	}
}
class BaseNode2 {
	get type() {
		return NodeType[this.nodeType];
	}
	constructor(nodeType, loc) {
		this.nodeType = nodeType;
		this.loc = loc;
	}
	toJSON() {
		var obj = { type: NodeType[this.nodeType], ...this };
		if ("name" in this) obj.name = this.name;
		if ("value" in this) obj.value = this.value;
		if ("kind" in this) obj.kind = this.kind;
		if ("token" in this) obj.token = this.token;
		return obj;
	}
}
class MissingNode extends BaseNode2 {
	get name() {
		return "";
	}
	constructor() {
		super(0, new Loc(GET_SOURCE(), getPreWhitespaceSkipPosition(), getPreWhitespaceSkipPosition()));
	}
}
class SourceFile extends BaseNode2 {
	l(index) {
		return getLineIndex(this.lineStarts, index);
	}
	lc(index) {
		return getLineChar(this.lineStarts, index);
	}
	url(index) {
		return urlAt(this.filepath ?? "undefined", this.lineStarts, index);
	}
	constructor(_code, _options) {
		super(1, new Loc(void 0, 0, _code.length));
		this.loc.src = this;
		this.code = _code;
		this.filepath = _options.filepath;
		this.parserOptions = _options;
		this.lineStarts = [0];
		withParserState(this, 0, () => {
			if (maybe_read(CharCode_UTF8BOM)) {
				this.UTF8BOM = true;
			}
			if (match_2(35, 33) && peek_not_match(2, 91)) {
				this.shebang = new Shebang();
			}
			this.program = new Program();
			return this.program;
		});
	}
}
class Shebang extends BaseNode2 {
	get value() {
		return this.loc.sliceText(2).trim();
	}
	constructor() {
		super(2, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_step_over_2();
		step_until_ln();
		__endRange(this);
	}
}
class Program extends BaseNode2 {
	get body() {
		return this.ast;
	}
	constructor() {
		super(3, new Loc(GET_SOURCE(), skip_whitespace_getProgramStartPos(), 0));
		read_top_statements(this);
		setRangeEnd(this, getProgramEndPos(this));
	}
}
class Snippet extends BaseNode2 {
	constructor(target, READ_SNIPPET) {
		super(4, target.loc.clone());
		READ_SNIPPET(this);
		SNIPPET_endAt(end(this));
	}
}
class Comment extends BaseNode2 {
	get value() {
		return this.loc.sliceText(2, this.line ? void 0 : -2);
	}
	constructor() {
		super(5, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.line = __is_line;
		read_comment();
		setRangeEnd(this, GET_POSITION());
	}
}
class Identifier extends BaseNode2 {
	get name() {
		return this.loc.getOwnText();
	}
	constructor() {
		super(6, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		switch (peek_keyword()) {
			case 0:
			case 2:
			case 5:
				exit3.expected("Identifier");
		}
		read_cached_keyword();
		__endRange(this);
	}
}
class Index extends BaseNode2 {
	get name() {
		return this.loc.getOwnText();
	}
	constructor() {
		super(7, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		do step();
		while (is_number(currChar()));
		__endRange(this);
	}
}
class LbIdentifier extends BaseNode2 {
	get name() {
		return this.loc.getOwnText();
	}
	constructor() {
		super(8, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		read_identifier_with();
		__endRange(this);
	}
}
class McIdentifier extends BaseNode2 {
	get name() {
		return this.loc.getOwnText();
	}
	constructor() {
		super(9, new Loc(GET_SOURCE(), GET_POSITION() - 1, 0));
		read_XID_CONTINUE();
		__endRange(this);
	}
}
class Literal extends BaseNode2 {
	get value() {
		return "suffix" in this ? this.loc.src.code.slice(ownStart(this), start(this.suffix)) : this.loc.getOwnText();
	}
	constructor() {
		super(10, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.kind = read_literal();
		switch (this.kind) {
			case 8:
			case 9:
			case 10:
			case 11:
			case 12:
				if (is_XID_Start(currChar())) {
					this.suffix = new Identifier();
				}
		}
		__endRange(this);
	}
}
class ItemPath extends BaseNode2 {
	constructor(namespace) {
		super(11, new Loc(GET_SOURCE(), void 0 === namespace ? GET_POSITION() : start(namespace), 0));
		this.namespace = namespace;
		this.segment = new Identifier();
		__endRange(this);
	}
}
class PunctuationToken extends BaseNode2 {
	get token() {
		return str_TK[this.tk];
	}
	constructor() {
		super(12, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.tk = ptTree();
		__endRange(this);
	}
}
class DelimGroup extends BaseNode2 {
	constructor() {
		super(13, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.segments = read_segments();
		__endRange(this);
	}
}
class Attribute extends BaseNode2 {
	get value() {
		return this.segments.loc.sliceText(1, -1);
	}
	get line() {
		return this.inner;
	}
	constructor() {
		super(14, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		const ws_preskip = _get_ws_preskip_pos();
		safe_skip();
		this.inner = maybe_read(33);
		this.segments = read_segments_withEnv(1, 2);
		setRangeEnd(this, GET_POSITION());
		setPreSkipWhitespace(ws_preskip);
	}
}
class DocCommentAttribute extends BaseNode2 {
	get value() {
		return this.loc.sliceText(3, this.line ? void 0 : -2);
	}
	get segments() {
		const loc = new Loc(this.loc.src, start(this) + 3, end(this) + (this.line ? 0 : -2));
		return createLocArray(0, loc, [mockNode(10, loc.clone(), { kind: 7, value: this.value })]);
	}
	constructor() {
		super(15, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.inner = __is_inner;
		this.line = __is_line;
		read_comment();
		setRangeEnd(this, GET_POSITION());
	}
}
class MacroInvocation extends BaseNode2 {
	constructor(callee) {
		super(16, new Loc(GET_SOURCE(), start(callee), 0));
		this.callee = callee;
		safe_skip();
		this.segments = read_segments_withEnv(2);
		__endRange(this);
	}
}
class MacroRulesDeclaration extends BaseNode2 {
	constructor() {
		super(17, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword(), safe_skip();
		this.id = read_identifier_token();
		switch (currChar()) {
			case 123:
				this.rules = read_rules(3);
				break;
			case 40:
				this.rules = read_rules(1);
				maybe_read(59);
				break;
			case 91:
				this.rules = read_rules(2);
				maybe_read(59);
				break;
			default:
				exit3.expected([123, 40, 91]);
		}
		__endRange(this);
	}
}
class MacroDeclaration extends BaseNode2 {
	constructor() {
		super(18, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.id = new Identifier();
		switch (currChar()) {
			case 40:
				this.rules = new MacroInlineRuleDeclaration();
				break;
			case 123:
				this.rules = read_rules(3);
				break;
			default:
				exit3.expected([40, 123]);
		}
		__endRange(this);
	}
}
class MacroRuleDeclaration extends BaseNode2 {
	constructor() {
		super(19, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		read_rule(this);
		__endRange(this);
	}
}
class MacroInlineRuleDeclaration extends BaseNode2 {
	constructor() {
		super(20, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		read_rule(this);
		__endRange(this);
	}
}
class MacroGroup extends BaseNode2 {
	constructor() {
		super(21, new Loc(GET_SOURCE(), GET_POSITION() - 1, 0));
		this.segments = read_segments(1);
		this.sep = maybe_read_sep();
		switch (currChar()) {
			case 42:
				this.kind = "*";
				safe_skip();
				break;
			case 43:
				this.kind = "+";
				safe_skip();
				break;
			case 63:
				this.kind = "?";
				safe_skip();
				break;
			default:
				exit3.expected([42, 43, 63]);
		}
		__endRange(this);
	}
}
class MacroParameterDeclaration extends BaseNode2 {
	constructor() {
		super(22, new Loc(GET_SOURCE(), GET_POSITION() - 1, 0));
		this.id = new McIdentifier();
		this.ty = read_maybe_missing(() => (maybe_read(58) ? read_identifier_token() : void 0));
		__endRange(this);
	}
}
class PubSpecifier extends BaseNode2 {
	constructor() {
		super(23, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		switch (peek_keyword()) {
			case 25:
				safe_skip_keyword();
				if (maybe_read(40)) {
					switch (peek_keyword()) {
						case 27:
							safe_skip_keyword();
							this.location = read_Identifier_or_ItemPath_unbound();
							read(41);
							break;
						case 43:
						case 42:
						case 45:
							this.location = new Identifier();
							read(41);
							break;
						default:
							EDGECASE_STEPBACK_TO_POS(start(this) + "pub".length);
							this.location = void 0;
							break;
					}
				} else {
					this.location = void 0;
				}
				break;
			case 45:
				this.location = new Identifier();
				break;
		}
		__endRange(this);
	}
}
class ExternSpecifier extends BaseNode2 {
	constructor(abi) {
		super(24, new Loc(GET_SOURCE(), 0, 0));
		this.abi = abi;
		__endRange(this);
	}
}
class ExpressionStatement extends BaseNode2 {
	constructor() {
		super(25, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		if (maybe_read(59)) {
			this.expression = void 0;
			this.semi = true;
		} else {
			this.expression = read_stmt_expression();
			this.semi = maybe_read(59);
		}
		__endRange(this);
	}
}
class UseStatement extends BaseNode2 {
	constructor() {
		super(26, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.import = read_import();
		maybe_read(59);
		__endRange(this);
	}
}
class NamedImport extends BaseNode2 {
	constructor(source, local) {
		super(27, new Loc(GET_SOURCE(), start(source), 0));
		this.source = source;
		this.local = local;
		__endRange(this);
	}
}
class AmbientImport extends BaseNode2 {
	constructor(source) {
		super(28, new Loc(GET_SOURCE(), void 0 === source ? GET_POSITION() : start(source), 0));
		this.source = source;
		safe_skip();
		__endRange(this);
	}
}
class AnonymousImport extends BaseNode2 {
	constructor(source) {
		super(29, new Loc(GET_SOURCE(), start(source), 0));
		this.source = source;
		safe_skip_keyword();
		__endRange(this);
	}
}
class DestructuredImport extends BaseNode2 {
	constructor(source) {
		super(30, new Loc(GET_SOURCE(), void 0 === source ? GET_POSITION() : start(source), 0));
		this.source = source;
		this.specifiers = read_sequence(3, () => read_import());
		__endRange(this);
	}
}
class ExternCrateStatement extends BaseNode2 {
	constructor() {
		super(31, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.import = read_import();
		maybe_read(59);
		__endRange(this);
	}
}
class ExternBlockDeclaration extends BaseNode2 {
	constructor(abi) {
		super(32, new Loc(GET_SOURCE(), 0, 0));
		this.abi = abi;
		read_body(this);
		__endRange(this);
	}
}
class TypeAliasDeclaration extends BaseNode2 {
	constructor() {
		super(33, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.id = new Identifier();
		this.generics = maybe_read_generics();
		this.typeBounds = maybe_read_colon_typeBounds();
		this.whereBounds = maybe_read_whereBounds();
		this.typeExpression = maybe_read(61) ? read_type(true) : void 0;
		maybe_read(59);
		__endRange(this);
	}
}
class ConstVariableDeclaration extends BaseNode2 {
	constructor() {
		super(34, new Loc(GET_SOURCE(), 0, 0));
		this.pattern = read_PatternNoUnion_unstrict();
		this.typeAnnotation = read_maybe_missing(() => maybe_read_typeAnnotation());
		this.expression = maybe_read(61) ? read_contained_expr_in_stmt() : void 0;
		maybe_read(59);
		__endRange(this);
	}
}
class StaticVariableDeclaration extends BaseNode2 {
	constructor() {
		super(35, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.pattern = read_PatternNoUnion_unstrict();
		this.typeAnnotation = read_maybe_missing(() => maybe_read_typeAnnotation());
		this.expression = maybe_read(61) ? read_contained_expr_in_stmt() : void 0;
		maybe_read(59);
		__endRange(this);
	}
}
class LetVariableDeclaration extends BaseNode2 {
	constructor() {
		super(36, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.pattern = read_PatternNoUnion_unstrict();
		this.typeAnnotation = maybe_read_typeAnnotation();
		if (maybe_read(61)) {
			this.expression = FG_with_outerAttributes_fromParentContext2(() => read_contained_expr_in_stmt());
			this.else = FG_property("let_else", this, "else", () => (maybe_read_keyword(38) ? new BlockExpression() : void 0));
		} else {
			this.expression = void 0;
		}
		maybe_read(59);
		__endRange(this);
	}
}
class ModuleDeclaration extends BaseNode2 {
	constructor() {
		super(37, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.id = new Identifier();
		maybe_read_body(this);
		__endRange(this);
	}
}
class FunctionDeclaration extends BaseNode2 {
	toJSON() {
		return { "parameters.self": this.parameters.self, ...this };
	}
	constructor() {
		super(38, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.id = new Identifier();
		this.generics = maybe_read_generics();
		this.parameters = readLocatedArrayDelim(1, (parameters) => {
			read(40);
			if (maybe_read(41)) {
				parameters.self = void 0;
			} else {
				with_outerAttributes_fromParentContext(() =>
					read_ahead_either(
						() => {
							const ref = maybe_read(38);
							const lt = ref ? maybe_read_lifetime() : void 0;
							const mut = maybe_read_keyword(31);
							return (parameters.self = match_keyword(43) ? new FunctionSelfParameterDeclaration(ref, lt, mut) : void 0);
						},
						() => (parameters[0] = read_function_parameter())
					)
				);
				FOR_EACH_UNTIL(44, () => parameters.push(with_outerAttributes_fromParentContext(() => read_function_parameter())), 41);
			}
		});
		this.returnType = maybe_read_ReturnType(true);
		this.whereBounds = maybe_read_whereBounds();
		maybe_read_body(this);
		__endRange(this);
	}
}
class FunctionSelfParameterDeclaration extends BaseNode2 {
	constructor(ref, lt, mut) {
		super(39, new Loc(GET_SOURCE(), 0, 0));
		safe_skip_keyword();
		this.ref = ref;
		this.lt = lt;
		this.mut = mut;
		this.typeAnnotation = maybe_read_typeAnnotation();
		__endRange(this);
	}
}
class FunctionParameterDeclaration extends BaseNode2 {
	constructor() {
		super(40, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.pattern = read_PatternNoUnion_unstrict();
		this.typeAnnotation = read_maybe_missing(() => (maybe_read(58) ? (match(46) ? new FunctionSpread() : read_type(true)) : void 0));
		__endRange(this);
	}
}
class FunctionSpread extends BaseNode2 {
	constructor() {
		super(41, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		step_over_3(46, 46, 46);
		__endRange(this);
	}
}
class StructDeclaration extends BaseNode2 {
	constructor(id, generics) {
		super(42, new Loc(GET_SOURCE(), 0, 0));
		this.id = id;
		this.generics = generics;
		this.whereBounds = maybe_read_whereBounds();
		this.properties = match(123) ? read_struct_properties_declaration() : (maybe_read(59), void 0);
		__endRange(this);
	}
}
class TupleStructDeclaration extends BaseNode2 {
	constructor(id, generics) {
		super(43, new Loc(GET_SOURCE(), 0, 0));
		this.id = id;
		this.generics = generics;
		this.items = read_struct_items_declaration();
		this.whereBounds = maybe_read_whereBounds();
		maybe_read(59);
		__endRange(this);
	}
}
class StructPropertyDeclaration extends BaseNode2 {
	constructor() {
		super(44, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.id = new Identifier();
		this.typeAnnotation = read_maybe_missing(() => maybe_read_typeAnnotation());
		__endRange(this);
	}
}
class TupleStructItemDeclaration extends BaseNode2 {
	constructor() {
		super(45, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.typeAnnotation = read_type(true);
		__endRange(this);
	}
}
class UnionDeclaration extends BaseNode2 {
	constructor() {
		super(46, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.id = new Identifier();
		this.generics = maybe_read_generics();
		this.whereBounds = maybe_read_whereBounds();
		this.properties = read_struct_properties_declaration();
		__endRange(this);
	}
}
class EnumDeclaration extends BaseNode2 {
	constructor() {
		super(47, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.id = new Identifier();
		this.generics = maybe_read_generics();
		this.whereBounds = maybe_read_whereBounds();
		this.members = read_sequence(3, () =>
			maybe_read_with_attr_pub(() =>
				read_ahead(() => {
					const id = new Identifier();
					switch (currChar()) {
						case 40:
							return new EnumMemberTupleDeclaration(id);
						case 123:
							return new EnumMemberStructDeclaration(id);
						default:
							return new EnumMemberDeclaration(id);
					}
				})
			)
		);
		__endRange(this);
	}
}
class EnumMemberDeclaration extends BaseNode2 {
	constructor(id) {
		super(48, new Loc(GET_SOURCE(), start(id), 0));
		this.id = id;
		this.value = maybe_read(61) ? read_contained_expr_in_stmt() : void 0;
		__endRange(this);
	}
}
class EnumMemberTupleDeclaration extends BaseNode2 {
	constructor(id) {
		super(49, new Loc(GET_SOURCE(), start(id), 0));
		this.id = id;
		this.items = read_struct_items_declaration();
		this.value = FG_property("arbitrary_enum_discriminant", this, "value", () =>
			maybe_read(61) ? read_contained_expr_in_stmt() : void 0
		);
		__endRange(this);
	}
}
class EnumMemberStructDeclaration extends BaseNode2 {
	constructor(id) {
		super(50, new Loc(GET_SOURCE(), start(id), 0));
		this.id = id;
		this.properties = read_struct_properties_declaration();
		this.value = FG_property("arbitrary_enum_discriminant", this, "value", () =>
			maybe_read(61) ? read_contained_expr_in_stmt() : void 0
		);
		__endRange(this);
	}
}
class TraitDeclaration extends BaseNode2 {
	constructor(id, generics) {
		super(51, new Loc(GET_SOURCE(), 0, 0));
		this.id = id;
		this.generics = generics;
		this.typeBounds = maybe_read_colon_typeBounds();
		this.whereBounds = maybe_read_whereBounds();
		read_body(this);
		__endRange(this);
	}
}
class AutoTraitDeclaration extends BaseNode2 {
	constructor() {
		super(52, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		read_keyword(13);
		this.id = new Identifier();
		read_body_noBody(this);
		__endRange(this);
	}
}
class TraitAliasDeclaration extends BaseNode2 {
	constructor(id, generics) {
		super(53, new Loc(GET_SOURCE(), 0, 0));
		this.id = id;
		this.generics = generics;
		safe_skip();
		this.typeBounds = match_keyword(32) || match(59) ? [] : read_typeBounds();
		this.whereBounds = maybe_read_whereBounds();
		__endRange(this);
	}
}
class ImplDeclaration extends BaseNode2 {
	constructor(generics) {
		super(54, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		if (_impl_startsWith_selector) {
			this.generics = void 0;
			this.const = false;
			this.trait = void 0;
			this.typeTarget = read_type(true);
		} else {
			this.generics = generics;
			if ((this.const = FG_property("const_trait_impl", this, "const", () => maybe_read_keyword(20)))) {
				this.trait = read_TypeNamespaceTargetNoSelector();
				this.typeTarget = (read_keyword(30), read_type(true));
			} else {
				const ty = read_type(true);
				if (maybe_read_keyword(30)) {
					this.trait = ty;
					this.typeTarget = read_type(true);
				} else {
					this.trait = void 0;
					this.typeTarget = ty;
				}
			}
		}
		this.whereBounds = maybe_read_whereBounds();
		read_body(this);
		__endRange(this);
	}
}
class NegativeImplDeclaration extends BaseNode2 {
	constructor(generics) {
		super(55, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.generics = generics;
		safe_skip();
		this.trait = read_TypeNamespaceTargetNoSelector();
		this.typeTarget = (read_keyword(30), read_type(true));
		this.whereBounds = maybe_read_whereBounds();
		read_body_noBody(this);
		__endRange(this);
	}
}
class ExpressionTypeSelector extends BaseNode2 {
	constructor() {
		super(56, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip();
		this.typeTarget = read_type(true);
		this.typeExpression = maybe_read_keyword(26) ? read_type(true) : void 0;
		read(62);
		__endRange(this);
	}
}
class ExpressionTypeCast extends BaseNode2 {
	constructor(typeCallee, typeArguments) {
		super(57, new Loc(GET_SOURCE(), start(typeCallee), 0));
		this.typeCallee = typeCallee;
		this.typeArguments = typeArguments;
		__endRange(this);
	}
}
class ExpressionPath extends BaseNode2 {
	constructor(namespace) {
		super(58, new Loc(GET_SOURCE(), void 0 === namespace ? GET_POSITION() : start(namespace), 0));
		this.namespace = namespace;
		this.segment = new Identifier();
		__endRange(this);
	}
}
class ExpressionAsTypeCast extends BaseNode2 {
	constructor(expression) {
		super(59, new Loc(GET_SOURCE(), start(expression), 0));
		this.expression = expression;
		this.typeExpression = read_type(false);
		__endRange(this);
	}
}
class ReturnExpression extends BaseNode2 {
	constructor() {
		super(60, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.expression = maybe_read_expression_rhs();
		__endRange(this);
	}
}
class BreakExpression extends BaseNode2 {
	constructor() {
		super(61, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		if (will_match_lt()) {
			const label = new LbIdentifier();
			if (match(58) && peek_not_match(1, 58)) {
				this.label = void 0;
				this.expression = read_labelled_block(label);
			} else {
				this.label = label;
				this.expression = maybe_read_expression_rhs();
			}
		} else {
			this.label = void 0;
			this.expression = maybe_read_expression_rhs();
		}
		__endRange(this);
	}
}
class ContinueExpression extends BaseNode2 {
	constructor() {
		super(62, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.label = match(39) ? new LbIdentifier() : void 0;
		__endRange(this);
	}
}
class YieldExpression extends BaseNode2 {
	constructor() {
		super(63, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.expression = maybe_read_expression_rhs();
		__endRange(this);
	}
}
class CallExpression extends BaseNode2 {
	constructor(callee, method, typeArguments) {
		super(64, new Loc(GET_SOURCE(), start(callee), 0));
		this.callee = callee;
		this.method = method;
		this.typeArguments = typeArguments;
		this.arguments = read_sequence(1, () => with_outerAttributes_fromParentContext(() => read_contained_expression(false)));
		__endRange(this);
	}
}
class MemberExpression extends BaseNode2 {
	constructor(expression, computed, property) {
		super(65, new Loc(GET_SOURCE(), start(expression), 0));
		this.expression = expression;
		this.computed = computed;
		this.property = property;
		__endRange(this);
	}
}
class AwaitExpression extends BaseNode2 {
	constructor(expression) {
		super(66, new Loc(GET_SOURCE(), start(expression), 0));
		this.expression = expression;
		safe_skip_keyword();
		__endRange(this);
	}
}
class UnwrapExpression extends BaseNode2 {
	constructor(expression) {
		super(67, new Loc(GET_SOURCE(), start(expression), 0));
		this.expression = expression;
		safe_skip();
		__endRange(this);
	}
}
class ParenthesizedExpression extends BaseNode2 {
	constructor(items) {
		super(68, new Loc(GET_SOURCE(), start(items), end(items)));
		this.expression = items[0];
	}
}
class MinusExpression extends BaseNode2 {
	constructor(read_expression2) {
		super(69, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip();
		this.expression = read_expression2();
		__endRange(this);
	}
}
class NotExpression extends BaseNode2 {
	constructor() {
		super(70, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip();
		this.expression = read_unary_rhs();
		__endRange(this);
	}
}
class OrExpression extends BaseNode2 {
	get kind() {
		return str_TK[this.tk];
	}
	constructor(left) {
		super(71, new Loc(GET_SOURCE(), start(left), 0));
		this.left = left;
		this.tk = 3;
		this.right = read_expression_rhs();
		__endRange(this);
	}
}
class AndExpression extends BaseNode2 {
	get kind() {
		return str_TK[this.tk];
	}
	constructor(left) {
		super(72, new Loc(GET_SOURCE(), start(left), 0));
		this.left = left;
		this.tk = 2;
		this.right = read_expression_rhs();
		__endRange(this);
	}
}
class ReassignmentExpression extends BaseNode2 {
	get kind() {
		return str_TK[this.tk];
	}
	constructor(left) {
		super(73, new Loc(GET_SOURCE(), start(left), 0));
		this.left = left;
		this.tk = 4;
		this.right = read_expression_rhs();
		__endRange(this);
	}
}
class UnassignedExpression extends BaseNode2 {
	constructor() {
		super(74, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		__endRange(this);
	}
}
class OperationExpression extends BaseNode2 {
	get kind() {
		return str_TK[this.tk];
	}
	constructor(left, tk) {
		super(75, new Loc(GET_SOURCE(), start(left), 0));
		this.left = left;
		this.tk = tk;
		this.right = read_expression_rhs();
		__endRange(this);
	}
}
class ReassignmentOperationExpression extends BaseNode2 {
	get kind() {
		return str_TK[this.tk];
	}
	constructor(left, tk) {
		super(76, new Loc(GET_SOURCE(), start(left), 0));
		this.left = left;
		this.tk = tk;
		this.right = read_expression_rhs();
		__endRange(this);
	}
}
class ComparisonExpression extends BaseNode2 {
	get kind() {
		return str_TK[this.tk];
	}
	constructor(left, tk) {
		super(77, new Loc(GET_SOURCE(), start(left), 0));
		this.left = left;
		this.tk = tk;
		this.right = read_expression_rhs();
		__endRange(this);
	}
}
class LetScrutinee extends BaseNode2 {
	constructor() {
		super(78, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.pattern = read_PatternNoUnion_unstrict();
		read(61);
		this.expression = read_scrutinee_rhs();
		__endRange(this);
	}
}
class ClosureFunctionExpression extends BaseNode2 {
	constructor() {
		super(79, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.parameters = read_sequence(5, () => with_outerAttributes_fromParentContext(() => new ClosureFunctionParameterDeclaration()));
		this.returnType = maybe_read_ReturnType(false);
		this.expression = read_closure_rhs();
		__endRange(this);
	}
}
class ClosureFunctionParameterDeclaration extends BaseNode2 {
	constructor() {
		super(80, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.pattern = read_pattern(false);
		this.typeAnnotation = maybe_read_typeAnnotation();
		__endRange(this);
	}
}
class BlockExpression extends BaseNode2 {
	constructor() {
		super(81, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.label = void 0;
		read_body(this);
		__endRange(this);
	}
}
class LoopBlockExpression extends BaseNode2 {
	constructor() {
		super(82, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.label = void 0;
		safe_skip_keyword();
		read_body(this);
		__endRange(this);
	}
}
class WhileBlockExpression extends BaseNode2 {
	constructor() {
		super(83, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.label = void 0;
		safe_skip_keyword();
		this.condition = read_ConditionExpressionNode();
		read_body(this);
		__endRange(this);
	}
}
class ForInBlockExpression extends BaseNode2 {
	constructor() {
		super(84, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.label = void 0;
		safe_skip_keyword();
		this.pattern = read_pattern(true);
		read_keyword(27);
		this.expression = read_contained_expression(true);
		read_body(this);
		__endRange(this);
	}
}
class TryBlockExpression extends BaseNode2 {
	constructor() {
		super(85, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		read_body(this);
		__endRange(this);
	}
}
class IfBlockExpression extends BaseNode2 {
	constructor() {
		super(86, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.label = void 0;
		safe_skip_keyword();
		this.condition = read_ConditionExpressionNode();
		read_body(this);
		this.else = maybe_read_keyword(38) ? (match_keyword(37) ? new IfBlockExpression() : new BlockExpression()) : void 0;
		__endRange(this);
	}
}
class MatchExpression extends BaseNode2 {
	constructor() {
		super(87, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.expression = read_contained_expression(true);
		read_group(this, 3, () => {
			const match_case = new MatchExpressionCase();
			maybe_read(44);
			return match_case;
		});
		__endRange(this);
	}
}
class MatchExpressionCase extends BaseNode2 {
	constructor() {
		super(88, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.pattern = read_pattern(true);
		this.condition = maybe_read_keyword(37) ? read_contained_expression(false, true) : void 0;
		read_2(61, 62);
		this.expression = read_stmt_expression();
		__endRange(this);
	}
}
class RangeLiteral extends BaseNode2 {
	constructor(lower, last) {
		super(89, new Loc(GET_SOURCE(), void 0 === lower ? GET_POSITION() : start(lower), 0));
		this.lower = lower;
		this.last = last;
		this.upper = maybe_read_expression_rhs();
		__endRange(this);
	}
}
class StructLiteral extends BaseNode2 {
	constructor(struct) {
		super(90, new Loc(GET_SOURCE(), start(struct), 0));
		this.struct = struct;
		this.properties = read_sequence(3, () =>
			with_outerAttributes_fromParentContext_if_test(
				() => {
					if (match(46)) {
						return read_ahead(() => {
							safe_skip_1_read_2(46, 46);
							return might_read_expression() ? new StructLiteralPropertySpread() : new StructLiteralRestUnassigned();
						});
					} else {
						const id = read_Identifier_or_Index();
						return match(58) ? new StructLiteralProperty(id) : new StructLiteralPropertyShorthand(id);
					}
				},
				(node) => {
					switch (node.nodeType) {
						case 91:
						case 92:
							return true;
						case 93:
						case 94:
							return false;
					}
				}
			)
		);
		__endRange(this);
	}
}
class StructLiteralProperty extends BaseNode2 {
	constructor(key) {
		super(91, new Loc(GET_SOURCE(), start(key), 0));
		this.key = key;
		safe_skip();
		this.value = read_contained_expression(false);
		__endRange(this);
	}
}
class StructLiteralPropertyShorthand extends BaseNode2 {
	constructor(value) {
		super(92, new Loc(GET_SOURCE(), start(value), 0));
		this.value = value;
		__endRange(this);
	}
}
class StructLiteralPropertySpread extends BaseNode2 {
	constructor() {
		super(93, new Loc(GET_SOURCE(), 0, 0));
		this.expression = read_contained_expression(false);
		__endRange(this);
	}
}
class StructLiteralRestUnassigned extends BaseNode2 {
	constructor() {
		super(94, new Loc(GET_SOURCE(), 0, 0));
		__endRange(this);
	}
}
class TupleLiteral extends BaseNode2 {
	constructor(items) {
		super(95, new Loc(GET_SOURCE(), start(items), end(items)));
		this.items = items;
	}
}
class ArrayLiteral extends BaseNode2 {
	constructor(items) {
		super(96, new Loc(GET_SOURCE(), start(items), end(items)));
		this.items = items;
	}
}
class SizedArrayLiteral extends BaseNode2 {
	constructor(items) {
		super(97, new Loc(GET_SOURCE(), start(items), end(items)));
		this.initExpression = items[0];
		this.sizeExpression = items[1];
	}
}
class ReferenceExpression extends BaseNode2 {
	constructor() {
		super(98, new Loc(GET_SOURCE(), 0, 0));
		this.mut = maybe_read_keyword(31);
		this.expression = read_unary_rhs();
		__endRange(this);
	}
}
class RawReferenceExpression extends BaseNode2 {
	constructor(kind) {
		super(99, new Loc(GET_SOURCE(), 0, 0));
		this.kind = kind;
		this.expression = read_unary_rhs();
		__endRange(this);
	}
}
class DereferenceExpression extends BaseNode2 {
	constructor() {
		super(100, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip();
		this.expression = read_unary_rhs();
		__endRange(this);
	}
}
class BoxExpression extends BaseNode2 {
	constructor() {
		super(101, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.expression = read_unary_rhs();
		__endRange(this);
	}
}
class UnionPattern extends BaseNode2 {
	constructor(pattern) {
		super(102, new Loc(GET_SOURCE(), void 0 === pattern ? GET_POSITION() : start(pattern), 0));
		this.patterns = void 0 === pattern ? [] : [pattern];
		while (maybe_read(124)) this.patterns.push(read_pattern(false));
		__endRange(this);
	}
}
class ParenthesizedPattern extends BaseNode2 {
	constructor(items) {
		super(103, new Loc(GET_SOURCE(), start(items), end(items)));
		this.pattern = items[0];
	}
}
class RestPattern extends BaseNode2 {
	constructor() {
		super(104, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		__endRange(this);
	}
}
class WildcardPattern extends BaseNode2 {
	constructor() {
		super(105, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		__endRange(this);
	}
}
class PatternVariableDeclaration extends BaseNode2 {
	constructor(id) {
		super(106, new Loc(GET_SOURCE(), void 0 === id ? GET_POSITION() : start(id), 0));
		this.ref = false;
		this.mut = false;
		if (id === void 0) {
			loop: while (true) {
				switch (peek_keyword()) {
					case 29:
						safe_skip_keyword();
						this.ref = true;
						break;
					case 31:
						safe_skip_keyword();
						this.mut = true;
						break;
					default:
						break loop;
				}
			}
			this.id = new Identifier();
			this.pattern = maybe_read(64) ? read_pattern(false) : void 0;
		} else {
			this.id = id;
			this.pattern = (safe_skip(), read_pattern(false));
		}
		__endRange(this);
	}
}
class StructPattern extends BaseNode2 {
	constructor(struct) {
		super(107, new Loc(GET_SOURCE(), start(struct), 0));
		this.struct = struct;
		this.properties = read_sequence(3, () =>
			with_outerAttributes_fromParentContext(() => {
				switch (peek_keyword()) {
					case 46:
					case 29:
					case 31:
						return new StructPatternPropertyShorthand(void 0);
					case 3:
					case 1:
					case 14: {
						const id = new Identifier();
						return match(58) ? new StructPatternPropertyDestructured(id) : new StructPatternPropertyShorthand(id);
					}
				}
				if (match(46)) {
					return read_ahead(() => {
						safe_skip_1_read_2(46, 46);
						return new RestPattern();
					});
				} else {
					return new StructPatternPropertyDestructured(read_Index());
				}
			})
		);
		__endRange(this);
	}
}
class StructPatternPropertyDestructured extends BaseNode2 {
	constructor(key) {
		super(108, new Loc(GET_SOURCE(), start(key), 0));
		this.key = key;
		safe_skip();
		this.pattern = read_PatternNoUnion_unstrict();
		__endRange(this);
	}
}
class StructPatternPropertyShorthand extends BaseNode2 {
	constructor(id) {
		super(109, new Loc(GET_SOURCE(), void 0 === id ? GET_POSITION() : start(id), 0));
		this.box = false;
		this.ref = false;
		this.mut = false;
		if (id === void 0) {
			loop: while (true) {
				switch (peek_keyword()) {
					case 46:
						safe_skip_keyword();
						this.box = FG_property_true();
						break;
					case 29:
						safe_skip_keyword();
						this.ref = true;
						break;
					case 31:
						safe_skip_keyword();
						this.mut = true;
						break;
					default:
						break loop;
				}
			}
			this.id = new Identifier();
		} else {
			this.id = id;
		}
		__endRange(this);
	}
}
class TuplePattern extends BaseNode2 {
	constructor(struct, items) {
		super(110, new Loc(GET_SOURCE(), 0, 0));
		this.struct = struct;
		this.items = items;
		withRange(this, struct ?? items, items);
	}
}
class ArrayPattern extends BaseNode2 {
	constructor() {
		super(111, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.items = read_sequence(2, () => read_pattern(true));
		__endRange(this);
	}
}
class ReferencePattern extends BaseNode2 {
	constructor() {
		super(112, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip();
		this.mut = maybe_read_keyword(31);
		this.pattern = read_pattern(false);
		__endRange(this);
	}
}
class BoxPattern extends BaseNode2 {
	constructor() {
		super(113, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.pattern = read_pattern(false);
		__endRange(this);
	}
}
class MinusPattern extends BaseNode2 {
	constructor(read_pattern2) {
		super(114, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip();
		this.pattern = read_pattern2();
		__endRange(this);
	}
}
class RangePattern extends BaseNode2 {
	constructor(lower) {
		super(115, new Loc(GET_SOURCE(), void 0 === lower ? GET_POSITION() : start(lower), 0));
		this.lower = lower;
		switch (currChar()) {
			case 61:
				if (peek_not_match(-1, 46) || peek_match(1, 62)) {
					this.last = false;
					this.upper = void 0;
					break;
				}
				safe_skip();
				this.last = true;
				this.upper = maybe_read_RangePatternBoundEnd();
				break;
			case 46:
				safe_skip();
				this.last = true;
				this.upper = maybe_read_RangePatternBoundEnd();
				break;
			default:
				this.last = false;
				this.upper = maybe_read_RangePatternBoundEnd();
				break;
		}
		__endRange(this);
	}
}
class TypePath extends BaseNode2 {
	constructor(namespace) {
		super(116, new Loc(GET_SOURCE(), void 0 === namespace ? GET_POSITION() : start(namespace), 0));
		this.namespace = namespace;
		this.segment = new Identifier();
		__endRange(this);
	}
}
class TypeCall extends BaseNode2 {
	constructor(callee) {
		super(117, new Loc(GET_SOURCE(), start(callee), 0));
		this.typeCallee = callee;
		this.typeArguments = read_TypeArguments();
		__endRange(this);
	}
}
class TypeCallNamedArgument extends BaseNode2 {
	constructor(id) {
		super(118, new Loc(GET_SOURCE(), start(id), 0));
		this.target = id;
		safe_skip();
		this.typeExpression = read_type(true);
		__endRange(this);
	}
}
class TypeCallNamedBound extends BaseNode2 {
	constructor(typeTarget) {
		super(119, new Loc(GET_SOURCE(), start(typeTarget), 0));
		this.typeTarget = typeTarget;
		safe_skip();
		this.typeBounds = read_typeBounds();
		__endRange(this);
	}
}
class LtIdentifier extends BaseNode2 {
	get name() {
		return this.loc.getOwnText();
	}
	constructor() {
		super(120, new Loc(GET_SOURCE(), GET_POSITION() - 1, 0));
		read_cached_keyword();
		__endRange(this);
	}
}
class LtElided extends BaseNode2 {
	constructor() {
		super(121, new Loc(GET_SOURCE(), GET_POSITION() - 1, 0));
		safe_skip_keyword();
		__endRange(this);
	}
}
class LtStatic extends BaseNode2 {
	constructor() {
		super(122, new Loc(GET_SOURCE(), GET_POSITION() - 1, 0));
		safe_skip_keyword();
		__endRange(this);
	}
}
class TypeNever extends BaseNode2 {
	constructor() {
		super(123, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip();
		__endRange(this);
	}
}
class TypeInferred extends BaseNode2 {
	constructor() {
		super(124, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		__endRange(this);
	}
}
class GenericTypeParameterDeclaration extends BaseNode2 {
	constructor() {
		super(125, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.id = new Identifier();
		if (maybe_read(58)) {
			switch (currChar()) {
				case 44:
				case 62:
					this.typeBounds = [];
					break;
				default:
					this.typeBounds = read_typeBounds();
					break;
			}
		} else {
			this.typeBounds = void 0;
		}
		this.typeDefault = maybe_read(61) ? read_type(true) : void 0;
		__endRange(this);
	}
}
class ConstTypeParameterDeclaration extends BaseNode2 {
	constructor() {
		super(126, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.id = new Identifier();
		this.typeAnnotation = read_maybe_missing(() => maybe_read_typeAnnotation());
		this.typeDefault = FG_property("const_generics_defaults", this, "typeDefault", () =>
			maybe_read(61) ? read_FG_typeDefault() : void 0
		);
		__endRange(this);
	}
}
class GenericLtParameterDeclaration extends BaseNode2 {
	constructor() {
		super(127, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		if (not_match(39)) exit3.expected("Lifetime");
		this.id = read_Lifetime();
		this.ltBounds = maybe_read_ltBounds();
		__endRange(this);
	}
}
class WhereTypeBoundDeclaration extends BaseNode2 {
	constructor() {
		super(128, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.ltParameters = maybe_read_forLtParameters();
		this.typeTarget = read_type(true);
		read(58);
		switch (currChar()) {
			case 44:
			case 61:
			case 123:
			case 59:
				this.typeBounds = [];
				break;
			default:
				this.typeBounds = read_typeBounds();
				break;
		}
		__endRange(this);
	}
}
class WhereLtBoundDeclaration extends BaseNode2 {
	constructor() {
		super(129, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		this.ltTarget = read_Lifetime();
		this.ltBounds = maybe_read_ltBounds();
		__endRange(this);
	}
}
class TypeTraitBound extends BaseNode2 {
	constructor(maybeConst, optional, ltParameters, typeExpression) {
		super(130, new Loc(GET_SOURCE(), 0, 0));
		this.maybeConst = FG_property("const_trait_impl", this, "maybeConst", () => maybeConst);
		this.optional = optional;
		this.ltParameters = ltParameters;
		this.typeExpression = typeExpression;
		__endRange(this);
	}
}
class TypeDynBounds extends BaseNode2 {
	constructor(dyn, typeBound) {
		super(131, new Loc(GET_SOURCE(), 0, 0));
		this.dyn = dyn;
		this.typeBounds = read_standalone_bounds(typeBound);
		__endRange(this);
	}
}
class TypeImplBounds extends BaseNode2 {
	constructor() {
		super(132, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip_keyword();
		this.typeBounds = read_standalone_bounds(read_TypeBound());
		__endRange(this);
	}
}
class TypeFnPointer extends BaseNode2 {
	constructor(ltParameters) {
		super(133, new Loc(GET_SOURCE(), void 0 === ltParameters ? GET_POSITION() : start(ltParameters), 0));
		this.ltParameters = ltParameters;
		safe_skip_keyword();
		this.parameters = read_sequence(1, () =>
			with_outerAttributes_fromParentContext(() => (match(46) ? new FunctionSpread() : new TypeFnPointerParameter()))
		);
		this.returnType = maybe_read_ReturnType(false);
		__endRange(this);
	}
}
class TypeFnPointerParameter extends BaseNode2 {
	constructor() {
		super(134, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		const lhs = read_type(true);
		if (maybe_read(58)) {
			this.id = lhs;
			this.typeAnnotation = read_type(true);
		} else {
			this.id = void 0;
			this.typeAnnotation = lhs;
		}
		__endRange(this);
	}
}
class TypeFunction extends BaseNode2 {
	constructor(callee) {
		super(135, new Loc(GET_SOURCE(), start(callee), 0));
		this.callee = callee;
		this.parameters = read_sequence(1, () => read_type(true));
		this.returnType = maybe_read_ReturnType(false);
		__endRange(this);
	}
}
class TypeTuple extends BaseNode2 {
	constructor(items) {
		super(136, new Loc(GET_SOURCE(), start(items), end(items)));
		this.items = items;
	}
}
class TypeSizedArray extends BaseNode2 {
	constructor(typeExpression) {
		super(137, new Loc(GET_SOURCE(), 0, 0));
		this.typeExpression = typeExpression;
		this.sizeExpression = read_expression_between(59, 93);
		__endRange(this);
	}
}
class TypeSlice extends BaseNode2 {
	constructor(typeExpression) {
		super(138, new Loc(GET_SOURCE(), 0, 0));
		this.typeExpression = typeExpression;
		safe_skip();
		__endRange(this);
	}
}
class TypeReference extends BaseNode2 {
	constructor() {
		super(139, new Loc(GET_SOURCE(), GET_POSITION(), 0));
		safe_skip();
		this.lt = maybe_read_lifetime();
		this.mut = maybe_read_keyword(31);
		this.typeExpression = read_type(false);
		__endRange(this);
	}
}
class TypeDereferenceConst extends BaseNode2 {
	constructor() {
		super(140, new Loc(GET_SOURCE(), 0, 0));
		safe_skip_keyword();
		this.typeExpression = read_type(false);
		__endRange(this);
	}
}
class TypeDereferenceMut extends BaseNode2 {
	constructor() {
		super(141, new Loc(GET_SOURCE(), 0, 0));
		safe_skip_keyword();
		this.typeExpression = read_type(false);
		__endRange(this);
	}
}
class TypeParenthesized extends BaseNode2 {
	constructor(items) {
		super(142, new Loc(GET_SOURCE(), start(items), end(items)));
		this.typeExpression = items[0];
	}
}
// src/parser/helpers.ts
function mockNode(nodeType, loc, obj) {
	var node = new BaseNode2(nodeType, loc);
	for (var key in obj) node[key] = obj[key];
	return node;
}
function createLocArray(dk, loc, array = []) {
	array.loc = loc;
	array.dk = dk;
	return array;
}
// src/utils/unicode.ts
var CharCode_UTF8BOM = 65279;
var did_init = false;
var XID = new Uint8Array(0);
function INIT_UNICODE() {
	did_init = true;
	XID = new Uint8Array(17 << 16);
	const {
		0: Common,
		1: Start,
		2: Continue,
	} = JSON.parse(
		"[[[65,90],[97,122],170,181,186,[192,214],[216,246],[248,705],[710,721],[736,740],748,750,[886,887],[891,893],895,908,[910,929],[931,1013],[1015,1153],[1162,1327],[1329,1366],1369,[1376,1416],[1488,1514],[1519,1522],1791,2042,[2144,2154],[2208,2228],[2230,2247],[2437,2444],[2447,2448],[2451,2472],[2474,2480],2482,[2486,2489],[2524,2525],2556,[2565,2570],[2575,2576],[2579,2600],[2602,2608],[2610,2611],[2613,2614],[2616,2617],[2649,2652],2654,[2693,2701],[2703,2705],[2707,2728],[2730,2736],[2738,2739],[2741,2745],2768,[2821,2828],[2831,2832],[2835,2856],[2858,2864],[2866,2867],[2869,2873],[2908,2909],2929,[2949,2954],[2958,2960],[2962,2965],[2969,2970],2972,[2974,2975],[2979,2980],[2984,2986],[2990,3001],3024,[3086,3088],[3090,3112],[3114,3129],[3160,3162],[3205,3212],[3214,3216],[3218,3240],[3242,3251],[3253,3257],3294,[3313,3314],[3342,3344],[3450,3455],[3461,3478],[3482,3505],[3507,3515],3517,[3520,3526],[3713,3714],3716,[3718,3722],[3724,3747],3749,[3776,3780],3782,[3804,3807],3840,[3913,3948],[4256,4293],4295,4301,[4304,4346],[4348,4680],[4682,4685],[4688,4694],4696,[4698,4701],[4704,4744],[4746,4749],[4752,4784],[4786,4789],[4792,4798],4800,[4802,4805],[4808,4822],[4824,4880],[4882,4885],[4888,4954],[4992,5007],[5024,5109],[5112,5117],[5121,5740],[5743,5759],[5761,5786],[5792,5866],[5870,5880],[5888,5900],[5984,5996],[5998,6000],6103,[6176,6264],[6320,6389],[6400,6430],[6512,6516],[6528,6571],[6576,6601],6823,[7296,7304],[7312,7354],[7357,7359],[7960,7965],[7968,8005],[8008,8013],[8016,8023],8025,8027,8029,[8031,8061],[8064,8116],[8118,8124],8126,[8130,8132],[8134,8140],[8144,8147],[8150,8155],[8160,8172],[8178,8180],[8182,8188],8305,8319,[8336,8348],8450,8455,[8458,8467],8469,[8472,8477],8484,8486,8488,[8490,8505],[8508,8511],[8517,8521],8526,[8544,8584],[11264,11310],[11312,11358],[11360,11492],[11520,11557],11559,11565,[11568,11623],11631,[11680,11686],[11688,11694],[11696,11702],[11704,11710],[11712,11718],[11720,11726],[11728,11734],[11736,11742],[12293,12295],[12337,12341],[12344,12348],[12353,12438],[12445,12447],[12449,12538],[12540,12543],[12549,12591],[12593,12686],[12704,12735],[12784,12799],[13312,19903],[19968,40956],[40960,42124],[42192,42237],[42240,42508],[42775,42783],[42786,42888],[42891,42943],[42946,42954],[43072,43123],43259,[43360,43388],[43616,43638],[43739,43741],[43777,43782],[43785,43790],[43793,43798],[43808,43814],[43816,43822],[43824,43866],[43868,43881],[44032,55203],[55216,55238],[55243,55291],[63744,64109],[64112,64217],[64256,64262],[64275,64279],[64298,64310],[64312,64316],64318,[64320,64321],[64323,64324],[64326,64433],[64467,64605],[64612,64829],[64848,64911],[64914,64967],[65008,65017],65137,65139,65143,65145,65147,65149,[65151,65276],[65313,65338],[65345,65370],[65474,65479],[65482,65487],[65490,65495],[65498,65500],[65536,65547],[65549,65574],[65576,65594],[65596,65597],[65599,65613],[65616,65629],[65664,65786],[65856,65908],[66176,66204],[66208,66256],[66304,66335],[66349,66378],[66432,66461],[66464,66499],[66504,66511],[66513,66517],[66560,66717],[66736,66771],[66776,66811],[66816,66855],[66864,66915],[67072,67382],[67392,67413],[67424,67431],[67584,67589],67592,[67594,67637],[67639,67640],67644,[67647,67669],[67680,67702],[67712,67742],[67808,67826],[67828,67829],[67840,67861],[67872,67897],[67968,68023],[68030,68031],[68117,68119],[68121,68149],[68192,68220],[68224,68252],[68288,68295],[68352,68405],[68416,68437],[68448,68466],[68480,68497],[68608,68680],[68736,68786],[68800,68850],[69248,69289],[69296,69297],[69376,69404],69415,[69552,69572],[69600,69622],[69840,69864],70006,70108,[70144,70161],[70272,70278],70280,[70282,70285],[70287,70301],[70303,70312],[70405,70412],[70415,70416],[70419,70440],[70442,70448],[70450,70451],[70453,70457],70480,70855,71236,[71424,71450],[71935,71942],71945,[71948,71955],[71957,71958],[72096,72103],72349,[72384,72440],[72704,72712],[72818,72847],[72960,72966],[72968,72969],[73056,73061],[73063,73064],73648,[73728,74649],[74752,74862],[74880,75075],[77824,78894],[82944,83526],[92160,92728],[92736,92766],[92880,92909],[92992,92995],[93027,93047],[93053,93071],[93760,93823],[93952,94026],[94176,94177],[94208,100343],[100352,101589],[101632,101640],[110592,110878],[110928,110930],[110948,110951],[110960,111355],[113664,113770],[113776,113788],[113792,113800],[113808,113817],[119808,119892],[119894,119964],[119966,119967],119970,[119973,119974],[119977,119980],[119982,119993],119995,[119997,120003],[120005,120069],[120071,120074],[120077,120084],[120086,120092],[120094,120121],[120123,120126],[120128,120132],120134,[120138,120144],[120146,120485],[120488,120512],[120514,120538],[120540,120570],[120572,120596],[120598,120628],[120630,120654],[120656,120686],[120688,120712],[120714,120744],[120746,120770],[120772,120779],[123136,123180],123214,[124928,125124],[126464,126467],[126469,126495],[126497,126498],126500,126503,[126505,126514],[126516,126519],126521,126523,126530,126535,126537,126539,[126541,126543],[126545,126546],126548,126551,126553,126555,126557,126559,[126561,126562],126564,[126567,126570],[126572,126578],[126580,126583],[126585,126588],126590,[126592,126601],[126603,126619],[126625,126627],[126629,126633],[126635,126651],[131072,173789],[173824,177972],[177984,178205],[178208,183969],[183984,191456],[194560,195101],[196608,201546]],[[880,884],902,[904,906],[1568,1610],[1646,1647],[1649,1747],1749,[1765,1766],[1774,1775],[1786,1788],1808,[1810,1839],[1869,1957],1969,[1994,2026],[2036,2037],[2048,2069],2074,2084,2088,[2112,2136],[2308,2361],2365,2384,[2392,2401],[2417,2432],2493,2510,[2527,2529],[2544,2545],[2674,2676],2749,[2784,2785],2809,2877,[2911,2913],2947,[3077,3084],3133,[3168,3169],3200,3261,[3296,3297],[3332,3340],[3346,3386],3389,3406,[3412,3414],[3423,3425],[3585,3632],3634,[3648,3654],[3751,3760],3762,3773,[3904,3911],[3976,3980],[4096,4138],4159,[4176,4181],[4186,4189],4193,[4197,4198],[4206,4208],[4213,4225],4238,[5902,5905],[5920,5937],[5952,5969],[6016,6067],6108,[6272,6312],6314,[6480,6509],[6656,6678],[6688,6740],[6917,6963],[6981,6987],[7043,7072],[7086,7087],[7098,7141],[7168,7203],[7245,7247],[7258,7293],[7401,7404],[7406,7411],[7413,7414],7418,[7424,7615],[7680,7957],[11499,11502],[11506,11507],[11648,11670],[12321,12329],[42512,42527],[42538,42539],[42560,42606],[42623,42653],[42656,42735],[42997,43009],[43011,43013],[43015,43018],[43020,43042],[43138,43187],[43250,43255],[43261,43262],[43274,43301],[43312,43334],[43396,43442],43471,[43488,43492],[43494,43503],[43514,43518],[43520,43560],[43584,43586],[43588,43595],43642,[43646,43695],43697,[43701,43702],[43705,43709],43712,43714,[43744,43754],[43762,43764],[43888,44002],64285,[64287,64296],[65382,65437],[65440,65470],[66384,66421],68096,[68112,68115],[68297,68324],[68864,68899],[69424,69445],[69635,69687],[69763,69807],[69891,69926],69956,69959,[69968,70002],[70019,70066],[70081,70084],70106,[70163,70187],[70320,70366],70461,[70493,70497],[70656,70708],[70727,70730],[70751,70753],[70784,70831],[70852,70853],[71040,71086],[71128,71131],[71168,71215],[71296,71338],71352,[71680,71723],[71840,71903],[71960,71983],71999,72001,[72106,72144],72161,72163,72192,[72203,72242],72250,72272,[72284,72329],[72714,72750],72768,[72971,73008],73030,[73066,73097],73112,[73440,73458],[92928,92975],94032,[94099,94111],94179,[123191,123197],[123584,123627],[125184,125251],125259],[[48,57],95,183,[768,884],[902,906],[1155,1159],[1425,1469],1471,[1473,1474],[1476,1477],1479,[1552,1562],[1568,1641],[1646,1747],[1749,1756],[1759,1768],[1770,1788],[1808,1866],[1869,1969],[1984,2037],2045,[2048,2093],[2112,2139],[2259,2273],[2275,2403],[2406,2415],[2417,2435],[2492,2500],[2503,2504],[2507,2510],2519,[2527,2531],[2534,2545],2558,[2561,2563],2620,[2622,2626],[2631,2632],[2635,2637],2641,[2662,2677],[2689,2691],[2748,2757],[2759,2761],[2763,2765],[2784,2787],[2790,2799],[2809,2815],[2817,2819],[2876,2884],[2887,2888],[2891,2893],[2901,2903],[2911,2915],[2918,2927],[2946,2947],[3006,3010],[3014,3016],[3018,3021],3031,[3046,3055],[3072,3084],[3133,3140],[3142,3144],[3146,3149],[3157,3158],[3168,3171],[3174,3183],[3200,3203],[3260,3268],[3270,3272],[3274,3277],[3285,3286],[3296,3299],[3302,3311],[3328,3340],[3346,3396],[3398,3400],[3402,3406],[3412,3415],[3423,3427],[3430,3439],[3457,3459],3530,[3535,3540],3542,[3544,3551],[3558,3567],[3570,3571],[3585,3642],[3648,3662],[3664,3673],[3751,3773],[3784,3789],[3792,3801],[3864,3865],[3872,3881],3893,3895,3897,[3902,3911],[3953,3972],[3974,3991],[3993,4028],4038,[4096,4169],[4176,4253],[4957,4959],[4969,4977],[5902,5908],[5920,5940],[5952,5971],[6002,6003],[6016,6099],[6108,6109],[6112,6121],[6155,6157],[6160,6169],[6272,6314],[6432,6443],[6448,6459],[6470,6509],[6608,6618],[6656,6683],[6688,6750],[6752,6780],[6783,6793],[6800,6809],[6832,6845],[6847,6848],[6912,6987],[6992,7001],[7019,7027],[7040,7155],[7168,7223],[7232,7241],[7245,7293],[7376,7378],[7380,7418],[7424,7673],[7675,7957],[8255,8256],8276,[8400,8412],8417,[8421,8432],[11499,11507],[11647,11670],[11744,11775],[12321,12335],[12441,12442],[42512,42539],[42560,42607],[42612,42621],[42623,42737],[42997,43047],43052,[43136,43205],[43216,43225],[43232,43255],[43261,43309],[43312,43347],[43392,43456],[43471,43481],[43488,43518],[43520,43574],[43584,43597],[43600,43609],[43642,43714],[43744,43759],[43762,43766],[43888,44010],[44012,44013],[44016,44025],[64285,64296],[65024,65039],[65056,65071],[65075,65076],[65101,65103],[65296,65305],65343,[65382,65470],66045,66272,[66384,66426],[66720,66729],[68096,68099],[68101,68102],[68108,68115],[68152,68154],68159,[68297,68326],[68864,68903],[68912,68921],[69291,69292],[69424,69456],[69632,69702],[69734,69743],[69759,69818],[69872,69881],[69888,69940],[69942,69951],[69956,69959],[69968,70003],[70016,70084],[70089,70092],[70094,70106],[70163,70199],70206,[70320,70378],[70384,70393],[70400,70403],[70459,70468],[70471,70472],[70475,70477],70487,[70493,70499],[70502,70508],[70512,70516],[70656,70730],[70736,70745],[70750,70753],[70784,70853],[70864,70873],[71040,71093],[71096,71104],[71128,71133],[71168,71232],[71248,71257],[71296,71352],[71360,71369],[71453,71467],[71472,71481],[71680,71738],[71840,71913],[71960,71989],[71991,71992],[71995,72003],[72016,72025],[72106,72151],[72154,72161],[72163,72164],[72192,72254],72263,[72272,72345],[72714,72758],[72760,72768],[72784,72793],[72850,72871],[72873,72886],[72971,73014],73018,[73020,73021],[73023,73031],[73040,73049],[73066,73102],[73104,73105],[73107,73112],[73120,73129],[73440,73462],[92768,92777],[92912,92916],[92928,92982],[93008,93017],[94031,94087],[94095,94111],[94179,94180],[94192,94193],[113821,113822],[119141,119145],[119149,119154],[119163,119170],[119173,119179],[119210,119213],[119362,119364],[120782,120831],[121344,121398],[121403,121452],121461,121476,[121499,121503],[121505,121519],[122880,122886],[122888,122904],[122907,122913],[122915,122916],[122918,122922],[123184,123197],[123200,123209],[123584,123641],[125136,125142],[125184,125259],[125264,125273],[130032,130041],[917760,917999]]]"
	);
	flag_each(Common, 1 | 2);
	flag_each(Start, 1);
	flag_each(Continue, 2);
	function flag_each(arr, flag) {
		for (var i = 0, j = 0; i < arr.length; i++) {
			if (typeof arr[i] === "number") XID[arr[i]] = flag;
			else for (j = arr[i][0]; j <= arr[i][1]; j++) XID[j] = flag;
		}
	}
}
function is_UNICODE_XID_Start(code) {
	if (false === did_init) INIT_UNICODE();
	return (XID[code] & 1) !== 0;
}
function is_UNICODE_XID_Continue(code) {
	if (false === did_init) INIT_UNICODE();
	return (XID[code] & 2) !== 0;
}
// src/utils/enum.ts
function is_number(char) {
	switch (char) {
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 56:
		case 57:
			return true;
		default:
			return false;
	}
}
function is_whitespaceOrSlash(char) {
	switch (char) {
		case 9:
		case 10:
		case 11:
		case 12:
		case 13:
		case 32:
		case 47:
		case 133:
		case 8206:
		case 8207:
		case 8232:
		case 8233:
			return true;
		default:
			return false;
	}
}
function is_UpperCase(char) {
	switch (char) {
		case 65:
		case 66:
		case 67:
		case 68:
		case 69:
		case 70:
		case 71:
		case 72:
		case 73:
		case 74:
		case 75:
		case 76:
		case 77:
		case 78:
		case 79:
		case 80:
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
		case 87:
		case 88:
		case 89:
		case 90:
			return true;
		default:
			return false;
	}
}
function is_XID_Start(char) {
	switch (char) {
		case 65:
		case 66:
		case 67:
		case 68:
		case 69:
		case 70:
		case 71:
		case 72:
		case 73:
		case 74:
		case 75:
		case 76:
		case 77:
		case 78:
		case 79:
		case 80:
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
		case 87:
		case 88:
		case 89:
		case 90:
		case 95:
		case 97:
		case 98:
		case 99:
		case 100:
		case 101:
		case 102:
		case 103:
		case 104:
		case 105:
		case 106:
		case 107:
		case 108:
		case 109:
		case 110:
		case 111:
		case 112:
		case 113:
		case 114:
		case 115:
		case 116:
		case 117:
		case 118:
		case 119:
		case 120:
		case 121:
		case 122:
			return true;
		default:
			return 128 < char && is_UNICODE_XID_Start(char);
	}
}
function is_XID_Continue(char) {
	switch (char) {
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 56:
		case 57:
		case 65:
		case 66:
		case 67:
		case 68:
		case 69:
		case 70:
		case 71:
		case 72:
		case 73:
		case 74:
		case 75:
		case 76:
		case 77:
		case 78:
		case 79:
		case 80:
		case 81:
		case 82:
		case 83:
		case 84:
		case 85:
		case 86:
		case 87:
		case 88:
		case 89:
		case 90:
		case 95:
		case 97:
		case 98:
		case 99:
		case 100:
		case 101:
		case 102:
		case 103:
		case 104:
		case 105:
		case 106:
		case 107:
		case 108:
		case 109:
		case 110:
		case 111:
		case 112:
		case 113:
		case 114:
		case 115:
		case 116:
		case 117:
		case 118:
		case 119:
		case 120:
		case 121:
		case 122:
			return true;
		default:
			return 128 < char && is_UNICODE_XID_Continue(char);
	}
}
function is_hex(char) {
	switch (char) {
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 56:
		case 57:
		case 65:
		case 66:
		case 67:
		case 68:
		case 69:
		case 70:
		case 97:
		case 98:
		case 99:
		case 100:
		case 101:
		case 102:
			return true;
		default:
			return false;
	}
}
function is_oct(char) {
	switch (char) {
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
			return true;
		default:
			return false;
	}
}
function is_bin(char) {
	switch (char) {
		case 48:
		case 49:
			return true;
		default:
			return false;
	}
}
function getDelimKind(startChar) {
	switch (startChar) {
		case 40:
			return 1;
		case 91:
			return 2;
		case 123:
			return 3;
		case 60:
			return 4;
		case 124:
			return 5;
	}
	return 0;
}
function getGroupDelimKind(startChar) {
	switch (startChar) {
		case 40:
			return 1;
		case 91:
			return 2;
		case 123:
			return 3;
	}
	exit(`Assertion failed: Expected GroupDelim start`);
}
function getDelimStartCharCode(kind) {
	switch (kind) {
		case 1:
			return 40;
		case 2:
			return 91;
		case 3:
			return 123;
		case 4:
			return 60;
		case 5:
			return 124;
	}
	exit(`Assertion failed: Expected SomeDelimKind`, kind);
}
function getDelimEndCharCode(kind) {
	switch (kind) {
		case 1:
			return 41;
		case 2:
			return 93;
		case 3:
			return 125;
		case 4:
			return 62;
		case 5:
			return 124;
	}
	exit(`Assertion failed: Expected SomeDelimKind`, kind);
}
// src/parser/state/constructor.ts
function __endRange(target) {
	if (didJustSkipWhitespace()) {
		setRangeEnd(target, _get_ws_preskip_pos());
	} else {
		setRangeEnd(target, GET_POSITION());
		skip_whitespace();
	}
}
function withEscapedParens(startNode, read2) {
	if (is_ParenthesizedNode(startNode)) {
		const inner = getParenthesizedNodeContent(startNode);
		{
			const innerStart = start(inner);
			unsafe_setRangeStart(inner, start(startNode));
			const res = read2(inner);
			unsafe_setRangeStart(inner, innerStart);
			return res;
		}
	} else {
		return read2(startNode);
	}
}
function escapeParens(startNode) {
	{
		return getParenthesizedNodeContent(startNode);
	}
}
// src/parser/state/whitespace.ts
var Attribute4;
function __EXPORT_ATTR_CTORS(_Attribute) {
	Attribute = _Attribute;
}
function skip_whitespace() {
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
			register_attribute(new Attribute());
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
				register_attribute(new Attribute());
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
var _ws_preskip_pos = 0;
var _ws_postskip_pos = 0;
var __is_line = false;
var __is_attr = false;
var __is_inner = false;
var _get_ws_preskip_pos = () => _ws_preskip_pos;
var _get_ws_postskip_pos = () => _ws_postskip_pos;
var setPreSkipWhitespace = (pos2) => (_ws_preskip_pos = pos2);
var setPostSkipWhitespace = (pos2) => (_ws_postskip_pos = pos2);
var savePreSkipPos = () => setPreSkipWhitespace(GET_POSITION());
var savePostSkipPos = () => setPostSkipWhitespace(GET_POSITION());
var didJustSkipWhitespace = () => GET_POSITION() === _get_ws_postskip_pos();
var getPreWhitespaceSkipPosition = () => (didJustSkipWhitespace() ? _get_ws_preskip_pos() : GET_POSITION());
function will_read_CommentNode() {
	switch (peek(1)) {
		case 47:
			return (__is_line = true);
		case 42:
			return (__is_line = false), true;
		default:
			return false;
	}
}
function read_CommentNode() {
	switch (Mc_ctx_isReadingTokens() ? 0 : peek(2)) {
		case 33:
			__is_attr = __is_inner = true;
			break;
		case 42:
			if ((__is_attr = !__is_line))
				switch (peek(3)) {
					case 42:
					case 47:
						__is_attr = false;
				}
			__is_inner = false;
			break;
		case 47:
			__is_attr = __is_line && peek_not_match(3, 47);
			__is_inner = false;
			break;
		default:
			__is_attr = __is_inner = false;
			break;
	}
	if (__is_attr) register_attribute(new DocCommentAttribute());
	else register_comment(new Comment());
}
function read_comment() {
	if (__is_line) {
		nStep(__is_attr ? 3 : 2);
		step_until_ln();
	} else {
		let depth = 1;
		let lpos = GET_POSITION();
		nStep(__is_attr ? 3 : 2);
		while (true) {
			step_until_match(47);
			if (lpos !== GET_POSITION() - 2 && peek_match(-1, 42)) {
				safe_step_over();
				if (--depth === 0) break;
			} else {
				safe_step_over();
				if (match(42)) {
					safe_step_over();
					lpos = GET_POSITION() - 2;
					depth++;
				}
			}
		}
	}
}
// src/parser/state/scanner.ts
function rhsTree() {
	switch (currChar()) {
		case 33:
			return peek_match(1, 61) ? rhs_resolve(16, 8, 2) : 43;
		case 37:
			return peek_match(1, 61) ? rhs_resolve(25, 8, 2) : rhs_resolve(9, 14, 1);
		case 38:
			switch (peek(1)) {
				case 38:
					return rhs_resolve(2, ES_ctx_insideScrutinee() ? 2 : 7, 2);
				case 61:
					return rhs_resolve(26, 4, 2);
				default:
					return rhs_resolve(10, 11, 1);
			}
		case 40:
			return 100;
		case 42:
			return peek_match(1, 61) ? rhs_resolve(23, 4, 2) : rhs_resolve(7, 14, 1);
		case 43:
			return peek_match(1, 61) ? rhs_resolve(21, 4, 2) : rhs_resolve(5, 13, 1);
		case 45:
			return peek_match(1, 61) ? rhs_resolve(22, 4, 2) : rhs_resolve(6, 13, 1);
		case 46:
			if (peek_match(1, 46)) {
				switch (peek(2)) {
					case 61:
						return rhs_resolve(36, 5, 3);
					case 46:
						return rhs_resolve(35, 5, 3);
					default:
						return rhs_resolve(34, 5, 2);
				}
			} else {
				return 1;
			}
		case 47:
			return peek_match(1, 61) ? rhs_resolve(24, 4, 2) : rhs_resolve(8, 14, 1);
		case 58:
			return maybe_skip_1_read_2(58) ? 40 : 0;
		case 60:
			switch (peek(1)) {
				case 60:
					return peek_match(2, 61) ? rhs_resolve(29, 4, 3) : rhs_resolve(13, 12, 2);
				case 61:
					return rhs_resolve(20, 8, 2);
				default:
					return rhs_resolve(19, 8, 1);
			}
		case 61:
			switch (peek(1)) {
				case 61:
					return rhs_resolve(15, 8, 2);
				case 62:
					return 0;
				default:
					return rhs_resolve(4, 4, 1);
			}
		case 62:
			switch (peek(1)) {
				case 61:
					return rhs_resolve(18, 8, 2);
				case 62:
					return peek_match(2, 61) ? rhs_resolve(30, 4, 3) : rhs_resolve(14, 12, 2);
				default:
					return rhs_resolve(17, 8, 1);
			}
		case 63:
			return 42;
		case 91:
			return 102;
		case 94:
			return peek_match(1, 61) ? rhs_resolve(28, 4, 2) : rhs_resolve(12, 10, 1);
		case 97:
			return match_keyword(Keyword.as) ? rhs_resolve(103, 15, 2) : 0;
		case 123:
			return !ES_ctx_exceptStructFormExpression() ? 101 : 0;
		case 124:
			switch (peek(1)) {
				case 61:
					return rhs_resolve(27, 4, 2);
				case 124:
					return rhs_resolve(3, ES_ctx_insideScrutinee() ? 1 : 6, 2);
				default:
					return rhs_resolve(11, 9, 1);
			}
		default:
			return 0;
	}
}
function will_actually_read_rString() {
	switch (currChar()) {
		case 35:
			while (true)
				switch (uc_next()) {
					case 35:
						continue;
					case 34:
						return true;
					default:
						do edgecase_stepback();
						while (match(35));
						return false;
				}
		case 34:
			return true;
		default:
			return false;
	}
}
function will_actually_read_bString() {
	switch (currChar()) {
		case 34:
			return true;
		case 39:
			return will_match_charLiteral_not_lt();
		default:
			return false;
	}
}
function will_actually_read_macro_rules() {
	return uc_next_match(33) || check_ahead(() => (skip_whitespace(), match(33)));
}
var Keyword = ((Keyword2) => {
	Keyword2[(Keyword2["NotAWord"] = 0)] = "NotAWord";
	Keyword2[(Keyword2["NotKeyword"] = 1)] = "NotKeyword";
	Keyword2[(Keyword2["Underscore"] = 2)] = "Underscore";
	Keyword2[(Keyword2["RawIdentifier"] = 3)] = "RawIdentifier";
	Keyword2[(Keyword2["macro_rules!"] = 4)] = "macro_rules!";
	Keyword2[(Keyword2["StringLiteral"] = 5)] = "StringLiteral";
	Keyword2[(Keyword2["auto"] = 6)] = "auto";
	Keyword2[(Keyword2["true"] = 7)] = "true";
	Keyword2[(Keyword2["false"] = 8)] = "false";
	Keyword2[(Keyword2["fn"] = 9)] = "fn";
	Keyword2[(Keyword2["mod"] = 10)] = "mod";
	Keyword2[(Keyword2["use"] = 11)] = "use";
	Keyword2[(Keyword2["struct"] = 12)] = "struct";
	Keyword2[(Keyword2["trait"] = 13)] = "trait";
	Keyword2[(Keyword2["union"] = 14)] = "union";
	Keyword2[(Keyword2["enum"] = 15)] = "enum";
	Keyword2[(Keyword2["impl"] = 16)] = "impl";
	Keyword2[(Keyword2["type"] = 17)] = "type";
	Keyword2[(Keyword2["let"] = 18)] = "let";
	Keyword2[(Keyword2["static"] = 19)] = "static";
	Keyword2[(Keyword2["const"] = 20)] = "const";
	Keyword2[(Keyword2["unsafe"] = 21)] = "unsafe";
	Keyword2[(Keyword2["async"] = 22)] = "async";
	Keyword2[(Keyword2["extern"] = 23)] = "extern";
	Keyword2[(Keyword2["move"] = 24)] = "move";
	Keyword2[(Keyword2["pub"] = 25)] = "pub";
	Keyword2[(Keyword2["as"] = 26)] = "as";
	Keyword2[(Keyword2["in"] = 27)] = "in";
	Keyword2[(Keyword2["dyn"] = 28)] = "dyn";
	Keyword2[(Keyword2["ref"] = 29)] = "ref";
	Keyword2[(Keyword2["for"] = 30)] = "for";
	Keyword2[(Keyword2["mut"] = 31)] = "mut";
	Keyword2[(Keyword2["where"] = 32)] = "where";
	Keyword2[(Keyword2["await"] = 33)] = "await";
	Keyword2[(Keyword2["return"] = 34)] = "return";
	Keyword2[(Keyword2["break"] = 35)] = "break";
	Keyword2[(Keyword2["continue"] = 36)] = "continue";
	Keyword2[(Keyword2["if"] = 37)] = "if";
	Keyword2[(Keyword2["else"] = 38)] = "else";
	Keyword2[(Keyword2["match"] = 39)] = "match";
	Keyword2[(Keyword2["loop"] = 40)] = "loop";
	Keyword2[(Keyword2["while"] = 41)] = "while";
	Keyword2[(Keyword2["super"] = 42)] = "super";
	Keyword2[(Keyword2["self"] = 43)] = "self";
	Keyword2[(Keyword2["Self"] = 44)] = "Self";
	Keyword2[(Keyword2["crate"] = 45)] = "crate";
	Keyword2[(Keyword2["box"] = 46)] = "box";
	Keyword2[(Keyword2["try"] = 47)] = "try";
	Keyword2[(Keyword2["yield"] = 48)] = "yield";
	Keyword2[(Keyword2["abstract"] = 49)] = "abstract";
	Keyword2[(Keyword2["become"] = 50)] = "become";
	Keyword2[(Keyword2["do"] = 51)] = "do";
	Keyword2[(Keyword2["final"] = 52)] = "final";
	Keyword2[(Keyword2["macro"] = 53)] = "macro";
	Keyword2[(Keyword2["override"] = 54)] = "override";
	Keyword2[(Keyword2["priv"] = 55)] = "priv";
	Keyword2[(Keyword2["typeof"] = 56)] = "typeof";
	Keyword2[(Keyword2["unsized"] = 57)] = "unsized";
	Keyword2[(Keyword2["virtual"] = 58)] = "virtual";
	return Keyword2;
})(Keyword || {});
var ToKeyword = [
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
];
function kwTree() {
	switch (currChar()) {
		case 83:
			if (uc_next_match(101) && uc_next_match(108) && uc_next_match(102)) {
				return kw_resolve(44);
			}
			break;
		case 95:
			return kw_resolve(2);
		case 97:
			switch (uc_next()) {
				case 98:
					if (
						uc_next_match(115) &&
						uc_next_match(116) &&
						uc_next_match(114) &&
						uc_next_match(97) &&
						uc_next_match(99) &&
						uc_next_match(116)
					) {
						return kw_resolve(49);
					}
					break;
				case 115:
					if (uc_next_match(121)) {
						if (uc_next_match(110) && uc_next_match(99)) {
							return kw_resolve(22);
						}
					} else {
						return kw_resolve_failed(26);
					}
					break;
				case 117:
					if (uc_next_match(116) && uc_next_match(111)) {
						return kw_resolve(6);
					}
					break;
				case 119:
					if (uc_next_match(97) && uc_next_match(105) && uc_next_match(116)) {
						return kw_resolve(33);
					}
					break;
			}
			break;
		case 98:
			switch (uc_next()) {
				case 101:
					if (uc_next_match(99) && uc_next_match(111) && uc_next_match(109) && uc_next_match(101)) {
						return kw_resolve(50);
					}
					break;
				case 111:
					if (uc_next_match(120)) {
						return kw_resolve(46);
					}
					break;
				case 114:
					if (uc_next_match(101)) {
						if (uc_next_match(97) && uc_next_match(107)) {
							return kw_resolve(35);
						}
					} else {
						if (will_actually_read_rString()) {
							return 5;
						}
					}
					break;
				default:
					if (will_actually_read_bString()) {
						return 5;
					}
			}
			break;
		case 99:
			switch (uc_next()) {
				case 111:
					if (uc_next_match(110)) {
						switch (uc_next()) {
							case 115:
								if (uc_next_match(116)) {
									return kw_resolve(20);
								}
								break;
							case 116:
								if (uc_next_match(105) && uc_next_match(110) && uc_next_match(117) && uc_next_match(101)) {
									return kw_resolve(36);
								}
								break;
						}
					}
					break;
				case 114:
					if (uc_next_match(97) && uc_next_match(116) && uc_next_match(101)) {
						return kw_resolve(45);
					}
					break;
			}
			break;
		case 100:
			switch (uc_next()) {
				case 111:
					return kw_resolve(51);
				case 121:
					if (uc_next_match(110)) {
						return kw_resolve(28);
					}
					break;
			}
			break;
		case 101:
			switch (uc_next()) {
				case 108:
					if (uc_next_match(115) && uc_next_match(101)) {
						return kw_resolve(38);
					}
					break;
				case 110:
					if (uc_next_match(117) && uc_next_match(109)) {
						return kw_resolve(15);
					}
					break;
				case 120:
					if (uc_next_match(116) && uc_next_match(101) && uc_next_match(114) && uc_next_match(110)) {
						return kw_resolve(23);
					}
					break;
			}
			break;
		case 102:
			switch (uc_next()) {
				case 97:
					if (uc_next_match(108) && uc_next_match(115) && uc_next_match(101)) {
						return kw_resolve(8);
					}
					break;
				case 105:
					if (uc_next_match(110) && uc_next_match(97) && uc_next_match(108)) {
						return kw_resolve(52);
					}
					break;
				case 110:
					return kw_resolve(9);
				case 111:
					if (uc_next_match(114)) {
						return kw_resolve(30);
					}
					break;
			}
			break;
		case 105:
			switch (uc_next()) {
				case 102:
					return kw_resolve(37);
				case 109:
					if (uc_next_match(112) && uc_next_match(108)) {
						return kw_resolve(16);
					}
					break;
				case 110:
					return kw_resolve(27);
			}
			break;
		case 108:
			switch (uc_next()) {
				case 101:
					if (uc_next_match(116)) {
						return kw_resolve(18);
					}
					break;
				case 111:
					if (uc_next_match(111) && uc_next_match(112)) {
						return kw_resolve(40);
					}
					break;
			}
			break;
		case 109:
			switch (uc_next()) {
				case 97:
					switch (uc_next()) {
						case 99:
							if (uc_next_match(114) && uc_next_match(111)) {
								if (uc_next_match(95)) {
									if (
										uc_next_match(114) &&
										uc_next_match(117) &&
										uc_next_match(108) &&
										uc_next_match(101) &&
										uc_next_match(115)
									) {
										if (will_actually_read_macro_rules()) {
											return 4;
										}
									}
								} else {
									return kw_resolve_failed(53);
								}
							}
							break;
						case 116:
							if (uc_next_match(99) && uc_next_match(104)) {
								return kw_resolve(39);
							}
							break;
					}
					break;
				case 111:
					switch (uc_next()) {
						case 100:
							return kw_resolve(10);
						case 118:
							if (uc_next_match(101)) {
								return kw_resolve(24);
							}
							break;
					}
					break;
				case 117:
					if (uc_next_match(116)) {
						return kw_resolve(31);
					}
					break;
			}
			break;
		case 111:
			if (
				uc_next_match(118) &&
				uc_next_match(101) &&
				uc_next_match(114) &&
				uc_next_match(114) &&
				uc_next_match(105) &&
				uc_next_match(100) &&
				uc_next_match(101)
			) {
				return kw_resolve(54);
			}
			break;
		case 112:
			switch (uc_next()) {
				case 114:
					if (uc_next_match(105) && uc_next_match(118)) {
						return kw_resolve(55);
					}
					break;
				case 117:
					if (uc_next_match(98)) {
						return kw_resolve(25);
					}
					break;
			}
			break;
		case 114:
			switch (uc_next()) {
				case 35:
					if (is_XID_Start(uc_next())) {
						read_XID_CONTINUE();
						return 3;
					}
					edgecase_stepback();
					if (will_actually_read_rString()) {
						return 5;
					}
					return 0;
				case 101:
					switch (uc_next()) {
						case 102:
							return kw_resolve(29);
						case 116:
							if (uc_next_match(117) && uc_next_match(114) && uc_next_match(110)) {
								return kw_resolve(34);
							}
							break;
					}
					break;
				default:
					if (will_actually_read_rString()) {
						return 5;
					}
			}
			break;
		case 115:
			switch (uc_next()) {
				case 101:
					if (uc_next_match(108) && uc_next_match(102)) {
						return kw_resolve(43);
					}
					break;
				case 116:
					switch (uc_next()) {
						case 97:
							if (uc_next_match(116) && uc_next_match(105) && uc_next_match(99)) {
								return kw_resolve(19);
							}
							break;
						case 114:
							if (uc_next_match(117) && uc_next_match(99) && uc_next_match(116)) {
								return kw_resolve(12);
							}
							break;
					}
					break;
				case 117:
					if (uc_next_match(112) && uc_next_match(101) && uc_next_match(114)) {
						return kw_resolve(42);
					}
					break;
			}
			break;
		case 116:
			switch (uc_next()) {
				case 114:
					switch (uc_next()) {
						case 97:
							if (uc_next_match(105) && uc_next_match(116)) {
								return kw_resolve(13);
							}
							break;
						case 117:
							if (uc_next_match(101)) {
								return kw_resolve(7);
							}
							break;
						case 121:
							return kw_resolve(47);
					}
					break;
				case 121:
					if (uc_next_match(112) && uc_next_match(101)) {
						if (uc_next_match(111)) {
							if (uc_next_match(102)) {
								return kw_resolve(56);
							}
						} else {
							return kw_resolve_failed(17);
						}
					}
					break;
			}
			break;
		case 117:
			switch (uc_next()) {
				case 110:
					switch (uc_next()) {
						case 105:
							if (uc_next_match(111) && uc_next_match(110)) {
								return kw_resolve(14);
							}
							break;
						case 115:
							switch (uc_next()) {
								case 97:
									if (uc_next_match(102) && uc_next_match(101)) {
										return kw_resolve(21);
									}
									break;
								case 105:
									if (uc_next_match(122) && uc_next_match(101) && uc_next_match(100)) {
										return kw_resolve(57);
									}
									break;
							}
							break;
					}
					break;
				case 115:
					if (uc_next_match(101)) {
						return kw_resolve(11);
					}
					break;
			}
			break;
		case 118:
			if (
				uc_next_match(105) &&
				uc_next_match(114) &&
				uc_next_match(116) &&
				uc_next_match(117) &&
				uc_next_match(97) &&
				uc_next_match(108)
			) {
				return kw_resolve(58);
			}
			break;
		case 119:
			if (uc_next_match(104)) {
				switch (uc_next()) {
					case 101:
						if (uc_next_match(114) && uc_next_match(101)) {
							return kw_resolve(32);
						}
						break;
					case 105:
						if (uc_next_match(108) && uc_next_match(101)) {
							return kw_resolve(41);
						}
						break;
				}
			}
			break;
		case 121:
			if (uc_next_match(105) && uc_next_match(101) && uc_next_match(108) && uc_next_match(100)) {
				return kw_resolve(48);
			}
			break;
		case 65:
		case 66:
		case 67:
		case 68:
		case 69:
		case 70:
		case 71:
		case 72:
		case 73:
		case 74:
		case 75:
		case 76:
		case 77:
		case 78:
		case 79:
		case 80:
		case 81:
		case 82:
		case 84:
		case 85:
		case 86:
		case 87:
		case 88:
		case 89:
		case 90:
		case 103:
		case 104:
		case 106:
		case 107:
		case 110:
		case 113:
		case 120:
		case 122:
			read_XID_CONTINUE();
			return 1;
		default:
			if (128 < currChar() && is_UNICODE_XID_Start(currChar())) {
				read_XID_CONTINUE();
				return 1;
			}
			return 0;
	}
	if (is_XID_Continue(currChar())) {
		read_XID_CONTINUE();
	}
	return 1;
}
function ptTree() {
	switch (uc_eat()) {
		case 33:
			return maybe_step_over(61) ? 16 : 43;
		case 35:
			return 41;
		case 36:
			return 31;
		case 37:
			return maybe_step_over(61) ? 25 : 9;
		case 38:
			switch (currChar()) {
				case 38:
					return step(), 2;
				case 61:
					return step(), 26;
				default:
					return 10;
			}
		case 42:
			return maybe_step_over(61) ? 23 : 7;
		case 43:
			return maybe_step_over(61) ? 21 : 5;
		case 44:
			return 37;
		case 45:
			switch (currChar()) {
				case 61:
					return step(), 22;
				case 62:
					return step(), 45;
				default:
					return 6;
			}
		case 46:
			if (maybe_step_over(46)) {
				switch (currChar()) {
					case 46:
						return step(), 35;
					case 61:
						return step(), 36;
					default:
						return 34;
				}
			} else {
				return 1;
			}
		case 47:
			return maybe_step_over(61) ? 24 : 8;
		case 58:
			return maybe_step_over(58) ? 40 : 39;
		case 59:
			return 38;
		case 60:
			switch (currChar()) {
				case 60:
					return step(), maybe_step_over(61) ? 29 : 13;
				case 61:
					return step(), 20;
				default:
					return 19;
			}
		case 61:
			switch (currChar()) {
				case 61:
					return step(), 15;
				case 62:
					return step(), 44;
				default:
					return 4;
			}
		case 62:
			switch (currChar()) {
				case 61:
					return step(), 18;
				case 62:
					return step(), maybe_step_over(61) ? 30 : 14;
				default:
					return 17;
			}
		case 63:
			return 42;
		case 64:
			return 32;
		case 94:
			return maybe_step_over(61) ? 28 : 12;
		case 95:
			return 33;
		case 124:
			switch (currChar()) {
				case 61:
					return step(), 27;
				case 124:
					return step(), 3;
				default:
					return 11;
			}
		case 126:
			return 46;
		default:
			edgecase_stepback();
			exit3.unexpected();
	}
}
// src/parser/state/index.ts
var src = void 0;
var src_text = "";
var pos = 0;
var line = 0;
var CharCode_INIT = 0;
var CharCode_EOF = 1;
var _cc = CharCode_INIT;
var IS_PARSING = () => CharCode_INIT !== _cc;
var GET_SOURCE = () => src;
var GET_LENGTH2 = () => src.code.length;
var GET_SOURCETEXT = () => (src == null ? void 0 : src.code) ?? "";
var GET_SOURCEFILEPATH = () => (src == null ? void 0 : src.filepath) ?? "";
var GET_POSITION = () => pos;
var GET_KEYWORD_NAME = (kw2) => ToKeyword[kw2];
function ccAt(index) {
	return src_text.charCodeAt(index);
}
var _ccAt = (index) => {
	return (_cc = ccAt(index));
};
var _ccSet = (pos2) => {
	_ccAt(pos2);
};
var _setPos = (npos) => {
	_ccAt((pos = npos));
};
var currChar = () => {
	return _cc;
};
function __create_save_state() {
	return [pos, line, kw, kw_endpos, _get_ws_preskip_pos(), _get_ws_postskip_pos()];
}
function __restore_init_state() {
	_cc = CharCode_INIT;
	pos = 0;
	line = 0;
	kw = 0;
	kw_endpos = -1;
	setPreSkipWhitespace(0);
	setPostSkipWhitespace(0);
	ATTRIBUTES_INNER.length = 0;
	ATTRIBUTES_OUTER.length = 0;
	ATTRIBUTES_DANGLING.length = 0;
	COMMENTS.length = 0;
}
function __restore_save_state(state) {
	if (pos !== state[0]) {
		___restorePos(state[0]);
		line = state[1];
		kw = state[2];
		kw_endpos = state[3];
		setPreSkipWhitespace(state[4]);
		setPostSkipWhitespace(state[5]);
	}
}
function __restore_state_to_pos(target) {
	___restorePos(target);
	if (pos > kw_endpos) kw_endpos = -1;
	while (src.lineStarts[line] > pos) --line;
}
function ___restorePos(target) {
	_setPos(target);
	__discardAfterPos(target);
}
function __discardAfterPos(target) {
	discard(COMMENTS);
	discard(ATTRIBUTES_DANGLING);
	discard(ATTRIBUTES_INNER);
	discard(ATTRIBUTES_OUTER);
	function discard(arr) {
		while (0 !== arr.length && start(last_of(arr)) >= target) {
			arr.pop();
		}
	}
}
var EDGECASE_STEPBACK_TO = (target) => {
	if (pos !== end(target)) {
		__restore_state_to_pos(end(target));
		skip_whitespace();
	}
};
var EDGECASE_STEPBACK_TO_POS = (target) => {
	if (pos !== target) {
		__restore_state_to_pos(target);
		skip_whitespace();
	}
};
var SNIPPET_endAt = (endpos) => {
	if (pos !== endpos) {
		if (endpos !== getPreWhitespaceSkipPosition()) {
			exit3.at(getPreWhitespaceSkipPosition(), "Snippet failed to parse all tokens");
		}
		__discardAfterPos(endpos);
	}
};
var match = (char) => char === currChar();
var match_2 = (char_0, char_1) => match(char_0) && peek_match(1, char_1);
var match_3 = (char_0, char_1, char_2) => match_2(char_0, char_1) && peek_match(2, char_2);
var not_match = (char) => char !== currChar();
var peek = (n) => ccAt(n + pos);
var peek_match = (n, char_n) => char_n === peek(n);
var peek_not_match = (n, char_n) => char_n !== peek(n);
var step = () => _ccSet(++pos);
var nStep = (n) => _ccSet((pos += n));
var step_ws = () => (step(), skip_whitespace());
var nStep_ws = (n) => (nStep(n), skip_whitespace());
var step_until_match = (char) => {
	if (match(CharCode_EOF)) exit3.unexpected();
	loop: while (true) {
		switch (currChar()) {
			case char:
				break loop;
			case 10:
				safe_step_eol();
				break;
			default:
				step();
				break;
		}
	}
};
var step_until_ln = () => {
	if (match(CharCode_EOF)) exit3.unexpected();
	while (not_match(10)) step();
};
var step_over = (char) => {
	if (match(char)) step();
	else exit3.expected(char);
};
var step_over_3 = (char_0, char_1, char_2) => {
	if (match_3(char_0, char_1, char_2)) nStep(3);
	else exit3.expected(strToken([char_0, char_1, char_2]));
};
var safe_step_over = (char) => {
	step();
};
var safe_step_over_2 = (word) => {
	nStep(2);
};
var maybe_step_over = (char) => match(char) && (step(), true);
var read = (char) => {
	if (match(char)) step_ws();
	else exit3.expected(char);
};
var read_2 = (char_0, char_1) => {
	if (!maybe_read_2(char_0, char_1)) exit3.expected(strToken([char_0, char_1]));
};
var maybe_read = (char_0) => match(char_0) && (step_ws(), true);
var maybe_read_2 = (char_0, char_1) => match(char_0) && maybe_skip_1_read_2(char_1);
function maybe_skip_1_read_2(char_1) {
	switch (peek(1)) {
		case char_1:
			nStep_ws(2);
			return true;
		case 9:
		case 10:
		case 11:
		case 12:
		case 13:
		case 32:
		case 133:
		case 8206:
		case 8207:
		case 8232:
		case 8233:
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
var match_keyword = (keyword) => keyword === peek_keyword();
var not_match_keyword = (keyword) => keyword !== peek_keyword();
var read_keyword = (kw2) => {
	if (match_keyword(kw2)) safe_skip_keyword();
	else exit3.expected(`Keyword '${GET_KEYWORD_NAME(kw2)}'`);
};
var maybe_read_keyword = (kw2) => match_keyword(kw2) && (safe_skip_keyword(), true);
var safe_skip = (char_0) => {
	safe_step_over();
	skip_whitespace();
};
var safe_skip_word = (WORD) => {
	nStep_ws(WORD.length);
};
var safe_skip_1_read_2 = (char_0, char_1) => {
	if (peek_match(1, char_1)) {
		nStep_ws(2);
	} else {
		safe_skip();
		if (match(char_1)) safe_skip();
		else exit3.expected(strToken([char_0, char_1]), getPreWhitespaceSkipPosition() - 1);
	}
};
function read_between(char_start, read_content, char_end) {
	safe_skip();
	const res = read_content();
	read(char_end);
	return res;
}
var will_match_charLiteral_not_lt = () => {
	return peek_match(2, 39) || peek_match(1, 92) || !is_XID_Start(peek(1));
};
var will_match_lt = () => match(39) && !will_match_charLiteral_not_lt();
var FOR_EACH_UNTIL = (CHAR_SEP, __RUN__, CHAR_END) => {
	loop: while (match(CHAR_SEP)) {
		safe_skip();
		switch (currChar()) {
			case CHAR_END:
			case CharCode_EOF:
				break loop;
		}
		__RUN__();
	}
	if (match(CHAR_END)) safe_skip();
	else exit3.expected([CHAR_SEP, CHAR_END]);
};
var FOR_ANY_BETWEEN = (CHAR_START, __RUN__, CHAR_END) => {
	safe_skip();
	loop: while (true) {
		switch (currChar()) {
			case CHAR_END:
				break loop;
			case CharCode_EOF:
				exit3.expected(CHAR_END);
		}
		__RUN__();
	}
	safe_skip();
};
function readLocatedArrayNoDelim(fn) {
	const ARRAY = createLocArray(0, new Loc(src, 0, 0));
	fn(ARRAY);
	if (ARRAY.length === 0) {
		setRange(ARRAY, pos, pos);
	} else {
		setRange(ARRAY, start(ARRAY[0]), getPreWhitespaceSkipPosition());
	}
	return ARRAY;
}
function readLocatedArrayDelim(dk, fn) {
	const ARRAY = createLocArray(dk, new Loc(src, pos, 0));
	fn(ARRAY);
	setRangeEnd(ARRAY, getPreWhitespaceSkipPosition());
	return ARRAY;
}
var read_sequence = (DK, EACH) => {
	const SEQUENCE = createLocArray(DK, new Loc(src, pos, 0));
	safe_skip(getDelimStartCharCode(DK));
	const END = getDelimEndCharCode(DK);
	LOOP: {
		while (not_match(END)) {
			SEQUENCE.push(EACH(SEQUENCE));
			switch (currChar()) {
				default:
					exit3.expected([COMMA, END]);
				case END:
					__read_sequence_lastHadTrailingSeparator = false;
					break LOOP;
				case COMMA:
					step_ws();
					break;
			}
		}
		__read_sequence_lastHadTrailingSeparator = true;
	}
	__set_endPos_eat(SEQUENCE);
	return SEQUENCE;
};
var read_group = (SELF, DK, EACH) => {
	const SEQUENCE = createLocArray(DK, new Loc(src, pos, 0));
	switch (SELF.nodeType) {
		case 4:
			SELF.ast = SEQUENCE;
			break;
		case 87:
			SELF.cases = SEQUENCE;
			break;
		default:
			SELF.body = SEQUENCE;
			break;
	}
	const END = getDelimEndCharCode(DK);
	read(getDelimStartCharCode(DK));
	LOOP: while (true) {
		maybe_read_inner_attributes(SELF);
		switch (currChar()) {
			case END:
				break LOOP;
			case CharCode_EOF:
				exit3.expected(END);
		}
		SEQUENCE.push(with_outerAttributes_fromStatementContext(() => EACH()));
	}
	__set_endPos_eat(SEQUENCE);
};
var read_group_noGroup = (SELF) => {
	read(123);
	maybe_read_inner_attributes(SELF);
	if (!maybe_read(125)) {
		if (match(CharCode_EOF)) exit3.expected(125);
		else exit3(`${SELF.type} cannot define items, expected '}'`);
	}
};
var read_group_noDelim = (SELF, EACH) => {
	const SEQUENCE = (SELF.ast = createLocArray(0, new Loc(src, pos, pos)));
	maybe_read_inner_attributes(SELF);
	if (not_match(CharCode_EOF)) {
		do {
			SEQUENCE.push(with_outerAttributes_fromStatementContext(() => EACH()));
			maybe_read_inner_attributes(SELF);
		} while (not_match(CharCode_EOF));
		setRange(SEQUENCE, start(SEQUENCE[0]), getPreWhitespaceSkipPosition());
	}
};
var sequence_hasTrailingComma = (sequence) => {
	return __read_sequence_lastHadTrailingSeparator;
};
var COMMA = 44;
var __read_sequence_lastHadTrailingSeparator = false;
var kw = 0;
var kw_endpos = -1;
function peek_keyword() {
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
var invalidate_kw = () => {
	kw_endpos = -1;
};
function read_identifier_with(char) {
	safe_step_over();
	if (!is_XID_Start(currChar())) exit3.expected("Identifier");
	read_XID_CONTINUE();
}
function read_XID_CONTINUE() {
	while (is_XID_Continue(uc_next()));
}
function safe_skip_keyword(keyword) {
	_setPos(kw_endpos);
	invalidate_kw();
	skip_whitespace();
}
var read_cached_keyword = () => safe_skip_keyword();
var __patch_startPos = (startPos, target) => (setRangeStart(target, startPos), target);
var __patch_endPos = (endPos, target) => (setRangeEnd(target, endPos), target);
var read_ahead = (fn) => __patch_startPos(pos, fn());
var CCPATH_read = (PATH_NODE) => __patch_startPos(pos, (safe_skip_1_read_2(58, 58), new PATH_NODE(void 0)));
var withStart = (head, target) => __patch_startPos(start(head), target);
var withEnd = (tail, target) => __patch_endPos(end(tail), target);
var withRange = (target, head, tail) => (setRangeStart(target, start(head)), __patch_endPos(end(tail), target));
var __set_endPos_eat = (target, char) => {
	setRangeEnd(target, 1 + pos);
	safe_skip();
};
function read_ahead_extern(fn) {
	const startPos = pos;
	const node = fn();
	if (!is_MaybeExternNode(node)) exit3.at(node, "Expected `extern` target");
	setRangeStart(node.extern, startPos);
	setRangeStart(node, startPos);
	return node;
}
function read_ahead_maybe_extern(fn) {
	const startPos = pos;
	const node = fn();
	if ("extern" in node) {
		setRangeStart(node.extern, startPos);
		setRangeEnd(node.extern, node.extern.abi ? end(node.extern.abi) : startPos + 6);
	}
	setRangeStart(node, startPos);
	return node;
}
function check_ahead(check) {
	const state = __create_save_state();
	const res = check();
	__restore_save_state(state);
	return res;
}
function read_ahead_either(fn_0, fn_1) {
	const state = __create_save_state();
	return read_ahead(() => fn_0() ?? (__restore_save_state(state), fn_1()));
}
function FG_property(feature, node, key, read_value) {
	return read_value();
}
function FG_property_true(feature, node, key) {
	return true;
}
function withParserState(SOURCE, START_POS, READ_AST) {
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
		return PROGRAM;
	} finally {
		src = void 0;
		src_text = "";
		__restore_init_state();
		__es_optional_start = -1;
		__ctx_ES_PRCD_i = 0;
		__ctx_ES_i = 0;
		__ctx_TY_i = 0;
		__ctx_MC_i = 0;
	}
}
function withLF(code) {
	return 10 === code.charCodeAt(code.length - 1) ? code : code + "\n";
}
var __es_optional_start = -1;
var ES_signal_optional_read = () => (__es_optional_start = pos);
var ES_consume_optional_read = () => __es_optional_start === ((__es_optional_start = -1), pos);
var __ctx_ES_i = 0;
var __ctx_ES_PRCD_i = 0;
var __ctx_Precedence = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
var __ctx_exceptStructFormExpression = [false, false, false, false, false, false, false, false, false, false, false, false];
var __ctx_insideScrutinee = [false, false, false, false, false, false, false, false, false, false, false, false];
var ES_ctx_precedence_pop = () => --__ctx_ES_PRCD_i;
var ES_ctx_precedence_push = (prcd) => (__ctx_Precedence[++__ctx_ES_PRCD_i] = prcd);
var ES_ctx_setCurrentPrecedence = (prcd) => (__ctx_Precedence[__ctx_ES_PRCD_i] = prcd);
var ES_ctx_hasHigherPrecedence = (prcd) => prcd > __ctx_Precedence[__ctx_ES_PRCD_i - 1];
var ES_ctx_exceptStructFormExpression = () => __ctx_exceptStructFormExpression[__ctx_ES_i];
var ES_ctx_insideScrutinee = () => __ctx_insideScrutinee[__ctx_ES_i];
var ES_withContext = (exceptStructFormExpression, insideScrutinee, fn) => {
	__ctx_exceptStructFormExpression[++__ctx_ES_i] = exceptStructFormExpression;
	__ctx_insideScrutinee[__ctx_ES_i] = insideScrutinee;
	const res = ES_withPrecedence(insideScrutinee ? 0 : 3, fn);
	--__ctx_ES_i;
	return res;
};
var ES_withPrecedence = (prcd, fn) => {
	ES_ctx_precedence_push(prcd);
	const res = fn();
	ES_ctx_precedence_pop();
	return res;
};
function rhs_resolve(RHS_next, PRCD_next, RHS_length) {
	if (ES_ctx_hasHigherPrecedence(PRCD_next)) {
		ES_ctx_setCurrentPrecedence(4 === PRCD_next ? 4 - 1 : PRCD_next);
		nStep_ws(RHS_length);
		return RHS_next;
	}
	return 0;
}
var __ctx_TY_i = 0;
var __ctx_allowMultipleBounds = [false, false, false, false, false, false, false, false, false, false, false, false];
var TY_ctx_allowMultipleBounds = () => __ctx_allowMultipleBounds[__ctx_TY_i];
var TY_ctx_allowMultipleBounds_set = (mB) => (__ctx_allowMultipleBounds[++__ctx_TY_i] = mB);
var TY_ctx_allowMultipleBounds_pop = () => --__ctx_TY_i;
var TY_withContext = (fn, allowMultipleBounds) => {
	TY_ctx_allowMultipleBounds_set(allowMultipleBounds);
	const res = fn();
	TY_ctx_allowMultipleBounds_pop();
	return res;
};
var _env = 0;
var getEnv = () => _env;
var setEnv = (env) => (_env = env);
var isReadingEnv = (env) => env === _env;
var __ctx_MC_i = 0;
var Mc_ctx_isReadingTokens = () => 0 !== __ctx_MC_i;
var MC_ctx_isReadingTokens_start = () => ++__ctx_MC_i;
var MC_ctx_isReadingTokens_end = () => --__ctx_MC_i;
var uc_eat = () => {
	const cc = _cc;
	return step(), cc;
};
var uc_next = () => _ccAt(++pos);
var uc_next_match = (char) => char === uc_next();
function kw_resolve(kw2) {
	if (is_XID_Continue(uc_next())) {
		read_XID_CONTINUE();
		return 1;
	}
	return kw2;
}
function kw_resolve_failed(kw2) {
	if (is_XID_Continue(currChar())) {
		read_XID_CONTINUE();
		return 1;
	}
	return kw2;
}
var edgecase_stepback = () => _ccSet(--pos);
var COMMENTS = [];
var ATTRIBUTES_DANGLING = [];
var ATTRIBUTES_INNER = [];
var ATTRIBUTES_OUTER = [];
var _setEOF = () => {
	src_text = src.code;
	pos = src.code.length;
	_cc = CharCode_EOF;
};
var step_eol = () => {
	if (++pos === src_text.length) _setEOF();
	else {
		_ccSet((src.lineStarts[++line] = pos));
	}
};
var safe_step_eol = () => {
	step_eol();
	if (match(CharCode_EOF)) exit3.unexpected();
};
var register_comment = (attr) => COMMENTS.push(attr);
var register_attribute = (attr) => (attr.inner ? ATTRIBUTES_INNER : ATTRIBUTES_OUTER).push(attr);
function __may_claim_attributes(attrs) {
	if (0 !== attrs.length && didJustSkipWhitespace()) {
		while (start(attrs[0]) < _get_ws_preskip_pos()) {
			insertNode(ATTRIBUTES_DANGLING, attrs.shift());
			if (0 === attrs.length) return false;
		}
		return true;
	}
	return false;
}
function skip_whitespace_getProgramStartPos() {
	skip_whitespace();
	return Math.min(
		pos,
		0 === COMMENTS.length ? pos : start(COMMENTS[0]),
		0 === ATTRIBUTES_INNER.length ? pos : start(ATTRIBUTES_INNER[0]),
		0 === ATTRIBUTES_OUTER.length ? pos : start(ATTRIBUTES_OUTER[0]),
		0 === ATTRIBUTES_DANGLING.length ? pos : start(ATTRIBUTES_DANGLING[0])
	);
}
function getProgramEndPos(program) {
	return Math.max(
		getPreWhitespaceSkipPosition(),
		0 === COMMENTS.length ? 0 : end(last_of(COMMENTS)),
		0 === ATTRIBUTES_INNER.length ? 0 : end(last_of(ATTRIBUTES_INNER)),
		0 === ATTRIBUTES_OUTER.length ? 0 : end(last_of(ATTRIBUTES_OUTER)),
		0 === ATTRIBUTES_DANGLING.length ? 0 : end(last_of(ATTRIBUTES_DANGLING)),
		!hasAttributes(program) ? 0 : end(last_of(program.attributes))
	);
}
function maybe_read_inner_attributes(target) {
	if (__may_claim_attributes(ATTRIBUTES_INNER)) {
		assignAttributes(target, ATTRIBUTES_INNER);
		ATTRIBUTES_INNER.length = 0;
	}
}
function with_outerAttributes(fn) {
	if (__may_claim_attributes(ATTRIBUTES_OUTER)) {
		const attributes = spliceAll(ATTRIBUTES_OUTER);
		const target = fn();
		assignAttributes(target, attributes);
		return target;
	}
	return fn();
}
function with_outerAttributes_fromParentContext_if_test(fn, accepts_args) {
	if (__may_claim_attributes(ATTRIBUTES_OUTER)) {
		const attributes = spliceAll(ATTRIBUTES_OUTER);
		const target = fn();
		if (accepts_args(target)) {
			assignAttributes(target, attributes);
		} else {
			insertNodes(ATTRIBUTES_DANGLING, attributes);
		}
		return target;
	}
	return fn();
}
function with_outerAttributes_fromStatementContext(fn) {
	return with_outerAttributes(fn);
}
function with_outerAttributes_fromParentContext(fn) {
	return with_outerAttributes(fn);
}
function FG_with_outerAttributes_fromParentContext2(fn) {
	if (__may_claim_attributes(ATTRIBUTES_OUTER)) {
		const attributes = spliceAll(ATTRIBUTES_OUTER);
		const target = fn();
		assignAttributes(target, attributes);
		return target;
	}
	return fn();
}
// src/parser/error.ts
function format_expectation(data) {
	switch (typeof data) {
		case "number":
			switch (data) {
				case 34:
					return `'"'`;
				case 39:
					return `"'"`;
				default:
					return `"${print_string(String.fromCharCode(data))}"`;
			}
		case "string":
			return data;
		case "object":
			return pretty_join(data.map(format_expectation).filter(Boolean), { tail: "or", empty: "undefined" });
	}
}
function name_thing(pos2) {
	if (pos2 >= GET_LENGTH2() - 1) return "End Of File";
	if (GET_POSITION() !== pos2) nStep(pos2 - GET_POSITION());
	return check_ahead(() => {
		const kw2 = kwTree();
		switch (kw2) {
			case 0:
				return `token '${strChar(currChar())}'`;
			case 1:
				return `Identifier`;
			default:
				const name = GET_KEYWORD_NAME(kw2);
				if (name[0].toUpperCase() === name[0]) return name;
				return `keyword '${name}'`;
		}
	});
}
function exit3(err, ...ctx) {
	exit3.at(GET_POSITION(), err, ...ctx);
}
((exit4) => {
	function at(pos2, msg, ...ctx) {
		const code = GET_SOURCETEXT();
		const { text, loc } = printCodeError(msg, code, pos2, GET_SOURCEFILEPATH());
		const error = createCustomError({
			message: msg,
			editStack(stack) {},
			style: { callee: (callee) => (callee.endsWith(".read") || callee.startsWith("new ") ? color.blue : color.cyan) },
		});
		error.loc = loc;
		error.ctx = ctx;
		error.toString = function () {
			return text;
		};
		throw error;
	}
	exit4.at = at;
	function never2() {
		exit4("Reached unreachable code");
	}
	exit4.never = never2;
	function infinite() {
		exit4("Maximum call stack size exceeded");
	}
	exit4.infinite = infinite;
	function expected(expected2 = "", pos2 = GET_POSITION()) {
		at(pos2, `Unexpected ${name_thing(pos2)}` + (expected2 && `, expected ${format_expectation(expected2)}`));
	}
	exit4.expected = expected;
	function unexpected() {
		expected();
	}
	exit4.unexpected = unexpected;
})(exit3 || (exit3 = {}));
__SET_PARSER_ERROR_MNGR(function (msg, ctx) {
	if (IS_PARSING()) exit3(msg, ...ctx);
});
function assert7(predicate, err, ...ctx) {
	if (false === predicate) exit3(err || "Assertion failed", ...ctx);
}
((assert10) => {
	function at(pos2, predicate, err, ...ctx) {
		if (false === predicate) exit3.at(pos2, err || "Assertion failed", ...ctx);
	}
	assert10.at = at;
	function nis2(node, nodeType) {
		if (nodeType !== node.nodeType) exit3.at(node, `Expected ${NodeType[nodeType]}, found ${NodeType[node.nodeType]}`);
	}
	assert10.nis = nis2;
	function match2(data) {
		expect(data === currChar());
	}
	assert10.match = match2;
	function expect(predicate, expected) {
		if (false === predicate) exit3.expected(expected);
	}
	assert10.expect = expect;
})(assert7 || (assert7 = {}));
function printCodeError(message, code, pos2, filepath) {
	const startPos = typeof pos2 === "number" ? pos2 : start(pos2);
	const lineStarts = getLineStarts(code);
	const { line: line2, char } = safe_getLineChar(code, lineStarts, startPos);
	const SEPARATOR = "\n" + "-".repeat(getTerminalWidth()) + "\n";
	const url = urlAt(filepath ?? "undefined", lineStarts, startPos);
	return {
		text:
			color.grey(
				"" +
					SEPARATOR +
					stringifyLines(
						line2 - 2,
						line2 - 1,
						...(hasIndex(code, startPos)
							? [
									{ line: line2, color: color.yellow },
									...(char + 2 + color.unstyledLength(message) > getTerminalWidth() - 8
										? [" ".repeat(char) + "^", message, ""]
										: [" ".repeat(char) + "^ " + message]
									).map((str) => color.red(str)),
							  ]
							: [line2, color.red(`x---- ${message}`)]),
						line2 + 1,
						line2 + 2
					) +
					SEPARATOR
			) +
			`ParserError at ${color.blue(color.underline(url))}
`,
		loc: {
			url,
			start: {
				line: line2 + 1,
				column: char + 1,
			},
		},
	};
	function hasIndex(data, i) {
		return 0 <= i && data.length > i;
	}
	function safe_getLineChar(code2, lineStarts2, pos3) {
		return hasIndex(code2, pos3) ? getLineChar(lineStarts2, pos3) : { line: pos3 < 0 ? -1 : lineStarts2.length, char: 0 };
	}
	function stringifyLines(...plines) {
		const dlines = plines.map((data) =>
			typeof data === "number"
				? [data, printLine(data)]
				: typeof data === "string"
				? [-1, data]
				: [data.line, data.color(printLine(data.line))]
		);
		const pre = padEndUniform(dlines.map((data) => "" + (1 + data[0] || "")));
		return dlines
			.map((data, i) => {
				let str = `  ${pre[i]} ${/[^ ]/.test(pre[i]) && !!data[1] ? "|" : " "} ${data[1] ?? ""}`;
				return data[0] !== -1 && data[0] !== line2 && color.unstyledLength(str) > getTerminalWidth()
					? str.slice(0, getTerminalWidth() - 4) + "..."
					: str;
			})
			.join("\n");
		function padEndUniform(arr) {
			const actualLengths = arr.map(color.unstyledLength);
			const topLength = Math.max(...actualLengths);
			return arr.map((str, i) => str + " ".repeat(topLength - actualLengths[i]));
		}
		function printLine(i) {
			return hasIndex(lineStarts, i) ? print_string(sliceLine(code, lineStarts, i)) : "";
		}
		function sliceLine(str, lineStarts2, i) {
			return hasIndex(lineStarts2, i)
				? str.slice(lineStarts2[i], i + 1 === lineStarts2.length ? str.length : lineStarts2[i + 1])
				: "";
		}
	}
}
// src/parser/options.ts
var defaultParserOptions = { filepath: void 0 };
function checkOptions(O) {
	for (var k in O)
		k in defaultParserOptions ||
			exit3(
				`Unknown parser option "${k}"
Valid options: ${pretty_join(Object.keys(defaultParserOptions), { quote: true })}`,
				O
			);
}
// src/parser/read/index.ts
function read_maybe_missing(fn) {
	return fn() ?? new MissingNode();
}
function read_identifier_token() {
	return new Identifier();
}
function read_Index() {
	assert7.expect(is_number(currChar()));
	return new Index();
}
function read_Identifier_or_Index() {
	return is_number(currChar()) ? new Index() : new Identifier();
}
function read_ccPath_or_Identifier(PATH_NODE) {
	return match(58) ? CCPATH_read(PATH_NODE) : new Identifier();
}
function read_Identifier_or_ItemPath_unbound() {
	let lhs = read_ccPath_or_Identifier(ItemPath);
	while (match(58)) {
		safe_skip_1_read_2(58, 58);
		lhs = new ItemPath(lhs);
	}
	return lhs;
}
// src/parser/read/literals.ts
var safe_step_over_number_us = () => {
	step();
};
function read_literal() {
	switch (peek_keyword()) {
		case 7:
			safe_skip_keyword();
			return 1;
		case 8:
			safe_skip_keyword();
			return 0;
		case 5:
			switch (currChar()) {
				case 98: {
					switch (peek(1)) {
						case 39:
							read_bChar();
							return 3;
						case 34:
							read_bString();
							return 4;
						case 114:
							read_brString();
							return 5;
					}
					exit3.never();
				}
				case 114:
					read_rString();
					return 6;
			}
			exit3.never();
		case 0:
			switch (currChar()) {
				case 34:
					read_String();
					return 7;
				case 39:
					read_Char();
					return 2;
				case 48:
					switch (peek(1)) {
						case 98:
							safe_step_over_2();
							while (is_bin(currChar()) || match(95)) step();
							return 8;
						case 111:
							safe_step_over_2();
							while (is_oct(currChar()) || match(95)) step();
							return 10;
						case 120:
							safe_step_over_2();
							while (is_hex(currChar()) || match(95)) step();
							return 9;
						default:
							return read_number();
					}
				case 49:
				case 50:
				case 51:
				case 52:
				case 53:
				case 54:
				case 55:
				case 56:
				case 57:
					return read_number();
			}
	}
	exit3.never();
}
function read_number() {
	step();
	while (true) {
		switch (currChar()) {
			case 95:
			case 48:
			case 49:
			case 50:
			case 51:
			case 52:
			case 53:
			case 54:
			case 55:
			case 56:
			case 57:
				safe_step_over_number_us();
				break;
			case 101:
			case 69:
				read_exponent();
				return 12;
			case 46:
				switch (peek(1)) {
					case 46:
						return 11;
					case 48:
					case 49:
					case 50:
					case 51:
					case 52:
					case 53:
					case 54:
					case 55:
					case 56:
					case 57:
						read_decimals();
						return 12;
					default:
						if (is_XID_Start(peek(1))) {
							return 11;
						} else {
							safe_step_over();
							return 12;
						}
				}
			default:
				return 11;
		}
	}
}
function read_decimals() {
	safe_step_over();
	step();
	while (true) {
		switch (currChar()) {
			case 95:
			case 48:
			case 49:
			case 50:
			case 51:
			case 52:
			case 53:
			case 54:
			case 55:
			case 56:
			case 57:
				safe_step_over_number_us();
				break;
			case 101:
			case 69:
				read_exponent();
				return;
			default:
				return;
		}
	}
}
function read_exponent() {
	step();
	switch (currChar()) {
		case 43:
		case 45:
			step();
	}
	loop: while (true) {
		switch (currChar()) {
			case 95:
			case 48:
			case 49:
			case 50:
			case 51:
			case 52:
			case 53:
			case 54:
			case 55:
			case 56:
			case 57:
				safe_step_over_number_us();
				break;
			default:
				break loop;
		}
	}
}
function read_escaped(kind) {
	safe_step_over();
	switch (currChar()) {
		case 13:
		case 10:
			break;
		case 34:
		case 39:
		case 48:
		case 92:
		case 110:
		case 114:
		case 116:
			step();
			break;
		case 120:
			switch (kind) {
				case 7:
				case 2:
					safe_step_over(), read_OCT(), read_HEX();
					break;
				default:
					safe_step_over(), read_HEX(), read_HEX();
					break;
			}
			break;
		case 117:
			safe_step_over();
			step_over(123);
			while (is_hex(currChar()) || match(95)) step();
			step_over(125);
			break;
		default:
			exit3("Invalid escape");
			break;
	}
}
function read_String() {
	safe_step_over();
	loop: while (true) {
		switch (currChar()) {
			case 10:
				safe_step_eol();
				break;
			case 34:
				break loop;
			case 92:
				read_escaped(7);
				break;
			default:
				step();
				break;
		}
	}
	safe_step_over();
}
function read_rString() {
	safe_step_over();
	if (maybe_step_over(35)) {
		let i = 0;
		let ht = 1;
		while (maybe_step_over(35)) ++ht;
		safe_step_over();
		while (true) {
			step_until_match(34), safe_step_over();
			while (maybe_step_over(35)) if (++i === ht) return;
			i = 0;
		}
	} else {
		safe_step_over();
		step_until_match(34);
		safe_step_over();
	}
}
function read_brString() {
	safe_step_over();
	read_rString();
}
function read_Char() {
	safe_step_over();
	switch (currChar()) {
		case 39:
		case 10:
		case 13:
		case 9:
			exit3.expected("character");
		case 92:
			read_escaped(2);
			break;
		default:
			for (let i = 0; i < 8; i++) {
				step();
				if (match(39)) break;
			}
			break;
	}
	read(39);
}
function read_bChar() {
	safe_step_over_2();
	switch (currChar()) {
		case 39:
		case 10:
		case 13:
		case 9:
			exit3.expected("character");
		case 92:
			read_escaped(3);
			break;
		default:
			step();
			break;
	}
	read(39);
}
function read_bString() {
	safe_step_over_2();
	loop: while (true) {
		switch (currChar()) {
			case 10:
				safe_step_eol();
				break;
			case 92:
				read_escaped(4);
				break;
			case 34:
				break loop;
			default:
				step();
				break;
		}
	}
	safe_step_over();
}
var read_OCT = () => {
	if (is_oct(currChar())) {
		step();
	} else {
		exit3.expected("oct digit /0-7/");
	}
};
var read_HEX = () => {
	if (is_hex(currChar())) {
		step();
	} else {
		exit3.expected("hex digit /0-9A-Fa-f/");
	}
};
// src/parser/read/specifiers.ts
function read_with_pub_specifier(fn) {
	return read_ahead(() => {
		const pub = new PubSpecifier();
		const node = fn();
		if (!is_MaybePubNode(node)) exit3.at(node, d`Expected 'pub' target, not ${node}`);
		node.pub = pub;
		return node;
	});
}
function will_match_crate_specifier() {
	return peek_not_match("crate".length, 58);
}
function maybe_read_with_pub_specifier(fn) {
	switch (peek_keyword()) {
		case 25:
			return read_with_pub_specifier(fn);
		case 45:
			return will_match_crate_specifier() ? read_with_pub_specifier(fn) : fn();
		default:
			return fn();
	}
}
function maybe_read_abi() {
	return match(34) || match_keyword(5) ? new Literal() : void 0;
}
function read_with_extern_specifier(fn) {
	return read_ahead_extern(() => {
		safe_skip_keyword();
		const extern = new ExternSpecifier(maybe_read_abi());
		const node = fn();
		if (!is_MaybeExternNode(node)) exit3.at(node, d`'extern' cannot be used with ${node}`);
		node.extern = extern;
		return node;
	});
}
function apply_const_modifier(node) {
	switch (node.nodeType) {
		case 25:
			node.expression.const = FG_property_true();
			break;
		case 81:
			node.const = FG_property_true();
			break;
		case 38:
			node.const = true;
			break;
		default:
			exit3.at(node, d`'const' cannot be used with ${node}`);
	}
	return node;
}
function read_with_const_modifier(read2) {
	return read_ahead(() => (safe_skip_keyword(), apply_const_modifier(read2())));
}
function read_with_async_modifier(read2) {
	return read_ahead(() => {
		safe_skip_keyword();
		const node = read2();
		if (!is_MaybeAsyncNode(node)) exit3.at(node, d`Expected 'async' target, found ${node}`);
		node.async = true;
		return node;
	});
}
function read_with_move_modifier(read2) {
	return read_ahead(() => {
		safe_skip_keyword();
		const node = read2();
		if (!is_MaybeMoveNode(node)) exit3.at(node, d`Expected 'move' target, found ${node}`);
		node.move = true;
		return node;
	});
}
function read_with_unsafe_modifier(read2) {
	return read_ahead(() => {
		safe_skip_keyword();
		const node = read2();
		if (!is_MaybeUnsafeNode(node)) exit3.at(node, d`Expected 'unsafe' target, found ${node}`);
		node.unsafe = true;
		return node;
	});
}
function read_with_static_modifier(read2) {
	return read_ahead(() => {
		safe_skip_keyword();
		const expr = read2();
		if (!is_MaybeStaticNode(expr)) exit3.at(expr, d`Expected 'static' target, found ${expr}`);
		expr.static = FG_property_true();
		return expr;
	});
}
// src/parser/read/patterns.ts
function maybe_read_RangePatternBoundEnd() {
	switch (currChar()) {
		case 40:
			return read_between(40, () => maybe_read_RangePatternBoundEnd(), 41);
		case 58:
			return read_PatternNamespaceTarget(CCPATH_read(ExpressionPath));
		case 60:
			return read_PatternNamespaceTarget(new ExpressionTypeSelector());
		case 45:
			return new MinusPattern(() => new Literal());
		case 34:
		case 39:
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 56:
		case 57:
			return new Literal();
	}
	switch (peek_keyword()) {
		case 5:
		case 8:
		case 7:
			return new Literal();
		case 20:
			return read_with_const_modifier(() => new BlockExpression());
		case 6:
		case 43:
		case 44:
		case 14:
		case 42:
		case 45:
		case 3:
		case 1:
			return read_PatternNamespaceTarget(new Identifier());
		default:
			return void 0;
	}
}
function read_PatternNamespaceTarget(namespace) {
	let lhs = namespace;
	loop: while (maybe_read_2(58, 58)) {
		switch (currChar()) {
			case 58:
				EDGECASE_STEPBACK_TO(lhs);
				break loop;
			case 60:
				lhs = new ExpressionTypeCast(lhs, read_TypeArguments());
				break;
			default:
				lhs = new ExpressionPath(lhs);
				break;
		}
	}
	return lhs;
}
function read_pattern_lhs() {
	switch (currChar()) {
		case 58:
			return read_PatternNamespaceTarget(CCPATH_read(ExpressionPath));
		case 60:
			return read_PatternNamespaceTarget(new ExpressionTypeSelector());
		case 38:
			return new ReferencePattern();
		case 40: {
			const items = read_sequence(1, (SEQUENCE) =>
				with_outerAttributes_fromParentContext_if_test(
					() => read_pattern(true),
					() => SEQUENCE.length > 0 || match(44)
				)
			);
			return items.length !== 1 || sequence_hasTrailingComma() || is_RestPattern(items[0])
				? new TuplePattern(void 0, items)
				: new ParenthesizedPattern(items);
		}
		case 91:
			return new ArrayPattern();
		case 46:
			return read_ahead(() => {
				safe_skip_1_read_2(46, 46);
				switch (currChar()) {
					case 44:
					case 41:
					case 93:
						return new RestPattern();
				}
				return new RangePattern(void 0);
			});
		case 124:
			return new UnionPattern(void 0);
		case 45:
			return new MinusPattern(() => new Literal());
		case 34:
		case 39:
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 56:
		case 57:
			return new Literal();
		default:
			switch (peek_keyword()) {
				case 5:
				case 8:
				case 7:
					return new Literal();
				case 20:
					return read_with_const_modifier(() => new BlockExpression());
				case 29:
				case 31:
					return new PatternVariableDeclaration(void 0);
				case 46:
					return new BoxPattern();
				case 2:
					return new WildcardPattern();
				default: {
					const id = new Identifier();
					return match(64) ? new PatternVariableDeclaration(id) : read_PatternNamespaceTarget(id);
				}
			}
	}
}
function read_PatternNoUnion_unstrict() {
	return read_pattern(true);
}
function read_pattern(allowUnionPattern) {
	return withEscapedParens(read_pattern_lhs(), (startNode) => {
		let lhs = startNode;
		loop: while (true) {
			switch (currChar()) {
				case 33:
					lhs = new MacroInvocation(lhs);
					break;
				case 123:
					lhs = new StructPattern(lhs);
					break;
				case 40:
					lhs = new TuplePattern(
						lhs,
						read_sequence(1, () => with_outerAttributes_fromParentContext(() => read_pattern(true)))
					);
					break;
				case 46:
					safe_skip_1_read_2(46, 46);
					lhs = new RangePattern(lhs);
					if (allowUnionPattern && match(124)) {
						lhs = new UnionPattern(lhs);
					}
					break loop;
				case 124:
					if (allowUnionPattern) {
						lhs = new UnionPattern(lhs);
					}
					break loop;
				default:
					break loop;
			}
		}
		return lhs;
	});
}
// src/parser/read/statements.ts
function read_expr_or_macroInvocation_stmt() {
	return new ExpressionStatement();
}
function safe_read_expr_stmt() {
	return new ExpressionStatement();
}
function read_import() {
	let lhs;
	switch (currChar()) {
		case 123:
			return new DestructuredImport(void 0);
		case 42:
			return new AmbientImport(void 0);
		case 58: {
			const res = read_ahead(() => {
				safe_skip_1_read_2(58, 58);
				switch (currChar()) {
					case 123:
						return new DestructuredImport(void 0);
					case 42:
						return new AmbientImport(void 0);
					default:
						return new ItemPath(void 0);
				}
			});
			if (is_ItemPath(res)) {
				lhs = res;
				break;
			} else {
				return res;
			}
		}
		default:
			lhs = new Identifier();
			break;
	}
	while (true) {
		if (match(58)) {
			safe_skip_1_read_2(58, 58);
			switch (currChar()) {
				case 123:
					return new DestructuredImport(lhs);
				case 42:
					return new AmbientImport(lhs);
				default:
					lhs = new ItemPath(lhs);
					break;
			}
		} else {
			return maybe_read_keyword(26)
				? match_keyword(2)
					? new AnonymousImport(lhs)
					: new NamedImport(lhs, new Identifier())
				: new NamedImport(lhs, void 0);
		}
	}
}
function read_function_parameter() {
	return match(46) ? new FunctionSpread() : new FunctionParameterDeclaration();
}
function read_struct_properties_declaration() {
	return read_sequence(3, () => maybe_read_with_attr_pub(() => new StructPropertyDeclaration()));
}
function read_struct_items_declaration() {
	return read_sequence(1, () => maybe_read_with_attr_pub(() => new TupleStructItemDeclaration()));
}
function read_struct_declaration() {
	return read_ahead(() => {
		safe_skip_keyword();
		const id = new Identifier();
		const generics = maybe_read_generics();
		return match(40) ? new TupleStructDeclaration(id, generics) : new StructDeclaration(id, generics);
	});
}
function read_TraitDeclaration() {
	return read_ahead(() => {
		safe_skip_keyword();
		const id = new Identifier();
		const generics = maybe_read_generics();
		return match(61) ? new TraitAliasDeclaration(id, generics) : new TraitDeclaration(id, generics);
	});
}
var _impl_startsWith_selector = false;
function read_impl() {
	return read_ahead(() => {
		safe_skip_keyword();
		if ((_impl_startsWith_selector = impl_match_selector_not_generics())) {
			return new ImplDeclaration(void 0);
		}
		const generics = maybe_read_generics();
		return match(33) && check_ahead(() => (safe_skip(), not_match(123) && not_match_keyword(32)))
			? new NegativeImplDeclaration(generics)
			: new ImplDeclaration(generics);
	});
}
function impl_match_selector_not_generics() {
	return (
		match(60) &&
		check_ahead(() => {
			safe_skip();
			switch (currChar()) {
				case 99:
					if (match_keyword(20)) return false;
					break;
				case 62:
					return false;
				case 39:
					safe_skip();
					break;
			}
			if (is_XID_Start(currChar())) {
				while (is_XID_Continue(peek(1))) step();
				safe_skip();
				switch (currChar()) {
					case 62:
					case 44:
					case 61:
						return false;
					case 58:
						if (maybe_skip_1_read_2(58)) {
							return not_match(58);
						}
						return false;
				}
			}
			return true;
		})
	);
}
function maybe_read_with_attr_pub(fn) {
	return with_outerAttributes_fromParentContext(() => maybe_read_with_pub_specifier(fn));
}
function read_statement() {
	switch (peek_keyword()) {
		case 23:
			return read_ahead_maybe_extern(() => {
				safe_skip_keyword();
				if (match_keyword(45)) {
					return new ExternCrateStatement();
				}
				const abi = maybe_read_abi();
				const unsafe = maybe_read_keyword(21);
				if (match(123)) {
					const stmt = new ExternBlockDeclaration(abi);
					if (unsafe) stmt.unsafe = true;
					return stmt;
				}
				if (match_keyword(9)) {
					const extern = new ExternSpecifier(abi);
					const stmt = new FunctionDeclaration();
					stmt.extern = extern;
					if (unsafe) stmt.unsafe = true;
					return stmt;
				}
				exit3("'extern' cannot be used here");
			});
		case 25:
			return read_with_pub_specifier(() => read_statement());
		case 45:
			return will_match_crate_specifier() ? read_with_pub_specifier(() => read_statement()) : read_expr_or_macroInvocation_stmt();
		case 22:
			return peek_match("async".length + 1, 123) ||
				check_ahead(() => (safe_skip_keyword(), maybe_read_keyword(24), match(123) || match(124)))
				? safe_read_expr_stmt()
				: read_with_async_modifier(() => read_statement());
		case 24:
			return safe_read_expr_stmt();
		case 21:
			return peek_match(7, 123) || check_ahead(() => (safe_skip_keyword(), match(123)))
				? safe_read_expr_stmt()
				: read_with_unsafe_modifier(() => read_statement());
		case 11:
			return new UseStatement();
		case 4:
			switch (check_ahead(() => (safe_skip_keyword(), safe_skip(), currChar()))) {
				case 40:
				case 91:
				case 123:
					return read_expr_or_macroInvocation_stmt();
				default:
					return new MacroRulesDeclaration();
			}
		case 53:
			return new MacroDeclaration();
		case 17:
			return new TypeAliasDeclaration();
		case 19:
			return !is_UpperCase(peek(7)) &&
				check_ahead(() => {
					safe_skip_keyword();
					switch (peek_keyword()) {
						case 22:
						case 24:
						case 19:
							return true;
						default:
							return match(124);
					}
				})
				? safe_read_expr_stmt()
				: new StaticVariableDeclaration();
		case 20:
			return read_ahead(() => {
				safe_skip_keyword();
				switch (peek_keyword()) {
					case 0:
						return match(123) ? apply_const_modifier(safe_read_expr_stmt()) : new ConstVariableDeclaration();
					case 21:
					case 23:
					case 22:
					case 9:
						return apply_const_modifier(read_statement());
					default:
						return new ConstVariableDeclaration();
				}
			});
		case 18:
			return new LetVariableDeclaration();
		case 10:
			return new ModuleDeclaration();
		case 9:
			return new FunctionDeclaration();
		case 12:
			return read_struct_declaration();
		case 14:
			return check_ahead(() => (safe_skip_keyword(), !match_keyword(0) && !match_keyword(26)))
				? new UnionDeclaration()
				: read_expr_or_macroInvocation_stmt();
		case 15:
			return new EnumDeclaration();
		case 6:
			return check_ahead(() => (safe_skip_keyword(), match_keyword(13)))
				? new AutoTraitDeclaration()
				: read_expr_or_macroInvocation_stmt();
		case 13:
			return read_TraitDeclaration();
		case 16:
			return read_impl();
		default:
			return read_expr_or_macroInvocation_stmt();
	}
}
function read_top_statements(target) {
	return read_group_noDelim(target, () => read_statement());
}
function read_body(target) {
	read_group(target, 3, () => read_statement());
}
function read_body_noBody(target) {
	read_group_noGroup(target);
}
function maybe_read_body(target) {
	if (match(123)) read_body(target);
	else maybe_read(59), (target.body = void 0);
}
// src/parser/read/types.ts
function read_Lifetime() {
	safe_step_over();
	switch (peek_keyword()) {
		case 19:
			return Mc_ctx_isReadingTokens() ? new LtIdentifier() : new LtStatic();
		case 2:
			return Mc_ctx_isReadingTokens() ? new LtIdentifier() : new LtElided();
		default:
			return new LtIdentifier();
		case 0:
		case 5:
			exit3.expected("Identifier");
	}
}
function maybe_read_lifetime() {
	return match(39) ? read_Lifetime() : void 0;
}
function read_charLiteral_or_lifetime() {
	return will_match_charLiteral_not_lt() ? new Literal() : read_Lifetime();
}
function read_FG_typeDefault() {
	switch (currChar()) {
		case 45:
			return new MinusExpression(() => new Literal());
		case 123:
			return new BlockExpression();
		case 39:
			return read_charLiteral_or_lifetime();
		case 34:
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 56:
		case 57:
			return new Literal();
		default:
			switch (peek_keyword()) {
				case 5:
				case 8:
				case 7:
					return new Literal();
				default:
					return read_type(true);
			}
	}
}
function maybe_read_ltBounds() {
	if (maybe_read(58)) {
		const ltBounds = [];
		if (match(39)) {
			do ltBounds.push(read_Lifetime());
			while (maybe_read(43) && match(39));
		}
		return ltBounds;
	}
	return void 0;
}
function read_TypeBound() {
	switch (currChar()) {
		case 40:
			return escapeParens(
				new TypeParenthesized(
					readLocatedArrayDelim(1, (arr) => {
						arr[0] = read_between(40, () => read_typeTraitBound(true), 41);
					})
				)
			);
		case 39:
			return read_Lifetime();
		default:
			return read_typeTraitBound();
	}
}
function read_typeBounds() {
	const typeBounds = [];
	do typeBounds.push(read_TypeBound());
	while (maybe_read_boundContinue());
	return typeBounds;
}
function maybe_read_colon_typeBounds() {
	return maybe_read(58) ? read_typeBounds() : void 0;
}
function read_TypeArguments() {
	return read_sequence(4, () => read_TypeArgument());
}
function read_TypeArgument() {
	switch (currChar()) {
		case 45:
			return new MinusExpression(() => new Literal());
		case 123:
			return new BlockExpression();
		case 39:
			return read_charLiteral_or_lifetime();
		case 34:
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 56:
		case 57:
			return new Literal();
		default:
			switch (peek_keyword()) {
				case 5:
				case 8:
				case 7:
					return new Literal();
				default: {
					const lhs = read_type(true);
					switch (currChar()) {
						case 61:
							return new TypeCallNamedArgument(lhs);
						case 58:
							return new TypeCallNamedBound(lhs);
						default:
							return lhs;
					}
				}
			}
	}
}
function asTypeDynBounds(typeBound) {
	return withStart(typeBound, new TypeDynBounds(false, typeBound));
}
function asTypeTraitBound(typeExpression) {
	return withStart(typeExpression, new TypeTraitBound(false, false, void 0, typeExpression));
}
function read_standalone_bounds(first) {
	const typeBounds = [first];
	if (TY_ctx_allowMultipleBounds() || is_Lifetime(first) || (is_TypeTraitBound(first) && !is_BareTypeTraitBound(first))) {
		while (maybe_read_boundContinue()) {
			typeBounds.push(read_TypeBound());
		}
	}
	return typeBounds;
}
function read_typeTraitBound(allowMultipleBounds = false) {
	return read_ahead(
		() =>
			new TypeTraitBound(
				maybe_read(126) ? (read_keyword(20), true) : false,
				maybe_read(63),
				maybe_read_forLtParameters(),
				read_type(allowMultipleBounds)
			)
	);
}
function read_TypeNamespaceTarget(init) {
	let lhs = init;
	loop: while (true) {
		switch (currChar()) {
			case 33:
				if (peek_match(1, 61)) break loop;
				lhs = new MacroInvocation(lhs);
				break loop;
			case 58:
				if (maybe_skip_1_read_2(58)) {
					switch (currChar()) {
						case 58:
							EDGECASE_STEPBACK_TO(lhs);
							break loop;
						case 60:
							lhs = new TypeCall(lhs);
							continue loop;
						default:
							lhs = new TypePath(lhs);
							continue loop;
					}
				}
				break loop;
			case 40:
				lhs = new TypeFunction(lhs);
				break;
			case 60:
				if (peek_match(1, 61)) break loop;
				lhs = new TypeCall(lhs);
				break;
			default:
				break loop;
		}
	}
	return lhs;
}
function read_TypeNamespaceTargetNoSelector() {
	return read_TypeNamespaceTarget(read_ccPath_or_Identifier(TypePath));
}
function maybe_read_generics() {
	return match(60)
		? read_sequence(4, () =>
				with_outerAttributes_fromParentContext(() =>
					match(39)
						? new GenericLtParameterDeclaration()
						: match_keyword(20)
						? new ConstTypeParameterDeclaration()
						: new GenericTypeParameterDeclaration()
				)
		  )
		: void 0;
}
function read_WhereBoundDeclaration() {
	return match(39) ? new WhereLtBoundDeclaration() : new WhereTypeBoundDeclaration();
}
function match_boundEnd() {
	switch (currChar()) {
		case 58:
			return check_ahead(() => {
				if (maybe_skip_1_read_2(58)) {
					return match(58);
				} else {
					return true;
				}
			});
		case 44:
		case 59:
		case 61:
		case 62:
		case 41:
		case 123:
			return true;
		default:
			return false;
	}
}
function maybe_read_boundContinue() {
	return maybe_read(43) && !match_boundEnd();
}
function maybe_read_whereBounds() {
	return match_keyword(32)
		? readLocatedArrayDelim(0, (whereBounds) => {
				safe_skip_keyword();
				if (!match_boundEnd()) {
					do whereBounds.push(read_WhereBoundDeclaration());
					while (maybe_read(44) && !match_boundEnd());
				}
		  })
		: void 0;
}
function read_type_lhs() {
	switch (currChar()) {
		case 60:
			return read_TypeNamespaceTarget(new ExpressionTypeSelector());
		case 91:
			return read_ahead(() => {
				safe_skip();
				const typeExpression = read_type(true);
				return match(93) ? new TypeSlice(typeExpression) : new TypeSizedArray(typeExpression);
			});
		case 40: {
			const items = read_sequence(1, () => read_type(true));
			return items.length !== 1 || sequence_hasTrailingComma() ? new TypeTuple(items) : new TypeParenthesized(items);
		}
		case 58:
			return read_TypeNamespaceTarget(CCPATH_read(TypePath));
		case 33:
			return new TypeNever();
		case 38:
			return new TypeReference();
		case 42:
			return read_ahead(() => {
				safe_skip();
				switch (peek_keyword()) {
					case 20:
						return new TypeDereferenceConst();
					case 31:
						return new TypeDereferenceMut();
					default:
						exit3.expected(["const", "mut"]);
				}
			});
		case 39:
			return asTypeDynBounds(read_Lifetime());
		case 63:
		case 126:
			return asTypeDynBounds(read_typeTraitBound());
		default:
			switch (peek_keyword()) {
				case 30: {
					const ty = read_ahead(() => {
						const ltParameters = read_forltParameters();
						return (function r() {
							switch (peek_keyword()) {
								case 21:
									return read_with_unsafe_modifier(r);
								case 23:
									return read_with_extern_specifier(r);
								case 9:
									return new TypeFnPointer(ltParameters);
								default:
									return new TypeTraitBound(false, false, ltParameters, read_TypeNamespaceTargetNoSelector());
							}
						})();
					});
					return is_TypeTraitBound(ty) ? asTypeDynBounds(ty) : ty;
				}
				case 21:
					return read_with_unsafe_modifier(() => read_type_lhs());
				case 23:
					return read_with_extern_specifier(() => read_type_lhs());
				case 24:
					exit3.never();
					return read_with_move_modifier(() => read_type_lhs());
				case 28:
					return read_ahead(() => (safe_skip_keyword(), TY_withContext(() => new TypeDynBounds(true, read_TypeBound()), true)));
				case 16:
					return TY_withContext(() => new TypeImplBounds(), true);
				case 2:
					return new TypeInferred();
				case 9:
					return new TypeFnPointer(void 0);
				case 6:
				case 43:
				case 44:
				case 14:
				case 42:
				case 45:
				case 3:
				case 1:
				case 4:
					return read_TypeNamespaceTarget(new Identifier());
				default:
					exit3.expected("TypeNode");
			}
	}
}
function read_forltParameters() {
	return read_ahead(
		() => (
			safe_skip_keyword(), read_sequence(4, () => with_outerAttributes_fromParentContext(() => new GenericLtParameterDeclaration()))
		)
	);
}
function maybe_read_forLtParameters() {
	return match_keyword(30) ? read_forltParameters() : void 0;
}
function read_type(allowMultipleBounds) {
	return TY_withContext(
		() =>
			withEscapedParens(read_type_lhs(), (ty) =>
				TY_ctx_allowMultipleBounds() && match(43) ? asTypeDynBounds(asTypeTraitBound(ty)) : ty
			),
		allowMultipleBounds
	);
}
function maybe_read_ReturnType(allowMultipleBounds) {
	return maybe_read_2(45, 62) ? read_type(allowMultipleBounds) : void 0;
}
function maybe_read_typeAnnotation() {
	return maybe_read(58) ? read_type(true) : void 0;
}
// src/parser/read/macro.ts
__EXPORT_ATTR_CTORS(Attribute);
function read_rules(delim) {
	return readLocatedArrayDelim(delim, (rules) => {
		FOR_ANY_BETWEEN(
			getDelimStartCharCode(rules.dk),
			() => {
				rules.push(new MacroRuleDeclaration());
				while (maybe_read(59) || maybe_read(44));
			},
			getDelimEndCharCode(rules.dk)
		);
	});
}
function read_rule(rule) {
	rule.match = read_segments_withEnv(3);
	maybe_read_2(61, 62);
	rule.transform = read_segments_withEnv(4);
}
function read_segments(tk = getGroupDelimKind(currChar())) {
	return readLocatedArrayDelim(tk, (tokens) => {
		MC_ctx_isReadingTokens_start();
		safe_skip(getDelimStartCharCode(tokens.dk));
		loop: while (true) {
			switch (currChar()) {
				case 41:
				case 93:
				case 125:
					break loop;
				case 40:
				case 91:
				case 123:
					tokens.push(new DelimGroup());
					break;
				case 39:
					tokens.push(read_charLiteral_or_lifetime());
					break;
				case 34:
				case 48:
				case 49:
				case 50:
				case 51:
				case 52:
				case 53:
				case 54:
				case 55:
				case 56:
				case 57:
					tokens.push(new Literal());
					break;
				case 36:
					tokens.push(read_dollar());
					break;
				default:
					switch (peek_keyword()) {
						case 7:
						case 8:
						case 5:
							tokens.push(new Literal());
							break;
						case 2:
							tokens.push(read_underscore());
							break;
						case 0:
							tokens.push(new PunctuationToken());
							break;
						default:
							tokens.push(read_identifier_token());
							break;
					}
					break;
			}
		}
		MC_ctx_isReadingTokens_end();
		if (isReadingEnv(1) && !Mc_ctx_isReadingTokens()) {
			step_over(getDelimEndCharCode(tokens.dk));
		} else {
			read(getDelimEndCharCode(tokens.dk));
		}
	});
}
function read_segments_withEnv(env, tk = getGroupDelimKind(currChar())) {
	setEnv(env);
	return read_segments(tk);
}
function maybe_read_sep() {
	switch (currChar()) {
		case 36:
		case 41:
		case 93:
		case 125:
		case 40:
		case 91:
		case 123:
			exit3.unexpected();
		case 42:
		case 43:
		case 63:
			return void 0;
		case 39:
			return read_charLiteral_or_lifetime();
		case 34:
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 56:
		case 57:
			return new Literal();
		default:
			switch (peek_keyword()) {
				case 7:
				case 8:
				case 5:
					return new Literal();
				case 2:
					return read_underscore();
				case 0:
					return new PunctuationToken();
				default:
					return read_identifier_token();
			}
	}
}
function read_dollar() {
	switch (getEnv()) {
		case 1:
		case 2:
			return new PunctuationToken();
	}
	safe_step_over();
	if (match(40)) {
		return new MacroGroup();
	}
	if (is_XID_Start(currChar())) {
		return isReadingEnv(3) ? new MacroParameterDeclaration() : new McIdentifier();
	}
	edgecase_stepback();
	return new PunctuationToken();
}
function read_underscore() {
	invalidate_kw();
	return new PunctuationToken();
}
function read_tokens_until(endPos) {
	setEnv(1);
	return readLocatedArrayNoDelim((tokens) => {
		MC_ctx_isReadingTokens_start();
		while (GET_POSITION() < endPos) {
			switch (currChar()) {
				case 41:
				case 93:
				case 125:
					exit3.unexpected();
				case 40:
				case 91:
				case 123:
					tokens.push(new DelimGroup());
					break;
				case 39:
					tokens.push(read_charLiteral_or_lifetime());
					break;
				case 34:
				case 48:
				case 49:
				case 50:
				case 51:
				case 52:
				case 53:
				case 54:
				case 55:
				case 56:
				case 57:
					tokens.push(new Literal());
					break;
				case 36:
					tokens.push(read_dollar());
					break;
				default:
					switch (peek_keyword()) {
						case 7:
						case 8:
						case 5:
							tokens.push(new Literal());
							break;
						case 2:
							tokens.push(read_underscore());
							break;
						case 0:
							tokens.push(new PunctuationToken());
							break;
						default:
							tokens.push(read_identifier_token());
							break;
					}
					break;
			}
		}
		MC_ctx_isReadingTokens_end();
	});
}
// src/parser/read/expressions.ts
function read_property_or_method(expression) {
	safe_skip();
	switch (peek_keyword()) {
		case 33:
			return new AwaitExpression(expression);
		case 6:
		case 43:
		case 44:
		case 14:
		case 42:
		case 45:
		case 3:
		case 1:
		case 4: {
			const property = new Identifier();
			switch (currChar()) {
				case 58:
					if (maybe_skip_1_read_2(58) && match(60)) {
						const typeArguments = read_TypeArguments();
						if (match(40)) {
							return new CallExpression(expression, property, typeArguments);
						} else {
							return new ExpressionTypeCast(
								withEnd(property, new MemberExpression(expression, false, property)),
								typeArguments
							);
						}
					} else {
						EDGECASE_STEPBACK_TO(property);
						return new MemberExpression(expression, false, property);
					}
				case 40:
					return new CallExpression(expression, property, void 0);
				default:
					return new MemberExpression(expression, false, property);
			}
		}
		default:
			return new MemberExpression(expression, false, read_Index());
	}
}
function read_computed_property(lhs) {
	return new MemberExpression(lhs, true, read_expression_between(91, 93));
}
function read_ConditionExpressionNode() {
	return read_contained_expression(true, true);
}
function read_turbofish(namespace) {
	const typeArguments = read_TypeArguments();
	return match(40) ? new CallExpression(namespace, void 0, typeArguments) : new ExpressionTypeCast(namespace, typeArguments);
}
function read_ExpressionNamespaceTarget(namespace) {
	let lhs = namespace;
	loop: while (maybe_read_2(58, 58)) {
		switch (currChar()) {
			case 58:
				EDGECASE_STEPBACK_TO(lhs);
				break loop;
			case 60:
				lhs = read_turbofish(lhs);
				break;
			default:
				lhs = new ExpressionPath(lhs);
				break;
		}
	}
	return lhs;
}
function read_expression_lhs() {
	switch (currChar()) {
		case 58:
			return read_ExpressionNamespaceTarget(CCPATH_read(ExpressionPath));
		case 60:
			return read_ExpressionNamespaceTarget(new ExpressionTypeSelector());
		case 46:
			return read_ahead(() => {
				safe_skip_1_read_2(46, 46);
				return new RangeLiteral(void 0, maybe_read(61));
			});
		case 123:
			return new BlockExpression();
		case 40: {
			const items = read_sequence(1, (SEQUENCE) =>
				with_outerAttributes_fromParentContext_if_test(
					() => read_contained_expression(false, ES_ctx_insideScrutinee() && 0 === SEQUENCE.length),
					() => SEQUENCE.length > 0 || match(44)
				)
			);
			return items.length !== 1 || sequence_hasTrailingComma() || (is_RangeLiteral(items[0]) && match(61) && !peek_match(1, 61))
				? new TupleLiteral(items)
				: new ParenthesizedExpression(items);
		}
		case 91: {
			let is_sized = false;
			const items = readLocatedArrayDelim(2, (items2) => {
				safe_skip();
				if (!maybe_read(93)) {
					items2[0] = read_item();
					if ((is_sized = maybe_read(59))) {
						(items2[1] = read_item()), read(93);
					} else {
						FOR_EACH_UNTIL(44, () => items2.push(read_item()), 93);
					}
				}
				function read_item() {
					return with_outerAttributes_fromParentContext(() => read_contained_expression(false));
				}
			});
			return is_sized ? new SizedArrayLiteral(items) : new ArrayLiteral(items);
		}
		case 124:
			return new ClosureFunctionExpression();
		case 45:
			return new MinusExpression(() => read_unary_rhs());
		case 33:
			return new NotExpression();
		case 38:
			return read_ahead(() => {
				safe_skip();
				return match_3(114, 97, 119) && is_whitespaceOrSlash(peek(3))
					? read_ahead_either(
							() => {
								safe_skip_word("raw");
								switch (peek_keyword()) {
									case 31:
										safe_skip_keyword();
										return new RawReferenceExpression("mut");
									case 20:
										safe_skip_keyword();
										return new RawReferenceExpression("const");
									default:
										return void 0;
								}
							},
							() => new ReferenceExpression()
					  )
					: new ReferenceExpression();
			});
		case 42:
			return new DereferenceExpression();
		case 39:
			return will_match_charLiteral_not_lt() ? new Literal() : read_labelled_block(new LbIdentifier());
		case 34:
		case 48:
		case 49:
		case 50:
		case 51:
		case 52:
		case 53:
		case 54:
		case 55:
		case 56:
		case 57:
			return new Literal();
		default:
			switch (peek_keyword()) {
				case 5:
				case 8:
				case 7:
					return new Literal();
				case 23:
				case 25:
					exit3.unexpected();
				case 19:
					return read_with_static_modifier(read_expression_lhs);
				case 20:
					return read_with_const_modifier(read_expression_lhs);
				case 22:
					return read_with_async_modifier(read_expression_lhs);
				case 24:
					return read_with_move_modifier(read_expression_lhs);
				case 21:
					return read_with_unsafe_modifier(read_expression_lhs);
				case 34:
					return new ReturnExpression();
				case 35:
					return new BreakExpression();
				case 36:
					return new ContinueExpression();
				case 48:
					return new YieldExpression();
				case 39:
					return new MatchExpression();
				case 40:
					return new LoopBlockExpression();
				case 41:
					return new WhileBlockExpression();
				case 30:
					return new ForInBlockExpression();
				case 37:
					return new IfBlockExpression();
				case 6:
				case 43:
				case 44:
				case 14:
				case 42:
				case 45:
				case 3:
				case 1:
				case 4:
					return read_ExpressionNamespaceTarget(new Identifier());
				case 2:
					return new UnassignedExpression();
				case 46:
					return new BoxExpression();
				case 47:
					switch (check_ahead(() => (safe_skip_keyword(), currChar()))) {
						case 33:
						case 40:
							return new Identifier();
						case 58:
							return read_ExpressionNamespaceTarget(new Identifier());
						case 123:
							return new TryBlockExpression();
						default:
							exit3.unexpected();
					}
				case 18:
					assert7(ES_ctx_insideScrutinee(), "let variable declarations are not allowed in this context");
					return new LetScrutinee();
				default:
					if (ES_consume_optional_read()) {
						return void 0;
					}
					exit3.expected("Expression");
			}
	}
}
function read_expression() {
	return read_contained_expression(false);
}
function read_contained_expression(exceptStructFormExpression, insideScrutinee = false) {
	return ES_withContext(exceptStructFormExpression, insideScrutinee, () => read_expression_rhs());
}
function read_labelled_block(label) {
	read(58);
	const lhs = read_expression_lhs();
	assert7.at(lhs, is_ExpressionWithBodyOrCases(lhs), `Expected ExpressionWithBodyOrCases, found ${lhs.type}`);
	return maybe_combine_expression_block(withStart((lhs.label = label), lhs));
}
function read_stmt_expression() {
	return ES_withContext(false, false, () => {
		let lhs = read_expression_lhs();
		if (!is_ParenthesizedExpression(lhs)) {
			if (is_ExpressionWithBodyOrCases(lhs)) {
				return maybe_combine_expression_block(lhs);
			}
			if (match(33) && !peek_match(1, 61)) {
				lhs = new MacroInvocation(lhs);
				if (lhs.segments.dk === 3) {
					return maybe_combine_expression_block(lhs);
				}
			}
		}
		return maybe_combine_expression(lhs);
	});
}
function maybe_combine_expression_block(lhs) {
	return should_combine_blockstmt() ? maybe_combine_expression(lhs) : lhs;
	function should_combine_blockstmt() {
		switch (currChar()) {
			default:
				return false;
			case 63:
				return true;
			case 46:
				return peek_not_match(1, 46);
			case 43:
				return true;
			case 97:
				return match_keyword(26);
		}
	}
}
function might_read_expression() {
	switch (currChar()) {
		case 41:
		case 125:
		case 93:
		case 59:
		case 44:
			return false;
		case 123:
			return !ES_ctx_exceptStructFormExpression();
		default:
			return true;
	}
}
function maybe_read_expression_rhs() {
	if (might_read_expression()) {
		ES_signal_optional_read();
		const lhs = read_expression_lhs();
		if (void 0 !== lhs) {
			return maybe_combine_expression(lhs);
		}
	}
	return void 0;
}
function read_expression_rhs() {
	return maybe_combine_expression(read_expression_lhs());
}
function read_unary_rhs() {
	return ES_withPrecedence(16, () => read_expression_rhs());
}
function read_closure_rhs() {
	return ES_withPrecedence(3, () => read_expression_rhs());
}
function read_scrutinee_rhs() {
	return ES_withPrecedence(3, () => read_expression_rhs());
}
function read_expression_between(charStart, charEnd) {
	safe_skip();
	const res = read_contained_expression(false);
	read(charEnd);
	return res;
}
function read_contained_expr_in_stmt() {
	return read_contained_expression(false);
}
function maybe_combine_expression(startNode) {
	return ES_withPrecedence(3, () =>
		withEscapedParens(startNode, (startNode2) => {
			let lhs = startNode2;
			let rhs = 0;
			loop: while (true) {
				switch ((rhs = rhsTree())) {
					case 40:
						lhs = match(60) ? read_turbofish(lhs) : new ExpressionPath(lhs);
						break;
					case 43:
						lhs = new MacroInvocation(lhs);
						break;
					case 100:
						lhs = new CallExpression(lhs, void 0, void 0);
						break;
					case 101:
						if (!is_ExpressionNamespaceTarget(lhs)) break loop;
						lhs = new StructLiteral(lhs);
						break;
					case 1:
						lhs = read_property_or_method(lhs);
						break;
					case 42:
						lhs = new UnwrapExpression(lhs);
						break;
					case 102:
						lhs = read_computed_property(lhs);
						break;
					case 103:
						lhs = new ExpressionAsTypeCast(lhs);
						break;
					case 34:
						lhs = new RangeLiteral(lhs, false);
						break;
					case 36:
					case 35:
						lhs = new RangeLiteral(lhs, true);
						break;
					case 2:
						lhs = new AndExpression(lhs);
						break;
					case 3:
						lhs = new OrExpression(lhs);
						break;
					case 4:
						lhs = new ReassignmentExpression(lhs);
						break;
					case 15:
					case 16:
						maybe_read(61);
					case 17:
					case 18:
					case 19:
					case 20:
						lhs = new ComparisonExpression(lhs, rhs);
						break;
					case 5:
					case 6:
					case 7:
					case 8:
					case 9:
					case 10:
					case 11:
					case 12:
					case 13:
					case 14:
						lhs = new OperationExpression(lhs, rhs);
						break;
					case 21:
					case 22:
					case 23:
					case 24:
					case 25:
					case 26:
					case 27:
					case 28:
					case 29:
					case 30:
						lhs = new ReassignmentOperationExpression(lhs, rhs);
						break;
					case 0:
						break loop;
				}
			}
			return lhs;
		})
	);
}
// src/parser/read/sourcefile.ts
function toSnippet(target, READ_SNIPPET) {
	return withParserState(target.loc.src, ownStart(target), () => new Snippet(target, READ_SNIPPET));
}
function parseFile(code, options = {}) {
	checkOptions(options);
	return new SourceFile(code, options);
}
function toExpression(tokens) {
	return toSnippet(tokens, (snippet) => {
		snippet.ast = read_expression();
	});
}
function toCallExpressionArguments(tokens) {
	return toSnippet(tokens, (snippet) => {
		snippet.ast = read_sequence(getGroupDelimKind(currChar()), () => read_expression());
	});
}
function toBlockBody(tokens) {
	return toSnippet(tokens, (snippet) => {
		const tk = getDelimKind(currChar());
		if (0 === tk) read_group_noDelim(snippet, () => read_statement());
		else read_group(snippet, tk, () => read_statement());
	});
}
function toTokens(node) {
	return toSnippet(node, (snippet) => {
		snippet.ast = read_tokens_until(clamp(0, GET_LENGTH2(), end(node) - 1));
	});
}
export {
	AmbientImport as AmbientImport,
	AndExpression as AndExpression,
	AnonymousImport as AnonymousImport,
	ArrayLiteral as ArrayLiteral,
	ArrayPattern as ArrayPattern,
	Attribute as Attribute,
	AutoTraitDeclaration as AutoTraitDeclaration,
	AwaitExpression as AwaitExpression,
	BaseNode2 as BaseNode,
	BlockExpression as BlockExpression,
	BoxExpression as BoxExpression,
	BoxPattern as BoxPattern,
	BreakExpression as BreakExpression,
	CallExpression as CallExpression,
	ClosureFunctionExpression as ClosureFunctionExpression,
	ClosureFunctionParameterDeclaration as ClosureFunctionParameterDeclaration,
	Comment as Comment,
	ComparisonExpression as ComparisonExpression,
	ConstTypeParameterDeclaration as ConstTypeParameterDeclaration,
	ConstVariableDeclaration as ConstVariableDeclaration,
	ContinueExpression as ContinueExpression,
	DelimGroup as DelimGroup,
	DelimKind,
	DereferenceExpression as DereferenceExpression,
	DestructuredImport as DestructuredImport,
	DocCommentAttribute as DocCommentAttribute,
	EnumDeclaration as EnumDeclaration,
	EnumMemberDeclaration as EnumMemberDeclaration,
	EnumMemberStructDeclaration as EnumMemberStructDeclaration,
	EnumMemberTupleDeclaration as EnumMemberTupleDeclaration,
	ExpressionAsTypeCast as ExpressionAsTypeCast,
	ExpressionPath as ExpressionPath,
	ExpressionStatement as ExpressionStatement,
	ExpressionTypeCast as ExpressionTypeCast,
	ExpressionTypeSelector as ExpressionTypeSelector,
	ExternBlockDeclaration as ExternBlockDeclaration,
	ExternCrateStatement as ExternCrateStatement,
	ExternSpecifier as ExternSpecifier,
	Feature,
	ForInBlockExpression as ForInBlockExpression,
	FunctionDeclaration as FunctionDeclaration,
	FunctionParameterDeclaration as FunctionParameterDeclaration,
	FunctionSelfParameterDeclaration as FunctionSelfParameterDeclaration,
	FunctionSpread as FunctionSpread,
	GenericLtParameterDeclaration as GenericLtParameterDeclaration,
	GenericTypeParameterDeclaration as GenericTypeParameterDeclaration,
	Identifier as Identifier,
	IfBlockExpression as IfBlockExpression,
	ImplDeclaration as ImplDeclaration,
	Index as Index,
	ItemPath as ItemPath,
	LbIdentifier as LbIdentifier,
	LetScrutinee as LetScrutinee,
	LetVariableDeclaration as LetVariableDeclaration,
	Literal as Literal,
	LiteralKind,
	Loc,
	LoopBlockExpression as LoopBlockExpression,
	LtElided as LtElided,
	LtIdentifier as LtIdentifier,
	LtStatic as LtStatic,
	MacroDeclaration as MacroDeclaration,
	MacroGroup as MacroGroup,
	MacroInlineRuleDeclaration as MacroInlineRuleDeclaration,
	MacroInvocation as MacroInvocation,
	MacroParameterDeclaration as MacroParameterDeclaration,
	MacroRuleDeclaration as MacroRuleDeclaration,
	MacroRulesDeclaration as MacroRulesDeclaration,
	MatchExpression as MatchExpression,
	MatchExpressionCase as MatchExpressionCase,
	McIdentifier as McIdentifier,
	MemberExpression as MemberExpression,
	MinusExpression as MinusExpression,
	MinusPattern as MinusPattern,
	MissingNode as MissingNode,
	ModuleDeclaration as ModuleDeclaration,
	NamedImport as NamedImport,
	NegativeImplDeclaration as NegativeImplDeclaration,
	NodeType,
	NotExpression as NotExpression,
	OperationExpression as OperationExpression,
	OrExpression as OrExpression,
	PRCD,
	ParenthesizedExpression as ParenthesizedExpression,
	ParenthesizedPattern as ParenthesizedPattern,
	PatternVariableDeclaration as PatternVariableDeclaration,
	Program as Program,
	PubSpecifier as PubSpecifier,
	PunctuationToken as PunctuationToken,
	RangeLiteral as RangeLiteral,
	RangePattern as RangePattern,
	RawReferenceExpression as RawReferenceExpression,
	ReassignmentExpression as ReassignmentExpression,
	ReassignmentOperationExpression as ReassignmentOperationExpression,
	ReferenceExpression as ReferenceExpression,
	ReferencePattern as ReferencePattern,
	RestPattern as RestPattern,
	ReturnExpression as ReturnExpression,
	Shebang as Shebang,
	SizedArrayLiteral as SizedArrayLiteral,
	Snippet as Snippet,
	SourceFile as SourceFile,
	StaticVariableDeclaration as StaticVariableDeclaration,
	StructDeclaration as StructDeclaration,
	StructLiteral as StructLiteral,
	StructLiteralProperty as StructLiteralProperty,
	StructLiteralPropertyShorthand as StructLiteralPropertyShorthand,
	StructLiteralPropertySpread as StructLiteralPropertySpread,
	StructLiteralRestUnassigned as StructLiteralRestUnassigned,
	StructPattern as StructPattern,
	StructPatternPropertyDestructured as StructPatternPropertyDestructured,
	StructPatternPropertyShorthand as StructPatternPropertyShorthand,
	StructPropertyDeclaration as StructPropertyDeclaration,
	TK,
	TraitAliasDeclaration as TraitAliasDeclaration,
	TraitDeclaration as TraitDeclaration,
	TryBlockExpression as TryBlockExpression,
	TupleLiteral as TupleLiteral,
	TuplePattern as TuplePattern,
	TupleStructDeclaration as TupleStructDeclaration,
	TupleStructItemDeclaration as TupleStructItemDeclaration,
	TyMacroMatch,
	TypeAliasDeclaration as TypeAliasDeclaration,
	TypeCall as TypeCall,
	TypeCallNamedArgument as TypeCallNamedArgument,
	TypeCallNamedBound as TypeCallNamedBound,
	TypeDereferenceConst as TypeDereferenceConst,
	TypeDereferenceMut as TypeDereferenceMut,
	TypeDynBounds as TypeDynBounds,
	TypeFnPointer as TypeFnPointer,
	TypeFnPointerParameter as TypeFnPointerParameter,
	TypeFunction as TypeFunction,
	TypeImplBounds as TypeImplBounds,
	TypeInferred as TypeInferred,
	TypeNever as TypeNever,
	TypeParenthesized as TypeParenthesized,
	TypePath as TypePath,
	TypeReference as TypeReference,
	TypeSizedArray as TypeSizedArray,
	TypeSlice as TypeSlice,
	TypeTraitBound as TypeTraitBound,
	TypeTuple as TypeTuple,
	UnassignedExpression as UnassignedExpression,
	UnionDeclaration as UnionDeclaration,
	UnionPattern as UnionPattern,
	UnwrapExpression as UnwrapExpression,
	UseStatement as UseStatement,
	WhereLtBoundDeclaration as WhereLtBoundDeclaration,
	WhereTypeBoundDeclaration as WhereTypeBoundDeclaration,
	WhileBlockExpression as WhileBlockExpression,
	WildcardPattern as WildcardPattern,
	YieldExpression as YieldExpression,
	parser_exports as rs,
};
