fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>                                                                                                     */
	let a = move async { };                                                                                                               /*
    let•a•=•move•async•{•};    LetVariableDeclaration
            move•async•{•}     BlockExpression                                                                                            */
	9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999                                        /*
    9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999↲    <ExpressionStatement>
    9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999     Literal                            */
    // boop
    //•boop    Comment
		;                                                                                                                                 /*
   ╚╚;    </ExpressionStatement>                                                                                                          */
	vec! { 1, 2, 3 }.len();                                                                                                               /*
    vec!•{•1,•2,•3•}.len();    ExpressionStatement
    vec!•{•1,•2,•3•}.len()     CallExpression
    vec!•{•1,•2,•3•}           MacroInvocation
           1                   Literal
            ,                  PunctuationToken
              2                Literal
               ,               PunctuationToken
                 3             Literal                                                                                                    */
    write! { vec![], "" }?;                                                                                                               /*
    write!•{•vec![],•""•}?;    ExpressionStatement
    write!•{•vec![],•""•}?     UnwrapExpression
    write!•{•vec![],•""•}      MacroInvocation
                !              PunctuationToken
                 []            DelimGroup
                   ,           PunctuationToken
                     ""        Literal                                                                                                    */
    println!{""}[0];                                                                                                                      /*
    println!{""}        ExpressionStatement, MacroInvocation
             ""         Literal
                [0];    ExpressionStatement
                [0]     ArrayLiteral
                 0      Literal                                                                                                           */
	b.a;                                                                                                                                  /*
    b.a;    ExpressionStatement
    b.a     MemberExpression                                                                                                              */
	*foo += 1;                                                                                                                            /*
    *foo•+=•1;    ExpressionStatement
    *foo•+=•1     ReassignmentOperationExpression
    *foo          DereferenceExpression
            1     Literal                                                                                                                 */
	let &_ = bar;                                                                                                                         /*
    let•&_•=•bar;    LetVariableDeclaration
        &_           ReferencePattern
         _           WildcardPattern                                                                                                      */
	let &mut _ = foo;                                                                                                                     /*
    let•&mut•_•=•foo;    LetVariableDeclaration
        &mut•_           ReferencePattern
             _           WildcardPattern                                                                                                  */
	if let _ = 0 {}                                                                                                                       /*
    if•let•_•=•0•{}    ExpressionStatement, IfBlockExpression
       let•_•=•0       LetScrutinee
           _           WildcardPattern
               0       Literal                                                                                                            */
    while let _ = 0 {}                                                                                                                    /*
    while•let•_•=•0•{}    ExpressionStatement, WhileBlockExpression
          let•_•=•0       LetScrutinee
              _           WildcardPattern
                  0       Literal                                                                                                         */
	let ((),()) = ((),());                                                                                                                /*
    let•((),())•=•((),());    LetVariableDeclaration
        ((),())               TuplePattern
         ()                   TuplePattern
            ()                TuplePattern
                  ((),())     TupleLiteral
                   ()         TupleLiteral
                      ()      TupleLiteral                                                                                                */
	let x: &[u8] = &[0];                                                                                                                  /*
    let•x:•&[u8]•=•&[0];    LetVariableDeclaration
           &[u8]            TypeReference
            [u8]            TypeSlice
                   &[0]     ReferenceExpression
                    [0]     ArrayLiteral
                     0      Literal                                                                                                       */
	let Foo { a, ref b, mut c, x: y, z: z } = foo;                                                                                        /*
    let•Foo•{•a,•ref•b,•mut•c,•x:•y,•z:•z•}•=•foo;    LetVariableDeclaration
        Foo•{•a,•ref•b,•mut•c,•x:•y,•z:•z•}           StructPattern
              a                                       StructPatternPropertyShorthand
                 ref•b                                StructPatternPropertyShorthand
                        mut•c                         StructPatternPropertyShorthand
                               x:•y                   StructPatternPropertyDestructured
                                     z:•z             StructPatternPropertyDestructured                                                   */
	let x = &raw const y;                                                                                                                 /*
    let•x•=•&raw•const•y;    LetVariableDeclaration
            &raw•const•y     RawReferenceExpression                                                                                       */
	let x = &raw mut y;                                                                                                                   /*
    let•x•=•&raw•mut•y;    LetVariableDeclaration
            &raw•mut•y     RawReferenceExpression                                                                                         */
	a::<Box<isize>, _>(box 1);                                                                                                            /*
    a::<Box<isize>,•_>(box•1);    ExpressionStatement
    a::<Box<isize>,•_>(box•1)     CallExpression
        Box<isize>                TypeCall
                    _             TypeInferred
                       box•1      BoxExpression
                           1      Literal                                                                                                 */
	if &raw const one == &raw mut one {}                                                                                                  /*
    if•&raw•const•one•==•&raw•mut•one•{}    ExpressionStatement, IfBlockExpression
       &raw•const•one•==•&raw•mut•one       ComparisonExpression
       &raw•const•one                       RawReferenceExpression
                         &raw•mut•one       RawReferenceExpression                                                                        */
	let _x = if false { 0 } else if true { panic!() } else { 10 };                                                                        /*
    let•_x•=•if•false•{•0•}•else•if•true•{•panic!()•}•else•{•10•};    LetVariableDeclaration
             if•false•{•0•}•else•if•true•{•panic!()•}•else•{•10•}     IfBlockExpression
                false                                                 Literal
                        0                                             ExpressionStatement, Literal
                                 if•true•{•panic!()•}•else•{•10•}     IfBlockExpression
                                    true                              Literal
                                           panic!()                   ExpressionStatement, MacroInvocation
                                                           {•10•}     BlockExpression
                                                             10       ExpressionStatement, Literal                                        */
	let _: <m::A as m::B>::C;                                                                                                             /*
    let•_:•<m::A•as•m::B>::C;    LetVariableDeclaration
        _                        WildcardPattern
           <m::A•as•m::B>::C     TypePath
           <m::A•as•m::B>        ExpressionTypeSelector
            m::A                 TypePath
                    m::B         TypePath                                                                                                 */
	let a: A = A { name: 0 };                                                                                                             /*
    let•a:•A•=•A•{•name:•0•};    LetVariableDeclaration
               A•{•name:•0•}     StructLiteral
                   name:•0       StructLiteralProperty
                         0       Literal                                                                                                  */
	let b1 = &mut *b;                                                                                                                     /*
    let•b1•=•&mut•*b;    LetVariableDeclaration
             &mut•*b     ReferenceExpression
                  *b     DereferenceExpression                                                                                            */
	let mut x: Box<_> = box 3;                                                                                                            /*
    let•mut•x:•Box<_>•=•box•3;    LetVariableDeclaration
        mut•x                     PatternVariableDeclaration
               Box<_>             TypeCall
                   _              TypeInferred
                        box•3     BoxExpression
                            3     Literal                                                                                                 */
	let x: (Box<_>,) = (box 1,);                                                                                                          /*
    let•x:•(Box<_>,)•=•(box•1,);    LetVariableDeclaration
           (Box<_>,)                TypeTuple
            Box<_>                  TypeCall
                _                   TypeInferred
                       (box•1,)     TupleLiteral
                        box•1       BoxExpression
                            1       Literal                                                                                               */
	let &mut ref x = b;                                                                                                                   /*
    let•&mut•ref•x•=•b;    LetVariableDeclaration
        &mut•ref•x         ReferencePattern
             ref•x         PatternVariableDeclaration                                                                                     */
	let &mut mut x = b;                                                                                                                   /*
    let•&mut•mut•x•=•b;    LetVariableDeclaration
        &mut•mut•x         ReferencePattern
             mut•x         PatternVariableDeclaration                                                                                     */
	let ref mut y = b;                                                                                                                    /*
    let•ref•mut•y•=•b;    LetVariableDeclaration
        ref•mut•y         PatternVariableDeclaration                                                                                      */
	let (a, b, c, d);                                                                                                                     /*
    let•(a,•b,•c,•d);    LetVariableDeclaration
        (a,•b,•c,•d)     TuplePattern                                                                                                     */
	let (mut c, mut d);                                                                                                                   /*
    let•(mut•c,•mut•d);    LetVariableDeclaration
        (mut•c,•mut•d)     TuplePattern
         mut•c             PatternVariableDeclaration
                mut•d      PatternVariableDeclaration                                                                                     */
	let s = S { x: 3, y: 4 };                                                                                                             /*
    let•s•=•S•{•x:•3,•y:•4•};    LetVariableDeclaration
            S•{•x:•3,•y:•4•}     StructLiteral
                x:•3             StructLiteralProperty
                   3             Literal
                      y:•4       StructLiteralProperty
                         4       Literal                                                                                                  */
	let mut r = R {c: Box::new(f)};                                                                                                       /*
    let•mut•r•=•R•{c:•Box::new(f)};    LetVariableDeclaration
        mut•r                          PatternVariableDeclaration
                R•{c:•Box::new(f)}     StructLiteral
                   c:•Box::new(f)      StructLiteralProperty
                      Box::new(f)      CallExpression
                      Box::new         ExpressionPath                                                                                     */
	let _:         & usize =     &1;                                                                                                      /*
    let•_:•••••••••&•usize•=•••••&1;    LetVariableDeclaration
        _                               WildcardPattern
                   &•usize              TypeReference
                                 &1     ReferenceExpression
                                  1     Literal                                                                                           */
    let _:       & & usize =    &&1;                                                                                                      /*
    let•_:•••••••&•&•usize•=••••&&1;    LetVariableDeclaration
        _                               WildcardPattern
                 &•&•usize              TypeReference
                   &•usize              TypeReference
                                &&1     ReferenceExpression
                                 &1     ReferenceExpression
                                  1     Literal                                                                                           */
    let _:     & & & usize =   &&&1;                                                                                                      /*
    let•_:•••••&•&•&•usize•=•••&&&1;    LetVariableDeclaration
        _                               WildcardPattern
               &•&•&•usize              TypeReference
                 &•&•usize              TypeReference
                   &•usize              TypeReference
                               &&&1     ReferenceExpression
                                &&1     ReferenceExpression
                                 &1     ReferenceExpression
                                  1     Literal                                                                                           */
    let _:     & & & usize =  & &&1;                                                                                                      /*
    let•_:•••••&•&•&•usize•=••&•&&1;    LetVariableDeclaration
        _                               WildcardPattern
               &•&•&•usize              TypeReference
                 &•&•usize              TypeReference
                   &•usize              TypeReference
                              &•&&1     ReferenceExpression
                                &&1     ReferenceExpression
                                 &1     ReferenceExpression
                                  1     Literal                                                                                           */
    let _:   & & & & usize =  &&&&1;                                                                                                      /*
    let•_:•••&•&•&•&•usize•=••&&&&1;    LetVariableDeclaration
        _                               WildcardPattern
             &•&•&•&•usize              TypeReference
               &•&•&•usize              TypeReference
                 &•&•usize              TypeReference
                   &•usize              TypeReference
                              &&&&1     ReferenceExpression
                               &&&1     ReferenceExpression
                                &&1     ReferenceExpression
                                 &1     ReferenceExpression
                                  1     Literal                                                                                           */
    let _:   & & & & usize = & &&&1;                                                                                                      /*
    let•_:•••&•&•&•&•usize•=•&•&&&1;    LetVariableDeclaration
        _                               WildcardPattern
             &•&•&•&•usize              TypeReference
               &•&•&•usize              TypeReference
                 &•&•usize              TypeReference
                   &•usize              TypeReference
                             &•&&&1     ReferenceExpression
                               &&&1     ReferenceExpression
                                &&1     ReferenceExpression
                                 &1     ReferenceExpression
                                  1     Literal                                                                                           */
    let _: & & & & & usize = &&&&&1;                                                                                                      /*
    let•_:•&•&•&•&•&•usize•=•&&&&&1;    LetVariableDeclaration
        _                               WildcardPattern
           &•&•&•&•&•usize              TypeReference
             &•&•&•&•usize              TypeReference
               &•&•&•usize              TypeReference
                 &•&•usize              TypeReference
                   &•usize              TypeReference
                             &&&&&1     ReferenceExpression
                              &&&&1     ReferenceExpression
                               &&&1     ReferenceExpression
                                &&1     ReferenceExpression
                                 &1     ReferenceExpression
                                  1     Literal                                                                                           */
	let x: T = **item;                                                                                                                    /*
    let•x:•T•=•**item;    LetVariableDeclaration
               **item     DereferenceExpression
                *item     DereferenceExpression                                                                                           */
	let &x = &(&1isize as &dyn T);                                                                                                        /*
    let•&x•=•&(&1isize•as•&dyn•T);    LetVariableDeclaration
        &x                            ReferencePattern
             &(&1isize•as•&dyn•T)     ReferenceExpression
               &1isize•as•&dyn•T      ExpressionAsTypeCast
               &1isize                ReferenceExpression
                1isize                Literal
                 isize                Identifier
                          &dyn•T      TypeReference
                           dyn•T      TypeDynBounds
                               T      TypeTraitBound                                                                                      */
    let &x = &&(&1isize as &dyn T);                                                                                                       /*
    let•&x•=•&&(&1isize•as•&dyn•T);    LetVariableDeclaration
        &x                             ReferencePattern
             &&(&1isize•as•&dyn•T)     ReferenceExpression
              &(&1isize•as•&dyn•T)     ReferenceExpression
                &1isize•as•&dyn•T      ExpressionAsTypeCast
                &1isize                ReferenceExpression
                 1isize                Literal
                  isize                Identifier
                           &dyn•T      TypeReference
                            dyn•T      TypeDynBounds
                                T      TypeTraitBound                                                                                     */
    let &&x = &&(&1isize as &dyn T);                                                                                                      /*
    let•&&x•=•&&(&1isize•as•&dyn•T);    LetVariableDeclaration
        &&x                             ReferencePattern
         &x                             ReferencePattern
              &&(&1isize•as•&dyn•T)     ReferenceExpression
               &(&1isize•as•&dyn•T)     ReferenceExpression
                 &1isize•as•&dyn•T      ExpressionAsTypeCast
                 &1isize                ReferenceExpression
                  1isize                Literal
                   isize                Identifier
                            &dyn•T      TypeReference
                             dyn•T      TypeDynBounds
                                 T      TypeTraitBound                                                                                    */
	let &x = &1isize as &dyn T;                                                                                                           /*
    let•&x•=•&1isize•as•&dyn•T;    LetVariableDeclaration
        &x                         ReferencePattern
             &1isize•as•&dyn•T     ExpressionAsTypeCast
             &1isize               ReferenceExpression
              1isize               Literal
               isize               Identifier
                        &dyn•T     TypeReference
                         dyn•T     TypeDynBounds
                             T     TypeTraitBound                                                                                         */
	let &&x = &1isize as &dyn T;                                                                                                          /*
    let•&&x•=•&1isize•as•&dyn•T;    LetVariableDeclaration
        &&x                         ReferencePattern
         &x                         ReferencePattern
              &1isize•as•&dyn•T     ExpressionAsTypeCast
              &1isize               ReferenceExpression
               1isize               Literal
                isize               Identifier
                         &dyn•T     TypeReference
                          dyn•T     TypeDynBounds
                              T     TypeTraitBound                                                                                        */
	let &&x = &(&1isize as &dyn T);                                                                                                       /*
    let•&&x•=•&(&1isize•as•&dyn•T);    LetVariableDeclaration
        &&x                            ReferencePattern
         &x                            ReferencePattern
              &(&1isize•as•&dyn•T)     ReferenceExpression
                &1isize•as•&dyn•T      ExpressionAsTypeCast
                &1isize                ReferenceExpression
                 1isize                Literal
                  isize                Identifier
                           &dyn•T      TypeReference
                            dyn•T      TypeDynBounds
                                T      TypeTraitBound                                                                                     */
	let &&&x = &(&1isize as &dyn T);                                                                                                      /*
    let•&&&x•=•&(&1isize•as•&dyn•T);    LetVariableDeclaration
        &&&x                            ReferencePattern
         &&x                            ReferencePattern
          &x                            ReferencePattern
               &(&1isize•as•&dyn•T)     ReferenceExpression
                 &1isize•as•&dyn•T      ExpressionAsTypeCast
                 &1isize                ReferenceExpression
                  1isize                Literal
                   isize                Identifier
                            &dyn•T      TypeReference
                             dyn•T      TypeDynBounds
                                 T      TypeTraitBound                                                                                    */
	let box x = box 1isize as Box<dyn T>;                                                                                                 /*
    let•box•x•=•box•1isize•as•Box<dyn•T>;    LetVariableDeclaration
        box•x                                BoxPattern
                box•1isize•as•Box<dyn•T>     ExpressionAsTypeCast
                box•1isize                   BoxExpression
                    1isize                   Literal
                     isize                   Identifier
                              Box<dyn•T>     TypeCall
                                  dyn•T      TypeDynBounds
                                      T      TypeTraitBound                                                                               */
	let box box x = box 1isize as Box<dyn T>;                                                                                             /*
    let•box•box•x•=•box•1isize•as•Box<dyn•T>;    LetVariableDeclaration
        box•box•x                                BoxPattern
            box•x                                BoxPattern
                    box•1isize•as•Box<dyn•T>     ExpressionAsTypeCast
                    box•1isize                   BoxExpression
                        1isize                   Literal
                         isize                   Identifier
                                  Box<dyn•T>     TypeCall
                                      dyn•T      TypeDynBounds
                                          T      TypeTraitBound                                                                           */
	let a = (b[0] as u64) << 0 | (b[1] as u64) << 8 | (b[2] as u64) << 16 | (b[3] as u64) << 24;                                          /*
    let•a•=•(b[0]•as•u64)•<<•0•|•(b[1]•as•u64)•<<•8•|•(b[2]•as•u64)•<<•16•|•(b[3]•as•u64)•<<•24;    LetVariableDeclaration
            (b[0]•as•u64)•<<•0•|•(b[1]•as•u64)•<<•8•|•(b[2]•as•u64)•<<•16•|•(b[3]•as•u64)•<<•24     OperationExpression
            (b[0]•as•u64)•<<•0•|•(b[1]•as•u64)•<<•8•|•(b[2]•as•u64)•<<•16                           OperationExpression
            (b[0]•as•u64)•<<•0•|•(b[1]•as•u64)•<<•8                                                 OperationExpression
            (b[0]•as•u64)•<<•0                                                                      OperationExpression
             b[0]•as•u64                                                                            ExpressionAsTypeCast
             b[0]                                                                                   MemberExpression
               0                                                                                    Literal
                             0                                                                      Literal
                                 (b[1]•as•u64)•<<•8                                                 OperationExpression
                                  b[1]•as•u64                                                       ExpressionAsTypeCast
                                  b[1]                                                              MemberExpression
                                    1                                                               Literal
                                                  8                                                 Literal
                                                      (b[2]•as•u64)•<<•16                           OperationExpression
                                                       b[2]•as•u64                                  ExpressionAsTypeCast
                                                       b[2]                                         MemberExpression
                                                         2                                          Literal
                                                                       16                           Literal
                                                                            (b[3]•as•u64)•<<•24     OperationExpression
                                                                             b[3]•as•u64            ExpressionAsTypeCast
                                                                             b[3]                   MemberExpression
                                                                               3                    Literal
                                                                                             24     Literal                               */
	let a = if let Err(b) = c { d } else { e ! ("") };                                                                                    /*
    let•a•=•if•let•Err(b)•=•c•{•d•}•else•{•e•!•("")•};    LetVariableDeclaration
            if•let•Err(b)•=•c•{•d•}•else•{•e•!•("")•}     IfBlockExpression
               let•Err(b)•=•c                             LetScrutinee
                   Err(b)                                 TuplePattern
                                d                         ExpressionStatement
                                         {•e•!•("")•}     BlockExpression
                                           e•!•("")       ExpressionStatement, MacroInvocation
                                                ""        Literal                                                                         */
	let mut n3 = N3 { n: N2(N1 { n: N0 { x: Box::new(42) } }) };                                                                          /*
    let•mut•n3•=•N3•{•n:•N2(N1•{•n:•N0•{•x:•Box::new(42)•}•})•};    LetVariableDeclaration
        mut•n3                                                      PatternVariableDeclaration
                 N3•{•n:•N2(N1•{•n:•N0•{•x:•Box::new(42)•}•})•}     StructLiteral
                      n:•N2(N1•{•n:•N0•{•x:•Box::new(42)•}•})       StructLiteralProperty
                         N2(N1•{•n:•N0•{•x:•Box::new(42)•}•})       CallExpression
                            N1•{•n:•N0•{•x:•Box::new(42)•}•}        StructLiteral
                                 n:•N0•{•x:•Box::new(42)•}          StructLiteralProperty
                                    N0•{•x:•Box::new(42)•}          StructLiteral
                                         x:•Box::new(42)            StructLiteralProperty
                                            Box::new(42)            CallExpression
                                            Box::new                ExpressionPath
                                                     42             Literal                                                               */
    n3.n.0.n.x = n3.n.0.n.x;                                                                                                              /*
    n3.n.0.n.x•=•n3.n.0.n.x;    ExpressionStatement
    n3.n.0.n.x•=•n3.n.0.n.x     ReassignmentExpression
    n3.n.0.n.x                  MemberExpression
    n3.n.0.n                    MemberExpression
    n3.n.0                      MemberExpression
    n3.n                        MemberExpression
         0                      Index
                 n3.n.0.n.x     MemberExpression
                 n3.n.0.n       MemberExpression
                 n3.n.0         MemberExpression
                 n3.n           MemberExpression
                      0         Index                                                                                                     */
    let mut t = (1, ((2, 3, (4, 5)),));                                                                                                   /*
    let•mut•t•=•(1,•((2,•3,•(4,•5)),));    LetVariableDeclaration
        mut•t                              PatternVariableDeclaration
                (1,•((2,•3,•(4,•5)),))     TupleLiteral
                 1                         Literal
                    ((2,•3,•(4,•5)),)      TupleLiteral
                     (2,•3,•(4,•5))        TupleLiteral
                      2                    Literal
                         3                 Literal
                            (4,•5)         TupleLiteral
                             4             Literal
                                5          Literal                                                                                        */
    t.1.0.2.1 = t.1.0.2.1;                                                                                                                /*
    t.1.0.2.1•=•t.1.0.2.1;    ExpressionStatement
    t.1.0.2.1•=•t.1.0.2.1     ReassignmentExpression
    t.1.0.2.1                 MemberExpression
    t.1.0.2                   MemberExpression
    t.1.0                     MemberExpression
    t.1                       MemberExpression
      1                       Index
        0                     Index
          2                   Index
            1                 Index
                t.1.0.2.1     MemberExpression
                t.1.0.2       MemberExpression
                t.1.0         MemberExpression
                t.1           MemberExpression
                  1           Index
                    0         Index
                      2       Index
                        1     Index                                                                                                       */
	let mut a: A<(), &mut i32> = try { 1 };                                                                                               /*
    let•mut•a:•A<(),•&mut•i32>•=•try•{•1•};    LetVariableDeclaration
        mut•a                                  PatternVariableDeclaration
               A<(),•&mut•i32>                 TypeCall
                 ()                            TypeTuple
                     &mut•i32                  TypeReference
                                 try•{•1•}     TryBlockExpression
                                       1       ExpressionStatement, Literal                                                               */
	let _ = &mut *s.0.borrow_mut();                                                                                                       /*
    let•_•=•&mut•*s.0.borrow_mut();    LetVariableDeclaration
        _                              WildcardPattern
            &mut•*s.0.borrow_mut()     ReferenceExpression
                 *s.0.borrow_mut()     DereferenceExpression
                  s.0.borrow_mut()     CallExpression
                  s.0                  MemberExpression
                    0                  Index                                                                                              */
	let _ = &mut *s[0].borrow_mut();                                                                                                      /*
    let•_•=•&mut•*s[0].borrow_mut();    LetVariableDeclaration
        _                               WildcardPattern
            &mut•*s[0].borrow_mut()     ReferenceExpression
                 *s[0].borrow_mut()     DereferenceExpression
                  s[0].borrow_mut()     CallExpression
                  s[0]                  MemberExpression
                    0                   Literal                                                                                           */
    let x: Foo<_> = Bar::<usize>(PhantomData);                                                                                            /*
    let•x:•Foo<_>•=•Bar::<usize>(PhantomData);    LetVariableDeclaration
           Foo<_>                                 TypeCall
               _                                  TypeInferred
                    Bar::<usize>(PhantomData)     CallExpression                                                                          */
    let f = A::<i32> { a: 10 };                                                                                                           /*
    let•f•=•A::<i32>•{•a:•10•};    LetVariableDeclaration
            A::<i32>•{•a:•10•}     StructLiteral
            A::<i32>               ExpressionTypeCast
                       a:•10       StructLiteralProperty
                          10       Literal                                                                                                */
	let v: <() as Lt<'_>>::T = ();                                                                                                        /*
    let•v:•<()•as•Lt<'_>>::T•=•();    LetVariableDeclaration
           <()•as•Lt<'_>>::T          TypePath
           <()•as•Lt<'_>>             ExpressionTypeSelector
            ()                        TypeTuple
                  Lt<'_>              TypeCall
                     '_               LtElided
                               ()     TupleLiteral                                                                                        */
	<E>::V() = E::V();                                                                                                                    /*
    <E>::V()•=•E::V();    ExpressionStatement
    <E>::V()•=•E::V()     ReassignmentExpression
    <E>::V()              CallExpression
    <E>::V                ExpressionPath
    <E>                   ExpressionTypeSelector
               E::V()     CallExpression
               E::V       ExpressionPath                                                                                                  */
	<E>::V {} = E::V();                                                                                                                   /*
    <E>::V•{}•=•E::V();    ExpressionStatement
    <E>::V•{}•=•E::V()     ReassignmentExpression
    <E>::V•{}              StructLiteral
    <E>::V                 ExpressionPath
    <E>                    ExpressionTypeSelector
                E::V()     CallExpression
                E::V       ExpressionPath                                                                                                 */
	let a = &mut b.0.0;                                                                                                                   /*
    let•a•=•&mut•b.0.0;    LetVariableDeclaration
            &mut•b.0.0     ReferenceExpression
                 b.0.0     MemberExpression
                 b.0       MemberExpression
                   0       Index
                     0     Index                                                                                                          */
	let a = &mut b.0[2];                                                                                                                  /*
    let•a•=•&mut•b.0[2];    LetVariableDeclaration
            &mut•b.0[2]     ReferenceExpression
                 b.0[2]     MemberExpression
                 b.0        MemberExpression
                   0        Index
                     2      Literal                                                                                                       */
	let _ = a::<N>(b().await).await;                                                                                                      /*
    let•_•=•a::<N>(b().await).await;    LetVariableDeclaration
        _                               WildcardPattern
            a::<N>(b().await).await     AwaitExpression
            a::<N>(b().await)           CallExpression
                   b().await            AwaitExpression
                   b()                  CallExpression                                                                                    */
	let _ = a(b::<N>().await).await;                                                                                                      /*
    let•_•=•a(b::<N>().await).await;    LetVariableDeclaration
        _                               WildcardPattern
            a(b::<N>().await).await     AwaitExpression
            a(b::<N>().await)           CallExpression
              b::<N>().await            AwaitExpression
              b::<N>()                  CallExpression                                                                                    */
	let _ = A == s!("e");                                                                                                                 /*
    let•_•=•A•==•s!("e");    LetVariableDeclaration
        _                    WildcardPattern
            A•==•s!("e")     ComparisonExpression
                 s!("e")     MacroInvocation
                    "e"      Literal                                                                                                      */
	let a: & str = & b;                                                                                                                   /*
    let•a:•&•str•=•&•b;    LetVariableDeclaration
           &•str           TypeReference
                   &•b     ReferenceExpression                                                                                            */
	::a::<f64, [u8; 8]>(a!());                                                                                                            /*
    ::a::<f64,•[u8;•8]>(a!());    ExpressionStatement
    ::a::<f64,•[u8;•8]>(a!())     CallExpression
    ::a                           ExpressionPath
               [u8;•8]            TypeSizedArray
                    8             Literal
                        a!()      MacroInvocation                                                                                         */
	let (the, guardian, stands, resolute) = ("the", "Turbofish", "remains", "undefeated");                                                /*
    let•(the,•guardian,•stands,•resolute)•=•("the",•"Turbofish",•"remains",•"undefeated");    LetVariableDeclaration
        (the,•guardian,•stands,•resolute)                                                     TuplePattern
                                            ("the",•"Turbofish",•"remains",•"undefeated")     TupleLiteral
                                             "the"                                            Literal
                                                    "Turbofish"                               Literal
                                                                 "remains"                    Literal
                                                                            "undefeated"      Literal                                     */
    let _: (bool, bool) = (the<guardian, stands>(resolute));                                                                              /*
    let•_:•(bool,•bool)•=•(the<guardian,•stands>(resolute));    LetVariableDeclaration
        _                                                       WildcardPattern
           (bool,•bool)                                         TypeTuple
                          (the<guardian,•stands>(resolute))     TupleLiteral
                           the<guardian                         ComparisonExpression
                                         stands>(resolute)      ComparisonExpression                                                      */
    let (A { x: _x, y: _y }, Z): (_, Z) = c(|| B { x: X, y: Y }, || Z);                                                                   /*
    let•(A•{•x:•_x,•y:•_y•},•Z):•(_,•Z)•=•c(||•B•{•x:•X,•y:•Y•},•||•Z);    LetVariableDeclaration
        (A•{•x:•_x,•y:•_y•},•Z)                                            TuplePattern
         A•{•x:•_x,•y:•_y•}                                                StructPattern
             x:•_x                                                         StructPatternPropertyDestructured
                    y:•_y                                                  StructPatternPropertyDestructured
                                 (_,•Z)                                    TypeTuple
                                  _                                        TypeInferred
                                          c(||•B•{•x:•X,•y:•Y•},•||•Z)     CallExpression
                                            ||•B•{•x:•X,•y:•Y•}            ClosureFunctionExpression
                                               B•{•x:•X,•y:•Y•}            StructLiteral
                                                   x:•X                    StructLiteralProperty
                                                         y:•Y              StructLiteralProperty
                                                                 ||•Z      ClosureFunctionExpression                                      */
	let _: A<{ 1+2 }>;                                                                                                                    /*
    let•_:•A<{•1+2•}>;    LetVariableDeclaration
        _                 WildcardPattern
           A<{•1+2•}>     TypeCall
             {•1+2•}      BlockExpression
               1+2        ExpressionStatement, OperationExpression
               1          Literal
                 2        Literal                                                                                                         */
	let _: A<{ 5 }>;                                                                                                                      /*
    let•_:•A<{•5•}>;    LetVariableDeclaration
        _               WildcardPattern
           A<{•5•}>     TypeCall
             {•5•}      BlockExpression
               5        ExpressionStatement, Literal                                                                                      */
    let A::<1, N>(N) = A::new();                                                                                                          /*
    let•A::<1,•N>(N)•=•A::new();    LetVariableDeclaration
        A::<1,•N>(N)                TuplePattern
        A::<1,•N>                   ExpressionTypeCast
            1                       Literal
                       A::new()     CallExpression
                       A::new       ExpressionPath                                                                                        */
    let _ = Some(Foo { _a: 42 }).map(|a| a as Foo::<i32>);                                                                                /*
    let•_•=•Some(Foo•{•_a:•42•}).map(|a|•a•as•Foo::<i32>);    LetVariableDeclaration
        _                                                     WildcardPattern
            Some(Foo•{•_a:•42•}).map(|a|•a•as•Foo::<i32>)     CallExpression
            Some(Foo•{•_a:•42•})                              CallExpression
                 Foo•{•_a:•42•}                               StructLiteral
                       _a:•42                                 StructLiteralProperty
                           42                                 Literal
                                     |a|•a•as•Foo::<i32>      ClosureFunctionExpression
                                      a                       ClosureFunctionParameterDeclaration
                                         a•as•Foo::<i32>      ExpressionAsTypeCast
                                              Foo::<i32>      TypeCall                                                                    */
    let _ = ()=()=()=();                                                                                                                  /*
    let•_•=•()=()=()=();    LetVariableDeclaration
        _                   WildcardPattern
            ()=()=()=()     ReassignmentExpression
            ()              TupleLiteral
               ()=()=()     ReassignmentExpression
               ()           TupleLiteral
                  ()=()     ReassignmentExpression
                  ()        TupleLiteral
                     ()     TupleLiteral                                                                                                  */
	String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>();                                                                /*
    String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>();    ExpressionStatement
    String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>()     CallExpression
    String::<>::from::<>("><>").chars::<>().rev::<>()                         CallExpression
    String::<>::from::<>("><>").chars::<>()                                   CallExpression
    String::<>::from::<>("><>")                                               CallExpression
    String::<>::from                                                          ExpressionPath
    String::<>                                                                ExpressionTypeCast
                         "><>"                                                Literal                                                     */
	fn a(x: &f<r>) { return while !x.f() { x.g(0); }; }                                                                                   /*
    fn•a(x:•&f<r>)•{•return•while•!x.f()•{•x.g(0);•};•}    FunctionDeclaration
         x:•&f<r>                                          FunctionParameterDeclaration
            &f<r>                                          TypeReference
             f<r>                                          TypeCall
                     return•while•!x.f()•{•x.g(0);•};      ExpressionStatement
                     return•while•!x.f()•{•x.g(0);•}       ReturnExpression
                            while•!x.f()•{•x.g(0);•}       WhileBlockExpression
                                  !x.f()                   NotExpression
                                   x.f()                   CallExpression
                                           x.g(0);         ExpressionStatement
                                           x.g(0)          CallExpression
                                               0           Literal                                                                        */
    let i = &f::s(0);                                                                                                                     /*
    let•i•=•&f::s(0);    LetVariableDeclaration
            &f::s(0)     ReferenceExpression
             f::s(0)     CallExpression
             f::s        ExpressionPath
                  0      Literal                                                                                                          */
	<u8 as D<13>>::e::<u8>();                                                                                                             /*
    <u8•as•D<13>>::e::<u8>();    ExpressionStatement
    <u8•as•D<13>>::e::<u8>()     CallExpression
    <u8•as•D<13>>::e             ExpressionPath
    <u8•as•D<13>>                ExpressionTypeSelector
           D<13>                 TypeCall
             13                  Literal                                                                                                  */
	let _: i32 = (match "" {                                                                                                              /*
    let•_:•i32•=•(match•""•{↲    <LetVariableDeclaration>
        _                        WildcardPattern
                 (match•""•{↲    <CallExpression>
                  match•""•{↲    <MatchExpression>
                        ""       Literal                                                                                                  */
        "+" => ::std::ops::Add::add,                                                                                                      /*
        "+"•=>•::std::ops::Add::add     MatchExpressionCase
        "+"                             Literal
               ::std::ops::Add::add     ExpressionPath
               ::std::ops::Add          ExpressionPath
               ::std::ops               ExpressionPath
               ::std                    ExpressionPath                                                                                    */
        "-" => ::std::ops::Sub::sub,                                                                                                      /*
        "-"•=>•::std::ops::Sub::sub     MatchExpressionCase
        "-"                             Literal
               ::std::ops::Sub::sub     ExpressionPath
               ::std::ops::Sub          ExpressionPath
               ::std::ops               ExpressionPath
               ::std                    ExpressionPath                                                                                    */
        "<" => |a,b| (a < b) as i32,                                                                                                      /*
        "<"•=>•|a,b|•(a•<•b)•as•i32     MatchExpressionCase
        "<"                             Literal
               |a,b|•(a•<•b)•as•i32     ClosureFunctionExpression
                a                       ClosureFunctionParameterDeclaration
                  b                     ClosureFunctionParameterDeclaration
                     (a•<•b)•as•i32     ExpressionAsTypeCast
                      a•<•b             ComparisonExpression                                                                              */
        _ => c!(),                                                                                                                        /*
        _•=>•c!()     MatchExpressionCase
        _             WildcardPattern
             c!()     MacroInvocation                                                                                                     */
    })(5, 5);                                                                                                                             /*
••••})(5,•5);    </LetVariableDeclaration>
••••})(5,•5)     </CallExpression>
••••}            </MatchExpression>
       5         Literal
          5      Literal                                                                                                                  */
	[].e().f(|_: &i32| {                                                                                                                  /*
    [].e().f(|_:•&i32|•{↲    <ExpressionStatement>, <CallExpression>
    [].e()                   CallExpression
    []                       ArrayLiteral
             |_:•&i32|•{↲    <ClosureFunctionExpression>
              _:•&i32        ClosureFunctionParameterDeclaration
              _              WildcardPattern
                 &i32        TypeReference
                       {↲    <BlockExpression>                                                                                            */
        [].e().f(move |_: &i32| {                                                                                                         /*
        [].e().f(move•|_:•&i32|•{↲    <ExpressionStatement>, <CallExpression>
        [].e()                        CallExpression
        []                            ArrayLiteral
                 move•|_:•&i32|•{↲    <ClosureFunctionExpression>
                       _:•&i32        ClosureFunctionParameterDeclaration
                       _              WildcardPattern
                          &i32        TypeReference
                                {↲    <BlockExpression>                                                                                   */
            i += 1;                                                                                                                       /*
            i•+=•1;    ExpressionStatement
            i•+=•1     ReassignmentOperationExpression
                 1     Literal                                                                                                            */
        });                                                                                                                               /*
••••••••});    </ExpressionStatement>
••••••••})     </CallExpression>
••••••••}      </ClosureFunctionExpression>, </BlockExpression>                                                                           */
    });                                                                                                                                   /*
••••});    </ExpressionStatement>
••••})     </CallExpression>
••••}      </ClosureFunctionExpression>, </BlockExpression>                                                                               */
    let _x2 = X { a: 1, b: 2, ..DX };                                                                                                     /*
    let•_x2•=•X•{•a:•1,•b:•2,•..DX•};    LetVariableDeclaration
              X•{•a:•1,•b:•2,•..DX•}     StructLiteral
                  a:•1                   StructLiteralProperty
                     1                   Literal
                        b:•2             StructLiteralProperty
                           2             Literal
                              ..DX       StructLiteralPropertySpread                                                                      */
	i[i[0]] = 0;                                                                                                                          /*
    i[i[0]]•=•0;    ExpressionStatement
    i[i[0]]•=•0     ReassignmentExpression
    i[i[0]]         MemberExpression
      i[0]          MemberExpression
        0           Literal
              0     Literal                                                                                                               */
    i[i[0] - 1] = 0;                                                                                                                      /*
    i[i[0]•-•1]•=•0;    ExpressionStatement
    i[i[0]•-•1]•=•0     ReassignmentExpression
    i[i[0]•-•1]         MemberExpression
      i[0]•-•1          OperationExpression
      i[0]              MemberExpression
        0               Literal
             1          Literal
                  0     Literal                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

[                                                                                                                                         /*
[↲    <ExpressionStatement>, <ArrayLiteral>                                                                                               */
	b.a,                                                                                                                                  /*
    b.a    MemberExpression                                                                                                               */
	X { len: 3 },                                                                                                                         /*
    X•{•len:•3•}    StructLiteral
        len:•3      StructLiteralProperty
             3      Literal                                                                                                               */
	x.len > (3),                                                                                                                          /*
    x.len•>•(3)    ComparisonExpression
    x.len          MemberExpression
             3     Literal                                                                                                                */
	x.len >> (3),                                                                                                                         /*
    x.len•>>•(3)    OperationExpression
    x.len           MemberExpression
              3     Literal                                                                                                               */
	vec![1, 2, 3].into_iter().collect::<Vec<usize>>(),                                                                                    /*
    vec![1,•2,•3].into_iter().collect::<Vec<usize>>()    CallExpression
    vec![1,•2,•3].into_iter()                            CallExpression
    vec![1,•2,•3]                                        MacroInvocation
         1                                               Literal
          ,                                              PunctuationToken
            2                                            Literal
             ,                                           PunctuationToken
               3                                         Literal
                                        Vec<usize>       TypeCall                                                                         */
	X(1, 2, 3),                                                                                                                           /*
    X(1,•2,•3)    CallExpression
      1           Literal
         2        Literal
            3     Literal                                                                                                                 */
	(1, 2, 3),                                                                                                                            /*
    (1,•2,•3)    TupleLiteral
     1           Literal
        2        Literal
           3     Literal                                                                                                                  */
	vec! { 1, 2, 3 }.len(),                                                                                                               /*
    vec!•{•1,•2,•3•}.len()    CallExpression
    vec!•{•1,•2,•3•}          MacroInvocation
           1                  Literal
            ,                 PunctuationToken
              2               Literal
               ,              PunctuationToken
                 3            Literal                                                                                                     */
	write! { vec![], "" }?,                                                                                                               /*
    write!•{•vec![],•""•}?    UnwrapExpression
    write!•{•vec![],•""•}     MacroInvocation
                !             PunctuationToken
                 []           DelimGroup
                   ,          PunctuationToken
                     ""       Literal                                                                                                     */
	&*d.borrow(),                                                                                                                         /*
    &*d.borrow()    ReferenceExpression
     *d.borrow()    DereferenceExpression
      d.borrow()    CallExpression                                                                                                        */
	**bar == Test::Baz || **bar == Test::Qux,                                                                                             /*
    **bar•==•Test::Baz•||•**bar•==•Test::Qux    OrExpression
    **bar•==•Test::Baz                          ComparisonExpression
    **bar                                       DereferenceExpression
     *bar                                       DereferenceExpression
             Test::Baz                          ExpressionPath
                          **bar•==•Test::Qux    ComparisonExpression
                          **bar                 DereferenceExpression
                           *bar                 DereferenceExpression
                                   Test::Qux    ExpressionPath                                                                            */
	&foo[0..1],                                                                                                                           /*
    &foo[0..1]    ReferenceExpression
     foo[0..1]    MemberExpression
         0..1     RangeLiteral
         0        Literal
            1     Literal                                                                                                                 */
	TypeId::of::<T>(),                                                                                                                    /*
    TypeId::of::<T>()    CallExpression
    TypeId::of           ExpressionPath                                                                                                   */
	&[*xs[0].x, *xs[1].x],                                                                                                                /*
    &[*xs[0].x,•*xs[1].x]    ReferenceExpression
     [*xs[0].x,•*xs[1].x]    ArrayLiteral
      *xs[0].x               DereferenceExpression
       xs[0].x               MemberExpression
       xs[0]                 MemberExpression
          0                  Literal
                *xs[1].x     DereferenceExpression
                 xs[1].x     MemberExpression
                 xs[1]       MemberExpression
                    1        Literal                                                                                                      */
	&mut tup.0,                                                                                                                           /*
    &mut•tup.0    ReferenceExpression
         tup.0    MemberExpression
             0    Index                                                                                                                   */
	<_>::f(),                                                                                                                             /*
    <_>::f()    CallExpression
    <_>::f      ExpressionPath
    <_>         ExpressionTypeSelector
     _          TypeInferred                                                                                                              */
	&(fop::<T> as fn()),                                                                                                                  /*
    &(fop::<T>•as•fn())    ReferenceExpression
      fop::<T>•as•fn()     ExpressionAsTypeCast
      fop::<T>             ExpressionTypeCast
                  fn()     TypeFnPointer                                                                                                  */
	(a)(),                                                                                                                                /*
    (a)()    CallExpression                                                                                                               */
	::foo::bar::baz::f(),                                                                                                                 /*
    ::foo::bar::baz::f()    CallExpression
    ::foo::bar::baz::f      ExpressionPath
    ::foo::bar::baz         ExpressionPath
    ::foo::bar              ExpressionPath
    ::foo                   ExpressionPath                                                                                                */
	<() as ::foo::T>::Assoc::f(),                                                                                                         /*
    <()•as•::foo::T>::Assoc::f()    CallExpression
    <()•as•::foo::T>::Assoc::f      ExpressionPath
    <()•as•::foo::T>::Assoc         ExpressionPath
    <()•as•::foo::T>                ExpressionTypeSelector
     ()                             TypeTuple
           ::foo::T                 TypePath
           ::foo                    TypePath                                                                                              */
	[].a(),                                                                                                                               /*
    [].a()    CallExpression
    []        ArrayLiteral                                                                                                                */
	id::<[i32; 3]>([1,2,3]),                                                                                                              /*
    id::<[i32;•3]>([1,2,3])    CallExpression
         [i32;•3]              TypeSizedArray
               3               Literal
                   [1,2,3]     ArrayLiteral
                    1          Literal
                      2        Literal
                        3      Literal                                                                                                    */
	m::Pub(0u8).method_with_priv_params(loop{}),                                                                                          /*
    m::Pub(0u8).method_with_priv_params(loop{})    CallExpression
    m::Pub(0u8)                                    CallExpression
    m::Pub                                         ExpressionPath
           0u8                                     Literal
            u8                                     Identifier
                                        loop{}     LoopBlockExpression                                                                    */
	<m::Pub<m::Alias>>::INHERENT_ASSOC_CONST,                                                                                             /*
    <m::Pub<m::Alias>>::INHERENT_ASSOC_CONST    ExpressionPath
    <m::Pub<m::Alias>>                          ExpressionTypeSelector
     m::Pub<m::Alias>                           TypeCall
     m::Pub                                     TypePath
            m::Alias                            TypePath                                                                                  */
	<a!() as B>::f(0),                                                                                                                    /*
    <a!()•as•B>::f(0)    CallExpression
    <a!()•as•B>::f       ExpressionPath
    <a!()•as•B>          ExpressionTypeSelector
     a!()                MacroInvocation
                   0     Literal                                                                                                          */
	a::<B<N, { N as u128 }>>(),                                                                                                           /*
    a::<B<N,•{•N•as•u128•}>>()    CallExpression
        B<N,•{•N•as•u128•}>       TypeCall
             {•N•as•u128•}        BlockExpression
               N•as•u128          ExpressionStatement, ExpressionAsTypeCast                                                               */
	Foo { f:(((((((x)).clone()))))) },                                                                                                    /*
    Foo•{•f:(((((((x)).clone())))))•}    StructLiteral
          f:(((((((x)).clone())))))      StructLiteralProperty
                 ((x)).clone()           CallExpression                                                                                   */
	a::<&str, (*const u8, u64)>(),                                                                                                        /*
    a::<&str,•(*const•u8,•u64)>()    CallExpression
        &str                         TypeReference
              (*const•u8,•u64)       TypeTuple
               *const•u8             TypeDereferenceConst                                                                                 */
	a("".b()).c("").d().await,                                                                                                            /*
    a("".b()).c("").d().await    AwaitExpression
    a("".b()).c("").d()          CallExpression
    a("".b()).c("")              CallExpression
    a("".b())                    CallExpression
      "".b()                     CallExpression
      ""                         Literal
                ""               Literal                                                                                                  */
	foo(&[vec![123]]).await,                                                                                                              /*
    foo(&[vec![123]]).await    AwaitExpression
    foo(&[vec![123]])          CallExpression
        &[vec![123]]           ReferenceExpression
         [vec![123]]           ArrayLiteral
          vec![123]            MacroInvocation
               123             Literal                                                                                                    */
	A::b::<C>(x).d(E("x"))?.f(1),                                                                                                         /*
    A::b::<C>(x).d(E("x"))?.f(1)    CallExpression
    A::b::<C>(x).d(E("x"))?         UnwrapExpression
    A::b::<C>(x).d(E("x"))          CallExpression
    A::b::<C>(x)                    CallExpression
    A::b                            ExpressionPath
                   E("x")           CallExpression
                     "x"            Literal
                              1     Literal                                                                                               */
	// std::<_ as _>,
    //•std::<_•as•_>,    Comment
	std::<0>,                                                                                                                             /*
    std::<0>    ExpressionTypeCast
          0     Literal                                                                                                                   */
	&raw const x,                                                                                                                         /*
    &raw•const•x    RawReferenceExpression                                                                                                */
	(A::a as fn(&(dyn A+'static))->B)(&"c"),                                                                                              /*
    (A::a•as•fn(&(dyn•A+'static))->B)(&"c")    CallExpression
     A::a•as•fn(&(dyn•A+'static))->B           ExpressionAsTypeCast
     A::a                                      ExpressionPath
             fn(&(dyn•A+'static))->B           TypeFnPointer
                &(dyn•A+'static)               TypeFnPointerParameter, TypeReference
                  dyn•A+'static                TypeDynBounds
                      A                        TypeTraitBound
                        'static                LtStatic
                                      &"c"     ReferenceExpression
                                       "c"     Literal                                                                                    */
	f::<<T as S>::E>(),                                                                                                                   /*
    f::<<T•as•S>::E>()    CallExpression
        <T•as•S>::E       TypePath
        <T•as•S>          ExpressionTypeSelector                                                                                          */
	<u64 as From<<T as Iterator>::Item>>::from(),                                                                                         /*
    <u64•as•From<<T•as•Iterator>::Item>>::from()    CallExpression
    <u64•as•From<<T•as•Iterator>::Item>>::from      ExpressionPath
    <u64•as•From<<T•as•Iterator>::Item>>            ExpressionTypeSelector
            From<<T•as•Iterator>::Item>             TypeCall
                 <T•as•Iterator>::Item              TypePath
                 <T•as•Iterator>                    ExpressionTypeSelector                                                                */
	<<Q as A<'_>>::B as C<Q::D>>::e(db),                                                                                                  /*
    <<Q•as•A<'_>>::B•as•C<Q::D>>::e(db)    CallExpression
    <<Q•as•A<'_>>::B•as•C<Q::D>>::e        ExpressionPath
    <<Q•as•A<'_>>::B•as•C<Q::D>>           ExpressionTypeSelector
     <Q•as•A<'_>>::B                       TypePath
     <Q•as•A<'_>>                          ExpressionTypeSelector
           A<'_>                           TypeCall
             '_                            LtElided
                        C<Q::D>            TypeCall
                          Q::D             TypePath                                                                                       */
	tuple. 0.0 ,                                                                                                                          /*
    tuple.•0.0    MemberExpression
    tuple.•0      MemberExpression
           0      Index
             0    Index                                                                                                                   */
	tuple.0. 0 ,                                                                                                                          /*
    tuple.0.•0    MemberExpression
    tuple.0       MemberExpression
          0       Index
             0    Index                                                                                                                   */
	tuple./*special cases*/0.0 //aaa
                                                                                                                                          /*
    tuple./*special•cases*/0.0          MemberExpression
    tuple./*special•cases*/0            MemberExpression
          /*special•cases*/             Comment
                           0            Index
                             0          Index
                               //aaa    Comment                                                                                           */
   ,(((),),),                                                                                                                             /*
    (((),),)     TupleLiteral
     ((),)       TupleLiteral
      ()         TupleLiteral                                                                                                             */
   	(1, (2, 3)).1.1,                                                                                                                   /*
       (1,•(2,•3)).1.1    MemberExpression
       (1,•(2,•3)).1      MemberExpression
       (1,•(2,•3))        TupleLiteral
        1                 Literal
           (2,•3)         TupleLiteral
            2             Literal
               3          Literal
                   1      Index
                     1    Index                                                                                                           */
   	(1, (2, (3, 4))).1.1.1                                                                                                             /*
       (1,•(2,•(3,•4))).1.1.1↲    <CallExpression>
       (1,•(2,•(3,•4))).1.1.1     MemberExpression
       (1,•(2,•(3,•4))).1.1       MemberExpression
       (1,•(2,•(3,•4))).1         MemberExpression
       (1,•(2,•(3,•4)))           TupleLiteral
        1                         Literal
           (2,•(3,•4))            TupleLiteral
            2                     Literal
               (3,•4)             TupleLiteral
                3                 Literal
                   4              Literal
                        1         Index
                          1       Index
                            1     Index                                                                                                   */
	(1,),                                                                                                                                 /*
   ╚(1,)    </CallExpression>
     1      Literal                                                                                                                       */
	(1),                                                                                                                                  /*
     1    Literal                                                                                                                         */
	a ((1,2.0,3)),                                                                                                                        /*
    a•((1,2.0,3))    CallExpression
       (1,2.0,3)     TupleLiteral
        1            Literal
          2.0        Literal
              3      Literal                                                                                                              */
	b ((1,)),                                                                                                                             /*
    b•((1,))    CallExpression
       (1,)     TupleLiteral
        1       Literal                                                                                                                   */
	1.f::<T>(),                                                                                                                           /*
    1.f::<T>()    CallExpression
    1             Literal                                                                                                                 */
	*a = &a[1..],                                                                                                                         /*
    *a•=•&a[1..]    ReassignmentExpression
    *a              DereferenceExpression
         &a[1..]    ReferenceExpression
          a[1..]    MemberExpression
            1..     RangeLiteral
            1       Literal                                                                                                               */
	a().await.0,                                                                                                                          /*
    a().await.0    MemberExpression
    a().await      AwaitExpression
    a()            CallExpression
              0    Index                                                                                                                  */
	a.b(c).await.d(e)?,                                                                                                                   /*
    a.b(c).await.d(e)?    UnwrapExpression
    a.b(c).await.d(e)     CallExpression
    a.b(c).await          AwaitExpression
    a.b(c)                CallExpression                                                                                                  */
	0 + 1, 0 * 1, 0 - 1, 0 / 1, 0 % 1, 0 & 1, 0 | 1, 0 << 1, 0 >> 1, 0 == 1, 0 != 1, 0 < 1, 0 > 1, 0 <= 1, 0 >= 1,                        /*
    0•+•1                                                                                                            OperationExpression
    0                                                                                                                Literal
        1                                                                                                            Literal
           0•*•1                                                                                                     OperationExpression
           0                                                                                                         Literal
               1                                                                                                     Literal
                  0•-•1                                                                                              OperationExpression
                  0                                                                                                  Literal
                      1                                                                                              Literal
                         0•/•1                                                                                       OperationExpression
                         0                                                                                           Literal
                             1                                                                                       Literal
                                0•%•1                                                                                OperationExpression
                                0                                                                                    Literal
                                    1                                                                                Literal
                                       0•&•1                                                                         OperationExpression
                                       0                                                                             Literal
                                           1                                                                         Literal
                                              0•|•1                                                                  OperationExpression
                                              0                                                                      Literal
                                                  1                                                                  Literal
                                                     0•<<•1                                                          OperationExpression
                                                     0                                                               Literal
                                                          1                                                          Literal
                                                             0•>>•1                                                  OperationExpression
                                                             0                                                       Literal
                                                                  1                                                  Literal
                                                                     0•==•1                                          ComparisonExpression
                                                                     0                                               Literal
                                                                          1                                          Literal
                                                                             0•!=•1                                  ComparisonExpression
                                                                             0                                       Literal
                                                                                  1                                  Literal
                                                                                     0•<•1                           ComparisonExpression
                                                                                     0                               Literal
                                                                                         1                           Literal
                                                                                            0•>•1                    ComparisonExpression
                                                                                            0                        Literal
                                                                                                1                    Literal
                                                                                                   0•<=•1            ComparisonExpression
                                                                                                   0                 Literal
                                                                                                        1            Literal
                                                                                                           0•>=•1    ComparisonExpression
                                                                                                           0         Literal
                                                                                                                1    Literal              */
	x -= 0, x *= 0, x /= 0, x &= 0, x %= 0, x ^= 0, x += 0, x <<= 0, x <<= 0, x >>= 0, x >>= 0, x |= 0,                                   /*
    x•-=•0                                                                                                ReassignmentOperationExpression
         0                                                                                                Literal
            x•*=•0                                                                                        ReassignmentOperationExpression
                 0                                                                                        Literal
                    x•/=•0                                                                                ReassignmentOperationExpression
                         0                                                                                Literal
                            x•&=•0                                                                        ReassignmentOperationExpression
                                 0                                                                        Literal
                                    x•%=•0                                                                ReassignmentOperationExpression
                                         0                                                                Literal
                                            x•^=•0                                                        ReassignmentOperationExpression
                                                 0                                                        Literal
                                                    x•+=•0                                                ReassignmentOperationExpression
                                                         0                                                Literal
                                                            x•<<=•0                                       ReassignmentOperationExpression
                                                                  0                                       Literal
                                                                     x•<<=•0                              ReassignmentOperationExpression
                                                                           0                              Literal
                                                                              x•>>=•0                     ReassignmentOperationExpression
                                                                                    0                     Literal
                                                                                       x•>>=•0            ReassignmentOperationExpression
                                                                                             0            Literal
                                                                                                x•|=•0    ReassignmentOperationExpression
                                                                                                     0    Literal                         */
	A::<1>::B(),                                                                                                                          /*
    A::<1>::B()    CallExpression
    A::<1>::B      ExpressionPath
    A::<1>         ExpressionTypeCast
        1          Literal                                                                                                                */
	A::<1>::B{},                                                                                                                          /*
    A::<1>::B{}    StructLiteral
    A::<1>::B      ExpressionPath
    A::<1>         ExpressionTypeCast
        1          Literal                                                                                                                */
	A::<1>(),                                                                                                                             /*
    A::<1>()    CallExpression
        1       Literal                                                                                                                   */
	A::<1>{},                                                                                                                             /*
    A::<1>{}    StructLiteral
    A::<1>      ExpressionTypeCast
        1       Literal                                                                                                                   */
	{ { } 2 },                                                                                                                            /*
    {•{•}•2•}    BlockExpression
      {•}        ExpressionStatement, BlockExpression
          2      ExpressionStatement, Literal                                                                                             */
	&mut [0; 1][..],                                                                                                                      /*
    &mut•[0;•1][..]    ReferenceExpression
         [0;•1][..]    MemberExpression
         [0;•1]        SizedArrayLiteral
          0            Literal
             1         Literal
                ..     RangeLiteral                                                                                                       */
	&B::<T>::A[0],                                                                                                                        /*
    &B::<T>::A[0]    ReferenceExpression
     B::<T>::A[0]    MemberExpression
     B::<T>::A       ExpressionPath
     B::<T>          ExpressionTypeCast
               0     Literal                                                                                                              */
	&B::<T>::A.0[0],                                                                                                                      /*
    &B::<T>::A.0[0]    ReferenceExpression
     B::<T>::A.0[0]    MemberExpression
     B::<T>::A.0       MemberExpression
     B::<T>::A         ExpressionPath
     B::<T>            ExpressionTypeCast
               0       Index
                 0     Literal                                                                                                            */
	&(B::<T>::A.0).1[0],                                                                                                                  /*
    &(B::<T>::A.0).1[0]    ReferenceExpression
     (B::<T>::A.0).1[0]    MemberExpression
     (B::<T>::A.0).1       MemberExpression
      B::<T>::A.0          MemberExpression
      B::<T>::A            ExpressionPath
      B::<T>               ExpressionTypeCast
                0          Index
                   1       Index
                     0     Literal                                                                                                        */
	[[0; 1]; 1],                                                                                                                          /*
    [[0;•1];•1]    SizedArrayLiteral
     [0;•1]        SizedArrayLiteral
      0            Literal
         1         Literal
             1     Literal                                                                                                                */
	std::ptr::null::<usize>().is_null(),                                                                                                  /*
    std::ptr::null::<usize>().is_null()    CallExpression
    std::ptr::null::<usize>()              CallExpression
    std::ptr::null                         ExpressionPath
    std::ptr                               ExpressionPath                                                                                 */
	&ss.1,                                                                                                                                /*
    &ss.1    ReferenceExpression
     ss.1    MemberExpression
        1    Index                                                                                                                        */
	&raw mut foo.x.0..1,                                                                                                                  /*
    &raw•mut•foo.x.0..1    RangeLiteral
    &raw•mut•foo.x.0       RawReferenceExpression
             foo.x.0       MemberExpression
             foo.x         MemberExpression
                   0       Index
                      1    Literal                                                                                                        */
	&mut **d,                                                                                                                             /*
    &mut•**d    ReferenceExpression
         **d    DereferenceExpression
          *d    DereferenceExpression                                                                                                     */
	[12, 34][0 + 1],                                                                                                                      /*
    [12,•34][0•+•1]    MemberExpression
    [12,•34]           ArrayLiteral
     12                Literal
         34            Literal
             0•+•1     OperationExpression
             0         Literal
                 1     Literal                                                                                                            */
	g(f())(()),                                                                                                                           /*
    g(f())(())    CallExpression
    g(f())        CallExpression
      f()         CallExpression
           ()     TupleLiteral                                                                                                            */
];                                                                                                                                        /*
];    </ExpressionStatement>
]     </ArrayLiteral>                                                                                                                     */


fn f() { s.e() .f(E::s) .f(|f| f.a()) .f(R::e) .e(|a| *a >= q) .d() }                                                                     /*
fn•f()•{•s.e()•.f(E::s)•.f(|f|•f.a())•.f(R::e)•.e(|a|•*a•>=•q)•.d()•}    FunctionDeclaration
         s.e()•.f(E::s)•.f(|f|•f.a())•.f(R::e)•.e(|a|•*a•>=•q)•.d()      ExpressionStatement, CallExpression
         s.e()•.f(E::s)•.f(|f|•f.a())•.f(R::e)•.e(|a|•*a•>=•q)           CallExpression
         s.e()•.f(E::s)•.f(|f|•f.a())•.f(R::e)                           CallExpression
         s.e()•.f(E::s)•.f(|f|•f.a())                                    CallExpression
         s.e()•.f(E::s)                                                  CallExpression
         s.e()                                                           CallExpression
                  E::s                                                   ExpressionPath
                           |f|•f.a()                                     ClosureFunctionExpression
                            f                                            ClosureFunctionParameterDeclaration
                               f.a()                                     CallExpression
                                         R::e                            ExpressionPath
                                                  |a|•*a•>=•q            ClosureFunctionExpression
                                                   a                     ClosureFunctionParameterDeclaration
                                                      *a•>=•q            ComparisonExpression
                                                      *a                 DereferenceExpression                                            */
fn f() { let q = E { r: f![] }; Q(Q(q)).s(|d| q.i(|mut d| { e.z(0); f.G = e; r }) ); }                                                    /*
fn•f()•{•let•q•=•E•{•r:•f![]•};•Q(Q(q)).s(|d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})•);•}    FunctionDeclaration
         let•q•=•E•{•r:•f![]•};                                                           LetVariableDeclaration
                 E•{•r:•f![]•}                                                            StructLiteral
                     r:•f![]                                                              StructLiteralProperty
                        f![]                                                              MacroInvocation
                                Q(Q(q)).s(|d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})•);      ExpressionStatement
                                Q(Q(q)).s(|d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})•)       CallExpression
                                Q(Q(q))                                                   CallExpression
                                  Q(q)                                                    CallExpression
                                          |d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})         ClosureFunctionExpression
                                           d                                              ClosureFunctionParameterDeclaration
                                              q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})         CallExpression
                                                  |mut•d|•{•e.z(0);•f.G•=•e;•r•}          ClosureFunctionExpression
                                                   mut•d                                  ClosureFunctionParameterDeclaration, PatternVariableDeclaration
                                                          {•e.z(0);•f.G•=•e;•r•}          BlockExpression
                                                            e.z(0);                       ExpressionStatement
                                                            e.z(0)                        CallExpression
                                                                0                         Literal
                                                                    f.G•=•e;              ExpressionStatement
                                                                    f.G•=•e               ReassignmentExpression
                                                                    f.G                   MemberExpression
                                                                             r            ExpressionStatement                             */
pub fn public_expr(_: [u8; a(0).0]) {}                                                                                                    /*
pub•fn•public_expr(_:•[u8;•a(0).0])•{}    FunctionDeclaration
pub                                       PubSpecifier
                   _:•[u8;•a(0).0]        FunctionParameterDeclaration
                   _                      WildcardPattern
                      [u8;•a(0).0]        TypeSizedArray
                           a(0).0         MemberExpression
                           a(0)           CallExpression
                             0            Literal
                                0         Index                                                                                           */
pub fn f() { return ::f(); }                                                                                                              /*
pub•fn•f()•{•return•::f();•}    FunctionDeclaration
pub                             PubSpecifier
             return•::f();      ExpressionStatement
             return•::f()       ReturnExpression
                    ::f()       CallExpression
                    ::f         ExpressionPath                                                                                            */
fn f() -> isize {                                                                                                                         /*
fn•f()•->•isize•{↲    <FunctionDeclaration>                                                                                               */
    (return 1, return 2)                                                                                                                  /*
    (return•1,•return•2)    ExpressionStatement, TupleLiteral
     return•1               ReturnExpression
            1               Literal
               return•2     ReturnExpression
                      2     Literal                                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
fn f(x: Box<isize>) -> Box<(Box<isize>,Box<isize>)> { box (x, x) }                                                                        /*
fn•f(x:•Box<isize>)•->•Box<(Box<isize>,Box<isize>)>•{•box•(x,•x)•}    FunctionDeclaration
     x:•Box<isize>                                                    FunctionParameterDeclaration
        Box<isize>                                                    TypeCall
                       Box<(Box<isize>,Box<isize>)>                   TypeCall
                           (Box<isize>,Box<isize>)                    TypeTuple
                            Box<isize>                                TypeCall
                                       Box<isize>                     TypeCall
                                                      box•(x,•x)      ExpressionStatement, BoxExpression
                                                          (x,•x)      TupleLiteral                                                        */
fn f<F>(f: F) -> isize where F: FnOnce(isize) -> isize {}                                                                                 /*
fn•f<F>(f:•F)•->•isize•where•F:•FnOnce(isize)•->•isize•{}    FunctionDeclaration
     F                                                       GenericTypeParameterDeclaration
        f:•F                                                 FunctionParameterDeclaration
                             F:•FnOnce(isize)•->•isize       WhereTypeBoundDeclaration
                                FnOnce(isize)•->•isize       TypeTraitBound, TypeFunction                                                 */
fn f() { if (return) { } }                                                                                                                /*
fn•f()•{•if•(return)•{•}•}    FunctionDeclaration
         if•(return)•{•}      ExpressionStatement, IfBlockExpression
             return           ReturnExpression                                                                                            */
fn f() { b! { } c }                                                                                                                       /*
fn•f()•{•b!•{•}•c•}    FunctionDeclaration
         b!•{•}        ExpressionStatement, MacroInvocation
                c      ExpressionStatement                                                                                                */
fn f<T: ToString>(arg: T) -> String {                                                                                                     /*
fn•f<T:•ToString>(arg:•T)•->•String•{↲    <FunctionDeclaration>
     T:•ToString                          GenericTypeParameterDeclaration
        ToString                          TypeTraitBound
                  arg:•T                  FunctionParameterDeclaration                                                                    */
    return <T as ToString>::to_string(&arg);                                                                                              /*
    return•<T•as•ToString>::to_string(&arg);    ExpressionStatement
    return•<T•as•ToString>::to_string(&arg)     ReturnExpression
           <T•as•ToString>::to_string(&arg)     CallExpression
           <T•as•ToString>::to_string           ExpressionPath
           <T•as•ToString>                      ExpressionTypeSelector
                                      &arg      ReferenceExpression                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
fn f<A:Clone + 'static>(a: A, b: u16) -> Box<dyn Invokable<A>+'static> {                                                                  /*
fn•f<A:Clone•+•'static>(a:•A,•b:•u16)•->•Box<dyn•Invokable<A>+'static>•{↲    <FunctionDeclaration>
     A:Clone•+•'static                                                       GenericTypeParameterDeclaration
       Clone                                                                 TypeTraitBound
               'static                                                       LtStatic
                        a:•A                                                 FunctionParameterDeclaration
                              b:•u16                                         FunctionParameterDeclaration
                                         Box<dyn•Invokable<A>+'static>       TypeCall
                                             dyn•Invokable<A>+'static        TypeDynBounds
                                                 Invokable<A>                TypeTraitBound, TypeCall
                                                              'static        LtStatic                                                     */
    box Invoker { a: a, b: b, } as Box<dyn Invokable<A>+'static>                                                                          /*
    box•Invoker•{•a:•a,•b:•b,•}•as•Box<dyn•Invokable<A>+'static>    ExpressionStatement, ExpressionAsTypeCast
    box•Invoker•{•a:•a,•b:•b,•}                                     BoxExpression
        Invoker•{•a:•a,•b:•b,•}                                     StructLiteral
                  a:•a                                              StructLiteralProperty
                        b:•b                                        StructLiteralProperty
                                   Box<dyn•Invokable<A>+'static>    TypeCall
                                       dyn•Invokable<A>+'static     TypeDynBounds
                                           Invokable<A>             TypeTraitBound, TypeCall
                                                        'static     LtStatic                                                              */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
fn f() { (return 1, return 2) }                                                                                                           /*
fn•f()•{•(return•1,•return•2)•}    FunctionDeclaration
         (return•1,•return•2)      ExpressionStatement, TupleLiteral
          return•1                 ReturnExpression
                 1                 Literal
                    return•2       ReturnExpression
                           2       Literal                                                                                                */
pub trait Foo: Iterator<Item=<Self as Foo>::Key>{}                                                                                        /*
pub•trait•Foo:•Iterator<Item=<Self•as•Foo>::Key>{}    TraitDeclaration
pub                                                   PubSpecifier
               Iterator<Item=<Self•as•Foo>::Key>      TypeTraitBound, TypeCall
                        Item=<Self•as•Foo>::Key       TypeCallNamedArgument
                             <Self•as•Foo>::Key       TypePath
                             <Self•as•Foo>            ExpressionTypeSelector                                                              */
fn f() {::m!(S, x);}                                                                                                                      /*
fn•f()•{::m!(S,•x);}    FunctionDeclaration
        ::m!(S,•x);     ExpressionStatement
        ::m!(S,•x)      MacroInvocation
        ::m             ExpressionPath
              ,         PunctuationToken                                                                                                  */

// Discarded Nodes: 28
// Parsed Nodes: 1736
// state_rollbacks: 15
// Total '.charCodeAt()' calls: 7447 (35% re-reads)
// Unnecessary 'skip_whitespace()' calls: 1016
// source: "../../samples/expressions/expr.rs"