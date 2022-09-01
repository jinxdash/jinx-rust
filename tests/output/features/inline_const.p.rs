#![feature(inline_const_pat)]                                                                                                             /*
#![feature(inline_const_pat)]↲    <Program>
#![feature(inline_const_pat)]     Attribute{inner}
  [feature(inline_const_pat)]     Attribute.segments{dk: "[]"}
          (inline_const_pat)      DelimGroup                                                                                              */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <Program.ast{dk: "None"}>
fn•f()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	const {};                                                                                                                             /*
	const•{};    ExpressionStatement{semi}
	      {}     BlockExpression{const}                                                                                                   */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 10
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 65 (25% re-reads)
// Unnecessary 'skip_whitespace()' calls: 5
// source: "../../samples/features/inline_const.rs"