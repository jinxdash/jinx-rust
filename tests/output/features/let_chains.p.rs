#![feature(let_chains)]                                                                                                                   /*
#![feature(let_chains)]↲    <Program>
#![feature(let_chains)]     Attribute{inner}
  [feature(let_chains)]     Attribute.segments{dk: "[]"}
          (let_chains)      DelimGroup                                                                                                    */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <Program.ast{dk: "None"}>
fn•f()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	if h && let A(x) = e {}                                                                                                               /*
	if•h•&&•let•A(x)•=•e•{}    ExpressionStatement{!semi}, IfBlockExpression
	   h•&&•let•A(x)•=•e       AndExpression{tk: "&&"}
	        let•A(x)•=•e       LetScrutinee
	            A(x)           TuplePattern
	             (x)           TuplePattern.items{dk: "()"}
	                     {}    IfBlockExpression.body{dk: "{}"}                                                                           */
	if a && let c = d && 1 == 1 {}                                                                                                        /*
	if•a•&&•let•c•=•d•&&•1•==•1•{}    ExpressionStatement{!semi}, IfBlockExpression
	   a•&&•let•c•=•d•&&•1•==•1       AndExpression{tk: "&&"}
	   a•&&•let•c•=•d                 AndExpression{tk: "&&"}
	        let•c•=•d                 LetScrutinee
	                     1•==•1       ComparisonExpression{tk: "=="}
	                     1            Literal{kind: Integer}
	                          1       Literal{kind: Integer}
	                            {}    IfBlockExpression.body{dk: "{}"}                                                                    */
	if (a && let c = d && 1 == 1) {}                                                                                                      /*
	if•(a•&&•let•c•=•d•&&•1•==•1)•{}    ExpressionStatement{!semi}, IfBlockExpression
	    a•&&•let•c•=•d•&&•1•==•1        AndExpression{tk: "&&"}
	    a•&&•let•c•=•d                  AndExpression{tk: "&&"}
	         let•c•=•d                  LetScrutinee
	                      1•==•1        ComparisonExpression{tk: "=="}
	                      1             Literal{kind: Integer}
	                           1        Literal{kind: Integer}
	                              {}    IfBlockExpression.body{dk: "{}"}                                                                  */
	if true && let x = 1 { let _ = x; }                                                                                                   /*
	if•true•&&•let•x•=•1•{•let•_•=•x;•}    ExpressionStatement{!semi}, IfBlockExpression
	   true•&&•let•x•=•1                   AndExpression{tk: "&&"}
	   true                                Literal{kind: True}
	           let•x•=•1                   LetScrutinee
	                   1                   Literal{kind: Integer}
	                     {•let•_•=•x;•}    IfBlockExpression.body{dk: "{}"}
	                       let•_•=•x;      LetVariableDeclaration
	                           _           WildcardPattern                                                                                */
	if (let a = b && let c = d && 1 == 1) {}                                                                                              /*
	if•(let•a•=•b•&&•let•c•=•d•&&•1•==•1)•{}    ExpressionStatement{!semi}, IfBlockExpression
	    let•a•=•b•&&•let•c•=•d•&&•1•==•1        AndExpression{tk: "&&"}
	    let•a•=•b•&&•let•c•=•d                  AndExpression{tk: "&&"}
	    let•a•=•b                               LetScrutinee
	                 let•c•=•d                  LetScrutinee
	                              1•==•1        ComparisonExpression{tk: "=="}
	                              1             Literal{kind: Integer}
	                                   1        Literal{kind: Integer}
	                                      {}    IfBlockExpression.body{dk: "{}"}                                                          */
	if let A { .. } = d(7, B) && let C { .. } = d(8, D) {}                                                                                /*
	if•let•A•{•..•}•=•d(7,•B)•&&•let•C•{•..•}•=•d(8,•D)•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•A•{•..•}•=•d(7,•B)•&&•let•C•{•..•}•=•d(8,•D)       AndExpression{tk: "&&"}
	   let•A•{•..•}•=•d(7,•B)                                 LetScrutinee
	       A•{•..•}                                           StructPattern
	         {•..•}                                           StructPattern.properties{dk: "{}"}
	           ..                                             RestPattern
	                  d(7,•B)                                 CallExpression
	                   (7,•B)                                 CallExpression.arguments{dk: "()"}
	                    7                                     Literal{kind: Integer}
	                             let•C•{•..•}•=•d(8,•D)       LetScrutinee
	                                 C•{•..•}                 StructPattern
	                                   {•..•}                 StructPattern.properties{dk: "{}"}
	                                     ..                   RestPattern
	                                            d(8,•D)       CallExpression
	                                             (8,•D)       CallExpression.arguments{dk: "()"}
	                                              8           Literal{kind: Integer}
	                                                    {}    IfBlockExpression.body{dk: "{}"}                                            */
	if let a = &e && let A(ref b) = a && let B = b.c {}                                                                                   /*
	if•let•a•=•&e•&&•let•A(ref•b)•=•a•&&•let•B•=•b.c•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•a•=•&e•&&•let•A(ref•b)•=•a•&&•let•B•=•b.c       AndExpression{tk: "&&"}
	   let•a•=•&e•&&•let•A(ref•b)•=•a                      AndExpression{tk: "&&"}
	   let•a•=•&e                                          LetScrutinee
	           &e                                          ReferenceExpression{!mut}
	                 let•A(ref•b)•=•a                      LetScrutinee
	                     A(ref•b)                          TuplePattern
	                      (ref•b)                          TuplePattern.items{dk: "()"}
	                       ref•b                           PatternVariableDeclaration{ref, !mut}
	                                     let•B•=•b.c       LetScrutinee
	                                             b.c       MemberExpression{!computed}
	                                                 {}    IfBlockExpression.body{dk: "{}"}                                               */
	if let a = b && let c = d && 1 == 1 {}                                                                                                /*
	if•let•a•=•b•&&•let•c•=•d•&&•1•==•1•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•a•=•b•&&•let•c•=•d•&&•1•==•1       AndExpression{tk: "&&"}
	   let•a•=•b•&&•let•c•=•d                 AndExpression{tk: "&&"}
	   let•a•=•b                              LetScrutinee
	                let•c•=•d                 LetScrutinee
	                             1•==•1       ComparisonExpression{tk: "=="}
	                             1            Literal{kind: Integer}
	                                  1       Literal{kind: Integer}
	                                    {}    IfBlockExpression.body{dk: "{}"}                                                            */
	if let A(_) = d(2, B(2)).c && let D { .. } = d(3, E) {} else {}                                                                       /*
	if•let•A(_)•=•d(2,•B(2)).c•&&•let•D•{•..•}•=•d(3,•E)•{}•else•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•A(_)•=•d(2,•B(2)).c•&&•let•D•{•..•}•=•d(3,•E)               AndExpression{tk: "&&"}
	   let•A(_)•=•d(2,•B(2)).c                                         LetScrutinee
	       A(_)                                                        TuplePattern
	        (_)                                                        TuplePattern.items{dk: "()"}
	         _                                                         WildcardPattern
	              d(2,•B(2)).c                                         MemberExpression{!computed}
	              d(2,•B(2))                                           CallExpression
	               (2,•B(2))                                           CallExpression.arguments{dk: "()"}
	                2                                                  Literal{kind: Integer}
	                   B(2)                                            CallExpression
	                    (2)                                            CallExpression.arguments{dk: "()"}
	                     2                                             Literal{kind: Integer}
	                              let•D•{•..•}•=•d(3,•E)               LetScrutinee
	                                  D•{•..•}                         StructPattern
	                                    {•..•}                         StructPattern.properties{dk: "{}"}
	                                      ..                           RestPattern
	                                             d(3,•E)               CallExpression
	                                              (3,•E)               CallExpression.arguments{dk: "()"}
	                                               3                   Literal{kind: Integer}
	                                                     {}            IfBlockExpression.body{dk: "{}"}
	                                                             {}    BlockExpression                                                    */
	if let A(a) = e && let A(b) = a && let A(p) = b && p == z {} else {}                                                                  /*
	if•let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z•{}•else•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z               AndExpression{tk: "&&"}
	   let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b                         AndExpression{tk: "&&"}
	   let•A(a)•=•e•&&•let•A(b)•=•a                                         AndExpression{tk: "&&"}
	   let•A(a)•=•e                                                         LetScrutinee
	       A(a)                                                             TuplePattern
	        (a)                                                             TuplePattern.items{dk: "()"}
	                   let•A(b)•=•a                                         LetScrutinee
	                       A(b)                                             TuplePattern
	                        (b)                                             TuplePattern.items{dk: "()"}
	                                   let•A(p)•=•b                         LetScrutinee
	                                       A(p)                             TuplePattern
	                                        (p)                             TuplePattern.items{dk: "()"}
	                                                   p•==•z               ComparisonExpression{tk: "=="}
	                                                          {}            IfBlockExpression.body{dk: "{}"}
	                                                                  {}    BlockExpression                                               */
	if let A(v) = x && v.q() {}                                                                                                           /*
	if•let•A(v)•=•x•&&•v.q()•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•A(v)•=•x•&&•v.q()       AndExpression{tk: "&&"}
	   let•A(v)•=•x                LetScrutinee
	       A(v)                    TuplePattern
	        (v)                    TuplePattern.items{dk: "()"}
	                   v.q()       CallExpression
	                      ()       CallExpression.arguments{dk: "()"}
	                         {}    IfBlockExpression.body{dk: "{}"}                                                                       */
    if let A(_) = Q(0) && let E(_) = R(1) {} else if let G(1) = F(2) {}                                                                   /*
    if•let•A(_)•=•Q(0)•&&•let•E(_)•=•R(1)•{}•else•if•let•G(1)•=•F(2)•{}    ExpressionStatement{!semi}, IfBlockExpression
       let•A(_)•=•Q(0)•&&•let•E(_)•=•R(1)                                  AndExpression{tk: "&&"}
       let•A(_)•=•Q(0)                                                     LetScrutinee
           A(_)                                                            TuplePattern
            (_)                                                            TuplePattern.items{dk: "()"}
             _                                                             WildcardPattern
                  Q(0)                                                     CallExpression
                   (0)                                                     CallExpression.arguments{dk: "()"}
                    0                                                      Literal{kind: Integer}
                          let•E(_)•=•R(1)                                  LetScrutinee
                              E(_)                                         TuplePattern
                               (_)                                         TuplePattern.items{dk: "()"}
                                _                                          WildcardPattern
                                     R(1)                                  CallExpression
                                      (1)                                  CallExpression.arguments{dk: "()"}
                                       1                                   Literal{kind: Integer}
                                          {}                               IfBlockExpression.body{dk: "{}"}
                                                  if•let•G(1)•=•F(2)•{}    IfBlockExpression
                                                     let•G(1)•=•F(2)       LetScrutinee
                                                         G(1)              TuplePattern
                                                          (1)              TuplePattern.items{dk: "()"}
                                                           1               Literal{kind: Integer}
                                                                F(2)       CallExpression
                                                                 (2)       CallExpression.arguments{dk: "()"}
                                                                  2        Literal{kind: Integer}
                                                                     {}    IfBlockExpression.body{dk: "{}"}                               */
    if let A(ref a) = e && let b = a && let _p = b {}                                                                                     /*
    if•let•A(ref•a)•=•e•&&•let•b•=•a•&&•let•_p•=•b•{}    ExpressionStatement{!semi}, IfBlockExpression
       let•A(ref•a)•=•e•&&•let•b•=•a•&&•let•_p•=•b       AndExpression{tk: "&&"}
       let•A(ref•a)•=•e•&&•let•b•=•a                     AndExpression{tk: "&&"}
       let•A(ref•a)•=•e                                  LetScrutinee
           A(ref•a)                                      TuplePattern
            (ref•a)                                      TuplePattern.items{dk: "()"}
             ref•a                                       PatternVariableDeclaration{ref, !mut}
                           let•b•=•a                     LetScrutinee
                                        let•_p•=•b       LetScrutinee
                                                   {}    IfBlockExpression.body{dk: "{}"}                                                 */
    if let A(ref a) = e && let R { c: d, y: _ } = a && let B = d {}                                                                       /*
    if•let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a•&&•let•B•=•d•{}    ExpressionStatement{!semi}, IfBlockExpression
       let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a•&&•let•B•=•d       AndExpression{tk: "&&"}
       let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a                    AndExpression{tk: "&&"}
       let•A(ref•a)•=•e                                                LetScrutinee
           A(ref•a)                                                    TuplePattern
            (ref•a)                                                    TuplePattern.items{dk: "()"}
             ref•a                                                     PatternVariableDeclaration{ref, !mut}
                           let•R•{•c:•d,•y:•_•}•=•a                    LetScrutinee
                               R•{•c:•d,•y:•_•}                        StructPattern
                                 {•c:•d,•y:•_•}                        StructPattern.properties{dk: "{}"}
                                   c:•d                                StructPatternPropertyDestructured
                                         y:•_                          StructPatternPropertyDestructured
                                            _                          WildcardPattern
                                                       let•B•=•d       LetScrutinee
                                                                 {}    IfBlockExpression.body{dk: "{}"}                                   */
	while let a = &e && let A(ref b) = a && let B = b.c {}                                                                                /*
	while•let•a•=•&e•&&•let•A(ref•b)•=•a•&&•let•B•=•b.c•{}    ExpressionStatement{!semi}, WhileBlockExpression
	      let•a•=•&e•&&•let•A(ref•b)•=•a•&&•let•B•=•b.c       AndExpression{tk: "&&"}
	      let•a•=•&e•&&•let•A(ref•b)•=•a                      AndExpression{tk: "&&"}
	      let•a•=•&e                                          LetScrutinee
	              &e                                          ReferenceExpression{!mut}
	                    let•A(ref•b)•=•a                      LetScrutinee
	                        A(ref•b)                          TuplePattern
	                         (ref•b)                          TuplePattern.items{dk: "()"}
	                          ref•b                           PatternVariableDeclaration{ref, !mut}
	                                        let•B•=•b.c       LetScrutinee
	                                                b.c       MemberExpression{!computed}
	                                                    {}    WhileBlockExpression.body{dk: "{}"}                                         */
	while let A(a) = e && let A(b) = a && let A(p) = b && p == z {}                                                                       /*
	while•let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z•{}    ExpressionStatement{!semi}, WhileBlockExpression
	      let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b•&&•p•==•z       AndExpression{tk: "&&"}
	      let•A(a)•=•e•&&•let•A(b)•=•a•&&•let•A(p)•=•b                 AndExpression{tk: "&&"}
	      let•A(a)•=•e•&&•let•A(b)•=•a                                 AndExpression{tk: "&&"}
	      let•A(a)•=•e                                                 LetScrutinee
	          A(a)                                                     TuplePattern
	           (a)                                                     TuplePattern.items{dk: "()"}
	                      let•A(b)•=•a                                 LetScrutinee
	                          A(b)                                     TuplePattern
	                           (b)                                     TuplePattern.items{dk: "()"}
	                                      let•A(p)•=•b                 LetScrutinee
	                                          A(p)                     TuplePattern
	                                           (p)                     TuplePattern.items{dk: "()"}
	                                                      p•==•z       ComparisonExpression{tk: "=="}
	                                                             {}    WhileBlockExpression.body{dk: "{}"}                                */
    while let A(ref a) = e && let b = a && let _p = b {}                                                                                  /*
    while•let•A(ref•a)•=•e•&&•let•b•=•a•&&•let•_p•=•b•{}    ExpressionStatement{!semi}, WhileBlockExpression
          let•A(ref•a)•=•e•&&•let•b•=•a•&&•let•_p•=•b       AndExpression{tk: "&&"}
          let•A(ref•a)•=•e•&&•let•b•=•a                     AndExpression{tk: "&&"}
          let•A(ref•a)•=•e                                  LetScrutinee
              A(ref•a)                                      TuplePattern
               (ref•a)                                      TuplePattern.items{dk: "()"}
                ref•a                                       PatternVariableDeclaration{ref, !mut}
                              let•b•=•a                     LetScrutinee
                                           let•_p•=•b       LetScrutinee
                                                      {}    WhileBlockExpression.body{dk: "{}"}                                           */
    while let A(ref a) = e && let R { c: d, y: _ } = a && let B = d {}                                                                    /*
    while•let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a•&&•let•B•=•d•{}    ExpressionStatement{!semi}, WhileBlockExpression
          let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a•&&•let•B•=•d       AndExpression{tk: "&&"}
          let•A(ref•a)•=•e•&&•let•R•{•c:•d,•y:•_•}•=•a                    AndExpression{tk: "&&"}
          let•A(ref•a)•=•e                                                LetScrutinee
              A(ref•a)                                                    TuplePattern
               (ref•a)                                                    TuplePattern.items{dk: "()"}
                ref•a                                                     PatternVariableDeclaration{ref, !mut}
                              let•R•{•c:•d,•y:•_•}•=•a                    LetScrutinee
                                  R•{•c:•d,•y:•_•}                        StructPattern
                                    {•c:•d,•y:•_•}                        StructPattern.properties{dk: "{}"}
                                      c:•d                                StructPatternPropertyDestructured
                                            y:•_                          StructPatternPropertyDestructured
                                               _                          WildcardPattern
                                                          let•B•=•d       LetScrutinee
                                                                    {}    WhileBlockExpression.body{dk: "{}"}                             */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 2
// Parsed Nodes: 317
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 1405 (46% re-reads)
// Unnecessary 'skip_whitespace()' calls: 75
// source: "../../samples/features/let_chains.rs"