
struct S<                                                                                                                                 /*
struct•S<↲    <Program>
struct•S<↲    <Program.ast{dk: "None"}>
struct•S<↲    <StructDeclaration>
        <↲    <StructDeclaration.generics{dk: "<>"}>                                                                                      */
    A: ?for<'a> Q,                                                                                                                        /*
    A:•?for<'a>•Q    GenericTypeParameterDeclaration
       ?for<'a>•Q    TypeTraitBound{!maybeConst, optional}
        for<'a>      TypeTraitBound.ltParameters{dk: "<>"}
            'a       GenericLtParameterDeclaration, LtIdentifier                                                                          */
    B: 'a + Q,                                                                                                                            /*
    B:•'a•+•Q    GenericTypeParameterDeclaration
       'a        LtIdentifier
            Q    TypeTraitBound{!maybeConst, !optional}                                                                                   */
    C: 'a,                                                                                                                                /*
    C:•'a    GenericTypeParameterDeclaration
       'a    LtIdentifier                                                                                                                 */
    G: Q + 'a,                                                                                                                            /*
    G:•Q•+•'a    GenericTypeParameterDeclaration
       Q         TypeTraitBound{!maybeConst, !optional}
           'a    LtIdentifier                                                                                                             */
    H: Q +,                                                                                                                               /*
    H:•Q•+    GenericTypeParameterDeclaration
       Q      TypeTraitBound{!maybeConst, !optional}                                                                                      */
    I:,                                                                                                                                   /*
    I:    GenericTypeParameterDeclaration                                                                                                 */
>;                                                                                                                                        /*
>     </StructDeclaration.generics>
>;    </StructDeclaration>                                                                                                                */
struct Empty1 {}                                                                                                                          /*
struct•Empty1•{}    StructDeclaration
              {}    StructDeclaration.properties{dk: "{}"}                                                                                */
struct Empty2;                                                                                                                            /*
struct•Empty2;    StructDeclaration                                                                                                       */
struct Empty7();                                                                                                                          /*
struct•Empty7();    TupleStructDeclaration
             ()     TupleStructDeclaration.items{dk: "()"}                                                                                */
struct Align8Many {                                                                                                                       /*
struct•Align8Many•{↲    <StructDeclaration>
                  {↲    <StructDeclaration.properties{dk: "{}"}>                                                                          */
    a: i32,                                                                                                                               /*
    a:•i32    StructPropertyDeclaration                                                                                                   */
    b: i32,                                                                                                                               /*
    b:•i32    StructPropertyDeclaration                                                                                                   */
    c: i32,                                                                                                                               /*
    c:•i32    StructPropertyDeclaration                                                                                                   */
    d: u8,                                                                                                                                /*
    d:•u8    StructPropertyDeclaration                                                                                                    */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */
struct A<T>([T]);                                                                                                                         /*
struct•A<T>([T]);    TupleStructDeclaration
        <T>          TupleStructDeclaration.generics{dk: "<>"}
         T           GenericTypeParameterDeclaration
           ([T])     TupleStructDeclaration.items{dk: "()"}
            [T]      TupleStructItemDeclaration, TypeSlice                                                                                */
struct A<T>(T);                                                                                                                           /*
struct•A<T>(T);    TupleStructDeclaration
        <T>        TupleStructDeclaration.generics{dk: "<>"}
         T         GenericTypeParameterDeclaration
           (T)     TupleStructDeclaration.items{dk: "()"}
            T      TupleStructItemDeclaration                                                                                             */
struct cat {                                                                                                                              /*
struct•cat•{↲    <StructDeclaration>
           {↲    <StructDeclaration.properties{dk: "{}"}>                                                                                 */
	done : extern "C" fn(usize),                                                                                                          /*
	done•:•extern•"C"•fn(usize)    StructPropertyDeclaration
	       extern•"C"•fn(usize)    TypeFnPointer
	       extern•"C"              ExternSpecifier
	              "C"              Literal{kind: String}
	                    (usize)    TypeFnPointer.parameters{dk: "()"}
	                     usize     TypeFnPointerParameter                                                                                 */
	meows : usize,                                                                                                                        /*
	meows•:•usize    StructPropertyDeclaration                                                                                            */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */
struct Test3<'a, 'b, 'c> {                                                                                                                /*
struct•Test3<'a,•'b,•'c>•{↲    <StructDeclaration>
            <'a,•'b,•'c>       StructDeclaration.generics{dk: "<>"}
             'a                GenericLtParameterDeclaration, LtIdentifier
                 'b            GenericLtParameterDeclaration, LtIdentifier
                     'c        GenericLtParameterDeclaration, LtIdentifier
                         {↲    <StructDeclaration.properties{dk: "{}"}>                                                                   */
    x: extern "Rust" fn(&'a isize),                                                                                                       /*
    x:•extern•"Rust"•fn(&'a•isize)    StructPropertyDeclaration
       extern•"Rust"•fn(&'a•isize)    TypeFnPointer
       extern•"Rust"                  ExternSpecifier
              "Rust"                  Literal{kind: String}
                       (&'a•isize)    TypeFnPointer.parameters{dk: "()"}
                        &'a•isize     TypeFnPointerParameter, TypeReference{!mut}
                         'a           LtIdentifier                                                                                        */
    y: extern "Rust" fn(&'b [isize]),                                                                                                     /*
    y:•extern•"Rust"•fn(&'b•[isize])    StructPropertyDeclaration
       extern•"Rust"•fn(&'b•[isize])    TypeFnPointer
       extern•"Rust"                    ExternSpecifier
              "Rust"                    Literal{kind: String}
                       (&'b•[isize])    TypeFnPointer.parameters{dk: "()"}
                        &'b•[isize]     TypeFnPointerParameter, TypeReference{!mut}
                         'b             LtIdentifier
                            [isize]     TypeSlice                                                                                         */
    c: extern "Rust" fn(&'c str),                                                                                                         /*
    c:•extern•"Rust"•fn(&'c•str)    StructPropertyDeclaration
       extern•"Rust"•fn(&'c•str)    TypeFnPointer
       extern•"Rust"                ExternSpecifier
              "Rust"                Literal{kind: String}
                       (&'c•str)    TypeFnPointer.parameters{dk: "()"}
                        &'c•str     TypeFnPointerParameter, TypeReference{!mut}
                         'c         LtIdentifier                                                                                          */
    a: fn(u32) -> u32,                                                                                                                    /*
    a:•fn(u32)•->•u32    StructPropertyDeclaration
       fn(u32)•->•u32    TypeFnPointer
         (u32)           TypeFnPointer.parameters{dk: "()"}
          u32            TypeFnPointerParameter                                                                                           */
    b: extern "C" fn(u32) -> u32,                                                                                                         /*
    b:•extern•"C"•fn(u32)•->•u32    StructPropertyDeclaration
       extern•"C"•fn(u32)•->•u32    TypeFnPointer
       extern•"C"                   ExternSpecifier
              "C"                   Literal{kind: String}
                    (u32)           TypeFnPointer.parameters{dk: "()"}
                     u32            TypeFnPointerParameter                                                                                */
    c: unsafe fn(u32) -> u32,                                                                                                             /*
    c:•unsafe•fn(u32)•->•u32    StructPropertyDeclaration
       unsafe•fn(u32)•->•u32    TypeFnPointer{unsafe}
                (u32)           TypeFnPointer.parameters{dk: "()"}
                 u32            TypeFnPointerParameter                                                                                    */
    d: unsafe extern "C" fn(u32) -> u32                                                                                                   /*
    d:•unsafe•extern•"C"•fn(u32)•->•u32    StructPropertyDeclaration
       unsafe•extern•"C"•fn(u32)•->•u32    TypeFnPointer{unsafe}
              extern•"C"                   ExternSpecifier
                     "C"                   Literal{kind: String}
                           (u32)           TypeFnPointer.parameters{dk: "()"}
                            u32            TypeFnPointerParameter                                                                         */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */
struct Test4<'a, 'b:'a> {                                                                                                                 /*
struct•Test4<'a,•'b:'a>•{↲    <StructDeclaration>
            <'a,•'b:'a>       StructDeclaration.generics{dk: "<>"}
             'a               GenericLtParameterDeclaration, LtIdentifier
                 'b:'a        GenericLtParameterDeclaration
                 'b           LtIdentifier
                    'a        LtIdentifier
                        {↲    <StructDeclaration.properties{dk: "{}"}>                                                                    */
    x: &'a mut &'b isize,                                                                                                                 /*
    x:•&'a•mut•&'b•isize    StructPropertyDeclaration
       &'a•mut•&'b•isize    TypeReference{mut}
        'a                  LtIdentifier
               &'b•isize    TypeReference{!mut}
                'b          LtIdentifier                                                                                                  */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */
struct Test6<'a, 'b:'a> {                                                                                                                 /*
struct•Test6<'a,•'b:'a>•{↲    <StructDeclaration>
            <'a,•'b:'a>       StructDeclaration.generics{dk: "<>"}
             'a               GenericLtParameterDeclaration, LtIdentifier
                 'b:'a        GenericLtParameterDeclaration
                 'b           LtIdentifier
                    'a        LtIdentifier
                        {↲    <StructDeclaration.properties{dk: "{}"}>                                                                    */
    x: &'a mut extern "Rust" fn(&'b isize),                                                                                               /*
    x:•&'a•mut•extern•"Rust"•fn(&'b•isize)    StructPropertyDeclaration
       &'a•mut•extern•"Rust"•fn(&'b•isize)    TypeReference{mut}
        'a                                    LtIdentifier
               extern•"Rust"•fn(&'b•isize)    TypeFnPointer
               extern•"Rust"                  ExternSpecifier
                      "Rust"                  Literal{kind: String}
                               (&'b•isize)    TypeFnPointer.parameters{dk: "()"}
                                &'b•isize     TypeFnPointerParameter, TypeReference{!mut}
                                 'b           LtIdentifier                                                                                */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */
struct Foo<'a> {                                                                                                                          /*
struct•Foo<'a>•{↲    <StructDeclaration>
          <'a>       StructDeclaration.generics{dk: "<>"}
           'a        GenericLtParameterDeclaration, LtIdentifier
               {↲    <StructDeclaration.properties{dk: "{}"}>                                                                             */
    x: Box<dyn Fn(i32) -> &'a i32 + 'static>                                                                                              /*
    x:•Box<dyn•Fn(i32)•->•&'a•i32•+•'static>    StructPropertyDeclaration
       Box<dyn•Fn(i32)•->•&'a•i32•+•'static>    TypeCall
          <dyn•Fn(i32)•->•&'a•i32•+•'static>    TypeCall.typeArguments{dk: "<>"}
           dyn•Fn(i32)•->•&'a•i32•+•'static     TypeDynBounds{dyn}
               Fn(i32)•->•&'a•i32               TypeTraitBound{!maybeConst, !optional}, TypeFunction
                 (i32)                          TypeFunction.parameters{dk: "()"}
                          &'a•i32               TypeReference{!mut}
                           'a                   LtIdentifier
                                    'static     LtStatic                                                                                  */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */
struct X;                                                                                                                                 /*
struct•X;    StructDeclaration                                                                                                            */
struct U {}                                                                                                                               /*
struct•U•{}    StructDeclaration
         {}    StructDeclaration.properties{dk: "{}"}                                                                                     */
struct P<T>(T);                                                                                                                           /*
struct•P<T>(T);    TupleStructDeclaration
        <T>        TupleStructDeclaration.generics{dk: "<>"}
         T         GenericTypeParameterDeclaration
           (T)     TupleStructDeclaration.items{dk: "()"}
            T      TupleStructItemDeclaration                                                                                             */
struct A<U> where U: E(U);                                                                                                                /*
struct•A<U>•where•U:•E(U);    StructDeclaration
        <U>                   StructDeclaration.generics{dk: "<>"}
         U                    GenericTypeParameterDeclaration
            where•U:•E(U)     StructDeclaration.whereBounds{dk: "None"}
                  U:•E(U)     WhereTypeBoundDeclaration
                     E(U)     TypeTraitBound{!maybeConst, !optional}, TypeFunction
                      (U)     TypeFunction.parameters{dk: "()"}                                                                           */
struct A<U> where U: E(U) -> R;                                                                                                           /*
struct•A<U>•where•U:•E(U)•->•R;    StructDeclaration
        <U>                        StructDeclaration.generics{dk: "<>"}
         U                         GenericTypeParameterDeclaration
            where•U:•E(U)•->•R     StructDeclaration.whereBounds{dk: "None"}
                  U:•E(U)•->•R     WhereTypeBoundDeclaration
                     E(U)•->•R     TypeTraitBound{!maybeConst, !optional}, TypeFunction
                      (U)          TypeFunction.parameters{dk: "()"}                                                                      */
struct A<U>(U) where U: Eq;                                                                                                               /*
struct•A<U>(U)•where•U:•Eq;    TupleStructDeclaration
        <U>                    TupleStructDeclaration.generics{dk: "<>"}
         U                     GenericTypeParameterDeclaration
           (U)                 TupleStructDeclaration.items{dk: "()"}
            U                  TupleStructItemDeclaration
               where•U:•Eq     TupleStructDeclaration.whereBounds{dk: "None"}
                     U:•Eq     WhereTypeBoundDeclaration
                        Eq     TypeTraitBound{!maybeConst, !optional}                                                                     */
struct K<'a>(&'a ());                                                                                                                     /*
struct•K<'a>(&'a•());    TupleStructDeclaration
        <'a>             TupleStructDeclaration.generics{dk: "<>"}
         'a              GenericLtParameterDeclaration, LtIdentifier
            (&'a•())     TupleStructDeclaration.items{dk: "()"}
             &'a•()      TupleStructItemDeclaration, TypeReference{!mut}
              'a         LtIdentifier
                 ()      TypeTuple                                                                                                        */
pub struct A([u8; 1]);                                                                                                                    /*
pub•struct•A([u8;•1]);    TupleStructDeclaration
pub                       PubSpecifier
            ([u8;•1])     TupleStructDeclaration.items{dk: "()"}
             [u8;•1]      TupleStructItemDeclaration, TypeSizedArray
                  1       Literal{kind: Integer}                                                                                          */
pub(crate) struct S<'a, I, E>(I, &'a E);                                                                                                  /*
pub(crate)•struct•S<'a,•I,•E>(I,•&'a•E);    TupleStructDeclaration
pub(crate)                                  PubSpecifier
                   <'a,•I,•E>               TupleStructDeclaration.generics{dk: "<>"}
                    'a                      GenericLtParameterDeclaration, LtIdentifier
                        I                   GenericTypeParameterDeclaration
                           E                GenericTypeParameterDeclaration
                             (I,•&'a•E)     TupleStructDeclaration.items{dk: "()"}
                              I             TupleStructItemDeclaration
                                 &'a•E      TupleStructItemDeclaration, TypeReference{!mut}
                                  'a        LtIdentifier                                                                                  */
pub struct A<T: B>(C);                                                                                                                    /*
pub•struct•A<T:•B>(C);    TupleStructDeclaration
pub                       PubSpecifier
            <T:•B>        TupleStructDeclaration.generics{dk: "<>"}
             T:•B         GenericTypeParameterDeclaration
                B         TypeTraitBound{!maybeConst, !optional}
                  (C)     TupleStructDeclaration.items{dk: "()"}
                   C      TupleStructItemDeclaration                                                                                      */
pub struct A<T = u8>(T);                                                                                                                  /*
pub•struct•A<T•=•u8>(T);    TupleStructDeclaration
pub                         PubSpecifier
            <T•=•u8>        TupleStructDeclaration.generics{dk: "<>"}
             T•=•u8         GenericTypeParameterDeclaration
                    (T)     TupleStructDeclaration.items{dk: "()"}
                     T      TupleStructItemDeclaration                                                                                    */
pub struct Table<T, const N: usize>([Option<T>; N]);                                                                                      /*
pub•struct•Table<T,•const•N:•usize>([Option<T>;•N]);    TupleStructDeclaration
pub                                                     PubSpecifier
                <T,•const•N:•usize>                     TupleStructDeclaration.generics{dk: "<>"}
                 T                                      GenericTypeParameterDeclaration
                    const•N:•usize                      ConstTypeParameterDeclaration
                                   ([Option<T>;•N])     TupleStructDeclaration.items{dk: "()"}
                                    [Option<T>;•N]      TupleStructItemDeclaration, TypeSizedArray
                                     Option<T>          TypeCall
                                           <T>          TypeCall.typeArguments{dk: "<>"}                                                  */
struct B;                                                                                                                                 /*
struct•B;    StructDeclaration                                                                                                            */
struct A<T: ?Sized>(C<T>, ());                                                                                                            /*
struct•A<T:•?Sized>(C<T>,•());    TupleStructDeclaration
        <T:•?Sized>               TupleStructDeclaration.generics{dk: "<>"}
         T:•?Sized                GenericTypeParameterDeclaration
            ?Sized                TypeTraitBound{!maybeConst, optional}
                   (C<T>,•())     TupleStructDeclaration.items{dk: "()"}
                    C<T>          TupleStructItemDeclaration, TypeCall
                     <T>          TypeCall.typeArguments{dk: "<>"}
                          ()      TupleStructItemDeclaration, TypeTuple                                                                   */
struct A<T = i32, U = i32>(B<T, U>) where B<T, U>: Marker;                                                                                /*
struct•A<T•=•i32,•U•=•i32>(B<T,•U>)•where•B<T,•U>:•Marker;    TupleStructDeclaration
        <T•=•i32,•U•=•i32>                                    TupleStructDeclaration.generics{dk: "<>"}
         T•=•i32                                              GenericTypeParameterDeclaration
                  U•=•i32                                     GenericTypeParameterDeclaration
                          (B<T,•U>)                           TupleStructDeclaration.items{dk: "()"}
                           B<T,•U>                            TupleStructItemDeclaration, TypeCall
                            <T,•U>                            TypeCall.typeArguments{dk: "<>"}
                                    where•B<T,•U>:•Marker     TupleStructDeclaration.whereBounds{dk: "None"}
                                          B<T,•U>:•Marker     WhereTypeBoundDeclaration
                                          B<T,•U>             TypeCall
                                           <T,•U>             TypeCall.typeArguments{dk: "<>"}
                                                   Marker     TypeTraitBound{!maybeConst, !optional}                                      */
struct A<T = u32, U = i32>(T, U) where B<T, U>: Marker;                                                                                   /*
struct•A<T•=•u32,•U•=•i32>(T,•U)•where•B<T,•U>:•Marker;    TupleStructDeclaration
        <T•=•u32,•U•=•i32>                                 TupleStructDeclaration.generics{dk: "<>"}
         T•=•u32                                           GenericTypeParameterDeclaration
                  U•=•i32                                  GenericTypeParameterDeclaration
                          (T,•U)                           TupleStructDeclaration.items{dk: "()"}
                           T                               TupleStructItemDeclaration
                              U                            TupleStructItemDeclaration
                                 where•B<T,•U>:•Marker     TupleStructDeclaration.whereBounds{dk: "None"}
                                       B<T,•U>:•Marker     WhereTypeBoundDeclaration
                                       B<T,•U>             TypeCall
                                        <T,•U>             TypeCall.typeArguments{dk: "<>"}
                                                Marker     TypeTraitBound{!maybeConst, !optional}                                         */
struct A<'a, S: B<'a> = i32>(S, &'a ());                                                                                                  /*
struct•A<'a,•S:•B<'a>•=•i32>(S,•&'a•());    TupleStructDeclaration
        <'a,•S:•B<'a>•=•i32>                TupleStructDeclaration.generics{dk: "<>"}
         'a                                 GenericLtParameterDeclaration, LtIdentifier
             S:•B<'a>•=•i32                 GenericTypeParameterDeclaration
                B<'a>                       TypeTraitBound{!maybeConst, !optional}, TypeCall
                 <'a>                       TypeCall.typeArguments{dk: "<>"}
                  'a                        LtIdentifier
                            (S,•&'a•())     TupleStructDeclaration.items{dk: "()"}
                             S              TupleStructItemDeclaration
                                &'a•()      TupleStructItemDeclaration, TypeReference{!mut}
                                 'a         LtIdentifier
                                    ()      TypeTuple                                                                                     */
struct S1(pub(in foo) (), pub(T), pub(crate) (), pub(((), T)));                                                                           /*
struct•S1(pub(in•foo)•(),•pub(T),•pub(crate)•(),•pub(((),•T)));    TupleStructDeclaration
         (pub(in•foo)•(),•pub(T),•pub(crate)•(),•pub(((),•T)))     TupleStructDeclaration.items{dk: "()"}
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
        <T,•U>                         TupleStructDeclaration.generics{dk: "<>"}
         T                             GenericTypeParameterDeclaration
            U                          GenericTypeParameterDeclaration
              (*const•T,•*const•U)     TupleStructDeclaration.items{dk: "()"}
               *const•T                TupleStructItemDeclaration, TypeDereferenceConst
                         *const•U      TupleStructItemDeclaration, TypeDereferenceConst                                                   */
pub struct Unique<T:?Sized> { s: *const T, }                                                                                              /*
pub•struct•Unique<T:?Sized>•{•s:•*const•T,•}    StructDeclaration
pub                                             PubSpecifier
                 <T:?Sized>                     StructDeclaration.generics{dk: "<>"}
                  T:?Sized                      GenericTypeParameterDeclaration
                    ?Sized                      TypeTraitBound{!maybeConst, optional}
                            {•s:•*const•T,•}    StructDeclaration.properties{dk: "{}"}
                              s:•*const•T       StructPropertyDeclaration
                                 *const•T       TypeDereferenceConst                                                                      */
unsafe impl<T: Send + ?Sized> Send for Unique<T> { }                                                                                      /*
unsafe•impl<T:•Send•+•?Sized>•Send•for•Unique<T>•{•}    ImplDeclaration{!const, unsafe}
           <T:•Send•+•?Sized>                           ImplDeclaration.generics{dk: "<>"}
            T:•Send•+•?Sized                            GenericTypeParameterDeclaration
               Send                                     TypeTraitBound{!maybeConst, !optional}
                      ?Sized                            TypeTraitBound{!maybeConst, optional}
                                       Unique<T>        TypeCall
                                             <T>        TypeCall.typeArguments{dk: "<>"}
                                                 {•}    ImplDeclaration.body{dk: "{}"}                                                    */
pub struct A(u32, ::b::Q);                                                                                                                /*
pub•struct•A(u32,•::b::Q);    TupleStructDeclaration
pub                           PubSpecifier
            (u32,•::b::Q)     TupleStructDeclaration.items{dk: "()"}
             u32              TupleStructItemDeclaration
                  ::b::Q      TupleStructItemDeclaration, TypePath
                  ::b         TypePath                                                                                                    */
struct S(<AT as A<DT>>::AS);                                                                                                              /*
struct•S(<AT•as•A<DT>>::AS);    TupleStructDeclaration
        (<AT•as•A<DT>>::AS)     TupleStructDeclaration.items{dk: "()"}
         <AT•as•A<DT>>::AS      TupleStructItemDeclaration, TypePath
         <AT•as•A<DT>>          ExpressionTypeSelector
                A<DT>           TypeCall
                 <DT>           TypeCall.typeArguments{dk: "<>"}                                                                          */
pub struct A<I> where I: B<C: D>, { w: <I::E as F>::G, }                                                                                  /*
pub•struct•A<I>•where•I:•B<C:•D>,•{•w:•<I::E•as•F>::G,•}    StructDeclaration
pub                                                         PubSpecifier
            <I>                                             StructDeclaration.generics{dk: "<>"}
             I                                              GenericTypeParameterDeclaration
                where•I:•B<C:•D>,                           StructDeclaration.whereBounds{dk: "None"}
                      I:•B<C:•D>                            WhereTypeBoundDeclaration
                         B<C:•D>                            TypeTraitBound{!maybeConst, !optional}, TypeCall
                          <C:•D>                            TypeCall.typeArguments{dk: "<>"}
                           C:•D                             TypeCallNamedBound
                              D                             TypeTraitBound{!maybeConst, !optional}
                                  {•w:•<I::E•as•F>::G,•}    StructDeclaration.properties{dk: "{}"}
                                    w:•<I::E•as•F>::G       StructPropertyDeclaration
                                       <I::E•as•F>::G       TypePath
                                       <I::E•as•F>          ExpressionTypeSelector
                                        I::E                TypePath
pub•struct•A<I>•where•I:•B<C:•D>,•{•w:•<I::E•as•F>::G,•}    </Program.ast>
pub•struct•A<I>•where•I:•B<C:•D>,•{•w:•<I::E•as•F>::G,•}    </Program>                                                                    */
// Discarded Nodes: 2
// Parsed Nodes: 455
// state_rollbacks: 12
// Total '.charCodeAt()' calls: 2244 (41% re-reads)
// Unnecessary 'skip_whitespace()' calls: 348
// source: "../../samples/statements/struct.rs"