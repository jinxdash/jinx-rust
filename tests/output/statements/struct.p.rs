
struct S<                                                                                                                                 /*
struct•S<↲    <StructDeclaration>                                                                                                         */
    A: ?for<'a> Q,                                                                                                                        /*
    A:•?for<'a>•Q     GenericTypeParameterDeclaration
       ?for<'a>•Q     TypeTraitBound
            'a        GenericLtParameterDeclaration, LtIdentifier                                                                         */
    B: 'a + Q,                                                                                                                            /*
    B:•'a•+•Q     GenericTypeParameterDeclaration
       'a         LtIdentifier
            Q     TypeTraitBound                                                                                                          */
    C: 'a,                                                                                                                                /*
    C:•'a     GenericTypeParameterDeclaration
       'a     LtIdentifier                                                                                                                */
    G: Q + 'a,                                                                                                                            /*
    G:•Q•+•'a     GenericTypeParameterDeclaration
       Q          TypeTraitBound
           'a     LtIdentifier                                                                                                            */
    H: Q +,                                                                                                                               /*
    H:•Q•+     GenericTypeParameterDeclaration
       Q       TypeTraitBound                                                                                                             */
    I:,                                                                                                                                   /*
    I:     GenericTypeParameterDeclaration                                                                                                */
>;                                                                                                                                        /*
>;    </StructDeclaration>                                                                                                                */
struct Empty1 {}                                                                                                                          /*
struct•Empty1•{}    StructDeclaration                                                                                                     */
struct Empty2;                                                                                                                            /*
struct•Empty2;    StructDeclaration                                                                                                       */
struct Empty7();                                                                                                                          /*
struct•Empty7();    TupleStructDeclaration                                                                                                */
struct Align8Many {                                                                                                                       /*
struct•Align8Many•{↲    <StructDeclaration>                                                                                               */
    a: i32,                                                                                                                               /*
    a:•i32     StructPropertyDeclaration                                                                                                  */
    b: i32,                                                                                                                               /*
    b:•i32     StructPropertyDeclaration                                                                                                  */
    c: i32,                                                                                                                               /*
    c:•i32     StructPropertyDeclaration                                                                                                  */
    d: u8,                                                                                                                                /*
    d:•u8     StructPropertyDeclaration                                                                                                   */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */
struct A<T>([T]);                                                                                                                         /*
struct•A<T>([T]);    TupleStructDeclaration
         T           GenericTypeParameterDeclaration
            [T]      TupleStructItemDeclaration, TypeSlice                                                                                */
struct A<T>(T);                                                                                                                           /*
struct•A<T>(T);    TupleStructDeclaration
         T         GenericTypeParameterDeclaration
            T      TupleStructItemDeclaration                                                                                             */
struct cat {                                                                                                                              /*
struct•cat•{↲    <StructDeclaration>                                                                                                      */
	done : extern "C" fn(usize),                                                                                                          /*
    done•:•extern•"C"•fn(usize)    StructPropertyDeclaration
           extern•"C"•fn(usize)    TypeFnPointer
           extern•"C"              ExternSpecifier
                  "C"              Literal
                         usize     TypeFnPointerParameter                                                                                 */
	meows : usize,                                                                                                                        /*
    meows•:•usize    StructPropertyDeclaration                                                                                            */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */
struct Test3<'a, 'b, 'c> {                                                                                                                /*
struct•Test3<'a,•'b,•'c>•{↲    <StructDeclaration>
             'a                GenericLtParameterDeclaration, LtIdentifier
                 'b            GenericLtParameterDeclaration, LtIdentifier
                     'c        GenericLtParameterDeclaration, LtIdentifier                                                                */
    x: extern "Rust" fn(&'a isize),                                                                                                       /*
    x:•extern•"Rust"•fn(&'a•isize)     StructPropertyDeclaration
       extern•"Rust"•fn(&'a•isize)     TypeFnPointer
       extern•"Rust"                   ExternSpecifier
              "Rust"                   Literal
                        &'a•isize      TypeFnPointerParameter, TypeReference
                         'a            LtIdentifier                                                                                       */
    y: extern "Rust" fn(&'b [isize]),                                                                                                     /*
    y:•extern•"Rust"•fn(&'b•[isize])     StructPropertyDeclaration
       extern•"Rust"•fn(&'b•[isize])     TypeFnPointer
       extern•"Rust"                     ExternSpecifier
              "Rust"                     Literal
                        &'b•[isize]      TypeFnPointerParameter, TypeReference
                         'b              LtIdentifier
                            [isize]      TypeSlice                                                                                        */
    c: extern "Rust" fn(&'c str),                                                                                                         /*
    c:•extern•"Rust"•fn(&'c•str)     StructPropertyDeclaration
       extern•"Rust"•fn(&'c•str)     TypeFnPointer
       extern•"Rust"                 ExternSpecifier
              "Rust"                 Literal
                        &'c•str      TypeFnPointerParameter, TypeReference
                         'c          LtIdentifier                                                                                         */
    a: fn(u32) -> u32,                                                                                                                    /*
    a:•fn(u32)•->•u32     StructPropertyDeclaration
       fn(u32)•->•u32     TypeFnPointer
          u32             TypeFnPointerParameter                                                                                          */
    b: extern "C" fn(u32) -> u32,                                                                                                         /*
    b:•extern•"C"•fn(u32)•->•u32     StructPropertyDeclaration
       extern•"C"•fn(u32)•->•u32     TypeFnPointer
       extern•"C"                    ExternSpecifier
              "C"                    Literal
                     u32             TypeFnPointerParameter                                                                               */
    c: unsafe fn(u32) -> u32,                                                                                                             /*
    c:•unsafe•fn(u32)•->•u32     StructPropertyDeclaration
       unsafe•fn(u32)•->•u32     TypeFnPointer
                 u32             TypeFnPointerParameter                                                                                   */
    d: unsafe extern "C" fn(u32) -> u32                                                                                                   /*
    d:•unsafe•extern•"C"•fn(u32)•->•u32    StructPropertyDeclaration
       unsafe•extern•"C"•fn(u32)•->•u32    TypeFnPointer
              extern•"C"                   ExternSpecifier
                     "C"                   Literal
                            u32            TypeFnPointerParameter                                                                         */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */
struct Test4<'a, 'b:'a> {                                                                                                                 /*
struct•Test4<'a,•'b:'a>•{↲    <StructDeclaration>
             'a               GenericLtParameterDeclaration, LtIdentifier
                 'b:'a        GenericLtParameterDeclaration
                 'b           LtIdentifier
                    'a        LtIdentifier                                                                                                */
    x: &'a mut &'b isize,                                                                                                                 /*
    x:•&'a•mut•&'b•isize     StructPropertyDeclaration
       &'a•mut•&'b•isize     TypeReference
        'a                   LtIdentifier
               &'b•isize     TypeReference
                'b           LtIdentifier                                                                                                 */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */
struct Test6<'a, 'b:'a> {                                                                                                                 /*
struct•Test6<'a,•'b:'a>•{↲    <StructDeclaration>
             'a               GenericLtParameterDeclaration, LtIdentifier
                 'b:'a        GenericLtParameterDeclaration
                 'b           LtIdentifier
                    'a        LtIdentifier                                                                                                */
    x: &'a mut extern "Rust" fn(&'b isize),                                                                                               /*
    x:•&'a•mut•extern•"Rust"•fn(&'b•isize)     StructPropertyDeclaration
       &'a•mut•extern•"Rust"•fn(&'b•isize)     TypeReference
        'a                                     LtIdentifier
               extern•"Rust"•fn(&'b•isize)     TypeFnPointer
               extern•"Rust"                   ExternSpecifier
                      "Rust"                   Literal
                                &'b•isize      TypeFnPointerParameter, TypeReference
                                 'b            LtIdentifier                                                                               */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */
struct Foo<'a> {                                                                                                                          /*
struct•Foo<'a>•{↲    <StructDeclaration>
           'a        GenericLtParameterDeclaration, LtIdentifier                                                                          */
    x: Box<dyn Fn(i32) -> &'a i32 + 'static>                                                                                              /*
    x:•Box<dyn•Fn(i32)•->•&'a•i32•+•'static>    StructPropertyDeclaration
       Box<dyn•Fn(i32)•->•&'a•i32•+•'static>    TypeCall
           dyn•Fn(i32)•->•&'a•i32•+•'static     TypeDynBounds
               Fn(i32)•->•&'a•i32               TypeTraitBound, TypeFunction
                          &'a•i32               TypeReference
                           'a                   LtIdentifier
                                    'static     LtStatic                                                                                  */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */
struct X;                                                                                                                                 /*
struct•X;    StructDeclaration                                                                                                            */
struct U {}                                                                                                                               /*
struct•U•{}    StructDeclaration                                                                                                          */
struct P<T>(T);                                                                                                                           /*
struct•P<T>(T);    TupleStructDeclaration
         T         GenericTypeParameterDeclaration
            T      TupleStructItemDeclaration                                                                                             */
struct A<U> where U: E(U);                                                                                                                /*
struct•A<U>•where•U:•E(U);    StructDeclaration
         U                    GenericTypeParameterDeclaration
                  U:•E(U)     WhereTypeBoundDeclaration
                     E(U)     TypeTraitBound, TypeFunction                                                                                */
struct A<U> where U: E(U) -> R;                                                                                                           /*
struct•A<U>•where•U:•E(U)•->•R;    StructDeclaration
         U                         GenericTypeParameterDeclaration
                  U:•E(U)•->•R     WhereTypeBoundDeclaration
                     E(U)•->•R     TypeTraitBound, TypeFunction                                                                           */
struct A<U>(U) where U: Eq;                                                                                                               /*
struct•A<U>(U)•where•U:•Eq;    TupleStructDeclaration
         U                     GenericTypeParameterDeclaration
            U                  TupleStructItemDeclaration
                     U:•Eq     WhereTypeBoundDeclaration
                        Eq     TypeTraitBound                                                                                             */
struct K<'a>(&'a ());                                                                                                                     /*
struct•K<'a>(&'a•());    TupleStructDeclaration
         'a              GenericLtParameterDeclaration, LtIdentifier
             &'a•()      TupleStructItemDeclaration, TypeReference
              'a         LtIdentifier
                 ()      TypeTuple                                                                                                        */
pub struct A([u8; 1]);                                                                                                                    /*
pub•struct•A([u8;•1]);    TupleStructDeclaration
pub                       PubSpecifier
             [u8;•1]      TupleStructItemDeclaration, TypeSizedArray
                  1       Literal                                                                                                         */
pub(crate) struct S<'a, I, E>(I, &'a E);                                                                                                  /*
pub(crate)•struct•S<'a,•I,•E>(I,•&'a•E);    TupleStructDeclaration
pub(crate)                                  PubSpecifier
                    'a                      GenericLtParameterDeclaration, LtIdentifier
                        I                   GenericTypeParameterDeclaration
                           E                GenericTypeParameterDeclaration
                              I             TupleStructItemDeclaration
                                 &'a•E      TupleStructItemDeclaration, TypeReference
                                  'a        LtIdentifier                                                                                  */
pub struct A<T: B>(C);                                                                                                                    /*
pub•struct•A<T:•B>(C);    TupleStructDeclaration
pub                       PubSpecifier
             T:•B         GenericTypeParameterDeclaration
                B         TypeTraitBound
                   C      TupleStructItemDeclaration                                                                                      */
pub struct A<T = u8>(T);                                                                                                                  /*
pub•struct•A<T•=•u8>(T);    TupleStructDeclaration
pub                         PubSpecifier
             T•=•u8         GenericTypeParameterDeclaration
                     T      TupleStructItemDeclaration                                                                                    */
pub struct Table<T, const N: usize>([Option<T>; N]);                                                                                      /*
pub•struct•Table<T,•const•N:•usize>([Option<T>;•N]);    TupleStructDeclaration
pub                                                     PubSpecifier
                 T                                      GenericTypeParameterDeclaration
                    const•N:•usize                      ConstTypeParameterDeclaration
                                    [Option<T>;•N]      TupleStructItemDeclaration, TypeSizedArray
                                     Option<T>          TypeCall                                                                          */
struct B;                                                                                                                                 /*
struct•B;    StructDeclaration                                                                                                            */
struct A<T: ?Sized>(C<T>, ());                                                                                                            /*
struct•A<T:•?Sized>(C<T>,•());    TupleStructDeclaration
         T:•?Sized                GenericTypeParameterDeclaration
            ?Sized                TypeTraitBound
                    C<T>          TupleStructItemDeclaration, TypeCall
                          ()      TupleStructItemDeclaration, TypeTuple                                                                   */
struct A<T = i32, U = i32>(B<T, U>) where B<T, U>: Marker;                                                                                /*
struct•A<T•=•i32,•U•=•i32>(B<T,•U>)•where•B<T,•U>:•Marker;    TupleStructDeclaration
         T•=•i32                                              GenericTypeParameterDeclaration
                  U•=•i32                                     GenericTypeParameterDeclaration
                           B<T,•U>                            TupleStructItemDeclaration, TypeCall
                                          B<T,•U>:•Marker     WhereTypeBoundDeclaration
                                          B<T,•U>             TypeCall
                                                   Marker     TypeTraitBound                                                              */
struct A<T = u32, U = i32>(T, U) where B<T, U>: Marker;                                                                                   /*
struct•A<T•=•u32,•U•=•i32>(T,•U)•where•B<T,•U>:•Marker;    TupleStructDeclaration
         T•=•u32                                           GenericTypeParameterDeclaration
                  U•=•i32                                  GenericTypeParameterDeclaration
                           T                               TupleStructItemDeclaration
                              U                            TupleStructItemDeclaration
                                       B<T,•U>:•Marker     WhereTypeBoundDeclaration
                                       B<T,•U>             TypeCall
                                                Marker     TypeTraitBound                                                                 */
struct A<'a, S: B<'a> = i32>(S, &'a ());                                                                                                  /*
struct•A<'a,•S:•B<'a>•=•i32>(S,•&'a•());    TupleStructDeclaration
         'a                                 GenericLtParameterDeclaration, LtIdentifier
             S:•B<'a>•=•i32                 GenericTypeParameterDeclaration
                B<'a>                       TypeTraitBound, TypeCall
                  'a                        LtIdentifier
                             S              TupleStructItemDeclaration
                                &'a•()      TupleStructItemDeclaration, TypeReference
                                 'a         LtIdentifier
                                    ()      TypeTuple                                                                                     */
struct S1(pub(in foo) (), pub(T), pub(crate) (), pub(((), T)));                                                                           /*
struct•S1(pub(in•foo)•(),•pub(T),•pub(crate)•(),•pub(((),•T)));    TupleStructDeclaration
          pub(in•foo)•()                                           TupleStructItemDeclaration
          pub(in•foo)                                              PubSpecifier
                      ()                                           TypeTuple
                          pub(T)                                   TupleStructItemDeclaration
                          pub                                      PubSpecifier
                                  pub(crate)•()                    TupleStructItemDeclaration
                                  pub(crate)                       PubSpecifier
                                             ()                    TypeTuple
                                                 pub(((),•T))      TupleStructItemDeclaration
                                                 pub               PubSpecifier
                                                     ((),•T)       TypeTuple
                                                      ()           TypeTuple                                                              */
struct G<T, U>(*const T, *const U);                                                                                                       /*
struct•G<T,•U>(*const•T,•*const•U);    TupleStructDeclaration
         T                             GenericTypeParameterDeclaration
            U                          GenericTypeParameterDeclaration
               *const•T                TupleStructItemDeclaration, TypeDereferenceConst
                         *const•U      TupleStructItemDeclaration, TypeDereferenceConst                                                   */
pub struct Unique<T:?Sized> { s: *const T, }                                                                                              /*
pub•struct•Unique<T:?Sized>•{•s:•*const•T,•}    StructDeclaration
pub                                             PubSpecifier
                  T:?Sized                      GenericTypeParameterDeclaration
                    ?Sized                      TypeTraitBound
                              s:•*const•T       StructPropertyDeclaration
                                 *const•T       TypeDereferenceConst                                                                      */
unsafe impl<T: Send + ?Sized> Send for Unique<T> { }                                                                                      /*
unsafe•impl<T:•Send•+•?Sized>•Send•for•Unique<T>•{•}    ImplDeclaration
            T:•Send•+•?Sized                            GenericTypeParameterDeclaration
               Send                                     TypeTraitBound
                      ?Sized                            TypeTraitBound
                                       Unique<T>        TypeCall                                                                          */
pub struct A(u32, ::b::Q);                                                                                                                /*
pub•struct•A(u32,•::b::Q);    TupleStructDeclaration
pub                           PubSpecifier
             u32              TupleStructItemDeclaration
                  ::b::Q      TupleStructItemDeclaration, TypePath
                  ::b         TypePath                                                                                                    */
struct S(<AT as A<DT>>::AS);                                                                                                              /*
struct•S(<AT•as•A<DT>>::AS);    TupleStructDeclaration
         <AT•as•A<DT>>::AS      TupleStructItemDeclaration, TypePath
         <AT•as•A<DT>>          ExpressionTypeSelector
                A<DT>           TypeCall                                                                                                  */
pub struct A<I> where I: B<C: D>, { w: <I::E as F>::G, }                                                                                  /*
pub•struct•A<I>•where•I:•B<C:•D>,•{•w:•<I::E•as•F>::G,•}    StructDeclaration
pub                                                         PubSpecifier
             I                                              GenericTypeParameterDeclaration
                      I:•B<C:•D>                            WhereTypeBoundDeclaration
                         B<C:•D>                            TypeTraitBound, TypeCall
                           C:•D                             TypeCallNamedBound
                              D                             TypeTraitBound
                                    w:•<I::E•as•F>::G       StructPropertyDeclaration
                                       <I::E•as•F>::G       TypePath
                                       <I::E•as•F>          ExpressionTypeSelector
                                        I::E                TypePath                                                                      */

// Discarded Nodes: 2
// Parsed Nodes: 455
// state_rollbacks: 12
// Total '.charCodeAt()' calls: 2244 (41% re-reads)
// Unnecessary 'skip_whitespace()' calls: 348
// source: "../../samples/statements/struct.rs"