#![feature(if_let_guard)]                                                                                                                 /*
#![feature(if_let_guard)]    Attribute
          (if_let_guard)     DelimGroup                                                                                                   */

fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>                                                                                                     */
	match e {                                                                                                                             /*
    match•e•{↲    <ExpressionStatement>, <MatchExpression>                                                                                */
        A(ref u) if let a = b && let c = d => {},                                                                                         /*
        A(ref•u)•if•let•a•=•b•&&•let•c•=•d•=>•{}     MatchExpressionCase
        A(ref•u)                                     TuplePattern
          ref•u                                      PatternVariableDeclaration
                    let•a•=•b•&&•let•c•=•d           AndExpression
                    let•a•=•b                        LetScrutinee
                                 let•c•=•d           LetScrutinee
                                              {}     BlockExpression                                                                      */
        A(ref u) if let x { a: b, c: d } = e && let f = g => {}                                                                           /*
        A(ref•u)•if•let•x•{•a:•b,•c:•d•}•=•e•&&•let•f•=•g•=>•{}    MatchExpressionCase
        A(ref•u)                                                   TuplePattern
          ref•u                                                    PatternVariableDeclaration
                    let•x•{•a:•b,•c:•d•}•=•e•&&•let•f•=•g          AndExpression
                    let•x•{•a:•b,•c:•d•}•=•e                       LetScrutinee
                        x•{•a:•b,•c:•d•}                           StructPattern
                            a:•b                                   StructPatternPropertyDestructured
                                  c:•d                             StructPatternPropertyDestructured
                                                let•f•=•g          LetScrutinee
                                                             {}    BlockExpression                                                        */
		A(a) if let A(b) = a && let A(p) = b && p == z => {}                                                                              /*
        A(a)•if•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z•=>•{}    MatchExpressionCase
        A(a)                                                    TuplePattern
                let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z          AndExpression
                let•A(b)•=•a•&&•let•A(p)•=•b                    AndExpression
                let•A(b)•=•a                                    LetScrutinee
                    A(b)                                        TuplePattern
                                let•A(p)•=•b                    LetScrutinee
                                    A(p)                        TuplePattern
                                                p•==•z          ComparisonExpression
                                                          {}    BlockExpression                                                           */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
	match e {                                                                                                                             /*
    match•e•{↲    <ExpressionStatement>, <MatchExpression>                                                                                */
        A(a) if let A(b) = a && let A(p) = b && p == z => {}                                                                              /*
        A(a)•if•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z•=>•{}    MatchExpressionCase
        A(a)                                                    TuplePattern
                let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z          AndExpression
                let•A(b)•=•a•&&•let•A(p)•=•b                    AndExpression
                let•A(b)•=•a                                    LetScrutinee
                    A(b)                                        TuplePattern
                                let•A(p)•=•b                    LetScrutinee
                                    A(p)                        TuplePattern
                                                p•==•z          ComparisonExpression
                                                          {}    BlockExpression                                                           */
        _ => {}                                                                                                                           /*
        _•=>•{}    MatchExpressionCase
        _          WildcardPattern
             {}    BlockExpression                                                                                                        */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 0
// Parsed Nodes: 90
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 461 (44% re-reads)
// Unnecessary 'skip_whitespace()' calls: 28
// source: "../../samples/features/if_let_guard.rs"