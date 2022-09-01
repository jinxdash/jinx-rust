fn main() {                                                                                                                               /*
fn•main()•{↲    <Program>
fn•main()•{↲    <Program.ast{dk: "None"}>
fn•main()•{↲    <FunctionDeclaration>
       ()       FunctionDeclaration.parameters{dk: "()"}
          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                      */
	let a = move async { };                                                                                                               /*
	let•a•=•move•async•{•};    LetVariableDeclaration
	        move•async•{•}     BlockExpression{async, move}
	                   {•}     BlockExpression.body{dk: "{}"}                                                                             */
	9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999                                        /*
	9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999↲    <ExpressionStatement{semi}>
	9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999     Literal{kind: Integer}             */
    // boop
    //•boop    Comment{line}
		;                                                                                                                                 /*
      ╚╚;    </ExpressionStatement>                                                                                                       */
	vec! { 1, 2, 3 }.len();                                                                                                               /*
	vec!•{•1,•2,•3•}.len();    ExpressionStatement{semi}
	vec!•{•1,•2,•3•}.len()     CallExpression
	vec!•{•1,•2,•3•}           MacroInvocation
	     {•1,•2,•3•}           MacroInvocation.segments{dk: "{}"}
	       1                   Literal{kind: Integer}
	        ,                  PunctuationToken{tk: ","}
	          2                Literal{kind: Integer}
	           ,               PunctuationToken{tk: ","}
	             3             Literal{kind: Integer}
	                    ()     CallExpression.arguments{dk: "()"}                                                                         */
    write! { vec![], "" }?;                                                                                                               /*
    write!•{•vec![],•""•}?;    ExpressionStatement{semi}
    write!•{•vec![],•""•}?     UnwrapExpression
    write!•{•vec![],•""•}      MacroInvocation
           {•vec![],•""•}      MacroInvocation.segments{dk: "{}"}
                !              PunctuationToken{tk: "!"}
                 []            DelimGroup
                   ,           PunctuationToken{tk: ","}
                     ""        Literal{kind: String}                                                                                      */
    println!{""}[0];                                                                                                                      /*
    println!{""}        ExpressionStatement{!semi}, MacroInvocation
            {""}        MacroInvocation.segments{dk: "{}"}
             ""         Literal{kind: String}
                [0];    ExpressionStatement{semi}
                [0]     ArrayLiteral
                 0      Literal{kind: Integer}                                                                                            */
	b.a;                                                                                                                                  /*
	b.a;    ExpressionStatement{semi}
	b.a     MemberExpression{!computed}                                                                                                   */
	*foo += 1;                                                                                                                            /*
	*foo•+=•1;    ExpressionStatement{semi}
	*foo•+=•1     ReassignmentOperationExpression{tk: "+="}
	*foo          DereferenceExpression
	        1     Literal{kind: Integer}                                                                                                  */
	let &_ = bar;                                                                                                                         /*
	let•&_•=•bar;    LetVariableDeclaration
	    &_           ReferencePattern{!mut}
	     _           WildcardPattern                                                                                                      */
	let &mut _ = foo;                                                                                                                     /*
	let•&mut•_•=•foo;    LetVariableDeclaration
	    &mut•_           ReferencePattern{mut}
	         _           WildcardPattern                                                                                                  */
	if let _ = 0 {}                                                                                                                       /*
	if•let•_•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•_•=•0       LetScrutinee
	       _           WildcardPattern
	           0       Literal{kind: Integer}
	             {}    IfBlockExpression.body{dk: "{}"}                                                                                   */
    while let _ = 0 {}                                                                                                                    /*
    while•let•_•=•0•{}    ExpressionStatement{!semi}, WhileBlockExpression
          let•_•=•0       LetScrutinee
              _           WildcardPattern
                  0       Literal{kind: Integer}
                    {}    WhileBlockExpression.body{dk: "{}"}                                                                             */
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
	       &[u8]            TypeReference{!mut}
	        [u8]            TypeSlice
	               &[0]     ReferenceExpression{!mut}
	                [0]     ArrayLiteral
	                 0      Literal{kind: Integer}                                                                                        */
	let Foo { a, ref b, mut c, x: y, z: z } = foo;                                                                                        /*
	let•Foo•{•a,•ref•b,•mut•c,•x:•y,•z:•z•}•=•foo;    LetVariableDeclaration
	    Foo•{•a,•ref•b,•mut•c,•x:•y,•z:•z•}           StructPattern
	        {•a,•ref•b,•mut•c,•x:•y,•z:•z•}           StructPattern.properties{dk: "{}"}
	          a                                       StructPatternPropertyShorthand{!box, !ref, !mut}
	             ref•b                                StructPatternPropertyShorthand{!box, ref, !mut}
	                    mut•c                         StructPatternPropertyShorthand{!box, !ref, mut}
	                           x:•y                   StructPatternPropertyDestructured
	                                 z:•z             StructPatternPropertyDestructured                                                   */
	let x = &raw const y;                                                                                                                 /*
	let•x•=•&raw•const•y;    LetVariableDeclaration
	        &raw•const•y     RawReferenceExpression{kind: "const"}                                                                        */
	let x = &raw mut y;                                                                                                                   /*
	let•x•=•&raw•mut•y;    LetVariableDeclaration
	        &raw•mut•y     RawReferenceExpression{kind: "mut"}                                                                            */
	a::<Box<isize>, _>(box 1);                                                                                                            /*
	a::<Box<isize>,•_>(box•1);    ExpressionStatement{semi}
	a::<Box<isize>,•_>(box•1)     CallExpression
	   <Box<isize>,•_>            CallExpression.typeArguments{dk: "<>"}
	    Box<isize>                TypeCall
	       <isize>                TypeCall.typeArguments{dk: "<>"}
	                _             TypeInferred
	                  (box•1)     CallExpression.arguments{dk: "()"}
	                   box•1      BoxExpression
	                       1      Literal{kind: Integer}                                                                                  */
	if &raw const one == &raw mut one {}                                                                                                  /*
	if•&raw•const•one•==•&raw•mut•one•{}    ExpressionStatement{!semi}, IfBlockExpression
	   &raw•const•one•==•&raw•mut•one       ComparisonExpression{tk: "=="}
	   &raw•const•one                       RawReferenceExpression{kind: "const"}
	                     &raw•mut•one       RawReferenceExpression{kind: "mut"}
	                                  {}    IfBlockExpression.body{dk: "{}"}                                                              */
	let _x = if false { 0 } else if true { panic!() } else { 10 };                                                                        /*
	let•_x•=•if•false•{•0•}•else•if•true•{•panic!()•}•else•{•10•};    LetVariableDeclaration
	         if•false•{•0•}•else•if•true•{•panic!()•}•else•{•10•}     IfBlockExpression
	            false                                                 Literal{kind: False}
	                  {•0•}                                           IfBlockExpression.body{dk: "{}"}
	                    0                                             ExpressionStatement{!semi}, Literal{kind: Integer}
	                             if•true•{•panic!()•}•else•{•10•}     IfBlockExpression
	                                true                              Literal{kind: True}
	                                     {•panic!()•}                 IfBlockExpression.body{dk: "{}"}
	                                       panic!()                   ExpressionStatement{!semi}, MacroInvocation
	                                             ()                   MacroInvocation.segments{dk: "()"}
	                                                       {•10•}     BlockExpression
	                                                         10       ExpressionStatement{!semi}, Literal{kind: Integer}                  */
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
	             {•name:•0•}     StructLiteral.properties{dk: "{}"}
	               name:•0       StructLiteralProperty
	                     0       Literal{kind: Integer}                                                                                   */
	let b1 = &mut *b;                                                                                                                     /*
	let•b1•=•&mut•*b;    LetVariableDeclaration
	         &mut•*b     ReferenceExpression{mut}
	              *b     DereferenceExpression                                                                                            */
	let mut x: Box<_> = box 3;                                                                                                            /*
	let•mut•x:•Box<_>•=•box•3;    LetVariableDeclaration
	    mut•x                     PatternVariableDeclaration{!ref, mut}
	           Box<_>             TypeCall
	              <_>             TypeCall.typeArguments{dk: "<>"}
	               _              TypeInferred
	                    box•3     BoxExpression
	                        3     Literal{kind: Integer}                                                                                  */
	let x: (Box<_>,) = (box 1,);                                                                                                          /*
	let•x:•(Box<_>,)•=•(box•1,);    LetVariableDeclaration
	       (Box<_>,)                TypeTuple
	        Box<_>                  TypeCall
	           <_>                  TypeCall.typeArguments{dk: "<>"}
	            _                   TypeInferred
	                   (box•1,)     TupleLiteral
	                    box•1       BoxExpression
	                        1       Literal{kind: Integer}                                                                                */
	let &mut ref x = b;                                                                                                                   /*
	let•&mut•ref•x•=•b;    LetVariableDeclaration
	    &mut•ref•x         ReferencePattern{mut}
	         ref•x         PatternVariableDeclaration{ref, !mut}                                                                          */
	let &mut mut x = b;                                                                                                                   /*
	let•&mut•mut•x•=•b;    LetVariableDeclaration
	    &mut•mut•x         ReferencePattern{mut}
	         mut•x         PatternVariableDeclaration{!ref, mut}                                                                          */
	let ref mut y = b;                                                                                                                    /*
	let•ref•mut•y•=•b;    LetVariableDeclaration
	    ref•mut•y         PatternVariableDeclaration{ref, mut}                                                                            */
	let (a, b, c, d);                                                                                                                     /*
	let•(a,•b,•c,•d);    LetVariableDeclaration
	    (a,•b,•c,•d)     TuplePattern                                                                                                     */
	let (mut c, mut d);                                                                                                                   /*
	let•(mut•c,•mut•d);    LetVariableDeclaration
	    (mut•c,•mut•d)     TuplePattern
	     mut•c             PatternVariableDeclaration{!ref, mut}
	            mut•d      PatternVariableDeclaration{!ref, mut}                                                                          */
	let s = S { x: 3, y: 4 };                                                                                                             /*
	let•s•=•S•{•x:•3,•y:•4•};    LetVariableDeclaration
	        S•{•x:•3,•y:•4•}     StructLiteral
	          {•x:•3,•y:•4•}     StructLiteral.properties{dk: "{}"}
	            x:•3             StructLiteralProperty
	               3             Literal{kind: Integer}
	                  y:•4       StructLiteralProperty
	                     4       Literal{kind: Integer}                                                                                   */
	let mut r = R {c: Box::new(f)};                                                                                                       /*
	let•mut•r•=•R•{c:•Box::new(f)};    LetVariableDeclaration
	    mut•r                          PatternVariableDeclaration{!ref, mut}
	            R•{c:•Box::new(f)}     StructLiteral
	              {c:•Box::new(f)}     StructLiteral.properties{dk: "{}"}
	               c:•Box::new(f)      StructLiteralProperty
	                  Box::new(f)      CallExpression
	                  Box::new         ExpressionPath
	                          (f)      CallExpression.arguments{dk: "()"}                                                                 */
	let _:         & usize =     &1;                                                                                                      /*
	let•_:•••••••••&•usize•=•••••&1;    LetVariableDeclaration
	    _                               WildcardPattern
	               &•usize              TypeReference{!mut}
	                             &1     ReferenceExpression{!mut}
	                              1     Literal{kind: Integer}                                                                            */
    let _:       & & usize =    &&1;                                                                                                      /*
    let•_:•••••••&•&•usize•=••••&&1;    LetVariableDeclaration
        _                               WildcardPattern
                 &•&•usize              TypeReference{!mut}
                   &•usize              TypeReference{!mut}
                                &&1     ReferenceExpression{!mut}
                                 &1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let _:     & & & usize =   &&&1;                                                                                                      /*
    let•_:•••••&•&•&•usize•=•••&&&1;    LetVariableDeclaration
        _                               WildcardPattern
               &•&•&•usize              TypeReference{!mut}
                 &•&•usize              TypeReference{!mut}
                   &•usize              TypeReference{!mut}
                               &&&1     ReferenceExpression{!mut}
                                &&1     ReferenceExpression{!mut}
                                 &1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let _:     & & & usize =  & &&1;                                                                                                      /*
    let•_:•••••&•&•&•usize•=••&•&&1;    LetVariableDeclaration
        _                               WildcardPattern
               &•&•&•usize              TypeReference{!mut}
                 &•&•usize              TypeReference{!mut}
                   &•usize              TypeReference{!mut}
                              &•&&1     ReferenceExpression{!mut}
                                &&1     ReferenceExpression{!mut}
                                 &1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let _:   & & & & usize =  &&&&1;                                                                                                      /*
    let•_:•••&•&•&•&•usize•=••&&&&1;    LetVariableDeclaration
        _                               WildcardPattern
             &•&•&•&•usize              TypeReference{!mut}
               &•&•&•usize              TypeReference{!mut}
                 &•&•usize              TypeReference{!mut}
                   &•usize              TypeReference{!mut}
                              &&&&1     ReferenceExpression{!mut}
                               &&&1     ReferenceExpression{!mut}
                                &&1     ReferenceExpression{!mut}
                                 &1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let _:   & & & & usize = & &&&1;                                                                                                      /*
    let•_:•••&•&•&•&•usize•=•&•&&&1;    LetVariableDeclaration
        _                               WildcardPattern
             &•&•&•&•usize              TypeReference{!mut}
               &•&•&•usize              TypeReference{!mut}
                 &•&•usize              TypeReference{!mut}
                   &•usize              TypeReference{!mut}
                             &•&&&1     ReferenceExpression{!mut}
                               &&&1     ReferenceExpression{!mut}
                                &&1     ReferenceExpression{!mut}
                                 &1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let _: & & & & & usize = &&&&&1;                                                                                                      /*
    let•_:•&•&•&•&•&•usize•=•&&&&&1;    LetVariableDeclaration
        _                               WildcardPattern
           &•&•&•&•&•usize              TypeReference{!mut}
             &•&•&•&•usize              TypeReference{!mut}
               &•&•&•usize              TypeReference{!mut}
                 &•&•usize              TypeReference{!mut}
                   &•usize              TypeReference{!mut}
                             &&&&&1     ReferenceExpression{!mut}
                              &&&&1     ReferenceExpression{!mut}
                               &&&1     ReferenceExpression{!mut}
                                &&1     ReferenceExpression{!mut}
                                 &1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
	let x: T = **item;                                                                                                                    /*
	let•x:•T•=•**item;    LetVariableDeclaration
	           **item     DereferenceExpression
	            *item     DereferenceExpression                                                                                           */
	let &x = &(&1isize as &dyn T);                                                                                                        /*
	let•&x•=•&(&1isize•as•&dyn•T);    LetVariableDeclaration
	    &x                            ReferencePattern{!mut}
	         &(&1isize•as•&dyn•T)     ReferenceExpression{!mut}
	           &1isize•as•&dyn•T      ExpressionAsTypeCast
	           &1isize                ReferenceExpression{!mut}
	            1isize                Literal{kind: Integer}
	             isize                Identifier
	                      &dyn•T      TypeReference{!mut}
	                       dyn•T      TypeDynBounds{dyn}
	                           T      TypeTraitBound{!maybeConst, !optional}                                                              */
    let &x = &&(&1isize as &dyn T);                                                                                                       /*
    let•&x•=•&&(&1isize•as•&dyn•T);    LetVariableDeclaration
        &x                             ReferencePattern{!mut}
             &&(&1isize•as•&dyn•T)     ReferenceExpression{!mut}
              &(&1isize•as•&dyn•T)     ReferenceExpression{!mut}
                &1isize•as•&dyn•T      ExpressionAsTypeCast
                &1isize                ReferenceExpression{!mut}
                 1isize                Literal{kind: Integer}
                  isize                Identifier
                           &dyn•T      TypeReference{!mut}
                            dyn•T      TypeDynBounds{dyn}
                                T      TypeTraitBound{!maybeConst, !optional}                                                             */
    let &&x = &&(&1isize as &dyn T);                                                                                                      /*
    let•&&x•=•&&(&1isize•as•&dyn•T);    LetVariableDeclaration
        &&x                             ReferencePattern{!mut}
         &x                             ReferencePattern{!mut}
              &&(&1isize•as•&dyn•T)     ReferenceExpression{!mut}
               &(&1isize•as•&dyn•T)     ReferenceExpression{!mut}
                 &1isize•as•&dyn•T      ExpressionAsTypeCast
                 &1isize                ReferenceExpression{!mut}
                  1isize                Literal{kind: Integer}
                   isize                Identifier
                            &dyn•T      TypeReference{!mut}
                             dyn•T      TypeDynBounds{dyn}
                                 T      TypeTraitBound{!maybeConst, !optional}                                                            */
	let &x = &1isize as &dyn T;                                                                                                           /*
	let•&x•=•&1isize•as•&dyn•T;    LetVariableDeclaration
	    &x                         ReferencePattern{!mut}
	         &1isize•as•&dyn•T     ExpressionAsTypeCast
	         &1isize               ReferenceExpression{!mut}
	          1isize               Literal{kind: Integer}
	           isize               Identifier
	                    &dyn•T     TypeReference{!mut}
	                     dyn•T     TypeDynBounds{dyn}
	                         T     TypeTraitBound{!maybeConst, !optional}                                                                 */
	let &&x = &1isize as &dyn T;                                                                                                          /*
	let•&&x•=•&1isize•as•&dyn•T;    LetVariableDeclaration
	    &&x                         ReferencePattern{!mut}
	     &x                         ReferencePattern{!mut}
	          &1isize•as•&dyn•T     ExpressionAsTypeCast
	          &1isize               ReferenceExpression{!mut}
	           1isize               Literal{kind: Integer}
	            isize               Identifier
	                     &dyn•T     TypeReference{!mut}
	                      dyn•T     TypeDynBounds{dyn}
	                          T     TypeTraitBound{!maybeConst, !optional}                                                                */
	let &&x = &(&1isize as &dyn T);                                                                                                       /*
	let•&&x•=•&(&1isize•as•&dyn•T);    LetVariableDeclaration
	    &&x                            ReferencePattern{!mut}
	     &x                            ReferencePattern{!mut}
	          &(&1isize•as•&dyn•T)     ReferenceExpression{!mut}
	            &1isize•as•&dyn•T      ExpressionAsTypeCast
	            &1isize                ReferenceExpression{!mut}
	             1isize                Literal{kind: Integer}
	              isize                Identifier
	                       &dyn•T      TypeReference{!mut}
	                        dyn•T      TypeDynBounds{dyn}
	                            T      TypeTraitBound{!maybeConst, !optional}                                                             */
	let &&&x = &(&1isize as &dyn T);                                                                                                      /*
	let•&&&x•=•&(&1isize•as•&dyn•T);    LetVariableDeclaration
	    &&&x                            ReferencePattern{!mut}
	     &&x                            ReferencePattern{!mut}
	      &x                            ReferencePattern{!mut}
	           &(&1isize•as•&dyn•T)     ReferenceExpression{!mut}
	             &1isize•as•&dyn•T      ExpressionAsTypeCast
	             &1isize                ReferenceExpression{!mut}
	              1isize                Literal{kind: Integer}
	               isize                Identifier
	                        &dyn•T      TypeReference{!mut}
	                         dyn•T      TypeDynBounds{dyn}
	                             T      TypeTraitBound{!maybeConst, !optional}                                                            */
	let box x = box 1isize as Box<dyn T>;                                                                                                 /*
	let•box•x•=•box•1isize•as•Box<dyn•T>;    LetVariableDeclaration
	    box•x                                BoxPattern
	            box•1isize•as•Box<dyn•T>     ExpressionAsTypeCast
	            box•1isize                   BoxExpression
	                1isize                   Literal{kind: Integer}
	                 isize                   Identifier
	                          Box<dyn•T>     TypeCall
	                             <dyn•T>     TypeCall.typeArguments{dk: "<>"}
	                              dyn•T      TypeDynBounds{dyn}
	                                  T      TypeTraitBound{!maybeConst, !optional}                                                       */
	let box box x = box 1isize as Box<dyn T>;                                                                                             /*
	let•box•box•x•=•box•1isize•as•Box<dyn•T>;    LetVariableDeclaration
	    box•box•x                                BoxPattern
	        box•x                                BoxPattern
	                box•1isize•as•Box<dyn•T>     ExpressionAsTypeCast
	                box•1isize                   BoxExpression
	                    1isize                   Literal{kind: Integer}
	                     isize                   Identifier
	                              Box<dyn•T>     TypeCall
	                                 <dyn•T>     TypeCall.typeArguments{dk: "<>"}
	                                  dyn•T      TypeDynBounds{dyn}
	                                      T      TypeTraitBound{!maybeConst, !optional}                                                   */
	let a = (b[0] as u64) << 0 | (b[1] as u64) << 8 | (b[2] as u64) << 16 | (b[3] as u64) << 24;                                          /*
	let•a•=•(b[0]•as•u64)•<<•0•|•(b[1]•as•u64)•<<•8•|•(b[2]•as•u64)•<<•16•|•(b[3]•as•u64)•<<•24;    LetVariableDeclaration
	        (b[0]•as•u64)•<<•0•|•(b[1]•as•u64)•<<•8•|•(b[2]•as•u64)•<<•16•|•(b[3]•as•u64)•<<•24     OperationExpression{tk: "|"}
	        (b[0]•as•u64)•<<•0•|•(b[1]•as•u64)•<<•8•|•(b[2]•as•u64)•<<•16                           OperationExpression{tk: "|"}
	        (b[0]•as•u64)•<<•0•|•(b[1]•as•u64)•<<•8                                                 OperationExpression{tk: "|"}
	        (b[0]•as•u64)•<<•0                                                                      OperationExpression{tk: "<<"}
	         b[0]•as•u64                                                                            ExpressionAsTypeCast
	         b[0]                                                                                   MemberExpression{computed}
	           0                                                                                    Literal{kind: Integer}
	                         0                                                                      Literal{kind: Integer}
	                             (b[1]•as•u64)•<<•8                                                 OperationExpression{tk: "<<"}
	                              b[1]•as•u64                                                       ExpressionAsTypeCast
	                              b[1]                                                              MemberExpression{computed}
	                                1                                                               Literal{kind: Integer}
	                                              8                                                 Literal{kind: Integer}
	                                                  (b[2]•as•u64)•<<•16                           OperationExpression{tk: "<<"}
	                                                   b[2]•as•u64                                  ExpressionAsTypeCast
	                                                   b[2]                                         MemberExpression{computed}
	                                                     2                                          Literal{kind: Integer}
	                                                                   16                           Literal{kind: Integer}
	                                                                        (b[3]•as•u64)•<<•24     OperationExpression{tk: "<<"}
	                                                                         b[3]•as•u64            ExpressionAsTypeCast
	                                                                         b[3]                   MemberExpression{computed}
	                                                                           3                    Literal{kind: Integer}
	                                                                                         24     Literal{kind: Integer}                */
	let a = if let Err(b) = c { d } else { e ! ("") };                                                                                    /*
	let•a•=•if•let•Err(b)•=•c•{•d•}•else•{•e•!•("")•};    LetVariableDeclaration
	        if•let•Err(b)•=•c•{•d•}•else•{•e•!•("")•}     IfBlockExpression
	           let•Err(b)•=•c                             LetScrutinee
	               Err(b)                                 TuplePattern
	                  (b)                                 TuplePattern.items{dk: "()"}
	                          {•d•}                       IfBlockExpression.body{dk: "{}"}
	                            d                         ExpressionStatement{!semi}
	                                     {•e•!•("")•}     BlockExpression
	                                       e•!•("")       ExpressionStatement{!semi}, MacroInvocation
	                                           ("")       MacroInvocation.segments{dk: "()"}
	                                            ""        Literal{kind: String}                                                           */
	let mut n3 = N3 { n: N2(N1 { n: N0 { x: Box::new(42) } }) };                                                                          /*
	let•mut•n3•=•N3•{•n:•N2(N1•{•n:•N0•{•x:•Box::new(42)•}•})•};    LetVariableDeclaration
	    mut•n3                                                      PatternVariableDeclaration{!ref, mut}
	             N3•{•n:•N2(N1•{•n:•N0•{•x:•Box::new(42)•}•})•}     StructLiteral
	                {•n:•N2(N1•{•n:•N0•{•x:•Box::new(42)•}•})•}     StructLiteral.properties{dk: "{}"}
	                  n:•N2(N1•{•n:•N0•{•x:•Box::new(42)•}•})       StructLiteralProperty
	                     N2(N1•{•n:•N0•{•x:•Box::new(42)•}•})       CallExpression
	                       (N1•{•n:•N0•{•x:•Box::new(42)•}•})       CallExpression.arguments{dk: "()"}
	                        N1•{•n:•N0•{•x:•Box::new(42)•}•}        StructLiteral
	                           {•n:•N0•{•x:•Box::new(42)•}•}        StructLiteral.properties{dk: "{}"}
	                             n:•N0•{•x:•Box::new(42)•}          StructLiteralProperty
	                                N0•{•x:•Box::new(42)•}          StructLiteral
	                                   {•x:•Box::new(42)•}          StructLiteral.properties{dk: "{}"}
	                                     x:•Box::new(42)            StructLiteralProperty
	                                        Box::new(42)            CallExpression
	                                        Box::new                ExpressionPath
	                                                (42)            CallExpression.arguments{dk: "()"}
	                                                 42             Literal{kind: Integer}                                                */
    n3.n.0.n.x = n3.n.0.n.x;                                                                                                              /*
    n3.n.0.n.x•=•n3.n.0.n.x;    ExpressionStatement{semi}
    n3.n.0.n.x•=•n3.n.0.n.x     ReassignmentExpression{tk: "="}
    n3.n.0.n.x                  MemberExpression{!computed}
    n3.n.0.n                    MemberExpression{!computed}
    n3.n.0                      MemberExpression{!computed}
    n3.n                        MemberExpression{!computed}
         0                      Index
                 n3.n.0.n.x     MemberExpression{!computed}
                 n3.n.0.n       MemberExpression{!computed}
                 n3.n.0         MemberExpression{!computed}
                 n3.n           MemberExpression{!computed}
                      0         Index                                                                                                     */
    let mut t = (1, ((2, 3, (4, 5)),));                                                                                                   /*
    let•mut•t•=•(1,•((2,•3,•(4,•5)),));    LetVariableDeclaration
        mut•t                              PatternVariableDeclaration{!ref, mut}
                (1,•((2,•3,•(4,•5)),))     TupleLiteral
                 1                         Literal{kind: Integer}
                    ((2,•3,•(4,•5)),)      TupleLiteral
                     (2,•3,•(4,•5))        TupleLiteral
                      2                    Literal{kind: Integer}
                         3                 Literal{kind: Integer}
                            (4,•5)         TupleLiteral
                             4             Literal{kind: Integer}
                                5          Literal{kind: Integer}                                                                         */
    t.1.0.2.1 = t.1.0.2.1;                                                                                                                /*
    t.1.0.2.1•=•t.1.0.2.1;    ExpressionStatement{semi}
    t.1.0.2.1•=•t.1.0.2.1     ReassignmentExpression{tk: "="}
    t.1.0.2.1                 MemberExpression{!computed}
    t.1.0.2                   MemberExpression{!computed}
    t.1.0                     MemberExpression{!computed}
    t.1                       MemberExpression{!computed}
      1                       Index
        0                     Index
          2                   Index
            1                 Index
                t.1.0.2.1     MemberExpression{!computed}
                t.1.0.2       MemberExpression{!computed}
                t.1.0         MemberExpression{!computed}
                t.1           MemberExpression{!computed}
                  1           Index
                    0         Index
                      2       Index
                        1     Index                                                                                                       */
	let mut a: A<(), &mut i32> = try { 1 };                                                                                               /*
	let•mut•a:•A<(),•&mut•i32>•=•try•{•1•};    LetVariableDeclaration
	    mut•a                                  PatternVariableDeclaration{!ref, mut}
	           A<(),•&mut•i32>                 TypeCall
	            <(),•&mut•i32>                 TypeCall.typeArguments{dk: "<>"}
	             ()                            TypeTuple
	                 &mut•i32                  TypeReference{mut}
	                             try•{•1•}     TryBlockExpression
	                                 {•1•}     TryBlockExpression.body{dk: "{}"}
	                                   1       ExpressionStatement{!semi}, Literal{kind: Integer}                                         */
	let _ = &mut *s.0.borrow_mut();                                                                                                       /*
	let•_•=•&mut•*s.0.borrow_mut();    LetVariableDeclaration
	    _                              WildcardPattern
	        &mut•*s.0.borrow_mut()     ReferenceExpression{mut}
	             *s.0.borrow_mut()     DereferenceExpression
	              s.0.borrow_mut()     CallExpression
	              s.0                  MemberExpression{!computed}
	                0                  Index
	                            ()     CallExpression.arguments{dk: "()"}                                                                 */
	let _ = &mut *s[0].borrow_mut();                                                                                                      /*
	let•_•=•&mut•*s[0].borrow_mut();    LetVariableDeclaration
	    _                               WildcardPattern
	        &mut•*s[0].borrow_mut()     ReferenceExpression{mut}
	             *s[0].borrow_mut()     DereferenceExpression
	              s[0].borrow_mut()     CallExpression
	              s[0]                  MemberExpression{computed}
	                0                   Literal{kind: Integer}
	                             ()     CallExpression.arguments{dk: "()"}                                                                */
    let x: Foo<_> = Bar::<usize>(PhantomData);                                                                                            /*
    let•x:•Foo<_>•=•Bar::<usize>(PhantomData);    LetVariableDeclaration
           Foo<_>                                 TypeCall
              <_>                                 TypeCall.typeArguments{dk: "<>"}
               _                                  TypeInferred
                    Bar::<usize>(PhantomData)     CallExpression
                         <usize>                  CallExpression.typeArguments{dk: "<>"}
                                (PhantomData)     CallExpression.arguments{dk: "()"}                                                      */
    let f = A::<i32> { a: 10 };                                                                                                           /*
    let•f•=•A::<i32>•{•a:•10•};    LetVariableDeclaration
            A::<i32>•{•a:•10•}     StructLiteral
            A::<i32>               ExpressionTypeCast
               <i32>               ExpressionTypeCast.typeArguments{dk: "<>"}
                     {•a:•10•}     StructLiteral.properties{dk: "{}"}
                       a:•10       StructLiteralProperty
                          10       Literal{kind: Integer}                                                                                 */
	let v: <() as Lt<'_>>::T = ();                                                                                                        /*
	let•v:•<()•as•Lt<'_>>::T•=•();    LetVariableDeclaration
	       <()•as•Lt<'_>>::T          TypePath
	       <()•as•Lt<'_>>             ExpressionTypeSelector
	        ()                        TypeTuple
	              Lt<'_>              TypeCall
	                <'_>              TypeCall.typeArguments{dk: "<>"}
	                 '_               LtElided
	                           ()     TupleLiteral                                                                                        */
	<E>::V() = E::V();                                                                                                                    /*
	<E>::V()•=•E::V();    ExpressionStatement{semi}
	<E>::V()•=•E::V()     ReassignmentExpression{tk: "="}
	<E>::V()              CallExpression
	<E>::V                ExpressionPath
	<E>                   ExpressionTypeSelector
	      ()              CallExpression.arguments{dk: "()"}
	           E::V()     CallExpression
	           E::V       ExpressionPath
	               ()     CallExpression.arguments{dk: "()"}                                                                              */
	<E>::V {} = E::V();                                                                                                                   /*
	<E>::V•{}•=•E::V();    ExpressionStatement{semi}
	<E>::V•{}•=•E::V()     ReassignmentExpression{tk: "="}
	<E>::V•{}              StructLiteral
	<E>::V                 ExpressionPath
	<E>                    ExpressionTypeSelector
	       {}              StructLiteral.properties{dk: "{}"}
	            E::V()     CallExpression
	            E::V       ExpressionPath
	                ()     CallExpression.arguments{dk: "()"}                                                                             */
	let a = &mut b.0.0;                                                                                                                   /*
	let•a•=•&mut•b.0.0;    LetVariableDeclaration
	        &mut•b.0.0     ReferenceExpression{mut}
	             b.0.0     MemberExpression{!computed}
	             b.0       MemberExpression{!computed}
	               0       Index
	                 0     Index                                                                                                          */
	let a = &mut b.0[2];                                                                                                                  /*
	let•a•=•&mut•b.0[2];    LetVariableDeclaration
	        &mut•b.0[2]     ReferenceExpression{mut}
	             b.0[2]     MemberExpression{computed}
	             b.0        MemberExpression{!computed}
	               0        Index
	                 2      Literal{kind: Integer}                                                                                        */
	let _ = a::<N>(b().await).await;                                                                                                      /*
	let•_•=•a::<N>(b().await).await;    LetVariableDeclaration
	    _                               WildcardPattern
	        a::<N>(b().await).await     AwaitExpression
	        a::<N>(b().await)           CallExpression
	           <N>                      CallExpression.typeArguments{dk: "<>"}
	              (b().await)           CallExpression.arguments{dk: "()"}
	               b().await            AwaitExpression
	               b()                  CallExpression
	                ()                  CallExpression.arguments{dk: "()"}                                                                */
	let _ = a(b::<N>().await).await;                                                                                                      /*
	let•_•=•a(b::<N>().await).await;    LetVariableDeclaration
	    _                               WildcardPattern
	        a(b::<N>().await).await     AwaitExpression
	        a(b::<N>().await)           CallExpression
	         (b::<N>().await)           CallExpression.arguments{dk: "()"}
	          b::<N>().await            AwaitExpression
	          b::<N>()                  CallExpression
	             <N>                    CallExpression.typeArguments{dk: "<>"}
	                ()                  CallExpression.arguments{dk: "()"}                                                                */
	let _ = A == s!("e");                                                                                                                 /*
	let•_•=•A•==•s!("e");    LetVariableDeclaration
	    _                    WildcardPattern
	        A•==•s!("e")     ComparisonExpression{tk: "=="}
	             s!("e")     MacroInvocation
	               ("e")     MacroInvocation.segments{dk: "()"}
	                "e"      Literal{kind: String}                                                                                        */
	let a: & str = & b;                                                                                                                   /*
	let•a:•&•str•=•&•b;    LetVariableDeclaration
	       &•str           TypeReference{!mut}
	               &•b     ReferenceExpression{!mut}                                                                                      */
	::a::<f64, [u8; 8]>(a!());                                                                                                            /*
	::a::<f64,•[u8;•8]>(a!());    ExpressionStatement{semi}
	::a::<f64,•[u8;•8]>(a!())     CallExpression
	::a                           ExpressionPath
	     <f64,•[u8;•8]>           CallExpression.typeArguments{dk: "<>"}
	           [u8;•8]            TypeSizedArray
	                8             Literal{kind: Integer}
	                   (a!())     CallExpression.arguments{dk: "()"}
	                    a!()      MacroInvocation
	                      ()      MacroInvocation.segments{dk: "()"}                                                                      */
	let (the, guardian, stands, resolute) = ("the", "Turbofish", "remains", "undefeated");                                                /*
	let•(the,•guardian,•stands,•resolute)•=•("the",•"Turbofish",•"remains",•"undefeated");    LetVariableDeclaration
	    (the,•guardian,•stands,•resolute)                                                     TuplePattern
	                                        ("the",•"Turbofish",•"remains",•"undefeated")     TupleLiteral
	                                         "the"                                            Literal{kind: String}
	                                                "Turbofish"                               Literal{kind: String}
	                                                             "remains"                    Literal{kind: String}
	                                                                        "undefeated"      Literal{kind: String}                       */
    let _: (bool, bool) = (the<guardian, stands>(resolute));                                                                              /*
    let•_:•(bool,•bool)•=•(the<guardian,•stands>(resolute));    LetVariableDeclaration
        _                                                       WildcardPattern
           (bool,•bool)                                         TypeTuple
                          (the<guardian,•stands>(resolute))     TupleLiteral
                           the<guardian                         ComparisonExpression{tk: "<"}
                                         stands>(resolute)      ComparisonExpression{tk: ">"}                                             */
    let (A { x: _x, y: _y }, Z): (_, Z) = c(|| B { x: X, y: Y }, || Z);                                                                   /*
    let•(A•{•x:•_x,•y:•_y•},•Z):•(_,•Z)•=•c(||•B•{•x:•X,•y:•Y•},•||•Z);    LetVariableDeclaration
        (A•{•x:•_x,•y:•_y•},•Z)                                            TuplePattern
         A•{•x:•_x,•y:•_y•}                                                StructPattern
           {•x:•_x,•y:•_y•}                                                StructPattern.properties{dk: "{}"}
             x:•_x                                                         StructPatternPropertyDestructured
                    y:•_y                                                  StructPatternPropertyDestructured
                                 (_,•Z)                                    TypeTuple
                                  _                                        TypeInferred
                                          c(||•B•{•x:•X,•y:•Y•},•||•Z)     CallExpression
                                           (||•B•{•x:•X,•y:•Y•},•||•Z)     CallExpression.arguments{dk: "()"}
                                            ||•B•{•x:•X,•y:•Y•}            ClosureFunctionExpression
                                            ||                             ClosureFunctionExpression.parameters{dk: "||"}
                                               B•{•x:•X,•y:•Y•}            StructLiteral
                                                 {•x:•X,•y:•Y•}            StructLiteral.properties{dk: "{}"}
                                                   x:•X                    StructLiteralProperty
                                                         y:•Y              StructLiteralProperty
                                                                 ||•Z      ClosureFunctionExpression
                                                                 ||        ClosureFunctionExpression.parameters{dk: "||"}                 */
	let _: A<{ 1+2 }>;                                                                                                                    /*
	let•_:•A<{•1+2•}>;    LetVariableDeclaration
	    _                 WildcardPattern
	       A<{•1+2•}>     TypeCall
	        <{•1+2•}>     TypeCall.typeArguments{dk: "<>"}
	         {•1+2•}      BlockExpression
	           1+2        ExpressionStatement{!semi}, OperationExpression{tk: "+"}
	           1          Literal{kind: Integer}
	             2        Literal{kind: Integer}                                                                                          */
	let _: A<{ 5 }>;                                                                                                                      /*
	let•_:•A<{•5•}>;    LetVariableDeclaration
	    _               WildcardPattern
	       A<{•5•}>     TypeCall
	        <{•5•}>     TypeCall.typeArguments{dk: "<>"}
	         {•5•}      BlockExpression
	           5        ExpressionStatement{!semi}, Literal{kind: Integer}                                                                */
    let A::<1, N>(N) = A::new();                                                                                                          /*
    let•A::<1,•N>(N)•=•A::new();    LetVariableDeclaration
        A::<1,•N>(N)                TuplePattern
        A::<1,•N>                   ExpressionTypeCast
           <1,•N>                   ExpressionTypeCast.typeArguments{dk: "<>"}
            1                       Literal{kind: Integer}
                 (N)                TuplePattern.items{dk: "()"}
                       A::new()     CallExpression
                       A::new       ExpressionPath
                             ()     CallExpression.arguments{dk: "()"}                                                                    */
    let _ = Some(Foo { _a: 42 }).map(|a| a as Foo::<i32>);                                                                                /*
    let•_•=•Some(Foo•{•_a:•42•}).map(|a|•a•as•Foo::<i32>);    LetVariableDeclaration
        _                                                     WildcardPattern
            Some(Foo•{•_a:•42•}).map(|a|•a•as•Foo::<i32>)     CallExpression
            Some(Foo•{•_a:•42•})                              CallExpression
                (Foo•{•_a:•42•})                              CallExpression.arguments{dk: "()"}
                 Foo•{•_a:•42•}                               StructLiteral
                     {•_a:•42•}                               StructLiteral.properties{dk: "{}"}
                       _a:•42                                 StructLiteralProperty
                           42                                 Literal{kind: Integer}
                                    (|a|•a•as•Foo::<i32>)     CallExpression.arguments{dk: "()"}
                                     |a|•a•as•Foo::<i32>      ClosureFunctionExpression
                                     |a|                      ClosureFunctionExpression.parameters{dk: "||"}
                                      a                       ClosureFunctionParameterDeclaration
                                         a•as•Foo::<i32>      ExpressionAsTypeCast
                                              Foo::<i32>      TypeCall
                                                   <i32>      TypeCall.typeArguments{dk: "<>"}                                            */
    let _ = ()=()=()=();                                                                                                                  /*
    let•_•=•()=()=()=();    LetVariableDeclaration
        _                   WildcardPattern
            ()=()=()=()     ReassignmentExpression{tk: "="}
            ()              TupleLiteral
               ()=()=()     ReassignmentExpression{tk: "="}
               ()           TupleLiteral
                  ()=()     ReassignmentExpression{tk: "="}
                  ()        TupleLiteral
                     ()     TupleLiteral                                                                                                  */
	String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>();                                                                /*
	String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>();    ExpressionStatement{semi}
	String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>()     CallExpression
	String::<>::from::<>("><>").chars::<>().rev::<>()                         CallExpression
	String::<>::from::<>("><>").chars::<>()                                   CallExpression
	String::<>::from::<>("><>")                                               CallExpression
	String::<>::from                                                          ExpressionPath
	String::<>                                                                ExpressionTypeCast
	        <>                                                                ExpressionTypeCast.typeArguments{dk: "<>"}
	                  <>                                                      CallExpression.typeArguments{dk: "<>"}
	                    ("><>")                                               CallExpression.arguments{dk: "()"}
	                     "><>"                                                Literal{kind: String}
	                                   <>                                     CallExpression.typeArguments{dk: "<>"}
	                                     ()                                   CallExpression.arguments{dk: "()"}
	                                             <>                           CallExpression.typeArguments{dk: "<>"}
	                                               ()                         CallExpression.arguments{dk: "()"}
	                                                           <String>       CallExpression.typeArguments{dk: "<>"}
	                                                                   ()     CallExpression.arguments{dk: "()"}                          */
	fn a(x: &f<r>) { return while !x.f() { x.g(0); }; }                                                                                   /*
	fn•a(x:•&f<r>)•{•return•while•!x.f()•{•x.g(0);•};•}    FunctionDeclaration
	    (x:•&f<r>)                                         FunctionDeclaration.parameters{dk: "()"}
	     x:•&f<r>                                          FunctionParameterDeclaration
	        &f<r>                                          TypeReference{!mut}
	         f<r>                                          TypeCall
	          <r>                                          TypeCall.typeArguments{dk: "<>"}
	               {•return•while•!x.f()•{•x.g(0);•};•}    FunctionDeclaration.body{dk: "{}"}
	                 return•while•!x.f()•{•x.g(0);•};      ExpressionStatement{semi}
	                 return•while•!x.f()•{•x.g(0);•}       ReturnExpression
	                        while•!x.f()•{•x.g(0);•}       WhileBlockExpression
	                              !x.f()                   NotExpression
	                               x.f()                   CallExpression
	                                  ()                   CallExpression.arguments{dk: "()"}
	                                     {•x.g(0);•}       WhileBlockExpression.body{dk: "{}"}
	                                       x.g(0);         ExpressionStatement{semi}
	                                       x.g(0)          CallExpression
	                                          (0)          CallExpression.arguments{dk: "()"}
	                                           0           Literal{kind: Integer}                                                         */
    let i = &f::s(0);                                                                                                                     /*
    let•i•=•&f::s(0);    LetVariableDeclaration
            &f::s(0)     ReferenceExpression{!mut}
             f::s(0)     CallExpression
             f::s        ExpressionPath
                 (0)     CallExpression.arguments{dk: "()"}
                  0      Literal{kind: Integer}                                                                                           */
	<u8 as D<13>>::e::<u8>();                                                                                                             /*
	<u8•as•D<13>>::e::<u8>();    ExpressionStatement{semi}
	<u8•as•D<13>>::e::<u8>()     CallExpression
	<u8•as•D<13>>::e             ExpressionPath
	<u8•as•D<13>>                ExpressionTypeSelector
	       D<13>                 TypeCall
	        <13>                 TypeCall.typeArguments{dk: "<>"}
	         13                  Literal{kind: Integer}
	                  <u8>       CallExpression.typeArguments{dk: "<>"}
	                      ()     CallExpression.arguments{dk: "()"}                                                                       */
	let _: i32 = (match "" {                                                                                                              /*
	let•_:•i32•=•(match•""•{↲    <LetVariableDeclaration>
	    _                        WildcardPattern
	             (match•""•{↲    <CallExpression>
	              match•""•{↲    <MatchExpression>
	                    ""       Literal{kind: String}
	                       {↲    <MatchExpression.cases{dk: "{}"}>                                                                        */
        "+" => ::std::ops::Add::add,                                                                                                      /*
        "+"•=>•::std::ops::Add::add    MatchExpressionCase
        "+"                            Literal{kind: String}
               ::std::ops::Add::add    ExpressionPath
               ::std::ops::Add         ExpressionPath
               ::std::ops              ExpressionPath
               ::std                   ExpressionPath                                                                                     */
        "-" => ::std::ops::Sub::sub,                                                                                                      /*
        "-"•=>•::std::ops::Sub::sub    MatchExpressionCase
        "-"                            Literal{kind: String}
               ::std::ops::Sub::sub    ExpressionPath
               ::std::ops::Sub         ExpressionPath
               ::std::ops              ExpressionPath
               ::std                   ExpressionPath                                                                                     */
        "<" => |a,b| (a < b) as i32,                                                                                                      /*
        "<"•=>•|a,b|•(a•<•b)•as•i32    MatchExpressionCase
        "<"                            Literal{kind: String}
               |a,b|•(a•<•b)•as•i32    ClosureFunctionExpression
               |a,b|                   ClosureFunctionExpression.parameters{dk: "||"}
                a                      ClosureFunctionParameterDeclaration
                  b                    ClosureFunctionParameterDeclaration
                     (a•<•b)•as•i32    ExpressionAsTypeCast
                      a•<•b            ComparisonExpression{tk: "<"}                                                                      */
        _ => c!(),                                                                                                                        /*
        _•=>•c!()    MatchExpressionCase
        _            WildcardPattern
             c!()    MacroInvocation
               ()    MacroInvocation.segments{dk: "()"}                                                                                   */
    })(5, 5);                                                                                                                             /*
••••}            </MatchExpression.cases>
••••}            </MatchExpression>
      (5,•5)     CallExpression.arguments{dk: "()"}
       5         Literal{kind: Integer}
          5      Literal{kind: Integer}
••••})(5,•5)     </CallExpression>
••••})(5,•5);    </LetVariableDeclaration>                                                                                                */
	[].e().f(|_: &i32| {                                                                                                                  /*
	[].e().f(|_:•&i32|•{↲    <ExpressionStatement{semi}>
	[].e().f(|_:•&i32|•{↲    <CallExpression>
	[].e()                   CallExpression
	[]                       ArrayLiteral
	    ()                   CallExpression.arguments{dk: "()"}
	        (|_:•&i32|•{↲    <CallExpression.arguments{dk: "()"}>
	         |_:•&i32|•{↲    <ClosureFunctionExpression>
	         |_:•&i32|       ClosureFunctionExpression.parameters{dk: "||"}
	          _:•&i32        ClosureFunctionParameterDeclaration
	          _              WildcardPattern
	             &i32        TypeReference{!mut}
	                   {↲    <BlockExpression>                                                                                            */
        [].e().f(move |_: &i32| {                                                                                                         /*
        [].e().f(move•|_:•&i32|•{↲    <ExpressionStatement{semi}>
        [].e().f(move•|_:•&i32|•{↲    <CallExpression>
        [].e()                        CallExpression
        []                            ArrayLiteral
            ()                        CallExpression.arguments{dk: "()"}
                (move•|_:•&i32|•{↲    <CallExpression.arguments{dk: "()"}>
                 move•|_:•&i32|•{↲    <ClosureFunctionExpression{move}>
                      |_:•&i32|       ClosureFunctionExpression.parameters{dk: "||"}
                       _:•&i32        ClosureFunctionParameterDeclaration
                       _              WildcardPattern
                          &i32        TypeReference{!mut}
                                {↲    <BlockExpression>                                                                                   */
            i += 1;                                                                                                                       /*
            i•+=•1;    ExpressionStatement{semi}
            i•+=•1     ReassignmentOperationExpression{tk: "+="}
                 1     Literal{kind: Integer}                                                                                             */
        });                                                                                                                               /*
••••••••}      </BlockExpression>
••••••••}      </ClosureFunctionExpression>
••••••••})     </CallExpression.arguments>
••••••••})     </CallExpression>
••••••••});    </ExpressionStatement>                                                                                                     */
    });                                                                                                                                   /*
••••}      </BlockExpression>
••••}      </ClosureFunctionExpression>
••••})     </CallExpression.arguments>
••••})     </CallExpression>
••••});    </ExpressionStatement>                                                                                                         */
    let _x2 = X { a: 1, b: 2, ..DX };                                                                                                     /*
    let•_x2•=•X•{•a:•1,•b:•2,•..DX•};    LetVariableDeclaration
              X•{•a:•1,•b:•2,•..DX•}     StructLiteral
                {•a:•1,•b:•2,•..DX•}     StructLiteral.properties{dk: "{}"}
                  a:•1                   StructLiteralProperty
                     1                   Literal{kind: Integer}
                        b:•2             StructLiteralProperty
                           2             Literal{kind: Integer}
                              ..DX       StructLiteralPropertySpread                                                                      */
	i[i[0]] = 0;                                                                                                                          /*
	i[i[0]]•=•0;    ExpressionStatement{semi}
	i[i[0]]•=•0     ReassignmentExpression{tk: "="}
	i[i[0]]         MemberExpression{computed}
	  i[0]          MemberExpression{computed}
	    0           Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
    i[i[0] - 1] = 0;                                                                                                                      /*
    i[i[0]•-•1]•=•0;    ExpressionStatement{semi}
    i[i[0]•-•1]•=•0     ReassignmentExpression{tk: "="}
    i[i[0]•-•1]         MemberExpression{computed}
      i[0]•-•1          OperationExpression{tk: "-"}
      i[0]              MemberExpression{computed}
        0               Literal{kind: Integer}
             1          Literal{kind: Integer}
                  0     Literal{kind: Integer}                                                                                            */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

[                                                                                                                                         /*
[↲    <ExpressionStatement{semi}>
[↲    <ArrayLiteral>                                                                                                                      */
	b.a,                                                                                                                                  /*
	b.a    MemberExpression{!computed}                                                                                                    */
	X { len: 3 },                                                                                                                         /*
	X•{•len:•3•}    StructLiteral
	  {•len:•3•}    StructLiteral.properties{dk: "{}"}
	    len:•3      StructLiteralProperty
	         3      Literal{kind: Integer}                                                                                                */
	x.len > (3),                                                                                                                          /*
	x.len•>•(3)    ComparisonExpression{tk: ">"}
	x.len          MemberExpression{!computed}
	         3     Literal{kind: Integer}                                                                                                 */
	x.len >> (3),                                                                                                                         /*
	x.len•>>•(3)    OperationExpression{tk: ">>"}
	x.len           MemberExpression{!computed}
	          3     Literal{kind: Integer}                                                                                                */
	vec![1, 2, 3].into_iter().collect::<Vec<usize>>(),                                                                                    /*
	vec![1,•2,•3].into_iter().collect::<Vec<usize>>()    CallExpression
	vec![1,•2,•3].into_iter()                            CallExpression
	vec![1,•2,•3]                                        MacroInvocation
	    [1,•2,•3]                                        MacroInvocation.segments{dk: "[]"}
	     1                                               Literal{kind: Integer}
	      ,                                              PunctuationToken{tk: ","}
	        2                                            Literal{kind: Integer}
	         ,                                           PunctuationToken{tk: ","}
	           3                                         Literal{kind: Integer}
	                       ()                            CallExpression.arguments{dk: "()"}
	                                   <Vec<usize>>      CallExpression.typeArguments{dk: "<>"}
	                                    Vec<usize>       TypeCall
	                                       <usize>       TypeCall.typeArguments{dk: "<>"}
	                                               ()    CallExpression.arguments{dk: "()"}                                               */
	X(1, 2, 3),                                                                                                                           /*
	X(1,•2,•3)    CallExpression
	 (1,•2,•3)    CallExpression.arguments{dk: "()"}
	  1           Literal{kind: Integer}
	     2        Literal{kind: Integer}
	        3     Literal{kind: Integer}                                                                                                  */
	(1, 2, 3),                                                                                                                            /*
	(1,•2,•3)    TupleLiteral
	 1           Literal{kind: Integer}
	    2        Literal{kind: Integer}
	       3     Literal{kind: Integer}                                                                                                   */
	vec! { 1, 2, 3 }.len(),                                                                                                               /*
	vec!•{•1,•2,•3•}.len()    CallExpression
	vec!•{•1,•2,•3•}          MacroInvocation
	     {•1,•2,•3•}          MacroInvocation.segments{dk: "{}"}
	       1                  Literal{kind: Integer}
	        ,                 PunctuationToken{tk: ","}
	          2               Literal{kind: Integer}
	           ,              PunctuationToken{tk: ","}
	             3            Literal{kind: Integer}
	                    ()    CallExpression.arguments{dk: "()"}                                                                          */
	write! { vec![], "" }?,                                                                                                               /*
	write!•{•vec![],•""•}?    UnwrapExpression
	write!•{•vec![],•""•}     MacroInvocation
	       {•vec![],•""•}     MacroInvocation.segments{dk: "{}"}
	            !             PunctuationToken{tk: "!"}
	             []           DelimGroup
	               ,          PunctuationToken{tk: ","}
	                 ""       Literal{kind: String}                                                                                       */
	&*d.borrow(),                                                                                                                         /*
	&*d.borrow()    ReferenceExpression{!mut}
	 *d.borrow()    DereferenceExpression
	  d.borrow()    CallExpression
	          ()    CallExpression.arguments{dk: "()"}                                                                                    */
	**bar == Test::Baz || **bar == Test::Qux,                                                                                             /*
	**bar•==•Test::Baz•||•**bar•==•Test::Qux    OrExpression{tk: "||"}
	**bar•==•Test::Baz                          ComparisonExpression{tk: "=="}
	**bar                                       DereferenceExpression
	 *bar                                       DereferenceExpression
	         Test::Baz                          ExpressionPath
	                      **bar•==•Test::Qux    ComparisonExpression{tk: "=="}
	                      **bar                 DereferenceExpression
	                       *bar                 DereferenceExpression
	                               Test::Qux    ExpressionPath                                                                            */
	&foo[0..1],                                                                                                                           /*
	&foo[0..1]    ReferenceExpression{!mut}
	 foo[0..1]    MemberExpression{computed}
	     0..1     RangeLiteral{!last}
	     0        Literal{kind: Integer}
	        1     Literal{kind: Integer}                                                                                                  */
	TypeId::of::<T>(),                                                                                                                    /*
	TypeId::of::<T>()    CallExpression
	TypeId::of           ExpressionPath
	            <T>      CallExpression.typeArguments{dk: "<>"}
	               ()    CallExpression.arguments{dk: "()"}                                                                               */
	&[*xs[0].x, *xs[1].x],                                                                                                                /*
	&[*xs[0].x,•*xs[1].x]    ReferenceExpression{!mut}
	 [*xs[0].x,•*xs[1].x]    ArrayLiteral
	  *xs[0].x               DereferenceExpression
	   xs[0].x               MemberExpression{!computed}
	   xs[0]                 MemberExpression{computed}
	      0                  Literal{kind: Integer}
	            *xs[1].x     DereferenceExpression
	             xs[1].x     MemberExpression{!computed}
	             xs[1]       MemberExpression{computed}
	                1        Literal{kind: Integer}                                                                                       */
	&mut tup.0,                                                                                                                           /*
	&mut•tup.0    ReferenceExpression{mut}
	     tup.0    MemberExpression{!computed}
	         0    Index                                                                                                                   */
	<_>::f(),                                                                                                                             /*
	<_>::f()    CallExpression
	<_>::f      ExpressionPath
	<_>         ExpressionTypeSelector
	 _          TypeInferred
	      ()    CallExpression.arguments{dk: "()"}                                                                                        */
	&(fop::<T> as fn()),                                                                                                                  /*
	&(fop::<T>•as•fn())    ReferenceExpression{!mut}
	  fop::<T>•as•fn()     ExpressionAsTypeCast
	  fop::<T>             ExpressionTypeCast
	       <T>             ExpressionTypeCast.typeArguments{dk: "<>"}
	              fn()     TypeFnPointer
	                ()     TypeFnPointer.parameters{dk: "()"}                                                                             */
	(a)(),                                                                                                                                /*
	(a)()    CallExpression
	   ()    CallExpression.arguments{dk: "()"}                                                                                           */
	::foo::bar::baz::f(),                                                                                                                 /*
	::foo::bar::baz::f()    CallExpression
	::foo::bar::baz::f      ExpressionPath
	::foo::bar::baz         ExpressionPath
	::foo::bar              ExpressionPath
	::foo                   ExpressionPath
	                  ()    CallExpression.arguments{dk: "()"}                                                                            */
	<() as ::foo::T>::Assoc::f(),                                                                                                         /*
	<()•as•::foo::T>::Assoc::f()    CallExpression
	<()•as•::foo::T>::Assoc::f      ExpressionPath
	<()•as•::foo::T>::Assoc         ExpressionPath
	<()•as•::foo::T>                ExpressionTypeSelector
	 ()                             TypeTuple
	       ::foo::T                 TypePath
	       ::foo                    TypePath
	                          ()    CallExpression.arguments{dk: "()"}                                                                    */
	[].a(),                                                                                                                               /*
	[].a()    CallExpression
	[]        ArrayLiteral
	    ()    CallExpression.arguments{dk: "()"}                                                                                          */
	id::<[i32; 3]>([1,2,3]),                                                                                                              /*
	id::<[i32;•3]>([1,2,3])    CallExpression
	    <[i32;•3]>             CallExpression.typeArguments{dk: "<>"}
	     [i32;•3]              TypeSizedArray
	           3               Literal{kind: Integer}
	              ([1,2,3])    CallExpression.arguments{dk: "()"}
	               [1,2,3]     ArrayLiteral
	                1          Literal{kind: Integer}
	                  2        Literal{kind: Integer}
	                    3      Literal{kind: Integer}                                                                                     */
	m::Pub(0u8).method_with_priv_params(loop{}),                                                                                          /*
	m::Pub(0u8).method_with_priv_params(loop{})    CallExpression
	m::Pub(0u8)                                    CallExpression
	m::Pub                                         ExpressionPath
	      (0u8)                                    CallExpression.arguments{dk: "()"}
	       0u8                                     Literal{kind: Integer}
	        u8                                     Identifier
	                                   (loop{})    CallExpression.arguments{dk: "()"}
	                                    loop{}     LoopBlockExpression
	                                        {}     LoopBlockExpression.body{dk: "{}"}                                                     */
	<m::Pub<m::Alias>>::INHERENT_ASSOC_CONST,                                                                                             /*
	<m::Pub<m::Alias>>::INHERENT_ASSOC_CONST    ExpressionPath
	<m::Pub<m::Alias>>                          ExpressionTypeSelector
	 m::Pub<m::Alias>                           TypeCall
	 m::Pub                                     TypePath
	       <m::Alias>                           TypeCall.typeArguments{dk: "<>"}
	        m::Alias                            TypePath                                                                                  */
	<a!() as B>::f(0),                                                                                                                    /*
	<a!()•as•B>::f(0)    CallExpression
	<a!()•as•B>::f       ExpressionPath
	<a!()•as•B>          ExpressionTypeSelector
	 a!()                MacroInvocation
	   ()                MacroInvocation.segments{dk: "()"}
	              (0)    CallExpression.arguments{dk: "()"}
	               0     Literal{kind: Integer}                                                                                           */
	a::<B<N, { N as u128 }>>(),                                                                                                           /*
	a::<B<N,•{•N•as•u128•}>>()    CallExpression
	   <B<N,•{•N•as•u128•}>>      CallExpression.typeArguments{dk: "<>"}
	    B<N,•{•N•as•u128•}>       TypeCall
	     <N,•{•N•as•u128•}>       TypeCall.typeArguments{dk: "<>"}
	         {•N•as•u128•}        BlockExpression
	           N•as•u128          ExpressionStatement{!semi}, ExpressionAsTypeCast
	                        ()    CallExpression.arguments{dk: "()"}                                                                      */
	Foo { f:(((((((x)).clone()))))) },                                                                                                    /*
	Foo•{•f:(((((((x)).clone())))))•}    StructLiteral
	    {•f:(((((((x)).clone())))))•}    StructLiteral.properties{dk: "{}"}
	      f:(((((((x)).clone())))))      StructLiteralProperty
	             ((x)).clone()           CallExpression
	                        ()           CallExpression.arguments{dk: "()"}                                                               */
	a::<&str, (*const u8, u64)>(),                                                                                                        /*
	a::<&str,•(*const•u8,•u64)>()    CallExpression
	   <&str,•(*const•u8,•u64)>      CallExpression.typeArguments{dk: "<>"}
	    &str                         TypeReference{!mut}
	          (*const•u8,•u64)       TypeTuple
	           *const•u8             TypeDereferenceConst
	                           ()    CallExpression.arguments{dk: "()"}                                                                   */
	a("".b()).c("").d().await,                                                                                                            /*
	a("".b()).c("").d().await    AwaitExpression
	a("".b()).c("").d()          CallExpression
	a("".b()).c("")              CallExpression
	a("".b())                    CallExpression
	 ("".b())                    CallExpression.arguments{dk: "()"}
	  "".b()                     CallExpression
	  ""                         Literal{kind: String}
	      ()                     CallExpression.arguments{dk: "()"}
	           ("")              CallExpression.arguments{dk: "()"}
	            ""               Literal{kind: String}
	                 ()          CallExpression.arguments{dk: "()"}                                                                       */
	foo(&[vec![123]]).await,                                                                                                              /*
	foo(&[vec![123]]).await    AwaitExpression
	foo(&[vec![123]])          CallExpression
	   (&[vec![123]])          CallExpression.arguments{dk: "()"}
	    &[vec![123]]           ReferenceExpression{!mut}
	     [vec![123]]           ArrayLiteral
	      vec![123]            MacroInvocation
	          [123]            MacroInvocation.segments{dk: "[]"}
	           123             Literal{kind: Integer}                                                                                     */
	A::b::<C>(x).d(E("x"))?.f(1),                                                                                                         /*
	A::b::<C>(x).d(E("x"))?.f(1)    CallExpression
	A::b::<C>(x).d(E("x"))?         UnwrapExpression
	A::b::<C>(x).d(E("x"))          CallExpression
	A::b::<C>(x)                    CallExpression
	A::b                            ExpressionPath
	      <C>                       CallExpression.typeArguments{dk: "<>"}
	         (x)                    CallExpression.arguments{dk: "()"}
	              (E("x"))          CallExpression.arguments{dk: "()"}
	               E("x")           CallExpression
	                ("x")           CallExpression.arguments{dk: "()"}
	                 "x"            Literal{kind: String}
	                         (1)    CallExpression.arguments{dk: "()"}
	                          1     Literal{kind: Integer}                                                                                */
	// std::<_ as _>,
	//•std::<_•as•_>,    Comment{line}
	std::<0>,                                                                                                                             /*
	std::<0>    ExpressionTypeCast
	     <0>    ExpressionTypeCast.typeArguments{dk: "<>"}
	      0     Literal{kind: Integer}                                                                                                    */
	&raw const x,                                                                                                                         /*
	&raw•const•x    RawReferenceExpression{kind: "const"}                                                                                 */
	(A::a as fn(&(dyn A+'static))->B)(&"c"),                                                                                              /*
	(A::a•as•fn(&(dyn•A+'static))->B)(&"c")    CallExpression
	 A::a•as•fn(&(dyn•A+'static))->B           ExpressionAsTypeCast
	 A::a                                      ExpressionPath
	         fn(&(dyn•A+'static))->B           TypeFnPointer
	           (&(dyn•A+'static))              TypeFnPointer.parameters{dk: "()"}
	            &(dyn•A+'static)               TypeFnPointerParameter, TypeReference{!mut}
	              dyn•A+'static                TypeDynBounds{dyn}
	                  A                        TypeTraitBound{!maybeConst, !optional}
	                    'static                LtStatic
	                                 (&"c")    CallExpression.arguments{dk: "()"}
	                                  &"c"     ReferenceExpression{!mut}
	                                   "c"     Literal{kind: String}                                                                      */
	f::<<T as S>::E>(),                                                                                                                   /*
	f::<<T•as•S>::E>()    CallExpression
	   <<T•as•S>::E>      CallExpression.typeArguments{dk: "<>"}
	    <T•as•S>::E       TypePath
	    <T•as•S>          ExpressionTypeSelector
	                ()    CallExpression.arguments{dk: "()"}                                                                              */
	<u64 as From<<T as Iterator>::Item>>::from(),                                                                                         /*
	<u64•as•From<<T•as•Iterator>::Item>>::from()    CallExpression
	<u64•as•From<<T•as•Iterator>::Item>>::from      ExpressionPath
	<u64•as•From<<T•as•Iterator>::Item>>            ExpressionTypeSelector
	        From<<T•as•Iterator>::Item>             TypeCall
	            <<T•as•Iterator>::Item>             TypeCall.typeArguments{dk: "<>"}
	             <T•as•Iterator>::Item              TypePath
	             <T•as•Iterator>                    ExpressionTypeSelector
	                                          ()    CallExpression.arguments{dk: "()"}                                                    */
	<<Q as A<'_>>::B as C<Q::D>>::e(db),                                                                                                  /*
	<<Q•as•A<'_>>::B•as•C<Q::D>>::e(db)    CallExpression
	<<Q•as•A<'_>>::B•as•C<Q::D>>::e        ExpressionPath
	<<Q•as•A<'_>>::B•as•C<Q::D>>           ExpressionTypeSelector
	 <Q•as•A<'_>>::B                       TypePath
	 <Q•as•A<'_>>                          ExpressionTypeSelector
	       A<'_>                           TypeCall
	        <'_>                           TypeCall.typeArguments{dk: "<>"}
	         '_                            LtElided
	                    C<Q::D>            TypeCall
	                     <Q::D>            TypeCall.typeArguments{dk: "<>"}
	                      Q::D             TypePath
	                               (db)    CallExpression.arguments{dk: "()"}                                                             */
	tuple. 0.0 ,                                                                                                                          /*
	tuple.•0.0    MemberExpression{!computed}
	tuple.•0      MemberExpression{!computed}
	       0      Index
	         0    Index                                                                                                                   */
	tuple.0. 0 ,                                                                                                                          /*
	tuple.0.•0    MemberExpression{!computed}
	tuple.0       MemberExpression{!computed}
	      0       Index
	         0    Index                                                                                                                   */
	tuple./*special cases*/0.0 //aaa
/*	tuple./*special•cases*/0.0          MemberExpression{!computed}
	tuple./*special•cases*/0            MemberExpression{!computed}
	      /*special•cases*/             Comment{!line}
	                       0            Index
	                         0          Index                                                                                             */
	                           //aaa    Comment{line}
   ,(((),),),                                                                                                                             /*
    (((),),)    TupleLiteral
     ((),)      TupleLiteral
      ()        TupleLiteral                                                                                                              */
   	(1, (2, 3)).1.1,                                                                                                                      /*
   	(1,•(2,•3)).1.1    MemberExpression{!computed}
   	(1,•(2,•3)).1      MemberExpression{!computed}
   	(1,•(2,•3))        TupleLiteral
   	 1                 Literal{kind: Integer}
   	    (2,•3)         TupleLiteral
   	     2             Literal{kind: Integer}
   	        3          Literal{kind: Integer}
   	            1      Index
   	              1    Index                                                                                                              */
   	(1, (2, (3, 4))).1.1.1                                                                                                                /*
   	(1,•(2,•(3,•4))).1.1.1↲    <CallExpression>
   	(1,•(2,•(3,•4))).1.1.1     MemberExpression{!computed}
   	(1,•(2,•(3,•4))).1.1       MemberExpression{!computed}
   	(1,•(2,•(3,•4))).1         MemberExpression{!computed}
   	(1,•(2,•(3,•4)))           TupleLiteral
   	 1                         Literal{kind: Integer}
   	    (2,•(3,•4))            TupleLiteral
   	     2                     Literal{kind: Integer}
   	        (3,•4)             TupleLiteral
   	         3                 Literal{kind: Integer}
   	            4              Literal{kind: Integer}
   	                 1         Index
   	                   1       Index
   	                     1     Index                                                                                                      */
	(1,),                                                                                                                                 /*
	(1,)    CallExpression.arguments{dk: "()"}
	 1      Literal{kind: Integer}
   ╚(1,)    </CallExpression>                                                                                                             */
	(1),                                                                                                                                  /*
	 1    Literal{kind: Integer}                                                                                                          */
	a ((1,2.0,3)),                                                                                                                        /*
	a•((1,2.0,3))    CallExpression
	  ((1,2.0,3))    CallExpression.arguments{dk: "()"}
	   (1,2.0,3)     TupleLiteral
	    1            Literal{kind: Integer}
	      2.0        Literal{kind: Float}
	          3      Literal{kind: Integer}                                                                                               */
	b ((1,)),                                                                                                                             /*
	b•((1,))    CallExpression
	  ((1,))    CallExpression.arguments{dk: "()"}
	   (1,)     TupleLiteral
	    1       Literal{kind: Integer}                                                                                                    */
	1.f::<T>(),                                                                                                                           /*
	1.f::<T>()    CallExpression
	1             Literal{kind: Integer}
	     <T>      CallExpression.typeArguments{dk: "<>"}
	        ()    CallExpression.arguments{dk: "()"}                                                                                      */
	*a = &a[1..],                                                                                                                         /*
	*a•=•&a[1..]    ReassignmentExpression{tk: "="}
	*a              DereferenceExpression
	     &a[1..]    ReferenceExpression{!mut}
	      a[1..]    MemberExpression{computed}
	        1..     RangeLiteral{!last}
	        1       Literal{kind: Integer}                                                                                                */
	a().await.0,                                                                                                                          /*
	a().await.0    MemberExpression{!computed}
	a().await      AwaitExpression
	a()            CallExpression
	 ()            CallExpression.arguments{dk: "()"}
	          0    Index                                                                                                                  */
	a.b(c).await.d(e)?,                                                                                                                   /*
	a.b(c).await.d(e)?    UnwrapExpression
	a.b(c).await.d(e)     CallExpression
	a.b(c).await          AwaitExpression
	a.b(c)                CallExpression
	   (c)                CallExpression.arguments{dk: "()"}
	              (e)     CallExpression.arguments{dk: "()"}                                                                              */
	0 + 1, 0 * 1, 0 - 1, 0 / 1, 0 % 1, 0 & 1, 0 | 1, 0 << 1, 0 >> 1, 0 == 1, 0 != 1, 0 < 1, 0 > 1, 0 <= 1, 0 >= 1,                        /*
	0•+•1                                                                                                            OperationExpression{tk: "+"}
	0                                                                                                                Literal{kind: Integer}
	    1                                                                                                            Literal{kind: Integer}
	       0•*•1                                                                                                     OperationExpression{tk: "*"}
	       0                                                                                                         Literal{kind: Integer}
	           1                                                                                                     Literal{kind: Integer}
	              0•-•1                                                                                              OperationExpression{tk: "-"}
	              0                                                                                                  Literal{kind: Integer}
	                  1                                                                                              Literal{kind: Integer}
	                     0•/•1                                                                                       OperationExpression{tk: "/"}
	                     0                                                                                           Literal{kind: Integer}
	                         1                                                                                       Literal{kind: Integer}
	                            0•%•1                                                                                OperationExpression{tk: "%"}
	                            0                                                                                    Literal{kind: Integer}
	                                1                                                                                Literal{kind: Integer}
	                                   0•&•1                                                                         OperationExpression{tk: "&"}
	                                   0                                                                             Literal{kind: Integer}
	                                       1                                                                         Literal{kind: Integer}
	                                          0•|•1                                                                  OperationExpression{tk: "|"}
	                                          0                                                                      Literal{kind: Integer}
	                                              1                                                                  Literal{kind: Integer}
	                                                 0•<<•1                                                          OperationExpression{tk: "<<"}
	                                                 0                                                               Literal{kind: Integer}
	                                                      1                                                          Literal{kind: Integer}
	                                                         0•>>•1                                                  OperationExpression{tk: ">>"}
	                                                         0                                                       Literal{kind: Integer}
	                                                              1                                                  Literal{kind: Integer}
	                                                                 0•==•1                                          ComparisonExpression{tk: "=="}
	                                                                 0                                               Literal{kind: Integer}
	                                                                      1                                          Literal{kind: Integer}
	                                                                         0•!=•1                                  ComparisonExpression{tk: "!="}
	                                                                         0                                       Literal{kind: Integer}
	                                                                              1                                  Literal{kind: Integer}
	                                                                                 0•<•1                           ComparisonExpression{tk: "<"}
	                                                                                 0                               Literal{kind: Integer}
	                                                                                     1                           Literal{kind: Integer}
	                                                                                        0•>•1                    ComparisonExpression{tk: ">"}
	                                                                                        0                        Literal{kind: Integer}
	                                                                                            1                    Literal{kind: Integer}
	                                                                                               0•<=•1            ComparisonExpression{tk: "<="}
	                                                                                               0                 Literal{kind: Integer}
	                                                                                                    1            Literal{kind: Integer}
	                                                                                                       0•>=•1    ComparisonExpression{tk: ">="}
	                                                                                                       0         Literal{kind: Integer}
	                                                                                                            1    Literal{kind: Integer}*/
	x -= 0, x *= 0, x /= 0, x &= 0, x %= 0, x ^= 0, x += 0, x <<= 0, x <<= 0, x >>= 0, x >>= 0, x |= 0,                                   /*
	x•-=•0                                                                                                ReassignmentOperationExpression{tk: "-="}
	     0                                                                                                Literal{kind: Integer}
	        x•*=•0                                                                                        ReassignmentOperationExpression{tk: "*="}
	             0                                                                                        Literal{kind: Integer}
	                x•/=•0                                                                                ReassignmentOperationExpression{tk: "/="}
	                     0                                                                                Literal{kind: Integer}
	                        x•&=•0                                                                        ReassignmentOperationExpression{tk: "&="}
	                             0                                                                        Literal{kind: Integer}
	                                x•%=•0                                                                ReassignmentOperationExpression{tk: "%="}
	                                     0                                                                Literal{kind: Integer}
	                                        x•^=•0                                                        ReassignmentOperationExpression{tk: "^="}
	                                             0                                                        Literal{kind: Integer}
	                                                x•+=•0                                                ReassignmentOperationExpression{tk: "+="}
	                                                     0                                                Literal{kind: Integer}
	                                                        x•<<=•0                                       ReassignmentOperationExpression{tk: "<<="}
	                                                              0                                       Literal{kind: Integer}
	                                                                 x•<<=•0                              ReassignmentOperationExpression{tk: "<<="}
	                                                                       0                              Literal{kind: Integer}
	                                                                          x•>>=•0                     ReassignmentOperationExpression{tk: ">>="}
	                                                                                0                     Literal{kind: Integer}
	                                                                                   x•>>=•0            ReassignmentOperationExpression{tk: ">>="}
	                                                                                         0            Literal{kind: Integer}
	                                                                                            x•|=•0    ReassignmentOperationExpression{tk: "|="}
	                                                                                                 0    Literal{kind: Integer}          */
	A::<1>::B(),                                                                                                                          /*
	A::<1>::B()    CallExpression
	A::<1>::B      ExpressionPath
	A::<1>         ExpressionTypeCast
	   <1>         ExpressionTypeCast.typeArguments{dk: "<>"}
	    1          Literal{kind: Integer}
	         ()    CallExpression.arguments{dk: "()"}                                                                                     */
	A::<1>::B{},                                                                                                                          /*
	A::<1>::B{}    StructLiteral
	A::<1>::B      ExpressionPath
	A::<1>         ExpressionTypeCast
	   <1>         ExpressionTypeCast.typeArguments{dk: "<>"}
	    1          Literal{kind: Integer}
	         {}    StructLiteral.properties{dk: "{}"}                                                                                     */
	A::<1>(),                                                                                                                             /*
	A::<1>()    CallExpression
	   <1>      CallExpression.typeArguments{dk: "<>"}
	    1       Literal{kind: Integer}
	      ()    CallExpression.arguments{dk: "()"}                                                                                        */
	A::<1>{},                                                                                                                             /*
	A::<1>{}    StructLiteral
	A::<1>      ExpressionTypeCast
	   <1>      ExpressionTypeCast.typeArguments{dk: "<>"}
	    1       Literal{kind: Integer}
	      {}    StructLiteral.properties{dk: "{}"}                                                                                        */
	{ { } 2 },                                                                                                                            /*
	{•{•}•2•}    BlockExpression
	  {•}        ExpressionStatement{!semi}, BlockExpression
	      2      ExpressionStatement{!semi}, Literal{kind: Integer}                                                                       */
	&mut [0; 1][..],                                                                                                                      /*
	&mut•[0;•1][..]    ReferenceExpression{mut}
	     [0;•1][..]    MemberExpression{computed}
	     [0;•1]        SizedArrayLiteral
	      0            Literal{kind: Integer}
	         1         Literal{kind: Integer}
	            ..     RangeLiteral{!last}                                                                                                */
	&B::<T>::A[0],                                                                                                                        /*
	&B::<T>::A[0]    ReferenceExpression{!mut}
	 B::<T>::A[0]    MemberExpression{computed}
	 B::<T>::A       ExpressionPath
	 B::<T>          ExpressionTypeCast
	    <T>          ExpressionTypeCast.typeArguments{dk: "<>"}
	           0     Literal{kind: Integer}                                                                                               */
	&B::<T>::A.0[0],                                                                                                                      /*
	&B::<T>::A.0[0]    ReferenceExpression{!mut}
	 B::<T>::A.0[0]    MemberExpression{computed}
	 B::<T>::A.0       MemberExpression{!computed}
	 B::<T>::A         ExpressionPath
	 B::<T>            ExpressionTypeCast
	    <T>            ExpressionTypeCast.typeArguments{dk: "<>"}
	           0       Index
	             0     Literal{kind: Integer}                                                                                             */
	&(B::<T>::A.0).1[0],                                                                                                                  /*
	&(B::<T>::A.0).1[0]    ReferenceExpression{!mut}
	 (B::<T>::A.0).1[0]    MemberExpression{computed}
	 (B::<T>::A.0).1       MemberExpression{!computed}
	  B::<T>::A.0          MemberExpression{!computed}
	  B::<T>::A            ExpressionPath
	  B::<T>               ExpressionTypeCast
	     <T>               ExpressionTypeCast.typeArguments{dk: "<>"}
	            0          Index
	               1       Index
	                 0     Literal{kind: Integer}                                                                                         */
	[[0; 1]; 1],                                                                                                                          /*
	[[0;•1];•1]    SizedArrayLiteral
	 [0;•1]        SizedArrayLiteral
	  0            Literal{kind: Integer}
	     1         Literal{kind: Integer}
	         1     Literal{kind: Integer}                                                                                                 */
	std::ptr::null::<usize>().is_null(),                                                                                                  /*
	std::ptr::null::<usize>().is_null()    CallExpression
	std::ptr::null::<usize>()              CallExpression
	std::ptr::null                         ExpressionPath
	std::ptr                               ExpressionPath
	                <usize>                CallExpression.typeArguments{dk: "<>"}
	                       ()              CallExpression.arguments{dk: "()"}
	                                 ()    CallExpression.arguments{dk: "()"}                                                             */
	&ss.1,                                                                                                                                /*
	&ss.1    ReferenceExpression{!mut}
	 ss.1    MemberExpression{!computed}
	    1    Index                                                                                                                        */
	&raw mut foo.x.0..1,                                                                                                                  /*
	&raw•mut•foo.x.0..1    RangeLiteral{!last}
	&raw•mut•foo.x.0       RawReferenceExpression{kind: "mut"}
	         foo.x.0       MemberExpression{!computed}
	         foo.x         MemberExpression{!computed}
	               0       Index
	                  1    Literal{kind: Integer}                                                                                         */
	&mut **d,                                                                                                                             /*
	&mut•**d    ReferenceExpression{mut}
	     **d    DereferenceExpression
	      *d    DereferenceExpression                                                                                                     */
	[12, 34][0 + 1],                                                                                                                      /*
	[12,•34][0•+•1]    MemberExpression{computed}
	[12,•34]           ArrayLiteral
	 12                Literal{kind: Integer}
	     34            Literal{kind: Integer}
	         0•+•1     OperationExpression{tk: "+"}
	         0         Literal{kind: Integer}
	             1     Literal{kind: Integer}                                                                                             */
	g(f())(()),                                                                                                                           /*
	g(f())(())    CallExpression
	g(f())        CallExpression
	 (f())        CallExpression.arguments{dk: "()"}
	  f()         CallExpression
	   ()         CallExpression.arguments{dk: "()"}
	      (())    CallExpression.arguments{dk: "()"}
	       ()     TupleLiteral                                                                                                            */
];                                                                                                                                        /*
]     </ArrayLiteral>
];    </ExpressionStatement>                                                                                                              */


fn f() { s.e() .f(E::s) .f(|f| f.a()) .f(R::e) .e(|a| *a >= q) .d() }                                                                     /*
fn•f()•{•s.e()•.f(E::s)•.f(|f|•f.a())•.f(R::e)•.e(|a|•*a•>=•q)•.d()•}    FunctionDeclaration
    ()                                                                   FunctionDeclaration.parameters{dk: "()"}
       {•s.e()•.f(E::s)•.f(|f|•f.a())•.f(R::e)•.e(|a|•*a•>=•q)•.d()•}    FunctionDeclaration.body{dk: "{}"}
         s.e()•.f(E::s)•.f(|f|•f.a())•.f(R::e)•.e(|a|•*a•>=•q)•.d()      ExpressionStatement{!semi}, CallExpression
         s.e()•.f(E::s)•.f(|f|•f.a())•.f(R::e)•.e(|a|•*a•>=•q)           CallExpression
         s.e()•.f(E::s)•.f(|f|•f.a())•.f(R::e)                           CallExpression
         s.e()•.f(E::s)•.f(|f|•f.a())                                    CallExpression
         s.e()•.f(E::s)                                                  CallExpression
         s.e()                                                           CallExpression
            ()                                                           CallExpression.arguments{dk: "()"}
                 (E::s)                                                  CallExpression.arguments{dk: "()"}
                  E::s                                                   ExpressionPath
                          (|f|•f.a())                                    CallExpression.arguments{dk: "()"}
                           |f|•f.a()                                     ClosureFunctionExpression
                           |f|                                           ClosureFunctionExpression.parameters{dk: "||"}
                            f                                            ClosureFunctionParameterDeclaration
                               f.a()                                     CallExpression
                                  ()                                     CallExpression.arguments{dk: "()"}
                                        (R::e)                           CallExpression.arguments{dk: "()"}
                                         R::e                            ExpressionPath
                                                 (|a|•*a•>=•q)           CallExpression.arguments{dk: "()"}
                                                  |a|•*a•>=•q            ClosureFunctionExpression
                                                  |a|                    ClosureFunctionExpression.parameters{dk: "||"}
                                                   a                     ClosureFunctionParameterDeclaration
                                                      *a•>=•q            ComparisonExpression{tk: ">="}
                                                      *a                 DereferenceExpression
                                                                 ()      CallExpression.arguments{dk: "()"}                               */
fn f() { let q = E { r: f![] }; Q(Q(q)).s(|d| q.i(|mut d| { e.z(0); f.G = e; r }) ); }                                                    /*
fn•f()•{•let•q•=•E•{•r:•f![]•};•Q(Q(q)).s(|d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})•);•}    FunctionDeclaration
    ()                                                                                    FunctionDeclaration.parameters{dk: "()"}
       {•let•q•=•E•{•r:•f![]•};•Q(Q(q)).s(|d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})•);•}    FunctionDeclaration.body{dk: "{}"}
         let•q•=•E•{•r:•f![]•};                                                           LetVariableDeclaration
                 E•{•r:•f![]•}                                                            StructLiteral
                   {•r:•f![]•}                                                            StructLiteral.properties{dk: "{}"}
                     r:•f![]                                                              StructLiteralProperty
                        f![]                                                              MacroInvocation
                          []                                                              MacroInvocation.segments{dk: "[]"}
                                Q(Q(q)).s(|d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})•);      ExpressionStatement{semi}
                                Q(Q(q)).s(|d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})•)       CallExpression
                                Q(Q(q))                                                   CallExpression
                                 (Q(q))                                                   CallExpression.arguments{dk: "()"}
                                  Q(q)                                                    CallExpression
                                   (q)                                                    CallExpression.arguments{dk: "()"}
                                         (|d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})•)       CallExpression.arguments{dk: "()"}
                                          |d|•q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})         ClosureFunctionExpression
                                          |d|                                             ClosureFunctionExpression.parameters{dk: "||"}
                                           d                                              ClosureFunctionParameterDeclaration
                                              q.i(|mut•d|•{•e.z(0);•f.G•=•e;•r•})         CallExpression
                                                 (|mut•d|•{•e.z(0);•f.G•=•e;•r•})         CallExpression.arguments{dk: "()"}
                                                  |mut•d|•{•e.z(0);•f.G•=•e;•r•}          ClosureFunctionExpression
                                                  |mut•d|                                 ClosureFunctionExpression.parameters{dk: "||"}
                                                   mut•d                                  ClosureFunctionParameterDeclaration, PatternVariableDeclaration{!ref, mut}
                                                          {•e.z(0);•f.G•=•e;•r•}          BlockExpression
                                                            e.z(0);                       ExpressionStatement{semi}
                                                            e.z(0)                        CallExpression
                                                               (0)                        CallExpression.arguments{dk: "()"}
                                                                0                         Literal{kind: Integer}
                                                                    f.G•=•e;              ExpressionStatement{semi}
                                                                    f.G•=•e               ReassignmentExpression{tk: "="}
                                                                    f.G                   MemberExpression{!computed}
                                                                             r            ExpressionStatement{!semi}                      */
pub fn public_expr(_: [u8; a(0).0]) {}                                                                                                    /*
pub•fn•public_expr(_:•[u8;•a(0).0])•{}    FunctionDeclaration
pub                                       PubSpecifier
                  (_:•[u8;•a(0).0])       FunctionDeclaration.parameters{dk: "()"}
                   _:•[u8;•a(0).0]        FunctionParameterDeclaration
                   _                      WildcardPattern
                      [u8;•a(0).0]        TypeSizedArray
                           a(0).0         MemberExpression{!computed}
                           a(0)           CallExpression
                            (0)           CallExpression.arguments{dk: "()"}
                             0            Literal{kind: Integer}
                                0         Index
                                    {}    FunctionDeclaration.body{dk: "{}"}                                                              */
pub fn f() { return ::f(); }                                                                                                              /*
pub•fn•f()•{•return•::f();•}    FunctionDeclaration
pub                             PubSpecifier
        ()                      FunctionDeclaration.parameters{dk: "()"}
           {•return•::f();•}    FunctionDeclaration.body{dk: "{}"}
             return•::f();      ExpressionStatement{semi}
             return•::f()       ReturnExpression
                    ::f()       CallExpression
                    ::f         ExpressionPath
                       ()       CallExpression.arguments{dk: "()"}                                                                        */
fn f() -> isize {                                                                                                                         /*
fn•f()•->•isize•{↲    <FunctionDeclaration>
    ()                FunctionDeclaration.parameters{dk: "()"}
                {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                */
    (return 1, return 2)                                                                                                                  /*
    (return•1,•return•2)    ExpressionStatement{!semi}, TupleLiteral
     return•1               ReturnExpression
            1               Literal{kind: Integer}
               return•2     ReturnExpression
                      2     Literal{kind: Integer}                                                                                        */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
fn f(x: Box<isize>) -> Box<(Box<isize>,Box<isize>)> { box (x, x) }                                                                        /*
fn•f(x:•Box<isize>)•->•Box<(Box<isize>,Box<isize>)>•{•box•(x,•x)•}    FunctionDeclaration
    (x:•Box<isize>)                                                   FunctionDeclaration.parameters{dk: "()"}
     x:•Box<isize>                                                    FunctionParameterDeclaration
        Box<isize>                                                    TypeCall
           <isize>                                                    TypeCall.typeArguments{dk: "<>"}
                       Box<(Box<isize>,Box<isize>)>                   TypeCall
                          <(Box<isize>,Box<isize>)>                   TypeCall.typeArguments{dk: "<>"}
                           (Box<isize>,Box<isize>)                    TypeTuple
                            Box<isize>                                TypeCall
                               <isize>                                TypeCall.typeArguments{dk: "<>"}
                                       Box<isize>                     TypeCall
                                          <isize>                     TypeCall.typeArguments{dk: "<>"}
                                                    {•box•(x,•x)•}    FunctionDeclaration.body{dk: "{}"}
                                                      box•(x,•x)      ExpressionStatement{!semi}, BoxExpression
                                                          (x,•x)      TupleLiteral                                                        */
fn f<F>(f: F) -> isize where F: FnOnce(isize) -> isize {}                                                                                 /*
fn•f<F>(f:•F)•->•isize•where•F:•FnOnce(isize)•->•isize•{}    FunctionDeclaration
    <F>                                                      FunctionDeclaration.generics{dk: "<>"}
     F                                                       GenericTypeParameterDeclaration
       (f:•F)                                                FunctionDeclaration.parameters{dk: "()"}
        f:•F                                                 FunctionParameterDeclaration
                       where•F:•FnOnce(isize)•->•isize       FunctionDeclaration.whereBounds{dk: "None"}
                             F:•FnOnce(isize)•->•isize       WhereTypeBoundDeclaration
                                FnOnce(isize)•->•isize       TypeTraitBound{!maybeConst, !optional}, TypeFunction
                                      (isize)                TypeFunction.parameters{dk: "()"}
                                                       {}    FunctionDeclaration.body{dk: "{}"}                                           */
fn f() { if (return) { } }                                                                                                                /*
fn•f()•{•if•(return)•{•}•}    FunctionDeclaration
    ()                        FunctionDeclaration.parameters{dk: "()"}
       {•if•(return)•{•}•}    FunctionDeclaration.body{dk: "{}"}
         if•(return)•{•}      ExpressionStatement{!semi}, IfBlockExpression
             return           ReturnExpression
                     {•}      IfBlockExpression.body{dk: "{}"}                                                                            */
fn f() { b! { } c }                                                                                                                       /*
fn•f()•{•b!•{•}•c•}    FunctionDeclaration
    ()                 FunctionDeclaration.parameters{dk: "()"}
       {•b!•{•}•c•}    FunctionDeclaration.body{dk: "{}"}
         b!•{•}        ExpressionStatement{!semi}, MacroInvocation
            {•}        MacroInvocation.segments{dk: "{}"}
                c      ExpressionStatement{!semi}                                                                                         */
fn f<T: ToString>(arg: T) -> String {                                                                                                     /*
fn•f<T:•ToString>(arg:•T)•->•String•{↲    <FunctionDeclaration>
    <T:•ToString>                         FunctionDeclaration.generics{dk: "<>"}
     T:•ToString                          GenericTypeParameterDeclaration
        ToString                          TypeTraitBound{!maybeConst, !optional}
                 (arg:•T)                 FunctionDeclaration.parameters{dk: "()"}
                  arg:•T                  FunctionParameterDeclaration
                                    {↲    <FunctionDeclaration.body{dk: "{}"}>                                                            */
    return <T as ToString>::to_string(&arg);                                                                                              /*
    return•<T•as•ToString>::to_string(&arg);    ExpressionStatement{semi}
    return•<T•as•ToString>::to_string(&arg)     ReturnExpression
           <T•as•ToString>::to_string(&arg)     CallExpression
           <T•as•ToString>::to_string           ExpressionPath
           <T•as•ToString>                      ExpressionTypeSelector
                                     (&arg)     CallExpression.arguments{dk: "()"}
                                      &arg      ReferenceExpression{!mut}                                                                 */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
fn f<A:Clone + 'static>(a: A, b: u16) -> Box<dyn Invokable<A>+'static> {                                                                  /*
fn•f<A:Clone•+•'static>(a:•A,•b:•u16)•->•Box<dyn•Invokable<A>+'static>•{↲    <FunctionDeclaration>
    <A:Clone•+•'static>                                                      FunctionDeclaration.generics{dk: "<>"}
     A:Clone•+•'static                                                       GenericTypeParameterDeclaration
       Clone                                                                 TypeTraitBound{!maybeConst, !optional}
               'static                                                       LtStatic
                       (a:•A,•b:•u16)                                        FunctionDeclaration.parameters{dk: "()"}
                        a:•A                                                 FunctionParameterDeclaration
                              b:•u16                                         FunctionParameterDeclaration
                                         Box<dyn•Invokable<A>+'static>       TypeCall
                                            <dyn•Invokable<A>+'static>       TypeCall.typeArguments{dk: "<>"}
                                             dyn•Invokable<A>+'static        TypeDynBounds{dyn}
                                                 Invokable<A>                TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                          <A>                TypeCall.typeArguments{dk: "<>"}
                                                              'static        LtStatic
                                                                       {↲    <FunctionDeclaration.body{dk: "{}"}>                         */
    box Invoker { a: a, b: b, } as Box<dyn Invokable<A>+'static>                                                                          /*
    box•Invoker•{•a:•a,•b:•b,•}•as•Box<dyn•Invokable<A>+'static>    ExpressionStatement{!semi}, ExpressionAsTypeCast
    box•Invoker•{•a:•a,•b:•b,•}                                     BoxExpression
        Invoker•{•a:•a,•b:•b,•}                                     StructLiteral
                {•a:•a,•b:•b,•}                                     StructLiteral.properties{dk: "{}"}
                  a:•a                                              StructLiteralProperty
                        b:•b                                        StructLiteralProperty
                                   Box<dyn•Invokable<A>+'static>    TypeCall
                                      <dyn•Invokable<A>+'static>    TypeCall.typeArguments{dk: "<>"}
                                       dyn•Invokable<A>+'static     TypeDynBounds{dyn}
                                           Invokable<A>             TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                    <A>             TypeCall.typeArguments{dk: "<>"}
                                                        'static     LtStatic                                                              */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
fn f() { (return 1, return 2) }                                                                                                           /*
fn•f()•{•(return•1,•return•2)•}    FunctionDeclaration
    ()                             FunctionDeclaration.parameters{dk: "()"}
       {•(return•1,•return•2)•}    FunctionDeclaration.body{dk: "{}"}
         (return•1,•return•2)      ExpressionStatement{!semi}, TupleLiteral
          return•1                 ReturnExpression
                 1                 Literal{kind: Integer}
                    return•2       ReturnExpression
                           2       Literal{kind: Integer}                                                                                 */
pub trait Foo: Iterator<Item=<Self as Foo>::Key>{}                                                                                        /*
pub•trait•Foo:•Iterator<Item=<Self•as•Foo>::Key>{}    TraitDeclaration
pub                                                   PubSpecifier
               Iterator<Item=<Self•as•Foo>::Key>      TypeTraitBound{!maybeConst, !optional}, TypeCall
                       <Item=<Self•as•Foo>::Key>      TypeCall.typeArguments{dk: "<>"}
                        Item=<Self•as•Foo>::Key       TypeCallNamedArgument
                             <Self•as•Foo>::Key       TypePath
                             <Self•as•Foo>            ExpressionTypeSelector
                                                {}    TraitDeclaration.body{dk: "{}"}                                                     */
fn f() {::m!(S, x);}                                                                                                                      /*
fn•f()•{::m!(S,•x);}    FunctionDeclaration
    ()                  FunctionDeclaration.parameters{dk: "()"}
       {::m!(S,•x);}    FunctionDeclaration.body{dk: "{}"}
        ::m!(S,•x);     ExpressionStatement{semi}
        ::m!(S,•x)      MacroInvocation
        ::m             ExpressionPath
            (S,•x)      MacroInvocation.segments{dk: "()"}
              ,         PunctuationToken{tk: ","}
fn•f()•{::m!(S,•x);}    </Program.ast>
fn•f()•{::m!(S,•x);}    </Program>                                                                                                        */
// Discarded Nodes: 28
// Parsed Nodes: 1736
// state_rollbacks: 15
// Total '.charCodeAt()' calls: 7447 (35% re-reads)
// Unnecessary 'skip_whitespace()' calls: 1016
// source: "../../samples/expressions/expr.rs"