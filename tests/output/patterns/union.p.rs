
fn fw1(H(Ok(mut x) | &Err(mut x)): H<R<'_>>) {}                                                                                           /*
fn•fw1(H(Ok(mut•x)•|•&Err(mut•x)):•H<R<'_>>)•{}    FunctionDeclaration
       H(Ok(mut•x)•|•&Err(mut•x)):•H<R<'_>>        FunctionParameterDeclaration
       H(Ok(mut•x)•|•&Err(mut•x))                  TuplePattern
         Ok(mut•x)•|•&Err(mut•x)                   UnionPattern
         Ok(mut•x)                                 TuplePattern
            mut•x                                  PatternVariableDeclaration
                     &Err(mut•x)                   ReferencePattern
                      Err(mut•x)                   TuplePattern
                          mut•x                    PatternVariableDeclaration
                                   H<R<'_>>        TypeCall
                                     R<'_>         TypeCall
                                       '_          LtElided                                                                               */
fn f1((Ok(mut x) | &Err(mut x)): R<'_>) {}                                                                                                /*
fn•f1((Ok(mut•x)•|•&Err(mut•x)):•R<'_>)•{}    FunctionDeclaration
      (Ok(mut•x)•|•&Err(mut•x)):•R<'_>        FunctionParameterDeclaration
       Ok(mut•x)•|•&Err(mut•x)                UnionPattern
       Ok(mut•x)                              TuplePattern
          mut•x                               PatternVariableDeclaration
                   &Err(mut•x)                ReferencePattern
                    Err(mut•x)                TuplePattern
                        mut•x                 PatternVariableDeclaration
                                 R<'_>        TypeCall
                                   '_         LtElided                                                                                    */
fn fw2(H(&(Ok(x) | Err(x))): H<R<'_>>) {}                                                                                                 /*
fn•fw2(H(&(Ok(x)•|•Err(x))):•H<R<'_>>)•{}    FunctionDeclaration
       H(&(Ok(x)•|•Err(x))):•H<R<'_>>        FunctionParameterDeclaration
       H(&(Ok(x)•|•Err(x)))                  TuplePattern
         &(Ok(x)•|•Err(x))                   ReferencePattern
           Ok(x)•|•Err(x)                    UnionPattern
           Ok(x)                             TuplePattern
                   Err(x)                    TuplePattern
                             H<R<'_>>        TypeCall
                               R<'_>         TypeCall
                                 '_          LtElided                                                                                     */
fn fw3(H(Ok(x) | Err(x)): H<R<'_>>) {}                                                                                                    /*
fn•fw3(H(Ok(x)•|•Err(x)):•H<R<'_>>)•{}    FunctionDeclaration
       H(Ok(x)•|•Err(x)):•H<R<'_>>        FunctionParameterDeclaration
       H(Ok(x)•|•Err(x))                  TuplePattern
         Ok(x)•|•Err(x)                   UnionPattern
         Ok(x)                            TuplePattern
                 Err(x)                   TuplePattern
                          H<R<'_>>        TypeCall
                            R<'_>         TypeCall
                              '_          LtElided                                                                                        */
fn f2(&(Ok(x) | Err(x)): R<'_>) {}                                                                                                        /*
fn•f2(&(Ok(x)•|•Err(x)):•R<'_>)•{}    FunctionDeclaration
      &(Ok(x)•|•Err(x)):•R<'_>        FunctionParameterDeclaration
      &(Ok(x)•|•Err(x))               ReferencePattern
        Ok(x)•|•Err(x)                UnionPattern
        Ok(x)                         TuplePattern
                Err(x)                TuplePattern
                         R<'_>        TypeCall
                           '_         LtElided                                                                                            */
fn f3((Ok(x) | Err(x)): R<'_>) {}                                                                                                         /*
fn•f3((Ok(x)•|•Err(x)):•R<'_>)•{}    FunctionDeclaration
      (Ok(x)•|•Err(x)):•R<'_>        FunctionParameterDeclaration
       Ok(x)•|•Err(x)                UnionPattern
       Ok(x)                         TuplePattern
               Err(x)                TuplePattern
                        R<'_>        TypeCall
                          '_         LtElided                                                                                             */
fn fun((A | B): _) {}                                                                                                                     /*
fn•fun((A•|•B):•_)•{}    FunctionDeclaration
       (A•|•B):•_        FunctionParameterDeclaration
        A•|•B            UnionPattern
                _        TypeInferred                                                                                                     */
fn f(x @ (A::R(_) | D::E(_)): Q) {}                                                                                                       /*
fn•f(x•@•(A::R(_)•|•D::E(_)):•Q)•{}    FunctionDeclaration
     x•@•(A::R(_)•|•D::E(_)):•Q        FunctionParameterDeclaration
     x•@•(A::R(_)•|•D::E(_))           PatternVariableDeclaration
          A::R(_)•|•D::E(_)            UnionPattern
          A::R(_)                      TuplePattern
          A::R                         ExpressionPath
               _                       WildcardPattern
                    D::E(_)            TuplePattern
                    D::E               ExpressionPath
                         _             WildcardPattern                                                                                    */

fn x() {                                                                                                                                  /*
fn•x()•{↲    <FunctionDeclaration>                                                                                                        */
	let (0 | (1 | _)) = 0;                                                                                                                /*
    let•(0•|•(1•|•_))•=•0;    LetVariableDeclaration
         0•|•(1•|•_)          UnionPattern
         0                    Literal
              1•|•_           UnionPattern
              1               Literal
                  _           WildcardPattern
                        0     Literal                                                                                                     */
    if let 0 | (1 | 2) = 0 {}                                                                                                             /*
    if•let•0•|•(1•|•2)•=•0•{}    ExpressionStatement, IfBlockExpression
       let•0•|•(1•|•2)•=•0       LetScrutinee
           0•|•(1•|•2)           UnionPattern
           0                     Literal
                1•|•2            UnionPattern
                1                Literal
                    2            Literal
                         0       Literal                                                                                                  */
    if let x @ 0 | x @ (1 | 2) = 0 {}                                                                                                     /*
    if•let•x•@•0•|•x•@•(1•|•2)•=•0•{}    ExpressionStatement, IfBlockExpression
       let•x•@•0•|•x•@•(1•|•2)•=•0       LetScrutinee
           x•@•0•|•x•@•(1•|•2)           UnionPattern
           x•@•0                         PatternVariableDeclaration
               0                         Literal
                   x•@•(1•|•2)           PatternVariableDeclaration
                        1•|•2            UnionPattern
                        1                Literal
                            2            Literal
                                 0       Literal                                                                                          */
    if let H(Ok(mut x) | &Err(mut x)) = a {                                                                                               /*
    if•let•H(Ok(mut•x)•|•&Err(mut•x))•=•a•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•H(Ok(mut•x)•|•&Err(mut•x))•=•a       LetScrutinee
           H(Ok(mut•x)•|•&Err(mut•x))           TuplePattern
             Ok(mut•x)•|•&Err(mut•x)            UnionPattern
             Ok(mut•x)                          TuplePattern
                mut•x                           PatternVariableDeclaration
                         &Err(mut•x)            ReferencePattern
                          Err(mut•x)            TuplePattern
                              mut•x             PatternVariableDeclaration                                                                */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */
    if let H(&(Ok(x) | Err(x))) = a {                                                                                                     /*
    if•let•H(&(Ok(x)•|•Err(x)))•=•a•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•H(&(Ok(x)•|•Err(x)))•=•a       LetScrutinee
           H(&(Ok(x)•|•Err(x)))           TuplePattern
             &(Ok(x)•|•Err(x))            ReferencePattern
               Ok(x)•|•Err(x)             UnionPattern
               Ok(x)                      TuplePattern
                       Err(x)             TuplePattern                                                                                    */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */
    if let H(Ok(x) | Err(x)) = a {                                                                                                        /*
    if•let•H(Ok(x)•|•Err(x))•=•a•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•H(Ok(x)•|•Err(x))•=•a       LetScrutinee
           H(Ok(x)•|•Err(x))           TuplePattern
             Ok(x)•|•Err(x)            UnionPattern
             Ok(x)                     TuplePattern
                     Err(x)            TuplePattern                                                                                       */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */
    for H(Ok(mut x) | &Err(mut x)) in std::iter::once(wres) {}                                                                            /*
    for•H(Ok(mut•x)•|•&Err(mut•x))•in•std::iter::once(wres)•{}    ExpressionStatement, ForInBlockExpression
        H(Ok(mut•x)•|•&Err(mut•x))                                TuplePattern
          Ok(mut•x)•|•&Err(mut•x)                                 UnionPattern
          Ok(mut•x)                                               TuplePattern
             mut•x                                                PatternVariableDeclaration
                      &Err(mut•x)                                 ReferencePattern
                       Err(mut•x)                                 TuplePattern
                           mut•x                                  PatternVariableDeclaration
                                      std::iter::once(wres)       CallExpression
                                      std::iter::once             ExpressionPath
                                      std::iter                   ExpressionPath                                                          */
    for H(&(Ok(x) | Err(x))) in std::iter::once(wres) {}                                                                                  /*
    for•H(&(Ok(x)•|•Err(x)))•in•std::iter::once(wres)•{}    ExpressionStatement, ForInBlockExpression
        H(&(Ok(x)•|•Err(x)))                                TuplePattern
          &(Ok(x)•|•Err(x))                                 ReferencePattern
            Ok(x)•|•Err(x)                                  UnionPattern
            Ok(x)                                           TuplePattern
                    Err(x)                                  TuplePattern
                                std::iter::once(wres)       CallExpression
                                std::iter::once             ExpressionPath
                                std::iter                   ExpressionPath                                                                */
    for H(Ok(x) | Err(x)) in std::iter::once(wres) {}                                                                                     /*
    for•H(Ok(x)•|•Err(x))•in•std::iter::once(wres)•{}    ExpressionStatement, ForInBlockExpression
        H(Ok(x)•|•Err(x))                                TuplePattern
          Ok(x)•|•Err(x)                                 UnionPattern
          Ok(x)                                          TuplePattern
                  Err(x)                                 TuplePattern
                             std::iter::once(wres)       CallExpression
                             std::iter::once             ExpressionPath
                             std::iter                   ExpressionPath                                                                   */

    let H(Ok(mut x) | &Err(mut x)) = wres;                                                                                                /*
    let•H(Ok(mut•x)•|•&Err(mut•x))•=•wres;    LetVariableDeclaration
        H(Ok(mut•x)•|•&Err(mut•x))            TuplePattern
          Ok(mut•x)•|•&Err(mut•x)             UnionPattern
          Ok(mut•x)                           TuplePattern
             mut•x                            PatternVariableDeclaration
                      &Err(mut•x)             ReferencePattern
                       Err(mut•x)             TuplePattern
                           mut•x              PatternVariableDeclaration                                                                  */
    let H(Ok(x) | Err(x)) = wres;                                                                                                         /*
    let•H(Ok(x)•|•Err(x))•=•wres;    LetVariableDeclaration
        H(Ok(x)•|•Err(x))            TuplePattern
          Ok(x)•|•Err(x)             UnionPattern
          Ok(x)                      TuplePattern
                  Err(x)             TuplePattern                                                                                         */
    let H(&(Ok(x) | Err(x))) = wres;                                                                                                      /*
    let•H(&(Ok(x)•|•Err(x)))•=•wres;    LetVariableDeclaration
        H(&(Ok(x)•|•Err(x)))            TuplePattern
          &(Ok(x)•|•Err(x))             ReferencePattern
            Ok(x)•|•Err(x)              UnionPattern
            Ok(x)                       TuplePattern
                    Err(x)              TuplePattern                                                                                      */


	let (Tri::A(Ok(mut x) | Err(mut x))                                                                                                   /*
    let•(Tri::A(Ok(mut•x)•|•Err(mut•x))↲    <LetVariableDeclaration>
         Tri::A(Ok(mut•x)•|•Err(mut•x))↲    <UnionPattern>
         Tri::A(Ok(mut•x)•|•Err(mut•x))     TuplePattern
         Tri::A                             ExpressionPath
                Ok(mut•x)•|•Err(mut•x)      UnionPattern
                Ok(mut•x)                   TuplePattern
                   mut•x                    PatternVariableDeclaration
                            Err(mut•x)      TuplePattern
                                mut•x       PatternVariableDeclaration                                                                    */
    | Tri::B(&Ok(mut x) | Err(mut x))                                                                                                     /*
      Tri::B(&Ok(mut•x)•|•Err(mut•x))    TuplePattern
      Tri::B                             ExpressionPath
             &Ok(mut•x)•|•Err(mut•x)     UnionPattern
             &Ok(mut•x)                  ReferencePattern
              Ok(mut•x)                  TuplePattern
                 mut•x                   PatternVariableDeclaration
                          Err(mut•x)     TuplePattern
                              mut•x      PatternVariableDeclaration                                                                       */
    | &Tri::C(Ok(mut x) | Err(mut x))) = tri;                                                                                             /*
••••|•&Tri::C(Ok(mut•x)•|•Err(mut•x)))•=•tri;    </LetVariableDeclaration>
••••|•&Tri::C(Ok(mut•x)•|•Err(mut•x))            </UnionPattern>
      &Tri::C(Ok(mut•x)•|•Err(mut•x))            ReferencePattern
       Tri::C(Ok(mut•x)•|•Err(mut•x))            TuplePattern
       Tri::C                                    ExpressionPath
              Ok(mut•x)•|•Err(mut•x)             UnionPattern
              Ok(mut•x)                          TuplePattern
                 mut•x                           PatternVariableDeclaration
                          Err(mut•x)             TuplePattern
                              mut•x              PatternVariableDeclaration                                                               */

    let (B(A(a, _) | B(a)) | A(a, A(a, _) | B(a))) = B(B(1));                                                                             /*
    let•(B(A(a,•_)•|•B(a))•|•A(a,•A(a,•_)•|•B(a)))•=•B(B(1));    LetVariableDeclaration
         B(A(a,•_)•|•B(a))•|•A(a,•A(a,•_)•|•B(a))                UnionPattern
         B(A(a,•_)•|•B(a))                                       TuplePattern
           A(a,•_)•|•B(a)                                        UnionPattern
           A(a,•_)                                               TuplePattern
                _                                                WildcardPattern
                     B(a)                                        TuplePattern
                             A(a,•A(a,•_)•|•B(a))                TuplePattern
                                  A(a,•_)•|•B(a)                 UnionPattern
                                  A(a,•_)                        TuplePattern
                                       _                         WildcardPattern
                                            B(a)                 TuplePattern
                                                     B(B(1))     CallExpression
                                                       B(1)      CallExpression
                                                         1       Literal                                                                  */
    let (B(_) | A(A(a, _) | B(a), A(a, _) | B(a))) = B(B(1));                                                                             /*
    let•(B(_)•|•A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a)))•=•B(B(1));    LetVariableDeclaration
         B(_)•|•A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                UnionPattern
         B(_)                                                    TuplePattern
           _                                                     WildcardPattern
                A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                TuplePattern
                  A(a,•_)•|•B(a)                                 UnionPattern
                  A(a,•_)                                        TuplePattern
                       _                                         WildcardPattern
                            B(a)                                 TuplePattern
                                  A(a,•_)•|•B(a)                 UnionPattern
                                  A(a,•_)                        TuplePattern
                                       _                         WildcardPattern
                                            B(a)                 TuplePattern
                                                     B(B(1))     CallExpression
                                                       B(1)      CallExpression
                                                         1       Literal                                                                  */
    let (B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a))) = B(B(1));                                                                /*
    let•(B(A(a,•_)•|•B(a))•|•A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a)))•=•B(B(1));    LetVariableDeclaration
         B(A(a,•_)•|•B(a))•|•A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                UnionPattern
         B(A(a,•_)•|•B(a))                                                    TuplePattern
           A(a,•_)•|•B(a)                                                     UnionPattern
           A(a,•_)                                                            TuplePattern
                _                                                             WildcardPattern
                     B(a)                                                     TuplePattern
                             A(A(a,•_)•|•B(a),•A(a,•_)•|•B(a))                TuplePattern
                               A(a,•_)•|•B(a)                                 UnionPattern
                               A(a,•_)                                        TuplePattern
                                    _                                         WildcardPattern
                                         B(a)                                 TuplePattern
                                               A(a,•_)•|•B(a)                 UnionPattern
                                               A(a,•_)                        TuplePattern
                                                    _                         WildcardPattern
                                                         B(a)                 TuplePattern
                                                                  B(B(1))     CallExpression
                                                                    B(1)      CallExpression
                                                                      1       Literal                                                     */

	let (Ok(a) | Err(a)) = Ok(0);                                                                                                         /*
    let•(Ok(a)•|•Err(a))•=•Ok(0);    LetVariableDeclaration
         Ok(a)•|•Err(a)              UnionPattern
         Ok(a)                       TuplePattern
                 Err(a)              TuplePattern
                           Ok(0)     CallExpression
                              0      Literal                                                                                              */
    let (Ok(ref a) | Err(ref a)) = Ok(0);                                                                                                 /*
    let•(Ok(ref•a)•|•Err(ref•a))•=•Ok(0);    LetVariableDeclaration
         Ok(ref•a)•|•Err(ref•a)              UnionPattern
         Ok(ref•a)                           TuplePattern
            ref•a                            PatternVariableDeclaration
                     Err(ref•a)              TuplePattern
                         ref•a               PatternVariableDeclaration
                                   Ok(0)     CallExpression
                                      0      Literal                                                                                      */
    let (Ok(ref mut a) | Err(ref mut a)) = Ok(0);                                                                                         /*
    let•(Ok(ref•mut•a)•|•Err(ref•mut•a))•=•Ok(0);    LetVariableDeclaration
         Ok(ref•mut•a)•|•Err(ref•mut•a)              UnionPattern
         Ok(ref•mut•a)                               TuplePattern
            ref•mut•a                                PatternVariableDeclaration
                         Err(ref•mut•a)              TuplePattern
                             ref•mut•a               PatternVariableDeclaration
                                           Ok(0)     CallExpression
                                              0      Literal                                                                              */
	let (Ok((V1(a) | V2(a) | V3(a), b)) | Err(Ok((a, b)) | Err((a, b)))): Result<_, Result<_, _>> =                                       /*
    let•(Ok((V1(a)•|•V2(a)•|•V3(a),•b))•|•Err(Ok((a,•b))•|•Err((a,•b)))):•Result<_,•Result<_,•_>>•=↲    <LetVariableDeclaration>
         Ok((V1(a)•|•V2(a)•|•V3(a),•b))•|•Err(Ok((a,•b))•|•Err((a,•b)))                                 UnionPattern
         Ok((V1(a)•|•V2(a)•|•V3(a),•b))                                                                 TuplePattern
            (V1(a)•|•V2(a)•|•V3(a),•b)                                                                  TuplePattern
             V1(a)•|•V2(a)•|•V3(a)                                                                      UnionPattern
             V1(a)                                                                                      TuplePattern
                     V2(a)                                                                              TuplePattern
                             V3(a)                                                                      TuplePattern
                                          Err(Ok((a,•b))•|•Err((a,•b)))                                 TuplePattern
                                              Ok((a,•b))•|•Err((a,•b))                                  UnionPattern
                                              Ok((a,•b))                                                TuplePattern
                                                 (a,•b)                                                 TuplePattern
                                                           Err((a,•b))                                  TuplePattern
                                                               (a,•b)                                   TuplePattern
                                                                          Result<_,•Result<_,•_>>       TypeCall
                                                                                 _                      TypeInferred
                                                                                    Result<_,•_>        TypeCall
                                                                                           _            TypeInferred
                                                                                              _         TypeInferred                      */
	Ok((V1(1), 1));                                                                                                                       /*
   ╚Ok((V1(1),•1));    </LetVariableDeclaration>
    Ok((V1(1),•1))     CallExpression
       (V1(1),•1)      TupleLiteral
        V1(1)          CallExpression
           1           Literal
               1       Literal                                                                                                            */

	let (Ok((V1(a) | V2(a) | V3(a), ref b)) | Err(Ok((a, ref b)) | Err((a, ref b)))): Result<                                             /*
    let•(Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))•|•Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))):•Result<↲    <LetVariableDeclaration>
         Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))•|•Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))               UnionPattern
         Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))                                                       TuplePattern
            (V1(a)•|•V2(a)•|•V3(a),•ref•b)                                                        TuplePattern
             V1(a)•|•V2(a)•|•V3(a)                                                                UnionPattern
             V1(a)                                                                                TuplePattern
                     V2(a)                                                                        TuplePattern
                             V3(a)                                                                TuplePattern
                                    ref•b                                                         PatternVariableDeclaration
                                              Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))               TuplePattern
                                                  Ok((a,•ref•b))•|•Err((a,•ref•b))                UnionPattern
                                                  Ok((a,•ref•b))                                  TuplePattern
                                                     (a,•ref•b)                                   TuplePattern
                                                         ref•b                                    PatternVariableDeclaration
                                                                   Err((a,•ref•b))                TuplePattern
                                                                       (a,•ref•b)                 TuplePattern
                                                                           ref•b                  PatternVariableDeclaration
                                                                                      Result<↲    <TypeCall>                              */
		_,                                                                                                                                /*
        _    TypeInferred                                                                                                                 */
		Result<_, _>,                                                                                                                     /*
        Result<_,•_>    TypeCall
               _        TypeInferred
                  _     TypeInferred                                                                                                      */
	> = Ok((V1(1), 1));                                                                                                                   /*
   ╚>•=•Ok((V1(1),•1));    </LetVariableDeclaration>
   ╚>                      </TypeCall>
        Ok((V1(1),•1))     CallExpression
           (V1(1),•1)      TupleLiteral
            V1(1)          CallExpression
               1           Literal
                   1       Literal                                                                                                        */

	let (                                                                                                                                 /*
    let•(↲    <LetVariableDeclaration>
        (↲    <TuplePattern>                                                                                                              */
        a,
        Err((ref mut b, ref c, d))                                                                                                        /*
        Err((ref•mut•b,•ref•c,•d))↲    <UnionPattern>
        Err((ref•mut•b,•ref•c,•d))     TuplePattern
            (ref•mut•b,•ref•c,•d)      TuplePattern
             ref•mut•b                 PatternVariableDeclaration
                        ref•c          PatternVariableDeclaration                                                                         */
        | Ok((                                                                                                                            /*
          Ok((↲    <TuplePattern>
             (↲    <TuplePattern>                                                                                                         */
            Ok(V1((ref c, d)) | V2((d, ref c)) | V3((ref c, Ok((_, d)) | Err((d, _)))))                                                   /*
            Ok(V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))↲    <UnionPattern>
            Ok(V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))     TuplePattern
               V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_))))      UnionPattern
               V1((ref•c,•d))                                                               TuplePattern
                  (ref•c,•d)                                                                TuplePattern
                   ref•c                                                                    PatternVariableDeclaration
                                V2((d,•ref•c))                                              TuplePattern
                                   (d,•ref•c)                                               TuplePattern
                                       ref•c                                                PatternVariableDeclaration
                                                 V3((ref•c,•Ok((_,•d))•|•Err((d,•_))))      TuplePattern
                                                    (ref•c,•Ok((_,•d))•|•Err((d,•_)))       TuplePattern
                                                     ref•c                                  PatternVariableDeclaration
                                                            Ok((_,•d))•|•Err((d,•_))        UnionPattern
                                                            Ok((_,•d))                      TuplePattern
                                                               (_,•d)                       TuplePattern
                                                                _                           WildcardPattern
                                                                         Err((d,•_))        TuplePattern
                                                                             (d,•_)         TuplePattern
                                                                                 _          WildcardPattern                               */
            | Err((ref c, d)),                                                                                                            /*
••••••••••••|•Err((ref•c,•d))     </UnionPattern>
              Err((ref•c,•d))     TuplePattern
                  (ref•c,•d)      TuplePattern
                   ref•c          PatternVariableDeclaration                                                                              */
            ref mut b,                                                                                                                    /*
            ref•mut•b     PatternVariableDeclaration                                                                                      */
        )),                                                                                                                               /*
••••••••))     </UnionPattern>, </TuplePattern>
••••••••)      </TuplePattern>                                                                                                            */
    ): (_, Result<_, _>) = (1, Ok((Ok(V3((1, Ok::<_, (i32, i32)>((1, 1))))), 1)));                                                        /*
••••):•(_,•Result<_,•_>)•=•(1,•Ok((Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1)));    </LetVariableDeclaration>
••••)                                                                                 </TuplePattern>
       (_,•Result<_,•_>)                                                              TypeTuple
        _                                                                             TypeInferred
           Result<_,•_>                                                               TypeCall
                  _                                                                   TypeInferred
                     _                                                                TypeInferred
                           (1,•Ok((Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1)))     TupleLiteral
                            1                                                         Literal
                               Ok((Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1))      CallExpression
                                  (Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1)       TupleLiteral
                                   Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1)))))           CallExpression
                                      V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))            CallExpression
                                         (1,•Ok::<_,•(i32,•i32)>((1,•1)))             TupleLiteral
                                          1                                           Literal
                                             Ok::<_,•(i32,•i32)>((1,•1))              CallExpression
                                                  _                                   TypeInferred
                                                     (i32,•i32)                       TypeTuple
                                                                 (1,•1)               TupleLiteral
                                                                  1                   Literal
                                                                     1                Literal
                                                                             1        Literal                                             */

	for &(Ok(i) | Err(i)) in &v {                                                                                                         /*
    for•&(Ok(i)•|•Err(i))•in•&v•{↲    <ExpressionStatement>, <ForInBlockExpression>
        &(Ok(i)•|•Err(i))             ReferencePattern
          Ok(i)•|•Err(i)              UnionPattern
          Ok(i)                       TuplePattern
                  Err(i)              TuplePattern
                             &v       ReferenceExpression                                                                                 */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </ForInBlockExpression>                                                                                  */
    for Ok(i) | Err(i) in v {                                                                                                             /*
    for•Ok(i)•|•Err(i)•in•v•{↲    <ExpressionStatement>, <ForInBlockExpression>
        Ok(i)•|•Err(i)            UnionPattern
        Ok(i)                     TuplePattern
                Err(i)            TuplePattern                                                                                            */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </ForInBlockExpression>                                                                                  */
    if let &(None | Some(6 | 7)) = &opt {                                                                                                 /*
    if•let•&(None•|•Some(6•|•7))•=•&opt•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•&(None•|•Some(6•|•7))•=•&opt       LetScrutinee
           &(None•|•Some(6•|•7))              ReferencePattern
             None•|•Some(6•|•7)               UnionPattern
                    Some(6•|•7)               TuplePattern
                         6•|•7                UnionPattern
                         6                    Literal
                             7                Literal
                                   &opt       ReferenceExpression                                                                         */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */
    if let Some(x @ (4 | 5 | 6)) = opt {                                                                                                  /*
    if•let•Some(x•@•(4•|•5•|•6))•=•opt•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•Some(x•@•(4•|•5•|•6))•=•opt       LetScrutinee
           Some(x•@•(4•|•5•|•6))             TuplePattern
                x•@•(4•|•5•|•6)              PatternVariableDeclaration
                     4•|•5•|•6               UnionPattern
                     4                       Literal
                         5                   Literal
                             6               Literal                                                                                      */
    } else {                                                                                                                              /*
           {↲    <BlockExpression>                                                                                                        */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */
    while let Some(ref mut val @ (3 | 4 | 6)) = opt {                                                                                     /*
    while•let•Some(ref•mut•val•@•(3•|•4•|•6))•=•opt•{↲    <ExpressionStatement>, <WhileBlockExpression>
          let•Some(ref•mut•val•@•(3•|•4•|•6))•=•opt       LetScrutinee
              Some(ref•mut•val•@•(3•|•4•|•6))             TuplePattern
                   ref•mut•val•@•(3•|•4•|•6)              PatternVariableDeclaration
                                  3•|•4•|•6               UnionPattern
                                  3                       Literal
                                      4                   Literal
                                          6               Literal                                                                         */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </WhileBlockExpression>                                                                                  */

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
                  0     Literal                                                                                                           */
    let (A | B): u8 = 0;                                                                                                                  /*
    let•(A•|•B):•u8•=•0;    LetVariableDeclaration
         A•|•B              UnionPattern
                      0     Literal                                                                                                       */
	for | A | B in 0 {}                                                                                                                   /*
    for•|•A•|•B•in•0•{}    ExpressionStatement, ForInBlockExpression
        |•A•|•B            UnionPattern
                   0       Literal                                                                                                        */
    for A | B in 0 {}                                                                                                                     /*
    for•A•|•B•in•0•{}    ExpressionStatement, ForInBlockExpression
        A•|•B            UnionPattern
                 0       Literal                                                                                                          */
	while let | A | B = 0 {}                                                                                                              /*
    while•let•|•A•|•B•=•0•{}    ExpressionStatement, WhileBlockExpression
          let•|•A•|•B•=•0       LetScrutinee
              |•A•|•B           UnionPattern
                        0       Literal                                                                                                   */
    while let A | B = 0 {}                                                                                                                /*
    while•let•A•|•B•=•0•{}    ExpressionStatement, WhileBlockExpression
          let•A•|•B•=•0       LetScrutinee
              A•|•B           UnionPattern
                      0       Literal                                                                                                     */
	if let | A | B = 0 {}                                                                                                                 /*
    if•let•|•A•|•B•=•0•{}    ExpressionStatement, IfBlockExpression
       let•|•A•|•B•=•0       LetScrutinee
           |•A•|•B           UnionPattern
                     0       Literal                                                                                                      */
    if let A | B = 0 {}                                                                                                                   /*
    if•let•A•|•B•=•0•{}    ExpressionStatement, IfBlockExpression
       let•A•|•B•=•0       LetScrutinee
           A•|•B           UnionPattern
                   0       Literal                                                                                                        */
	if let Ok(mut x) | &Err(mut x) = res {                                                                                                /*
    if•let•Ok(mut•x)•|•&Err(mut•x)•=•res•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•Ok(mut•x)•|•&Err(mut•x)•=•res       LetScrutinee
           Ok(mut•x)•|•&Err(mut•x)             UnionPattern
           Ok(mut•x)                           TuplePattern
              mut•x                            PatternVariableDeclaration
                       &Err(mut•x)             ReferencePattern
                        Err(mut•x)             TuplePattern
                            mut•x              PatternVariableDeclaration                                                                 */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */
	if let &(Ok(x) | Err(x)) = res {                                                                                                      /*
    if•let•&(Ok(x)•|•Err(x))•=•res•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•&(Ok(x)•|•Err(x))•=•res       LetScrutinee
           &(Ok(x)•|•Err(x))             ReferencePattern
             Ok(x)•|•Err(x)              UnionPattern
             Ok(x)                       TuplePattern
                     Err(x)              TuplePattern                                                                                     */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */
    let (Ok(mut x) | &Err(mut x)) = res;                                                                                                  /*
    let•(Ok(mut•x)•|•&Err(mut•x))•=•res;    LetVariableDeclaration
         Ok(mut•x)•|•&Err(mut•x)            UnionPattern
         Ok(mut•x)                          TuplePattern
            mut•x                           PatternVariableDeclaration
                     &Err(mut•x)            ReferencePattern
                      Err(mut•x)            TuplePattern
                          mut•x             PatternVariableDeclaration                                                                    */
    let &(Ok(x) | Err(x)) = res;                                                                                                          /*
    let•&(Ok(x)•|•Err(x))•=•res;    LetVariableDeclaration
        &(Ok(x)•|•Err(x))           ReferencePattern
          Ok(x)•|•Err(x)            UnionPattern
          Ok(x)                     TuplePattern
                  Err(x)            TuplePattern                                                                                          */
    let (Ok(x) | Err(x)) = res;                                                                                                           /*
    let•(Ok(x)•|•Err(x))•=•res;    LetVariableDeclaration
         Ok(x)•|•Err(x)            UnionPattern
         Ok(x)                     TuplePattern
                 Err(x)            TuplePattern                                                                                           */
	for Ok(mut x) | &Err(mut x) in std::iter::once(res) {                                                                                 /*
    for•Ok(mut•x)•|•&Err(mut•x)•in•std::iter::once(res)•{↲    <ExpressionStatement>, <ForInBlockExpression>
        Ok(mut•x)•|•&Err(mut•x)                               UnionPattern
        Ok(mut•x)                                             TuplePattern
           mut•x                                              PatternVariableDeclaration
                    &Err(mut•x)                               ReferencePattern
                     Err(mut•x)                               TuplePattern
                         mut•x                                PatternVariableDeclaration
                                   std::iter::once(res)       CallExpression
                                   std::iter::once            ExpressionPath
                                   std::iter                  ExpressionPath                                                              */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </ForInBlockExpression>                                                                                  */
	for &(Ok(x) | Err(x)) in std::iter::once(res) {                                                                                       /*
    for•&(Ok(x)•|•Err(x))•in•std::iter::once(res)•{↲    <ExpressionStatement>, <ForInBlockExpression>
        &(Ok(x)•|•Err(x))                               ReferencePattern
          Ok(x)•|•Err(x)                                UnionPattern
          Ok(x)                                         TuplePattern
                  Err(x)                                TuplePattern
                             std::iter::once(res)       CallExpression
                             std::iter::once            ExpressionPath
                             std::iter                  ExpressionPath                                                                    */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </ForInBlockExpression>                                                                                  */
    for Ok(x) | Err(x) in std::iter::once(res) {                                                                                          /*
    for•Ok(x)•|•Err(x)•in•std::iter::once(res)•{↲    <ExpressionStatement>, <ForInBlockExpression>
        Ok(x)•|•Err(x)                               UnionPattern
        Ok(x)                                        TuplePattern
                Err(x)                               TuplePattern
                          std::iter::once(res)       CallExpression
                          std::iter::once            ExpressionPath
                          std::iter                  ExpressionPath                                                                       */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </ForInBlockExpression>                                                                                  */
	let _ = |(A | B): u8| ();                                                                                                             /*
    let•_•=•|(A•|•B):•u8|•();    LetVariableDeclaration
        _                        WildcardPattern
            |(A•|•B):•u8|•()     ClosureFunctionExpression
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
          B•|•C      UnionPattern                                                                                                         */
    let E::V(B | C);                                                                                                                      /*
    let•E::V(B•|•C);    LetVariableDeclaration
        E::V(B•|•C)     TuplePattern
        E::V            ExpressionPath
             B•|•C      UnionPattern                                                                                                      */
	let S { f1: B | C, f2 };                                                                                                              /*
    let•S•{•f1:•B•|•C,•f2•};    LetVariableDeclaration
        S•{•f1:•B•|•C,•f2•}     StructPattern
            f1:•B•|•C           StructPatternPropertyDestructured
                B•|•C           UnionPattern
                       f2       StructPatternPropertyShorthand                                                                            */
    let E::V { f1: B | C, f2 };                                                                                                           /*
    let•E::V•{•f1:•B•|•C,•f2•};    LetVariableDeclaration
        E::V•{•f1:•B•|•C,•f2•}     StructPattern
        E::V                       ExpressionPath
               f1:•B•|•C           StructPatternPropertyDestructured
                   B•|•C           UnionPattern
                          f2       StructPatternPropertyShorthand                                                                         */
	let [A | B, .. | ..];                                                                                                                 /*
    let•[A•|•B,•..•|•..];    LetVariableDeclaration
        [A•|•B,•..•|•..]     ArrayPattern
         A•|•B               UnionPattern
                ..•|•..      UnionPattern
                ..           RangePattern
                     ..      RestPattern                                                                                                  */
	let (box 0 | 1);                                                                                                                      /*
    let•(box•0•|•1);    LetVariableDeclaration
         box•0•|•1      UnionPattern
         box•0          BoxPattern
             0          Literal
                 1      Literal                                                                                                           */
    let (&0 | 1);                                                                                                                         /*
    let•(&0•|•1);    LetVariableDeclaration
         &0•|•1      UnionPattern
         &0          ReferencePattern
          0          Literal
              1      Literal                                                                                                              */
    let (&mut 0 | 1);                                                                                                                     /*
    let•(&mut•0•|•1);    LetVariableDeclaration
         &mut•0•|•1      UnionPattern
         &mut•0          ReferencePattern
              0          Literal
                  1      Literal                                                                                                          */
    let (x @ 0 | 1);                                                                                                                      /*
    let•(x•@•0•|•1);    LetVariableDeclaration
         x•@•0•|•1      UnionPattern
         x•@•0          PatternVariableDeclaration
             0          Literal
                 1      Literal                                                                                                           */
    let (ref x @ 0 | 1);                                                                                                                  /*
    let•(ref•x•@•0•|•1);    LetVariableDeclaration
         ref•x•@•0•|•1      UnionPattern
         ref•x•@•0          PatternVariableDeclaration
                 0          Literal
                     1      Literal                                                                                                       */
    let (ref mut x @ 0 | 1);                                                                                                              /*
    let•(ref•mut•x•@•0•|•1);    LetVariableDeclaration
         ref•mut•x•@•0•|•1      UnionPattern
         ref•mut•x•@•0          PatternVariableDeclaration
                     0          Literal
                         1      Literal                                                                                                   */

    let (a, A(a, _) | B(a)) = (0, A(1, 2));                                                                                               /*
    let•(a,•A(a,•_)•|•B(a))•=•(0,•A(1,•2));    LetVariableDeclaration
        (a,•A(a,•_)•|•B(a))                    TuplePattern
            A(a,•_)•|•B(a)                     UnionPattern
            A(a,•_)                            TuplePattern
                 _                             WildcardPattern
                      B(a)                     TuplePattern
                              (0,•A(1,•2))     TupleLiteral
                               0               Literal
                                  A(1,•2)      CallExpression
                                    1          Literal
                                       2       Literal                                                                                    */
    let (A(a, _) | B(a), a) = (A(0, 1), 2);                                                                                               /*
    let•(A(a,•_)•|•B(a),•a)•=•(A(0,•1),•2);    LetVariableDeclaration
        (A(a,•_)•|•B(a),•a)                    TuplePattern
         A(a,•_)•|•B(a)                        UnionPattern
         A(a,•_)                               TuplePattern
              _                                WildcardPattern
                   B(a)                        TuplePattern
                              (A(0,•1),•2)     TupleLiteral
                               A(0,•1)         CallExpression
                                 0             Literal
                                    1          Literal
                                        2      Literal                                                                                    */
    let (A(a, a) | B(a)) = A(0, 1);                                                                                                       /*
    let•(A(a,•a)•|•B(a))•=•A(0,•1);    LetVariableDeclaration
         A(a,•a)•|•B(a)                UnionPattern
         A(a,•a)                       TuplePattern
                   B(a)                TuplePattern
                           A(0,•1)     CallExpression
                             0         Literal
                                1      Literal                                                                                            */
    let (B(a) | A(a, a)) = A(0, 1);                                                                                                       /*
    let•(B(a)•|•A(a,•a))•=•A(0,•1);    LetVariableDeclaration
         B(a)•|•A(a,•a)                UnionPattern
         B(a)                          TuplePattern
                A(a,•a)                TuplePattern
                           A(0,•1)     CallExpression
                             0         Literal
                                1      Literal                                                                                            */
    match A(0, 1) {                                                                                                                       /*
    match•A(0,•1)•{↲    <ExpressionStatement>, <MatchExpression>
          A(0,•1)       CallExpression
            0           Literal
               1        Literal                                                                                                           */
        | A | B => 0,                                                                                                                     /*
        |•A•|•B•=>•0     MatchExpressionCase
        |•A•|•B          UnionPattern
                   0     Literal                                                                                                          */
        A | B => 0,                                                                                                                       /*
        A•|•B•=>•0     MatchExpressionCase
        A•|•B          UnionPattern
                 0     Literal                                                                                                            */
        B(a) | A(a, a) => 0,                                                                                                              /*
        B(a)•|•A(a,•a)•=>•0     MatchExpressionCase
        B(a)•|•A(a,•a)          UnionPattern
        B(a)                    TuplePattern
               A(a,•a)          TuplePattern
                          0     Literal                                                                                                   */
		Ok(x @ 4) | Err(x @ (6 | 8)) => 0,                                                                                                /*
        Ok(x•@•4)•|•Err(x•@•(6•|•8))•=>•0    MatchExpressionCase
        Ok(x•@•4)•|•Err(x•@•(6•|•8))         UnionPattern
        Ok(x•@•4)                            TuplePattern
           x•@•4                             PatternVariableDeclaration
               4                             Literal
                    Err(x•@•(6•|•8))         TuplePattern
                        x•@•(6•|•8)          PatternVariableDeclaration
                             6•|•8           UnionPattern
                             6               Literal
                                 8           Literal
                                        0    Literal                                                                                      */
        Ok(x @ 1 | x @ 2) => 0,                                                                                                           /*
        Ok(x•@•1•|•x•@•2)•=>•0     MatchExpressionCase
        Ok(x•@•1•|•x•@•2)          TuplePattern
           x•@•1•|•x•@•2           UnionPattern
           x•@•1                   PatternVariableDeclaration
               1                   Literal
                   x•@•2           PatternVariableDeclaration
                       2           Literal
                             0     Literal                                                                                                */
        Err(x @ (0..=10 | 30..=40)) if x % 2 == 0 => 0,                                                                                   /*
        Err(x•@•(0..=10•|•30..=40))•if•x•%•2•==•0•=>•0     MatchExpressionCase
        Err(x•@•(0..=10•|•30..=40))                        TuplePattern
            x•@•(0..=10•|•30..=40)                         PatternVariableDeclaration
                 0..=10•|•30..=40                          UnionPattern
                 0..=10                                    RangePattern
                 0                                         Literal
                     10                                    Literal
                          30..=40                          RangePattern
                          30                               Literal
                               40                          Literal
                                       x•%•2•==•0          ComparisonExpression
                                       x•%•2               OperationExpression
                                           2               Literal
                                                0          Literal
                                                     0     Literal                                                                        */
        Err(x @ 0..=40) => 0,                                                                                                             /*
        Err(x•@•0..=40)•=>•0     MatchExpressionCase
        Err(x•@•0..=40)          TuplePattern
            x•@•0..=40           PatternVariableDeclaration
                0..=40           RangePattern
                0                Literal
                    40           Literal
                           0     Literal                                                                                                  */
		Some(box Test::Foo | box Test::Bar) => 0,                                                                                         /*
        Some(box•Test::Foo•|•box•Test::Bar)•=>•0    MatchExpressionCase
        Some(box•Test::Foo•|•box•Test::Bar)         TuplePattern
             box•Test::Foo•|•box•Test::Bar          UnionPattern
             box•Test::Foo                          BoxPattern
                 Test::Foo                          ExpressionPath
                             box•Test::Bar          BoxPattern
                                 Test::Bar          ExpressionPath
                                               0    Literal                                                                               */
		&((true, y) | (y, true), z @ (0 | 4)) => (y as u8) + z,                                                                           /*
        &((true,•y)•|•(y,•true),•z•@•(0•|•4))•=>•(y•as•u8)•+•z    MatchExpressionCase
        &((true,•y)•|•(y,•true),•z•@•(0•|•4))                     ReferencePattern
         ((true,•y)•|•(y,•true),•z•@•(0•|•4))                     TuplePattern
          (true,•y)•|•(y,•true)                                   UnionPattern
          (true,•y)                                               TuplePattern
           true                                                   Literal
                      (y,•true)                                   TuplePattern
                          true                                    Literal
                                 z•@•(0•|•4)                      PatternVariableDeclaration
                                      0•|•4                       UnionPattern
                                      0                           Literal
                                          4                       Literal
                                                 (y•as•u8)•+•z    OperationExpression
                                                  y•as•u8         ExpressionAsTypeCast                                                    */
        Foo::One(0) | Foo::One(1) | Foo::One(2) => 0,                                                                                     /*
        Foo::One(0)•|•Foo::One(1)•|•Foo::One(2)•=>•0     MatchExpressionCase
        Foo::One(0)•|•Foo::One(1)•|•Foo::One(2)          UnionPattern
        Foo::One(0)                                      TuplePattern
        Foo::One                                         ExpressionPath
                 0                                       Literal
                      Foo::One(1)                        TuplePattern
                      Foo::One                           ExpressionPath
                               1                         Literal
                                    Foo::One(2)          TuplePattern
                                    Foo::One             ExpressionPath
                                             2           Literal
                                                   0     Literal                                                                          */
        Foo::One(42 | 255) => 0,                                                                                                          /*
        Foo::One(42•|•255)•=>•0     MatchExpressionCase
        Foo::One(42•|•255)          TuplePattern
        Foo::One                    ExpressionPath
                 42•|•255           UnionPattern
                 42                 Literal
                      255           Literal
                              0     Literal                                                                                               */
        Foo::Two(42 | 255, 1024 | 2048) => 0,                                                                                             /*
        Foo::Two(42•|•255,•1024•|•2048)•=>•0     MatchExpressionCase
        Foo::Two(42•|•255,•1024•|•2048)          TuplePattern
        Foo::Two                                 ExpressionPath
                 42•|•255                        UnionPattern
                 42                              Literal
                      255                        Literal
                           1024•|•2048           UnionPattern
                           1024                  Literal
                                  2048           Literal
                                           0     Literal                                                                                  */
        Foo::One(100 | 110..=120 | 210..=220) => 0,                                                                                       /*
        Foo::One(100•|•110..=120•|•210..=220)•=>•0     MatchExpressionCase
        Foo::One(100•|•110..=120•|•210..=220)          TuplePattern
        Foo::One                                       ExpressionPath
                 100•|•110..=120•|•210..=220           UnionPattern
                 100                                   Literal
                       110..=120                       RangePattern
                       110                             Literal
                             120                       Literal
                                   210..=220           RangePattern
                                   210                 Literal
                                         220           Literal
                                                 0     Literal                                                                            */
        Foo::Two(0..=10 | 100..=110, 0 | _) => 0,                                                                                         /*
        Foo::Two(0..=10•|•100..=110,•0•|•_)•=>•0     MatchExpressionCase
        Foo::Two(0..=10•|•100..=110,•0•|•_)          TuplePattern
        Foo::Two                                     ExpressionPath
                 0..=10•|•100..=110                  UnionPattern
                 0..=10                              RangePattern
                 0                                   Literal
                     10                              Literal
                          100..=110                  RangePattern
                          100                        Literal
                                110                  Literal
                                     0•|•_           UnionPattern
                                     0               Literal
                                         _           WildcardPattern
                                               0     Literal                                                                              */
		([] | [0 | 1..=255] | [_, ..],) => 0,                                                                                             /*
        ([]•|•[0•|•1..=255]•|•[_,•..],)•=>•0    MatchExpressionCase
        ([]•|•[0•|•1..=255]•|•[_,•..],)         TuplePattern
         []•|•[0•|•1..=255]•|•[_,•..]           UnionPattern
         []                                     ArrayPattern
              [0•|•1..=255]                     ArrayPattern
               0•|•1..=255                      UnionPattern
               0                                Literal
                   1..=255                      RangePattern
                   1                            Literal
                       255                      Literal
                              [_,•..]           ArrayPattern
                               _                WildcardPattern
                                  ..            RestPattern
                                           0    Literal                                                                                   */
		((0, 0) | (0, 1),) => 0,                                                                                                          /*
        ((0,•0)•|•(0,•1),)•=>•0    MatchExpressionCase
        ((0,•0)•|•(0,•1),)         TuplePattern
         (0,•0)•|•(0,•1)           UnionPattern
         (0,•0)                    TuplePattern
          0                        Literal
             0                     Literal
                  (0,•1)           TuplePattern
                   0               Literal
                      1            Literal
                              0    Literal                                                                                                */
		(a,_) | (_,a) if a > 10 => 0,                                                                                                     /*
        (a,_)•|•(_,a)•if•a•>•10•=>•0    MatchExpressionCase
        (a,_)•|•(_,a)                   UnionPattern
        (a,_)                           TuplePattern
           _                            WildcardPattern
                (_,a)                   TuplePattern
                 _                      WildcardPattern
                         a•>•10         ComparisonExpression
                             10         Literal
                                   0    Literal                                                                                           */
		Some((a, _)) | Some((_, a)) if a > 10 => 0,                                                                                       /*
        Some((a,•_))•|•Some((_,•a))•if•a•>•10•=>•0    MatchExpressionCase
        Some((a,•_))•|•Some((_,•a))                   UnionPattern
        Some((a,•_))                                  TuplePattern
             (a,•_)                                   TuplePattern
                 _                                    WildcardPattern
                       Some((_,•a))                   TuplePattern
                            (_,•a)                    TuplePattern
                             _                        WildcardPattern
                                       a•>•10         ComparisonExpression
                                           10         Literal
                                                 0    Literal                                                                             */
		Some((a, _) | (_, a)) if a > 10 => 0,                                                                                             /*
        Some((a,•_)•|•(_,•a))•if•a•>•10•=>•0    MatchExpressionCase
        Some((a,•_)•|•(_,•a))                   TuplePattern
             (a,•_)•|•(_,•a)                    UnionPattern
             (a,•_)                             TuplePattern
                 _                              WildcardPattern
                      (_,•a)                    TuplePattern
                       _                        WildcardPattern
                                 a•>•10         ComparisonExpression
                                     10         Literal
                                           0    Literal                                                                                   */
		e @ &(1..=2) | e @ &(3..=4) => 0,                                                                                                 /*
        e•@•&(1..=2)•|•e•@•&(3..=4)•=>•0    MatchExpressionCase
        e•@•&(1..=2)•|•e•@•&(3..=4)         UnionPattern
        e•@•&(1..=2)                        PatternVariableDeclaration
            &(1..=2)                        ReferencePattern
              1..=2                         RangePattern
              1                             Literal
                  2                         Literal
                       e•@•&(3..=4)         PatternVariableDeclaration
                           &(3..=4)         ReferencePattern
                             3..=4          RangePattern
                             3              Literal
                                 4          Literal
                                       0    Literal                                                                                       */
		Ok(mut x) | &Err(mut x) => 0,                                                                                                     /*
        Ok(mut•x)•|•&Err(mut•x)•=>•0    MatchExpressionCase
        Ok(mut•x)•|•&Err(mut•x)         UnionPattern
        Ok(mut•x)                       TuplePattern
           mut•x                        PatternVariableDeclaration
                    &Err(mut•x)         ReferencePattern
                     Err(mut•x)         TuplePattern
                         mut•x          PatternVariableDeclaration
                                   0    Literal                                                                                           */
		Tri::A(Ok(mut x) | Err(mut x))                                                                                                    /*
        Tri::A(Ok(mut•x)•|•Err(mut•x))↲    <MatchExpressionCase>, <UnionPattern>
        Tri::A(Ok(mut•x)•|•Err(mut•x))     TuplePattern
        Tri::A                             ExpressionPath
               Ok(mut•x)•|•Err(mut•x)      UnionPattern
               Ok(mut•x)                   TuplePattern
                  mut•x                    PatternVariableDeclaration
                           Err(mut•x)      TuplePattern
                               mut•x       PatternVariableDeclaration                                                                     */
        | Tri::B(&Ok(mut x) | Err(mut x))                                                                                                 /*
          Tri::B(&Ok(mut•x)•|•Err(mut•x))    TuplePattern
          Tri::B                             ExpressionPath
                 &Ok(mut•x)•|•Err(mut•x)     UnionPattern
                 &Ok(mut•x)                  ReferencePattern
                  Ok(mut•x)                  TuplePattern
                     mut•x                   PatternVariableDeclaration
                              Err(mut•x)     TuplePattern
                                  mut•x      PatternVariableDeclaration                                                                   */
        | &Tri::C(Ok(mut x) | Err(mut x)) => 0,                                                                                           /*
••••••••|•&Tri::C(Ok(mut•x)•|•Err(mut•x))•=>•0     </MatchExpressionCase>
••••••••|•&Tri::C(Ok(mut•x)•|•Err(mut•x))          </UnionPattern>
          &Tri::C(Ok(mut•x)•|•Err(mut•x))          ReferencePattern
           Tri::C(Ok(mut•x)•|•Err(mut•x))          TuplePattern
           Tri::C                                  ExpressionPath
                  Ok(mut•x)•|•Err(mut•x)           UnionPattern
                  Ok(mut•x)                        TuplePattern
                     mut•x                         PatternVariableDeclaration
                              Err(mut•x)           TuplePattern
                                  mut•x            PatternVariableDeclaration
                                             0     Literal                                                                                */
        | A | B => 0,                                                                                                                     /*
        |•A•|•B•=>•0     MatchExpressionCase
        |•A•|•B          UnionPattern
                   0     Literal                                                                                                          */
        A | B => 0,                                                                                                                       /*
        A•|•B•=>•0     MatchExpressionCase
        A•|•B          UnionPattern
                 0     Literal                                                                                                            */
		[.., Some(Test::Qux | Test::Foo)] => 0,                                                                                           /*
        [..,•Some(Test::Qux•|•Test::Foo)]•=>•0    MatchExpressionCase
        [..,•Some(Test::Qux•|•Test::Foo)]         ArrayPattern
         ..                                       RestPattern
             Some(Test::Qux•|•Test::Foo)          TuplePattern
                  Test::Qux•|•Test::Foo           UnionPattern
                  Test::Qux                       ExpressionPath
                              Test::Foo           ExpressionPath
                                             0    Literal                                                                                 */
        [Some(Test::Foo), .., Some(Test::Baz | Test::Bar)] => 0,                                                                          /*
        [Some(Test::Foo),•..,•Some(Test::Baz•|•Test::Bar)]•=>•0     MatchExpressionCase
        [Some(Test::Foo),•..,•Some(Test::Baz•|•Test::Bar)]          ArrayPattern
         Some(Test::Foo)                                            TuplePattern
              Test::Foo                                             ExpressionPath
                          ..                                        RestPattern
                              Some(Test::Baz•|•Test::Bar)           TuplePattern
                                   Test::Baz•|•Test::Bar            UnionPattern
                                   Test::Baz                        ExpressionPath
                                               Test::Bar            ExpressionPath
                                                              0     Literal                                                               */
        [.., Some(Test::Bar | Test::Baz), _] => 0,                                                                                        /*
        [..,•Some(Test::Bar•|•Test::Baz),•_]•=>•0     MatchExpressionCase
        [..,•Some(Test::Bar•|•Test::Baz),•_]          ArrayPattern
         ..                                           RestPattern
             Some(Test::Bar•|•Test::Baz)              TuplePattern
                  Test::Bar•|•Test::Baz               UnionPattern
                  Test::Bar                           ExpressionPath
                              Test::Baz               ExpressionPath
                                          _           WildcardPattern
                                                0     Literal                                                                             */
		Some(                                                                                                                             /*
        Some(↲    <MatchExpressionCase>, <TuplePattern>                                                                                   */
            Test::Foo { first: 1024 | 2048, second: 2048 | 4096 }                                                                         /*
            Test::Foo•{•first:•1024•|•2048,•second:•2048•|•4096•}↲    <UnionPattern>
            Test::Foo•{•first:•1024•|•2048,•second:•2048•|•4096•}     StructPattern
            Test::Foo                                                 ExpressionPath
                        first:•1024•|•2048                            StructPatternPropertyDestructured
                               1024•|•2048                            UnionPattern
                               1024                                   Literal
                                      2048                            Literal
                                            second:•2048•|•4096       StructPatternPropertyDestructured
                                                    2048•|•4096       UnionPattern
                                                    2048              Literal
                                                           4096       Literal                                                             */
            | Test::Bar { other: Some(Other::One | Other::Two) },                                                                         /*
••••••••••••|•Test::Bar•{•other:•Some(Other::One•|•Other::Two)•}     </UnionPattern>
              Test::Bar•{•other:•Some(Other::One•|•Other::Two)•}     StructPattern
              Test::Bar                                              ExpressionPath
                          other:•Some(Other::One•|•Other::Two)       StructPatternPropertyDestructured
                                 Some(Other::One•|•Other::Two)       TuplePattern
                                      Other::One•|•Other::Two        UnionPattern
                                      Other::One                     ExpressionPath
                                                   Other::Two        ExpressionPath                                                       */
        ) => 0,                                                                                                                           /*
••••••••)•=>•0     </MatchExpressionCase>
••••••••)          </TuplePattern>
             0     Literal                                                                                                                */
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
                           b•@•_                                                       PatternVariableDeclaration
                               _                                                       WildcardPattern
                                  _                                                    WildcardPattern
                                       (_,•b•@•_)                                      TuplePattern
                                        _                                              WildcardPattern
                                           b•@•_                                       PatternVariableDeclaration
                                               _                                       WildcardPattern
                                                   (c•@•false,•_)•|•(_,•c•@•true)      UnionPattern
                                                   (c•@•false,•_)                      TuplePattern
                                                    c•@•false                          PatternVariableDeclaration
                                                        false                          Literal
                                                               _                       WildcardPattern
                                                                    (_,•c•@•true)      TuplePattern
                                                                     _                 WildcardPattern
                                                                        c•@•true       PatternVariableDeclaration
                                                                            true       Literal                                            */
            if {                                                                                                                          /*
               {↲    <BlockExpression>                                                                                                    */
                guard_count += 1;                                                                                                         /*
                guard_count•+=•1;    ExpressionStatement
                guard_count•+=•1     ReassignmentOperationExpression
                               1     Literal                                                                                              */
                (a, b, c) == target                                                                                                       /*
                (a,•b,•c)•==•target    ExpressionStatement, ComparisonExpression
                (a,•b,•c)              TupleLiteral                                                                                       */
            } => 0,                                                                                                                       /*
••••••••••••}•=>•0     </MatchExpressionCase>
••••••••••••}          </BlockExpression>
                 0     Literal                                                                                                            */
		((a, _), (b @ _, _), (c @ false, _))                                                                                              /*
        ((a,•_),•(b•@•_,•_),•(c•@•false,•_))↲    <MatchExpressionCase>, <UnionPattern>
        ((a,•_),•(b•@•_,•_),•(c•@•false,•_))     TuplePattern
         (a,•_)                                  TuplePattern
             _                                   WildcardPattern
                 (b•@•_,•_)                      TuplePattern
                  b•@•_                          PatternVariableDeclaration
                      _                          WildcardPattern
                         _                       WildcardPattern
                             (c•@•false,•_)      TuplePattern
                              c•@•false          PatternVariableDeclaration
                                  false          Literal
                                         _       WildcardPattern                                                                          */
		| ((a, _), (b @ _, _), (_, c @ true))                                                                                             /*
          ((a,•_),•(b•@•_,•_),•(_,•c•@•true))    TuplePattern
           (a,•_)                                TuplePattern
               _                                 WildcardPattern
                   (b•@•_,•_)                    TuplePattern
                    b•@•_                        PatternVariableDeclaration
                        _                        WildcardPattern
                           _                     WildcardPattern
                               (_,•c•@•true)     TuplePattern
                                _                WildcardPattern
                                   c•@•true      PatternVariableDeclaration
                                       true      Literal                                                                                  */
		| ((a, _), (_, b @ _), (c @ false, _))                                                                                            /*
          ((a,•_),•(_,•b•@•_),•(c•@•false,•_))    TuplePattern
           (a,•_)                                 TuplePattern
               _                                  WildcardPattern
                   (_,•b•@•_)                     TuplePattern
                    _                             WildcardPattern
                       b•@•_                      PatternVariableDeclaration
                           _                      WildcardPattern
                               (c•@•false,•_)     TuplePattern
                                c•@•false         PatternVariableDeclaration
                                    false         Literal
                                           _      WildcardPattern                                                                         */
		| ((a, _), (_, b @ _), (_, c @ true))                                                                                             /*
          ((a,•_),•(_,•b•@•_),•(_,•c•@•true))    TuplePattern
           (a,•_)                                TuplePattern
               _                                 WildcardPattern
                   (_,•b•@•_)                    TuplePattern
                    _                            WildcardPattern
                       b•@•_                     PatternVariableDeclaration
                           _                     WildcardPattern
                               (_,•c•@•true)     TuplePattern
                                _                WildcardPattern
                                   c•@•true      PatternVariableDeclaration
                                       true      Literal                                                                                  */
		| ((_, a), (b @ _, _), (c @ false, _))                                                                                            /*
          ((_,•a),•(b•@•_,•_),•(c•@•false,•_))    TuplePattern
           (_,•a)                                 TuplePattern
            _                                     WildcardPattern
                   (b•@•_,•_)                     TuplePattern
                    b•@•_                         PatternVariableDeclaration
                        _                         WildcardPattern
                           _                      WildcardPattern
                               (c•@•false,•_)     TuplePattern
                                c•@•false         PatternVariableDeclaration
                                    false         Literal
                                           _      WildcardPattern                                                                         */
		| ((_, a), (b @ _, _), (_, c @ true))                                                                                             /*
          ((_,•a),•(b•@•_,•_),•(_,•c•@•true))    TuplePattern
           (_,•a)                                TuplePattern
            _                                    WildcardPattern
                   (b•@•_,•_)                    TuplePattern
                    b•@•_                        PatternVariableDeclaration
                        _                        WildcardPattern
                           _                     WildcardPattern
                               (_,•c•@•true)     TuplePattern
                                _                WildcardPattern
                                   c•@•true      PatternVariableDeclaration
                                       true      Literal                                                                                  */
		| ((_, a), (_, b @ _), (c @ false, _))                                                                                            /*
          ((_,•a),•(_,•b•@•_),•(c•@•false,•_))    TuplePattern
           (_,•a)                                 TuplePattern
            _                                     WildcardPattern
                   (_,•b•@•_)                     TuplePattern
                    _                             WildcardPattern
                       b•@•_                      PatternVariableDeclaration
                           _                      WildcardPattern
                               (c•@•false,•_)     TuplePattern
                                c•@•false         PatternVariableDeclaration
                                    false         Literal
                                           _      WildcardPattern                                                                         */
		| ((_, a), (_, b @ _), (_, c @ true))                                                                                             /*
   ╚╚|•((_,•a),•(_,•b•@•_),•(_,•c•@•true))       </UnionPattern>
          ((_,•a),•(_,•b•@•_),•(_,•c•@•true))    TuplePattern
           (_,•a)                                TuplePattern
            _                                    WildcardPattern
                   (_,•b•@•_)                    TuplePattern
                    _                            WildcardPattern
                       b•@•_                     PatternVariableDeclaration
                           _                     WildcardPattern
                               (_,•c•@•true)     TuplePattern
                                _                WildcardPattern
                                   c•@•true      PatternVariableDeclaration
                                       true      Literal                                                                                  */
			if {                                                                                                                          /*
               {↲    <BlockExpression>                                                                                                    */
				guard_count += 1;                                                                                                         /*
                guard_count•+=•1;    ExpressionStatement
                guard_count•+=•1     ReassignmentOperationExpression
                               1     Literal                                                                                              */
				(a, b, c) == target                                                                                                       /*
                (a,•b,•c)•==•target    ExpressionStatement, ComparisonExpression
                (a,•b,•c)              TupleLiteral                                                                                       */
			} => 0,                                                                                                                       /*
   ╚╚╚}•=>•0          </MatchExpressionCase>
   ╚╚╚}               </BlockExpression>
                 0    Literal                                                                                                             */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

accept_pat!((p | q));                                                                                                                     /*
accept_pat!((p•|•q));    ExpressionStatement
accept_pat!((p•|•q))     MacroInvocation
            (p•|•q)      DelimGroup
               |         PunctuationToken                                                                                                 */
accept_pat!((p | q,));                                                                                                                    /*
accept_pat!((p•|•q,));    ExpressionStatement
accept_pat!((p•|•q,))     MacroInvocation
            (p•|•q,)      DelimGroup
               |          PunctuationToken
                  ,       PunctuationToken                                                                                                */
accept_pat!(TS(p | q));                                                                                                                   /*
accept_pat!(TS(p•|•q));    ExpressionStatement
accept_pat!(TS(p•|•q))     MacroInvocation
              (p•|•q)      DelimGroup
                 |         PunctuationToken                                                                                               */
accept_pat!(NS { f: p | q });                                                                                                             /*
accept_pat!(NS•{•f:•p•|•q•});    ExpressionStatement
accept_pat!(NS•{•f:•p•|•q•})     MacroInvocation
               {•f:•p•|•q•}      DelimGroup
                  :              PunctuationToken
                      |          PunctuationToken                                                                                         */
accept_pat!([p | q]);                                                                                                                     /*
accept_pat!([p•|•q]);    ExpressionStatement
accept_pat!([p•|•q])     MacroInvocation
            [p•|•q]      DelimGroup
               |         PunctuationToken                                                                                                 */

// Discarded Nodes: 54
// Parsed Nodes: 1660
// state_rollbacks: 1
// Total '.charCodeAt()' calls: 7082 (36% re-reads)
// Unnecessary 'skip_whitespace()' calls: 913
// source: "../../samples/patterns/union.rs"