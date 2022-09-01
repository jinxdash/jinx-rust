fn a() {                                                                                                                                  /*
fn•a()•{↲    <Program>
fn•a()•{↲    <Program.ast{dk: "None"}>
fn•a()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	fn eq(&&other: S) { false }                                                                                                           /*
	fn•eq(&&other:•S)•{•false•}    FunctionDeclaration
	     (&&other:•S)              FunctionDeclaration.parameters{dk: "()"}
	      &&other:•S               FunctionParameterDeclaration
	      &&other                  ReferencePattern{!mut}
	       &other                  ReferencePattern{!mut}
	                  {•false•}    FunctionDeclaration.body{dk: "{}"}
	                    false      ExpressionStatement{!semi}, Literal{kind: False}                                                       */
    let -2147483648..=2147483647 = 1;                                                                                                     /*
    let•-2147483648..=2147483647•=•1;    LetVariableDeclaration
        -2147483648..=2147483647         RangePattern{last}
        -2147483648                      MinusPattern
         2147483648                      Literal{kind: Integer}
                      2147483647         Literal{kind: Integer}
                                   1     Literal{kind: Integer}                                                                           */
    let 0..=255 = 0u8;                                                                                                                    /*
    let•0..=255•=•0u8;    LetVariableDeclaration
        0..=255           RangePattern{last}
        0                 Literal{kind: Integer}
            255           Literal{kind: Integer}
                  0u8     Literal{kind: Integer}
                   u8     Identifier                                                                                                      */
    let -128..=127 = 0i8;                                                                                                                 /*
    let•-128..=127•=•0i8;    LetVariableDeclaration
        -128..=127           RangePattern{last}
        -128                 MinusPattern
         128                 Literal{kind: Integer}
               127           Literal{kind: Integer}
                     0i8     Literal{kind: Integer}
                      i8     Identifier                                                                                                   */
    let '\u{0000}'..='\u{10FFFF}' = 'v';                                                                                                  /*
    let•'\u{0000}'..='\u{10FFFF}'•=•'v';    LetVariableDeclaration
        '\u{0000}'..='\u{10FFFF}'           RangePattern{last}
        '\u{0000}'                          Literal{kind: Char}
                     '\u{10FFFF}'           Literal{kind: Char}
                                    'v'     Literal{kind: Char}                                                                           */
	let f = |3: isize| println!("hello");                                                                                                 /*
	let•f•=•|3:•isize|•println!("hello");    LetVariableDeclaration
	        |3:•isize|•println!("hello")     ClosureFunctionExpression
	        |3:•isize|                       ClosureFunctionExpression.parameters{dk: "||"}
	         3:•isize                        ClosureFunctionParameterDeclaration
	         3                               Literal{kind: Integer}
	                   println!("hello")     MacroInvocation
	                           ("hello")     MacroInvocation.segments{dk: "()"}
	                            "hello"      Literal{kind: String}                                                                        */

	match *self {                                                                                                                         /*
	match•*self•{↲    <ExpressionStatement{!semi}>
	match•*self•{↲    <MatchExpression>
	      *self       DereferenceExpression
	            {↲    <MatchExpression.cases{dk: "{}"}>                                                                                   */
		Foo::<T>(ref x, ref y) => x                                                                                                       /*
		Foo::<T>(ref•x,•ref•y)•=>•x    MatchExpressionCase
		Foo::<T>(ref•x,•ref•y)         TuplePattern
		Foo::<T>                       ExpressionTypeCast
		     <T>                       ExpressionTypeCast.typeArguments{dk: "<>"}
		        (ref•x,•ref•y)         TuplePattern.items{dk: "()"}
		         ref•x                 PatternVariableDeclaration{ref, !mut}
		                ref•y          PatternVariableDeclaration{ref, !mut}                                                              */
	}                                                                                                                                     /*
   ╚}    </MatchExpression.cases>
   ╚}    </MatchExpression>
   ╚}    </ExpressionStatement>                                                                                                           */

	let A { foo, } = mka();                                                                                                               /*
	let•A•{•foo,•}•=•mka();    LetVariableDeclaration
	    A•{•foo,•}             StructPattern
	      {•foo,•}             StructPattern.properties{dk: "{}"}
	        foo                StructPatternPropertyShorthand{!box, !ref, !mut}
	                 mka()     CallExpression
	                    ()     CallExpression.arguments{dk: "()"}                                                                         */
    let A {                                                                                                                               /*
    let•A•{↲    <LetVariableDeclaration>
        A•{↲    <StructPattern>
          {↲    <StructPattern.properties{dk: "{}"}>                                                                                      */
        foo,                                                                                                                              /*
        foo    StructPatternPropertyShorthand{!box, !ref, !mut}                                                                           */
    } = mka();                                                                                                                            /*
••••}             </StructPattern.properties>
••••}             </StructPattern>
        mka()     CallExpression
           ()     CallExpression.arguments{dk: "()"}
••••}•=•mka();    </LetVariableDeclaration>                                                                                               */

    let B { a, b, c, } = mkb();                                                                                                           /*
    let•B•{•a,•b,•c,•}•=•mkb();    LetVariableDeclaration
        B•{•a,•b,•c,•}             StructPattern
          {•a,•b,•c,•}             StructPattern.properties{dk: "{}"}
            a                      StructPatternPropertyShorthand{!box, !ref, !mut}
               b                   StructPatternPropertyShorthand{!box, !ref, !mut}
                  c                StructPatternPropertyShorthand{!box, !ref, !mut}
                         mkb()     CallExpression
                            ()     CallExpression.arguments{dk: "()"}                                                                     */

    match mka() {                                                                                                                         /*
    match•mka()•{↲    <ExpressionStatement{!semi}>
    match•mka()•{↲    <MatchExpression>
          mka()       CallExpression
             ()       CallExpression.arguments{dk: "()"}
                {↲    <MatchExpression.cases{dk: "{}"}>                                                                                   */
        A { foo: _foo, } => {}                                                                                                            /*
        A•{•foo:•_foo,•}•=>•{}    MatchExpressionCase
        A•{•foo:•_foo,•}          StructPattern
          {•foo:•_foo,•}          StructPattern.properties{dk: "{}"}
            foo:•_foo             StructPatternPropertyDestructured
                            {}    BlockExpression                                                                                         */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */

    match Some(mka()) {                                                                                                                   /*
    match•Some(mka())•{↲    <ExpressionStatement{!semi}>
    match•Some(mka())•{↲    <MatchExpression>
          Some(mka())       CallExpression
              (mka())       CallExpression.arguments{dk: "()"}
               mka()        CallExpression
                  ()        CallExpression.arguments{dk: "()"}
                      {↲    <MatchExpression.cases{dk: "{}"}>                                                                             */
		S { .. } => (),                                                                                                                   /*
		S•{•..•}•=>•()    MatchExpressionCase
		S•{•..•}          StructPattern
		  {•..•}          StructPattern.properties{dk: "{}"}
		    ..            RestPattern
		            ()    TupleLiteral                                                                                                    */
        Some(A { foo: _foo, }) => {}                                                                                                      /*
        Some(A•{•foo:•_foo,•})•=>•{}    MatchExpressionCase
        Some(A•{•foo:•_foo,•})          TuplePattern
            (A•{•foo:•_foo,•})          TuplePattern.items{dk: "()"}
             A•{•foo:•_foo,•}           StructPattern
               {•foo:•_foo,•}           StructPattern.properties{dk: "{}"}
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
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
	match (x, y) {                                                                                                                        /*
	match•(x,•y)•{↲    <ExpressionStatement{!semi}>
	match•(x,•y)•{↲    <MatchExpression>
	      (x,•y)       TupleLiteral
	             {↲    <MatchExpression.cases{dk: "{}"}>                                                                                  */
        (1, 1) => 1,                                                                                                                      /*
        (1,•1)•=>•1    MatchExpressionCase
        (1,•1)         TuplePattern
         1             Literal{kind: Integer}
            1          Literal{kind: Integer}
                  1    Literal{kind: Integer}                                                                                             */
        (2, 2) => 2,                                                                                                                      /*
        (2,•2)•=>•2    MatchExpressionCase
        (2,•2)         TuplePattern
         2             Literal{kind: Integer}
            2          Literal{kind: Integer}
                  2    Literal{kind: Integer}                                                                                             */
        (1..=2, 2) => 3,                                                                                                                  /*
        (1..=2,•2)•=>•3    MatchExpressionCase
        (1..=2,•2)         TuplePattern
         1..=2             RangePattern{last}
         1                 Literal{kind: Integer}
             2             Literal{kind: Integer}
                2          Literal{kind: Integer}
                      3    Literal{kind: Integer}                                                                                         */
        _ => 4,                                                                                                                           /*
        _•=>•4    MatchExpressionCase
        _         WildcardPattern
             4    Literal{kind: Integer}                                                                                                  */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */

	if let Some([b'@', filename @ ..]) = Some(b"@abc123") {                                                                               /*
	if•let•Some([b'@',•filename•@•..])•=•Some(b"@abc123")•{↲    <ExpressionStatement{!semi}>
	if•let•Some([b'@',•filename•@•..])•=•Some(b"@abc123")•{↲    <IfBlockExpression>
	   let•Some([b'@',•filename•@•..])•=•Some(b"@abc123")       LetScrutinee
	       Some([b'@',•filename•@•..])                          TuplePattern
	           ([b'@',•filename•@•..])                          TuplePattern.items{dk: "()"}
	            [b'@',•filename•@•..]                           ArrayPattern
	             b'@'                                           Literal{kind: bChar}
	                   filename•@•..                            PatternVariableDeclaration{!ref, !mut}
	                              ..                            RestPattern
	                                     Some(b"@abc123")       CallExpression
	                                         (b"@abc123")       CallExpression.arguments{dk: "()"}
	                                          b"@abc123"        Literal{kind: bString}
	                                                      {↲    <IfBlockExpression.body{dk: "{}"}>                                        */
        println!("filename {:?}", filename);                                                                                              /*
        println!("filename•{:?}",•filename);    ExpressionStatement{semi}
        println!("filename•{:?}",•filename)     MacroInvocation
                ("filename•{:?}",•filename)     MacroInvocation.segments{dk: "()"}
                 "filename•{:?}"                Literal{kind: String}
                                ,               PunctuationToken{tk: ","}                                                                 */
    }                                                                                                                                     /*
••••}    </IfBlockExpression.body>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */

	
	fn f(X(_): A) {}                                                                                                                      /*
	fn•f(X(_):•A)•{}    FunctionDeclaration
	    (X(_):•A)       FunctionDeclaration.parameters{dk: "()"}
	     X(_):•A        FunctionParameterDeclaration
	     X(_)           TuplePattern
	      (_)           TuplePattern.items{dk: "()"}
	       _            WildcardPattern
	              {}    FunctionDeclaration.body{dk: "{}"}                                                                                */

	let Ok(0): Option<u8> = 42u8;                                                                                                         /*
	let•Ok(0):•Option<u8>•=•42u8;    LetVariableDeclaration
	    Ok(0)                        TuplePattern
	      (0)                        TuplePattern.items{dk: "()"}
	       0                         Literal{kind: Integer}
	           Option<u8>            TypeCall
	                 <u8>            TypeCall.typeArguments{dk: "<>"}
	                        42u8     Literal{kind: Integer}
	                          u8     Identifier                                                                                           */
	let Ok(0): Option<u8>;                                                                                                                /*
	let•Ok(0):•Option<u8>;    LetVariableDeclaration
	    Ok(0)                 TuplePattern
	      (0)                 TuplePattern.items{dk: "()"}
	       0                  Literal{kind: Integer}
	           Option<u8>     TypeCall
	                 <u8>     TypeCall.typeArguments{dk: "<>"}                                                                            */
	let Ok(0) = 42u8;                                                                                                                     /*
	let•Ok(0)•=•42u8;    LetVariableDeclaration
	    Ok(0)            TuplePattern
	      (0)            TuplePattern.items{dk: "()"}
	       0             Literal{kind: Integer}
	            42u8     Literal{kind: Integer}
	              u8     Identifier                                                                                                       */

	

	match t {                                                                                                                             /*
	match•t•{•↲    <ExpressionStatement{!semi}>
	match•t•{•↲    <MatchExpression>
	        {•↲    <MatchExpression.cases{dk: "{}"}>                                                                                      */
		Bar::T1(_, Some(x)) => {                                                                                                          /*
		Bar::T1(_,•Some(x))•=>•{•↲    <MatchExpressionCase>
		Bar::T1(_,•Some(x))           TuplePattern
		Bar::T1                       ExpressionPath
		       (_,•Some(x))           TuplePattern.items{dk: "()"}
		        _                     WildcardPattern
		           Some(x)            TuplePattern
		               (x)            TuplePattern.items{dk: "()"}
		                       {•↲    <BlockExpression>                                                                                   */
			return x * 3;                                                                                                                 /*
			return•x•*•3;    ExpressionStatement{semi}
			return•x•*•3     ReturnExpression
			       x•*•3     OperationExpression{tk: "*"}
			           3     Literal{kind: Integer}                                                                                       */
		}                                                                                                                                 /*
      ╚╚}    </BlockExpression>
      ╚╚}    </MatchExpressionCase>                                                                                                       */
		_ => { panic!(); }                                                                                                                /*
		_•=>•{•panic!();•}    MatchExpressionCase
		_                     WildcardPattern
		     {•panic!();•}    BlockExpression
		       panic!();      ExpressionStatement{semi}
		       panic!()       MacroInvocation
		             ()       MacroInvocation.segments{dk: "()"}                                                                          */
	}                                                                                                                                     /*
   ╚}    </MatchExpression.cases>
   ╚}    </MatchExpression>
   ╚}    </ExpressionStatement>                                                                                                           */

	match t {                                                                                                                             /*
	match•t•{↲    <ExpressionStatement{!semi}>
	match•t•{↲    <MatchExpression>
	        {↲    <MatchExpression.cases{dk: "{}"}>                                                                                       */
		Bar::T1(_, Some::<isize>(x)) => {                                                                                                 /*
		Bar::T1(_,•Some::<isize>(x))•=>•{↲    <MatchExpressionCase>
		Bar::T1(_,•Some::<isize>(x))          TuplePattern
		Bar::T1                               ExpressionPath
		       (_,•Some::<isize>(x))          TuplePattern.items{dk: "()"}
		        _                             WildcardPattern
		           Some::<isize>(x)           TuplePattern
		           Some::<isize>              ExpressionTypeCast
		                 <isize>              ExpressionTypeCast.typeArguments{dk: "<>"}
		                        (x)           TuplePattern.items{dk: "()"}
		                                {↲    <BlockExpression>                                                                           */
		  println!("{}", x);                                                                                                              /*
		  println!("{}",•x);    ExpressionStatement{semi}
		  println!("{}",•x)     MacroInvocation
		          ("{}",•x)     MacroInvocation.segments{dk: "()"}
		           "{}"         Literal{kind: String}
		               ,        PunctuationToken{tk: ","}                                                                                 */
		}                                                                                                                                 /*
      ╚╚}    </BlockExpression>
      ╚╚}    </MatchExpressionCase>                                                                                                       */
		_ => { panic!(); }                                                                                                                /*
		_•=>•{•panic!();•}    MatchExpressionCase
		_                     WildcardPattern
		     {•panic!();•}    BlockExpression
		       panic!();      ExpressionStatement{semi}
		       panic!()       MacroInvocation
		             ()       MacroInvocation.segments{dk: "()"}                                                                          */
	}                                                                                                                                     /*
   ╚}    </MatchExpression.cases>
   ╚}    </MatchExpression>
   ╚}    </ExpressionStatement>                                                                                                           */

	match unimplemented!() {                                                                                                              /*
	match•unimplemented!()•{↲    <ExpressionStatement{!semi}>
	match•unimplemented!()•{↲    <MatchExpression>
	      unimplemented!()       MacroInvocation
	                    ()       MacroInvocation.segments{dk: "()"}
	                       {↲    <MatchExpression.cases{dk: "{}"}>                                                                        */
        &&&42 => {},                                                                                                                      /*
        &&&42•=>•{}    MatchExpressionCase
        &&&42          ReferencePattern{!mut}
         &&42          ReferencePattern{!mut}
          &42          ReferencePattern{!mut}
           42          Literal{kind: Integer}
                 {}    BlockExpression                                                                                                    */
        FOO => {},                                                                                                                        /*
        FOO•=>•{}    MatchExpressionCase
               {}    BlockExpression                                                                                                      */
        _ => {},                                                                                                                          /*
        _•=>•{}    MatchExpressionCase
        _          WildcardPattern
             {}    BlockExpression                                                                                                        */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
	fn f4(ref a @ box ref b: Box<NC>) {}                                                                                                  /*
	fn•f4(ref•a•@•box•ref•b:•Box<NC>)•{}    FunctionDeclaration
	     (ref•a•@•box•ref•b:•Box<NC>)       FunctionDeclaration.parameters{dk: "()"}
	      ref•a•@•box•ref•b:•Box<NC>        FunctionParameterDeclaration
	      ref•a•@•box•ref•b                 PatternVariableDeclaration{ref, !mut}
	              box•ref•b                 BoxPattern
	                  ref•b                 PatternVariableDeclaration{ref, !mut}
	                         Box<NC>        TypeCall
	                            <NC>        TypeCall.typeArguments{dk: "<>"}
	                                  {}    FunctionDeclaration.body{dk: "{}"}                                                            */
    fn f1(a @ ref b: U) {}                                                                                                                /*
    fn•f1(a•@•ref•b:•U)•{}    FunctionDeclaration
         (a•@•ref•b:•U)       FunctionDeclaration.parameters{dk: "()"}
          a•@•ref•b:•U        FunctionParameterDeclaration
          a•@•ref•b           PatternVariableDeclaration{!ref, !mut}
              ref•b           PatternVariableDeclaration{ref, !mut}
                        {}    FunctionDeclaration.body{dk: "{}"}                                                                          */
	fn _f(_a @ _b: u8) {}                                                                                                                 /*
	fn•_f(_a•@•_b:•u8)•{}    FunctionDeclaration
	     (_a•@•_b:•u8)       FunctionDeclaration.parameters{dk: "()"}
	      _a•@•_b:•u8        FunctionParameterDeclaration
	      _a•@•_b            PatternVariableDeclaration{!ref, !mut}
	                   {}    FunctionDeclaration.body{dk: "{}"}                                                                           */
	let s: &[bool] = &[true];                                                                                                             /*
	let•s:•&[bool]•=•&[true];    LetVariableDeclaration
	       &[bool]               TypeReference{!mut}
	        [bool]               TypeSlice
	                 &[true]     ReferenceExpression{!mut}
	                  [true]     ArrayLiteral
	                   true      Literal{kind: True}                                                                                      */
    let s0: &[bool; 0] = &[];                                                                                                             /*
    let•s0:•&[bool;•0]•=•&[];    LetVariableDeclaration
            &[bool;•0]           TypeReference{!mut}
             [bool;•0]           TypeSizedArray
                    0            Literal{kind: Integer}
                         &[]     ReferenceExpression{!mut}
                          []     ArrayLiteral                                                                                             */
    let s1: &[bool; 1] = &[false; 1];                                                                                                     /*
    let•s1:•&[bool;•1]•=•&[false;•1];    LetVariableDeclaration
            &[bool;•1]                   TypeReference{!mut}
             [bool;•1]                   TypeSizedArray
                    1                    Literal{kind: Integer}
                         &[false;•1]     ReferenceExpression{!mut}
                          [false;•1]     SizedArrayLiteral
                           false         Literal{kind: False}
                                  1      Literal{kind: Integer}                                                                           */
    let s2: &[bool; 2] = &[false; 2];                                                                                                     /*
    let•s2:•&[bool;•2]•=•&[false;•2];    LetVariableDeclaration
            &[bool;•2]                   TypeReference{!mut}
             [bool;•2]                   TypeSizedArray
                    2                    Literal{kind: Integer}
                         &[false;•2]     ReferenceExpression{!mut}
                          [false;•2]     SizedArrayLiteral
                           false         Literal{kind: False}
                                  2      Literal{kind: Integer}                                                                           */
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
	while•let•0..=2•|•1•=•0•{}    ExpressionStatement{!semi}, WhileBlockExpression
	      let•0..=2•|•1•=•0       LetScrutinee
	          0..=2•|•1           UnionPattern
	          0..=2               RangePattern{last}
	          0                   Literal{kind: Integer}
	              2               Literal{kind: Integer}
	                  1           Literal{kind: Integer}
	                      0       Literal{kind: Integer}
	                        {}    WhileBlockExpression.body{dk: "{}"}                                                                     */
	if let 0..=2 | 1 = 0 {}                                                                                                               /*
	if•let•0..=2•|•1•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0..=2•|•1•=•0       LetScrutinee
	       0..=2•|•1           UnionPattern
	       0..=2               RangePattern{last}
	       0                   Literal{kind: Integer}
	           2               Literal{kind: Integer}
	               1           Literal{kind: Integer}
	                   0       Literal{kind: Integer}
	                     {}    IfBlockExpression.body{dk: "{}"}                                                                           */
	match 0u8 { 0 | 0 => {} }                                                                                                             /*
	match•0u8•{•0•|•0•=>•{}•}    ExpressionStatement{!semi}, MatchExpression
	      0u8                    Literal{kind: Integer}
	       u8                    Identifier
	          {•0•|•0•=>•{}•}    MatchExpression.cases{dk: "{}"}
	            0•|•0•=>•{}      MatchExpressionCase
	            0•|•0            UnionPattern
	            0                Literal{kind: Integer}
	                0            Literal{kind: Integer}
	                     {}      BlockExpression                                                                                          */
	if let (0 | 0) = 0 {} else { return };                                                                                                /*
	if•let•(0•|•0)•=•0•{}•else•{•return•};    ExpressionStatement{semi}
	if•let•(0•|•0)•=•0•{}•else•{•return•}     IfBlockExpression
	   let•(0•|•0)•=•0                        LetScrutinee
	        0•|•0                             UnionPattern
	        0                                 Literal{kind: Integer}
	            0                             Literal{kind: Integer}
	                 0                        Literal{kind: Integer}
	                   {}                     IfBlockExpression.body{dk: "{}"}
	                           {•return•}     BlockExpression
	                             return       ExpressionStatement{!semi}, ReturnExpression                                                */
	let mut arr = [U, U, U, U, U, U, U, U];                                                                                               /*
	let•mut•arr•=•[U,•U,•U,•U,•U,•U,•U,•U];    LetVariableDeclaration
	    mut•arr                                PatternVariableDeclaration{!ref, mut}
	              [U,•U,•U,•U,•U,•U,•U,•U]     ArrayLiteral                                                                               */
	let mut tup = (U, U, U, U, U);                                                                                                        /*
	let•mut•tup•=•(U,•U,•U,•U,•U);    LetVariableDeclaration
	    mut•tup                       PatternVariableDeclaration{!ref, mut}
	              (U,•U,•U,•U,•U)     TupleLiteral                                                                                        */
	let (Ok((V1(a) | V2(a) | V3(a), b)) | Err(Ok((a, b)) | Err((a, b)))): Result<_, Result<_, _>> = Ok((V1(1), 1));                       /*
	let•(Ok((V1(a)•|•V2(a)•|•V3(a),•b))•|•Err(Ok((a,•b))•|•Err((a,•b)))):•Result<_,•Result<_,•_>>•=•Ok((V1(1),•1));    LetVariableDeclaration
	     Ok((V1(a)•|•V2(a)•|•V3(a),•b))•|•Err(Ok((a,•b))•|•Err((a,•b)))                                                UnionPattern
	     Ok((V1(a)•|•V2(a)•|•V3(a),•b))                                                                                TuplePattern
	       ((V1(a)•|•V2(a)•|•V3(a),•b))                                                                                TuplePattern.items{dk: "()"}
	        (V1(a)•|•V2(a)•|•V3(a),•b)                                                                                 TuplePattern
	         V1(a)•|•V2(a)•|•V3(a)                                                                                     UnionPattern
	         V1(a)                                                                                                     TuplePattern
	           (a)                                                                                                     TuplePattern.items{dk: "()"}
	                 V2(a)                                                                                             TuplePattern
	                   (a)                                                                                             TuplePattern.items{dk: "()"}
	                         V3(a)                                                                                     TuplePattern
	                           (a)                                                                                     TuplePattern.items{dk: "()"}
	                                      Err(Ok((a,•b))•|•Err((a,•b)))                                                TuplePattern
	                                         (Ok((a,•b))•|•Err((a,•b)))                                                TuplePattern.items{dk: "()"}
	                                          Ok((a,•b))•|•Err((a,•b))                                                 UnionPattern
	                                          Ok((a,•b))                                                               TuplePattern
	                                            ((a,•b))                                                               TuplePattern.items{dk: "()"}
	                                             (a,•b)                                                                TuplePattern
	                                                       Err((a,•b))                                                 TuplePattern
	                                                          ((a,•b))                                                 TuplePattern.items{dk: "()"}
	                                                           (a,•b)                                                  TuplePattern
	                                                                      Result<_,•Result<_,•_>>                      TypeCall
	                                                                            <_,•Result<_,•_>>                      TypeCall.typeArguments{dk: "<>"}
	                                                                             _                                     TypeInferred
	                                                                                Result<_,•_>                       TypeCall
	                                                                                      <_,•_>                       TypeCall.typeArguments{dk: "<>"}
	                                                                                       _                           TypeInferred
	                                                                                          _                        TypeInferred
	                                                                                                Ok((V1(1),•1))     CallExpression
	                                                                                                  ((V1(1),•1))     CallExpression.arguments{dk: "()"}
	                                                                                                   (V1(1),•1)      TupleLiteral
	                                                                                                    V1(1)          CallExpression
	                                                                                                      (1)          CallExpression.arguments{dk: "()"}
	                                                                                                       1           Literal{kind: Integer}
	                                                                                                           1       Literal{kind: Integer}*/
    let (Ok((V1(a) | V2(a) | V3(a), ref b)) | Err(Ok((a, ref b)) | Err((a, ref b)))): Result< _, Result<_, _>, > = Ok((V1(1), 1));        /*
    let•(Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))•|•Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))):•Result<•_,•Result<_,•_>,•>•=•Ok((V1(1),•1));    LetVariableDeclaration
         Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))•|•Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))                                                   UnionPattern
         Ok((V1(a)•|•V2(a)•|•V3(a),•ref•b))                                                                                           TuplePattern
           ((V1(a)•|•V2(a)•|•V3(a),•ref•b))                                                                                           TuplePattern.items{dk: "()"}
            (V1(a)•|•V2(a)•|•V3(a),•ref•b)                                                                                            TuplePattern
             V1(a)•|•V2(a)•|•V3(a)                                                                                                    UnionPattern
             V1(a)                                                                                                                    TuplePattern
               (a)                                                                                                                    TuplePattern.items{dk: "()"}
                     V2(a)                                                                                                            TuplePattern
                       (a)                                                                                                            TuplePattern.items{dk: "()"}
                             V3(a)                                                                                                    TuplePattern
                               (a)                                                                                                    TuplePattern.items{dk: "()"}
                                    ref•b                                                                                             PatternVariableDeclaration{ref, !mut}
                                              Err(Ok((a,•ref•b))•|•Err((a,•ref•b)))                                                   TuplePattern
                                                 (Ok((a,•ref•b))•|•Err((a,•ref•b)))                                                   TuplePattern.items{dk: "()"}
                                                  Ok((a,•ref•b))•|•Err((a,•ref•b))                                                    UnionPattern
                                                  Ok((a,•ref•b))                                                                      TuplePattern
                                                    ((a,•ref•b))                                                                      TuplePattern.items{dk: "()"}
                                                     (a,•ref•b)                                                                       TuplePattern
                                                         ref•b                                                                        PatternVariableDeclaration{ref, !mut}
                                                                   Err((a,•ref•b))                                                    TuplePattern
                                                                      ((a,•ref•b))                                                    TuplePattern.items{dk: "()"}
                                                                       (a,•ref•b)                                                     TuplePattern
                                                                           ref•b                                                      PatternVariableDeclaration{ref, !mut}
                                                                                      Result<•_,•Result<_,•_>,•>                      TypeCall
                                                                                            <•_,•Result<_,•_>,•>                      TypeCall.typeArguments{dk: "<>"}
                                                                                              _                                       TypeInferred
                                                                                                 Result<_,•_>                         TypeCall
                                                                                                       <_,•_>                         TypeCall.typeArguments{dk: "<>"}
                                                                                                        _                             TypeInferred
                                                                                                           _                          TypeInferred
                                                                                                                   Ok((V1(1),•1))     CallExpression
                                                                                                                     ((V1(1),•1))     CallExpression.arguments{dk: "()"}
                                                                                                                      (V1(1),•1)      TupleLiteral
                                                                                                                       V1(1)          CallExpression
                                                                                                                         (1)          CallExpression.arguments{dk: "()"}
                                                                                                                          1           Literal{kind: Integer}
                                                                                                                              1       Literal{kind: Integer}*/
	let (a, Err((ref mut b, ref c, d)) | Ok((                                                                                             /*
	let•(a,•Err((ref•mut•b,•ref•c,•d))•|•Ok((↲    <LetVariableDeclaration>
	    (a,•Err((ref•mut•b,•ref•c,•d))•|•Ok((↲    <TuplePattern>
	        Err((ref•mut•b,•ref•c,•d))•|•Ok((↲    <UnionPattern>
	        Err((ref•mut•b,•ref•c,•d))            TuplePattern
	           ((ref•mut•b,•ref•c,•d))            TuplePattern.items{dk: "()"}
	            (ref•mut•b,•ref•c,•d)             TuplePattern
	             ref•mut•b                        PatternVariableDeclaration{ref, mut}
	                        ref•c                 PatternVariableDeclaration{ref, !mut}
	                                     Ok((↲    <TuplePattern>
	                                       ((↲    <TuplePattern.items{dk: "()"}>
	                                        (↲    <TuplePattern>                                                                          */
        Ok(V1((ref c, d)) | V2((d, ref c)) | V3((ref c, Ok((_, d)) | Err((d, _))))) | Err((ref c, d)), ref mut b,                         /*
        Ok(V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))•|•Err((ref•c,•d))               UnionPattern
        Ok(V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))                                 TuplePattern
          (V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_)))))                                 TuplePattern.items{dk: "()"}
           V1((ref•c,•d))•|•V2((d,•ref•c))•|•V3((ref•c,•Ok((_,•d))•|•Err((d,•_))))                                  UnionPattern
           V1((ref•c,•d))                                                                                           TuplePattern
             ((ref•c,•d))                                                                                           TuplePattern.items{dk: "()"}
              (ref•c,•d)                                                                                            TuplePattern
               ref•c                                                                                                PatternVariableDeclaration{ref, !mut}
                            V2((d,•ref•c))                                                                          TuplePattern
                              ((d,•ref•c))                                                                          TuplePattern.items{dk: "()"}
                               (d,•ref•c)                                                                           TuplePattern
                                   ref•c                                                                            PatternVariableDeclaration{ref, !mut}
                                             V3((ref•c,•Ok((_,•d))•|•Err((d,•_))))                                  TuplePattern
                                               ((ref•c,•Ok((_,•d))•|•Err((d,•_))))                                  TuplePattern.items{dk: "()"}
                                                (ref•c,•Ok((_,•d))•|•Err((d,•_)))                                   TuplePattern
                                                 ref•c                                                              PatternVariableDeclaration{ref, !mut}
                                                        Ok((_,•d))•|•Err((d,•_))                                    UnionPattern
                                                        Ok((_,•d))                                                  TuplePattern
                                                          ((_,•d))                                                  TuplePattern.items{dk: "()"}
                                                           (_,•d)                                                   TuplePattern
                                                            _                                                       WildcardPattern
                                                                     Err((d,•_))                                    TuplePattern
                                                                        ((d,•_))                                    TuplePattern.items{dk: "()"}
                                                                         (d,•_)                                     TuplePattern
                                                                             _                                      WildcardPattern
                                                                                      Err((ref•c,•d))               TuplePattern
                                                                                         ((ref•c,•d))               TuplePattern.items{dk: "()"}
                                                                                          (ref•c,•d)                TuplePattern
                                                                                           ref•c                    PatternVariableDeclaration{ref, !mut}
                                                                                                       ref•mut•b    PatternVariableDeclaration{ref, mut}*/
    )),): (_, Result<_, _>) = (1, Ok((Ok(V3((1, Ok::<_, (i32, i32)>((1, 1))))), 1)));                                                     /*
••••)                                                                                    </TuplePattern>
••••))                                                                                   </TuplePattern.items>
••••))                                                                                   </TuplePattern>
••••))                                                                                   </UnionPattern>
••••)),)                                                                                 </TuplePattern>
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
••••)),):•(_,•Result<_,•_>)•=•(1,•Ok((Ok(V3((1,•Ok::<_,•(i32,•i32)>((1,•1))))),•1)));    </LetVariableDeclaration>                        */
	let [ref mut _x0, _, ref _x2, _, _x4, ref mut _x5, _x6, _] = arr;                                                                     /*
	let•[ref•mut•_x0,•_,•ref•_x2,•_,•_x4,•ref•mut•_x5,•_x6,•_]•=•arr;    LetVariableDeclaration
	    [ref•mut•_x0,•_,•ref•_x2,•_,•_x4,•ref•mut•_x5,•_x6,•_]           ArrayPattern
	     ref•mut•_x0                                                     PatternVariableDeclaration{ref, mut}
	                  _                                                  WildcardPattern
	                     ref•_x2                                         PatternVariableDeclaration{ref, !mut}
	                              _                                      WildcardPattern
	                                      ref•mut•_x5                    PatternVariableDeclaration{ref, mut}
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
	*_x0•=•U;    ExpressionStatement{semi}
	*_x0•=•U     ReassignmentExpression{tk: "="}
	*_x0         DereferenceExpression                                                                                                    */
	let a @ (b, c) = (S, S);                                                                                                              /*
	let•a•@•(b,•c)•=•(S,•S);    LetVariableDeclaration
	    a•@•(b,•c)              PatternVariableDeclaration{!ref, !mut}
	        (b,•c)              TuplePattern
	                 (S,•S)     TupleLiteral                                                                                              */
	let mut x@B {b, ..} = B {a: 10, b: C {c: 20}};                                                                                        /*
	let•mut•x@B•{b,•..}•=•B•{a:•10,•b:•C•{c:•20}};    LetVariableDeclaration
	    mut•x@B•{b,•..}                               PatternVariableDeclaration{!ref, mut}
	          B•{b,•..}                               StructPattern
	            {b,•..}                               StructPattern.properties{dk: "{}"}
	             b                                    StructPatternPropertyShorthand{!box, !ref, !mut}
	                ..                                RestPattern
	                      B•{a:•10,•b:•C•{c:•20}}     StructLiteral
	                        {a:•10,•b:•C•{c:•20}}     StructLiteral.properties{dk: "{}"}
	                         a:•10                    StructLiteralProperty
	                            10                    Literal{kind: Integer}
	                                b:•C•{c:•20}      StructLiteralProperty
	                                   C•{c:•20}      StructLiteral
	                                     {c:•20}      StructLiteral.properties{dk: "{}"}
	                                      c:•20       StructLiteralProperty
	                                         20       Literal{kind: Integer}                                                              */
	if let Some(x @ B { b: mut b @ C { c }, .. }) = some_b {}                                                                             /*
	if•let•Some(x•@•B•{•b:•mut•b•@•C•{•c•},•..•})•=•some_b•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•Some(x•@•B•{•b:•mut•b•@•C•{•c•},•..•})•=•some_b       LetScrutinee
	       Some(x•@•B•{•b:•mut•b•@•C•{•c•},•..•})                TuplePattern
	           (x•@•B•{•b:•mut•b•@•C•{•c•},•..•})                TuplePattern.items{dk: "()"}
	            x•@•B•{•b:•mut•b•@•C•{•c•},•..•}                 PatternVariableDeclaration{!ref, !mut}
	                B•{•b:•mut•b•@•C•{•c•},•..•}                 StructPattern
	                  {•b:•mut•b•@•C•{•c•},•..•}                 StructPattern.properties{dk: "{}"}
	                    b:•mut•b•@•C•{•c•}                       StructPatternPropertyDestructured
	                       mut•b•@•C•{•c•}                       PatternVariableDeclaration{!ref, mut}
	                               C•{•c•}                       StructPattern
	                                 {•c•}                       StructPattern.properties{dk: "{}"}
	                                   c                         StructPatternPropertyShorthand{!box, !ref, !mut}
	                                        ..                   RestPattern
	                                                       {}    IfBlockExpression.body{dk: "{}"}                                         */
	match x { Some(ref mut _y @ ..) => {} }                                                                                               /*
	match•x•{•Some(ref•mut•_y•@•..)•=>•{}•}    ExpressionStatement{!semi}, MatchExpression
	        {•Some(ref•mut•_y•@•..)•=>•{}•}    MatchExpression.cases{dk: "{}"}
	          Some(ref•mut•_y•@•..)•=>•{}      MatchExpressionCase
	          Some(ref•mut•_y•@•..)            TuplePattern
	              (ref•mut•_y•@•..)            TuplePattern.items{dk: "()"}
	               ref•mut•_y•@•..             PatternVariableDeclaration{ref, mut}
	                            ..             RestPattern
	                                   {}      BlockExpression                                                                            */
	let ref a @ box ref b = Box::new(NC);                                                                                                 /*
	let•ref•a•@•box•ref•b•=•Box::new(NC);    LetVariableDeclaration
	    ref•a•@•box•ref•b                    PatternVariableDeclaration{ref, !mut}
	            box•ref•b                    BoxPattern
	                ref•b                    PatternVariableDeclaration{ref, !mut}
	                        Box::new(NC)     CallExpression
	                        Box::new         ExpressionPath
	                                (NC)     CallExpression.arguments{dk: "()"}                                                           */
	let a @ b @ c @ d = C;                                                                                                                /*
	let•a•@•b•@•c•@•d•=•C;    LetVariableDeclaration
	    a•@•b•@•c•@•d         PatternVariableDeclaration{!ref, !mut}
	        b•@•c•@•d         PatternVariableDeclaration{!ref, !mut}
	            c•@•d         PatternVariableDeclaration{!ref, !mut}                                                                      */
	let a @ (b, c) = (C, mk_c());                                                                                                         /*
	let•a•@•(b,•c)•=•(C,•mk_c());    LetVariableDeclaration
	    a•@•(b,•c)                   PatternVariableDeclaration{!ref, !mut}
	        (b,•c)                   TuplePattern
	                 (C,•mk_c())     TupleLiteral
	                     mk_c()      CallExpression
	                         ()      CallExpression.arguments{dk: "()"}                                                                   */
	let a @ P(b, P(c, d)) = P(mk_c(), P(C, C));                                                                                           /*
	let•a•@•P(b,•P(c,•d))•=•P(mk_c(),•P(C,•C));    LetVariableDeclaration
	    a•@•P(b,•P(c,•d))                          PatternVariableDeclaration{!ref, !mut}
	        P(b,•P(c,•d))                          TuplePattern
	         (b,•P(c,•d))                          TuplePattern.items{dk: "()"}
	             P(c,•d)                           TuplePattern
	              (c,•d)                           TuplePattern.items{dk: "()"}
	                        P(mk_c(),•P(C,•C))     CallExpression
	                         (mk_c(),•P(C,•C))     CallExpression.arguments{dk: "()"}
	                          mk_c()               CallExpression
	                              ()               CallExpression.arguments{dk: "()"}
	                                  P(C,•C)      CallExpression
	                                   (C,•C)      CallExpression.arguments{dk: "()"}                                                     */
	let a @ [b, c] = [C, C];                                                                                                              /*
	let•a•@•[b,•c]•=•[C,•C];    LetVariableDeclaration
	    a•@•[b,•c]              PatternVariableDeclaration{!ref, !mut}
	        [b,•c]              ArrayPattern
	                 [C,•C]     ArrayLiteral                                                                                              */
	let a @ &(b, c) = &(C, C);                                                                                                            /*
	let•a•@•&(b,•c)•=•&(C,•C);    LetVariableDeclaration
	    a•@•&(b,•c)               PatternVariableDeclaration{!ref, !mut}
	        &(b,•c)               ReferencePattern{!mut}
	         (b,•c)               TuplePattern
	                  &(C,•C)     ReferenceExpression{!mut}
	                   (C,•C)     TupleLiteral                                                                                            */
	let a @ &(b, &P(c, d)) = &(mk_c(), &P(C, C));                                                                                         /*
	let•a•@•&(b,•&P(c,•d))•=•&(mk_c(),•&P(C,•C));    LetVariableDeclaration
	    a•@•&(b,•&P(c,•d))                           PatternVariableDeclaration{!ref, !mut}
	        &(b,•&P(c,•d))                           ReferencePattern{!mut}
	         (b,•&P(c,•d))                           TuplePattern
	             &P(c,•d)                            ReferencePattern{!mut}
	              P(c,•d)                            TuplePattern
	               (c,•d)                            TuplePattern.items{dk: "()"}
	                         &(mk_c(),•&P(C,•C))     ReferenceExpression{!mut}
	                          (mk_c(),•&P(C,•C))     TupleLiteral
	                           mk_c()                CallExpression
	                               ()                CallExpression.arguments{dk: "()"}
	                                   &P(C,•C)      ReferenceExpression{!mut}
	                                    P(C,•C)      CallExpression
	                                     (C,•C)      CallExpression.arguments{dk: "()"}                                                   */
    let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());                                                                                  /*
    let•a•@•(mut•b•@•ref•mut•c,•d•@•ref•e)•=•(u(),•u());    LetVariableDeclaration
        a•@•(mut•b•@•ref•mut•c,•d•@•ref•e)                  PatternVariableDeclaration{!ref, !mut}
            (mut•b•@•ref•mut•c,•d•@•ref•e)                  TuplePattern
             mut•b•@•ref•mut•c                              PatternVariableDeclaration{!ref, mut}
                     ref•mut•c                              PatternVariableDeclaration{ref, mut}
                                d•@•ref•e                   PatternVariableDeclaration{!ref, !mut}
                                    ref•e                   PatternVariableDeclaration{ref, !mut}
                                             (u(),•u())     TupleLiteral
                                              u()           CallExpression
                                               ()           CallExpression.arguments{dk: "()"}
                                                   u()      CallExpression
                                                    ()      CallExpression.arguments{dk: "()"}                                            */
    let a @ ref b = U;                                                                                                                    /*
    let•a•@•ref•b•=•U;    LetVariableDeclaration
        a•@•ref•b         PatternVariableDeclaration{!ref, !mut}
            ref•b         PatternVariableDeclaration{ref, !mut}                                                                           */
	let ref mut a @ ref mut b = U;                                                                                                        /*
	let•ref•mut•a•@•ref•mut•b•=•U;    LetVariableDeclaration
	    ref•mut•a•@•ref•mut•b         PatternVariableDeclaration{ref, mut}
	                ref•mut•b         PatternVariableDeclaration{ref, mut}                                                                */
	let a @ &mut ref mut b = &mut U;                                                                                                      /*
	let•a•@•&mut•ref•mut•b•=•&mut•U;    LetVariableDeclaration
	    a•@•&mut•ref•mut•b              PatternVariableDeclaration{!ref, !mut}
	        &mut•ref•mut•b              ReferencePattern{mut}
	             ref•mut•b              PatternVariableDeclaration{ref, mut}
	                         &mut•U     ReferenceExpression{mut}                                                                          */
    let a @ &mut (ref mut b, ref mut c) = &mut (U, U);                                                                                    /*
    let•a•@•&mut•(ref•mut•b,•ref•mut•c)•=•&mut•(U,•U);    LetVariableDeclaration
        a•@•&mut•(ref•mut•b,•ref•mut•c)                   PatternVariableDeclaration{!ref, !mut}
            &mut•(ref•mut•b,•ref•mut•c)                   ReferencePattern{mut}
                 (ref•mut•b,•ref•mut•c)                   TuplePattern
                  ref•mut•b                               PatternVariableDeclaration{ref, mut}
                             ref•mut•c                    PatternVariableDeclaration{ref, mut}
                                          &mut•(U,•U)     ReferenceExpression{mut}
                                               (U,•U)     TupleLiteral                                                                    */
	let a @ NC(b, c) = NC(C, C);                                                                                                          /*
	let•a•@•NC(b,•c)•=•NC(C,•C);    LetVariableDeclaration
	    a•@•NC(b,•c)                PatternVariableDeclaration{!ref, !mut}
	        NC(b,•c)                TuplePattern
	          (b,•c)                TuplePattern.items{dk: "()"}
	                   NC(C,•C)     CallExpression
	                     (C,•C)     CallExpression.arguments{dk: "()"}                                                                    */
	let a @ NC(b, c @ NC(d, e)) = NC(C, NC(C, C));                                                                                        /*
	let•a•@•NC(b,•c•@•NC(d,•e))•=•NC(C,•NC(C,•C));    LetVariableDeclaration
	    a•@•NC(b,•c•@•NC(d,•e))                       PatternVariableDeclaration{!ref, !mut}
	        NC(b,•c•@•NC(d,•e))                       TuplePattern
	          (b,•c•@•NC(d,•e))                       TuplePattern.items{dk: "()"}
	              c•@•NC(d,•e)                        PatternVariableDeclaration{!ref, !mut}
	                  NC(d,•e)                        TuplePattern
	                    (d,•e)                        TuplePattern.items{dk: "()"}
	                              NC(C,•NC(C,•C))     CallExpression
	                                (C,•NC(C,•C))     CallExpression.arguments{dk: "()"}
	                                    NC(C,•C)      CallExpression
	                                      (C,•C)      CallExpression.arguments{dk: "()"}                                                  */
	let _a @ _b: u8 = 0;                                                                                                                  /*
	let•_a•@•_b:•u8•=•0;    LetVariableDeclaration
	    _a•@•_b             PatternVariableDeclaration{!ref, !mut}
	                  0     Literal{kind: Integer}                                                                                        */
	let     &_ =         & 1_usize;                                                                                                       /*
	let•••••&_•=•••••••••&•1_usize;    LetVariableDeclaration
	        &_                         ReferencePattern{!mut}
	         _                         WildcardPattern
	                     &•1_usize     ReferenceExpression{!mut}
	                       1_usize     Literal{kind: Integer}
	                         usize     Identifier                                                                                         */
    let    &&_ =       & & 1_usize;                                                                                                       /*
    let••••&&_•=•••••••&•&•1_usize;    LetVariableDeclaration
           &&_                         ReferencePattern{!mut}
            &_                         ReferencePattern{!mut}
             _                         WildcardPattern
                       &•&•1_usize     ReferenceExpression{!mut}
                         &•1_usize     ReferenceExpression{!mut}
                           1_usize     Literal{kind: Integer}
                             usize     Identifier                                                                                         */
    let   &&&_ =     & & & 1_usize;                                                                                                       /*
    let•••&&&_•=•••••&•&•&•1_usize;    LetVariableDeclaration
          &&&_                         ReferencePattern{!mut}
           &&_                         ReferencePattern{!mut}
            &_                         ReferencePattern{!mut}
             _                         WildcardPattern
                     &•&•&•1_usize     ReferenceExpression{!mut}
                       &•&•1_usize     ReferenceExpression{!mut}
                         &•1_usize     ReferenceExpression{!mut}
                           1_usize     Literal{kind: Integer}
                             usize     Identifier                                                                                         */
    let  & &&_ =     & & & 1_usize;                                                                                                       /*
    let••&•&&_•=•••••&•&•&•1_usize;    LetVariableDeclaration
         &•&&_                         ReferencePattern{!mut}
           &&_                         ReferencePattern{!mut}
            &_                         ReferencePattern{!mut}
             _                         WildcardPattern
                     &•&•&•1_usize     ReferenceExpression{!mut}
                       &•&•1_usize     ReferenceExpression{!mut}
                         &•1_usize     ReferenceExpression{!mut}
                           1_usize     Literal{kind: Integer}
                             usize     Identifier                                                                                         */
    let  &&&&_ =   & & & & 1_usize;                                                                                                       /*
    let••&&&&_•=•••&•&•&•&•1_usize;    LetVariableDeclaration
         &&&&_                         ReferencePattern{!mut}
          &&&_                         ReferencePattern{!mut}
           &&_                         ReferencePattern{!mut}
            &_                         ReferencePattern{!mut}
             _                         WildcardPattern
                   &•&•&•&•1_usize     ReferenceExpression{!mut}
                     &•&•&•1_usize     ReferenceExpression{!mut}
                       &•&•1_usize     ReferenceExpression{!mut}
                         &•1_usize     ReferenceExpression{!mut}
                           1_usize     Literal{kind: Integer}
                             usize     Identifier                                                                                         */
    let & &&&_ =   & & & & 1_usize;                                                                                                       /*
    let•&•&&&_•=•••&•&•&•&•1_usize;    LetVariableDeclaration
        &•&&&_                         ReferencePattern{!mut}
          &&&_                         ReferencePattern{!mut}
           &&_                         ReferencePattern{!mut}
            &_                         ReferencePattern{!mut}
             _                         WildcardPattern
                   &•&•&•&•1_usize     ReferenceExpression{!mut}
                     &•&•&•1_usize     ReferenceExpression{!mut}
                       &•&•1_usize     ReferenceExpression{!mut}
                         &•1_usize     ReferenceExpression{!mut}
                           1_usize     Literal{kind: Integer}
                             usize     Identifier                                                                                         */
    let &&&&&_ = & & & & & 1_usize;                                                                                                       /*
    let•&&&&&_•=•&•&•&•&•&•1_usize;    LetVariableDeclaration
        &&&&&_                         ReferencePattern{!mut}
         &&&&_                         ReferencePattern{!mut}
          &&&_                         ReferencePattern{!mut}
           &&_                         ReferencePattern{!mut}
            &_                         ReferencePattern{!mut}
             _                         WildcardPattern
                 &•&•&•&•&•1_usize     ReferenceExpression{!mut}
                   &•&•&•&•1_usize     ReferenceExpression{!mut}
                     &•&•&•1_usize     ReferenceExpression{!mut}
                       &•&•1_usize     ReferenceExpression{!mut}
                         &•1_usize     ReferenceExpression{!mut}
                           1_usize     Literal{kind: Integer}
                             usize     Identifier                                                                                         */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 3
// Parsed Nodes: 899
// state_rollbacks: 8
// Total '.charCodeAt()' calls: 4218 (34% re-reads)
// Unnecessary 'skip_whitespace()' calls: 463
// source: "../../samples/patterns/pattern.rs"