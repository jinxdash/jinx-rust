# jinx-rust &middot; ![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg) [![npm version](https://img.shields.io/npm/v/jinx-rust.svg?style=flat)](https://www.npmjs.com/package/jinx-rust) ![GitHub Repo stars](https://img.shields.io/github/stars/jinxdash/jinx-rust?style=social) [![Twitter Follow](https://img.shields.io/twitter/follow/jinxdash?style=social)](https://twitter.com/jinxdash)

`jinx-rust` is a [Rust](https://www.rust-lang.org/) parser written in Typescript; it enables the development of Rust Tooling in Typescript.

Example project using `jinx-rust`: [Prettier Rust formatter](https://github.com/jinxdash/prettier-plugin-rust)

## Get Started

```sh
npm install jinx-rust
```

```ts
import { rs } from "jinx-rust";

const file = rs.parseFile("let leet: u32 = 1337;");

console.log(JSON.stringify(file));

{
  "type": "SourceFile",
  "program": {
    "type": "Program",
    "ast": [
      {
        "type": "LetVariableDeclaration",
        "pattern": { "type": "Identifier", "name": "leet" },
        "typeAnnotation": { "type": "Identifier", "name": "u32" },
        "expression": { "type": "Literal", "kind": 11, "value": "1337" }
      }
    ],
    "danglingAttributes": [],
    "comments": []
  }
}
```

## A tolerant parser for better Rust Tooling

`jinx-rust` is unstrict by default and tolerates bad syntax

- Tokens:

  - Missing semicolons
  - Missing commas in some places
  - Whitespace within long tokens
  - Javascript-ey syntax, (currently just `===` and `!==`)
  - Unsyntactic parentheses (e.g. RangePattern bounds)
  - Closures with a returnType and a non-block expression

- Nodes:

  - Unsyntactic Labels, Attributes and Doc Attributes
  - Arbitrary specifier order (e.g. `unsafe pub fn ...`)
  - Forbidden node types (e.g. patterns in const variables, expressions in top level)
  - Missing typeAnnotations


```ts
import { rs } from "jinx-rust";

const arg_0 = rs.parseFile("fn foo(arg_0) {}").program.ast[0].parameters[0];

assert(arg_0.typeAnnotation.type === "MissingNode");
```

In the future, `jinx-rust` should eventually get a `strict` option.

## Conversion between nodes and tokens

Attributes and Macro invocation arguments are returned as tokens in `rs.parseFile`.  
`rs` exposes other methods to re-read tokens in arbitrary contexts, or to re-read nodes as tokens.

```ts
import { rs, MacroInvocation } from "jinx-rust";

const node = rs.parseFile("foo!(123);").program.ast[0].expression as MacroInvocation;

const args = rs.toCallExpressionArguments(node.tokens).ast; // ExpressionNode[]
const block = rs.toBlockBody(node.tokens).ast; // StatementNode[]

const tokens = rs.toTokens(node).ast; // TokenNode[]
```

## Nightly features

`jinx-rust` supports 23 nightly features. The full list can be found in `src/parser/nodes.ts` under `enum Feature`. ([link](https://github.com/jinxdash/jinx-rust/blob/5fcd69e007e8401220db94710c5a879d686ee795/src/parser/nodes.ts#L93-L139))

## `jinx-rust/utils`

`jinx-rust/utils` is automatically included on install. It is a library of (mostly) auto-generated helpers from the parser's type declarations. E.g. tree traversing helpers, `is_{Type}(node)` functions for every type declared exported by the parser.

```ts
import { each_node, is_StatementNode } from "jinx-rust/utils";

declare const target: Node;

each_node(target, (child, parent) => {
  if (is_StatementNode(child)) {
    // gets called for every statement in target
  }
});
```

## Gotchas

- When a node has outer attributes, its start location expands to include them,  
  its own start positions is saved under `node.loc.ownStart`

* Prefer `jinx-rust/utils` helpers to access locations, e.g. `start(node)` instead of `node.loc[0]`

- Exported `.d.ts` nodes each document their syntax, hit `Go to Definition` or hover over their class to see it.

  ```ts
  import { TraitDeclaration } from "jinx-rust";

  TraitDeclaration;
  // ^? import TraitDeclaration
  //    trait id<...generics>?: ...typeBounds? where ...whereBounds? { ...body }
  ```

- `filepath` is not required but useful in debugging: `rs.parseFile(code, { filepath })`

  - Using `node.loc.url()` is an easy, quick way to get a clickable link to nodes causing problems.

  - To facilitate debugging in NodeJS, `class Loc` implements `nodejs.util.inspect.custom`  
    Hence in `console.log(node)`, loc is logged as `node.loc.url()`  
    e.g. `TraiDeclaration { ..., loc: "path/to/file.rs:51:18", ... }`

---

## Why `jinx-rust`? And why in Typescript?

- ### The case for Rust Tooling to split from rustc

  Tooling and compiler should only share the spec in common. Take the tripartite system in Javascript:

  - The spec TC39 ("legislative")
  - The core V8/JC/SM ("judicial")
  - The tooling Typescript ("executive")

  Through that lens, Rust is in a rather bleek shape:

  - No spec, the 1 implementation _is_ the law
  - The core is rustc
  - The tooling is done on top of core, sometimes even arbitrarily within core's algorithms

  Building Rust Tooling on top of rustc is a mistake akin to building Typescript's editor integration on top of v8's Turbofan.

- ### The case for Core and Tooling to use different Parsers and ASTs

  Both have drastically different needs, and attempting to serve both at the same time is grossly counterproductive.

  - Core needs a minimalistic, clear and no bs parser & AST that follows the syntax allowed in the spec by the dot and to the dot. Arbitrarily injecting lints in the core parser or bloating its data structure with CST is frankly out of scope and corrupts the entire language foundation.

  - Tooling needs a parser that is much more tolerant and flexible, it should be able to overlook or bend the spec syntax rules to best infer user's intent.

- ### The case for writing Rust Tooling in Typescript

  Tooling requires a language with a structure that is much more flexible, dynamic and hackable to accomodate for the wide range and open-endedness of the solutions it seeks to provide. Rust is simply not the right pick for the job, Rust is not the right pick for everything damnit.
