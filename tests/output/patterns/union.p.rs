
fn fw1(H(Ok(mut x) | &Err(mut x)): H<R<'_>>) {}                                                                                           /*
fn•fw1(H(Ok(mut•x)•|•&Err(mut•x)):•H<R<'_>>)•{}↲    <Program>
fn•fw1(H(Ok(mut•x)•|•&Err(mut•x)):•H<R<'_>>)•{}↲    <Program.ast{dk: "None"}>
fn•fw1(H(Ok(mut•x)•|•&Err(mut•x)):•H<R<'_>>)•{}     FunctionDeclaration
      (H(Ok(mut•x)•|•&Err(mut•x)):•H<R<'_>>)        FunctionDeclaration.parameters{dk: "()"}
       H(Ok(mut•x)•|•&Err(mut•x)):•H<R<'_>>         FunctionParameterDeclaration
       H(Ok(mut•x)•|•&Err(mut•x))                   TuplePattern
        (Ok(mut•x)•|•&Err(mut•x))                   TuplePattern.items{dk: "()"}
         Ok(mut•x)•|•&Err(mut•x)                    UnionPattern
         Ok(mut•x)                                  TuplePattern
           (mut•x)                                  TuplePattern.items{dk: "()"}
            mut•x                                   PatternVariableDeclaration{!ref, mut}
                     &Err(mut•x)                    ReferencePattern{!mut}
                      Err(mut•x)                    TuplePattern
                         (mut•x)                    TuplePattern.items{dk: "()"}
                          mut•x                     PatternVariableDeclaration{!ref, mut}
                                   H<R<'_>>         TypeCall
                                    <R<'_>>         TypeCall.typeArguments{dk: "<>"}
                                     R<'_>          TypeCall
                                      <'_>          TypeCall.typeArguments{dk: "<>"}
                                       '_           LtElided
                                             {}     FunctionDeclaration.body{dk: "{}"}                                                    */
fn f1((Ok(mut x) | &Err(mut x)): R<'_>) {}                                                                                                /*
fn•f1((Ok(mut•x)•|•&Err(mut•x)):•R<'_>)•{}    FunctionDeclaration
     ((Ok(mut•x)•|•&Err(mut•x)):•R<'_>)       FunctionDeclaration.parameters{dk: "()"}
      (Ok(mut•x)•|•&Err(mut•x)):•R<'_>        FunctionParameterDeclaration
       Ok(mut•x)•|•&Err(mut•x)                UnionPattern
       Ok(mut•x)                              TuplePattern
         (mut•x)                              TuplePattern.items{dk: "()"}
          mut•x                               PatternVariableDeclaration{!ref, mut}
                   &Err(mut•x)                ReferencePattern{!mut}
                    Err(mut•x)                TuplePattern
                       (mut•x)                TuplePattern.items{dk: "()"}
                        mut•x                 PatternVariableDeclaration{!ref, mut}
                                 R<'_>        TypeCall
                                  <'_>        TypeCall.typeArguments{dk: "<>"}
                                   '_         LtElided
                                        {}    FunctionDeclaration.body{dk: "{}"}                                                          */
fn fw2(H(&(Ok(x) | Err(x))): H<R<'_>>) {}                                                                                                 /*
fn•fw2(H(&(Ok(x)•|•Err(x))):•H<R<'_>>)•{}    FunctionDeclaration
      (H(&(Ok(x)•|•Err(x))):•H<R<'_>>)       FunctionDeclaration.parameters{dk: "()"}
       H(&(Ok(x)•|•Err(x))):•H<R<'_>>        FunctionParameterDeclaration
       H(&(Ok(x)•|•Err(x)))                  TuplePattern
        (&(Ok(x)•|•Err(x)))                  TuplePattern.items{dk: "()"}
         &(Ok(x)•|•Err(x))                   ReferencePattern{!mut}
           Ok(x)•|•Err(x)                    UnionPattern
           Ok(x)                             TuplePattern
             (x)                             TuplePattern.items{dk: "()"}
                   Err(x)                    TuplePattern
                      (x)                    TuplePattern.items{dk: "()"}
                             H<R<'_>>        TypeCall
                              <R<'_>>        TypeCall.typeArguments{dk: "<>"}
                               R<'_>         TypeCall
                                <'_>         TypeCall.typeArguments{dk: "<>"}
                                 '_          LtElided
                                       {}    FunctionDeclaration.body{dk: "{}"}                                                           */
fn fw3(H(Ok(x) | Err(x)): H<R<'_>>) {}                                                                                                    /*
fn•fw3(H(Ok(x)•|•Err(x)):•H<R<'_>>)•{}    FunctionDeclaration
      (H(Ok(x)•|•Err(x)):•H<R<'_>>)       FunctionDeclaration.parameters{dk: "()"}
       H(Ok(x)•|•Err(x)):•H<R<'_>>        FunctionParameterDeclaration
       H(Ok(x)•|•Err(x))                  TuplePattern
        (Ok(x)•|•Err(x))                  TuplePattern.items{dk: "()"}
         Ok(x)•|•Err(x)                   UnionPattern
         Ok(x)                            TuplePattern
           (x)                            TuplePattern.items{dk: "()"}
                 Err(x)                   TuplePattern
                    (x)                   TuplePattern.items{dk: "()"}
                          H<R<'_>>        TypeCall
                           <R<'_>>        TypeCall.typeArguments{dk: "<>"}
                            R<'_>         TypeCall
                             <'_>         TypeCall.typeArguments{dk: "<>"}
                              '_          LtElided
                                    {}    FunctionDeclaration.body{dk: "{}"}                                                              */
fn f2(&(Ok(x) | Err(x)): R<'_>) {}                                                                                                        /*
fn•f2(&(Ok(x)•|•Err(x)):•R<'_>)•{}    FunctionDeclaration
     (&(Ok(x)•|•Err(x)):•R<'_>)       FunctionDeclaration.parameters{dk: "()"}
      &(Ok(x)•|•Err(x)):•R<'_>        FunctionParameterDeclaration
      &(Ok(x)•|•Err(x))               ReferencePattern{!mut}
        Ok(x)•|•Err(x)                UnionPattern
        Ok(x)                         TuplePattern
          (x)                         TuplePattern.items{dk: "()"}
                Err(x)                TuplePattern
                   (x)                TuplePattern.items{dk: "()"}
                         R<'_>        TypeCall
                          <'_>        TypeCall.typeArguments{dk: "<>"}
                           '_         LtElided
                                {}    FunctionDeclaration.body{dk: "{}"}                                                                  */
fn f3((Ok(x) | Err(x)): R<'_>) {}                                                                                                         /*
fn•f3((Ok(x)•|•Err(x)):•R<'_>)•{}    FunctionDeclaration
     ((Ok(x)•|•Err(x)):•R<'_>)       FunctionDeclaration.parameters{dk: "()"}
      (Ok(x)•|•Err(x)):•R<'_>        FunctionParameterDeclaration
       Ok(x)•|•Err(x)                UnionPattern
       Ok(x)                         TuplePattern
         (x)                         TuplePattern.items{dk: "()"}
               Err(x)                TuplePattern
                  (x)                TuplePattern.items{dk: "()"}
                        R<'_>        TypeCall
                         <'_>        TypeCall.typeArguments{dk: "<>"}
                          '_         LtElided
                               {}    FunctionDeclaration.body{dk: "{}"}                                                                   */
fn fun((A | B): _) {}                                                                                                                     /*
fn•fun((A•|•B):•_)•{}    FunctionDeclaration
      ((A•|•B):•_)       FunctionDeclaration.parameters{dk: "()"}
       (A•|•B):•_        FunctionParameterDeclaration
        A•|•B            UnionPattern
                _        TypeInferred
                   {}    FunctionDeclaration.body{dk: "{}"}                                                                               */
fn f(x @ (A::R(_) | D::E(_)): Q) {}                                                                                                       /*
fn•f(x•@•(A::R(_)•|•D::E(_)):•Q)•{}    FunctionDeclaration
    (x•@•(A::R(_)•|•D::E(_)):•Q)       FunctionDeclaration.parameters{dk: "()"}
     x•@•(A::R(_)•|•D::E(_)):•Q        FunctionParameterDeclaration
     x•@•(A::R(_)•|•D::E(_))           PatternVariableDeclaration{!ref, !mut}
          A::R(_)•|•D::E(_)            UnionPattern
          A::R(_)                      TuplePattern
          A::R                         ExpressionPath
              (_)                      TuplePattern.items{dk: "()"}
               _                       WildcardPattern
                    D::E(_)            TuplePattern
                    D::E               ExpressionPath
                        (_)            TuplePattern.items{dk: "()"}
                         _             WildcardPattern
                                 {}    FunctionDeclaration.body{dk: "{}"}                                                                 */

fn x() {                                                                                                                                  /*
fn•x()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	let (0 | (1 | _)) = 0;                                                                                                                /*
	let•(0•|•(1•|•_))•=•0;    LetVariableDeclaration
	     0•|•(1•|•_)          UnionPattern
	     0                    Literal{kind: Integer}
	          1•|•_           UnionPattern
	          1               Literal{kind: Integer}
	              _           WildcardPattern
	                    0     Literal{kind: Integer}                                                                                      */
    if let 0 | (1 | 2) = 0 {}                                                                                                             /*
    if•let•0•|•(1•|•2)•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
       let•0•|•(1•|•2)•=•0       LetScrutinee
           0•|•(1•|•2)           UnionPattern
           0                     Literal{kind: Integer}
                1•|•2            UnionPattern
                1                Literal{kind: Integer}
                    2            Literal{kind: Integer}
                         0       Literal{kind: Integer}
                           {}    IfBlockExpression.body{dk: "{}"}                                                                         */
    if let x @ 0 | x @ (1 | 2) = 0 {}                                                                                                     /*
    if•let•x•@•0•|•x•@•(1•|•2)•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
       let•x•@•0•|•x•@•(1•|•2)•=•0       LetScrutinee
           x•@•0•|•x•@•(1•|•2)           UnionPattern
           x•@•0                         PatternVariableDeclaration{!ref, !mut}
               0                         Literal{kind: Integer}
                   x•@•(1•|•2)           PatternVariableDeclaration{!ref, !mut}
                        1•|•2            UnionPattern
                        1                Literal{kind: Integer}
                            2            Literal{kind: Integer}
                                 0       Literal{kind: Integer}
                                   {}    IfBlockExpression.body{dk: "{}"}                                                                 */
    if let H(Ok(mut x) | &Err(mut x)) = a {                                                                                               /*
    if•let•H(Ok(mut•x)•|•&Err(mut•x))•=•a•{↲    <ExpressionStatement{!semi}>
    if•let•H(Ok(mut•x)•|•&Err(mut•x))•=•a•{↲    <IfBlockExpression>
       let•H(Ok(mut•x)•|•&Err(mut•x))•=•a       LetScrutinee
           H(Ok(mut•x)•|•&Err(mut•x))           TuplePattern
            (Ok(mut•x)•|•&Err(mut•x))           TuplePattern.items{dk: "()"}
             Ok(mut•x)•|•&Err(mut•x)            UnionPattern
             Ok(mut•x)                          TuplePattern
               (mut•x)                          TuplePattern.items{dk: "()"}
                mut•x                           PatternVariableDeclaration{!ref, mut}
                         &Err(mut•x)            ReferencePattern{!mut}
                          Err(mut•x)            TuplePattern
                             (mut•x)            TuplePattern.items{dk: "()"}
                              mut•x             PatternVariableDeclaration{!ref, mut}
                                          {↲    <IfBlockExpression.body{dk: "{}"}>                                                        */
    }                                                                                                                                     /*
••••}    </IfBlockExpression.body>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
    if let H(&(Ok(x) | Err(x))) = a {                                                                                                     /*
    if•let•H(&(Ok(x)•|•Err(x)))•=•a•{↲    <ExpressionStatement{!semi}>
    if•let•H(&(Ok(x)•|•Err(x)))•=•a•{↲    <IfBlockExpression>
       let•H(&(Ok(x)•|•Err(x)))•=•a       LetScrutinee
           H(&(Ok(x)•|•Err(x)))           TuplePattern
            (&(Ok(x)•|•Err(x)))           TuplePattern.items{dk: "()"}
             &(Ok(x)•|•Err(x))            ReferencePattern{!mut}
               Ok(x)•|•Err(x)             UnionPattern
               Ok(x)                      TuplePattern
                 (x)                      TuplePattern.items{dk: "()"}
                       Err(x)             TuplePattern
                          (x)             TuplePattern.items{dk: "()"}
                                    {↲    <IfBlockExpression.body{dk: "{}"}>                                                              */
    }                                                                                                                                     /*
••••}    </IfBlockExpression.body>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
    if let H(Ok(x) | Err(x)) = a {                                                                                                        /*
    if•let•H(Ok(x)•|•Err(x))•=•a•{↲    <ExpressionStatement{!semi}>
    if•let•H(Ok(x)•|•Err(x))•=•a•{↲    <IfBlockExpression>
       let•H(Ok(x)•|•Err(x))•=•a       LetScrutinee
           H(Ok(x)•|•Err(x))           TuplePattern
            (Ok(x)•|•Err(x))           TuplePattern.items{dk: "()"}
             Ok(x)•|•Err(x)            UnionPattern
             Ok(x)                     TuplePattern
               (x)                     TuplePattern.items{dk: "()"}
                     Err(x)            TuplePattern
                        (x)            TuplePattern.items{dk: "()"}
                                 {↲    <IfBlockExpression.body{dk: "{}"}>                                                                 */
    }                                                                                                                                     /*
••••}    </IfBlockExpression.body>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
    for H(Ok(mut x) | &Err(mut x)) in std::iter::once(wres) {}                                                                            /*
    for•H(Ok(mut•x)•|•&Err(mut•x))•in•std::iter::once(wres)•{}    ExpressionStatement{!semi}, ForInBlockExpression
        H(Ok(mut•x)•|•&Err(mut•x))                                TuplePattern
         (Ok(mut•x)•|•&Err(mut•x))                                TuplePattern.items{dk: "()"}
          Ok(mut•x)•|•&Err(mut•x)                                 UnionPattern
          Ok(mut•x)                                               TuplePattern
            (mut•x)                                               TuplePattern.items{dk: "()"}
             mut•x                                                PatternVariableDeclaration{!ref, mut}
                      &Err(mut•x)                                 ReferencePattern{!mut}
                       Err(mut•x)                                 TuplePattern
                          (mut•x)                                 TuplePattern.items{dk: "()"}
                           mut•x                                  PatternVariableDeclaration{!ref, mut}
                                      std::iter::once(wres)       CallExpression
                                      std::iter::once             ExpressionPath
                                      std::iter                   ExpressionPath
                                                     (wres)       CallExpression.arguments{dk: "()"}
                                                            {}    ForInBlockExpression.body{dk: "{}"}                                     */
    for H(&(Ok(x) | Err(x))) in std::iter::once(wres) {}                                                                                  /*
    for•H(&(Ok(x)•|•Err(x)))•in•std::iter::once(wres)•{}    ExpressionStatement{!semi}, ForInBlockExpression
        H(&(Ok(x)•|•Err(x)))                                TuplePattern
         (&(Ok(x)•|•Err(x)))                                TuplePattern.items{dk: "()"}
          &(Ok(x)•|•Err(x))                                 ReferencePattern{!mut}
            Ok(x)•|•Err(x)                                  UnionPattern
            Ok(x)                                           TuplePattern
              (x)                                           TuplePattern.items{dk: "()"}
                    Err(x)                                  TuplePattern
                       (x)                                  TuplePattern.items{dk: "()"}
                                std::iter::once(wres)       CallExpression
                                std::iter::once             ExpressionPath
                                std::iter                   ExpressionPath
                                               (wres)       CallExpression.arguments{dk: "()"}
                                                      {}    ForInBlockExpression.body{dk: "{}"}                                           */
    for H(Ok(x) | Err(x)) in std::iter::once(wres) {}                                                                                     /*
    for•H(Ok(x)•|•Err(x))•in•std::iter::once(wres)•{}    ExpressionStatement{!semi}, ForInBlockExpression
        H(Ok(x)•|•Err(x))                                TuplePattern
         (Ok(x)•|•Err(x))                                TuplePattern.items{dk: "()"}
          Ok(x)•|•Err(x)                                 UnionPattern
          Ok(x)                                          TuplePattern
            (x)                                          TuplePattern.items{dk: "()"}
                  Err(x)                                 TuplePattern
                     (x)                                 TuplePattern.items{dk: "()"}
                             std::iter::once(wres)       CallExpression
                             std::iter::once             ExpressionPath
                             std::iter                   ExpressionPath
                                            (wres)       CallExpression.arguments{dk: "()"}
                                                   {}    ForInBlockExpression.body{dk: "{}"}                                              */
    
    let H(Ok(mut x) | &Err(mut x)) = wres;                                                                                                /*
    let•H(Ok(mut•x)•|•&Err(mut•x))•=•wres;    LetVariableDeclaration
        H(Ok(mut•x)•|•&Err(mut•x))            TuplePattern
         (Ok(mut•x)•|•&Err(mut•x))            TuplePattern.items{dk: "()"}
          Ok(mut•x)•|•&Err(mut•x)             UnionPattern
          Ok(mut•x)                           TuplePattern
            (mut•x)                           TuplePattern.items{dk: "()"}
             mut•x                            PatternVariableDeclaration{!ref, mut}
                      &Err(mut•x)             ReferencePattern{!mut}
                       Err(mut•x)             TuplePattern
                          (mut•x)             TuplePattern.items{dk: "()"}
                           mut•x              PatternVariableDeclaration{!ref, mut}                                                       */
    let H(Ok(x) | Err(x)) = wres;                                                                                                         /*
    let•H(Ok(x)•|•Err(x))•=•wres;    LetVariableDeclaration
        H(Ok(x)•|•Err(x))            TuplePattern
         (Ok(x)•|•Err(x))            TuplePattern.items{dk: "()"}
          Ok(x)•|•Err(x)             UnionPattern
          Ok(x)                      TuplePattern
            (x)                      TuplePattern.items{dk: "()"}
                  Err(x)             TuplePattern
                     (x)             TuplePattern.items{dk: "()"}                                                                         */
    let H(&(Ok(x) | Err(x))) = wres;                                                                                                      /*
    let•H(&(Ok(x)•|•Err(x)))•=•wres;    LetVariableDeclaration
        H(&(Ok(x)•|•Err(x)))            TuplePattern
         (&(Ok(x)•|•Err(x)))            TuplePattern.items{dk: "()"}
          &(Ok(x)•|•Err(x))             ReferencePattern{!mut}
            Ok(x)•|•Err(x)              UnionPattern
            Ok(x)                       TuplePattern
              (x)                       TuplePattern.items{dk: "()"}
                    Err(x)              TuplePattern
                       (x)              TuplePattern.items{dk: "()"}                                                                      */


	let (Tri::A(Ok(mut x) | Err(mut x))                                                                                                   /*
	let•(Tri::A(Ok(mut•x)•|•Err(mut•x))↲    <LetVariableDeclaration>
	     Tri::A(Ok(mut•x)•|•Err(mut•x))↲    <UnionPattern>
	     Tri::A(Ok(mut•x)•|•Err(mut•x))     TuplePattern
	     Tri::A                             ExpressionPath
	           (Ok(mut•x)•|•Err(mut•x))     TuplePattern.items{dk: "()"}
	            Ok(mut•x)•|•Err(mut•x)      UnionPattern
	            Ok(mut•x)                   TuplePattern
	              (mut•x)                   TuplePattern.items{dk: "()"}
	               mut•x                    PatternVariableDeclaration{!ref, mut}
	                        Err(mut•x)      TuplePattern
	                           (mut•x)      TuplePattern.items{dk: "()"}
	                            mut•x       PatternVariableDeclaration{!ref, mut}                                                         */
    | Tri::B(&Ok(mut x) | Err(mut x))                                                                                                     /*
      Tri::B(&Ok(mut•x)•|•Err(mut•x))    TuplePattern
      Tri::B                             ExpressionPath
            (&Ok(mut•x)•|•Err(mut•x))    TuplePattern.items{dk: "()"}
             &Ok(mut•x)•|•Err(mut•x)     UnionPattern
             &Ok(mut•x)                  ReferencePattern{!mut}
              Ok(mut•x)                  TuplePattern
                (mut•x)                  TuplePattern.items{dk: "()"}
                 mut•x                   PatternVariableDeclaration{!ref, mut}
                          Err(mut•x)     TuplePattern
                             (mut•x)     TuplePattern.items{dk: "()"}
                              mut•x      PatternVariableDeclaration{!ref, mut}                                                            */
    | &Tri::C(Ok(mut x) | Err(mut x))) = tri;                                                                                             /*
      &Tri::C(Ok(mut•x)•|•Err(mut•x))            ReferencePattern{!mut}
       Tri::C(Ok(mut•x)•|•Err(mut•x))            TuplePattern
       Tri::C                                    ExpressionPath
             (Ok(mut•x)•|•Err(mut•x))            TuplePattern.items{dk: "()"}
              Ok(mut•x)•|•Err(mut•x)             UnionPattern
              Ok(mut•x)                          TuplePattern
                (mut•x)                          TuplePattern.items{dk: "()"}
                 mut•x                           PatternVariableDeclaration{!ref, mut}
                          Err(mut•x)             TuplePattern
                             (mut•x)             TuplePattern.items{dk: "()"}
                              mut•x              PatternVariableDeclaration{!ref, mut}
••••|•&Tri::C(Ok(mut•x)•|•Err(mut•x))            </UnionPattern>
••••|•&Tri::C(Ok(mut•x)•|•Err(mut•x)))•=•tri;    </LetVariableDeclaration>                                                                */

    let (B(A(a, _) | B(a)) | A(a, A(a, _) | B(a))) = B(B(1));                                                                             /*
    let•(B(A(a,•_)•|•B(a))•|•A(a,•A(a,•_)•|•B(a)))•=•B(B(1));    LetVariableDeclaration
         B(A(a,•_)•|•B(a))•|•A(a,•A(a,•_)•|•B(a))                UnionPattern
         B(A(a,•_)•|•B(a))                                       TuplePattern
          (A(a,•_)•|•B(a))                                       TuplePattern.items{dk: "()"}
           A(a,•_)•|•B(a)                                        UnionPattern
           A(a,•_)                                               TuplePattern
            (a,•_)                                               TuplePattern.items{dk: "()"}
                _                                                WildcardPattern
                     B(a)                                        TuplePattern
                      (a)                                        TuplePattern.items{dk: "()"}
                             A(a,•A(a,•_)•|•B(a))                TuplePattern
                              (a,•A(a,•_)•|•B(a))                TuplePattern.items{dk: "()"}
                                  A(a,•_)•|•B(a)                 UnionPattern
                                  A(a,•_)                        TuplePattern
                                   (a,•_)                        TuplePattern.items{dk: "()"}
                                       _                         WildcardPattern
                                            B(a)                 TuplePattern
                                             (a)                 TuplePattern.items{dk: "()"}
                                                     B(B(1))     CallExpression
                                                      (B(1))     CallExpression.arguments{dk: "()"}
                                                       B(1)      CallExpression
                                                        (1)      CallExpression.arguments{dk: "()"}
                                                         1       Literal{kind: Integer}                                                   */
    let (B(_) | A(A(a, _) | B(a), A(a, _) | B(a))) = B(B(1));                                                                             /*
    let•(B(_)•|•A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a)))•=•B(B(1));    LetVariableDeclaration
         B(_)•|•A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                UnionPattern
         B(_)                                                    TuplePattern
          (_)                                                    TuplePattern.items{dk: "()"}
           _                                                     WildcardPattern
                A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                TuplePattern
                 (A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                TuplePattern.items{dk: "()"}
                  A(a,•_)•|•B(a)                                 UnionPattern
                  A(a,•_)                                        TuplePattern
                   (a,•_)                                        TuplePattern.items{dk: "()"}
                       _                                         WildcardPattern
                            B(a)                                 TuplePattern
                             (a)                                 TuplePattern.items{dk: "()"}
                                  A(a,•_)•|•B(a)                 UnionPattern
                                  A(a,•_)                        TuplePattern
                                   (a,•_)                        TuplePattern.items{dk: "()"}
                                       _                         WildcardPattern
                                            B(a)                 TuplePattern
                                             (a)                 TuplePattern.items{dk: "()"}
                                                     B(B(1))     CallExpression
                                                      (B(1))     CallExpression.arguments{dk: "()"}
                                                       B(1)      CallExpression
                                                        (1)      CallExpression.arguments{dk: "()"}
                                                         1       Literal{kind: Integer}                                                   */
    let (B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a))) = B(B(1));                                                                /*
    let•(B(A(a,•_)•|•B(a))•|•A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a)))•=•B(B(1));    LetVariableDeclaration
         B(A(a,•_)•|•B(a))•|•A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                UnionPattern
         B(A(a,•_)•|•B(a))                                                    TuplePattern
          (A(a,•_)•|•B(a))                                                    TuplePattern.items{dk: "()"}
           A(a,•_)•|•B(a)                                                     UnionPattern
           A(a,•_)                                                            TuplePattern
            (a,•_)                                                            TuplePattern.items{dk: "()"}
                _                                                             WildcardPattern
                     B(a)                                                     TuplePattern
                      (a)                                                     TuplePattern.items{dk: "()"}
                             A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                TuplePattern
                              (A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                TuplePattern.items{dk: "()"}
                               A(a,•_)•|•B(a)                                 UnionPattern
                               A(a,•_)                                        TuplePattern
                                (a,•_)                                        TuplePattern.items{dk: "()"}
                                    _                                         WildcardPattern
                                         B(a)                                 TuplePattern
                                          (a)                                 TuplePattern.items{dk: "()"}
                                               A(a,•_)•|•B(a)                 UnionPattern
                                               A(a,•_)                        TuplePattern
                                                (a,•_)                        TuplePattern.items{dk: "()"}
                                                    _                         WildcardPattern
                                                         B(a)                 TuplePattern
                                                          (a)                 TuplePattern.items{dk: "()"}
                                                                  B(B(1))     CallExpression
                                                                   (B(1))     CallExpression.arguments{dk: "()"}
                                                                    B(1)      CallExpression
                                                                     (1)      CallExpression.arguments{dk: "()"}
                                                                      1       Literal{kind: Integer}                                      */

	let (Ok(a) | Err(a)) = Ok(0);                                                                                                         /*
	let•(Ok(a)•|•Err(a))•=•Ok(0);    LetVariableDeclaration
	     Ok(a)•|•Err(a)              UnionPattern
	     Ok(a)                       TuplePattern
	       (a)                       TuplePattern.items{dk: "()"}
	             Err(a)              TuplePattern
	                (a)              TuplePattern.items{dk: "()"}
	                       Ok(0)     CallExpression
	                         (0)     CallExpression.arguments{dk: "()"}
	                          0      Literal{kind: Integer}                                                                               */
    let (Ok(ref a) | Err(ref a)) = Ok(0);                                                                                                 /*
    let•(Ok(ref•a)•|•Err(ref•a))•=•Ok(0);    LetVariableDeclaration
         Ok(ref•a)•|•Err(ref•a)              UnionPattern
         Ok(ref•a)                           TuplePattern
           (ref•a)                           TuplePattern.items{dk: "()"}
            ref•a                            PatternVariableDeclaration{ref, !mut}
                     Err(ref•a)              TuplePattern
                        (ref•a)              TuplePattern.items{dk: "()"}
                         ref•a               PatternVariableDeclaration{ref, !mut}
                                   Ok(0)     CallExpression
                                     (0)     CallExpression.arguments{dk: "()"}
                                      0      Literal{kind: Integer}                                                                       */
    let (Ok(ref mut a) | Err(ref mut a)) = Ok(0);                                                                                         /*
    let•(Ok(ref•mut•a)•|•Err(ref•mut•a))•=•Ok(0);    LetVariableDeclaration
         Ok(ref•mut•a)•|•Err(ref•mut•a)              UnionPattern
         Ok(ref•mut•a)                               TuplePattern
           (ref•mut•a)                               TuplePattern.items{dk: "()"}
            ref•mut•a                                PatternVariableDeclaration{ref, mut}
                         Err(ref•mut•a)              TuplePattern
                            (ref•mut•a)              TuplePattern.items{dk: "()"}
                             ref•mut•a               PatternVariableDeclaration{ref, mut}
                                           Ok(0)     CallExpression
                                             (0)     CallExpression.arguments{dk: "()"}
                                              0      Literal{kind: Integer}                                                               */
	let (Ok((V1(a) | V2(a) | V3(a), b)) | Err(Ok((a, b)) | Err((a, b)))): Result<_, Result<_, _>> =                                       /*
	let•(Ok((V1(a)•|•V2(a)•|•V3(a),•b))•|•Err(Ok((a,•b))•|•Err((a,•b)))):•Result<_,•Result<_,•_>>•=↲    <LetVariableDeclaration>
	     Ok((V1(a)•|•V2(a)•|•V3(a),•b))•|•Err(Ok((a,•b))•|•Err((a,•b)))                                 UnionPattern
	     Ok((V1(a)•|•V2(a)•|•V3(a),•b))                                                                 TuplePattern
	       ((V1(a)•|•V2(a)•|•V3(a),•b))                                                                 TuplePattern.items{dk: "()"}
	        (V1(a)•|•V2(a)•|•V3(a),•b)                                                                  TuplePattern
	         V1(a)•|•V2(a)•|•V3(a)                                                                      UnionPattern
	         V1(a)                                                                                      TuplePattern
	           (a)                                                                                      TuplePattern.items{dk: "()"}
	                 V2(a)                                                                              TuplePattern
	                   (a)                                                                              TuplePattern.items{dk: "()"}
	                         V3(a)                                                                      TuplePattern
	                           (a)                                                                      TuplePattern.items{dk: "()"}
	                                      Err(Ok((a,•b))•|•Err((a,•b)))                                 TuplePattern
	                                         (Ok((a,•b))•|•Err((a,•b)))                                 TuplePattern.items{dk: "()"}
	                                          Ok((a,•b))•|•Err((a,•b))                                  UnionPattern
	                                          Ok((a,•b))                                                TuplePattern
	                                            ((a,•b))                                                TuplePattern.items{dk: "()"}
	                                             (a,•b)                                                 TuplePattern
	                                                       Err((a,•b))                                  TuplePattern
	                                                          ((a,•b))                                  TuplePattern.items{dk: "()"}
	                                                           (a,•b)                                   TuplePattern
	                                                                      Result<_,•Result<_,•_>>       TypeCall
	                                                                            <_,•Result<_,•_>>       TypeCall.typeArguments{dk: "<>"}
	                                                                             _                      TypeInferred
	                                                                                Result<_,•_>        TypeCall
	                                                                                      <_,•_>        TypeCall.typeArguments{dk: "<>"}
	                                                                                       _            TypeInferred
	                                                                                          _         TypeInferred                      */
	Ok((V1(1), 1));                                                                                                                       /*
	Ok((V1(1),•1))     CallExpression
	  ((V1(1),•1))     CallExpression.arguments{dk: "()"}
	   (V1(1),•1)      TupleLiteral
	    V1(1)          CallExpression
	      (1)          CallExpression.arguments{dk: "()"}
	       1           Literal{kind: Integer}
	           1       Literal{kind: Integer}
   ╚Ok((V1(1),•1));    </LetVariableDeclaration>                                                                                          */

	let (Ok((V1(a) | V2(a) | V3(a), ref b)) | Err(Ok((a, ref b)) | Err((a, ref b)))): Result<                                             /*
	let•(Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))•|•Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))):•Result<↲    <LetVariableDeclaration>
	     Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))•|•Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))               UnionPattern
	     Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))                                                       TuplePattern
	       ((V1(a)•|•V2(a)•|•V3(a),•ref•b))                                                       TuplePattern.items{dk: "()"}
	        (V1(a)•|•V2(a)•|•V3(a),•ref•b)                                                        TuplePattern
	         V1(a)•|•V2(a)•|•V3(a)                                                                UnionPattern
	         V1(a)                                                                                TuplePattern
	           (a)                                                                                TuplePattern.items{dk: "()"}
	                 V2(a)                                                                        TuplePattern
	                   (a)                                                                        TuplePattern.items{dk: "()"}
	                         V3(a)                                                                TuplePattern
	                           (a)                                                                TuplePattern.items{dk: "()"}
	                                ref•b                                                         PatternVariableDeclaration{ref, !mut}
	                                          Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))               TuplePattern
	                                             (Ok((a,•ref•b))•|•Err((a,•ref•b)))               TuplePattern.items{dk: "()"}
	                                              Ok((a,•ref•b))•|•Err((a,•ref•b))                UnionPattern
	                                              Ok((a,•ref•b))                                  TuplePattern
	                                                ((a,•ref•b))                                  TuplePattern.items{dk: "()"}
	                                                 (a,•ref•b)                                   TuplePattern
	                                                     ref•b                                    PatternVariableDeclaration{ref, !mut}
	                                                               Err((a,•ref•b))                TuplePattern
	                                                                  ((a,•ref•b))                TuplePattern.items{dk: "()"}
	                                                                   (a,•ref•b)                 TuplePattern
	                                                                       ref•b                  PatternVariableDeclaration{ref, !mut}
	                                                                                  Result<↲    <TypeCall>
	                                                                                        <↲    <TypeCall.typeArguments{dk: "<>"}>      */
		_,                                                                                                                                /*
		_    TypeInferred                                                                                                                 */
		Result<_, _>,                                                                                                                     /*
		Result<_,•_>    TypeCall
		      <_,•_>    TypeCall.typeArguments{dk: "<>"}
		       _        TypeInferred
		          _     TypeInferred                                                                                                      */
	> = Ok((V1(1), 1));                                                                                                                   /*
   ╚>                      </TypeCall.typeArguments>
   ╚>                      </TypeCall>
	    Ok((V1(1),•1))     CallExpression
	      ((V1(1),•1))     CallExpression.arguments{dk: "()"}
	       (V1(1),•1)      TupleLiteral
	        V1(1)          CallExpression
	          (1)          CallExpression.arguments{dk: "()"}
	           1           Literal{kind: Integer}
	               1       Literal{kind: Integer}
   ╚>•=•Ok((V1(1),•1));    </LetVariableDeclaration>                                                                                      */

	let (                                                                                                                                 /*
	let•(↲    <LetVariableDeclaration>
	    (↲    <TuplePattern>                                                                                                              */
        a,
        Err((ref mut b, ref c, d))                                                                                                        /*
        Err((ref•mut•b,•ref•c,•d))↲    <UnionPattern>
        Err((ref•mut•b,•ref•c,•d))     TuplePattern
           ((ref•mut•b,•ref•c,•d))     TuplePattern.items{dk: "()"}
            (ref•mut•b,•ref•c,•d)      TuplePattern
             ref•mut•b                 PatternVariableDeclaration{ref, mut}
                        ref•c          PatternVariableDeclaration{ref, !mut}                                                              */
        | Ok((                                                                                                                            /*
          Ok((↲    <TuplePattern>
            ((↲    <TuplePattern.items{dk: "()"}>
             (↲    <TuplePattern>                                                                                                         */
            Ok(V1((ref c, d)) | V2((d, ref c)) | V3((ref c, Ok((_, d)) | Err((d, _)))))                                                   /*
            Ok(V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))↲    <UnionPattern>
            Ok(V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))     TuplePattern
              (V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))     TuplePattern.items{dk: "()"}
               V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_))))      UnionPattern
               V1((ref•c,•d))                                                               TuplePattern
                 ((ref•c,•d))                                                               TuplePattern.items{dk: "()"}
                  (ref•c,•d)                                                                TuplePattern
                   ref•c                                                                    PatternVariableDeclaration{ref, !mut}
                                V2((d,•ref•c))                                              TuplePattern
                                  ((d,•ref•c))                                              TuplePattern.items{dk: "()"}
                                   (d,•ref•c)                                               TuplePattern
                                       ref•c                                                PatternVariableDeclaration{ref, !mut}
                                                 V3((ref•c,•Ok((_,•d))•|•Err((d,•_))))      TuplePattern
                                                   ((ref•c,•Ok((_,•d))•|•Err((d,•_))))      TuplePattern.items{dk: "()"}
                                                    (ref•c,•Ok((_,•d))•|•Err((d,•_)))       TuplePattern
                                                     ref•c                                  PatternVariableDeclaration{ref, !mut}
                                                            Ok((_,•d))•|•Err((d,•_))        UnionPattern
                                                            Ok((_,•d))                      TuplePattern
                                                              ((_,•d))                      TuplePattern.items{dk: "()"}
                                                               (_,•d)                       TuplePattern
                                                                _                           WildcardPattern
                                                                         Err((d,•_))        TuplePattern
                                                                            ((d,•_))        TuplePattern.items{dk: "()"}
                                                                             (d,•_)         TuplePattern
                                                                                 _          WildcardPattern                               */
            | Err((ref c, d)),                                                                                                            /*
              Err((ref•c,•d))    TuplePattern
                 ((ref•c,•d))    TuplePattern.items{dk: "()"}
                  (ref•c,•d)     TuplePattern
                   ref•c         PatternVariableDeclaration{ref, !mut}
••••••••••••|•Err((ref•c,•d))    </UnionPattern>                                                                                          */
            ref mut b,                                                                                                                    /*
            ref•mut•b    PatternVariableDeclaration{ref, mut}                                                                             */
        )),                                                                                                                               /*
••••••••)     </TuplePattern>
••••••••))    </TuplePattern.items>
••••••••))    </TuplePattern>
••••••••))    </UnionPattern>                                                                                                             */
    ): (_, Result<_, _>) = (1, Ok((Ok(V3((1, Ok::<_, (i32, i32)>((1, 1))))), 1)));                                                        /*
••••)                                                                                 </TuplePattern>
       (_,•Result<_,•_>)                                                              TypeTuple
        _                                                                             TypeInferred
           Result<_,•_>                                                               TypeCall
                 <_,•_>                                                               TypeCall.typeArguments{dk: "<>"}
                  _                                                                   TypeInferred
                     _                                                                TypeInferred
                           (1,•Ok((Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1)))     TupleLiteral
                            1                                                         Literal{kind: Integer}
                               Ok((Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1))      CallExpression
                                 ((Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1))      CallExpression.arguments{dk: "()"}
                                  (Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1)       TupleLiteral
                                   Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1)))))           CallExpression
                                     (V3((1,•Ok::<_,•(i32,•i32)>((1,•1)))))           CallExpression.arguments{dk: "()"}
                                      V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))            CallExpression
                                        ((1,•Ok::<_,•(i32,•i32)>((1,•1))))            CallExpression.arguments{dk: "()"}
                                         (1,•Ok::<_,•(i32,•i32)>((1,•1)))             TupleLiteral
                                          1                                           Literal{kind: Integer}
                                             Ok::<_,•(i32,•i32)>((1,•1))              CallExpression
                                                 <_,•(i32,•i32)>                      CallExpression.typeArguments{dk: "<>"}
                                                  _                                   TypeInferred
                                                     (i32,•i32)                       TypeTuple
                                                                ((1,•1))              CallExpression.arguments{dk: "()"}
                                                                 (1,•1)               TupleLiteral
                                                                  1                   Literal{kind: Integer}
                                                                     1                Literal{kind: Integer}
                                                                             1        Literal{kind: Integer}
••••):•(_,•Result<_,•_>)•=•(1,•Ok((Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1)));    </LetVariableDeclaration>                           */

	for &(Ok(i) | Err(i)) in &v {                                                                                                         /*
	for•&(Ok(i)•|•Err(i))•in•&v•{↲    <ExpressionStatement{!semi}>
	for•&(Ok(i)•|•Err(i))•in•&v•{↲    <ForInBlockExpression>
	    &(Ok(i)•|•Err(i))             ReferencePattern{!mut}
	      Ok(i)•|•Err(i)              UnionPattern
	      Ok(i)                       TuplePattern
	        (i)                       TuplePattern.items{dk: "()"}
	              Err(i)              TuplePattern
	                 (i)              TuplePattern.items{dk: "()"}
	                         &v       ReferenceExpression{!mut}
	                            {↲    <ForInBlockExpression.body{dk: "{}"}>                                                               */
    }                                                                                                                                     /*
••••}    </ForInBlockExpression.body>
••••}    </ForInBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
    for Ok(i) | Err(i) in v {                                                                                                             /*
    for•Ok(i)•|•Err(i)•in•v•{↲    <ExpressionStatement{!semi}>
    for•Ok(i)•|•Err(i)•in•v•{↲    <ForInBlockExpression>
        Ok(i)•|•Err(i)            UnionPattern
        Ok(i)                     TuplePattern
          (i)                     TuplePattern.items{dk: "()"}
                Err(i)            TuplePattern
                   (i)            TuplePattern.items{dk: "()"}
                            {↲    <ForInBlockExpression.body{dk: "{}"}>                                                                   */
    }                                                                                                                                     /*
••••}    </ForInBlockExpression.body>
••••}    </ForInBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
    if let &(None | Some(6 | 7)) = &opt {                                                                                                 /*
    if•let•&(None•|•Some(6•|•7))•=•&opt•{↲    <ExpressionStatement{!semi}>
    if•let•&(None•|•Some(6•|•7))•=•&opt•{↲    <IfBlockExpression>
       let•&(None•|•Some(6•|•7))•=•&opt       LetScrutinee
           &(None•|•Some(6•|•7))              ReferencePattern{!mut}
             None•|•Some(6•|•7)               UnionPattern
                    Some(6•|•7)               TuplePattern
                        (6•|•7)               TuplePattern.items{dk: "()"}
                         6•|•7                UnionPattern
                         6                    Literal{kind: Integer}
                             7                Literal{kind: Integer}
                                   &opt       ReferenceExpression{!mut}
                                        {↲    <IfBlockExpression.body{dk: "{}"}>                                                          */
    }                                                                                                                                     /*
••••}    </IfBlockExpression.body>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
    if let Some(x @ (4 | 5 | 6)) = opt {                                                                                                  /*
    if•let•Some(x•@•(4•|•5•|•6))•=•opt•{↲    <ExpressionStatement{!semi}>
    if•let•Some(x•@•(4•|•5•|•6))•=•opt•{↲    <IfBlockExpression>
       let•Some(x•@•(4•|•5•|•6))•=•opt       LetScrutinee
           Some(x•@•(4•|•5•|•6))             TuplePattern
               (x•@•(4•|•5•|•6))             TuplePattern.items{dk: "()"}
                x•@•(4•|•5•|•6)              PatternVariableDeclaration{!ref, !mut}
                     4•|•5•|•6               UnionPattern
                     4                       Literal{kind: Integer}
                         5                   Literal{kind: Integer}
                             6               Literal{kind: Integer}
                                       {↲    <IfBlockExpression.body{dk: "{}"}>                                                           */
    } else {                                                                                                                              /*
••••}            </IfBlockExpression.body>
           {↲    <BlockExpression>                                                                                                        */
    }                                                                                                                                     /*
••••}    </BlockExpression>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
    while let Some(ref mut val @ (3 | 4 | 6)) = opt {                                                                                     /*
    while•let•Some(ref•mut•val•@•(3•|•4•|•6))•=•opt•{↲    <ExpressionStatement{!semi}>
    while•let•Some(ref•mut•val•@•(3•|•4•|•6))•=•opt•{↲    <WhileBlockExpression>
          let•Some(ref•mut•val•@•(3•|•4•|•6))•=•opt       LetScrutinee
              Some(ref•mut•val•@•(3•|•4•|•6))             TuplePattern
                  (ref•mut•val•@•(3•|•4•|•6))             TuplePattern.items{dk: "()"}
                   ref•mut•val•@•(3•|•4•|•6)              PatternVariableDeclaration{ref, mut}
                                  3•|•4•|•6               UnionPattern
                                  3                       Literal{kind: Integer}
                                      4                   Literal{kind: Integer}
                                          6               Literal{kind: Integer}
                                                    {↲    <WhileBlockExpression.body{dk: "{}"}>                                           */
    }                                                                                                                                     /*
••••}    </WhileBlockExpression.body>
••••}    </WhileBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
	
	let (| A | B);                                                                                                                        /*
	let•(|•A•|•B);    LetVariableDeclaration
	     |•A•|•B      UnionPattern                                                                                                        */
    let (A | B);                                                                                                                          /*
    let•(A•|•B);    LetVariableDeclaration
         A•|•B      UnionPattern                                                                                                          */
    let (A | B): u8;                                                                                                                      /*
    let•(A•|•B):•u8;    LetVariableDeclaration
         A•|•B          UnionPattern                                                                                                      */
    let (A | B) = 0;                                                                                                                      /*
    let•(A•|•B)•=•0;    LetVariableDeclaration
         A•|•B          UnionPattern
                  0     Literal{kind: Integer}                                                                                            */
    let (A | B): u8 = 0;                                                                                                                  /*
    let•(A•|•B):•u8•=•0;    LetVariableDeclaration
         A•|•B              UnionPattern
                      0     Literal{kind: Integer}                                                                                        */
	for | A | B in 0 {}                                                                                                                   /*
	for•|•A•|•B•in•0•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    |•A•|•B            UnionPattern
	               0       Literal{kind: Integer}
	                 {}    ForInBlockExpression.body{dk: "{}"}                                                                            */
    for A | B in 0 {}                                                                                                                     /*
    for•A•|•B•in•0•{}    ExpressionStatement{!semi}, ForInBlockExpression
        A•|•B            UnionPattern
                 0       Literal{kind: Integer}
                   {}    ForInBlockExpression.body{dk: "{}"}                                                                              */
	while let | A | B = 0 {}                                                                                                              /*
	while•let•|•A•|•B•=•0•{}    ExpressionStatement{!semi}, WhileBlockExpression
	      let•|•A•|•B•=•0       LetScrutinee
	          |•A•|•B           UnionPattern
	                    0       Literal{kind: Integer}
	                      {}    WhileBlockExpression.body{dk: "{}"}                                                                       */
    while let A | B = 0 {}                                                                                                                /*
    while•let•A•|•B•=•0•{}    ExpressionStatement{!semi}, WhileBlockExpression
          let•A•|•B•=•0       LetScrutinee
              A•|•B           UnionPattern
                      0       Literal{kind: Integer}
                        {}    WhileBlockExpression.body{dk: "{}"}                                                                         */
	if let | A | B = 0 {}                                                                                                                 /*
	if•let•|•A•|•B•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•|•A•|•B•=•0       LetScrutinee
	       |•A•|•B           UnionPattern
	                 0       Literal{kind: Integer}
	                   {}    IfBlockExpression.body{dk: "{}"}                                                                             */
    if let A | B = 0 {}                                                                                                                   /*
    if•let•A•|•B•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
       let•A•|•B•=•0       LetScrutinee
           A•|•B           UnionPattern
                   0       Literal{kind: Integer}
                     {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let Ok(mut x) | &Err(mut x) = res {                                                                                                /*
	if•let•Ok(mut•x)•|•&Err(mut•x)•=•res•{↲    <ExpressionStatement{!semi}>
	if•let•Ok(mut•x)•|•&Err(mut•x)•=•res•{↲    <IfBlockExpression>
	   let•Ok(mut•x)•|•&Err(mut•x)•=•res       LetScrutinee
	       Ok(mut•x)•|•&Err(mut•x)             UnionPattern
	       Ok(mut•x)                           TuplePattern
	         (mut•x)                           TuplePattern.items{dk: "()"}
	          mut•x                            PatternVariableDeclaration{!ref, mut}
	                   &Err(mut•x)             ReferencePattern{!mut}
	                    Err(mut•x)             TuplePattern
	                       (mut•x)             TuplePattern.items{dk: "()"}
	                        mut•x              PatternVariableDeclaration{!ref, mut}
	                                     {↲    <IfBlockExpression.body{dk: "{}"}>                                                         */
    }                                                                                                                                     /*
••••}    </IfBlockExpression.body>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
	if let &(Ok(x) | Err(x)) = res {                                                                                                      /*
	if•let•&(Ok(x)•|•Err(x))•=•res•{↲    <ExpressionStatement{!semi}>
	if•let•&(Ok(x)•|•Err(x))•=•res•{↲    <IfBlockExpression>
	   let•&(Ok(x)•|•Err(x))•=•res       LetScrutinee
	       &(Ok(x)•|•Err(x))             ReferencePattern{!mut}
	         Ok(x)•|•Err(x)              UnionPattern
	         Ok(x)                       TuplePattern
	           (x)                       TuplePattern.items{dk: "()"}
	                 Err(x)              TuplePattern
	                    (x)              TuplePattern.items{dk: "()"}
	                               {↲    <IfBlockExpression.body{dk: "{}"}>                                                               */
    }                                                                                                                                     /*
••••}    </IfBlockExpression.body>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
    let (Ok(mut x) | &Err(mut x)) = res;                                                                                                  /*
    let•(Ok(mut•x)•|•&Err(mut•x))•=•res;    LetVariableDeclaration
         Ok(mut•x)•|•&Err(mut•x)            UnionPattern
         Ok(mut•x)                          TuplePattern
           (mut•x)                          TuplePattern.items{dk: "()"}
            mut•x                           PatternVariableDeclaration{!ref, mut}
                     &Err(mut•x)            ReferencePattern{!mut}
                      Err(mut•x)            TuplePattern
                         (mut•x)            TuplePattern.items{dk: "()"}
                          mut•x             PatternVariableDeclaration{!ref, mut}                                                         */
    let &(Ok(x) | Err(x)) = res;                                                                                                          /*
    let•&(Ok(x)•|•Err(x))•=•res;    LetVariableDeclaration
        &(Ok(x)•|•Err(x))           ReferencePattern{!mut}
          Ok(x)•|•Err(x)            UnionPattern
          Ok(x)                     TuplePattern
            (x)                     TuplePattern.items{dk: "()"}
                  Err(x)            TuplePattern
                     (x)            TuplePattern.items{dk: "()"}                                                                          */
    let (Ok(x) | Err(x)) = res;                                                                                                           /*
    let•(Ok(x)•|•Err(x))•=•res;    LetVariableDeclaration
         Ok(x)•|•Err(x)            UnionPattern
         Ok(x)                     TuplePattern
           (x)                     TuplePattern.items{dk: "()"}
                 Err(x)            TuplePattern
                    (x)            TuplePattern.items{dk: "()"}                                                                           */
	for Ok(mut x) | &Err(mut x) in std::iter::once(res) {                                                                                 /*
	for•Ok(mut•x)•|•&Err(mut•x)•in•std::iter::once(res)•{↲    <ExpressionStatement{!semi}>
	for•Ok(mut•x)•|•&Err(mut•x)•in•std::iter::once(res)•{↲    <ForInBlockExpression>
	    Ok(mut•x)•|•&Err(mut•x)                               UnionPattern
	    Ok(mut•x)                                             TuplePattern
	      (mut•x)                                             TuplePattern.items{dk: "()"}
	       mut•x                                              PatternVariableDeclaration{!ref, mut}
	                &Err(mut•x)                               ReferencePattern{!mut}
	                 Err(mut•x)                               TuplePattern
	                    (mut•x)                               TuplePattern.items{dk: "()"}
	                     mut•x                                PatternVariableDeclaration{!ref, mut}
	                               std::iter::once(res)       CallExpression
	                               std::iter::once            ExpressionPath
	                               std::iter                  ExpressionPath
	                                              (res)       CallExpression.arguments{dk: "()"}
	                                                    {↲    <ForInBlockExpression.body{dk: "{}"}>                                       */
    }                                                                                                                                     /*
••••}    </ForInBlockExpression.body>
••••}    </ForInBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
	for &(Ok(x) | Err(x)) in std::iter::once(res) {                                                                                       /*
	for•&(Ok(x)•|•Err(x))•in•std::iter::once(res)•{↲    <ExpressionStatement{!semi}>
	for•&(Ok(x)•|•Err(x))•in•std::iter::once(res)•{↲    <ForInBlockExpression>
	    &(Ok(x)•|•Err(x))                               ReferencePattern{!mut}
	      Ok(x)•|•Err(x)                                UnionPattern
	      Ok(x)                                         TuplePattern
	        (x)                                         TuplePattern.items{dk: "()"}
	              Err(x)                                TuplePattern
	                 (x)                                TuplePattern.items{dk: "()"}
	                         std::iter::once(res)       CallExpression
	                         std::iter::once            ExpressionPath
	                         std::iter                  ExpressionPath
	                                        (res)       CallExpression.arguments{dk: "()"}
	                                              {↲    <ForInBlockExpression.body{dk: "{}"}>                                             */
    }                                                                                                                                     /*
••••}    </ForInBlockExpression.body>
••••}    </ForInBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
    for Ok(x) | Err(x) in std::iter::once(res) {                                                                                          /*
    for•Ok(x)•|•Err(x)•in•std::iter::once(res)•{↲    <ExpressionStatement{!semi}>
    for•Ok(x)•|•Err(x)•in•std::iter::once(res)•{↲    <ForInBlockExpression>
        Ok(x)•|•Err(x)                               UnionPattern
        Ok(x)                                        TuplePattern
          (x)                                        TuplePattern.items{dk: "()"}
                Err(x)                               TuplePattern
                   (x)                               TuplePattern.items{dk: "()"}
                          std::iter::once(res)       CallExpression
                          std::iter::once            ExpressionPath
                          std::iter                  ExpressionPath
                                         (res)       CallExpression.arguments{dk: "()"}
                                               {↲    <ForInBlockExpression.body{dk: "{}"}>                                                */
    }                                                                                                                                     /*
••••}    </ForInBlockExpression.body>
••••}    </ForInBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
	let _ = |(A | B): u8| ();                                                                                                             /*
	let•_•=•|(A•|•B):•u8|•();    LetVariableDeclaration
	    _                        WildcardPattern
	        |(A•|•B):•u8|•()     ClosureFunctionExpression
	        |(A•|•B):•u8|        ClosureFunctionExpression.parameters{dk: "||"}
	         (A•|•B):•u8         ClosureFunctionParameterDeclaration
	          A•|•B              UnionPattern
	                      ()     TupleLiteral                                                                                             */
	let (A | B);                                                                                                                          /*
	let•(A•|•B);    LetVariableDeclaration
	     A•|•B      UnionPattern                                                                                                          */
    let (A | B,);                                                                                                                         /*
    let•(A•|•B,);    LetVariableDeclaration
        (A•|•B,)     TuplePattern
         A•|•B       UnionPattern                                                                                                         */
	let _ = |(A | B): u8| ();                                                                                                             /*
	let•_•=•|(A•|•B):•u8|•();    LetVariableDeclaration
	    _                        WildcardPattern
	        |(A•|•B):•u8|•()     ClosureFunctionExpression
	        |(A•|•B):•u8|        ClosureFunctionExpression.parameters{dk: "||"}
	         (A•|•B):•u8         ClosureFunctionParameterDeclaration
	          A•|•B              UnionPattern
	                      ()     TupleLiteral                                                                                             */
	let (A | B);                                                                                                                          /*
	let•(A•|•B);    LetVariableDeclaration
	     A•|•B      UnionPattern                                                                                                          */
    let (A | B,);                                                                                                                         /*
    let•(A•|•B,);    LetVariableDeclaration
        (A•|•B,)     TuplePattern
         A•|•B       UnionPattern                                                                                                         */
	let A(B | C);                                                                                                                         /*
	let•A(B•|•C);    LetVariableDeclaration
	    A(B•|•C)     TuplePattern
	     (B•|•C)     TuplePattern.items{dk: "()"}
	      B•|•C      UnionPattern                                                                                                         */
    let E::V(B | C);                                                                                                                      /*
    let•E::V(B•|•C);    LetVariableDeclaration
        E::V(B•|•C)     TuplePattern
        E::V            ExpressionPath
            (B•|•C)     TuplePattern.items{dk: "()"}
             B•|•C      UnionPattern                                                                                                      */
	let S { f1: B | C, f2 };                                                                                                              /*
	let•S•{•f1:•B•|•C,•f2•};    LetVariableDeclaration
	    S•{•f1:•B•|•C,•f2•}     StructPattern
	      {•f1:•B•|•C,•f2•}     StructPattern.properties{dk: "{}"}
	        f1:•B•|•C           StructPatternPropertyDestructured
	            B•|•C           UnionPattern
	                   f2       StructPatternPropertyShorthand{!box, !ref, !mut}                                                          */
    let E::V { f1: B | C, f2 };                                                                                                           /*
    let•E::V•{•f1:•B•|•C,•f2•};    LetVariableDeclaration
        E::V•{•f1:•B•|•C,•f2•}     StructPattern
        E::V                       ExpressionPath
             {•f1:•B•|•C,•f2•}     StructPattern.properties{dk: "{}"}
               f1:•B•|•C           StructPatternPropertyDestructured
                   B•|•C           UnionPattern
                          f2       StructPatternPropertyShorthand{!box, !ref, !mut}                                                       */
	let [A | B, .. | ..];                                                                                                                 /*
	let•[A•|•B,•..•|•..];    LetVariableDeclaration
	    [A•|•B,•..•|•..]     ArrayPattern
	     A•|•B               UnionPattern
	            ..•|•..      UnionPattern
	            ..           RangePattern{!last}
	                 ..      RestPattern                                                                                                  */
	let (box 0 | 1);                                                                                                                      /*
	let•(box•0•|•1);    LetVariableDeclaration
	     box•0•|•1      UnionPattern
	     box•0          BoxPattern
	         0          Literal{kind: Integer}
	             1      Literal{kind: Integer}                                                                                            */
    let (&0 | 1);                                                                                                                         /*
    let•(&0•|•1);    LetVariableDeclaration
         &0•|•1      UnionPattern
         &0          ReferencePattern{!mut}
          0          Literal{kind: Integer}
              1      Literal{kind: Integer}                                                                                               */
    let (&mut 0 | 1);                                                                                                                     /*
    let•(&mut•0•|•1);    LetVariableDeclaration
         &mut•0•|•1      UnionPattern
         &mut•0          ReferencePattern{mut}
              0          Literal{kind: Integer}
                  1      Literal{kind: Integer}                                                                                           */
    let (x @ 0 | 1);                                                                                                                      /*
    let•(x•@•0•|•1);    LetVariableDeclaration
         x•@•0•|•1      UnionPattern
         x•@•0          PatternVariableDeclaration{!ref, !mut}
             0          Literal{kind: Integer}
                 1      Literal{kind: Integer}                                                                                            */
    let (ref x @ 0 | 1);                                                                                                                  /*
    let•(ref•x•@•0•|•1);    LetVariableDeclaration
         ref•x•@•0•|•1      UnionPattern
         ref•x•@•0          PatternVariableDeclaration{ref, !mut}
                 0          Literal{kind: Integer}
                     1      Literal{kind: Integer}                                                                                        */
    let (ref mut x @ 0 | 1);                                                                                                              /*
    let•(ref•mut•x•@•0•|•1);    LetVariableDeclaration
         ref•mut•x•@•0•|•1      UnionPattern
         ref•mut•x•@•0          PatternVariableDeclaration{ref, mut}
                     0          Literal{kind: Integer}
                         1      Literal{kind: Integer}                                                                                    */
	
    let (a, A(a, _) | B(a)) = (0, A(1, 2));                                                                                               /*
    let•(a,•A(a,•_)•|•B(a))•=•(0,•A(1,•2));    LetVariableDeclaration
        (a,•A(a,•_)•|•B(a))                    TuplePattern
            A(a,•_)•|•B(a)                     UnionPattern
            A(a,•_)                            TuplePattern
             (a,•_)                            TuplePattern.items{dk: "()"}
                 _                             WildcardPattern
                      B(a)                     TuplePattern
                       (a)                     TuplePattern.items{dk: "()"}
                              (0,•A(1,•2))     TupleLiteral
                               0               Literal{kind: Integer}
                                  A(1,•2)      CallExpression
                                   (1,•2)      CallExpression.arguments{dk: "()"}
                                    1          Literal{kind: Integer}
                                       2       Literal{kind: Integer}                                                                     */
    let (A(a, _) | B(a), a) = (A(0, 1), 2);                                                                                               /*
    let•(A(a,•_)•|•B(a),•a)•=•(A(0,•1),•2);    LetVariableDeclaration
        (A(a,•_)•|•B(a),•a)                    TuplePattern
         A(a,•_)•|•B(a)                        UnionPattern
         A(a,•_)                               TuplePattern
          (a,•_)                               TuplePattern.items{dk: "()"}
              _                                WildcardPattern
                   B(a)                        TuplePattern
                    (a)                        TuplePattern.items{dk: "()"}
                              (A(0,•1),•2)     TupleLiteral
                               A(0,•1)         CallExpression
                                (0,•1)         CallExpression.arguments{dk: "()"}
                                 0             Literal{kind: Integer}
                                    1          Literal{kind: Integer}
                                        2      Literal{kind: Integer}                                                                     */
    let (A(a, a) | B(a)) = A(0, 1);                                                                                                       /*
    let•(A(a,•a)•|•B(a))•=•A(0,•1);    LetVariableDeclaration
         A(a,•a)•|•B(a)                UnionPattern
         A(a,•a)                       TuplePattern
          (a,•a)                       TuplePattern.items{dk: "()"}
                   B(a)                TuplePattern
                    (a)                TuplePattern.items{dk: "()"}
                           A(0,•1)     CallExpression
                            (0,•1)     CallExpression.arguments{dk: "()"}
                             0         Literal{kind: Integer}
                                1      Literal{kind: Integer}                                                                             */
    let (B(a) | A(a, a)) = A(0, 1);                                                                                                       /*
    let•(B(a)•|•A(a,•a))•=•A(0,•1);    LetVariableDeclaration
         B(a)•|•A(a,•a)                UnionPattern
         B(a)                          TuplePattern
          (a)                          TuplePattern.items{dk: "()"}
                A(a,•a)                TuplePattern
                 (a,•a)                TuplePattern.items{dk: "()"}
                           A(0,•1)     CallExpression
                            (0,•1)     CallExpression.arguments{dk: "()"}
                             0         Literal{kind: Integer}
                                1      Literal{kind: Integer}                                                                             */
    match A(0, 1) {                                                                                                                       /*
    match•A(0,•1)•{↲    <ExpressionStatement{!semi}>
    match•A(0,•1)•{↲    <MatchExpression>
          A(0,•1)       CallExpression
           (0,•1)       CallExpression.arguments{dk: "()"}
            0           Literal{kind: Integer}
               1        Literal{kind: Integer}
                  {↲    <MatchExpression.cases{dk: "{}"}>                                                                                 */
        | A | B => 0,                                                                                                                     /*
        |•A•|•B•=>•0    MatchExpressionCase
        |•A•|•B         UnionPattern
                   0    Literal{kind: Integer}                                                                                            */
        A | B => 0,                                                                                                                       /*
        A•|•B•=>•0    MatchExpressionCase
        A•|•B         UnionPattern
                 0    Literal{kind: Integer}                                                                                              */
        B(a) | A(a, a) => 0,                                                                                                              /*
        B(a)•|•A(a,•a)•=>•0    MatchExpressionCase
        B(a)•|•A(a,•a)         UnionPattern
        B(a)                   TuplePattern
         (a)                   TuplePattern.items{dk: "()"}
               A(a,•a)         TuplePattern
                (a,•a)         TuplePattern.items{dk: "()"}
                          0    Literal{kind: Integer}                                                                                     */
		Ok(x @ 4) | Err(x @ (6 | 8)) => 0,                                                                                                /*
		Ok(x•@•4)•|•Err(x•@•(6•|•8))•=>•0    MatchExpressionCase
		Ok(x•@•4)•|•Err(x•@•(6•|•8))         UnionPattern
		Ok(x•@•4)                            TuplePattern
		  (x•@•4)                            TuplePattern.items{dk: "()"}
		   x•@•4                             PatternVariableDeclaration{!ref, !mut}
		       4                             Literal{kind: Integer}
		            Err(x•@•(6•|•8))         TuplePattern
		               (x•@•(6•|•8))         TuplePattern.items{dk: "()"}
		                x•@•(6•|•8)          PatternVariableDeclaration{!ref, !mut}
		                     6•|•8           UnionPattern
		                     6               Literal{kind: Integer}
		                         8           Literal{kind: Integer}
		                                0    Literal{kind: Integer}                                                                       */
        Ok(x @ 1 | x @ 2) => 0,                                                                                                           /*
        Ok(x•@•1•|•x•@•2)•=>•0    MatchExpressionCase
        Ok(x•@•1•|•x•@•2)         TuplePattern
          (x•@•1•|•x•@•2)         TuplePattern.items{dk: "()"}
           x•@•1•|•x•@•2          UnionPattern
           x•@•1                  PatternVariableDeclaration{!ref, !mut}
               1                  Literal{kind: Integer}
                   x•@•2          PatternVariableDeclaration{!ref, !mut}
                       2          Literal{kind: Integer}
                             0    Literal{kind: Integer}                                                                                  */
        Err(x @ (0..=10 | 30..=40)) if x % 2 == 0 => 0,                                                                                   /*
        Err(x•@•(0..=10•|•30..=40))•if•x•%•2•==•0•=>•0    MatchExpressionCase
        Err(x•@•(0..=10•|•30..=40))                       TuplePattern
           (x•@•(0..=10•|•30..=40))                       TuplePattern.items{dk: "()"}
            x•@•(0..=10•|•30..=40)                        PatternVariableDeclaration{!ref, !mut}
                 0..=10•|•30..=40                         UnionPattern
                 0..=10                                   RangePattern{last}
                 0                                        Literal{kind: Integer}
                     10                                   Literal{kind: Integer}
                          30..=40                         RangePattern{last}
                          30                              Literal{kind: Integer}
                               40                         Literal{kind: Integer}
                                       x•%•2•==•0         ComparisonExpression{tk: "=="}
                                       x•%•2              OperationExpression{tk: "%"}
                                           2              Literal{kind: Integer}
                                                0         Literal{kind: Integer}
                                                     0    Literal{kind: Integer}                                                          */
        Err(x @ 0..=40) => 0,                                                                                                             /*
        Err(x•@•0..=40)•=>•0    MatchExpressionCase
        Err(x•@•0..=40)         TuplePattern
           (x•@•0..=40)         TuplePattern.items{dk: "()"}
            x•@•0..=40          PatternVariableDeclaration{!ref, !mut}
                0..=40          RangePattern{last}
                0               Literal{kind: Integer}
                    40          Literal{kind: Integer}
                           0    Literal{kind: Integer}                                                                                    */
		Some(box Test::Foo | box Test::Bar) => 0,                                                                                         /*
		Some(box•Test::Foo•|•box•Test::Bar)•=>•0    MatchExpressionCase
		Some(box•Test::Foo•|•box•Test::Bar)         TuplePattern
		    (box•Test::Foo•|•box•Test::Bar)         TuplePattern.items{dk: "()"}
		     box•Test::Foo•|•box•Test::Bar          UnionPattern
		     box•Test::Foo                          BoxPattern
		         Test::Foo                          ExpressionPath
		                     box•Test::Bar          BoxPattern
		                         Test::Bar          ExpressionPath
		                                       0    Literal{kind: Integer}                                                                */
		&((true, y) | (y, true), z @ (0 | 4)) => (y as u8) + z,                                                                           /*
		&((true,•y)•|•(y,•true),•z•@•(0•|•4))•=>•(y•as•u8)•+•z    MatchExpressionCase
		&((true,•y)•|•(y,•true),•z•@•(0•|•4))                     ReferencePattern{!mut}
		 ((true,•y)•|•(y,•true),•z•@•(0•|•4))                     TuplePattern
		  (true,•y)•|•(y,•true)                                   UnionPattern
		  (true,•y)                                               TuplePattern
		   true                                                   Literal{kind: True}
		              (y,•true)                                   TuplePattern
		                  true                                    Literal{kind: True}
		                         z•@•(0•|•4)                      PatternVariableDeclaration{!ref, !mut}
		                              0•|•4                       UnionPattern
		                              0                           Literal{kind: Integer}
		                                  4                       Literal{kind: Integer}
		                                         (y•as•u8)•+•z    OperationExpression{tk: "+"}
		                                          y•as•u8         ExpressionAsTypeCast                                                    */
        Foo::One(0) | Foo::One(1) | Foo::One(2) => 0,                                                                                     /*
        Foo::One(0)•|•Foo::One(1)•|•Foo::One(2)•=>•0    MatchExpressionCase
        Foo::One(0)•|•Foo::One(1)•|•Foo::One(2)         UnionPattern
        Foo::One(0)                                     TuplePattern
        Foo::One                                        ExpressionPath
                (0)                                     TuplePattern.items{dk: "()"}
                 0                                      Literal{kind: Integer}
                      Foo::One(1)                       TuplePattern
                      Foo::One                          ExpressionPath
                              (1)                       TuplePattern.items{dk: "()"}
                               1                        Literal{kind: Integer}
                                    Foo::One(2)         TuplePattern
                                    Foo::One            ExpressionPath
                                            (2)         TuplePattern.items{dk: "()"}
                                             2          Literal{kind: Integer}
                                                   0    Literal{kind: Integer}                                                            */
        Foo::One(42 | 255) => 0,                                                                                                          /*
        Foo::One(42•|•255)•=>•0    MatchExpressionCase
        Foo::One(42•|•255)         TuplePattern
        Foo::One                   ExpressionPath
                (42•|•255)         TuplePattern.items{dk: "()"}
                 42•|•255          UnionPattern
                 42                Literal{kind: Integer}
                      255          Literal{kind: Integer}
                              0    Literal{kind: Integer}                                                                                 */
        Foo::Two(42 | 255, 1024 | 2048) => 0,                                                                                             /*
        Foo::Two(42•|•255,•1024•|•2048)•=>•0    MatchExpressionCase
        Foo::Two(42•|•255,•1024•|•2048)         TuplePattern
        Foo::Two                                ExpressionPath
                (42•|•255,•1024•|•2048)         TuplePattern.items{dk: "()"}
                 42•|•255                       UnionPattern
                 42                             Literal{kind: Integer}
                      255                       Literal{kind: Integer}
                           1024•|•2048          UnionPattern
                           1024                 Literal{kind: Integer}
                                  2048          Literal{kind: Integer}
                                           0    Literal{kind: Integer}                                                                    */
        Foo::One(100 | 110..=120 | 210..=220) => 0,                                                                                       /*
        Foo::One(100•|•110..=120•|•210..=220)•=>•0    MatchExpressionCase
        Foo::One(100•|•110..=120•|•210..=220)         TuplePattern
        Foo::One                                      ExpressionPath
                (100•|•110..=120•|•210..=220)         TuplePattern.items{dk: "()"}
                 100•|•110..=120•|•210..=220          UnionPattern
                 100                                  Literal{kind: Integer}
                       110..=120                      RangePattern{last}
                       110                            Literal{kind: Integer}
                             120                      Literal{kind: Integer}
                                   210..=220          RangePattern{last}
                                   210                Literal{kind: Integer}
                                         220          Literal{kind: Integer}
                                                 0    Literal{kind: Integer}                                                              */
        Foo::Two(0..=10 | 100..=110, 0 | _) => 0,                                                                                         /*
        Foo::Two(0..=10•|•100..=110,•0•|•_)•=>•0    MatchExpressionCase
        Foo::Two(0..=10•|•100..=110,•0•|•_)         TuplePattern
        Foo::Two                                    ExpressionPath
                (0..=10•|•100..=110,•0•|•_)         TuplePattern.items{dk: "()"}
                 0..=10•|•100..=110                 UnionPattern
                 0..=10                             RangePattern{last}
                 0                                  Literal{kind: Integer}
                     10                             Literal{kind: Integer}
                          100..=110                 RangePattern{last}
                          100                       Literal{kind: Integer}
                                110                 Literal{kind: Integer}
                                     0•|•_          UnionPattern
                                     0              Literal{kind: Integer}
                                         _          WildcardPattern
                                               0    Literal{kind: Integer}                                                                */
		([] | [0 | 1..=255] | [_, ..],) => 0,                                                                                             /*
		([]•|•[0•|•1..=255]•|•[_,•..],)•=>•0    MatchExpressionCase
		([]•|•[0•|•1..=255]•|•[_,•..],)         TuplePattern
		 []•|•[0•|•1..=255]•|•[_,•..]           UnionPattern
		 []                                     ArrayPattern
		      [0•|•1..=255]                     ArrayPattern
		       0•|•1..=255                      UnionPattern
		       0                                Literal{kind: Integer}
		           1..=255                      RangePattern{last}
		           1                            Literal{kind: Integer}
		               255                      Literal{kind: Integer}
		                      [_,•..]           ArrayPattern
		                       _                WildcardPattern
		                          ..            RestPattern
		                                   0    Literal{kind: Integer}                                                                    */
		((0, 0) | (0, 1),) => 0,                                                                                                          /*
		((0,•0)•|•(0,•1),)•=>•0    MatchExpressionCase
		((0,•0)•|•(0,•1),)         TuplePattern
		 (0,•0)•|•(0,•1)           UnionPattern
		 (0,•0)                    TuplePattern
		  0                        Literal{kind: Integer}
		     0                     Literal{kind: Integer}
		          (0,•1)           TuplePattern
		           0               Literal{kind: Integer}
		              1            Literal{kind: Integer}
		                      0    Literal{kind: Integer}                                                                                 */
		(a,_) | (_,a) if a > 10 => 0,                                                                                                     /*
		(a,_)•|•(_,a)•if•a•>•10•=>•0    MatchExpressionCase
		(a,_)•|•(_,a)                   UnionPattern
		(a,_)                           TuplePattern
		   _                            WildcardPattern
		        (_,a)                   TuplePattern
		         _                      WildcardPattern
		                 a•>•10         ComparisonExpression{tk: ">"}
		                     10         Literal{kind: Integer}
		                           0    Literal{kind: Integer}                                                                            */
		Some((a, _)) | Some((_, a)) if a > 10 => 0,                                                                                       /*
		Some((a,•_))•|•Some((_,•a))•if•a•>•10•=>•0    MatchExpressionCase
		Some((a,•_))•|•Some((_,•a))                   UnionPattern
		Some((a,•_))                                  TuplePattern
		    ((a,•_))                                  TuplePattern.items{dk: "()"}
		     (a,•_)                                   TuplePattern
		         _                                    WildcardPattern
		               Some((_,•a))                   TuplePattern
		                   ((_,•a))                   TuplePattern.items{dk: "()"}
		                    (_,•a)                    TuplePattern
		                     _                        WildcardPattern
		                               a•>•10         ComparisonExpression{tk: ">"}
		                                   10         Literal{kind: Integer}
		                                         0    Literal{kind: Integer}                                                              */
		Some((a, _) | (_, a)) if a > 10 => 0,                                                                                             /*
		Some((a,•_)•|•(_,•a))•if•a•>•10•=>•0    MatchExpressionCase
		Some((a,•_)•|•(_,•a))                   TuplePattern
		    ((a,•_)•|•(_,•a))                   TuplePattern.items{dk: "()"}
		     (a,•_)•|•(_,•a)                    UnionPattern
		     (a,•_)                             TuplePattern
		         _                              WildcardPattern
		              (_,•a)                    TuplePattern
		               _                        WildcardPattern
		                         a•>•10         ComparisonExpression{tk: ">"}
		                             10         Literal{kind: Integer}
		                                   0    Literal{kind: Integer}                                                                    */
		e @ &(1..=2) | e @ &(3..=4) => 0,                                                                                                 /*
		e•@•&(1..=2)•|•e•@•&(3..=4)•=>•0    MatchExpressionCase
		e•@•&(1..=2)•|•e•@•&(3..=4)         UnionPattern
		e•@•&(1..=2)                        PatternVariableDeclaration{!ref, !mut}
		    &(1..=2)                        ReferencePattern{!mut}
		      1..=2                         RangePattern{last}
		      1                             Literal{kind: Integer}
		          2                         Literal{kind: Integer}
		               e•@•&(3..=4)         PatternVariableDeclaration{!ref, !mut}
		                   &(3..=4)         ReferencePattern{!mut}
		                     3..=4          RangePattern{last}
		                     3              Literal{kind: Integer}
		                         4          Literal{kind: Integer}
		                               0    Literal{kind: Integer}                                                                        */
		Ok(mut x) | &Err(mut x) => 0,                                                                                                     /*
		Ok(mut•x)•|•&Err(mut•x)•=>•0    MatchExpressionCase
		Ok(mut•x)•|•&Err(mut•x)         UnionPattern
		Ok(mut•x)                       TuplePattern
		  (mut•x)                       TuplePattern.items{dk: "()"}
		   mut•x                        PatternVariableDeclaration{!ref, mut}
		            &Err(mut•x)         ReferencePattern{!mut}
		             Err(mut•x)         TuplePattern
		                (mut•x)         TuplePattern.items{dk: "()"}
		                 mut•x          PatternVariableDeclaration{!ref, mut}
		                           0    Literal{kind: Integer}                                                                            */
		Tri::A(Ok(mut x) | Err(mut x))                                                                                                    /*
		Tri::A(Ok(mut•x)•|•Err(mut•x))↲    <MatchExpressionCase>
		Tri::A(Ok(mut•x)•|•Err(mut•x))↲    <UnionPattern>
		Tri::A(Ok(mut•x)•|•Err(mut•x))     TuplePattern
		Tri::A                             ExpressionPath
		      (Ok(mut•x)•|•Err(mut•x))     TuplePattern.items{dk: "()"}
		       Ok(mut•x)•|•Err(mut•x)      UnionPattern
		       Ok(mut•x)                   TuplePattern
		         (mut•x)                   TuplePattern.items{dk: "()"}
		          mut•x                    PatternVariableDeclaration{!ref, mut}
		                   Err(mut•x)      TuplePattern
		                      (mut•x)      TuplePattern.items{dk: "()"}
		                       mut•x       PatternVariableDeclaration{!ref, mut}                                                          */
        | Tri::B(&Ok(mut x) | Err(mut x))                                                                                                 /*
          Tri::B(&Ok(mut•x)•|•Err(mut•x))    TuplePattern
          Tri::B                             ExpressionPath
                (&Ok(mut•x)•|•Err(mut•x))    TuplePattern.items{dk: "()"}
                 &Ok(mut•x)•|•Err(mut•x)     UnionPattern
                 &Ok(mut•x)                  ReferencePattern{!mut}
                  Ok(mut•x)                  TuplePattern
                    (mut•x)                  TuplePattern.items{dk: "()"}
                     mut•x                   PatternVariableDeclaration{!ref, mut}
                              Err(mut•x)     TuplePattern
                                 (mut•x)     TuplePattern.items{dk: "()"}
                                  mut•x      PatternVariableDeclaration{!ref, mut}                                                        */
        | &Tri::C(Ok(mut x) | Err(mut x)) => 0,                                                                                           /*
          &Tri::C(Ok(mut•x)•|•Err(mut•x))         ReferencePattern{!mut}
           Tri::C(Ok(mut•x)•|•Err(mut•x))         TuplePattern
           Tri::C                                 ExpressionPath
                 (Ok(mut•x)•|•Err(mut•x))         TuplePattern.items{dk: "()"}
                  Ok(mut•x)•|•Err(mut•x)          UnionPattern
                  Ok(mut•x)                       TuplePattern
                    (mut•x)                       TuplePattern.items{dk: "()"}
                     mut•x                        PatternVariableDeclaration{!ref, mut}
                              Err(mut•x)          TuplePattern
                                 (mut•x)          TuplePattern.items{dk: "()"}
                                  mut•x           PatternVariableDeclaration{!ref, mut}
••••••••|•&Tri::C(Ok(mut•x)•|•Err(mut•x))         </UnionPattern>
                                             0    Literal{kind: Integer}
••••••••|•&Tri::C(Ok(mut•x)•|•Err(mut•x))•=>•0    </MatchExpressionCase>                                                                  */
        | A | B => 0,                                                                                                                     /*
        |•A•|•B•=>•0    MatchExpressionCase
        |•A•|•B         UnionPattern
                   0    Literal{kind: Integer}                                                                                            */
        A | B => 0,                                                                                                                       /*
        A•|•B•=>•0    MatchExpressionCase
        A•|•B         UnionPattern
                 0    Literal{kind: Integer}                                                                                              */
		[.., Some(Test::Qux | Test::Foo)] => 0,                                                                                           /*
		[..,•Some(Test::Qux•|•Test::Foo)]•=>•0    MatchExpressionCase
		[..,•Some(Test::Qux•|•Test::Foo)]         ArrayPattern
		 ..                                       RestPattern
		     Some(Test::Qux•|•Test::Foo)          TuplePattern
		         (Test::Qux•|•Test::Foo)          TuplePattern.items{dk: "()"}
		          Test::Qux•|•Test::Foo           UnionPattern
		          Test::Qux                       ExpressionPath
		                      Test::Foo           ExpressionPath
		                                     0    Literal{kind: Integer}                                                                  */
        [Some(Test::Foo), .., Some(Test::Baz | Test::Bar)] => 0,                                                                          /*
        [Some(Test::Foo),•..,•Some(Test::Baz•|•Test::Bar)]•=>•0    MatchExpressionCase
        [Some(Test::Foo),•..,•Some(Test::Baz•|•Test::Bar)]         ArrayPattern
         Some(Test::Foo)                                           TuplePattern
             (Test::Foo)                                           TuplePattern.items{dk: "()"}
              Test::Foo                                            ExpressionPath
                          ..                                       RestPattern
                              Some(Test::Baz•|•Test::Bar)          TuplePattern
                                  (Test::Baz•|•Test::Bar)          TuplePattern.items{dk: "()"}
                                   Test::Baz•|•Test::Bar           UnionPattern
                                   Test::Baz                       ExpressionPath
                                               Test::Bar           ExpressionPath
                                                              0    Literal{kind: Integer}                                                 */
        [.., Some(Test::Bar | Test::Baz), _] => 0,                                                                                        /*
        [..,•Some(Test::Bar•|•Test::Baz),•_]•=>•0    MatchExpressionCase
        [..,•Some(Test::Bar•|•Test::Baz),•_]         ArrayPattern
         ..                                          RestPattern
             Some(Test::Bar•|•Test::Baz)             TuplePattern
                 (Test::Bar•|•Test::Baz)             TuplePattern.items{dk: "()"}
                  Test::Bar•|•Test::Baz              UnionPattern
                  Test::Bar                          ExpressionPath
                              Test::Baz              ExpressionPath
                                          _          WildcardPattern
                                                0    Literal{kind: Integer}                                                               */
		Some(                                                                                                                             /*
		Some(↲    <MatchExpressionCase>
		Some(↲    <TuplePattern>
		    (↲    <TuplePattern.items{dk: "()"}>                                                                                          */
            Test::Foo { first: 1024 | 2048, second: 2048 | 4096 }                                                                         /*
            Test::Foo•{•first:•1024•|•2048,•second:•2048•|•4096•}↲    <UnionPattern>
            Test::Foo•{•first:•1024•|•2048,•second:•2048•|•4096•}     StructPattern
            Test::Foo                                                 ExpressionPath
                      {•first:•1024•|•2048,•second:•2048•|•4096•}     StructPattern.properties{dk: "{}"}
                        first:•1024•|•2048                            StructPatternPropertyDestructured
                               1024•|•2048                            UnionPattern
                               1024                                   Literal{kind: Integer}
                                      2048                            Literal{kind: Integer}
                                            second:•2048•|•4096       StructPatternPropertyDestructured
                                                    2048•|•4096       UnionPattern
                                                    2048              Literal{kind: Integer}
                                                           4096       Literal{kind: Integer}                                              */
            | Test::Bar { other: Some(Other::One | Other::Two) },                                                                         /*
              Test::Bar•{•other:•Some(Other::One•|•Other::Two)•}    StructPattern
              Test::Bar                                             ExpressionPath
                        {•other:•Some(Other::One•|•Other::Two)•}    StructPattern.properties{dk: "{}"}
                          other:•Some(Other::One•|•Other::Two)      StructPatternPropertyDestructured
                                 Some(Other::One•|•Other::Two)      TuplePattern
                                     (Other::One•|•Other::Two)      TuplePattern.items{dk: "()"}
                                      Other::One•|•Other::Two       UnionPattern
                                      Other::One                    ExpressionPath
                                                   Other::Two       ExpressionPath
••••••••••••|•Test::Bar•{•other:•Some(Other::One•|•Other::Two)•}    </UnionPattern>                                                       */
        ) => 0,                                                                                                                           /*
••••••••)         </TuplePattern.items>
••••••••)         </TuplePattern>
             0    Literal{kind: Integer}
••••••••)•=>•0    </MatchExpressionCase>                                                                                                  */
		((a, _) | (_, a), (b @ _, _) | (_, b @ _), (c @ false, _) | (_, c @ true))                                                        /*
		((a,•_)•|•(_,•a),•(b•@•_,•_)•|•(_,•b•@•_),•(c•@•false,•_)•|•(_,•c•@•true))↲    <MatchExpressionCase>
		((a,•_)•|•(_,•a),•(b•@•_,•_)•|•(_,•b•@•_),•(c•@•false,•_)•|•(_,•c•@•true))     TuplePattern
		 (a,•_)•|•(_,•a)                                                               UnionPattern
		 (a,•_)                                                                        TuplePattern
		     _                                                                         WildcardPattern
		          (_,•a)                                                               TuplePattern
		           _                                                                   WildcardPattern
		                  (b•@•_,•_)•|•(_,•b•@•_)                                      UnionPattern
		                  (b•@•_,•_)                                                   TuplePattern
		                   b•@•_                                                       PatternVariableDeclaration{!ref, !mut}
		                       _                                                       WildcardPattern
		                          _                                                    WildcardPattern
		                               (_,•b•@•_)                                      TuplePattern
		                                _                                              WildcardPattern
		                                   b•@•_                                       PatternVariableDeclaration{!ref, !mut}
		                                       _                                       WildcardPattern
		                                           (c•@•false,•_)•|•(_,•c•@•true)      UnionPattern
		                                           (c•@•false,•_)                      TuplePattern
		                                            c•@•false                          PatternVariableDeclaration{!ref, !mut}
		                                                false                          Literal{kind: False}
		                                                       _                       WildcardPattern
		                                                            (_,•c•@•true)      TuplePattern
		                                                             _                 WildcardPattern
		                                                                c•@•true       PatternVariableDeclaration{!ref, !mut}
		                                                                    true       Literal{kind: True}                                */
            if {                                                                                                                          /*
               {↲    <BlockExpression>                                                                                                    */
                guard_count += 1;                                                                                                         /*
                guard_count•+=•1;    ExpressionStatement{semi}
                guard_count•+=•1     ReassignmentOperationExpression{tk: "+="}
                               1     Literal{kind: Integer}                                                                               */
                (a, b, c) == target                                                                                                       /*
                (a,•b,•c)•==•target    ExpressionStatement{!semi}, ComparisonExpression{tk: "=="}
                (a,•b,•c)              TupleLiteral                                                                                       */
            } => 0,                                                                                                                       /*
••••••••••••}         </BlockExpression>
                 0    Literal{kind: Integer}
••••••••••••}•=>•0    </MatchExpressionCase>                                                                                              */
		((a, _), (b @ _, _), (c @ false, _))                                                                                              /*
		((a,•_),•(b•@•_,•_),•(c•@•false,•_))↲    <MatchExpressionCase>
		((a,•_),•(b•@•_,•_),•(c•@•false,•_))↲    <UnionPattern>
		((a,•_),•(b•@•_,•_),•(c•@•false,•_))     TuplePattern
		 (a,•_)                                  TuplePattern
		     _                                   WildcardPattern
		         (b•@•_,•_)                      TuplePattern
		          b•@•_                          PatternVariableDeclaration{!ref, !mut}
		              _                          WildcardPattern
		                 _                       WildcardPattern
		                     (c•@•false,•_)      TuplePattern
		                      c•@•false          PatternVariableDeclaration{!ref, !mut}
		                          false          Literal{kind: False}
		                                 _       WildcardPattern                                                                          */
		| ((a, _), (b @ _, _), (_, c @ true))                                                                                             /*
		  ((a,•_),•(b•@•_,•_),•(_,•c•@•true))    TuplePattern
		   (a,•_)                                TuplePattern
		       _                                 WildcardPattern
		           (b•@•_,•_)                    TuplePattern
		            b•@•_                        PatternVariableDeclaration{!ref, !mut}
		                _                        WildcardPattern
		                   _                     WildcardPattern
		                       (_,•c•@•true)     TuplePattern
		                        _                WildcardPattern
		                           c•@•true      PatternVariableDeclaration{!ref, !mut}
		                               true      Literal{kind: True}                                                                      */
		| ((a, _), (_, b @ _), (c @ false, _))                                                                                            /*
		  ((a,•_),•(_,•b•@•_),•(c•@•false,•_))    TuplePattern
		   (a,•_)                                 TuplePattern
		       _                                  WildcardPattern
		           (_,•b•@•_)                     TuplePattern
		            _                             WildcardPattern
		               b•@•_                      PatternVariableDeclaration{!ref, !mut}
		                   _                      WildcardPattern
		                       (c•@•false,•_)     TuplePattern
		                        c•@•false         PatternVariableDeclaration{!ref, !mut}
		                            false         Literal{kind: False}
		                                   _      WildcardPattern                                                                         */
		| ((a, _), (_, b @ _), (_, c @ true))                                                                                             /*
		  ((a,•_),•(_,•b•@•_),•(_,•c•@•true))    TuplePattern
		   (a,•_)                                TuplePattern
		       _                                 WildcardPattern
		           (_,•b•@•_)                    TuplePattern
		            _                            WildcardPattern
		               b•@•_                     PatternVariableDeclaration{!ref, !mut}
		                   _                     WildcardPattern
		                       (_,•c•@•true)     TuplePattern
		                        _                WildcardPattern
		                           c•@•true      PatternVariableDeclaration{!ref, !mut}
		                               true      Literal{kind: True}                                                                      */
		| ((_, a), (b @ _, _), (c @ false, _))                                                                                            /*
		  ((_,•a),•(b•@•_,•_),•(c•@•false,•_))    TuplePattern
		   (_,•a)                                 TuplePattern
		    _                                     WildcardPattern
		           (b•@•_,•_)                     TuplePattern
		            b•@•_                         PatternVariableDeclaration{!ref, !mut}
		                _                         WildcardPattern
		                   _                      WildcardPattern
		                       (c•@•false,•_)     TuplePattern
		                        c•@•false         PatternVariableDeclaration{!ref, !mut}
		                            false         Literal{kind: False}
		                                   _      WildcardPattern                                                                         */
		| ((_, a), (b @ _, _), (_, c @ true))                                                                                             /*
		  ((_,•a),•(b•@•_,•_),•(_,•c•@•true))    TuplePattern
		   (_,•a)                                TuplePattern
		    _                                    WildcardPattern
		           (b•@•_,•_)                    TuplePattern
		            b•@•_                        PatternVariableDeclaration{!ref, !mut}
		                _                        WildcardPattern
		                   _                     WildcardPattern
		                       (_,•c•@•true)     TuplePattern
		                        _                WildcardPattern
		                           c•@•true      PatternVariableDeclaration{!ref, !mut}
		                               true      Literal{kind: True}                                                                      */
		| ((_, a), (_, b @ _), (c @ false, _))                                                                                            /*
		  ((_,•a),•(_,•b•@•_),•(c•@•false,•_))    TuplePattern
		   (_,•a)                                 TuplePattern
		    _                                     WildcardPattern
		           (_,•b•@•_)                     TuplePattern
		            _                             WildcardPattern
		               b•@•_                      PatternVariableDeclaration{!ref, !mut}
		                   _                      WildcardPattern
		                       (c•@•false,•_)     TuplePattern
		                        c•@•false         PatternVariableDeclaration{!ref, !mut}
		                            false         Literal{kind: False}
		                                   _      WildcardPattern                                                                         */
		| ((_, a), (_, b @ _), (_, c @ true))                                                                                             /*
		  ((_,•a),•(_,•b•@•_),•(_,•c•@•true))    TuplePattern
		   (_,•a)                                TuplePattern
		    _                                    WildcardPattern
		           (_,•b•@•_)                    TuplePattern
		            _                            WildcardPattern
		               b•@•_                     PatternVariableDeclaration{!ref, !mut}
		                   _                     WildcardPattern
		                       (_,•c•@•true)     TuplePattern
		                        _                WildcardPattern
		                           c•@•true      PatternVariableDeclaration{!ref, !mut}
		                               true      Literal{kind: True}
      ╚╚|•((_,•a),•(_,•b•@•_),•(_,•c•@•true))    </UnionPattern>                                                                          */
			if {                                                                                                                          /*
			   {↲    <BlockExpression>                                                                                                    */
				guard_count += 1;                                                                                                         /*
				guard_count•+=•1;    ExpressionStatement{semi}
				guard_count•+=•1     ReassignmentOperationExpression{tk: "+="}
				               1     Literal{kind: Integer}                                                                               */
				(a, b, c) == target                                                                                                       /*
				(a,•b,•c)•==•target    ExpressionStatement{!semi}, ComparisonExpression{tk: "=="}
				(a,•b,•c)              TupleLiteral                                                                                       */
			} => 0,                                                                                                                       /*
         ╚╚╚}         </BlockExpression>
			     0    Literal{kind: Integer}
         ╚╚╚}•=>•0    </MatchExpressionCase>                                                                                              */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

accept_pat!((p | q));                                                                                                                     /*
accept_pat!((p•|•q));    ExpressionStatement{semi}
accept_pat!((p•|•q))     MacroInvocation
           ((p•|•q))     MacroInvocation.segments{dk: "()"}
            (p•|•q)      DelimGroup
               |         PunctuationToken{tk: "|"}                                                                                        */
accept_pat!((p | q,));                                                                                                                    /*
accept_pat!((p•|•q,));    ExpressionStatement{semi}
accept_pat!((p•|•q,))     MacroInvocation
           ((p•|•q,))     MacroInvocation.segments{dk: "()"}
            (p•|•q,)      DelimGroup
               |          PunctuationToken{tk: "|"}
                  ,       PunctuationToken{tk: ","}                                                                                       */
accept_pat!(TS(p | q));                                                                                                                   /*
accept_pat!(TS(p•|•q));    ExpressionStatement{semi}
accept_pat!(TS(p•|•q))     MacroInvocation
           (TS(p•|•q))     MacroInvocation.segments{dk: "()"}
              (p•|•q)      DelimGroup
                 |         PunctuationToken{tk: "|"}                                                                                      */
accept_pat!(NS { f: p | q });                                                                                                             /*
accept_pat!(NS•{•f:•p•|•q•});    ExpressionStatement{semi}
accept_pat!(NS•{•f:•p•|•q•})     MacroInvocation
           (NS•{•f:•p•|•q•})     MacroInvocation.segments{dk: "()"}
               {•f:•p•|•q•}      DelimGroup
                  :              PunctuationToken{tk: ":"}
                      |          PunctuationToken{tk: "|"}                                                                                */
accept_pat!([p | q]);                                                                                                                     /*
accept_pat!([p•|•q]);    ExpressionStatement{semi}
accept_pat!([p•|•q])     MacroInvocation
           ([p•|•q])     MacroInvocation.segments{dk: "()"}
            [p•|•q]      DelimGroup
               |         PunctuationToken{tk: "|"}
accept_pat!([p•|•q]);    </Program.ast>
accept_pat!([p•|•q]);    </Program>                                                                                                       */
// Discarded Nodes: 54
// Parsed Nodes: 1660
// state_rollbacks: 1
// Total '.charCodeAt()' calls: 7082 (36% re-reads)
// Unnecessary 'skip_whitespace()' calls: 913
// source: "../../samples/patterns/union.rs"