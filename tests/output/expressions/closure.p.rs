fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>                                                                                                     */
	let lam = |(a, ref b, c, ref mut d): (X, X, X, X)| {};                                                                                /*
    let•lam•=•|(a,•ref•b,•c,•ref•mut•d):•(X,•X,•X,•X)|•{};    LetVariableDeclaration
              |(a,•ref•b,•c,•ref•mut•d):•(X,•X,•X,•X)|•{}     ClosureFunctionExpression
               (a,•ref•b,•c,•ref•mut•d):•(X,•X,•X,•X)         ClosureFunctionParameterDeclaration
               (a,•ref•b,•c,•ref•mut•d)                       TuplePattern
                   ref•b                                      PatternVariableDeclaration
                             ref•mut•d                        PatternVariableDeclaration
                                         (X,•X,•X,•X)         TypeTuple
                                                       {}     BlockExpression                                                             */
	let x = |_: ()| ();                                                                                                                   /*
    let•x•=•|_:•()|•();    LetVariableDeclaration
            |_:•()|•()     ClosureFunctionExpression
             _:•()         ClosureFunctionParameterDeclaration
             _             WildcardPattern
                ()         TypeTuple
                    ()     TupleLiteral                                                                                                   */
	let y = || x(());                                                                                                                     /*
    let•y•=•||•x(());    LetVariableDeclaration
            ||•x(())     ClosureFunctionExpression
               x(())     CallExpression
                 ()      TupleLiteral                                                                                                     */
	let mut x = |_: ()| {                                                                                                                 /*
    let•mut•x•=•|_:•()|•{↲    <LetVariableDeclaration>
        mut•x                 PatternVariableDeclaration
                |_:•()|•{↲    <ClosureFunctionExpression>
                 _:•()        ClosureFunctionParameterDeclaration
                 _            WildcardPattern
                    ()        TypeTuple
                        {↲    <BlockExpression>                                                                                           */
        outer = 4;                                                                                                                        /*
        outer•=•4;    ExpressionStatement
        outer•=•4     ReassignmentExpression
                4     Literal                                                                                                             */
        ()                                                                                                                                /*
        ()    ExpressionStatement, TupleLiteral                                                                                           */
    };                                                                                                                                    /*
••••};    </LetVariableDeclaration>
••••}     </ClosureFunctionExpression>, </BlockExpression>                                                                                */
	let x = move |_: ()| {                                                                                                                /*
    let•x•=•move•|_:•()|•{↲    <LetVariableDeclaration>
            move•|_:•()|•{↲    <ClosureFunctionExpression>
                  _:•()        ClosureFunctionParameterDeclaration
                  _            WildcardPattern
                     ()        TypeTuple
                         {↲    <BlockExpression>                                                                                          */
        let inner = outer;                                                                                                                /*
        let•inner•=•outer;    LetVariableDeclaration                                                                                      */
        ()                                                                                                                                /*
        ()    ExpressionStatement, TupleLiteral                                                                                           */
    };                                                                                                                                    /*
••••};    </LetVariableDeclaration>
••••}     </ClosureFunctionExpression>, </BlockExpression>                                                                                */
	const VTABLE: &'static VTable<DST> = &VTable { _to_dst_ptr: |_: *mut ()| unsafe { std::mem::zeroed() }, };                            /*
    const•VTABLE:•&'static•VTable<DST>•=•&VTable•{•_to_dst_ptr:•|_:•*mut•()|•unsafe•{•std::mem::zeroed()•},•};    ConstVariableDeclaration
                  &'static•VTable<DST>                                                                            TypeReference
                   'static                                                                                        LtStatic
                           VTable<DST>                                                                            TypeCall
                                         &VTable•{•_to_dst_ptr:•|_:•*mut•()|•unsafe•{•std::mem::zeroed()•},•}     ReferenceExpression
                                          VTable•{•_to_dst_ptr:•|_:•*mut•()|•unsafe•{•std::mem::zeroed()•},•}     StructLiteral
                                                   _to_dst_ptr:•|_:•*mut•()|•unsafe•{•std::mem::zeroed()•}        StructLiteralProperty
                                                                |_:•*mut•()|•unsafe•{•std::mem::zeroed()•}        ClosureFunctionExpression
                                                                 _:•*mut•()                                       ClosureFunctionParameterDeclaration
                                                                 _                                                WildcardPattern
                                                                    *mut•()                                       TypeDereferenceMut
                                                                         ()                                       TypeTuple
                                                                             unsafe•{•std::mem::zeroed()•}        BlockExpression
                                                                                      std::mem::zeroed()          ExpressionStatement, CallExpression
                                                                                      std::mem::zeroed            ExpressionPath
                                                                                      std::mem                    ExpressionPath          */
    let z = a(&mut |x| x - 22);                                                                                                           /*
    let•z•=•a(&mut•|x|•x•-•22);    LetVariableDeclaration
            a(&mut•|x|•x•-•22)     CallExpression
              &mut•|x|•x•-•22      ReferenceExpression
                   |x|•x•-•22      ClosureFunctionExpression
                    x              ClosureFunctionParameterDeclaration
                       x•-•22      OperationExpression
                           22      Literal                                                                                                */
    let mut unboxed = || {};                                                                                                              /*
    let•mut•unboxed•=•||•{};    LetVariableDeclaration
        mut•unboxed             PatternVariableDeclaration
                      ||•{}     ClosureFunctionExpression
                         {}     BlockExpression                                                                                           */
    Box::new(move |y| { x + y });                                                                                                         /*
    Box::new(move•|y|•{•x•+•y•});    ExpressionStatement
    Box::new(move•|y|•{•x•+•y•})     CallExpression
    Box::new                         ExpressionPath
             move•|y|•{•x•+•y•}      ClosureFunctionExpression
                   y                 ClosureFunctionParameterDeclaration
                      {•x•+•y•}      BlockExpression
                        x•+•y        ExpressionStatement, OperationExpression                                                             */
    s(|f| (*f)(), Box::new(|| {}));                                                                                                       /*
    s(|f|•(*f)(),•Box::new(||•{}));    ExpressionStatement
    s(|f|•(*f)(),•Box::new(||•{}))     CallExpression
      |f|•(*f)()                       ClosureFunctionExpression
       f                               ClosureFunctionParameterDeclaration
          (*f)()                       CallExpression
           *f                          DereferenceExpression
                  Box::new(||•{})      CallExpression
                  Box::new             ExpressionPath
                           ||•{}       ClosureFunctionExpression
                              {}       BlockExpression                                                                                    */
	(0..42).e(|_x| match E(()) as R<(), _> { O(()) => s.push(()), E(_) => (), });                                                         /*
    (0..42).e(|_x|•match•E(())•as•R<(),•_>•{•O(())•=>•s.push(()),•E(_)•=>•(),•});    ExpressionStatement
    (0..42).e(|_x|•match•E(())•as•R<(),•_>•{•O(())•=>•s.push(()),•E(_)•=>•(),•})     CallExpression
     0..42                                                                           RangeLiteral
     0                                                                               Literal
        42                                                                           Literal
              |_x|•match•E(())•as•R<(),•_>•{•O(())•=>•s.push(()),•E(_)•=>•(),•}      ClosureFunctionExpression
               _x                                                                    ClosureFunctionParameterDeclaration
                   match•E(())•as•R<(),•_>•{•O(())•=>•s.push(()),•E(_)•=>•(),•}      MatchExpression
                         E(())•as•R<(),•_>                                           ExpressionAsTypeCast
                         E(())                                                       CallExpression
                           ()                                                        TupleLiteral
                                  R<(),•_>                                           TypeCall
                                    ()                                               TypeTuple
                                        _                                            TypeInferred
                                             O(())•=>•s.push(())                     MatchExpressionCase
                                             O(())                                   TuplePattern
                                               ()                                    TuplePattern
                                                      s.push(())                     CallExpression
                                                             ()                      TupleLiteral
                                                                  E(_)•=>•()         MatchExpressionCase
                                                                  E(_)               TuplePattern
                                                                    _                WildcardPattern
                                                                          ()         TupleLiteral                                         */
	<()>::a(|| ());                                                                                                                       /*
    <()>::a(||•());    ExpressionStatement
    <()>::a(||•())     CallExpression
    <()>::a            ExpressionPath
    <()>               ExpressionTypeSelector
     ()                TypeTuple
            ||•()      ClosureFunctionExpression
               ()      TupleLiteral                                                                                                       */
    let f: &mut dyn FnMut<(_,), E = ()> = &mut |_: <() as Lt<'_>>::T| {};                                                                 /*
    let•f:•&mut•dyn•FnMut<(_,),•E•=•()>•=•&mut•|_:•<()•as•Lt<'_>>::T|•{};    LetVariableDeclaration
           &mut•dyn•FnMut<(_,),•E•=•()>                                      TypeReference
                dyn•FnMut<(_,),•E•=•()>                                      TypeDynBounds
                    FnMut<(_,),•E•=•()>                                      TypeTraitBound, TypeCall
                          (_,)                                               TypeTuple
                           _                                                 TypeInferred
                                E•=•()                                       TypeCallNamedArgument
                                    ()                                       TypeTuple
                                          &mut•|_:•<()•as•Lt<'_>>::T|•{}     ReferenceExpression
                                               |_:•<()•as•Lt<'_>>::T|•{}     ClosureFunctionExpression
                                                _:•<()•as•Lt<'_>>::T         ClosureFunctionParameterDeclaration
                                                _                            WildcardPattern
                                                   <()•as•Lt<'_>>::T         TypePath
                                                   <()•as•Lt<'_>>            ExpressionTypeSelector
                                                    ()                       TypeTuple
                                                          Lt<'_>             TypeCall
                                                             '_              LtElided
                                                                      {}     BlockExpression                                              */
	Box::new(move |v| { (|_| e.d()())(v); X }) as Box<dyn Fn(i32) -> Q<i32>>;                                                             /*
    Box::new(move•|v|•{•(|_|•e.d()())(v);•X•})•as•Box<dyn•Fn(i32)•->•Q<i32>>;    ExpressionStatement
    Box::new(move•|v|•{•(|_|•e.d()())(v);•X•})•as•Box<dyn•Fn(i32)•->•Q<i32>>     ExpressionAsTypeCast
    Box::new(move•|v|•{•(|_|•e.d()())(v);•X•})                                   CallExpression
    Box::new                                                                     ExpressionPath
             move•|v|•{•(|_|•e.d()())(v);•X•}                                    ClosureFunctionExpression
                   v                                                             ClosureFunctionParameterDeclaration
                      {•(|_|•e.d()())(v);•X•}                                    BlockExpression
                        (|_|•e.d()())(v);                                        ExpressionStatement
                        (|_|•e.d()())(v)                                         CallExpression
                         |_|•e.d()()                                             ClosureFunctionExpression
                          _                                                      ClosureFunctionParameterDeclaration, WildcardPattern
                             e.d()()                                             CallExpression
                             e.d()                                               CallExpression
                                          X                                      ExpressionStatement
                                                  Box<dyn•Fn(i32)•->•Q<i32>>     TypeCall
                                                      dyn•Fn(i32)•->•Q<i32>      TypeDynBounds
                                                          Fn(i32)•->•Q<i32>      TypeTraitBound, TypeFunction
                                                                     Q<i32>      TypeCall                                                 */
	let _c = || match b.0.c(1) as D<(), _> { _ => 0, };                                                                                   /*
    let•_c•=•||•match•b.0.c(1)•as•D<(),•_>•{•_•=>•0,•};    LetVariableDeclaration
             ||•match•b.0.c(1)•as•D<(),•_>•{•_•=>•0,•}     ClosureFunctionExpression
                match•b.0.c(1)•as•D<(),•_>•{•_•=>•0,•}     MatchExpression
                      b.0.c(1)•as•D<(),•_>                 ExpressionAsTypeCast
                      b.0.c(1)                             CallExpression
                      b.0                                  MemberExpression
                        0                                  Index
                            1                              Literal
                                  D<(),•_>                 TypeCall
                                    ()                     TypeTuple
                                        _                  TypeInferred
                                             _•=>•0        MatchExpressionCase
                                             _             WildcardPattern
                                                  0        Literal                                                                        */
    let f: &dyn Fn(i32) -> _ = &|x| x + x;                                                                                                /*
    let•f:•&dyn•Fn(i32)•->•_•=•&|x|•x•+•x;    LetVariableDeclaration
           &dyn•Fn(i32)•->•_                  TypeReference
            dyn•Fn(i32)•->•_                  TypeDynBounds
                Fn(i32)•->•_                  TypeTraitBound, TypeFunction
                           _                  TypeInferred
                               &|x|•x•+•x     ReferenceExpression
                                |x|•x•+•x     ClosureFunctionExpression
                                 x            ClosureFunctionParameterDeclaration
                                    x•+•x     OperationExpression                                                                         */
	let f = |x: u32| -> u32 { 1 };                                                                                                        /*
    let•f•=•|x:•u32|•->•u32•{•1•};    LetVariableDeclaration
            |x:•u32|•->•u32•{•1•}     ClosureFunctionExpression
             x:•u32                   ClosureFunctionParameterDeclaration
                            {•1•}     BlockExpression
                              1       ExpressionStatement, Literal                                                                        */
	for f in &[d, g, |x| x] { a!("{}", f(6)); }                                                                                           /*
    for•f•in•&[d,•g,•|x|•x]•{•a!("{}",•f(6));•}    ExpressionStatement, ForInBlockExpression
             &[d,•g,•|x|•x]                        ReferenceExpression
              [d,•g,•|x|•x]                        ArrayLiteral
                     |x|•x                         ClosureFunctionExpression
                      x                            ClosureFunctionParameterDeclaration
                              a!("{}",•f(6));      ExpressionStatement
                              a!("{}",•f(6))       MacroInvocation
                                 "{}"              Literal
                                     ,             PunctuationToken
                                        (6)        DelimGroup
                                         6         Literal                                                                                */
	(|| { (|| { c.d })(); (move || { a.b })(); })();                                                                                      /*
    (||•{•(||•{•c.d•})();•(move•||•{•a.b•})();•})();    ExpressionStatement
    (||•{•(||•{•c.d•})();•(move•||•{•a.b•})();•})()     CallExpression
     ||•{•(||•{•c.d•})();•(move•||•{•a.b•})();•}        ClosureFunctionExpression
        {•(||•{•c.d•})();•(move•||•{•a.b•})();•}        BlockExpression
          (||•{•c.d•})();                               ExpressionStatement
          (||•{•c.d•})()                                CallExpression
           ||•{•c.d•}                                   ClosureFunctionExpression
              {•c.d•}                                   BlockExpression
                c.d                                     ExpressionStatement, MemberExpression
                          (move•||•{•a.b•})();          ExpressionStatement
                          (move•||•{•a.b•})()           CallExpression
                           move•||•{•a.b•}              ClosureFunctionExpression
                                   {•a.b•}              BlockExpression
                                     a.b                ExpressionStatement, MemberExpression                                             */
	let q = a.e(async move { b(move || async move { d!() }) });                                                                           /*
    let•q•=•a.e(async•move•{•b(move•||•async•move•{•d!()•})•});    LetVariableDeclaration
            a.e(async•move•{•b(move•||•async•move•{•d!()•})•})     CallExpression
                async•move•{•b(move•||•async•move•{•d!()•})•}      BlockExpression
                             b(move•||•async•move•{•d!()•})        ExpressionStatement, CallExpression
                               move•||•async•move•{•d!()•}         ClosureFunctionExpression
                                       async•move•{•d!()•}         BlockExpression
                                                    d!()           ExpressionStatement, MacroInvocation                                   */
    let g = {||z(i)};                                                                                                                     /*
    let•g•=•{||z(i)};    LetVariableDeclaration
            {||z(i)}     BlockExpression
             ||z(i)      ExpressionStatement, ClosureFunctionExpression
               z(i)      CallExpression                                                                                                   */
    let _ = || a.e(async { r });                                                                                                          /*
    let•_•=•||•a.e(async•{•r•});    LetVariableDeclaration
        _                           WildcardPattern
            ||•a.e(async•{•r•})     ClosureFunctionExpression
               a.e(async•{•r•})     CallExpression
                   async•{•r•}      BlockExpression
                           r        ExpressionStatement                                                                                   */
    let _ = E(0).d().q(|ref _a| true);                                                                                                    /*
    let•_•=•E(0).d().q(|ref•_a|•true);    LetVariableDeclaration
        _                                 WildcardPattern
            E(0).d().q(|ref•_a|•true)     CallExpression
            E(0).d()                      CallExpression
            E(0)                          CallExpression
              0                           Literal
                       |ref•_a|•true      ClosureFunctionExpression
                        ref•_a            ClosureFunctionParameterDeclaration, PatternVariableDeclaration
                                true      Literal                                                                                         */
	let _ = !(4..5).a(|x| x == 1 || x == 3 || x == 5);                                                                                    /*
    let•_•=•!(4..5).a(|x|•x•==•1•||•x•==•3•||•x•==•5);    LetVariableDeclaration
        _                                                 WildcardPattern
            !(4..5).a(|x|•x•==•1•||•x•==•3•||•x•==•5)     NotExpression
             (4..5).a(|x|•x•==•1•||•x•==•3•||•x•==•5)     CallExpression
              4..5                                        RangeLiteral
              4                                           Literal
                 5                                        Literal
                      |x|•x•==•1•||•x•==•3•||•x•==•5      ClosureFunctionExpression
                       x                                  ClosureFunctionParameterDeclaration
                          x•==•1•||•x•==•3•||•x•==•5      OrExpression
                          x•==•1•||•x•==•3                OrExpression
                          x•==•1                          ComparisonExpression
                               1                          Literal
                                    x•==•3                ComparisonExpression
                                         3                Literal
                                              x•==•5      ComparisonExpression
                                                   5      Literal                                                                         */
	let _ = !(1..3).a(|x| [1, 2, 3].b(&x));                                                                                               /*
    let•_•=•!(1..3).a(|x|•[1,•2,•3].b(&x));    LetVariableDeclaration
        _                                      WildcardPattern
            !(1..3).a(|x|•[1,•2,•3].b(&x))     NotExpression
             (1..3).a(|x|•[1,•2,•3].b(&x))     CallExpression
              1..3                             RangeLiteral
              1                                Literal
                 3                             Literal
                      |x|•[1,•2,•3].b(&x)      ClosureFunctionExpression
                       x                       ClosureFunctionParameterDeclaration
                          [1,•2,•3].b(&x)      CallExpression
                          [1,•2,•3]            ArrayLiteral
                           1                   Literal
                              2                Literal
                                 3             Literal
                                      &x       ReferenceExpression                                                                        */
	let _ = !(1..3).a(|x| x == 0 || [1, 2, 3].b(&x));                                                                                     /*
    let•_•=•!(1..3).a(|x|•x•==•0•||•[1,•2,•3].b(&x));    LetVariableDeclaration
        _                                                WildcardPattern
            !(1..3).a(|x|•x•==•0•||•[1,•2,•3].b(&x))     NotExpression
             (1..3).a(|x|•x•==•0•||•[1,•2,•3].b(&x))     CallExpression
              1..3                                       RangeLiteral
              1                                          Literal
                 3                                       Literal
                      |x|•x•==•0•||•[1,•2,•3].b(&x)      ClosureFunctionExpression
                       x                                 ClosureFunctionParameterDeclaration
                          x•==•0•||•[1,•2,•3].b(&x)      OrExpression
                          x•==•0                         ComparisonExpression
                               0                         Literal
                                    [1,•2,•3].b(&x)      CallExpression
                                    [1,•2,•3]            ArrayLiteral
                                     1                   Literal
                                        2                Literal
                                           3             Literal
                                                &x       ReferenceExpression                                                              */
	let _ = !(1..3).a(|x| [1, 2, 3].b(&x) || x == 0);                                                                                     /*
    let•_•=•!(1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0);    LetVariableDeclaration
        _                                                WildcardPattern
            !(1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0)     NotExpression
             (1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0)     CallExpression
              1..3                                       RangeLiteral
              1                                          Literal
                 3                                       Literal
                      |x|•[1,•2,•3].b(&x)•||•x•==•0      ClosureFunctionExpression
                       x                                 ClosureFunctionParameterDeclaration
                          [1,•2,•3].b(&x)•||•x•==•0      OrExpression
                          [1,•2,•3].b(&x)                CallExpression
                          [1,•2,•3]                      ArrayLiteral
                           1                             Literal
                              2                          Literal
                                 3                       Literal
                                      &x                 ReferenceExpression
                                             x•==•0      ComparisonExpression
                                                  0      Literal                                                                          */
    let _ = !(1..3).a(|x| [1, 2, 3].b(&x) || x == 0 || [4, 5, 6].c(&x) || x == -1);                                                       /*
    let•_•=•!(1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1);    LetVariableDeclaration
        _                                                                              WildcardPattern
            !(1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1)     NotExpression
             (1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1)     CallExpression
              1..3                                                                     RangeLiteral
              1                                                                        Literal
                 3                                                                     Literal
                      |x|•[1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1      ClosureFunctionExpression
                       x                                                               ClosureFunctionParameterDeclaration
                          [1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1      OrExpression
                          [1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)                 OrExpression
                          [1,•2,•3].b(&x)•||•x•==•0                                    OrExpression
                          [1,•2,•3].b(&x)                                              CallExpression
                          [1,•2,•3]                                                    ArrayLiteral
                           1                                                           Literal
                              2                                                        Literal
                                 3                                                     Literal
                                      &x                                               ReferenceExpression
                                             x•==•0                                    ComparisonExpression
                                                  0                                    Literal
                                                       [4,•5,•6].c(&x)                 CallExpression
                                                       [4,•5,•6]                       ArrayLiteral
                                                        4                              Literal
                                                           5                           Literal
                                                              6                        Literal
                                                                   &x                  ReferenceExpression
                                                                          x•==•-1      ComparisonExpression
                                                                               -1      MinusExpression
                                                                                1      Literal                                            */
	let hash: &Fn(&&Block) -> u64 = &|block| -> u64 { 1 };                                                                                /*
    let•hash:•&Fn(&&Block)•->•u64•=•&|block|•->•u64•{•1•};    LetVariableDeclaration
              &Fn(&&Block)•->•u64                             TypeReference
               Fn(&&Block)•->•u64                             TypeFunction
                  &&Block                                     TypeReference
                   &Block                                     TypeReference
                                    &|block|•->•u64•{•1•}     ReferenceExpression
                                     |block|•->•u64•{•1•}     ClosureFunctionExpression
                                      block                   ClosureFunctionParameterDeclaration
                                                    {•1•}     BlockExpression
                                                      1       ExpressionStatement, Literal                                                */
	if outer_guard.map_or(true, |(Guard::If(e) | Guard::IfLet(_, e))| !is_local_used(cx, *e, binding_id)) {}                              /*
    if•outer_guard.map_or(true,•|(Guard::If(e)•|•Guard::IfLet(_,•e))|•!is_local_used(cx,•*e,•binding_id))•{}    ExpressionStatement, IfBlockExpression
       outer_guard.map_or(true,•|(Guard::If(e)•|•Guard::IfLet(_,•e))|•!is_local_used(cx,•*e,•binding_id))       CallExpression
                          true                                                                                  Literal
                                |(Guard::If(e)•|•Guard::IfLet(_,•e))|•!is_local_used(cx,•*e,•binding_id)        ClosureFunctionExpression
                                 (Guard::If(e)•|•Guard::IfLet(_,•e))                                            ClosureFunctionParameterDeclaration
                                  Guard::If(e)•|•Guard::IfLet(_,•e)                                             UnionPattern
                                  Guard::If(e)                                                                  TuplePattern
                                  Guard::If                                                                     ExpressionPath
                                                 Guard::IfLet(_,•e)                                             TuplePattern
                                                 Guard::IfLet                                                   ExpressionPath
                                                              _                                                 WildcardPattern
                                                                      !is_local_used(cx,•*e,•binding_id)        NotExpression
                                                                       is_local_used(cx,•*e,•binding_id)        CallExpression
                                                                                         *e                     DereferenceExpression     */
	a =| | b;                                                                                                                             /*
    a•=|•|•b;    ExpressionStatement
    a•=|•|•b     ReassignmentExpression
       |•|•b     ClosureFunctionExpression                                                                                                */
	[                                                                                                                                     /*
    [↲    <ExpressionStatement>, <ArrayLiteral>                                                                                           */
		foo(|| ()),                                                                                                                       /*
        foo(||•())    CallExpression
            ||•()     ClosureFunctionExpression
               ()     TupleLiteral                                                                                                        */
		|x: u32| x + 1,                                                                                                                   /*
        |x:•u32|•x•+•1    ClosureFunctionExpression
         x:•u32           ClosureFunctionParameterDeclaration
                 x•+•1    OperationExpression
                     1    Literal                                                                                                         */
		(|| Box::new(|| {}) as Box<dyn Fn()>)(),                                                                                          /*
        (||•Box::new(||•{})•as•Box<dyn•Fn()>)()    CallExpression
         ||•Box::new(||•{})•as•Box<dyn•Fn()>       ClosureFunctionExpression
            Box::new(||•{})•as•Box<dyn•Fn()>       ExpressionAsTypeCast
            Box::new(||•{})                        CallExpression
            Box::new                               ExpressionPath
                     ||•{}                         ClosureFunctionExpression
                        {}                         BlockExpression
                               Box<dyn•Fn()>       TypeCall
                                   dyn•Fn()        TypeDynBounds
                                       Fn()        TypeTraitBound, TypeFunction                                                           */
		|_: T| 3,                                                                                                                         /*
        |_:•T|•3    ClosureFunctionExpression
         _:•T       ClosureFunctionParameterDeclaration
         _          WildcardPattern
               3    Literal                                                                                                               */
		move |x: isize, y| x + y + z,                                                                                                     /*
        move•|x:•isize,•y|•x•+•y•+•z    ClosureFunctionExpression
              x:•isize                  ClosureFunctionParameterDeclaration
                        y               ClosureFunctionParameterDeclaration
                           x•+•y•+•z    OperationExpression
                           x•+•y        OperationExpression                                                                               */
		&mut || 22,                                                                                                                       /*
        &mut•||•22    ReferenceExpression
             ||•22    ClosureFunctionExpression
                22    Literal                                                                                                             */
		&|| 22,                                                                                                                           /*
        &||•22    ReferenceExpression
         ||•22    ClosureFunctionExpression
            22    Literal                                                                                                                 */
		|| x += 1,                                                                                                                        /*
        ||•x•+=•1    ClosureFunctionExpression
           x•+=•1    ReassignmentOperationExpression
                1    Literal                                                                                                              */
		call(&|| {}, ()),                                                                                                                 /*
        call(&||•{},•())    CallExpression
             &||•{}         ReferenceExpression
              ||•{}         ClosureFunctionExpression
                 {}         BlockExpression
                     ()     TupleLiteral                                                                                                  */
		<()>::drive(|| ()),                                                                                                               /*
        <()>::drive(||•())    CallExpression
        <()>::drive           ExpressionPath
        <()>                  ExpressionTypeSelector
         ()                   TypeTuple
                    ||•()     ClosureFunctionExpression
                       ()     TupleLiteral                                                                                                */
		h2(|_: (), _: (), _: (), _: ()| {}),                                                                                              /*
        h2(|_:•(),•_:•(),•_:•(),•_:•()|•{})    CallExpression
           |_:•(),•_:•(),•_:•(),•_:•()|•{}     ClosureFunctionExpression
            _:•()                              ClosureFunctionParameterDeclaration
            _                                  WildcardPattern
               ()                              TypeTuple
                   _:•()                       ClosureFunctionParameterDeclaration
                   _                           WildcardPattern
                      ()                       TypeTuple
                          _:•()                ClosureFunctionParameterDeclaration
                          _                    WildcardPattern
                             ()                TypeTuple
                                 _:•()         ClosureFunctionParameterDeclaration
                                 _             WildcardPattern
                                    ()         TypeTuple
                                        {}     BlockExpression                                                                            */
		move |a: isize, b| { a + b },                                                                                                     /*
        move•|a:•isize,•b|•{•a•+•b•}    ClosureFunctionExpression
              a:•isize                  ClosureFunctionParameterDeclaration
                        b               ClosureFunctionParameterDeclaration
                           {•a•+•b•}    BlockExpression
                             a•+•b      ExpressionStatement, OperationExpression                                                          */
		move |a: isize, b| { z; zz; a + b },                                                                                              /*
        move•|a:•isize,•b|•{•z;•zz;•a•+•b•}    ClosureFunctionExpression
              a:•isize                         ClosureFunctionParameterDeclaration
                        b                      ClosureFunctionParameterDeclaration
                           {•z;•zz;•a•+•b•}    BlockExpression
                             z;                ExpressionStatement
                                zz;            ExpressionStatement
                                    a•+•b      ExpressionStatement, OperationExpression                                                   */
		|x: usize| x * 2,                                                                                                                 /*
        |x:•usize|•x•*•2    ClosureFunctionExpression
         x:•usize           ClosureFunctionParameterDeclaration
                   x•*•2    OperationExpression
                       2    Literal                                                                                                       */
		|x: usize| { x } * 2,                                                                                                             /*
        |x:•usize|•{•x•}•*•2    ClosureFunctionExpression
         x:•usize               ClosureFunctionParameterDeclaration
                   {•x•}•*•2    OperationExpression
                   {•x•}        BlockExpression
                     x          ExpressionStatement
                           2    Literal                                                                                                   */
		|x: usize| { x } (),                                                                                                              /*
        |x:•usize|•{•x•}•()    ClosureFunctionExpression
         x:•usize              ClosureFunctionParameterDeclaration
                   {•x•}•()    CallExpression
                   {•x•}       BlockExpression
                     x         ExpressionStatement                                                                                        */
		|x| lib::d!(x),                                                                                                                   /*
        |x|•lib::d!(x)    ClosureFunctionExpression
         x                ClosureFunctionParameterDeclaration
            lib::d!(x)    MacroInvocation
            lib::d        ExpressionPath                                                                                                  */
		|x| match x { a => { g(a) } },                                                                                                    /*
        |x|•match•x•{•a•=>•{•g(a)•}•}    ClosureFunctionExpression
         x                               ClosureFunctionParameterDeclaration
            match•x•{•a•=>•{•g(a)•}•}    MatchExpression
                      a•=>•{•g(a)•}      MatchExpressionCase
                           {•g(a)•}      BlockExpression
                             g(a)        ExpressionStatement, CallExpression                                                              */
		|x| d!(x),                                                                                                                        /*
        |x|•d!(x)    ClosureFunctionExpression
         x           ClosureFunctionParameterDeclaration
            d!(x)    MacroInvocation                                                                                                      */
		|_| async { () },                                                                                                                 /*
        |_|•async•{•()•}    ClosureFunctionExpression
         _                  ClosureFunctionParameterDeclaration, WildcardPattern
            async•{•()•}    BlockExpression
                    ()      ExpressionStatement, TupleLiteral                                                                             */
		|x,y| {},                                                                                                                         /*
        |x,y|•{}    ClosureFunctionExpression
         x          ClosureFunctionParameterDeclaration
           y        ClosureFunctionParameterDeclaration
              {}    BlockExpression                                                                                                       */
		|x:&u64, y:&u64| {},                                                                                                              /*
        |x:&u64,•y:&u64|•{}    ClosureFunctionExpression
         x:&u64                ClosureFunctionParameterDeclaration
           &u64                TypeReference
                 y:&u64        ClosureFunctionParameterDeclaration
                   &u64        TypeReference
                         {}    BlockExpression                                                                                            */
		|x:&u64, y| {},                                                                                                                   /*
        |x:&u64,•y|•{}    ClosureFunctionExpression
         x:&u64           ClosureFunctionParameterDeclaration
           &u64           TypeReference
                 y        ClosureFunctionParameterDeclaration
                    {}    BlockExpression                                                                                                 */
		|x, y:&u64| {},                                                                                                                   /*
        |x,•y:&u64|•{}    ClosureFunctionExpression
         x                ClosureFunctionParameterDeclaration
            y:&u64        ClosureFunctionParameterDeclaration
              &u64        TypeReference
                    {}    BlockExpression                                                                                                 */
		match 0 { 2 => |a| 2, 1 => 0, },                                                                                                  /*
        match•0•{•2•=>•|a|•2,•1•=>•0,•}    MatchExpression
              0                            Literal
                  2•=>•|a|•2               MatchExpressionCase
                  2                        Literal
                       |a|•2               ClosureFunctionExpression
                        a                  ClosureFunctionParameterDeclaration
                           2               Literal
                              1•=>•0       MatchExpressionCase
                              1            Literal
                                   0       Literal                                                                                        */
		[b, |a| 2],                                                                                                                       /*
        [b,•|a|•2]    ArrayLiteral
            |a|•2     ClosureFunctionExpression
             a        ClosureFunctionParameterDeclaration
                2     Literal                                                                                                             */
		[|a| 2, b],                                                                                                                       /*
        [|a|•2,•b]    ArrayLiteral
         |a|•2        ClosureFunctionExpression
          a           ClosureFunctionParameterDeclaration
             2        Literal                                                                                                             */
		async || 1,                                                                                                                       /*
        async•||•1    ClosureFunctionExpression
                 1    Literal                                                                                                             */
		|ctx: Ctx<(String, String)>| -> io::Result<Response> {                                                                            /*
        |ctx:•Ctx<(String,•String)>|•->•io::Result<Response>•{↲    <ClosureFunctionExpression>
         ctx:•Ctx<(String,•String)>                                ClosureFunctionParameterDeclaration
              Ctx<(String,•String)>                                TypeCall
                  (String,•String)                                 TypeTuple
                                        io::Result<Response>       TypeCall
                                        io::Result                 TypePath
                                                             {↲    <BlockExpression>                                                      */
			Ok(Response::new().with_body(ctx.params.0))                                                                                   /*
            Ok(Response::new().with_body(ctx.params.0))    ExpressionStatement, CallExpression
               Response::new().with_body(ctx.params.0)     CallExpression
               Response::new()                             CallExpression
               Response::new                               ExpressionPath
                                         ctx.params.0      MemberExpression
                                         ctx.params        MemberExpression
                                                    0      Index                                                                          */
		},                                                                                                                                /*
   ╚╚}    </ClosureFunctionExpression>, </BlockExpression>                                                                                */
		rayon::join(                                                                                                                      /*
        rayon::join(↲    <CallExpression>
        rayon::join      ExpressionPath                                                                                                   */
			|| recurse(left, is_less, pred, limit),                                                                                       /*
            ||•recurse(left,•is_less,•pred,•limit)    ClosureFunctionExpression
               recurse(left,•is_less,•pred,•limit)    CallExpression                                                                      */
			|| recurse(right, is_less, Some(pivot), limit),                                                                               /*
            ||•recurse(right,•is_less,•Some(pivot),•limit)    ClosureFunctionExpression
               recurse(right,•is_less,•Some(pivot),•limit)    CallExpression
                                       Some(pivot)            CallExpression                                                              */
		),                                                                                                                                /*
   ╚╚)    </CallExpression>                                                                                                               */
		rayon::join(                                                                                                                      /*
        rayon::join(↲    <CallExpression>
        rayon::join      ExpressionPath                                                                                                   */
			1,                                                                                                                            /*
            1    Literal                                                                                                                  */
			|| recurse(left, is_less, pred, limit),                                                                                       /*
            ||•recurse(left,•is_less,•pred,•limit)    ClosureFunctionExpression
               recurse(left,•is_less,•pred,•limit)    CallExpression                                                                      */
			2,                                                                                                                            /*
            2    Literal                                                                                                                  */
			|| recurse(right, is_less, Some(pivot), limit),                                                                               /*
            ||•recurse(right,•is_less,•Some(pivot),•limit)    ClosureFunctionExpression
               recurse(right,•is_less,•Some(pivot),•limit)    CallExpression
                                       Some(pivot)            CallExpression                                                              */
		),                                                                                                                                /*
   ╚╚)    </CallExpression>                                                                                                               */
	];                                                                                                                                    /*
   ╚];    </ExpressionStatement>
   ╚]     </ArrayLiteral>                                                                                                                 */
	bifornCringer =                                                                                                                       /*
    bifornCringer•=↲    <ExpressionStatement>, <ReassignmentExpression>                                                                   */
  		askTrovenaBeenaDepends =                                                                                                        /*
          askTrovenaBeenaDepends•=↲    <ReassignmentExpression>                                                                           */
  		glimseGlyphs =                                                                                                                  /*
          glimseGlyphs•=↲    <ReassignmentExpression>                                                                                     */
  		  	|argumentOne, argumentTwo, argumentThree|                                                                                 /*
                |argumentOne,•argumentTwo,•argumentThree|↲    <ClosureFunctionExpression>
                 argumentOne                                  ClosureFunctionParameterDeclaration
                              argumentTwo                     ClosureFunctionParameterDeclaration
                                           argumentThree      ClosureFunctionParameterDeclaration                                         */
  		  	|restOfTheArguments12345678| {                                                                                            /*
                |restOfTheArguments12345678|•{↲    <ClosureFunctionExpression>
                 restOfTheArguments12345678        ClosureFunctionParameterDeclaration
                                             {↲    <BlockExpression>                                                                      */
  		  		return "baz";                                                                                                         /*
                    return•"baz";    ExpressionStatement
                    return•"baz"     ReturnExpression
                           "baz"     Literal                                                                                              */
  		  	};                                                                                                                        /*
••╚╚••╚};    </ExpressionStatement>
••╚╚••╚}     </ReassignmentExpression>, </ReassignmentExpression>                                                                         */
	aaaaaaaaaaaaaaaa.map(|x| {                                                                                                            /*
    aaaaaaaaaaaaaaaa.map(|x|•{↲    <ExpressionStatement>, <MemberExpression>
                         |x|•{↲    <ClosureFunctionExpression>
                          x        ClosureFunctionParameterDeclaration
                             {↲    <BlockExpression>                                                                                      */
        x += 1;                                                                                                                           /*
        x•+=•1;    ExpressionStatement
        x•+=•1     ReassignmentOperationExpression
             1     Literal                                                                                                                */
        x                                                                                                                                 /*
        x    ExpressionStatement                                                                                                          */
    })                                                                                                                                    /*
••••})    </CallExpression>
••••}     </ClosureFunctionExpression>, </BlockExpression>                                                                                */
        .filter;                                                                                                                          /*
••••••••.filter;    </ExpressionStatement>
••••••••.filter     </MemberExpression>                                                                                                   */
	let f = |x| {{{{x}}}};                                                                                                                /*
    let•f•=•|x|•{{{{x}}}};    LetVariableDeclaration
            |x|•{{{{x}}}}     ClosureFunctionExpression
             x                ClosureFunctionParameterDeclaration
                {{{{x}}}}     BlockExpression
                 {{{x}}}      ExpressionStatement, BlockExpression
                  {{x}}       ExpressionStatement, BlockExpression
                   {x}        ExpressionStatement, BlockExpression
                    x         ExpressionStatement                                                                                         */
    let f = |x| {{{x}}};                                                                                                                  /*
    let•f•=•|x|•{{{x}}};    LetVariableDeclaration
            |x|•{{{x}}}     ClosureFunctionExpression
             x              ClosureFunctionParameterDeclaration
                {{{x}}}     BlockExpression
                 {{x}}      ExpressionStatement, BlockExpression
                  {x}       ExpressionStatement, BlockExpression
                   x        ExpressionStatement                                                                                           */
    let f = |x| {{x}};                                                                                                                    /*
    let•f•=•|x|•{{x}};    LetVariableDeclaration
            |x|•{{x}}     ClosureFunctionExpression
             x            ClosureFunctionParameterDeclaration
                {{x}}     BlockExpression
                 {x}      ExpressionStatement, BlockExpression
                  x       ExpressionStatement                                                                                             */
    let f = |x| {x};                                                                                                                      /*
    let•f•=•|x|•{x};    LetVariableDeclaration
            |x|•{x}     ClosureFunctionExpression
             x          ClosureFunctionParameterDeclaration
                {x}     BlockExpression
                 x      ExpressionStatement                                                                                               */
    let f = |x| x;                                                                                                                        /*
    let•f•=•|x|•x;    LetVariableDeclaration
            |x|•x     ClosureFunctionExpression
             x        ClosureFunctionParameterDeclaration                                                                                 */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

fn f(_n: isize) -> isize { id(|| { 1 }) - 0 }                                                                                             /*
fn•f(_n:•isize)•->•isize•{•id(||•{•1•})•-•0•}    FunctionDeclaration
     _n:•isize                                   FunctionParameterDeclaration
                           id(||•{•1•})•-•0      ExpressionStatement, OperationExpression
                           id(||•{•1•})          CallExpression
                              ||•{•1•}           ClosureFunctionExpression
                                 {•1•}           BlockExpression
                                   1             ExpressionStatement, Literal
                                          0      Literal                                                                                  */
fn f(){ || x += y }                                                                                                                       /*
fn•f(){•||•x•+=•y•}    FunctionDeclaration
        ||•x•+=•y      ExpressionStatement, ClosureFunctionExpression
           x•+=•y      ReassignmentOperationExpression                                                                                    */
struct A {                                                                                                                                /*
struct•A•{↲    <StructDeclaration>                                                                                                        */
    b: [(); match || 1 { a => 0 }],                                                                                                       /*
    b:•[();•match•||•1•{•a•=>•0•}]     StructPropertyDeclaration
       [();•match•||•1•{•a•=>•0•}]     TypeSizedArray
        ()                             TypeTuple
            match•||•1•{•a•=>•0•}      MatchExpression
                  ||•1                 ClosureFunctionExpression
                     1                 Literal
                         a•=>•0        MatchExpressionCase
                              0        Literal                                                                                            */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */
enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>                                                                                                            */
    V([(); { let _ = || 1; 0 }]),                                                                                                         /*
    V([();•{•let•_•=•||•1;•0•}])     EnumMemberTupleDeclaration
      [();•{•let•_•=•||•1;•0•}]      TupleStructItemDeclaration, TypeSizedArray
       ()                            TypeTuple
           {•let•_•=•||•1;•0•}       BlockExpression
             let•_•=•||•1;           LetVariableDeclaration
                 _                   WildcardPattern
                     ||•1            ClosureFunctionExpression
                        1            Literal
                           0         ExpressionStatement, Literal                                                                         */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */
type Ty = [(); { let _ = || 1; 0 }];                                                                                                      /*
type•Ty•=•[();•{•let•_•=•||•1;•0•}];    TypeAliasDeclaration
          [();•{•let•_•=•||•1;•0•}]     TypeSizedArray
           ()                           TypeTuple
               {•let•_•=•||•1;•0•}      BlockExpression
                 let•_•=•||•1;          LetVariableDeclaration
                     _                  WildcardPattern
                         ||•1           ClosureFunctionExpression
                            1           Literal
                               0        ExpressionStatement, Literal                                                                      */

// Discarded Nodes: 13
// Parsed Nodes: 963
// state_rollbacks: 14
// Total '.charCodeAt()' calls: 4074 (32% re-reads)
// Unnecessary 'skip_whitespace()' calls: 637
// source: "../../samples/expressions/closure.rs"