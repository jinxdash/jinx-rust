#![feature(if_let_guard)]                                                                                                                 /*
#![feature(if_let_guard)]↲    <Program>
#![feature(if_let_guard)]     Attribute{inner}
  [feature(if_let_guard)]     Attribute.segments{dk: "[]"}
          (if_let_guard)      DelimGroup                                                                                                  */

fn main() {                                                                                                                               /*
fn•main()•{↲    <Program.ast{dk: "None"}>
fn•main()•{↲    <FunctionDeclaration>
       ()       FunctionDeclaration.parameters{dk: "()"}
          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                      */
	match e {                                                                                                                             /*
	match•e•{↲    <ExpressionStatement{!semi}>
	match•e•{↲    <MatchExpression>
	        {↲    <MatchExpression.cases{dk: "{}"}>                                                                                       */
        A(ref u) if let a = b && let c = d => {},                                                                                         /*
        A(ref•u)•if•let•a•=•b•&&•let•c•=•d•=>•{}    MatchExpressionCase
        A(ref•u)                                    TuplePattern
         (ref•u)                                    TuplePattern.items{dk: "()"}
          ref•u                                     PatternVariableDeclaration{ref, !mut}
                    let•a•=•b•&&•let•c•=•d          AndExpression{tk: "&&"}
                    let•a•=•b                       LetScrutinee
                                 let•c•=•d          LetScrutinee
                                              {}    BlockExpression                                                                       */
        A(ref u) if let x { a: b, c: d } = e && let f = g => {}                                                                           /*
        A(ref•u)•if•let•x•{•a:•b,•c:•d•}•=•e•&&•let•f•=•g•=>•{}    MatchExpressionCase
        A(ref•u)                                                   TuplePattern
         (ref•u)                                                   TuplePattern.items{dk: "()"}
          ref•u                                                    PatternVariableDeclaration{ref, !mut}
                    let•x•{•a:•b,•c:•d•}•=•e•&&•let•f•=•g          AndExpression{tk: "&&"}
                    let•x•{•a:•b,•c:•d•}•=•e                       LetScrutinee
                        x•{•a:•b,•c:•d•}                           StructPattern
                          {•a:•b,•c:•d•}                           StructPattern.properties{dk: "{}"}
                            a:•b                                   StructPatternPropertyDestructured
                                  c:•d                             StructPatternPropertyDestructured
                                                let•f•=•g          LetScrutinee
                                                             {}    BlockExpression                                                        */
		A(a) if let A(b) = a && let A(p) = b && p == z => {}                                                                              /*
		A(a)•if•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z•=>•{}    MatchExpressionCase
		A(a)                                                    TuplePattern
		 (a)                                                    TuplePattern.items{dk: "()"}
		        let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z          AndExpression{tk: "&&"}
		        let•A(b)•=•a•&&•let•A(p)•=•b                    AndExpression{tk: "&&"}
		        let•A(b)•=•a                                    LetScrutinee
		            A(b)                                        TuplePattern
		             (b)                                        TuplePattern.items{dk: "()"}
		                        let•A(p)•=•b                    LetScrutinee
		                            A(p)                        TuplePattern
		                             (p)                        TuplePattern.items{dk: "()"}
		                                        p•==•z          ComparisonExpression{tk: "=="}
		                                                  {}    BlockExpression                                                           */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
	match e {                                                                                                                             /*
	match•e•{↲    <ExpressionStatement{!semi}>
	match•e•{↲    <MatchExpression>
	        {↲    <MatchExpression.cases{dk: "{}"}>                                                                                       */
        A(a) if let A(b) = a && let A(p) = b && p == z => {}                                                                              /*
        A(a)•if•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z•=>•{}    MatchExpressionCase
        A(a)                                                    TuplePattern
         (a)                                                    TuplePattern.items{dk: "()"}
                let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z          AndExpression{tk: "&&"}
                let•A(b)•=•a•&&•let•A(p)•=•b                    AndExpression{tk: "&&"}
                let•A(b)•=•a                                    LetScrutinee
                    A(b)                                        TuplePattern
                     (b)                                        TuplePattern.items{dk: "()"}
                                let•A(p)•=•b                    LetScrutinee
                                    A(p)                        TuplePattern
                                     (p)                        TuplePattern.items{dk: "()"}
                                                p•==•z          ComparisonExpression{tk: "=="}
                                                          {}    BlockExpression                                                           */
        _ => {}                                                                                                                           /*
        _•=>•{}    MatchExpressionCase
        _          WildcardPattern
             {}    BlockExpression                                                                                                        */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 90
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 461 (44% re-reads)
// Unnecessary 'skip_whitespace()' calls: 28
// source: "../../samples/features/if_let_guard.rs"