#![feature(let_chains)]                                                                                                                   /*
#![feature(let_chains)]    Attribute
          (let_chains)     DelimGroup                                                                                                     */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>                                                                                                        */
	if h && let A(x) = e {}                                                                                                               /*
    if•h•&&•let•A(x)•=•e•{}    ExpressionStatement, IfBlockExpression
       h•&&•let•A(x)•=•e       AndExpression
            let•A(x)•=•e       LetScrutinee
                A(x)           TuplePattern                                                                                               */
	if a && let c = d && 1 == 1 {}                                                                                                        /*
    if•a•&&•let•c•=•d•&&•1•==•1•{}    ExpressionStatement, IfBlockExpression
       a•&&•let•c•=•d•&&•1•==•1       AndExpression
       a•&&•let•c•=•d                 AndExpression
            let•c•=•d                 LetScrutinee
                         1•==•1       ComparisonExpression
                         1            Literal
                              1       Literal                                                                                             */
	if (a && let c = d && 1 == 1) {}                                                                                                      /*
    if•(a•&&•let•c•=•d•&&•1•==•1)•{}    ExpressionStatement, IfBlockExpression
        a•&&•let•c•=•d•&&•1•==•1        AndExpression
        a•&&•let•c•=•d                  AndExpression
             let•c•=•d                  LetScrutinee
                          1•==•1        ComparisonExpression
                          1             Literal
                               1        Literal                                                                                           */
	if true && let x = 1 { let _ = x; }                                                                                                   /*
    if•true•&&•let•x•=•1•{•let•_•=•x;•}    ExpressionStatement, IfBlockExpression
       true•&&•let•x•=•1                   AndExpression
       true                                Literal
               let•x•=•1                   LetScrutinee
                       1                   Literal
                           let•_•=•x;      LetVariableDeclaration
                               _           WildcardPattern                                                                                */
	if (let a = b && let c = d && 1 == 1) {}                                                                                              /*
    if•(let•a•=•b•&&•let•c•=•d•&&•1•==•1)•{}    ExpressionStatement, IfBlockExpression
        let•a•=•b•&&•let•c•=•d•&&•1•==•1        AndExpression
        let•a•=•b•&&•let•c•=•d                  AndExpression
        let•a•=•b                               LetScrutinee
                     let•c•=•d                  LetScrutinee
                                  1•==•1        ComparisonExpression
                                  1             Literal
                                       1        Literal                                                                                   */
	if let A { .. } = d(7, B) && let C { .. } = d(8, D) {}                                                                                /*
    if•let•A•{•..•}•=•d(7,•B)•&&•let•C•{•..•}•=•d(8,•D)•{}    ExpressionStatement, IfBlockExpression
       let•A•{•..•}•=•d(7,•B)•&&•let•C•{•..•}•=•d(8,•D)       AndExpression
       let•A•{•..•}•=•d(7,•B)                                 LetScrutinee
           A•{•..•}                                           StructPattern
               ..                                             RestPattern
                      d(7,•B)                                 CallExpression
                        7                                     Literal
                                 let•C•{•..•}•=•d(8,•D)       LetScrutinee
                                     C•{•..•}                 StructPattern
                                         ..                   RestPattern
                                                d(8,•D)       CallExpression
                                                  8           Literal                                                                     */
	if let a = &e && let A(ref b) = a && let B = b.c {}                                                                                   /*
    if•let•a•=•&e•&&•let•A(ref•b)•=•a•&&•let•B•=•b.c•{}    ExpressionStatement, IfBlockExpression
       let•a•=•&e•&&•let•A(ref•b)•=•a•&&•let•B•=•b.c       AndExpression
       let•a•=•&e•&&•let•A(ref•b)•=•a                      AndExpression
       let•a•=•&e                                          LetScrutinee
               &e                                          ReferenceExpression
                     let•A(ref•b)•=•a                      LetScrutinee
                         A(ref•b)                          TuplePattern
                           ref•b                           PatternVariableDeclaration
                                         let•B•=•b.c       LetScrutinee
                                                 b.c       MemberExpression                                                               */
	if let a = b && let c = d && 1 == 1 {}                                                                                                /*
    if•let•a•=•b•&&•let•c•=•d•&&•1•==•1•{}    ExpressionStatement, IfBlockExpression
       let•a•=•b•&&•let•c•=•d•&&•1•==•1       AndExpression
       let•a•=•b•&&•let•c•=•d                 AndExpression
       let•a•=•b                              LetScrutinee
                    let•c•=•d                 LetScrutinee
                                 1•==•1       ComparisonExpression
                                 1            Literal
                                      1       Literal                                                                                     */
	if let A(_) = d(2, B(2)).c && let D { .. } = d(3, E) {} else {}                                                                       /*
    if•let•A(_)•=•d(2,•B(2)).c•&&•let•D•{•..•}•=•d(3,•E)•{}•else•{}    ExpressionStatement, IfBlockExpression
       let•A(_)•=•d(2,•B(2)).c•&&•let•D•{•..•}•=•d(3,•E)               AndExpression
       let•A(_)•=•d(2,•B(2)).c                                         LetScrutinee
           A(_)                                                        TuplePattern
             _                                                         WildcardPattern
                  d(2,•B(2)).c                                         MemberExpression
                  d(2,•B(2))                                           CallExpression
                    2                                                  Literal
                       B(2)                                            CallExpression
                         2                                             Literal
                                  let•D•{•..•}•=•d(3,•E)               LetScrutinee
                                      D•{•..•}                         StructPattern
                                          ..                           RestPattern
                                                 d(3,•E)               CallExpression
                                                   3                   Literal
                                                                 {}    BlockExpression                                                    */
	if let A(a) = e && let A(b) = a && let A(p) = b && p == z {} else {}                                                                  /*
    if•let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z•{}•else•{}    ExpressionStatement, IfBlockExpression
       let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z               AndExpression
       let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b                         AndExpression
       let•A(a)•=•e•&&•let•A(b)•=•a                                         AndExpression
       let•A(a)•=•e                                                         LetScrutinee
           A(a)                                                             TuplePattern
                       let•A(b)•=•a                                         LetScrutinee
                           A(b)                                             TuplePattern
                                       let•A(p)•=•b                         LetScrutinee
                                           A(p)                             TuplePattern
                                                       p•==•z               ComparisonExpression
                                                                      {}    BlockExpression                                               */
	if let A(v) = x && v.q() {}                                                                                                           /*
    if•let•A(v)•=•x•&&•v.q()•{}    ExpressionStatement, IfBlockExpression
       let•A(v)•=•x•&&•v.q()       AndExpression
       let•A(v)•=•x                LetScrutinee
           A(v)                    TuplePattern
                       v.q()       CallExpression                                                                                         */
    if let A(_) = Q(0) && let E(_) = R(1) {} else if let G(1) = F(2) {}                                                                   /*
    if•let•A(_)•=•Q(0)•&&•let•E(_)•=•R(1)•{}•else•if•let•G(1)•=•F(2)•{}    ExpressionStatement, IfBlockExpression
       let•A(_)•=•Q(0)•&&•let•E(_)•=•R(1)                                  AndExpression
       let•A(_)•=•Q(0)                                                     LetScrutinee
           A(_)                                                            TuplePattern
             _                                                             WildcardPattern
                  Q(0)                                                     CallExpression
                    0                                                      Literal
                          let•E(_)•=•R(1)                                  LetScrutinee
                              E(_)                                         TuplePattern
                                _                                          WildcardPattern
                                     R(1)                                  CallExpression
                                       1                                   Literal
                                                  if•let•G(1)•=•F(2)•{}    IfBlockExpression
                                                     let•G(1)•=•F(2)       LetScrutinee
                                                         G(1)              TuplePattern
                                                           1               Literal
                                                                F(2)       CallExpression
                                                                  2        Literal                                                        */
    if let A(ref a) = e && let b = a && let _p = b {}                                                                                     /*
    if•let•A(ref•a)•=•e•&&•let•b•=•a•&&•let•_p•=•b•{}    ExpressionStatement, IfBlockExpression
       let•A(ref•a)•=•e•&&•let•b•=•a•&&•let•_p•=•b       AndExpression
       let•A(ref•a)•=•e•&&•let•b•=•a                     AndExpression
       let•A(ref•a)•=•e                                  LetScrutinee
           A(ref•a)                                      TuplePattern
             ref•a                                       PatternVariableDeclaration
                           let•b•=•a                     LetScrutinee
                                        let•_p•=•b       LetScrutinee                                                                     */
    if let A(ref a) = e && let R { c: d, y: _ } = a && let B = d {}                                                                       /*
    if•let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a•&&•let•B•=•d•{}    ExpressionStatement, IfBlockExpression
       let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a•&&•let•B•=•d       AndExpression
       let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a                    AndExpression
       let•A(ref•a)•=•e                                                LetScrutinee
           A(ref•a)                                                    TuplePattern
             ref•a                                                     PatternVariableDeclaration
                           let•R•{•c:•d,•y:•_•}•=•a                    LetScrutinee
                               R•{•c:•d,•y:•_•}                        StructPattern
                                   c:•d                                StructPatternPropertyDestructured
                                         y:•_                          StructPatternPropertyDestructured
                                            _                          WildcardPattern
                                                       let•B•=•d       LetScrutinee                                                       */
	while let a = &e && let A(ref b) = a && let B = b.c {}                                                                                /*
    while•let•a•=•&e•&&•let•A(ref•b)•=•a•&&•let•B•=•b.c•{}    ExpressionStatement, WhileBlockExpression
          let•a•=•&e•&&•let•A(ref•b)•=•a•&&•let•B•=•b.c       AndExpression
          let•a•=•&e•&&•let•A(ref•b)•=•a                      AndExpression
          let•a•=•&e                                          LetScrutinee
                  &e                                          ReferenceExpression
                        let•A(ref•b)•=•a                      LetScrutinee
                            A(ref•b)                          TuplePattern
                              ref•b                           PatternVariableDeclaration
                                            let•B•=•b.c       LetScrutinee
                                                    b.c       MemberExpression                                                            */
	while let A(a) = e && let A(b) = a && let A(p) = b && p == z {}                                                                       /*
    while•let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z•{}    ExpressionStatement, WhileBlockExpression
          let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z       AndExpression
          let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b                 AndExpression
          let•A(a)•=•e•&&•let•A(b)•=•a                                 AndExpression
          let•A(a)•=•e                                                 LetScrutinee
              A(a)                                                     TuplePattern
                          let•A(b)•=•a                                 LetScrutinee
                              A(b)                                     TuplePattern
                                          let•A(p)•=•b                 LetScrutinee
                                              A(p)                     TuplePattern
                                                          p•==•z       ComparisonExpression                                               */
    while let A(ref a) = e && let b = a && let _p = b {}                                                                                  /*
    while•let•A(ref•a)•=•e•&&•let•b•=•a•&&•let•_p•=•b•{}    ExpressionStatement, WhileBlockExpression
          let•A(ref•a)•=•e•&&•let•b•=•a•&&•let•_p•=•b       AndExpression
          let•A(ref•a)•=•e•&&•let•b•=•a                     AndExpression
          let•A(ref•a)•=•e                                  LetScrutinee
              A(ref•a)                                      TuplePattern
                ref•a                                       PatternVariableDeclaration
                              let•b•=•a                     LetScrutinee
                                           let•_p•=•b       LetScrutinee                                                                  */
    while let A(ref a) = e && let R { c: d, y: _ } = a && let B = d {}                                                                    /*
    while•let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a•&&•let•B•=•d•{}    ExpressionStatement, WhileBlockExpression
          let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a•&&•let•B•=•d       AndExpression
          let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a                    AndExpression
          let•A(ref•a)•=•e                                                LetScrutinee
              A(ref•a)                                                    TuplePattern
                ref•a                                                     PatternVariableDeclaration
                              let•R•{•c:•d,•y:•_•}•=•a                    LetScrutinee
                                  R•{•c:•d,•y:•_•}                        StructPattern
                                      c:•d                                StructPatternPropertyDestructured
                                            y:•_                          StructPatternPropertyDestructured
                                               _                          WildcardPattern
                                                          let•B•=•d       LetScrutinee                                                    */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 2
// Parsed Nodes: 317
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 1405 (46% re-reads)
// Unnecessary 'skip_whitespace()' calls: 75
// source: "../../samples/features/let_chains.rs"