fn a() {                                                                                                                                  /*
fn•a()•{↲    <FunctionDeclaration>                                                                                                        */
	fn eq(&&other: S) { false }                                                                                                           /*
    fn•eq(&&other:•S)•{•false•}    FunctionDeclaration
          &&other:•S               FunctionParameterDeclaration
          &&other                  ReferencePattern
           &other                  ReferencePattern
                        false      ExpressionStatement, Literal                                                                           */
    let -2147483648..=2147483647 = 1;                                                                                                     /*
    let•-2147483648..=2147483647•=•1;    LetVariableDeclaration
        -2147483648..=2147483647         RangePattern
        -2147483648                      MinusPattern
         2147483648                      Literal
                      2147483647         Literal
                                   1     Literal                                                                                          */
    let 0..=255 = 0u8;                                                                                                                    /*
    let•0..=255•=•0u8;    LetVariableDeclaration
        0..=255           RangePattern
        0                 Literal
            255           Literal
                  0u8     Literal
                   u8     Identifier                                                                                                      */
    let -128..=127 = 0i8;                                                                                                                 /*
    let•-128..=127•=•0i8;    LetVariableDeclaration
        -128..=127           RangePattern
        -128                 MinusPattern
         128                 Literal
               127           Literal
                     0i8     Literal
                      i8     Identifier                                                                                                   */
    let '\u{0000}'..='\u{10FFFF}' = 'v';                                                                                                  /*
    let•'\u{0000}'..='\u{10FFFF}'•=•'v';    LetVariableDeclaration
        '\u{0000}'..='\u{10FFFF}'           RangePattern
        '\u{0000}'                          Literal
                     '\u{10FFFF}'           Literal
                                    'v'     Literal                                                                                       */
	let f = |3: isize| println!("hello");                                                                                                 /*
    let•f•=•|3:•isize|•println!("hello");    LetVariableDeclaration
            |3:•isize|•println!("hello")     ClosureFunctionExpression
             3:•isize                        ClosureFunctionParameterDeclaration
             3                               Literal
                       println!("hello")     MacroInvocation
                                "hello"      Literal                                                                                      */

	match *self {                                                                                                                         /*
    match•*self•{↲    <ExpressionStatement>, <MatchExpression>
          *self       DereferenceExpression                                                                                               */
		Foo::<T>(ref x, ref y) => x                                                                                                       /*
        Foo::<T>(ref•x,•ref•y)•=>•x    MatchExpressionCase
        Foo::<T>(ref•x,•ref•y)         TuplePattern
        Foo::<T>                       ExpressionTypeCast
                 ref•x                 PatternVariableDeclaration
                        ref•y          PatternVariableDeclaration                                                                         */
	}                                                                                                                                     /*
   ╚}    </ExpressionStatement>, </MatchExpression>                                                                                       */

	let A { foo, } = mka();                                                                                                               /*
    let•A•{•foo,•}•=•mka();    LetVariableDeclaration
        A•{•foo,•}             StructPattern
            foo                StructPatternPropertyShorthand
                     mka()     CallExpression                                                                                             */
    let A {                                                                                                                               /*
    let•A•{↲    <LetVariableDeclaration>
        A•{↲    <StructPattern>                                                                                                           */
        foo,                                                                                                                              /*
        foo     StructPatternPropertyShorthand                                                                                            */
    } = mka();                                                                                                                            /*
••••}•=•mka();    </LetVariableDeclaration>
••••}             </StructPattern>
        mka()     CallExpression                                                                                                          */

    let B { a, b, c, } = mkb();                                                                                                           /*
    let•B•{•a,•b,•c,•}•=•mkb();    LetVariableDeclaration
        B•{•a,•b,•c,•}             StructPattern
            a                      StructPatternPropertyShorthand
               b                   StructPatternPropertyShorthand
                  c                StructPatternPropertyShorthand
                         mkb()     CallExpression                                                                                         */

    match mka() {                                                                                                                         /*
    match•mka()•{↲    <ExpressionStatement>, <MatchExpression>
          mka()       CallExpression                                                                                                      */
        A { foo: _foo, } => {}                                                                                                            /*
        A•{•foo:•_foo,•}•=>•{}    MatchExpressionCase
        A•{•foo:•_foo,•}          StructPattern
            foo:•_foo             StructPatternPropertyDestructured
                            {}    BlockExpression                                                                                         */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */

    match Some(mka()) {                                                                                                                   /*
    match•Some(mka())•{↲    <ExpressionStatement>, <MatchExpression>
          Some(mka())       CallExpression
               mka()        CallExpression                                                                                                */
		S { .. } => (),                                                                                                                   /*
        S•{•..•}•=>•()    MatchExpressionCase
        S•{•..•}          StructPattern
            ..            RestPattern
                    ()    TupleLiteral                                                                                                    */
        Some(A { foo: _foo, }) => {}                                                                                                      /*
        Some(A•{•foo:•_foo,•})•=>•{}    MatchExpressionCase
        Some(A•{•foo:•_foo,•})          TuplePattern
             A•{•foo:•_foo,•}           StructPattern
                 foo:•_foo              StructPatternPropertyDestructured
                                  {}    BlockExpression                                                                                   */
        None => {}                                                                                                                        /*
        None•=>•{}    MatchExpressionCase
                {}    BlockExpression                                                                                                     */
		_ => ()                                                                                                                           /*
        _•=>•()    MatchExpressionCase
        _          WildcardPattern
             ()    TupleLiteral                                                                                                           */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
	match (x, y) {                                                                                                                        /*
    match•(x,•y)•{↲    <ExpressionStatement>, <MatchExpression>
          (x,•y)       TupleLiteral                                                                                                       */
        (1, 1) => 1,                                                                                                                      /*
        (1,•1)•=>•1     MatchExpressionCase
        (1,•1)          TuplePattern
         1              Literal
            1           Literal
                  1     Literal                                                                                                           */
        (2, 2) => 2,                                                                                                                      /*
        (2,•2)•=>•2     MatchExpressionCase
        (2,•2)          TuplePattern
         2              Literal
            2           Literal
                  2     Literal                                                                                                           */
        (1..=2, 2) => 3,                                                                                                                  /*
        (1..=2,•2)•=>•3     MatchExpressionCase
        (1..=2,•2)          TuplePattern
         1..=2              RangePattern
         1                  Literal
             2              Literal
                2           Literal
                      3     Literal                                                                                                       */
        _ => 4,                                                                                                                           /*
        _•=>•4     MatchExpressionCase
        _          WildcardPattern
             4     Literal                                                                                                                */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */

	if let Some([b'@', filename @ ..]) = Some(b"@abc123") {                                                                               /*
    if•let•Some([b'@',•filename•@•..])•=•Some(b"@abc123")•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•Some([b'@',•filename•@•..])•=•Some(b"@abc123")       LetScrutinee
           Some([b'@',•filename•@•..])                          TuplePattern
                [b'@',•filename•@•..]                           ArrayPattern
                 b'@'                                           Literal
                       filename•@•..                            PatternVariableDeclaration
                                  ..                            RestPattern
                                         Some(b"@abc123")       CallExpression
                                              b"@abc123"        Literal                                                                   */
        println!("filename {:?}", filename);                                                                                              /*
        println!("filename•{:?}",•filename);    ExpressionStatement
        println!("filename•{:?}",•filename)     MacroInvocation
                 "filename•{:?}"                Literal
                                ,               PunctuationToken                                                                          */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */


	fn f(X(_): A) {}                                                                                                                      /*
    fn•f(X(_):•A)•{}    FunctionDeclaration
         X(_):•A        FunctionParameterDeclaration
         X(_)           TuplePattern
           _            WildcardPattern                                                                                                   */

	let Ok(0): Option<u8> = 42u8;                                                                                                         /*
    let•Ok(0):•Option<u8>•=•42u8;    LetVariableDeclaration
        Ok(0)                        TuplePattern
           0                         Literal
               Option<u8>            TypeCall
                            42u8     Literal
                              u8     Identifier                                                                                           */
	let Ok(0): Option<u8>;                                                                                                                /*
    let•Ok(0):•Option<u8>;    LetVariableDeclaration
        Ok(0)                 TuplePattern
           0                  Literal
               Option<u8>     TypeCall                                                                                                    */
	let Ok(0) = 42u8;                                                                                                                     /*
    let•Ok(0)•=•42u8;    LetVariableDeclaration
        Ok(0)            TuplePattern
           0             Literal
                42u8     Literal
                  u8     Identifier                                                                                                       */



	match t {                                                                                                                             /*
    match•t•{•↲    <ExpressionStatement>, <MatchExpression>                                                                               */
		Bar::T1(_, Some(x)) => {                                                                                                          /*
        Bar::T1(_,•Some(x))•=>•{•↲    <MatchExpressionCase>
        Bar::T1(_,•Some(x))           TuplePattern
        Bar::T1                       ExpressionPath
                _                     WildcardPattern
                   Some(x)            TuplePattern
                               {•↲    <BlockExpression>                                                                                   */
			return x * 3;                                                                                                                 /*
            return•x•*•3;    ExpressionStatement
            return•x•*•3     ReturnExpression
                   x•*•3     OperationExpression
                       3     Literal                                                                                                      */
		}                                                                                                                                 /*
   ╚╚}    </MatchExpressionCase>, </BlockExpression>                                                                                      */
		_ => { panic!(); }                                                                                                                /*
        _•=>•{•panic!();•}    MatchExpressionCase
        _                     WildcardPattern
             {•panic!();•}    BlockExpression
               panic!();      ExpressionStatement
               panic!()       MacroInvocation                                                                                             */
	}                                                                                                                                     /*
   ╚}    </ExpressionStatement>, </MatchExpression>                                                                                       */

	match t {                                                                                                                             /*
    match•t•{↲    <ExpressionStatement>, <MatchExpression>                                                                                */
		Bar::T1(_, Some::<isize>(x)) => {                                                                                                 /*
        Bar::T1(_,•Some::<isize>(x))•=>•{↲    <MatchExpressionCase>
        Bar::T1(_,•Some::<isize>(x))          TuplePattern
        Bar::T1                               ExpressionPath
                _                             WildcardPattern
                   Some::<isize>(x)           TuplePattern
                   Some::<isize>              ExpressionTypeCast
                                        {↲    <BlockExpression>                                                                           */
		  println!("{}", x);                                                                                                              /*
          println!("{}",•x);    ExpressionStatement
          println!("{}",•x)     MacroInvocation
                   "{}"         Literal
                       ,        PunctuationToken                                                                                          */
		}                                                                                                                                 /*
   ╚╚}    </MatchExpressionCase>, </BlockExpression>                                                                                      */
		_ => { panic!(); }                                                                                                                /*
        _•=>•{•panic!();•}    MatchExpressionCase
        _                     WildcardPattern
             {•panic!();•}    BlockExpression
               panic!();      ExpressionStatement
               panic!()       MacroInvocation                                                                                             */
	}                                                                                                                                     /*
   ╚}    </ExpressionStatement>, </MatchExpression>                                                                                       */

	match unimplemented!() {                                                                                                              /*
    match•unimplemented!()•{↲    <ExpressionStatement>, <MatchExpression>
          unimplemented!()       MacroInvocation                                                                                          */
        &&&42 => {},                                                                                                                      /*
        &&&42•=>•{}     MatchExpressionCase
        &&&42           ReferencePattern
         &&42           ReferencePattern
          &42           ReferencePattern
           42           Literal
                 {}     BlockExpression                                                                                                   */
        FOO => {},                                                                                                                        /*
        FOO•=>•{}     MatchExpressionCase
               {}     BlockExpression                                                                                                     */
        _ => {},                                                                                                                          /*
        _•=>•{}     MatchExpressionCase
        _           WildcardPattern
             {}     BlockExpression                                                                                                       */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
	fn f4(ref a @ box ref b: Box<NC>) {}                                                                                                  /*
    fn•f4(ref•a•@•box•ref•b:•Box<NC>)•{}    FunctionDeclaration
          ref•a•@•box•ref•b:•Box<NC>        FunctionParameterDeclaration
          ref•a•@•box•ref•b                 PatternVariableDeclaration
                  box•ref•b                 BoxPattern
                      ref•b                 PatternVariableDeclaration
                             Box<NC>        TypeCall                                                                                      */
    fn f1(a @ ref b: U) {}                                                                                                                /*
    fn•f1(a•@•ref•b:•U)•{}    FunctionDeclaration
          a•@•ref•b:•U        FunctionParameterDeclaration
          a•@•ref•b           PatternVariableDeclaration
              ref•b           PatternVariableDeclaration                                                                                  */
	fn _f(_a @ _b: u8) {}                                                                                                                 /*
    fn•_f(_a•@•_b:•u8)•{}    FunctionDeclaration
          _a•@•_b:•u8        FunctionParameterDeclaration
          _a•@•_b            PatternVariableDeclaration                                                                                   */
	let s: &[bool] = &[true];                                                                                                             /*
    let•s:•&[bool]•=•&[true];    LetVariableDeclaration
           &[bool]               TypeReference
            [bool]               TypeSlice
                     &[true]     ReferenceExpression
                      [true]     ArrayLiteral
                       true      Literal                                                                                                  */
    let s0: &[bool; 0] = &[];                                                                                                             /*
    let•s0:•&[bool;•0]•=•&[];    LetVariableDeclaration
            &[bool;•0]           TypeReference
             [bool;•0]           TypeSizedArray
                    0            Literal
                         &[]     ReferenceExpression
                          []     ArrayLiteral                                                                                             */
    let s1: &[bool; 1] = &[false; 1];                                                                                                     /*
    let•s1:•&[bool;•1]•=•&[false;•1];    LetVariableDeclaration
            &[bool;•1]                   TypeReference
             [bool;•1]                   TypeSizedArray
                    1                    Literal
                         &[false;•1]     ReferenceExpression
                          [false;•1]     SizedArrayLiteral
                           false         Literal
                                  1      Literal                                                                                          */
    let s2: &[bool; 2] = &[false; 2];                                                                                                     /*
    let•s2:•&[bool;•2]•=•&[false;•2];    LetVariableDeclaration
            &[bool;•2]                   TypeReference
             [bool;•2]                   TypeSizedArray
                    2                    Literal
                         &[false;•2]     ReferenceExpression
                          [false;•2]     SizedArrayLiteral
                           false         Literal
                                  2      Literal                                                                                          */
    let [] = s0;                                                                                                                          /*
    let•[]•=•s0;    LetVariableDeclaration
        []          ArrayPattern                                                                                                          */
    let [_] = s1;                                                                                                                         /*
    let•[_]•=•s1;    LetVariableDeclaration
        [_]          ArrayPattern
         _           WildcardPattern                                                                                                      */
    let [_, _] = s2;                                                                                                                      /*
    let•[_,•_]•=•s2;    LetVariableDeclaration
        [_,•_]          ArrayPattern
         _              WildcardPattern
            _           WildcardPattern                                                                                                   */
	while let 0..=2 | 1 = 0 {}                                                                                                            /*
    while•let•0..=2•|•1•=•0•{}    ExpressionStatement, WhileBlockExpression
          let•0..=2•|•1•=•0       LetScrutinee
              0..=2•|•1           UnionPattern
              0..=2               RangePattern
              0                   Literal
                  2               Literal
                      1           Literal
                          0       Literal                                                                                                 */
	if let 0..=2 | 1 = 0 {}                                                                                                               /*
    if•let•0..=2•|•1•=•0•{}    ExpressionStatement, IfBlockExpression
       let•0..=2•|•1•=•0       LetScrutinee
           0..=2•|•1           UnionPattern
           0..=2               RangePattern
           0                   Literal
               2               Literal
                   1           Literal
                       0       Literal                                                                                                    */
	match 0u8 { 0 | 0 => {} }                                                                                                             /*
    match•0u8•{•0•|•0•=>•{}•}    ExpressionStatement, MatchExpression
          0u8                    Literal
           u8                    Identifier
                0•|•0•=>•{}      MatchExpressionCase
                0•|•0            UnionPattern
                0                Literal
                    0            Literal
                         {}      BlockExpression                                                                                          */
	if let (0 | 0) = 0 {} else { return };                                                                                                /*
    if•let•(0•|•0)•=•0•{}•else•{•return•};    ExpressionStatement
    if•let•(0•|•0)•=•0•{}•else•{•return•}     IfBlockExpression
       let•(0•|•0)•=•0                        LetScrutinee
            0•|•0                             UnionPattern
            0                                 Literal
                0                             Literal
                     0                        Literal
                               {•return•}     BlockExpression
                                 return       ExpressionStatement, ReturnExpression                                                       */
	let mut arr = [U, U, U, U, U, U, U, U];                                                                                               /*
    let•mut•arr•=•[U,•U,•U,•U,•U,•U,•U,•U];    LetVariableDeclaration
        mut•arr                                PatternVariableDeclaration
                  [U,•U,•U,•U,•U,•U,•U,•U]     ArrayLiteral                                                                               */
	let mut tup = (U, U, U, U, U);                                                                                                        /*
    let•mut•tup•=•(U,•U,•U,•U,•U);    LetVariableDeclaration
        mut•tup                       PatternVariableDeclaration
                  (U,•U,•U,•U,•U)     TupleLiteral                                                                                        */
	let (Ok((V1(a) | V2(a) | V3(a), b)) | Err(Ok((a, b)) | Err((a, b)))): Result<_, Result<_, _>> = Ok((V1(1), 1));                       /*
    let•(Ok((V1(a)•|•V2(a)•|•V3(a),•b))•|•Err(Ok((a,•b))•|•Err((a,•b)))):•Result<_,•Result<_,•_>>•=•Ok((V1(1),•1));    LetVariableDeclaration
         Ok((V1(a)•|•V2(a)•|•V3(a),•b))•|•Err(Ok((a,•b))•|•Err((a,•b)))                                                UnionPattern
         Ok((V1(a)•|•V2(a)•|•V3(a),•b))                                                                                TuplePattern
            (V1(a)•|•V2(a)•|•V3(a),•b)                                                                                 TuplePattern
             V1(a)•|•V2(a)•|•V3(a)                                                                                     UnionPattern
             V1(a)                                                                                                     TuplePattern
                     V2(a)                                                                                             TuplePattern
                             V3(a)                                                                                     TuplePattern
                                          Err(Ok((a,•b))•|•Err((a,•b)))                                                TuplePattern
                                              Ok((a,•b))•|•Err((a,•b))                                                 UnionPattern
                                              Ok((a,•b))                                                               TuplePattern
                                                 (a,•b)                                                                TuplePattern
                                                           Err((a,•b))                                                 TuplePattern
                                                               (a,•b)                                                  TuplePattern
                                                                          Result<_,•Result<_,•_>>                      TypeCall
                                                                                 _                                     TypeInferred
                                                                                    Result<_,•_>                       TypeCall
                                                                                           _                           TypeInferred
                                                                                              _                        TypeInferred
                                                                                                    Ok((V1(1),•1))     CallExpression
                                                                                                       (V1(1),•1)      TupleLiteral
                                                                                                        V1(1)          CallExpression
                                                                                                           1           Literal
                                                                                                               1       Literal            */
    let (Ok((V1(a) | V2(a) | V3(a), ref b)) | Err(Ok((a, ref b)) | Err((a, ref b)))): Result< _, Result<_, _>, > = Ok((V1(1), 1));        /*
    let•(Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))•|•Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))):•Result<•_,•Result<_,•_>,•>•=•Ok((V1(1),•1));    LetVariableDeclaration
         Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))•|•Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))                                                   UnionPattern
         Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))                                                                                           TuplePattern
            (V1(a)•|•V2(a)•|•V3(a),•ref•b)                                                                                            TuplePattern
             V1(a)•|•V2(a)•|•V3(a)                                                                                                    UnionPattern
             V1(a)                                                                                                                    TuplePattern
                     V2(a)                                                                                                            TuplePattern
                             V3(a)                                                                                                    TuplePattern
                                    ref•b                                                                                             PatternVariableDeclaration
                                              Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))                                                   TuplePattern
                                                  Ok((a,•ref•b))•|•Err((a,•ref•b))                                                    UnionPattern
                                                  Ok((a,•ref•b))                                                                      TuplePattern
                                                     (a,•ref•b)                                                                       TuplePattern
                                                         ref•b                                                                        PatternVariableDeclaration
                                                                   Err((a,•ref•b))                                                    TuplePattern
                                                                       (a,•ref•b)                                                     TuplePattern
                                                                           ref•b                                                      PatternVariableDeclaration
                                                                                      Result<•_,•Result<_,•_>,•>                      TypeCall
                                                                                              _                                       TypeInferred
                                                                                                 Result<_,•_>                         TypeCall
                                                                                                        _                             TypeInferred
                                                                                                           _                          TypeInferred
                                                                                                                   Ok((V1(1),•1))     CallExpression
                                                                                                                      (V1(1),•1)      TupleLiteral
                                                                                                                       V1(1)          CallExpression
                                                                                                                          1           Literal
                                                                                                                              1       Literal*/
	let (a, Err((ref mut b, ref c, d)) | Ok((                                                                                             /*
    let•(a,•Err((ref•mut•b,•ref•c,•d))•|•Ok((↲    <LetVariableDeclaration>
        (a,•Err((ref•mut•b,•ref•c,•d))•|•Ok((↲    <TuplePattern>
            Err((ref•mut•b,•ref•c,•d))•|•Ok((↲    <UnionPattern>
            Err((ref•mut•b,•ref•c,•d))            TuplePattern
                (ref•mut•b,•ref•c,•d)             TuplePattern
                 ref•mut•b                        PatternVariableDeclaration
                            ref•c                 PatternVariableDeclaration
                                         Ok((↲    <TuplePattern>
                                            (↲    <TuplePattern>                                                                          */
        Ok(V1((ref c, d)) | V2((d, ref c)) | V3((ref c, Ok((_, d)) | Err((d, _))))) | Err((ref c, d)), ref mut b,                         /*
        Ok(V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))•|•Err((ref•c,•d))                UnionPattern
        Ok(V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))                                  TuplePattern
           V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_))))                                   UnionPattern
           V1((ref•c,•d))                                                                                            TuplePattern
              (ref•c,•d)                                                                                             TuplePattern
               ref•c                                                                                                 PatternVariableDeclaration
                            V2((d,•ref•c))                                                                           TuplePattern
                               (d,•ref•c)                                                                            TuplePattern
                                   ref•c                                                                             PatternVariableDeclaration
                                             V3((ref•c,•Ok((_,•d))•|•Err((d,•_))))                                   TuplePattern
                                                (ref•c,•Ok((_,•d))•|•Err((d,•_)))                                    TuplePattern
                                                 ref•c                                                               PatternVariableDeclaration
                                                        Ok((_,•d))•|•Err((d,•_))                                     UnionPattern
                                                        Ok((_,•d))                                                   TuplePattern
                                                           (_,•d)                                                    TuplePattern
                                                            _                                                        WildcardPattern
                                                                     Err((d,•_))                                     TuplePattern
                                                                         (d,•_)                                      TuplePattern
                                                                             _                                       WildcardPattern
                                                                                      Err((ref•c,•d))                TuplePattern
                                                                                          (ref•c,•d)                 TuplePattern
                                                                                           ref•c                     PatternVariableDeclaration
                                                                                                       ref•mut•b     PatternVariableDeclaration*/
    )),): (_, Result<_, _>) = (1, Ok((Ok(V3((1, Ok::<_, (i32, i32)>((1, 1))))), 1)));                                                     /*
••••)),):•(_,•Result<_,•_>)•=•(1,•Ok((Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1)));    </LetVariableDeclaration>
••••)),)                                                                                 </TuplePattern>
••••))                                                                                   </UnionPattern>, </TuplePattern>
••••)                                                                                    </TuplePattern>
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
                                                                                1        Literal                                          */
	let [ref mut _x0, _, ref _x2, _, _x4, ref mut _x5, _x6, _] = arr;                                                                     /*
    let•[ref•mut•_x0,•_,•ref•_x2,•_,•_x4,•ref•mut•_x5,•_x6,•_]•=•arr;    LetVariableDeclaration
        [ref•mut•_x0,•_,•ref•_x2,•_,•_x4,•ref•mut•_x5,•_x6,•_]           ArrayPattern
         ref•mut•_x0                                                     PatternVariableDeclaration
                      _                                                  WildcardPattern
                         ref•_x2                                         PatternVariableDeclaration
                                  _                                      WildcardPattern
                                          ref•mut•_x5                    PatternVariableDeclaration
                                                            _            WildcardPattern                                                  */
	let [_, _, _x2, _, _, _x5, _, _] = arr;                                                                                               /*
    let•[_,•_,•_x2,•_,•_,•_x5,•_,•_]•=•arr;    LetVariableDeclaration
        [_,•_,•_x2,•_,•_,•_x5,•_,•_]           ArrayPattern
         _                                     WildcardPattern
            _                                  WildcardPattern
                    _                          WildcardPattern
                       _                       WildcardPattern
                               _               WildcardPattern
                                  _            WildcardPattern                                                                            */
	*_x0 = U;                                                                                                                             /*
    *_x0•=•U;    ExpressionStatement
    *_x0•=•U     ReassignmentExpression
    *_x0         DereferenceExpression                                                                                                    */
	let a @ (b, c) = (S, S);                                                                                                              /*
    let•a•@•(b,•c)•=•(S,•S);    LetVariableDeclaration
        a•@•(b,•c)              PatternVariableDeclaration
            (b,•c)              TuplePattern
                     (S,•S)     TupleLiteral                                                                                              */
	let mut x@B {b, ..} = B {a: 10, b: C {c: 20}};                                                                                        /*
    let•mut•x@B•{b,•..}•=•B•{a:•10,•b:•C•{c:•20}};    LetVariableDeclaration
        mut•x@B•{b,•..}                               PatternVariableDeclaration
              B•{b,•..}                               StructPattern
                 b                                    StructPatternPropertyShorthand
                    ..                                RestPattern
                          B•{a:•10,•b:•C•{c:•20}}     StructLiteral
                             a:•10                    StructLiteralProperty
                                10                    Literal
                                    b:•C•{c:•20}      StructLiteralProperty
                                       C•{c:•20}      StructLiteral
                                          c:•20       StructLiteralProperty
                                             20       Literal                                                                             */
	if let Some(x @ B { b: mut b @ C { c }, .. }) = some_b {}                                                                             /*
    if•let•Some(x•@•B•{•b:•mut•b•@•C•{•c•},•..•})•=•some_b•{}    ExpressionStatement, IfBlockExpression
       let•Some(x•@•B•{•b:•mut•b•@•C•{•c•},•..•})•=•some_b       LetScrutinee
           Some(x•@•B•{•b:•mut•b•@•C•{•c•},•..•})                TuplePattern
                x•@•B•{•b:•mut•b•@•C•{•c•},•..•}                 PatternVariableDeclaration
                    B•{•b:•mut•b•@•C•{•c•},•..•}                 StructPattern
                        b:•mut•b•@•C•{•c•}                       StructPatternPropertyDestructured
                           mut•b•@•C•{•c•}                       PatternVariableDeclaration
                                   C•{•c•}                       StructPattern
                                       c                         StructPatternPropertyShorthand
                                            ..                   RestPattern                                                              */
	match x { Some(ref mut _y @ ..) => {} }                                                                                               /*
    match•x•{•Some(ref•mut•_y•@•..)•=>•{}•}    ExpressionStatement, MatchExpression
              Some(ref•mut•_y•@•..)•=>•{}      MatchExpressionCase
              Some(ref•mut•_y•@•..)            TuplePattern
                   ref•mut•_y•@•..             PatternVariableDeclaration
                                ..             RestPattern
                                       {}      BlockExpression                                                                            */
	let ref a @ box ref b = Box::new(NC);                                                                                                 /*
    let•ref•a•@•box•ref•b•=•Box::new(NC);    LetVariableDeclaration
        ref•a•@•box•ref•b                    PatternVariableDeclaration
                box•ref•b                    BoxPattern
                    ref•b                    PatternVariableDeclaration
                            Box::new(NC)     CallExpression
                            Box::new         ExpressionPath                                                                               */
	let a @ b @ c @ d = C;                                                                                                                /*
    let•a•@•b•@•c•@•d•=•C;    LetVariableDeclaration
        a•@•b•@•c•@•d         PatternVariableDeclaration
            b•@•c•@•d         PatternVariableDeclaration
                c•@•d         PatternVariableDeclaration                                                                                  */
	let a @ (b, c) = (C, mk_c());                                                                                                         /*
    let•a•@•(b,•c)•=•(C,•mk_c());    LetVariableDeclaration
        a•@•(b,•c)                   PatternVariableDeclaration
            (b,•c)                   TuplePattern
                     (C,•mk_c())     TupleLiteral
                         mk_c()      CallExpression                                                                                       */
	let a @ P(b, P(c, d)) = P(mk_c(), P(C, C));                                                                                           /*
    let•a•@•P(b,•P(c,•d))•=•P(mk_c(),•P(C,•C));    LetVariableDeclaration
        a•@•P(b,•P(c,•d))                          PatternVariableDeclaration
            P(b,•P(c,•d))                          TuplePattern
                 P(c,•d)                           TuplePattern
                            P(mk_c(),•P(C,•C))     CallExpression
                              mk_c()               CallExpression
                                      P(C,•C)      CallExpression                                                                         */
	let a @ [b, c] = [C, C];                                                                                                              /*
    let•a•@•[b,•c]•=•[C,•C];    LetVariableDeclaration
        a•@•[b,•c]              PatternVariableDeclaration
            [b,•c]              ArrayPattern
                     [C,•C]     ArrayLiteral                                                                                              */
	let a @ &(b, c) = &(C, C);                                                                                                            /*
    let•a•@•&(b,•c)•=•&(C,•C);    LetVariableDeclaration
        a•@•&(b,•c)               PatternVariableDeclaration
            &(b,•c)               ReferencePattern
             (b,•c)               TuplePattern
                      &(C,•C)     ReferenceExpression
                       (C,•C)     TupleLiteral                                                                                            */
	let a @ &(b, &P(c, d)) = &(mk_c(), &P(C, C));                                                                                         /*
    let•a•@•&(b,•&P(c,•d))•=•&(mk_c(),•&P(C,•C));    LetVariableDeclaration
        a•@•&(b,•&P(c,•d))                           PatternVariableDeclaration
            &(b,•&P(c,•d))                           ReferencePattern
             (b,•&P(c,•d))                           TuplePattern
                 &P(c,•d)                            ReferencePattern
                  P(c,•d)                            TuplePattern
                             &(mk_c(),•&P(C,•C))     ReferenceExpression
                              (mk_c(),•&P(C,•C))     TupleLiteral
                               mk_c()                CallExpression
                                       &P(C,•C)      ReferenceExpression
                                        P(C,•C)      CallExpression                                                                       */
    let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());                                                                                  /*
    let•a•@•(mut•b•@•ref•mut•c,•d•@•ref•e)•=•(u(),•u());    LetVariableDeclaration
        a•@•(mut•b•@•ref•mut•c,•d•@•ref•e)                  PatternVariableDeclaration
            (mut•b•@•ref•mut•c,•d•@•ref•e)                  TuplePattern
             mut•b•@•ref•mut•c                              PatternVariableDeclaration
                     ref•mut•c                              PatternVariableDeclaration
                                d•@•ref•e                   PatternVariableDeclaration
                                    ref•e                   PatternVariableDeclaration
                                             (u(),•u())     TupleLiteral
                                              u()           CallExpression
                                                   u()      CallExpression                                                                */
    let a @ ref b = U;                                                                                                                    /*
    let•a•@•ref•b•=•U;    LetVariableDeclaration
        a•@•ref•b         PatternVariableDeclaration
            ref•b         PatternVariableDeclaration                                                                                      */
	let ref mut a @ ref mut b = U;                                                                                                        /*
    let•ref•mut•a•@•ref•mut•b•=•U;    LetVariableDeclaration
        ref•mut•a•@•ref•mut•b         PatternVariableDeclaration
                    ref•mut•b         PatternVariableDeclaration                                                                          */
	let a @ &mut ref mut b = &mut U;                                                                                                      /*
    let•a•@•&mut•ref•mut•b•=•&mut•U;    LetVariableDeclaration
        a•@•&mut•ref•mut•b              PatternVariableDeclaration
            &mut•ref•mut•b              ReferencePattern
                 ref•mut•b              PatternVariableDeclaration
                             &mut•U     ReferenceExpression                                                                               */
    let a @ &mut (ref mut b, ref mut c) = &mut (U, U);                                                                                    /*
    let•a•@•&mut•(ref•mut•b,•ref•mut•c)•=•&mut•(U,•U);    LetVariableDeclaration
        a•@•&mut•(ref•mut•b,•ref•mut•c)                   PatternVariableDeclaration
            &mut•(ref•mut•b,•ref•mut•c)                   ReferencePattern
                 (ref•mut•b,•ref•mut•c)                   TuplePattern
                  ref•mut•b                               PatternVariableDeclaration
                             ref•mut•c                    PatternVariableDeclaration
                                          &mut•(U,•U)     ReferenceExpression
                                               (U,•U)     TupleLiteral                                                                    */
	let a @ NC(b, c) = NC(C, C);                                                                                                          /*
    let•a•@•NC(b,•c)•=•NC(C,•C);    LetVariableDeclaration
        a•@•NC(b,•c)                PatternVariableDeclaration
            NC(b,•c)                TuplePattern
                       NC(C,•C)     CallExpression                                                                                        */
	let a @ NC(b, c @ NC(d, e)) = NC(C, NC(C, C));                                                                                        /*
    let•a•@•NC(b,•c•@•NC(d,•e))•=•NC(C,•NC(C,•C));    LetVariableDeclaration
        a•@•NC(b,•c•@•NC(d,•e))                       PatternVariableDeclaration
            NC(b,•c•@•NC(d,•e))                       TuplePattern
                  c•@•NC(d,•e)                        PatternVariableDeclaration
                      NC(d,•e)                        TuplePattern
                                  NC(C,•NC(C,•C))     CallExpression
                                        NC(C,•C)      CallExpression                                                                      */
	let _a @ _b: u8 = 0;                                                                                                                  /*
    let•_a•@•_b:•u8•=•0;    LetVariableDeclaration
        _a•@•_b             PatternVariableDeclaration
                      0     Literal                                                                                                       */
	let     &_ =         & 1_usize;                                                                                                       /*
    let•••••&_•=•••••••••&•1_usize;    LetVariableDeclaration
            &_                         ReferencePattern
             _                         WildcardPattern
                         &•1_usize     ReferenceExpression
                           1_usize     Literal
                             usize     Identifier                                                                                         */
    let    &&_ =       & & 1_usize;                                                                                                       /*
    let••••&&_•=•••••••&•&•1_usize;    LetVariableDeclaration
           &&_                         ReferencePattern
            &_                         ReferencePattern
             _                         WildcardPattern
                       &•&•1_usize     ReferenceExpression
                         &•1_usize     ReferenceExpression
                           1_usize     Literal
                             usize     Identifier                                                                                         */
    let   &&&_ =     & & & 1_usize;                                                                                                       /*
    let•••&&&_•=•••••&•&•&•1_usize;    LetVariableDeclaration
          &&&_                         ReferencePattern
           &&_                         ReferencePattern
            &_                         ReferencePattern
             _                         WildcardPattern
                     &•&•&•1_usize     ReferenceExpression
                       &•&•1_usize     ReferenceExpression
                         &•1_usize     ReferenceExpression
                           1_usize     Literal
                             usize     Identifier                                                                                         */
    let  & &&_ =     & & & 1_usize;                                                                                                       /*
    let••&•&&_•=•••••&•&•&•1_usize;    LetVariableDeclaration
         &•&&_                         ReferencePattern
           &&_                         ReferencePattern
            &_                         ReferencePattern
             _                         WildcardPattern
                     &•&•&•1_usize     ReferenceExpression
                       &•&•1_usize     ReferenceExpression
                         &•1_usize     ReferenceExpression
                           1_usize     Literal
                             usize     Identifier                                                                                         */
    let  &&&&_ =   & & & & 1_usize;                                                                                                       /*
    let••&&&&_•=•••&•&•&•&•1_usize;    LetVariableDeclaration
         &&&&_                         ReferencePattern
          &&&_                         ReferencePattern
           &&_                         ReferencePattern
            &_                         ReferencePattern
             _                         WildcardPattern
                   &•&•&•&•1_usize     ReferenceExpression
                     &•&•&•1_usize     ReferenceExpression
                       &•&•1_usize     ReferenceExpression
                         &•1_usize     ReferenceExpression
                           1_usize     Literal
                             usize     Identifier                                                                                         */
    let & &&&_ =   & & & & 1_usize;                                                                                                       /*
    let•&•&&&_•=•••&•&•&•&•1_usize;    LetVariableDeclaration
        &•&&&_                         ReferencePattern
          &&&_                         ReferencePattern
           &&_                         ReferencePattern
            &_                         ReferencePattern
             _                         WildcardPattern
                   &•&•&•&•1_usize     ReferenceExpression
                     &•&•&•1_usize     ReferenceExpression
                       &•&•1_usize     ReferenceExpression
                         &•1_usize     ReferenceExpression
                           1_usize     Literal
                             usize     Identifier                                                                                         */
    let &&&&&_ = & & & & & 1_usize;                                                                                                       /*
    let•&&&&&_•=•&•&•&•&•&•1_usize;    LetVariableDeclaration
        &&&&&_                         ReferencePattern
         &&&&_                         ReferencePattern
          &&&_                         ReferencePattern
           &&_                         ReferencePattern
            &_                         ReferencePattern
             _                         WildcardPattern
                 &•&•&•&•&•1_usize     ReferenceExpression
                   &•&•&•&•1_usize     ReferenceExpression
                     &•&•&•1_usize     ReferenceExpression
                       &•&•1_usize     ReferenceExpression
                         &•1_usize     ReferenceExpression
                           1_usize     Literal
                             usize     Identifier                                                                                         */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 3
// Parsed Nodes: 899
// state_rollbacks: 8
// Total '.charCodeAt()' calls: 4218 (34% re-reads)
// Unnecessary 'skip_whitespace()' calls: 463
// source: "../../samples/patterns/pattern.rs"