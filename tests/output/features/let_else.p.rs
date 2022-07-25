#![feature(let_else)]                                                                                                                     /*
#![feature(let_else)]    Attribute
          (let_else)     DelimGroup                                                                                                       */

let () = if true {} else { return }                                                                                                       /*
let•()•=•if•true•{}•else•{•return•}    LetVariableDeclaration
    ()                                 TuplePattern
         if•true•{}•else•{•return•}    IfBlockExpression
            true                       Literal
                         {•return•}    BlockExpression
                           return      ExpressionStatement, ReturnExpression                                                              */

// Discarded Nodes: 0
// Parsed Nodes: 13
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 75 (29% re-reads)
// Unnecessary 'skip_whitespace()' calls: 3
// source: "../../samples/features/let_else.rs"