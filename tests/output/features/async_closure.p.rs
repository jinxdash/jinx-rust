#![feature(async_closure)]                                                                                                                /*
#![feature(async_closure)]↲    <Program>
#![feature(async_closure)]     Attribute{inner}
  [feature(async_closure)]     Attribute.segments{dk: "[]"}
          (async_closure)      DelimGroup                                                                                                 */

const X = async |x| 0;                                                                                                                    /*
const•X•=•async•|x|•0;    Program.ast{dk: "None"}
const•X•=•async•|x|•0;    ConstVariableDeclaration
       ^                  MissingNode (ConstVariableDeclaration.typeAnnotation)
          async•|x|•0     ClosureFunctionExpression{async}
                |x|       ClosureFunctionExpression.parameters{dk: "||"}
                 x        ClosureFunctionParameterDeclaration
                    0     Literal{kind: Integer}
const•X•=•async•|x|•0;    </Program>                                                                                                      */
// Discarded Nodes: 0
// Parsed Nodes: 13
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 66 (32% re-reads)
// Unnecessary 'skip_whitespace()' calls: 6
// source: "../../samples/features/async_closure.rs"