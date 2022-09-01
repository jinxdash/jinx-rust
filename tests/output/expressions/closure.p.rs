fn main() {                                                                                                                               /*
fn•main()•{↲    <Program>
fn•main()•{↲    <Program.ast{dk: "None"}>
fn•main()•{↲    <FunctionDeclaration>
       ()       FunctionDeclaration.parameters{dk: "()"}
          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                      */
	let lam = |(a, ref b, c, ref mut d): (X, X, X, X)| {};                                                                                /*
	let•lam•=•|(a,•ref•b,•c,•ref•mut•d):•(X,•X,•X,•X)|•{};    LetVariableDeclaration
	          |(a,•ref•b,•c,•ref•mut•d):•(X,•X,•X,•X)|•{}     ClosureFunctionExpression
	          |(a,•ref•b,•c,•ref•mut•d):•(X,•X,•X,•X)|        ClosureFunctionExpression.parameters{dk: "||"}
	           (a,•ref•b,•c,•ref•mut•d):•(X,•X,•X,•X)         ClosureFunctionParameterDeclaration
	           (a,•ref•b,•c,•ref•mut•d)                       TuplePattern
	               ref•b                                      PatternVariableDeclaration{ref, !mut}
	                         ref•mut•d                        PatternVariableDeclaration{ref, mut}
	                                     (X,•X,•X,•X)         TypeTuple
	                                                   {}     BlockExpression                                                             */
	let x = |_: ()| ();                                                                                                                   /*
	let•x•=•|_:•()|•();    LetVariableDeclaration
	        |_:•()|•()     ClosureFunctionExpression
	        |_:•()|        ClosureFunctionExpression.parameters{dk: "||"}
	         _:•()         ClosureFunctionParameterDeclaration
	         _             WildcardPattern
	            ()         TypeTuple
	                ()     TupleLiteral                                                                                                   */
	let y = || x(());                                                                                                                     /*
	let•y•=•||•x(());    LetVariableDeclaration
	        ||•x(())     ClosureFunctionExpression
	        ||           ClosureFunctionExpression.parameters{dk: "||"}
	           x(())     CallExpression
	            (())     CallExpression.arguments{dk: "()"}
	             ()      TupleLiteral                                                                                                     */
	let mut x = |_: ()| {                                                                                                                 /*
	let•mut•x•=•|_:•()|•{↲    <LetVariableDeclaration>
	    mut•x                 PatternVariableDeclaration{!ref, mut}
	            |_:•()|•{↲    <ClosureFunctionExpression>
	            |_:•()|       ClosureFunctionExpression.parameters{dk: "||"}
	             _:•()        ClosureFunctionParameterDeclaration
	             _            WildcardPattern
	                ()        TypeTuple
	                    {↲    <BlockExpression>                                                                                           */
        outer = 4;                                                                                                                        /*
        outer•=•4;    ExpressionStatement{semi}
        outer•=•4     ReassignmentExpression{tk: "="}
                4     Literal{kind: Integer}                                                                                              */
        ()                                                                                                                                /*
        ()    ExpressionStatement{!semi}, TupleLiteral                                                                                    */
    };                                                                                                                                    /*
••••}     </BlockExpression>
••••}     </ClosureFunctionExpression>
••••};    </LetVariableDeclaration>                                                                                                       */
	let x = move |_: ()| {                                                                                                                /*
	let•x•=•move•|_:•()|•{↲    <LetVariableDeclaration>
	        move•|_:•()|•{↲    <ClosureFunctionExpression{move}>
	             |_:•()|       ClosureFunctionExpression.parameters{dk: "||"}
	              _:•()        ClosureFunctionParameterDeclaration
	              _            WildcardPattern
	                 ()        TypeTuple
	                     {↲    <BlockExpression>                                                                                          */
        let inner = outer;                                                                                                                /*
        let•inner•=•outer;    LetVariableDeclaration                                                                                      */
        ()                                                                                                                                /*
        ()    ExpressionStatement{!semi}, TupleLiteral                                                                                    */
    };                                                                                                                                    /*
••••}     </BlockExpression>
••••}     </ClosureFunctionExpression>
••••};    </LetVariableDeclaration>                                                                                                       */
	const VTABLE: &'static VTable<DST> = &VTable { _to_dst_ptr: |_: *mut ()| unsafe { std::mem::zeroed() }, };                            /*
	const•VTABLE:•&'static•VTable<DST>•=•&VTable•{•_to_dst_ptr:•|_:•*mut•()|•unsafe•{•std::mem::zeroed()•},•};    ConstVariableDeclaration
	              &'static•VTable<DST>                                                                            TypeReference{!mut}
	               'static                                                                                        LtStatic
	                       VTable<DST>                                                                            TypeCall
	                             <DST>                                                                            TypeCall.typeArguments{dk: "<>"}
	                                     &VTable•{•_to_dst_ptr:•|_:•*mut•()|•unsafe•{•std::mem::zeroed()•},•}     ReferenceExpression{!mut}
	                                      VTable•{•_to_dst_ptr:•|_:•*mut•()|•unsafe•{•std::mem::zeroed()•},•}     StructLiteral
	                                             {•_to_dst_ptr:•|_:•*mut•()|•unsafe•{•std::mem::zeroed()•},•}     StructLiteral.properties{dk: "{}"}
	                                               _to_dst_ptr:•|_:•*mut•()|•unsafe•{•std::mem::zeroed()•}        StructLiteralProperty
	                                                            |_:•*mut•()|•unsafe•{•std::mem::zeroed()•}        ClosureFunctionExpression
	                                                            |_:•*mut•()|                                      ClosureFunctionExpression.parameters{dk: "||"}
	                                                             _:•*mut•()                                       ClosureFunctionParameterDeclaration
	                                                             _                                                WildcardPattern
	                                                                *mut•()                                       TypeDereferenceMut
	                                                                     ()                                       TypeTuple
	                                                                         unsafe•{•std::mem::zeroed()•}        BlockExpression{unsafe}
	                                                                                {•std::mem::zeroed()•}        BlockExpression.body{dk: "{}"}
	                                                                                  std::mem::zeroed()          ExpressionStatement{!semi}, CallExpression
	                                                                                  std::mem::zeroed            ExpressionPath
	                                                                                  std::mem                    ExpressionPath
	                                                                                                  ()          CallExpression.arguments{dk: "()"}*/
    let z = a(&mut |x| x - 22);                                                                                                           /*
    let•z•=•a(&mut•|x|•x•-•22);    LetVariableDeclaration
            a(&mut•|x|•x•-•22)     CallExpression
             (&mut•|x|•x•-•22)     CallExpression.arguments{dk: "()"}
              &mut•|x|•x•-•22      ReferenceExpression{mut}
                   |x|•x•-•22      ClosureFunctionExpression
                   |x|             ClosureFunctionExpression.parameters{dk: "||"}
                    x              ClosureFunctionParameterDeclaration
                       x•-•22      OperationExpression{tk: "-"}
                           22      Literal{kind: Integer}                                                                                 */
    let mut unboxed = || {};                                                                                                              /*
    let•mut•unboxed•=•||•{};    LetVariableDeclaration
        mut•unboxed             PatternVariableDeclaration{!ref, mut}
                      ||•{}     ClosureFunctionExpression
                      ||        ClosureFunctionExpression.parameters{dk: "||"}
                         {}     BlockExpression                                                                                           */
    Box::new(move |y| { x + y });                                                                                                         /*
    Box::new(move•|y|•{•x•+•y•});    ExpressionStatement{semi}
    Box::new(move•|y|•{•x•+•y•})     CallExpression
    Box::new                         ExpressionPath
            (move•|y|•{•x•+•y•})     CallExpression.arguments{dk: "()"}
             move•|y|•{•x•+•y•}      ClosureFunctionExpression{move}
                  |y|                ClosureFunctionExpression.parameters{dk: "||"}
                   y                 ClosureFunctionParameterDeclaration
                      {•x•+•y•}      BlockExpression
                        x•+•y        ExpressionStatement{!semi}, OperationExpression{tk: "+"}                                             */
    s(|f| (*f)(), Box::new(|| {}));                                                                                                       /*
    s(|f|•(*f)(),•Box::new(||•{}));    ExpressionStatement{semi}
    s(|f|•(*f)(),•Box::new(||•{}))     CallExpression
     (|f|•(*f)(),•Box::new(||•{}))     CallExpression.arguments{dk: "()"}
      |f|•(*f)()                       ClosureFunctionExpression
      |f|                              ClosureFunctionExpression.parameters{dk: "||"}
       f                               ClosureFunctionParameterDeclaration
          (*f)()                       CallExpression
           *f                          DereferenceExpression
              ()                       CallExpression.arguments{dk: "()"}
                  Box::new(||•{})      CallExpression
                  Box::new             ExpressionPath
                          (||•{})      CallExpression.arguments{dk: "()"}
                           ||•{}       ClosureFunctionExpression
                           ||          ClosureFunctionExpression.parameters{dk: "||"}
                              {}       BlockExpression                                                                                    */
	(0..42).e(|_x| match E(()) as R<(), _> { O(()) => s.push(()), E(_) => (), });                                                         /*
	(0..42).e(|_x|•match•E(())•as•R<(),•_>•{•O(())•=>•s.push(()),•E(_)•=>•(),•});    ExpressionStatement{semi}
	(0..42).e(|_x|•match•E(())•as•R<(),•_>•{•O(())•=>•s.push(()),•E(_)•=>•(),•})     CallExpression
	 0..42                                                                           RangeLiteral{!last}
	 0                                                                               Literal{kind: Integer}
	    42                                                                           Literal{kind: Integer}
	         (|_x|•match•E(())•as•R<(),•_>•{•O(())•=>•s.push(()),•E(_)•=>•(),•})     CallExpression.arguments{dk: "()"}
	          |_x|•match•E(())•as•R<(),•_>•{•O(())•=>•s.push(()),•E(_)•=>•(),•}      ClosureFunctionExpression
	          |_x|                                                                   ClosureFunctionExpression.parameters{dk: "||"}
	           _x                                                                    ClosureFunctionParameterDeclaration
	               match•E(())•as•R<(),•_>•{•O(())•=>•s.push(()),•E(_)•=>•(),•}      MatchExpression
	                     E(())•as•R<(),•_>                                           ExpressionAsTypeCast
	                     E(())                                                       CallExpression
	                      (())                                                       CallExpression.arguments{dk: "()"}
	                       ()                                                        TupleLiteral
	                              R<(),•_>                                           TypeCall
	                               <(),•_>                                           TypeCall.typeArguments{dk: "<>"}
	                                ()                                               TypeTuple
	                                    _                                            TypeInferred
	                                       {•O(())•=>•s.push(()),•E(_)•=>•(),•}      MatchExpression.cases{dk: "{}"}
	                                         O(())•=>•s.push(())                     MatchExpressionCase
	                                         O(())                                   TuplePattern
	                                          (())                                   TuplePattern.items{dk: "()"}
	                                           ()                                    TuplePattern
	                                                  s.push(())                     CallExpression
	                                                        (())                     CallExpression.arguments{dk: "()"}
	                                                         ()                      TupleLiteral
	                                                              E(_)•=>•()         MatchExpressionCase
	                                                              E(_)               TuplePattern
	                                                               (_)               TuplePattern.items{dk: "()"}
	                                                                _                WildcardPattern
	                                                                      ()         TupleLiteral                                         */
	<()>::a(|| ());                                                                                                                       /*
	<()>::a(||•());    ExpressionStatement{semi}
	<()>::a(||•())     CallExpression
	<()>::a            ExpressionPath
	<()>               ExpressionTypeSelector
	 ()                TypeTuple
	       (||•())     CallExpression.arguments{dk: "()"}
	        ||•()      ClosureFunctionExpression
	        ||         ClosureFunctionExpression.parameters{dk: "||"}
	           ()      TupleLiteral                                                                                                       */
    let f: &mut dyn FnMut<(_,), E = ()> = &mut |_: <() as Lt<'_>>::T| {};                                                                 /*
    let•f:•&mut•dyn•FnMut<(_,),•E•=•()>•=•&mut•|_:•<()•as•Lt<'_>>::T|•{};    LetVariableDeclaration
           &mut•dyn•FnMut<(_,),•E•=•()>                                      TypeReference{mut}
                dyn•FnMut<(_,),•E•=•()>                                      TypeDynBounds{dyn}
                    FnMut<(_,),•E•=•()>                                      TypeTraitBound{!maybeConst, !optional}, TypeCall
                         <(_,),•E•=•()>                                      TypeCall.typeArguments{dk: "<>"}
                          (_,)                                               TypeTuple
                           _                                                 TypeInferred
                                E•=•()                                       TypeCallNamedArgument
                                    ()                                       TypeTuple
                                          &mut•|_:•<()•as•Lt<'_>>::T|•{}     ReferenceExpression{mut}
                                               |_:•<()•as•Lt<'_>>::T|•{}     ClosureFunctionExpression
                                               |_:•<()•as•Lt<'_>>::T|        ClosureFunctionExpression.parameters{dk: "||"}
                                                _:•<()•as•Lt<'_>>::T         ClosureFunctionParameterDeclaration
                                                _                            WildcardPattern
                                                   <()•as•Lt<'_>>::T         TypePath
                                                   <()•as•Lt<'_>>            ExpressionTypeSelector
                                                    ()                       TypeTuple
                                                          Lt<'_>             TypeCall
                                                            <'_>             TypeCall.typeArguments{dk: "<>"}
                                                             '_              LtElided
                                                                      {}     BlockExpression                                              */
	Box::new(move |v| { (|_| e.d()())(v); X }) as Box<dyn Fn(i32) -> Q<i32>>;                                                             /*
	Box::new(move•|v|•{•(|_|•e.d()())(v);•X•})•as•Box<dyn•Fn(i32)•->•Q<i32>>;    ExpressionStatement{semi}
	Box::new(move•|v|•{•(|_|•e.d()())(v);•X•})•as•Box<dyn•Fn(i32)•->•Q<i32>>     ExpressionAsTypeCast
	Box::new(move•|v|•{•(|_|•e.d()())(v);•X•})                                   CallExpression
	Box::new                                                                     ExpressionPath
	        (move•|v|•{•(|_|•e.d()())(v);•X•})                                   CallExpression.arguments{dk: "()"}
	         move•|v|•{•(|_|•e.d()())(v);•X•}                                    ClosureFunctionExpression{move}
	              |v|                                                            ClosureFunctionExpression.parameters{dk: "||"}
	               v                                                             ClosureFunctionParameterDeclaration
	                  {•(|_|•e.d()())(v);•X•}                                    BlockExpression
	                    (|_|•e.d()())(v);                                        ExpressionStatement{semi}
	                    (|_|•e.d()())(v)                                         CallExpression
	                     |_|•e.d()()                                             ClosureFunctionExpression
	                     |_|                                                     ClosureFunctionExpression.parameters{dk: "||"}
	                      _                                                      ClosureFunctionParameterDeclaration, WildcardPattern
	                         e.d()()                                             CallExpression
	                         e.d()                                               CallExpression
	                            ()                                               CallExpression.arguments{dk: "()"}
	                              ()                                             CallExpression.arguments{dk: "()"}
	                                 (v)                                         CallExpression.arguments{dk: "()"}
	                                      X                                      ExpressionStatement{!semi}
	                                              Box<dyn•Fn(i32)•->•Q<i32>>     TypeCall
	                                                 <dyn•Fn(i32)•->•Q<i32>>     TypeCall.typeArguments{dk: "<>"}
	                                                  dyn•Fn(i32)•->•Q<i32>      TypeDynBounds{dyn}
	                                                      Fn(i32)•->•Q<i32>      TypeTraitBound{!maybeConst, !optional}, TypeFunction
	                                                        (i32)                TypeFunction.parameters{dk: "()"}
	                                                                 Q<i32>      TypeCall
	                                                                  <i32>      TypeCall.typeArguments{dk: "<>"}                         */
	let _c = || match b.0.c(1) as D<(), _> { _ => 0, };                                                                                   /*
	let•_c•=•||•match•b.0.c(1)•as•D<(),•_>•{•_•=>•0,•};    LetVariableDeclaration
	         ||•match•b.0.c(1)•as•D<(),•_>•{•_•=>•0,•}     ClosureFunctionExpression
	         ||                                            ClosureFunctionExpression.parameters{dk: "||"}
	            match•b.0.c(1)•as•D<(),•_>•{•_•=>•0,•}     MatchExpression
	                  b.0.c(1)•as•D<(),•_>                 ExpressionAsTypeCast
	                  b.0.c(1)                             CallExpression
	                  b.0                                  MemberExpression{!computed}
	                    0                                  Index
	                       (1)                             CallExpression.arguments{dk: "()"}
	                        1                              Literal{kind: Integer}
	                              D<(),•_>                 TypeCall
	                               <(),•_>                 TypeCall.typeArguments{dk: "<>"}
	                                ()                     TypeTuple
	                                    _                  TypeInferred
	                                       {•_•=>•0,•}     MatchExpression.cases{dk: "{}"}
	                                         _•=>•0        MatchExpressionCase
	                                         _             WildcardPattern
	                                              0        Literal{kind: Integer}                                                         */
    let f: &dyn Fn(i32) -> _ = &|x| x + x;                                                                                                /*
    let•f:•&dyn•Fn(i32)•->•_•=•&|x|•x•+•x;    LetVariableDeclaration
           &dyn•Fn(i32)•->•_                  TypeReference{!mut}
            dyn•Fn(i32)•->•_                  TypeDynBounds{dyn}
                Fn(i32)•->•_                  TypeTraitBound{!maybeConst, !optional}, TypeFunction
                  (i32)                       TypeFunction.parameters{dk: "()"}
                           _                  TypeInferred
                               &|x|•x•+•x     ReferenceExpression{!mut}
                                |x|•x•+•x     ClosureFunctionExpression
                                |x|           ClosureFunctionExpression.parameters{dk: "||"}
                                 x            ClosureFunctionParameterDeclaration
                                    x•+•x     OperationExpression{tk: "+"}                                                                */
	let f = |x: u32| -> u32 { 1 };                                                                                                        /*
	let•f•=•|x:•u32|•->•u32•{•1•};    LetVariableDeclaration
	        |x:•u32|•->•u32•{•1•}     ClosureFunctionExpression
	        |x:•u32|                  ClosureFunctionExpression.parameters{dk: "||"}
	         x:•u32                   ClosureFunctionParameterDeclaration
	                        {•1•}     BlockExpression
	                          1       ExpressionStatement{!semi}, Literal{kind: Integer}                                                  */
	for f in &[d, g, |x| x] { a!("{}", f(6)); }                                                                                           /*
	for•f•in•&[d,•g,•|x|•x]•{•a!("{}",•f(6));•}    ExpressionStatement{!semi}, ForInBlockExpression
	         &[d,•g,•|x|•x]                        ReferenceExpression{!mut}
	          [d,•g,•|x|•x]                        ArrayLiteral
	                 |x|•x                         ClosureFunctionExpression
	                 |x|                           ClosureFunctionExpression.parameters{dk: "||"}
	                  x                            ClosureFunctionParameterDeclaration
	                        {•a!("{}",•f(6));•}    ForInBlockExpression.body{dk: "{}"}
	                          a!("{}",•f(6));      ExpressionStatement{semi}
	                          a!("{}",•f(6))       MacroInvocation
	                            ("{}",•f(6))       MacroInvocation.segments{dk: "()"}
	                             "{}"              Literal{kind: String}
	                                 ,             PunctuationToken{tk: ","}
	                                    (6)        DelimGroup
	                                     6         Literal{kind: Integer}                                                                 */
	(|| { (|| { c.d })(); (move || { a.b })(); })();                                                                                      /*
	(||•{•(||•{•c.d•})();•(move•||•{•a.b•})();•})();    ExpressionStatement{semi}
	(||•{•(||•{•c.d•})();•(move•||•{•a.b•})();•})()     CallExpression
	 ||•{•(||•{•c.d•})();•(move•||•{•a.b•})();•}        ClosureFunctionExpression
	 ||                                                 ClosureFunctionExpression.parameters{dk: "||"}
	    {•(||•{•c.d•})();•(move•||•{•a.b•})();•}        BlockExpression
	      (||•{•c.d•})();                               ExpressionStatement{semi}
	      (||•{•c.d•})()                                CallExpression
	       ||•{•c.d•}                                   ClosureFunctionExpression
	       ||                                           ClosureFunctionExpression.parameters{dk: "||"}
	          {•c.d•}                                   BlockExpression
	            c.d                                     ExpressionStatement{!semi}, MemberExpression{!computed}
	                  ()                                CallExpression.arguments{dk: "()"}
	                      (move•||•{•a.b•})();          ExpressionStatement{semi}
	                      (move•||•{•a.b•})()           CallExpression
	                       move•||•{•a.b•}              ClosureFunctionExpression{move}
	                            ||                      ClosureFunctionExpression.parameters{dk: "||"}
	                               {•a.b•}              BlockExpression
	                                 a.b                ExpressionStatement{!semi}, MemberExpression{!computed}
	                                       ()           CallExpression.arguments{dk: "()"}
	                                             ()     CallExpression.arguments{dk: "()"}                                                */
	let q = a.e(async move { b(move || async move { d!() }) });                                                                           /*
	let•q•=•a.e(async•move•{•b(move•||•async•move•{•d!()•})•});    LetVariableDeclaration
	        a.e(async•move•{•b(move•||•async•move•{•d!()•})•})     CallExpression
	           (async•move•{•b(move•||•async•move•{•d!()•})•})     CallExpression.arguments{dk: "()"}
	            async•move•{•b(move•||•async•move•{•d!()•})•}      BlockExpression{move, async}
	                       {•b(move•||•async•move•{•d!()•})•}      BlockExpression.body{dk: "{}"}
	                         b(move•||•async•move•{•d!()•})        ExpressionStatement{!semi}, CallExpression
	                          (move•||•async•move•{•d!()•})        CallExpression.arguments{dk: "()"}
	                           move•||•async•move•{•d!()•}         ClosureFunctionExpression{move}
	                                ||                             ClosureFunctionExpression.parameters{dk: "||"}
	                                   async•move•{•d!()•}         BlockExpression{move, async}
	                                              {•d!()•}         BlockExpression.body{dk: "{}"}
	                                                d!()           ExpressionStatement{!semi}, MacroInvocation
	                                                  ()           MacroInvocation.segments{dk: "()"}                                     */
    let g = {||z(i)};                                                                                                                     /*
    let•g•=•{||z(i)};    LetVariableDeclaration
            {||z(i)}     BlockExpression
             ||z(i)      ExpressionStatement{!semi}, ClosureFunctionExpression
             ||          ClosureFunctionExpression.parameters{dk: "||"}
               z(i)      CallExpression
                (i)      CallExpression.arguments{dk: "()"}                                                                               */
    let _ = || a.e(async { r });                                                                                                          /*
    let•_•=•||•a.e(async•{•r•});    LetVariableDeclaration
        _                           WildcardPattern
            ||•a.e(async•{•r•})     ClosureFunctionExpression
            ||                      ClosureFunctionExpression.parameters{dk: "||"}
               a.e(async•{•r•})     CallExpression
                  (async•{•r•})     CallExpression.arguments{dk: "()"}
                   async•{•r•}      BlockExpression{async}
                         {•r•}      BlockExpression.body{dk: "{}"}
                           r        ExpressionStatement{!semi}                                                                            */
    let _ = E(0).d().q(|ref _a| true);                                                                                                    /*
    let•_•=•E(0).d().q(|ref•_a|•true);    LetVariableDeclaration
        _                                 WildcardPattern
            E(0).d().q(|ref•_a|•true)     CallExpression
            E(0).d()                      CallExpression
            E(0)                          CallExpression
             (0)                          CallExpression.arguments{dk: "()"}
              0                           Literal{kind: Integer}
                  ()                      CallExpression.arguments{dk: "()"}
                      (|ref•_a|•true)     CallExpression.arguments{dk: "()"}
                       |ref•_a|•true      ClosureFunctionExpression
                       |ref•_a|           ClosureFunctionExpression.parameters{dk: "||"}
                        ref•_a            ClosureFunctionParameterDeclaration, PatternVariableDeclaration{ref, !mut}
                                true      Literal{kind: True}                                                                             */
	let _ = !(4..5).a(|x| x == 1 || x == 3 || x == 5);                                                                                    /*
	let•_•=•!(4..5).a(|x|•x•==•1•||•x•==•3•||•x•==•5);    LetVariableDeclaration
	    _                                                 WildcardPattern
	        !(4..5).a(|x|•x•==•1•||•x•==•3•||•x•==•5)     NotExpression
	         (4..5).a(|x|•x•==•1•||•x•==•3•||•x•==•5)     CallExpression
	          4..5                                        RangeLiteral{!last}
	          4                                           Literal{kind: Integer}
	             5                                        Literal{kind: Integer}
	                 (|x|•x•==•1•||•x•==•3•||•x•==•5)     CallExpression.arguments{dk: "()"}
	                  |x|•x•==•1•||•x•==•3•||•x•==•5      ClosureFunctionExpression
	                  |x|                                 ClosureFunctionExpression.parameters{dk: "||"}
	                   x                                  ClosureFunctionParameterDeclaration
	                      x•==•1•||•x•==•3•||•x•==•5      OrExpression{tk: "||"}
	                      x•==•1•||•x•==•3                OrExpression{tk: "||"}
	                      x•==•1                          ComparisonExpression{tk: "=="}
	                           1                          Literal{kind: Integer}
	                                x•==•3                ComparisonExpression{tk: "=="}
	                                     3                Literal{kind: Integer}
	                                          x•==•5      ComparisonExpression{tk: "=="}
	                                               5      Literal{kind: Integer}                                                          */
	let _ = !(1..3).a(|x| [1, 2, 3].b(&x));                                                                                               /*
	let•_•=•!(1..3).a(|x|•[1,•2,•3].b(&x));    LetVariableDeclaration
	    _                                      WildcardPattern
	        !(1..3).a(|x|•[1,•2,•3].b(&x))     NotExpression
	         (1..3).a(|x|•[1,•2,•3].b(&x))     CallExpression
	          1..3                             RangeLiteral{!last}
	          1                                Literal{kind: Integer}
	             3                             Literal{kind: Integer}
	                 (|x|•[1,•2,•3].b(&x))     CallExpression.arguments{dk: "()"}
	                  |x|•[1,•2,•3].b(&x)      ClosureFunctionExpression
	                  |x|                      ClosureFunctionExpression.parameters{dk: "||"}
	                   x                       ClosureFunctionParameterDeclaration
	                      [1,•2,•3].b(&x)      CallExpression
	                      [1,•2,•3]            ArrayLiteral
	                       1                   Literal{kind: Integer}
	                          2                Literal{kind: Integer}
	                             3             Literal{kind: Integer}
	                                 (&x)      CallExpression.arguments{dk: "()"}
	                                  &x       ReferenceExpression{!mut}                                                                  */
	let _ = !(1..3).a(|x| x == 0 || [1, 2, 3].b(&x));                                                                                     /*
	let•_•=•!(1..3).a(|x|•x•==•0•||•[1,•2,•3].b(&x));    LetVariableDeclaration
	    _                                                WildcardPattern
	        !(1..3).a(|x|•x•==•0•||•[1,•2,•3].b(&x))     NotExpression
	         (1..3).a(|x|•x•==•0•||•[1,•2,•3].b(&x))     CallExpression
	          1..3                                       RangeLiteral{!last}
	          1                                          Literal{kind: Integer}
	             3                                       Literal{kind: Integer}
	                 (|x|•x•==•0•||•[1,•2,•3].b(&x))     CallExpression.arguments{dk: "()"}
	                  |x|•x•==•0•||•[1,•2,•3].b(&x)      ClosureFunctionExpression
	                  |x|                                ClosureFunctionExpression.parameters{dk: "||"}
	                   x                                 ClosureFunctionParameterDeclaration
	                      x•==•0•||•[1,•2,•3].b(&x)      OrExpression{tk: "||"}
	                      x•==•0                         ComparisonExpression{tk: "=="}
	                           0                         Literal{kind: Integer}
	                                [1,•2,•3].b(&x)      CallExpression
	                                [1,•2,•3]            ArrayLiteral
	                                 1                   Literal{kind: Integer}
	                                    2                Literal{kind: Integer}
	                                       3             Literal{kind: Integer}
	                                           (&x)      CallExpression.arguments{dk: "()"}
	                                            &x       ReferenceExpression{!mut}                                                        */
	let _ = !(1..3).a(|x| [1, 2, 3].b(&x) || x == 0);                                                                                     /*
	let•_•=•!(1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0);    LetVariableDeclaration
	    _                                                WildcardPattern
	        !(1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0)     NotExpression
	         (1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0)     CallExpression
	          1..3                                       RangeLiteral{!last}
	          1                                          Literal{kind: Integer}
	             3                                       Literal{kind: Integer}
	                 (|x|•[1,•2,•3].b(&x)•||•x•==•0)     CallExpression.arguments{dk: "()"}
	                  |x|•[1,•2,•3].b(&x)•||•x•==•0      ClosureFunctionExpression
	                  |x|                                ClosureFunctionExpression.parameters{dk: "||"}
	                   x                                 ClosureFunctionParameterDeclaration
	                      [1,•2,•3].b(&x)•||•x•==•0      OrExpression{tk: "||"}
	                      [1,•2,•3].b(&x)                CallExpression
	                      [1,•2,•3]                      ArrayLiteral
	                       1                             Literal{kind: Integer}
	                          2                          Literal{kind: Integer}
	                             3                       Literal{kind: Integer}
	                                 (&x)                CallExpression.arguments{dk: "()"}
	                                  &x                 ReferenceExpression{!mut}
	                                         x•==•0      ComparisonExpression{tk: "=="}
	                                              0      Literal{kind: Integer}                                                           */
    let _ = !(1..3).a(|x| [1, 2, 3].b(&x) || x == 0 || [4, 5, 6].c(&x) || x == -1);                                                       /*
    let•_•=•!(1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1);    LetVariableDeclaration
        _                                                                              WildcardPattern
            !(1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1)     NotExpression
             (1..3).a(|x|•[1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1)     CallExpression
              1..3                                                                     RangeLiteral{!last}
              1                                                                        Literal{kind: Integer}
                 3                                                                     Literal{kind: Integer}
                     (|x|•[1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1)     CallExpression.arguments{dk: "()"}
                      |x|•[1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1      ClosureFunctionExpression
                      |x|                                                              ClosureFunctionExpression.parameters{dk: "||"}
                       x                                                               ClosureFunctionParameterDeclaration
                          [1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)•||•x•==•-1      OrExpression{tk: "||"}
                          [1,•2,•3].b(&x)•||•x•==•0•||•[4,•5,•6].c(&x)                 OrExpression{tk: "||"}
                          [1,•2,•3].b(&x)•||•x•==•0                                    OrExpression{tk: "||"}
                          [1,•2,•3].b(&x)                                              CallExpression
                          [1,•2,•3]                                                    ArrayLiteral
                           1                                                           Literal{kind: Integer}
                              2                                                        Literal{kind: Integer}
                                 3                                                     Literal{kind: Integer}
                                     (&x)                                              CallExpression.arguments{dk: "()"}
                                      &x                                               ReferenceExpression{!mut}
                                             x•==•0                                    ComparisonExpression{tk: "=="}
                                                  0                                    Literal{kind: Integer}
                                                       [4,•5,•6].c(&x)                 CallExpression
                                                       [4,•5,•6]                       ArrayLiteral
                                                        4                              Literal{kind: Integer}
                                                           5                           Literal{kind: Integer}
                                                              6                        Literal{kind: Integer}
                                                                  (&x)                 CallExpression.arguments{dk: "()"}
                                                                   &x                  ReferenceExpression{!mut}
                                                                          x•==•-1      ComparisonExpression{tk: "=="}
                                                                               -1      MinusExpression
                                                                                1      Literal{kind: Integer}                             */
	let hash: &Fn(&&Block) -> u64 = &|block| -> u64 { 1 };                                                                                /*
	let•hash:•&Fn(&&Block)•->•u64•=•&|block|•->•u64•{•1•};    LetVariableDeclaration
	          &Fn(&&Block)•->•u64                             TypeReference{!mut}
	           Fn(&&Block)•->•u64                             TypeFunction
	             (&&Block)                                    TypeFunction.parameters{dk: "()"}
	              &&Block                                     TypeReference{!mut}
	               &Block                                     TypeReference{!mut}
	                                &|block|•->•u64•{•1•}     ReferenceExpression{!mut}
	                                 |block|•->•u64•{•1•}     ClosureFunctionExpression
	                                 |block|                  ClosureFunctionExpression.parameters{dk: "||"}
	                                  block                   ClosureFunctionParameterDeclaration
	                                                {•1•}     BlockExpression
	                                                  1       ExpressionStatement{!semi}, Literal{kind: Integer}                          */
	if outer_guard.map_or(true, |(Guard::If(e) | Guard::IfLet(_, e))| !is_local_used(cx, *e, binding_id)) {}                              /*
	if•outer_guard.map_or(true,•|(Guard::If(e)•|•Guard::IfLet(_,•e))|•!is_local_used(cx,•*e,•binding_id))•{}    ExpressionStatement{!semi}, IfBlockExpression
	   outer_guard.map_or(true,•|(Guard::If(e)•|•Guard::IfLet(_,•e))|•!is_local_used(cx,•*e,•binding_id))       CallExpression
	                     (true,•|(Guard::If(e)•|•Guard::IfLet(_,•e))|•!is_local_used(cx,•*e,•binding_id))       CallExpression.arguments{dk: "()"}
	                      true                                                                                  Literal{kind: True}
	                            |(Guard::If(e)•|•Guard::IfLet(_,•e))|•!is_local_used(cx,•*e,•binding_id)        ClosureFunctionExpression
	                            |(Guard::If(e)•|•Guard::IfLet(_,•e))|                                           ClosureFunctionExpression.parameters{dk: "||"}
	                             (Guard::If(e)•|•Guard::IfLet(_,•e))                                            ClosureFunctionParameterDeclaration
	                              Guard::If(e)•|•Guard::IfLet(_,•e)                                             UnionPattern
	                              Guard::If(e)                                                                  TuplePattern
	                              Guard::If                                                                     ExpressionPath
	                                       (e)                                                                  TuplePattern.items{dk: "()"}
	                                             Guard::IfLet(_,•e)                                             TuplePattern
	                                             Guard::IfLet                                                   ExpressionPath
	                                                         (_,•e)                                             TuplePattern.items{dk: "()"}
	                                                          _                                                 WildcardPattern
	                                                                  !is_local_used(cx,•*e,•binding_id)        NotExpression
	                                                                   is_local_used(cx,•*e,•binding_id)        CallExpression
	                                                                                (cx,•*e,•binding_id)        CallExpression.arguments{dk: "()"}
	                                                                                     *e                     DereferenceExpression
	                                                                                                      {}    IfBlockExpression.body{dk: "{}"}*/
	a =| | b;                                                                                                                             /*
	a•=|•|•b;    ExpressionStatement{semi}
	a•=|•|•b     ReassignmentExpression{tk: "="}
	   |•|•b     ClosureFunctionExpression
	   |•|       ClosureFunctionExpression.parameters{dk: "||"}                                                                           */
	[                                                                                                                                     /*
	[↲    <ExpressionStatement{semi}>
	[↲    <ArrayLiteral>                                                                                                                  */
		foo(|| ()),                                                                                                                       /*
		foo(||•())    CallExpression
		   (||•())    CallExpression.arguments{dk: "()"}
		    ||•()     ClosureFunctionExpression
		    ||        ClosureFunctionExpression.parameters{dk: "||"}
		       ()     TupleLiteral                                                                                                        */
		|x: u32| x + 1,                                                                                                                   /*
		|x:•u32|•x•+•1    ClosureFunctionExpression
		|x:•u32|          ClosureFunctionExpression.parameters{dk: "||"}
		 x:•u32           ClosureFunctionParameterDeclaration
		         x•+•1    OperationExpression{tk: "+"}
		             1    Literal{kind: Integer}                                                                                          */
		(|| Box::new(|| {}) as Box<dyn Fn()>)(),                                                                                          /*
		(||•Box::new(||•{})•as•Box<dyn•Fn()>)()    CallExpression
		 ||•Box::new(||•{})•as•Box<dyn•Fn()>       ClosureFunctionExpression
		 ||                                        ClosureFunctionExpression.parameters{dk: "||"}
		    Box::new(||•{})•as•Box<dyn•Fn()>       ExpressionAsTypeCast
		    Box::new(||•{})                        CallExpression
		    Box::new                               ExpressionPath
		            (||•{})                        CallExpression.arguments{dk: "()"}
		             ||•{}                         ClosureFunctionExpression
		             ||                            ClosureFunctionExpression.parameters{dk: "||"}
		                {}                         BlockExpression
		                       Box<dyn•Fn()>       TypeCall
		                          <dyn•Fn()>       TypeCall.typeArguments{dk: "<>"}
		                           dyn•Fn()        TypeDynBounds{dyn}
		                               Fn()        TypeTraitBound{!maybeConst, !optional}, TypeFunction
		                                 ()        TypeFunction.parameters{dk: "()"}
		                                     ()    CallExpression.arguments{dk: "()"}                                                     */
		|_: T| 3,                                                                                                                         /*
		|_:•T|•3    ClosureFunctionExpression
		|_:•T|      ClosureFunctionExpression.parameters{dk: "||"}
		 _:•T       ClosureFunctionParameterDeclaration
		 _          WildcardPattern
		       3    Literal{kind: Integer}                                                                                                */
		move |x: isize, y| x + y + z,                                                                                                     /*
		move•|x:•isize,•y|•x•+•y•+•z    ClosureFunctionExpression{move}
		     |x:•isize,•y|              ClosureFunctionExpression.parameters{dk: "||"}
		      x:•isize                  ClosureFunctionParameterDeclaration
		                y               ClosureFunctionParameterDeclaration
		                   x•+•y•+•z    OperationExpression{tk: "+"}
		                   x•+•y        OperationExpression{tk: "+"}                                                                      */
		&mut || 22,                                                                                                                       /*
		&mut•||•22    ReferenceExpression{mut}
		     ||•22    ClosureFunctionExpression
		     ||       ClosureFunctionExpression.parameters{dk: "||"}
		        22    Literal{kind: Integer}                                                                                              */
		&|| 22,                                                                                                                           /*
		&||•22    ReferenceExpression{!mut}
		 ||•22    ClosureFunctionExpression
		 ||       ClosureFunctionExpression.parameters{dk: "||"}
		    22    Literal{kind: Integer}                                                                                                  */
		|| x += 1,                                                                                                                        /*
		||•x•+=•1    ClosureFunctionExpression
		||           ClosureFunctionExpression.parameters{dk: "||"}
		   x•+=•1    ReassignmentOperationExpression{tk: "+="}
		        1    Literal{kind: Integer}                                                                                               */
		call(&|| {}, ()),                                                                                                                 /*
		call(&||•{},•())    CallExpression
		    (&||•{},•())    CallExpression.arguments{dk: "()"}
		     &||•{}         ReferenceExpression{!mut}
		      ||•{}         ClosureFunctionExpression
		      ||            ClosureFunctionExpression.parameters{dk: "||"}
		         {}         BlockExpression
		             ()     TupleLiteral                                                                                                  */
		<()>::drive(|| ()),                                                                                                               /*
		<()>::drive(||•())    CallExpression
		<()>::drive           ExpressionPath
		<()>                  ExpressionTypeSelector
		 ()                   TypeTuple
		           (||•())    CallExpression.arguments{dk: "()"}
		            ||•()     ClosureFunctionExpression
		            ||        ClosureFunctionExpression.parameters{dk: "||"}
		               ()     TupleLiteral                                                                                                */
		h2(|_: (), _: (), _: (), _: ()| {}),                                                                                              /*
		h2(|_:•(),•_:•(),•_:•(),•_:•()|•{})    CallExpression
		  (|_:•(),•_:•(),•_:•(),•_:•()|•{})    CallExpression.arguments{dk: "()"}
		   |_:•(),•_:•(),•_:•(),•_:•()|•{}     ClosureFunctionExpression
		   |_:•(),•_:•(),•_:•(),•_:•()|        ClosureFunctionExpression.parameters{dk: "||"}
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
		move•|a:•isize,•b|•{•a•+•b•}    ClosureFunctionExpression{move}
		     |a:•isize,•b|              ClosureFunctionExpression.parameters{dk: "||"}
		      a:•isize                  ClosureFunctionParameterDeclaration
		                b               ClosureFunctionParameterDeclaration
		                   {•a•+•b•}    BlockExpression
		                     a•+•b      ExpressionStatement{!semi}, OperationExpression{tk: "+"}                                          */
		move |a: isize, b| { z; zz; a + b },                                                                                              /*
		move•|a:•isize,•b|•{•z;•zz;•a•+•b•}    ClosureFunctionExpression{move}
		     |a:•isize,•b|                     ClosureFunctionExpression.parameters{dk: "||"}
		      a:•isize                         ClosureFunctionParameterDeclaration
		                b                      ClosureFunctionParameterDeclaration
		                   {•z;•zz;•a•+•b•}    BlockExpression
		                     z;                ExpressionStatement{semi}
		                        zz;            ExpressionStatement{semi}
		                            a•+•b      ExpressionStatement{!semi}, OperationExpression{tk: "+"}                                   */
		|x: usize| x * 2,                                                                                                                 /*
		|x:•usize|•x•*•2    ClosureFunctionExpression
		|x:•usize|          ClosureFunctionExpression.parameters{dk: "||"}
		 x:•usize           ClosureFunctionParameterDeclaration
		           x•*•2    OperationExpression{tk: "*"}
		               2    Literal{kind: Integer}                                                                                        */
		|x: usize| { x } * 2,                                                                                                             /*
		|x:•usize|•{•x•}•*•2    ClosureFunctionExpression
		|x:•usize|              ClosureFunctionExpression.parameters{dk: "||"}
		 x:•usize               ClosureFunctionParameterDeclaration
		           {•x•}•*•2    OperationExpression{tk: "*"}
		           {•x•}        BlockExpression
		             x          ExpressionStatement{!semi}
		                   2    Literal{kind: Integer}                                                                                    */
		|x: usize| { x } (),                                                                                                              /*
		|x:•usize|•{•x•}•()    ClosureFunctionExpression
		|x:•usize|             ClosureFunctionExpression.parameters{dk: "||"}
		 x:•usize              ClosureFunctionParameterDeclaration
		           {•x•}•()    CallExpression
		           {•x•}       BlockExpression
		             x         ExpressionStatement{!semi}
		                 ()    CallExpression.arguments{dk: "()"}                                                                         */
		|x| lib::d!(x),                                                                                                                   /*
		|x|•lib::d!(x)    ClosureFunctionExpression
		|x|               ClosureFunctionExpression.parameters{dk: "||"}
		 x                ClosureFunctionParameterDeclaration
		    lib::d!(x)    MacroInvocation
		    lib::d        ExpressionPath
		           (x)    MacroInvocation.segments{dk: "()"}                                                                              */
		|x| match x { a => { g(a) } },                                                                                                    /*
		|x|•match•x•{•a•=>•{•g(a)•}•}    ClosureFunctionExpression
		|x|                              ClosureFunctionExpression.parameters{dk: "||"}
		 x                               ClosureFunctionParameterDeclaration
		    match•x•{•a•=>•{•g(a)•}•}    MatchExpression
		            {•a•=>•{•g(a)•}•}    MatchExpression.cases{dk: "{}"}
		              a•=>•{•g(a)•}      MatchExpressionCase
		                   {•g(a)•}      BlockExpression
		                     g(a)        ExpressionStatement{!semi}, CallExpression
		                      (a)        CallExpression.arguments{dk: "()"}                                                               */
		|x| d!(x),                                                                                                                        /*
		|x|•d!(x)    ClosureFunctionExpression
		|x|          ClosureFunctionExpression.parameters{dk: "||"}
		 x           ClosureFunctionParameterDeclaration
		    d!(x)    MacroInvocation
		      (x)    MacroInvocation.segments{dk: "()"}                                                                                   */
		|_| async { () },                                                                                                                 /*
		|_|•async•{•()•}    ClosureFunctionExpression
		|_|                 ClosureFunctionExpression.parameters{dk: "||"}
		 _                  ClosureFunctionParameterDeclaration, WildcardPattern
		    async•{•()•}    BlockExpression{async}
		          {•()•}    BlockExpression.body{dk: "{}"}
		            ()      ExpressionStatement{!semi}, TupleLiteral                                                                      */
		|x,y| {},                                                                                                                         /*
		|x,y|•{}    ClosureFunctionExpression
		|x,y|       ClosureFunctionExpression.parameters{dk: "||"}
		 x          ClosureFunctionParameterDeclaration
		   y        ClosureFunctionParameterDeclaration
		      {}    BlockExpression                                                                                                       */
		|x:&u64, y:&u64| {},                                                                                                              /*
		|x:&u64,•y:&u64|•{}    ClosureFunctionExpression
		|x:&u64,•y:&u64|       ClosureFunctionExpression.parameters{dk: "||"}
		 x:&u64                ClosureFunctionParameterDeclaration
		   &u64                TypeReference{!mut}
		         y:&u64        ClosureFunctionParameterDeclaration
		           &u64        TypeReference{!mut}
		                 {}    BlockExpression                                                                                            */
		|x:&u64, y| {},                                                                                                                   /*
		|x:&u64,•y|•{}    ClosureFunctionExpression
		|x:&u64,•y|       ClosureFunctionExpression.parameters{dk: "||"}
		 x:&u64           ClosureFunctionParameterDeclaration
		   &u64           TypeReference{!mut}
		         y        ClosureFunctionParameterDeclaration
		            {}    BlockExpression                                                                                                 */
		|x, y:&u64| {},                                                                                                                   /*
		|x,•y:&u64|•{}    ClosureFunctionExpression
		|x,•y:&u64|       ClosureFunctionExpression.parameters{dk: "||"}
		 x                ClosureFunctionParameterDeclaration
		    y:&u64        ClosureFunctionParameterDeclaration
		      &u64        TypeReference{!mut}
		            {}    BlockExpression                                                                                                 */
		match 0 { 2 => |a| 2, 1 => 0, },                                                                                                  /*
		match•0•{•2•=>•|a|•2,•1•=>•0,•}    MatchExpression
		      0                            Literal{kind: Integer}
		        {•2•=>•|a|•2,•1•=>•0,•}    MatchExpression.cases{dk: "{}"}
		          2•=>•|a|•2               MatchExpressionCase
		          2                        Literal{kind: Integer}
		               |a|•2               ClosureFunctionExpression
		               |a|                 ClosureFunctionExpression.parameters{dk: "||"}
		                a                  ClosureFunctionParameterDeclaration
		                   2               Literal{kind: Integer}
		                      1•=>•0       MatchExpressionCase
		                      1            Literal{kind: Integer}
		                           0       Literal{kind: Integer}                                                                         */
		[b, |a| 2],                                                                                                                       /*
		[b,•|a|•2]    ArrayLiteral
		    |a|•2     ClosureFunctionExpression
		    |a|       ClosureFunctionExpression.parameters{dk: "||"}
		     a        ClosureFunctionParameterDeclaration
		        2     Literal{kind: Integer}                                                                                              */
		[|a| 2, b],                                                                                                                       /*
		[|a|•2,•b]    ArrayLiteral
		 |a|•2        ClosureFunctionExpression
		 |a|          ClosureFunctionExpression.parameters{dk: "||"}
		  a           ClosureFunctionParameterDeclaration
		     2        Literal{kind: Integer}                                                                                              */
		async || 1,                                                                                                                       /*
		async•||•1    ClosureFunctionExpression{async}
		      ||      ClosureFunctionExpression.parameters{dk: "||"}
		         1    Literal{kind: Integer}                                                                                              */
		|ctx: Ctx<(String, String)>| -> io::Result<Response> {                                                                            /*
		|ctx:•Ctx<(String,•String)>|•->•io::Result<Response>•{↲    <ClosureFunctionExpression>
		|ctx:•Ctx<(String,•String)>|                               ClosureFunctionExpression.parameters{dk: "||"}
		 ctx:•Ctx<(String,•String)>                                ClosureFunctionParameterDeclaration
		      Ctx<(String,•String)>                                TypeCall
		         <(String,•String)>                                TypeCall.typeArguments{dk: "<>"}
		          (String,•String)                                 TypeTuple
		                                io::Result<Response>       TypeCall
		                                io::Result                 TypePath
		                                          <Response>       TypeCall.typeArguments{dk: "<>"}
		                                                     {↲    <BlockExpression>                                                      */
			Ok(Response::new().with_body(ctx.params.0))                                                                                   /*
			Ok(Response::new().with_body(ctx.params.0))    ExpressionStatement{!semi}, CallExpression
			  (Response::new().with_body(ctx.params.0))    CallExpression.arguments{dk: "()"}
			   Response::new().with_body(ctx.params.0)     CallExpression
			   Response::new()                             CallExpression
			   Response::new                               ExpressionPath
			                ()                             CallExpression.arguments{dk: "()"}
			                            (ctx.params.0)     CallExpression.arguments{dk: "()"}
			                             ctx.params.0      MemberExpression{!computed}
			                             ctx.params        MemberExpression{!computed}
			                                        0      Index                                                                          */
		},                                                                                                                                /*
      ╚╚}    </BlockExpression>
      ╚╚}    </ClosureFunctionExpression>                                                                                                 */
		rayon::join(                                                                                                                      /*
		rayon::join(↲    <CallExpression>
		rayon::join      ExpressionPath
		           (↲    <CallExpression.arguments{dk: "()"}>                                                                             */
			|| recurse(left, is_less, pred, limit),                                                                                       /*
			||•recurse(left,•is_less,•pred,•limit)    ClosureFunctionExpression
			||                                        ClosureFunctionExpression.parameters{dk: "||"}
			   recurse(left,•is_less,•pred,•limit)    CallExpression
			          (left,•is_less,•pred,•limit)    CallExpression.arguments{dk: "()"}                                                  */
			|| recurse(right, is_less, Some(pivot), limit),                                                                               /*
			||•recurse(right,•is_less,•Some(pivot),•limit)    ClosureFunctionExpression
			||                                                ClosureFunctionExpression.parameters{dk: "||"}
			   recurse(right,•is_less,•Some(pivot),•limit)    CallExpression
			          (right,•is_less,•Some(pivot),•limit)    CallExpression.arguments{dk: "()"}
			                           Some(pivot)            CallExpression
			                               (pivot)            CallExpression.arguments{dk: "()"}                                          */
		),                                                                                                                                /*
      ╚╚)    </CallExpression.arguments>
      ╚╚)    </CallExpression>                                                                                                            */
		rayon::join(                                                                                                                      /*
		rayon::join(↲    <CallExpression>
		rayon::join      ExpressionPath
		           (↲    <CallExpression.arguments{dk: "()"}>                                                                             */
			1,                                                                                                                            /*
			1    Literal{kind: Integer}                                                                                                   */
			|| recurse(left, is_less, pred, limit),                                                                                       /*
			||•recurse(left,•is_less,•pred,•limit)    ClosureFunctionExpression
			||                                        ClosureFunctionExpression.parameters{dk: "||"}
			   recurse(left,•is_less,•pred,•limit)    CallExpression
			          (left,•is_less,•pred,•limit)    CallExpression.arguments{dk: "()"}                                                  */
			2,                                                                                                                            /*
			2    Literal{kind: Integer}                                                                                                   */
			|| recurse(right, is_less, Some(pivot), limit),                                                                               /*
			||•recurse(right,•is_less,•Some(pivot),•limit)    ClosureFunctionExpression
			||                                                ClosureFunctionExpression.parameters{dk: "||"}
			   recurse(right,•is_less,•Some(pivot),•limit)    CallExpression
			          (right,•is_less,•Some(pivot),•limit)    CallExpression.arguments{dk: "()"}
			                           Some(pivot)            CallExpression
			                               (pivot)            CallExpression.arguments{dk: "()"}                                          */
		),                                                                                                                                /*
      ╚╚)    </CallExpression.arguments>
      ╚╚)    </CallExpression>                                                                                                            */
	];                                                                                                                                    /*
   ╚]     </ArrayLiteral>
   ╚];    </ExpressionStatement>                                                                                                          */
	bifornCringer =                                                                                                                       /*
	bifornCringer•=↲    <ExpressionStatement{semi}>
	bifornCringer•=↲    <ReassignmentExpression{tk: "="}>                                                                                 */
  		askTrovenaBeenaDepends =                                                                                                          /*
  		askTrovenaBeenaDepends•=↲    <ReassignmentExpression{tk: "="}>                                                                    */
  		glimseGlyphs =                                                                                                                    /*
  		glimseGlyphs•=↲    <ReassignmentExpression{tk: "="}>                                                                              */
  		  	|argumentOne, argumentTwo, argumentThree|                                                                                     /*
  		  	|argumentOne,•argumentTwo,•argumentThree|↲    <ClosureFunctionExpression>
  		  	|argumentOne,•argumentTwo,•argumentThree|     ClosureFunctionExpression.parameters{dk: "||"}
  		  	 argumentOne                                  ClosureFunctionParameterDeclaration
  		  	              argumentTwo                     ClosureFunctionParameterDeclaration
  		  	                           argumentThree      ClosureFunctionParameterDeclaration                                             */
  		  	|restOfTheArguments12345678| {                                                                                                /*
  		  	|restOfTheArguments12345678|•{↲    <ClosureFunctionExpression>
  		  	|restOfTheArguments12345678|       ClosureFunctionExpression.parameters{dk: "||"}
  		  	 restOfTheArguments12345678        ClosureFunctionParameterDeclaration
  		  	                             {↲    <BlockExpression>                                                                          */
  		  		return "baz";                                                                                                             /*
  		  		return•"baz";    ExpressionStatement{semi}
  		  		return•"baz"     ReturnExpression
  		  		       "baz"     Literal{kind: String}                                                                                    */
  		  	};                                                                                                                            /*
••╚╚••╚}     </BlockExpression>
••╚╚••╚}     </ClosureFunctionExpression>
••╚╚••╚}     </ClosureFunctionExpression>
••╚╚••╚}     </ReassignmentExpression>
••╚╚••╚}     </ReassignmentExpression>
••╚╚••╚}     </ReassignmentExpression>
••╚╚••╚};    </ExpressionStatement>                                                                                                       */
	aaaaaaaaaaaaaaaa.map(|x| {                                                                                                            /*
	aaaaaaaaaaaaaaaa.map(|x|•{↲    <ExpressionStatement{semi}>
	aaaaaaaaaaaaaaaa.map(|x|•{↲    <MemberExpression{!computed}>
	aaaaaaaaaaaaaaaa.map(|x|•{↲    <CallExpression>
	                    (|x|•{↲    <CallExpression.arguments{dk: "()"}>
	                     |x|•{↲    <ClosureFunctionExpression>
	                     |x|       ClosureFunctionExpression.parameters{dk: "||"}
	                      x        ClosureFunctionParameterDeclaration
	                         {↲    <BlockExpression>                                                                                      */
        x += 1;                                                                                                                           /*
        x•+=•1;    ExpressionStatement{semi}
        x•+=•1     ReassignmentOperationExpression{tk: "+="}
             1     Literal{kind: Integer}                                                                                                 */
        x                                                                                                                                 /*
        x    ExpressionStatement{!semi}                                                                                                   */
    })                                                                                                                                    /*
••••}     </BlockExpression>
••••}     </ClosureFunctionExpression>
••••})    </CallExpression.arguments>
••••})    </CallExpression>                                                                                                               */
        .filter;                                                                                                                          /*
••••••••.filter     </MemberExpression>
••••••••.filter;    </ExpressionStatement>                                                                                                */
	let f = |x| {{{{x}}}};                                                                                                                /*
	let•f•=•|x|•{{{{x}}}};    LetVariableDeclaration
	        |x|•{{{{x}}}}     ClosureFunctionExpression
	        |x|               ClosureFunctionExpression.parameters{dk: "||"}
	         x                ClosureFunctionParameterDeclaration
	            {{{{x}}}}     BlockExpression
	             {{{x}}}      ExpressionStatement{!semi}, BlockExpression
	              {{x}}       ExpressionStatement{!semi}, BlockExpression
	               {x}        ExpressionStatement{!semi}, BlockExpression
	                x         ExpressionStatement{!semi}                                                                                  */
    let f = |x| {{{x}}};                                                                                                                  /*
    let•f•=•|x|•{{{x}}};    LetVariableDeclaration
            |x|•{{{x}}}     ClosureFunctionExpression
            |x|             ClosureFunctionExpression.parameters{dk: "||"}
             x              ClosureFunctionParameterDeclaration
                {{{x}}}     BlockExpression
                 {{x}}      ExpressionStatement{!semi}, BlockExpression
                  {x}       ExpressionStatement{!semi}, BlockExpression
                   x        ExpressionStatement{!semi}                                                                                    */
    let f = |x| {{x}};                                                                                                                    /*
    let•f•=•|x|•{{x}};    LetVariableDeclaration
            |x|•{{x}}     ClosureFunctionExpression
            |x|           ClosureFunctionExpression.parameters{dk: "||"}
             x            ClosureFunctionParameterDeclaration
                {{x}}     BlockExpression
                 {x}      ExpressionStatement{!semi}, BlockExpression
                  x       ExpressionStatement{!semi}                                                                                      */
    let f = |x| {x};                                                                                                                      /*
    let•f•=•|x|•{x};    LetVariableDeclaration
            |x|•{x}     ClosureFunctionExpression
            |x|         ClosureFunctionExpression.parameters{dk: "||"}
             x          ClosureFunctionParameterDeclaration
                {x}     BlockExpression
                 x      ExpressionStatement{!semi}                                                                                        */
    let f = |x| x;                                                                                                                        /*
    let•f•=•|x|•x;    LetVariableDeclaration
            |x|•x     ClosureFunctionExpression
            |x|       ClosureFunctionExpression.parameters{dk: "||"}
             x        ClosureFunctionParameterDeclaration                                                                                 */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

fn f(_n: isize) -> isize { id(|| { 1 }) - 0 }                                                                                             /*
fn•f(_n:•isize)•->•isize•{•id(||•{•1•})•-•0•}    FunctionDeclaration
    (_n:•isize)                                  FunctionDeclaration.parameters{dk: "()"}
     _n:•isize                                   FunctionParameterDeclaration
                         {•id(||•{•1•})•-•0•}    FunctionDeclaration.body{dk: "{}"}
                           id(||•{•1•})•-•0      ExpressionStatement{!semi}, OperationExpression{tk: "-"}
                           id(||•{•1•})          CallExpression
                             (||•{•1•})          CallExpression.arguments{dk: "()"}
                              ||•{•1•}           ClosureFunctionExpression
                              ||                 ClosureFunctionExpression.parameters{dk: "||"}
                                 {•1•}           BlockExpression
                                   1             ExpressionStatement{!semi}, Literal{kind: Integer}
                                          0      Literal{kind: Integer}                                                                   */
fn f(){ || x += y }                                                                                                                       /*
fn•f(){•||•x•+=•y•}    FunctionDeclaration
    ()                 FunctionDeclaration.parameters{dk: "()"}
      {•||•x•+=•y•}    FunctionDeclaration.body{dk: "{}"}
        ||•x•+=•y      ExpressionStatement{!semi}, ClosureFunctionExpression
        ||             ClosureFunctionExpression.parameters{dk: "||"}
           x•+=•y      ReassignmentOperationExpression{tk: "+="}                                                                          */
struct A {                                                                                                                                /*
struct•A•{↲    <StructDeclaration>
         {↲    <StructDeclaration.properties{dk: "{}"}>                                                                                   */
    b: [(); match || 1 { a => 0 }],                                                                                                       /*
    b:•[();•match•||•1•{•a•=>•0•}]    StructPropertyDeclaration
       [();•match•||•1•{•a•=>•0•}]    TypeSizedArray
        ()                            TypeTuple
            match•||•1•{•a•=>•0•}     MatchExpression
                  ||•1                ClosureFunctionExpression
                  ||                  ClosureFunctionExpression.parameters{dk: "||"}
                     1                Literal{kind: Integer}
                       {•a•=>•0•}     MatchExpression.cases{dk: "{}"}
                         a•=>•0       MatchExpressionCase
                              0       Literal{kind: Integer}                                                                              */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */
enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>
       {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                          */
    V([(); { let _ = || 1; 0 }]),                                                                                                         /*
    V([();•{•let•_•=•||•1;•0•}])    EnumMemberTupleDeclaration
     ([();•{•let•_•=•||•1;•0•}])    EnumMemberTupleDeclaration.items{dk: "()"}
      [();•{•let•_•=•||•1;•0•}]     TupleStructItemDeclaration, TypeSizedArray
       ()                           TypeTuple
           {•let•_•=•||•1;•0•}      BlockExpression
             let•_•=•||•1;          LetVariableDeclaration
                 _                  WildcardPattern
                     ||•1           ClosureFunctionExpression
                     ||             ClosureFunctionExpression.parameters{dk: "||"}
                        1           Literal{kind: Integer}
                           0        ExpressionStatement{!semi}, Literal{kind: Integer}                                                    */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
type Ty = [(); { let _ = || 1; 0 }];                                                                                                      /*
type•Ty•=•[();•{•let•_•=•||•1;•0•}];    TypeAliasDeclaration
          [();•{•let•_•=•||•1;•0•}]     TypeSizedArray
           ()                           TypeTuple
               {•let•_•=•||•1;•0•}      BlockExpression
                 let•_•=•||•1;          LetVariableDeclaration
                     _                  WildcardPattern
                         ||•1           ClosureFunctionExpression
                         ||             ClosureFunctionExpression.parameters{dk: "||"}
                            1           Literal{kind: Integer}
                               0        ExpressionStatement{!semi}, Literal{kind: Integer}
type•Ty•=•[();•{•let•_•=•||•1;•0•}];    </Program.ast>
type•Ty•=•[();•{•let•_•=•||•1;•0•}];    </Program>                                                                                        */
// Discarded Nodes: 13
// Parsed Nodes: 963
// state_rollbacks: 14
// Total '.charCodeAt()' calls: 4074 (32% re-reads)
// Unnecessary 'skip_whitespace()' calls: 637
// source: "../../samples/expressions/closure.rs"