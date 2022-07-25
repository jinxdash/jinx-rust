fn a() {                                                                                                                                  /*
fn•a()•{↲    <FunctionDeclaration>                                                                                                        */
	match x {}                                                                                                                            /*
    match•x•{}    ExpressionStatement, MatchExpression                                                                                    */
	match () {}                                                                                                                           /*
    match•()•{}    ExpressionStatement, MatchExpression
          ()       TupleLiteral                                                                                                           */
	match (Sd { x: A, y: () }) {}                                                                                                         /*
    match•(Sd•{•x:•A,•y:•()•})•{}    ExpressionStatement, MatchExpression
           Sd•{•x:•A,•y:•()•}        StructLiteral
                x:•A                 StructLiteralProperty
                      y:•()          StructLiteralProperty
                         ()          TupleLiteral                                                                                         */
	match *c {}                                                                                                                           /*
    match•*c•{}    ExpressionStatement, MatchExpression
          *c       DereferenceExpression                                                                                                  */
	match ((A, ()), ()) {}                                                                                                                /*
    match•((A,•()),•())•{}    ExpressionStatement, MatchExpression
          ((A,•()),•())       TupleLiteral
           (A,•())            TupleLiteral
               ()             TupleLiteral
                    ()        TupleLiteral                                                                                                */
	match [0u8; LARGE_SIZE] {}                                                                                                            /*
    match•[0u8;•LARGE_SIZE]•{}    ExpressionStatement, MatchExpression
          [0u8;•LARGE_SIZE]       SizedArrayLiteral
           0u8                    Literal
            u8                    Identifier                                                                                              */
	match na.kind {}                                                                                                                      /*
    match•na.kind•{}    ExpressionStatement, MatchExpression
          na.kind       MemberExpression                                                                                                  */
	match (T::T1(()), V::V2(true)) {}                                                                                                     /*
    match•(T::T1(()),•V::V2(true))•{}    ExpressionStatement, MatchExpression
          (T::T1(()),•V::V2(true))       TupleLiteral
           T::T1(())                     CallExpression
           T::T1                         ExpressionPath
                 ()                      TupleLiteral
                      V::V2(true)        CallExpression
                      V::V2              ExpressionPath
                            true         Literal                                                                                          */
	match (Sd { x: A, y: () }) {}                                                                                                         /*
    match•(Sd•{•x:•A,•y:•()•})•{}    ExpressionStatement, MatchExpression
           Sd•{•x:•A,•y:•()•}        StructLiteral
                x:•A                 StructLiteralProperty
                      y:•()          StructLiteralProperty
                         ()          TupleLiteral                                                                                         */
	match "a" {}                                                                                                                          /*
    match•"a"•{}    ExpressionStatement, MatchExpression
          "a"       Literal                                                                                                               */
	match (&"foo", "bar") {}                                                                                                              /*
    match•(&"foo",•"bar")•{}    ExpressionStatement, MatchExpression
          (&"foo",•"bar")       TupleLiteral
           &"foo"               ReferenceExpression
            "foo"               Literal
                   "bar"        Literal                                                                                                   */
	match (Foo{foo: true, bar: Some(10), baz: 20}) {}                                                                                     /*
    match•(Foo{foo:•true,•bar:•Some(10),•baz:•20})•{}    ExpressionStatement, MatchExpression
           Foo{foo:•true,•bar:•Some(10),•baz:•20}        StructLiteral
               foo:•true                                 StructLiteralProperty
                    true                                 Literal
                          bar:•Some(10)                  StructLiteralProperty
                               Some(10)                  CallExpression
                                    10                   Literal
                                         baz:•20         StructLiteralProperty
                                              20         Literal                                                                          */
	match (l1, l2) {}                                                                                                                     /*
    match•(l1,•l2)•{}    ExpressionStatement, MatchExpression
          (l1,•l2)       TupleLiteral                                                                                                     */

	match 0 { 0 if false => () }                                                                                                          /*
    match•0•{•0•if•false•=>•()•}    ExpressionStatement, MatchExpression
          0                         Literal
              0•if•false•=>•()      MatchExpressionCase
              0                     Literal
                   false            Literal
                            ()      TupleLiteral                                                                                          */
	match true { true => true }                                                                                                           /*
    match•true•{•true•=>•true•}    ExpressionStatement, MatchExpression
          true                     Literal
                 true•=>•true      MatchExpressionCase
                 true              Literal
                         true      Literal                                                                                                */

	let v: isize = match &*sl {};                                                                                                         /*
    let•v:•isize•=•match•&*sl•{};    LetVariableDeclaration
                   match•&*sl•{}     MatchExpression
                         &*sl        ReferenceExpression
                          *sl        DereferenceExpression                                                                                */
	let a: isize = match 1 {                                                                                                              /*
    let•a:•isize•=•match•1•{•↲    <LetVariableDeclaration>
                   match•1•{•↲    <MatchExpression>
                         1        Literal                                                                                                 */
		x if x < 2 => { 3 }                                                                                                               /*
        x•if•x•<•2•=>•{•3•}    MatchExpressionCase
             x•<•2             ComparisonExpression
                 2             Literal
                      {•3•}    BlockExpression
                        3      ExpressionStatement, Literal                                                                               */
		x if x < 4 => { 5 }                                                                                                               /*
        x•if•x•<•4•=>•{•5•}    MatchExpressionCase
             x•<•4             ComparisonExpression
                 4             Literal
                      {•5•}    BlockExpression
                        5      ExpressionStatement, Literal                                                                               */
		6 => { 7 }                                                                                                                        /*
        6•=>•{•7•}    MatchExpressionCase
        6             Literal
             {•7•}    BlockExpression
               7      ExpressionStatement, Literal                                                                                        */
		_ => { 8 }                                                                                                                        /*
        _•=>•{•8•}    MatchExpressionCase
        _             WildcardPattern
             {•8•}    BlockExpression
               8      ExpressionStatement, Literal                                                                                        */
	};                                                                                                                                    /*
   ╚};    </LetVariableDeclaration>
   ╚}     </MatchExpression>                                                                                                              */
	let val = match match match match match () { () => () } { () => () } { () => () } { () => () } { () => () };                          /*
    let•val•=•match•match•match•match•match•()•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•};    LetVariableDeclaration
              match•match•match•match•match•()•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}     MatchExpression
                    match•match•match•match•()•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}                  MatchExpression
                          match•match•match•()•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}                               MatchExpression
                                match•match•()•{•()•=>•()•}•{•()•=>•()•}                                            MatchExpression
                                      match•()•{•()•=>•()•}                                                         MatchExpression
                                            ()                                                                      TupleLiteral
                                                 ()•=>•()                                                           MatchExpressionCase
                                                 ()                                                                 TuplePattern
                                                       ()                                                           TupleLiteral
                                                              ()•=>•()                                              MatchExpressionCase
                                                              ()                                                    TuplePattern
                                                                    ()                                              TupleLiteral
                                                                           ()•=>•()                                 MatchExpressionCase
                                                                           ()                                       TuplePattern
                                                                                 ()                                 TupleLiteral
                                                                                        ()•=>•()                    MatchExpressionCase
                                                                                        ()                          TuplePattern
                                                                                              ()                    TupleLiteral
                                                                                                     ()•=>•()       MatchExpressionCase
                                                                                                     ()             TuplePattern
                                                                                                           ()       TupleLiteral          */
	let b: isize =                                                                                                                        /*
    let•b:•isize•=↲    <LetVariableDeclaration>                                                                                           */
		match (A {x: 10, y: 20}) {                                                                                                        /*
        match•(A•{x:•10,•y:•20})•{↲    <MatchExpression>
               A•{x:•10,•y:•20}        StructLiteral
                  x:•10                StructLiteralProperty
                     10                Literal
                         y:•20         StructLiteralProperty
                            20         Literal                                                                                            */
			x if x.x < 5 && x.y < 5 => { 1 }                                                                                              /*
            x•if•x.x•<•5•&&•x.y•<•5•=>•{•1•}    MatchExpressionCase
                 x.x•<•5•&&•x.y•<•5             AndExpression
                 x.x•<•5                        ComparisonExpression
                 x.x                            MemberExpression
                       5                        Literal
                            x.y•<•5             ComparisonExpression
                            x.y                 MemberExpression
                                  5             Literal
                                       {•1•}    BlockExpression
                                         1      ExpressionStatement, Literal                                                              */
			A {..} if x == 10 && y == 20 => { 2 }                                                                                         /*
            A•{..}•if•x•==•10•&&•y•==•20•=>•{•2•}    MatchExpressionCase
            A•{..}                                   StructPattern
               ..                                    RestPattern
                      x•==•10•&&•y•==•20             AndExpression
                      x•==•10                        ComparisonExpression
                           10                        Literal
                                 y•==•20             ComparisonExpression
                                      20             Literal
                                            {•2•}    BlockExpression
                                              2      ExpressionStatement, Literal                                                         */
			A {..} => { 3 }                                                                                                               /*
            A•{..}•=>•{•3•}    MatchExpressionCase
            A•{..}             StructPattern
               ..              RestPattern
                      {•3•}    BlockExpression
                        3      ExpressionStatement, Literal                                                                               */
		};                                                                                                                                /*
   ╚╚};    </LetVariableDeclaration>
   ╚╚}     </MatchExpression>                                                                                                             */

	match true {                                                                                                                          /*
    match•true•{↲    <ExpressionStatement>, <MatchExpression>
          true       Literal                                                                                                              */
        true if true => (),                                                                                                               /*
        true•if•true•=>•()     MatchExpressionCase
        true                   Literal
                true           Literal
                        ()     TupleLiteral                                                                                               */
        false if false => unsafe { },                                                                                                     /*
        false•if•false•=>•unsafe•{•}     MatchExpressionCase
        false                            Literal
                 false                   Literal
                          unsafe•{•}     BlockExpression                                                                                  */
        true => { }                                                                                                                       /*
        true•=>•{•}    MatchExpressionCase
        true           Literal
                {•}    BlockExpression                                                                                                    */
        false => (),                                                                                                                      /*
        false•=>•()     MatchExpressionCase
        false           Literal
                 ()     TupleLiteral                                                                                                      */
		&[] => 0,                                                                                                                         /*
        &[]•=>•0    MatchExpressionCase
        &[]         ReferencePattern
         []         ArrayPattern
               0    Literal                                                                                                               */
        &[a,b,c] => 3,                                                                                                                    /*
        &[a,b,c]•=>•3     MatchExpressionCase
        &[a,b,c]          ReferencePattern
         [a,b,c]          ArrayPattern
                    3     Literal                                                                                                         */
        &[a, ref d @ ..] => a,                                                                                                            /*
        &[a,•ref•d•@•..]•=>•a     MatchExpressionCase
        &[a,•ref•d•@•..]          ReferencePattern
         [a,•ref•d•@•..]          ArrayPattern
             ref•d•@•..           PatternVariableDeclaration
                     ..           RestPattern                                                                                             */
        &[10,a, ref d @ ..] => 10,                                                                                                        /*
        &[10,a,•ref•d•@•..]•=>•10     MatchExpressionCase
        &[10,a,•ref•d•@•..]           ReferencePattern
         [10,a,•ref•d•@•..]           ArrayPattern
          10                          Literal
                ref•d•@•..            PatternVariableDeclaration
                        ..            RestPattern
                               10     Literal                                                                                             */
        [h, ..] if h > n => 0,                                                                                                            /*
        [h,•..]•if•h•>•n•=>•0     MatchExpressionCase
        [h,•..]                   ArrayPattern
            ..                    RestPattern
                   h•>•n          ComparisonExpression
                            0     Literal                                                                                                 */
        [h, ..] if h == n => 1,                                                                                                           /*
        [h,•..]•if•h•==•n•=>•1     MatchExpressionCase
        [h,•..]                    ArrayPattern
            ..                     RestPattern
                   h•==•n          ComparisonExpression
                             1     Literal                                                                                                */
        [h, ref ts] => foo(c, n - h) + foo(ts, n),                                                                                        /*
        [h,•ref•ts]•=>•foo(c,•n•-•h)•+•foo(ts,•n)     MatchExpressionCase
        [h,•ref•ts]                                   ArrayPattern
            ref•ts                                    PatternVariableDeclaration
                       foo(c,•n•-•h)•+•foo(ts,•n)     OperationExpression
                       foo(c,•n•-•h)                  CallExpression
                              n•-•h                   OperationExpression
                                       foo(ts,•n)     CallExpression                                                                      */
        [] => 0,                                                                                                                          /*
        []•=>•0     MatchExpressionCase
        []          ArrayPattern
              0     Literal                                                                                                               */
		&A::C(v, box ref a) => tail(e),                                                                                                   /*
        &A::C(v,•box•ref•a)•=>•tail(e)    MatchExpressionCase
        &A::C(v,•box•ref•a)               ReferencePattern
         A::C(v,•box•ref•a)               TuplePattern
         A::C                             ExpressionPath
                 box•ref•a                BoxPattern
                     ref•a                PatternVariableDeclaration
                               tail(e)    CallExpression                                                                                  */
        &A::C(x, box A::S)  => A::C(c, box A::R),                                                                                         /*
        &A::C(x,•box•A::S)••=>•A::C(c,•box•A::R)     MatchExpressionCase
        &A::C(x,•box•A::S)                           ReferencePattern
         A::C(x,•box•A::S)                           TuplePattern
         A::C                                        ExpressionPath
                 box•A::S                            BoxPattern
                     A::S                            ExpressionPath
                               A::C(c,•box•A::R)     CallExpression
                               A::C                  ExpressionPath
                                       box•A::R      BoxExpression
                                           A::R      ExpressionPath                                                                       */
		0 => return e(j::h::r(a::e::d, "")),                                                                                              /*
        0•=>•return•e(j::h::r(a::e::d,•""))    MatchExpressionCase
        0                                      Literal
             return•e(j::h::r(a::e::d,•""))    ReturnExpression
                    e(j::h::r(a::e::d,•""))    CallExpression
                      j::h::r(a::e::d,•"")     CallExpression
                      j::h::r                  ExpressionPath
                      j::h                     ExpressionPath
                              a::e::d          ExpressionPath
                              a::e             ExpressionPath
                                       ""      Literal                                                                                    */
		n => r = &mut a::d(&mut e, &mut [])[n..],                                                                                         /*
        n•=>•r•=•&mut•a::d(&mut•e,•&mut•[])[n..]    MatchExpressionCase
             r•=•&mut•a::d(&mut•e,•&mut•[])[n..]    ReassignmentExpression
                 &mut•a::d(&mut•e,•&mut•[])[n..]    ReferenceExpression
                      a::d(&mut•e,•&mut•[])[n..]    MemberExpression
                      a::d(&mut•e,•&mut•[])         CallExpression
                      a::d                          ExpressionPath
                           &mut•e                   ReferenceExpression
                                   &mut•[]          ReferenceExpression
                                        []          ArrayLiteral
                                            n..     RangeLiteral                                                                          */
        box Q::V(ed) =>                                                                                                                   /*
        box•Q::V(ed)•=>•↲    <MatchExpressionCase>
        box•Q::V(ed)         BoxPattern
            Q::V(ed)         TuplePattern
            Q::V             ExpressionPath                                                                                               */
			match ed.q {                                                                                                                  /*
            match•ed.q•{↲    <MatchExpression>
                  ed.q       MemberExpression                                                                                             */
           		box R::E(ref d) if d.d.r() => { true }                                                                                 /*
                   box•R::E(ref•d)•if•d.d.r()•=>•{•true•}    MatchExpressionCase
                   box•R::E(ref•d)                           BoxPattern
                       R::E(ref•d)                           TuplePattern
                       R::E                                  ExpressionPath
                            ref•d                            PatternVariableDeclaration
                                      d.d.r()                CallExpression
                                      d.d                    MemberExpression
                                                 {•true•}    BlockExpression
                                                   true      ExpressionStatement, Literal                                                 */
        	},                                                                                                                            /*
••••••••╚}     </MatchExpressionCase>, </MatchExpression>                                                                                 */
		_ => panic!(),                                                                                                                    /*
        _•=>•panic!()    MatchExpressionCase
        _                WildcardPattern
             panic!()    MacroInvocation                                                                                                  */
		ref _x => unreachable!(),                                                                                                         /*
        ref•_x•=>•unreachable!()    MatchExpressionCase
        ref•_x                      PatternVariableDeclaration
                  unreachable!()    MacroInvocation                                                                                       */
        0 => return,                                                                                                                      /*
        0•=>•return     MatchExpressionCase
        0               Literal
             return     ReturnExpression                                                                                                  */
		A { a: v } if *v.clone() == 42 => v,                                                                                              /*
        A•{•a:•v•}•if•*v.clone()•==•42•=>•v    MatchExpressionCase
        A•{•a:•v•}                             StructPattern
            a:•v                               StructPatternPropertyDestructured
                      *v.clone()•==•42         ComparisonExpression
                      *v.clone()               DereferenceExpression
                       v.clone()               CallExpression
                                    42         Literal                                                                                    */
		A((a,)) => *a = 0,                                                                                                                /*
        A((a,))•=>•*a•=•0    MatchExpressionCase
        A((a,))              TuplePattern
          (a,)               TuplePattern
                   *a•=•0    ReassignmentExpression
                   *a        DereferenceExpression
                        0    Literal                                                                                                      */
		Some(x) if let Some(y) = x => (x, y),                                                                                             /*
        Some(x)•if•let•Some(y)•=•x•=>•(x,•y)    MatchExpressionCase
        Some(x)                                 TuplePattern
                   let•Some(y)•=•x              LetScrutinee
                       Some(y)                  TuplePattern
                                      (x,•y)    TupleLiteral                                                                              */
		Some((x, _)) if let Foo::Bar = bar(x) => panic!(),                                                                                /*
        Some((x,•_))•if•let•Foo::Bar•=•bar(x)•=>•panic!()    MatchExpressionCase
        Some((x,•_))                                         TuplePattern
             (x,•_)                                          TuplePattern
                 _                                           WildcardPattern
                        let•Foo::Bar•=•bar(x)                LetScrutinee
                            Foo::Bar                         ExpressionPath
                                       bar(x)                CallExpression
                                                 panic!()    MacroInvocation                                                              */
        Some((_, x)) if let Foo::Baz = baz(x) => {},                                                                                      /*
        Some((_,•x))•if•let•Foo::Baz•=•baz(x)•=>•{}     MatchExpressionCase
        Some((_,•x))                                    TuplePattern
             (_,•x)                                     TuplePattern
              _                                         WildcardPattern
                        let•Foo::Baz•=•baz(x)           LetScrutinee
                            Foo::Baz                    ExpressionPath
                                       baz(x)           CallExpression
                                                 {}     BlockExpression                                                                   */
        Some(x) if let Foo::Qux(y) = qux(x) => assert_eq!(y, 84),                                                                         /*
        Some(x)•if•let•Foo::Qux(y)•=•qux(x)•=>•assert_eq!(y,•84)     MatchExpressionCase
        Some(x)                                                      TuplePattern
                   let•Foo::Qux(y)•=•qux(x)                          LetScrutinee
                       Foo::Qux(y)                                   TuplePattern
                       Foo::Qux                                      ExpressionPath
                                     qux(x)                          CallExpression
                                               assert_eq!(y,•84)     MacroInvocation
                                                           ,         PunctuationToken
                                                             84      Literal                                                              */
        Ok(mut r) | Err(mut r) if true => r = 1,                                                                                          /*
        Ok(mut•r)•|•Err(mut•r)•if•true•=>•r•=•1     MatchExpressionCase
        Ok(mut•r)•|•Err(mut•r)                      UnionPattern
        Ok(mut•r)                                   TuplePattern
           mut•r                                    PatternVariableDeclaration
                    Err(mut•r)                      TuplePattern
                        mut•r                       PatternVariableDeclaration
                                  true              Literal
                                          r•=•1     ReassignmentExpression
                                              1     Literal                                                                               */
        Color::Rgb(r, g, b) => r | g == 0 || r | b == 0 || g | b == 0,                                                                    /*
        Color::Rgb(r,•g,•b)•=>•r•|•g•==•0•||•r•|•b•==•0•||•g•|•b•==•0     MatchExpressionCase
        Color::Rgb(r,•g,•b)                                               TuplePattern
        Color::Rgb                                                        ExpressionPath
                               r•|•g•==•0•||•r•|•b•==•0•||•g•|•b•==•0     OrExpression
                               r•|•g•==•0•||•r•|•b•==•0                   OrExpression
                               r•|•g•==•0                                 ComparisonExpression
                               r•|•g                                      OperationExpression
                                        0                                 Literal
                                             r•|•b•==•0                   ComparisonExpression
                                             r•|•b                        OperationExpression
                                                      0                   Literal
                                                           g•|•b•==•0     ComparisonExpression
                                                           g•|•b          OperationExpression
                                                                    0     Literal                                                         */
        not_red @ Color::Green | not_red @ Color::Blue | not_red @ Color::Rgb(..) | not_red @ Color::Cyan => format!("{:?}", not_red),    /*
        not_red•@•Color::Green•|•not_red•@•Color::Blue•|•not_red•@•Color::Rgb(..)•|•not_red•@•Color::Cyan•=>•format!("{:?}",•not_red)     MatchExpressionCase
        not_red•@•Color::Green•|•not_red•@•Color::Blue•|•not_red•@•Color::Rgb(..)•|•not_red•@•Color::Cyan                                 UnionPattern
        not_red•@•Color::Green                                                                                                            PatternVariableDeclaration
                  Color::Green                                                                                                            ExpressionPath
                                 not_red•@•Color::Blue                                                                                    PatternVariableDeclaration
                                           Color::Blue                                                                                    ExpressionPath
                                                         not_red•@•Color::Rgb(..)                                                         PatternVariableDeclaration
                                                                   Color::Rgb(..)                                                         TuplePattern
                                                                   Color::Rgb                                                             ExpressionPath
                                                                              ..                                                          RestPattern
                                                                                    not_red•@•Color::Cyan                                 PatternVariableDeclaration
                                                                                              Color::Cyan                                 ExpressionPath
                                                                                                             format!("{:?}",•not_red)     MacroInvocation
                                                                                                                     "{:?}"               Literal
                                                                                                                           ,              PunctuationToken*/
        Ok(x) if let Err(_) = x => {},                                                                                                    /*
        Ok(x)•if•let•Err(_)•=•x•=>•{}     MatchExpressionCase
        Ok(x)                             TuplePattern
                 let•Err(_)•=•x           LetScrutinee
                     Err(_)               TuplePattern
                         _                WildcardPattern
                                   {}     BlockExpression                                                                                 */
		// _ if let _ = !Foo{ a: 1 } => {},
        //•_•if•let•_•=•!Foo{•a:•1•}•=>•{},    Comment
		_ if !Foo{ a: 1 } => {}                                                                                                           /*
        _•if•!Foo{•a:•1•}•=>•{}    MatchExpressionCase
        _                          WildcardPattern
             !Foo{•a:•1•}          NotExpression
              Foo{•a:•1•}          StructLiteral
                   a:•1            StructLiteralProperty
                      1            Literal
                             {}    BlockExpression                                                                                        */
		E { x: A, y: _ } => {}                                                                                                            /*
        E•{•x:•A,•y:•_•}•=>•{}    MatchExpressionCase
        E•{•x:•A,•y:•_•}          StructPattern
            x:•A                  StructPatternPropertyDestructured
                  y:•_            StructPatternPropertyDestructured
                     _            WildcardPattern
                            {}    BlockExpression                                                                                         */
        D { a: _a } | C { a: _a } if true => {}                                                                                           /*
        D•{•a:•_a•}•|•C•{•a:•_a•}•if•true•=>•{}    MatchExpressionCase
        D•{•a:•_a•}•|•C•{•a:•_a•}                  UnionPattern
        D•{•a:•_a•}                                StructPattern
            a:•_a                                  StructPatternPropertyDestructured
                      C•{•a:•_a•}                  StructPattern
                          a:•_a                    StructPatternPropertyDestructured
                                     true          Literal
                                             {}    BlockExpression                                                                        */
		Some(a::B { misc: false, .. }) => {}                                                                                              /*
        Some(a::B•{•misc:•false,•..•})•=>•{}    MatchExpressionCase
        Some(a::B•{•misc:•false,•..•})          TuplePattern
             a::B•{•misc:•false,•..•}           StructPattern
             a::B                               ExpressionPath
                    misc:•false                 StructPatternPropertyDestructured
                          false                 Literal
                                 ..             RestPattern
                                          {}    BlockExpression                                                                           */
		ref _x if false => {}                                                                                                             /*
        ref•_x•if•false•=>•{}    MatchExpressionCase
        ref•_x                   PatternVariableDeclaration
                  false          Literal
                           {}    BlockExpression                                                                                          */
        "b" => {}                                                                                                                         /*
        "b"•=>•{}    MatchExpressionCase
        "b"          Literal
               {}    BlockExpression                                                                                                      */
        "b" => {}                                                                                                                         /*
        "b"•=>•{}    MatchExpressionCase
        "b"          Literal
               {}    BlockExpression                                                                                                      */
        _ => {},                                                                                                                          /*
        _•=>•{}     MatchExpressionCase
        _           WildcardPattern
             {}     BlockExpression                                                                                                       */
		() if f == Foo { x: 42 } => {}                                                                                                    /*
        ()•if•f•==•Foo•{•x:•42•}•=>•{}    MatchExpressionCase
        ()                                TuplePattern
              f•==•Foo•{•x:•42•}          ComparisonExpression
                   Foo•{•x:•42•}          StructLiteral
                         x:•42            StructLiteralProperty
                            42            Literal
                                    {}    BlockExpression                                                                                 */
        _ => {}                                                                                                                           /*
        _•=>•{}    MatchExpressionCase
        _          WildcardPattern
             {}    BlockExpression                                                                                                        */
        0 => {}                                                                                                                           /*
        0•=>•{}    MatchExpressionCase
        0          Literal
             {}    BlockExpression                                                                                                        */
        a => {}                                                                                                                           /*
        a•=>•{}    MatchExpressionCase
             {}    BlockExpression                                                                                                        */
        a::X => {}                                                                                                                        /*
        a::X•=>•{}    MatchExpressionCase
        a::X          ExpressionPath
                {}    BlockExpression                                                                                                     */
        _ => {}                                                                                                                           /*
        _•=>•{}    MatchExpressionCase
        _          WildcardPattern
             {}    BlockExpression                                                                                                        */
        (a, ..,) => {}                                                                                                                    /*
        (a,•..,)•=>•{}    MatchExpressionCase
        (a,•..,)          TuplePattern
            ..            RestPattern
                    {}    BlockExpression                                                                                                 */
		0 .. 128 => {}                                                                                                                    /*
        0•..•128•=>•{}    MatchExpressionCase
        0•..•128          RangePattern
        0                 Literal
             128          Literal
                    {}    BlockExpression                                                                                                 */
        128 ..= 255 => {}                                                                                                                 /*
        128•..=•255•=>•{}    MatchExpressionCase
        128•..=•255          RangePattern
        128                  Literal
                255          Literal
                       {}    BlockExpression                                                                                              */
		128 ..= 255 if 1 => {}                                                                                                            /*
        128•..=•255•if•1•=>•{}    MatchExpressionCase
        128•..=•255               RangePattern
        128                       Literal
                255               Literal
                       1          Literal
                            {}    BlockExpression                                                                                         */
		(Some(_), None) | (None, Some(_)) => {}                                                                                           /*
        (Some(_),•None)•|•(None,•Some(_))•=>•{}    MatchExpressionCase
        (Some(_),•None)•|•(None,•Some(_))          UnionPattern
        (Some(_),•None)                            TuplePattern
         Some(_)                                   TuplePattern
              _                                    WildcardPattern
                          (None,•Some(_))          TuplePattern
                                 Some(_)           TuplePattern
                                      _            WildcardPattern
                                             {}    BlockExpression                                                                        */
		S::<{a()}> => {}                                                                                                                  /*
        S::<{a()}>•=>•{}    MatchExpressionCase
        S::<{a()}>          ExpressionTypeCast
            {a()}           BlockExpression
             a()            ExpressionStatement, CallExpression
                      {}    BlockExpression                                                                                               */
		((A, _), _) => {}                                                                                                                 /*
        ((A,•_),•_)•=>•{}    MatchExpressionCase
        ((A,•_),•_)          TuplePattern
         (A,•_)              TuplePattern
             _               WildcardPattern
                 _           WildcardPattern
                       {}    BlockExpression                                                                                              */
		[..] => {}                                                                                                                        /*
        [..]•=>•{}    MatchExpressionCase
        [..]          ArrayPattern
         ..           RestPattern
                {}    BlockExpression                                                                                                     */
        &[] => {}                                                                                                                         /*
        &[]•=>•{}    MatchExpressionCase
        &[]          ReferencePattern
         []          ArrayPattern
               {}    BlockExpression                                                                                                      */
        &[1..=255] => {}                                                                                                                  /*
        &[1..=255]•=>•{}    MatchExpressionCase
        &[1..=255]          ReferencePattern
         [1..=255]          ArrayPattern
          1..=255           RangePattern
          1                 Literal
              255           Literal
                      {}    BlockExpression                                                                                               */
        C0 => {}                                                                                                                          /*
        C0•=>•{}    MatchExpressionCase
              {}    BlockExpression                                                                                                       */
		T::A {} => {}                                                                                                                     /*
        T::A•{}•=>•{}    MatchExpressionCase
        T::A•{}          StructPattern
        T::A             ExpressionPath
                   {}    BlockExpression                                                                                                  */
        &[_, _, ..] => {}                                                                                                                 /*
        &[_,•_,•..]•=>•{}    MatchExpressionCase
        &[_,•_,•..]          ReferencePattern
         [_,•_,•..]          ArrayPattern
          _                  WildcardPattern
             _               WildcardPattern
                ..           RestPattern
                       {}    BlockExpression                                                                                              */
		[Some(..), None, ref tail @ ..] => {}                                                                                             /*
        [Some(..),•None,•ref•tail•@•..]•=>•{}    MatchExpressionCase
        [Some(..),•None,•ref•tail•@•..]          ArrayPattern
         Some(..)                                TuplePattern
              ..                                 RestPattern
                         ref•tail•@•..           PatternVariableDeclaration
                                    ..           RestPattern
                                           {}    BlockExpression                                                                          */
		[Some(..), Some(..), ref tail @ ..] => {}                                                                                         /*
        [Some(..),•Some(..),•ref•tail•@•..]•=>•{}    MatchExpressionCase
        [Some(..),•Some(..),•ref•tail•@•..]          ArrayPattern
         Some(..)                                    TuplePattern
              ..                                     RestPattern
                   Some(..)                          TuplePattern
                        ..                           RestPattern
                             ref•tail•@•..           PatternVariableDeclaration
                                        ..           RestPattern
                                               {}    BlockExpression                                                                      */
		[None, None, ref tail @ ..] => {}                                                                                                 /*
        [None,•None,•ref•tail•@•..]•=>•{}    MatchExpressionCase
        [None,•None,•ref•tail•@•..]          ArrayPattern
                     ref•tail•@•..           PatternVariableDeclaration
                                ..           RestPattern
                                       {}    BlockExpression                                                                              */
		[None, Some(..), ref tail @ ..] => {}                                                                                             /*
        [None,•Some(..),•ref•tail•@•..]•=>•{}    MatchExpressionCase
        [None,•Some(..),•ref•tail•@•..]          ArrayPattern
               Some(..)                          TuplePattern
                    ..                           RestPattern
                         ref•tail•@•..           PatternVariableDeclaration
                                    ..           RestPattern
                                           {}    BlockExpression                                                                          */
		[_, _, ref tail @ .., _] => {}                                                                                                    /*
        [_,•_,•ref•tail•@•..,•_]•=>•{}    MatchExpressionCase
        [_,•_,•ref•tail•@•..,•_]          ArrayPattern
         _                                WildcardPattern
            _                             WildcardPattern
               ref•tail•@•..              PatternVariableDeclaration
                          ..              RestPattern
                              _           WildcardPattern
                                    {}    BlockExpression                                                                                 */
		(&"foo", &_) => {}                                                                                                                /*
        (&"foo",•&_)•=>•{}    MatchExpressionCase
        (&"foo",•&_)          TuplePattern
         &"foo"               ReferencePattern
          "foo"               Literal
                 &_           ReferencePattern
                  _           WildcardPattern
                        {}    BlockExpression                                                                                             */
		(&&_, &_) => {}                                                                                                                   /*
        (&&_,•&_)•=>•{}    MatchExpressionCase
        (&&_,•&_)          TuplePattern
         &&_               ReferencePattern
          &_               ReferencePattern
           _               WildcardPattern
              &_           ReferencePattern
               _           WildcardPattern
                     {}    BlockExpression                                                                                                */
		Foo{foo: true, bar: Some(_), ..} => {}                                                                                            /*
        Foo{foo:•true,•bar:•Some(_),•..}•=>•{}    MatchExpressionCase
        Foo{foo:•true,•bar:•Some(_),•..}          StructPattern
            foo:•true                             StructPatternPropertyDestructured
                 true                             Literal
                       bar:•Some(_)               StructPatternPropertyDestructured
                            Some(_)               TuplePattern
                                 _                WildcardPattern
                                     ..           RestPattern
                                            {}    BlockExpression                                                                         */
		Foo{foo: false, bar: None, ..} => {}                                                                                              /*
        Foo{foo:•false,•bar:•None,•..}•=>•{}    MatchExpressionCase
        Foo{foo:•false,•bar:•None,•..}          StructPattern
            foo:•false                          StructPatternPropertyDestructured
                 false                          Literal
                        bar:•None               StructPatternPropertyDestructured
                                   ..           RestPattern
                                          {}    BlockExpression                                                                           */
		Foo{foo: true, bar: None, ..} => {}                                                                                               /*
        Foo{foo:•true,•bar:•None,•..}•=>•{}    MatchExpressionCase
        Foo{foo:•true,•bar:•None,•..}          StructPattern
            foo:•true                          StructPatternPropertyDestructured
                 true                          Literal
                       bar:•None               StructPatternPropertyDestructured
                                  ..           RestPattern
                                         {}    BlockExpression                                                                            */
		Foo{foo: false, bar: Some(_), ..} => {}                                                                                           /*
        Foo{foo:•false,•bar:•Some(_),•..}•=>•{}    MatchExpressionCase
        Foo{foo:•false,•bar:•Some(_),•..}          StructPattern
            foo:•false                             StructPatternPropertyDestructured
                 false                             Literal
                        bar:•Some(_)               StructPatternPropertyDestructured
                             Some(_)               TuplePattern
                                  _                WildcardPattern
                                      ..           RestPattern
                                             {}    BlockExpression                                                                        */
		(Some(&[]), Ok(&[])) => {}                                                                                                        /*
        (Some(&[]),•Ok(&[]))•=>•{}    MatchExpressionCase
        (Some(&[]),•Ok(&[]))          TuplePattern
         Some(&[])                    TuplePattern
              &[]                     ReferencePattern
               []                     ArrayPattern
                    Ok(&[])           TuplePattern
                       &[]            ReferencePattern
                        []            ArrayPattern
                                {}    BlockExpression                                                                                     */
		(Some(&[_, ..]), Ok(_)) | (Some(&[_, ..]), Err(())) => {},                                                                        /*
        (Some(&[_,•..]),•Ok(_))•|•(Some(&[_,•..]),•Err(()))•=>•{}    MatchExpressionCase
        (Some(&[_,•..]),•Ok(_))•|•(Some(&[_,•..]),•Err(()))          UnionPattern
        (Some(&[_,•..]),•Ok(_))                                      TuplePattern
         Some(&[_,•..])                                              TuplePattern
              &[_,•..]                                               ReferencePattern
               [_,•..]                                               ArrayPattern
                _                                                    WildcardPattern
                   ..                                                RestPattern
                         Ok(_)                                       TuplePattern
                            _                                        WildcardPattern
                                  (Some(&[_,•..]),•Err(()))          TuplePattern
                                   Some(&[_,•..])                    TuplePattern
                                        &[_,•..]                     ReferencePattern
                                         [_,•..]                     ArrayPattern
                                          _                          WildcardPattern
                                             ..                      RestPattern
                                                   Err(())           TuplePattern
                                                       ()            TuplePattern
                                                               {}    BlockExpression                                                      */
		(None, Ok(&[])) | (None, Err(())) | (None, Ok(&[_])) => {}                                                                        /*
        (None,•Ok(&[]))•|•(None,•Err(()))•|•(None,•Ok(&[_]))•=>•{}    MatchExpressionCase
        (None,•Ok(&[]))•|•(None,•Err(()))•|•(None,•Ok(&[_]))          UnionPattern
        (None,•Ok(&[]))                                               TuplePattern
               Ok(&[])                                                TuplePattern
                  &[]                                                 ReferencePattern
                   []                                                 ArrayPattern
                          (None,•Err(()))                             TuplePattern
                                 Err(())                              TuplePattern
                                     ()                               TuplePattern
                                            (None,•Ok(&[_]))          TuplePattern
                                                   Ok(&[_])           TuplePattern
                                                      &[_]            ReferencePattern
                                                       [_]            ArrayPattern
                                                        _             WildcardPattern
                                                                {}    BlockExpression                                                     */
		(None, Ok(&[_, _, ..])) => {}                                                                                                     /*
        (None,•Ok(&[_,•_,•..]))•=>•{}    MatchExpressionCase
        (None,•Ok(&[_,•_,•..]))          TuplePattern
               Ok(&[_,•_,•..])           TuplePattern
                  &[_,•_,•..]            ReferencePattern
                   [_,•_,•..]            ArrayPattern
                    _                    WildcardPattern
                       _                 WildcardPattern
                          ..             RestPattern
                                   {}    BlockExpression                                                                                  */
		(T::T1(()), V::V1(i)) => {}                                                                                                       /*
        (T::T1(()),•V::V1(i))•=>•{}    MatchExpressionCase
        (T::T1(()),•V::V1(i))          TuplePattern
         T::T1(())                     TuplePattern
         T::T1                         ExpressionPath
               ()                      TuplePattern
                    V::V1(i)           TuplePattern
                    V::V1              ExpressionPath
                                 {}    BlockExpression                                                                                    */
        (T::T2(()), V::V2(b)) => {}                                                                                                       /*
        (T::T2(()),•V::V2(b))•=>•{}    MatchExpressionCase
        (T::T2(()),•V::V2(b))          TuplePattern
         T::T2(())                     TuplePattern
         T::T2                         ExpressionPath
               ()                      TuplePattern
                    V::V2(b)           TuplePattern
                    V::V2              ExpressionPath
                                 {}    BlockExpression                                                                                    */
		Foo::Bar { bar: Bar::A, .. } => {}                                                                                                /*
        Foo::Bar•{•bar:•Bar::A,•..•}•=>•{}    MatchExpressionCase
        Foo::Bar•{•bar:•Bar::A,•..•}          StructPattern
        Foo::Bar                              ExpressionPath
                   bar:•Bar::A                StructPatternPropertyDestructured
                        Bar::A                ExpressionPath
                                ..            RestPattern
                                        {}    BlockExpression                                                                             */
		::A::B(3) => {}                                                                                                                   /*
        ::A::B(3)•=>•{}    MatchExpressionCase
        ::A::B(3)          TuplePattern
        ::A::B             ExpressionPath
        ::A                ExpressionPath
               3           Literal
                     {}    BlockExpression                                                                                                */
		::A::B(_) if false => {}                                                                                                          /*
        ::A::B(_)•if•false•=>•{}    MatchExpressionCase
        ::A::B(_)                   TuplePattern
        ::A::B                      ExpressionPath
        ::A                         ExpressionPath
               _                    WildcardPattern
                     false          Literal
                              {}    BlockExpression                                                                                       */
		::A::B(..) if false => {}                                                                                                         /*
        ::A::B(..)•if•false•=>•{}    MatchExpressionCase
        ::A::B(..)                   TuplePattern
        ::A::B                       ExpressionPath
        ::A                          ExpressionPath
               ..                    RestPattern
                      false          Literal
                               {}    BlockExpression                                                                                      */
		::A::B(_n) => {}                                                                                                                  /*
        ::A::B(_n)•=>•{}    MatchExpressionCase
        ::A::B(_n)          TuplePattern
        ::A::B              ExpressionPath
        ::A                 ExpressionPath
                      {}    BlockExpression                                                                                               */
		::A::B => {}                                                                                                                      /*
        ::A::B•=>•{}    MatchExpressionCase
        ::A::B          ExpressionPath
        ::A             ExpressionPath
                  {}    BlockExpression                                                                                                   */
		::A::B(::A::B) => {}                                                                                                              /*
        ::A::B(::A::B)•=>•{}    MatchExpressionCase
        ::A::B(::A::B)          TuplePattern
        ::A::B                  ExpressionPath
        ::A                     ExpressionPath
               ::A::B           ExpressionPath
               ::A              ExpressionPath
                          {}    BlockExpression                                                                                           */
		::A::B(::A::B(_)) => {}                                                                                                           /*
        ::A::B(::A::B(_))•=>•{}    MatchExpressionCase
        ::A::B(::A::B(_))          TuplePattern
        ::A::B                     ExpressionPath
        ::A                        ExpressionPath
               ::A::B(_)           TuplePattern
               ::A::B              ExpressionPath
               ::A                 ExpressionPath
                      _            WildcardPattern
                             {}    BlockExpression                                                                                        */
		::A::B(::A::B, ::A::B(_)) => {}                                                                                                   /*
        ::A::B(::A::B,•::A::B(_))•=>•{}    MatchExpressionCase
        ::A::B(::A::B,•::A::B(_))          TuplePattern
        ::A::B                             ExpressionPath
        ::A                                ExpressionPath
               ::A::B                      ExpressionPath
               ::A                         ExpressionPath
                       ::A::B(_)           TuplePattern
                       ::A::B              ExpressionPath
                       ::A                 ExpressionPath
                              _            WildcardPattern
                                     {}    BlockExpression                                                                                */
		::A::B(::A::B(..), ::A::B) => {}                                                                                                  /*
        ::A::B(::A::B(..),•::A::B)•=>•{}    MatchExpressionCase
        ::A::B(::A::B(..),•::A::B)          TuplePattern
        ::A::B                              ExpressionPath
        ::A                                 ExpressionPath
               ::A::B(..)                   TuplePattern
               ::A::B                       ExpressionPath
               ::A                          ExpressionPath
                      ..                    RestPattern
                           ::A::B           ExpressionPath
                           ::A              ExpressionPath
                                      {}    BlockExpression                                                                               */
		::A::B(..) => {}                                                                                                                  /*
        ::A::B(..)•=>•{}    MatchExpressionCase
        ::A::B(..)          TuplePattern
        ::A::B              ExpressionPath
        ::A                 ExpressionPath
               ..           RestPattern
                      {}    BlockExpression                                                                                               */
		A::<A<u8>> { x: A(10, 11) } => {}                                                                                                 /*
        A::<A<u8>>•{•x:•A(10,•11)•}•=>•{}    MatchExpressionCase
        A::<A<u8>>•{•x:•A(10,•11)•}          StructPattern
        A::<A<u8>>                           ExpressionTypeCast
            A<u8>                            TypeCall
                     x:•A(10,•11)            StructPatternPropertyDestructured
                        A(10,•11)            TuplePattern
                          10                 Literal
                              11             Literal
                                       {}    BlockExpression                                                                              */
		::B::<<A<_> as C>::U> { x: A::<u8>(11, 16) } => {}                                                                                /*
        ::B::<<A<_>•as•C>::U>•{•x:•A::<u8>(11,•16)•}•=>•{}    MatchExpressionCase
        ::B::<<A<_>•as•C>::U>•{•x:•A::<u8>(11,•16)•}          StructPattern
        ::B::<<A<_>•as•C>::U>                                 ExpressionTypeCast
        ::B                                                   ExpressionPath
              <A<_>•as•C>::U                                  TypePath
              <A<_>•as•C>                                     ExpressionTypeSelector
               A<_>                                           TypeCall
                 _                                            TypeInferred
                                x:•A::<u8>(11,•16)            StructPatternPropertyDestructured
                                   A::<u8>(11,•16)            TuplePattern
                                   A::<u8>                    ExpressionTypeCast
                                           11                 Literal
                                               16             Literal
                                                        {}    BlockExpression                                                             */
		isize::MIN..5 | 5..=isize::MAX => {}                                                                                              /*
        isize::MIN..5•|•5..=isize::MAX•=>•{}    MatchExpressionCase
        isize::MIN..5•|•5..=isize::MAX          UnionPattern
        isize::MIN..5                           RangePattern
        isize::MIN                              ExpressionPath
                    5                           Literal
                        5..=isize::MAX          RangePattern
                        5                       Literal
                            isize::MAX          ExpressionPath
                                          {}    BlockExpression                                                                           */
		0..5 | 5..=usize::MAX => {}                                                                                                       /*
        0..5•|•5..=usize::MAX•=>•{}    MatchExpressionCase
        0..5•|•5..=usize::MAX          UnionPattern
        0..5                           RangePattern
        0                              Literal
           5                           Literal
               5..=usize::MAX          RangePattern
               5                       Literal
                   usize::MAX          ExpressionPath
                                 {}    BlockExpression                                                                                    */
		(0..5, true) | (5..=usize::MAX, true) | (0..=usize::MAX, false) => {}                                                             /*
        (0..5,•true)•|•(5..=usize::MAX,•true)•|•(0..=usize::MAX,•false)•=>•{}    MatchExpressionCase
        (0..5,•true)•|•(5..=usize::MAX,•true)•|•(0..=usize::MAX,•false)          UnionPattern
        (0..5,•true)                                                             TuplePattern
         0..5                                                                    RangePattern
         0                                                                       Literal
            5                                                                    Literal
               true                                                              Literal
                       (5..=usize::MAX,•true)                                    TuplePattern
                        5..=usize::MAX                                           RangePattern
                        5                                                        Literal
                            usize::MAX                                           ExpressionPath
                                        true                                     Literal
                                                (0..=usize::MAX,•false)          TuplePattern
                                                 0..=usize::MAX                  RangePattern
                                                 0                               Literal
                                                     usize::MAX                  ExpressionPath
                                                                 false           Literal
                                                                           {}    BlockExpression                                          */
		[Ok(box ref a), ref xs @ .., Err(box b), Err(box ref mut c)] => {}                                                                /*
        [Ok(box•ref•a),•ref•xs•@•..,•Err(box•b),•Err(box•ref•mut•c)]•=>•{}    MatchExpressionCase
        [Ok(box•ref•a),•ref•xs•@•..,•Err(box•b),•Err(box•ref•mut•c)]          ArrayPattern
         Ok(box•ref•a)                                                        TuplePattern
            box•ref•a                                                         BoxPattern
                ref•a                                                         PatternVariableDeclaration
                        ref•xs•@•..                                           PatternVariableDeclaration
                                 ..                                           RestPattern
                                     Err(box•b)                               TuplePattern
                                         box•b                                BoxPattern
                                                 Err(box•ref•mut•c)           TuplePattern
                                                     box•ref•mut•c            BoxPattern
                                                         ref•mut•c            PatternVariableDeclaration
                                                                        {}    BlockExpression                                             */
		[Ok(box a), ref xs @ .., Err(box ref b), Err(box ref c)] => {}                                                                    /*
        [Ok(box•a),•ref•xs•@•..,•Err(box•ref•b),•Err(box•ref•c)]•=>•{}    MatchExpressionCase
        [Ok(box•a),•ref•xs•@•..,•Err(box•ref•b),•Err(box•ref•c)]          ArrayPattern
         Ok(box•a)                                                        TuplePattern
            box•a                                                         BoxPattern
                    ref•xs•@•..                                           PatternVariableDeclaration
                             ..                                           RestPattern
                                 Err(box•ref•b)                           TuplePattern
                                     box•ref•b                            BoxPattern
                                         ref•b                            PatternVariableDeclaration
                                                 Err(box•ref•c)           TuplePattern
                                                     box•ref•c            BoxPattern
                                                         ref•c            PatternVariableDeclaration
                                                                    {}    BlockExpression                                                 */
		box a => {Foo(box 1)}                                                                                                             /*
        box•a•=>•{Foo(box•1)}    MatchExpressionCase
        box•a                    BoxPattern
                 {Foo(box•1)}    BlockExpression
                  Foo(box•1)     ExpressionStatement, CallExpression
                      box•1      BoxExpression
                          1      Literal                                                                                                  */
		box [Ok(a), ref xs @ .., Err(ref b)] => {}                                                                                        /*
        box•[Ok(a),•ref•xs•@•..,•Err(ref•b)]•=>•{}    MatchExpressionCase
        box•[Ok(a),•ref•xs•@•..,•Err(ref•b)]          BoxPattern
            [Ok(a),•ref•xs•@•..,•Err(ref•b)]          ArrayPattern
             Ok(a)                                    TuplePattern
                    ref•xs•@•..                       PatternVariableDeclaration
                             ..                       RestPattern
                                 Err(ref•b)           TuplePattern
                                     ref•b            PatternVariableDeclaration
                                                {}    BlockExpression                                                                     */
		ref a @ box b => {}                                                                                                               /*
        ref•a•@•box•b•=>•{}    MatchExpressionCase
        ref•a•@•box•b          PatternVariableDeclaration
                box•b          BoxPattern
                         {}    BlockExpression                                                                                            */
		ref a @ box ref b => {}                                                                                                           /*
        ref•a•@•box•ref•b•=>•{}    MatchExpressionCase
        ref•a•@•box•ref•b          PatternVariableDeclaration
                box•ref•b          BoxPattern
                    ref•b          PatternVariableDeclaration
                             {}    BlockExpression                                                                                        */
		Ok(ref a @ b) | Err(b @ ref a) => {}                                                                                              /*
        Ok(ref•a•@•b)•|•Err(b•@•ref•a)•=>•{}    MatchExpressionCase
        Ok(ref•a•@•b)•|•Err(b•@•ref•a)          UnionPattern
        Ok(ref•a•@•b)                           TuplePattern
           ref•a•@•b                            PatternVariableDeclaration
                        Err(b•@•ref•a)          TuplePattern
                            b•@•ref•a           PatternVariableDeclaration
                                ref•a           PatternVariableDeclaration
                                          {}    BlockExpression                                                                           */
		ref a @ Ok(ref b) | ref a @ Err(ref b) => {}                                                                                      /*
        ref•a•@•Ok(ref•b)•|•ref•a•@•Err(ref•b)•=>•{}    MatchExpressionCase
        ref•a•@•Ok(ref•b)•|•ref•a•@•Err(ref•b)          UnionPattern
        ref•a•@•Ok(ref•b)                               PatternVariableDeclaration
                Ok(ref•b)                               TuplePattern
                   ref•b                                PatternVariableDeclaration
                            ref•a•@•Err(ref•b)          PatternVariableDeclaration
                                    Err(ref•b)          TuplePattern
                                        ref•b           PatternVariableDeclaration
                                                  {}    BlockExpression                                                                   */
		ref a @ Ok(ref mut b) | ref a @ Err(ref mut b) if { *b = U; false } => {}                                                         /*
        ref•a•@•Ok(ref•mut•b)•|•ref•a•@•Err(ref•mut•b)•if•{•*b•=•U;•false•}•=>•{}    MatchExpressionCase
        ref•a•@•Ok(ref•mut•b)•|•ref•a•@•Err(ref•mut•b)                               UnionPattern
        ref•a•@•Ok(ref•mut•b)                                                        PatternVariableDeclaration
                Ok(ref•mut•b)                                                        TuplePattern
                   ref•mut•b                                                         PatternVariableDeclaration
                                ref•a•@•Err(ref•mut•b)                               PatternVariableDeclaration
                                        Err(ref•mut•b)                               TuplePattern
                                            ref•mut•b                                PatternVariableDeclaration
                                                          {•*b•=•U;•false•}          BlockExpression
                                                            *b•=•U;                  ExpressionStatement
                                                            *b•=•U                   ReassignmentExpression
                                                            *b                       DereferenceExpression
                                                                    false            ExpressionStatement, Literal
                                                                               {}    BlockExpression                                      */
		ref mut a @ Ok(ref b) | ref mut a @ Err(ref b) if { *a = Err(U); false } => {}                                                    /*
        ref•mut•a•@•Ok(ref•b)•|•ref•mut•a•@•Err(ref•b)•if•{•*a•=•Err(U);•false•}•=>•{}    MatchExpressionCase
        ref•mut•a•@•Ok(ref•b)•|•ref•mut•a•@•Err(ref•b)                                    UnionPattern
        ref•mut•a•@•Ok(ref•b)                                                             PatternVariableDeclaration
                    Ok(ref•b)                                                             TuplePattern
                       ref•b                                                              PatternVariableDeclaration
                                ref•mut•a•@•Err(ref•b)                                    PatternVariableDeclaration
                                            Err(ref•b)                                    TuplePattern
                                                ref•b                                     PatternVariableDeclaration
                                                          {•*a•=•Err(U);•false•}          BlockExpression
                                                            *a•=•Err(U);                  ExpressionStatement
                                                            *a•=•Err(U)                   ReassignmentExpression
                                                            *a                            DereferenceExpression
                                                                 Err(U)                   CallExpression
                                                                         false            ExpressionStatement, Literal
                                                                                    {}    BlockExpression                                 */
		a @ Some((mut b @ ref mut c, d @ ref e)) => {}                                                                                    /*
        a•@•Some((mut•b•@•ref•mut•c,•d•@•ref•e))•=>•{}    MatchExpressionCase
        a•@•Some((mut•b•@•ref•mut•c,•d•@•ref•e))          PatternVariableDeclaration
            Some((mut•b•@•ref•mut•c,•d•@•ref•e))          TuplePattern
                 (mut•b•@•ref•mut•c,•d•@•ref•e)           TuplePattern
                  mut•b•@•ref•mut•c                       PatternVariableDeclaration
                          ref•mut•c                       PatternVariableDeclaration
                                     d•@•ref•e            PatternVariableDeclaration
                                         ref•e            PatternVariableDeclaration
                                                    {}    BlockExpression                                                                 */
		mut a @ Some([ref b, ref mut c]) => {}                                                                                            /*
        mut•a•@•Some([ref•b,•ref•mut•c])•=>•{}    MatchExpressionCase
        mut•a•@•Some([ref•b,•ref•mut•c])          PatternVariableDeclaration
                Some([ref•b,•ref•mut•c])          TuplePattern
                     [ref•b,•ref•mut•c]           ArrayPattern
                      ref•b                       PatternVariableDeclaration
                             ref•mut•c            PatternVariableDeclaration
                                            {}    BlockExpression                                                                         */
		ref mut a @ Some([b, mut c]) => {}                                                                                                /*
        ref•mut•a•@•Some([b,•mut•c])•=>•{}    MatchExpressionCase
        ref•mut•a•@•Some([b,•mut•c])          PatternVariableDeclaration
                    Some([b,•mut•c])          TuplePattern
                         [b,•mut•c]           ArrayPattern
                             mut•c            PatternVariableDeclaration
                                        {}    BlockExpression                                                                             */
		ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {}                                                                      /*
        ref•mut•a•@•Ok(ref•mut•b)•|•ref•mut•a•@•Err(ref•mut•b)•=>•{}    MatchExpressionCase
        ref•mut•a•@•Ok(ref•mut•b)•|•ref•mut•a•@•Err(ref•mut•b)          UnionPattern
        ref•mut•a•@•Ok(ref•mut•b)                                       PatternVariableDeclaration
                    Ok(ref•mut•b)                                       TuplePattern
                       ref•mut•b                                        PatternVariableDeclaration
                                    ref•mut•a•@•Err(ref•mut•b)          PatternVariableDeclaration
                                                Err(ref•mut•b)          TuplePattern
                                                    ref•mut•b           PatternVariableDeclaration
                                                                  {}    BlockExpression                                                   */
		ref bar @ Some(box n) if n > 0 => {}                                                                                              /*
        ref•bar•@•Some(box•n)•if•n•>•0•=>•{}    MatchExpressionCase
        ref•bar•@•Some(box•n)                   PatternVariableDeclaration
                  Some(box•n)                   TuplePattern
                       box•n                    BoxPattern
                                 n•>•0          ComparisonExpression
                                     0          Literal
                                          {}    BlockExpression                                                                           */
		Some(ref bar @ box n) if n < 0 => {}                                                                                              /*
        Some(ref•bar•@•box•n)•if•n•<•0•=>•{}    MatchExpressionCase
        Some(ref•bar•@•box•n)                   TuplePattern
             ref•bar•@•box•n                    PatternVariableDeclaration
                       box•n                    BoxPattern
                                 n•<•0          ComparisonExpression
                                     0          Literal
                                          {}    BlockExpression                                                                           */
		ref x @ A { ref a, b: 20 } => {}                                                                                                  /*
        ref•x•@•A•{•ref•a,•b:•20•}•=>•{}    MatchExpressionCase
        ref•x•@•A•{•ref•a,•b:•20•}          PatternVariableDeclaration
                A•{•ref•a,•b:•20•}          StructPattern
                    ref•a                   StructPatternPropertyShorthand
                           b:•20            StructPatternPropertyDestructured
                              20            Literal
                                      {}    BlockExpression                                                                               */
        (a,_) | (_,a) if a > 10 => 0,                                                                                                     /*
        (a,_)•|•(_,a)•if•a•>•10•=>•0     MatchExpressionCase
        (a,_)•|•(_,a)                    UnionPattern
        (a,_)                            TuplePattern
           _                             WildcardPattern
                (_,a)                    TuplePattern
                 _                       WildcardPattern
                         a•>•10          ComparisonExpression
                             10          Literal
                                   0     Literal                                                                                          */
        e @ &(1..=2) | e @ &(3..=4) => {}                                                                                                 /*
        e•@•&(1..=2)•|•e•@•&(3..=4)•=>•{}    MatchExpressionCase
        e•@•&(1..=2)•|•e•@•&(3..=4)          UnionPattern
        e•@•&(1..=2)                         PatternVariableDeclaration
            &(1..=2)                         ReferencePattern
              1..=2                          RangePattern
              1                              Literal
                  2                          Literal
                       e•@•&(3..=4)          PatternVariableDeclaration
                           &(3..=4)          ReferencePattern
                             3..=4           RangePattern
                             3               Literal
                                 4           Literal
                                       {}    BlockExpression                                                                              */
        0 | &1 => {}                                                                                                                      /*
        0•|•&1•=>•{}    MatchExpressionCase
        0•|•&1          UnionPattern
        0               Literal
            &1          ReferencePattern
             1          Literal
                  {}    BlockExpression                                                                                                   */
        Ok(x) | Err(x) => 0,                                                                                                              /*
        Ok(x)•|•Err(x)•=>•0     MatchExpressionCase
        Ok(x)•|•Err(x)          UnionPattern
        Ok(x)                   TuplePattern
                Err(x)          TuplePattern
                          0     Literal                                                                                                   */
        &(Ok(x) | Err(x)) => 0,                                                                                                           /*
        &(Ok(x)•|•Err(x))•=>•0     MatchExpressionCase
        &(Ok(x)•|•Err(x))          ReferencePattern
          Ok(x)•|•Err(x)           UnionPattern
          Ok(x)                    TuplePattern
                  Err(x)           TuplePattern
                             0     Literal                                                                                                */
        Ok(mut x) | &Err(mut x) => 0,                                                                                                     /*
        Ok(mut•x)•|•&Err(mut•x)•=>•0     MatchExpressionCase
        Ok(mut•x)•|•&Err(mut•x)          UnionPattern
        Ok(mut•x)                        TuplePattern
           mut•x                         PatternVariableDeclaration
                    &Err(mut•x)          ReferencePattern
                     Err(mut•x)          TuplePattern
                         mut•x           PatternVariableDeclaration
                                   0     Literal                                                                                          */
        Some((a, _) | (_, a)) if a > 10 => 0,                                                                                             /*
        Some((a,•_)•|•(_,•a))•if•a•>•10•=>•0     MatchExpressionCase
        Some((a,•_)•|•(_,•a))                    TuplePattern
             (a,•_)•|•(_,•a)                     UnionPattern
             (a,•_)                              TuplePattern
                 _                               WildcardPattern
                      (_,•a)                     TuplePattern
                       _                         WildcardPattern
                                 a•>•10          ComparisonExpression
                                     10          Literal
                                           0     Literal                                                                                  */
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
		Some(ref bar @ box Test::Baz | ref bar @ box Test::Qux) => 0,                                                                     /*
        Some(ref•bar•@•box•Test::Baz•|•ref•bar•@•box•Test::Qux)•=>•0    MatchExpressionCase
        Some(ref•bar•@•box•Test::Baz•|•ref•bar•@•box•Test::Qux)         TuplePattern
             ref•bar•@•box•Test::Baz•|•ref•bar•@•box•Test::Qux          UnionPattern
             ref•bar•@•box•Test::Baz                                    PatternVariableDeclaration
                       box•Test::Baz                                    BoxPattern
                           Test::Baz                                    ExpressionPath
                                       ref•bar•@•box•Test::Qux          PatternVariableDeclaration
                                                 box•Test::Qux          BoxPattern
                                                     Test::Qux          ExpressionPath
                                                                   0    Literal                                                           */
		Some(x) if let Foo::Qux(y) = qux(x) => 0,                                                                                         /*
        Some(x)•if•let•Foo::Qux(y)•=•qux(x)•=>•0    MatchExpressionCase
        Some(x)                                     TuplePattern
                   let•Foo::Qux(y)•=•qux(x)         LetScrutinee
                       Foo::Qux(y)                  TuplePattern
                       Foo::Qux                     ExpressionPath
                                     qux(x)         CallExpression
                                               0    Literal                                                                               */
		[bar @ .., n] if n == &5 => {}                                                                                                    /*
        [bar•@•..,•n]•if•n•==•&5•=>•{}    MatchExpressionCase
        [bar•@•..,•n]                     ArrayPattern
         bar•@•..                         PatternVariableDeclaration
               ..                         RestPattern
                         n•==•&5          ComparisonExpression
                              &5          ReferenceExpression
                               5          Literal
                                    {}    BlockExpression                                                                                 */
		&A { a: 2 } if a.b().c() => {}                                                                                                    /*
        &A•{•a:•2•}•if•a.b().c()•=>•{}    MatchExpressionCase
        &A•{•a:•2•}                       ReferencePattern
         A•{•a:•2•}                       StructPattern
             a:•2                         StructPatternPropertyDestructured
                2                         Literal
                       a.b().c()          CallExpression
                       a.b()              CallExpression
                                    {}    BlockExpression                                                                                 */
		A::B { a } => {}                                                                                                                  /*
        A::B•{•a•}•=>•{}    MatchExpressionCase
        A::B•{•a•}          StructPattern
        A::B                ExpressionPath
               a            StructPatternPropertyShorthand
                      {}    BlockExpression                                                                                               */
		&A::B { a } => {}                                                                                                                 /*
        &A::B•{•a•}•=>•{}    MatchExpressionCase
        &A::B•{•a•}          ReferencePattern
         A::B•{•a•}          StructPattern
         A::B                ExpressionPath
                a            StructPatternPropertyShorthand
                       {}    BlockExpression                                                                                              */
		box A::B { a } => {}                                                                                                              /*
        box•A::B•{•a•}•=>•{}    MatchExpressionCase
        box•A::B•{•a•}          BoxPattern
            A::B•{•a•}          StructPattern
            A::B                ExpressionPath
                   a            StructPatternPropertyShorthand
                          {}    BlockExpression                                                                                           */
		(A::B { a },) => {}                                                                                                               /*
        (A::B•{•a•},)•=>•{}    MatchExpressionCase
        (A::B•{•a•},)          TuplePattern
         A::B•{•a•}            StructPattern
         A::B                  ExpressionPath
                a              StructPatternPropertyShorthand
                         {}    BlockExpression                                                                                            */
		[A::B { a }] => {}                                                                                                                /*
        [A::B•{•a•}]•=>•{}    MatchExpressionCase
        [A::B•{•a•}]          ArrayPattern
         A::B•{•a•}           StructPattern
         A::B                 ExpressionPath
                a             StructPatternPropertyShorthand
                        {}    BlockExpression                                                                                             */
		C(A::B { a }, ()) => {}                                                                                                           /*
        C(A::B•{•a•},•())•=>•{}    MatchExpressionCase
        C(A::B•{•a•},•())          TuplePattern
          A::B•{•a•}               StructPattern
          A::B                     ExpressionPath
                 a                 StructPatternPropertyShorthand
                      ()           TuplePattern
                             {}    BlockExpression                                                                                        */
        ((0 | 1,) | (2 | 3,),) => {}                                                                                                      /*
        ((0•|•1,)•|•(2•|•3,),)•=>•{}    MatchExpressionCase
        ((0•|•1,)•|•(2•|•3,),)          TuplePattern
         (0•|•1,)•|•(2•|•3,)            UnionPattern
         (0•|•1,)                       TuplePattern
          0•|•1                         UnionPattern
          0                             Literal
              1                         Literal
                    (2•|•3,)            TuplePattern
                     2•|•3              UnionPattern
                     2                  Literal
                         3              Literal
                                  {}    BlockExpression                                                                                   */
        (Some(2..=255),) => {}                                                                                                            /*
        (Some(2..=255),)•=>•{}    MatchExpressionCase
        (Some(2..=255),)          TuplePattern
         Some(2..=255)            TuplePattern
              2..=255             RangePattern
              2                   Literal
                  255             Literal
                            {}    BlockExpression                                                                                         */
        (None | Some(0 | 1),) => {}                                                                                                       /*
        (None•|•Some(0•|•1),)•=>•{}    MatchExpressionCase
        (None•|•Some(0•|•1),)          TuplePattern
         None•|•Some(0•|•1)            UnionPattern
                Some(0•|•1)            TuplePattern
                     0•|•1             UnionPattern
                     0                 Literal
                         1             Literal
                                 {}    BlockExpression                                                                                    */
        (1 | 2,) => {}                                                                                                                    /*
        (1•|•2,)•=>•{}    MatchExpressionCase
        (1•|•2,)          TuplePattern
         1•|•2            UnionPattern
         1                Literal
             2            Literal
                    {}    BlockExpression                                                                                                 */
        (1 | 2, 3 | 4) => {}                                                                                                              /*
        (1•|•2,•3•|•4)•=>•{}    MatchExpressionCase
        (1•|•2,•3•|•4)          TuplePattern
         1•|•2                  UnionPattern
         1                      Literal
             2                  Literal
                3•|•4           UnionPattern
                3               Literal
                    4           Literal
                          {}    BlockExpression                                                                                           */
        ([] | [0 | 1..=255] | [_, ..],) => {}                                                                                             /*
        ([]•|•[0•|•1..=255]•|•[_,•..],)•=>•{}    MatchExpressionCase
        ([]•|•[0•|•1..=255]•|•[_,•..],)          TuplePattern
         []•|•[0•|•1..=255]•|•[_,•..]            UnionPattern
         []                                      ArrayPattern
              [0•|•1..=255]                      ArrayPattern
               0•|•1..=255                       UnionPattern
               0                                 Literal
                   1..=255                       RangePattern
                   1                             Literal
                       255                       Literal
                              [_,•..]            ArrayPattern
                               _                 WildcardPattern
                                  ..             RestPattern
                                           {}    BlockExpression                                                                          */
        ((0, 0) | (0, 1),) => {}                                                                                                          /*
        ((0,•0)•|•(0,•1),)•=>•{}    MatchExpressionCase
        ((0,•0)•|•(0,•1),)          TuplePattern
         (0,•0)•|•(0,•1)            UnionPattern
         (0,•0)                     TuplePattern
          0                         Literal
             0                      Literal
                  (0,•1)            TuplePattern
                   0                Literal
                      1             Literal
                              {}    BlockExpression                                                                                       */
        ((0, 0) | (1, 0),) => {}                                                                                                          /*
        ((0,•0)•|•(1,•0),)•=>•{}    MatchExpressionCase
        ((0,•0)•|•(1,•0),)          TuplePattern
         (0,•0)•|•(1,•0)            UnionPattern
         (0,•0)                     TuplePattern
          0                         Literal
             0                      Literal
                  (1,•0)            TuplePattern
                   1                Literal
                      0             Literal
                              {}    BlockExpression                                                                                       */
		Tri::A(Ok(mut x) | Err(mut x)) | Tri::B(&Ok(mut x) | Err(mut x)) | &Tri::C(Ok(mut x) | Err(mut x)) => 0,                          /*
        Tri::A(Ok(mut•x)•|•Err(mut•x))•|•Tri::B(&Ok(mut•x)•|•Err(mut•x))•|•&Tri::C(Ok(mut•x)•|•Err(mut•x))•=>•0    MatchExpressionCase
        Tri::A(Ok(mut•x)•|•Err(mut•x))•|•Tri::B(&Ok(mut•x)•|•Err(mut•x))•|•&Tri::C(Ok(mut•x)•|•Err(mut•x))         UnionPattern
        Tri::A(Ok(mut•x)•|•Err(mut•x))                                                                             TuplePattern
        Tri::A                                                                                                     ExpressionPath
               Ok(mut•x)•|•Err(mut•x)                                                                              UnionPattern
               Ok(mut•x)                                                                                           TuplePattern
                  mut•x                                                                                            PatternVariableDeclaration
                           Err(mut•x)                                                                              TuplePattern
                               mut•x                                                                               PatternVariableDeclaration
                                         Tri::B(&Ok(mut•x)•|•Err(mut•x))                                           TuplePattern
                                         Tri::B                                                                    ExpressionPath
                                                &Ok(mut•x)•|•Err(mut•x)                                            UnionPattern
                                                &Ok(mut•x)                                                         ReferencePattern
                                                 Ok(mut•x)                                                         TuplePattern
                                                    mut•x                                                          PatternVariableDeclaration
                                                             Err(mut•x)                                            TuplePattern
                                                                 mut•x                                             PatternVariableDeclaration
                                                                           &Tri::C(Ok(mut•x)•|•Err(mut•x))         ReferencePattern
                                                                            Tri::C(Ok(mut•x)•|•Err(mut•x))         TuplePattern
                                                                            Tri::C                                 ExpressionPath
                                                                                   Ok(mut•x)•|•Err(mut•x)          UnionPattern
                                                                                   Ok(mut•x)                       TuplePattern
                                                                                      mut•x                        PatternVariableDeclaration
                                                                                               Err(mut•x)          TuplePattern
                                                                                                   mut•x           PatternVariableDeclaration
                                                                                                              0    Literal                */
		Wrap(Ok(mut x) | &Err(mut x)) => 0,                                                                                               /*
        Wrap(Ok(mut•x)•|•&Err(mut•x))•=>•0    MatchExpressionCase
        Wrap(Ok(mut•x)•|•&Err(mut•x))         TuplePattern
             Ok(mut•x)•|•&Err(mut•x)          UnionPattern
             Ok(mut•x)                        TuplePattern
                mut•x                         PatternVariableDeclaration
                         &Err(mut•x)          ReferencePattern
                          Err(mut•x)          TuplePattern
                              mut•x           PatternVariableDeclaration
                                         0    Literal                                                                                     */
		Wrap(&(Ok(x) | Err(x))) => 0,                                                                                                     /*
        Wrap(&(Ok(x)•|•Err(x)))•=>•0    MatchExpressionCase
        Wrap(&(Ok(x)•|•Err(x)))         TuplePattern
             &(Ok(x)•|•Err(x))          ReferencePattern
               Ok(x)•|•Err(x)           UnionPattern
               Ok(x)                    TuplePattern
                       Err(x)           TuplePattern
                                   0    Literal                                                                                           */
		Wrap(Ok(x) | Err(x)) => 0,                                                                                                        /*
        Wrap(Ok(x)•|•Err(x))•=>•0    MatchExpressionCase
        Wrap(Ok(x)•|•Err(x))         TuplePattern
             Ok(x)•|•Err(x)          UnionPattern
             Ok(x)                   TuplePattern
                     Err(x)          TuplePattern
                                0    Literal                                                                                              */
		() if if if if 0 {0} else {0} {0} else {0} {0} else {0} => 0,                                                                     /*
        ()•if•if•if•if•0•{0}•else•{0}•{0}•else•{0}•{0}•else•{0}•=>•0    MatchExpressionCase
        ()                                                              TuplePattern
              if•if•if•0•{0}•else•{0}•{0}•else•{0}•{0}•else•{0}         IfBlockExpression
                 if•if•0•{0}•else•{0}•{0}•else•{0}                      IfBlockExpression
                    if•0•{0}•else•{0}                                   IfBlockExpression
                       0                                                Literal
                          0                                             ExpressionStatement, Literal
                                  {0}                                   BlockExpression
                                   0                                    ExpressionStatement, Literal
                                       0                                ExpressionStatement, Literal
                                               {0}                      BlockExpression
                                                0                       ExpressionStatement, Literal
                                                    0                   ExpressionStatement, Literal
                                                            {0}         BlockExpression
                                                             0          ExpressionStatement, Literal
                                                                   0    Literal                                                           */
		Add | Mul | And | Or | BitXor | BitAnd | BitOr | Eq | Ne => 0,                                                                    /*
        Add•|•Mul•|•And•|•Or•|•BitXor•|•BitAnd•|•BitOr•|•Eq•|•Ne•=>•0    MatchExpressionCase
        Add•|•Mul•|•And•|•Or•|•BitXor•|•BitAnd•|•BitOr•|•Eq•|•Ne         UnionPattern
                                                                    0    Literal                                                          */
        Sub | Div | Rem | Shl | Shr | Lt | Le | Ge | Gt => 0,                                                                             /*
        Sub•|•Div•|•Rem•|•Shl•|•Shr•|•Lt•|•Le•|•Ge•|•Gt•=>•0     MatchExpressionCase
        Sub•|•Div•|•Rem•|•Shl•|•Shr•|•Lt•|•Le•|•Ge•|•Gt          UnionPattern
                                                           0     Literal                                                                  */
		ThisIsA::ReallyLongPatternNameToHelpOverflowTheNextValueOntoTheNextLine | ThisIsA::SecondValueSeparatedByAPipe | ThisIsA::ThirdValueSeparatedByAPipe => 0,/*
        ThisIsA::ReallyLongPatternNameToHelpOverflowTheNextValueOntoTheNextLine•|•ThisIsA::SecondValueSeparatedByAPipe•|•ThisIsA::ThirdValueSeparatedByAPipe•=>•0    MatchExpressionCase
        ThisIsA::ReallyLongPatternNameToHelpOverflowTheNextValueOntoTheNextLine•|•ThisIsA::SecondValueSeparatedByAPipe•|•ThisIsA::ThirdValueSeparatedByAPipe         UnionPattern
        ThisIsA::ReallyLongPatternNameToHelpOverflowTheNextValueOntoTheNextLine                                                                                      ExpressionPath
                                                                                  ThisIsA::SecondValueSeparatedByAPipe                                               ExpressionPath
                                                                                                                         ThisIsA::ThirdValueSeparatedByAPipe         ExpressionPath
                                                                                                                                                                0    Literal*/
		MyEnum::Option1 if cfg!(target_os="windows") =>                                                                                   /*
        MyEnum::Option1•if•cfg!(target_os="windows")•=>↲    <MatchExpressionCase>
        MyEnum::Option1                                     ExpressionPath
                           cfg!(target_os="windows")        MacroInvocation
                                         =                  PunctuationToken
                                          "windows"         Literal                                                                       */
            #[cfg(target_os="windows")]{                                                                                                  /*
            #[cfg(target_os="windows")]      Attribute (dangling)
                 (target_os="windows")       DelimGroup
                           =                 PunctuationToken
                            "windows"        Literal
                                       {↲    <BlockExpression>                                                                            */
                1                                                                                                                         /*
                1    ExpressionStatement, Literal                                                                                         */
            }                                                                                                                             /*
••••••••••••}    </MatchExpressionCase>, </BlockExpression>                                                                               */
        MyEnum::Option1 if cfg!(target_os="windows") =>                                                                                   /*
        MyEnum::Option1•if•cfg!(target_os="windows")•=>↲    <MatchExpressionCase>
        MyEnum::Option1                                     ExpressionPath
                           cfg!(target_os="windows")        MacroInvocation
                                         =                  PunctuationToken
                                          "windows"         Literal                                                                       */
            #[cfg(target_os="windows")]                                                                                                   /*
            #[cfg(target_os="windows")]    Attribute (dangling)
                 (target_os="windows")     DelimGroup
                           =               PunctuationToken
                            "windows"      Literal                                                                                        */
                2,                                                                                                                        /*
••••••••••••••••2     </MatchExpressionCase>
                2     Literal                                                                                                             */
		Some(RegionResolutionError::SubSupConflict(                                                                                       /*
        Some(RegionResolutionError::SubSupConflict(↲    <MatchExpressionCase>, <TuplePattern>
             RegionResolutionError::SubSupConflict(↲    <TuplePattern>
             RegionResolutionError::SubSupConflict      ExpressionPath                                                                    */
                vid,
                _,                                                                                                                        /*
                _     WildcardPattern                                                                                                     */
                SubregionOrigin::Subtype(box TypeTrace { cause, values }),                                                                /*
                SubregionOrigin::Subtype(box•TypeTrace•{•cause,•values•})     TuplePattern
                SubregionOrigin::Subtype                                      ExpressionPath
                                         box•TypeTrace•{•cause,•values•}      BoxPattern
                                             TypeTrace•{•cause,•values•}      StructPattern
                                                         cause                StructPatternPropertyShorthand
                                                                values        StructPatternPropertyShorthand                              */
                sub_placeholder @ Region(Interned(RePlaceholder(_), _)),                                                                  /*
                sub_placeholder•@•Region(Interned(RePlaceholder(_),•_))     PatternVariableDeclaration
                                  Region(Interned(RePlaceholder(_),•_))     TuplePattern
                                         Interned(RePlaceholder(_),•_)      TuplePattern
                                                  RePlaceholder(_)          TuplePattern
                                                                _           WildcardPattern
                                                                    _       WildcardPattern                                               */
                _,                                                                                                                        /*
                _     WildcardPattern                                                                                                     */
                sup_placeholder @ Region(Interned(RePlaceholder(_), _)),                                                                  /*
                sup_placeholder•@•Region(Interned(RePlaceholder(_),•_))     PatternVariableDeclaration
                                  Region(Interned(RePlaceholder(_),•_))     TuplePattern
                                         Interned(RePlaceholder(_),•_)      TuplePattern
                                                  RePlaceholder(_)          TuplePattern
                                                                _           WildcardPattern
                                                                    _       WildcardPattern                                               */
                _,                                                                                                                        /*
                _     WildcardPattern                                                                                                     */
            )) => self.try_report_trait_placeholder_mismatch(                                                                             /*
••••••••••••))                                                    </TuplePattern>
••••••••••••)                                                     </TuplePattern>
                  self.try_report_trait_placeholder_mismatch(↲    <CallExpression>                                                        */
                Some(self.tcx().mk_region(ReVar(*vid))),                                                                                  /*
                Some(self.tcx().mk_region(ReVar(*vid)))     CallExpression
                     self.tcx().mk_region(ReVar(*vid))      CallExpression
                     self.tcx()                             CallExpression
                                          ReVar(*vid)       CallExpression
                                                *vid        DereferenceExpression                                                         */
                cause,
                Some(*sub_placeholder),                                                                                                   /*
                Some(*sub_placeholder)     CallExpression
                     *sub_placeholder      DereferenceExpression                                                                          */
                Some(*sup_placeholder),                                                                                                   /*
                Some(*sup_placeholder)     CallExpression
                     *sup_placeholder      DereferenceExpression                                                                          */
                values,
            ),                                                                                                                            /*
••••••••••••)     </MatchExpressionCase>, </CallExpression>                                                                               */
		GenericParamKind::Const { kw_span, default: Some(default), .. } => {                                                              /*
        GenericParamKind::Const•{•kw_span,•default:•Some(default),•..•}•=>•{↲    <MatchExpressionCase>
        GenericParamKind::Const•{•kw_span,•default:•Some(default),•..•}          StructPattern
        GenericParamKind::Const                                                  ExpressionPath
                                  kw_span                                        StructPatternPropertyShorthand
                                           default:•Some(default)                StructPatternPropertyDestructured
                                                    Some(default)                TuplePattern
                                                                   ..            RestPattern
                                                                           {↲    <BlockExpression>                                        */
            kw_span.to(default.value.span)                                                                                                /*
            kw_span.to(default.value.span)    ExpressionStatement, CallExpression
                       default.value.span     MemberExpression
                       default.value          MemberExpression                                                                            */
        }                                                                                                                                 /*
••••••••}    </MatchExpressionCase>, </BlockExpression>                                                                                   */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 8
// Parsed Nodes: 1913
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 8987 (28% re-reads)
// Unnecessary 'skip_whitespace()' calls: 870
// source: "../../samples/expressions/match.rs"