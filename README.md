# jinx-rust &middot; ![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg) [![npm version](https://img.shields.io/npm/v/jinx-rust.svg?style=flat)](https://www.npmjs.com/package/jinx-rust) ![GitHub Repo stars](https://img.shields.io/github/stars/jinxdash/jinx-rust?style=social) [![Twitter Follow](https://img.shields.io/twitter/follow/jinxdash?style=social)](https://twitter.com/jinxdash)

`jinx-rust` is a [Rust](https://www.rust-lang.org/) parser written in Typescript.   

## Get Started

```
npm install jinx-rust
```

```ts
import { rs } from "jinx-rust";

const file = rs.parseFile(`let foo: u8 = 1;`);

console.log(JSON.stringify(file));
```

```json
{
	"type": "SourceFile",
	"program": {
		"type": "Program",
		"ast": [
			{
				"type": "LetVariableDeclaration",
				"pattern": { "type": "Identifier", "name": "foo" },
				"typeAnnotation": { "type": "Identifier", "name": "u8" },
				"expression": { "type": "Literal", "kind": 11, "value": "1" }
			}
		],
		"danglingAttributes": [],
		"comments": []
	}
}
```

## Loose parsing

Though eventually there should be a `strict` parser option to validate the AST,  
`jinx-rust` is unstrict by default and tolerates many things:

- Missing semicolons
- Missing comas in `match` and declarations
- Labels on blocks that can't have them
- Unsyntactic Attributes and Doc Attributes
- Unsyntactic parentheses in RangePatterns
- Malformed tokens, Javascript's `===` and `!==`
- Closures with a returnType followed by an inlined expression 
- Forbidden node types (e.g. expressions in top level, patterns in `const` declarations)
- Missing nodes (e.g. fn `parameter.typeAnnotation`)


```ts
import { rs } from "jinx-rust";

// Would not parse in Rust or syn
const arg_0 = rs.parseFile("fn foo(arg_0) {}").program.ast[0].parameters[0];

assert(arg_0.typeAnnotation.type === "MissingNode");
```

## Conversion between nodes and tokens

Attributes and Macro invocation arguments are returned as tokens in `rs.parseFile`.  
`rs` exposes other methods to re-read tokens in arbitrary contexts, or to re-read nodes as tokens.

```ts
import { rs, MacroInvocation } from "jinx-rust";

const node = rs.parseFile("foo!(123);").program.ast[0].expression as MacroInvocation;

// ExpressionNode[]
const args = rs.toCallExpressionArguments(node.tokens).ast;

// StatementNode[]
const block = rs.toBlockBody(node.tokens).ast;

// TokenNode[]
const tokens = rs.toTokens(node).ast;
```

## import AST helpers from `"jinx-rust/utils"`

`jinx-rust/utils` is automatically included on install. It is a library of (mostly) auto-generated helpers from the parser's type declarations. Like `each_node` traversing, or `is_{NodeType}` functions for every node type, and `is_{Type}` for every type exported by the parser.  

```ts
import { each_node, is_StatementNode } from "jinx-rust/utils";

declare const target: Node;

each_node(target, (child, parent) => {
	if (is_StatementNode(child)) {
		// gets called for every statement in target
	}
});
```

## Gotchas when working with `jinx-rust`


- When a node has outer attributes, its start location expands to include them. Its own start position is saved under `node.loc.ownStart`   
```ts
import { Node } from "jinx-rust";
import { start, end, ownStart, has_OuterAttributes, hasOwnStart } from "jinx-rust/utils";

declare const node: Node;

start(node) === node.loc[0]; end(node) === node.loc[1];

has_OuterAttributes(node) === hasOwnStart(node);
ownStart(node) === (node.loc.ownStart ?? node.loc[0]);
```

## Projects using `jinx-rust`

- [`prettier-plugin-rust`]("https://github.com/jinxdash/prettier-plugin-rust")
