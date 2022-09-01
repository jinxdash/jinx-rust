fn main() {                                                                                                                               /*
fn•main()•{↲    <Program>
fn•main()•{↲    <Program.ast{dk: "None"}>
fn•main()•{↲    <FunctionDeclaration>
       ()       FunctionDeclaration.parameters{dk: "()"}
          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                      */
	holds_callable.callable();                                                                                                            /*
	holds_callable.callable();    ExpressionStatement{semi}
	holds_callable.callable()     CallExpression
	                       ()     CallExpression.arguments{dk: "()"}                                                                      */
	(holds_callable.callable)();                                                                                                          /*
	(holds_callable.callable)();    ExpressionStatement{semi}
	(holds_callable.callable)()     CallExpression
	 holds_callable.callable        MemberExpression{!computed}
	                         ()     CallExpression.arguments{dk: "()"}                                                                    */
    a = b = ( c );                                                                                                                        /*
    a•=•b•=•(•c•);    ExpressionStatement{semi}
    a•=•b•=•(•c•)     ReassignmentExpression{tk: "="}
        b•=•(•c•)     ReassignmentExpression{tk: "="}                                                                                     */
	mystruct.myfield;                                                                                                                     /*
	mystruct.myfield;    ExpressionStatement{semi}
	mystruct.myfield     MemberExpression{!computed}                                                                                      */
	foo().x;                                                                                                                              /*
	foo().x;    ExpressionStatement{semi}
	foo().x     MemberExpression{!computed}
	foo()       CallExpression
	   ()       CallExpression.arguments{dk: "()"}                                                                                        */
	(Struct {a: 10, b: 20}).a;                                                                                                            /*
	(Struct•{a:•10,•b:•20}).a;    ExpressionStatement{semi}
	(Struct•{a:•10,•b:•20}).a     MemberExpression{!computed}
	 Struct•{a:•10,•b:•20}        StructLiteral
	        {a:•10,•b:•20}        StructLiteral.properties{dk: "{}"}
	         a:•10                StructLiteralProperty
	            10                Literal{kind: Integer}
	                b:•20         StructLiteralProperty
	                   20         Literal{kind: Integer}                                                                                  */
	(mystruct.function_field)();                                                                                                          /*
	(mystruct.function_field)();    ExpressionStatement{semi}
	(mystruct.function_field)()     CallExpression
	 mystruct.function_field        MemberExpression{!computed}
	                         ()     CallExpression.arguments{dk: "()"}                                                                    */
	let name: &'static str = (|| "Rust")();                                                                                               /*
	let•name:•&'static•str•=•(||•"Rust")();    LetVariableDeclaration
	          &'static•str                     TypeReference{!mut}
	           'static                         LtStatic
	                         (||•"Rust")()     CallExpression
	                          ||•"Rust"        ClosureFunctionExpression
	                          ||               ClosureFunctionExpression.parameters{dk: "||"}
	                             "Rust"        Literal{kind: String}
	                                    ()     CallExpression.arguments{dk: "()"}                                                         */
	let x: i32 = 2 + 3 * 4;                                                                                                               /*
	let•x:•i32•=•2•+•3•*•4;    LetVariableDeclaration
	             2•+•3•*•4     OperationExpression{tk: "+"}
	             2             Literal{kind: Integer}
	                 3•*•4     OperationExpression{tk: "*"}
	                 3         Literal{kind: Integer}
	                     4     Literal{kind: Integer}                                                                                     */
	let y: i32 = (2 + 3) * 4;                                                                                                             /*
	let•y:•i32•=•(2•+•3)•*•4;    LetVariableDeclaration
	             (2•+•3)•*•4     OperationExpression{tk: "*"}
	              2•+•3          OperationExpression{tk: "+"}
	              2              Literal{kind: Integer}
	                  3          Literal{kind: Integer}
	                       4     Literal{kind: Integer}                                                                                   */
	let lhs = &this.thir[lhs];                                                                                                            /*
	let•lhs•=•&this.thir[lhs];    LetVariableDeclaration
	          &this.thir[lhs]     ReferenceExpression{!mut}
	           this.thir[lhs]     MemberExpression{computed}
	           this.thir          MemberExpression{!computed}                                                                             */
    (*f)(&x);                                                                                                                             /*
    (*f)(&x);    ExpressionStatement{semi}
    (*f)(&x)     CallExpression
     *f          DereferenceExpression
        (&x)     CallExpression.arguments{dk: "()"}
         &x      ReferenceExpression{!mut}                                                                                                */
	(*x) * (*x);                                                                                                                          /*
	(*x)•*•(*x);    ExpressionStatement{semi}
	(*x)•*•(*x)     OperationExpression{tk: "*"}
	 *x             DereferenceExpression
	        *x      DereferenceExpression                                                                                                 */
	println!("{}", (self.0)());                                                                                                           /*
	println!("{}",•(self.0)());    ExpressionStatement{semi}
	println!("{}",•(self.0)())     MacroInvocation
	        ("{}",•(self.0)())     MacroInvocation.segments{dk: "()"}
	         "{}"                  Literal{kind: String}
	             ,                 PunctuationToken{tk: ","}
	               (self.0)        DelimGroup
	                    .          PunctuationToken{tk: "."}
	                     0         Literal{kind: Integer}
	                       ()      DelimGroup                                                                                             */
	(self.0)(ecx, span, meta_item, &item, &mut |a| items.push(a));                                                                        /*
	(self.0)(ecx,•span,•meta_item,•&item,•&mut•|a|•items.push(a));    ExpressionStatement{semi}
	(self.0)(ecx,•span,•meta_item,•&item,•&mut•|a|•items.push(a))     CallExpression
	 self.0                                                           MemberExpression{!computed}
	      0                                                           Index
	        (ecx,•span,•meta_item,•&item,•&mut•|a|•items.push(a))     CallExpression.arguments{dk: "()"}
	                               &item                              ReferenceExpression{!mut}
	                                      &mut•|a|•items.push(a)      ReferenceExpression{mut}
	                                           |a|•items.push(a)      ClosureFunctionExpression
	                                           |a|                    ClosureFunctionExpression.parameters{dk: "||"}
	                                            a                     ClosureFunctionParameterDeclaration
	                                               items.push(a)      CallExpression
	                                                         (a)      CallExpression.arguments{dk: "()"}                                  */
	(|_, _, _| {})(0u8, 42u16, 0u8);                                                                                                      /*
	(|_,•_,•_|•{})(0u8,•42u16,•0u8);    ExpressionStatement{semi}
	(|_,•_,•_|•{})(0u8,•42u16,•0u8)     CallExpression
	 |_,•_,•_|•{}                       ClosureFunctionExpression
	 |_,•_,•_|                          ClosureFunctionExpression.parameters{dk: "||"}
	  _                                 ClosureFunctionParameterDeclaration, WildcardPattern
	     _                              ClosureFunctionParameterDeclaration, WildcardPattern
	        _                           ClosureFunctionParameterDeclaration, WildcardPattern
	           {}                       BlockExpression
	              (0u8,•42u16,•0u8)     CallExpression.arguments{dk: "()"}
	               0u8                  Literal{kind: Integer}
	                u8                  Identifier
	                    42u16           Literal{kind: Integer}
	                      u16           Identifier
	                           0u8      Literal{kind: Integer}
	                            u8      Identifier                                                                                        */
	(|_, _| {})(0u8, 42u16);                                                                                                              /*
	(|_,•_|•{})(0u8,•42u16);    ExpressionStatement{semi}
	(|_,•_|•{})(0u8,•42u16)     CallExpression
	 |_,•_|•{}                  ClosureFunctionExpression
	 |_,•_|                     ClosureFunctionExpression.parameters{dk: "||"}
	  _                         ClosureFunctionParameterDeclaration, WildcardPattern
	     _                      ClosureFunctionParameterDeclaration, WildcardPattern
	        {}                  BlockExpression
	           (0u8,•42u16)     CallExpression.arguments{dk: "()"}
	            0u8             Literal{kind: Integer}
	             u8             Identifier
	                 42u16      Literal{kind: Integer}
	                   u16      Identifier                                                                                                */
	let x = &[0u32, 42u32] as &[u32];                                                                                                     /*
	let•x•=•&[0u32,•42u32]•as•&[u32];    LetVariableDeclaration
	        &[0u32,•42u32]•as•&[u32]     ExpressionAsTypeCast
	        &[0u32,•42u32]               ReferenceExpression{!mut}
	         [0u32,•42u32]               ArrayLiteral
	          0u32                       Literal{kind: Integer}
	           u32                       Identifier
	                42u32                Literal{kind: Integer}
	                  u32                Identifier
	                          &[u32]     TypeReference{!mut}
	                           [u32]     TypeSlice                                                                                        */
    match x {                                                                                                                             /*
    match•x•{↲    <ExpressionStatement{!semi}>
    match•x•{↲    <MatchExpression>
            {↲    <MatchExpression.cases{dk: "{}"}>                                                                                       */
        [] => assert_eq!(0u32, 1),                                                                                                        /*
        []•=>•assert_eq!(0u32,•1)    MatchExpressionCase
        []                           ArrayPattern
              assert_eq!(0u32,•1)    MacroInvocation
                        (0u32,•1)    MacroInvocation.segments{dk: "()"}
                         0u32        Literal{kind: Integer}
                          u32        Identifier
                             ,       PunctuationToken{tk: ","}
                               1     Literal{kind: Integer}                                                                               */
        [_, ref y @ ..] => assert_eq!(&x[1] as *const u32 as usize, &y[0] as *const u32 as usize),                                        /*
        [_,•ref•y•@•..]•=>•assert_eq!(&x[1]•as•*const•u32•as•usize,•&y[0]•as•*const•u32•as•usize)    MatchExpressionCase
        [_,•ref•y•@•..]                                                                              ArrayPattern
         _                                                                                           WildcardPattern
            ref•y•@•..                                                                               PatternVariableDeclaration{ref, !mut}
                    ..                                                                               RestPattern
                           assert_eq!(&x[1]•as•*const•u32•as•usize,•&y[0]•as•*const•u32•as•usize)    MacroInvocation
                                     (&x[1]•as•*const•u32•as•usize,•&y[0]•as•*const•u32•as•usize)    MacroInvocation.segments{dk: "()"}
                                      &                                                              PunctuationToken{tk: "&"}
                                        [1]                                                          DelimGroup
                                         1                                                           Literal{kind: Integer}
                                               *                                                     PunctuationToken{tk: "*"}
                                                                  ,                                  PunctuationToken{tk: ","}
                                                                    &                                PunctuationToken{tk: "&"}
                                                                      [0]                            DelimGroup
                                                                       0                             Literal{kind: Integer}
                                                                             *                       PunctuationToken{tk: "*"}            */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
	unsafe { assert_eq!(ABC as usize, 0); }                                                                                               /*
	unsafe•{•assert_eq!(ABC•as•usize,•0);•}    ExpressionStatement{!semi}, BlockExpression{unsafe}
	       {•assert_eq!(ABC•as•usize,•0);•}    BlockExpression.body{dk: "{}"}
	         assert_eq!(ABC•as•usize,•0);      ExpressionStatement{semi}
	         assert_eq!(ABC•as•usize,•0)       MacroInvocation
	                   (ABC•as•usize,•0)       MacroInvocation.segments{dk: "()"}
	                                ,          PunctuationToken{tk: ","}
	                                  0        Literal{kind: Integer}                                                                     */
	&mut (|| Some(0 as *const ())) as &mut dyn FnMut() -> Option<*const ()>;                                                              /*
	&mut•(||•Some(0•as•*const•()))•as•&mut•dyn•FnMut()•->•Option<*const•()>;    ExpressionStatement{semi}
	&mut•(||•Some(0•as•*const•()))•as•&mut•dyn•FnMut()•->•Option<*const•()>     ExpressionAsTypeCast
	&mut•(||•Some(0•as•*const•()))                                              ReferenceExpression{mut}
	      ||•Some(0•as•*const•())                                               ClosureFunctionExpression
	      ||                                                                    ClosureFunctionExpression.parameters{dk: "||"}
	         Some(0•as•*const•())                                               CallExpression
	             (0•as•*const•())                                               CallExpression.arguments{dk: "()"}
	              0•as•*const•()                                                ExpressionAsTypeCast
	              0                                                             Literal{kind: Integer}
	                   *const•()                                                TypeDereferenceConst
	                          ()                                                TypeTuple
	                                  &mut•dyn•FnMut()•->•Option<*const•()>     TypeReference{mut}
	                                       dyn•FnMut()•->•Option<*const•()>     TypeDynBounds{dyn}
	                                           FnMut()•->•Option<*const•()>     TypeTraitBound{!maybeConst, !optional}, TypeFunction
	                                                ()                          TypeFunction.parameters{dk: "()"}
	                                                      Option<*const•()>     TypeCall
	                                                            <*const•()>     TypeCall.typeArguments{dk: "<>"}
	                                                             *const•()      TypeDereferenceConst
	                                                                    ()      TypeTuple                                                 */
	unsafe {                                                                                                                              /*
	unsafe•{↲    <ExpressionStatement{!semi}>
	unsafe•{↲    <BlockExpression{unsafe}>
	       {↲    <BlockExpression.body{dk: "{}"}>                                                                                         */
		NUM = 6 * 7 + 1 + (1u8 == 1u8) as u8; // 44
/*		NUM•=•6•*•7•+•1•+•(1u8•==•1u8)•as•u8;          ExpressionStatement{semi}
		NUM•=•6•*•7•+•1•+•(1u8•==•1u8)•as•u8           ReassignmentExpression{tk: "="}
		      6•*•7•+•1•+•(1u8•==•1u8)•as•u8           OperationExpression{tk: "+"}
		      6•*•7•+•1                                OperationExpression{tk: "+"}
		      6•*•7                                    OperationExpression{tk: "*"}
		      6                                        Literal{kind: Integer}
		          7                                    Literal{kind: Integer}
		              1                                Literal{kind: Integer}
		                  (1u8•==•1u8)•as•u8           ExpressionAsTypeCast
		                   1u8•==•1u8                  ComparisonExpression{tk: "=="}
		                   1u8                         Literal{kind: Integer}
		                    u8                         Identifier
		                          1u8                  Literal{kind: Integer}
		                           u8                  Identifier                                                                         */
		                                      //•44    Comment{line}
		assert_eq!(*NUM_REF as i32, 44);                                                                                                  /*
		assert_eq!(*NUM_REF•as•i32,•44);    ExpressionStatement{semi}
		assert_eq!(*NUM_REF•as•i32,•44)     MacroInvocation
		          (*NUM_REF•as•i32,•44)     MacroInvocation.segments{dk: "()"}
		           *                        PunctuationToken{tk: "*"}
		                          ,         PunctuationToken{tk: ","}
		                            44      Literal{kind: Integer}                                                                        */
	}                                                                                                                                     /*
   ╚}    </BlockExpression.body>
   ╚}    </BlockExpression>
   ╚}    </ExpressionStatement>                                                                                                           */
	unsafe { puts(*argv as *const i8); }                                                                                                  /*
	unsafe•{•puts(*argv•as•*const•i8);•}    ExpressionStatement{!semi}, BlockExpression{unsafe}
	       {•puts(*argv•as•*const•i8);•}    BlockExpression.body{dk: "{}"}
	         puts(*argv•as•*const•i8);      ExpressionStatement{semi}
	         puts(*argv•as•*const•i8)       CallExpression
	             (*argv•as•*const•i8)       CallExpression.arguments{dk: "()"}
	              *argv•as•*const•i8        ExpressionAsTypeCast
	              *argv                     DereferenceExpression
	                       *const•i8        TypeDereferenceConst                                                                          */
	unsafe { puts(*((argv as usize + intrinsics::size_of::<*const u8>()) as *const *const i8)); }                                         /*
	unsafe•{•puts(*((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));•}    ExpressionStatement{!semi}, BlockExpression{unsafe}
	       {•puts(*((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));•}    BlockExpression.body{dk: "{}"}
	         puts(*((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));      ExpressionStatement{semi}
	         puts(*((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8))       CallExpression
	             (*((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8))       CallExpression.arguments{dk: "()"}
	              *((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8)        DereferenceExpression
	                (argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8         ExpressionAsTypeCast
	                 argv•as•usize•+•intrinsics::size_of::<*const•u8>()                              OperationExpression{tk: "+"}
	                 argv•as•usize                                                                   ExpressionAsTypeCast
	                                 intrinsics::size_of::<*const•u8>()                              CallExpression
	                                 intrinsics::size_of                                             ExpressionPath
	                                                      <*const•u8>                                CallExpression.typeArguments{dk: "<>"}
	                                                       *const•u8                                 TypeDereferenceConst
	                                                                 ()                              CallExpression.arguments{dk: "()"}
	                                                                        *const•*const•i8         TypeDereferenceConst
	                                                                               *const•i8         TypeDereferenceConst                 */
	unsafe { puts(*((argv as usize + 2 * intrinsics::size_of::<*const u8>()) as *const *const i8)); }                                     /*
	unsafe•{•puts(*((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));•}    ExpressionStatement{!semi}, BlockExpression{unsafe}
	       {•puts(*((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));•}    BlockExpression.body{dk: "{}"}
	         puts(*((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));      ExpressionStatement{semi}
	         puts(*((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8))       CallExpression
	             (*((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8))       CallExpression.arguments{dk: "()"}
	              *((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8)        DereferenceExpression
	                (argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8         ExpressionAsTypeCast
	                 argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>()                              OperationExpression{tk: "+"}
	                 argv•as•usize                                                                       ExpressionAsTypeCast
	                                 2•*•intrinsics::size_of::<*const•u8>()                              OperationExpression{tk: "*"}
	                                 2                                                                   Literal{kind: Integer}
	                                     intrinsics::size_of::<*const•u8>()                              CallExpression
	                                     intrinsics::size_of                                             ExpressionPath
	                                                          <*const•u8>                                CallExpression.typeArguments{dk: "<>"}
	                                                           *const•u8                                 TypeDereferenceConst
	                                                                     ()                              CallExpression.arguments{dk: "()"}
	                                                                            *const•*const•i8         TypeDereferenceConst
	                                                                                   *const•i8         TypeDereferenceConst             */
	intrinsics::write_bytes(&mut uninit.value.value as *mut T, 0, 1);                                                                     /*
	intrinsics::write_bytes(&mut•uninit.value.value•as•*mut•T,•0,•1);    ExpressionStatement{semi}
	intrinsics::write_bytes(&mut•uninit.value.value•as•*mut•T,•0,•1)     CallExpression
	intrinsics::write_bytes                                              ExpressionPath
	                       (&mut•uninit.value.value•as•*mut•T,•0,•1)     CallExpression.arguments{dk: "()"}
	                        &mut•uninit.value.value•as•*mut•T            ExpressionAsTypeCast
	                        &mut•uninit.value.value                      ReferenceExpression{mut}
	                             uninit.value.value                      MemberExpression{!computed}
	                             uninit.value                            MemberExpression{!computed}
	                                                   *mut•T            TypeDereferenceMut
	                                                           0         Literal{kind: Integer}
	                                                              1      Literal{kind: Integer}                                           */
	assert_eq!(slice_ptr as usize % 4, 0);                                                                                                /*
	assert_eq!(slice_ptr•as•usize•%•4,•0);    ExpressionStatement{semi}
	assert_eq!(slice_ptr•as•usize•%•4,•0)     MacroInvocation
	          (slice_ptr•as•usize•%•4,•0)     MacroInvocation.segments{dk: "()"}
	                              %           PunctuationToken{tk: "%"}
	                                4         Literal{kind: Integer}
	                                 ,        PunctuationToken{tk: ","}
	                                   0      Literal{kind: Integer}                                                                      */
	printf("Hello %s\n\0" as *const str as *const i8, "printf\0" as *const str as *const i8);                                             /*
	printf("Hello•%s\n\0"•as•*const•str•as•*const•i8,•"printf\0"•as•*const•str•as•*const•i8);    ExpressionStatement{semi}
	printf("Hello•%s\n\0"•as•*const•str•as•*const•i8,•"printf\0"•as•*const•str•as•*const•i8)     CallExpression
	      ("Hello•%s\n\0"•as•*const•str•as•*const•i8,•"printf\0"•as•*const•str•as•*const•i8)     CallExpression.arguments{dk: "()"}
	       "Hello•%s\n\0"•as•*const•str•as•*const•i8                                             ExpressionAsTypeCast
	       "Hello•%s\n\0"•as•*const•str                                                          ExpressionAsTypeCast
	       "Hello•%s\n\0"                                                                        Literal{kind: String}
	                         *const•str                                                          TypeDereferenceConst
	                                       *const•i8                                             TypeDereferenceConst
	                                                  "printf\0"•as•*const•str•as•*const•i8      ExpressionAsTypeCast
	                                                  "printf\0"•as•*const•str                   ExpressionAsTypeCast
	                                                  "printf\0"                                 Literal{kind: String}
	                                                                *const•str                   TypeDereferenceConst
	                                                                              *const•i8      TypeDereferenceConst                     */
	let hello: &[u8] = b"Hello\0" as &[u8; 6];                                                                                            /*
	let•hello:•&[u8]•=•b"Hello\0"•as•&[u8;•6];    LetVariableDeclaration
	           &[u8]                              TypeReference{!mut}
	            [u8]                              TypeSlice
	                   b"Hello\0"•as•&[u8;•6]     ExpressionAsTypeCast
	                   b"Hello\0"                 Literal{kind: bString}
	                                 &[u8;•6]     TypeReference{!mut}
	                                  [u8;•6]     TypeSizedArray
	                                       6      Literal{kind: Integer}                                                                  */
	let ptr: *const i8 = hello as *const [u8] as *const i8;                                                                               /*
	let•ptr:•*const•i8•=•hello•as•*const•[u8]•as•*const•i8;    LetVariableDeclaration
	         *const•i8                                         TypeDereferenceConst
	                     hello•as•*const•[u8]•as•*const•i8     ExpressionAsTypeCast
	                     hello•as•*const•[u8]                  ExpressionAsTypeCast
	                              *const•[u8]                  TypeDereferenceConst
	                                     [u8]                  TypeSlice
	                                             *const•i8     TypeDereferenceConst                                                       */
	let world: Box<&str> = box "World!\0";                                                                                                /*
	let•world:•Box<&str>•=•box•"World!\0";    LetVariableDeclaration
	           Box<&str>                      TypeCall
	              <&str>                      TypeCall.typeArguments{dk: "<>"}
	               &str                       TypeReference{!mut}
	                       box•"World!\0"     BoxExpression
	                           "World!\0"     Literal{kind: String}                                                                       */
	puts(*world as *const str as *const i8);                                                                                              /*
	puts(*world•as•*const•str•as•*const•i8);    ExpressionStatement{semi}
	puts(*world•as•*const•str•as•*const•i8)     CallExpression
	    (*world•as•*const•str•as•*const•i8)     CallExpression.arguments{dk: "()"}
	     *world•as•*const•str•as•*const•i8      ExpressionAsTypeCast
	     *world•as•*const•str                   ExpressionAsTypeCast
	     *world                                 DereferenceExpression
	               *const•str                   TypeDereferenceConst
	                             *const•i8      TypeDereferenceConst                                                                      */
	assert_eq!( a.f (), "The method f");                                                                                                  /*
	assert_eq!(•a.f•(),•"The•method•f");    ExpressionStatement{semi}
	assert_eq!(•a.f•(),•"The•method•f")     MacroInvocation
	          (•a.f•(),•"The•method•f")     MacroInvocation.segments{dk: "()"}
	             .                          PunctuationToken{tk: "."}
	                ()                      DelimGroup
	                  ,                     PunctuationToken{tk: ","}
	                    "The•method•f"      Literal{kind: String}                                                                         */
	assert_eq!((a.f)(), "The field f");                                                                                                   /*
	assert_eq!((a.f)(),•"The•field•f");    ExpressionStatement{semi}
	assert_eq!((a.f)(),•"The•field•f")     MacroInvocation
	          ((a.f)(),•"The•field•f")     MacroInvocation.segments{dk: "()"}
	           (a.f)                       DelimGroup
	             .                         PunctuationToken{tk: "."}
	                ()                     DelimGroup
	                  ,                    PunctuationToken{tk: ","}
	                    "The•field•f"      Literal{kind: String}                                                                          */
	assert_eq!(((|()| 42u8) as fn(()) -> u8)(()), 42);                                                                                    /*
	assert_eq!(((|()|•42u8)•as•fn(())•->•u8)(()),•42);    ExpressionStatement{semi}
	assert_eq!(((|()|•42u8)•as•fn(())•->•u8)(()),•42)     MacroInvocation
	          (((|()|•42u8)•as•fn(())•->•u8)(()),•42)     MacroInvocation.segments{dk: "()"}
	           ((|()|•42u8)•as•fn(())•->•u8)              DelimGroup
	            (|()|•42u8)                               DelimGroup
	             |                                        PunctuationToken{tk: "|"}
	              ()                                      DelimGroup
	                |                                     PunctuationToken{tk: "|"}
	                  42u8                                Literal{kind: Integer}
	                    u8                                Identifier
	                             (())                     DelimGroup
	                              ()                      DelimGroup
	                                  ->                  PunctuationToken{tk: "->"}
	                                        (())          DelimGroup
	                                         ()           DelimGroup
	                                            ,         PunctuationToken{tk: ","}
	                                              42      Literal{kind: Integer}                                                          */
	assert_eq!(intrinsics::bitreverse(0b10101000u8), 0b00010101u8);                                                                       /*
	assert_eq!(intrinsics::bitreverse(0b10101000u8),•0b00010101u8);    ExpressionStatement{semi}
	assert_eq!(intrinsics::bitreverse(0b10101000u8),•0b00010101u8)     MacroInvocation
	          (intrinsics::bitreverse(0b10101000u8),•0b00010101u8)     MacroInvocation.segments{dk: "()"}
	                     ::                                            PunctuationToken{tk: "::"}
	                                 (0b10101000u8)                    DelimGroup
	                                  0b10101000u8                     Literal{kind: Binary}
	                                            u8                     Identifier
	                                               ,                   PunctuationToken{tk: ","}
	                                                 0b00010101u8      Literal{kind: Binary}
	                                                           u8      Identifier                                                         */
	assert_eq!(intrinsics::bswap(0xabu8), 0xabu8);                                                                                        /*
	assert_eq!(intrinsics::bswap(0xabu8),•0xabu8);    ExpressionStatement{semi}
	assert_eq!(intrinsics::bswap(0xabu8),•0xabu8)     MacroInvocation
	          (intrinsics::bswap(0xabu8),•0xabu8)     MacroInvocation.segments{dk: "()"}
	                     ::                           PunctuationToken{tk: "::"}
	                            (0xabu8)              DelimGroup
	                             0xabu8               Literal{kind: Hex}
	                                 u8               Identifier
	                                    ,             PunctuationToken{tk: ","}
	                                      0xabu8      Literal{kind: Hex}
	                                          u8      Identifier                                                                          */
	assert_eq!(intrinsics::bswap(0xddccu16), 0xccddu16);                                                                                  /*
	assert_eq!(intrinsics::bswap(0xddccu16),•0xccddu16);    ExpressionStatement{semi}
	assert_eq!(intrinsics::bswap(0xddccu16),•0xccddu16)     MacroInvocation
	          (intrinsics::bswap(0xddccu16),•0xccddu16)     MacroInvocation.segments{dk: "()"}
	                     ::                                 PunctuationToken{tk: "::"}
	                            (0xddccu16)                 DelimGroup
	                             0xddccu16                  Literal{kind: Hex}
	                                   u16                  Identifier
	                                       ,                PunctuationToken{tk: ","}
	                                         0xccddu16      Literal{kind: Hex}
	                                               u16      Identifier                                                                    */
	assert_eq!(intrinsics::bswap(0xffee_ddccu32), 0xccdd_eeffu32);                                                                        /*
	assert_eq!(intrinsics::bswap(0xffee_ddccu32),•0xccdd_eeffu32);    ExpressionStatement{semi}
	assert_eq!(intrinsics::bswap(0xffee_ddccu32),•0xccdd_eeffu32)     MacroInvocation
	          (intrinsics::bswap(0xffee_ddccu32),•0xccdd_eeffu32)     MacroInvocation.segments{dk: "()"}
	                     ::                                           PunctuationToken{tk: "::"}
	                            (0xffee_ddccu32)                      DelimGroup
	                             0xffee_ddccu32                       Literal{kind: Hex}
	                                        u32                       Identifier
	                                            ,                     PunctuationToken{tk: ","}
	                                              0xccdd_eeffu32      Literal{kind: Hex}
	                                                         u32      Identifier                                                          */
	assert_eq!(intrinsics::bswap(0x1234_5678_ffee_ddccu64), 0xccdd_eeff_7856_3412u64);                                                    /*
	assert_eq!(intrinsics::bswap(0x1234_5678_ffee_ddccu64),•0xccdd_eeff_7856_3412u64);    ExpressionStatement{semi}
	assert_eq!(intrinsics::bswap(0x1234_5678_ffee_ddccu64),•0xccdd_eeff_7856_3412u64)     MacroInvocation
	          (intrinsics::bswap(0x1234_5678_ffee_ddccu64),•0xccdd_eeff_7856_3412u64)     MacroInvocation.segments{dk: "()"}
	                     ::                                                               PunctuationToken{tk: "::"}
	                            (0x1234_5678_ffee_ddccu64)                                DelimGroup
	                             0x1234_5678_ffee_ddccu64                                 Literal{kind: Hex}
	                                                  u64                                 Identifier
	                                                      ,                               PunctuationToken{tk: ","}
	                                                        0xccdd_eeff_7856_3412u64      Literal{kind: Hex}
	                                                                             u64      Identifier                                      */
    let mut passes: Vec<_> = passes.iter().map(|p| (p)()).collect();                                                                      /*
    let•mut•passes:•Vec<_>•=•passes.iter().map(|p|•(p)()).collect();    LetVariableDeclaration
        mut•passes                                                      PatternVariableDeclaration{!ref, mut}
                    Vec<_>                                              TypeCall
                       <_>                                              TypeCall.typeArguments{dk: "<>"}
                        _                                               TypeInferred
                             passes.iter().map(|p|•(p)()).collect()     CallExpression
                             passes.iter().map(|p|•(p)())               CallExpression
                             passes.iter()                              CallExpression
                                        ()                              CallExpression.arguments{dk: "()"}
                                              (|p|•(p)())               CallExpression.arguments{dk: "()"}
                                               |p|•(p)()                ClosureFunctionExpression
                                               |p|                      ClosureFunctionExpression.parameters{dk: "||"}
                                                p                       ClosureFunctionParameterDeclaration
                                                   (p)()                CallExpression
                                                      ()                CallExpression.arguments{dk: "()"}
                                                                 ()     CallExpression.arguments{dk: "()"}                                */
	(*DEFAULT_HOOK)(info);                                                                                                                /*
	(*DEFAULT_HOOK)(info);    ExpressionStatement{semi}
	(*DEFAULT_HOOK)(info)     CallExpression
	 *DEFAULT_HOOK            DereferenceExpression
	               (info)     CallExpression.arguments{dk: "()"}                                                                          */
	(group.apply)(&mut opts);                                                                                                             /*
	(group.apply)(&mut•opts);    ExpressionStatement{semi}
	(group.apply)(&mut•opts)     CallExpression
	 group.apply                 MemberExpression{!computed}
	             (&mut•opts)     CallExpression.arguments{dk: "()"}
	              &mut•opts      ReferenceExpression{mut}                                                                                 */
	Some((size, 1u128 << (size.bits() as u128 - 1)));                                                                                     /*
	Some((size,•1u128•<<•(size.bits()•as•u128•-•1)));    ExpressionStatement{semi}
	Some((size,•1u128•<<•(size.bits()•as•u128•-•1)))     CallExpression
	    ((size,•1u128•<<•(size.bits()•as•u128•-•1)))     CallExpression.arguments{dk: "()"}
	     (size,•1u128•<<•(size.bits()•as•u128•-•1))      TupleLiteral
	            1u128•<<•(size.bits()•as•u128•-•1)       OperationExpression{tk: "<<"}
	            1u128                                    Literal{kind: Integer}
	             u128                                    Identifier
	                      size.bits()•as•u128•-•1        OperationExpression{tk: "-"}
	                      size.bits()•as•u128            ExpressionAsTypeCast
	                      size.bits()                    CallExpression
	                               ()                    CallExpression.arguments{dk: "()"}
	                                            1        Literal{kind: Integer}                                                           */
	(lo == other_hi || hi == other_lo) && !self.is_singleton() && !other.is_singleton();                                                  /*
	(lo•==•other_hi•||•hi•==•other_lo)•&&•!self.is_singleton()•&&•!other.is_singleton();    ExpressionStatement{semi}
	(lo•==•other_hi•||•hi•==•other_lo)•&&•!self.is_singleton()•&&•!other.is_singleton()     AndExpression{tk: "&&"}
	(lo•==•other_hi•||•hi•==•other_lo)•&&•!self.is_singleton()                              AndExpression{tk: "&&"}
	 lo•==•other_hi•||•hi•==•other_lo                                                       OrExpression{tk: "||"}
	 lo•==•other_hi                                                                         ComparisonExpression{tk: "=="}
	                   hi•==•other_lo                                                       ComparisonExpression{tk: "=="}
	                                      !self.is_singleton()                              NotExpression
	                                       self.is_singleton()                              CallExpression
	                                                        ()                              CallExpression.arguments{dk: "()"}
	                                                              !other.is_singleton()     NotExpression
	                                                               other.is_singleton()     CallExpression
	                                                                                 ()     CallExpression.arguments{dk: "()"}            */
	
	(|A { x: mut t }: A| { t = t+1; t })(A { x: 34 });                                                                                    /*
	(|A•{•x:•mut•t•}:•A|•{•t•=•t+1;•t•})(A•{•x:•34•});    ExpressionStatement{semi}
	(|A•{•x:•mut•t•}:•A|•{•t•=•t+1;•t•})(A•{•x:•34•})     CallExpression
	 |A•{•x:•mut•t•}:•A|•{•t•=•t+1;•t•}                   ClosureFunctionExpression
	 |A•{•x:•mut•t•}:•A|                                  ClosureFunctionExpression.parameters{dk: "||"}
	  A•{•x:•mut•t•}:•A                                   ClosureFunctionParameterDeclaration
	  A•{•x:•mut•t•}                                      StructPattern
	    {•x:•mut•t•}                                      StructPattern.properties{dk: "{}"}
	      x:•mut•t                                        StructPatternPropertyDestructured
	         mut•t                                        PatternVariableDeclaration{!ref, mut}
	                     {•t•=•t+1;•t•}                   BlockExpression
	                       t•=•t+1;                       ExpressionStatement{semi}
	                       t•=•t+1                        ReassignmentExpression{tk: "="}
	                           t+1                        OperationExpression{tk: "+"}
	                             1                        Literal{kind: Integer}
	                                t                     ExpressionStatement{!semi}
	                                    (A•{•x:•34•})     CallExpression.arguments{dk: "()"}
	                                     A•{•x:•34•}      StructLiteral
	                                       {•x:•34•}      StructLiteral.properties{dk: "{}"}
	                                         x:•34        StructLiteralProperty
	                                            34        Literal{kind: Integer}                                                          */
	(async || 2333)().await;                                                                                                              /*
	(async•||•2333)().await;    ExpressionStatement{semi}
	(async•||•2333)().await     AwaitExpression
	(async•||•2333)()           CallExpression
	 async•||•2333              ClosureFunctionExpression{async}
	       ||                   ClosureFunctionExpression.parameters{dk: "||"}
	          2333              Literal{kind: Integer}
	               ()           CallExpression.arguments{dk: "()"}                                                                        */
    (async move || -> u8 { 42 })();                                                                                                       /*
    (async•move•||•->•u8•{•42•})();    ExpressionStatement{semi}
    (async•move•||•->•u8•{•42•})()     CallExpression
     async•move•||•->•u8•{•42•}        ClosureFunctionExpression{move, async}
                ||                     ClosureFunctionExpression.parameters{dk: "||"}
                         {•42•}        BlockExpression
                           42          ExpressionStatement{!semi}, Literal{kind: Integer}
                                ()     CallExpression.arguments{dk: "()"}                                                                 */
	(S.g(1,2))(true);                                                                                                                     /*
	(S.g(1,2))(true);    ExpressionStatement{semi}
	(S.g(1,2))(true)     CallExpression
	 S.g(1,2)            CallExpression
	    (1,2)            CallExpression.arguments{dk: "()"}
	     1               Literal{kind: Integer}
	       2             Literal{kind: Integer}
	          (true)     CallExpression.arguments{dk: "()"}
	           true      Literal{kind: True}                                                                                              */
	&Ast::Num((*f)(x));                                                                                                                   /*
	&Ast::Num((*f)(x));    ExpressionStatement{semi}
	&Ast::Num((*f)(x))     ReferenceExpression{!mut}
	 Ast::Num((*f)(x))     CallExpression
	 Ast::Num              ExpressionPath
	         ((*f)(x))     CallExpression.arguments{dk: "()"}
	          (*f)(x)      CallExpression
	           *f          DereferenceExpression
	              (x)      CallExpression.arguments{dk: "()"}                                                                             */
	f(&mut "Hello".to_owned());                                                                                                           /*
	f(&mut•"Hello".to_owned());    ExpressionStatement{semi}
	f(&mut•"Hello".to_owned())     CallExpression
	 (&mut•"Hello".to_owned())     CallExpression.arguments{dk: "()"}
	  &mut•"Hello".to_owned()      ReferenceExpression{mut}
	       "Hello".to_owned()      CallExpression
	       "Hello"                 Literal{kind: String}
	                       ()      CallExpression.arguments{dk: "()"}                                                                     */
	Box::new(move |x| f()(x));                                                                                                            /*
	Box::new(move•|x|•f()(x));    ExpressionStatement{semi}
	Box::new(move•|x|•f()(x))     CallExpression
	Box::new                      ExpressionPath
	        (move•|x|•f()(x))     CallExpression.arguments{dk: "()"}
	         move•|x|•f()(x)      ClosureFunctionExpression{move}
	              |x|             ClosureFunctionExpression.parameters{dk: "||"}
	               x              ClosureFunctionParameterDeclaration
	                  f()(x)      CallExpression
	                  f()         CallExpression
	                   ()         CallExpression.arguments{dk: "()"}
	                     (x)      CallExpression.arguments{dk: "()"}                                                                      */
	let a = Some(1u8).map(|a| foo(a));                                                                                                    /*
	let•a•=•Some(1u8).map(|a|•foo(a));    LetVariableDeclaration
	        Some(1u8).map(|a|•foo(a))     CallExpression
	        Some(1u8)                     CallExpression
	            (1u8)                     CallExpression.arguments{dk: "()"}
	             1u8                      Literal{kind: Integer}
	              u8                      Identifier
	                     (|a|•foo(a))     CallExpression.arguments{dk: "()"}
	                      |a|•foo(a)      ClosureFunctionExpression
	                      |a|             ClosureFunctionExpression.parameters{dk: "||"}
	                       a              ClosureFunctionParameterDeclaration
	                          foo(a)      CallExpression
	                             (a)      CallExpression.arguments{dk: "()"}                                                              */
    let c = Some(1u8).map(|a| {1+2; foo}(a));                                                                                             /*
    let•c•=•Some(1u8).map(|a|•{1+2;•foo}(a));    LetVariableDeclaration
            Some(1u8).map(|a|•{1+2;•foo}(a))     CallExpression
            Some(1u8)                            CallExpression
                (1u8)                            CallExpression.arguments{dk: "()"}
                 1u8                             Literal{kind: Integer}
                  u8                             Identifier
                         (|a|•{1+2;•foo}(a))     CallExpression.arguments{dk: "()"}
                          |a|•{1+2;•foo}(a)      ClosureFunctionExpression
                          |a|                    ClosureFunctionExpression.parameters{dk: "||"}
                           a                     ClosureFunctionParameterDeclaration
                              {1+2;•foo}(a)      CallExpression
                              {1+2;•foo}         BlockExpression
                               1+2;              ExpressionStatement{semi}
                               1+2               OperationExpression{tk: "+"}
                               1                 Literal{kind: Integer}
                                 2               Literal{kind: Integer}
                                    foo          ExpressionStatement{!semi}
                                        (a)      CallExpression.arguments{dk: "()"}                                                       */
    true.then(|| mac!());                                                                                                                 /*
    true.then(||•mac!());    ExpressionStatement{semi}
    true.then(||•mac!())     CallExpression
    true                     Literal{kind: True}
             (||•mac!())     CallExpression.arguments{dk: "()"}
              ||•mac!()      ClosureFunctionExpression
              ||             ClosureFunctionExpression.parameters{dk: "||"}
                 mac!()      MacroInvocation
                     ()      MacroInvocation.segments{dk: "()"}                                                                           */
    Some(1).map(closure_mac!());                                                                                                          /*
    Some(1).map(closure_mac!());    ExpressionStatement{semi}
    Some(1).map(closure_mac!())     CallExpression
    Some(1)                         CallExpression
        (1)                         CallExpression.arguments{dk: "()"}
         1                          Literal{kind: Integer}
               (closure_mac!())     CallExpression.arguments{dk: "()"}
                closure_mac!()      MacroInvocation
                            ()      MacroInvocation.segments{dk: "()"}                                                                    */
    let _: Option<Vec<u8>> = true.then(|| vec![]);                                                                                        /*
    let•_:•Option<Vec<u8>>•=•true.then(||•vec![]);    LetVariableDeclaration
        _                                             WildcardPattern
           Option<Vec<u8>>                            TypeCall
                 <Vec<u8>>                            TypeCall.typeArguments{dk: "<>"}
                  Vec<u8>                             TypeCall
                     <u8>                             TypeCall.typeArguments{dk: "<>"}
                             true.then(||•vec![])     CallExpression
                             true                     Literal{kind: True}
                                      (||•vec![])     CallExpression.arguments{dk: "()"}
                                       ||•vec![]      ClosureFunctionExpression
                                       ||             ClosureFunctionExpression.parameters{dk: "||"}
                                          vec![]      MacroInvocation
                                              []      MacroInvocation.segments{dk: "[]"}                                                  */
    let d = Some(1u8).map(|a| foo((|b| foo2(b))(a)));                                                                                     /*
    let•d•=•Some(1u8).map(|a|•foo((|b|•foo2(b))(a)));    LetVariableDeclaration
            Some(1u8).map(|a|•foo((|b|•foo2(b))(a)))     CallExpression
            Some(1u8)                                    CallExpression
                (1u8)                                    CallExpression.arguments{dk: "()"}
                 1u8                                     Literal{kind: Integer}
                  u8                                     Identifier
                         (|a|•foo((|b|•foo2(b))(a)))     CallExpression.arguments{dk: "()"}
                          |a|•foo((|b|•foo2(b))(a))      ClosureFunctionExpression
                          |a|                            ClosureFunctionExpression.parameters{dk: "||"}
                           a                             ClosureFunctionParameterDeclaration
                              foo((|b|•foo2(b))(a))      CallExpression
                                 ((|b|•foo2(b))(a))      CallExpression.arguments{dk: "()"}
                                  (|b|•foo2(b))(a)       CallExpression
                                   |b|•foo2(b)           ClosureFunctionExpression
                                   |b|                   ClosureFunctionExpression.parameters{dk: "||"}
                                    b                    ClosureFunctionParameterDeclaration
                                       foo2(b)           CallExpression
                                           (b)           CallExpression.arguments{dk: "()"}
                                               (a)       CallExpression.arguments{dk: "()"}                                               */
    all(&[1, 2, 3], &&2, |x, y| below(x, y));                                                                                             /*
    all(&[1,•2,•3],•&&2,•|x,•y|•below(x,•y));    ExpressionStatement{semi}
    all(&[1,•2,•3],•&&2,•|x,•y|•below(x,•y))     CallExpression
       (&[1,•2,•3],•&&2,•|x,•y|•below(x,•y))     CallExpression.arguments{dk: "()"}
        &[1,•2,•3]                               ReferenceExpression{!mut}
         [1,•2,•3]                               ArrayLiteral
          1                                      Literal{kind: Integer}
             2                                   Literal{kind: Integer}
                3                                Literal{kind: Integer}
                    &&2                          ReferenceExpression{!mut}
                     &2                          ReferenceExpression{!mut}
                      2                          Literal{kind: Integer}
                         |x,•y|•below(x,•y)      ClosureFunctionExpression
                         |x,•y|                  ClosureFunctionExpression.parameters{dk: "||"}
                          x                      ClosureFunctionParameterDeclaration
                             y                   ClosureFunctionParameterDeclaration
                                below(x,•y)      CallExpression
                                     (x,•y)      CallExpression.arguments{dk: "()"}                                                       */
	let a: Option<Box<dyn (::std::ops::Deref<Target = [i32]>)>> =                                                                         /*
	let•a:•Option<Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>>•=↲    <LetVariableDeclaration>
	       Option<Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>>       TypeCall
	             <Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>>       TypeCall.typeArguments{dk: "<>"}
	              Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>        TypeCall
	                 <dyn•(::std::ops::Deref<Target•=•[i32]>)>        TypeCall.typeArguments{dk: "<>"}
	                  dyn•(::std::ops::Deref<Target•=•[i32]>)         TypeDynBounds{dyn}
	                       ::std::ops::Deref<Target•=•[i32]>          TypeTraitBound{!maybeConst, !optional}, TypeCall
	                       ::std::ops::Deref                          TypePath
	                       ::std::ops                                 TypePath
	                       ::std                                      TypePath
	                                        <Target•=•[i32]>          TypeCall.typeArguments{dk: "<>"}
	                                         Target•=•[i32]           TypeCallNamedArgument
	                                                  [i32]           TypeSlice                                                           */
        Some(vec![1i32, 2]).map(|v| -> Box<dyn (::std::ops::Deref<Target = [i32]>)> { Box::new(v) });                                     /*
        Some(vec![1i32,•2]).map(|v|•->•Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>•{•Box::new(v)•})     CallExpression
        Some(vec![1i32,•2])                                                                              CallExpression
            (vec![1i32,•2])                                                                              CallExpression.arguments{dk: "()"}
             vec![1i32,•2]                                                                               MacroInvocation
                 [1i32,•2]                                                                               MacroInvocation.segments{dk: "[]"}
                  1i32                                                                                   Literal{kind: Integer}
                   i32                                                                                   Identifier
                      ,                                                                                  PunctuationToken{tk: ","}
                        2                                                                                Literal{kind: Integer}
                               (|v|•->•Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>•{•Box::new(v)•})     CallExpression.arguments{dk: "()"}
                                |v|•->•Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>•{•Box::new(v)•}      ClosureFunctionExpression
                                |v|                                                                      ClosureFunctionExpression.parameters{dk: "||"}
                                 v                                                                       ClosureFunctionParameterDeclaration
                                       Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>                      TypeCall
                                          <dyn•(::std::ops::Deref<Target•=•[i32]>)>                      TypeCall.typeArguments{dk: "<>"}
                                           dyn•(::std::ops::Deref<Target•=•[i32]>)                       TypeDynBounds{dyn}
                                                ::std::ops::Deref<Target•=•[i32]>                        TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                ::std::ops::Deref                                        TypePath
                                                ::std::ops                                               TypePath
                                                ::std                                                    TypePath
                                                                 <Target•=•[i32]>                        TypeCall.typeArguments{dk: "<>"}
                                                                  Target•=•[i32]                         TypeCallNamedArgument
                                                                           [i32]                         TypeSlice
                                                                                    {•Box::new(v)•}      BlockExpression
                                                                                      Box::new(v)        ExpressionStatement{!semi}, CallExpression
                                                                                      Box::new           ExpressionPath
                                                                                              (v)        CallExpression.arguments{dk: "()"}
••••••••Some(vec![1i32,•2]).map(|v|•->•Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>•{•Box::new(v)•});    </LetVariableDeclaration>        */
	#[allow(clippy::needless_return)]                                                                                                     /*
	#[allow(clippy::needless_return)]↲    <ExpressionStatement{semi}>
	#[allow(clippy::needless_return)]     Attribute{!inner}
	 [allow(clippy::needless_return)]     Attribute.segments{dk: "[]"}
	       (clippy::needless_return)      DelimGroup
	              ::                      PunctuationToken{tk: "::"}                                                                      */
    (|| return 2)();                                                                                                                      /*
    (||•return•2)();    ExpressionStatement~ownStart
    (||•return•2)()     CallExpression
     ||•return•2        ClosureFunctionExpression
     ||                 ClosureFunctionExpression.parameters{dk: "||"}
        return•2        ReturnExpression
               2        Literal{kind: Integer}
                 ()     CallExpression.arguments{dk: "()"}
••••(||•return•2)();    </ExpressionStatement>                                                                                            */
    (|| -> Option<i32> { None? })();                                                                                                      /*
    (||•->•Option<i32>•{•None?•})();    ExpressionStatement{semi}
    (||•->•Option<i32>•{•None?•})()     CallExpression
     ||•->•Option<i32>•{•None?•}        ClosureFunctionExpression
     ||                                 ClosureFunctionExpression.parameters{dk: "||"}
           Option<i32>                  TypeCall
                 <i32>                  TypeCall.typeArguments{dk: "<>"}
                       {•None?•}        BlockExpression
                         None?          ExpressionStatement{!semi}, UnwrapExpression
                                 ()     CallExpression.arguments{dk: "()"}                                                                */
    #[allow(clippy::try_err)]                                                                                                             /*
    #[allow(clippy::try_err)]↲    <ExpressionStatement{semi}>
    #[allow(clippy::try_err)]     Attribute{!inner}
     [allow(clippy::try_err)]     Attribute.segments{dk: "[]"}
           (clippy::try_err)      DelimGroup
                  ::              PunctuationToken{tk: "::"}                                                                              */
    (|| -> Result<i32, i32> { Err(2)? })();                                                                                               /*
    (||•->•Result<i32,•i32>•{•Err(2)?•})();    ExpressionStatement~ownStart
    (||•->•Result<i32,•i32>•{•Err(2)?•})()     CallExpression
     ||•->•Result<i32,•i32>•{•Err(2)?•}        ClosureFunctionExpression
     ||                                        ClosureFunctionExpression.parameters{dk: "||"}
           Result<i32,•i32>                    TypeCall
                 <i32,•i32>                    TypeCall.typeArguments{dk: "<>"}
                            {•Err(2)?•}        BlockExpression
                              Err(2)?          ExpressionStatement{!semi}, UnwrapExpression
                              Err(2)           CallExpression
                                 (2)           CallExpression.arguments{dk: "()"}
                                  2            Literal{kind: Integer}
                                        ()     CallExpression.arguments{dk: "()"}
••••(||•->•Result<i32,•i32>•{•Err(2)?•})();    </ExpressionStatement>                                                                     */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

static mut NUM: u8 = 6 * 7;                                                                                                               /*
static•mut•NUM:•u8•=•6•*•7;    StaticVariableDeclaration
       mut•NUM                 PatternVariableDeclaration{!ref, mut}
                     6•*•7     OperationExpression{tk: "*"}
                     6         Literal{kind: Integer}
                         7     Literal{kind: Integer}                                                                                     */
static NUM_REF: &'static u8 = unsafe { &NUM };                                                                                            /*
static•NUM_REF:•&'static•u8•=•unsafe•{•&NUM•};    StaticVariableDeclaration
                &'static•u8                       TypeReference{!mut}
                 'static                          LtStatic
                              unsafe•{•&NUM•}     BlockExpression{unsafe}
                                     {•&NUM•}     BlockExpression.body{dk: "{}"}
                                       &NUM       ExpressionStatement{!semi}, ReferenceExpression{!mut}                                   */
impl<T: ?Sized, U: ?Sized> CoerceUnsized<Unique<U>> for Unique<T> where T: Unsize<U> {}                                                   /*
impl<T:•?Sized,•U:•?Sized>•CoerceUnsized<Unique<U>>•for•Unique<T>•where•T:•Unsize<U>•{}    ImplDeclaration{!const}
    <T:•?Sized,•U:•?Sized>                                                                 ImplDeclaration.generics{dk: "<>"}
     T:•?Sized                                                                             GenericTypeParameterDeclaration
        ?Sized                                                                             TypeTraitBound{!maybeConst, optional}
                U:•?Sized                                                                  GenericTypeParameterDeclaration
                   ?Sized                                                                  TypeTraitBound{!maybeConst, optional}
                           CoerceUnsized<Unique<U>>                                        TypeCall
                                        <Unique<U>>                                        TypeCall.typeArguments{dk: "<>"}
                                         Unique<U>                                         TypeCall
                                               <U>                                         TypeCall.typeArguments{dk: "<>"}
                                                        Unique<T>                          TypeCall
                                                              <T>                          TypeCall.typeArguments{dk: "<>"}
                                                                  where•T:•Unsize<U>       ImplDeclaration.whereBounds{dk: "None"}
                                                                        T:•Unsize<U>       WhereTypeBoundDeclaration
                                                                           Unsize<U>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                                                 <U>       TypeCall.typeArguments{dk: "<>"}
                                                                                     {}    ImplDeclaration.body{dk: "{}"}                 */

fn cvgsk_nichqsd_bhvior () {                                                                                                              /*
fn•cvgsk_nichqsd_bhvior•()•{↲    <FunctionDeclaration>
                        ()       FunctionDeclaration.parameters{dk: "()"}
                           {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                     */
    if let E1::V2 { .. } = (E1::V1 { f: true }) {                                                                                         /*
    if•let•E1::V2•{•..•}•=•(E1::V1•{•f:•true•})•{↲    <ExpressionStatement{!semi}>
    if•let•E1::V2•{•..•}•=•(E1::V1•{•f:•true•})•{↲    <IfBlockExpression>
       let•E1::V2•{•..•}•=•(E1::V1•{•f:•true•})       LetScrutinee
           E1::V2•{•..•}                              StructPattern
           E1::V2                                     ExpressionPath
                  {•..•}                              StructPattern.properties{dk: "{}"}
                    ..                                RestPattern
                            E1::V1•{•f:•true•}        StructLiteral
                            E1::V1                    ExpressionPath
                                   {•f:•true•}        StructLiteral.properties{dk: "{}"}
                                     f:•true          StructLiteralProperty
                                        true          Literal{kind: True}
                                                {↲    <IfBlockExpression.body{dk: "{}"}>                                                  */
        intarvics::avort();                                                                                                               /*
        intarvics::avort();    ExpressionStatement{semi}
        intarvics::avort()     CallExpression
        intarvics::avort       ExpressionPath
                        ()     CallExpression.arguments{dk: "()"}                                                                         */
    }                                                                                                                                     /*
••••}    </IfBlockExpression.body>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */

    if let E2::V1 { .. } = E2::V3::<Inwxvlible> {                                                                                         /*
    if•let•E2::V1•{•..•}•=•E2::V3::<Inwxvlible>•{↲    <ExpressionStatement{!semi}>
    if•let•E2::V1•{•..•}•=•E2::V3::<Inwxvlible>•{↲    <IfBlockExpression>
       let•E2::V1•{•..•}•=•E2::V3::<Inwxvlible>       LetScrutinee
           E2::V1•{•..•}                              StructPattern
           E2::V1                                     ExpressionPath
                  {•..•}                              StructPattern.properties{dk: "{}"}
                    ..                                RestPattern
                           E2::V3::<Inwxvlible>       ExpressionTypeCast
                           E2::V3                     ExpressionPath
                                   <Inwxvlible>       ExpressionTypeCast.typeArguments{dk: "<>"}
                                                {↲    <IfBlockExpression.body{dk: "{}"}>                                                  */
        inzadqsics::abort();                                                                                                              /*
        inzadqsics::abort();    ExpressionStatement{semi}
        inzadqsics::abort()     CallExpression
        inzadqsics::abort       ExpressionPath
                         ()     CallExpression.arguments{dk: "()"}                                                                        */
    }                                                                                                                                     /*
••••}    </IfBlockExpression.body>
••••}    </IfBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

impl<'a, 'b> FnOnce<(&'a &'b [u16],)> for IsNotEmpty {                                                                                    /*
impl<'a,•'b>•FnOnce<(&'a•&'b•[u16],)>•for•IsNotEmpty•{↲    <ImplDeclaration{!const}>
    <'a,•'b>                                               ImplDeclaration.generics{dk: "<>"}
     'a                                                    GenericLtParameterDeclaration, LtIdentifier
         'b                                                GenericLtParameterDeclaration, LtIdentifier
             FnOnce<(&'a•&'b•[u16],)>                      TypeCall
                   <(&'a•&'b•[u16],)>                      TypeCall.typeArguments{dk: "<>"}
                    (&'a•&'b•[u16],)                       TypeTuple
                     &'a•&'b•[u16]                         TypeReference{!mut}
                      'a                                   LtIdentifier
                         &'b•[u16]                         TypeReference{!mut}
                          'b                               LtIdentifier
                             [u16]                         TypeSlice
                                                     {↲    <ImplDeclaration.body{dk: "{}"}>                                               */
    extern "rust-call" fn call_once(mut self, arg: (&'a &'b [u16],)) -> (u8, u8) {                                                        /*
    extern•"rust-call"•fn•call_once(mut•self,•arg:•(&'a•&'b•[u16],))•->•(u8,•u8)•{↲    <FunctionDeclaration>
    extern•"rust-call"                                                                 ExternSpecifier
           "rust-call"                                                                 Literal{kind: String}
                                   (mut•self,•arg:•(&'a•&'b•[u16],))                   FunctionDeclaration.parameters{dk: "()"}
                                    mut•self                                           FunctionSelfParameterDeclaration{!ref, mut}
                                              arg:•(&'a•&'b•[u16],)                    FunctionParameterDeclaration
                                                   (&'a•&'b•[u16],)                    TypeTuple
                                                    &'a•&'b•[u16]                      TypeReference{!mut}
                                                     'a                                LtIdentifier
                                                        &'b•[u16]                      TypeReference{!mut}
                                                         'b                            LtIdentifier
                                                            [u16]                      TypeSlice
                                                                        (u8,•u8)       TypeTuple
                                                                                 {↲    <FunctionDeclaration.body{dk: "{}"}>               */
        self.call_mut(arg)                                                                                                                /*
        self.call_mut(arg)    ExpressionStatement{!semi}, CallExpression
                     (arg)    CallExpression.arguments{dk: "()"}                                                                          */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration.body>
••••}    </FunctionDeclaration>                                                                                                           */
    extern "rust-call" fn call_once123(mut self, arg: (&'a &'b [u16],)) -> (u8, u8) {                                                     /*
    extern•"rust-call"•fn•call_once123(mut•self,•arg:•(&'a•&'b•[u16],))•->•(u8,•u8)•{↲    <FunctionDeclaration>
    extern•"rust-call"                                                                    ExternSpecifier
           "rust-call"                                                                    Literal{kind: String}
                                      (mut•self,•arg:•(&'a•&'b•[u16],))                   FunctionDeclaration.parameters{dk: "()"}
                                       mut•self                                           FunctionSelfParameterDeclaration{!ref, mut}
                                                 arg:•(&'a•&'b•[u16],)                    FunctionParameterDeclaration
                                                      (&'a•&'b•[u16],)                    TypeTuple
                                                       &'a•&'b•[u16]                      TypeReference{!mut}
                                                        'a                                LtIdentifier
                                                           &'b•[u16]                      TypeReference{!mut}
                                                            'b                            LtIdentifier
                                                               [u16]                      TypeSlice
                                                                           (u8,•u8)       TypeTuple
                                                                                    {↲    <FunctionDeclaration.body{dk: "{}"}>            */
        self.call_mut(arg)                                                                                                                /*
        self.call_mut(arg)    ExpressionStatement{!semi}, CallExpression
                     (arg)    CallExpression.arguments{dk: "()"}                                                                          */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration.body>
••••}    </FunctionDeclaration>                                                                                                           */
    extern "rust-call" fn call_mut(&mut self, _arg: (&'a &'b [u16],)) -> (u8, u8) {                                                       /*
    extern•"rust-call"•fn•call_mut(&mut•self,•_arg:•(&'a•&'b•[u16],))•->•(u8,•u8)•{↲    <FunctionDeclaration>
    extern•"rust-call"                                                                  ExternSpecifier
           "rust-call"                                                                  Literal{kind: String}
                                  (&mut•self,•_arg:•(&'a•&'b•[u16],))                   FunctionDeclaration.parameters{dk: "()"}
                                   &mut•self                                            FunctionSelfParameterDeclaration{ref, mut}
                                              _arg:•(&'a•&'b•[u16],)                    FunctionParameterDeclaration
                                                    (&'a•&'b•[u16],)                    TypeTuple
                                                     &'a•&'b•[u16]                      TypeReference{!mut}
                                                      'a                                LtIdentifier
                                                         &'b•[u16]                      TypeReference{!mut}
                                                          'b                            LtIdentifier
                                                             [u16]                      TypeSlice
                                                                         (u8,•u8)       TypeTuple
                                                                                  {↲    <FunctionDeclaration.body{dk: "{}"}>              */
        (0, 42)                                                                                                                           /*
        (0,•42)    ExpressionStatement{!semi}, TupleLiteral
         0         Literal{kind: Integer}
            42     Literal{kind: Integer}                                                                                                 */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration.body>
••••}    </FunctionDeclaration>                                                                                                           */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */

pub fn call_is_not_empty() {                                                                                                              /*
pub•fn•call_is_not_empty()•{↲    <FunctionDeclaration>
pub                              PubSpecifier
                        ()       FunctionDeclaration.parameters{dk: "()"}
                           {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                     */
    IsNotEmpty.call_once((&(&[0u16] as &[_]),));                                                                                          /*
    IsNotEmpty.call_once((&(&[0u16]•as•&[_]),));    ExpressionStatement{semi}
    IsNotEmpty.call_once((&(&[0u16]•as•&[_]),))     CallExpression
                        ((&(&[0u16]•as•&[_]),))     CallExpression.arguments{dk: "()"}
                         (&(&[0u16]•as•&[_]),)      TupleLiteral
                          &(&[0u16]•as•&[_])        ReferenceExpression{!mut}
                            &[0u16]•as•&[_]         ExpressionAsTypeCast
                            &[0u16]                 ReferenceExpression{!mut}
                             [0u16]                 ArrayLiteral
                              0u16                  Literal{kind: Integer}
                               u16                  Identifier
                                       &[_]         TypeReference{!mut}
                                        [_]         TypeSlice
                                         _          TypeInferred                                                                          */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

EnumTypeFoldableImpl! {                                                                                                                   /*
EnumTypeFoldableImpl!•{↲    <ExpressionStatement{!semi}>
EnumTypeFoldableImpl!•{↲    <MacroInvocation>
                      {↲    <MacroInvocation.segments{dk: "{}"}>                                                                          */
    impl<'tcx, T> TypeFoldable<'tcx> for Option<T> {                                                                                      /*
        <                                                PunctuationToken{tk: "<"}
         'tcx                                            LtIdentifier
             ,                                           PunctuationToken{tk: ","}
                >                                        PunctuationToken{tk: ">"}
                              <                          PunctuationToken{tk: "<"}
                               'tcx                      LtIdentifier
                                   >                     PunctuationToken{tk: ">"}
                                               <         PunctuationToken{tk: "<"}
                                                 >       PunctuationToken{tk: ">"}
                                                   {↲    <DelimGroup>                                                                     */
        (Some)(a),                                                                                                                        /*
        (Some)        DelimGroup
              (a)     DelimGroup
                 ,    PunctuationToken{tk: ","}                                                                                           */
        (None),                                                                                                                           /*
        (None)     DelimGroup
              ,    PunctuationToken{tk: ","}                                                                                              */
    } where T: TypeFoldable<'tcx>                                                                                                         /*
••••}                                </DelimGroup>
             :                       PunctuationToken{tk: ":"}
                           <         PunctuationToken{tk: "<"}
                            'tcx     LtIdentifier
                                >    PunctuationToken{tk: ">"}                                                                            */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

fn x() {                                                                                                                                  /*
fn•x()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
                      a   .b   .c    ;                                                                                                    /*
                      a•••.b•••.c••••;    ExpressionStatement{semi}
                      a•••.b•••.c         MemberExpression{!computed}
                      a•••.b              MemberExpression{!computed}                                                                     */
                    ( a ) .b   .c    ;                                                                                                    /*
                    (•a•)•.b•••.c••••;    ExpressionStatement{semi}
                    (•a•)•.b•••.c         MemberExpression{!computed}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
                  ( ( a ) .b ) .c    ;                                                                                                    /*
                  (•(•a•)•.b•)•.c••••;    ExpressionStatement{semi}
                  (•(•a•)•.b•)•.c         MemberExpression{!computed}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
                ( ( ( a ) .b ) .c )  ;                                                                                                    /*
                (•(•(•a•)•.b•)•.c•)••;    ExpressionStatement{semi}
                  (•(•a•)•.b•)•.c         MemberExpression{!computed}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
                (   ( a ) .b   .c )  ;                                                                                                    /*
                (•••(•a•)•.b•••.c•)••;    ExpressionStatement{semi}
                    (•a•)•.b•••.c         MemberExpression{!computed}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
                  (   a   .b ) .c    ;                                                                                                    /*
                  (•••a•••.b•)•.c••••;    ExpressionStatement{semi}
                  (•••a•••.b•)•.c         MemberExpression{!computed}
                      a•••.b              MemberExpression{!computed}                                                                     */
                ( (   a   .b ) .c )  ;                                                                                                    /*
                (•(•••a•••.b•)•.c•)••;    ExpressionStatement{semi}
                  (•••a•••.b•)•.c         MemberExpression{!computed}
                      a•••.b              MemberExpression{!computed}                                                                     */
                (     a   .b   .c )  ;                                                                                                    /*
                (•••••a•••.b•••.c•)••;    ExpressionStatement{semi}
                      a•••.b•••.c         MemberExpression{!computed}
                      a•••.b              MemberExpression{!computed}                                                                     */
              
              (       a   .b   .c   );                                                                                                    /*
              (•••••••a•••.b•••.c•••);    ExpressionStatement{semi}
                      a•••.b•••.c         MemberExpression{!computed}
                      a•••.b              MemberExpression{!computed}                                                                     */
              (     ( a ) .b   .c   );                                                                                                    /*
              (•••••(•a•)•.b•••.c•••);    ExpressionStatement{semi}
                    (•a•)•.b•••.c         MemberExpression{!computed}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
              (   ( ( a ) .b ) .c   );                                                                                                    /*
              (•••(•(•a•)•.b•)•.c•••);    ExpressionStatement{semi}
                  (•(•a•)•.b•)•.c         MemberExpression{!computed}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
              ( ( ( ( a ) .b ) .c ) );                                                                                                    /*
              (•(•(•(•a•)•.b•)•.c•)•);    ExpressionStatement{semi}
                  (•(•a•)•.b•)•.c         MemberExpression{!computed}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
              ( (   ( a ) .b   .c ) );                                                                                                    /*
              (•(•••(•a•)•.b•••.c•)•);    ExpressionStatement{semi}
                    (•a•)•.b•••.c         MemberExpression{!computed}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
              (   (   a   .b ) .c   );                                                                                                    /*
              (•••(•••a•••.b•)•.c•••);    ExpressionStatement{semi}
                  (•••a•••.b•)•.c         MemberExpression{!computed}
                      a•••.b              MemberExpression{!computed}                                                                     */
              ( ( (   a   .b ) .c ) );                                                                                                    /*
              (•(•(•••a•••.b•)•.c•)•);    ExpressionStatement{semi}
                  (•••a•••.b•)•.c         MemberExpression{!computed}
                      a•••.b              MemberExpression{!computed}                                                                     */
              ( (     a   .b   .c ) );                                                                                                    /*
              (•(•••••a•••.b•••.c•)•);    ExpressionStatement{semi}
                      a•••.b•••.c         MemberExpression{!computed}
                      a•••.b              MemberExpression{!computed}                                                                     */

    foo(#[attr]       a   .b   .c   );                                                                                                    /*
    foo(#[attr]•••••••a•••.b•••.c•••);    ExpressionStatement{semi}
    foo(#[attr]•••••••a•••.b•••.c•••)     CallExpression
       (#[attr]•••••••a•••.b•••.c•••)     CallExpression.arguments{dk: "()"}
        #[attr]•••••••a•••.b•••.c         MemberExpression{!computed}
                      a•••.b•••.c         MemberExpression~ownStart
        #[attr]                           Attribute{!inner}
         [attr]                           Attribute.segments{dk: "[]"}
                      a•••.b              MemberExpression{!computed}                                                                     */
    foo(#[attr]     ( a ) .b   .c   );                                                                                                    /*
    foo(#[attr]•••••(•a•)•.b•••.c•••);    ExpressionStatement{semi}
    foo(#[attr]•••••(•a•)•.b•••.c•••)     CallExpression
       (#[attr]•••••(•a•)•.b•••.c•••)     CallExpression.arguments{dk: "()"}
        #[attr]•••••(•a•)•.b•••.c         MemberExpression{!computed}
                    (•a•)•.b•••.c         MemberExpression~ownStart
        #[attr]                           Attribute{!inner}
         [attr]                           Attribute.segments{dk: "[]"}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
    foo(#[attr]   ( ( a ) .b ) .c   );                                                                                                    /*
    foo(#[attr]•••(•(•a•)•.b•)•.c•••);    ExpressionStatement{semi}
    foo(#[attr]•••(•(•a•)•.b•)•.c•••)     CallExpression
       (#[attr]•••(•(•a•)•.b•)•.c•••)     CallExpression.arguments{dk: "()"}
        #[attr]•••(•(•a•)•.b•)•.c         MemberExpression{!computed}
                  (•(•a•)•.b•)•.c         MemberExpression~ownStart
        #[attr]                           Attribute{!inner}
         [attr]                           Attribute.segments{dk: "[]"}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
    foo(#[attr] ( ( ( a ) .b ) .c ) );                                                                                                    /*
    foo(#[attr]•(•(•(•a•)•.b•)•.c•)•);    ExpressionStatement{semi}
    foo(#[attr]•(•(•(•a•)•.b•)•.c•)•)     CallExpression
       (#[attr]•(•(•(•a•)•.b•)•.c•)•)     CallExpression.arguments{dk: "()"}
        #[attr]•(•(•(•a•)•.b•)•.c         MemberExpression{!computed}
                  (•(•a•)•.b•)•.c         MemberExpression~ownStart
        #[attr]                           Attribute{!inner}
         [attr]                           Attribute.segments{dk: "[]"}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
    foo(#[attr] (   ( a ) .b   .c ) );                                                                                                    /*
    foo(#[attr]•(•••(•a•)•.b•••.c•)•);    ExpressionStatement{semi}
    foo(#[attr]•(•••(•a•)•.b•••.c•)•)     CallExpression
       (#[attr]•(•••(•a•)•.b•••.c•)•)     CallExpression.arguments{dk: "()"}
        #[attr]•(•••(•a•)•.b•••.c         MemberExpression{!computed}
                    (•a•)•.b•••.c         MemberExpression~ownStart
        #[attr]                           Attribute{!inner}
         [attr]                           Attribute.segments{dk: "[]"}
                    (•a•)•.b              MemberExpression{!computed}                                                                     */
    foo(#[attr]   (   a   .b ) .c   );                                                                                                    /*
    foo(#[attr]•••(•••a•••.b•)•.c•••);    ExpressionStatement{semi}
    foo(#[attr]•••(•••a•••.b•)•.c•••)     CallExpression
       (#[attr]•••(•••a•••.b•)•.c•••)     CallExpression.arguments{dk: "()"}
        #[attr]•••(•••a•••.b•)•.c         MemberExpression{!computed}
                  (•••a•••.b•)•.c         MemberExpression~ownStart
        #[attr]                           Attribute{!inner}
         [attr]                           Attribute.segments{dk: "[]"}
                      a•••.b              MemberExpression{!computed}                                                                     */
    foo(#[attr] ( (   a   .b ) .c ) );                                                                                                    /*
    foo(#[attr]•(•(•••a•••.b•)•.c•)•);    ExpressionStatement{semi}
    foo(#[attr]•(•(•••a•••.b•)•.c•)•)     CallExpression
       (#[attr]•(•(•••a•••.b•)•.c•)•)     CallExpression.arguments{dk: "()"}
        #[attr]•(•(•••a•••.b•)•.c         MemberExpression{!computed}
                  (•••a•••.b•)•.c         MemberExpression~ownStart
        #[attr]                           Attribute{!inner}
         [attr]                           Attribute.segments{dk: "[]"}
                      a•••.b              MemberExpression{!computed}                                                                     */
    foo(#[attr] (     a   .b   .c ) );                                                                                                    /*
    foo(#[attr]•(•••••a•••.b•••.c•)•);    ExpressionStatement{semi}
    foo(#[attr]•(•••••a•••.b•••.c•)•)     CallExpression
       (#[attr]•(•••••a•••.b•••.c•)•)     CallExpression.arguments{dk: "()"}
        #[attr]•(•••••a•••.b•••.c         MemberExpression{!computed}
                      a•••.b•••.c         MemberExpression~ownStart
        #[attr]                           Attribute{!inner}
         [attr]                           Attribute.segments{dk: "[]"}
                      a•••.b              MemberExpression{!computed}                                                                     */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 80
// Parsed Nodes: 1211
// state_rollbacks: 16
// Total '.charCodeAt()' calls: 6408 (29% re-reads)
// Unnecessary 'skip_whitespace()' calls: 685
// source: "../../samples/expressions/parens.rs"