enum E {}                                                                                                                                 /*
enum•E•{}    EnumDeclaration                                                                                                              */

enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>                                                                                                            */
    Foo { limb_with_align16: Align16 },                                                                                                   /*
    Foo•{•limb_with_align16:•Align16•}     EnumMemberStructDeclaration
          limb_with_align16:•Align16       StructPropertyDeclaration                                                                      */
    Bar,                                                                                                                                  /*
    Bar     EnumMemberDeclaration                                                                                                         */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */
enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>                                                                                                            */
    Foo { foo: u32 },                                                                                                                     /*
    Foo•{•foo:•u32•}     EnumMemberStructDeclaration
          foo:•u32       StructPropertyDeclaration                                                                                        */
    Bar { bar: u32 },                                                                                                                     /*
    Bar•{•bar:•u32•}     EnumMemberStructDeclaration
          bar:•u32       StructPropertyDeclaration                                                                                        */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */

enum A {                                                                                                                                  /*
enum•A•{↲    <EnumDeclaration>                                                                                                            */
	Ok = u8::MAX - 1,                                                                                                                     /*
    Ok•=•u8::MAX•-•1    EnumMemberDeclaration
         u8::MAX•-•1    OperationExpression
         u8::MAX        ExpressionPath
                   1    Literal                                                                                                           */
	Ok2 = -1,                                                                                                                             /*
    Ok2•=•-1    EnumMemberDeclaration
          -1    MinusExpression
           1    Literal                                                                                                                   */
	OhNo = u8::MIN,                                                                                                                       /*
    OhNo•=•u8::MIN    EnumMemberDeclaration
           u8::MIN    ExpressionPath                                                                                                      */
	Bi64 = 0x8000_0000,                                                                                                                   /*
    Bi64•=•0x8000_0000    EnumMemberDeclaration
           0x8000_0000    Literal                                                                                                         */
	orange = 8 >> 1                                                                                                                       /*
    orange•=•8•>>•1    EnumMemberDeclaration
             8•>>•1    OperationExpression
             8         Literal
                  1    Literal                                                                                                            */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */
enum A { union }                                                                                                                          /*
enum•A•{•union•}    EnumDeclaration
         union      EnumMemberDeclaration                                                                                                 */
enum B { union {} }                                                                                                                       /*
enum•B•{•union•{}•}    EnumDeclaration
         union•{}      EnumMemberStructDeclaration                                                                                        */
enum C { union() }                                                                                                                        /*
enum•C•{•union()•}    EnumDeclaration
         union()      EnumMemberTupleDeclaration                                                                                          */

enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>                                                                                                            */
    A = { enum F { B } 0 }                                                                                                                /*
    A•=•{•enum•F•{•B•}•0•}    EnumMemberDeclaration
        {•enum•F•{•B•}•0•}    BlockExpression
          enum•F•{•B•}        EnumDeclaration
                   B          EnumMemberDeclaration
                       0      ExpressionStatement, Literal                                                                                */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */
enum E<T> {                                                                                                                               /*
enum•E<T>•{↲    <EnumDeclaration>
       T        GenericTypeParameterDeclaration                                                                                           */
    _None,                                                                                                                                /*
    _None     EnumMemberDeclaration                                                                                                       */
    _Some(T),                                                                                                                             /*
    _Some(T)     EnumMemberTupleDeclaration
          T      TupleStructItemDeclaration                                                                                               */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */
enum E<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized> {                                                                                      /*
enum•E<W:•?Sized,•X:•?Sized,•Y:•?Sized,•Z:•?Sized>•{↲    <EnumDeclaration>
       W:•?Sized                                         GenericTypeParameterDeclaration
          ?Sized                                         TypeTraitBound
                  X:•?Sized                              GenericTypeParameterDeclaration
                     ?Sized                              TypeTraitBound
                             Y:•?Sized                   GenericTypeParameterDeclaration
                                ?Sized                   TypeTraitBound
                                        Z:•?Sized        GenericTypeParameterDeclaration
                                           ?Sized        TypeTraitBound                                                                   */
    EM(W),                                                                                                                                /*
    EM(W)     EnumMemberTupleDeclaration
       W      TupleStructItemDeclaration                                                                                                  */
    EM{x: X},                                                                                                                             /*
    EM{x:•X}     EnumMemberStructDeclaration
       x:•X      StructPropertyDeclaration                                                                                                */
    EM(isize, Y),                                                                                                                         /*
    EM(isize,•Y)     EnumMemberTupleDeclaration
       isize         TupleStructItemDeclaration
              Y      TupleStructItemDeclaration                                                                                           */
    EM{u: isize, x: Z},                                                                                                                   /*
    EM{u:•isize,•x:•Z}     EnumMemberStructDeclaration
       u:•isize            StructPropertyDeclaration
                 x:•Z      StructPropertyDeclaration                                                                                      */
    EM([u8]),                                                                                                                             /*
    EM([u8])     EnumMemberTupleDeclaration
       [u8]      TupleStructItemDeclaration, TypeSlice                                                                                    */
    EM{x: str},                                                                                                                           /*
    EM{x:•str}     EnumMemberStructDeclaration
       x:•str      StructPropertyDeclaration                                                                                              */
    EM(isize, [f32]),                                                                                                                     /*
    EM(isize,•[f32])     EnumMemberTupleDeclaration
       isize             TupleStructItemDeclaration
              [f32]      TupleStructItemDeclaration, TypeSlice                                                                            */
    EM{u: isize, x: [u32]},                                                                                                               /*
    EM{u:•isize,•x:•[u32]}     EnumMemberStructDeclaration
       u:•isize                StructPropertyDeclaration
                 x:•[u32]      StructPropertyDeclaration
                    [u32]      TypeSlice                                                                                                  */
    EM(Path1),                                                                                                                            /*
    EM(Path1)     EnumMemberTupleDeclaration
       Path1      TupleStructItemDeclaration                                                                                              */
    EM{x: Path2},                                                                                                                         /*
    EM{x:•Path2}     EnumMemberStructDeclaration
       x:•Path2      StructPropertyDeclaration                                                                                            */
    EM(isize, Path3),                                                                                                                     /*
    EM(isize,•Path3)     EnumMemberTupleDeclaration
       isize             TupleStructItemDeclaration
              Path3      TupleStructItemDeclaration                                                                                       */
    EM{u: isize, x: Path4},                                                                                                               /*
    EM{u:•isize,•x:•Path4}     EnumMemberStructDeclaration
       u:•isize                StructPropertyDeclaration
                 x:•Path4      StructPropertyDeclaration                                                                                  */
    EM(dyn Foo),                                                                                                                          /*
    EM(dyn•Foo)     EnumMemberTupleDeclaration
       dyn•Foo      TupleStructItemDeclaration, TypeDynBounds
           Foo      TypeTraitBound                                                                                                        */
    EM{x: dyn Bar},                                                                                                                       /*
    EM{x:•dyn•Bar}     EnumMemberStructDeclaration
       x:•dyn•Bar      StructPropertyDeclaration
          dyn•Bar      TypeDynBounds
              Bar      TypeTraitBound                                                                                                     */
    EM(isize, dyn FooBar),                                                                                                                /*
    EM(isize,•dyn•FooBar)     EnumMemberTupleDeclaration
       isize                  TupleStructItemDeclaration
              dyn•FooBar      TupleStructItemDeclaration, TypeDynBounds
                  FooBar      TypeTraitBound                                                                                              */
    EM{u: isize, x: dyn BarFoo},                                                                                                          /*
    EM{u:•isize,•x:•dyn•BarFoo}     EnumMemberStructDeclaration
       u:•isize                     StructPropertyDeclaration
                 x:•dyn•BarFoo      StructPropertyDeclaration
                    dyn•BarFoo      TypeDynBounds
                        BarFoo      TypeTraitBound                                                                                        */
    EM(<&'static [i8] as Deref>::Target),                                                                                                 /*
    EM(<&'static•[i8]•as•Deref>::Target)     EnumMemberTupleDeclaration
       <&'static•[i8]•as•Deref>::Target      TupleStructItemDeclaration, TypePath
       <&'static•[i8]•as•Deref>              ExpressionTypeSelector
        &'static•[i8]                        TypeReference
         'static                             LtStatic
                 [i8]                        TypeSlice                                                                                    */
    EM{x: <&'static [char] as Deref>::Target},                                                                                            /*
    EM{x:•<&'static•[char]•as•Deref>::Target}     EnumMemberStructDeclaration
       x:•<&'static•[char]•as•Deref>::Target      StructPropertyDeclaration
          <&'static•[char]•as•Deref>::Target      TypePath
          <&'static•[char]•as•Deref>              ExpressionTypeSelector
           &'static•[char]                        TypeReference
            'static                               LtStatic
                    [char]                        TypeSlice                                                                               */
    EM(isize, <&'static [f64] as Deref>::Target),                                                                                         /*
    EM(isize,•<&'static•[f64]•as•Deref>::Target)     EnumMemberTupleDeclaration
       isize                                         TupleStructItemDeclaration
              <&'static•[f64]•as•Deref>::Target      TupleStructItemDeclaration, TypePath
              <&'static•[f64]•as•Deref>              ExpressionTypeSelector
               &'static•[f64]                        TypeReference
                'static                              LtStatic
                        [f64]                        TypeSlice                                                                            */
    EM{u: isize, x: <&'static [i32] as Deref>::Target},                                                                                   /*
    EM{u:•isize,•x:•<&'static•[i32]•as•Deref>::Target}     EnumMemberStructDeclaration
       u:•isize                                            StructPropertyDeclaration
                 x:•<&'static•[i32]•as•Deref>::Target      StructPropertyDeclaration
                    <&'static•[i32]•as•Deref>::Target      TypePath
                    <&'static•[i32]•as•Deref>              ExpressionTypeSelector
                     &'static•[i32]                        TypeReference
                      'static                              LtStatic
                              [i32]                        TypeSlice                                                                      */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */
enum E<'a, 'b, 'c:'b> {                                                                                                                   /*
enum•E<'a,•'b,•'c:'b>•{↲    <EnumDeclaration>
       'a                   GenericLtParameterDeclaration, LtIdentifier
           'b               GenericLtParameterDeclaration, LtIdentifier
               'c:'b        GenericLtParameterDeclaration
               'c           LtIdentifier
                  'b        LtIdentifier                                                                                                  */
    A(extern "Rust" fn(&'a isize)),                                                                                                       /*
    A(extern•"Rust"•fn(&'a•isize))     EnumMemberTupleDeclaration
      extern•"Rust"•fn(&'a•isize)      TupleStructItemDeclaration, TypeFnPointer
      extern•"Rust"                    ExternSpecifier
             "Rust"                    Literal
                       &'a•isize       TypeFnPointerParameter, TypeReference
                        'a             LtIdentifier                                                                                       */
    B(&'b [isize]),                                                                                                                       /*
    B(&'b•[isize])     EnumMemberTupleDeclaration
      &'b•[isize]      TupleStructItemDeclaration, TypeReference
       'b              LtIdentifier
          [isize]      TypeSlice                                                                                                          */
    C(&'b mut &'c str),                                                                                                                   /*
    C(&'b•mut•&'c•str)     EnumMemberTupleDeclaration
      &'b•mut•&'c•str      TupleStructItemDeclaration, TypeReference
       'b                  LtIdentifier
              &'c•str      TypeReference
               'c          LtIdentifier                                                                                                   */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */

pub enum X<D> where D: Copy + Debug + Eq {}                                                                                               /*
pub•enum•X<D>•where•D:•Copy•+•Debug•+•Eq•{}    EnumDeclaration
pub                                            PubSpecifier
           D                                   GenericTypeParameterDeclaration
                    D:•Copy•+•Debug•+•Eq       WhereTypeBoundDeclaration
                       Copy                    TypeTraitBound
                              Debug            TypeTraitBound
                                      Eq       TypeTraitBound                                                                             */

// Discarded Nodes: 0
// Parsed Nodes: 303
// state_rollbacks: 1
// Total '.charCodeAt()' calls: 1434 (32% re-reads)
// Unnecessary 'skip_whitespace()' calls: 219
// source: "../../samples/statements/enum.rs"