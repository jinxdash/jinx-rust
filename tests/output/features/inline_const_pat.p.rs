#![feature(inline_const_pat)]                                                                                                             /*
#![feature(inline_const_pat)]    Attribute
          (inline_const_pat)     DelimGroup                                                                                               */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>                                                                                                        */
	match () {                                                                                                                            /*
    match•()•{↲    <ExpressionStatement>, <MatchExpression>
          ()       TupleLiteral                                                                                                           */
		y @ 0..const { 5 + 1 } => {}                                                                                                      /*
        y•@•0..const•{•5•+•1•}•=>•{}    MatchExpressionCase
        y•@•0..const•{•5•+•1•}          PatternVariableDeclaration
            0..const•{•5•+•1•}          RangePattern
            0                           Literal
               const•{•5•+•1•}          BlockExpression
                       5•+•1            ExpressionStatement, OperationExpression
                       5                Literal
                           1            Literal
                                  {}    BlockExpression                                                                                   */
		1 ..= const { two() } => {}                                                                                                       /*
        1•..=•const•{•two()•}•=>•{}    MatchExpressionCase
        1•..=•const•{•two()•}          RangePattern
        1                              Literal
              const•{•two()•}          BlockExpression
                      two()            ExpressionStatement, CallExpression
                                 {}    BlockExpression                                                                                    */
		const { one() } => {}                                                                                                             /*
        const•{•one()•}•=>•{}    MatchExpressionCase
        const•{•one()•}          BlockExpression
                one()            ExpressionStatement, CallExpression
                           {}    BlockExpression                                                                                          */
	}                                                                                                                                     /*
   ╚}    </ExpressionStatement>, </MatchExpression>                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 0
// Parsed Nodes: 36
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 171 (21% re-reads)
// Unnecessary 'skip_whitespace()' calls: 6
// source: "../../samples/features/inline_const_pat.rs"