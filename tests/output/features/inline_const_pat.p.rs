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
	match () {                                                                                                                            /*
	match•()•{↲    <ExpressionStatement{!semi}>
	match•()•{↲    <MatchExpression>
	      ()       TupleLiteral
	         {↲    <MatchExpression.cases{dk: "{}"}>                                                                                      */
		y @ 0..const { 5 + 1 } => {}                                                                                                      /*
		y•@•0..const•{•5•+•1•}•=>•{}    MatchExpressionCase
		y•@•0..const•{•5•+•1•}          PatternVariableDeclaration{!ref, !mut}
		    0..const•{•5•+•1•}          RangePattern{!last}
		    0                           Literal{kind: Integer}
		       const•{•5•+•1•}          BlockExpression{const}
		             {•5•+•1•}          BlockExpression.body{dk: "{}"}
		               5•+•1            ExpressionStatement{!semi}, OperationExpression{tk: "+"}
		               5                Literal{kind: Integer}
		                   1            Literal{kind: Integer}
		                          {}    BlockExpression                                                                                   */
		1 ..= const { two() } => {}                                                                                                       /*
		1•..=•const•{•two()•}•=>•{}    MatchExpressionCase
		1•..=•const•{•two()•}          RangePattern{last}
		1                              Literal{kind: Integer}
		      const•{•two()•}          BlockExpression{const}
		            {•two()•}          BlockExpression.body{dk: "{}"}
		              two()            ExpressionStatement{!semi}, CallExpression
		                 ()            CallExpression.arguments{dk: "()"}
		                         {}    BlockExpression                                                                                    */
		const { one() } => {}                                                                                                             /*
		const•{•one()•}•=>•{}    MatchExpressionCase
		const•{•one()•}          BlockExpression{const}
		      {•one()•}          BlockExpression.body{dk: "{}"}
		        one()            ExpressionStatement{!semi}, CallExpression
		           ()            CallExpression.arguments{dk: "()"}
		                   {}    BlockExpression                                                                                          */
	}                                                                                                                                     /*
   ╚}    </MatchExpression.cases>
   ╚}    </MatchExpression>
   ╚}    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 36
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 171 (21% re-reads)
// Unnecessary 'skip_whitespace()' calls: 6
// source: "../../samples/features/inline_const_pat.rs"