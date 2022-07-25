#![feature(inline_const_pat)]                                                                                                             /*
#![feature(inline_const_pat)]    Attribute
          (inline_const_pat)     DelimGroup                                                                                               */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>                                                                                                        */
	const {};                                                                                                                             /*
    const•{};    ExpressionStatement
          {}     BlockExpression                                                                                                          */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 0
// Parsed Nodes: 10
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 65 (25% re-reads)
// Unnecessary 'skip_whitespace()' calls: 5
// source: "../../samples/features/inline_const.rs"