impl X {}                                                                                                                                 /*
impl•X•{}↲    <Program>
impl•X•{}↲    <Program.ast{dk: "None"}>
impl•X•{}     ImplDeclaration{!const}
       {}     ImplDeclaration.body{dk: "{}"}                                                                                              */
impl X {                                                                                                                                  /*
impl•X•{↲    <ImplDeclaration{!const}>
       {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                             */
	fn f();                                                                                                                               /*
	fn•f();    FunctionDeclaration
	    ()     FunctionDeclaration.parameters{dk: "()"}                                                                                   */
	fn f() {}                                                                                                                             /*
	fn•f()•{}    FunctionDeclaration
	    ()       FunctionDeclaration.parameters{dk: "()"}
	       {}    FunctionDeclaration.body{dk: "{}"}                                                                                       */
    type Y;                                                                                                                               /*
    type•Y;    TypeAliasDeclaration                                                                                                       */
    type Z: Ord;                                                                                                                          /*
    type•Z:•Ord;    TypeAliasDeclaration
            Ord     TypeTraitBound{!maybeConst, !optional}                                                                                */
    type W: Ord where Self: Eq;                                                                                                           /*
    type•W:•Ord•where•Self:•Eq;    TypeAliasDeclaration
            Ord                    TypeTraitBound{!maybeConst, !optional}
                where•Self:•Eq     TypeAliasDeclaration.whereBounds{dk: "None"}
                      Self:•Eq     WhereTypeBoundDeclaration
                            Eq     TypeTraitBound{!maybeConst, !optional}                                                                 */
    type W where Self: Eq;                                                                                                                /*
    type•W•where•Self:•Eq;    TypeAliasDeclaration
           where•Self:•Eq     TypeAliasDeclaration.whereBounds{dk: "None"}
                 Self:•Eq     WhereTypeBoundDeclaration
                       Eq     TypeTraitBound{!maybeConst, !optional}                                                                      */
	fn foo() {                                                                                                                            /*
	fn•foo()•{↲    <FunctionDeclaration>
	      ()       FunctionDeclaration.parameters{dk: "()"}
	         {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                   */
        struct S;                                                                                                                         /*
        struct•S;    StructDeclaration                                                                                                    */
        impl S {                                                                                                                          /*
        impl•S•{↲    <ImplDeclaration{!const}>
               {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                     */
            pub const X: u8 = 0;                                                                                                          /*
            pub•const•X:•u8•=•0;    ConstVariableDeclaration
            pub                     PubSpecifier
                              0     Literal{kind: Integer}                                                                                */
            pub const fn bar() {}                                                                                                         /*
            pub•const•fn•bar()•{}    FunctionDeclaration{const}
            pub                      PubSpecifier
                            ()       FunctionDeclaration.parameters{dk: "()"}
                               {}    FunctionDeclaration.body{dk: "{}"}                                                                   */
            async fn qux() {}                                                                                                             /*
            async•fn•qux()•{}    FunctionDeclaration{async}
                        ()       FunctionDeclaration.parameters{dk: "()"}
                           {}    FunctionDeclaration.body{dk: "{}"}                                                                       */
        }                                                                                                                                 /*
••••••••}    </ImplDeclaration.body>
••••••••}    </ImplDeclaration>                                                                                                           */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration.body>
••••}    </FunctionDeclaration>                                                                                                           */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */
impl X for () {}                                                                                                                          /*
impl•X•for•()•{}    ImplDeclaration{!const}
           ()       TypeTuple
              {}    ImplDeclaration.body{dk: "{}"}                                                                                        */
impl X for Y {}                                                                                                                           /*
impl•X•for•Y•{}    ImplDeclaration{!const}
             {}    ImplDeclaration.body{dk: "{}"}                                                                                         */
impl ! {}                                                                                                                                 /*
impl•!•{}    ImplDeclaration{!const}
     !       TypeNever
       {}    ImplDeclaration.body{dk: "{}"}                                                                                               */
impl ! where u8: A {}                                                                                                                     /*
impl•!•where•u8:•A•{}    ImplDeclaration{!const}
     !                   TypeNever
       where•u8:•A       ImplDeclaration.whereBounds{dk: "None"}
             u8:•A       WhereTypeBoundDeclaration
                 A       TypeTraitBound{!maybeConst, !optional}
                   {}    ImplDeclaration.body{dk: "{}"}                                                                                   */
impl !where u8: A {}                                                                                                                      /*
impl•!where•u8:•A•{}    ImplDeclaration{!const}
     !                  TypeNever
      where•u8:•A       ImplDeclaration.whereBounds{dk: "None"}
            u8:•A       WhereTypeBoundDeclaration
                A       TypeTraitBound{!maybeConst, !optional}
                  {}    ImplDeclaration.body{dk: "{}"}                                                                                    */
impl !Send for A { }                                                                                                                      /*
impl•!Send•for•A•{•}    NegativeImplDeclaration                                                                                           */
impl <*const u8>::C {}                                                                                                                    /*
impl•<*const•u8>::C•{}    ImplDeclaration{!const}
     <*const•u8>::C       TypePath
     <*const•u8>          ExpressionTypeSelector
      *const•u8           TypeDereferenceConst
                    {}    ImplDeclaration.body{dk: "{}"}                                                                                  */
impl <A as B>::C {}                                                                                                                       /*
impl•<A•as•B>::C•{}    ImplDeclaration{!const}
     <A•as•B>::C       TypePath
     <A•as•B>          ExpressionTypeSelector
                 {}    ImplDeclaration.body{dk: "{}"}                                                                                     */
impl <'a + B>::C {}                                                                                                                       /*
impl•<'a•+•B>::C•{}    ImplDeclaration{!const}
     <'a•+•B>::C       TypePath
     <'a•+•B>          ExpressionTypeSelector
      'a•+•B           TypeDynBounds{!dyn}
      'a               LtIdentifier
           B           TypeTraitBound{!maybeConst, !optional}
                 {}    ImplDeclaration.body{dk: "{}"}                                                                                     */
impl <<A>::B>::C {}                                                                                                                       /*
impl•<<A>::B>::C•{}    ImplDeclaration{!const}
     <<A>::B>::C       TypePath
     <<A>::B>          ExpressionTypeSelector
      <A>::B           TypePath
      <A>              ExpressionTypeSelector
                 {}    ImplDeclaration.body{dk: "{}"}                                                                                     */
impl<'a, I, T: 'a, E> Iterator for Y<'a, I, E> where I: Iterator<Item = &'a (T, E)>,{}                                                    /*
impl<'a,•I,•T:•'a,•E>•Iterator•for•Y<'a,•I,•E>•where•I:•Iterator<Item•=•&'a•(T,•E)>,{}    ImplDeclaration{!const}
    <'a,•I,•T:•'a,•E>                                                                     ImplDeclaration.generics{dk: "<>"}
     'a                                                                                   GenericLtParameterDeclaration, LtIdentifier
         I                                                                                GenericTypeParameterDeclaration
            T:•'a                                                                         GenericTypeParameterDeclaration
               'a                                                                         LtIdentifier
                   E                                                                      GenericTypeParameterDeclaration
                                   Y<'a,•I,•E>                                            TypeCall
                                    <'a,•I,•E>                                            TypeCall.typeArguments{dk: "<>"}
                                     'a                                                   LtIdentifier
                                               where•I:•Iterator<Item•=•&'a•(T,•E)>,      ImplDeclaration.whereBounds{dk: "None"}
                                                     I:•Iterator<Item•=•&'a•(T,•E)>       WhereTypeBoundDeclaration
                                                        Iterator<Item•=•&'a•(T,•E)>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                                <Item•=•&'a•(T,•E)>       TypeCall.typeArguments{dk: "<>"}
                                                                 Item•=•&'a•(T,•E)        TypeCallNamedArgument
                                                                        &'a•(T,•E)        TypeReference{!mut}
                                                                         'a               LtIdentifier
                                                                            (T,•E)        TypeTuple
                                                                                    {}    ImplDeclaration.body{dk: "{}"}                  */
impl<T: Q> S for E<T> {}                                                                                                                  /*
impl<T:•Q>•S•for•E<T>•{}    ImplDeclaration{!const}
    <T:•Q>                  ImplDeclaration.generics{dk: "<>"}
     T:•Q                   GenericTypeParameterDeclaration
        Q                   TypeTraitBound{!maybeConst, !optional}
                 E<T>       TypeCall
                  <T>       TypeCall.typeArguments{dk: "<>"}
                      {}    ImplDeclaration.body{dk: "{}"}                                                                                */
unsafe impl A for isize {}                                                                                                                /*
unsafe•impl•A•for•isize•{}    ImplDeclaration{!const, unsafe}
                        {}    ImplDeclaration.body{dk: "{}"}                                                                              */
unsafe impl Send for A {}                                                                                                                 /*
unsafe•impl•Send•for•A•{}    ImplDeclaration{!const, unsafe}
                       {}    ImplDeclaration.body{dk: "{}"}                                                                               */
unsafe impl Sync for A {}                                                                                                                 /*
unsafe•impl•Sync•for•A•{}    ImplDeclaration{!const, unsafe}
                       {}    ImplDeclaration.body{dk: "{}"}                                                                               */
impl<'a> A for &'a [isize] {}                                                                                                             /*
impl<'a>•A•for•&'a•[isize]•{}    ImplDeclaration{!const}
    <'a>                         ImplDeclaration.generics{dk: "<>"}
     'a                          GenericLtParameterDeclaration, LtIdentifier
               &'a•[isize]       TypeReference{!mut}
                'a               LtIdentifier
                   [isize]       TypeSlice
                           {}    ImplDeclaration.body{dk: "{}"}                                                                           */
impl ::A::B for ::C {}                                                                                                                    /*
impl•::A::B•for•::C•{}    ImplDeclaration{!const}
     ::A::B               TypePath
     ::A                  TypePath
                ::C       TypePath
                    {}    ImplDeclaration.body{dk: "{}"}                                                                                  */
impl ::A for () {}                                                                                                                        /*
impl•::A•for•()•{}    ImplDeclaration{!const}
     ::A              TypePath
             ()       TypeTuple
                {}    ImplDeclaration.body{dk: "{}"}                                                                                      */
impl ::A {}                                                                                                                               /*
impl•::A•{}    ImplDeclaration{!const}
     ::A       TypePath
         {}    ImplDeclaration.body{dk: "{}"}                                                                                             */
impl A for [B; 1] {}                                                                                                                      /*
impl•A•for•[B;•1]•{}    ImplDeclaration{!const}
           [B;•1]       TypeSizedArray
               1        Literal{kind: Integer}
                  {}    ImplDeclaration.body{dk: "{}"}                                                                                    */
impl A for (<B as C>::D, E) {}                                                                                                            /*
impl•A•for•(<B•as•C>::D,•E)•{}    ImplDeclaration{!const}
           (<B•as•C>::D,•E)       TypeTuple
            <B•as•C>::D           TypePath
            <B•as•C>              ExpressionTypeSelector
                            {}    ImplDeclaration.body{dk: "{}"}                                                                          */
impl ::A for [B; 0] {}                                                                                                                    /*
impl•::A•for•[B;•0]•{}    ImplDeclaration{!const}
     ::A                  TypePath
             [B;•0]       TypeSizedArray
                 0        Literal{kind: Integer}
                    {}    ImplDeclaration.body{dk: "{}"}                                                                                  */
impl<'a> A for &'a [isize] {}                                                                                                             /*
impl<'a>•A•for•&'a•[isize]•{}    ImplDeclaration{!const}
    <'a>                         ImplDeclaration.generics{dk: "<>"}
     'a                          GenericLtParameterDeclaration, LtIdentifier
               &'a•[isize]       TypeReference{!mut}
                'a               LtIdentifier
                   [isize]       TypeSlice
                           {}    ImplDeclaration.body{dk: "{}"}                                                                           */
impl<'a> dyn T+'a {}                                                                                                                      /*
impl<'a>•dyn•T+'a•{}    ImplDeclaration{!const}
    <'a>                ImplDeclaration.generics{dk: "<>"}
     'a                 GenericLtParameterDeclaration, LtIdentifier
         dyn•T+'a       TypeDynBounds{dyn}
             T          TypeTraitBound{!maybeConst, !optional}
               'a       LtIdentifier
                  {}    ImplDeclaration.body{dk: "{}"}                                                                                    */
impl<'a> dyn T + 'a {}                                                                                                                    /*
impl<'a>•dyn•T•+•'a•{}    ImplDeclaration{!const}
    <'a>                  ImplDeclaration.generics{dk: "<>"}
     'a                   GenericLtParameterDeclaration, LtIdentifier
         dyn•T•+•'a       TypeDynBounds{dyn}
             T            TypeTraitBound{!maybeConst, !optional}
                 'a       LtIdentifier
                    {}    ImplDeclaration.body{dk: "{}"}                                                                                  */
impl<'a> dyn (::Foo::Trait) + 'a {}                                                                                                       /*
impl<'a>•dyn•(::Foo::Trait)•+•'a•{}    ImplDeclaration{!const}
    <'a>                               ImplDeclaration.generics{dk: "<>"}
     'a                                GenericLtParameterDeclaration, LtIdentifier
         dyn•(::Foo::Trait)•+•'a       TypeDynBounds{dyn}
              ::Foo::Trait             TypeTraitBound{!maybeConst, !optional}, TypePath
              ::Foo                    TypePath
                              'a       LtIdentifier
                                 {}    ImplDeclaration.body{dk: "{}"}                                                                     */
impl<T:?Sized> A for T { }                                                                                                                /*
impl<T:?Sized>•A•for•T•{•}    ImplDeclaration{!const}
    <T:?Sized>                ImplDeclaration.generics{dk: "<>"}
     T:?Sized                 GenericTypeParameterDeclaration
       ?Sized                 TypeTraitBound{!maybeConst, optional}
                       {•}    ImplDeclaration.body{dk: "{}"}                                                                              */
impl<'a,'b,'c> S for &'a &'b &'c Q {}                                                                                                     /*
impl<'a,'b,'c>•S•for•&'a•&'b•&'c•Q•{}    ImplDeclaration{!const}
    <'a,'b,'c>                           ImplDeclaration.generics{dk: "<>"}
     'a                                  GenericLtParameterDeclaration, LtIdentifier
        'b                               GenericLtParameterDeclaration, LtIdentifier
           'c                            GenericLtParameterDeclaration, LtIdentifier
                     &'a•&'b•&'c•Q       TypeReference{!mut}
                      'a                 LtIdentifier
                         &'b•&'c•Q       TypeReference{!mut}
                          'b             LtIdentifier
                             &'c•Q       TypeReference{!mut}
                              'c         LtIdentifier
                                   {}    ImplDeclaration.body{dk: "{}"}                                                                   */
impl<A,F:Fn(A)> Foo<A> for F {}                                                                                                           /*
impl<A,F:Fn(A)>•Foo<A>•for•F•{}    ImplDeclaration{!const}
    <A,F:Fn(A)>                    ImplDeclaration.generics{dk: "<>"}
     A                             GenericTypeParameterDeclaration
       F:Fn(A)                     GenericTypeParameterDeclaration
         Fn(A)                     TypeTraitBound{!maybeConst, !optional}, TypeFunction
           (A)                     TypeFunction.parameters{dk: "()"}
                Foo<A>             TypeCall
                   <A>             TypeCall.typeArguments{dk: "<>"}
                             {}    ImplDeclaration.body{dk: "{}"}                                                                         */
impl<T> A for T where for<'a> T: B<'a> {}                                                                                                 /*
impl<T>•A•for•T•where•for<'a>•T:•B<'a>•{}    ImplDeclaration{!const}
    <T>                                      ImplDeclaration.generics{dk: "<>"}
     T                                       GenericTypeParameterDeclaration
                where•for<'a>•T:•B<'a>       ImplDeclaration.whereBounds{dk: "None"}
                      for<'a>•T:•B<'a>       WhereTypeBoundDeclaration
                      for<'a>                WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                          'a                 GenericLtParameterDeclaration, LtIdentifier
                                 B<'a>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                                  <'a>       TypeCall.typeArguments{dk: "<>"}
                                   'a        LtIdentifier
                                       {}    ImplDeclaration.body{dk: "{}"}                                                               */
impl<'a, T, const N: usize> IntoIterator for &'a Table<T, N> {}                                                                           /*
impl<'a,•T,•const•N:•usize>•IntoIterator•for•&'a•Table<T,•N>•{}    ImplDeclaration{!const}
    <'a,•T,•const•N:•usize>                                        ImplDeclaration.generics{dk: "<>"}
     'a                                                            GenericLtParameterDeclaration, LtIdentifier
         T                                                         GenericTypeParameterDeclaration
            const•N:•usize                                         ConstTypeParameterDeclaration
                                             &'a•Table<T,•N>       TypeReference{!mut}
                                              'a                   LtIdentifier
                                                 Table<T,•N>       TypeCall
                                                      <T,•N>       TypeCall.typeArguments{dk: "<>"}
                                                             {}    ImplDeclaration.body{dk: "{}"}                                         */
impl A for (B,) {}                                                                                                                        /*
impl•A•for•(B,)•{}    ImplDeclaration{!const}
           (B,)       TypeTuple
                {}    ImplDeclaration.body{dk: "{}"}                                                                                      */
impl !A for (B,) {}                                                                                                                       /*
impl•!A•for•(B,)•{}    NegativeImplDeclaration
            (B,)       TypeTuple                                                                                                          */
impl A for Box<C> {}                                                                                                                      /*
impl•A•for•Box<C>•{}    ImplDeclaration{!const}
           Box<C>       TypeCall
              <C>       TypeCall.typeArguments{dk: "<>"}
                  {}    ImplDeclaration.body{dk: "{}"}                                                                                    */
impl A for lib::Something<C> {}                                                                                                           /*
impl•A•for•lib::Something<C>•{}    ImplDeclaration{!const}
           lib::Something<C>       TypeCall
           lib::Something          TypePath
                         <C>       TypeCall.typeArguments{dk: "<>"}
                             {}    ImplDeclaration.body{dk: "{}"}                                                                         */
impl A for D<C> {}                                                                                                                        /*
impl•A•for•D<C>•{}    ImplDeclaration{!const}
           D<C>       TypeCall
            <C>       TypeCall.typeArguments{dk: "<>"}
                {}    ImplDeclaration.body{dk: "{}"}                                                                                      */
impl<T: ::std::fmt::Display> A<T> {}                                                                                                      /*
impl<T:•::std::fmt::Display>•A<T>•{}    ImplDeclaration{!const}
    <T:•::std::fmt::Display>            ImplDeclaration.generics{dk: "<>"}
     T:•::std::fmt::Display             GenericTypeParameterDeclaration
        ::std::fmt::Display             TypeTraitBound{!maybeConst, !optional}, TypePath
        ::std::fmt                      TypePath
        ::std                           TypePath
                             A<T>       TypeCall
                              <T>       TypeCall.typeArguments{dk: "<>"}
                                  {}    ImplDeclaration.body{dk: "{}"}
impl<T:•::std::fmt::Display>•A<T>•{}    </Program.ast>
impl<T:•::std::fmt::Display>•A<T>•{}    </Program>                                                                                        */
// Discarded Nodes: 1
// Parsed Nodes: 318
// state_rollbacks: 34
// Total '.charCodeAt()' calls: 1975 (53% re-reads)
// Unnecessary 'skip_whitespace()' calls: 169
// source: "../../samples/statements/impl.rs"