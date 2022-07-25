fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>                                                                                                     */
	holds_callable.callable();                                                                                                            /*
    holds_callable.callable();    ExpressionStatement
    holds_callable.callable()     CallExpression                                                                                          */
	(holds_callable.callable)();                                                                                                          /*
    (holds_callable.callable)();    ExpressionStatement
    (holds_callable.callable)()     CallExpression
     holds_callable.callable        MemberExpression                                                                                      */
    a = b = ( c );                                                                                                                        /*
    a•=•b•=•(•c•);    ExpressionStatement
    a•=•b•=•(•c•)     ReassignmentExpression
        b•=•(•c•)     ReassignmentExpression                                                                                              */
	mystruct.myfield;                                                                                                                     /*
    mystruct.myfield;    ExpressionStatement
    mystruct.myfield     MemberExpression                                                                                                 */
	foo().x;                                                                                                                              /*
    foo().x;    ExpressionStatement
    foo().x     MemberExpression
    foo()       CallExpression                                                                                                            */
	(Struct {a: 10, b: 20}).a;                                                                                                            /*
    (Struct•{a:•10,•b:•20}).a;    ExpressionStatement
    (Struct•{a:•10,•b:•20}).a     MemberExpression
     Struct•{a:•10,•b:•20}        StructLiteral
             a:•10                StructLiteralProperty
                10                Literal
                    b:•20         StructLiteralProperty
                       20         Literal                                                                                                 */
	(mystruct.function_field)();                                                                                                          /*
    (mystruct.function_field)();    ExpressionStatement
    (mystruct.function_field)()     CallExpression
     mystruct.function_field        MemberExpression                                                                                      */
	let name: &'static str = (|| "Rust")();                                                                                               /*
    let•name:•&'static•str•=•(||•"Rust")();    LetVariableDeclaration
              &'static•str                     TypeReference
               'static                         LtStatic
                             (||•"Rust")()     CallExpression
                              ||•"Rust"        ClosureFunctionExpression
                                 "Rust"        Literal                                                                                    */
	let x: i32 = 2 + 3 * 4;                                                                                                               /*
    let•x:•i32•=•2•+•3•*•4;    LetVariableDeclaration
                 2•+•3•*•4     OperationExpression
                 2             Literal
                     3•*•4     OperationExpression
                     3         Literal
                         4     Literal                                                                                                    */
	let y: i32 = (2 + 3) * 4;                                                                                                             /*
    let•y:•i32•=•(2•+•3)•*•4;    LetVariableDeclaration
                 (2•+•3)•*•4     OperationExpression
                  2•+•3          OperationExpression
                  2              Literal
                      3          Literal
                           4     Literal                                                                                                  */
	let lhs = &this.thir[lhs];                                                                                                            /*
    let•lhs•=•&this.thir[lhs];    LetVariableDeclaration
              &this.thir[lhs]     ReferenceExpression
               this.thir[lhs]     MemberExpression
               this.thir          MemberExpression                                                                                        */
    (*f)(&x);                                                                                                                             /*
    (*f)(&x);    ExpressionStatement
    (*f)(&x)     CallExpression
     *f          DereferenceExpression
         &x      ReferenceExpression                                                                                                      */
	(*x) * (*x);                                                                                                                          /*
    (*x)•*•(*x);    ExpressionStatement
    (*x)•*•(*x)     OperationExpression
     *x             DereferenceExpression
            *x      DereferenceExpression                                                                                                 */
	println!("{}", (self.0)());                                                                                                           /*
    println!("{}",•(self.0)());    ExpressionStatement
    println!("{}",•(self.0)())     MacroInvocation
             "{}"                  Literal
                 ,                 PunctuationToken
                   (self.0)        DelimGroup
                        .          PunctuationToken
                         0         Literal
                           ()      DelimGroup                                                                                             */
	(self.0)(ecx, span, meta_item, &item, &mut |a| items.push(a));                                                                        /*
    (self.0)(ecx,•span,•meta_item,•&item,•&mut•|a|•items.push(a));    ExpressionStatement
    (self.0)(ecx,•span,•meta_item,•&item,•&mut•|a|•items.push(a))     CallExpression
     self.0                                                           MemberExpression
          0                                                           Index
                                   &item                              ReferenceExpression
                                          &mut•|a|•items.push(a)      ReferenceExpression
                                               |a|•items.push(a)      ClosureFunctionExpression
                                                a                     ClosureFunctionParameterDeclaration
                                                   items.push(a)      CallExpression                                                      */
	(|_, _, _| {})(0u8, 42u16, 0u8);                                                                                                      /*
    (|_,•_,•_|•{})(0u8,•42u16,•0u8);    ExpressionStatement
    (|_,•_,•_|•{})(0u8,•42u16,•0u8)     CallExpression
     |_,•_,•_|•{}                       ClosureFunctionExpression
      _                                 ClosureFunctionParameterDeclaration, WildcardPattern
         _                              ClosureFunctionParameterDeclaration, WildcardPattern
            _                           ClosureFunctionParameterDeclaration, WildcardPattern
               {}                       BlockExpression
                   0u8                  Literal
                    u8                  Identifier
                        42u16           Literal
                          u16           Identifier
                               0u8      Literal
                                u8      Identifier                                                                                        */
	(|_, _| {})(0u8, 42u16);                                                                                                              /*
    (|_,•_|•{})(0u8,•42u16);    ExpressionStatement
    (|_,•_|•{})(0u8,•42u16)     CallExpression
     |_,•_|•{}                  ClosureFunctionExpression
      _                         ClosureFunctionParameterDeclaration, WildcardPattern
         _                      ClosureFunctionParameterDeclaration, WildcardPattern
            {}                  BlockExpression
                0u8             Literal
                 u8             Identifier
                     42u16      Literal
                       u16      Identifier                                                                                                */
	let x = &[0u32, 42u32] as &[u32];                                                                                                     /*
    let•x•=•&[0u32,•42u32]•as•&[u32];    LetVariableDeclaration
            &[0u32,•42u32]•as•&[u32]     ExpressionAsTypeCast
            &[0u32,•42u32]               ReferenceExpression
             [0u32,•42u32]               ArrayLiteral
              0u32                       Literal
               u32                       Identifier
                    42u32                Literal
                      u32                Identifier
                              &[u32]     TypeReference
                               [u32]     TypeSlice                                                                                        */
    match x {                                                                                                                             /*
    match•x•{↲    <ExpressionStatement>, <MatchExpression>                                                                                */
        [] => assert_eq!(0u32, 1),                                                                                                        /*
        []•=>•assert_eq!(0u32,•1)     MatchExpressionCase
        []                            ArrayPattern
              assert_eq!(0u32,•1)     MacroInvocation
                         0u32         Literal
                          u32         Identifier
                             ,        PunctuationToken
                               1      Literal                                                                                             */
        [_, ref y @ ..] => assert_eq!(&x[1] as *const u32 as usize, &y[0] as *const u32 as usize),                                        /*
        [_,•ref•y•@•..]•=>•assert_eq!(&x[1]•as•*const•u32•as•usize,•&y[0]•as•*const•u32•as•usize)     MatchExpressionCase
        [_,•ref•y•@•..]                                                                               ArrayPattern
         _                                                                                            WildcardPattern
            ref•y•@•..                                                                                PatternVariableDeclaration
                    ..                                                                                RestPattern
                           assert_eq!(&x[1]•as•*const•u32•as•usize,•&y[0]•as•*const•u32•as•usize)     MacroInvocation
                                      &                                                               PunctuationToken
                                        [1]                                                           DelimGroup
                                         1                                                            Literal
                                               *                                                      PunctuationToken
                                                                  ,                                   PunctuationToken
                                                                    &                                 PunctuationToken
                                                                      [0]                             DelimGroup
                                                                       0                              Literal
                                                                             *                        PunctuationToken                    */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
	unsafe { assert_eq!(ABC as usize, 0); }                                                                                               /*
    unsafe•{•assert_eq!(ABC•as•usize,•0);•}    ExpressionStatement, BlockExpression
             assert_eq!(ABC•as•usize,•0);      ExpressionStatement
             assert_eq!(ABC•as•usize,•0)       MacroInvocation
                                    ,          PunctuationToken
                                      0        Literal                                                                                    */
	&mut (|| Some(0 as *const ())) as &mut dyn FnMut() -> Option<*const ()>;                                                              /*
    &mut•(||•Some(0•as•*const•()))•as•&mut•dyn•FnMut()•->•Option<*const•()>;    ExpressionStatement
    &mut•(||•Some(0•as•*const•()))•as•&mut•dyn•FnMut()•->•Option<*const•()>     ExpressionAsTypeCast
    &mut•(||•Some(0•as•*const•()))                                              ReferenceExpression
          ||•Some(0•as•*const•())                                               ClosureFunctionExpression
             Some(0•as•*const•())                                               CallExpression
                  0•as•*const•()                                                ExpressionAsTypeCast
                  0                                                             Literal
                       *const•()                                                TypeDereferenceConst
                              ()                                                TypeTuple
                                      &mut•dyn•FnMut()•->•Option<*const•()>     TypeReference
                                           dyn•FnMut()•->•Option<*const•()>     TypeDynBounds
                                               FnMut()•->•Option<*const•()>     TypeTraitBound, TypeFunction
                                                          Option<*const•()>     TypeCall
                                                                 *const•()      TypeDereferenceConst
                                                                        ()      TypeTuple                                                 */
	unsafe {                                                                                                                              /*
    unsafe•{↲    <ExpressionStatement>, <BlockExpression>                                                                                 */
		NUM = 6 * 7 + 1 + (1u8 == 1u8) as u8; // 44
                                                                                                                                          /*
        NUM•=•6•*•7•+•1•+•(1u8•==•1u8)•as•u8;          ExpressionStatement
        NUM•=•6•*•7•+•1•+•(1u8•==•1u8)•as•u8           ReassignmentExpression
              6•*•7•+•1•+•(1u8•==•1u8)•as•u8           OperationExpression
              6•*•7•+•1                                OperationExpression
              6•*•7                                    OperationExpression
              6                                        Literal
                  7                                    Literal
                      1                                Literal
                          (1u8•==•1u8)•as•u8           ExpressionAsTypeCast
                           1u8•==•1u8                  ComparisonExpression
                           1u8                         Literal
                            u8                         Identifier
                                  1u8                  Literal
                                   u8                  Identifier
                                              //•44    Comment                                                                            */
		assert_eq!(*NUM_REF as i32, 44);                                                                                                  /*
        assert_eq!(*NUM_REF•as•i32,•44);    ExpressionStatement
        assert_eq!(*NUM_REF•as•i32,•44)     MacroInvocation
                   *                        PunctuationToken
                                  ,         PunctuationToken
                                    44      Literal                                                                                       */
	}                                                                                                                                     /*
   ╚}    </ExpressionStatement>, </BlockExpression>                                                                                       */
	unsafe { puts(*argv as *const i8); }                                                                                                  /*
    unsafe•{•puts(*argv•as•*const•i8);•}    ExpressionStatement, BlockExpression
             puts(*argv•as•*const•i8);      ExpressionStatement
             puts(*argv•as•*const•i8)       CallExpression
                  *argv•as•*const•i8        ExpressionAsTypeCast
                  *argv                     DereferenceExpression
                           *const•i8        TypeDereferenceConst                                                                          */
	unsafe { puts(*((argv as usize + intrinsics::size_of::<*const u8>()) as *const *const i8)); }                                         /*
    unsafe•{•puts(*((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));•}    ExpressionStatement, BlockExpression
             puts(*((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));      ExpressionStatement
             puts(*((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8))       CallExpression
                  *((argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8)        DereferenceExpression
                    (argv•as•usize•+•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8         ExpressionAsTypeCast
                     argv•as•usize•+•intrinsics::size_of::<*const•u8>()                              OperationExpression
                     argv•as•usize                                                                   ExpressionAsTypeCast
                                     intrinsics::size_of::<*const•u8>()                              CallExpression
                                     intrinsics::size_of                                             ExpressionPath
                                                           *const•u8                                 TypeDereferenceConst
                                                                            *const•*const•i8         TypeDereferenceConst
                                                                                   *const•i8         TypeDereferenceConst                 */
	unsafe { puts(*((argv as usize + 2 * intrinsics::size_of::<*const u8>()) as *const *const i8)); }                                     /*
    unsafe•{•puts(*((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));•}    ExpressionStatement, BlockExpression
             puts(*((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8));      ExpressionStatement
             puts(*((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8))       CallExpression
                  *((argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8)        DereferenceExpression
                    (argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>())•as•*const•*const•i8         ExpressionAsTypeCast
                     argv•as•usize•+•2•*•intrinsics::size_of::<*const•u8>()                              OperationExpression
                     argv•as•usize                                                                       ExpressionAsTypeCast
                                     2•*•intrinsics::size_of::<*const•u8>()                              OperationExpression
                                     2                                                                   Literal
                                         intrinsics::size_of::<*const•u8>()                              CallExpression
                                         intrinsics::size_of                                             ExpressionPath
                                                               *const•u8                                 TypeDereferenceConst
                                                                                *const•*const•i8         TypeDereferenceConst
                                                                                       *const•i8         TypeDereferenceConst             */
	intrinsics::write_bytes(&mut uninit.value.value as *mut T, 0, 1);                                                                     /*
    intrinsics::write_bytes(&mut•uninit.value.value•as•*mut•T,•0,•1);    ExpressionStatement
    intrinsics::write_bytes(&mut•uninit.value.value•as•*mut•T,•0,•1)     CallExpression
    intrinsics::write_bytes                                              ExpressionPath
                            &mut•uninit.value.value•as•*mut•T            ExpressionAsTypeCast
                            &mut•uninit.value.value                      ReferenceExpression
                                 uninit.value.value                      MemberExpression
                                 uninit.value                            MemberExpression
                                                       *mut•T            TypeDereferenceMut
                                                               0         Literal
                                                                  1      Literal                                                          */
	assert_eq!(slice_ptr as usize % 4, 0);                                                                                                /*
    assert_eq!(slice_ptr•as•usize•%•4,•0);    ExpressionStatement
    assert_eq!(slice_ptr•as•usize•%•4,•0)     MacroInvocation
                                  %           PunctuationToken
                                    4         Literal
                                     ,        PunctuationToken
                                       0      Literal                                                                                     */
	printf("Hello %s\n\0" as *const str as *const i8, "printf\0" as *const str as *const i8);                                             /*
    printf("Hello•%s\n\0"•as•*const•str•as•*const•i8,•"printf\0"•as•*const•str•as•*const•i8);    ExpressionStatement
    printf("Hello•%s\n\0"•as•*const•str•as•*const•i8,•"printf\0"•as•*const•str•as•*const•i8)     CallExpression
           "Hello•%s\n\0"•as•*const•str•as•*const•i8                                             ExpressionAsTypeCast
           "Hello•%s\n\0"•as•*const•str                                                          ExpressionAsTypeCast
           "Hello•%s\n\0"                                                                        Literal
                             *const•str                                                          TypeDereferenceConst
                                           *const•i8                                             TypeDereferenceConst
                                                      "printf\0"•as•*const•str•as•*const•i8      ExpressionAsTypeCast
                                                      "printf\0"•as•*const•str                   ExpressionAsTypeCast
                                                      "printf\0"                                 Literal
                                                                    *const•str                   TypeDereferenceConst
                                                                                  *const•i8      TypeDereferenceConst                     */
	let hello: &[u8] = b"Hello\0" as &[u8; 6];                                                                                            /*
    let•hello:•&[u8]•=•b"Hello\0"•as•&[u8;•6];    LetVariableDeclaration
               &[u8]                              TypeReference
                [u8]                              TypeSlice
                       b"Hello\0"•as•&[u8;•6]     ExpressionAsTypeCast
                       b"Hello\0"                 Literal
                                     &[u8;•6]     TypeReference
                                      [u8;•6]     TypeSizedArray
                                           6      Literal                                                                                 */
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
                   &str                       TypeReference
                           box•"World!\0"     BoxExpression
                               "World!\0"     Literal                                                                                     */
	puts(*world as *const str as *const i8);                                                                                              /*
    puts(*world•as•*const•str•as•*const•i8);    ExpressionStatement
    puts(*world•as•*const•str•as•*const•i8)     CallExpression
         *world•as•*const•str•as•*const•i8      ExpressionAsTypeCast
         *world•as•*const•str                   ExpressionAsTypeCast
         *world                                 DereferenceExpression
                   *const•str                   TypeDereferenceConst
                                 *const•i8      TypeDereferenceConst                                                                      */
	assert_eq!( a.f (), "The method f");                                                                                                  /*
    assert_eq!(•a.f•(),•"The•method•f");    ExpressionStatement
    assert_eq!(•a.f•(),•"The•method•f")     MacroInvocation
                 .                          PunctuationToken
                    ()                      DelimGroup
                      ,                     PunctuationToken
                        "The•method•f"      Literal                                                                                       */
	assert_eq!((a.f)(), "The field f");                                                                                                   /*
    assert_eq!((a.f)(),•"The•field•f");    ExpressionStatement
    assert_eq!((a.f)(),•"The•field•f")     MacroInvocation
               (a.f)                       DelimGroup
                 .                         PunctuationToken
                    ()                     DelimGroup
                      ,                    PunctuationToken
                        "The•field•f"      Literal                                                                                        */
	assert_eq!(((|()| 42u8) as fn(()) -> u8)(()), 42);                                                                                    /*
    assert_eq!(((|()|•42u8)•as•fn(())•->•u8)(()),•42);    ExpressionStatement
    assert_eq!(((|()|•42u8)•as•fn(())•->•u8)(()),•42)     MacroInvocation
               ((|()|•42u8)•as•fn(())•->•u8)              DelimGroup
                (|()|•42u8)                               DelimGroup
                 |                                        PunctuationToken
                  ()                                      DelimGroup
                    |                                     PunctuationToken
                      42u8                                Literal
                        u8                                Identifier
                                 (())                     DelimGroup
                                  ()                      DelimGroup
                                      ->                  PunctuationToken
                                            (())          DelimGroup
                                             ()           DelimGroup
                                                ,         PunctuationToken
                                                  42      Literal                                                                         */
	assert_eq!(intrinsics::bitreverse(0b10101000u8), 0b00010101u8);                                                                       /*
    assert_eq!(intrinsics::bitreverse(0b10101000u8),•0b00010101u8);    ExpressionStatement
    assert_eq!(intrinsics::bitreverse(0b10101000u8),•0b00010101u8)     MacroInvocation
                         ::                                            PunctuationToken
                                     (0b10101000u8)                    DelimGroup
                                      0b10101000u8                     Literal
                                                u8                     Identifier
                                                   ,                   PunctuationToken
                                                     0b00010101u8      Literal
                                                               u8      Identifier                                                         */
	assert_eq!(intrinsics::bswap(0xabu8), 0xabu8);                                                                                        /*
    assert_eq!(intrinsics::bswap(0xabu8),•0xabu8);    ExpressionStatement
    assert_eq!(intrinsics::bswap(0xabu8),•0xabu8)     MacroInvocation
                         ::                           PunctuationToken
                                (0xabu8)              DelimGroup
                                 0xabu8               Literal
                                     u8               Identifier
                                        ,             PunctuationToken
                                          0xabu8      Literal
                                              u8      Identifier                                                                          */
	assert_eq!(intrinsics::bswap(0xddccu16), 0xccddu16);                                                                                  /*
    assert_eq!(intrinsics::bswap(0xddccu16),•0xccddu16);    ExpressionStatement
    assert_eq!(intrinsics::bswap(0xddccu16),•0xccddu16)     MacroInvocation
                         ::                                 PunctuationToken
                                (0xddccu16)                 DelimGroup
                                 0xddccu16                  Literal
                                       u16                  Identifier
                                           ,                PunctuationToken
                                             0xccddu16      Literal
                                                   u16      Identifier                                                                    */
	assert_eq!(intrinsics::bswap(0xffee_ddccu32), 0xccdd_eeffu32);                                                                        /*
    assert_eq!(intrinsics::bswap(0xffee_ddccu32),•0xccdd_eeffu32);    ExpressionStatement
    assert_eq!(intrinsics::bswap(0xffee_ddccu32),•0xccdd_eeffu32)     MacroInvocation
                         ::                                           PunctuationToken
                                (0xffee_ddccu32)                      DelimGroup
                                 0xffee_ddccu32                       Literal
                                            u32                       Identifier
                                                ,                     PunctuationToken
                                                  0xccdd_eeffu32      Literal
                                                             u32      Identifier                                                          */
	assert_eq!(intrinsics::bswap(0x1234_5678_ffee_ddccu64), 0xccdd_eeff_7856_3412u64);                                                    /*
    assert_eq!(intrinsics::bswap(0x1234_5678_ffee_ddccu64),•0xccdd_eeff_7856_3412u64);    ExpressionStatement
    assert_eq!(intrinsics::bswap(0x1234_5678_ffee_ddccu64),•0xccdd_eeff_7856_3412u64)     MacroInvocation
                         ::                                                               PunctuationToken
                                (0x1234_5678_ffee_ddccu64)                                DelimGroup
                                 0x1234_5678_ffee_ddccu64                                 Literal
                                                      u64                                 Identifier
                                                          ,                               PunctuationToken
                                                            0xccdd_eeff_7856_3412u64      Literal
                                                                                 u64      Identifier                                      */
    let mut passes: Vec<_> = passes.iter().map(|p| (p)()).collect();                                                                      /*
    let•mut•passes:•Vec<_>•=•passes.iter().map(|p|•(p)()).collect();    LetVariableDeclaration
        mut•passes                                                      PatternVariableDeclaration
                    Vec<_>                                              TypeCall
                        _                                               TypeInferred
                             passes.iter().map(|p|•(p)()).collect()     CallExpression
                             passes.iter().map(|p|•(p)())               CallExpression
                             passes.iter()                              CallExpression
                                               |p|•(p)()                ClosureFunctionExpression
                                                p                       ClosureFunctionParameterDeclaration
                                                   (p)()                CallExpression                                                    */
	(*DEFAULT_HOOK)(info);                                                                                                                /*
    (*DEFAULT_HOOK)(info);    ExpressionStatement
    (*DEFAULT_HOOK)(info)     CallExpression
     *DEFAULT_HOOK            DereferenceExpression                                                                                       */
	(group.apply)(&mut opts);                                                                                                             /*
    (group.apply)(&mut•opts);    ExpressionStatement
    (group.apply)(&mut•opts)     CallExpression
     group.apply                 MemberExpression
                  &mut•opts      ReferenceExpression                                                                                      */
	Some((size, 1u128 << (size.bits() as u128 - 1)));                                                                                     /*
    Some((size,•1u128•<<•(size.bits()•as•u128•-•1)));    ExpressionStatement
    Some((size,•1u128•<<•(size.bits()•as•u128•-•1)))     CallExpression
         (size,•1u128•<<•(size.bits()•as•u128•-•1))      TupleLiteral
                1u128•<<•(size.bits()•as•u128•-•1)       OperationExpression
                1u128                                    Literal
                 u128                                    Identifier
                          size.bits()•as•u128•-•1        OperationExpression
                          size.bits()•as•u128            ExpressionAsTypeCast
                          size.bits()                    CallExpression
                                                1        Literal                                                                          */
	(lo == other_hi || hi == other_lo) && !self.is_singleton() && !other.is_singleton();                                                  /*
    (lo•==•other_hi•||•hi•==•other_lo)•&&•!self.is_singleton()•&&•!other.is_singleton();    ExpressionStatement
    (lo•==•other_hi•||•hi•==•other_lo)•&&•!self.is_singleton()•&&•!other.is_singleton()     AndExpression
    (lo•==•other_hi•||•hi•==•other_lo)•&&•!self.is_singleton()                              AndExpression
     lo•==•other_hi•||•hi•==•other_lo                                                       OrExpression
     lo•==•other_hi                                                                         ComparisonExpression
                       hi•==•other_lo                                                       ComparisonExpression
                                          !self.is_singleton()                              NotExpression
                                           self.is_singleton()                              CallExpression
                                                                  !other.is_singleton()     NotExpression
                                                                   other.is_singleton()     CallExpression                                */

	(|A { x: mut t }: A| { t = t+1; t })(A { x: 34 });                                                                                    /*
    (|A•{•x:•mut•t•}:•A|•{•t•=•t+1;•t•})(A•{•x:•34•});    ExpressionStatement
    (|A•{•x:•mut•t•}:•A|•{•t•=•t+1;•t•})(A•{•x:•34•})     CallExpression
     |A•{•x:•mut•t•}:•A|•{•t•=•t+1;•t•}                   ClosureFunctionExpression
      A•{•x:•mut•t•}:•A                                   ClosureFunctionParameterDeclaration
      A•{•x:•mut•t•}                                      StructPattern
          x:•mut•t                                        StructPatternPropertyDestructured
             mut•t                                        PatternVariableDeclaration
                         {•t•=•t+1;•t•}                   BlockExpression
                           t•=•t+1;                       ExpressionStatement
                           t•=•t+1                        ReassignmentExpression
                               t+1                        OperationExpression
                                 1                        Literal
                                    t                     ExpressionStatement
                                         A•{•x:•34•}      StructLiteral
                                             x:•34        StructLiteralProperty
                                                34        Literal                                                                         */
	(async || 2333)().await;                                                                                                              /*
    (async•||•2333)().await;    ExpressionStatement
    (async•||•2333)().await     AwaitExpression
    (async•||•2333)()           CallExpression
     async•||•2333              ClosureFunctionExpression
              2333              Literal                                                                                                   */
    (async move || -> u8 { 42 })();                                                                                                       /*
    (async•move•||•->•u8•{•42•})();    ExpressionStatement
    (async•move•||•->•u8•{•42•})()     CallExpression
     async•move•||•->•u8•{•42•}        ClosureFunctionExpression
                         {•42•}        BlockExpression
                           42          ExpressionStatement, Literal                                                                       */
	(S.g(1,2))(true);                                                                                                                     /*
    (S.g(1,2))(true);    ExpressionStatement
    (S.g(1,2))(true)     CallExpression
     S.g(1,2)            CallExpression
         1               Literal
           2             Literal
               true      Literal                                                                                                          */
	&Ast::Num((*f)(x));                                                                                                                   /*
    &Ast::Num((*f)(x));    ExpressionStatement
    &Ast::Num((*f)(x))     ReferenceExpression
     Ast::Num((*f)(x))     CallExpression
     Ast::Num              ExpressionPath
              (*f)(x)      CallExpression
               *f          DereferenceExpression                                                                                          */
	f(&mut "Hello".to_owned());                                                                                                           /*
    f(&mut•"Hello".to_owned());    ExpressionStatement
    f(&mut•"Hello".to_owned())     CallExpression
      &mut•"Hello".to_owned()      ReferenceExpression
           "Hello".to_owned()      CallExpression
           "Hello"                 Literal                                                                                                */
	Box::new(move |x| f()(x));                                                                                                            /*
    Box::new(move•|x|•f()(x));    ExpressionStatement
    Box::new(move•|x|•f()(x))     CallExpression
    Box::new                      ExpressionPath
             move•|x|•f()(x)      ClosureFunctionExpression
                   x              ClosureFunctionParameterDeclaration
                      f()(x)      CallExpression
                      f()         CallExpression                                                                                          */
	let a = Some(1u8).map(|a| foo(a));                                                                                                    /*
    let•a•=•Some(1u8).map(|a|•foo(a));    LetVariableDeclaration
            Some(1u8).map(|a|•foo(a))     CallExpression
            Some(1u8)                     CallExpression
                 1u8                      Literal
                  u8                      Identifier
                          |a|•foo(a)      ClosureFunctionExpression
                           a              ClosureFunctionParameterDeclaration
                              foo(a)      CallExpression                                                                                  */
    let c = Some(1u8).map(|a| {1+2; foo}(a));                                                                                             /*
    let•c•=•Some(1u8).map(|a|•{1+2;•foo}(a));    LetVariableDeclaration
            Some(1u8).map(|a|•{1+2;•foo}(a))     CallExpression
            Some(1u8)                            CallExpression
                 1u8                             Literal
                  u8                             Identifier
                          |a|•{1+2;•foo}(a)      ClosureFunctionExpression
                           a                     ClosureFunctionParameterDeclaration
                              {1+2;•foo}(a)      CallExpression
                              {1+2;•foo}         BlockExpression
                               1+2;              ExpressionStatement
                               1+2               OperationExpression
                               1                 Literal
                                 2               Literal
                                    foo          ExpressionStatement                                                                      */
    true.then(|| mac!());                                                                                                                 /*
    true.then(||•mac!());    ExpressionStatement
    true.then(||•mac!())     CallExpression
    true                     Literal
              ||•mac!()      ClosureFunctionExpression
                 mac!()      MacroInvocation                                                                                              */
    Some(1).map(closure_mac!());                                                                                                          /*
    Some(1).map(closure_mac!());    ExpressionStatement
    Some(1).map(closure_mac!())     CallExpression
    Some(1)                         CallExpression
         1                          Literal
                closure_mac!()      MacroInvocation                                                                                       */
    let _: Option<Vec<u8>> = true.then(|| vec![]);                                                                                        /*
    let•_:•Option<Vec<u8>>•=•true.then(||•vec![]);    LetVariableDeclaration
        _                                             WildcardPattern
           Option<Vec<u8>>                            TypeCall
                  Vec<u8>                             TypeCall
                             true.then(||•vec![])     CallExpression
                             true                     Literal
                                       ||•vec![]      ClosureFunctionExpression
                                          vec![]      MacroInvocation                                                                     */
    let d = Some(1u8).map(|a| foo((|b| foo2(b))(a)));                                                                                     /*
    let•d•=•Some(1u8).map(|a|•foo((|b|•foo2(b))(a)));    LetVariableDeclaration
            Some(1u8).map(|a|•foo((|b|•foo2(b))(a)))     CallExpression
            Some(1u8)                                    CallExpression
                 1u8                                     Literal
                  u8                                     Identifier
                          |a|•foo((|b|•foo2(b))(a))      ClosureFunctionExpression
                           a                             ClosureFunctionParameterDeclaration
                              foo((|b|•foo2(b))(a))      CallExpression
                                  (|b|•foo2(b))(a)       CallExpression
                                   |b|•foo2(b)           ClosureFunctionExpression
                                    b                    ClosureFunctionParameterDeclaration
                                       foo2(b)           CallExpression                                                                   */
    all(&[1, 2, 3], &&2, |x, y| below(x, y));                                                                                             /*
    all(&[1,•2,•3],•&&2,•|x,•y|•below(x,•y));    ExpressionStatement
    all(&[1,•2,•3],•&&2,•|x,•y|•below(x,•y))     CallExpression
        &[1,•2,•3]                               ReferenceExpression
         [1,•2,•3]                               ArrayLiteral
          1                                      Literal
             2                                   Literal
                3                                Literal
                    &&2                          ReferenceExpression
                     &2                          ReferenceExpression
                      2                          Literal
                         |x,•y|•below(x,•y)      ClosureFunctionExpression
                          x                      ClosureFunctionParameterDeclaration
                             y                   ClosureFunctionParameterDeclaration
                                below(x,•y)      CallExpression                                                                           */
	let a: Option<Box<dyn (::std::ops::Deref<Target = [i32]>)>> =                                                                         /*
    let•a:•Option<Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>>•=↲    <LetVariableDeclaration>
           Option<Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>>       TypeCall
                  Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>        TypeCall
                      dyn•(::std::ops::Deref<Target•=•[i32]>)         TypeDynBounds
                           ::std::ops::Deref<Target•=•[i32]>          TypeTraitBound, TypeCall
                           ::std::ops::Deref                          TypePath
                           ::std::ops                                 TypePath
                           ::std                                      TypePath
                                             Target•=•[i32]           TypeCallNamedArgument
                                                      [i32]           TypeSlice                                                           */
        Some(vec![1i32, 2]).map(|v| -> Box<dyn (::std::ops::Deref<Target = [i32]>)> { Box::new(v) });                                     /*
••••••••Some(vec![1i32,•2]).map(|v|•->•Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>•{•Box::new(v)•});    </LetVariableDeclaration>
        Some(vec![1i32,•2]).map(|v|•->•Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>•{•Box::new(v)•})     CallExpression
        Some(vec![1i32,•2])                                                                              CallExpression
             vec![1i32,•2]                                                                               MacroInvocation
                  1i32                                                                                   Literal
                   i32                                                                                   Identifier
                      ,                                                                                  PunctuationToken
                        2                                                                                Literal
                                |v|•->•Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>•{•Box::new(v)•}      ClosureFunctionExpression
                                 v                                                                       ClosureFunctionParameterDeclaration
                                       Box<dyn•(::std::ops::Deref<Target•=•[i32]>)>                      TypeCall
                                           dyn•(::std::ops::Deref<Target•=•[i32]>)                       TypeDynBounds
                                                ::std::ops::Deref<Target•=•[i32]>                        TypeTraitBound, TypeCall
                                                ::std::ops::Deref                                        TypePath
                                                ::std::ops                                               TypePath
                                                ::std                                                    TypePath
                                                                  Target•=•[i32]                         TypeCallNamedArgument
                                                                           [i32]                         TypeSlice
                                                                                    {•Box::new(v)•}      BlockExpression
                                                                                      Box::new(v)        ExpressionStatement, CallExpression
                                                                                      Box::new           ExpressionPath                   */
	#[allow(clippy::needless_return)]                                                                                                     /*
    #[allow(clippy::needless_return)]↲    <ExpressionStatement>
    #[allow(clippy::needless_return)]     Attribute
           (clippy::needless_return)      DelimGroup
                  ::                      PunctuationToken                                                                                */
    (|| return 2)();                                                                                                                      /*
••••(||•return•2)();    </ExpressionStatement>
    (||•return•2)()     CallExpression
     ||•return•2        ClosureFunctionExpression
        return•2        ReturnExpression
               2        Literal                                                                                                           */
    (|| -> Option<i32> { None? })();                                                                                                      /*
    (||•->•Option<i32>•{•None?•})();    ExpressionStatement
    (||•->•Option<i32>•{•None?•})()     CallExpression
     ||•->•Option<i32>•{•None?•}        ClosureFunctionExpression
           Option<i32>                  TypeCall
                       {•None?•}        BlockExpression
                         None?          ExpressionStatement, UnwrapExpression                                                             */
    #[allow(clippy::try_err)]                                                                                                             /*
    #[allow(clippy::try_err)]↲    <ExpressionStatement>
    #[allow(clippy::try_err)]     Attribute
           (clippy::try_err)      DelimGroup
                  ::              PunctuationToken                                                                                        */
    (|| -> Result<i32, i32> { Err(2)? })();                                                                                               /*
••••(||•->•Result<i32,•i32>•{•Err(2)?•})();    </ExpressionStatement>
    (||•->•Result<i32,•i32>•{•Err(2)?•})()     CallExpression
     ||•->•Result<i32,•i32>•{•Err(2)?•}        ClosureFunctionExpression
           Result<i32,•i32>                    TypeCall
                            {•Err(2)?•}        BlockExpression
                              Err(2)?          ExpressionStatement, UnwrapExpression
                              Err(2)           CallExpression
                                  2            Literal                                                                                    */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

static mut NUM: u8 = 6 * 7;                                                                                                               /*
static•mut•NUM:•u8•=•6•*•7;    StaticVariableDeclaration
       mut•NUM                 PatternVariableDeclaration
                     6•*•7     OperationExpression
                     6         Literal
                         7     Literal                                                                                                    */
static NUM_REF: &'static u8 = unsafe { &NUM };                                                                                            /*
static•NUM_REF:•&'static•u8•=•unsafe•{•&NUM•};    StaticVariableDeclaration
                &'static•u8                       TypeReference
                 'static                          LtStatic
                              unsafe•{•&NUM•}     BlockExpression
                                       &NUM       ExpressionStatement, ReferenceExpression                                                */
impl<T: ?Sized, U: ?Sized> CoerceUnsized<Unique<U>> for Unique<T> where T: Unsize<U> {}                                                   /*
impl<T:•?Sized,•U:•?Sized>•CoerceUnsized<Unique<U>>•for•Unique<T>•where•T:•Unsize<U>•{}    ImplDeclaration
     T:•?Sized                                                                             GenericTypeParameterDeclaration
        ?Sized                                                                             TypeTraitBound
                U:•?Sized                                                                  GenericTypeParameterDeclaration
                   ?Sized                                                                  TypeTraitBound
                           CoerceUnsized<Unique<U>>                                        TypeCall
                                         Unique<U>                                         TypeCall
                                                        Unique<T>                          TypeCall
                                                                        T:•Unsize<U>       WhereTypeBoundDeclaration
                                                                           Unsize<U>       TypeTraitBound, TypeCall                       */

fn cvgsk_nichqsd_bhvior () {                                                                                                              /*
fn•cvgsk_nichqsd_bhvior•()•{↲    <FunctionDeclaration>                                                                                    */
    if let E1::V2 { .. } = (E1::V1 { f: true }) {                                                                                         /*
    if•let•E1::V2•{•..•}•=•(E1::V1•{•f:•true•})•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•E1::V2•{•..•}•=•(E1::V1•{•f:•true•})       LetScrutinee
           E1::V2•{•..•}                              StructPattern
           E1::V2                                     ExpressionPath
                    ..                                RestPattern
                            E1::V1•{•f:•true•}        StructLiteral
                            E1::V1                    ExpressionPath
                                     f:•true          StructLiteralProperty
                                        true          Literal                                                                             */
        intarvics::avort();                                                                                                               /*
        intarvics::avort();    ExpressionStatement
        intarvics::avort()     CallExpression
        intarvics::avort       ExpressionPath                                                                                             */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */

    if let E2::V1 { .. } = E2::V3::<Inwxvlible> {                                                                                         /*
    if•let•E2::V1•{•..•}•=•E2::V3::<Inwxvlible>•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•E2::V1•{•..•}•=•E2::V3::<Inwxvlible>       LetScrutinee
           E2::V1•{•..•}                              StructPattern
           E2::V1                                     ExpressionPath
                    ..                                RestPattern
                           E2::V3::<Inwxvlible>       ExpressionTypeCast
                           E2::V3                     ExpressionPath                                                                      */
        inzadqsics::abort();                                                                                                              /*
        inzadqsics::abort();    ExpressionStatement
        inzadqsics::abort()     CallExpression
        inzadqsics::abort       ExpressionPath                                                                                            */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

impl<'a, 'b> FnOnce<(&'a &'b [u16],)> for IsNotEmpty {                                                                                    /*
impl<'a,•'b>•FnOnce<(&'a•&'b•[u16],)>•for•IsNotEmpty•{↲    <ImplDeclaration>
     'a                                                    GenericLtParameterDeclaration, LtIdentifier
         'b                                                GenericLtParameterDeclaration, LtIdentifier
             FnOnce<(&'a•&'b•[u16],)>                      TypeCall
                    (&'a•&'b•[u16],)                       TypeTuple
                     &'a•&'b•[u16]                         TypeReference
                      'a                                   LtIdentifier
                         &'b•[u16]                         TypeReference
                          'b                               LtIdentifier
                             [u16]                         TypeSlice                                                                      */
    extern "rust-call" fn call_once(mut self, arg: (&'a &'b [u16],)) -> (u8, u8) {                                                        /*
    extern•"rust-call"•fn•call_once(mut•self,•arg:•(&'a•&'b•[u16],))•->•(u8,•u8)•{↲    <FunctionDeclaration>
    extern•"rust-call"                                                                 ExternSpecifier
           "rust-call"                                                                 Literal
                                    mut•self                                           FunctionSelfParameterDeclaration
                                              arg:•(&'a•&'b•[u16],)                    FunctionParameterDeclaration
                                                   (&'a•&'b•[u16],)                    TypeTuple
                                                    &'a•&'b•[u16]                      TypeReference
                                                     'a                                LtIdentifier
                                                        &'b•[u16]                      TypeReference
                                                         'b                            LtIdentifier
                                                            [u16]                      TypeSlice
                                                                        (u8,•u8)       TypeTuple                                          */
        self.call_mut(arg)                                                                                                                /*
        self.call_mut(arg)    ExpressionStatement, CallExpression                                                                         */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration>                                                                                                           */
    extern "rust-call" fn call_once123(mut self, arg: (&'a &'b [u16],)) -> (u8, u8) {                                                     /*
    extern•"rust-call"•fn•call_once123(mut•self,•arg:•(&'a•&'b•[u16],))•->•(u8,•u8)•{↲    <FunctionDeclaration>
    extern•"rust-call"                                                                    ExternSpecifier
           "rust-call"                                                                    Literal
                                       mut•self                                           FunctionSelfParameterDeclaration
                                                 arg:•(&'a•&'b•[u16],)                    FunctionParameterDeclaration
                                                      (&'a•&'b•[u16],)                    TypeTuple
                                                       &'a•&'b•[u16]                      TypeReference
                                                        'a                                LtIdentifier
                                                           &'b•[u16]                      TypeReference
                                                            'b                            LtIdentifier
                                                               [u16]                      TypeSlice
                                                                           (u8,•u8)       TypeTuple                                       */
        self.call_mut(arg)                                                                                                                /*
        self.call_mut(arg)    ExpressionStatement, CallExpression                                                                         */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration>                                                                                                           */
    extern "rust-call" fn call_mut(&mut self, _arg: (&'a &'b [u16],)) -> (u8, u8) {                                                       /*
    extern•"rust-call"•fn•call_mut(&mut•self,•_arg:•(&'a•&'b•[u16],))•->•(u8,•u8)•{↲    <FunctionDeclaration>
    extern•"rust-call"                                                                  ExternSpecifier
           "rust-call"                                                                  Literal
                                   &mut•self                                            FunctionSelfParameterDeclaration
                                              _arg:•(&'a•&'b•[u16],)                    FunctionParameterDeclaration
                                                    (&'a•&'b•[u16],)                    TypeTuple
                                                     &'a•&'b•[u16]                      TypeReference
                                                      'a                                LtIdentifier
                                                         &'b•[u16]                      TypeReference
                                                          'b                            LtIdentifier
                                                             [u16]                      TypeSlice
                                                                         (u8,•u8)       TypeTuple                                         */
        (0, 42)                                                                                                                           /*
        (0,•42)    ExpressionStatement, TupleLiteral
         0         Literal
            42     Literal                                                                                                                */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration>                                                                                                           */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */

pub fn call_is_not_empty() {                                                                                                              /*
pub•fn•call_is_not_empty()•{↲    <FunctionDeclaration>
pub                              PubSpecifier                                                                                             */
    IsNotEmpty.call_once((&(&[0u16] as &[_]),));                                                                                          /*
    IsNotEmpty.call_once((&(&[0u16]•as•&[_]),));    ExpressionStatement
    IsNotEmpty.call_once((&(&[0u16]•as•&[_]),))     CallExpression
                         (&(&[0u16]•as•&[_]),)      TupleLiteral
                          &(&[0u16]•as•&[_])        ReferenceExpression
                            &[0u16]•as•&[_]         ExpressionAsTypeCast
                            &[0u16]                 ReferenceExpression
                             [0u16]                 ArrayLiteral
                              0u16                  Literal
                               u16                  Identifier
                                       &[_]         TypeReference
                                        [_]         TypeSlice
                                         _          TypeInferred                                                                          */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

EnumTypeFoldableImpl! {                                                                                                                   /*
EnumTypeFoldableImpl!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                      */
    impl<'tcx, T> TypeFoldable<'tcx> for Option<T> {                                                                                      /*
        <                                                PunctuationToken
         'tcx                                            LtIdentifier
             ,                                           PunctuationToken
                >                                        PunctuationToken
                              <                          PunctuationToken
                               'tcx                      LtIdentifier
                                   >                     PunctuationToken
                                               <         PunctuationToken
                                                 >       PunctuationToken
                                                   {↲    <DelimGroup>                                                                     */
        (Some)(a),                                                                                                                        /*
        (Some)        DelimGroup
              (a)     DelimGroup
                 ,    PunctuationToken                                                                                                    */
        (None),                                                                                                                           /*
        (None)     DelimGroup
              ,    PunctuationToken                                                                                                       */
    } where T: TypeFoldable<'tcx>                                                                                                         /*
••••}                                </DelimGroup>
             :                       PunctuationToken
                           <         PunctuationToken
                            'tcx     LtIdentifier
                                >    PunctuationToken                                                                                     */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

fn x() {                                                                                                                                  /*
fn•x()•{↲    <FunctionDeclaration>                                                                                                        */
                      a   .b   .c    ;                                                                                                    /*
                      a•••.b•••.c••••;    ExpressionStatement
                      a•••.b•••.c         MemberExpression
                      a•••.b              MemberExpression                                                                                */
                    ( a ) .b   .c    ;                                                                                                    /*
                    (•a•)•.b•••.c••••;    ExpressionStatement
                    (•a•)•.b•••.c         MemberExpression
                    (•a•)•.b              MemberExpression                                                                                */
                  ( ( a ) .b ) .c    ;                                                                                                    /*
                  (•(•a•)•.b•)•.c••••;    ExpressionStatement
                  (•(•a•)•.b•)•.c         MemberExpression
                    (•a•)•.b              MemberExpression                                                                                */
                ( ( ( a ) .b ) .c )  ;                                                                                                    /*
                (•(•(•a•)•.b•)•.c•)••;    ExpressionStatement
                  (•(•a•)•.b•)•.c         MemberExpression
                    (•a•)•.b              MemberExpression                                                                                */
                (   ( a ) .b   .c )  ;                                                                                                    /*
                (•••(•a•)•.b•••.c•)••;    ExpressionStatement
                    (•a•)•.b•••.c         MemberExpression
                    (•a•)•.b              MemberExpression                                                                                */
                  (   a   .b ) .c    ;                                                                                                    /*
                  (•••a•••.b•)•.c••••;    ExpressionStatement
                  (•••a•••.b•)•.c         MemberExpression
                      a•••.b              MemberExpression                                                                                */
                ( (   a   .b ) .c )  ;                                                                                                    /*
                (•(•••a•••.b•)•.c•)••;    ExpressionStatement
                  (•••a•••.b•)•.c         MemberExpression
                      a•••.b              MemberExpression                                                                                */
                (     a   .b   .c )  ;                                                                                                    /*
                (•••••a•••.b•••.c•)••;    ExpressionStatement
                      a•••.b•••.c         MemberExpression
                      a•••.b              MemberExpression                                                                                */

              (       a   .b   .c   );                                                                                                    /*
              (•••••••a•••.b•••.c•••);    ExpressionStatement
                      a•••.b•••.c         MemberExpression
                      a•••.b              MemberExpression                                                                                */
              (     ( a ) .b   .c   );                                                                                                    /*
              (•••••(•a•)•.b•••.c•••);    ExpressionStatement
                    (•a•)•.b•••.c         MemberExpression
                    (•a•)•.b              MemberExpression                                                                                */
              (   ( ( a ) .b ) .c   );                                                                                                    /*
              (•••(•(•a•)•.b•)•.c•••);    ExpressionStatement
                  (•(•a•)•.b•)•.c         MemberExpression
                    (•a•)•.b              MemberExpression                                                                                */
              ( ( ( ( a ) .b ) .c ) );                                                                                                    /*
              (•(•(•(•a•)•.b•)•.c•)•);    ExpressionStatement
                  (•(•a•)•.b•)•.c         MemberExpression
                    (•a•)•.b              MemberExpression                                                                                */
              ( (   ( a ) .b   .c ) );                                                                                                    /*
              (•(•••(•a•)•.b•••.c•)•);    ExpressionStatement
                    (•a•)•.b•••.c         MemberExpression
                    (•a•)•.b              MemberExpression                                                                                */
              (   (   a   .b ) .c   );                                                                                                    /*
              (•••(•••a•••.b•)•.c•••);    ExpressionStatement
                  (•••a•••.b•)•.c         MemberExpression
                      a•••.b              MemberExpression                                                                                */
              ( ( (   a   .b ) .c ) );                                                                                                    /*
              (•(•(•••a•••.b•)•.c•)•);    ExpressionStatement
                  (•••a•••.b•)•.c         MemberExpression
                      a•••.b              MemberExpression                                                                                */
              ( (     a   .b   .c ) );                                                                                                    /*
              (•(•••••a•••.b•••.c•)•);    ExpressionStatement
                      a•••.b•••.c         MemberExpression
                      a•••.b              MemberExpression                                                                                */

    foo(#[attr]       a   .b   .c   );                                                                                                    /*
    foo(#[attr]•••••••a•••.b•••.c•••);    ExpressionStatement
    foo(#[attr]•••••••a•••.b•••.c•••)     CallExpression
        #[attr]•••••••a•••.b•••.c         MemberExpression
        #[attr]                           Attribute
                      a•••.b              MemberExpression                                                                                */
    foo(#[attr]     ( a ) .b   .c   );                                                                                                    /*
    foo(#[attr]•••••(•a•)•.b•••.c•••);    ExpressionStatement
    foo(#[attr]•••••(•a•)•.b•••.c•••)     CallExpression
        #[attr]•••••(•a•)•.b•••.c         MemberExpression
        #[attr]                           Attribute
                    (•a•)•.b              MemberExpression                                                                                */
    foo(#[attr]   ( ( a ) .b ) .c   );                                                                                                    /*
    foo(#[attr]•••(•(•a•)•.b•)•.c•••);    ExpressionStatement
    foo(#[attr]•••(•(•a•)•.b•)•.c•••)     CallExpression
        #[attr]•••(•(•a•)•.b•)•.c         MemberExpression
        #[attr]                           Attribute
                    (•a•)•.b              MemberExpression                                                                                */
    foo(#[attr] ( ( ( a ) .b ) .c ) );                                                                                                    /*
    foo(#[attr]•(•(•(•a•)•.b•)•.c•)•);    ExpressionStatement
    foo(#[attr]•(•(•(•a•)•.b•)•.c•)•)     CallExpression
        #[attr]•(•(•(•a•)•.b•)•.c         MemberExpression
        #[attr]                           Attribute
                    (•a•)•.b              MemberExpression                                                                                */
    foo(#[attr] (   ( a ) .b   .c ) );                                                                                                    /*
    foo(#[attr]•(•••(•a•)•.b•••.c•)•);    ExpressionStatement
    foo(#[attr]•(•••(•a•)•.b•••.c•)•)     CallExpression
        #[attr]•(•••(•a•)•.b•••.c         MemberExpression
        #[attr]                           Attribute
                    (•a•)•.b              MemberExpression                                                                                */
    foo(#[attr]   (   a   .b ) .c   );                                                                                                    /*
    foo(#[attr]•••(•••a•••.b•)•.c•••);    ExpressionStatement
    foo(#[attr]•••(•••a•••.b•)•.c•••)     CallExpression
        #[attr]•••(•••a•••.b•)•.c         MemberExpression
        #[attr]                           Attribute
                      a•••.b              MemberExpression                                                                                */
    foo(#[attr] ( (   a   .b ) .c ) );                                                                                                    /*
    foo(#[attr]•(•(•••a•••.b•)•.c•)•);    ExpressionStatement
    foo(#[attr]•(•(•••a•••.b•)•.c•)•)     CallExpression
        #[attr]•(•(•••a•••.b•)•.c         MemberExpression
        #[attr]                           Attribute
                      a•••.b              MemberExpression                                                                                */
    foo(#[attr] (     a   .b   .c ) );                                                                                                    /*
    foo(#[attr]•(•••••a•••.b•••.c•)•);    ExpressionStatement
    foo(#[attr]•(•••••a•••.b•••.c•)•)     CallExpression
        #[attr]•(•••••a•••.b•••.c         MemberExpression
        #[attr]                           Attribute
                      a•••.b              MemberExpression                                                                                */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 80
// Parsed Nodes: 1211
// state_rollbacks: 16
// Total '.charCodeAt()' calls: 6408 (29% re-reads)
// Unnecessary 'skip_whitespace()' calls: 685
// source: "../../samples/expressions/parens.rs"