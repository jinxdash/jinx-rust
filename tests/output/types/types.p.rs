type A where 'a: 'b + 'c = u8;                                                                                                            /*
type•A•where•'a:•'b•+•'c•=•u8;↲    <Program>
type•A•where•'a:•'b•+•'c•=•u8;↲    <Program.ast{dk: "None"}>
type•A•where•'a:•'b•+•'c•=•u8;     TypeAliasDeclaration
       where•'a:•'b•+•'c           TypeAliasDeclaration.whereBounds{dk: "None"}
             'a:•'b•+•'c           WhereLtBoundDeclaration
             'a                    LtIdentifier
                 'b                LtIdentifier
                      'c           LtIdentifier                                                                                           */
type A where 'a: 'b + 'c = u8;                                                                                                            /*
type•A•where•'a:•'b•+•'c•=•u8;    TypeAliasDeclaration
       where•'a:•'b•+•'c          TypeAliasDeclaration.whereBounds{dk: "None"}
             'a:•'b•+•'c          WhereLtBoundDeclaration
             'a                   LtIdentifier
                 'b               LtIdentifier
                      'c          LtIdentifier                                                                                            */
type A where 'a: 'b +    = u8;                                                                                                            /*
type•A•where•'a:•'b•+••••=•u8;    TypeAliasDeclaration
       where•'a:•'b•+             TypeAliasDeclaration.whereBounds{dk: "None"}
             'a:•'b•+             WhereLtBoundDeclaration
             'a                   LtIdentifier
                 'b               LtIdentifier                                                                                            */
type A where 'a: 'b,     = u8;                                                                                                            /*
type•A•where•'a:•'b,•••••=•u8;    TypeAliasDeclaration
       where•'a:•'b,              TypeAliasDeclaration.whereBounds{dk: "None"}
             'a:•'b               WhereLtBoundDeclaration
             'a                   LtIdentifier
                 'b               LtIdentifier                                                                                            */
type A where 'a:         = u8;                                                                                                            /*
type•A•where•'a:•••••••••=•u8;    TypeAliasDeclaration
       where•'a:                  TypeAliasDeclaration.whereBounds{dk: "None"}
             'a:                  WhereLtBoundDeclaration
             'a                   LtIdentifier                                                                                            */
type A where 'a:,        = u8;                                                                                                            /*
type•A•where•'a:,••••••••=•u8;    TypeAliasDeclaration
       where•'a:,                 TypeAliasDeclaration.whereBounds{dk: "None"}
             'a:                  WhereLtBoundDeclaration
             'a                   LtIdentifier                                                                                            */
type A where             = u8;                                                                                                            /*
type•A•where•••••••••••••=•u8;    TypeAliasDeclaration
       where                      TypeAliasDeclaration.whereBounds{dk: "None"}                                                            */

// type A = for<'a, T>       fn();
//•type•A•=•for<'a,•T>•••••••fn();    Comment{line}
type A = for<'a: 'b + 'c> fn();                                                                                                           /*
type•A•=•for<'a:•'b•+•'c>•fn();    TypeAliasDeclaration
         for<'a:•'b•+•'c>•fn()     TypeFnPointer
         for<'a:•'b•+•'c>          TypeFnPointer.ltParameters{dk: "<>"}
             'a:•'b•+•'c           GenericLtParameterDeclaration
             'a                    LtIdentifier
                 'b                LtIdentifier
                      'c           LtIdentifier
                            ()     TypeFnPointer.parameters{dk: "()"}                                                                     */
type A = for<'a: 'b +>    fn();                                                                                                           /*
type•A•=•for<'a:•'b•+>••••fn();    TypeAliasDeclaration
         for<'a:•'b•+>••••fn()     TypeFnPointer
         for<'a:•'b•+>             TypeFnPointer.ltParameters{dk: "<>"}
             'a:•'b•+              GenericLtParameterDeclaration
             'a                    LtIdentifier
                 'b                LtIdentifier
                            ()     TypeFnPointer.parameters{dk: "()"}                                                                     */
type A = for<'a: 'b,>     fn();                                                                                                           /*
type•A•=•for<'a:•'b,>•••••fn();    TypeAliasDeclaration
         for<'a:•'b,>•••••fn()     TypeFnPointer
         for<'a:•'b,>              TypeFnPointer.ltParameters{dk: "<>"}
             'a:•'b                GenericLtParameterDeclaration
             'a                    LtIdentifier
                 'b                LtIdentifier
                            ()     TypeFnPointer.parameters{dk: "()"}                                                                     */
type A = for<'a:,>        fn();                                                                                                           /*
type•A•=•for<'a:,>••••••••fn();    TypeAliasDeclaration
         for<'a:,>••••••••fn()     TypeFnPointer
         for<'a:,>                 TypeFnPointer.ltParameters{dk: "<>"}
             'a:                   GenericLtParameterDeclaration
             'a                    LtIdentifier
                            ()     TypeFnPointer.parameters{dk: "()"}                                                                     */
type A = for<'a:>         fn();                                                                                                           /*
type•A•=•for<'a:>•••••••••fn();    TypeAliasDeclaration
         for<'a:>•••••••••fn()     TypeFnPointer
         for<'a:>                  TypeFnPointer.ltParameters{dk: "<>"}
             'a:                   GenericLtParameterDeclaration
             'a                    LtIdentifier
                            ()     TypeFnPointer.parameters{dk: "()"}                                                                     */
type A = for<'a>          fn();                                                                                                           /*
type•A•=•for<'a>••••••••••fn();    TypeAliasDeclaration
         for<'a>••••••••••fn()     TypeFnPointer
         for<'a>                   TypeFnPointer.ltParameters{dk: "<>"}
             'a                    GenericLtParameterDeclaration, LtIdentifier
                            ()     TypeFnPointer.parameters{dk: "()"}                                                                     */
type A = for<>            fn();                                                                                                           /*
type•A•=•for<>••••••••••••fn();    TypeAliasDeclaration
         for<>••••••••••••fn()     TypeFnPointer
         for<>                     TypeFnPointer.ltParameters{dk: "<>"}
                            ()     TypeFnPointer.parameters{dk: "()"}                                                                     */

type A = Box<(Fn(u8) -> u8) + 'static + Send + Sync>;                                                                                     /*
type•A•=•Box<(Fn(u8)•->•u8)•+•'static•+•Send•+•Sync>;    TypeAliasDeclaration
         Box<(Fn(u8)•->•u8)•+•'static•+•Send•+•Sync>     TypeCall
            <(Fn(u8)•->•u8)•+•'static•+•Send•+•Sync>     TypeCall.typeArguments{dk: "<>"}
             (Fn(u8)•->•u8)•+•'static•+•Send•+•Sync      TypeDynBounds{!dyn}
             (Fn(u8)•->•u8)                              TypeTraitBound{!maybeConst, !optional}
              Fn(u8)•->•u8                               TypeFunction
                (u8)                                     TypeFunction.parameters{dk: "()"}
                              'static                    LtStatic
                                        Send             TypeTraitBound{!maybeConst, !optional}
                                               Sync      TypeTraitBound{!maybeConst, !optional}                                           */
type A = impl B;                                                                                                                          /*
type•A•=•impl•B;    TypeAliasDeclaration
         impl•B     TypeImplBounds
              B     TypeTraitBound{!maybeConst, !optional}                                                                                */

type A where = u8;                                                                                                                        /*
type•A•where•=•u8;    TypeAliasDeclaration
       where          TypeAliasDeclaration.whereBounds{dk: "None"}                                                                        */
type A where for<'a> for<'b> Trait1 + ?Trait2: 'a + Trait = u8;                                                                           /*
type•A•where•for<'a>•for<'b>•Trait1•+•?Trait2:•'a•+•Trait•=•u8;    TypeAliasDeclaration
       where•for<'a>•for<'b>•Trait1•+•?Trait2:•'a•+•Trait          TypeAliasDeclaration.whereBounds{dk: "None"}
             for<'a>•for<'b>•Trait1•+•?Trait2:•'a•+•Trait          WhereTypeBoundDeclaration
             for<'a>                                               WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                 'a                                                GenericLtParameterDeclaration, LtIdentifier
                     for<'b>•Trait1•+•?Trait2                      TypeDynBounds{!dyn}
                     for<'b>•Trait1                                TypeTraitBound{!maybeConst, !optional}
                     for<'b>                                       TypeTraitBound.ltParameters{dk: "<>"}
                         'b                                        GenericLtParameterDeclaration, LtIdentifier
                                      ?Trait2                      TypeTraitBound{!maybeConst, optional}
                                               'a                  LtIdentifier
                                                    Trait          TypeTraitBound{!maybeConst, !optional}                                 */
type A where T: = u8;                                                                                                                     /*
type•A•where•T:•=•u8;    TypeAliasDeclaration
       where•T:          TypeAliasDeclaration.whereBounds{dk: "None"}
             T:          WhereTypeBoundDeclaration                                                                                        */
type A where T: Trait + = u8;                                                                                                             /*
type•A•where•T:•Trait•+•=•u8;    TypeAliasDeclaration
       where•T:•Trait•+          TypeAliasDeclaration.whereBounds{dk: "None"}
             T:•Trait•+          WhereTypeBoundDeclaration
                Trait            TypeTraitBound{!maybeConst, !optional}                                                                   */
type A where T: Trait + Trait = u8;                                                                                                       /*
type•A•where•T:•Trait•+•Trait•=•u8;    TypeAliasDeclaration
       where•T:•Trait•+•Trait          TypeAliasDeclaration.whereBounds{dk: "None"}
             T:•Trait•+•Trait          WhereTypeBoundDeclaration
                Trait                  TypeTraitBound{!maybeConst, !optional}
                        Trait          TypeTraitBound{!maybeConst, !optional}                                                             */
type A where T: Trait, = u8;                                                                                                              /*
type•A•where•T:•Trait,•=•u8;    TypeAliasDeclaration
       where•T:•Trait,          TypeAliasDeclaration.whereBounds{dk: "None"}
             T:•Trait           WhereTypeBoundDeclaration
                Trait           TypeTraitBound{!maybeConst, !optional}                                                                    */
type A where T:, = u8;                                                                                                                    /*
type•A•where•T:,•=•u8;    TypeAliasDeclaration
       where•T:,          TypeAliasDeclaration.whereBounds{dk: "None"}
             T:           WhereTypeBoundDeclaration                                                                                       */

type A = <m::Alias as m::Trait>::X;                                                                                                       /*
type•A•=•<m::Alias•as•m::Trait>::X;    TypeAliasDeclaration
         <m::Alias•as•m::Trait>::X     TypePath
         <m::Alias•as•m::Trait>        ExpressionTypeSelector
          m::Alias                     TypePath
                      m::Trait         TypePath                                                                                           */

pub type A<T> where T: B = T;                                                                                                             /*
pub•type•A<T>•where•T:•B•=•T;    TypeAliasDeclaration
pub                              PubSpecifier
          <T>                    TypeAliasDeclaration.generics{dk: "<>"}
           T                     GenericTypeParameterDeclaration
              where•T:•B         TypeAliasDeclaration.whereBounds{dk: "None"}
                    T:•B         WhereTypeBoundDeclaration
                       B         TypeTraitBound{!maybeConst, !optional}                                                                   */

type A: Ord;                                                                                                                              /*
type•A:•Ord;    TypeAliasDeclaration
        Ord     TypeTraitBound{!maybeConst, !optional}                                                                                    */
type B: Ord = u8;                                                                                                                         /*
type•B:•Ord•=•u8;    TypeAliasDeclaration
        Ord          TypeTraitBound{!maybeConst, !optional}                                                                               */
type C: Ord where 'static: 'static = u8;                                                                                                  /*
type•C:•Ord•where•'static:•'static•=•u8;    TypeAliasDeclaration
        Ord                                 TypeTraitBound{!maybeConst, !optional}
            where•'static:•'static          TypeAliasDeclaration.whereBounds{dk: "None"}
                  'static:•'static          WhereLtBoundDeclaration
                  'static                   LtStatic
                           'static          LtStatic                                                                                      */
type D<_T>: Ord;                                                                                                                          /*
type•D<_T>:•Ord;    TypeAliasDeclaration
      <_T>          TypeAliasDeclaration.generics{dk: "<>"}
       _T           GenericTypeParameterDeclaration
            Ord     TypeTraitBound{!maybeConst, !optional}                                                                                */
type E<_T>: Ord = u8;                                                                                                                     /*
type•E<_T>:•Ord•=•u8;    TypeAliasDeclaration
      <_T>               TypeAliasDeclaration.generics{dk: "<>"}
       _T                GenericTypeParameterDeclaration
            Ord          TypeTraitBound{!maybeConst, !optional}                                                                           */
type F<_T>: Ord where 'static: 'static = u8;                                                                                              /*
type•F<_T>:•Ord•where•'static:•'static•=•u8;    TypeAliasDeclaration
      <_T>                                      TypeAliasDeclaration.generics{dk: "<>"}
       _T                                       GenericTypeParameterDeclaration
            Ord                                 TypeTraitBound{!maybeConst, !optional}
                where•'static:•'static          TypeAliasDeclaration.whereBounds{dk: "None"}
                      'static:•'static          WhereLtBoundDeclaration
                      'static                   LtStatic
                               'static          LtStatic                                                                                  */

type Y<T> where Self: Sized = u32;                                                                                                        /*
type•Y<T>•where•Self:•Sized•=•u32;    TypeAliasDeclaration
      <T>                             TypeAliasDeclaration.generics{dk: "<>"}
       T                              GenericTypeParameterDeclaration
          where•Self:•Sized           TypeAliasDeclaration.whereBounds{dk: "None"}
                Self:•Sized           WhereTypeBoundDeclaration
                      Sized           TypeTraitBound{!maybeConst, !optional}                                                              */
type Y<T>: A where Self: Sized;                                                                                                           /*
type•Y<T>:•A•where•Self:•Sized;    TypeAliasDeclaration
      <T>                          TypeAliasDeclaration.generics{dk: "<>"}
       T                           GenericTypeParameterDeclaration
           A                       TypeTraitBound{!maybeConst, !optional}
             where•Self:•Sized     TypeAliasDeclaration.whereBounds{dk: "None"}
                   Self:•Sized     WhereTypeBoundDeclaration
                         Sized     TypeTraitBound{!maybeConst, !optional}                                                                 */

pub const FN: &'static fn() = &(fop::<i32> as fn());                                                                                      /*
pub•const•FN:•&'static•fn()•=•&(fop::<i32>•as•fn());    ConstVariableDeclaration
pub                                                     PubSpecifier
              &'static•fn()                             TypeReference{!mut}
               'static                                  LtStatic
                       fn()                             TypeFnPointer
                         ()                             TypeFnPointer.parameters{dk: "()"}
                              &(fop::<i32>•as•fn())     ReferenceExpression{!mut}
                                fop::<i32>•as•fn()      ExpressionAsTypeCast
                                fop::<i32>              ExpressionTypeCast
                                     <i32>              ExpressionTypeCast.typeArguments{dk: "<>"}
                                              fn()      TypeFnPointer
                                                ()      TypeFnPointer.parameters{dk: "()"}                                                */
const A: &&&u32 = &&&42;                                                                                                                  /*
const•A:•&&&u32•=•&&&42;    ConstVariableDeclaration
         &&&u32             TypeReference{!mut}
          &&u32             TypeReference{!mut}
           &u32             TypeReference{!mut}
                  &&&42     ReferenceExpression{!mut}
                   &&42     ReferenceExpression{!mut}
                    &42     ReferenceExpression{!mut}
                     42     Literal{kind: Integer}                                                                                        */
const CONST1: &[bool; 1] = &[true];                                                                                                       /*
const•CONST1:•&[bool;•1]•=•&[true];    ConstVariableDeclaration
              &[bool;•1]               TypeReference{!mut}
               [bool;•1]               TypeSizedArray
                      1                Literal{kind: Integer}
                           &[true]     ReferenceExpression{!mut}
                            [true]     ArrayLiteral
                             true      Literal{kind: True}                                                                                */
const CONST: &[Option<()>; 1] = &[Some(())];                                                                                              /*
const•CONST:•&[Option<()>;•1]•=•&[Some(())];    ConstVariableDeclaration
             &[Option<()>;•1]                   TypeReference{!mut}
              [Option<()>;•1]                   TypeSizedArray
               Option<()>                       TypeCall
                     <()>                       TypeCall.typeArguments{dk: "<>"}
                      ()                        TypeTuple
                           1                    Literal{kind: Integer}
                                &[Some(())]     ReferenceExpression{!mut}
                                 [Some(())]     ArrayLiteral
                                  Some(())      CallExpression
                                      (())      CallExpression.arguments{dk: "()"}
                                       ()       TupleLiteral                                                                              */
const A: [u32; 1] = [4];const F: &'static dyn PartialEq<u32> = &7u32;                                                                     /*
const•A:•[u32;•1]•=•[4];                                                 ConstVariableDeclaration
         [u32;•1]                                                        TypeSizedArray
               1                                                         Literal{kind: Integer}
                    [4]                                                  ArrayLiteral
                     4                                                   Literal{kind: Integer}
                        const•F:•&'static•dyn•PartialEq<u32>•=•&7u32;    ConstVariableDeclaration
                                 &'static•dyn•PartialEq<u32>             TypeReference{!mut}
                                  'static                                LtStatic
                                          dyn•PartialEq<u32>             TypeDynBounds{dyn}
                                              PartialEq<u32>             TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                       <u32>             TypeCall.typeArguments{dk: "<>"}
                                                               &7u32     ReferenceExpression{!mut}
                                                                7u32     Literal{kind: Integer}
                                                                 u32     Identifier                                                       */
struct R<'a> { c: Box<dyn FnMut(&mut R, bool) + 'a> }                                                                                     /*
struct•R<'a>•{•c:•Box<dyn•FnMut(&mut•R,•bool)•+•'a>•}    StructDeclaration
        <'a>                                             StructDeclaration.generics{dk: "<>"}
         'a                                              GenericLtParameterDeclaration, LtIdentifier
             {•c:•Box<dyn•FnMut(&mut•R,•bool)•+•'a>•}    StructDeclaration.properties{dk: "{}"}
               c:•Box<dyn•FnMut(&mut•R,•bool)•+•'a>      StructPropertyDeclaration
                  Box<dyn•FnMut(&mut•R,•bool)•+•'a>      TypeCall
                     <dyn•FnMut(&mut•R,•bool)•+•'a>      TypeCall.typeArguments{dk: "<>"}
                      dyn•FnMut(&mut•R,•bool)•+•'a       TypeDynBounds{dyn}
                          FnMut(&mut•R,•bool)            TypeTraitBound{!maybeConst, !optional}, TypeFunction
                               (&mut•R,•bool)            TypeFunction.parameters{dk: "()"}
                                &mut•R                   TypeReference{mut}
                                                'a       LtIdentifier                                                                     */
fn g() -> impl Tr2<m::Alias> {}                                                                                                           /*
fn•g()•->•impl•Tr2<m::Alias>•{}    FunctionDeclaration
    ()                             FunctionDeclaration.parameters{dk: "()"}
          impl•Tr2<m::Alias>       TypeImplBounds
               Tr2<m::Alias>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                  <m::Alias>       TypeCall.typeArguments{dk: "<>"}
                   m::Alias        TypePath
                             {}    FunctionDeclaration.body{dk: "{}"}                                                                     */
fn leak_dyn_nonprincipal() -> Box<dyn PubPrincipal + PrivNonPrincipal> {}                                                                 /*
fn•leak_dyn_nonprincipal()•->•Box<dyn•PubPrincipal•+•PrivNonPrincipal>•{}    FunctionDeclaration
                        ()                                                   FunctionDeclaration.parameters{dk: "()"}
                              Box<dyn•PubPrincipal•+•PrivNonPrincipal>       TypeCall
                                 <dyn•PubPrincipal•+•PrivNonPrincipal>       TypeCall.typeArguments{dk: "<>"}
                                  dyn•PubPrincipal•+•PrivNonPrincipal        TypeDynBounds{dyn}
                                      PubPrincipal                           TypeTraitBound{!maybeConst, !optional}
                                                     PrivNonPrincipal        TypeTraitBound{!maybeConst, !optional}
                                                                       {}    FunctionDeclaration.body{dk: "{}"}                           */
fn method() -> Self::Pub {}                                                                                                               /*
fn•method()•->•Self::Pub•{}    FunctionDeclaration
         ()                    FunctionDeclaration.parameters{dk: "()"}
               Self::Pub       TypePath
                         {}    FunctionDeclaration.body{dk: "{}"}                                                                         */
fn f<T: PrivTr>(arg: T) {}                                                                                                                /*
fn•f<T:•PrivTr>(arg:•T)•{}    FunctionDeclaration
    <T:•PrivTr>               FunctionDeclaration.generics{dk: "<>"}
     T:•PrivTr                GenericTypeParameterDeclaration
        PrivTr                TypeTraitBound{!maybeConst, !optional}
               (arg:•T)       FunctionDeclaration.parameters{dk: "()"}
                arg:•T        FunctionParameterDeclaration
                        {}    FunctionDeclaration.body{dk: "{}"}                                                                          */
pub fn unused<const T: usize>() -> usize {}                                                                                               /*
pub•fn•unused<const•T:•usize>()•->•usize•{}    FunctionDeclaration
pub                                            PubSpecifier
             <const•T:•usize>                  FunctionDeclaration.generics{dk: "<>"}
              const•T:•usize                   ConstTypeParameterDeclaration
                             ()                FunctionDeclaration.parameters{dk: "()"}
                                         {}    FunctionDeclaration.body{dk: "{}"}                                                         */
fn start(_: isize, _: *const *const u8) -> isize {}                                                                                       /*
fn•start(_:•isize,•_:•*const•*const•u8)•->•isize•{}    FunctionDeclaration
        (_:•isize,•_:•*const•*const•u8)                FunctionDeclaration.parameters{dk: "()"}
         _:•isize                                      FunctionParameterDeclaration
         _                                             WildcardPattern
                   _:•*const•*const•u8                 FunctionParameterDeclaration
                   _                                   WildcardPattern
                      *const•*const•u8                 TypeDereferenceConst
                             *const•u8                 TypeDereferenceConst
                                                 {}    FunctionDeclaration.body{dk: "{}"}                                                 */
fn as_ptr(&self) -> *const Self::Item;                                                                                                    /*
fn•as_ptr(&self)•->•*const•Self::Item;    FunctionDeclaration
         (&self)                          FunctionDeclaration.parameters{dk: "()"}
          &self                           FunctionSelfParameterDeclaration{ref, !mut}
                    *const•Self::Item     TypeDereferenceConst
                           Self::Item     TypePath                                                                                        */
fn as_mut_ptr(&mut self) -> *mut Self::Item;                                                                                              /*
fn•as_mut_ptr(&mut•self)•->•*mut•Self::Item;    FunctionDeclaration
             (&mut•self)                        FunctionDeclaration.parameters{dk: "()"}
              &mut•self                         FunctionSelfParameterDeclaration{ref, mut}
                            *mut•Self::Item     TypeDereferenceMut
                                 Self::Item     TypePath                                                                                  */
fn as_ptr(&self) -> *const T { self as *const _ as *const _ }                                                                             /*
fn•as_ptr(&self)•->•*const•T•{•self•as•*const•_•as•*const•_•}    FunctionDeclaration
         (&self)                                                 FunctionDeclaration.parameters{dk: "()"}
          &self                                                  FunctionSelfParameterDeclaration{ref, !mut}
                    *const•T                                     TypeDereferenceConst
                             {•self•as•*const•_•as•*const•_•}    FunctionDeclaration.body{dk: "{}"}
                               self•as•*const•_•as•*const•_      ExpressionStatement{!semi}, ExpressionAsTypeCast
                               self•as•*const•_                  ExpressionAsTypeCast
                                       *const•_                  TypeDereferenceConst
                                              _                  TypeInferred
                                                   *const•_      TypeDereferenceConst
                                                          _      TypeInferred                                                             */
fn as_mut_ptr(&mut self) -> *mut T { self as *mut _ as *mut _}                                                                            /*
fn•as_mut_ptr(&mut•self)•->•*mut•T•{•self•as•*mut•_•as•*mut•_}    FunctionDeclaration
             (&mut•self)                                          FunctionDeclaration.parameters{dk: "()"}
              &mut•self                                           FunctionSelfParameterDeclaration{ref, mut}
                            *mut•T                                TypeDereferenceMut
                                   {•self•as•*mut•_•as•*mut•_}    FunctionDeclaration.body{dk: "{}"}
                                     self•as•*mut•_•as•*mut•_     ExpressionStatement{!semi}, ExpressionAsTypeCast
                                     self•as•*mut•_               ExpressionAsTypeCast
                                             *mut•_               TypeDereferenceMut
                                                  _               TypeInferred
                                                       *mut•_     TypeDereferenceMut
                                                            _     TypeInferred                                                            */
fn y_uses_f(f: impl Fn()) {}                                                                                                              /*
fn•y_uses_f(f:•impl•Fn())•{}    FunctionDeclaration
           (f:•impl•Fn())       FunctionDeclaration.parameters{dk: "()"}
            f:•impl•Fn()        FunctionParameterDeclaration
               impl•Fn()        TypeImplBounds
                    Fn()        TypeTraitBound{!maybeConst, !optional}, TypeFunction
                      ()        TypeFunction.parameters{dk: "()"}
                          {}    FunctionDeclaration.body{dk: "{}"}                                                                        */
fn infer<T: a::B>(c: T) -> T { c }                                                                                                        /*
fn•infer<T:•a::B>(c:•T)•->•T•{•c•}    FunctionDeclaration
        <T:•a::B>                     FunctionDeclaration.generics{dk: "<>"}
         T:•a::B                      GenericTypeParameterDeclaration
            a::B                      TypeTraitBound{!maybeConst, !optional}, TypePath
                 (c:•T)               FunctionDeclaration.parameters{dk: "()"}
                  c:•T                FunctionParameterDeclaration
                             {•c•}    FunctionDeclaration.body{dk: "{}"}
                               c      ExpressionStatement{!semi}                                                                          */
fn f1<'a, 'b, 'c>(_x: &'a u32, _y: &'b u32, _z: &'c u32) where 'c: 'a + 'b { }                                                            /*
fn•f1<'a,•'b,•'c>(_x:•&'a•u32,•_y:•&'b•u32,•_z:•&'c•u32)•where•'c:•'a•+•'b•{•}    FunctionDeclaration
     <'a,•'b,•'c>                                                                 FunctionDeclaration.generics{dk: "<>"}
      'a                                                                          GenericLtParameterDeclaration, LtIdentifier
          'b                                                                      GenericLtParameterDeclaration, LtIdentifier
              'c                                                                  GenericLtParameterDeclaration, LtIdentifier
                 (_x:•&'a•u32,•_y:•&'b•u32,•_z:•&'c•u32)                          FunctionDeclaration.parameters{dk: "()"}
                  _x:•&'a•u32                                                     FunctionParameterDeclaration
                      &'a•u32                                                     TypeReference{!mut}
                       'a                                                         LtIdentifier
                               _y:•&'b•u32                                        FunctionParameterDeclaration
                                   &'b•u32                                        TypeReference{!mut}
                                    'b                                            LtIdentifier
                                            _z:•&'c•u32                           FunctionParameterDeclaration
                                                &'c•u32                           TypeReference{!mut}
                                                 'c                               LtIdentifier
                                                         where•'c:•'a•+•'b        FunctionDeclaration.whereBounds{dk: "None"}
                                                               'c:•'a•+•'b        WhereLtBoundDeclaration
                                                               'c                 LtIdentifier
                                                                   'a             LtIdentifier
                                                                        'b        LtIdentifier
                                                                           {•}    FunctionDeclaration.body{dk: "{}"}                      */
fn syntax() {                                                                                                                             /*
fn•syntax()•{↲    <FunctionDeclaration>
         ()       FunctionDeclaration.parameters{dk: "()"}
            {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                    */
    A::<T = u8, T: Ord, String>();                                                                                                        /*
    A::<T•=•u8,•T:•Ord,•String>();    ExpressionStatement{semi}
    A::<T•=•u8,•T:•Ord,•String>()     CallExpression
       <T•=•u8,•T:•Ord,•String>       CallExpression.typeArguments{dk: "<>"}
        T•=•u8                        TypeCallNamedArgument
                T:•Ord                TypeCallNamedBound
                   Ord                TypeTraitBound{!maybeConst, !optional}
                               ()     CallExpression.arguments{dk: "()"}                                                                  */
    A::<T = u8, 'a, T: Ord>();                                                                                                            /*
    A::<T•=•u8,•'a,•T:•Ord>();    ExpressionStatement{semi}
    A::<T•=•u8,•'a,•T:•Ord>()     CallExpression
       <T•=•u8,•'a,•T:•Ord>       CallExpression.typeArguments{dk: "<>"}
        T•=•u8                    TypeCallNamedArgument
                'a                LtIdentifier
                    T:•Ord        TypeCallNamedBound
                       Ord        TypeTraitBound{!maybeConst, !optional}
                           ()     CallExpression.arguments{dk: "()"}                                                                      */
	fn y<'a>(y: &mut 'a + Send);                                                                                                          /*
	fn•y<'a>(y:•&mut•'a•+•Send);    FunctionDeclaration
	    <'a>                        FunctionDeclaration.generics{dk: "<>"}
	     'a                         GenericLtParameterDeclaration, LtIdentifier
	        (y:•&mut•'a•+•Send)     FunctionDeclaration.parameters{dk: "()"}
	         y:•&mut•'a•+•Send      FunctionParameterDeclaration
	            &mut•'a•+•Send      TypeReference{mut}
	                 'a•+•Send      TypeDynBounds{!dyn}
	                 'a             LtIdentifier
	                      Send      TypeTraitBound{!maybeConst, !optional}                                                                */
	let z = y as &mut 'a + Send;                                                                                                          /*
	let•z•=•y•as•&mut•'a•+•Send;    LetVariableDeclaration
	        y•as•&mut•'a•+•Send     ExpressionAsTypeCast
	             &mut•'a•+•Send     TypeReference{mut}
	                  'a•+•Send     TypeDynBounds{!dyn}
	                  'a            LtIdentifier
	                       Send     TypeTraitBound{!maybeConst, !optional}                                                                */
	let x: &'static str = "A";                                                                                                            /*
	let•x:•&'static•str•=•"A";    LetVariableDeclaration
	       &'static•str           TypeReference{!mut}
	        'static               LtStatic
	                      "A"     Literal{kind: String}                                                                                   */
	fn A() -> Box<<Self as A>::T>;                                                                                                        /*
	fn•A()•->•Box<<Self•as•A>::T>;    FunctionDeclaration
	    ()                            FunctionDeclaration.parameters{dk: "()"}
	          Box<<Self•as•A>::T>     TypeCall
	             <<Self•as•A>::T>     TypeCall.typeArguments{dk: "<>"}
	              <Self•as•A>::T      TypePath
	              <Self•as•A>         ExpressionTypeSelector                                                                              */
	let a = |a, b: _| -> _ { 0 };                                                                                                         /*
	let•a•=•|a,•b:•_|•->•_•{•0•};    LetVariableDeclaration
	        |a,•b:•_|•->•_•{•0•}     ClosureFunctionExpression
	        |a,•b:•_|                ClosureFunctionExpression.parameters{dk: "||"}
	         a                       ClosureFunctionParameterDeclaration
	            b:•_                 ClosureFunctionParameterDeclaration
	               _                 TypeInferred
	                     _           TypeInferred
	                       {•0•}     BlockExpression
	                         0       ExpressionStatement{!semi}, Literal{kind: Integer}                                                   */
	let a:     &usize =         & 1;                                                                                                      /*
	let•a:•••••&usize•=•••••••••&•1;    LetVariableDeclaration
	           &usize                   TypeReference{!mut}
	                            &•1     ReferenceExpression{!mut}
	                              1     Literal{kind: Integer}                                                                            */
    let a:    &&usize =       & & 1;                                                                                                      /*
    let•a:••••&&usize•=•••••••&•&•1;    LetVariableDeclaration
              &&usize                   TypeReference{!mut}
               &usize                   TypeReference{!mut}
                              &•&•1     ReferenceExpression{!mut}
                                &•1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let a:   &&&usize =     & & & 1;                                                                                                      /*
    let•a:•••&&&usize•=•••••&•&•&•1;    LetVariableDeclaration
             &&&usize                   TypeReference{!mut}
              &&usize                   TypeReference{!mut}
               &usize                   TypeReference{!mut}
                            &•&•&•1     ReferenceExpression{!mut}
                              &•&•1     ReferenceExpression{!mut}
                                &•1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let a:  & &&usize =     & & & 1;                                                                                                      /*
    let•a:••&•&&usize•=•••••&•&•&•1;    LetVariableDeclaration
            &•&&usize                   TypeReference{!mut}
              &&usize                   TypeReference{!mut}
               &usize                   TypeReference{!mut}
                            &•&•&•1     ReferenceExpression{!mut}
                              &•&•1     ReferenceExpression{!mut}
                                &•1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let a:  &&&&usize =   & & & & 1;                                                                                                      /*
    let•a:••&&&&usize•=•••&•&•&•&•1;    LetVariableDeclaration
            &&&&usize                   TypeReference{!mut}
             &&&usize                   TypeReference{!mut}
              &&usize                   TypeReference{!mut}
               &usize                   TypeReference{!mut}
                          &•&•&•&•1     ReferenceExpression{!mut}
                            &•&•&•1     ReferenceExpression{!mut}
                              &•&•1     ReferenceExpression{!mut}
                                &•1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let a: & &&&usize =   & & & & 1;                                                                                                      /*
    let•a:•&•&&&usize•=•••&•&•&•&•1;    LetVariableDeclaration
           &•&&&usize                   TypeReference{!mut}
             &&&usize                   TypeReference{!mut}
              &&usize                   TypeReference{!mut}
               &usize                   TypeReference{!mut}
                          &•&•&•&•1     ReferenceExpression{!mut}
                            &•&•&•1     ReferenceExpression{!mut}
                              &•&•1     ReferenceExpression{!mut}
                                &•1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
    let a: &&&&&usize = & & & & & 1;                                                                                                      /*
    let•a:•&&&&&usize•=•&•&•&•&•&•1;    LetVariableDeclaration
           &&&&&usize                   TypeReference{!mut}
            &&&&usize                   TypeReference{!mut}
             &&&usize                   TypeReference{!mut}
              &&usize                   TypeReference{!mut}
               &usize                   TypeReference{!mut}
                        &•&•&•&•&•1     ReferenceExpression{!mut}
                          &•&•&•&•1     ReferenceExpression{!mut}
                            &•&•&•1     ReferenceExpression{!mut}
                              &•&•1     ReferenceExpression{!mut}
                                &•1     ReferenceExpression{!mut}
                                  1     Literal{kind: Integer}                                                                            */
	let a: Box<Debug+> = box 3 as Box<Debug+>;                                                                                            /*
	let•a:•Box<Debug+>•=•box•3•as•Box<Debug+>;    LetVariableDeclaration
	       Box<Debug+>                            TypeCall
	          <Debug+>                            TypeCall.typeArguments{dk: "<>"}
	           Debug+                             TypeDynBounds{!dyn}
	           Debug                              TypeTraitBound{!maybeConst, !optional}
	                     box•3•as•Box<Debug+>     ExpressionAsTypeCast
	                     box•3                    BoxExpression
	                         3                    Literal{kind: Integer}
	                              Box<Debug+>     TypeCall
	                                 <Debug+>     TypeCall.typeArguments{dk: "<>"}
	                                  Debug+      TypeDynBounds{!dyn}
	                                  Debug       TypeTraitBound{!maybeConst, !optional}                                                  */
	let a: Box<((A)) + B>;                                                                                                                /*
	let•a:•Box<((A))•+•B>;    LetVariableDeclaration
	       Box<((A))•+•B>     TypeCall
	          <((A))•+•B>     TypeCall.typeArguments{dk: "<>"}
	           ((A))•+•B      TypeDynBounds{!dyn}
	           ((A))          TypeTraitBound{!maybeConst, !optional}
	                   B      TypeTraitBound{!maybeConst, !optional}                                                                      */
	let a: Box<(A + B) + C>;                                                                                                              /*
	let•a:•Box<(A•+•B)•+•C>;    LetVariableDeclaration
	       Box<(A•+•B)•+•C>     TypeCall
	          <(A•+•B)•+•C>     TypeCall.typeArguments{dk: "<>"}
	           (A•+•B)•+•C      TypeDynBounds{!dyn}
	           (A•+•B)          TypeTraitBound{!maybeConst, !optional}
	            A•+•B           TypeDynBounds{!dyn}
	            A               TypeTraitBound{!maybeConst, !optional}
	                B           TypeTraitBound{!maybeConst, !optional}
	                     C      TypeTraitBound{!maybeConst, !optional}                                                                    */
	let a: Box<(A +) + B>;                                                                                                                /*
	let•a:•Box<(A•+)•+•B>;    LetVariableDeclaration
	       Box<(A•+)•+•B>     TypeCall
	          <(A•+)•+•B>     TypeCall.typeArguments{dk: "<>"}
	           (A•+)•+•B      TypeDynBounds{!dyn}
	           (A•+)          TypeTraitBound{!maybeConst, !optional}
	            A•+           TypeDynBounds{!dyn}
	            A             TypeTraitBound{!maybeConst, !optional}
	                   B      TypeTraitBound{!maybeConst, !optional}                                                                      */
	let a: Box<(dyn A) + B>;                                                                                                              /*
	let•a:•Box<(dyn•A)•+•B>;    LetVariableDeclaration
	       Box<(dyn•A)•+•B>     TypeCall
	          <(dyn•A)•+•B>     TypeCall.typeArguments{dk: "<>"}
	           (dyn•A)•+•B      TypeDynBounds{!dyn}
	           (dyn•A)          TypeTraitBound{!maybeConst, !optional}
	            dyn•A           TypeDynBounds{dyn}
	                A           TypeTraitBound{!maybeConst, !optional}
	                     B      TypeTraitBound{!maybeConst, !optional}                                                                    */
	let a: Box<dyn A + (B + C)>;                                                                                                          /*
	let•a:•Box<dyn•A•+•(B•+•C)>;    LetVariableDeclaration
	       Box<dyn•A•+•(B•+•C)>     TypeCall
	          <dyn•A•+•(B•+•C)>     TypeCall.typeArguments{dk: "<>"}
	           dyn•A•+•(B•+•C)      TypeDynBounds{dyn}
	               A                TypeTraitBound{!maybeConst, !optional}
	                    B•+•C       TypeTraitBound{!maybeConst, !optional}, TypeDynBounds{!dyn}
	                    B           TypeTraitBound{!maybeConst, !optional}
	                        C       TypeTraitBound{!maybeConst, !optional}                                                                */
	let a: Box<impl A + (B + C)>;                                                                                                         /*
	let•a:•Box<impl•A•+•(B•+•C)>;    LetVariableDeclaration
	       Box<impl•A•+•(B•+•C)>     TypeCall
	          <impl•A•+•(B•+•C)>     TypeCall.typeArguments{dk: "<>"}
	           impl•A•+•(B•+•C)      TypeImplBounds
	                A                TypeTraitBound{!maybeConst, !optional}
	                     B•+•C       TypeTraitBound{!maybeConst, !optional}, TypeDynBounds{!dyn}
	                     B           TypeTraitBound{!maybeConst, !optional}
	                         C       TypeTraitBound{!maybeConst, !optional}                                                               */
	let a: Box<(impl A + B) + C>;                                                                                                         /*
	let•a:•Box<(impl•A•+•B)•+•C>;    LetVariableDeclaration
	       Box<(impl•A•+•B)•+•C>     TypeCall
	          <(impl•A•+•B)•+•C>     TypeCall.typeArguments{dk: "<>"}
	           (impl•A•+•B)•+•C      TypeDynBounds{!dyn}
	           (impl•A•+•B)          TypeTraitBound{!maybeConst, !optional}
	            impl•A•+•B           TypeImplBounds
	                 A               TypeTraitBound{!maybeConst, !optional}
	                     B           TypeTraitBound{!maybeConst, !optional}
	                          C      TypeTraitBound{!maybeConst, !optional}                                                               */
	let a: Box<impl A + (B + C) + D>;                                                                                                     /*
	let•a:•Box<impl•A•+•(B•+•C)•+•D>;    LetVariableDeclaration
	       Box<impl•A•+•(B•+•C)•+•D>     TypeCall
	          <impl•A•+•(B•+•C)•+•D>     TypeCall.typeArguments{dk: "<>"}
	           impl•A•+•(B•+•C)•+•D      TypeImplBounds
	                A                    TypeTraitBound{!maybeConst, !optional}
	                     B•+•C           TypeTraitBound{!maybeConst, !optional}, TypeDynBounds{!dyn}
	                     B               TypeTraitBound{!maybeConst, !optional}
	                         C           TypeTraitBound{!maybeConst, !optional}
	                              D      TypeTraitBound{!maybeConst, !optional}                                                           */
	let a: Box<dyn A + (B + (C + D)) + E>;                                                                                                /*
	let•a:•Box<dyn•A•+•(B•+•(C•+•D))•+•E>;    LetVariableDeclaration
	       Box<dyn•A•+•(B•+•(C•+•D))•+•E>     TypeCall
	          <dyn•A•+•(B•+•(C•+•D))•+•E>     TypeCall.typeArguments{dk: "<>"}
	           dyn•A•+•(B•+•(C•+•D))•+•E      TypeDynBounds{dyn}
	               A                          TypeTraitBound{!maybeConst, !optional}
	                    B•+•(C•+•D)           TypeTraitBound{!maybeConst, !optional}, TypeDynBounds{!dyn}
	                    B                     TypeTraitBound{!maybeConst, !optional}
	                         C•+•D            TypeTraitBound{!maybeConst, !optional}, TypeDynBounds{!dyn}
	                         C                TypeTraitBound{!maybeConst, !optional}
	                             D            TypeTraitBound{!maybeConst, !optional}
	                                   E      TypeTraitBound{!maybeConst, !optional}                                                      */
	let a: &for<'a> Trait<'a> + 'static;                                                                                                  /*
	let•a:•&for<'a>•Trait<'a>•+•'static;    LetVariableDeclaration
	       &for<'a>•Trait<'a>•+•'static     TypeReference{!mut}
	        for<'a>•Trait<'a>•+•'static     TypeDynBounds{!dyn}
	        for<'a>•Trait<'a>               TypeTraitBound{!maybeConst, !optional}
	        for<'a>                         TypeTraitBound.ltParameters{dk: "<>"}
	            'a                          GenericLtParameterDeclaration, LtIdentifier
	                Trait<'a>               TypeCall
	                     <'a>               TypeCall.typeArguments{dk: "<>"}
	                      'a                LtIdentifier
	                            'static     LtStatic                                                                                      */
	let a: &dyn PartialEq<u32> = &7u32;                                                                                                   /*
	let•a:•&dyn•PartialEq<u32>•=•&7u32;    LetVariableDeclaration
	       &dyn•PartialEq<u32>             TypeReference{!mut}
	        dyn•PartialEq<u32>             TypeDynBounds{dyn}
	            PartialEq<u32>             TypeTraitBound{!maybeConst, !optional}, TypeCall
	                     <u32>             TypeCall.typeArguments{dk: "<>"}
	                             &7u32     ReferenceExpression{!mut}
	                              7u32     Literal{kind: Integer}
	                               u32     Identifier                                                                                     */
	let a: Option<!> = None;                                                                                                              /*
	let•a:•Option<!>•=•None;    LetVariableDeclaration
	       Option<!>            TypeCall
	             <!>            TypeCall.typeArguments{dk: "<>"}
	              !             TypeNever                                                                                                 */
	let a = &() as *const () as *const Bottom;                                                                                            /*
	let•a•=•&()•as•*const•()•as•*const•Bottom;    LetVariableDeclaration
	        &()•as•*const•()•as•*const•Bottom     ExpressionAsTypeCast
	        &()•as•*const•()                      ExpressionAsTypeCast
	        &()                                   ReferenceExpression{!mut}
	         ()                                   TupleLiteral
	               *const•()                      TypeDereferenceConst
	                      ()                      TypeTuple
	                            *const•Bottom     TypeDereferenceConst                                                                    */
	let a = id(|_: &isize, _: &isize| {});                                                                                                /*
	let•a•=•id(|_:•&isize,•_:•&isize|•{});    LetVariableDeclaration
	        id(|_:•&isize,•_:•&isize|•{})     CallExpression
	          (|_:•&isize,•_:•&isize|•{})     CallExpression.arguments{dk: "()"}
	           |_:•&isize,•_:•&isize|•{}      ClosureFunctionExpression
	           |_:•&isize,•_:•&isize|         ClosureFunctionExpression.parameters{dk: "||"}
	            _:•&isize                     ClosureFunctionParameterDeclaration
	            _                             WildcardPattern
	               &isize                     TypeReference{!mut}
	                       _:•&isize          ClosureFunctionParameterDeclaration
	                       _                  WildcardPattern
	                          &isize          TypeReference{!mut}
	                                  {}      BlockExpression                                                                             */
	let a = id(|_: &isize, _: &isize| {});                                                                                                /*
	let•a•=•id(|_:•&isize,•_:•&isize|•{});    LetVariableDeclaration
	        id(|_:•&isize,•_:•&isize|•{})     CallExpression
	          (|_:•&isize,•_:•&isize|•{})     CallExpression.arguments{dk: "()"}
	           |_:•&isize,•_:•&isize|•{}      ClosureFunctionExpression
	           |_:•&isize,•_:•&isize|         ClosureFunctionExpression.parameters{dk: "||"}
	            _:•&isize                     ClosureFunctionParameterDeclaration
	            _                             WildcardPattern
	               &isize                     TypeReference{!mut}
	                       _:•&isize          ClosureFunctionParameterDeclaration
	                       _                  WildcardPattern
	                          &isize          TypeReference{!mut}
	                                  {}      BlockExpression                                                                             */
	
	
	
	fn equal1<T>(_: &T, _: &T) -> bool where {}                                                                                           /*
	fn•equal1<T>(_:•&T,•_:•&T)•->•bool•where•{}    FunctionDeclaration
	         <T>                                   FunctionDeclaration.generics{dk: "<>"}
	          T                                    GenericTypeParameterDeclaration
	            (_:•&T,•_:•&T)                     FunctionDeclaration.parameters{dk: "()"}
	             _:•&T                             FunctionParameterDeclaration
	             _                                 WildcardPattern
	                &T                             TypeReference{!mut}
	                    _:•&T                      FunctionParameterDeclaration
	                    _                          WildcardPattern
	                       &T                      TypeReference{!mut}
	                                   where       FunctionDeclaration.whereBounds{dk: "None"}
	                                         {}    FunctionDeclaration.body{dk: "{}"}                                                     */
	fn equal2<T>(_: &T, _: &T) -> bool where T: {}                                                                                        /*
	fn•equal2<T>(_:•&T,•_:•&T)•->•bool•where•T:•{}    FunctionDeclaration
	         <T>                                      FunctionDeclaration.generics{dk: "<>"}
	          T                                       GenericTypeParameterDeclaration
	            (_:•&T,•_:•&T)                        FunctionDeclaration.parameters{dk: "()"}
	             _:•&T                                FunctionParameterDeclaration
	             _                                    WildcardPattern
	                &T                                TypeReference{!mut}
	                    _:•&T                         FunctionParameterDeclaration
	                    _                             WildcardPattern
	                       &T                         TypeReference{!mut}
	                                   where•T:       FunctionDeclaration.whereBounds{dk: "None"}
	                                         T:       WhereTypeBoundDeclaration
	                                            {}    FunctionDeclaration.body{dk: "{}"}                                                  */
	fn A<'a>() where 'a: {}                                                                                                               /*
	fn•A<'a>()•where•'a:•{}    FunctionDeclaration
	    <'a>                   FunctionDeclaration.generics{dk: "<>"}
	     'a                    GenericLtParameterDeclaration, LtIdentifier
	        ()                 FunctionDeclaration.parameters{dk: "()"}
	           where•'a:       FunctionDeclaration.whereBounds{dk: "None"}
	                 'a:       WhereLtBoundDeclaration
	                 'a        LtIdentifier
	                     {}    FunctionDeclaration.body{dk: "{}"}                                                                         */
	pub fn A<T: 'static>(_: T) -> TypeId {}                                                                                               /*
	pub•fn•A<T:•'static>(_:•T)•->•TypeId•{}    FunctionDeclaration
	pub                                        PubSpecifier
	        <T:•'static>                       FunctionDeclaration.generics{dk: "<>"}
	         T:•'static                        GenericTypeParameterDeclaration
	            'static                        LtStatic
	                    (_:•T)                 FunctionDeclaration.parameters{dk: "()"}
	                     _:•T                  FunctionParameterDeclaration
	                     _                     WildcardPattern
	                                     {}    FunctionDeclaration.body{dk: "{}"}                                                         */
	pub fn unused<'a, T>(_: &'a u32) {}                                                                                                   /*
	pub•fn•unused<'a,•T>(_:•&'a•u32)•{}    FunctionDeclaration
	pub                                    PubSpecifier
	             <'a,•T>                   FunctionDeclaration.generics{dk: "<>"}
	              'a                       GenericLtParameterDeclaration, LtIdentifier
	                  T                    GenericTypeParameterDeclaration
	                    (_:•&'a•u32)       FunctionDeclaration.parameters{dk: "()"}
	                     _:•&'a•u32        FunctionParameterDeclaration
	                     _                 WildcardPattern
	                        &'a•u32        TypeReference{!mut}
	                         'a            LtIdentifier
	                                 {}    FunctionDeclaration.body{dk: "{}"}                                                             */
	let f: fn(_, i32) -> i32 = q;                                                                                                         /*
	let•f:•fn(_,•i32)•->•i32•=•q;    LetVariableDeclaration
	       fn(_,•i32)•->•i32         TypeFnPointer
	         (_,•i32)                TypeFnPointer.parameters{dk: "()"}
	          _                      TypeFnPointerParameter, TypeInferred
	             i32                 TypeFnPointerParameter                                                                               */
    let _ = S::<>;                                                                                                                        /*
    let•_•=•S::<>;    LetVariableDeclaration
        _             WildcardPattern
            S::<>     ExpressionTypeCast
               <>     ExpressionTypeCast.typeArguments{dk: "<>"}                                                                          */
    let _ = E::<>::V;                                                                                                                     /*
    let•_•=•E::<>::V;    LetVariableDeclaration
        _                WildcardPattern
            E::<>::V     ExpressionPath
            E::<>        ExpressionTypeCast
               <>        ExpressionTypeCast.typeArguments{dk: "<>"}                                                                       */
    let a: i32<>;                                                                                                                         /*
    let•a:•i32<>;    LetVariableDeclaration
           i32<>     TypeCall
              <>     TypeCall.typeArguments{dk: "<>"}                                                                                     */
	let a = (                                                                                                                             /*
	let•a•=•(↲    <LetVariableDeclaration>
	        (↲    <TupleLiteral>                                                                                                          */
		A::b::<fn(&'static isize, &'static isize)>(),                                                                                     /*
		A::b::<fn(&'static•isize,•&'static•isize)>()    CallExpression
		A::b                                            ExpressionPath
		      <fn(&'static•isize,•&'static•isize)>      CallExpression.typeArguments{dk: "<>"}
		       fn(&'static•isize,•&'static•isize)       TypeFnPointer
		         (&'static•isize,•&'static•isize)       TypeFnPointer.parameters{dk: "()"}
		          &'static•isize                        TypeFnPointerParameter, TypeReference{!mut}
		           'static                              LtStatic
		                          &'static•isize        TypeFnPointerParameter, TypeReference{!mut}
		                           'static              LtStatic
		                                          ()    CallExpression.arguments{dk: "()"}                                                */
		A::b::<for<'a> fn(&'static isize, &'a isize)>(),                                                                                  /*
		A::b::<for<'a>•fn(&'static•isize,•&'a•isize)>()    CallExpression
		A::b                                               ExpressionPath
		      <for<'a>•fn(&'static•isize,•&'a•isize)>      CallExpression.typeArguments{dk: "<>"}
		       for<'a>•fn(&'static•isize,•&'a•isize)       TypeFnPointer
		       for<'a>                                     TypeFnPointer.ltParameters{dk: "<>"}
		           'a                                      GenericLtParameterDeclaration, LtIdentifier
		                 (&'static•isize,•&'a•isize)       TypeFnPointer.parameters{dk: "()"}
		                  &'static•isize                   TypeFnPointerParameter, TypeReference{!mut}
		                   'static                         LtStatic
		                                  &'a•isize        TypeFnPointerParameter, TypeReference{!mut}
		                                   'a              LtIdentifier
		                                             ()    CallExpression.arguments{dk: "()"}                                             */
		A::b::<for<'a, 'b> fn(&'a isize, &'b isize)>(),                                                                                   /*
		A::b::<for<'a,•'b>•fn(&'a•isize,•&'b•isize)>()    CallExpression
		A::b                                              ExpressionPath
		      <for<'a,•'b>•fn(&'a•isize,•&'b•isize)>      CallExpression.typeArguments{dk: "<>"}
		       for<'a,•'b>•fn(&'a•isize,•&'b•isize)       TypeFnPointer
		       for<'a,•'b>                                TypeFnPointer.ltParameters{dk: "<>"}
		           'a                                     GenericLtParameterDeclaration, LtIdentifier
		               'b                                 GenericLtParameterDeclaration, LtIdentifier
		                     (&'a•isize,•&'b•isize)       TypeFnPointer.parameters{dk: "()"}
		                      &'a•isize                   TypeFnPointerParameter, TypeReference{!mut}
		                       'a                         LtIdentifier
		                                 &'b•isize        TypeFnPointerParameter, TypeReference{!mut}
		                                  'b              LtIdentifier
		                                            ()    CallExpression.arguments{dk: "()"}                                              */
		A::b::<for<'a, 'b> fn(&'b isize, &'a isize)>(),                                                                                   /*
		A::b::<for<'a,•'b>•fn(&'b•isize,•&'a•isize)>()    CallExpression
		A::b                                              ExpressionPath
		      <for<'a,•'b>•fn(&'b•isize,•&'a•isize)>      CallExpression.typeArguments{dk: "<>"}
		       for<'a,•'b>•fn(&'b•isize,•&'a•isize)       TypeFnPointer
		       for<'a,•'b>                                TypeFnPointer.ltParameters{dk: "<>"}
		           'a                                     GenericLtParameterDeclaration, LtIdentifier
		               'b                                 GenericLtParameterDeclaration, LtIdentifier
		                     (&'b•isize,•&'a•isize)       TypeFnPointer.parameters{dk: "()"}
		                      &'b•isize                   TypeFnPointerParameter, TypeReference{!mut}
		                       'b                         LtIdentifier
		                                 &'a•isize        TypeFnPointerParameter, TypeReference{!mut}
		                                  'a              LtIdentifier
		                                            ()    CallExpression.arguments{dk: "()"}                                              */
		A::b::<for<'a> fn(fn(&'a isize) -> &'a isize)>(),                                                                                 /*
		A::b::<for<'a>•fn(fn(&'a•isize)•->•&'a•isize)>()    CallExpression
		A::b                                                ExpressionPath
		      <for<'a>•fn(fn(&'a•isize)•->•&'a•isize)>      CallExpression.typeArguments{dk: "<>"}
		       for<'a>•fn(fn(&'a•isize)•->•&'a•isize)       TypeFnPointer
		       for<'a>                                      TypeFnPointer.ltParameters{dk: "<>"}
		           'a                                       GenericLtParameterDeclaration, LtIdentifier
		                 (fn(&'a•isize)•->•&'a•isize)       TypeFnPointer.parameters{dk: "()"}
		                  fn(&'a•isize)•->•&'a•isize        TypeFnPointerParameter, TypeFnPointer
		                    (&'a•isize)                     TypeFnPointer.parameters{dk: "()"}
		                     &'a•isize                      TypeFnPointerParameter, TypeReference{!mut}
		                      'a                            LtIdentifier
		                                   &'a•isize        TypeReference{!mut}
		                                    'a              LtIdentifier
		                                              ()    CallExpression.arguments{dk: "()"}                                            */
		A::b::<fn(for<'a> fn(&'a isize) -> &'a isize)>(),                                                                                 /*
		A::b::<fn(for<'a>•fn(&'a•isize)•->•&'a•isize)>()    CallExpression
		A::b                                                ExpressionPath
		      <fn(for<'a>•fn(&'a•isize)•->•&'a•isize)>      CallExpression.typeArguments{dk: "<>"}
		       fn(for<'a>•fn(&'a•isize)•->•&'a•isize)       TypeFnPointer
		         (for<'a>•fn(&'a•isize)•->•&'a•isize)       TypeFnPointer.parameters{dk: "()"}
		          for<'a>•fn(&'a•isize)•->•&'a•isize        TypeFnPointerParameter, TypeFnPointer
		          for<'a>                                   TypeFnPointer.ltParameters{dk: "<>"}
		              'a                                    GenericLtParameterDeclaration, LtIdentifier
		                    (&'a•isize)                     TypeFnPointer.parameters{dk: "()"}
		                     &'a•isize                      TypeFnPointerParameter, TypeReference{!mut}
		                      'a                            LtIdentifier
		                                   &'a•isize        TypeReference{!mut}
		                                    'a              LtIdentifier
		                                              ()    CallExpression.arguments{dk: "()"}                                            */
		A::b::<for<'a> fn(&'a dyn Trait<'a>) -> Struct<'a>>(),                                                                            /*
		A::b::<for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'a>>()    CallExpression
		A::b                                                     ExpressionPath
		      <for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'a>>      CallExpression.typeArguments{dk: "<>"}
		       for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'a>       TypeFnPointer
		       for<'a>                                           TypeFnPointer.ltParameters{dk: "<>"}
		           'a                                            GenericLtParameterDeclaration, LtIdentifier
		                 (&'a•dyn•Trait<'a>)                     TypeFnPointer.parameters{dk: "()"}
		                  &'a•dyn•Trait<'a>                      TypeFnPointerParameter, TypeReference{!mut}
		                   'a                                    LtIdentifier
		                      dyn•Trait<'a>                      TypeDynBounds{dyn}
		                          Trait<'a>                      TypeTraitBound{!maybeConst, !optional}, TypeCall
		                               <'a>                      TypeCall.typeArguments{dk: "<>"}
		                                'a                       LtIdentifier
		                                        Struct<'a>       TypeCall
		                                              <'a>       TypeCall.typeArguments{dk: "<>"}
		                                               'a        LtIdentifier
		                                                   ()    CallExpression.arguments{dk: "()"}                                       */
		A::b::<for<'a> fn(&'a dyn Trait<'a>) -> Struct<'static>>(),                                                                       /*
		A::b::<for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'static>>()    CallExpression
		A::b                                                          ExpressionPath
		      <for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'static>>      CallExpression.typeArguments{dk: "<>"}
		       for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'static>       TypeFnPointer
		       for<'a>                                                TypeFnPointer.ltParameters{dk: "<>"}
		           'a                                                 GenericLtParameterDeclaration, LtIdentifier
		                 (&'a•dyn•Trait<'a>)                          TypeFnPointer.parameters{dk: "()"}
		                  &'a•dyn•Trait<'a>                           TypeFnPointerParameter, TypeReference{!mut}
		                   'a                                         LtIdentifier
		                      dyn•Trait<'a>                           TypeDynBounds{dyn}
		                          Trait<'a>                           TypeTraitBound{!maybeConst, !optional}, TypeCall
		                               <'a>                           TypeCall.typeArguments{dk: "<>"}
		                                'a                            LtIdentifier
		                                        Struct<'static>       TypeCall
		                                              <'static>       TypeCall.typeArguments{dk: "<>"}
		                                               'static        LtStatic
		                                                        ()    CallExpression.arguments{dk: "()"}                                  */
		A::b::<for<'a, 'b> fn(&'a dyn Trait<'b>) -> Struct<'b>>(),                                                                        /*
		A::b::<for<'a,•'b>•fn(&'a•dyn•Trait<'b>)•->•Struct<'b>>()    CallExpression
		A::b                                                         ExpressionPath
		      <for<'a,•'b>•fn(&'a•dyn•Trait<'b>)•->•Struct<'b>>      CallExpression.typeArguments{dk: "<>"}
		       for<'a,•'b>•fn(&'a•dyn•Trait<'b>)•->•Struct<'b>       TypeFnPointer
		       for<'a,•'b>                                           TypeFnPointer.ltParameters{dk: "<>"}
		           'a                                                GenericLtParameterDeclaration, LtIdentifier
		               'b                                            GenericLtParameterDeclaration, LtIdentifier
		                     (&'a•dyn•Trait<'b>)                     TypeFnPointer.parameters{dk: "()"}
		                      &'a•dyn•Trait<'b>                      TypeFnPointerParameter, TypeReference{!mut}
		                       'a                                    LtIdentifier
		                          dyn•Trait<'b>                      TypeDynBounds{dyn}
		                              Trait<'b>                      TypeTraitBound{!maybeConst, !optional}, TypeCall
		                                   <'b>                      TypeCall.typeArguments{dk: "<>"}
		                                    'b                       LtIdentifier
		                                            Struct<'b>       TypeCall
		                                                  <'b>       TypeCall.typeArguments{dk: "<>"}
		                                                   'b        LtIdentifier
		                                                       ()    CallExpression.arguments{dk: "()"}                                   */
		A::b::<fn(for<'a> fn(&'a isize) -> &'a usize)>(),                                                                                 /*
		A::b::<fn(for<'a>•fn(&'a•isize)•->•&'a•usize)>()    CallExpression
		A::b                                                ExpressionPath
		      <fn(for<'a>•fn(&'a•isize)•->•&'a•usize)>      CallExpression.typeArguments{dk: "<>"}
		       fn(for<'a>•fn(&'a•isize)•->•&'a•usize)       TypeFnPointer
		         (for<'a>•fn(&'a•isize)•->•&'a•usize)       TypeFnPointer.parameters{dk: "()"}
		          for<'a>•fn(&'a•isize)•->•&'a•usize        TypeFnPointerParameter, TypeFnPointer
		          for<'a>                                   TypeFnPointer.ltParameters{dk: "<>"}
		              'a                                    GenericLtParameterDeclaration, LtIdentifier
		                    (&'a•isize)                     TypeFnPointer.parameters{dk: "()"}
		                     &'a•isize                      TypeFnPointerParameter, TypeReference{!mut}
		                      'a                            LtIdentifier
		                                   &'a•usize        TypeReference{!mut}
		                                    'a              LtIdentifier
		                                              ()    CallExpression.arguments{dk: "()"}                                            */
		A::b::<fn(for<'b> fn(&'b isize) -> &'b usize)>(),                                                                                 /*
		A::b::<fn(for<'b>•fn(&'b•isize)•->•&'b•usize)>()    CallExpression
		A::b                                                ExpressionPath
		      <fn(for<'b>•fn(&'b•isize)•->•&'b•usize)>      CallExpression.typeArguments{dk: "<>"}
		       fn(for<'b>•fn(&'b•isize)•->•&'b•usize)       TypeFnPointer
		         (for<'b>•fn(&'b•isize)•->•&'b•usize)       TypeFnPointer.parameters{dk: "()"}
		          for<'b>•fn(&'b•isize)•->•&'b•usize        TypeFnPointerParameter, TypeFnPointer
		          for<'b>                                   TypeFnPointer.ltParameters{dk: "<>"}
		              'b                                    GenericLtParameterDeclaration, LtIdentifier
		                    (&'b•isize)                     TypeFnPointer.parameters{dk: "()"}
		                     &'b•isize                      TypeFnPointerParameter, TypeReference{!mut}
		                      'b                            LtIdentifier
		                                   &'b•usize        TypeReference{!mut}
		                                    'b              LtIdentifier
		                                              ()    CallExpression.arguments{dk: "()"}                                            */
		A::b::<Box<dyn Fn(&'static isize, &'static isize)>>(),                                                                            /*
		A::b::<Box<dyn•Fn(&'static•isize,•&'static•isize)>>()    CallExpression
		A::b                                                     ExpressionPath
		      <Box<dyn•Fn(&'static•isize,•&'static•isize)>>      CallExpression.typeArguments{dk: "<>"}
		       Box<dyn•Fn(&'static•isize,•&'static•isize)>       TypeCall
		          <dyn•Fn(&'static•isize,•&'static•isize)>       TypeCall.typeArguments{dk: "<>"}
		           dyn•Fn(&'static•isize,•&'static•isize)        TypeDynBounds{dyn}
		               Fn(&'static•isize,•&'static•isize)        TypeTraitBound{!maybeConst, !optional}, TypeFunction
		                 (&'static•isize,•&'static•isize)        TypeFunction.parameters{dk: "()"}
		                  &'static•isize                         TypeReference{!mut}
		                   'static                               LtStatic
		                                  &'static•isize         TypeReference{!mut}
		                                   'static               LtStatic
		                                                   ()    CallExpression.arguments{dk: "()"}                                       */
		A::b::<Box<dyn for<'a> Fn(&'static isize, &'a isize)>>(),                                                                         /*
		A::b::<Box<dyn•for<'a>•Fn(&'static•isize,•&'a•isize)>>()    CallExpression
		A::b                                                        ExpressionPath
		      <Box<dyn•for<'a>•Fn(&'static•isize,•&'a•isize)>>      CallExpression.typeArguments{dk: "<>"}
		       Box<dyn•for<'a>•Fn(&'static•isize,•&'a•isize)>       TypeCall
		          <dyn•for<'a>•Fn(&'static•isize,•&'a•isize)>       TypeCall.typeArguments{dk: "<>"}
		           dyn•for<'a>•Fn(&'static•isize,•&'a•isize)        TypeDynBounds{dyn}
		               for<'a>•Fn(&'static•isize,•&'a•isize)        TypeTraitBound{!maybeConst, !optional}
		               for<'a>                                      TypeTraitBound.ltParameters{dk: "<>"}
		                   'a                                       GenericLtParameterDeclaration, LtIdentifier
		                       Fn(&'static•isize,•&'a•isize)        TypeFunction
		                         (&'static•isize,•&'a•isize)        TypeFunction.parameters{dk: "()"}
		                          &'static•isize                    TypeReference{!mut}
		                           'static                          LtStatic
		                                          &'a•isize         TypeReference{!mut}
		                                           'a               LtIdentifier
		                                                      ()    CallExpression.arguments{dk: "()"}                                    */
		A::b::<Box<dyn for<'a, 'b> Fn(&'a isize, &'b isize)>>(),                                                                          /*
		A::b::<Box<dyn•for<'a,•'b>•Fn(&'a•isize,•&'b•isize)>>()    CallExpression
		A::b                                                       ExpressionPath
		      <Box<dyn•for<'a,•'b>•Fn(&'a•isize,•&'b•isize)>>      CallExpression.typeArguments{dk: "<>"}
		       Box<dyn•for<'a,•'b>•Fn(&'a•isize,•&'b•isize)>       TypeCall
		          <dyn•for<'a,•'b>•Fn(&'a•isize,•&'b•isize)>       TypeCall.typeArguments{dk: "<>"}
		           dyn•for<'a,•'b>•Fn(&'a•isize,•&'b•isize)        TypeDynBounds{dyn}
		               for<'a,•'b>•Fn(&'a•isize,•&'b•isize)        TypeTraitBound{!maybeConst, !optional}
		               for<'a,•'b>                                 TypeTraitBound.ltParameters{dk: "<>"}
		                   'a                                      GenericLtParameterDeclaration, LtIdentifier
		                       'b                                  GenericLtParameterDeclaration, LtIdentifier
		                           Fn(&'a•isize,•&'b•isize)        TypeFunction
		                             (&'a•isize,•&'b•isize)        TypeFunction.parameters{dk: "()"}
		                              &'a•isize                    TypeReference{!mut}
		                               'a                          LtIdentifier
		                                         &'b•isize         TypeReference{!mut}
		                                          'b               LtIdentifier
		                                                     ()    CallExpression.arguments{dk: "()"}                                     */
		A::b::<Box<dyn for<'a, 'b> Fn(&'b isize, &'a isize)>>(),                                                                          /*
		A::b::<Box<dyn•for<'a,•'b>•Fn(&'b•isize,•&'a•isize)>>()    CallExpression
		A::b                                                       ExpressionPath
		      <Box<dyn•for<'a,•'b>•Fn(&'b•isize,•&'a•isize)>>      CallExpression.typeArguments{dk: "<>"}
		       Box<dyn•for<'a,•'b>•Fn(&'b•isize,•&'a•isize)>       TypeCall
		          <dyn•for<'a,•'b>•Fn(&'b•isize,•&'a•isize)>       TypeCall.typeArguments{dk: "<>"}
		           dyn•for<'a,•'b>•Fn(&'b•isize,•&'a•isize)        TypeDynBounds{dyn}
		               for<'a,•'b>•Fn(&'b•isize,•&'a•isize)        TypeTraitBound{!maybeConst, !optional}
		               for<'a,•'b>                                 TypeTraitBound.ltParameters{dk: "<>"}
		                   'a                                      GenericLtParameterDeclaration, LtIdentifier
		                       'b                                  GenericLtParameterDeclaration, LtIdentifier
		                           Fn(&'b•isize,•&'a•isize)        TypeFunction
		                             (&'b•isize,•&'a•isize)        TypeFunction.parameters{dk: "()"}
		                              &'b•isize                    TypeReference{!mut}
		                               'b                          LtIdentifier
		                                         &'a•isize         TypeReference{!mut}
		                                          'a               LtIdentifier
		                                                     ()    CallExpression.arguments{dk: "()"}                                     */
		A::b::<Box<dyn for<'a> Fn(Box<dyn Fn(&'a isize) -> &'a isize>)>>(),                                                               /*
		A::b::<Box<dyn•for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)>>()    CallExpression
		A::b                                                                  ExpressionPath
		      <Box<dyn•for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)>>      CallExpression.typeArguments{dk: "<>"}
		       Box<dyn•for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)>       TypeCall
		          <dyn•for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)>       TypeCall.typeArguments{dk: "<>"}
		           dyn•for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)        TypeDynBounds{dyn}
		               for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)        TypeTraitBound{!maybeConst, !optional}
		               for<'a>                                                TypeTraitBound.ltParameters{dk: "<>"}
		                   'a                                                 GenericLtParameterDeclaration, LtIdentifier
		                       Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)        TypeFunction
		                         (Box<dyn•Fn(&'a•isize)•->•&'a•isize>)        TypeFunction.parameters{dk: "()"}
		                          Box<dyn•Fn(&'a•isize)•->•&'a•isize>         TypeCall
		                             <dyn•Fn(&'a•isize)•->•&'a•isize>         TypeCall.typeArguments{dk: "<>"}
		                              dyn•Fn(&'a•isize)•->•&'a•isize          TypeDynBounds{dyn}
		                                  Fn(&'a•isize)•->•&'a•isize          TypeTraitBound{!maybeConst, !optional}, TypeFunction
		                                    (&'a•isize)                       TypeFunction.parameters{dk: "()"}
		                                     &'a•isize                        TypeReference{!mut}
		                                      'a                              LtIdentifier
		                                                   &'a•isize          TypeReference{!mut}
		                                                    'a                LtIdentifier
		                                                                ()    CallExpression.arguments{dk: "()"}                          */
		A::b::<Box<dyn Fn(Box<dyn for<'a> Fn(&'a isize) -> &'a isize>)>>(),                                                               /*
		A::b::<Box<dyn•Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)>>()    CallExpression
		A::b                                                                  ExpressionPath
		      <Box<dyn•Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)>>      CallExpression.typeArguments{dk: "<>"}
		       Box<dyn•Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)>       TypeCall
		          <dyn•Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)>       TypeCall.typeArguments{dk: "<>"}
		           dyn•Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)        TypeDynBounds{dyn}
		               Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)        TypeTraitBound{!maybeConst, !optional}, TypeFunction
		                 (Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)        TypeFunction.parameters{dk: "()"}
		                  Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>         TypeCall
		                     <dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>         TypeCall.typeArguments{dk: "<>"}
		                      dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize          TypeDynBounds{dyn}
		                          for<'a>•Fn(&'a•isize)•->•&'a•isize          TypeTraitBound{!maybeConst, !optional}
		                          for<'a>                                     TypeTraitBound.ltParameters{dk: "<>"}
		                              'a                                      GenericLtParameterDeclaration, LtIdentifier
		                                  Fn(&'a•isize)•->•&'a•isize          TypeFunction
		                                    (&'a•isize)                       TypeFunction.parameters{dk: "()"}
		                                     &'a•isize                        TypeReference{!mut}
		                                      'a                              LtIdentifier
		                                                   &'a•isize          TypeReference{!mut}
		                                                    'a                LtIdentifier
		                                                                ()    CallExpression.arguments{dk: "()"}                          */
		a::<L64<L64<()>>>(),                                                                                                              /*
		a::<L64<L64<()>>>()    CallExpression
		   <L64<L64<()>>>      CallExpression.typeArguments{dk: "<>"}
		    L64<L64<()>>       TypeCall
		       <L64<()>>       TypeCall.typeArguments{dk: "<>"}
		        L64<()>        TypeCall
		           <()>        TypeCall.typeArguments{dk: "<>"}
		            ()         TypeTuple
		                 ()    CallExpression.arguments{dk: "()"}                                                                         */
		a::<L<L64<L64<()>>>>(),                                                                                                           /*
		a::<L<L64<L64<()>>>>()    CallExpression
		   <L<L64<L64<()>>>>      CallExpression.typeArguments{dk: "<>"}
		    L<L64<L64<()>>>       TypeCall
		     <L64<L64<()>>>       TypeCall.typeArguments{dk: "<>"}
		      L64<L64<()>>        TypeCall
		         <L64<()>>        TypeCall.typeArguments{dk: "<>"}
		          L64<()>         TypeCall
		             <()>         TypeCall.typeArguments{dk: "<>"}
		              ()          TypeTuple
		                    ()    CallExpression.arguments{dk: "()"}                                                                      */
		<&dyn A<B = u8>>::x(&e::r(1)),                                                                                                    /*
		<&dyn•A<B•=•u8>>::x(&e::r(1))    CallExpression
		<&dyn•A<B•=•u8>>::x              ExpressionPath
		<&dyn•A<B•=•u8>>                 ExpressionTypeSelector
		 &dyn•A<B•=•u8>                  TypeReference{!mut}
		  dyn•A<B•=•u8>                  TypeDynBounds{dyn}
		      A<B•=•u8>                  TypeTraitBound{!maybeConst, !optional}, TypeCall
		       <B•=•u8>                  TypeCall.typeArguments{dk: "<>"}
		        B•=•u8                   TypeCallNamedArgument
		                   (&e::r(1))    CallExpression.arguments{dk: "()"}
		                    &e::r(1)     ReferenceExpression{!mut}
		                     e::r(1)     CallExpression
		                     e::r        ExpressionPath
		                         (1)     CallExpression.arguments{dk: "()"}
		                          1      Literal{kind: Integer}                                                                           */
		<&'static str>::f(&""),                                                                                                           /*
		<&'static•str>::f(&"")    CallExpression
		<&'static•str>::f         ExpressionPath
		<&'static•str>            ExpressionTypeSelector
		 &'static•str             TypeReference{!mut}
		  'static                 LtStatic
		                 (&"")    CallExpression.arguments{dk: "()"}
		                  &""     ReferenceExpression{!mut}
		                   ""     Literal{kind: String}                                                                                   */
		a::<>(),                                                                                                                          /*
		a::<>()    CallExpression
		   <>      CallExpression.typeArguments{dk: "<>"}
		     ()    CallExpression.arguments{dk: "()"}                                                                                     */
		a as &[&dyn Fn(usize)->()],                                                                                                       /*
		a•as•&[&dyn•Fn(usize)->()]    ExpressionAsTypeCast
		     &[&dyn•Fn(usize)->()]    TypeReference{!mut}
		      [&dyn•Fn(usize)->()]    TypeSlice
		       &dyn•Fn(usize)->()     TypeReference{!mut}
		        dyn•Fn(usize)->()     TypeDynBounds{dyn}
		            Fn(usize)->()     TypeTraitBound{!maybeConst, !optional}, TypeFunction
		              (usize)         TypeFunction.parameters{dk: "()"}
		                       ()     TypeTuple                                                                                           */
		a::<&U>(a),                                                                                                                       /*
		a::<&U>(a)    CallExpression
		   <&U>       CallExpression.typeArguments{dk: "<>"}
		    &U        TypeReference{!mut}
		       (a)    CallExpression.arguments{dk: "()"}                                                                                  */
		a::<U>(a),                                                                                                                        /*
		a::<U>(a)    CallExpression
		   <U>       CallExpression.typeArguments{dk: "<>"}
		      (a)    CallExpression.arguments{dk: "()"}                                                                                   */
		a::<&mut U>(a),                                                                                                                   /*
		a::<&mut•U>(a)    CallExpression
		   <&mut•U>       CallExpression.typeArguments{dk: "<>"}
		    &mut•U        TypeReference{mut}
		           (a)    CallExpression.arguments{dk: "()"}                                                                              */
		
	);                                                                                                                                    /*
   ╚)     </TupleLiteral>
   ╚);    </LetVariableDeclaration>                                                                                                       */
	let s: Foo<'a'> = Foo;                                                                                                                /*
	let•s:•Foo<'a'>•=•Foo;    LetVariableDeclaration
	       Foo<'a'>           TypeCall
	          <'a'>           TypeCall.typeArguments{dk: "<>"}
	           'a'            Literal{kind: Char}                                                                                         */
	let _: Foo<'b'> = s.into();                                                                                                           /*
	let•_:•Foo<'b'>•=•s.into();    LetVariableDeclaration
	    _                          WildcardPattern
	       Foo<'b'>                TypeCall
	          <'b'>                TypeCall.typeArguments{dk: "<>"}
	           'b'                 Literal{kind: Char}
	                  s.into()     CallExpression
	                        ()     CallExpression.arguments{dk: "()"}                                                                     */
	let s2: Foo<'a'> = Foo;                                                                                                               /*
	let•s2:•Foo<'a'>•=•Foo;    LetVariableDeclaration
	        Foo<'a'>           TypeCall
	           <'a'>           TypeCall.typeArguments{dk: "<>"}
	            'a'            Literal{kind: Char}                                                                                        */
	let _: Foo<'a'> = s2;                                                                                                                 /*
	let•_:•Foo<'a'>•=•s2;    LetVariableDeclaration
	    _                    WildcardPattern
	       Foo<'a'>          TypeCall
	          <'a'>          TypeCall.typeArguments{dk: "<>"}
	           'a'           Literal{kind: Char}                                                                                          */
	let s3: Foo<'a'> = Foo;                                                                                                               /*
	let•s3:•Foo<'a'>•=•Foo;    LetVariableDeclaration
	        Foo<'a'>           TypeCall
	           <'a'>           TypeCall.typeArguments{dk: "<>"}
	            'a'            Literal{kind: Char}                                                                                        */
	let _ = s3;                                                                                                                           /*
	let•_•=•s3;    LetVariableDeclaration
	    _          WildcardPattern                                                                                                        */
	let s4: Foo<'a'> = Foo;                                                                                                               /*
	let•s4:•Foo<'a'>•=•Foo;    LetVariableDeclaration
	        Foo<'a'>           TypeCall
	           <'a'>           TypeCall.typeArguments{dk: "<>"}
	            'a'            Literal{kind: Char}                                                                                        */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
fn A(x: Option<fn(i32)>) -> Option<unsafe fn(i32)> {}                                                                                     /*
fn•A(x:•Option<fn(i32)>)•->•Option<unsafe•fn(i32)>•{}    FunctionDeclaration
    (x:•Option<fn(i32)>)                                 FunctionDeclaration.parameters{dk: "()"}
     x:•Option<fn(i32)>                                  FunctionParameterDeclaration
        Option<fn(i32)>                                  TypeCall
              <fn(i32)>                                  TypeCall.typeArguments{dk: "<>"}
               fn(i32)                                   TypeFnPointer
                 (i32)                                   TypeFnPointer.parameters{dk: "()"}
                  i32                                    TypeFnPointerParameter
                            Option<unsafe•fn(i32)>       TypeCall
                                  <unsafe•fn(i32)>       TypeCall.typeArguments{dk: "<>"}
                                   unsafe•fn(i32)        TypeFnPointer{unsafe}
                                            (i32)        TypeFnPointer.parameters{dk: "()"}
                                             i32         TypeFnPointerParameter
                                                   {}    FunctionDeclaration.body{dk: "{}"}                                               */
fn f(x: fn(i32)) -> unsafe fn(i32) {}                                                                                                     /*
fn•f(x:•fn(i32))•->•unsafe•fn(i32)•{}    FunctionDeclaration
    (x:•fn(i32))                         FunctionDeclaration.parameters{dk: "()"}
     x:•fn(i32)                          FunctionParameterDeclaration
        fn(i32)                          TypeFnPointer
          (i32)                          TypeFnPointer.parameters{dk: "()"}
           i32                           TypeFnPointerParameter
                    unsafe•fn(i32)       TypeFnPointer{unsafe}
                             (i32)       TypeFnPointer.parameters{dk: "()"}
                              i32        TypeFnPointerParameter
                                   {}    FunctionDeclaration.body{dk: "{}"}                                                               */
fn f<'b, L: X<&'b Q<K>>>() {}                                                                                                             /*
fn•f<'b,•L:•X<&'b•Q<K>>>()•{}    FunctionDeclaration
    <'b,•L:•X<&'b•Q<K>>>         FunctionDeclaration.generics{dk: "<>"}
     'b                          GenericLtParameterDeclaration, LtIdentifier
         L:•X<&'b•Q<K>>          GenericTypeParameterDeclaration
            X<&'b•Q<K>>          TypeTraitBound{!maybeConst, !optional}, TypeCall
             <&'b•Q<K>>          TypeCall.typeArguments{dk: "<>"}
              &'b•Q<K>           TypeReference{!mut}
               'b                LtIdentifier
                  Q<K>           TypeCall
                   <K>           TypeCall.typeArguments{dk: "<>"}
                        ()       FunctionDeclaration.parameters{dk: "()"}
                           {}    FunctionDeclaration.body{dk: "{}"}                                                                       */
struct A<T, U = [u8; a::<T>()]>(T, U);                                                                                                    /*
struct•A<T,•U•=•[u8;•a::<T>()]>(T,•U);    TupleStructDeclaration
        <T,•U•=•[u8;•a::<T>()]>           TupleStructDeclaration.generics{dk: "<>"}
         T                                GenericTypeParameterDeclaration
            U•=•[u8;•a::<T>()]            GenericTypeParameterDeclaration
                [u8;•a::<T>()]            TypeSizedArray
                     a::<T>()             CallExpression
                        <T>               CallExpression.typeArguments{dk: "<>"}
                           ()             CallExpression.arguments{dk: "()"}
                               (T,•U)     TupleStructDeclaration.items{dk: "()"}
                                T         TupleStructItemDeclaration
                                   U      TupleStructItemDeclaration                                                                      */
impl Q for () {}                                                                                                                          /*
impl•Q•for•()•{}    ImplDeclaration{!const}
           ()       TypeTuple
              {}    ImplDeclaration.body{dk: "{}"}                                                                                        */
trait Q<P = Self> {}                                                                                                                      /*
trait•Q<P•=•Self>•{}    TraitDeclaration
       <P•=•Self>       TraitDeclaration.generics{dk: "<>"}
        P•=•Self        GenericTypeParameterDeclaration
                  {}    TraitDeclaration.body{dk: "{}"}                                                                                   */
trait Q<P: Sized = [Self]> {}                                                                                                             /*
trait•Q<P:•Sized•=•[Self]>•{}    TraitDeclaration
       <P:•Sized•=•[Self]>       TraitDeclaration.generics{dk: "<>"}
        P:•Sized•=•[Self]        GenericTypeParameterDeclaration
           Sized                 TypeTraitBound{!maybeConst, !optional}
                   [Self]        TypeSlice
                           {}    TraitDeclaration.body{dk: "{}"}                                                                          */
trait H<'d, 'e>: for<'f> I<'d, 'f, 'e> + 'd {}                                                                                            /*
trait•H<'d,•'e>:•for<'f>•I<'d,•'f,•'e>•+•'d•{}    TraitDeclaration
       <'d,•'e>                                   TraitDeclaration.generics{dk: "<>"}
        'd                                        GenericLtParameterDeclaration, LtIdentifier
            'e                                    GenericLtParameterDeclaration, LtIdentifier
                 for<'f>•I<'d,•'f,•'e>            TypeTraitBound{!maybeConst, !optional}
                 for<'f>                          TypeTraitBound.ltParameters{dk: "<>"}
                     'f                           GenericLtParameterDeclaration, LtIdentifier
                         I<'d,•'f,•'e>            TypeCall
                          <'d,•'f,•'e>            TypeCall.typeArguments{dk: "<>"}
                           'd                     LtIdentifier
                               'f                 LtIdentifier
                                   'e             LtIdentifier
                                         'd       LtIdentifier
                                            {}    TraitDeclaration.body{dk: "{}"}                                                         */
trait F<'f>: for<'a> A<'a> + for<'e> E<'e> {}                                                                                             /*
trait•F<'f>:•for<'a>•A<'a>•+•for<'e>•E<'e>•{}    TraitDeclaration
       <'f>                                      TraitDeclaration.generics{dk: "<>"}
        'f                                       GenericLtParameterDeclaration, LtIdentifier
             for<'a>•A<'a>                       TypeTraitBound{!maybeConst, !optional}
             for<'a>                             TypeTraitBound.ltParameters{dk: "<>"}
                 'a                              GenericLtParameterDeclaration, LtIdentifier
                     A<'a>                       TypeCall
                      <'a>                       TypeCall.typeArguments{dk: "<>"}
                       'a                        LtIdentifier
                             for<'e>•E<'e>       TypeTraitBound{!maybeConst, !optional}
                             for<'e>             TypeTraitBound.ltParameters{dk: "<>"}
                                 'e              GenericLtParameterDeclaration, LtIdentifier
                                     E<'e>       TypeCall
                                      <'e>       TypeCall.typeArguments{dk: "<>"}
                                       'e        LtIdentifier
                                           {}    TraitDeclaration.body{dk: "{}"}                                                          */
struct Q<A = S, T>(A, T);                                                                                                                 /*
struct•Q<A•=•S,•T>(A,•T);    TupleStructDeclaration
        <A•=•S,•T>           TupleStructDeclaration.generics{dk: "<>"}
         A•=•S               GenericTypeParameterDeclaration
                T            GenericTypeParameterDeclaration
                  (A,•T)     TupleStructDeclaration.items{dk: "()"}
                   A         TupleStructItemDeclaration
                      T      TupleStructItemDeclaration                                                                                   */
struct Q<A, B = Vec<C>, C>(A, B, C);                                                                                                      /*
struct•Q<A,•B•=•Vec<C>,•C>(A,•B,•C);    TupleStructDeclaration
        <A,•B•=•Vec<C>,•C>              TupleStructDeclaration.generics{dk: "<>"}
         A                              GenericTypeParameterDeclaration
            B•=•Vec<C>                  GenericTypeParameterDeclaration
                Vec<C>                  TypeCall
                   <C>                  TypeCall.typeArguments{dk: "<>"}
                        C               GenericTypeParameterDeclaration
                          (A,•B,•C)     TupleStructDeclaration.items{dk: "()"}
                           A            TupleStructItemDeclaration
                              B         TupleStructItemDeclaration
                                 C      TupleStructItemDeclaration                                                                        */
impl<'a> A<'a> for &'a str { fn f<T: B<'a>>(self) -> &'a str {} }                                                                         /*
impl<'a>•A<'a>•for•&'a•str•{•fn•f<T:•B<'a>>(self)•->•&'a•str•{}•}    ImplDeclaration{!const}
    <'a>                                                             ImplDeclaration.generics{dk: "<>"}
     'a                                                              GenericLtParameterDeclaration, LtIdentifier
         A<'a>                                                       TypeCall
          <'a>                                                       TypeCall.typeArguments{dk: "<>"}
           'a                                                        LtIdentifier
                   &'a•str                                           TypeReference{!mut}
                    'a                                               LtIdentifier
                           {•fn•f<T:•B<'a>>(self)•->•&'a•str•{}•}    ImplDeclaration.body{dk: "{}"}
                             fn•f<T:•B<'a>>(self)•->•&'a•str•{}      FunctionDeclaration
                                 <T:•B<'a>>                          FunctionDeclaration.generics{dk: "<>"}
                                  T:•B<'a>                           GenericTypeParameterDeclaration
                                     B<'a>                           TypeTraitBound{!maybeConst, !optional}, TypeCall
                                      <'a>                           TypeCall.typeArguments{dk: "<>"}
                                       'a                            LtIdentifier
                                           (self)                    FunctionDeclaration.parameters{dk: "()"}
                                            self                     FunctionSelfParameterDeclaration{!ref, !mut}
                                                     &'a•str         TypeReference{!mut}
                                                      'a             LtIdentifier
                                                             {}      FunctionDeclaration.body{dk: "{}"}                                   */
extern "C" fn A<T: Add>(a: T, b: T) -> T::Output { a + b }                                                                                /*
extern•"C"•fn•A<T:•Add>(a:•T,•b:•T)•->•T::Output•{•a•+•b•}    FunctionDeclaration
extern•"C"                                                    ExternSpecifier
       "C"                                                    Literal{kind: String}
               <T:•Add>                                       FunctionDeclaration.generics{dk: "<>"}
                T:•Add                                        GenericTypeParameterDeclaration
                   Add                                        TypeTraitBound{!maybeConst, !optional}
                       (a:•T,•b:•T)                           FunctionDeclaration.parameters{dk: "()"}
                        a:•T                                  FunctionParameterDeclaration
                              b:•T                            FunctionParameterDeclaration
                                       T::Output              TypePath
                                                 {•a•+•b•}    FunctionDeclaration.body{dk: "{}"}
                                                   a•+•b      ExpressionStatement{!semi}, OperationExpression{tk: "+"}                    */
extern "C" {                                                                                                                              /*
extern•"C"•{↲    <ExternBlockDeclaration>
       "C"       Literal{kind: String}
           {↲    <ExternBlockDeclaration.body{dk: "{}"}>                                                                                  */
    pub fn f<'a>(x: &'a i32);                                                                                                             /*
    pub•fn•f<'a>(x:•&'a•i32);    FunctionDeclaration
    pub                          PubSpecifier
            <'a>                 FunctionDeclaration.generics{dk: "<>"}
             'a                  GenericLtParameterDeclaration, LtIdentifier
                (x:•&'a•i32)     FunctionDeclaration.parameters{dk: "()"}
                 x:•&'a•i32      FunctionParameterDeclaration
                    &'a•i32      TypeReference{!mut}
                     'a          LtIdentifier                                                                                             */
    pub fn f<'b>(x: &'a i32, y: &'b i32);                                                                                                 /*
    pub•fn•f<'b>(x:•&'a•i32,•y:•&'b•i32);    FunctionDeclaration
    pub                                      PubSpecifier
            <'b>                             FunctionDeclaration.generics{dk: "<>"}
             'b                              GenericLtParameterDeclaration, LtIdentifier
                (x:•&'a•i32,•y:•&'b•i32)     FunctionDeclaration.parameters{dk: "()"}
                 x:•&'a•i32                  FunctionParameterDeclaration
                    &'a•i32                  TypeReference{!mut}
                     'a                      LtIdentifier
                             y:•&'b•i32      FunctionParameterDeclaration
                                &'b•i32      TypeReference{!mut}
                                 'b          LtIdentifier                                                                                 */
    pub fn f<'a>(x: &'a i32, y: &i32) -> &'a i32;                                                                                         /*
    pub•fn•f<'a>(x:•&'a•i32,•y:•&i32)•->•&'a•i32;    FunctionDeclaration
    pub                                              PubSpecifier
            <'a>                                     FunctionDeclaration.generics{dk: "<>"}
             'a                                      GenericLtParameterDeclaration, LtIdentifier
                (x:•&'a•i32,•y:•&i32)                FunctionDeclaration.parameters{dk: "()"}
                 x:•&'a•i32                          FunctionParameterDeclaration
                    &'a•i32                          TypeReference{!mut}
                     'a                              LtIdentifier
                             y:•&i32                 FunctionParameterDeclaration
                                &i32                 TypeReference{!mut}
                                         &'a•i32     TypeReference{!mut}
                                          'a         LtIdentifier                                                                         */
    pub fn f<'b>(x: for<'c> fn(&'a i32));                                                                                                 /*
    pub•fn•f<'b>(x:•for<'c>•fn(&'a•i32));    FunctionDeclaration
    pub                                      PubSpecifier
            <'b>                             FunctionDeclaration.generics{dk: "<>"}
             'b                              GenericLtParameterDeclaration, LtIdentifier
                (x:•for<'c>•fn(&'a•i32))     FunctionDeclaration.parameters{dk: "()"}
                 x:•for<'c>•fn(&'a•i32)      FunctionParameterDeclaration
                    for<'c>•fn(&'a•i32)      TypeFnPointer
                    for<'c>                  TypeFnPointer.ltParameters{dk: "<>"}
                        'c                   GenericLtParameterDeclaration, LtIdentifier
                              (&'a•i32)      TypeFnPointer.parameters{dk: "()"}
                               &'a•i32       TypeFnPointerParameter, TypeReference{!mut}
                                'a           LtIdentifier                                                                                 */
    pub fn f<'b>(x: for<'c> fn(&'b i32));                                                                                                 /*
    pub•fn•f<'b>(x:•for<'c>•fn(&'b•i32));    FunctionDeclaration
    pub                                      PubSpecifier
            <'b>                             FunctionDeclaration.generics{dk: "<>"}
             'b                              GenericLtParameterDeclaration, LtIdentifier
                (x:•for<'c>•fn(&'b•i32))     FunctionDeclaration.parameters{dk: "()"}
                 x:•for<'c>•fn(&'b•i32)      FunctionParameterDeclaration
                    for<'c>•fn(&'b•i32)      TypeFnPointer
                    for<'c>                  TypeFnPointer.ltParameters{dk: "<>"}
                        'c                   GenericLtParameterDeclaration, LtIdentifier
                              (&'b•i32)      TypeFnPointer.parameters{dk: "()"}
                               &'b•i32       TypeFnPointerParameter, TypeReference{!mut}
                                'b           LtIdentifier                                                                                 */
    pub fn f<'b>(x: for<'c> fn(&'c i32));                                                                                                 /*
    pub•fn•f<'b>(x:•for<'c>•fn(&'c•i32));    FunctionDeclaration
    pub                                      PubSpecifier
            <'b>                             FunctionDeclaration.generics{dk: "<>"}
             'b                              GenericLtParameterDeclaration, LtIdentifier
                (x:•for<'c>•fn(&'c•i32))     FunctionDeclaration.parameters{dk: "()"}
                 x:•for<'c>•fn(&'c•i32)      FunctionParameterDeclaration
                    for<'c>•fn(&'c•i32)      TypeFnPointer
                    for<'c>                  TypeFnPointer.ltParameters{dk: "<>"}
                        'c                   GenericLtParameterDeclaration, LtIdentifier
                              (&'c•i32)      TypeFnPointer.parameters{dk: "()"}
                               &'c•i32       TypeFnPointerParameter, TypeReference{!mut}
                                'c           LtIdentifier                                                                                 */
    pub fn f<'b>() -> for<'c> fn(&'a i32);                                                                                                /*
    pub•fn•f<'b>()•->•for<'c>•fn(&'a•i32);    FunctionDeclaration
    pub                                       PubSpecifier
            <'b>                              FunctionDeclaration.generics{dk: "<>"}
             'b                               GenericLtParameterDeclaration, LtIdentifier
                ()                            FunctionDeclaration.parameters{dk: "()"}
                      for<'c>•fn(&'a•i32)     TypeFnPointer
                      for<'c>                 TypeFnPointer.ltParameters{dk: "<>"}
                          'c                  GenericLtParameterDeclaration, LtIdentifier
                                (&'a•i32)     TypeFnPointer.parameters{dk: "()"}
                                 &'a•i32      TypeFnPointerParameter, TypeReference{!mut}
                                  'a          LtIdentifier                                                                                */
    pub fn f<'b>() -> for<'c> fn(&'b i32);                                                                                                /*
    pub•fn•f<'b>()•->•for<'c>•fn(&'b•i32);    FunctionDeclaration
    pub                                       PubSpecifier
            <'b>                              FunctionDeclaration.generics{dk: "<>"}
             'b                               GenericLtParameterDeclaration, LtIdentifier
                ()                            FunctionDeclaration.parameters{dk: "()"}
                      for<'c>•fn(&'b•i32)     TypeFnPointer
                      for<'c>                 TypeFnPointer.ltParameters{dk: "<>"}
                          'c                  GenericLtParameterDeclaration, LtIdentifier
                                (&'b•i32)     TypeFnPointer.parameters{dk: "()"}
                                 &'b•i32      TypeFnPointerParameter, TypeReference{!mut}
                                  'b          LtIdentifier                                                                                */
    pub fn f<'b>() -> for<'c> fn(&'c i32);                                                                                                /*
    pub•fn•f<'b>()•->•for<'c>•fn(&'c•i32);    FunctionDeclaration
    pub                                       PubSpecifier
            <'b>                              FunctionDeclaration.generics{dk: "<>"}
             'b                               GenericLtParameterDeclaration, LtIdentifier
                ()                            FunctionDeclaration.parameters{dk: "()"}
                      for<'c>•fn(&'c•i32)     TypeFnPointer
                      for<'c>                 TypeFnPointer.ltParameters{dk: "<>"}
                          'c                  GenericLtParameterDeclaration, LtIdentifier
                                (&'c•i32)     TypeFnPointer.parameters{dk: "()"}
                                 &'c•i32      TypeFnPointerParameter, TypeReference{!mut}
                                  'c          LtIdentifier                                                                                */
}                                                                                                                                         /*
}    </ExternBlockDeclaration.body>
}    </ExternBlockDeclaration>                                                                                                            */
struct X<'x, 'y> { x: std::marker::PhantomData<&'x ()>, y: std::marker::PhantomData<&'y ()>, }                                            /*
struct•X<'x,•'y>•{•x:•std::marker::PhantomData<&'x•()>,•y:•std::marker::PhantomData<&'y•()>,•}    StructDeclaration
        <'x,•'y>                                                                                  StructDeclaration.generics{dk: "<>"}
         'x                                                                                       GenericLtParameterDeclaration, LtIdentifier
             'y                                                                                   GenericLtParameterDeclaration, LtIdentifier
                 {•x:•std::marker::PhantomData<&'x•()>,•y:•std::marker::PhantomData<&'y•()>,•}    StructDeclaration.properties{dk: "{}"}
                   x:•std::marker::PhantomData<&'x•()>                                            StructPropertyDeclaration
                      std::marker::PhantomData<&'x•()>                                            TypeCall
                      std::marker::PhantomData                                                    TypePath
                      std::marker                                                                 TypePath
                                              <&'x•()>                                            TypeCall.typeArguments{dk: "<>"}
                                               &'x•()                                             TypeReference{!mut}
                                                'x                                                LtIdentifier
                                                   ()                                             TypeTuple
                                                        y:•std::marker::PhantomData<&'y•()>       StructPropertyDeclaration
                                                           std::marker::PhantomData<&'y•()>       TypeCall
                                                           std::marker::PhantomData               TypePath
                                                           std::marker                            TypePath
                                                                                   <&'y•()>       TypeCall.typeArguments{dk: "<>"}
                                                                                    &'y•()        TypeReference{!mut}
                                                                                     'y           LtIdentifier
                                                                                        ()        TypeTuple                               */
struct G<T> where for<'f> T: F<'f, As: E<'f>> + 'f,{ t: std::marker::PhantomData<T>,}                                                     /*
struct•G<T>•where•for<'f>•T:•F<'f,•As:•E<'f>>•+•'f,{•t:•std::marker::PhantomData<T>,}    StructDeclaration
        <T>                                                                              StructDeclaration.generics{dk: "<>"}
         T                                                                               GenericTypeParameterDeclaration
            where•for<'f>•T:•F<'f,•As:•E<'f>>•+•'f,                                      StructDeclaration.whereBounds{dk: "None"}
                  for<'f>•T:•F<'f,•As:•E<'f>>•+•'f                                       WhereTypeBoundDeclaration
                  for<'f>                                                                WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                      'f                                                                 GenericLtParameterDeclaration, LtIdentifier
                             F<'f,•As:•E<'f>>                                            TypeTraitBound{!maybeConst, !optional}, TypeCall
                              <'f,•As:•E<'f>>                                            TypeCall.typeArguments{dk: "<>"}
                               'f                                                        LtIdentifier
                                   As:•E<'f>                                             TypeCallNamedBound
                                       E<'f>                                             TypeTraitBound{!maybeConst, !optional}, TypeCall
                                        <'f>                                             TypeCall.typeArguments{dk: "<>"}
                                         'f                                              LtIdentifier
                                                'f                                       LtIdentifier
                                                   {•t:•std::marker::PhantomData<T>,}    StructDeclaration.properties{dk: "{}"}
                                                     t:•std::marker::PhantomData<T>      StructPropertyDeclaration
                                                        std::marker::PhantomData<T>      TypeCall
                                                        std::marker::PhantomData         TypePath
                                                        std::marker                      TypePath
                                                                                <T>      TypeCall.typeArguments{dk: "<>"}                 */
struct D<T> where T: for<'c> C<'c, As: A<'c>>, { t: std::marker::PhantomData<T>, }                                                        /*
struct•D<T>•where•T:•for<'c>•C<'c,•As:•A<'c>>,•{•t:•std::marker::PhantomData<T>,•}    StructDeclaration
        <T>                                                                           StructDeclaration.generics{dk: "<>"}
         T                                                                            GenericTypeParameterDeclaration
            where•T:•for<'c>•C<'c,•As:•A<'c>>,                                        StructDeclaration.whereBounds{dk: "None"}
                  T:•for<'c>•C<'c,•As:•A<'c>>                                         WhereTypeBoundDeclaration
                     for<'c>•C<'c,•As:•A<'c>>                                         TypeTraitBound{!maybeConst, !optional}
                     for<'c>                                                          TypeTraitBound.ltParameters{dk: "<>"}
                         'c                                                           GenericLtParameterDeclaration, LtIdentifier
                             C<'c,•As:•A<'c>>                                         TypeCall
                              <'c,•As:•A<'c>>                                         TypeCall.typeArguments{dk: "<>"}
                               'c                                                     LtIdentifier
                                   As:•A<'c>                                          TypeCallNamedBound
                                       A<'c>                                          TypeTraitBound{!maybeConst, !optional}, TypeCall
                                        <'c>                                          TypeCall.typeArguments{dk: "<>"}
                                         'c                                           LtIdentifier
                                               {•t:•std::marker::PhantomData<T>,•}    StructDeclaration.properties{dk: "{}"}
                                                 t:•std::marker::PhantomData<T>       StructPropertyDeclaration
                                                    std::marker::PhantomData<T>       TypeCall
                                                    std::marker::PhantomData          TypePath
                                                    std::marker                       TypePath
                                                                            <T>       TypeCall.typeArguments{dk: "<>"}                    */
fn f<T>() where T: A, T::U: B, {}                                                                                                         /*
fn•f<T>()•where•T:•A,•T::U:•B,•{}    FunctionDeclaration
    <T>                              FunctionDeclaration.generics{dk: "<>"}
     T                               GenericTypeParameterDeclaration
       ()                            FunctionDeclaration.parameters{dk: "()"}
          where•T:•A,•T::U:•B,       FunctionDeclaration.whereBounds{dk: "None"}
                T:•A                 WhereTypeBoundDeclaration
                   A                 TypeTraitBound{!maybeConst, !optional}
                      T::U:•B        WhereTypeBoundDeclaration
                      T::U           TypePath
                            B        TypeTraitBound{!maybeConst, !optional}
                               {}    FunctionDeclaration.body{dk: "{}"}                                                                   */
fn f(a: isize, b: *const *const u8) -> isize {}                                                                                           /*
fn•f(a:•isize,•b:•*const•*const•u8)•->•isize•{}    FunctionDeclaration
    (a:•isize,•b:•*const•*const•u8)                FunctionDeclaration.parameters{dk: "()"}
     a:•isize                                      FunctionParameterDeclaration
               b:•*const•*const•u8                 FunctionParameterDeclaration
                  *const•*const•u8                 TypeDereferenceConst
                         *const•u8                 TypeDereferenceConst
                                             {}    FunctionDeclaration.body{dk: "{}"}                                                     */
fn f<G:FnMut(A,A) -> A>(mut a: G, b: A, c: A) -> A {}                                                                                     /*
fn•f<G:FnMut(A,A)•->•A>(mut•a:•G,•b:•A,•c:•A)•->•A•{}    FunctionDeclaration
    <G:FnMut(A,A)•->•A>                                  FunctionDeclaration.generics{dk: "<>"}
     G:FnMut(A,A)•->•A                                   GenericTypeParameterDeclaration
       FnMut(A,A)•->•A                                   TypeTraitBound{!maybeConst, !optional}, TypeFunction
            (A,A)                                        TypeFunction.parameters{dk: "()"}
                       (mut•a:•G,•b:•A,•c:•A)            FunctionDeclaration.parameters{dk: "()"}
                        mut•a:•G                         FunctionParameterDeclaration
                        mut•a                            PatternVariableDeclaration{!ref, mut}
                                  b:•A                   FunctionParameterDeclaration
                                        c:•A             FunctionParameterDeclaration
                                                   {}    FunctionDeclaration.body{dk: "{}"}                                               */
fn f<T:A<B=T>+C>(x: T) -> T {}                                                                                                            /*
fn•f<T:A<B=T>+C>(x:•T)•->•T•{}    FunctionDeclaration
    <T:A<B=T>+C>                  FunctionDeclaration.generics{dk: "<>"}
     T:A<B=T>+C                   GenericTypeParameterDeclaration
       A<B=T>                     TypeTraitBound{!maybeConst, !optional}, TypeCall
        <B=T>                     TypeCall.typeArguments{dk: "<>"}
         B=T                      TypeCallNamedArgument
              C                   TypeTraitBound{!maybeConst, !optional}
                (x:•T)            FunctionDeclaration.parameters{dk: "()"}
                 x:•T             FunctionParameterDeclaration
                            {}    FunctionDeclaration.body{dk: "{}"}                                                                      */
fn f<A, B: a<A>>(x: B) -> C<A> {}                                                                                                         /*
fn•f<A,•B:•a<A>>(x:•B)•->•C<A>•{}    FunctionDeclaration
    <A,•B:•a<A>>                     FunctionDeclaration.generics{dk: "<>"}
     A                               GenericTypeParameterDeclaration
        B:•a<A>                      GenericTypeParameterDeclaration
           a<A>                      TypeTraitBound{!maybeConst, !optional}, TypeCall
            <A>                      TypeCall.typeArguments{dk: "<>"}
                (x:•B)               FunctionDeclaration.parameters{dk: "()"}
                 x:•B                FunctionParameterDeclaration
                          C<A>       TypeCall
                           <A>       TypeCall.typeArguments{dk: "<>"}
                               {}    FunctionDeclaration.body{dk: "{}"}                                                                   */
struct Whitespace<T: Clone + = ()> { t: T }                                                                                               /*
struct•Whitespace<T:•Clone•+•=•()>•{•t:•T•}    StructDeclaration
                 <T:•Clone•+•=•()>             StructDeclaration.generics{dk: "<>"}
                  T:•Clone•+•=•()              GenericTypeParameterDeclaration
                     Clone                     TypeTraitBound{!maybeConst, !optional}
                               ()              TypeTuple
                                   {•t:•T•}    StructDeclaration.properties{dk: "{}"}
                                     t:•T      StructPropertyDeclaration                                                                  */
struct TokenSplit<T: Clone +=  ()> { t: T }                                                                                               /*
struct•TokenSplit<T:•Clone•+=••()>•{•t:•T•}    StructDeclaration
                 <T:•Clone•+=••()>             StructDeclaration.generics{dk: "<>"}
                  T:•Clone•+=••()              GenericTypeParameterDeclaration
                     Clone                     TypeTraitBound{!maybeConst, !optional}
                               ()              TypeTuple
                                   {•t:•T•}    StructDeclaration.properties{dk: "{}"}
                                     t:•T      StructPropertyDeclaration                                                                  */
fn f<'a, 'b, T>(t: T) -> isize where T: 'a, 'a: 'b, T: Eq { 0 }                                                                           /*
fn•f<'a,•'b,•T>(t:•T)•->•isize•where•T:•'a,•'a:•'b,•T:•Eq•{•0•}    FunctionDeclaration
    <'a,•'b,•T>                                                    FunctionDeclaration.generics{dk: "<>"}
     'a                                                            GenericLtParameterDeclaration, LtIdentifier
         'b                                                        GenericLtParameterDeclaration, LtIdentifier
             T                                                     GenericTypeParameterDeclaration
               (t:•T)                                              FunctionDeclaration.parameters{dk: "()"}
                t:•T                                               FunctionParameterDeclaration
                               where•T:•'a,•'a:•'b,•T:•Eq          FunctionDeclaration.whereBounds{dk: "None"}
                                     T:•'a                         WhereTypeBoundDeclaration
                                        'a                         LtIdentifier
                                            'a:•'b                 WhereLtBoundDeclaration
                                            'a                     LtIdentifier
                                                'b                 LtIdentifier
                                                    T:•Eq          WhereTypeBoundDeclaration
                                                       Eq          TypeTraitBound{!maybeConst, !optional}
                                                          {•0•}    FunctionDeclaration.body{dk: "{}"}
                                                            0      ExpressionStatement{!semi}, Literal{kind: Integer}                     */
impl<T: ?Sized, U: ?Sized> A<B<U>> for C<T>  where T: D<U> {}                                                                             /*
impl<T:•?Sized,•U:•?Sized>•A<B<U>>•for•C<T>••where•T:•D<U>•{}    ImplDeclaration{!const}
    <T:•?Sized,•U:•?Sized>                                       ImplDeclaration.generics{dk: "<>"}
     T:•?Sized                                                   GenericTypeParameterDeclaration
        ?Sized                                                   TypeTraitBound{!maybeConst, optional}
                U:•?Sized                                        GenericTypeParameterDeclaration
                   ?Sized                                        TypeTraitBound{!maybeConst, optional}
                           A<B<U>>                               TypeCall
                            <B<U>>                               TypeCall.typeArguments{dk: "<>"}
                             B<U>                                TypeCall
                              <U>                                TypeCall.typeArguments{dk: "<>"}
                                       C<T>                      TypeCall
                                        <T>                      TypeCall.typeArguments{dk: "<>"}
                                             where•T:•D<U>       ImplDeclaration.whereBounds{dk: "None"}
                                                   T:•D<U>       WhereTypeBoundDeclaration
                                                      D<U>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                       <U>       TypeCall.typeArguments{dk: "<>"}
                                                           {}    ImplDeclaration.body{dk: "{}"}                                           */
fn f()where T: for<'a> A<'a> + 'a,{}                                                                                                      /*
fn•f()where•T:•for<'a>•A<'a>•+•'a,{}    FunctionDeclaration
    ()                                  FunctionDeclaration.parameters{dk: "()"}
      where•T:•for<'a>•A<'a>•+•'a,      FunctionDeclaration.whereBounds{dk: "None"}
            T:•for<'a>•A<'a>•+•'a       WhereTypeBoundDeclaration
               for<'a>•A<'a>            TypeTraitBound{!maybeConst, !optional}
               for<'a>                  TypeTraitBound.ltParameters{dk: "<>"}
                   'a                   GenericLtParameterDeclaration, LtIdentifier
                       A<'a>            TypeCall
                        <'a>            TypeCall.typeArguments{dk: "<>"}
                         'a             LtIdentifier
                               'a       LtIdentifier
                                  {}    FunctionDeclaration.body{dk: "{}"}                                                                */
fn f()where T: for<'g> H<'g, 'g, As: for<'h> H<'h, 'g> + 'g>,{}                                                                           /*
fn•f()where•T:•for<'g>•H<'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>,{}    FunctionDeclaration
    ()                                                             FunctionDeclaration.parameters{dk: "()"}
      where•T:•for<'g>•H<'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>,      FunctionDeclaration.whereBounds{dk: "None"}
            T:•for<'g>•H<'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>       WhereTypeBoundDeclaration
               for<'g>•H<'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>       TypeTraitBound{!maybeConst, !optional}
               for<'g>                                             TypeTraitBound.ltParameters{dk: "<>"}
                   'g                                              GenericLtParameterDeclaration, LtIdentifier
                       H<'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>       TypeCall
                        <'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>       TypeCall.typeArguments{dk: "<>"}
                         'g                                        LtIdentifier
                             'g                                    LtIdentifier
                                 As:•for<'h>•H<'h,•'g>•+•'g        TypeCallNamedBound
                                     for<'h>•H<'h,•'g>             TypeTraitBound{!maybeConst, !optional}
                                     for<'h>                       TypeTraitBound.ltParameters{dk: "<>"}
                                         'h                        GenericLtParameterDeclaration, LtIdentifier
                                             H<'h,•'g>             TypeCall
                                              <'h,•'g>             TypeCall.typeArguments{dk: "<>"}
                                               'h                  LtIdentifier
                                                   'g              LtIdentifier
                                                         'g        LtIdentifier
                                                             {}    FunctionDeclaration.body{dk: "{}"}                                     */
fn f()where T: for<'i> H<'i, 'i, As: for<'j> H<'j, 'i, As: for<'k> I<'i, 'k, 'j> + 'j> + 'i>,{}                                           /*
fn•f()where•T:•for<'i>•H<'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>,{}    FunctionDeclaration
    ()                                                                                             FunctionDeclaration.parameters{dk: "()"}
      where•T:•for<'i>•H<'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>,      FunctionDeclaration.whereBounds{dk: "None"}
            T:•for<'i>•H<'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>       WhereTypeBoundDeclaration
               for<'i>•H<'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>       TypeTraitBound{!maybeConst, !optional}
               for<'i>                                                                             TypeTraitBound.ltParameters{dk: "<>"}
                   'i                                                                              GenericLtParameterDeclaration, LtIdentifier
                       H<'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>       TypeCall
                        <'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>       TypeCall.typeArguments{dk: "<>"}
                         'i                                                                        LtIdentifier
                             'i                                                                    LtIdentifier
                                 As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i        TypeCallNamedBound
                                     for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>             TypeTraitBound{!maybeConst, !optional}
                                     for<'j>                                                       TypeTraitBound.ltParameters{dk: "<>"}
                                         'j                                                        GenericLtParameterDeclaration, LtIdentifier
                                             H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>             TypeCall
                                              <'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>             TypeCall.typeArguments{dk: "<>"}
                                               'j                                                  LtIdentifier
                                                   'i                                              LtIdentifier
                                                       As:•for<'k>•I<'i,•'k,•'j>•+•'j              TypeCallNamedBound
                                                           for<'k>•I<'i,•'k,•'j>                   TypeTraitBound{!maybeConst, !optional}
                                                           for<'k>                                 TypeTraitBound.ltParameters{dk: "<>"}
                                                               'k                                  GenericLtParameterDeclaration, LtIdentifier
                                                                   I<'i,•'k,•'j>                   TypeCall
                                                                    <'i,•'k,•'j>                   TypeCall.typeArguments{dk: "<>"}
                                                                     'i                            LtIdentifier
                                                                         'k                        LtIdentifier
                                                                             'j                    LtIdentifier
                                                                                   'j              LtIdentifier
                                                                                         'i        LtIdentifier
                                                                                             {}    FunctionDeclaration.body{dk: "{}"}     */
fn f()where T: for<'l, 'i> H<'l, 'i, As: for<'j> H<'j, 'i, As: for<'k> I<'l, 'k, 'j> + 'j> + 'i>,{}                                       /*
fn•f()where•T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>,{}    FunctionDeclaration
    ()                                                                                                 FunctionDeclaration.parameters{dk: "()"}
      where•T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>,      FunctionDeclaration.whereBounds{dk: "None"}
            T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>       WhereTypeBoundDeclaration
               for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>       TypeTraitBound{!maybeConst, !optional}
               for<'l,•'i>                                                                             TypeTraitBound.ltParameters{dk: "<>"}
                   'l                                                                                  GenericLtParameterDeclaration, LtIdentifier
                       'i                                                                              GenericLtParameterDeclaration, LtIdentifier
                           H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>       TypeCall
                            <'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>       TypeCall.typeArguments{dk: "<>"}
                             'l                                                                        LtIdentifier
                                 'i                                                                    LtIdentifier
                                     As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i        TypeCallNamedBound
                                         for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>             TypeTraitBound{!maybeConst, !optional}
                                         for<'j>                                                       TypeTraitBound.ltParameters{dk: "<>"}
                                             'j                                                        GenericLtParameterDeclaration, LtIdentifier
                                                 H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>             TypeCall
                                                  <'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>             TypeCall.typeArguments{dk: "<>"}
                                                   'j                                                  LtIdentifier
                                                       'i                                              LtIdentifier
                                                           As:•for<'k>•I<'l,•'k,•'j>•+•'j              TypeCallNamedBound
                                                               for<'k>•I<'l,•'k,•'j>                   TypeTraitBound{!maybeConst, !optional}
                                                               for<'k>                                 TypeTraitBound.ltParameters{dk: "<>"}
                                                                   'k                                  GenericLtParameterDeclaration, LtIdentifier
                                                                       I<'l,•'k,•'j>                   TypeCall
                                                                        <'l,•'k,•'j>                   TypeCall.typeArguments{dk: "<>"}
                                                                         'l                            LtIdentifier
                                                                             'k                        LtIdentifier
                                                                                 'j                    LtIdentifier
                                                                                       'j              LtIdentifier
                                                                                             'i        LtIdentifier
                                                                                                 {}    FunctionDeclaration.body{dk: "{}"} */
fn f()where T: for<'l, 'i> H<'l, 'i, As: for<'j> H<'j, 'i, As: for<'k> H<'j, 'k, As = X<'j, 'k>> + 'j> + 'i>{}                            /*
fn•f()where•T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>{}    FunctionDeclaration
    ()                                                                                                            FunctionDeclaration.parameters{dk: "()"}
      where•T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>      FunctionDeclaration.whereBounds{dk: "None"}
            T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>      WhereTypeBoundDeclaration
               for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>      TypeTraitBound{!maybeConst, !optional}
               for<'l,•'i>                                                                                        TypeTraitBound.ltParameters{dk: "<>"}
                   'l                                                                                             GenericLtParameterDeclaration, LtIdentifier
                       'i                                                                                         GenericLtParameterDeclaration, LtIdentifier
                           H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>      TypeCall
                            <'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>      TypeCall.typeArguments{dk: "<>"}
                             'l                                                                                   LtIdentifier
                                 'i                                                                               LtIdentifier
                                     As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i       TypeCallNamedBound
                                         for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>            TypeTraitBound{!maybeConst, !optional}
                                         for<'j>                                                                  TypeTraitBound.ltParameters{dk: "<>"}
                                             'j                                                                   GenericLtParameterDeclaration, LtIdentifier
                                                 H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>            TypeCall
                                                  <'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>            TypeCall.typeArguments{dk: "<>"}
                                                   'j                                                             LtIdentifier
                                                       'i                                                         LtIdentifier
                                                           As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j             TypeCallNamedBound
                                                               for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>                  TypeTraitBound{!maybeConst, !optional}
                                                               for<'k>                                            TypeTraitBound.ltParameters{dk: "<>"}
                                                                   'k                                             GenericLtParameterDeclaration, LtIdentifier
                                                                       H<'j,•'k,•As•=•X<'j,•'k>>                  TypeCall
                                                                        <'j,•'k,•As•=•X<'j,•'k>>                  TypeCall.typeArguments{dk: "<>"}
                                                                         'j                                       LtIdentifier
                                                                             'k                                   LtIdentifier
                                                                                 As•=•X<'j,•'k>                   TypeCallNamedArgument
                                                                                      X<'j,•'k>                   TypeCall
                                                                                       <'j,•'k>                   TypeCall.typeArguments{dk: "<>"}
                                                                                        'j                        LtIdentifier
                                                                                            'k                    LtIdentifier
                                                                                                   'j             LtIdentifier
                                                                                                         'i       LtIdentifier
                                                                                                            {}    FunctionDeclaration.body{dk: "{}"}*/
fn f() where T:         Fn(&(),    &())                                    {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••&())••••••••••••••••••••••••••••••••••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•••••••••Fn(&(),••••&())                                          FunctionDeclaration.whereBounds{dk: "None"}
             T:•••••••••Fn(&(),••••&())                                          WhereTypeBoundDeclaration
                        Fn(&(),••••&())                                          TypeTraitBound{!maybeConst, !optional}, TypeFunction
                          (&(),••••&())                                          TypeFunction.parameters{dk: "()"}
                           &()                                                   TypeReference{!mut}
                            ()                                                   TypeTuple
                                   &()                                           TypeReference{!mut}
                                    ()                                           TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T:         Fn(&'a (), &())                                    {}                                                             /*
fn•f()•where•T:•••••••••Fn(&'a•(),•&())••••••••••••••••••••••••••••••••••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•••••••••Fn(&'a•(),•&())                                          FunctionDeclaration.whereBounds{dk: "None"}
             T:•••••••••Fn(&'a•(),•&())                                          WhereTypeBoundDeclaration
                        Fn(&'a•(),•&())                                          TypeTraitBound{!maybeConst, !optional}, TypeFunction
                          (&'a•(),•&())                                          TypeFunction.parameters{dk: "()"}
                           &'a•()                                                TypeReference{!mut}
                            'a                                                   LtIdentifier
                               ()                                                TypeTuple
                                   &()                                           TypeReference{!mut}
                                    ()                                           TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T:         Fn(&(),    Box<dyn Fn(&())>)                       {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>)•••••••••••••••••••••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>)                             FunctionDeclaration.whereBounds{dk: "None"}
             T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>)                             WhereTypeBoundDeclaration
                        Fn(&(),••••Box<dyn•Fn(&())>)                             TypeTraitBound{!maybeConst, !optional}, TypeFunction
                          (&(),••••Box<dyn•Fn(&())>)                             TypeFunction.parameters{dk: "()"}
                           &()                                                   TypeReference{!mut}
                            ()                                                   TypeTuple
                                   Box<dyn•Fn(&())>                              TypeCall
                                      <dyn•Fn(&())>                              TypeCall.typeArguments{dk: "<>"}
                                       dyn•Fn(&())                               TypeDynBounds{dyn}
                                           Fn(&())                               TypeTraitBound{!maybeConst, !optional}, TypeFunction
                                             (&())                               TypeFunction.parameters{dk: "()"}
                                              &()                                TypeReference{!mut}
                                               ()                                TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T:         Fn(&(),    fn(&()))                                {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••fn(&()))••••••••••••••••••••••••••••••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•••••••••Fn(&(),••••fn(&()))                                      FunctionDeclaration.whereBounds{dk: "None"}
             T:•••••••••Fn(&(),••••fn(&()))                                      WhereTypeBoundDeclaration
                        Fn(&(),••••fn(&()))                                      TypeTraitBound{!maybeConst, !optional}, TypeFunction
                          (&(),••••fn(&()))                                      TypeFunction.parameters{dk: "()"}
                           &()                                                   TypeReference{!mut}
                            ()                                                   TypeTuple
                                   fn(&())                                       TypeFnPointer
                                     (&())                                       TypeFnPointer.parameters{dk: "()"}
                                      &()                                        TypeFnPointerParameter, TypeReference{!mut}
                                       ()                                        TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T:         Fn(&(),    for<'a> fn(&'a ()))                     {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••for<'a>•fn(&'a•()))•••••••••••••••••••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•••••••••Fn(&(),••••for<'a>•fn(&'a•()))                           FunctionDeclaration.whereBounds{dk: "None"}
             T:•••••••••Fn(&(),••••for<'a>•fn(&'a•()))                           WhereTypeBoundDeclaration
                        Fn(&(),••••for<'a>•fn(&'a•()))                           TypeTraitBound{!maybeConst, !optional}, TypeFunction
                          (&(),••••for<'a>•fn(&'a•()))                           TypeFunction.parameters{dk: "()"}
                           &()                                                   TypeReference{!mut}
                            ()                                                   TypeTuple
                                   for<'a>•fn(&'a•())                            TypeFnPointer
                                   for<'a>                                       TypeFnPointer.ltParameters{dk: "<>"}
                                       'a                                        GenericLtParameterDeclaration, LtIdentifier
                                             (&'a•())                            TypeFnPointer.parameters{dk: "()"}
                                              &'a•()                             TypeFnPointerParameter, TypeReference{!mut}
                                               'a                                LtIdentifier
                                                  ()                             TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T:         Fn(&(),    Box<dyn Fn(&())>, &(), fn(&(), &()))    {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>,•&(),•fn(&(),•&()))••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>,•&(),•fn(&(),•&()))          FunctionDeclaration.whereBounds{dk: "None"}
             T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>,•&(),•fn(&(),•&()))          WhereTypeBoundDeclaration
                        Fn(&(),••••Box<dyn•Fn(&())>,•&(),•fn(&(),•&()))          TypeTraitBound{!maybeConst, !optional}, TypeFunction
                          (&(),••••Box<dyn•Fn(&())>,•&(),•fn(&(),•&()))          TypeFunction.parameters{dk: "()"}
                           &()                                                   TypeReference{!mut}
                            ()                                                   TypeTuple
                                   Box<dyn•Fn(&())>                              TypeCall
                                      <dyn•Fn(&())>                              TypeCall.typeArguments{dk: "<>"}
                                       dyn•Fn(&())                               TypeDynBounds{dyn}
                                           Fn(&())                               TypeTraitBound{!maybeConst, !optional}, TypeFunction
                                             (&())                               TypeFunction.parameters{dk: "()"}
                                              &()                                TypeReference{!mut}
                                               ()                                TypeTuple
                                                     &()                         TypeReference{!mut}
                                                      ()                         TypeTuple
                                                          fn(&(),•&())           TypeFnPointer
                                                            (&(),•&())           TypeFnPointer.parameters{dk: "()"}
                                                             &()                 TypeFnPointerParameter, TypeReference{!mut}
                                                              ()                 TypeTuple
                                                                  &()            TypeFnPointerParameter, TypeReference{!mut}
                                                                   ()            TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T: for<'a> Fn(&'a (), &())                                    {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&'a•(),•&())••••••••••••••••••••••••••••••••••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•for<'a>•Fn(&'a•(),•&())                                          FunctionDeclaration.whereBounds{dk: "None"}
             T:•for<'a>•Fn(&'a•(),•&())                                          WhereTypeBoundDeclaration
                for<'a>•Fn(&'a•(),•&())                                          TypeTraitBound{!maybeConst, !optional}
                for<'a>                                                          TypeTraitBound.ltParameters{dk: "<>"}
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&'a•(),•&())                                          TypeFunction
                          (&'a•(),•&())                                          TypeFunction.parameters{dk: "()"}
                           &'a•()                                                TypeReference{!mut}
                            'a                                                   LtIdentifier
                               ()                                                TypeTuple
                                   &()                                           TypeReference{!mut}
                                    ()                                           TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T: for<'a> Fn(&(),    &'a ())                                 {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&(),••••&'a•())•••••••••••••••••••••••••••••••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•for<'a>•Fn(&(),••••&'a•())                                       FunctionDeclaration.whereBounds{dk: "None"}
             T:•for<'a>•Fn(&(),••••&'a•())                                       WhereTypeBoundDeclaration
                for<'a>•Fn(&(),••••&'a•())                                       TypeTraitBound{!maybeConst, !optional}
                for<'a>                                                          TypeTraitBound.ltParameters{dk: "<>"}
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&(),••••&'a•())                                       TypeFunction
                          (&(),••••&'a•())                                       TypeFunction.parameters{dk: "()"}
                           &()                                                   TypeReference{!mut}
                            ()                                                   TypeTuple
                                   &'a•()                                        TypeReference{!mut}
                                    'a                                           LtIdentifier
                                       ()                                        TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T: for<'a> Fn(&'a (), &'a ())                                 {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&'a•(),•&'a•())•••••••••••••••••••••••••••••••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•for<'a>•Fn(&'a•(),•&'a•())                                       FunctionDeclaration.whereBounds{dk: "None"}
             T:•for<'a>•Fn(&'a•(),•&'a•())                                       WhereTypeBoundDeclaration
                for<'a>•Fn(&'a•(),•&'a•())                                       TypeTraitBound{!maybeConst, !optional}
                for<'a>                                                          TypeTraitBound.ltParameters{dk: "<>"}
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&'a•(),•&'a•())                                       TypeFunction
                          (&'a•(),•&'a•())                                       TypeFunction.parameters{dk: "()"}
                           &'a•()                                                TypeReference{!mut}
                            'a                                                   LtIdentifier
                               ()                                                TypeTuple
                                   &'a•()                                        TypeReference{!mut}
                                    'a                                           LtIdentifier
                                       ()                                        TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T: for<'a> Fn(&'a (), Box<dyn Fn(&())>)                       {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&'a•(),•Box<dyn•Fn(&())>)•••••••••••••••••••••••{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•for<'a>•Fn(&'a•(),•Box<dyn•Fn(&())>)                             FunctionDeclaration.whereBounds{dk: "None"}
             T:•for<'a>•Fn(&'a•(),•Box<dyn•Fn(&())>)                             WhereTypeBoundDeclaration
                for<'a>•Fn(&'a•(),•Box<dyn•Fn(&())>)                             TypeTraitBound{!maybeConst, !optional}
                for<'a>                                                          TypeTraitBound.ltParameters{dk: "<>"}
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&'a•(),•Box<dyn•Fn(&())>)                             TypeFunction
                          (&'a•(),•Box<dyn•Fn(&())>)                             TypeFunction.parameters{dk: "()"}
                           &'a•()                                                TypeReference{!mut}
                            'a                                                   LtIdentifier
                               ()                                                TypeTuple
                                   Box<dyn•Fn(&())>                              TypeCall
                                      <dyn•Fn(&())>                              TypeCall.typeArguments{dk: "<>"}
                                       dyn•Fn(&())                               TypeDynBounds{dyn}
                                           Fn(&())                               TypeTraitBound{!maybeConst, !optional}, TypeFunction
                                             (&())                               TypeFunction.parameters{dk: "()"}
                                              &()                                TypeReference{!mut}
                                               ()                                TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T: for<'a> Fn(&(),    Box<dyn Fn(&())>, &'a (), fn(&(), &())) {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))•{}    FunctionDeclaration
    ()                                                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•for<'a>•Fn(&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))       FunctionDeclaration.whereBounds{dk: "None"}
             T:•for<'a>•Fn(&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))       WhereTypeBoundDeclaration
                for<'a>•Fn(&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))       TypeTraitBound{!maybeConst, !optional}
                for<'a>                                                          TypeTraitBound.ltParameters{dk: "<>"}
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))       TypeFunction
                          (&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))       TypeFunction.parameters{dk: "()"}
                           &()                                                   TypeReference{!mut}
                            ()                                                   TypeTuple
                                   Box<dyn•Fn(&())>                              TypeCall
                                      <dyn•Fn(&())>                              TypeCall.typeArguments{dk: "<>"}
                                       dyn•Fn(&())                               TypeDynBounds{dyn}
                                           Fn(&())                               TypeTraitBound{!maybeConst, !optional}, TypeFunction
                                             (&())                               TypeFunction.parameters{dk: "()"}
                                              &()                                TypeReference{!mut}
                                               ()                                TypeTuple
                                                     &'a•()                      TypeReference{!mut}
                                                      'a                         LtIdentifier
                                                         ()                      TypeTuple
                                                             fn(&(),•&())        TypeFnPointer
                                                               (&(),•&())        TypeFnPointer.parameters{dk: "()"}
                                                                &()              TypeFnPointerParameter, TypeReference{!mut}
                                                                 ()              TypeTuple
                                                                     &()         TypeFnPointerParameter, TypeReference{!mut}
                                                                      ()         TypeTuple
                                                                           {}    FunctionDeclaration.body{dk: "{}"}                       */
fn f() where T: A, F: FnOnce(B<T>) -> bool {}                                                                                             /*
fn•f()•where•T:•A,•F:•FnOnce(B<T>)•->•bool•{}    FunctionDeclaration
    ()                                           FunctionDeclaration.parameters{dk: "()"}
       where•T:•A,•F:•FnOnce(B<T>)•->•bool       FunctionDeclaration.whereBounds{dk: "None"}
             T:•A                                WhereTypeBoundDeclaration
                A                                TypeTraitBound{!maybeConst, !optional}
                   F:•FnOnce(B<T>)•->•bool       WhereTypeBoundDeclaration
                      FnOnce(B<T>)•->•bool       TypeTraitBound{!maybeConst, !optional}, TypeFunction
                            (B<T>)               TypeFunction.parameters{dk: "()"}
                             B<T>                TypeCall
                              <T>                TypeCall.typeArguments{dk: "<>"}
                                           {}    FunctionDeclaration.body{dk: "{}"}                                                       */
fn f() -> impl std::borrow::Borrow<<u8 as A>::S> {}                                                                                       /*
fn•f()•->•impl•std::borrow::Borrow<<u8•as•A>::S>•{}    FunctionDeclaration
    ()                                                 FunctionDeclaration.parameters{dk: "()"}
          impl•std::borrow::Borrow<<u8•as•A>::S>       TypeImplBounds
               std::borrow::Borrow<<u8•as•A>::S>       TypeTraitBound{!maybeConst, !optional}, TypeCall
               std::borrow::Borrow                     TypePath
               std::borrow                             TypePath
                                  <<u8•as•A>::S>       TypeCall.typeArguments{dk: "<>"}
                                   <u8•as•A>::S        TypePath
                                   <u8•as•A>           ExpressionTypeSelector
                                                 {}    FunctionDeclaration.body{dk: "{}"}                                                 */
fn f(_: <() as A<Self::B>>::C) {}                                                                                                         /*
fn•f(_:•<()•as•A<Self::B>>::C)•{}    FunctionDeclaration
    (_:•<()•as•A<Self::B>>::C)       FunctionDeclaration.parameters{dk: "()"}
     _:•<()•as•A<Self::B>>::C        FunctionParameterDeclaration
     _                               WildcardPattern
        <()•as•A<Self::B>>::C        TypePath
        <()•as•A<Self::B>>           ExpressionTypeSelector
         ()                          TypeTuple
               A<Self::B>            TypeCall
                <Self::B>            TypeCall.typeArguments{dk: "<>"}
                 Self::B             TypePath
                               {}    FunctionDeclaration.body{dk: "{}"}                                                                   */
struct S<>;                                                                                                                               /*
struct•S<>;    StructDeclaration
        <>     StructDeclaration.generics{dk: "<>"}                                                                                       */
trait T<> {}                                                                                                                              /*
trait•T<>•{}    TraitDeclaration
       <>       TraitDeclaration.generics{dk: "<>"}
          {}    TraitDeclaration.body{dk: "{}"}                                                                                           */
enum E<> { V }                                                                                                                            /*
enum•E<>•{•V•}    EnumDeclaration
      <>          EnumDeclaration.generics{dk: "<>"}
         {•V•}    EnumDeclaration.members{dk: "{}"}
           V      EnumMemberDeclaration                                                                                                   */
impl<> T<> for S<> {}                                                                                                                     /*
impl<>•T<>•for•S<>•{}    ImplDeclaration{!const}
    <>                   ImplDeclaration.generics{dk: "<>"}
       T<>               TypeCall
        <>               TypeCall.typeArguments{dk: "<>"}
               S<>       TypeCall
                <>       TypeCall.typeArguments{dk: "<>"}
                   {}    ImplDeclaration.body{dk: "{}"}                                                                                   */
fn f<'a>(x: for<'b, 'c: 'a+'b> fn(&'a A, &'b B) -> &'c C)                                                                                 /*
fn•f<'a>(x:•for<'b,•'c:•'a+'b>•fn(&'a•A,•&'b•B)•->•&'c•C)•↲    <FunctionDeclaration>
    <'a>                                                       FunctionDeclaration.generics{dk: "<>"}
     'a                                                        GenericLtParameterDeclaration, LtIdentifier
        (x:•for<'b,•'c:•'a+'b>•fn(&'a•A,•&'b•B)•->•&'c•C)      FunctionDeclaration.parameters{dk: "()"}
         x:•for<'b,•'c:•'a+'b>•fn(&'a•A,•&'b•B)•->•&'c•C       FunctionParameterDeclaration
            for<'b,•'c:•'a+'b>•fn(&'a•A,•&'b•B)•->•&'c•C       TypeFnPointer
            for<'b,•'c:•'a+'b>                                 TypeFnPointer.ltParameters{dk: "<>"}
                'b                                             GenericLtParameterDeclaration, LtIdentifier
                    'c:•'a+'b                                  GenericLtParameterDeclaration
                    'c                                         LtIdentifier
                        'a                                     LtIdentifier
                           'b                                  LtIdentifier
                                 (&'a•A,•&'b•B)                TypeFnPointer.parameters{dk: "()"}
                                  &'a•A                        TypeFnPointerParameter, TypeReference{!mut}
                                   'a                          LtIdentifier
                                         &'b•B                 TypeFnPointerParameter, TypeReference{!mut}
                                          'b                   LtIdentifier
                                                   &'c•C       TypeReference{!mut}
                                                    'c         LtIdentifier                                                               */
   where F: for<'a, 'b: 'a>    Fn(&'a A, &'b B) -> &'c C,                                                                                 /*
   where•F:•for<'a,•'b:•'a>••••Fn(&'a•A,•&'b•B)•->•&'c•C,↲    <FunctionDeclaration.whereBounds{dk: "None"}>
         F:•for<'a,•'b:•'a>••••Fn(&'a•A,•&'b•B)•->•&'c•C      WhereTypeBoundDeclaration
            for<'a,•'b:•'a>••••Fn(&'a•A,•&'b•B)•->•&'c•C      TypeTraitBound{!maybeConst, !optional}
            for<'a,•'b:•'a>                                   TypeTraitBound.ltParameters{dk: "<>"}
                'a                                            GenericLtParameterDeclaration, LtIdentifier
                    'b:•'a                                    GenericLtParameterDeclaration
                    'b                                        LtIdentifier
                        'a                                    LtIdentifier
                               Fn(&'a•A,•&'b•B)•->•&'c•C      TypeFunction
                                 (&'a•A,•&'b•B)               TypeFunction.parameters{dk: "()"}
                                  &'a•A                       TypeReference{!mut}
                                   'a                         LtIdentifier
                                         &'b•B                TypeReference{!mut}
                                          'b                  LtIdentifier
                                                   &'c•C      TypeReference{!mut}
                                                    'c        LtIdentifier                                                                */
            for<'a, 'b: 'a> F: Fn(&'a A, &'b B) -> &'c C                                                                                  /*
            for<'a,•'b:•'a>•F:•Fn(&'a•A,•&'b•B)•->•&'c•C    WhereTypeBoundDeclaration
            for<'a,•'b:•'a>                                 WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                'a                                          GenericLtParameterDeclaration, LtIdentifier
                    'b:•'a                                  GenericLtParameterDeclaration
                    'b                                      LtIdentifier
                        'a                                  LtIdentifier
                               Fn(&'a•A,•&'b•B)•->•&'c•C    TypeTraitBound{!maybeConst, !optional}, TypeFunction
                                 (&'a•A,•&'b•B)             TypeFunction.parameters{dk: "()"}
                                  &'a•A                     TypeReference{!mut}
                                   'a                       LtIdentifier
                                         &'b•B              TypeReference{!mut}
                                          'b                LtIdentifier
                                                   &'c•C    TypeReference{!mut}
                                                    'c      LtIdentifier
••••••••••••for<'a,•'b:•'a>•F:•Fn(&'a•A,•&'b•B)•->•&'c•C    </FunctionDeclaration.whereBounds>                                            */
{}                                                                                                                                        /*
{}    FunctionDeclaration.body{dk: "{}"}
{}    </FunctionDeclaration>                                                                                                              */
struct S<F: for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C>(F);                                                                                /*
struct•S<F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C>(F);    TupleStructDeclaration
        <F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C>        TupleStructDeclaration.generics{dk: "<>"}
         F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C         GenericTypeParameterDeclaration
            for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C         TypeTraitBound{!maybeConst, !optional}
            for<'a,•'b:•'a>                                   TypeTraitBound.ltParameters{dk: "<>"}
                'a                                            GenericLtParameterDeclaration, LtIdentifier
                    'b:•'a                                    GenericLtParameterDeclaration
                    'b                                        LtIdentifier
                        'a                                    LtIdentifier
                            Fn(&'a•A,•&'b•B)•->•&'c•C         TypeFunction
                              (&'a•A,•&'b•B)                  TypeFunction.parameters{dk: "()"}
                               &'a•A                          TypeReference{!mut}
                                'a                            LtIdentifier
                                      &'b•B                   TypeReference{!mut}
                                       'b                     LtIdentifier
                                                &'c•C         TypeReference{!mut}
                                                 'c           LtIdentifier
                                                      (F)     TupleStructDeclaration.items{dk: "()"}
                                                       F      TupleStructItemDeclaration                                                  */
struct S<F>(F) where F: for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C;                                                                        /*
struct•S<F>(F)•where•F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C;    TupleStructDeclaration
        <F>                                                           TupleStructDeclaration.generics{dk: "<>"}
         F                                                            GenericTypeParameterDeclaration
           (F)                                                        TupleStructDeclaration.items{dk: "()"}
            F                                                         TupleStructItemDeclaration
               where•F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C     TupleStructDeclaration.whereBounds{dk: "None"}
                     F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C     WhereTypeBoundDeclaration
                        for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C     TypeTraitBound{!maybeConst, !optional}
                        for<'a,•'b:•'a>                               TypeTraitBound.ltParameters{dk: "<>"}
                            'a                                        GenericLtParameterDeclaration, LtIdentifier
                                'b:•'a                                GenericLtParameterDeclaration
                                'b                                    LtIdentifier
                                    'a                                LtIdentifier
                                        Fn(&'a•A,•&'b•B)•->•&'c•C     TypeFunction
                                          (&'a•A,•&'b•B)              TypeFunction.parameters{dk: "()"}
                                           &'a•A                      TypeReference{!mut}
                                            'a                        LtIdentifier
                                                  &'b•B               TypeReference{!mut}
                                                   'b                 LtIdentifier
                                                            &'c•C     TypeReference{!mut}
                                                             'c       LtIdentifier                                                        */
struct S(for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C);                                                                                      /*
struct•S(for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C);    TupleStructDeclaration
        (for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C)     TupleStructDeclaration.items{dk: "()"}
         for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C      TupleStructItemDeclaration, TypeDynBounds{!dyn}, TypeTraitBound{!maybeConst, !optional}
         for<'a,•'b:•'a>                                TypeTraitBound.ltParameters{dk: "<>"}
             'a                                         GenericLtParameterDeclaration, LtIdentifier
                 'b:•'a                                 GenericLtParameterDeclaration
                 'b                                     LtIdentifier
                     'a                                 LtIdentifier
                         Fn(&'a•A,•&'b•B)•->•&'c•C      TypeFunction
                           (&'a•A,•&'b•B)               TypeFunction.parameters{dk: "()"}
                            &'a•A                       TypeReference{!mut}
                             'a                         LtIdentifier
                                   &'b•B                TypeReference{!mut}
                                    'b                  LtIdentifier
                                             &'c•C      TypeReference{!mut}
                                              'c        LtIdentifier                                                                      */
type T = Box<dyn for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C>;                                                                              /*
type•T•=•Box<dyn•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C>;    TypeAliasDeclaration
         Box<dyn•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C>     TypeCall
            <dyn•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C>     TypeCall.typeArguments{dk: "<>"}
             dyn•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C      TypeDynBounds{dyn}
                 for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C      TypeTraitBound{!maybeConst, !optional}
                 for<'a,•'b:•'a>                                TypeTraitBound.ltParameters{dk: "<>"}
                     'a                                         GenericLtParameterDeclaration, LtIdentifier
                         'b:•'a                                 GenericLtParameterDeclaration
                         'b                                     LtIdentifier
                             'a                                 LtIdentifier
                                 Fn(&'a•A,•&'b•B)•->•&'c•C      TypeFunction
                                   (&'a•A,•&'b•B)               TypeFunction.parameters{dk: "()"}
                                    &'a•A                       TypeReference{!mut}
                                     'a                         LtIdentifier
                                           &'b•B                TypeReference{!mut}
                                            'b                  LtIdentifier
                                                     &'c•C      TypeReference{!mut}
                                                      'c        LtIdentifier                                                              */
type L8<T> = L<L<L<L<L<L<L<L<T>>>>>>>>;                                                                                                   /*
type•L8<T>•=•L<L<L<L<L<L<L<L<T>>>>>>>>;    TypeAliasDeclaration
       <T>                                 TypeAliasDeclaration.generics{dk: "<>"}
        T                                  GenericTypeParameterDeclaration
             L<L<L<L<L<L<L<L<T>>>>>>>>     TypeCall
              <L<L<L<L<L<L<L<T>>>>>>>>     TypeCall.typeArguments{dk: "<>"}
               L<L<L<L<L<L<L<T>>>>>>>      TypeCall
                <L<L<L<L<L<L<T>>>>>>>      TypeCall.typeArguments{dk: "<>"}
                 L<L<L<L<L<L<T>>>>>>       TypeCall
                  <L<L<L<L<L<T>>>>>>       TypeCall.typeArguments{dk: "<>"}
                   L<L<L<L<L<T>>>>>        TypeCall
                    <L<L<L<L<T>>>>>        TypeCall.typeArguments{dk: "<>"}
                     L<L<L<L<T>>>>         TypeCall
                      <L<L<L<T>>>>         TypeCall.typeArguments{dk: "<>"}
                       L<L<L<T>>>          TypeCall
                        <L<L<T>>>          TypeCall.typeArguments{dk: "<>"}
                         L<L<T>>           TypeCall
                          <L<T>>           TypeCall.typeArguments{dk: "<>"}
                           L<T>            TypeCall
                            <T>            TypeCall.typeArguments{dk: "<>"}                                                               */
type L64<T> = L8<L8<L8<L8<T>>>>;                                                                                                          /*
type•L64<T>•=•L8<L8<L8<L8<T>>>>;    TypeAliasDeclaration
        <T>                         TypeAliasDeclaration.generics{dk: "<>"}
         T                          GenericTypeParameterDeclaration
              L8<L8<L8<L8<T>>>>     TypeCall
                <L8<L8<L8<T>>>>     TypeCall.typeArguments{dk: "<>"}
                 L8<L8<L8<T>>>      TypeCall
                   <L8<L8<T>>>      TypeCall.typeArguments{dk: "<>"}
                    L8<L8<T>>       TypeCall
                      <L8<T>>       TypeCall.typeArguments{dk: "<>"}
                       L8<T>        TypeCall
                         <T>        TypeCall.typeArguments{dk: "<>"}                                                                      */
impl<T> A for T where T: B {                                                                                                              /*
impl<T>•A•for•T•where•T:•B•{↲    <ImplDeclaration{!const}>
    <T>                          ImplDeclaration.generics{dk: "<>"}
     T                           GenericTypeParameterDeclaration
                where•T:•B       ImplDeclaration.whereBounds{dk: "None"}
                      T:•B       WhereTypeBoundDeclaration
                         B       TypeTraitBound{!maybeConst, !optional}
                           {↲    <ImplDeclaration.body{dk: "{}"}>                                                                         */
    type C<'a> = <T as D>::E<'a, 'static>;                                                                                                /*
    type•C<'a>•=•<T•as•D>::E<'a,•'static>;    TypeAliasDeclaration
          <'a>                                TypeAliasDeclaration.generics{dk: "<>"}
           'a                                 GenericLtParameterDeclaration, LtIdentifier
                 <T•as•D>::E<'a,•'static>     TypeCall
                 <T•as•D>::E                  TypePath
                 <T•as•D>                     ExpressionTypeSelector
                            <'a,•'static>     TypeCall.typeArguments{dk: "<>"}
                             'a               LtIdentifier
                                 'static      LtStatic                                                                                    */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */
impl<T> A<<() as B<T::C>>::D> for E<T>                                                                                                    /*
impl<T>•A<<()•as•B<T::C>>::D>•for•E<T>↲    <ImplDeclaration{!const}>
    <T>                                    ImplDeclaration.generics{dk: "<>"}
     T                                     GenericTypeParameterDeclaration
        A<<()•as•B<T::C>>::D>              TypeCall
         <<()•as•B<T::C>>::D>              TypeCall.typeArguments{dk: "<>"}
          <()•as•B<T::C>>::D               TypePath
          <()•as•B<T::C>>                  ExpressionTypeSelector
           ()                              TypeTuple
                 B<T::C>                   TypeCall
                  <T::C>                   TypeCall.typeArguments{dk: "<>"}
                   T::C                    TypePath
                                  E<T>     TypeCall
                                   <T>     TypeCall.typeArguments{dk: "<>"}                                                               */
where T: F, (): G<T::H>,                                                                                                                  /*
where•T:•F,•():•G<T::H>,    ImplDeclaration.whereBounds{dk: "None"}
      T:•F                  WhereTypeBoundDeclaration
         F                  TypeTraitBound{!maybeConst, !optional}
            ():•G<T::H>     WhereTypeBoundDeclaration
            ()              TypeTuple
                G<T::H>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                 <T::H>     TypeCall.typeArguments{dk: "<>"}
                  T::H      TypePath                                                                                                      */
{}                                                                                                                                        /*
{}    ImplDeclaration.body{dk: "{}"}
{}    </ImplDeclaration>                                                                                                                  */
type Y<'a> = &'a ();                                                                                                                      /*
type•Y<'a>•=•&'a•();    TypeAliasDeclaration
      <'a>              TypeAliasDeclaration.generics{dk: "<>"}
       'a               GenericLtParameterDeclaration, LtIdentifier
             &'a•()     TypeReference{!mut}
              'a        LtIdentifier
                 ()     TypeTuple                                                                                                         */
type Q<'a>;                                                                                                                               /*
type•Q<'a>;    TypeAliasDeclaration
      <'a>     TypeAliasDeclaration.generics{dk: "<>"}
       'a      GenericLtParameterDeclaration, LtIdentifier                                                                                */
type Q<'a, 'b>;                                                                                                                           /*
type•Q<'a,•'b>;    TypeAliasDeclaration
      <'a,•'b>     TypeAliasDeclaration.generics{dk: "<>"}
       'a          GenericLtParameterDeclaration, LtIdentifier
           'b      GenericLtParameterDeclaration, LtIdentifier                                                                            */
type Q<'a, 'b,>;                                                                                                                          /*
type•Q<'a,•'b,>;    TypeAliasDeclaration
      <'a,•'b,>     TypeAliasDeclaration.generics{dk: "<>"}
       'a           GenericLtParameterDeclaration, LtIdentifier
           'b       GenericLtParameterDeclaration, LtIdentifier                                                                           */
type Q<'a, 'b, T>;                                                                                                                        /*
type•Q<'a,•'b,•T>;    TypeAliasDeclaration
      <'a,•'b,•T>     TypeAliasDeclaration.generics{dk: "<>"}
       'a             GenericLtParameterDeclaration, LtIdentifier
           'b         GenericLtParameterDeclaration, LtIdentifier
               T      GenericTypeParameterDeclaration                                                                                     */
type Q<'a, 'b, T, U>;                                                                                                                     /*
type•Q<'a,•'b,•T,•U>;    TypeAliasDeclaration
      <'a,•'b,•T,•U>     TypeAliasDeclaration.generics{dk: "<>"}
       'a                GenericLtParameterDeclaration, LtIdentifier
           'b            GenericLtParameterDeclaration, LtIdentifier
               T         GenericTypeParameterDeclaration
                  U      GenericTypeParameterDeclaration                                                                                  */
type Q<'a, 'b, T, U,>;                                                                                                                    /*
type•Q<'a,•'b,•T,•U,>;    TypeAliasDeclaration
      <'a,•'b,•T,•U,>     TypeAliasDeclaration.generics{dk: "<>"}
       'a                 GenericLtParameterDeclaration, LtIdentifier
           'b             GenericLtParameterDeclaration, LtIdentifier
               T          GenericTypeParameterDeclaration
                  U       GenericTypeParameterDeclaration                                                                                 */
type Q<'a, 'b, T: S, U,>;                                                                                                                 /*
type•Q<'a,•'b,•T:•S,•U,>;    TypeAliasDeclaration
      <'a,•'b,•T:•S,•U,>     TypeAliasDeclaration.generics{dk: "<>"}
       'a                    GenericLtParameterDeclaration, LtIdentifier
           'b                GenericLtParameterDeclaration, LtIdentifier
               T:•S          GenericTypeParameterDeclaration
                  S          TypeTraitBound{!maybeConst, !optional}
                     U       GenericTypeParameterDeclaration                                                                              */
type Q<'a, 'b, T: S, U,>: S;                                                                                                              /*
type•Q<'a,•'b,•T:•S,•U,>:•S;    TypeAliasDeclaration
      <'a,•'b,•T:•S,•U,>        TypeAliasDeclaration.generics{dk: "<>"}
       'a                       GenericLtParameterDeclaration, LtIdentifier
           'b                   GenericLtParameterDeclaration, LtIdentifier
               T:•S             GenericTypeParameterDeclaration
                  S             TypeTraitBound{!maybeConst, !optional}
                     U          GenericTypeParameterDeclaration
                          S     TypeTraitBound{!maybeConst, !optional}                                                                    */
type Q<'a, 'b, T: S, U,>: E<Target = T> + Into<U>;                                                                                        /*
type•Q<'a,•'b,•T:•S,•U,>:•E<Target•=•T>•+•Into<U>;    TypeAliasDeclaration
      <'a,•'b,•T:•S,•U,>                              TypeAliasDeclaration.generics{dk: "<>"}
       'a                                             GenericLtParameterDeclaration, LtIdentifier
           'b                                         GenericLtParameterDeclaration, LtIdentifier
               T:•S                                   GenericTypeParameterDeclaration
                  S                                   TypeTraitBound{!maybeConst, !optional}
                     U                                GenericTypeParameterDeclaration
                          E<Target•=•T>               TypeTraitBound{!maybeConst, !optional}, TypeCall
                           <Target•=•T>               TypeCall.typeArguments{dk: "<>"}
                            Target•=•T                TypeCallNamedArgument
                                          Into<U>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                                              <U>     TypeCall.typeArguments{dk: "<>"}                                                    */
type Q<'a, 'b, T: S, U,> where T: E<Target = U>, U: Into<T>;                                                                              /*
type•Q<'a,•'b,•T:•S,•U,>•where•T:•E<Target•=•U>,•U:•Into<T>;    TypeAliasDeclaration
      <'a,•'b,•T:•S,•U,>                                        TypeAliasDeclaration.generics{dk: "<>"}
       'a                                                       GenericLtParameterDeclaration, LtIdentifier
           'b                                                   GenericLtParameterDeclaration, LtIdentifier
               T:•S                                             GenericTypeParameterDeclaration
                  S                                             TypeTraitBound{!maybeConst, !optional}
                     U                                          GenericTypeParameterDeclaration
                         where•T:•E<Target•=•U>,•U:•Into<T>     TypeAliasDeclaration.whereBounds{dk: "None"}
                               T:•E<Target•=•U>                 WhereTypeBoundDeclaration
                                  E<Target•=•U>                 TypeTraitBound{!maybeConst, !optional}, TypeCall
                                   <Target•=•U>                 TypeCall.typeArguments{dk: "<>"}
                                    Target•=•U                  TypeCallNamedArgument
                                                 U:•Into<T>     WhereTypeBoundDeclaration
                                                    Into<T>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                        <T>     TypeCall.typeArguments{dk: "<>"}                                          */
type Q<'a, 'b, T: S, U,>: E<Target = T> + Into<U> where T: E<Target = U>, U: Into<T>;                                                     /*
type•Q<'a,•'b,•T:•S,•U,>:•E<Target•=•T>•+•Into<U>•where•T:•E<Target•=•U>,•U:•Into<T>;    TypeAliasDeclaration
      <'a,•'b,•T:•S,•U,>                                                                 TypeAliasDeclaration.generics{dk: "<>"}
       'a                                                                                GenericLtParameterDeclaration, LtIdentifier
           'b                                                                            GenericLtParameterDeclaration, LtIdentifier
               T:•S                                                                      GenericTypeParameterDeclaration
                  S                                                                      TypeTraitBound{!maybeConst, !optional}
                     U                                                                   GenericTypeParameterDeclaration
                          E<Target•=•T>                                                  TypeTraitBound{!maybeConst, !optional}, TypeCall
                           <Target•=•T>                                                  TypeCall.typeArguments{dk: "<>"}
                            Target•=•T                                                   TypeCallNamedArgument
                                          Into<U>                                        TypeTraitBound{!maybeConst, !optional}, TypeCall
                                              <U>                                        TypeCall.typeArguments{dk: "<>"}
                                                  where•T:•E<Target•=•U>,•U:•Into<T>     TypeAliasDeclaration.whereBounds{dk: "None"}
                                                        T:•E<Target•=•U>                 WhereTypeBoundDeclaration
                                                           E<Target•=•U>                 TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                            <Target•=•U>                 TypeCall.typeArguments{dk: "<>"}
                                                             Target•=•U                  TypeCallNamedArgument
                                                                          U:•Into<T>     WhereTypeBoundDeclaration
                                                                             Into<T>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                                                 <T>     TypeCall.typeArguments{dk: "<>"}                 */
type Q<'a>: E<Target = <Self::D<'a> as B>::A<'a, 'static>> where Self: 'a;                                                                /*
type•Q<'a>:•E<Target•=•<Self::D<'a>•as•B>::A<'a,•'static>>•where•Self:•'a;    TypeAliasDeclaration
      <'a>                                                                    TypeAliasDeclaration.generics{dk: "<>"}
       'a                                                                     GenericLtParameterDeclaration, LtIdentifier
            E<Target•=•<Self::D<'a>•as•B>::A<'a,•'static>>                    TypeTraitBound{!maybeConst, !optional}, TypeCall
             <Target•=•<Self::D<'a>•as•B>::A<'a,•'static>>                    TypeCall.typeArguments{dk: "<>"}
              Target•=•<Self::D<'a>•as•B>::A<'a,•'static>                     TypeCallNamedArgument
                       <Self::D<'a>•as•B>::A<'a,•'static>                     TypeCall
                       <Self::D<'a>•as•B>::A                                  TypePath
                       <Self::D<'a>•as•B>                                     ExpressionTypeSelector
                        Self::D<'a>                                           TypeCall
                        Self::D                                               TypePath
                               <'a>                                           TypeCall.typeArguments{dk: "<>"}
                                'a                                            LtIdentifier
                                            <'a,•'static>                     TypeCall.typeArguments{dk: "<>"}
                                             'a                               LtIdentifier
                                                 'static                      LtStatic
                                                           where•Self:•'a     TypeAliasDeclaration.whereBounds{dk: "None"}
                                                                 Self:•'a     WhereTypeBoundDeclaration
                                                                       'a     LtIdentifier                                                */
type S<'a>: Iterator<Q = Self::A<'a>> + E<R = Self::B<'b>>;                                                                               /*
type•S<'a>:•Iterator<Q•=•Self::A<'a>>•+•E<R•=•Self::B<'b>>;    TypeAliasDeclaration
      <'a>                                                     TypeAliasDeclaration.generics{dk: "<>"}
       'a                                                      GenericLtParameterDeclaration, LtIdentifier
            Iterator<Q•=•Self::A<'a>>                          TypeTraitBound{!maybeConst, !optional}, TypeCall
                    <Q•=•Self::A<'a>>                          TypeCall.typeArguments{dk: "<>"}
                     Q•=•Self::A<'a>                           TypeCallNamedArgument
                         Self::A<'a>                           TypeCall
                         Self::A                               TypePath
                                <'a>                           TypeCall.typeArguments{dk: "<>"}
                                 'a                            LtIdentifier
                                        E<R•=•Self::B<'b>>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                                         <R•=•Self::B<'b>>     TypeCall.typeArguments{dk: "<>"}
                                          R•=•Self::B<'b>      TypeCallNamedArgument
                                              Self::B<'b>      TypeCall
                                              Self::B          TypePath
                                                     <'b>      TypeCall.typeArguments{dk: "<>"}
                                                      'b       LtIdentifier                                                               */
type Z = dyn for<'x> Send;                                                                                                                /*
type•Z•=•dyn•for<'x>•Send;    TypeAliasDeclaration
         dyn•for<'x>•Send     TypeDynBounds{dyn}
             for<'x>•Send     TypeTraitBound{!maybeConst, !optional}
             for<'x>          TypeTraitBound.ltParameters{dk: "<>"}
                 'x           GenericLtParameterDeclaration, LtIdentifier                                                                 */
type A = (*const E::R, D);                                                                                                                /*
type•A•=•(*const•E::R,•D);    TypeAliasDeclaration
         (*const•E::R,•D)     TypeTuple
          *const•E::R         TypeDereferenceConst
                 E::R         TypePath                                                                                                    */
fn f(&self) -> Pin<Box<dyn Future<Output = Self::B> + '_>>;                                                                               /*
fn•f(&self)•->•Pin<Box<dyn•Future<Output•=•Self::B>•+•'_>>;    FunctionDeclaration
    (&self)                                                    FunctionDeclaration.parameters{dk: "()"}
     &self                                                     FunctionSelfParameterDeclaration{ref, !mut}
               Pin<Box<dyn•Future<Output•=•Self::B>•+•'_>>     TypeCall
                  <Box<dyn•Future<Output•=•Self::B>•+•'_>>     TypeCall.typeArguments{dk: "<>"}
                   Box<dyn•Future<Output•=•Self::B>•+•'_>      TypeCall
                      <dyn•Future<Output•=•Self::B>•+•'_>      TypeCall.typeArguments{dk: "<>"}
                       dyn•Future<Output•=•Self::B>•+•'_       TypeDynBounds{dyn}
                           Future<Output•=•Self::B>            TypeTraitBound{!maybeConst, !optional}, TypeCall
                                 <Output•=•Self::B>            TypeCall.typeArguments{dk: "<>"}
                                  Output•=•Self::B             TypeCallNamedArgument
                                           Self::B             TypePath
                                                      '_       LtElided                                                                   */
fn f(&self) -> Self::Y<'_>{}                                                                                                              /*
fn•f(&self)•->•Self::Y<'_>{}    FunctionDeclaration
    (&self)                     FunctionDeclaration.parameters{dk: "()"}
     &self                      FunctionSelfParameterDeclaration{ref, !mut}
               Self::Y<'_>      TypeCall
               Self::Y          TypePath
                      <'_>      TypeCall.typeArguments{dk: "<>"}
                       '_       LtElided
                          {}    FunctionDeclaration.body{dk: "{}"}                                                                        */
fn f(x: &()) -> &() {}                                                                                                                    /*
fn•f(x:•&())•->•&()•{}    FunctionDeclaration
    (x:•&())              FunctionDeclaration.parameters{dk: "()"}
     x:•&()               FunctionParameterDeclaration
        &()               TypeReference{!mut}
         ()               TypeTuple
                &()       TypeReference{!mut}
                 ()       TypeTuple
                    {}    FunctionDeclaration.body{dk: "{}"}                                                                              */
fn f(x: &impl for<'a> X<Y<'a> = &'a ()>) -> &() {}                                                                                        /*
fn•f(x:•&impl•for<'a>•X<Y<'a>•=•&'a•()>)•->•&()•{}    FunctionDeclaration
    (x:•&impl•for<'a>•X<Y<'a>•=•&'a•()>)              FunctionDeclaration.parameters{dk: "()"}
     x:•&impl•for<'a>•X<Y<'a>•=•&'a•()>               FunctionParameterDeclaration
        &impl•for<'a>•X<Y<'a>•=•&'a•()>               TypeReference{!mut}
         impl•for<'a>•X<Y<'a>•=•&'a•()>               TypeImplBounds
              for<'a>•X<Y<'a>•=•&'a•()>               TypeTraitBound{!maybeConst, !optional}
              for<'a>                                 TypeTraitBound.ltParameters{dk: "<>"}
                  'a                                  GenericLtParameterDeclaration, LtIdentifier
                      X<Y<'a>•=•&'a•()>               TypeCall
                       <Y<'a>•=•&'a•()>               TypeCall.typeArguments{dk: "<>"}
                        Y<'a>•=•&'a•()                TypeCallNamedArgument
                        Y<'a>                         TypeCall
                         <'a>                         TypeCall.typeArguments{dk: "<>"}
                          'a                          LtIdentifier
                                &'a•()                TypeReference{!mut}
                                 'a                   LtIdentifier
                                    ()                TypeTuple
                                            &()       TypeReference{!mut}
                                             ()       TypeTuple
                                                {}    FunctionDeclaration.body{dk: "{}"}                                                  */
fn f<'a, T: for<'b> Fun<F<'b> = T>>(t: T) -> T::F<'a>{}                                                                                   /*
fn•f<'a,•T:•for<'b>•Fun<F<'b>•=•T>>(t:•T)•->•T::F<'a>{}    FunctionDeclaration
    <'a,•T:•for<'b>•Fun<F<'b>•=•T>>                        FunctionDeclaration.generics{dk: "<>"}
     'a                                                    GenericLtParameterDeclaration, LtIdentifier
         T:•for<'b>•Fun<F<'b>•=•T>                         GenericTypeParameterDeclaration
            for<'b>•Fun<F<'b>•=•T>                         TypeTraitBound{!maybeConst, !optional}
            for<'b>                                        TypeTraitBound.ltParameters{dk: "<>"}
                'b                                         GenericLtParameterDeclaration, LtIdentifier
                    Fun<F<'b>•=•T>                         TypeCall
                       <F<'b>•=•T>                         TypeCall.typeArguments{dk: "<>"}
                        F<'b>•=•T                          TypeCallNamedArgument
                        F<'b>                              TypeCall
                         <'b>                              TypeCall.typeArguments{dk: "<>"}
                          'b                               LtIdentifier
                                   (t:•T)                  FunctionDeclaration.parameters{dk: "()"}
                                    t:•T                   FunctionParameterDeclaration
                                             T::F<'a>      TypeCall
                                             T::F          TypePath
                                                 <'a>      TypeCall.typeArguments{dk: "<>"}
                                                  'a       LtIdentifier
                                                     {}    FunctionDeclaration.body{dk: "{}"}                                             */
fn f<'a, T: Fun<F<'a> = T>>(t: T) -> T::F<'a> {}                                                                                          /*
fn•f<'a,•T:•Fun<F<'a>•=•T>>(t:•T)•->•T::F<'a>•{}    FunctionDeclaration
    <'a,•T:•Fun<F<'a>•=•T>>                         FunctionDeclaration.generics{dk: "<>"}
     'a                                             GenericLtParameterDeclaration, LtIdentifier
         T:•Fun<F<'a>•=•T>                          GenericTypeParameterDeclaration
            Fun<F<'a>•=•T>                          TypeTraitBound{!maybeConst, !optional}, TypeCall
               <F<'a>•=•T>                          TypeCall.typeArguments{dk: "<>"}
                F<'a>•=•T                           TypeCallNamedArgument
                F<'a>                               TypeCall
                 <'a>                               TypeCall.typeArguments{dk: "<>"}
                  'a                                LtIdentifier
                           (t:•T)                   FunctionDeclaration.parameters{dk: "()"}
                            t:•T                    FunctionParameterDeclaration
                                     T::F<'a>       TypeCall
                                     T::F           TypePath
                                         <'a>       TypeCall.typeArguments{dk: "<>"}
                                          'a        LtIdentifier
                                              {}    FunctionDeclaration.body{dk: "{}"}                                                    */
fn f<T: ?for<'a> Sized>() {}                                                                                                              /*
fn•f<T:•?for<'a>•Sized>()•{}    FunctionDeclaration
    <T:•?for<'a>•Sized>         FunctionDeclaration.generics{dk: "<>"}
     T:•?for<'a>•Sized          GenericTypeParameterDeclaration
        ?for<'a>•Sized          TypeTraitBound{!maybeConst, optional}
         for<'a>                TypeTraitBound.ltParameters{dk: "<>"}
             'a                 GenericLtParameterDeclaration, LtIdentifier
                       ()       FunctionDeclaration.parameters{dk: "()"}
                          {}    FunctionDeclaration.body{dk: "{}"}                                                                        */
fn f<'a, T1: X<Y = T1>>(t : T1) -> T1::Y<'a>;                                                                                             /*
fn•f<'a,•T1:•X<Y•=•T1>>(t•:•T1)•->•T1::Y<'a>;    FunctionDeclaration
    <'a,•T1:•X<Y•=•T1>>                          FunctionDeclaration.generics{dk: "<>"}
     'a                                          GenericLtParameterDeclaration, LtIdentifier
         T1:•X<Y•=•T1>                           GenericTypeParameterDeclaration
             X<Y•=•T1>                           TypeTraitBound{!maybeConst, !optional}, TypeCall
              <Y•=•T1>                           TypeCall.typeArguments{dk: "<>"}
               Y•=•T1                            TypeCallNamedArgument
                       (t•:•T1)                  FunctionDeclaration.parameters{dk: "()"}
                        t•:•T1                   FunctionParameterDeclaration
                                   T1::Y<'a>     TypeCall
                                   T1::Y         TypePath
                                        <'a>     TypeCall.typeArguments{dk: "<>"}
                                         'a      LtIdentifier                                                                             */
fn f<'a>(s: Box<dyn X<Y<'a>=&'a ()>>) {}                                                                                                  /*
fn•f<'a>(s:•Box<dyn•X<Y<'a>=&'a•()>>)•{}    FunctionDeclaration
    <'a>                                    FunctionDeclaration.generics{dk: "<>"}
     'a                                     GenericLtParameterDeclaration, LtIdentifier
        (s:•Box<dyn•X<Y<'a>=&'a•()>>)       FunctionDeclaration.parameters{dk: "()"}
         s:•Box<dyn•X<Y<'a>=&'a•()>>        FunctionParameterDeclaration
            Box<dyn•X<Y<'a>=&'a•()>>        TypeCall
               <dyn•X<Y<'a>=&'a•()>>        TypeCall.typeArguments{dk: "<>"}
                dyn•X<Y<'a>=&'a•()>         TypeDynBounds{dyn}
                    X<Y<'a>=&'a•()>         TypeTraitBound{!maybeConst, !optional}, TypeCall
                     <Y<'a>=&'a•()>         TypeCall.typeArguments{dk: "<>"}
                      Y<'a>=&'a•()          TypeCallNamedArgument
                      Y<'a>                 TypeCall
                       <'a>                 TypeCall.typeArguments{dk: "<>"}
                        'a                  LtIdentifier
                            &'a•()          TypeReference{!mut}
                             'a             LtIdentifier
                                ()          TypeTuple
                                      {}    FunctionDeclaration.body{dk: "{}"}                                                            */
fn f<'a>(t : Self::Y<'a>) -> Self::Y<'a>;                                                                                                 /*
fn•f<'a>(t•:•Self::Y<'a>)•->•Self::Y<'a>;    FunctionDeclaration
    <'a>                                     FunctionDeclaration.generics{dk: "<>"}
     'a                                      GenericLtParameterDeclaration, LtIdentifier
        (t•:•Self::Y<'a>)                    FunctionDeclaration.parameters{dk: "()"}
         t•:•Self::Y<'a>                     FunctionParameterDeclaration
             Self::Y<'a>                     TypeCall
             Self::Y                         TypePath
                    <'a>                     TypeCall.typeArguments{dk: "<>"}
                     'a                      LtIdentifier
                             Self::Y<'a>     TypeCall
                             Self::Y         TypePath
                                    <'a>     TypeCall.typeArguments{dk: "<>"}
                                     'a      LtIdentifier                                                                                 */
fn f<T: for<'a> X<Y<'a> = &'a ()>>(x: &T) -> &() {}                                                                                       /*
fn•f<T:•for<'a>•X<Y<'a>•=•&'a•()>>(x:•&T)•->•&()•{}    FunctionDeclaration
    <T:•for<'a>•X<Y<'a>•=•&'a•()>>                     FunctionDeclaration.generics{dk: "<>"}
     T:•for<'a>•X<Y<'a>•=•&'a•()>                      GenericTypeParameterDeclaration
        for<'a>•X<Y<'a>•=•&'a•()>                      TypeTraitBound{!maybeConst, !optional}
        for<'a>                                        TypeTraitBound.ltParameters{dk: "<>"}
            'a                                         GenericLtParameterDeclaration, LtIdentifier
                X<Y<'a>•=•&'a•()>                      TypeCall
                 <Y<'a>•=•&'a•()>                      TypeCall.typeArguments{dk: "<>"}
                  Y<'a>•=•&'a•()                       TypeCallNamedArgument
                  Y<'a>                                TypeCall
                   <'a>                                TypeCall.typeArguments{dk: "<>"}
                    'a                                 LtIdentifier
                          &'a•()                       TypeReference{!mut}
                           'a                          LtIdentifier
                              ()                       TypeTuple
                                  (x:•&T)              FunctionDeclaration.parameters{dk: "()"}
                                   x:•&T               FunctionParameterDeclaration
                                      &T               TypeReference{!mut}
                                             &()       TypeReference{!mut}
                                              ()       TypeTuple
                                                 {}    FunctionDeclaration.body{dk: "{}"}                                                 */
fn f<'a, T: ?Sized + Fun<F<'a> = [u8]>>(_ : Box<T>) -> &'static T::F<'a> {}                                                               /*
fn•f<'a,•T:•?Sized•+•Fun<F<'a>•=•[u8]>>(_•:•Box<T>)•->•&'static•T::F<'a>•{}    FunctionDeclaration
    <'a,•T:•?Sized•+•Fun<F<'a>•=•[u8]>>                                        FunctionDeclaration.generics{dk: "<>"}
     'a                                                                        GenericLtParameterDeclaration, LtIdentifier
         T:•?Sized•+•Fun<F<'a>•=•[u8]>                                         GenericTypeParameterDeclaration
            ?Sized                                                             TypeTraitBound{!maybeConst, optional}
                     Fun<F<'a>•=•[u8]>                                         TypeTraitBound{!maybeConst, !optional}, TypeCall
                        <F<'a>•=•[u8]>                                         TypeCall.typeArguments{dk: "<>"}
                         F<'a>•=•[u8]                                          TypeCallNamedArgument
                         F<'a>                                                 TypeCall
                          <'a>                                                 TypeCall.typeArguments{dk: "<>"}
                           'a                                                  LtIdentifier
                                 [u8]                                          TypeSlice
                                       (_•:•Box<T>)                            FunctionDeclaration.parameters{dk: "()"}
                                        _•:•Box<T>                             FunctionParameterDeclaration
                                        _                                      WildcardPattern
                                            Box<T>                             TypeCall
                                               <T>                             TypeCall.typeArguments{dk: "<>"}
                                                       &'static•T::F<'a>       TypeReference{!mut}
                                                        'static                LtStatic
                                                                T::F<'a>       TypeCall
                                                                T::F           TypePath
                                                                    <'a>       TypeCall.typeArguments{dk: "<>"}
                                                                     'a        LtIdentifier
                                                                         {}    FunctionDeclaration.body{dk: "{}"}                         */
fn f<'a>(t: &'a Self::F<'a>) -> &'a Self::F<'a>{}                                                                                         /*
fn•f<'a>(t:•&'a•Self::F<'a>)•->•&'a•Self::F<'a>{}    FunctionDeclaration
    <'a>                                             FunctionDeclaration.generics{dk: "<>"}
     'a                                              GenericLtParameterDeclaration, LtIdentifier
        (t:•&'a•Self::F<'a>)                         FunctionDeclaration.parameters{dk: "()"}
         t:•&'a•Self::F<'a>                          FunctionParameterDeclaration
            &'a•Self::F<'a>                          TypeReference{!mut}
             'a                                      LtIdentifier
                Self::F<'a>                          TypeCall
                Self::F                              TypePath
                       <'a>                          TypeCall.typeArguments{dk: "<>"}
                        'a                           LtIdentifier
                                &'a•Self::F<'a>      TypeReference{!mut}
                                 'a                  LtIdentifier
                                    Self::F<'a>      TypeCall
                                    Self::F          TypePath
                                           <'a>      TypeCall.typeArguments{dk: "<>"}
                                            'a       LtIdentifier
                                               {}    FunctionDeclaration.body{dk: "{}"}                                                   */
fn f<T>() where T: S, for<'a> T::Item<'a>: Q {}                                                                                           /*
fn•f<T>()•where•T:•S,•for<'a>•T::Item<'a>:•Q•{}    FunctionDeclaration
    <T>                                            FunctionDeclaration.generics{dk: "<>"}
     T                                             GenericTypeParameterDeclaration
       ()                                          FunctionDeclaration.parameters{dk: "()"}
          where•T:•S,•for<'a>•T::Item<'a>:•Q       FunctionDeclaration.whereBounds{dk: "None"}
                T:•S                               WhereTypeBoundDeclaration
                   S                               TypeTraitBound{!maybeConst, !optional}
                      for<'a>•T::Item<'a>:•Q       WhereTypeBoundDeclaration
                      for<'a>                      WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                          'a                       GenericLtParameterDeclaration, LtIdentifier
                              T::Item<'a>          TypeCall
                              T::Item              TypePath
                                     <'a>          TypeCall.typeArguments{dk: "<>"}
                                      'a           LtIdentifier
                                           Q       TypeTraitBound{!maybeConst, !optional}
                                             {}    FunctionDeclaration.body{dk: "{}"}                                                     */
fn f<'c, 'd>(s: Box<dyn X<Y = (&'c u32, &'d u32)>>) {}                                                                                    /*
fn•f<'c,•'d>(s:•Box<dyn•X<Y•=•(&'c•u32,•&'d•u32)>>)•{}    FunctionDeclaration
    <'c,•'d>                                              FunctionDeclaration.generics{dk: "<>"}
     'c                                                   GenericLtParameterDeclaration, LtIdentifier
         'd                                               GenericLtParameterDeclaration, LtIdentifier
            (s:•Box<dyn•X<Y•=•(&'c•u32,•&'d•u32)>>)       FunctionDeclaration.parameters{dk: "()"}
             s:•Box<dyn•X<Y•=•(&'c•u32,•&'d•u32)>>        FunctionParameterDeclaration
                Box<dyn•X<Y•=•(&'c•u32,•&'d•u32)>>        TypeCall
                   <dyn•X<Y•=•(&'c•u32,•&'d•u32)>>        TypeCall.typeArguments{dk: "<>"}
                    dyn•X<Y•=•(&'c•u32,•&'d•u32)>         TypeDynBounds{dyn}
                        X<Y•=•(&'c•u32,•&'d•u32)>         TypeTraitBound{!maybeConst, !optional}, TypeCall
                         <Y•=•(&'c•u32,•&'d•u32)>         TypeCall.typeArguments{dk: "<>"}
                          Y•=•(&'c•u32,•&'d•u32)          TypeCallNamedArgument
                              (&'c•u32,•&'d•u32)          TypeTuple
                               &'c•u32                    TypeReference{!mut}
                                'c                        LtIdentifier
                                        &'d•u32           TypeReference{!mut}
                                         'd               LtIdentifier
                                                    {}    FunctionDeclaration.body{dk: "{}"}                                              */
fn f(e: &impl for<'a> X<Y<'a> = &'a ()>) -> &'static () {}                                                                                /*
fn•f(e:•&impl•for<'a>•X<Y<'a>•=•&'a•()>)•->•&'static•()•{}    FunctionDeclaration
    (e:•&impl•for<'a>•X<Y<'a>•=•&'a•()>)                      FunctionDeclaration.parameters{dk: "()"}
     e:•&impl•for<'a>•X<Y<'a>•=•&'a•()>                       FunctionParameterDeclaration
        &impl•for<'a>•X<Y<'a>•=•&'a•()>                       TypeReference{!mut}
         impl•for<'a>•X<Y<'a>•=•&'a•()>                       TypeImplBounds
              for<'a>•X<Y<'a>•=•&'a•()>                       TypeTraitBound{!maybeConst, !optional}
              for<'a>                                         TypeTraitBound.ltParameters{dk: "<>"}
                  'a                                          GenericLtParameterDeclaration, LtIdentifier
                      X<Y<'a>•=•&'a•()>                       TypeCall
                       <Y<'a>•=•&'a•()>                       TypeCall.typeArguments{dk: "<>"}
                        Y<'a>•=•&'a•()                        TypeCallNamedArgument
                        Y<'a>                                 TypeCall
                         <'a>                                 TypeCall.typeArguments{dk: "<>"}
                          'a                                  LtIdentifier
                                &'a•()                        TypeReference{!mut}
                                 'a                           LtIdentifier
                                    ()                        TypeTuple
                                            &'static•()       TypeReference{!mut}
                                             'static          LtStatic
                                                     ()       TypeTuple
                                                        {}    FunctionDeclaration.body{dk: "{}"}                                          */
fn f<T: for<'a> X<Y<'a> = &'a ()>>(x: &T) -> &'static () {}                                                                               /*
fn•f<T:•for<'a>•X<Y<'a>•=•&'a•()>>(x:•&T)•->•&'static•()•{}    FunctionDeclaration
    <T:•for<'a>•X<Y<'a>•=•&'a•()>>                             FunctionDeclaration.generics{dk: "<>"}
     T:•for<'a>•X<Y<'a>•=•&'a•()>                              GenericTypeParameterDeclaration
        for<'a>•X<Y<'a>•=•&'a•()>                              TypeTraitBound{!maybeConst, !optional}
        for<'a>                                                TypeTraitBound.ltParameters{dk: "<>"}
            'a                                                 GenericLtParameterDeclaration, LtIdentifier
                X<Y<'a>•=•&'a•()>                              TypeCall
                 <Y<'a>•=•&'a•()>                              TypeCall.typeArguments{dk: "<>"}
                  Y<'a>•=•&'a•()                               TypeCallNamedArgument
                  Y<'a>                                        TypeCall
                   <'a>                                        TypeCall.typeArguments{dk: "<>"}
                    'a                                         LtIdentifier
                          &'a•()                               TypeReference{!mut}
                           'a                                  LtIdentifier
                              ()                               TypeTuple
                                  (x:•&T)                      FunctionDeclaration.parameters{dk: "()"}
                                   x:•&T                       FunctionParameterDeclaration
                                      &T                       TypeReference{!mut}
                                             &'static•()       TypeReference{!mut}
                                              'static          LtStatic
                                                      ()       TypeTuple
                                                         {}    FunctionDeclaration.body{dk: "{}"}                                         */
fn f(x: &mut dyn for<'a> E<R<'a> = &'a i32>) -> usize {}                                                                                  /*
fn•f(x:•&mut•dyn•for<'a>•E<R<'a>•=•&'a•i32>)•->•usize•{}    FunctionDeclaration
    (x:•&mut•dyn•for<'a>•E<R<'a>•=•&'a•i32>)                FunctionDeclaration.parameters{dk: "()"}
     x:•&mut•dyn•for<'a>•E<R<'a>•=•&'a•i32>                 FunctionParameterDeclaration
        &mut•dyn•for<'a>•E<R<'a>•=•&'a•i32>                 TypeReference{mut}
             dyn•for<'a>•E<R<'a>•=•&'a•i32>                 TypeDynBounds{dyn}
                 for<'a>•E<R<'a>•=•&'a•i32>                 TypeTraitBound{!maybeConst, !optional}
                 for<'a>                                    TypeTraitBound.ltParameters{dk: "<>"}
                     'a                                     GenericLtParameterDeclaration, LtIdentifier
                         E<R<'a>•=•&'a•i32>                 TypeCall
                          <R<'a>•=•&'a•i32>                 TypeCall.typeArguments{dk: "<>"}
                           R<'a>•=•&'a•i32                  TypeCallNamedArgument
                           R<'a>                            TypeCall
                            <'a>                            TypeCall.typeArguments{dk: "<>"}
                             'a                             LtIdentifier
                                   &'a•i32                  TypeReference{!mut}
                                    'a                      LtIdentifier
                                                      {}    FunctionDeclaration.body{dk: "{}"}                                            */
fn f() where 'static: 'static, dyn 'static +: 'static + Copy,{}                                                                           /*
fn•f()•where•'static:•'static,•dyn•'static•+:•'static•+•Copy,{}    FunctionDeclaration
    ()                                                             FunctionDeclaration.parameters{dk: "()"}
       where•'static:•'static,•dyn•'static•+:•'static•+•Copy,      FunctionDeclaration.whereBounds{dk: "None"}
             'static:•'static                                      WhereLtBoundDeclaration
             'static                                               LtStatic
                      'static                                      LtStatic
                               dyn•'static•+:•'static•+•Copy       WhereTypeBoundDeclaration
                               dyn•'static•+                       TypeDynBounds{dyn}
                                   'static                         LtStatic
                                              'static              LtStatic
                                                        Copy       TypeTraitBound{!maybeConst, !optional}
                                                             {}    FunctionDeclaration.body{dk: "{}"}                                     */
fn f() where 'static: 'static, dyn 'static + ::Foo: 'static + Copy,{}                                                                     /*
fn•f()•where•'static:•'static,•dyn•'static•+•::Foo:•'static•+•Copy,{}    FunctionDeclaration
    ()                                                                   FunctionDeclaration.parameters{dk: "()"}
       where•'static:•'static,•dyn•'static•+•::Foo:•'static•+•Copy,      FunctionDeclaration.whereBounds{dk: "None"}
             'static:•'static                                            WhereLtBoundDeclaration
             'static                                                     LtStatic
                      'static                                            LtStatic
                               dyn•'static•+•::Foo:•'static•+•Copy       WhereTypeBoundDeclaration
                               dyn•'static•+•::Foo                       TypeDynBounds{dyn}
                                   'static                               LtStatic
                                             ::Foo                       TypeTraitBound{!maybeConst, !optional}, TypePath
                                                    'static              LtStatic
                                                              Copy       TypeTraitBound{!maybeConst, !optional}
                                                                   {}    FunctionDeclaration.body{dk: "{}"}                               */
fn f<F: A>() where F::B: Copy {}                                                                                                          /*
fn•f<F:•A>()•where•F::B:•Copy•{}    FunctionDeclaration
    <F:•A>                          FunctionDeclaration.generics{dk: "<>"}
     F:•A                           GenericTypeParameterDeclaration
        A                           TypeTraitBound{!maybeConst, !optional}
          ()                        FunctionDeclaration.parameters{dk: "()"}
             where•F::B:•Copy       FunctionDeclaration.whereBounds{dk: "None"}
                   F::B:•Copy       WhereTypeBoundDeclaration
                   F::B             TypePath
                         Copy       TypeTraitBound{!maybeConst, !optional}
                              {}    FunctionDeclaration.body{dk: "{}"}                                                                    */
fn f<F: A>() where <F as A>::B: Copy {}                                                                                                   /*
fn•f<F:•A>()•where•<F•as•A>::B:•Copy•{}    FunctionDeclaration
    <F:•A>                                 FunctionDeclaration.generics{dk: "<>"}
     F:•A                                  GenericTypeParameterDeclaration
        A                                  TypeTraitBound{!maybeConst, !optional}
          ()                               FunctionDeclaration.parameters{dk: "()"}
             where•<F•as•A>::B:•Copy       FunctionDeclaration.whereBounds{dk: "None"}
                   <F•as•A>::B:•Copy       WhereTypeBoundDeclaration
                   <F•as•A>::B             TypePath
                   <F•as•A>                ExpressionTypeSelector
                                Copy       TypeTraitBound{!maybeConst, !optional}
                                     {}    FunctionDeclaration.body{dk: "{}"}                                                             */
fn f<F: A<B: A>>() where F::B: Copy {}                                                                                                    /*
fn•f<F:•A<B:•A>>()•where•F::B:•Copy•{}    FunctionDeclaration
    <F:•A<B:•A>>                          FunctionDeclaration.generics{dk: "<>"}
     F:•A<B:•A>                           GenericTypeParameterDeclaration
        A<B:•A>                           TypeTraitBound{!maybeConst, !optional}, TypeCall
         <B:•A>                           TypeCall.typeArguments{dk: "<>"}
          B:•A                            TypeCallNamedBound
             A                            TypeTraitBound{!maybeConst, !optional}
                ()                        FunctionDeclaration.parameters{dk: "()"}
                   where•F::B:•Copy       FunctionDeclaration.whereBounds{dk: "None"}
                         F::B:•Copy       WhereTypeBoundDeclaration
                         F::B             TypePath
                               Copy       TypeTraitBound{!maybeConst, !optional}
                                    {}    FunctionDeclaration.body{dk: "{}"}                                                              */
fn f<T: S<<Self as A>::Q>>(&self, r: &T) -> u64;                                                                                          /*
fn•f<T:•S<<Self•as•A>::Q>>(&self,•r:•&T)•->•u64;    FunctionDeclaration
    <T:•S<<Self•as•A>::Q>>                          FunctionDeclaration.generics{dk: "<>"}
     T:•S<<Self•as•A>::Q>                           GenericTypeParameterDeclaration
        S<<Self•as•A>::Q>                           TypeTraitBound{!maybeConst, !optional}, TypeCall
         <<Self•as•A>::Q>                           TypeCall.typeArguments{dk: "<>"}
          <Self•as•A>::Q                            TypePath
          <Self•as•A>                               ExpressionTypeSelector
                          (&self,•r:•&T)            FunctionDeclaration.parameters{dk: "()"}
                           &self                    FunctionSelfParameterDeclaration{ref, !mut}
                                  r:•&T             FunctionParameterDeclaration
                                     &T             TypeReference{!mut}                                                                   */
fn f() -> impl Default {}                                                                                                                 /*
fn•f()•->•impl•Default•{}    FunctionDeclaration
    ()                       FunctionDeclaration.parameters{dk: "()"}
          impl•Default       TypeImplBounds
               Default       TypeTraitBound{!maybeConst, !optional}
                       {}    FunctionDeclaration.body{dk: "{}"}                                                                           */
fn f(t: Box<dyn for<'a> Get<i32, i32>>) { }                                                                                               /*
fn•f(t:•Box<dyn•for<'a>•Get<i32,•i32>>)•{•}    FunctionDeclaration
    (t:•Box<dyn•for<'a>•Get<i32,•i32>>)        FunctionDeclaration.parameters{dk: "()"}
     t:•Box<dyn•for<'a>•Get<i32,•i32>>         FunctionParameterDeclaration
        Box<dyn•for<'a>•Get<i32,•i32>>         TypeCall
           <dyn•for<'a>•Get<i32,•i32>>         TypeCall.typeArguments{dk: "<>"}
            dyn•for<'a>•Get<i32,•i32>          TypeDynBounds{dyn}
                for<'a>•Get<i32,•i32>          TypeTraitBound{!maybeConst, !optional}
                for<'a>                        TypeTraitBound.ltParameters{dk: "<>"}
                    'a                         GenericLtParameterDeclaration, LtIdentifier
                        Get<i32,•i32>          TypeCall
                           <i32,•i32>          TypeCall.typeArguments{dk: "<>"}
                                        {•}    FunctionDeclaration.body{dk: "{}"}                                                         */
fn f(t: Box<dyn for<'a> Fn(i32) -> i32>) { }                                                                                              /*
fn•f(t:•Box<dyn•for<'a>•Fn(i32)•->•i32>)•{•}    FunctionDeclaration
    (t:•Box<dyn•for<'a>•Fn(i32)•->•i32>)        FunctionDeclaration.parameters{dk: "()"}
     t:•Box<dyn•for<'a>•Fn(i32)•->•i32>         FunctionParameterDeclaration
        Box<dyn•for<'a>•Fn(i32)•->•i32>         TypeCall
           <dyn•for<'a>•Fn(i32)•->•i32>         TypeCall.typeArguments{dk: "<>"}
            dyn•for<'a>•Fn(i32)•->•i32          TypeDynBounds{dyn}
                for<'a>•Fn(i32)•->•i32          TypeTraitBound{!maybeConst, !optional}
                for<'a>                         TypeTraitBound.ltParameters{dk: "<>"}
                    'a                          GenericLtParameterDeclaration, LtIdentifier
                        Fn(i32)•->•i32          TypeFunction
                          (i32)                 TypeFunction.parameters{dk: "()"}
                                         {•}    FunctionDeclaration.body{dk: "{}"}                                                        */
fn f(t: for<'a> fn(i32) -> i32) { }                                                                                                       /*
fn•f(t:•for<'a>•fn(i32)•->•i32)•{•}    FunctionDeclaration
    (t:•for<'a>•fn(i32)•->•i32)        FunctionDeclaration.parameters{dk: "()"}
     t:•for<'a>•fn(i32)•->•i32         FunctionParameterDeclaration
        for<'a>•fn(i32)•->•i32         TypeFnPointer
        for<'a>                        TypeFnPointer.ltParameters{dk: "<>"}
            'a                         GenericLtParameterDeclaration, LtIdentifier
                  (i32)                TypeFnPointer.parameters{dk: "()"}
                   i32                 TypeFnPointerParameter
                                {•}    FunctionDeclaration.body{dk: "{}"}                                                                 */
fn f(t: for<'a> unsafe fn(i32) -> i32) { }                                                                                                /*
fn•f(t:•for<'a>•unsafe•fn(i32)•->•i32)•{•}    FunctionDeclaration
    (t:•for<'a>•unsafe•fn(i32)•->•i32)        FunctionDeclaration.parameters{dk: "()"}
     t:•for<'a>•unsafe•fn(i32)•->•i32         FunctionParameterDeclaration
        for<'a>•unsafe•fn(i32)•->•i32         TypeFnPointer{unsafe}
        for<'a>                               TypeFnPointer.ltParameters{dk: "<>"}
            'a                                GenericLtParameterDeclaration, LtIdentifier
                         (i32)                TypeFnPointer.parameters{dk: "()"}
                          i32                 TypeFnPointerParameter
                                       {•}    FunctionDeclaration.body{dk: "{}"}                                                          */
fn f(t: for<'a> extern "C" fn(i32) -> i32) { }                                                                                            /*
fn•f(t:•for<'a>•extern•"C"•fn(i32)•->•i32)•{•}    FunctionDeclaration
    (t:•for<'a>•extern•"C"•fn(i32)•->•i32)        FunctionDeclaration.parameters{dk: "()"}
     t:•for<'a>•extern•"C"•fn(i32)•->•i32         FunctionParameterDeclaration
        for<'a>•extern•"C"•fn(i32)•->•i32         TypeFnPointer
        for<'a>                                   TypeFnPointer.ltParameters{dk: "<>"}
            'a                                    GenericLtParameterDeclaration, LtIdentifier
                extern•"C"                        ExternSpecifier
                       "C"                        Literal{kind: String}
                             (i32)                TypeFnPointer.parameters{dk: "()"}
                              i32                 TypeFnPointerParameter
                                           {•}    FunctionDeclaration.body{dk: "{}"}                                                      */
fn f(t: for<'a> unsafe extern "C" fn(i32) -> i32) { }                                                                                     /*
fn•f(t:•for<'a>•unsafe•extern•"C"•fn(i32)•->•i32)•{•}    FunctionDeclaration
    (t:•for<'a>•unsafe•extern•"C"•fn(i32)•->•i32)        FunctionDeclaration.parameters{dk: "()"}
     t:•for<'a>•unsafe•extern•"C"•fn(i32)•->•i32         FunctionParameterDeclaration
        for<'a>•unsafe•extern•"C"•fn(i32)•->•i32         TypeFnPointer{unsafe}
        for<'a>                                          TypeFnPointer.ltParameters{dk: "<>"}
            'a                                           GenericLtParameterDeclaration, LtIdentifier
                       extern•"C"                        ExternSpecifier
                              "C"                        Literal{kind: String}
                                    (i32)                TypeFnPointer.parameters{dk: "()"}
                                     i32                 TypeFnPointerParameter
                                                  {•}    FunctionDeclaration.body{dk: "{}"}                                               */
impl<T: Trait1, F: FnMut(<T as Trait1>::C)> Callback<T> for F {}                                                                          /*
impl<T:•Trait1,•F:•FnMut(<T•as•Trait1>::C)>•Callback<T>•for•F•{}    ImplDeclaration{!const}
    <T:•Trait1,•F:•FnMut(<T•as•Trait1>::C)>                         ImplDeclaration.generics{dk: "<>"}
     T:•Trait1                                                      GenericTypeParameterDeclaration
        Trait1                                                      TypeTraitBound{!maybeConst, !optional}
                F:•FnMut(<T•as•Trait1>::C)                          GenericTypeParameterDeclaration
                   FnMut(<T•as•Trait1>::C)                          TypeTraitBound{!maybeConst, !optional}, TypeFunction
                        (<T•as•Trait1>::C)                          TypeFunction.parameters{dk: "()"}
                         <T•as•Trait1>::C                           TypePath
                         <T•as•Trait1>                              ExpressionTypeSelector
                                            Callback<T>             TypeCall
                                                    <T>             TypeCall.typeArguments{dk: "<>"}
                                                              {}    ImplDeclaration.body{dk: "{}"}                                        */
impl Bar<N, M> for Foo<N, M> where A<{ N > 1 }>: B, A<{ M > 1 }>: B, {}                                                                   /*
impl•Bar<N,•M>•for•Foo<N,•M>•where•A<{•N•>•1•}>:•B,•A<{•M•>•1•}>:•B,•{}    ImplDeclaration{!const}
     Bar<N,•M>                                                             TypeCall
        <N,•M>                                                             TypeCall.typeArguments{dk: "<>"}
                   Foo<N,•M>                                               TypeCall
                      <N,•M>                                               TypeCall.typeArguments{dk: "<>"}
                             where•A<{•N•>•1•}>:•B,•A<{•M•>•1•}>:•B,       ImplDeclaration.whereBounds{dk: "None"}
                                   A<{•N•>•1•}>:•B                         WhereTypeBoundDeclaration
                                   A<{•N•>•1•}>                            TypeCall
                                    <{•N•>•1•}>                            TypeCall.typeArguments{dk: "<>"}
                                     {•N•>•1•}                             BlockExpression
                                       N•>•1                               ExpressionStatement{!semi}, ComparisonExpression{tk: ">"}
                                           1                               Literal{kind: Integer}
                                                 B                         TypeTraitBound{!maybeConst, !optional}
                                                    A<{•M•>•1•}>:•B        WhereTypeBoundDeclaration
                                                    A<{•M•>•1•}>           TypeCall
                                                     <{•M•>•1•}>           TypeCall.typeArguments{dk: "<>"}
                                                      {•M•>•1•}            BlockExpression
                                                        M•>•1              ExpressionStatement{!semi}, ComparisonExpression{tk: ">"}
                                                            1              Literal{kind: Integer}
                                                                  B        TypeTraitBound{!maybeConst, !optional}
                                                                     {}    ImplDeclaration.body{dk: "{}"}                                 */
async fn f( _: impl for<'a> Add<&'a u8>, _: impl for<'b> Add<&'b u8>, ) {}                                                                /*
async•fn•f(•_:•impl•for<'a>•Add<&'a•u8>,•_:•impl•for<'b>•Add<&'b•u8>,•)•{}    FunctionDeclaration{async}
          (•_:•impl•for<'a>•Add<&'a•u8>,•_:•impl•for<'b>•Add<&'b•u8>,•)       FunctionDeclaration.parameters{dk: "()"}
            _:•impl•for<'a>•Add<&'a•u8>                                       FunctionParameterDeclaration
            _                                                                 WildcardPattern
               impl•for<'a>•Add<&'a•u8>                                       TypeImplBounds
                    for<'a>•Add<&'a•u8>                                       TypeTraitBound{!maybeConst, !optional}
                    for<'a>                                                   TypeTraitBound.ltParameters{dk: "<>"}
                        'a                                                    GenericLtParameterDeclaration, LtIdentifier
                            Add<&'a•u8>                                       TypeCall
                               <&'a•u8>                                       TypeCall.typeArguments{dk: "<>"}
                                &'a•u8                                        TypeReference{!mut}
                                 'a                                           LtIdentifier
                                         _:•impl•for<'b>•Add<&'b•u8>          FunctionParameterDeclaration
                                         _                                    WildcardPattern
                                            impl•for<'b>•Add<&'b•u8>          TypeImplBounds
                                                 for<'b>•Add<&'b•u8>          TypeTraitBound{!maybeConst, !optional}
                                                 for<'b>                      TypeTraitBound.ltParameters{dk: "<>"}
                                                     'b                       GenericLtParameterDeclaration, LtIdentifier
                                                         Add<&'b•u8>          TypeCall
                                                            <&'b•u8>          TypeCall.typeArguments{dk: "<>"}
                                                             &'b•u8           TypeReference{!mut}
                                                              'b              LtIdentifier
                                                                        {}    FunctionDeclaration.body{dk: "{}"}                          */
async fn f<'a>(_: &'a ()) -> impl A<dyn B> {}                                                                                             /*
async•fn•f<'a>(_:•&'a•())•->•impl•A<dyn•B>•{}    FunctionDeclaration{async}
          <'a>                                   FunctionDeclaration.generics{dk: "<>"}
           'a                                    GenericLtParameterDeclaration, LtIdentifier
              (_:•&'a•())                        FunctionDeclaration.parameters{dk: "()"}
               _:•&'a•()                         FunctionParameterDeclaration
               _                                 WildcardPattern
                  &'a•()                         TypeReference{!mut}
                   'a                            LtIdentifier
                      ()                         TypeTuple
                             impl•A<dyn•B>       TypeImplBounds
                                  A<dyn•B>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                                   <dyn•B>       TypeCall.typeArguments{dk: "<>"}
                                    dyn•B        TypeDynBounds{dyn}
                                        B        TypeTraitBound{!maybeConst, !optional}
                                           {}    FunctionDeclaration.body{dk: "{}"}                                                       */
fn f<D: A>() where D::S: {}                                                                                                               /*
fn•f<D:•A>()•where•D::S:•{}    FunctionDeclaration
    <D:•A>                     FunctionDeclaration.generics{dk: "<>"}
     D:•A                      GenericTypeParameterDeclaration
        A                      TypeTraitBound{!maybeConst, !optional}
          ()                   FunctionDeclaration.parameters{dk: "()"}
             where•D::S:       FunctionDeclaration.whereBounds{dk: "None"}
                   D::S:       WhereTypeBoundDeclaration
                   D::S        TypePath
                         {}    FunctionDeclaration.body{dk: "{}"}                                                                         */
type T: Iterator<Item=<S as T>::T>;                                                                                                       /*
type•T:•Iterator<Item=<S•as•T>::T>;    TypeAliasDeclaration
        Iterator<Item=<S•as•T>::T>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                <Item=<S•as•T>::T>     TypeCall.typeArguments{dk: "<>"}
                 Item=<S•as•T>::T      TypeCallNamedArgument
                      <S•as•T>::T      TypePath
                      <S•as•T>         ExpressionTypeSelector                                                                             */
struct R<'a> { s: dyn for<'b> E<D<&'b ()>> + 'a, }                                                                                        /*
struct•R<'a>•{•s:•dyn•for<'b>•E<D<&'b•()>>•+•'a,•}    StructDeclaration
        <'a>                                          StructDeclaration.generics{dk: "<>"}
         'a                                           GenericLtParameterDeclaration, LtIdentifier
             {•s:•dyn•for<'b>•E<D<&'b•()>>•+•'a,•}    StructDeclaration.properties{dk: "{}"}
               s:•dyn•for<'b>•E<D<&'b•()>>•+•'a       StructPropertyDeclaration
                  dyn•for<'b>•E<D<&'b•()>>•+•'a       TypeDynBounds{dyn}
                      for<'b>•E<D<&'b•()>>            TypeTraitBound{!maybeConst, !optional}
                      for<'b>                         TypeTraitBound.ltParameters{dk: "<>"}
                          'b                          GenericLtParameterDeclaration, LtIdentifier
                              E<D<&'b•()>>            TypeCall
                               <D<&'b•()>>            TypeCall.typeArguments{dk: "<>"}
                                D<&'b•()>             TypeCall
                                 <&'b•()>             TypeCall.typeArguments{dk: "<>"}
                                  &'b•()              TypeReference{!mut}
                                   'b                 LtIdentifier
                                      ()              TypeTuple
                                             'a       LtIdentifier                                                                        */
fn f() -> [u8; 4 * 1024 * 1024 * 1024 * 1024] {}                                                                                          /*
fn•f()•->•[u8;•4•*•1024•*•1024•*•1024•*•1024]•{}    FunctionDeclaration
    ()                                              FunctionDeclaration.parameters{dk: "()"}
          [u8;•4•*•1024•*•1024•*•1024•*•1024]       TypeSizedArray
               4•*•1024•*•1024•*•1024•*•1024        OperationExpression{tk: "*"}
               4•*•1024•*•1024•*•1024               OperationExpression{tk: "*"}
               4•*•1024•*•1024                      OperationExpression{tk: "*"}
               4•*•1024                             OperationExpression{tk: "*"}
               4                                    Literal{kind: Integer}
                   1024                             Literal{kind: Integer}
                          1024                      Literal{kind: Integer}
                                 1024               Literal{kind: Integer}
                                        1024        Literal{kind: Integer}
                                              {}    FunctionDeclaration.body{dk: "{}"}                                                    */
trait Foo where T: Borrow<U> + ?Sized, U: ?Sized + 'b, 'a: 'b, Box<T>:, { }                                                               /*
trait•Foo•where•T:•Borrow<U>•+•?Sized,•U:•?Sized•+•'b,•'a:•'b,•Box<T>:,•{•}    TraitDeclaration
          where•T:•Borrow<U>•+•?Sized,•U:•?Sized•+•'b,•'a:•'b,•Box<T>:,        TraitDeclaration.whereBounds{dk: "None"}
                T:•Borrow<U>•+•?Sized                                          WhereTypeBoundDeclaration
                   Borrow<U>                                                   TypeTraitBound{!maybeConst, !optional}, TypeCall
                         <U>                                                   TypeCall.typeArguments{dk: "<>"}
                               ?Sized                                          TypeTraitBound{!maybeConst, optional}
                                       U:•?Sized•+•'b                          WhereTypeBoundDeclaration
                                          ?Sized                               TypeTraitBound{!maybeConst, optional}
                                                   'b                          LtIdentifier
                                                       'a:•'b                  WhereLtBoundDeclaration
                                                       'a                      LtIdentifier
                                                           'b                  LtIdentifier
                                                               Box<T>:         WhereTypeBoundDeclaration
                                                               Box<T>          TypeCall
                                                                  <T>          TypeCall.typeArguments{dk: "<>"}
                                                                        {•}    TraitDeclaration.body{dk: "{}"}                            */
trait Map where for<'a> &'a Self: IntoIterator<Item = (&'a Self::Key, &'a Self::Value)>, {}                                               /*
trait•Map•where•for<'a>•&'a•Self:•IntoIterator<Item•=•(&'a•Self::Key,•&'a•Self::Value)>,•{}    TraitDeclaration
          where•for<'a>•&'a•Self:•IntoIterator<Item•=•(&'a•Self::Key,•&'a•Self::Value)>,       TraitDeclaration.whereBounds{dk: "None"}
                for<'a>•&'a•Self:•IntoIterator<Item•=•(&'a•Self::Key,•&'a•Self::Value)>        WhereTypeBoundDeclaration
                for<'a>                                                                        WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                    'a                                                                         GenericLtParameterDeclaration, LtIdentifier
                        &'a•Self                                                               TypeReference{!mut}
                         'a                                                                    LtIdentifier
                                  IntoIterator<Item•=•(&'a•Self::Key,•&'a•Self::Value)>        TypeTraitBound{!maybeConst, !optional}, TypeCall
                                              <Item•=•(&'a•Self::Key,•&'a•Self::Value)>        TypeCall.typeArguments{dk: "<>"}
                                               Item•=•(&'a•Self::Key,•&'a•Self::Value)         TypeCallNamedArgument
                                                      (&'a•Self::Key,•&'a•Self::Value)         TypeTuple
                                                       &'a•Self::Key                           TypeReference{!mut}
                                                        'a                                     LtIdentifier
                                                           Self::Key                           TypePath
                                                                      &'a•Self::Value          TypeReference{!mut}
                                                                       'a                      LtIdentifier
                                                                          Self::Value          TypePath
                                                                                         {}    TraitDeclaration.body{dk: "{}"}            */
trait S: A + AsRef<Self::B> {}                                                                                                            /*
trait•S:•A•+•AsRef<Self::B>•{}    TraitDeclaration
         A                        TypeTraitBound{!maybeConst, !optional}
             AsRef<Self::B>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                  <Self::B>       TypeCall.typeArguments{dk: "<>"}
                   Self::B        TypePath
                            {}    TraitDeclaration.body{dk: "{}"}                                                                         */
struct Bar<const N: u8>([u8; (N + 2) as usize]) where [(); (N + 2) as usize]:;                                                            /*
struct•Bar<const•N:•u8>([u8;•(N•+•2)•as•usize])•where•[();•(N•+•2)•as•usize]:;    TupleStructDeclaration
          <const•N:•u8>                                                           TupleStructDeclaration.generics{dk: "<>"}
           const•N:•u8                                                            ConstTypeParameterDeclaration
                       ([u8;•(N•+•2)•as•usize])                                   TupleStructDeclaration.items{dk: "()"}
                        [u8;•(N•+•2)•as•usize]                                    TupleStructItemDeclaration, TypeSizedArray
                             (N•+•2)•as•usize                                     ExpressionAsTypeCast
                              N•+•2                                               OperationExpression{tk: "+"}
                                  2                                               Literal{kind: Integer}
                                                where•[();•(N•+•2)•as•usize]:     TupleStructDeclaration.whereBounds{dk: "None"}
                                                      [();•(N•+•2)•as•usize]:     WhereTypeBoundDeclaration
                                                      [();•(N•+•2)•as•usize]      TypeSizedArray
                                                       ()                         TypeTuple
                                                           (N•+•2)•as•usize       ExpressionAsTypeCast
                                                            N•+•2                 OperationExpression{tk: "+"}
                                                                2                 Literal{kind: Integer}                                  */
fn f<const N: u8>() where D<{N as usize as u16 }>:{}                                                                                      /*
fn•f<const•N:•u8>()•where•D<{N•as•usize•as•u16•}>:{}    FunctionDeclaration
    <const•N:•u8>                                       FunctionDeclaration.generics{dk: "<>"}
     const•N:•u8                                        ConstTypeParameterDeclaration
                 ()                                     FunctionDeclaration.parameters{dk: "()"}
                    where•D<{N•as•usize•as•u16•}>:      FunctionDeclaration.whereBounds{dk: "None"}
                          D<{N•as•usize•as•u16•}>:      WhereTypeBoundDeclaration
                          D<{N•as•usize•as•u16•}>       TypeCall
                           <{N•as•usize•as•u16•}>       TypeCall.typeArguments{dk: "<>"}
                            {N•as•usize•as•u16•}        BlockExpression
                             N•as•usize•as•u16          ExpressionStatement{!semi}, ExpressionAsTypeCast
                             N•as•usize                 ExpressionAsTypeCast
                                                  {}    FunctionDeclaration.body{dk: "{}"}                                                */
fn f<T>() where for<'a> T: TraitA<'a, AsA: for<'b> TraitB<'a, 'b, AsB: for<'c> TraitC<'a, 'b, 'c>>>, { }                                  /*
fn•f<T>()•where•for<'a>•T:•TraitA<'a,•AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>>,•{•}    FunctionDeclaration
    <T>                                                                                                     FunctionDeclaration.generics{dk: "<>"}
     T                                                                                                      GenericTypeParameterDeclaration
       ()                                                                                                   FunctionDeclaration.parameters{dk: "()"}
          where•for<'a>•T:•TraitA<'a,•AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>>,        FunctionDeclaration.whereBounds{dk: "None"}
                for<'a>•T:•TraitA<'a,•AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>>         WhereTypeBoundDeclaration
                for<'a>                                                                                     WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                    'a                                                                                      GenericLtParameterDeclaration, LtIdentifier
                           TraitA<'a,•AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>>         TypeTraitBound{!maybeConst, !optional}, TypeCall
                                 <'a,•AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>>         TypeCall.typeArguments{dk: "<>"}
                                  'a                                                                        LtIdentifier
                                      AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>          TypeCallNamedBound
                                           for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>          TypeTraitBound{!maybeConst, !optional}
                                           for<'b>                                                          TypeTraitBound.ltParameters{dk: "<>"}
                                               'b                                                           GenericLtParameterDeclaration, LtIdentifier
                                                   TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>          TypeCall
                                                         <'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>          TypeCall.typeArguments{dk: "<>"}
                                                          'a                                                LtIdentifier
                                                              'b                                            LtIdentifier
                                                                  AsB:•for<'c>•TraitC<'a,•'b,•'c>           TypeCallNamedBound
                                                                       for<'c>•TraitC<'a,•'b,•'c>           TypeTraitBound{!maybeConst, !optional}
                                                                       for<'c>                              TypeTraitBound.ltParameters{dk: "<>"}
                                                                           'c                               GenericLtParameterDeclaration, LtIdentifier
                                                                               TraitC<'a,•'b,•'c>           TypeCall
                                                                                     <'a,•'b,•'c>           TypeCall.typeArguments{dk: "<>"}
                                                                                      'a                    LtIdentifier
                                                                                          'b                LtIdentifier
                                                                                              'c            LtIdentifier
                                                                                                     {•}    FunctionDeclaration.body{dk: "{}"}*/
fn f<'u, 'a, F>() where for<'b> F: Iterator<Item: for<'c> B<'a, 'b, 'c> + for<'c> A<'a, 'c>>, { }                                         /*
fn•f<'u,•'a,•F>()•where•for<'b>•F:•Iterator<Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>>,•{•}    FunctionDeclaration
    <'u,•'a,•F>                                                                                      FunctionDeclaration.generics{dk: "<>"}
     'u                                                                                              GenericLtParameterDeclaration, LtIdentifier
         'a                                                                                          GenericLtParameterDeclaration, LtIdentifier
             F                                                                                       GenericTypeParameterDeclaration
               ()                                                                                    FunctionDeclaration.parameters{dk: "()"}
                  where•for<'b>•F:•Iterator<Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>>,        FunctionDeclaration.whereBounds{dk: "None"}
                        for<'b>•F:•Iterator<Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>>         WhereTypeBoundDeclaration
                        for<'b>                                                                      WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                            'b                                                                       GenericLtParameterDeclaration, LtIdentifier
                                   Iterator<Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>>         TypeTraitBound{!maybeConst, !optional}, TypeCall
                                           <Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>>         TypeCall.typeArguments{dk: "<>"}
                                            Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>          TypeCallNamedBound
                                                  for<'c>•B<'a,•'b,•'c>                              TypeTraitBound{!maybeConst, !optional}
                                                  for<'c>                                            TypeTraitBound.ltParameters{dk: "<>"}
                                                      'c                                             GenericLtParameterDeclaration, LtIdentifier
                                                          B<'a,•'b,•'c>                              TypeCall
                                                           <'a,•'b,•'c>                              TypeCall.typeArguments{dk: "<>"}
                                                            'a                                       LtIdentifier
                                                                'b                                   LtIdentifier
                                                                    'c                               LtIdentifier
                                                                          for<'c>•A<'a,•'c>          TypeTraitBound{!maybeConst, !optional}
                                                                          for<'c>                    TypeTraitBound.ltParameters{dk: "<>"}
                                                                              'c                     GenericLtParameterDeclaration, LtIdentifier
                                                                                  A<'a,•'c>          TypeCall
                                                                                   <'a,•'c>          TypeCall.typeArguments{dk: "<>"}
                                                                                    'a               LtIdentifier
                                                                                        'c           LtIdentifier
                                                                                              {•}    FunctionDeclaration.body{dk: "{}"}   */
fn f(&self, db: &<Q as QueryDb<'_>>::DynDb) {}                                                                                            /*
fn•f(&self,•db:•&<Q•as•QueryDb<'_>>::DynDb)•{}    FunctionDeclaration
    (&self,•db:•&<Q•as•QueryDb<'_>>::DynDb)       FunctionDeclaration.parameters{dk: "()"}
     &self                                        FunctionSelfParameterDeclaration{ref, !mut}
            db:•&<Q•as•QueryDb<'_>>::DynDb        FunctionParameterDeclaration
                &<Q•as•QueryDb<'_>>::DynDb        TypeReference{!mut}
                 <Q•as•QueryDb<'_>>::DynDb        TypePath
                 <Q•as•QueryDb<'_>>               ExpressionTypeSelector
                       QueryDb<'_>                TypeCall
                              <'_>                TypeCall.typeArguments{dk: "<>"}
                               '_                 LtElided
                                            {}    FunctionDeclaration.body{dk: "{}"}                                                      */
pub fn f<'a, I>() -> impl B<I, D = (), C = impl S + 'a> where I: A<E = &'a [()]>, {}                                                      /*
pub•fn•f<'a,•I>()•->•impl•B<I,•D•=•(),•C•=•impl•S•+•'a>•where•I:•A<E•=•&'a•[()]>,•{}    FunctionDeclaration
pub                                                                                     PubSpecifier
        <'a,•I>                                                                         FunctionDeclaration.generics{dk: "<>"}
         'a                                                                             GenericLtParameterDeclaration, LtIdentifier
             I                                                                          GenericTypeParameterDeclaration
               ()                                                                       FunctionDeclaration.parameters{dk: "()"}
                     impl•B<I,•D•=•(),•C•=•impl•S•+•'a>                                 TypeImplBounds
                          B<I,•D•=•(),•C•=•impl•S•+•'a>                                 TypeTraitBound{!maybeConst, !optional}, TypeCall
                           <I,•D•=•(),•C•=•impl•S•+•'a>                                 TypeCall.typeArguments{dk: "<>"}
                               D•=•()                                                   TypeCallNamedArgument
                                   ()                                                   TypeTuple
                                       C•=•impl•S•+•'a                                  TypeCallNamedArgument
                                           impl•S•+•'a                                  TypeImplBounds
                                                S                                       TypeTraitBound{!maybeConst, !optional}
                                                    'a                                  LtIdentifier
                                                        where•I:•A<E•=•&'a•[()]>,       FunctionDeclaration.whereBounds{dk: "None"}
                                                              I:•A<E•=•&'a•[()]>        WhereTypeBoundDeclaration
                                                                 A<E•=•&'a•[()]>        TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                                  <E•=•&'a•[()]>        TypeCall.typeArguments{dk: "<>"}
                                                                   E•=•&'a•[()]         TypeCallNamedArgument
                                                                       &'a•[()]         TypeReference{!mut}
                                                                        'a              LtIdentifier
                                                                           [()]         TypeSlice
                                                                            ()          TypeTuple
                                                                                  {}    FunctionDeclaration.body{dk: "{}"}                */
type S<T: A<B: for<'a> C<&'a u8>>> = D;                                                                                                   /*
type•S<T:•A<B:•for<'a>•C<&'a•u8>>>•=•D;    TypeAliasDeclaration
      <T:•A<B:•for<'a>•C<&'a•u8>>>         TypeAliasDeclaration.generics{dk: "<>"}
       T:•A<B:•for<'a>•C<&'a•u8>>          GenericTypeParameterDeclaration
          A<B:•for<'a>•C<&'a•u8>>          TypeTraitBound{!maybeConst, !optional}, TypeCall
           <B:•for<'a>•C<&'a•u8>>          TypeCall.typeArguments{dk: "<>"}
            B:•for<'a>•C<&'a•u8>           TypeCallNamedBound
               for<'a>•C<&'a•u8>           TypeTraitBound{!maybeConst, !optional}
               for<'a>                     TypeTraitBound.ltParameters{dk: "<>"}
                   'a                      GenericLtParameterDeclaration, LtIdentifier
                       C<&'a•u8>           TypeCall
                        <&'a•u8>           TypeCall.typeArguments{dk: "<>"}
                         &'a•u8            TypeReference{!mut}
                          'a               LtIdentifier                                                                                   */
type S<T>;                                                                                                                                /*
type•S<T>;    TypeAliasDeclaration
      <T>     TypeAliasDeclaration.generics{dk: "<>"}
       T      GenericTypeParameterDeclaration                                                                                             */
type A = a::b!();                                                                                                                         /*
type•A•=•a::b!();    TypeAliasDeclaration
         a::b!()     MacroInvocation
         a::b        TypePath
              ()     MacroInvocation.segments{dk: "()"}                                                                                   */
type S where Self: Sized;                                                                                                                 /*
type•S•where•Self:•Sized;    TypeAliasDeclaration
       where•Self:•Sized     TypeAliasDeclaration.whereBounds{dk: "None"}
             Self:•Sized     WhereTypeBoundDeclaration
                   Sized     TypeTraitBound{!maybeConst, !optional}                                                                       */
fn f(&self, a: &!){}                                                                                                                      /*
fn•f(&self,•a:•&!){}    FunctionDeclaration
    (&self,•a:•&!)      FunctionDeclaration.parameters{dk: "()"}
     &self              FunctionSelfParameterDeclaration{ref, !mut}
            a:•&!       FunctionParameterDeclaration
               &!       TypeReference{!mut}
                !       TypeNever
                  {}    FunctionDeclaration.body{dk: "{}"}                                                                                */
type S<T> where T: Display;                                                                                                               /*
type•S<T>•where•T:•Display;    TypeAliasDeclaration
      <T>                      TypeAliasDeclaration.generics{dk: "<>"}
       T                       GenericTypeParameterDeclaration
          where•T:•Display     TypeAliasDeclaration.whereBounds{dk: "None"}
                T:•Display     WhereTypeBoundDeclaration
                   Display     TypeTraitBound{!maybeConst, !optional}                                                                     */
type S<'a, T: Debug + 'a>: ?Sized = dyn Iterator<Item=T>;                                                                                 /*
type•S<'a,•T:•Debug•+•'a>:•?Sized•=•dyn•Iterator<Item=T>;    TypeAliasDeclaration
      <'a,•T:•Debug•+•'a>                                    TypeAliasDeclaration.generics{dk: "<>"}
       'a                                                    GenericLtParameterDeclaration, LtIdentifier
           T:•Debug•+•'a                                     GenericTypeParameterDeclaration
              Debug                                          TypeTraitBound{!maybeConst, !optional}
                      'a                                     LtIdentifier
                           ?Sized                            TypeTraitBound{!maybeConst, optional}
                                    dyn•Iterator<Item=T>     TypeDynBounds{dyn}
                                        Iterator<Item=T>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                <Item=T>     TypeCall.typeArguments{dk: "<>"}
                                                 Item=T      TypeCallNamedArgument                                                        */
type S<'x> where T: 'x = (&'x ());                                                                                                        /*
type•S<'x>•where•T:•'x•=•(&'x•());    TypeAliasDeclaration
      <'x>                            TypeAliasDeclaration.generics{dk: "<>"}
       'x                             GenericLtParameterDeclaration, LtIdentifier
           where•T:•'x                TypeAliasDeclaration.whereBounds{dk: "None"}
                 T:•'x                WhereTypeBoundDeclaration
                    'x                LtIdentifier
                          &'x•()      TypeReference{!mut}
                           'x         LtIdentifier
                              ()      TypeTuple                                                                                           */
type S<'u, 'v> where 'u: 'v = (&'v &'u ());                                                                                               /*
type•S<'u,•'v>•where•'u:•'v•=•(&'v•&'u•());    TypeAliasDeclaration
      <'u,•'v>                                 TypeAliasDeclaration.generics{dk: "<>"}
       'u                                      GenericLtParameterDeclaration, LtIdentifier
           'v                                  GenericLtParameterDeclaration, LtIdentifier
               where•'u:•'v                    TypeAliasDeclaration.whereBounds{dk: "None"}
                     'u:•'v                    WhereLtBoundDeclaration
                     'u                        LtIdentifier
                         'v                    LtIdentifier
                               &'v•&'u•()      TypeReference{!mut}
                                'v             LtIdentifier
                                   &'u•()      TypeReference{!mut}
                                    'u         LtIdentifier
                                       ()      TypeTuple                                                                                  */
type S where Self: Q + S = E;                                                                                                             /*
type•S•where•Self:•Q•+•S•=•E;    TypeAliasDeclaration
       where•Self:•Q•+•S         TypeAliasDeclaration.whereBounds{dk: "None"}
             Self:•Q•+•S         WhereTypeBoundDeclaration
                   Q             TypeTraitBound{!maybeConst, !optional}
                       S         TypeTraitBound{!maybeConst, !optional}                                                                   */
type S<'a: 'b, 'b> = (&'a(), &'b ());                                                                                                     /*
type•S<'a:•'b,•'b>•=•(&'a(),•&'b•());    TypeAliasDeclaration
      <'a:•'b,•'b>                       TypeAliasDeclaration.generics{dk: "<>"}
       'a:•'b                            GenericLtParameterDeclaration
       'a                                LtIdentifier
           'b                            LtIdentifier
               'b                        GenericLtParameterDeclaration, LtIdentifier
                     (&'a(),•&'b•())     TypeTuple
                      &'a()              TypeReference{!mut}
                       'a                LtIdentifier
                         ()              TypeTuple
                             &'b•()      TypeReference{!mut}
                              'b         LtIdentifier
                                 ()      TypeTuple                                                                                        */
type S<'a> where Self: 'static = (&'a ());                                                                                                /*
type•S<'a>•where•Self:•'static•=•(&'a•());    TypeAliasDeclaration
      <'a>                                    TypeAliasDeclaration.generics{dk: "<>"}
       'a                                     GenericLtParameterDeclaration, LtIdentifier
           where•Self:•'static                TypeAliasDeclaration.whereBounds{dk: "None"}
                 Self:•'static                WhereTypeBoundDeclaration
                       'static                LtStatic
                                  &'a•()      TypeReference{!mut}
                                   'a         LtIdentifier
                                      ()      TypeTuple                                                                                   */
type S<'a, 'b> where 'b: 'a = (&'a(), &'b ());                                                                                            /*
type•S<'a,•'b>•where•'b:•'a•=•(&'a(),•&'b•());    TypeAliasDeclaration
      <'a,•'b>                                    TypeAliasDeclaration.generics{dk: "<>"}
       'a                                         GenericLtParameterDeclaration, LtIdentifier
           'b                                     GenericLtParameterDeclaration, LtIdentifier
               where•'b:•'a                       TypeAliasDeclaration.whereBounds{dk: "None"}
                     'b:•'a                       WhereLtBoundDeclaration
                     'b                           LtIdentifier
                         'a                       LtIdentifier
                              (&'a(),•&'b•())     TypeTuple
                               &'a()              TypeReference{!mut}
                                'a                LtIdentifier
                                  ()              TypeTuple
                                      &'b•()      TypeReference{!mut}
                                       'b         LtIdentifier
                                          ()      TypeTuple                                                                               */
type S<'a>: B<&'a [u8]>;                                                                                                                  /*
type•S<'a>:•B<&'a•[u8]>;    TypeAliasDeclaration
      <'a>                  TypeAliasDeclaration.generics{dk: "<>"}
       'a                   GenericLtParameterDeclaration, LtIdentifier
            B<&'a•[u8]>     TypeTraitBound{!maybeConst, !optional}, TypeCall
             <&'a•[u8]>     TypeCall.typeArguments{dk: "<>"}
              &'a•[u8]      TypeReference{!mut}
               'a           LtIdentifier
                  [u8]      TypeSlice                                                                                                     */
type S<'a>: 'a;                                                                                                                           /*
type•S<'a>:•'a;    TypeAliasDeclaration
      <'a>         TypeAliasDeclaration.generics{dk: "<>"}
       'a          GenericLtParameterDeclaration, LtIdentifier
            'a     LtIdentifier                                                                                                           */
type S<'a: 'a>;                                                                                                                           /*
type•S<'a:•'a>;    TypeAliasDeclaration
      <'a:•'a>     TypeAliasDeclaration.generics{dk: "<>"}
       'a:•'a      GenericLtParameterDeclaration
       'a          LtIdentifier
           'a      LtIdentifier                                                                                                           */
type S<'a> = &'a ();                                                                                                                      /*
type•S<'a>•=•&'a•();    TypeAliasDeclaration
      <'a>              TypeAliasDeclaration.generics{dk: "<>"}
       'a               GenericLtParameterDeclaration, LtIdentifier
             &'a•()     TypeReference{!mut}
              'a        LtIdentifier
                 ()     TypeTuple                                                                                                         */
type S<B>: S<A=B>;                                                                                                                        /*
type•S<B>:•S<A=B>;    TypeAliasDeclaration
      <B>             TypeAliasDeclaration.generics{dk: "<>"}
       B              GenericTypeParameterDeclaration
           S<A=B>     TypeTraitBound{!maybeConst, !optional}, TypeCall
            <A=B>     TypeCall.typeArguments{dk: "<>"}
             A=B      TypeCallNamedArgument                                                                                               */
type S<'a, const N: usize>;                                                                                                               /*
type•S<'a,•const•N:•usize>;    TypeAliasDeclaration
      <'a,•const•N:•usize>     TypeAliasDeclaration.generics{dk: "<>"}
       'a                      GenericLtParameterDeclaration, LtIdentifier
           const•N:•usize      ConstTypeParameterDeclaration                                                                              */
type S<'a> where <A as B>::T: 'a, <A as B>::T: 'a = R<&'a S::T, &'a E::T>;                                                                /*
type•S<'a>•where•<A•as•B>::T:•'a,•<A•as•B>::T:•'a•=•R<&'a•S::T,•&'a•E::T>;    TypeAliasDeclaration
      <'a>                                                                    TypeAliasDeclaration.generics{dk: "<>"}
       'a                                                                     GenericLtParameterDeclaration, LtIdentifier
           where•<A•as•B>::T:•'a,•<A•as•B>::T:•'a                             TypeAliasDeclaration.whereBounds{dk: "None"}
                 <A•as•B>::T:•'a                                              WhereTypeBoundDeclaration
                 <A•as•B>::T                                                  TypePath
                 <A•as•B>                                                     ExpressionTypeSelector
                              'a                                              LtIdentifier
                                  <A•as•B>::T:•'a                             WhereTypeBoundDeclaration
                                  <A•as•B>::T                                 TypePath
                                  <A•as•B>                                    ExpressionTypeSelector
                                               'a                             LtIdentifier
                                                    R<&'a•S::T,•&'a•E::T>     TypeCall
                                                     <&'a•S::T,•&'a•E::T>     TypeCall.typeArguments{dk: "<>"}
                                                      &'a•S::T                TypeReference{!mut}
                                                       'a                     LtIdentifier
                                                          S::T                TypePath
                                                                &'a•E::T      TypeReference{!mut}
                                                                 'a           LtIdentifier
                                                                    E::T      TypePath                                                    */
type S<T> = Self::E<'static, T>;                                                                                                          /*
type•S<T>•=•Self::E<'static,•T>;    TypeAliasDeclaration
      <T>                           TypeAliasDeclaration.generics{dk: "<>"}
       T                            GenericTypeParameterDeclaration
            Self::E<'static,•T>     TypeCall
            Self::E                 TypePath
                   <'static,•T>     TypeCall.typeArguments{dk: "<>"}
                    'static         LtStatic                                                                                              */
type S = Self::E<'static, 'static>;                                                                                                       /*
type•S•=•Self::E<'static,•'static>;    TypeAliasDeclaration
         Self::E<'static,•'static>     TypeCall
         Self::E                       TypePath
                <'static,•'static>     TypeCall.typeArguments{dk: "<>"}
                 'static               LtStatic
                          'static      LtStatic                                                                                           */
impl<'b> ATy for &'b () {}                                                                                                                /*
impl<'b>•ATy•for•&'b•()•{}    ImplDeclaration{!const}
    <'b>                      ImplDeclaration.generics{dk: "<>"}
     'b                       GenericLtParameterDeclaration, LtIdentifier
                 &'b•()       TypeReference{!mut}
                  'b          LtIdentifier
                     ()       TypeTuple
                        {}    ImplDeclaration.body{dk: "{}"}                                                                              */
impl<T: Copy + std::ops::Deref> UnsafeCopy<T> for T {}                                                                                    /*
impl<T:•Copy•+•std::ops::Deref>•UnsafeCopy<T>•for•T•{}    ImplDeclaration{!const}
    <T:•Copy•+•std::ops::Deref>                           ImplDeclaration.generics{dk: "<>"}
     T:•Copy•+•std::ops::Deref                            GenericTypeParameterDeclaration
        Copy                                              TypeTraitBound{!maybeConst, !optional}
               std::ops::Deref                            TypeTraitBound{!maybeConst, !optional}, TypePath
               std::ops                                   TypePath
                                UnsafeCopy<T>             TypeCall
                                          <T>             TypeCall.typeArguments{dk: "<>"}
                                                    {}    ImplDeclaration.body{dk: "{}"}                                                  */
impl<T: X<Y<i32> = i32>> M for T {}                                                                                                       /*
impl<T:•X<Y<i32>•=•i32>>•M•for•T•{}    ImplDeclaration{!const}
    <T:•X<Y<i32>•=•i32>>               ImplDeclaration.generics{dk: "<>"}
     T:•X<Y<i32>•=•i32>                GenericTypeParameterDeclaration
        X<Y<i32>•=•i32>                TypeTraitBound{!maybeConst, !optional}, TypeCall
         <Y<i32>•=•i32>                TypeCall.typeArguments{dk: "<>"}
          Y<i32>•=•i32                 TypeCallNamedArgument
          Y<i32>                       TypeCall
           <i32>                       TypeCall.typeArguments{dk: "<>"}
                                 {}    ImplDeclaration.body{dk: "{}"}                                                                     */
type S: Sized where <Self as B>::C: Sized;                                                                                                /*
type•S:•Sized•where•<Self•as•B>::C:•Sized;    TypeAliasDeclaration
        Sized                                 TypeTraitBound{!maybeConst, !optional}
              where•<Self•as•B>::C:•Sized     TypeAliasDeclaration.whereBounds{dk: "None"}
                    <Self•as•B>::C:•Sized     WhereTypeBoundDeclaration
                    <Self•as•B>::C            TypePath
                    <Self•as•B>               ExpressionTypeSelector
                                    Sized     TypeTraitBound{!maybeConst, !optional}                                                      */
type S = Q<<T as R>::E>;                                                                                                                  /*
type•S•=•Q<<T•as•R>::E>;    TypeAliasDeclaration
         Q<<T•as•R>::E>     TypeCall
          <<T•as•R>::E>     TypeCall.typeArguments{dk: "<>"}
           <T•as•R>::E      TypePath
           <T•as•R>         ExpressionTypeSelector                                                                                        */
struct B<'a, T: for<'r> X<Y<'r> = &'r ()>> {f: <T as X>::Y<'a>}                                                                           /*
struct•B<'a,•T:•for<'r>•X<Y<'r>•=•&'r•()>>•{f:•<T•as•X>::Y<'a>}    StructDeclaration
        <'a,•T:•for<'r>•X<Y<'r>•=•&'r•()>>                         StructDeclaration.generics{dk: "<>"}
         'a                                                        GenericLtParameterDeclaration, LtIdentifier
             T:•for<'r>•X<Y<'r>•=•&'r•()>                          GenericTypeParameterDeclaration
                for<'r>•X<Y<'r>•=•&'r•()>                          TypeTraitBound{!maybeConst, !optional}
                for<'r>                                            TypeTraitBound.ltParameters{dk: "<>"}
                    'r                                             GenericLtParameterDeclaration, LtIdentifier
                        X<Y<'r>•=•&'r•()>                          TypeCall
                         <Y<'r>•=•&'r•()>                          TypeCall.typeArguments{dk: "<>"}
                          Y<'r>•=•&'r•()                           TypeCallNamedArgument
                          Y<'r>                                    TypeCall
                           <'r>                                    TypeCall.typeArguments{dk: "<>"}
                            'r                                     LtIdentifier
                                  &'r•()                           TypeReference{!mut}
                                   'r                              LtIdentifier
                                      ()                           TypeTuple
                                           {f:•<T•as•X>::Y<'a>}    StructDeclaration.properties{dk: "{}"}
                                            f:•<T•as•X>::Y<'a>     StructPropertyDeclaration
                                               <T•as•X>::Y<'a>     TypeCall
                                               <T•as•X>::Y         TypePath
                                               <T•as•X>            ExpressionTypeSelector
                                                          <'a>     TypeCall.typeArguments{dk: "<>"}
                                                           'a      LtIdentifier                                                           */
enum E<'a> { S(<S as A>::B<'a>) }                                                                                                         /*
enum•E<'a>•{•S(<S•as•A>::B<'a>)•}    EnumDeclaration
      <'a>                           EnumDeclaration.generics{dk: "<>"}
       'a                            GenericLtParameterDeclaration, LtIdentifier
           {•S(<S•as•A>::B<'a>)•}    EnumDeclaration.members{dk: "{}"}
             S(<S•as•A>::B<'a>)      EnumMemberTupleDeclaration
              (<S•as•A>::B<'a>)      EnumMemberTupleDeclaration.items{dk: "()"}
               <S•as•A>::B<'a>       TupleStructItemDeclaration, TypeCall
               <S•as•A>::B           TypePath
               <S•as•A>              ExpressionTypeSelector
                          <'a>       TypeCall.typeArguments{dk: "<>"}
                           'a        LtIdentifier                                                                                         */
pub type T<P: Send + Send + Send> = P;                                                                                                    /*
pub•type•T<P:•Send•+•Send•+•Send>•=•P;    TypeAliasDeclaration
pub                                       PubSpecifier
          <P:•Send•+•Send•+•Send>         TypeAliasDeclaration.generics{dk: "<>"}
           P:•Send•+•Send•+•Send          GenericTypeParameterDeclaration
              Send                        TypeTraitBound{!maybeConst, !optional}
                     Send                 TypeTraitBound{!maybeConst, !optional}
                            Send          TypeTraitBound{!maybeConst, !optional}                                                          */
type S<'b, 'a: 'b + 'b> = (&'b u32, Vec<&'a i32>);                                                                                        /*
type•S<'b,•'a:•'b•+•'b>•=•(&'b•u32,•Vec<&'a•i32>);    TypeAliasDeclaration
      <'b,•'a:•'b•+•'b>                               TypeAliasDeclaration.generics{dk: "<>"}
       'b                                             GenericLtParameterDeclaration, LtIdentifier
           'a:•'b•+•'b                                GenericLtParameterDeclaration
           'a                                         LtIdentifier
               'b                                     LtIdentifier
                    'b                                LtIdentifier
                          (&'b•u32,•Vec<&'a•i32>)     TypeTuple
                           &'b•u32                    TypeReference{!mut}
                            'b                        LtIdentifier
                                    Vec<&'a•i32>      TypeCall
                                       <&'a•i32>      TypeCall.typeArguments{dk: "<>"}
                                        &'a•i32       TypeReference{!mut}
                                         'a           LtIdentifier                                                                        */
type S<'b, T: 'b + 'b> = (&'b u32, Vec<T>);                                                                                               /*
type•S<'b,•T:•'b•+•'b>•=•(&'b•u32,•Vec<T>);    TypeAliasDeclaration
      <'b,•T:•'b•+•'b>                         TypeAliasDeclaration.generics{dk: "<>"}
       'b                                      GenericLtParameterDeclaration, LtIdentifier
           T:•'b•+•'b                          GenericTypeParameterDeclaration
              'b                               LtIdentifier
                   'b                          LtIdentifier
                         (&'b•u32,•Vec<T>)     TypeTuple
                          &'b•u32              TypeReference{!mut}
                           'b                  LtIdentifier
                                   Vec<T>      TypeCall
                                      <T>      TypeCall.typeArguments{dk: "<>"}                                                           */
type S<'b, T> where T: 'b, T: 'b = (&'b u32, Vec<T>);                                                                                     /*
type•S<'b,•T>•where•T:•'b,•T:•'b•=•(&'b•u32,•Vec<T>);    TypeAliasDeclaration
      <'b,•T>                                            TypeAliasDeclaration.generics{dk: "<>"}
       'b                                                GenericLtParameterDeclaration, LtIdentifier
           T                                             GenericTypeParameterDeclaration
              where•T:•'b,•T:•'b                         TypeAliasDeclaration.whereBounds{dk: "None"}
                    T:•'b                                WhereTypeBoundDeclaration
                       'b                                LtIdentifier
                           T:•'b                         WhereTypeBoundDeclaration
                              'b                         LtIdentifier
                                   (&'b•u32,•Vec<T>)     TypeTuple
                                    &'b•u32              TypeReference{!mut}
                                     'b                  LtIdentifier
                                             Vec<T>      TypeCall
                                                <T>      TypeCall.typeArguments{dk: "<>"}                                                 */
type A = dyn S + ?Sized + ?Sized;                                                                                                         /*
type•A•=•dyn•S•+•?Sized•+•?Sized;    TypeAliasDeclaration
         dyn•S•+•?Sized•+•?Sized     TypeDynBounds{dyn}
             S                       TypeTraitBound{!maybeConst, !optional}
                 ?Sized              TypeTraitBound{!maybeConst, optional}
                          ?Sized     TypeTraitBound{!maybeConst, optional}                                                                */
type R = dyn ?Sized + A;                                                                                                                  /*
type•R•=•dyn•?Sized•+•A;    TypeAliasDeclaration
         dyn•?Sized•+•A     TypeDynBounds{dyn}
             ?Sized         TypeTraitBound{!maybeConst, optional}
                      A     TypeTraitBound{!maybeConst, !optional}                                                                        */
type Q = dyn for<'a> E<'a> + for<'b> R<'b>;                                                                                               /*
type•Q•=•dyn•for<'a>•E<'a>•+•for<'b>•R<'b>;    TypeAliasDeclaration
         dyn•for<'a>•E<'a>•+•for<'b>•R<'b>     TypeDynBounds{dyn}
             for<'a>•E<'a>                     TypeTraitBound{!maybeConst, !optional}
             for<'a>                           TypeTraitBound.ltParameters{dk: "<>"}
                 'a                            GenericLtParameterDeclaration, LtIdentifier
                     E<'a>                     TypeCall
                      <'a>                     TypeCall.typeArguments{dk: "<>"}
                       'a                      LtIdentifier
                             for<'b>•R<'b>     TypeTraitBound{!maybeConst, !optional}
                             for<'b>           TypeTraitBound.ltParameters{dk: "<>"}
                                 'b            GenericLtParameterDeclaration, LtIdentifier
                                     R<'b>     TypeCall
                                      <'b>     TypeCall.typeArguments{dk: "<>"}
                                       'b      LtIdentifier                                                                               */
type S = dyn Q<for<'a> fn(&'a u8)> + G<for<'b> fn(&'b u8)>;                                                                               /*
type•S•=•dyn•Q<for<'a>•fn(&'a•u8)>•+•G<for<'b>•fn(&'b•u8)>;    TypeAliasDeclaration
         dyn•Q<for<'a>•fn(&'a•u8)>•+•G<for<'b>•fn(&'b•u8)>     TypeDynBounds{dyn}
             Q<for<'a>•fn(&'a•u8)>                             TypeTraitBound{!maybeConst, !optional}, TypeCall
              <for<'a>•fn(&'a•u8)>                             TypeCall.typeArguments{dk: "<>"}
               for<'a>•fn(&'a•u8)                              TypeFnPointer
               for<'a>                                         TypeFnPointer.ltParameters{dk: "<>"}
                   'a                                          GenericLtParameterDeclaration, LtIdentifier
                         (&'a•u8)                              TypeFnPointer.parameters{dk: "()"}
                          &'a•u8                               TypeFnPointerParameter, TypeReference{!mut}
                           'a                                  LtIdentifier
                                     G<for<'b>•fn(&'b•u8)>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                                      <for<'b>•fn(&'b•u8)>     TypeCall.typeArguments{dk: "<>"}
                                       for<'b>•fn(&'b•u8)      TypeFnPointer
                                       for<'b>                 TypeFnPointer.ltParameters{dk: "<>"}
                                           'b                  GenericLtParameterDeclaration, LtIdentifier
                                                 (&'b•u8)      TypeFnPointer.parameters{dk: "()"}
                                                  &'b•u8       TypeFnPointerParameter, TypeReference{!mut}
                                                   'b          LtIdentifier                                                               */
type A = dyn ?Sized;                                                                                                                      /*
type•A•=•dyn•?Sized;    TypeAliasDeclaration
         dyn•?Sized     TypeDynBounds{dyn}
             ?Sized     TypeTraitBound{!maybeConst, optional}                                                                             */
type A = <S as Tr>::A::f<u8>;                                                                                                             /*
type•A•=•<S•as•Tr>::A::f<u8>;    TypeAliasDeclaration
         <S•as•Tr>::A::f<u8>     TypeCall
         <S•as•Tr>::A::f         TypePath
         <S•as•Tr>::A            TypePath
         <S•as•Tr>               ExpressionTypeSelector
                        <u8>     TypeCall.typeArguments{dk: "<>"}                                                                         */
trait A: B<i32> + std::fmt::Debug + Send + Sync {}                                                                                        /*
trait•A:•B<i32>•+•std::fmt::Debug•+•Send•+•Sync•{}    TraitDeclaration
         B<i32>                                       TypeTraitBound{!maybeConst, !optional}, TypeCall
          <i32>                                       TypeCall.typeArguments{dk: "<>"}
                  std::fmt::Debug                     TypeTraitBound{!maybeConst, !optional}, TypePath
                  std::fmt                            TypePath
                                    Send              TypeTraitBound{!maybeConst, !optional}
                                           Sync       TypeTraitBound{!maybeConst, !optional}
                                                {}    TraitDeclaration.body{dk: "{}"}                                                     */
struct R<Z:?Sized = E<i32, i32>>(Z);                                                                                                      /*
struct•R<Z:?Sized•=•E<i32,•i32>>(Z);    TupleStructDeclaration
        <Z:?Sized•=•E<i32,•i32>>        TupleStructDeclaration.generics{dk: "<>"}
         Z:?Sized•=•E<i32,•i32>         GenericTypeParameterDeclaration
           ?Sized                       TypeTraitBound{!maybeConst, optional}
                    E<i32,•i32>         TypeCall
                     <i32,•i32>         TypeCall.typeArguments{dk: "<>"}
                                (Z)     TupleStructDeclaration.items{dk: "()"}
                                 Z      TupleStructItemDeclaration                                                                        */
mod a {                                                                                                                                   /*
mod•a•{↲    <ModuleDeclaration>
      {↲    <ModuleDeclaration.body{dk: "{}"}>                                                                                            */
    trait A {                                                                                                                             /*
    trait•A•{↲    <TraitDeclaration>
            {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                       */
        const A: u8 = 0;                                                                                                                  /*
        const•A:•u8•=•0;    ConstVariableDeclaration
                      0     Literal{kind: Integer}                                                                                        */
    }                                                                                                                                     /*
••••}    </TraitDeclaration.body>
••••}    </TraitDeclaration>                                                                                                              */

    pub trait B {                                                                                                                         /*
    pub•trait•B•{↲    <TraitDeclaration>
    pub               PubSpecifier
                {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                   */
        const B: u8 = 0;                                                                                                                  /*
        const•B:•u8•=•0;    ConstVariableDeclaration
                      0     Literal{kind: Integer}                                                                                        */
    }                                                                                                                                     /*
••••}    </TraitDeclaration.body>
••••}    </TraitDeclaration>                                                                                                              */

    pub trait C: A + B {                                                                                                                  /*
    pub•trait•C:•A•+•B•{↲    <TraitDeclaration>
    pub                      PubSpecifier
                 A           TypeTraitBound{!maybeConst, !optional}
                     B       TypeTraitBound{!maybeConst, !optional}
                       {↲    <TraitDeclaration.body{dk: "{}"}>                                                                            */
        const C: u8 = 0;                                                                                                                  /*
        const•C:•u8•=•0;    ConstVariableDeclaration
                      0     Literal{kind: Integer}                                                                                        */
    }                                                                                                                                     /*
••••}    </TraitDeclaration.body>
••••}    </TraitDeclaration>                                                                                                              */

    impl A for ::S {}                                                                                                                     /*
    impl•A•for•::S•{}    ImplDeclaration{!const}
               ::S       TypePath
                   {}    ImplDeclaration.body{dk: "{}"}                                                                                   */
    impl B for ::S {}                                                                                                                     /*
    impl•B•for•::S•{}    ImplDeclaration{!const}
               ::S       TypePath
                   {}    ImplDeclaration.body{dk: "{}"}                                                                                   */
    impl C for ::S {}                                                                                                                     /*
    impl•C•for•::S•{}    ImplDeclaration{!const}
               ::S       TypePath
                   {}    ImplDeclaration.body{dk: "{}"}                                                                                   */
}                                                                                                                                         /*
}    </ModuleDeclaration.body>
}    </ModuleDeclaration>                                                                                                                 */

pub type b = Box<dyn t + sync::Send + sync::Sync + 'static>;                                                                              /*
pub•type•b•=•Box<dyn•t•+•sync::Send•+•sync::Sync•+•'static>;    TypeAliasDeclaration
pub                                                             PubSpecifier
             Box<dyn•t•+•sync::Send•+•sync::Sync•+•'static>     TypeCall
                <dyn•t•+•sync::Send•+•sync::Sync•+•'static>     TypeCall.typeArguments{dk: "<>"}
                 dyn•t•+•sync::Send•+•sync::Sync•+•'static      TypeDynBounds{dyn}
                     t                                          TypeTraitBound{!maybeConst, !optional}
                         sync::Send                             TypeTraitBound{!maybeConst, !optional}, TypePath
                                      sync::Sync                TypeTraitBound{!maybeConst, !optional}, TypePath
                                                   'static      LtStatic                                                                  */
pub type b = Box<dyn for<'tcx> e<'tcx> + sync::Send + sync::Sync + 'static>;                                                              /*
pub•type•b•=•Box<dyn•for<'tcx>•e<'tcx>•+•sync::Send•+•sync::Sync•+•'static>;    TypeAliasDeclaration
pub                                                                             PubSpecifier
             Box<dyn•for<'tcx>•e<'tcx>•+•sync::Send•+•sync::Sync•+•'static>     TypeCall
                <dyn•for<'tcx>•e<'tcx>•+•sync::Send•+•sync::Sync•+•'static>     TypeCall.typeArguments{dk: "<>"}
                 dyn•for<'tcx>•e<'tcx>•+•sync::Send•+•sync::Sync•+•'static      TypeDynBounds{dyn}
                     for<'tcx>•e<'tcx>                                          TypeTraitBound{!maybeConst, !optional}
                     for<'tcx>                                                  TypeTraitBound.ltParameters{dk: "<>"}
                         'tcx                                                   GenericLtParameterDeclaration, LtIdentifier
                               e<'tcx>                                          TypeCall
                                <'tcx>                                          TypeCall.typeArguments{dk: "<>"}
                                 'tcx                                           LtIdentifier
                                         sync::Send                             TypeTraitBound{!maybeConst, !optional}, TypePath
                                                      sync::Sync                TypeTraitBound{!maybeConst, !optional}, TypePath
                                                                   'static      LtStatic
pub•type•b•=•Box<dyn•for<'tcx>•e<'tcx>•+•sync::Send•+•sync::Sync•+•'static>;    </Program.ast>
pub•type•b•=•Box<dyn•for<'tcx>•e<'tcx>•+•sync::Send•+•sync::Sync•+•'static>;    </Program>                                                */
// Discarded Nodes: 18
// Parsed Nodes: 4056
// state_rollbacks: 189
// Total '.charCodeAt()' calls: 19830 (49% re-reads)
// Unnecessary 'skip_whitespace()' calls: 3077
// source: "../../samples/types/types.rs"