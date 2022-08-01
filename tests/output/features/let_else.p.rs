#![feature(let_else)]                                                                                                                     /*
#![feature(let_else)]    Attribute
          (let_else)     DelimGroup                                                                                                       */

let a = 1 else { 2 };                                                                                                                     /*
let•a•=•1•else•{•2•};    LetVariableDeclaration
        1                Literal
               {•2•}     BlockExpression
                 2       ExpressionStatement, Literal                                                                                     */

// Discarded Nodes: 0
// Parsed Nodes: 12
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 57 (27% re-reads)
// Unnecessary 'skip_whitespace()' calls: 4
// source: "../../samples/features/let_else.rs"