// src/utils/debug.ts
var cwd = typeof process === "object" && typeof (process == null ? void 0 : process.cwd) === "function" ? /* @__PURE__ */ normPath(/* @__PURE__ */ process.cwd() ?? "") : "";
function normPath_strip_cwd(filepath) {
  let normFilePath = normPath(filepath);
  return normFilePath.startsWith(cwd) ? normFilePath.slice(cwd.length + 1) : normFilePath;
}
var StackLine = class {
  constructor(raw) {
    ({
      1: this.callee = "",
      2: this.filepath = "",
      3: this.line = "",
      4: this.col = "",
      5: this.other = ""
    } = (this.raw = raw).match(/at (?:(.+?)\s+\()?(?:(.+?):([0-9]+)(?::([0-9]+))?|([^)]+))\)?/) ?? ["", "", "", "", "", ""]);
    this.url = this.filepath ? normPath_strip_cwd(this.filepath) + (this.line && this.col && `:${this.line}:${this.col}`) : this.other === "native" ? "<native>" : "";
  }
};
function getPrintWidth() {
  return clamp(0, getTerminalWidth(128), 200) - 4;
}
var StackItem = class extends StackLine {
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
    for (let i = 0; i < n; i++)
      (_a = this.at(i)) == null ? void 0 : _a.hide();
  }
  hideWhileTrue(test) {
    let line = this;
    while (line && test(line))
      line = line.hide().next();
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
    const calleeColor = ((_b = (_a = this.stack.style) == null ? void 0 : _a.callee) == null ? void 0 : _b.call(_a, this.callee, this)) ?? color.cyan;
    const urlColor = ((_d = (_c = this.stack.style) == null ? void 0 : _c.url) == null ? void 0 : _d.call(_c, url, this)) ?? color.grey;
    return compose2Cols("    at " + calleeColor(this.callee), urlColor(url), getPrintWidth());
  }
};
function createStack(message, Error_stack, style) {
  for (var STACK = [], i = 0, stack = Error_stack.split("\n").slice(1); i < stack.length; i++)
    STACK[i] = new StackItem(STACK, i, stack[i]);
  return STACK.message = message, STACK.style = style, STACK;
}
function composeStack(stack) {
  var hidden = 0;
  var str = stack.message;
  for (var item of stack)
    item.hidden ? ++hidden : str += "\n" + item.toString();
  return str + (hidden > 0 ? "\n" + color.grey(compose2Cols("", `...filtered ${hidden} lines`, getPrintWidth())) : "");
}
function createCustomError({
  ctor = createCustomError,
  message = "Unknown Error",
  editStack = (stack) => {
  },
  style = void 0,
  stackTraceLimit = 20
}) {
  const _stackTraceLimit = Error.stackTraceLimit;
  const _prepareStackTrace = Error.prepareStackTrace;
  Error.stackTraceLimit = stackTraceLimit;
  const _ctx = {};
  Error.captureStackTrace(_ctx, ctor);
  const stack = createStack(message, _ctx.stack, style);
  Error.prepareStackTrace = function(err2, calls) {
    editStack(stack);
    return composeStack(stack);
  };
  const err = new Error(message);
  err.stack = err.stack;
  Error.stackTraceLimit = _stackTraceLimit;
  Error.prepareStackTrace = _prepareStackTrace;
  return err;
}
function compose2Cols(left, right, len = 64, min = 1) {
  return left + " ".repeat(clamp(min, len, len - (color.unstyledLength(left) + color.unstyledLength(right)))) + right;
}
function exit(message, ...ctx) {
  if (ctx.length > 0)
    console.log("Error context:", { ...ctx });
  throw createCustomError({ message, ctor: exit });
}
exit.never = function never(...ctx) {
  exit("Reached unreachable code", ...ctx);
};
function Identity(v) {
  return v;
}
function last_of(arr) {
  return arr[arr.length - 1];
}
function normPath(filepath) {
  return filepath.replace(/^file:\/\/\//, "").replace(/\\\\?/g, "/");
}
function binarySearchIn(array, target, toValue) {
  if (isEmpty(array))
    return -1;
  let i = 0;
  let low = 0;
  let high = array.length - 1;
  let value = toValue(array[high]);
  if (target >= value)
    return high;
  else
    high--;
  while (low <= high) {
    i = low + (high - low >> 1);
    value = toValue(array[i]);
    if (target === value)
      return i;
    if (target > value)
      low = i + 1;
    else
      high = i - 1;
  }
  return low - 1;
}
function binaryInsertIn(array, item, toValue) {
  if (0 === array.length || toValue(item) >= toValue(array[array.length - 1]))
    array[array.length] = item;
  else
    array.splice(1 + binarySearchIn(array, toValue(item), toValue), 0, item);
}
function binaryInsertEach(array, items, toValue) {
  for (var i = 0; i !== items.length; i++)
    binaryInsertIn(array, items[i], toValue);
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
  link: (str) => color.underline(color.blue(str))
}))(
  (c1) => isBrowser ? Identity : (str) => `\x1B[${c1}m${str.replace(/\x1B\[39m/g, `\x1B[${c1}m`)}\x1B[39m`,
  (c1, c2) => isBrowser ? Identity : (str) => `\x1B[${c1}m${str}\x1B[${c2}m`
);
function isEmpty(array) {
  return 0 === array.length;
}
function has_key(o, k) {
  return k in o;
}
function has_key_undefined(o, k) {
  return k in o && void 0 === o[k];
}
function has_key_defined(o, k) {
  return k in o && void 0 !== o[k];
}
function has_key_non_empty_array(o, k) {
  return k in o && void 0 !== o[k] && "length" in o[k] && 0 !== o[k].length;
}
function is_defined(data) {
  return void 0 !== data;
}
function is_object(data) {
  return "object" === typeof data && null !== data;
}
function is_array(data) {
  return Array.isArray(data);
}
function clamp(min, max, value) {
  return value > min ? value < max ? value : max : min;
}

// src/utils/ast/nodetype.ts
function nis(node, nodeType) {
  return nodeType === node.nodeType;
}
function nisAnyOf(node, nodeTypes) {
  return nodeTypes.includes(node.nodeType);
}
function isTK(node, tk) {
  return is_PunctuationToken(node) && node.tk === tk;
}
function includesTK(node, tk) {
  return node.segments.some((segment) => isTK(segment, tk));
}
function is_MissingNode(node) {
  return nis(node, 0);
}
function is_SourceFile(node) {
  return nis(node, 1);
}
function is_Shebang(node) {
  return nis(node, 2);
}
function is_Program(node) {
  return nis(node, 3);
}
function is_Snippet(node) {
  return nis(node, 4);
}
function is_Comment(node) {
  return nis(node, 5);
}
function is_Identifier(node) {
  return nis(node, 6);
}
function is_Index(node) {
  return nis(node, 7);
}
function is_LbIdentifier(node) {
  return nis(node, 8);
}
function is_McIdentifier(node) {
  return nis(node, 9);
}
function is_Literal(node) {
  return nis(node, 10);
}
function is_ItemPath(node) {
  return nis(node, 11);
}
function is_PunctuationToken(node) {
  return nis(node, 12);
}
function is_DelimGroup(node) {
  return nis(node, 13);
}
function is_Attribute(node) {
  return nis(node, 14);
}
function is_DocCommentAttribute(node) {
  return nis(node, 15);
}
function is_MacroInvocation(node) {
  return nis(node, 16);
}
function is_MacroRulesDeclaration(node) {
  return nis(node, 17);
}
function is_MacroDeclaration(node) {
  return nis(node, 18);
}
function is_MacroRuleDeclaration(node) {
  return nis(node, 19);
}
function is_MacroInlineRuleDeclaration(node) {
  return nis(node, 20);
}
function is_MacroGroup(node) {
  return nis(node, 21);
}
function is_MacroParameterDeclaration(node) {
  return nis(node, 22);
}
function is_PubSpecifier(node) {
  return nis(node, 23);
}
function is_ExternSpecifier(node) {
  return nis(node, 24);
}
function is_ExpressionStatement(node) {
  return nis(node, 25);
}
function is_UseStatement(node) {
  return nis(node, 26);
}
function is_NamedImport(node) {
  return nis(node, 27);
}
function is_AmbientImport(node) {
  return nis(node, 28);
}
function is_AnonymousImport(node) {
  return nis(node, 29);
}
function is_DestructuredImport(node) {
  return nis(node, 30);
}
function is_ExternCrateStatement(node) {
  return nis(node, 31);
}
function is_ExternBlockDeclaration(node) {
  return nis(node, 32);
}
function is_TypeAliasDeclaration(node) {
  return nis(node, 33);
}
function is_ConstVariableDeclaration(node) {
  return nis(node, 34);
}
function is_StaticVariableDeclaration(node) {
  return nis(node, 35);
}
function is_LetVariableDeclaration(node) {
  return nis(node, 36);
}
function is_ModuleDeclaration(node) {
  return nis(node, 37);
}
function is_FunctionDeclaration(node) {
  return nis(node, 38);
}
function is_FunctionSelfParameterDeclaration(node) {
  return nis(node, 39);
}
function is_FunctionParameterDeclaration(node) {
  return nis(node, 40);
}
function is_FunctionSpread(node) {
  return nis(node, 41);
}
function is_StructDeclaration(node) {
  return nis(node, 42);
}
function is_TupleStructDeclaration(node) {
  return nis(node, 43);
}
function is_StructPropertyDeclaration(node) {
  return nis(node, 44);
}
function is_TupleStructItemDeclaration(node) {
  return nis(node, 45);
}
function is_UnionDeclaration(node) {
  return nis(node, 46);
}
function is_EnumDeclaration(node) {
  return nis(node, 47);
}
function is_EnumMemberDeclaration(node) {
  return nis(node, 48);
}
function is_EnumMemberTupleDeclaration(node) {
  return nis(node, 49);
}
function is_EnumMemberStructDeclaration(node) {
  return nis(node, 50);
}
function is_TraitDeclaration(node) {
  return nis(node, 51);
}
function is_AutoTraitDeclaration(node) {
  return nis(node, 52);
}
function is_TraitAliasDeclaration(node) {
  return nis(node, 53);
}
function is_ImplDeclaration(node) {
  return nis(node, 54);
}
function is_NegativeImplDeclaration(node) {
  return nis(node, 55);
}
function is_ExpressionTypeSelector(node) {
  return nis(node, 56);
}
function is_ExpressionTypeCast(node) {
  return nis(node, 57);
}
function is_ExpressionPath(node) {
  return nis(node, 58);
}
function is_ExpressionAsTypeCast(node) {
  return nis(node, 59);
}
function is_ReturnExpression(node) {
  return nis(node, 60);
}
function is_BreakExpression(node) {
  return nis(node, 61);
}
function is_ContinueExpression(node) {
  return nis(node, 62);
}
function is_YieldExpression(node) {
  return nis(node, 63);
}
function is_CallExpression(node) {
  return nis(node, 64);
}
function is_MemberExpression(node) {
  return nis(node, 65);
}
function is_AwaitExpression(node) {
  return nis(node, 66);
}
function is_UnwrapExpression(node) {
  return nis(node, 67);
}
function is_ParenthesizedExpression(node) {
  return nis(node, 68);
}
function is_MinusExpression(node) {
  return nis(node, 69);
}
function is_NotExpression(node) {
  return nis(node, 70);
}
function is_OrExpression(node) {
  return nis(node, 71);
}
function is_AndExpression(node) {
  return nis(node, 72);
}
function is_ReassignmentExpression(node) {
  return nis(node, 73);
}
function is_UnassignedExpression(node) {
  return nis(node, 74);
}
function is_OperationExpression(node) {
  return nis(node, 75);
}
function is_ReassignmentOperationExpression(node) {
  return nis(node, 76);
}
function is_ComparisonExpression(node) {
  return nis(node, 77);
}
function is_LetScrutinee(node) {
  return nis(node, 78);
}
function is_ClosureFunctionExpression(node) {
  return nis(node, 79);
}
function is_ClosureFunctionParameterDeclaration(node) {
  return nis(node, 80);
}
function is_BlockExpression(node) {
  return nis(node, 81);
}
function is_LoopBlockExpression(node) {
  return nis(node, 82);
}
function is_WhileBlockExpression(node) {
  return nis(node, 83);
}
function is_ForInBlockExpression(node) {
  return nis(node, 84);
}
function is_TryBlockExpression(node) {
  return nis(node, 85);
}
function is_IfBlockExpression(node) {
  return nis(node, 86);
}
function is_MatchExpression(node) {
  return nis(node, 87);
}
function is_MatchExpressionCase(node) {
  return nis(node, 88);
}
function is_RangeLiteral(node) {
  return nis(node, 89);
}
function is_StructLiteral(node) {
  return nis(node, 90);
}
function is_StructLiteralProperty(node) {
  return nis(node, 91);
}
function is_StructLiteralPropertyShorthand(node) {
  return nis(node, 92);
}
function is_StructLiteralPropertySpread(node) {
  return nis(node, 93);
}
function is_StructLiteralRestUnassigned(node) {
  return nis(node, 94);
}
function is_TupleLiteral(node) {
  return nis(node, 95);
}
function is_ArrayLiteral(node) {
  return nis(node, 96);
}
function is_SizedArrayLiteral(node) {
  return nis(node, 97);
}
function is_ReferenceExpression(node) {
  return nis(node, 98);
}
function is_RawReferenceExpression(node) {
  return nis(node, 99);
}
function is_DereferenceExpression(node) {
  return nis(node, 100);
}
function is_BoxExpression(node) {
  return nis(node, 101);
}
function is_UnionPattern(node) {
  return nis(node, 102);
}
function is_ParenthesizedPattern(node) {
  return nis(node, 103);
}
function is_RestPattern(node) {
  return nis(node, 104);
}
function is_WildcardPattern(node) {
  return nis(node, 105);
}
function is_PatternVariableDeclaration(node) {
  return nis(node, 106);
}
function is_StructPattern(node) {
  return nis(node, 107);
}
function is_StructPatternPropertyDestructured(node) {
  return nis(node, 108);
}
function is_StructPatternPropertyShorthand(node) {
  return nis(node, 109);
}
function is_TuplePattern(node) {
  return nis(node, 110);
}
function is_ArrayPattern(node) {
  return nis(node, 111);
}
function is_ReferencePattern(node) {
  return nis(node, 112);
}
function is_BoxPattern(node) {
  return nis(node, 113);
}
function is_MinusPattern(node) {
  return nis(node, 114);
}
function is_RangePattern(node) {
  return nis(node, 115);
}
function is_TypePath(node) {
  return nis(node, 116);
}
function is_TypeCall(node) {
  return nis(node, 117);
}
function is_TypeCallNamedArgument(node) {
  return nis(node, 118);
}
function is_TypeCallNamedBound(node) {
  return nis(node, 119);
}
function is_LtIdentifier(node) {
  return nis(node, 120);
}
function is_LtElided(node) {
  return nis(node, 121);
}
function is_LtStatic(node) {
  return nis(node, 122);
}
function is_TypeNever(node) {
  return nis(node, 123);
}
function is_TypeInferred(node) {
  return nis(node, 124);
}
function is_GenericTypeParameterDeclaration(node) {
  return nis(node, 125);
}
function is_ConstTypeParameterDeclaration(node) {
  return nis(node, 126);
}
function is_GenericLtParameterDeclaration(node) {
  return nis(node, 127);
}
function is_WhereTypeBoundDeclaration(node) {
  return nis(node, 128);
}
function is_WhereLtBoundDeclaration(node) {
  return nis(node, 129);
}
function is_TypeTraitBound(node) {
  return nis(node, 130);
}
function is_TypeDynBounds(node) {
  return nis(node, 131);
}
function is_TypeImplBounds(node) {
  return nis(node, 132);
}
function is_TypeFnPointer(node) {
  return nis(node, 133);
}
function is_TypeFnPointerParameter(node) {
  return nis(node, 134);
}
function is_TypeFunction(node) {
  return nis(node, 135);
}
function is_TypeTuple(node) {
  return nis(node, 136);
}
function is_TypeSizedArray(node) {
  return nis(node, 137);
}
function is_TypeSlice(node) {
  return nis(node, 138);
}
function is_TypeReference(node) {
  return nis(node, 139);
}
function is_TypeDereferenceConst(node) {
  return nis(node, 140);
}
function is_TypeDereferenceMut(node) {
  return nis(node, 141);
}
function is_TypeParenthesized(node) {
  return nis(node, 142);
}
function is_ImportNode(node) {
  switch (node.nodeType) {
    case 27:
    case 28:
    case 29:
    case 30:
      return true;
    default:
      return false;
  }
}
function is_StatementNode(node) {
  switch (node.nodeType) {
    case 17:
    case 18:
    case 25:
    case 26:
    case 31:
    case 32:
    case 33:
    case 34:
    case 35:
    case 36:
    case 37:
    case 38:
    case 42:
    case 43:
    case 46:
    case 47:
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
function is_ExpressionNode(node) {
  switch (node.nodeType) {
    case 6:
    case 10:
    case 16:
    case 56:
    case 57:
    case 58:
    case 59:
    case 60:
    case 61:
    case 62:
    case 63:
    case 64:
    case 65:
    case 66:
    case 67:
    case 68:
    case 69:
    case 70:
    case 71:
    case 72:
    case 73:
    case 75:
    case 76:
    case 77:
    case 79:
    case 81:
    case 82:
    case 83:
    case 84:
    case 85:
    case 86:
    case 87:
    case 89:
    case 90:
    case 95:
    case 96:
    case 97:
    case 98:
    case 99:
    case 100:
    case 101:
      return true;
    default:
      return false;
  }
}
function is_PatternNode(node) {
  switch (node.nodeType) {
    case 6:
    case 10:
    case 16:
    case 56:
    case 57:
    case 58:
    case 81:
    case 102:
    case 103:
    case 104:
    case 105:
    case 106:
    case 107:
    case 110:
    case 111:
    case 112:
    case 113:
    case 114:
    case 115:
      return true;
    default:
      return false;
  }
}
function is_TypeNode(node) {
  switch (node.nodeType) {
    case 6:
    case 16:
    case 56:
    case 116:
    case 117:
    case 123:
    case 124:
    case 131:
    case 132:
    case 133:
    case 135:
    case 136:
    case 137:
    case 138:
    case 139:
    case 140:
    case 141:
    case 142:
      return true;
    default:
      return false;
  }
}
function is_AttributeOrComment(node) {
  switch (node.nodeType) {
    case 5:
    case 14:
    case 15:
      return true;
    default:
      return false;
  }
}
function is_AttributeOrDocComment(node) {
  switch (node.nodeType) {
    case 14:
    case 15:
      return true;
    default:
      return false;
  }
}
function is_CommentOrDocComment(node) {
  switch (node.nodeType) {
    case 5:
    case 15:
      return true;
    default:
      return false;
  }
}
function is_IdentifierOrIndex(node) {
  switch (node.nodeType) {
    case 6:
    case 7:
      return true;
    default:
      return false;
  }
}
function is_IdentifierOrItemPath(node) {
  switch (node.nodeType) {
    case 6:
    case 11:
      return true;
    default:
      return false;
  }
}
function is_ExpressionNamespaceTargetNoSelector(node) {
  switch (node.nodeType) {
    case 6:
      return true;
    case 58:
      return void 0 === node.namespace || is_ExpressionNamespaceTargetNoSelector(node.namespace);
    case 57:
      return is_ExpressionNamespaceTargetNoSelector(node.typeCallee);
    default:
      return false;
  }
}
function is_TypeNamespaceTargetNoSelector(node) {
  switch (node.nodeType) {
    case 6:
      return true;
    case 116:
      return void 0 === node.namespace || is_TypeNamespaceTargetNoSelector(node.namespace);
    case 117:
      return is_TypeNamespaceTargetNoSelector(node.typeCallee);
    case 135:
      return is_TypeNamespaceTargetNoSelector(node.callee);
    default:
      return false;
  }
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
function is_TypeNamespaceTarget(node) {
  switch (node.nodeType) {
    case 6:
    case 56:
    case 116:
    case 117:
    case 135:
      return true;
    default:
      return false;
  }
}
function is_PatternNoUnion(node) {
  switch (node.nodeType) {
    case 6:
    case 10:
    case 16:
    case 56:
    case 57:
    case 58:
    case 81:
    case 103:
    case 104:
    case 105:
    case 106:
    case 107:
    case 110:
    case 111:
    case 112:
    case 113:
    case 114:
    case 115:
      return true;
    default:
      return false;
  }
}
function is_PatternNoUnionNoRange(node) {
  switch (node.nodeType) {
    case 6:
    case 10:
    case 16:
    case 56:
    case 57:
    case 58:
    case 81:
    case 103:
    case 104:
    case 105:
    case 106:
    case 107:
    case 110:
    case 111:
    case 112:
    case 113:
    case 114:
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
function is_MacroRule(node) {
  switch (node.nodeType) {
    case 19:
    case 20:
      return true;
    default:
      return false;
  }
}
function is_PathNode(node) {
  switch (node.nodeType) {
    case 11:
    case 58:
    case 116:
      return true;
    default:
      return false;
  }
}
function is_RangeNode(node) {
  switch (node.nodeType) {
    case 89:
    case 115:
      return true;
    default:
      return false;
  }
}
function is_FunctionNode(node) {
  switch (node.nodeType) {
    case 38:
    case 79:
      return true;
    default:
      return false;
  }
}
function is_TypeFunctionNode(node) {
  switch (node.nodeType) {
    case 133:
    case 135:
      return true;
    default:
      return false;
  }
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
function is_ObjectNode(node) {
  switch (node.nodeType) {
    case 42:
    case 46:
    case 50:
    case 90:
    case 107:
      return true;
    default:
      return false;
  }
}
function is_ArrayLikeNode(node) {
  switch (node.nodeType) {
    case 43:
    case 49:
    case 95:
    case 96:
    case 110:
    case 111:
    case 136:
      return true;
    default:
      return false;
  }
}
function is_ArrayOrTupleLiteral(node) {
  switch (node.nodeType) {
    case 95:
    case 96:
      return true;
    default:
      return false;
  }
}
function is_TupleNode(node) {
  switch (node.nodeType) {
    case 43:
    case 49:
    case 95:
    case 110:
    case 136:
      return true;
    default:
      return false;
  }
}
function is_DeclarationNode(node) {
  switch (node.nodeType) {
    case 33:
    case 38:
    case 42:
    case 43:
    case 46:
    case 47:
    case 51:
    case 53:
    case 54:
    case 55:
      return true;
    default:
      return false;
  }
}
function is_TraitDeclarationNode(node) {
  switch (node.nodeType) {
    case 51:
    case 52:
    case 53:
      return true;
    default:
      return false;
  }
}
function is_ImplDeclarationNode(node) {
  switch (node.nodeType) {
    case 54:
    case 55:
      return true;
    default:
      return false;
  }
}
function is_NodeWithSegments(node) {
  switch (node.nodeType) {
    case 13:
    case 14:
    case 15:
    case 16:
    case 21:
      return true;
    default:
      return false;
  }
}
function is_NodeWithBody(node) {
  switch (node.nodeType) {
    case 3:
    case 32:
    case 37:
    case 38:
    case 51:
    case 54:
    case 81:
    case 82:
    case 83:
    case 84:
    case 85:
    case 86:
      return true;
    default:
      return false;
  }
}
function is_NodeWithBodyOrCases(node) {
  switch (node.nodeType) {
    case 3:
    case 32:
    case 37:
    case 38:
    case 51:
    case 54:
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
function is_NodeWithBodyNoBody(node) {
  switch (node.nodeType) {
    case 52:
    case 55:
      return true;
    default:
      return false;
  }
}
function is_NodeWithCondition(node) {
  switch (node.nodeType) {
    case 83:
    case 86:
    case 88:
      return true;
    default:
      return false;
  }
}
function is_ImplicitReturnAbleNode(node) {
  switch (node.nodeType) {
    case 38:
    case 81:
    case 85:
    case 86:
    case 87:
      return true;
    default:
      return false;
  }
}
function is_ExpressionWithBody(node) {
  switch (node.nodeType) {
    case 81:
    case 82:
    case 83:
    case 84:
    case 85:
    case 86:
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
function is_LogicalExpression(node) {
  switch (node.nodeType) {
    case 71:
    case 72:
      return true;
    default:
      return false;
  }
}
function is_LeftRightExpression(node) {
  switch (node.nodeType) {
    case 71:
    case 72:
    case 73:
    case 75:
    case 76:
    case 77:
      return true;
    default:
      return false;
  }
}
function is_FlowControlExpression(node) {
  switch (node.nodeType) {
    case 60:
    case 61:
    case 62:
    case 63:
      return true;
    default:
      return false;
  }
}
function is_UnaryExpression(node) {
  switch (node.nodeType) {
    case 69:
    case 70:
    case 98:
    case 99:
    case 100:
    case 101:
      return true;
    default:
      return false;
  }
}
function is_UnaryPattern(node) {
  switch (node.nodeType) {
    case 112:
    case 113:
    case 114:
      return true;
    default:
      return false;
  }
}
function is_UnaryType(node) {
  switch (node.nodeType) {
    case 139:
    case 140:
    case 141:
      return true;
    default:
      return false;
  }
}
function is_PostfixExpression(node) {
  switch (node.nodeType) {
    case 66:
    case 67:
      return true;
    default:
      return false;
  }
}
function is_VariableDeclarationNode(node) {
  switch (node.nodeType) {
    case 34:
    case 35:
    case 36:
    case 78:
      return true;
    default:
      return false;
  }
}
function is_ReassignmentNode(node) {
  switch (node.nodeType) {
    case 73:
    case 76:
      return true;
    default:
      return false;
  }
}
function is_NodeWithTypeBounds(node) {
  switch (node.nodeType) {
    case 33:
    case 51:
    case 53:
    case 119:
    case 125:
    case 128:
    case 131:
    case 132:
      return true;
    default:
      return false;
  }
}
function is_TypeBoundsStandaloneNode(node) {
  switch (node.nodeType) {
    case 131:
    case 132:
      return true;
    default:
      return false;
  }
}
function is_FunctionLikeNode(node) {
  switch (node.nodeType) {
    case 38:
    case 79:
    case 133:
    case 135:
      return true;
    default:
      return false;
  }
}
function is_FunctionParameterNode(node) {
  switch (node.nodeType) {
    case 6:
    case 16:
    case 39:
    case 40:
    case 41:
    case 56:
    case 80:
    case 116:
    case 117:
    case 123:
    case 124:
    case 131:
    case 132:
    case 133:
    case 134:
    case 135:
    case 136:
    case 137:
    case 138:
    case 139:
    case 140:
    case 141:
    case 142:
      return true;
    default:
      return false;
  }
}
function is_NodeWithMaybePatternNoUnionBody(node) {
  switch (node.nodeType) {
    case 34:
    case 35:
    case 36:
    case 40:
    case 80:
    case 106:
    case 112:
    case 113:
    case 114:
      return true;
    default:
      return false;
  }
}
function is_EnumDeclarationMember(node) {
  switch (node.nodeType) {
    case 48:
    case 49:
    case 50:
      return true;
    default:
      return false;
  }
}
function is_StructProperty(node) {
  switch (node.nodeType) {
    case 91:
    case 92:
    case 93:
    case 94:
      return true;
    default:
      return false;
  }
}
function is_StructPatternProperty(node) {
  switch (node.nodeType) {
    case 104:
    case 108:
    case 109:
      return true;
    default:
      return false;
  }
}
function is_RangePatternBound(node) {
  switch (node.nodeType) {
    case 6:
    case 10:
    case 56:
    case 57:
    case 58:
    case 81:
    case 114:
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
function is_GenericParameterDeclaration(node) {
  switch (node.nodeType) {
    case 125:
    case 126:
    case 127:
      return true;
    default:
      return false;
  }
}
function is_WhereBoundDeclaration(node) {
  switch (node.nodeType) {
    case 128:
    case 129:
      return true;
    default:
      return false;
  }
}
function is_TypeBound(node) {
  switch (node.nodeType) {
    case 120:
    case 121:
    case 122:
    case 130:
    case 142:
      return true;
    default:
      return false;
  }
}
function can_have_OuterAttributes(node, parent, stmt_expr_attributes) {
  if (void 0 !== parent) {
    switch (parent.nodeType) {
      case 36:
        return stmt_expr_attributes && node === parent.expression;
      case 64:
        return parent.arguments.includes(node);
      case 90:
      case 107:
        return parent.properties.includes(node);
      case 95:
      case 96:
      case 110:
        return parent.items.includes(node);
    }
  }
  switch (node.nodeType) {
    case 3:
    case 4:
    case 17:
    case 18:
    case 25:
    case 26:
    case 31:
    case 32:
    case 33:
    case 34:
    case 35:
    case 36:
    case 37:
    case 38:
    case 39:
    case 40:
    case 41:
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
    case 80:
    case 81:
    case 82:
    case 83:
    case 84:
    case 85:
    case 86:
    case 87:
    case 88:
    case 91:
    case 92:
    case 108:
    case 109:
    case 125:
    case 126:
    case 127:
    case 134:
      return true;
    default:
      return false;
  }
}
function getPrecedence(node, insideScrutinee) {
  switch (node.nodeType) {
    case 16 /* MacroInvocation */:
    case 58 /* ExpressionPath */:
    case 64 /* CallExpression */:
    case 65 /* MemberExpression */:
    case 66 /* AwaitExpression */:
    case 67 /* UnwrapExpression */:
    case 90 /* StructLiteral */:
      return 17 /* Top */;
    case 69 /* MinusExpression */:
    case 70 /* NotExpression */:
    case 98 /* ReferenceExpression */:
    case 99 /* RawReferenceExpression */:
    case 100 /* DereferenceExpression */:
    case 101 /* BoxExpression */:
      return 16 /* Unary */;
    case 59 /* ExpressionAsTypeCast */:
      return 15 /* as */;
    case 75 /* OperationExpression */:
      switch (node.tk) {
        case 7 /* * */:
        case 8 /* / */:
        case 9 /* % */:
          return 14;
        case 5 /* + */:
        case 6 /* - */:
          return 13 /* +- */;
        case 13 /* << */:
        case 14 /* >> */:
          return 12 /* >> */;
        case 10 /* & */:
          return 11 /* & */;
        case 12 /* ^ */:
          return 10 /* ^ */;
        case 11 /* | */:
          return 9 /* | */;
      }
      exit.never(node);
    case 77 /* ComparisonExpression */:
      return 8 /* == */;
    case 72 /* AndExpression */:
      return insideScrutinee ? 2 /* Scrutinee && */ : 7 /* && */;
    case 71 /* OrExpression */:
      return insideScrutinee ? 1 /* Scrutinee || */ : 6 /* || */;
    case 89 /* RangeLiteral */:
      return 5 /* .. */;
    case 73 /* ReassignmentExpression */:
    case 76 /* ReassignmentOperationExpression */:
      return 4 /* = */;
    default:
      return insideScrutinee ? 0 /* ScrutineeDefault */ : 3 /* Default */;
  }
}
function is_MacroInvocation_BlockLike(node) {
  return is_MacroInvocation(node) && node.segments.dk === 3 /* {} */;
}
function is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation(node) {
  return is_ExpressionWithBodyOrCases(node) || is_MacroInvocation_BlockLike(node);
}
function is_ElseBlock(node, parent) {
  return is_IfBlockExpression(parent) && parent.else === node;
}
function is_CaseBlock(node, parent) {
  return is_MatchExpressionCase(parent) && parent.expression === node;
}
function is_ClosureBlock(node, parent) {
  return is_ClosureFunctionExpression(parent) && parent.expression === node;
}
function is_FlowControlMaybeValueExpression(node) {
  return is_FlowControlExpression(node) && !is_ContinueExpression(node);
}
function is_BareTypeTraitBound(node) {
  return !hasAttributes(node) && !node.maybeConst && !node.optional && void 0 === node.ltParameters;
}
function is_LiteralStringLike(node) {
  switch (is_Literal(node) ? node.kind : 0 /* False */) {
    case 4 /* bString */:
    case 5 /* brString */:
    case 6 /* rString */:
    case 7 /* String */:
      return true;
    default:
      return false;
  }
}
function is_LiteralRawStringLike(node) {
  switch (is_Literal(node) ? node.kind : 0 /* False */) {
    case 6 /* rString */:
    case 5 /* brString */:
      return true;
    default:
      return false;
  }
}
function is_LiteralNumberLike(node) {
  switch (is_Literal(node) ? node.kind : 0 /* False */) {
    case 8 /* Binary */:
    case 9 /* Hex */:
    case 10 /* Octal */:
    case 11 /* Integer */:
    case 12 /* Float */:
      return true;
    default:
      return false;
  }
}
function is_LiteralBooleanLike(node) {
  switch (is_Literal(node) ? node.kind : 2 /* Char */) {
    case 0 /* False */:
    case 1 /* True */:
      return true;
    default:
      return false;
  }
}
function isInner(node) {
  return node.inner;
}
function isOuter(node) {
  return !node.inner;
}
function isDangling(node) {
  return node.loc.src.program.danglingAttributes.includes(node);
}
function is_LineCommentKind(node) {
  return node.line;
}
function is_BlockCommentKind(node) {
  return !node.line;
}
function is_BlockCommentNode(node) {
  return is_CommentOrDocComment(node) && is_BlockCommentKind(node);
}
function is_LineCommentNode(node) {
  return is_CommentOrDocComment(node) && is_LineCommentKind(node);
}
function can_have_InnerAttributes(node) {
  return is_NodeWithBodyOrCases(node) || is_NodeWithBodyNoBody(node);
}
function can_have_Attributes(node, parent, stmt_expr_attributes) {
  return can_have_InnerAttributes(node) || can_have_OuterAttributes(node, parent, stmt_expr_attributes);
}
function is_multiplicativeOperator(tk) {
  switch (tk) {
    case 7 /* * */:
    case 8 /* / */:
    case 9 /* % */:
      return true;
    default:
      return false;
  }
}
function is_bitshiftOperator(tk) {
  switch (tk) {
    case 13 /* << */:
    case 14 /* >> */:
      return true;
    default:
      return false;
  }
}
function is_BitwiseOperator(tk) {
  switch (tk) {
    case 10 /* & */:
    case 11 /* | */:
    case 12 /* ^ */:
    case 13 /* << */:
    case 14 /* >> */:
      return true;
    default:
      return false;
  }
}
function is_LargerLesserOperator(tk) {
  switch (tk) {
    case 17 /* > */:
    case 18 /* >= */:
    case 19 /* < */:
    case 20 /* <= */:
      return true;
    default:
      return false;
  }
}
function is_EqualityOperator(tk) {
  switch (tk) {
    case 15 /* == */:
    case 16 /* != */:
      return true;
    default:
      return false;
  }
}

// src/utils/ast/helpers.ts
function hasAttributes(node) {
  return has_key(node, "attributes");
}
function hasOuterAttributes(node) {
  return hasAttributes(node) && isOuter(node.attributes[0]);
}
function hasInnerAttributes(node) {
  return hasAttributes(node) && isInner(last_of(node.attributes));
}
function hasSuffix(node) {
  return has_key(node, "suffix");
}
function hasCondition(node) {
  return is_NodeWithCondition(node) && is_defined(node.condition);
}
function hasLetScrutineeCondition(node) {
  return hasCondition(node) && r(node.condition);
  function r(node2) {
    return is_LogicalExpression(node2) ? r(node2.left) || r(node2.right) : is_LetScrutinee(node2);
  }
}
function hasSemiNoProperties(node) {
  return has_key_undefined(node, "properties");
}
function hasSemiNoBody(node) {
  return has_key_undefined(node, "body");
}
function hasProperties(node) {
  return has_key_non_empty_array(node, "properties");
}
function hasItems(node) {
  return has_key_non_empty_array(node, "items");
}
function hasBody(node) {
  return has_key_non_empty_array(node, "body");
}
function hasGenerics(node) {
  return is_DeclarationNode(node) && has_key_non_empty_array(node, "generics");
}
function hasSelfParameter(node) {
  return is_FunctionDeclaration(node) && is_defined(node.parameters.self);
}
function hasParameters(node) {
  return is_FunctionLikeNode(node) && (node.parameters.length > 0 || hasSelfParameter(node));
}
function hasMethod(node) {
  return is_defined(node.method);
}
function hasTypeBounds(node) {
  return is_NodeWithTypeBounds(node) && is_defined(node.typeBounds);
}
function hasExpression(node) {
  return has_key_defined(node, "expression");
}
function getParameters(node) {
  return hasSelfParameter(node) ? [node.parameters.self, ...node.parameters] : node.parameters;
}
function getFirstParameter(node) {
  return hasSelfParameter(node) ? node.parameters.self : node.parameters.length > 0 ? node.parameters[0] : void 0;
}
function getLastParameter(node) {
  return isEmpty(node.parameters) ? hasSelfParameter(node) ? node.parameters.self : void 0 : last_of(node.parameters);
}
function getBodyOrCases(node) {
  return "body" in node ? node.body : node.cases;
}
function getMacroName(node) {
  const callee = node.callee;
  if (is_Identifier(callee))
    return callee.name;
  if (is_PathNode(callee))
    return callee.segment.name;
  exit("Expected Identifier | PathNode", node);
}
function getLeftMostCondition(node) {
  for (var target = node; is_LogicalExpression(target); target = target.left)
    ;
  return target;
}
function getParenthesizedNodeContent(node) {
  switch (node.nodeType) {
    case 68 /* ParenthesizedExpression */:
      return node.expression;
    case 103 /* ParenthesizedPattern */:
      return node.pattern;
    case 142 /* TypeParenthesized */:
      return node.typeExpression;
  }
  exit.never(node);
}
function countRawLiteralHashtags(node) {
  return is_LiteralRawStringLike(node) ? node.loc.len() - (1 + node.value.lastIndexOf('"')) : 0;
}
function getDelimChars(node) {
  switch (node.dk) {
    case 5 /* || */:
      return { left: "|", right: "|" };
    case 1 /* () */:
      return { left: "(", right: ")" };
    case 2 /* [] */:
      return { left: "[", right: "]" };
    case 3 /* {} */:
      return { left: "{", right: "}" };
    case 4 /* <> */:
      return { left: "<", right: ">" };
  }
  exit.never(node);
}

// src/utils/ast/transform.ts
function unsafe_set_nodeType(node, nodeType) {
  node.nodeType = nodeType;
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
function deleteAttributes(target) {
  if (hasOwnStart(target)) {
    unsafe_setRangeStart(target, ownStart(target));
    delete target.loc[2];
  }
  delete target.attributes;
}
function assignAttributes(target, attrs) {
  if (hasAttributes(target))
    insertNodes(target.attributes, attrs);
  else
    target.attributes = [...attrs];
  if (isOuter(attrs[0])) {
    if (!hasOwnStart(target))
      internal_saveOwnStart(target);
    setRangeStart(target, start(attrs[0]));
  }
}
function transferAttributes(from, to) {
  if (hasAttributes(from)) {
    assignAttributes(to, from.attributes);
    deleteAttributes(from);
  }
}
function insertNode(array, node) {
  binaryInsertIn(array, node, start);
}
function insertNodes(array, nodes) {
  binaryInsertEach(array, nodes, start);
}

// src/utils/ast/iterator.ts
function _it_arr(fn, array, key) {
  for (var i = 0; i < array.length; i++)
    fn(array[i], key, i);
}
function iterate_nodes(node, fn) {
  if ("attributes" in node)
    _it_arr(fn, node.attributes, "attributes");
  switch (node.nodeType) {
    case 0:
    case 2:
    case 5:
    case 6:
    case 7:
    case 8:
    case 9:
    case 12:
    case 15:
    case 41:
    case 74:
    case 94:
    case 104:
    case 105:
    case 120:
    case 121:
    case 122:
    case 123:
    case 124:
      break;
    case 1:
      if ("shebang" in node)
        fn(node.shebang, "shebang");
      fn(node.program, "program");
      break;
    case 3:
      _it_arr(fn, node.ast, "ast");
      _it_arr(fn, node.danglingAttributes, "danglingAttributes");
      _it_arr(fn, node.comments, "comments");
      break;
    case 4:
      if (is_LocArray(node.ast))
        _it_arr(fn, node.ast, "ast");
      else
        fn(node.ast, "ast");
      _it_arr(fn, node.danglingAttributes, "danglingAttributes");
      _it_arr(fn, node.comments, "comments");
      break;
    case 10:
      if ("suffix" in node)
        fn(node.suffix, "suffix");
      break;
    case 11:
    case 58:
    case 116:
      if (void 0 !== node.namespace)
        fn(node.namespace, "namespace");
      fn(node.segment, "segment");
      break;
    case 13:
    case 14:
      _it_arr(fn, node.segments, "segments");
      break;
    case 16:
      fn(node.callee, "callee");
      _it_arr(fn, node.segments, "segments");
      break;
    case 17:
      fn(node.id, "id");
      _it_arr(fn, node.rules, "rules");
      break;
    case 18:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (is_LocArray(node.rules))
        _it_arr(fn, node.rules, "rules");
      else
        fn(node.rules, "rules");
      break;
    case 19:
    case 20:
      _it_arr(fn, node.match, "match");
      _it_arr(fn, node.transform, "transform");
      break;
    case 21:
      _it_arr(fn, node.segments, "segments");
      if (void 0 !== node.sep)
        fn(node.sep, "sep");
      break;
    case 22:
      fn(node.id, "id");
      fn(node.ty, "ty");
      break;
    case 23:
      if (void 0 !== node.location)
        fn(node.location, "location");
      break;
    case 24:
      if (void 0 !== node.abi)
        fn(node.abi, "abi");
      break;
    case 25:
    case 60:
    case 63:
      if (void 0 !== node.expression)
        fn(node.expression, "expression");
      break;
    case 26:
    case 31:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.import, "import");
      break;
    case 27:
      fn(node.source, "source");
      if (void 0 !== node.local)
        fn(node.local, "local");
      break;
    case 28:
      if (void 0 !== node.source)
        fn(node.source, "source");
      break;
    case 29:
      fn(node.source, "source");
      break;
    case 30:
      if (void 0 !== node.source)
        fn(node.source, "source");
      _it_arr(fn, node.specifiers, "specifiers");
      break;
    case 32:
      if ("pub" in node)
        fn(node.pub, "pub");
      if (void 0 !== node.abi)
        fn(node.abi, "abi");
      _it_arr(fn, node.body, "body");
      break;
    case 33:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      if (void 0 !== node.typeBounds)
        _it_arr(fn, node.typeBounds, "typeBounds");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      if (void 0 !== node.typeExpression)
        fn(node.typeExpression, "typeExpression");
      break;
    case 34:
    case 35:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.pattern, "pattern");
      fn(node.typeAnnotation, "typeAnnotation");
      if (void 0 !== node.expression)
        fn(node.expression, "expression");
      break;
    case 36:
      fn(node.pattern, "pattern");
      if (void 0 !== node.typeAnnotation)
        fn(node.typeAnnotation, "typeAnnotation");
      if (void 0 !== node.expression)
        fn(node.expression, "expression");
      if (void 0 !== node.else)
        fn(node.else, "else");
      break;
    case 37:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (void 0 !== node.body)
        _it_arr(fn, node.body, "body");
      break;
    case 38:
      if ("pub" in node)
        fn(node.pub, "pub");
      if ("extern" in node)
        fn(node.extern, "extern");
      fn(node.id, "id");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      if (void 0 !== node.parameters.self)
        fn(node.parameters.self, "parameters", "self");
      _it_arr(fn, node.parameters, "parameters");
      if (void 0 !== node.returnType)
        fn(node.returnType, "returnType");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      if (void 0 !== node.body)
        _it_arr(fn, node.body, "body");
      break;
    case 39:
      if (void 0 !== node.lt)
        fn(node.lt, "lt");
      if (void 0 !== node.typeAnnotation)
        fn(node.typeAnnotation, "typeAnnotation");
      break;
    case 40:
      fn(node.pattern, "pattern");
      fn(node.typeAnnotation, "typeAnnotation");
      break;
    case 42:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      if (void 0 !== node.properties)
        _it_arr(fn, node.properties, "properties");
      break;
    case 43:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      _it_arr(fn, node.items, "items");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      break;
    case 44:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      fn(node.typeAnnotation, "typeAnnotation");
      break;
    case 45:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.typeAnnotation, "typeAnnotation");
      break;
    case 46:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      _it_arr(fn, node.properties, "properties");
      break;
    case 47:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      _it_arr(fn, node.members, "members");
      break;
    case 48:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (void 0 !== node.value)
        fn(node.value, "value");
      break;
    case 49:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      _it_arr(fn, node.items, "items");
      if (void 0 !== node.value)
        fn(node.value, "value");
      break;
    case 50:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      _it_arr(fn, node.properties, "properties");
      if (void 0 !== node.value)
        fn(node.value, "value");
      break;
    case 51:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      if (void 0 !== node.typeBounds)
        _it_arr(fn, node.typeBounds, "typeBounds");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      _it_arr(fn, node.body, "body");
      break;
    case 52:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      break;
    case 53:
      if ("pub" in node)
        fn(node.pub, "pub");
      fn(node.id, "id");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      _it_arr(fn, node.typeBounds, "typeBounds");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      break;
    case 54:
      if ("pub" in node)
        fn(node.pub, "pub");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      if (void 0 !== node.trait)
        fn(node.trait, "trait");
      fn(node.typeTarget, "typeTarget");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      _it_arr(fn, node.body, "body");
      break;
    case 55:
      if ("pub" in node)
        fn(node.pub, "pub");
      if (void 0 !== node.generics)
        _it_arr(fn, node.generics, "generics");
      fn(node.trait, "trait");
      fn(node.typeTarget, "typeTarget");
      if (void 0 !== node.whereBounds)
        _it_arr(fn, node.whereBounds, "whereBounds");
      break;
    case 56:
      fn(node.typeTarget, "typeTarget");
      if (void 0 !== node.typeExpression)
        fn(node.typeExpression, "typeExpression");
      break;
    case 57:
    case 117:
      fn(node.typeCallee, "typeCallee");
      _it_arr(fn, node.typeArguments, "typeArguments");
      break;
    case 59:
      fn(node.expression, "expression");
      fn(node.typeExpression, "typeExpression");
      break;
    case 61:
      if (void 0 !== node.label)
        fn(node.label, "label");
      if (void 0 !== node.expression)
        fn(node.expression, "expression");
      break;
    case 62:
      if (void 0 !== node.label)
        fn(node.label, "label");
      break;
    case 64:
      fn(node.callee, "callee");
      if (void 0 !== node.method)
        fn(node.method, "method");
      if (void 0 !== node.typeArguments)
        _it_arr(fn, node.typeArguments, "typeArguments");
      _it_arr(fn, node.arguments, "arguments");
      break;
    case 65:
      fn(node.expression, "expression");
      fn(node.property, "property");
      break;
    case 66:
    case 67:
    case 68:
    case 69:
    case 70:
    case 93:
    case 98:
    case 99:
    case 100:
    case 101:
      fn(node.expression, "expression");
      break;
    case 71:
    case 72:
    case 73:
    case 75:
    case 76:
    case 77:
      fn(node.left, "left");
      fn(node.right, "right");
      break;
    case 78:
      fn(node.pattern, "pattern");
      fn(node.expression, "expression");
      break;
    case 79:
      _it_arr(fn, node.parameters, "parameters");
      if (void 0 !== node.returnType)
        fn(node.returnType, "returnType");
      fn(node.expression, "expression");
      break;
    case 80:
      fn(node.pattern, "pattern");
      if (void 0 !== node.typeAnnotation)
        fn(node.typeAnnotation, "typeAnnotation");
      break;
    case 81:
    case 82:
    case 85:
      if (void 0 !== node.label)
        fn(node.label, "label");
      _it_arr(fn, node.body, "body");
      break;
    case 83:
      if (void 0 !== node.label)
        fn(node.label, "label");
      fn(node.condition, "condition");
      _it_arr(fn, node.body, "body");
      break;
    case 84:
      if (void 0 !== node.label)
        fn(node.label, "label");
      fn(node.pattern, "pattern");
      fn(node.expression, "expression");
      _it_arr(fn, node.body, "body");
      break;
    case 86:
      if (void 0 !== node.label)
        fn(node.label, "label");
      fn(node.condition, "condition");
      _it_arr(fn, node.body, "body");
      if (void 0 !== node.else)
        fn(node.else, "else");
      break;
    case 87:
      if (void 0 !== node.label)
        fn(node.label, "label");
      fn(node.expression, "expression");
      _it_arr(fn, node.cases, "cases");
      break;
    case 88:
      fn(node.pattern, "pattern");
      if (void 0 !== node.condition)
        fn(node.condition, "condition");
      fn(node.expression, "expression");
      break;
    case 89:
    case 115:
      if (void 0 !== node.lower)
        fn(node.lower, "lower");
      if (void 0 !== node.upper)
        fn(node.upper, "upper");
      break;
    case 90:
    case 107:
      fn(node.struct, "struct");
      _it_arr(fn, node.properties, "properties");
      break;
    case 91:
      fn(node.key, "key");
      fn(node.value, "value");
      break;
    case 92:
      fn(node.value, "value");
      break;
    case 95:
    case 96:
    case 111:
    case 136:
      _it_arr(fn, node.items, "items");
      break;
    case 97:
      fn(node.initExpression, "initExpression");
      fn(node.sizeExpression, "sizeExpression");
      break;
    case 102:
      _it_arr(fn, node.patterns, "patterns");
      break;
    case 103:
    case 112:
    case 113:
    case 114:
      fn(node.pattern, "pattern");
      break;
    case 106:
      fn(node.id, "id");
      if (void 0 !== node.pattern)
        fn(node.pattern, "pattern");
      break;
    case 108:
      fn(node.key, "key");
      fn(node.pattern, "pattern");
      break;
    case 109:
      fn(node.id, "id");
      break;
    case 110:
      if (void 0 !== node.struct)
        fn(node.struct, "struct");
      _it_arr(fn, node.items, "items");
      break;
    case 118:
      fn(node.target, "target");
      fn(node.typeExpression, "typeExpression");
      break;
    case 119:
      fn(node.typeTarget, "typeTarget");
      _it_arr(fn, node.typeBounds, "typeBounds");
      break;
    case 125:
      fn(node.id, "id");
      if (void 0 !== node.typeBounds)
        _it_arr(fn, node.typeBounds, "typeBounds");
      if (void 0 !== node.typeDefault)
        fn(node.typeDefault, "typeDefault");
      break;
    case 126:
      fn(node.id, "id");
      fn(node.typeAnnotation, "typeAnnotation");
      if (void 0 !== node.typeDefault)
        fn(node.typeDefault, "typeDefault");
      break;
    case 127:
      fn(node.id, "id");
      if (void 0 !== node.ltBounds)
        _it_arr(fn, node.ltBounds, "ltBounds");
      break;
    case 128:
      if (void 0 !== node.ltParameters)
        _it_arr(fn, node.ltParameters, "ltParameters");
      fn(node.typeTarget, "typeTarget");
      _it_arr(fn, node.typeBounds, "typeBounds");
      break;
    case 129:
      fn(node.ltTarget, "ltTarget");
      _it_arr(fn, node.ltBounds, "ltBounds");
      break;
    case 130:
      if (void 0 !== node.ltParameters)
        _it_arr(fn, node.ltParameters, "ltParameters");
      fn(node.typeExpression, "typeExpression");
      break;
    case 131:
    case 132:
      _it_arr(fn, node.typeBounds, "typeBounds");
      break;
    case 133:
      if ("extern" in node)
        fn(node.extern, "extern");
      if (void 0 !== node.ltParameters)
        _it_arr(fn, node.ltParameters, "ltParameters");
      _it_arr(fn, node.parameters, "parameters");
      if (void 0 !== node.returnType)
        fn(node.returnType, "returnType");
      break;
    case 134:
      if (void 0 !== node.id)
        fn(node.id, "id");
      fn(node.typeAnnotation, "typeAnnotation");
      break;
    case 135:
      fn(node.callee, "callee");
      _it_arr(fn, node.parameters, "parameters");
      if (void 0 !== node.returnType)
        fn(node.returnType, "returnType");
      break;
    case 137:
      fn(node.typeExpression, "typeExpression");
      fn(node.sizeExpression, "sizeExpression");
      break;
    case 138:
    case 140:
    case 141:
    case 142:
      fn(node.typeExpression, "typeExpression");
      break;
    case 139:
      if (void 0 !== node.lt)
        fn(node.lt, "lt");
      fn(node.typeExpression, "typeExpression");
      break;
    default:
      exit.never(node);
  }
}
function each_childNode(target, callback) {
  iterate_nodes(target, function f(child, ...key_ind) {
    callback(child, target, ...key_ind);
  });
}
function each_node(target, callback) {
  let i = 0;
  const ancestry = [target];
  iterate_nodes(target, function f(child, ...key_ind) {
    iterate_nodes(ancestry[++i] = child, f);
    callback(child, ancestry[--i], ...key_ind);
  });
}
function getNodeChildren(node) {
  const children = [];
  iterate_nodes(node, function f(child) {
    insertNode(children, child);
  });
  return children;
}
function hasChildren(node) {
  try {
    iterate_nodes(node, () => {
      throw 0;
    });
  } catch (e) {
    if (0 === e)
      return true;
    throw e;
  }
  return false;
}
function getAstPath(parent, target) {
  let j = 0;
  const p = [];
  try {
    (function r(parent2) {
      if (parent2 === target)
        throw 0;
      iterate_nodes(parent2, (n, k, i) => {
        if (void 0 === i)
          p[j++] = k, r(n), j--;
        else
          p[j++] = k, p[j++] = i, r(n), j -= 2;
      });
    })(parent);
  } catch (e) {
    if (0 === e)
      return p.length = j, p;
    throw e;
  }
  exit(`Could not find target ${target.type} in parent ${parent.type}`, { parent, target });
}
function getOwnChildAstPath(parent, child) {
  let r;
  try {
    iterate_nodes(parent, (n, ...p) => {
      if (n === child) {
        r = p;
        throw 0;
      }
    });
  } catch (e) {
    if (0 === e)
      return r;
    throw e;
  }
  exit(`Could not find child ${child.type} in parent ${parent.type}`, { parent, child });
}
function reassignNodeProperty(value, parent, key, index) {
  if (typeof index === "undefined")
    parent[key] = value;
  else
    parent[key][index] = value;
}
function _hasSelf(value) {
  return "self" in value && is_defined(value.self);
}
function getActualNodeChildren(node) {
  const children = [];
  for (var key in node) {
    var value = node[key];
    if (is_array(value)) {
      if (_hasSelf(value))
        insertNode(children, value.self);
      insertNodes(children, value);
    } else if (is_Node(value)) {
      insertNode(children, value);
    }
  }
  return children;
}
function countActualNodeChildren(node) {
  var l = 0;
  for (var key in node) {
    var value = node[key];
    if (is_array(value)) {
      if (_hasSelf(value))
        l += 1;
      l += value.length;
    } else if (is_Node(value)) {
      l += 1;
    }
  }
  return l;
}

// src/utils/ast/index.ts
function is_Loc(data) {
  return is_object(data) && "src" in data;
}
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
function isLocEqual(a, b) {
  return (2 in a ? 2 in b && a[2] === b[2] : !(2 in b)) && a[0] === b[0] && a[1] === b[1] && a.src === b.src;
}

export { assignAttributes, can_have_Attributes, can_have_InnerAttributes, can_have_OuterAttributes, countActualNodeChildren, countRawLiteralHashtags, deleteAttributes, each_childNode, each_node, end, getActualNodeChildren, getAstPath, getBodyOrCases, getDelimChars, getFirstParameter, getLastParameter, getLeftMostCondition, getMacroName, getNodeChildren, getOwnChildAstPath, getParameters, getParenthesizedNodeContent, getPrecedence, hasAttributes, hasBody, hasChildren, hasCondition, hasExpression, hasGenerics, hasInnerAttributes, hasItems, hasLetScrutineeCondition, hasMethod, hasOuterAttributes, hasOwnStart, hasParameters, hasProperties, hasSelfParameter, hasSemiNoBody, hasSemiNoProperties, hasSuffix, hasTypeBounds, includesTK, insertNode, insertNodes, isDangling, isInner, isLocEqual, isOuter, isTK, is_AmbientImport, is_AndExpression, is_AnonymousImport, is_ArrayLikeNode, is_ArrayLiteral, is_ArrayOrTupleLiteral, is_ArrayPattern, is_Attribute, is_AttributeOrComment, is_AttributeOrDocComment, is_AutoTraitDeclaration, is_AwaitExpression, is_BareTypeTraitBound, is_BitwiseOperator, is_BlockCommentKind, is_BlockCommentNode, is_BlockExpression, is_BoxExpression, is_BoxPattern, is_BreakExpression, is_CallExpression, is_CaseBlock, is_ClosureBlock, is_ClosureFunctionExpression, is_ClosureFunctionParameterDeclaration, is_Comment, is_CommentOrDocComment, is_ComparisonExpression, is_ConstTypeParameterDeclaration, is_ConstVariableDeclaration, is_ContinueExpression, is_DeclarationNode, is_DelimGroup, is_DereferenceExpression, is_DestructuredImport, is_DocCommentAttribute, is_ElseBlock, is_EnumDeclaration, is_EnumDeclarationMember, is_EnumMemberDeclaration, is_EnumMemberStructDeclaration, is_EnumMemberTupleDeclaration, is_EqualityOperator, is_ExpressionAsTypeCast, is_ExpressionNamespaceTarget, is_ExpressionNamespaceTargetNoSelector, is_ExpressionNode, is_ExpressionPath, is_ExpressionStatement, is_ExpressionTypeCast, is_ExpressionTypeSelector, is_ExpressionWithBody, is_ExpressionWithBodyOrCases, is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation, is_ExternBlockDeclaration, is_ExternCrateStatement, is_ExternSpecifier, is_FlowControlExpression, is_FlowControlMaybeValueExpression, is_ForInBlockExpression, is_FunctionDeclaration, is_FunctionLikeNode, is_FunctionNode, is_FunctionParameterDeclaration, is_FunctionParameterNode, is_FunctionSelfParameterDeclaration, is_FunctionSpread, is_GenericLtParameterDeclaration, is_GenericParameterDeclaration, is_GenericTypeParameterDeclaration, is_Identifier, is_IdentifierOrIndex, is_IdentifierOrItemPath, is_IfBlockExpression, is_ImplDeclaration, is_ImplDeclarationNode, is_ImplicitReturnAbleNode, is_ImportNode, is_Index, is_ItemPath, is_LargerLesserOperator, is_LbIdentifier, is_LeftRightExpression, is_LetScrutinee, is_LetVariableDeclaration, is_Lifetime, is_LineCommentKind, is_LineCommentNode, is_Literal, is_LiteralBooleanLike, is_LiteralNumberLike, is_LiteralRawStringLike, is_LiteralStringLike, is_Loc, is_LocArray, is_Located, is_LogicalExpression, is_LoopBlockExpression, is_LtElided, is_LtIdentifier, is_LtStatic, is_MacroDeclaration, is_MacroGroup, is_MacroInlineRuleDeclaration, is_MacroInvocation, is_MacroInvocation_BlockLike, is_MacroParameterDeclaration, is_MacroRule, is_MacroRuleDeclaration, is_MacroRulesDeclaration, is_MatchExpression, is_MatchExpressionCase, is_MaybeAsyncNode, is_MaybeExternNode, is_MaybeMoveNode, is_MaybePubNode, is_MaybeStaticNode, is_MaybeUnsafeNode, is_McIdentifier, is_MemberExpression, is_MinusExpression, is_MinusPattern, is_MissingNode, is_ModuleDeclaration, is_NamedImport, is_NegativeImplDeclaration, is_Node, is_NodeWithBody, is_NodeWithBodyNoBody, is_NodeWithBodyOrCases, is_NodeWithCondition, is_NodeWithMaybePatternNoUnionBody, is_NodeWithSegments, is_NodeWithTypeBounds, is_NotExpression, is_ObjectNode, is_OperationExpression, is_OrExpression, is_ParenthesizedExpression, is_ParenthesizedNode, is_ParenthesizedPattern, is_PathNode, is_PatternNoUnion, is_PatternNoUnionNoRange, is_PatternNode, is_PatternVariableDeclaration, is_PostfixExpression, is_Program, is_PubSpecifier, is_PunctuationToken, is_RangeLiteral, is_RangeNode, is_RangePattern, is_RangePatternBound, is_RawReferenceExpression, is_ReassignmentExpression, is_ReassignmentNode, is_ReassignmentOperationExpression, is_ReferenceExpression, is_ReferencePattern, is_RestPattern, is_ReturnExpression, is_Shebang, is_SizedArrayLiteral, is_Snippet, is_SourceFile, is_StatementNode, is_StaticVariableDeclaration, is_StructDeclaration, is_StructLiteral, is_StructLiteralProperty, is_StructLiteralPropertyShorthand, is_StructLiteralPropertySpread, is_StructLiteralRestUnassigned, is_StructPattern, is_StructPatternProperty, is_StructPatternPropertyDestructured, is_StructPatternPropertyShorthand, is_StructProperty, is_StructPropertyDeclaration, is_TraitAliasDeclaration, is_TraitDeclaration, is_TraitDeclarationNode, is_TryBlockExpression, is_TupleLiteral, is_TupleNode, is_TuplePattern, is_TupleStructDeclaration, is_TupleStructItemDeclaration, is_TypeAliasDeclaration, is_TypeBound, is_TypeBoundsStandaloneNode, is_TypeCall, is_TypeCallNamedArgument, is_TypeCallNamedBound, is_TypeDereferenceConst, is_TypeDereferenceMut, is_TypeDynBounds, is_TypeFnPointer, is_TypeFnPointerParameter, is_TypeFunction, is_TypeFunctionNode, is_TypeImplBounds, is_TypeInferred, is_TypeNamespaceTarget, is_TypeNamespaceTargetNoSelector, is_TypeNever, is_TypeNode, is_TypeParenthesized, is_TypePath, is_TypeReference, is_TypeSizedArray, is_TypeSlice, is_TypeTraitBound, is_TypeTuple, is_UnaryExpression, is_UnaryPattern, is_UnaryType, is_UnassignedExpression, is_UnionDeclaration, is_UnionPattern, is_UnwrapExpression, is_UseStatement, is_VariableDeclarationNode, is_WhereBoundDeclaration, is_WhereLtBoundDeclaration, is_WhereTypeBoundDeclaration, is_WhileBlockExpression, is_WildcardPattern, is_YieldExpression, is_bitshiftOperator, is_multiplicativeOperator, nisAnyOf, ownStart, reassignNodeProperty, setRange, setRangeEnd, setRangeStart, start, transferAttributes, unsafe_setRangeEnd, unsafe_setRangeStart, unsafe_set_nodeType };
