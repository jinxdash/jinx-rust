enum E {}                                                                                                                                 /*
enum•E•{}↲    <Program>
enum•E•{}↲    <Program.ast{dk: "None"}>
enum•E•{}     EnumDeclaration
       {}     EnumDeclaration.members{dk: "{}"}                                                                                           */

enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>
       {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                          */
    Foo { limb_with_align16: Align16 },                                                                                                   /*
    Foo•{•limb_with_align16:•Align16•}    EnumMemberStructDeclaration
        {•limb_with_align16:•Align16•}    EnumMemberStructDeclaration.properties{dk: "{}"}
          limb_with_align16:•Align16      StructPropertyDeclaration                                                                       */
    Bar,                                                                                                                                  /*
    Bar    EnumMemberDeclaration                                                                                                          */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>
       {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                          */
    Foo { foo: u32 },                                                                                                                     /*
    Foo•{•foo:•u32•}    EnumMemberStructDeclaration
        {•foo:•u32•}    EnumMemberStructDeclaration.properties{dk: "{}"}
          foo:•u32      StructPropertyDeclaration                                                                                         */
    Bar { bar: u32 },                                                                                                                     /*
    Bar•{•bar:•u32•}    EnumMemberStructDeclaration
        {•bar:•u32•}    EnumMemberStructDeclaration.properties{dk: "{}"}
          bar:•u32      StructPropertyDeclaration                                                                                         */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */

enum A {                                                                                                                                  /*
enum•A•{↲    <EnumDeclaration>
       {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                          */
	Ok = u8::MAX - 1,                                                                                                                     /*
	Ok•=•u8::MAX•-•1    EnumMemberDeclaration
	     u8::MAX•-•1    OperationExpression{tk: "-"}
	     u8::MAX        ExpressionPath
	               1    Literal{kind: Integer}                                                                                            */
	Ok2 = -1,                                                                                                                             /*
	Ok2•=•-1    EnumMemberDeclaration
	      -1    MinusExpression
	       1    Literal{kind: Integer}                                                                                                    */
	OhNo = u8::MIN,                                                                                                                       /*
	OhNo•=•u8::MIN    EnumMemberDeclaration
	       u8::MIN    ExpressionPath                                                                                                      */
	Bi64 = 0x8000_0000,                                                                                                                   /*
	Bi64•=•0x8000_0000    EnumMemberDeclaration
	       0x8000_0000    Literal{kind: Hex}                                                                                              */
	orange = 8 >> 1                                                                                                                       /*
	orange•=•8•>>•1    EnumMemberDeclaration
	         8•>>•1    OperationExpression{tk: ">>"}
	         8         Literal{kind: Integer}
	              1    Literal{kind: Integer}                                                                                             */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
enum A { union }                                                                                                                          /*
enum•A•{•union•}    EnumDeclaration
       {•union•}    EnumDeclaration.members{dk: "{}"}
         union      EnumMemberDeclaration                                                                                                 */
enum B { union {} }                                                                                                                       /*
enum•B•{•union•{}•}    EnumDeclaration
       {•union•{}•}    EnumDeclaration.members{dk: "{}"}
         union•{}      EnumMemberStructDeclaration
               {}      EnumMemberStructDeclaration.properties{dk: "{}"}                                                                   */
enum C { union() }                                                                                                                        /*
enum•C•{•union()•}    EnumDeclaration
       {•union()•}    EnumDeclaration.members{dk: "{}"}
         union()      EnumMemberTupleDeclaration
              ()      EnumMemberTupleDeclaration.items{dk: "()"}                                                                          */

enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>
       {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                          */
    A = { enum F { B } 0 }                                                                                                                /*
    A•=•{•enum•F•{•B•}•0•}    EnumMemberDeclaration
        {•enum•F•{•B•}•0•}    BlockExpression
          enum•F•{•B•}        EnumDeclaration
                 {•B•}        EnumDeclaration.members{dk: "{}"}
                   B          EnumMemberDeclaration
                       0      ExpressionStatement{!semi}, Literal{kind: Integer}                                                          */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
enum E<T> {                                                                                                                               /*
enum•E<T>•{↲    <EnumDeclaration>
      <T>       EnumDeclaration.generics{dk: "<>"}
       T        GenericTypeParameterDeclaration
          {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                       */
    _None,                                                                                                                                /*
    _None    EnumMemberDeclaration                                                                                                        */
    _Some(T),                                                                                                                             /*
    _Some(T)    EnumMemberTupleDeclaration
         (T)    EnumMemberTupleDeclaration.items{dk: "()"}
          T     TupleStructItemDeclaration                                                                                                */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
enum E<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized> {                                                                                      /*
enum•E<W:•?Sized,•X:•?Sized,•Y:•?Sized,•Z:•?Sized>•{↲    <EnumDeclaration>
      <W:•?Sized,•X:•?Sized,•Y:•?Sized,•Z:•?Sized>       EnumDeclaration.generics{dk: "<>"}
       W:•?Sized                                         GenericTypeParameterDeclaration
          ?Sized                                         TypeTraitBound{!maybeConst, optional}
                  X:•?Sized                              GenericTypeParameterDeclaration
                     ?Sized                              TypeTraitBound{!maybeConst, optional}
                             Y:•?Sized                   GenericTypeParameterDeclaration
                                ?Sized                   TypeTraitBound{!maybeConst, optional}
                                        Z:•?Sized        GenericTypeParameterDeclaration
                                           ?Sized        TypeTraitBound{!maybeConst, optional}
                                                   {↲    <EnumDeclaration.members{dk: "{}"}>                                              */
    EM(W),                                                                                                                                /*
    EM(W)    EnumMemberTupleDeclaration
      (W)    EnumMemberTupleDeclaration.items{dk: "()"}
       W     TupleStructItemDeclaration                                                                                                   */
    EM{x: X},                                                                                                                             /*
    EM{x:•X}    EnumMemberStructDeclaration
      {x:•X}    EnumMemberStructDeclaration.properties{dk: "{}"}
       x:•X     StructPropertyDeclaration                                                                                                 */
    EM(isize, Y),                                                                                                                         /*
    EM(isize,•Y)    EnumMemberTupleDeclaration
      (isize,•Y)    EnumMemberTupleDeclaration.items{dk: "()"}
       isize        TupleStructItemDeclaration
              Y     TupleStructItemDeclaration                                                                                            */
    EM{u: isize, x: Z},                                                                                                                   /*
    EM{u:•isize,•x:•Z}    EnumMemberStructDeclaration
      {u:•isize,•x:•Z}    EnumMemberStructDeclaration.properties{dk: "{}"}
       u:•isize           StructPropertyDeclaration
                 x:•Z     StructPropertyDeclaration                                                                                       */
    EM([u8]),                                                                                                                             /*
    EM([u8])    EnumMemberTupleDeclaration
      ([u8])    EnumMemberTupleDeclaration.items{dk: "()"}
       [u8]     TupleStructItemDeclaration, TypeSlice                                                                                     */
    EM{x: str},                                                                                                                           /*
    EM{x:•str}    EnumMemberStructDeclaration
      {x:•str}    EnumMemberStructDeclaration.properties{dk: "{}"}
       x:•str     StructPropertyDeclaration                                                                                               */
    EM(isize, [f32]),                                                                                                                     /*
    EM(isize,•[f32])    EnumMemberTupleDeclaration
      (isize,•[f32])    EnumMemberTupleDeclaration.items{dk: "()"}
       isize            TupleStructItemDeclaration
              [f32]     TupleStructItemDeclaration, TypeSlice                                                                             */
    EM{u: isize, x: [u32]},                                                                                                               /*
    EM{u:•isize,•x:•[u32]}    EnumMemberStructDeclaration
      {u:•isize,•x:•[u32]}    EnumMemberStructDeclaration.properties{dk: "{}"}
       u:•isize               StructPropertyDeclaration
                 x:•[u32]     StructPropertyDeclaration
                    [u32]     TypeSlice                                                                                                   */
    EM(Path1),                                                                                                                            /*
    EM(Path1)    EnumMemberTupleDeclaration
      (Path1)    EnumMemberTupleDeclaration.items{dk: "()"}
       Path1     TupleStructItemDeclaration                                                                                               */
    EM{x: Path2},                                                                                                                         /*
    EM{x:•Path2}    EnumMemberStructDeclaration
      {x:•Path2}    EnumMemberStructDeclaration.properties{dk: "{}"}
       x:•Path2     StructPropertyDeclaration                                                                                             */
    EM(isize, Path3),                                                                                                                     /*
    EM(isize,•Path3)    EnumMemberTupleDeclaration
      (isize,•Path3)    EnumMemberTupleDeclaration.items{dk: "()"}
       isize            TupleStructItemDeclaration
              Path3     TupleStructItemDeclaration                                                                                        */
    EM{u: isize, x: Path4},                                                                                                               /*
    EM{u:•isize,•x:•Path4}    EnumMemberStructDeclaration
      {u:•isize,•x:•Path4}    EnumMemberStructDeclaration.properties{dk: "{}"}
       u:•isize               StructPropertyDeclaration
                 x:•Path4     StructPropertyDeclaration                                                                                   */
    EM(dyn Foo),                                                                                                                          /*
    EM(dyn•Foo)    EnumMemberTupleDeclaration
      (dyn•Foo)    EnumMemberTupleDeclaration.items{dk: "()"}
       dyn•Foo     TupleStructItemDeclaration, TypeDynBounds{dyn}
           Foo     TypeTraitBound{!maybeConst, !optional}                                                                                 */
    EM{x: dyn Bar},                                                                                                                       /*
    EM{x:•dyn•Bar}    EnumMemberStructDeclaration
      {x:•dyn•Bar}    EnumMemberStructDeclaration.properties{dk: "{}"}
       x:•dyn•Bar     StructPropertyDeclaration
          dyn•Bar     TypeDynBounds{dyn}
              Bar     TypeTraitBound{!maybeConst, !optional}                                                                              */
    EM(isize, dyn FooBar),                                                                                                                /*
    EM(isize,•dyn•FooBar)    EnumMemberTupleDeclaration
      (isize,•dyn•FooBar)    EnumMemberTupleDeclaration.items{dk: "()"}
       isize                 TupleStructItemDeclaration
              dyn•FooBar     TupleStructItemDeclaration, TypeDynBounds{dyn}
                  FooBar     TypeTraitBound{!maybeConst, !optional}                                                                       */
    EM{u: isize, x: dyn BarFoo},                                                                                                          /*
    EM{u:•isize,•x:•dyn•BarFoo}    EnumMemberStructDeclaration
      {u:•isize,•x:•dyn•BarFoo}    EnumMemberStructDeclaration.properties{dk: "{}"}
       u:•isize                    StructPropertyDeclaration
                 x:•dyn•BarFoo     StructPropertyDeclaration
                    dyn•BarFoo     TypeDynBounds{dyn}
                        BarFoo     TypeTraitBound{!maybeConst, !optional}                                                                 */
    EM(<&'static [i8] as Deref>::Target),                                                                                                 /*
    EM(<&'static•[i8]•as•Deref>::Target)    EnumMemberTupleDeclaration
      (<&'static•[i8]•as•Deref>::Target)    EnumMemberTupleDeclaration.items{dk: "()"}
       <&'static•[i8]•as•Deref>::Target     TupleStructItemDeclaration, TypePath
       <&'static•[i8]•as•Deref>             ExpressionTypeSelector
        &'static•[i8]                       TypeReference{!mut}
         'static                            LtStatic
                 [i8]                       TypeSlice                                                                                     */
    EM{x: <&'static [char] as Deref>::Target},                                                                                            /*
    EM{x:•<&'static•[char]•as•Deref>::Target}    EnumMemberStructDeclaration
      {x:•<&'static•[char]•as•Deref>::Target}    EnumMemberStructDeclaration.properties{dk: "{}"}
       x:•<&'static•[char]•as•Deref>::Target     StructPropertyDeclaration
          <&'static•[char]•as•Deref>::Target     TypePath
          <&'static•[char]•as•Deref>             ExpressionTypeSelector
           &'static•[char]                       TypeReference{!mut}
            'static                              LtStatic
                    [char]                       TypeSlice                                                                                */
    EM(isize, <&'static [f64] as Deref>::Target),                                                                                         /*
    EM(isize,•<&'static•[f64]•as•Deref>::Target)    EnumMemberTupleDeclaration
      (isize,•<&'static•[f64]•as•Deref>::Target)    EnumMemberTupleDeclaration.items{dk: "()"}
       isize                                        TupleStructItemDeclaration
              <&'static•[f64]•as•Deref>::Target     TupleStructItemDeclaration, TypePath
              <&'static•[f64]•as•Deref>             ExpressionTypeSelector
               &'static•[f64]                       TypeReference{!mut}
                'static                             LtStatic
                        [f64]                       TypeSlice                                                                             */
    EM{u: isize, x: <&'static [i32] as Deref>::Target},                                                                                   /*
    EM{u:•isize,•x:•<&'static•[i32]•as•Deref>::Target}    EnumMemberStructDeclaration
      {u:•isize,•x:•<&'static•[i32]•as•Deref>::Target}    EnumMemberStructDeclaration.properties{dk: "{}"}
       u:•isize                                           StructPropertyDeclaration
                 x:•<&'static•[i32]•as•Deref>::Target     StructPropertyDeclaration
                    <&'static•[i32]•as•Deref>::Target     TypePath
                    <&'static•[i32]•as•Deref>             ExpressionTypeSelector
                     &'static•[i32]                       TypeReference{!mut}
                      'static                             LtStatic
                              [i32]                       TypeSlice                                                                       */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
enum E<'a, 'b, 'c:'b> {                                                                                                                   /*
enum•E<'a,•'b,•'c:'b>•{↲    <EnumDeclaration>
      <'a,•'b,•'c:'b>       EnumDeclaration.generics{dk: "<>"}
       'a                   GenericLtParameterDeclaration, LtIdentifier
           'b               GenericLtParameterDeclaration, LtIdentifier
               'c:'b        GenericLtParameterDeclaration
               'c           LtIdentifier
                  'b        LtIdentifier
                      {↲    <EnumDeclaration.members{dk: "{}"}>                                                                           */
    A(extern "Rust" fn(&'a isize)),                                                                                                       /*
    A(extern•"Rust"•fn(&'a•isize))    EnumMemberTupleDeclaration
     (extern•"Rust"•fn(&'a•isize))    EnumMemberTupleDeclaration.items{dk: "()"}
      extern•"Rust"•fn(&'a•isize)     TupleStructItemDeclaration, TypeFnPointer
      extern•"Rust"                   ExternSpecifier
             "Rust"                   Literal{kind: String}
                      (&'a•isize)     TypeFnPointer.parameters{dk: "()"}
                       &'a•isize      TypeFnPointerParameter, TypeReference{!mut}
                        'a            LtIdentifier                                                                                        */
    B(&'b [isize]),                                                                                                                       /*
    B(&'b•[isize])    EnumMemberTupleDeclaration
     (&'b•[isize])    EnumMemberTupleDeclaration.items{dk: "()"}
      &'b•[isize]     TupleStructItemDeclaration, TypeReference{!mut}
       'b             LtIdentifier
          [isize]     TypeSlice                                                                                                           */
    C(&'b mut &'c str),                                                                                                                   /*
    C(&'b•mut•&'c•str)    EnumMemberTupleDeclaration
     (&'b•mut•&'c•str)    EnumMemberTupleDeclaration.items{dk: "()"}
      &'b•mut•&'c•str     TupleStructItemDeclaration, TypeReference{mut}
       'b                 LtIdentifier
              &'c•str     TypeReference{!mut}
               'c         LtIdentifier                                                                                                    */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */

pub enum X<D> where D: Copy + Debug + Eq {}                                                                                               /*
pub•enum•X<D>•where•D:•Copy•+•Debug•+•Eq•{}    EnumDeclaration
pub                                            PubSpecifier
          <D>                                  EnumDeclaration.generics{dk: "<>"}
           D                                   GenericTypeParameterDeclaration
              where•D:•Copy•+•Debug•+•Eq       EnumDeclaration.whereBounds{dk: "None"}
                    D:•Copy•+•Debug•+•Eq       WhereTypeBoundDeclaration
                       Copy                    TypeTraitBound{!maybeConst, !optional}
                              Debug            TypeTraitBound{!maybeConst, !optional}
                                      Eq       TypeTraitBound{!maybeConst, !optional}
                                         {}    EnumDeclaration.members{dk: "{}"}
pub•enum•X<D>•where•D:•Copy•+•Debug•+•Eq•{}    </Program.ast>
pub•enum•X<D>•where•D:•Copy•+•Debug•+•Eq•{}    </Program>                                                                                 */
// Discarded Nodes: 0
// Parsed Nodes: 303
// state_rollbacks: 1
// Total '.charCodeAt()' calls: 1434 (32% re-reads)
// Unnecessary 'skip_whitespace()' calls: 219
// source: "../../samples/statements/enum.rs"