impl X {}                                                                                                                                 /*
impl•X•{}    ImplDeclaration                                                                                                              */
impl X {                                                                                                                                  /*
impl•X•{↲    <ImplDeclaration>                                                                                                            */
	fn f();                                                                                                                               /*
    fn•f();    FunctionDeclaration                                                                                                        */
	fn f() {}                                                                                                                             /*
    fn•f()•{}    FunctionDeclaration                                                                                                      */
    type Y;                                                                                                                               /*
    type•Y;    TypeAliasDeclaration                                                                                                       */
    type Z: Ord;                                                                                                                          /*
    type•Z:•Ord;    TypeAliasDeclaration
            Ord     TypeTraitBound                                                                                                        */
    type W: Ord where Self: Eq;                                                                                                           /*
    type•W:•Ord•where•Self:•Eq;    TypeAliasDeclaration
            Ord                    TypeTraitBound
                      Self:•Eq     WhereTypeBoundDeclaration
                            Eq     TypeTraitBound                                                                                         */
    type W where Self: Eq;                                                                                                                /*
    type•W•where•Self:•Eq;    TypeAliasDeclaration
                 Self:•Eq     WhereTypeBoundDeclaration
                       Eq     TypeTraitBound                                                                                              */
	fn foo() {                                                                                                                            /*
    fn•foo()•{↲    <FunctionDeclaration>                                                                                                  */
        struct S;                                                                                                                         /*
        struct•S;    StructDeclaration                                                                                                    */
        impl S {                                                                                                                          /*
        impl•S•{↲    <ImplDeclaration>                                                                                                    */
            pub const X: u8 = 0;                                                                                                          /*
            pub•const•X:•u8•=•0;    ConstVariableDeclaration
            pub                     PubSpecifier
                              0     Literal                                                                                               */
            pub const fn bar() {}                                                                                                         /*
            pub•const•fn•bar()•{}    FunctionDeclaration
            pub                      PubSpecifier                                                                                         */
            async fn qux() {}                                                                                                             /*
            async•fn•qux()•{}    FunctionDeclaration                                                                                      */
        }                                                                                                                                 /*
••••••••}    </ImplDeclaration>                                                                                                           */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration>                                                                                                           */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */
impl X for () {}                                                                                                                          /*
impl•X•for•()•{}    ImplDeclaration
           ()       TypeTuple                                                                                                             */
impl X for Y {}                                                                                                                           /*
impl•X•for•Y•{}    ImplDeclaration                                                                                                        */
impl ! {}                                                                                                                                 /*
impl•!•{}    ImplDeclaration
     !       TypeNever                                                                                                                    */
impl ! where u8: A {}                                                                                                                     /*
impl•!•where•u8:•A•{}    ImplDeclaration
     !                   TypeNever
             u8:•A       WhereTypeBoundDeclaration
                 A       TypeTraitBound                                                                                                   */
impl !where u8: A {}                                                                                                                      /*
impl•!where•u8:•A•{}    ImplDeclaration
     !                  TypeNever
            u8:•A       WhereTypeBoundDeclaration
                A       TypeTraitBound                                                                                                    */
impl !Send for A { }                                                                                                                      /*
impl•!Send•for•A•{•}    NegativeImplDeclaration                                                                                           */
impl <*const u8>::C {}                                                                                                                    /*
impl•<*const•u8>::C•{}    ImplDeclaration
     <*const•u8>::C       TypePath
     <*const•u8>          ExpressionTypeSelector
      *const•u8           TypeDereferenceConst                                                                                            */
impl <A as B>::C {}                                                                                                                       /*
impl•<A•as•B>::C•{}    ImplDeclaration
     <A•as•B>::C       TypePath
     <A•as•B>          ExpressionTypeSelector                                                                                             */
impl <'a + B>::C {}                                                                                                                       /*
impl•<'a•+•B>::C•{}    ImplDeclaration
     <'a•+•B>::C       TypePath
     <'a•+•B>          ExpressionTypeSelector
      'a•+•B           TypeDynBounds
      'a               LtIdentifier
           B           TypeTraitBound                                                                                                     */
impl <<A>::B>::C {}                                                                                                                       /*
impl•<<A>::B>::C•{}    ImplDeclaration
     <<A>::B>::C       TypePath
     <<A>::B>          ExpressionTypeSelector
      <A>::B           TypePath
      <A>              ExpressionTypeSelector                                                                                             */
impl<'a, I, T: 'a, E> Iterator for Y<'a, I, E> where I: Iterator<Item = &'a (T, E)>,{}                                                    /*
impl<'a,•I,•T:•'a,•E>•Iterator•for•Y<'a,•I,•E>•where•I:•Iterator<Item•=•&'a•(T,•E)>,{}    ImplDeclaration
     'a                                                                                   GenericLtParameterDeclaration, LtIdentifier
         I                                                                                GenericTypeParameterDeclaration
            T:•'a                                                                         GenericTypeParameterDeclaration
               'a                                                                         LtIdentifier
                   E                                                                      GenericTypeParameterDeclaration
                                   Y<'a,•I,•E>                                            TypeCall
                                     'a                                                   LtIdentifier
                                                     I:•Iterator<Item•=•&'a•(T,•E)>       WhereTypeBoundDeclaration
                                                        Iterator<Item•=•&'a•(T,•E)>       TypeTraitBound, TypeCall
                                                                 Item•=•&'a•(T,•E)        TypeCallNamedArgument
                                                                        &'a•(T,•E)        TypeReference
                                                                         'a               LtIdentifier
                                                                            (T,•E)        TypeTuple                                       */
impl<T: Q> S for E<T> {}                                                                                                                  /*
impl<T:•Q>•S•for•E<T>•{}    ImplDeclaration
     T:•Q                   GenericTypeParameterDeclaration
        Q                   TypeTraitBound
                 E<T>       TypeCall                                                                                                      */
unsafe impl A for isize {}                                                                                                                /*
unsafe•impl•A•for•isize•{}    ImplDeclaration                                                                                             */
unsafe impl Send for A {}                                                                                                                 /*
unsafe•impl•Send•for•A•{}    ImplDeclaration                                                                                              */
unsafe impl Sync for A {}                                                                                                                 /*
unsafe•impl•Sync•for•A•{}    ImplDeclaration                                                                                              */
impl<'a> A for &'a [isize] {}                                                                                                             /*
impl<'a>•A•for•&'a•[isize]•{}    ImplDeclaration
     'a                          GenericLtParameterDeclaration, LtIdentifier
               &'a•[isize]       TypeReference
                'a               LtIdentifier
                   [isize]       TypeSlice                                                                                                */
impl ::A::B for ::C {}                                                                                                                    /*
impl•::A::B•for•::C•{}    ImplDeclaration
     ::A::B               TypePath
     ::A                  TypePath
                ::C       TypePath                                                                                                        */
impl ::A for () {}                                                                                                                        /*
impl•::A•for•()•{}    ImplDeclaration
     ::A              TypePath
             ()       TypeTuple                                                                                                           */
impl ::A {}                                                                                                                               /*
impl•::A•{}    ImplDeclaration
     ::A       TypePath                                                                                                                   */
impl A for [B; 1] {}                                                                                                                      /*
impl•A•for•[B;•1]•{}    ImplDeclaration
           [B;•1]       TypeSizedArray
               1        Literal                                                                                                           */
impl A for (<B as C>::D, E) {}                                                                                                            /*
impl•A•for•(<B•as•C>::D,•E)•{}    ImplDeclaration
           (<B•as•C>::D,•E)       TypeTuple
            <B•as•C>::D           TypePath
            <B•as•C>              ExpressionTypeSelector                                                                                  */
impl ::A for [B; 0] {}                                                                                                                    /*
impl•::A•for•[B;•0]•{}    ImplDeclaration
     ::A                  TypePath
             [B;•0]       TypeSizedArray
                 0        Literal                                                                                                         */
impl<'a> A for &'a [isize] {}                                                                                                             /*
impl<'a>•A•for•&'a•[isize]•{}    ImplDeclaration
     'a                          GenericLtParameterDeclaration, LtIdentifier
               &'a•[isize]       TypeReference
                'a               LtIdentifier
                   [isize]       TypeSlice                                                                                                */
impl<'a> dyn T+'a {}                                                                                                                      /*
impl<'a>•dyn•T+'a•{}    ImplDeclaration
     'a                 GenericLtParameterDeclaration, LtIdentifier
         dyn•T+'a       TypeDynBounds
             T          TypeTraitBound
               'a       LtIdentifier                                                                                                      */
impl<'a> dyn T + 'a {}                                                                                                                    /*
impl<'a>•dyn•T•+•'a•{}    ImplDeclaration
     'a                   GenericLtParameterDeclaration, LtIdentifier
         dyn•T•+•'a       TypeDynBounds
             T            TypeTraitBound
                 'a       LtIdentifier                                                                                                    */
impl<'a> dyn (::Foo::Trait) + 'a {}                                                                                                       /*
impl<'a>•dyn•(::Foo::Trait)•+•'a•{}    ImplDeclaration
     'a                                GenericLtParameterDeclaration, LtIdentifier
         dyn•(::Foo::Trait)•+•'a       TypeDynBounds
              ::Foo::Trait             TypeTraitBound, TypePath
              ::Foo                    TypePath
                              'a       LtIdentifier                                                                                       */
impl<T:?Sized> A for T { }                                                                                                                /*
impl<T:?Sized>•A•for•T•{•}    ImplDeclaration
     T:?Sized                 GenericTypeParameterDeclaration
       ?Sized                 TypeTraitBound                                                                                              */
impl<'a,'b,'c> S for &'a &'b &'c Q {}                                                                                                     /*
impl<'a,'b,'c>•S•for•&'a•&'b•&'c•Q•{}    ImplDeclaration
     'a                                  GenericLtParameterDeclaration, LtIdentifier
        'b                               GenericLtParameterDeclaration, LtIdentifier
           'c                            GenericLtParameterDeclaration, LtIdentifier
                     &'a•&'b•&'c•Q       TypeReference
                      'a                 LtIdentifier
                         &'b•&'c•Q       TypeReference
                          'b             LtIdentifier
                             &'c•Q       TypeReference
                              'c         LtIdentifier                                                                                     */
impl<A,F:Fn(A)> Foo<A> for F {}                                                                                                           /*
impl<A,F:Fn(A)>•Foo<A>•for•F•{}    ImplDeclaration
     A                             GenericTypeParameterDeclaration
       F:Fn(A)                     GenericTypeParameterDeclaration
         Fn(A)                     TypeTraitBound, TypeFunction
                Foo<A>             TypeCall                                                                                               */
impl<T> A for T where for<'a> T: B<'a> {}                                                                                                 /*
impl<T>•A•for•T•where•for<'a>•T:•B<'a>•{}    ImplDeclaration
     T                                       GenericTypeParameterDeclaration
                      for<'a>•T:•B<'a>       WhereTypeBoundDeclaration
                          'a                 GenericLtParameterDeclaration, LtIdentifier
                                 B<'a>       TypeTraitBound, TypeCall
                                   'a        LtIdentifier                                                                                 */
impl<'a, T, const N: usize> IntoIterator for &'a Table<T, N> {}                                                                           /*
impl<'a,•T,•const•N:•usize>•IntoIterator•for•&'a•Table<T,•N>•{}    ImplDeclaration
     'a                                                            GenericLtParameterDeclaration, LtIdentifier
         T                                                         GenericTypeParameterDeclaration
            const•N:•usize                                         ConstTypeParameterDeclaration
                                             &'a•Table<T,•N>       TypeReference
                                              'a                   LtIdentifier
                                                 Table<T,•N>       TypeCall                                                               */
impl A for (B,) {}                                                                                                                        /*
impl•A•for•(B,)•{}    ImplDeclaration
           (B,)       TypeTuple                                                                                                           */
impl !A for (B,) {}                                                                                                                       /*
impl•!A•for•(B,)•{}    NegativeImplDeclaration
            (B,)       TypeTuple                                                                                                          */
impl A for Box<C> {}                                                                                                                      /*
impl•A•for•Box<C>•{}    ImplDeclaration
           Box<C>       TypeCall                                                                                                          */
impl A for lib::Something<C> {}                                                                                                           /*
impl•A•for•lib::Something<C>•{}    ImplDeclaration
           lib::Something<C>       TypeCall
           lib::Something          TypePath                                                                                               */
impl A for D<C> {}                                                                                                                        /*
impl•A•for•D<C>•{}    ImplDeclaration
           D<C>       TypeCall                                                                                                            */
impl<T: ::std::fmt::Display> A<T> {}                                                                                                      /*
impl<T:•::std::fmt::Display>•A<T>•{}    ImplDeclaration
     T:•::std::fmt::Display             GenericTypeParameterDeclaration
        ::std::fmt::Display             TypeTraitBound, TypePath
        ::std::fmt                      TypePath
        ::std                           TypePath
                             A<T>       TypeCall                                                                                          */

// Discarded Nodes: 1
// Parsed Nodes: 318
// state_rollbacks: 34
// Total '.charCodeAt()' calls: 1975 (53% re-reads)
// Unnecessary 'skip_whitespace()' calls: 169
// source: "../../samples/statements/impl.rs"