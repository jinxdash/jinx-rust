#![feature(async_closure)]                                                                                                                /*
#![feature(async_closure)]    Attribute
          (async_closure)     DelimGroup                                                                                                  */

const X = async |x| 0;                                                                                                                    /*
const•X•=•async•|x|•0;    ConstVariableDeclaration
       ^                  MissingNode
          async•|x|•0     ClosureFunctionExpression
                 x        ClosureFunctionParameterDeclaration
                    0     Literal                                                                                                         */

// Discarded Nodes: 0
// Parsed Nodes: 13
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 66 (32% re-reads)
// Unnecessary 'skip_whitespace()' calls: 6
// source: "../../samples/features/async_closure.rs"