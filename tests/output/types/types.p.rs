type A where 'a: 'b + 'c = u8;                                                                                                            /*
type•A•where•'a:•'b•+•'c•=•u8;    TypeAliasDeclaration
             'a:•'b•+•'c          WhereLtBoundDeclaration
             'a                   LtIdentifier
                 'b               LtIdentifier
                      'c          LtIdentifier                                                                                            */
type A where 'a: 'b + 'c = u8;                                                                                                            /*
type•A•where•'a:•'b•+•'c•=•u8;    TypeAliasDeclaration
             'a:•'b•+•'c          WhereLtBoundDeclaration
             'a                   LtIdentifier
                 'b               LtIdentifier
                      'c          LtIdentifier                                                                                            */
type A where 'a: 'b +    = u8;                                                                                                            /*
type•A•where•'a:•'b•+••••=•u8;    TypeAliasDeclaration
             'a:•'b•+             WhereLtBoundDeclaration
             'a                   LtIdentifier
                 'b               LtIdentifier                                                                                            */
type A where 'a: 'b,     = u8;                                                                                                            /*
type•A•where•'a:•'b,•••••=•u8;    TypeAliasDeclaration
             'a:•'b               WhereLtBoundDeclaration
             'a                   LtIdentifier
                 'b               LtIdentifier                                                                                            */
type A where 'a:         = u8;                                                                                                            /*
type•A•where•'a:•••••••••=•u8;    TypeAliasDeclaration
             'a:                  WhereLtBoundDeclaration
             'a                   LtIdentifier                                                                                            */
type A where 'a:,        = u8;                                                                                                            /*
type•A•where•'a:,••••••••=•u8;    TypeAliasDeclaration
             'a:                  WhereLtBoundDeclaration
             'a                   LtIdentifier                                                                                            */
type A where             = u8;                                                                                                            /*
type•A•where•••••••••••••=•u8;    TypeAliasDeclaration                                                                                    */

// type A = for<'a, T>       fn();
//•type•A•=•for<'a,•T>•••••••fn();    Comment
type A = for<'a: 'b + 'c> fn();                                                                                                           /*
type•A•=•for<'a:•'b•+•'c>•fn();    TypeAliasDeclaration
         for<'a:•'b•+•'c>•fn()     TypeFnPointer
             'a:•'b•+•'c           GenericLtParameterDeclaration
             'a                    LtIdentifier
                 'b                LtIdentifier
                      'c           LtIdentifier                                                                                           */
type A = for<'a: 'b +>    fn();                                                                                                           /*
type•A•=•for<'a:•'b•+>••••fn();    TypeAliasDeclaration
         for<'a:•'b•+>••••fn()     TypeFnPointer
             'a:•'b•+              GenericLtParameterDeclaration
             'a                    LtIdentifier
                 'b                LtIdentifier                                                                                           */
type A = for<'a: 'b,>     fn();                                                                                                           /*
type•A•=•for<'a:•'b,>•••••fn();    TypeAliasDeclaration
         for<'a:•'b,>•••••fn()     TypeFnPointer
             'a:•'b                GenericLtParameterDeclaration
             'a                    LtIdentifier
                 'b                LtIdentifier                                                                                           */
type A = for<'a:,>        fn();                                                                                                           /*
type•A•=•for<'a:,>••••••••fn();    TypeAliasDeclaration
         for<'a:,>••••••••fn()     TypeFnPointer
             'a:                   GenericLtParameterDeclaration
             'a                    LtIdentifier                                                                                           */
type A = for<'a:>         fn();                                                                                                           /*
type•A•=•for<'a:>•••••••••fn();    TypeAliasDeclaration
         for<'a:>•••••••••fn()     TypeFnPointer
             'a:                   GenericLtParameterDeclaration
             'a                    LtIdentifier                                                                                           */
type A = for<'a>          fn();                                                                                                           /*
type•A•=•for<'a>••••••••••fn();    TypeAliasDeclaration
         for<'a>••••••••••fn()     TypeFnPointer
             'a                    GenericLtParameterDeclaration, LtIdentifier                                                            */
type A = for<>            fn();                                                                                                           /*
type•A•=•for<>••••••••••••fn();    TypeAliasDeclaration
         for<>••••••••••••fn()     TypeFnPointer                                                                                          */

type A = Box<(Fn(u8) -> u8) + 'static + Send + Sync>;                                                                                     /*
type•A•=•Box<(Fn(u8)•->•u8)•+•'static•+•Send•+•Sync>;    TypeAliasDeclaration
         Box<(Fn(u8)•->•u8)•+•'static•+•Send•+•Sync>     TypeCall
             (Fn(u8)•->•u8)•+•'static•+•Send•+•Sync      TypeDynBounds
             (Fn(u8)•->•u8)                              TypeTraitBound
              Fn(u8)•->•u8                               TypeFunction
                              'static                    LtStatic
                                        Send             TypeTraitBound
                                               Sync      TypeTraitBound                                                                   */
type A = impl B;                                                                                                                          /*
type•A•=•impl•B;    TypeAliasDeclaration
         impl•B     TypeImplBounds
              B     TypeTraitBound                                                                                                        */

type A where = u8;                                                                                                                        /*
type•A•where•=•u8;    TypeAliasDeclaration                                                                                                */
type A where for<'a> for<'b> Trait1 + ?Trait2: 'a + Trait = u8;                                                                           /*
type•A•where•for<'a>•for<'b>•Trait1•+•?Trait2:•'a•+•Trait•=•u8;    TypeAliasDeclaration
             for<'a>•for<'b>•Trait1•+•?Trait2:•'a•+•Trait          WhereTypeBoundDeclaration
                 'a                                                GenericLtParameterDeclaration, LtIdentifier
                     for<'b>•Trait1•+•?Trait2                      TypeDynBounds
                     for<'b>•Trait1                                TypeTraitBound
                         'b                                        GenericLtParameterDeclaration, LtIdentifier
                                      ?Trait2                      TypeTraitBound
                                               'a                  LtIdentifier
                                                    Trait          TypeTraitBound                                                         */
type A where T: = u8;                                                                                                                     /*
type•A•where•T:•=•u8;    TypeAliasDeclaration
             T:          WhereTypeBoundDeclaration                                                                                        */
type A where T: Trait + = u8;                                                                                                             /*
type•A•where•T:•Trait•+•=•u8;    TypeAliasDeclaration
             T:•Trait•+          WhereTypeBoundDeclaration
                Trait            TypeTraitBound                                                                                           */
type A where T: Trait + Trait = u8;                                                                                                       /*
type•A•where•T:•Trait•+•Trait•=•u8;    TypeAliasDeclaration
             T:•Trait•+•Trait          WhereTypeBoundDeclaration
                Trait                  TypeTraitBound
                        Trait          TypeTraitBound                                                                                     */
type A where T: Trait, = u8;                                                                                                              /*
type•A•where•T:•Trait,•=•u8;    TypeAliasDeclaration
             T:•Trait           WhereTypeBoundDeclaration
                Trait           TypeTraitBound                                                                                            */
type A where T:, = u8;                                                                                                                    /*
type•A•where•T:,•=•u8;    TypeAliasDeclaration
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
           T                     GenericTypeParameterDeclaration
                    T:•B         WhereTypeBoundDeclaration
                       B         TypeTraitBound                                                                                           */

type A: Ord;                                                                                                                              /*
type•A:•Ord;    TypeAliasDeclaration
        Ord     TypeTraitBound                                                                                                            */
type B: Ord = u8;                                                                                                                         /*
type•B:•Ord•=•u8;    TypeAliasDeclaration
        Ord          TypeTraitBound                                                                                                       */
type C: Ord where 'static: 'static = u8;                                                                                                  /*
type•C:•Ord•where•'static:•'static•=•u8;    TypeAliasDeclaration
        Ord                                 TypeTraitBound
                  'static:•'static          WhereLtBoundDeclaration
                  'static                   LtStatic
                           'static          LtStatic                                                                                      */
type D<_T>: Ord;                                                                                                                          /*
type•D<_T>:•Ord;    TypeAliasDeclaration
       _T           GenericTypeParameterDeclaration
            Ord     TypeTraitBound                                                                                                        */
type E<_T>: Ord = u8;                                                                                                                     /*
type•E<_T>:•Ord•=•u8;    TypeAliasDeclaration
       _T                GenericTypeParameterDeclaration
            Ord          TypeTraitBound                                                                                                   */
type F<_T>: Ord where 'static: 'static = u8;                                                                                              /*
type•F<_T>:•Ord•where•'static:•'static•=•u8;    TypeAliasDeclaration
       _T                                       GenericTypeParameterDeclaration
            Ord                                 TypeTraitBound
                      'static:•'static          WhereLtBoundDeclaration
                      'static                   LtStatic
                               'static          LtStatic                                                                                  */

type Y<T> where Self: Sized = u32;                                                                                                        /*
type•Y<T>•where•Self:•Sized•=•u32;    TypeAliasDeclaration
       T                              GenericTypeParameterDeclaration
                Self:•Sized           WhereTypeBoundDeclaration
                      Sized           TypeTraitBound                                                                                      */
type Y<T>: A where Self: Sized;                                                                                                           /*
type•Y<T>:•A•where•Self:•Sized;    TypeAliasDeclaration
       T                           GenericTypeParameterDeclaration
           A                       TypeTraitBound
                   Self:•Sized     WhereTypeBoundDeclaration
                         Sized     TypeTraitBound                                                                                         */

pub const FN: &'static fn() = &(fop::<i32> as fn());                                                                                      /*
pub•const•FN:•&'static•fn()•=•&(fop::<i32>•as•fn());    ConstVariableDeclaration
pub                                                     PubSpecifier
              &'static•fn()                             TypeReference
               'static                                  LtStatic
                       fn()                             TypeFnPointer
                              &(fop::<i32>•as•fn())     ReferenceExpression
                                fop::<i32>•as•fn()      ExpressionAsTypeCast
                                fop::<i32>              ExpressionTypeCast
                                              fn()      TypeFnPointer                                                                     */
const A: &&&u32 = &&&42;                                                                                                                  /*
const•A:•&&&u32•=•&&&42;    ConstVariableDeclaration
         &&&u32             TypeReference
          &&u32             TypeReference
           &u32             TypeReference
                  &&&42     ReferenceExpression
                   &&42     ReferenceExpression
                    &42     ReferenceExpression
                     42     Literal                                                                                                       */
const CONST1: &[bool; 1] = &[true];                                                                                                       /*
const•CONST1:•&[bool;•1]•=•&[true];    ConstVariableDeclaration
              &[bool;•1]               TypeReference
               [bool;•1]               TypeSizedArray
                      1                Literal
                           &[true]     ReferenceExpression
                            [true]     ArrayLiteral
                             true      Literal                                                                                            */
const CONST: &[Option<()>; 1] = &[Some(())];                                                                                              /*
const•CONST:•&[Option<()>;•1]•=•&[Some(())];    ConstVariableDeclaration
             &[Option<()>;•1]                   TypeReference
              [Option<()>;•1]                   TypeSizedArray
               Option<()>                       TypeCall
                      ()                        TypeTuple
                           1                    Literal
                                &[Some(())]     ReferenceExpression
                                 [Some(())]     ArrayLiteral
                                  Some(())      CallExpression
                                       ()       TupleLiteral                                                                              */
const A: [u32; 1] = [4];const F: &'static dyn PartialEq<u32> = &7u32;                                                                     /*
const•A:•[u32;•1]•=•[4];                                                 ConstVariableDeclaration
         [u32;•1]                                                        TypeSizedArray
               1                                                         Literal
                    [4]                                                  ArrayLiteral
                     4                                                   Literal
                        const•F:•&'static•dyn•PartialEq<u32>•=•&7u32;    ConstVariableDeclaration
                                 &'static•dyn•PartialEq<u32>             TypeReference
                                  'static                                LtStatic
                                          dyn•PartialEq<u32>             TypeDynBounds
                                              PartialEq<u32>             TypeTraitBound, TypeCall
                                                               &7u32     ReferenceExpression
                                                                7u32     Literal
                                                                 u32     Identifier                                                       */
struct R<'a> { c: Box<dyn FnMut(&mut R, bool) + 'a> }                                                                                     /*
struct•R<'a>•{•c:•Box<dyn•FnMut(&mut•R,•bool)•+•'a>•}    StructDeclaration
         'a                                              GenericLtParameterDeclaration, LtIdentifier
               c:•Box<dyn•FnMut(&mut•R,•bool)•+•'a>      StructPropertyDeclaration
                  Box<dyn•FnMut(&mut•R,•bool)•+•'a>      TypeCall
                      dyn•FnMut(&mut•R,•bool)•+•'a       TypeDynBounds
                          FnMut(&mut•R,•bool)            TypeTraitBound, TypeFunction
                                &mut•R                   TypeReference
                                                'a       LtIdentifier                                                                     */
fn g() -> impl Tr2<m::Alias> {}                                                                                                           /*
fn•g()•->•impl•Tr2<m::Alias>•{}    FunctionDeclaration
          impl•Tr2<m::Alias>       TypeImplBounds
               Tr2<m::Alias>       TypeTraitBound, TypeCall
                   m::Alias        TypePath                                                                                               */
fn leak_dyn_nonprincipal() -> Box<dyn PubPrincipal + PrivNonPrincipal> {}                                                                 /*
fn•leak_dyn_nonprincipal()•->•Box<dyn•PubPrincipal•+•PrivNonPrincipal>•{}    FunctionDeclaration
                              Box<dyn•PubPrincipal•+•PrivNonPrincipal>       TypeCall
                                  dyn•PubPrincipal•+•PrivNonPrincipal        TypeDynBounds
                                      PubPrincipal                           TypeTraitBound
                                                     PrivNonPrincipal        TypeTraitBound                                               */
fn method() -> Self::Pub {}                                                                                                               /*
fn•method()•->•Self::Pub•{}    FunctionDeclaration
               Self::Pub       TypePath                                                                                                   */
fn f<T: PrivTr>(arg: T) {}                                                                                                                /*
fn•f<T:•PrivTr>(arg:•T)•{}    FunctionDeclaration
     T:•PrivTr                GenericTypeParameterDeclaration
        PrivTr                TypeTraitBound
                arg:•T        FunctionParameterDeclaration                                                                                */
pub fn unused<const T: usize>() -> usize {}                                                                                               /*
pub•fn•unused<const•T:•usize>()•->•usize•{}    FunctionDeclaration
pub                                            PubSpecifier
              const•T:•usize                   ConstTypeParameterDeclaration                                                              */
fn start(_: isize, _: *const *const u8) -> isize {}                                                                                       /*
fn•start(_:•isize,•_:•*const•*const•u8)•->•isize•{}    FunctionDeclaration
         _:•isize                                      FunctionParameterDeclaration
         _                                             WildcardPattern
                   _:•*const•*const•u8                 FunctionParameterDeclaration
                   _                                   WildcardPattern
                      *const•*const•u8                 TypeDereferenceConst
                             *const•u8                 TypeDereferenceConst                                                               */
fn as_ptr(&self) -> *const Self::Item;                                                                                                    /*
fn•as_ptr(&self)•->•*const•Self::Item;    FunctionDeclaration
          &self                           FunctionSelfParameterDeclaration
                    *const•Self::Item     TypeDereferenceConst
                           Self::Item     TypePath                                                                                        */
fn as_mut_ptr(&mut self) -> *mut Self::Item;                                                                                              /*
fn•as_mut_ptr(&mut•self)•->•*mut•Self::Item;    FunctionDeclaration
              &mut•self                         FunctionSelfParameterDeclaration
                            *mut•Self::Item     TypeDereferenceMut
                                 Self::Item     TypePath                                                                                  */
fn as_ptr(&self) -> *const T { self as *const _ as *const _ }                                                                             /*
fn•as_ptr(&self)•->•*const•T•{•self•as•*const•_•as•*const•_•}    FunctionDeclaration
          &self                                                  FunctionSelfParameterDeclaration
                    *const•T                                     TypeDereferenceConst
                               self•as•*const•_•as•*const•_      ExpressionStatement, ExpressionAsTypeCast
                               self•as•*const•_                  ExpressionAsTypeCast
                                       *const•_                  TypeDereferenceConst
                                              _                  TypeInferred
                                                   *const•_      TypeDereferenceConst
                                                          _      TypeInferred                                                             */
fn as_mut_ptr(&mut self) -> *mut T { self as *mut _ as *mut _}                                                                            /*
fn•as_mut_ptr(&mut•self)•->•*mut•T•{•self•as•*mut•_•as•*mut•_}    FunctionDeclaration
              &mut•self                                           FunctionSelfParameterDeclaration
                            *mut•T                                TypeDereferenceMut
                                     self•as•*mut•_•as•*mut•_     ExpressionStatement, ExpressionAsTypeCast
                                     self•as•*mut•_               ExpressionAsTypeCast
                                             *mut•_               TypeDereferenceMut
                                                  _               TypeInferred
                                                       *mut•_     TypeDereferenceMut
                                                            _     TypeInferred                                                            */
fn y_uses_f(f: impl Fn()) {}                                                                                                              /*
fn•y_uses_f(f:•impl•Fn())•{}    FunctionDeclaration
            f:•impl•Fn()        FunctionParameterDeclaration
               impl•Fn()        TypeImplBounds
                    Fn()        TypeTraitBound, TypeFunction                                                                              */
fn infer<T: a::B>(c: T) -> T { c }                                                                                                        /*
fn•infer<T:•a::B>(c:•T)•->•T•{•c•}    FunctionDeclaration
         T:•a::B                      GenericTypeParameterDeclaration
            a::B                      TypeTraitBound, TypePath
                  c:•T                FunctionParameterDeclaration
                               c      ExpressionStatement                                                                                 */
fn f1<'a, 'b, 'c>(_x: &'a u32, _y: &'b u32, _z: &'c u32) where 'c: 'a + 'b { }                                                            /*
fn•f1<'a,•'b,•'c>(_x:•&'a•u32,•_y:•&'b•u32,•_z:•&'c•u32)•where•'c:•'a•+•'b•{•}    FunctionDeclaration
      'a                                                                          GenericLtParameterDeclaration, LtIdentifier
          'b                                                                      GenericLtParameterDeclaration, LtIdentifier
              'c                                                                  GenericLtParameterDeclaration, LtIdentifier
                  _x:•&'a•u32                                                     FunctionParameterDeclaration
                      &'a•u32                                                     TypeReference
                       'a                                                         LtIdentifier
                               _y:•&'b•u32                                        FunctionParameterDeclaration
                                   &'b•u32                                        TypeReference
                                    'b                                            LtIdentifier
                                            _z:•&'c•u32                           FunctionParameterDeclaration
                                                &'c•u32                           TypeReference
                                                 'c                               LtIdentifier
                                                               'c:•'a•+•'b        WhereLtBoundDeclaration
                                                               'c                 LtIdentifier
                                                                   'a             LtIdentifier
                                                                        'b        LtIdentifier                                            */
fn syntax() {                                                                                                                             /*
fn•syntax()•{↲    <FunctionDeclaration>                                                                                                   */
    A::<T = u8, T: Ord, String>();                                                                                                        /*
    A::<T•=•u8,•T:•Ord,•String>();    ExpressionStatement
    A::<T•=•u8,•T:•Ord,•String>()     CallExpression
        T•=•u8                        TypeCallNamedArgument
                T:•Ord                TypeCallNamedBound
                   Ord                TypeTraitBound                                                                                      */
    A::<T = u8, 'a, T: Ord>();                                                                                                            /*
    A::<T•=•u8,•'a,•T:•Ord>();    ExpressionStatement
    A::<T•=•u8,•'a,•T:•Ord>()     CallExpression
        T•=•u8                    TypeCallNamedArgument
                'a                LtIdentifier
                    T:•Ord        TypeCallNamedBound
                       Ord        TypeTraitBound                                                                                          */
	fn y<'a>(y: &mut 'a + Send);                                                                                                          /*
    fn•y<'a>(y:•&mut•'a•+•Send);    FunctionDeclaration
         'a                         GenericLtParameterDeclaration, LtIdentifier
             y:•&mut•'a•+•Send      FunctionParameterDeclaration
                &mut•'a•+•Send      TypeReference
                     'a•+•Send      TypeDynBounds
                     'a             LtIdentifier
                          Send      TypeTraitBound                                                                                        */
	let z = y as &mut 'a + Send;                                                                                                          /*
    let•z•=•y•as•&mut•'a•+•Send;    LetVariableDeclaration
            y•as•&mut•'a•+•Send     ExpressionAsTypeCast
                 &mut•'a•+•Send     TypeReference
                      'a•+•Send     TypeDynBounds
                      'a            LtIdentifier
                           Send     TypeTraitBound                                                                                        */
	let x: &'static str = "A";                                                                                                            /*
    let•x:•&'static•str•=•"A";    LetVariableDeclaration
           &'static•str           TypeReference
            'static               LtStatic
                          "A"     Literal                                                                                                 */
	fn A() -> Box<<Self as A>::T>;                                                                                                        /*
    fn•A()•->•Box<<Self•as•A>::T>;    FunctionDeclaration
              Box<<Self•as•A>::T>     TypeCall
                  <Self•as•A>::T      TypePath
                  <Self•as•A>         ExpressionTypeSelector                                                                              */
	let a = |a, b: _| -> _ { 0 };                                                                                                         /*
    let•a•=•|a,•b:•_|•->•_•{•0•};    LetVariableDeclaration
            |a,•b:•_|•->•_•{•0•}     ClosureFunctionExpression
             a                       ClosureFunctionParameterDeclaration
                b:•_                 ClosureFunctionParameterDeclaration
                   _                 TypeInferred
                         _           TypeInferred
                           {•0•}     BlockExpression
                             0       ExpressionStatement, Literal                                                                         */
	let a:     &usize =         & 1;                                                                                                      /*
    let•a:•••••&usize•=•••••••••&•1;    LetVariableDeclaration
               &usize                   TypeReference
                                &•1     ReferenceExpression
                                  1     Literal                                                                                           */
    let a:    &&usize =       & & 1;                                                                                                      /*
    let•a:••••&&usize•=•••••••&•&•1;    LetVariableDeclaration
              &&usize                   TypeReference
               &usize                   TypeReference
                              &•&•1     ReferenceExpression
                                &•1     ReferenceExpression
                                  1     Literal                                                                                           */
    let a:   &&&usize =     & & & 1;                                                                                                      /*
    let•a:•••&&&usize•=•••••&•&•&•1;    LetVariableDeclaration
             &&&usize                   TypeReference
              &&usize                   TypeReference
               &usize                   TypeReference
                            &•&•&•1     ReferenceExpression
                              &•&•1     ReferenceExpression
                                &•1     ReferenceExpression
                                  1     Literal                                                                                           */
    let a:  & &&usize =     & & & 1;                                                                                                      /*
    let•a:••&•&&usize•=•••••&•&•&•1;    LetVariableDeclaration
            &•&&usize                   TypeReference
              &&usize                   TypeReference
               &usize                   TypeReference
                            &•&•&•1     ReferenceExpression
                              &•&•1     ReferenceExpression
                                &•1     ReferenceExpression
                                  1     Literal                                                                                           */
    let a:  &&&&usize =   & & & & 1;                                                                                                      /*
    let•a:••&&&&usize•=•••&•&•&•&•1;    LetVariableDeclaration
            &&&&usize                   TypeReference
             &&&usize                   TypeReference
              &&usize                   TypeReference
               &usize                   TypeReference
                          &•&•&•&•1     ReferenceExpression
                            &•&•&•1     ReferenceExpression
                              &•&•1     ReferenceExpression
                                &•1     ReferenceExpression
                                  1     Literal                                                                                           */
    let a: & &&&usize =   & & & & 1;                                                                                                      /*
    let•a:•&•&&&usize•=•••&•&•&•&•1;    LetVariableDeclaration
           &•&&&usize                   TypeReference
             &&&usize                   TypeReference
              &&usize                   TypeReference
               &usize                   TypeReference
                          &•&•&•&•1     ReferenceExpression
                            &•&•&•1     ReferenceExpression
                              &•&•1     ReferenceExpression
                                &•1     ReferenceExpression
                                  1     Literal                                                                                           */
    let a: &&&&&usize = & & & & & 1;                                                                                                      /*
    let•a:•&&&&&usize•=•&•&•&•&•&•1;    LetVariableDeclaration
           &&&&&usize                   TypeReference
            &&&&usize                   TypeReference
             &&&usize                   TypeReference
              &&usize                   TypeReference
               &usize                   TypeReference
                        &•&•&•&•&•1     ReferenceExpression
                          &•&•&•&•1     ReferenceExpression
                            &•&•&•1     ReferenceExpression
                              &•&•1     ReferenceExpression
                                &•1     ReferenceExpression
                                  1     Literal                                                                                           */
	let a: Box<Debug+> = box 3 as Box<Debug+>;                                                                                            /*
    let•a:•Box<Debug+>•=•box•3•as•Box<Debug+>;    LetVariableDeclaration
           Box<Debug+>                            TypeCall
               Debug+                             TypeDynBounds
               Debug                              TypeTraitBound
                         box•3•as•Box<Debug+>     ExpressionAsTypeCast
                         box•3                    BoxExpression
                             3                    Literal
                                  Box<Debug+>     TypeCall
                                      Debug+      TypeDynBounds
                                      Debug       TypeTraitBound                                                                          */
	let a: Box<((A)) + B>;                                                                                                                /*
    let•a:•Box<((A))•+•B>;    LetVariableDeclaration
           Box<((A))•+•B>     TypeCall
               ((A))•+•B      TypeDynBounds
               ((A))          TypeTraitBound
                       B      TypeTraitBound                                                                                              */
	let a: Box<(A + B) + C>;                                                                                                              /*
    let•a:•Box<(A•+•B)•+•C>;    LetVariableDeclaration
           Box<(A•+•B)•+•C>     TypeCall
               (A•+•B)•+•C      TypeDynBounds
               (A•+•B)          TypeTraitBound
                A•+•B           TypeDynBounds
                A               TypeTraitBound
                    B           TypeTraitBound
                         C      TypeTraitBound                                                                                            */
	let a: Box<(A +) + B>;                                                                                                                /*
    let•a:•Box<(A•+)•+•B>;    LetVariableDeclaration
           Box<(A•+)•+•B>     TypeCall
               (A•+)•+•B      TypeDynBounds
               (A•+)          TypeTraitBound
                A•+           TypeDynBounds
                A             TypeTraitBound
                       B      TypeTraitBound                                                                                              */
	let a: Box<(dyn A) + B>;                                                                                                              /*
    let•a:•Box<(dyn•A)•+•B>;    LetVariableDeclaration
           Box<(dyn•A)•+•B>     TypeCall
               (dyn•A)•+•B      TypeDynBounds
               (dyn•A)          TypeTraitBound
                dyn•A           TypeDynBounds
                    A           TypeTraitBound
                         B      TypeTraitBound                                                                                            */
	let a: Box<dyn A + (B + C)>;                                                                                                          /*
    let•a:•Box<dyn•A•+•(B•+•C)>;    LetVariableDeclaration
           Box<dyn•A•+•(B•+•C)>     TypeCall
               dyn•A•+•(B•+•C)      TypeDynBounds
                   A                TypeTraitBound
                        B•+•C       TypeTraitBound, TypeDynBounds
                        B           TypeTraitBound
                            C       TypeTraitBound                                                                                        */
	let a: Box<impl A + (B + C)>;                                                                                                         /*
    let•a:•Box<impl•A•+•(B•+•C)>;    LetVariableDeclaration
           Box<impl•A•+•(B•+•C)>     TypeCall
               impl•A•+•(B•+•C)      TypeImplBounds
                    A                TypeTraitBound
                         B•+•C       TypeTraitBound, TypeDynBounds
                         B           TypeTraitBound
                             C       TypeTraitBound                                                                                       */
	let a: Box<(impl A + B) + C>;                                                                                                         /*
    let•a:•Box<(impl•A•+•B)•+•C>;    LetVariableDeclaration
           Box<(impl•A•+•B)•+•C>     TypeCall
               (impl•A•+•B)•+•C      TypeDynBounds
               (impl•A•+•B)          TypeTraitBound
                impl•A•+•B           TypeImplBounds
                     A               TypeTraitBound
                         B           TypeTraitBound
                              C      TypeTraitBound                                                                                       */
	let a: Box<impl A + (B + C) + D>;                                                                                                     /*
    let•a:•Box<impl•A•+•(B•+•C)•+•D>;    LetVariableDeclaration
           Box<impl•A•+•(B•+•C)•+•D>     TypeCall
               impl•A•+•(B•+•C)•+•D      TypeImplBounds
                    A                    TypeTraitBound
                         B•+•C           TypeTraitBound, TypeDynBounds
                         B               TypeTraitBound
                             C           TypeTraitBound
                                  D      TypeTraitBound                                                                                   */
	let a: Box<dyn A + (B + (C + D)) + E>;                                                                                                /*
    let•a:•Box<dyn•A•+•(B•+•(C•+•D))•+•E>;    LetVariableDeclaration
           Box<dyn•A•+•(B•+•(C•+•D))•+•E>     TypeCall
               dyn•A•+•(B•+•(C•+•D))•+•E      TypeDynBounds
                   A                          TypeTraitBound
                        B•+•(C•+•D)           TypeTraitBound, TypeDynBounds
                        B                     TypeTraitBound
                             C•+•D            TypeTraitBound, TypeDynBounds
                             C                TypeTraitBound
                                 D            TypeTraitBound
                                       E      TypeTraitBound                                                                              */
	let a: &for<'a> Trait<'a> + 'static;                                                                                                  /*
    let•a:•&for<'a>•Trait<'a>•+•'static;    LetVariableDeclaration
           &for<'a>•Trait<'a>•+•'static     TypeReference
            for<'a>•Trait<'a>•+•'static     TypeDynBounds
            for<'a>•Trait<'a>               TypeTraitBound
                'a                          GenericLtParameterDeclaration, LtIdentifier
                    Trait<'a>               TypeCall
                          'a                LtIdentifier
                                'static     LtStatic                                                                                      */
	let a: &dyn PartialEq<u32> = &7u32;                                                                                                   /*
    let•a:•&dyn•PartialEq<u32>•=•&7u32;    LetVariableDeclaration
           &dyn•PartialEq<u32>             TypeReference
            dyn•PartialEq<u32>             TypeDynBounds
                PartialEq<u32>             TypeTraitBound, TypeCall
                                 &7u32     ReferenceExpression
                                  7u32     Literal
                                   u32     Identifier                                                                                     */
	let a: Option<!> = None;                                                                                                              /*
    let•a:•Option<!>•=•None;    LetVariableDeclaration
           Option<!>            TypeCall
                  !             TypeNever                                                                                                 */
	let a = &() as *const () as *const Bottom;                                                                                            /*
    let•a•=•&()•as•*const•()•as•*const•Bottom;    LetVariableDeclaration
            &()•as•*const•()•as•*const•Bottom     ExpressionAsTypeCast
            &()•as•*const•()                      ExpressionAsTypeCast
            &()                                   ReferenceExpression
             ()                                   TupleLiteral
                   *const•()                      TypeDereferenceConst
                          ()                      TypeTuple
                                *const•Bottom     TypeDereferenceConst                                                                    */
	let a = id(|_: &isize, _: &isize| {});                                                                                                /*
    let•a•=•id(|_:•&isize,•_:•&isize|•{});    LetVariableDeclaration
            id(|_:•&isize,•_:•&isize|•{})     CallExpression
               |_:•&isize,•_:•&isize|•{}      ClosureFunctionExpression
                _:•&isize                     ClosureFunctionParameterDeclaration
                _                             WildcardPattern
                   &isize                     TypeReference
                           _:•&isize          ClosureFunctionParameterDeclaration
                           _                  WildcardPattern
                              &isize          TypeReference
                                      {}      BlockExpression                                                                             */
	let a = id(|_: &isize, _: &isize| {});                                                                                                /*
    let•a•=•id(|_:•&isize,•_:•&isize|•{});    LetVariableDeclaration
            id(|_:•&isize,•_:•&isize|•{})     CallExpression
               |_:•&isize,•_:•&isize|•{}      ClosureFunctionExpression
                _:•&isize                     ClosureFunctionParameterDeclaration
                _                             WildcardPattern
                   &isize                     TypeReference
                           _:•&isize          ClosureFunctionParameterDeclaration
                           _                  WildcardPattern
                              &isize          TypeReference
                                      {}      BlockExpression                                                                             */



	fn equal1<T>(_: &T, _: &T) -> bool where {}                                                                                           /*
    fn•equal1<T>(_:•&T,•_:•&T)•->•bool•where•{}    FunctionDeclaration
              T                                    GenericTypeParameterDeclaration
                 _:•&T                             FunctionParameterDeclaration
                 _                                 WildcardPattern
                    &T                             TypeReference
                        _:•&T                      FunctionParameterDeclaration
                        _                          WildcardPattern
                           &T                      TypeReference                                                                          */
	fn equal2<T>(_: &T, _: &T) -> bool where T: {}                                                                                        /*
    fn•equal2<T>(_:•&T,•_:•&T)•->•bool•where•T:•{}    FunctionDeclaration
              T                                       GenericTypeParameterDeclaration
                 _:•&T                                FunctionParameterDeclaration
                 _                                    WildcardPattern
                    &T                                TypeReference
                        _:•&T                         FunctionParameterDeclaration
                        _                             WildcardPattern
                           &T                         TypeReference
                                             T:       WhereTypeBoundDeclaration                                                           */
	fn A<'a>() where 'a: {}                                                                                                               /*
    fn•A<'a>()•where•'a:•{}    FunctionDeclaration
         'a                    GenericLtParameterDeclaration, LtIdentifier
                     'a:       WhereLtBoundDeclaration
                     'a        LtIdentifier                                                                                               */
	pub fn A<T: 'static>(_: T) -> TypeId {}                                                                                               /*
    pub•fn•A<T:•'static>(_:•T)•->•TypeId•{}    FunctionDeclaration
    pub                                        PubSpecifier
             T:•'static                        GenericTypeParameterDeclaration
                'static                        LtStatic
                         _:•T                  FunctionParameterDeclaration
                         _                     WildcardPattern                                                                            */
	pub fn unused<'a, T>(_: &'a u32) {}                                                                                                   /*
    pub•fn•unused<'a,•T>(_:•&'a•u32)•{}    FunctionDeclaration
    pub                                    PubSpecifier
                  'a                       GenericLtParameterDeclaration, LtIdentifier
                      T                    GenericTypeParameterDeclaration
                         _:•&'a•u32        FunctionParameterDeclaration
                         _                 WildcardPattern
                            &'a•u32        TypeReference
                             'a            LtIdentifier                                                                                   */
	let f: fn(_, i32) -> i32 = q;                                                                                                         /*
    let•f:•fn(_,•i32)•->•i32•=•q;    LetVariableDeclaration
           fn(_,•i32)•->•i32         TypeFnPointer
              _                      TypeFnPointerParameter, TypeInferred
                 i32                 TypeFnPointerParameter                                                                               */
    let _ = S::<>;                                                                                                                        /*
    let•_•=•S::<>;    LetVariableDeclaration
        _             WildcardPattern
            S::<>     ExpressionTypeCast                                                                                                  */
    let _ = E::<>::V;                                                                                                                     /*
    let•_•=•E::<>::V;    LetVariableDeclaration
        _                WildcardPattern
            E::<>::V     ExpressionPath
            E::<>        ExpressionTypeCast                                                                                               */
    let a: i32<>;                                                                                                                         /*
    let•a:•i32<>;    LetVariableDeclaration
           i32<>     TypeCall                                                                                                             */
	let a = (                                                                                                                             /*
    let•a•=•(↲    <LetVariableDeclaration>
            (↲    <TupleLiteral>                                                                                                          */
		A::b::<fn(&'static isize, &'static isize)>(),                                                                                     /*
        A::b::<fn(&'static•isize,•&'static•isize)>()    CallExpression
        A::b                                            ExpressionPath
               fn(&'static•isize,•&'static•isize)       TypeFnPointer
                  &'static•isize                        TypeFnPointerParameter, TypeReference
                   'static                              LtStatic
                                  &'static•isize        TypeFnPointerParameter, TypeReference
                                   'static              LtStatic                                                                          */
		A::b::<for<'a> fn(&'static isize, &'a isize)>(),                                                                                  /*
        A::b::<for<'a>•fn(&'static•isize,•&'a•isize)>()    CallExpression
        A::b                                               ExpressionPath
               for<'a>•fn(&'static•isize,•&'a•isize)       TypeFnPointer
                   'a                                      GenericLtParameterDeclaration, LtIdentifier
                          &'static•isize                   TypeFnPointerParameter, TypeReference
                           'static                         LtStatic
                                          &'a•isize        TypeFnPointerParameter, TypeReference
                                           'a              LtIdentifier                                                                   */
		A::b::<for<'a, 'b> fn(&'a isize, &'b isize)>(),                                                                                   /*
        A::b::<for<'a,•'b>•fn(&'a•isize,•&'b•isize)>()    CallExpression
        A::b                                              ExpressionPath
               for<'a,•'b>•fn(&'a•isize,•&'b•isize)       TypeFnPointer
                   'a                                     GenericLtParameterDeclaration, LtIdentifier
                       'b                                 GenericLtParameterDeclaration, LtIdentifier
                              &'a•isize                   TypeFnPointerParameter, TypeReference
                               'a                         LtIdentifier
                                         &'b•isize        TypeFnPointerParameter, TypeReference
                                          'b              LtIdentifier                                                                    */
		A::b::<for<'a, 'b> fn(&'b isize, &'a isize)>(),                                                                                   /*
        A::b::<for<'a,•'b>•fn(&'b•isize,•&'a•isize)>()    CallExpression
        A::b                                              ExpressionPath
               for<'a,•'b>•fn(&'b•isize,•&'a•isize)       TypeFnPointer
                   'a                                     GenericLtParameterDeclaration, LtIdentifier
                       'b                                 GenericLtParameterDeclaration, LtIdentifier
                              &'b•isize                   TypeFnPointerParameter, TypeReference
                               'b                         LtIdentifier
                                         &'a•isize        TypeFnPointerParameter, TypeReference
                                          'a              LtIdentifier                                                                    */
		A::b::<for<'a> fn(fn(&'a isize) -> &'a isize)>(),                                                                                 /*
        A::b::<for<'a>•fn(fn(&'a•isize)•->•&'a•isize)>()    CallExpression
        A::b                                                ExpressionPath
               for<'a>•fn(fn(&'a•isize)•->•&'a•isize)       TypeFnPointer
                   'a                                       GenericLtParameterDeclaration, LtIdentifier
                          fn(&'a•isize)•->•&'a•isize        TypeFnPointerParameter, TypeFnPointer
                             &'a•isize                      TypeFnPointerParameter, TypeReference
                              'a                            LtIdentifier
                                           &'a•isize        TypeReference
                                            'a              LtIdentifier                                                                  */
		A::b::<fn(for<'a> fn(&'a isize) -> &'a isize)>(),                                                                                 /*
        A::b::<fn(for<'a>•fn(&'a•isize)•->•&'a•isize)>()    CallExpression
        A::b                                                ExpressionPath
               fn(for<'a>•fn(&'a•isize)•->•&'a•isize)       TypeFnPointer
                  for<'a>•fn(&'a•isize)•->•&'a•isize        TypeFnPointerParameter, TypeFnPointer
                      'a                                    GenericLtParameterDeclaration, LtIdentifier
                             &'a•isize                      TypeFnPointerParameter, TypeReference
                              'a                            LtIdentifier
                                           &'a•isize        TypeReference
                                            'a              LtIdentifier                                                                  */
		A::b::<for<'a> fn(&'a dyn Trait<'a>) -> Struct<'a>>(),                                                                            /*
        A::b::<for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'a>>()    CallExpression
        A::b                                                     ExpressionPath
               for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'a>       TypeFnPointer
                   'a                                            GenericLtParameterDeclaration, LtIdentifier
                          &'a•dyn•Trait<'a>                      TypeFnPointerParameter, TypeReference
                           'a                                    LtIdentifier
                              dyn•Trait<'a>                      TypeDynBounds
                                  Trait<'a>                      TypeTraitBound, TypeCall
                                        'a                       LtIdentifier
                                                Struct<'a>       TypeCall
                                                       'a        LtIdentifier                                                             */
		A::b::<for<'a> fn(&'a dyn Trait<'a>) -> Struct<'static>>(),                                                                       /*
        A::b::<for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'static>>()    CallExpression
        A::b                                                          ExpressionPath
               for<'a>•fn(&'a•dyn•Trait<'a>)•->•Struct<'static>       TypeFnPointer
                   'a                                                 GenericLtParameterDeclaration, LtIdentifier
                          &'a•dyn•Trait<'a>                           TypeFnPointerParameter, TypeReference
                           'a                                         LtIdentifier
                              dyn•Trait<'a>                           TypeDynBounds
                                  Trait<'a>                           TypeTraitBound, TypeCall
                                        'a                            LtIdentifier
                                                Struct<'static>       TypeCall
                                                       'static        LtStatic                                                            */
		A::b::<for<'a, 'b> fn(&'a dyn Trait<'b>) -> Struct<'b>>(),                                                                        /*
        A::b::<for<'a,•'b>•fn(&'a•dyn•Trait<'b>)•->•Struct<'b>>()    CallExpression
        A::b                                                         ExpressionPath
               for<'a,•'b>•fn(&'a•dyn•Trait<'b>)•->•Struct<'b>       TypeFnPointer
                   'a                                                GenericLtParameterDeclaration, LtIdentifier
                       'b                                            GenericLtParameterDeclaration, LtIdentifier
                              &'a•dyn•Trait<'b>                      TypeFnPointerParameter, TypeReference
                               'a                                    LtIdentifier
                                  dyn•Trait<'b>                      TypeDynBounds
                                      Trait<'b>                      TypeTraitBound, TypeCall
                                            'b                       LtIdentifier
                                                    Struct<'b>       TypeCall
                                                           'b        LtIdentifier                                                         */
		A::b::<fn(for<'a> fn(&'a isize) -> &'a usize)>(),                                                                                 /*
        A::b::<fn(for<'a>•fn(&'a•isize)•->•&'a•usize)>()    CallExpression
        A::b                                                ExpressionPath
               fn(for<'a>•fn(&'a•isize)•->•&'a•usize)       TypeFnPointer
                  for<'a>•fn(&'a•isize)•->•&'a•usize        TypeFnPointerParameter, TypeFnPointer
                      'a                                    GenericLtParameterDeclaration, LtIdentifier
                             &'a•isize                      TypeFnPointerParameter, TypeReference
                              'a                            LtIdentifier
                                           &'a•usize        TypeReference
                                            'a              LtIdentifier                                                                  */
		A::b::<fn(for<'b> fn(&'b isize) -> &'b usize)>(),                                                                                 /*
        A::b::<fn(for<'b>•fn(&'b•isize)•->•&'b•usize)>()    CallExpression
        A::b                                                ExpressionPath
               fn(for<'b>•fn(&'b•isize)•->•&'b•usize)       TypeFnPointer
                  for<'b>•fn(&'b•isize)•->•&'b•usize        TypeFnPointerParameter, TypeFnPointer
                      'b                                    GenericLtParameterDeclaration, LtIdentifier
                             &'b•isize                      TypeFnPointerParameter, TypeReference
                              'b                            LtIdentifier
                                           &'b•usize        TypeReference
                                            'b              LtIdentifier                                                                  */
		A::b::<Box<dyn Fn(&'static isize, &'static isize)>>(),                                                                            /*
        A::b::<Box<dyn•Fn(&'static•isize,•&'static•isize)>>()    CallExpression
        A::b                                                     ExpressionPath
               Box<dyn•Fn(&'static•isize,•&'static•isize)>       TypeCall
                   dyn•Fn(&'static•isize,•&'static•isize)        TypeDynBounds
                       Fn(&'static•isize,•&'static•isize)        TypeTraitBound, TypeFunction
                          &'static•isize                         TypeReference
                           'static                               LtStatic
                                          &'static•isize         TypeReference
                                           'static               LtStatic                                                                 */
		A::b::<Box<dyn for<'a> Fn(&'static isize, &'a isize)>>(),                                                                         /*
        A::b::<Box<dyn•for<'a>•Fn(&'static•isize,•&'a•isize)>>()    CallExpression
        A::b                                                        ExpressionPath
               Box<dyn•for<'a>•Fn(&'static•isize,•&'a•isize)>       TypeCall
                   dyn•for<'a>•Fn(&'static•isize,•&'a•isize)        TypeDynBounds
                       for<'a>•Fn(&'static•isize,•&'a•isize)        TypeTraitBound
                           'a                                       GenericLtParameterDeclaration, LtIdentifier
                               Fn(&'static•isize,•&'a•isize)        TypeFunction
                                  &'static•isize                    TypeReference
                                   'static                          LtStatic
                                                  &'a•isize         TypeReference
                                                   'a               LtIdentifier                                                          */
		A::b::<Box<dyn for<'a, 'b> Fn(&'a isize, &'b isize)>>(),                                                                          /*
        A::b::<Box<dyn•for<'a,•'b>•Fn(&'a•isize,•&'b•isize)>>()    CallExpression
        A::b                                                       ExpressionPath
               Box<dyn•for<'a,•'b>•Fn(&'a•isize,•&'b•isize)>       TypeCall
                   dyn•for<'a,•'b>•Fn(&'a•isize,•&'b•isize)        TypeDynBounds
                       for<'a,•'b>•Fn(&'a•isize,•&'b•isize)        TypeTraitBound
                           'a                                      GenericLtParameterDeclaration, LtIdentifier
                               'b                                  GenericLtParameterDeclaration, LtIdentifier
                                   Fn(&'a•isize,•&'b•isize)        TypeFunction
                                      &'a•isize                    TypeReference
                                       'a                          LtIdentifier
                                                 &'b•isize         TypeReference
                                                  'b               LtIdentifier                                                           */
		A::b::<Box<dyn for<'a, 'b> Fn(&'b isize, &'a isize)>>(),                                                                          /*
        A::b::<Box<dyn•for<'a,•'b>•Fn(&'b•isize,•&'a•isize)>>()    CallExpression
        A::b                                                       ExpressionPath
               Box<dyn•for<'a,•'b>•Fn(&'b•isize,•&'a•isize)>       TypeCall
                   dyn•for<'a,•'b>•Fn(&'b•isize,•&'a•isize)        TypeDynBounds
                       for<'a,•'b>•Fn(&'b•isize,•&'a•isize)        TypeTraitBound
                           'a                                      GenericLtParameterDeclaration, LtIdentifier
                               'b                                  GenericLtParameterDeclaration, LtIdentifier
                                   Fn(&'b•isize,•&'a•isize)        TypeFunction
                                      &'b•isize                    TypeReference
                                       'b                          LtIdentifier
                                                 &'a•isize         TypeReference
                                                  'a               LtIdentifier                                                           */
		A::b::<Box<dyn for<'a> Fn(Box<dyn Fn(&'a isize) -> &'a isize>)>>(),                                                               /*
        A::b::<Box<dyn•for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)>>()    CallExpression
        A::b                                                                  ExpressionPath
               Box<dyn•for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)>       TypeCall
                   dyn•for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)        TypeDynBounds
                       for<'a>•Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)        TypeTraitBound
                           'a                                                 GenericLtParameterDeclaration, LtIdentifier
                               Fn(Box<dyn•Fn(&'a•isize)•->•&'a•isize>)        TypeFunction
                                  Box<dyn•Fn(&'a•isize)•->•&'a•isize>         TypeCall
                                      dyn•Fn(&'a•isize)•->•&'a•isize          TypeDynBounds
                                          Fn(&'a•isize)•->•&'a•isize          TypeTraitBound, TypeFunction
                                             &'a•isize                        TypeReference
                                              'a                              LtIdentifier
                                                           &'a•isize          TypeReference
                                                            'a                LtIdentifier                                                */
		A::b::<Box<dyn Fn(Box<dyn for<'a> Fn(&'a isize) -> &'a isize>)>>(),                                                               /*
        A::b::<Box<dyn•Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)>>()    CallExpression
        A::b                                                                  ExpressionPath
               Box<dyn•Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)>       TypeCall
                   dyn•Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)        TypeDynBounds
                       Fn(Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>)        TypeTraitBound, TypeFunction
                          Box<dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize>         TypeCall
                              dyn•for<'a>•Fn(&'a•isize)•->•&'a•isize          TypeDynBounds
                                  for<'a>•Fn(&'a•isize)•->•&'a•isize          TypeTraitBound
                                      'a                                      GenericLtParameterDeclaration, LtIdentifier
                                          Fn(&'a•isize)•->•&'a•isize          TypeFunction
                                             &'a•isize                        TypeReference
                                              'a                              LtIdentifier
                                                           &'a•isize          TypeReference
                                                            'a                LtIdentifier                                                */
		a::<L64<L64<()>>>(),                                                                                                              /*
        a::<L64<L64<()>>>()    CallExpression
            L64<L64<()>>       TypeCall
                L64<()>        TypeCall
                    ()         TypeTuple                                                                                                  */
		a::<L<L64<L64<()>>>>(),                                                                                                           /*
        a::<L<L64<L64<()>>>>()    CallExpression
            L<L64<L64<()>>>       TypeCall
              L64<L64<()>>        TypeCall
                  L64<()>         TypeCall
                      ()          TypeTuple                                                                                               */
		<&dyn A<B = u8>>::x(&e::r(1)),                                                                                                    /*
        <&dyn•A<B•=•u8>>::x(&e::r(1))    CallExpression
        <&dyn•A<B•=•u8>>::x              ExpressionPath
        <&dyn•A<B•=•u8>>                 ExpressionTypeSelector
         &dyn•A<B•=•u8>                  TypeReference
          dyn•A<B•=•u8>                  TypeDynBounds
              A<B•=•u8>                  TypeTraitBound, TypeCall
                B•=•u8                   TypeCallNamedArgument
                            &e::r(1)     ReferenceExpression
                             e::r(1)     CallExpression
                             e::r        ExpressionPath
                                  1      Literal                                                                                          */
		<&'static str>::f(&""),                                                                                                           /*
        <&'static•str>::f(&"")    CallExpression
        <&'static•str>::f         ExpressionPath
        <&'static•str>            ExpressionTypeSelector
         &'static•str             TypeReference
          'static                 LtStatic
                          &""     ReferenceExpression
                           ""     Literal                                                                                                 */
		a::<>(),                                                                                                                          /*
        a::<>()    CallExpression                                                                                                         */
		a as &[&dyn Fn(usize)->()],                                                                                                       /*
        a•as•&[&dyn•Fn(usize)->()]    ExpressionAsTypeCast
             &[&dyn•Fn(usize)->()]    TypeReference
              [&dyn•Fn(usize)->()]    TypeSlice
               &dyn•Fn(usize)->()     TypeReference
                dyn•Fn(usize)->()     TypeDynBounds
                    Fn(usize)->()     TypeTraitBound, TypeFunction
                               ()     TypeTuple                                                                                           */
		a::<&U>(a),                                                                                                                       /*
        a::<&U>(a)    CallExpression
            &U        TypeReference                                                                                                       */
		a::<U>(a),                                                                                                                        /*
        a::<U>(a)    CallExpression                                                                                                       */
		a::<&mut U>(a),                                                                                                                   /*
        a::<&mut•U>(a)    CallExpression
            &mut•U        TypeReference                                                                                                   */

	);                                                                                                                                    /*
   ╚);    </LetVariableDeclaration>
   ╚)     </TupleLiteral>                                                                                                                 */
	let s: Foo<'a'> = Foo;                                                                                                                /*
    let•s:•Foo<'a'>•=•Foo;    LetVariableDeclaration
           Foo<'a'>           TypeCall
               'a'            Literal                                                                                                     */
	let _: Foo<'b'> = s.into();                                                                                                           /*
    let•_:•Foo<'b'>•=•s.into();    LetVariableDeclaration
        _                          WildcardPattern
           Foo<'b'>                TypeCall
               'b'                 Literal
                      s.into()     CallExpression                                                                                         */
	let s2: Foo<'a'> = Foo;                                                                                                               /*
    let•s2:•Foo<'a'>•=•Foo;    LetVariableDeclaration
            Foo<'a'>           TypeCall
                'a'            Literal                                                                                                    */
	let _: Foo<'a'> = s2;                                                                                                                 /*
    let•_:•Foo<'a'>•=•s2;    LetVariableDeclaration
        _                    WildcardPattern
           Foo<'a'>          TypeCall
               'a'           Literal                                                                                                      */
	let s3: Foo<'a'> = Foo;                                                                                                               /*
    let•s3:•Foo<'a'>•=•Foo;    LetVariableDeclaration
            Foo<'a'>           TypeCall
                'a'            Literal                                                                                                    */
	let _ = s3;                                                                                                                           /*
    let•_•=•s3;    LetVariableDeclaration
        _          WildcardPattern                                                                                                        */
	let s4: Foo<'a'> = Foo;                                                                                                               /*
    let•s4:•Foo<'a'>•=•Foo;    LetVariableDeclaration
            Foo<'a'>           TypeCall
                'a'            Literal                                                                                                    */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
fn A(x: Option<fn(i32)>) -> Option<unsafe fn(i32)> {}                                                                                     /*
fn•A(x:•Option<fn(i32)>)•->•Option<unsafe•fn(i32)>•{}    FunctionDeclaration
     x:•Option<fn(i32)>                                  FunctionParameterDeclaration
        Option<fn(i32)>                                  TypeCall
               fn(i32)                                   TypeFnPointer
                  i32                                    TypeFnPointerParameter
                            Option<unsafe•fn(i32)>       TypeCall
                                   unsafe•fn(i32)        TypeFnPointer
                                             i32         TypeFnPointerParameter                                                           */
fn f(x: fn(i32)) -> unsafe fn(i32) {}                                                                                                     /*
fn•f(x:•fn(i32))•->•unsafe•fn(i32)•{}    FunctionDeclaration
     x:•fn(i32)                          FunctionParameterDeclaration
        fn(i32)                          TypeFnPointer
           i32                           TypeFnPointerParameter
                    unsafe•fn(i32)       TypeFnPointer
                              i32        TypeFnPointerParameter                                                                           */
fn f<'b, L: X<&'b Q<K>>>() {}                                                                                                             /*
fn•f<'b,•L:•X<&'b•Q<K>>>()•{}    FunctionDeclaration
     'b                          GenericLtParameterDeclaration, LtIdentifier
         L:•X<&'b•Q<K>>          GenericTypeParameterDeclaration
            X<&'b•Q<K>>          TypeTraitBound, TypeCall
              &'b•Q<K>           TypeReference
               'b                LtIdentifier
                  Q<K>           TypeCall                                                                                                 */
struct A<T, U = [u8; a::<T>()]>(T, U);                                                                                                    /*
struct•A<T,•U•=•[u8;•a::<T>()]>(T,•U);    TupleStructDeclaration
         T                                GenericTypeParameterDeclaration
            U•=•[u8;•a::<T>()]            GenericTypeParameterDeclaration
                [u8;•a::<T>()]            TypeSizedArray
                     a::<T>()             CallExpression
                                T         TupleStructItemDeclaration
                                   U      TupleStructItemDeclaration                                                                      */
impl Q for () {}                                                                                                                          /*
impl•Q•for•()•{}    ImplDeclaration
           ()       TypeTuple                                                                                                             */
trait Q<P = Self> {}                                                                                                                      /*
trait•Q<P•=•Self>•{}    TraitDeclaration
        P•=•Self        GenericTypeParameterDeclaration                                                                                   */
trait Q<P: Sized = [Self]> {}                                                                                                             /*
trait•Q<P:•Sized•=•[Self]>•{}    TraitDeclaration
        P:•Sized•=•[Self]        GenericTypeParameterDeclaration
           Sized                 TypeTraitBound
                   [Self]        TypeSlice                                                                                                */
trait H<'d, 'e>: for<'f> I<'d, 'f, 'e> + 'd {}                                                                                            /*
trait•H<'d,•'e>:•for<'f>•I<'d,•'f,•'e>•+•'d•{}    TraitDeclaration
        'd                                        GenericLtParameterDeclaration, LtIdentifier
            'e                                    GenericLtParameterDeclaration, LtIdentifier
                 for<'f>•I<'d,•'f,•'e>            TypeTraitBound
                     'f                           GenericLtParameterDeclaration, LtIdentifier
                         I<'d,•'f,•'e>            TypeCall
                           'd                     LtIdentifier
                               'f                 LtIdentifier
                                   'e             LtIdentifier
                                         'd       LtIdentifier                                                                            */
trait F<'f>: for<'a> A<'a> + for<'e> E<'e> {}                                                                                             /*
trait•F<'f>:•for<'a>•A<'a>•+•for<'e>•E<'e>•{}    TraitDeclaration
        'f                                       GenericLtParameterDeclaration, LtIdentifier
             for<'a>•A<'a>                       TypeTraitBound
                 'a                              GenericLtParameterDeclaration, LtIdentifier
                     A<'a>                       TypeCall
                       'a                        LtIdentifier
                             for<'e>•E<'e>       TypeTraitBound
                                 'e              GenericLtParameterDeclaration, LtIdentifier
                                     E<'e>       TypeCall
                                       'e        LtIdentifier                                                                             */
struct Q<A = S, T>(A, T);                                                                                                                 /*
struct•Q<A•=•S,•T>(A,•T);    TupleStructDeclaration
         A•=•S               GenericTypeParameterDeclaration
                T            GenericTypeParameterDeclaration
                   A         TupleStructItemDeclaration
                      T      TupleStructItemDeclaration                                                                                   */
struct Q<A, B = Vec<C>, C>(A, B, C);                                                                                                      /*
struct•Q<A,•B•=•Vec<C>,•C>(A,•B,•C);    TupleStructDeclaration
         A                              GenericTypeParameterDeclaration
            B•=•Vec<C>                  GenericTypeParameterDeclaration
                Vec<C>                  TypeCall
                        C               GenericTypeParameterDeclaration
                           A            TupleStructItemDeclaration
                              B         TupleStructItemDeclaration
                                 C      TupleStructItemDeclaration                                                                        */
impl<'a> A<'a> for &'a str { fn f<T: B<'a>>(self) -> &'a str {} }                                                                         /*
impl<'a>•A<'a>•for•&'a•str•{•fn•f<T:•B<'a>>(self)•->•&'a•str•{}•}    ImplDeclaration
     'a                                                              GenericLtParameterDeclaration, LtIdentifier
         A<'a>                                                       TypeCall
           'a                                                        LtIdentifier
                   &'a•str                                           TypeReference
                    'a                                               LtIdentifier
                             fn•f<T:•B<'a>>(self)•->•&'a•str•{}      FunctionDeclaration
                                  T:•B<'a>                           GenericTypeParameterDeclaration
                                     B<'a>                           TypeTraitBound, TypeCall
                                       'a                            LtIdentifier
                                            self                     FunctionSelfParameterDeclaration
                                                     &'a•str         TypeReference
                                                      'a             LtIdentifier                                                         */
extern "C" fn A<T: Add>(a: T, b: T) -> T::Output { a + b }                                                                                /*
extern•"C"•fn•A<T:•Add>(a:•T,•b:•T)•->•T::Output•{•a•+•b•}    FunctionDeclaration
extern•"C"                                                    ExternSpecifier
       "C"                                                    Literal
                T:•Add                                        GenericTypeParameterDeclaration
                   Add                                        TypeTraitBound
                        a:•T                                  FunctionParameterDeclaration
                              b:•T                            FunctionParameterDeclaration
                                       T::Output              TypePath
                                                   a•+•b      ExpressionStatement, OperationExpression                                    */
extern "C" {                                                                                                                              /*
extern•"C"•{↲    <ExternBlockDeclaration>
       "C"       Literal                                                                                                                  */
    pub fn f<'a>(x: &'a i32);                                                                                                             /*
    pub•fn•f<'a>(x:•&'a•i32);    FunctionDeclaration
    pub                          PubSpecifier
             'a                  GenericLtParameterDeclaration, LtIdentifier
                 x:•&'a•i32      FunctionParameterDeclaration
                    &'a•i32      TypeReference
                     'a          LtIdentifier                                                                                             */
    pub fn f<'b>(x: &'a i32, y: &'b i32);                                                                                                 /*
    pub•fn•f<'b>(x:•&'a•i32,•y:•&'b•i32);    FunctionDeclaration
    pub                                      PubSpecifier
             'b                              GenericLtParameterDeclaration, LtIdentifier
                 x:•&'a•i32                  FunctionParameterDeclaration
                    &'a•i32                  TypeReference
                     'a                      LtIdentifier
                             y:•&'b•i32      FunctionParameterDeclaration
                                &'b•i32      TypeReference
                                 'b          LtIdentifier                                                                                 */
    pub fn f<'a>(x: &'a i32, y: &i32) -> &'a i32;                                                                                         /*
    pub•fn•f<'a>(x:•&'a•i32,•y:•&i32)•->•&'a•i32;    FunctionDeclaration
    pub                                              PubSpecifier
             'a                                      GenericLtParameterDeclaration, LtIdentifier
                 x:•&'a•i32                          FunctionParameterDeclaration
                    &'a•i32                          TypeReference
                     'a                              LtIdentifier
                             y:•&i32                 FunctionParameterDeclaration
                                &i32                 TypeReference
                                         &'a•i32     TypeReference
                                          'a         LtIdentifier                                                                         */
    pub fn f<'b>(x: for<'c> fn(&'a i32));                                                                                                 /*
    pub•fn•f<'b>(x:•for<'c>•fn(&'a•i32));    FunctionDeclaration
    pub                                      PubSpecifier
             'b                              GenericLtParameterDeclaration, LtIdentifier
                 x:•for<'c>•fn(&'a•i32)      FunctionParameterDeclaration
                    for<'c>•fn(&'a•i32)      TypeFnPointer
                        'c                   GenericLtParameterDeclaration, LtIdentifier
                               &'a•i32       TypeFnPointerParameter, TypeReference
                                'a           LtIdentifier                                                                                 */
    pub fn f<'b>(x: for<'c> fn(&'b i32));                                                                                                 /*
    pub•fn•f<'b>(x:•for<'c>•fn(&'b•i32));    FunctionDeclaration
    pub                                      PubSpecifier
             'b                              GenericLtParameterDeclaration, LtIdentifier
                 x:•for<'c>•fn(&'b•i32)      FunctionParameterDeclaration
                    for<'c>•fn(&'b•i32)      TypeFnPointer
                        'c                   GenericLtParameterDeclaration, LtIdentifier
                               &'b•i32       TypeFnPointerParameter, TypeReference
                                'b           LtIdentifier                                                                                 */
    pub fn f<'b>(x: for<'c> fn(&'c i32));                                                                                                 /*
    pub•fn•f<'b>(x:•for<'c>•fn(&'c•i32));    FunctionDeclaration
    pub                                      PubSpecifier
             'b                              GenericLtParameterDeclaration, LtIdentifier
                 x:•for<'c>•fn(&'c•i32)      FunctionParameterDeclaration
                    for<'c>•fn(&'c•i32)      TypeFnPointer
                        'c                   GenericLtParameterDeclaration, LtIdentifier
                               &'c•i32       TypeFnPointerParameter, TypeReference
                                'c           LtIdentifier                                                                                 */
    pub fn f<'b>() -> for<'c> fn(&'a i32);                                                                                                /*
    pub•fn•f<'b>()•->•for<'c>•fn(&'a•i32);    FunctionDeclaration
    pub                                       PubSpecifier
             'b                               GenericLtParameterDeclaration, LtIdentifier
                      for<'c>•fn(&'a•i32)     TypeFnPointer
                          'c                  GenericLtParameterDeclaration, LtIdentifier
                                 &'a•i32      TypeFnPointerParameter, TypeReference
                                  'a          LtIdentifier                                                                                */
    pub fn f<'b>() -> for<'c> fn(&'b i32);                                                                                                /*
    pub•fn•f<'b>()•->•for<'c>•fn(&'b•i32);    FunctionDeclaration
    pub                                       PubSpecifier
             'b                               GenericLtParameterDeclaration, LtIdentifier
                      for<'c>•fn(&'b•i32)     TypeFnPointer
                          'c                  GenericLtParameterDeclaration, LtIdentifier
                                 &'b•i32      TypeFnPointerParameter, TypeReference
                                  'b          LtIdentifier                                                                                */
    pub fn f<'b>() -> for<'c> fn(&'c i32);                                                                                                /*
    pub•fn•f<'b>()•->•for<'c>•fn(&'c•i32);    FunctionDeclaration
    pub                                       PubSpecifier
             'b                               GenericLtParameterDeclaration, LtIdentifier
                      for<'c>•fn(&'c•i32)     TypeFnPointer
                          'c                  GenericLtParameterDeclaration, LtIdentifier
                                 &'c•i32      TypeFnPointerParameter, TypeReference
                                  'c          LtIdentifier                                                                                */
}                                                                                                                                         /*
}    </ExternBlockDeclaration>                                                                                                            */
struct X<'x, 'y> { x: std::marker::PhantomData<&'x ()>, y: std::marker::PhantomData<&'y ()>, }                                            /*
struct•X<'x,•'y>•{•x:•std::marker::PhantomData<&'x•()>,•y:•std::marker::PhantomData<&'y•()>,•}    StructDeclaration
         'x                                                                                       GenericLtParameterDeclaration, LtIdentifier
             'y                                                                                   GenericLtParameterDeclaration, LtIdentifier
                   x:•std::marker::PhantomData<&'x•()>                                            StructPropertyDeclaration
                      std::marker::PhantomData<&'x•()>                                            TypeCall
                      std::marker::PhantomData                                                    TypePath
                      std::marker                                                                 TypePath
                                               &'x•()                                             TypeReference
                                                'x                                                LtIdentifier
                                                   ()                                             TypeTuple
                                                        y:•std::marker::PhantomData<&'y•()>       StructPropertyDeclaration
                                                           std::marker::PhantomData<&'y•()>       TypeCall
                                                           std::marker::PhantomData               TypePath
                                                           std::marker                            TypePath
                                                                                    &'y•()        TypeReference
                                                                                     'y           LtIdentifier
                                                                                        ()        TypeTuple                               */
struct G<T> where for<'f> T: F<'f, As: E<'f>> + 'f,{ t: std::marker::PhantomData<T>,}                                                     /*
struct•G<T>•where•for<'f>•T:•F<'f,•As:•E<'f>>•+•'f,{•t:•std::marker::PhantomData<T>,}    StructDeclaration
         T                                                                               GenericTypeParameterDeclaration
                  for<'f>•T:•F<'f,•As:•E<'f>>•+•'f                                       WhereTypeBoundDeclaration
                      'f                                                                 GenericLtParameterDeclaration, LtIdentifier
                             F<'f,•As:•E<'f>>                                            TypeTraitBound, TypeCall
                               'f                                                        LtIdentifier
                                   As:•E<'f>                                             TypeCallNamedBound
                                       E<'f>                                             TypeTraitBound, TypeCall
                                         'f                                              LtIdentifier
                                                'f                                       LtIdentifier
                                                     t:•std::marker::PhantomData<T>      StructPropertyDeclaration
                                                        std::marker::PhantomData<T>      TypeCall
                                                        std::marker::PhantomData         TypePath
                                                        std::marker                      TypePath                                         */
struct D<T> where T: for<'c> C<'c, As: A<'c>>, { t: std::marker::PhantomData<T>, }                                                        /*
struct•D<T>•where•T:•for<'c>•C<'c,•As:•A<'c>>,•{•t:•std::marker::PhantomData<T>,•}    StructDeclaration
         T                                                                            GenericTypeParameterDeclaration
                  T:•for<'c>•C<'c,•As:•A<'c>>                                         WhereTypeBoundDeclaration
                     for<'c>•C<'c,•As:•A<'c>>                                         TypeTraitBound
                         'c                                                           GenericLtParameterDeclaration, LtIdentifier
                             C<'c,•As:•A<'c>>                                         TypeCall
                               'c                                                     LtIdentifier
                                   As:•A<'c>                                          TypeCallNamedBound
                                       A<'c>                                          TypeTraitBound, TypeCall
                                         'c                                           LtIdentifier
                                                 t:•std::marker::PhantomData<T>       StructPropertyDeclaration
                                                    std::marker::PhantomData<T>       TypeCall
                                                    std::marker::PhantomData          TypePath
                                                    std::marker                       TypePath                                            */
fn f<T>() where T: A, T::U: B, {}                                                                                                         /*
fn•f<T>()•where•T:•A,•T::U:•B,•{}    FunctionDeclaration
     T                               GenericTypeParameterDeclaration
                T:•A                 WhereTypeBoundDeclaration
                   A                 TypeTraitBound
                      T::U:•B        WhereTypeBoundDeclaration
                      T::U           TypePath
                            B        TypeTraitBound                                                                                       */
fn f(a: isize, b: *const *const u8) -> isize {}                                                                                           /*
fn•f(a:•isize,•b:•*const•*const•u8)•->•isize•{}    FunctionDeclaration
     a:•isize                                      FunctionParameterDeclaration
               b:•*const•*const•u8                 FunctionParameterDeclaration
                  *const•*const•u8                 TypeDereferenceConst
                         *const•u8                 TypeDereferenceConst                                                                   */
fn f<G:FnMut(A,A) -> A>(mut a: G, b: A, c: A) -> A {}                                                                                     /*
fn•f<G:FnMut(A,A)•->•A>(mut•a:•G,•b:•A,•c:•A)•->•A•{}    FunctionDeclaration
     G:FnMut(A,A)•->•A                                   GenericTypeParameterDeclaration
       FnMut(A,A)•->•A                                   TypeTraitBound, TypeFunction
                        mut•a:•G                         FunctionParameterDeclaration
                        mut•a                            PatternVariableDeclaration
                                  b:•A                   FunctionParameterDeclaration
                                        c:•A             FunctionParameterDeclaration                                                     */
fn f<T:A<B=T>+C>(x: T) -> T {}                                                                                                            /*
fn•f<T:A<B=T>+C>(x:•T)•->•T•{}    FunctionDeclaration
     T:A<B=T>+C                   GenericTypeParameterDeclaration
       A<B=T>                     TypeTraitBound, TypeCall
         B=T                      TypeCallNamedArgument
              C                   TypeTraitBound
                 x:•T             FunctionParameterDeclaration                                                                            */
fn f<A, B: a<A>>(x: B) -> C<A> {}                                                                                                         /*
fn•f<A,•B:•a<A>>(x:•B)•->•C<A>•{}    FunctionDeclaration
     A                               GenericTypeParameterDeclaration
        B:•a<A>                      GenericTypeParameterDeclaration
           a<A>                      TypeTraitBound, TypeCall
                 x:•B                FunctionParameterDeclaration
                          C<A>       TypeCall                                                                                             */
struct Whitespace<T: Clone + = ()> { t: T }                                                                                               /*
struct•Whitespace<T:•Clone•+•=•()>•{•t:•T•}    StructDeclaration
                  T:•Clone•+•=•()              GenericTypeParameterDeclaration
                     Clone                     TypeTraitBound
                               ()              TypeTuple
                                     t:•T      StructPropertyDeclaration                                                                  */
struct TokenSplit<T: Clone +=  ()> { t: T }                                                                                               /*
struct•TokenSplit<T:•Clone•+=••()>•{•t:•T•}    StructDeclaration
                  T:•Clone•+=••()              GenericTypeParameterDeclaration
                     Clone                     TypeTraitBound
                               ()              TypeTuple
                                     t:•T      StructPropertyDeclaration                                                                  */
fn f<'a, 'b, T>(t: T) -> isize where T: 'a, 'a: 'b, T: Eq { 0 }                                                                           /*
fn•f<'a,•'b,•T>(t:•T)•->•isize•where•T:•'a,•'a:•'b,•T:•Eq•{•0•}    FunctionDeclaration
     'a                                                            GenericLtParameterDeclaration, LtIdentifier
         'b                                                        GenericLtParameterDeclaration, LtIdentifier
             T                                                     GenericTypeParameterDeclaration
                t:•T                                               FunctionParameterDeclaration
                                     T:•'a                         WhereTypeBoundDeclaration
                                        'a                         LtIdentifier
                                            'a:•'b                 WhereLtBoundDeclaration
                                            'a                     LtIdentifier
                                                'b                 LtIdentifier
                                                    T:•Eq          WhereTypeBoundDeclaration
                                                       Eq          TypeTraitBound
                                                            0      ExpressionStatement, Literal                                           */
impl<T: ?Sized, U: ?Sized> A<B<U>> for C<T>  where T: D<U> {}                                                                             /*
impl<T:•?Sized,•U:•?Sized>•A<B<U>>•for•C<T>••where•T:•D<U>•{}    ImplDeclaration
     T:•?Sized                                                   GenericTypeParameterDeclaration
        ?Sized                                                   TypeTraitBound
                U:•?Sized                                        GenericTypeParameterDeclaration
                   ?Sized                                        TypeTraitBound
                           A<B<U>>                               TypeCall
                             B<U>                                TypeCall
                                       C<T>                      TypeCall
                                                   T:•D<U>       WhereTypeBoundDeclaration
                                                      D<U>       TypeTraitBound, TypeCall                                                 */
fn f()where T: for<'a> A<'a> + 'a,{}                                                                                                      /*
fn•f()where•T:•for<'a>•A<'a>•+•'a,{}    FunctionDeclaration
            T:•for<'a>•A<'a>•+•'a       WhereTypeBoundDeclaration
               for<'a>•A<'a>            TypeTraitBound
                   'a                   GenericLtParameterDeclaration, LtIdentifier
                       A<'a>            TypeCall
                         'a             LtIdentifier
                               'a       LtIdentifier                                                                                      */
fn f()where T: for<'g> H<'g, 'g, As: for<'h> H<'h, 'g> + 'g>,{}                                                                           /*
fn•f()where•T:•for<'g>•H<'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>,{}    FunctionDeclaration
            T:•for<'g>•H<'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>       WhereTypeBoundDeclaration
               for<'g>•H<'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>       TypeTraitBound
                   'g                                              GenericLtParameterDeclaration, LtIdentifier
                       H<'g,•'g,•As:•for<'h>•H<'h,•'g>•+•'g>       TypeCall
                         'g                                        LtIdentifier
                             'g                                    LtIdentifier
                                 As:•for<'h>•H<'h,•'g>•+•'g        TypeCallNamedBound
                                     for<'h>•H<'h,•'g>             TypeTraitBound
                                         'h                        GenericLtParameterDeclaration, LtIdentifier
                                             H<'h,•'g>             TypeCall
                                               'h                  LtIdentifier
                                                   'g              LtIdentifier
                                                         'g        LtIdentifier                                                           */
fn f()where T: for<'i> H<'i, 'i, As: for<'j> H<'j, 'i, As: for<'k> I<'i, 'k, 'j> + 'j> + 'i>,{}                                           /*
fn•f()where•T:•for<'i>•H<'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>,{}    FunctionDeclaration
            T:•for<'i>•H<'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>       WhereTypeBoundDeclaration
               for<'i>•H<'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>       TypeTraitBound
                   'i                                                                              GenericLtParameterDeclaration, LtIdentifier
                       H<'i,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i>       TypeCall
                         'i                                                                        LtIdentifier
                             'i                                                                    LtIdentifier
                                 As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>•+•'i        TypeCallNamedBound
                                     for<'j>•H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>             TypeTraitBound
                                         'j                                                        GenericLtParameterDeclaration, LtIdentifier
                                             H<'j,•'i,•As:•for<'k>•I<'i,•'k,•'j>•+•'j>             TypeCall
                                               'j                                                  LtIdentifier
                                                   'i                                              LtIdentifier
                                                       As:•for<'k>•I<'i,•'k,•'j>•+•'j              TypeCallNamedBound
                                                           for<'k>•I<'i,•'k,•'j>                   TypeTraitBound
                                                               'k                                  GenericLtParameterDeclaration, LtIdentifier
                                                                   I<'i,•'k,•'j>                   TypeCall
                                                                     'i                            LtIdentifier
                                                                         'k                        LtIdentifier
                                                                             'j                    LtIdentifier
                                                                                   'j              LtIdentifier
                                                                                         'i        LtIdentifier                           */
fn f()where T: for<'l, 'i> H<'l, 'i, As: for<'j> H<'j, 'i, As: for<'k> I<'l, 'k, 'j> + 'j> + 'i>,{}                                       /*
fn•f()where•T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>,{}    FunctionDeclaration
            T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>       WhereTypeBoundDeclaration
               for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>       TypeTraitBound
                   'l                                                                                  GenericLtParameterDeclaration, LtIdentifier
                       'i                                                                              GenericLtParameterDeclaration, LtIdentifier
                           H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i>       TypeCall
                             'l                                                                        LtIdentifier
                                 'i                                                                    LtIdentifier
                                     As:•for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>•+•'i        TypeCallNamedBound
                                         for<'j>•H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>             TypeTraitBound
                                             'j                                                        GenericLtParameterDeclaration, LtIdentifier
                                                 H<'j,•'i,•As:•for<'k>•I<'l,•'k,•'j>•+•'j>             TypeCall
                                                   'j                                                  LtIdentifier
                                                       'i                                              LtIdentifier
                                                           As:•for<'k>•I<'l,•'k,•'j>•+•'j              TypeCallNamedBound
                                                               for<'k>•I<'l,•'k,•'j>                   TypeTraitBound
                                                                   'k                                  GenericLtParameterDeclaration, LtIdentifier
                                                                       I<'l,•'k,•'j>                   TypeCall
                                                                         'l                            LtIdentifier
                                                                             'k                        LtIdentifier
                                                                                 'j                    LtIdentifier
                                                                                       'j              LtIdentifier
                                                                                             'i        LtIdentifier                       */
fn f()where T: for<'l, 'i> H<'l, 'i, As: for<'j> H<'j, 'i, As: for<'k> H<'j, 'k, As = X<'j, 'k>> + 'j> + 'i>{}                            /*
fn•f()where•T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>{}    FunctionDeclaration
            T:•for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>      WhereTypeBoundDeclaration
               for<'l,•'i>•H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>      TypeTraitBound
                   'l                                                                                             GenericLtParameterDeclaration, LtIdentifier
                       'i                                                                                         GenericLtParameterDeclaration, LtIdentifier
                           H<'l,•'i,•As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i>      TypeCall
                             'l                                                                                   LtIdentifier
                                 'i                                                                               LtIdentifier
                                     As:•for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>•+•'i       TypeCallNamedBound
                                         for<'j>•H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>            TypeTraitBound
                                             'j                                                                   GenericLtParameterDeclaration, LtIdentifier
                                                 H<'j,•'i,•As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j>            TypeCall
                                                   'j                                                             LtIdentifier
                                                       'i                                                         LtIdentifier
                                                           As:•for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>•+•'j             TypeCallNamedBound
                                                               for<'k>•H<'j,•'k,•As•=•X<'j,•'k>>                  TypeTraitBound
                                                                   'k                                             GenericLtParameterDeclaration, LtIdentifier
                                                                       H<'j,•'k,•As•=•X<'j,•'k>>                  TypeCall
                                                                         'j                                       LtIdentifier
                                                                             'k                                   LtIdentifier
                                                                                 As•=•X<'j,•'k>                   TypeCallNamedArgument
                                                                                      X<'j,•'k>                   TypeCall
                                                                                        'j                        LtIdentifier
                                                                                            'k                    LtIdentifier
                                                                                                   'j             LtIdentifier
                                                                                                         'i       LtIdentifier            */
fn f() where T:         Fn(&(),    &())                                    {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••&())••••••••••••••••••••••••••••••••••••{}    FunctionDeclaration
             T:•••••••••Fn(&(),••••&())                                          WhereTypeBoundDeclaration
                        Fn(&(),••••&())                                          TypeTraitBound, TypeFunction
                           &()                                                   TypeReference
                            ()                                                   TypeTuple
                                   &()                                           TypeReference
                                    ()                                           TypeTuple                                                */
fn f() where T:         Fn(&'a (), &())                                    {}                                                             /*
fn•f()•where•T:•••••••••Fn(&'a•(),•&())••••••••••••••••••••••••••••••••••••{}    FunctionDeclaration
             T:•••••••••Fn(&'a•(),•&())                                          WhereTypeBoundDeclaration
                        Fn(&'a•(),•&())                                          TypeTraitBound, TypeFunction
                           &'a•()                                                TypeReference
                            'a                                                   LtIdentifier
                               ()                                                TypeTuple
                                   &()                                           TypeReference
                                    ()                                           TypeTuple                                                */
fn f() where T:         Fn(&(),    Box<dyn Fn(&())>)                       {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>)•••••••••••••••••••••••{}    FunctionDeclaration
             T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>)                             WhereTypeBoundDeclaration
                        Fn(&(),••••Box<dyn•Fn(&())>)                             TypeTraitBound, TypeFunction
                           &()                                                   TypeReference
                            ()                                                   TypeTuple
                                   Box<dyn•Fn(&())>                              TypeCall
                                       dyn•Fn(&())                               TypeDynBounds
                                           Fn(&())                               TypeTraitBound, TypeFunction
                                              &()                                TypeReference
                                               ()                                TypeTuple                                                */
fn f() where T:         Fn(&(),    fn(&()))                                {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••fn(&()))••••••••••••••••••••••••••••••••{}    FunctionDeclaration
             T:•••••••••Fn(&(),••••fn(&()))                                      WhereTypeBoundDeclaration
                        Fn(&(),••••fn(&()))                                      TypeTraitBound, TypeFunction
                           &()                                                   TypeReference
                            ()                                                   TypeTuple
                                   fn(&())                                       TypeFnPointer
                                      &()                                        TypeFnPointerParameter, TypeReference
                                       ()                                        TypeTuple                                                */
fn f() where T:         Fn(&(),    for<'a> fn(&'a ()))                     {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••for<'a>•fn(&'a•()))•••••••••••••••••••••{}    FunctionDeclaration
             T:•••••••••Fn(&(),••••for<'a>•fn(&'a•()))                           WhereTypeBoundDeclaration
                        Fn(&(),••••for<'a>•fn(&'a•()))                           TypeTraitBound, TypeFunction
                           &()                                                   TypeReference
                            ()                                                   TypeTuple
                                   for<'a>•fn(&'a•())                            TypeFnPointer
                                       'a                                        GenericLtParameterDeclaration, LtIdentifier
                                              &'a•()                             TypeFnPointerParameter, TypeReference
                                               'a                                LtIdentifier
                                                  ()                             TypeTuple                                                */
fn f() where T:         Fn(&(),    Box<dyn Fn(&())>, &(), fn(&(), &()))    {}                                                             /*
fn•f()•where•T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>,•&(),•fn(&(),•&()))••••{}    FunctionDeclaration
             T:•••••••••Fn(&(),••••Box<dyn•Fn(&())>,•&(),•fn(&(),•&()))          WhereTypeBoundDeclaration
                        Fn(&(),••••Box<dyn•Fn(&())>,•&(),•fn(&(),•&()))          TypeTraitBound, TypeFunction
                           &()                                                   TypeReference
                            ()                                                   TypeTuple
                                   Box<dyn•Fn(&())>                              TypeCall
                                       dyn•Fn(&())                               TypeDynBounds
                                           Fn(&())                               TypeTraitBound, TypeFunction
                                              &()                                TypeReference
                                               ()                                TypeTuple
                                                     &()                         TypeReference
                                                      ()                         TypeTuple
                                                          fn(&(),•&())           TypeFnPointer
                                                             &()                 TypeFnPointerParameter, TypeReference
                                                              ()                 TypeTuple
                                                                  &()            TypeFnPointerParameter, TypeReference
                                                                   ()            TypeTuple                                                */
fn f() where T: for<'a> Fn(&'a (), &())                                    {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&'a•(),•&())••••••••••••••••••••••••••••••••••••{}    FunctionDeclaration
             T:•for<'a>•Fn(&'a•(),•&())                                          WhereTypeBoundDeclaration
                for<'a>•Fn(&'a•(),•&())                                          TypeTraitBound
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&'a•(),•&())                                          TypeFunction
                           &'a•()                                                TypeReference
                            'a                                                   LtIdentifier
                               ()                                                TypeTuple
                                   &()                                           TypeReference
                                    ()                                           TypeTuple                                                */
fn f() where T: for<'a> Fn(&(),    &'a ())                                 {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&(),••••&'a•())•••••••••••••••••••••••••••••••••{}    FunctionDeclaration
             T:•for<'a>•Fn(&(),••••&'a•())                                       WhereTypeBoundDeclaration
                for<'a>•Fn(&(),••••&'a•())                                       TypeTraitBound
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&(),••••&'a•())                                       TypeFunction
                           &()                                                   TypeReference
                            ()                                                   TypeTuple
                                   &'a•()                                        TypeReference
                                    'a                                           LtIdentifier
                                       ()                                        TypeTuple                                                */
fn f() where T: for<'a> Fn(&'a (), &'a ())                                 {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&'a•(),•&'a•())•••••••••••••••••••••••••••••••••{}    FunctionDeclaration
             T:•for<'a>•Fn(&'a•(),•&'a•())                                       WhereTypeBoundDeclaration
                for<'a>•Fn(&'a•(),•&'a•())                                       TypeTraitBound
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&'a•(),•&'a•())                                       TypeFunction
                           &'a•()                                                TypeReference
                            'a                                                   LtIdentifier
                               ()                                                TypeTuple
                                   &'a•()                                        TypeReference
                                    'a                                           LtIdentifier
                                       ()                                        TypeTuple                                                */
fn f() where T: for<'a> Fn(&'a (), Box<dyn Fn(&())>)                       {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&'a•(),•Box<dyn•Fn(&())>)•••••••••••••••••••••••{}    FunctionDeclaration
             T:•for<'a>•Fn(&'a•(),•Box<dyn•Fn(&())>)                             WhereTypeBoundDeclaration
                for<'a>•Fn(&'a•(),•Box<dyn•Fn(&())>)                             TypeTraitBound
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&'a•(),•Box<dyn•Fn(&())>)                             TypeFunction
                           &'a•()                                                TypeReference
                            'a                                                   LtIdentifier
                               ()                                                TypeTuple
                                   Box<dyn•Fn(&())>                              TypeCall
                                       dyn•Fn(&())                               TypeDynBounds
                                           Fn(&())                               TypeTraitBound, TypeFunction
                                              &()                                TypeReference
                                               ()                                TypeTuple                                                */
fn f() where T: for<'a> Fn(&(),    Box<dyn Fn(&())>, &'a (), fn(&(), &())) {}                                                             /*
fn•f()•where•T:•for<'a>•Fn(&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))•{}    FunctionDeclaration
             T:•for<'a>•Fn(&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))       WhereTypeBoundDeclaration
                for<'a>•Fn(&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))       TypeTraitBound
                    'a                                                           GenericLtParameterDeclaration, LtIdentifier
                        Fn(&(),••••Box<dyn•Fn(&())>,•&'a•(),•fn(&(),•&()))       TypeFunction
                           &()                                                   TypeReference
                            ()                                                   TypeTuple
                                   Box<dyn•Fn(&())>                              TypeCall
                                       dyn•Fn(&())                               TypeDynBounds
                                           Fn(&())                               TypeTraitBound, TypeFunction
                                              &()                                TypeReference
                                               ()                                TypeTuple
                                                     &'a•()                      TypeReference
                                                      'a                         LtIdentifier
                                                         ()                      TypeTuple
                                                             fn(&(),•&())        TypeFnPointer
                                                                &()              TypeFnPointerParameter, TypeReference
                                                                 ()              TypeTuple
                                                                     &()         TypeFnPointerParameter, TypeReference
                                                                      ()         TypeTuple                                                */
fn f() where T: A, F: FnOnce(B<T>) -> bool {}                                                                                             /*
fn•f()•where•T:•A,•F:•FnOnce(B<T>)•->•bool•{}    FunctionDeclaration
             T:•A                                WhereTypeBoundDeclaration
                A                                TypeTraitBound
                   F:•FnOnce(B<T>)•->•bool       WhereTypeBoundDeclaration
                      FnOnce(B<T>)•->•bool       TypeTraitBound, TypeFunction
                             B<T>                TypeCall                                                                                 */
fn f() -> impl std::borrow::Borrow<<u8 as A>::S> {}                                                                                       /*
fn•f()•->•impl•std::borrow::Borrow<<u8•as•A>::S>•{}    FunctionDeclaration
          impl•std::borrow::Borrow<<u8•as•A>::S>       TypeImplBounds
               std::borrow::Borrow<<u8•as•A>::S>       TypeTraitBound, TypeCall
               std::borrow::Borrow                     TypePath
               std::borrow                             TypePath
                                   <u8•as•A>::S        TypePath
                                   <u8•as•A>           ExpressionTypeSelector                                                             */
fn f(_: <() as A<Self::B>>::C) {}                                                                                                         /*
fn•f(_:•<()•as•A<Self::B>>::C)•{}    FunctionDeclaration
     _:•<()•as•A<Self::B>>::C        FunctionParameterDeclaration
     _                               WildcardPattern
        <()•as•A<Self::B>>::C        TypePath
        <()•as•A<Self::B>>           ExpressionTypeSelector
         ()                          TypeTuple
               A<Self::B>            TypeCall
                 Self::B             TypePath                                                                                             */
struct S<>;                                                                                                                               /*
struct•S<>;    StructDeclaration                                                                                                          */
trait T<> {}                                                                                                                              /*
trait•T<>•{}    TraitDeclaration                                                                                                          */
enum E<> { V }                                                                                                                            /*
enum•E<>•{•V•}    EnumDeclaration
           V      EnumMemberDeclaration                                                                                                   */
impl<> T<> for S<> {}                                                                                                                     /*
impl<>•T<>•for•S<>•{}    ImplDeclaration
       T<>               TypeCall
               S<>       TypeCall                                                                                                         */
fn f<'a>(x: for<'b, 'c: 'a+'b> fn(&'a A, &'b B) -> &'c C)                                                                                 /*
fn•f<'a>(x:•for<'b,•'c:•'a+'b>•fn(&'a•A,•&'b•B)•->•&'c•C)•↲    <FunctionDeclaration>
     'a                                                        GenericLtParameterDeclaration, LtIdentifier
         x:•for<'b,•'c:•'a+'b>•fn(&'a•A,•&'b•B)•->•&'c•C       FunctionParameterDeclaration
            for<'b,•'c:•'a+'b>•fn(&'a•A,•&'b•B)•->•&'c•C       TypeFnPointer
                'b                                             GenericLtParameterDeclaration, LtIdentifier
                    'c:•'a+'b                                  GenericLtParameterDeclaration
                    'c                                         LtIdentifier
                        'a                                     LtIdentifier
                           'b                                  LtIdentifier
                                  &'a•A                        TypeFnPointerParameter, TypeReference
                                   'a                          LtIdentifier
                                         &'b•B                 TypeFnPointerParameter, TypeReference
                                          'b                   LtIdentifier
                                                   &'c•C       TypeReference
                                                    'c         LtIdentifier                                                               */
   where F: for<'a, 'b: 'a>    Fn(&'a A, &'b B) -> &'c C,                                                                                 /*
         F:•for<'a,•'b:•'a>••••Fn(&'a•A,•&'b•B)•->•&'c•C     WhereTypeBoundDeclaration
            for<'a,•'b:•'a>••••Fn(&'a•A,•&'b•B)•->•&'c•C     TypeTraitBound
                'a                                           GenericLtParameterDeclaration, LtIdentifier
                    'b:•'a                                   GenericLtParameterDeclaration
                    'b                                       LtIdentifier
                        'a                                   LtIdentifier
                               Fn(&'a•A,•&'b•B)•->•&'c•C     TypeFunction
                                  &'a•A                      TypeReference
                                   'a                        LtIdentifier
                                         &'b•B               TypeReference
                                          'b                 LtIdentifier
                                                   &'c•C     TypeReference
                                                    'c       LtIdentifier                                                                 */
            for<'a, 'b: 'a> F: Fn(&'a A, &'b B) -> &'c C                                                                                  /*
            for<'a,•'b:•'a>•F:•Fn(&'a•A,•&'b•B)•->•&'c•C    WhereTypeBoundDeclaration
                'a                                          GenericLtParameterDeclaration, LtIdentifier
                    'b:•'a                                  GenericLtParameterDeclaration
                    'b                                      LtIdentifier
                        'a                                  LtIdentifier
                               Fn(&'a•A,•&'b•B)•->•&'c•C    TypeTraitBound, TypeFunction
                                  &'a•A                     TypeReference
                                   'a                       LtIdentifier
                                         &'b•B              TypeReference
                                          'b                LtIdentifier
                                                   &'c•C    TypeReference
                                                    'c      LtIdentifier                                                                  */
{}                                                                                                                                        /*
{}    </FunctionDeclaration>                                                                                                              */
struct S<F: for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C>(F);                                                                                /*
struct•S<F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C>(F);    TupleStructDeclaration
         F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C         GenericTypeParameterDeclaration
            for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C         TypeTraitBound
                'a                                            GenericLtParameterDeclaration, LtIdentifier
                    'b:•'a                                    GenericLtParameterDeclaration
                    'b                                        LtIdentifier
                        'a                                    LtIdentifier
                            Fn(&'a•A,•&'b•B)•->•&'c•C         TypeFunction
                               &'a•A                          TypeReference
                                'a                            LtIdentifier
                                      &'b•B                   TypeReference
                                       'b                     LtIdentifier
                                                &'c•C         TypeReference
                                                 'c           LtIdentifier
                                                       F      TupleStructItemDeclaration                                                  */
struct S<F>(F) where F: for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C;                                                                        /*
struct•S<F>(F)•where•F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C;    TupleStructDeclaration
         F                                                            GenericTypeParameterDeclaration
            F                                                         TupleStructItemDeclaration
                     F:•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C     WhereTypeBoundDeclaration
                        for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C     TypeTraitBound
                            'a                                        GenericLtParameterDeclaration, LtIdentifier
                                'b:•'a                                GenericLtParameterDeclaration
                                'b                                    LtIdentifier
                                    'a                                LtIdentifier
                                        Fn(&'a•A,•&'b•B)•->•&'c•C     TypeFunction
                                           &'a•A                      TypeReference
                                            'a                        LtIdentifier
                                                  &'b•B               TypeReference
                                                   'b                 LtIdentifier
                                                            &'c•C     TypeReference
                                                             'c       LtIdentifier                                                        */
struct S(for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C);                                                                                      /*
struct•S(for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C);    TupleStructDeclaration
         for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C      TupleStructItemDeclaration, TypeDynBounds
             'a                                         GenericLtParameterDeclaration, LtIdentifier
                 'b:•'a                                 GenericLtParameterDeclaration
                 'b                                     LtIdentifier
                     'a                                 LtIdentifier
                         Fn(&'a•A,•&'b•B)•->•&'c•C      TypeFunction
                            &'a•A                       TypeReference
                             'a                         LtIdentifier
                                   &'b•B                TypeReference
                                    'b                  LtIdentifier
                                             &'c•C      TypeReference
                                              'c        LtIdentifier                                                                      */
type T = Box<dyn for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C>;                                                                              /*
type•T•=•Box<dyn•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C>;    TypeAliasDeclaration
         Box<dyn•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C>     TypeCall
             dyn•for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C      TypeDynBounds
                 for<'a,•'b:•'a>•Fn(&'a•A,•&'b•B)•->•&'c•C      TypeTraitBound
                     'a                                         GenericLtParameterDeclaration, LtIdentifier
                         'b:•'a                                 GenericLtParameterDeclaration
                         'b                                     LtIdentifier
                             'a                                 LtIdentifier
                                 Fn(&'a•A,•&'b•B)•->•&'c•C      TypeFunction
                                    &'a•A                       TypeReference
                                     'a                         LtIdentifier
                                           &'b•B                TypeReference
                                            'b                  LtIdentifier
                                                     &'c•C      TypeReference
                                                      'c        LtIdentifier                                                              */
type L8<T> = L<L<L<L<L<L<L<L<T>>>>>>>>;                                                                                                   /*
type•L8<T>•=•L<L<L<L<L<L<L<L<T>>>>>>>>;    TypeAliasDeclaration
        T                                  GenericTypeParameterDeclaration
             L<L<L<L<L<L<L<L<T>>>>>>>>     TypeCall
               L<L<L<L<L<L<L<T>>>>>>>      TypeCall
                 L<L<L<L<L<L<T>>>>>>       TypeCall
                   L<L<L<L<L<T>>>>>        TypeCall
                     L<L<L<L<T>>>>         TypeCall
                       L<L<L<T>>>          TypeCall
                         L<L<T>>           TypeCall
                           L<T>            TypeCall                                                                                       */
type L64<T> = L8<L8<L8<L8<T>>>>;                                                                                                          /*
type•L64<T>•=•L8<L8<L8<L8<T>>>>;    TypeAliasDeclaration
         T                          GenericTypeParameterDeclaration
              L8<L8<L8<L8<T>>>>     TypeCall
                 L8<L8<L8<T>>>      TypeCall
                    L8<L8<T>>       TypeCall
                       L8<T>        TypeCall                                                                                              */
impl<T> A for T where T: B {                                                                                                              /*
impl<T>•A•for•T•where•T:•B•{↲    <ImplDeclaration>
     T                           GenericTypeParameterDeclaration
                      T:•B       WhereTypeBoundDeclaration
                         B       TypeTraitBound                                                                                           */
    type C<'a> = <T as D>::E<'a, 'static>;                                                                                                /*
    type•C<'a>•=•<T•as•D>::E<'a,•'static>;    TypeAliasDeclaration
           'a                                 GenericLtParameterDeclaration, LtIdentifier
                 <T•as•D>::E<'a,•'static>     TypeCall
                 <T•as•D>::E                  TypePath
                 <T•as•D>                     ExpressionTypeSelector
                             'a               LtIdentifier
                                 'static      LtStatic                                                                                    */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */
impl<T> A<<() as B<T::C>>::D> for E<T>                                                                                                    /*
impl<T>•A<<()•as•B<T::C>>::D>•for•E<T>↲    <ImplDeclaration>
     T                                     GenericTypeParameterDeclaration
        A<<()•as•B<T::C>>::D>              TypeCall
          <()•as•B<T::C>>::D               TypePath
          <()•as•B<T::C>>                  ExpressionTypeSelector
           ()                              TypeTuple
                 B<T::C>                   TypeCall
                   T::C                    TypePath
                                  E<T>     TypeCall                                                                                       */
where T: F, (): G<T::H>,                                                                                                                  /*
      T:•F                  WhereTypeBoundDeclaration
         F                  TypeTraitBound
            ():•G<T::H>     WhereTypeBoundDeclaration
            ()              TypeTuple
                G<T::H>     TypeTraitBound, TypeCall
                  T::H      TypePath                                                                                                      */
{}                                                                                                                                        /*
{}    </ImplDeclaration>                                                                                                                  */
type Y<'a> = &'a ();                                                                                                                      /*
type•Y<'a>•=•&'a•();    TypeAliasDeclaration
       'a               GenericLtParameterDeclaration, LtIdentifier
             &'a•()     TypeReference
              'a        LtIdentifier
                 ()     TypeTuple                                                                                                         */
type Q<'a>;                                                                                                                               /*
type•Q<'a>;    TypeAliasDeclaration
       'a      GenericLtParameterDeclaration, LtIdentifier                                                                                */
type Q<'a, 'b>;                                                                                                                           /*
type•Q<'a,•'b>;    TypeAliasDeclaration
       'a          GenericLtParameterDeclaration, LtIdentifier
           'b      GenericLtParameterDeclaration, LtIdentifier                                                                            */
type Q<'a, 'b,>;                                                                                                                          /*
type•Q<'a,•'b,>;    TypeAliasDeclaration
       'a           GenericLtParameterDeclaration, LtIdentifier
           'b       GenericLtParameterDeclaration, LtIdentifier                                                                           */
type Q<'a, 'b, T>;                                                                                                                        /*
type•Q<'a,•'b,•T>;    TypeAliasDeclaration
       'a             GenericLtParameterDeclaration, LtIdentifier
           'b         GenericLtParameterDeclaration, LtIdentifier
               T      GenericTypeParameterDeclaration                                                                                     */
type Q<'a, 'b, T, U>;                                                                                                                     /*
type•Q<'a,•'b,•T,•U>;    TypeAliasDeclaration
       'a                GenericLtParameterDeclaration, LtIdentifier
           'b            GenericLtParameterDeclaration, LtIdentifier
               T         GenericTypeParameterDeclaration
                  U      GenericTypeParameterDeclaration                                                                                  */
type Q<'a, 'b, T, U,>;                                                                                                                    /*
type•Q<'a,•'b,•T,•U,>;    TypeAliasDeclaration
       'a                 GenericLtParameterDeclaration, LtIdentifier
           'b             GenericLtParameterDeclaration, LtIdentifier
               T          GenericTypeParameterDeclaration
                  U       GenericTypeParameterDeclaration                                                                                 */
type Q<'a, 'b, T: S, U,>;                                                                                                                 /*
type•Q<'a,•'b,•T:•S,•U,>;    TypeAliasDeclaration
       'a                    GenericLtParameterDeclaration, LtIdentifier
           'b                GenericLtParameterDeclaration, LtIdentifier
               T:•S          GenericTypeParameterDeclaration
                  S          TypeTraitBound
                     U       GenericTypeParameterDeclaration                                                                              */
type Q<'a, 'b, T: S, U,>: S;                                                                                                              /*
type•Q<'a,•'b,•T:•S,•U,>:•S;    TypeAliasDeclaration
       'a                       GenericLtParameterDeclaration, LtIdentifier
           'b                   GenericLtParameterDeclaration, LtIdentifier
               T:•S             GenericTypeParameterDeclaration
                  S             TypeTraitBound
                     U          GenericTypeParameterDeclaration
                          S     TypeTraitBound                                                                                            */
type Q<'a, 'b, T: S, U,>: E<Target = T> + Into<U>;                                                                                        /*
type•Q<'a,•'b,•T:•S,•U,>:•E<Target•=•T>•+•Into<U>;    TypeAliasDeclaration
       'a                                             GenericLtParameterDeclaration, LtIdentifier
           'b                                         GenericLtParameterDeclaration, LtIdentifier
               T:•S                                   GenericTypeParameterDeclaration
                  S                                   TypeTraitBound
                     U                                GenericTypeParameterDeclaration
                          E<Target•=•T>               TypeTraitBound, TypeCall
                            Target•=•T                TypeCallNamedArgument
                                          Into<U>     TypeTraitBound, TypeCall                                                            */
type Q<'a, 'b, T: S, U,> where T: E<Target = U>, U: Into<T>;                                                                              /*
type•Q<'a,•'b,•T:•S,•U,>•where•T:•E<Target•=•U>,•U:•Into<T>;    TypeAliasDeclaration
       'a                                                       GenericLtParameterDeclaration, LtIdentifier
           'b                                                   GenericLtParameterDeclaration, LtIdentifier
               T:•S                                             GenericTypeParameterDeclaration
                  S                                             TypeTraitBound
                     U                                          GenericTypeParameterDeclaration
                               T:•E<Target•=•U>                 WhereTypeBoundDeclaration
                                  E<Target•=•U>                 TypeTraitBound, TypeCall
                                    Target•=•U                  TypeCallNamedArgument
                                                 U:•Into<T>     WhereTypeBoundDeclaration
                                                    Into<T>     TypeTraitBound, TypeCall                                                  */
type Q<'a, 'b, T: S, U,>: E<Target = T> + Into<U> where T: E<Target = U>, U: Into<T>;                                                     /*
type•Q<'a,•'b,•T:•S,•U,>:•E<Target•=•T>•+•Into<U>•where•T:•E<Target•=•U>,•U:•Into<T>;    TypeAliasDeclaration
       'a                                                                                GenericLtParameterDeclaration, LtIdentifier
           'b                                                                            GenericLtParameterDeclaration, LtIdentifier
               T:•S                                                                      GenericTypeParameterDeclaration
                  S                                                                      TypeTraitBound
                     U                                                                   GenericTypeParameterDeclaration
                          E<Target•=•T>                                                  TypeTraitBound, TypeCall
                            Target•=•T                                                   TypeCallNamedArgument
                                          Into<U>                                        TypeTraitBound, TypeCall
                                                        T:•E<Target•=•U>                 WhereTypeBoundDeclaration
                                                           E<Target•=•U>                 TypeTraitBound, TypeCall
                                                             Target•=•U                  TypeCallNamedArgument
                                                                          U:•Into<T>     WhereTypeBoundDeclaration
                                                                             Into<T>     TypeTraitBound, TypeCall                         */
type Q<'a>: E<Target = <Self::D<'a> as B>::A<'a, 'static>> where Self: 'a;                                                                /*
type•Q<'a>:•E<Target•=•<Self::D<'a>•as•B>::A<'a,•'static>>•where•Self:•'a;    TypeAliasDeclaration
       'a                                                                     GenericLtParameterDeclaration, LtIdentifier
            E<Target•=•<Self::D<'a>•as•B>::A<'a,•'static>>                    TypeTraitBound, TypeCall
              Target•=•<Self::D<'a>•as•B>::A<'a,•'static>                     TypeCallNamedArgument
                       <Self::D<'a>•as•B>::A<'a,•'static>                     TypeCall
                       <Self::D<'a>•as•B>::A                                  TypePath
                       <Self::D<'a>•as•B>                                     ExpressionTypeSelector
                        Self::D<'a>                                           TypeCall
                        Self::D                                               TypePath
                                'a                                            LtIdentifier
                                             'a                               LtIdentifier
                                                 'static                      LtStatic
                                                                 Self:•'a     WhereTypeBoundDeclaration
                                                                       'a     LtIdentifier                                                */
type S<'a>: Iterator<Q = Self::A<'a>> + E<R = Self::B<'b>>;                                                                               /*
type•S<'a>:•Iterator<Q•=•Self::A<'a>>•+•E<R•=•Self::B<'b>>;    TypeAliasDeclaration
       'a                                                      GenericLtParameterDeclaration, LtIdentifier
            Iterator<Q•=•Self::A<'a>>                          TypeTraitBound, TypeCall
                     Q•=•Self::A<'a>                           TypeCallNamedArgument
                         Self::A<'a>                           TypeCall
                         Self::A                               TypePath
                                 'a                            LtIdentifier
                                        E<R•=•Self::B<'b>>     TypeTraitBound, TypeCall
                                          R•=•Self::B<'b>      TypeCallNamedArgument
                                              Self::B<'b>      TypeCall
                                              Self::B          TypePath
                                                      'b       LtIdentifier                                                               */
type Z = dyn for<'x> Send;                                                                                                                /*
type•Z•=•dyn•for<'x>•Send;    TypeAliasDeclaration
         dyn•for<'x>•Send     TypeDynBounds
             for<'x>•Send     TypeTraitBound
                 'x           GenericLtParameterDeclaration, LtIdentifier                                                                 */
type A = (*const E::R, D);                                                                                                                /*
type•A•=•(*const•E::R,•D);    TypeAliasDeclaration
         (*const•E::R,•D)     TypeTuple
          *const•E::R         TypeDereferenceConst
                 E::R         TypePath                                                                                                    */
fn f(&self) -> Pin<Box<dyn Future<Output = Self::B> + '_>>;                                                                               /*
fn•f(&self)•->•Pin<Box<dyn•Future<Output•=•Self::B>•+•'_>>;    FunctionDeclaration
     &self                                                     FunctionSelfParameterDeclaration
               Pin<Box<dyn•Future<Output•=•Self::B>•+•'_>>     TypeCall
                   Box<dyn•Future<Output•=•Self::B>•+•'_>      TypeCall
                       dyn•Future<Output•=•Self::B>•+•'_       TypeDynBounds
                           Future<Output•=•Self::B>            TypeTraitBound, TypeCall
                                  Output•=•Self::B             TypeCallNamedArgument
                                           Self::B             TypePath
                                                      '_       LtElided                                                                   */
fn f(&self) -> Self::Y<'_>{}                                                                                                              /*
fn•f(&self)•->•Self::Y<'_>{}    FunctionDeclaration
     &self                      FunctionSelfParameterDeclaration
               Self::Y<'_>      TypeCall
               Self::Y          TypePath
                       '_       LtElided                                                                                                  */
fn f(x: &()) -> &() {}                                                                                                                    /*
fn•f(x:•&())•->•&()•{}    FunctionDeclaration
     x:•&()               FunctionParameterDeclaration
        &()               TypeReference
         ()               TypeTuple
                &()       TypeReference
                 ()       TypeTuple                                                                                                       */
fn f(x: &impl for<'a> X<Y<'a> = &'a ()>) -> &() {}                                                                                        /*
fn•f(x:•&impl•for<'a>•X<Y<'a>•=•&'a•()>)•->•&()•{}    FunctionDeclaration
     x:•&impl•for<'a>•X<Y<'a>•=•&'a•()>               FunctionParameterDeclaration
        &impl•for<'a>•X<Y<'a>•=•&'a•()>               TypeReference
         impl•for<'a>•X<Y<'a>•=•&'a•()>               TypeImplBounds
              for<'a>•X<Y<'a>•=•&'a•()>               TypeTraitBound
                  'a                                  GenericLtParameterDeclaration, LtIdentifier
                      X<Y<'a>•=•&'a•()>               TypeCall
                        Y<'a>•=•&'a•()                TypeCallNamedArgument
                        Y<'a>                         TypeCall
                          'a                          LtIdentifier
                                &'a•()                TypeReference
                                 'a                   LtIdentifier
                                    ()                TypeTuple
                                            &()       TypeReference
                                             ()       TypeTuple                                                                           */
fn f<'a, T: for<'b> Fun<F<'b> = T>>(t: T) -> T::F<'a>{}                                                                                   /*
fn•f<'a,•T:•for<'b>•Fun<F<'b>•=•T>>(t:•T)•->•T::F<'a>{}    FunctionDeclaration
     'a                                                    GenericLtParameterDeclaration, LtIdentifier
         T:•for<'b>•Fun<F<'b>•=•T>                         GenericTypeParameterDeclaration
            for<'b>•Fun<F<'b>•=•T>                         TypeTraitBound
                'b                                         GenericLtParameterDeclaration, LtIdentifier
                    Fun<F<'b>•=•T>                         TypeCall
                        F<'b>•=•T                          TypeCallNamedArgument
                        F<'b>                              TypeCall
                          'b                               LtIdentifier
                                    t:•T                   FunctionParameterDeclaration
                                             T::F<'a>      TypeCall
                                             T::F          TypePath
                                                  'a       LtIdentifier                                                                   */
fn f<'a, T: Fun<F<'a> = T>>(t: T) -> T::F<'a> {}                                                                                          /*
fn•f<'a,•T:•Fun<F<'a>•=•T>>(t:•T)•->•T::F<'a>•{}    FunctionDeclaration
     'a                                             GenericLtParameterDeclaration, LtIdentifier
         T:•Fun<F<'a>•=•T>                          GenericTypeParameterDeclaration
            Fun<F<'a>•=•T>                          TypeTraitBound, TypeCall
                F<'a>•=•T                           TypeCallNamedArgument
                F<'a>                               TypeCall
                  'a                                LtIdentifier
                            t:•T                    FunctionParameterDeclaration
                                     T::F<'a>       TypeCall
                                     T::F           TypePath
                                          'a        LtIdentifier                                                                          */
fn f<T: ?for<'a> Sized>() {}                                                                                                              /*
fn•f<T:•?for<'a>•Sized>()•{}    FunctionDeclaration
     T:•?for<'a>•Sized          GenericTypeParameterDeclaration
        ?for<'a>•Sized          TypeTraitBound
             'a                 GenericLtParameterDeclaration, LtIdentifier                                                               */
fn f<'a, T1: X<Y = T1>>(t : T1) -> T1::Y<'a>;                                                                                             /*
fn•f<'a,•T1:•X<Y•=•T1>>(t•:•T1)•->•T1::Y<'a>;    FunctionDeclaration
     'a                                          GenericLtParameterDeclaration, LtIdentifier
         T1:•X<Y•=•T1>                           GenericTypeParameterDeclaration
             X<Y•=•T1>                           TypeTraitBound, TypeCall
               Y•=•T1                            TypeCallNamedArgument
                        t•:•T1                   FunctionParameterDeclaration
                                   T1::Y<'a>     TypeCall
                                   T1::Y         TypePath
                                         'a      LtIdentifier                                                                             */
fn f<'a>(s: Box<dyn X<Y<'a>=&'a ()>>) {}                                                                                                  /*
fn•f<'a>(s:•Box<dyn•X<Y<'a>=&'a•()>>)•{}    FunctionDeclaration
     'a                                     GenericLtParameterDeclaration, LtIdentifier
         s:•Box<dyn•X<Y<'a>=&'a•()>>        FunctionParameterDeclaration
            Box<dyn•X<Y<'a>=&'a•()>>        TypeCall
                dyn•X<Y<'a>=&'a•()>         TypeDynBounds
                    X<Y<'a>=&'a•()>         TypeTraitBound, TypeCall
                      Y<'a>=&'a•()          TypeCallNamedArgument
                      Y<'a>                 TypeCall
                        'a                  LtIdentifier
                            &'a•()          TypeReference
                             'a             LtIdentifier
                                ()          TypeTuple                                                                                     */
fn f<'a>(t : Self::Y<'a>) -> Self::Y<'a>;                                                                                                 /*
fn•f<'a>(t•:•Self::Y<'a>)•->•Self::Y<'a>;    FunctionDeclaration
     'a                                      GenericLtParameterDeclaration, LtIdentifier
         t•:•Self::Y<'a>                     FunctionParameterDeclaration
             Self::Y<'a>                     TypeCall
             Self::Y                         TypePath
                     'a                      LtIdentifier
                             Self::Y<'a>     TypeCall
                             Self::Y         TypePath
                                     'a      LtIdentifier                                                                                 */
fn f<T: for<'a> X<Y<'a> = &'a ()>>(x: &T) -> &() {}                                                                                       /*
fn•f<T:•for<'a>•X<Y<'a>•=•&'a•()>>(x:•&T)•->•&()•{}    FunctionDeclaration
     T:•for<'a>•X<Y<'a>•=•&'a•()>                      GenericTypeParameterDeclaration
        for<'a>•X<Y<'a>•=•&'a•()>                      TypeTraitBound
            'a                                         GenericLtParameterDeclaration, LtIdentifier
                X<Y<'a>•=•&'a•()>                      TypeCall
                  Y<'a>•=•&'a•()                       TypeCallNamedArgument
                  Y<'a>                                TypeCall
                    'a                                 LtIdentifier
                          &'a•()                       TypeReference
                           'a                          LtIdentifier
                              ()                       TypeTuple
                                   x:•&T               FunctionParameterDeclaration
                                      &T               TypeReference
                                             &()       TypeReference
                                              ()       TypeTuple                                                                          */
fn f<'a, T: ?Sized + Fun<F<'a> = [u8]>>(_ : Box<T>) -> &'static T::F<'a> {}                                                               /*
fn•f<'a,•T:•?Sized•+•Fun<F<'a>•=•[u8]>>(_•:•Box<T>)•->•&'static•T::F<'a>•{}    FunctionDeclaration
     'a                                                                        GenericLtParameterDeclaration, LtIdentifier
         T:•?Sized•+•Fun<F<'a>•=•[u8]>                                         GenericTypeParameterDeclaration
            ?Sized                                                             TypeTraitBound
                     Fun<F<'a>•=•[u8]>                                         TypeTraitBound, TypeCall
                         F<'a>•=•[u8]                                          TypeCallNamedArgument
                         F<'a>                                                 TypeCall
                           'a                                                  LtIdentifier
                                 [u8]                                          TypeSlice
                                        _•:•Box<T>                             FunctionParameterDeclaration
                                        _                                      WildcardPattern
                                            Box<T>                             TypeCall
                                                       &'static•T::F<'a>       TypeReference
                                                        'static                LtStatic
                                                                T::F<'a>       TypeCall
                                                                T::F           TypePath
                                                                     'a        LtIdentifier                                               */
fn f<'a>(t: &'a Self::F<'a>) -> &'a Self::F<'a>{}                                                                                         /*
fn•f<'a>(t:•&'a•Self::F<'a>)•->•&'a•Self::F<'a>{}    FunctionDeclaration
     'a                                              GenericLtParameterDeclaration, LtIdentifier
         t:•&'a•Self::F<'a>                          FunctionParameterDeclaration
            &'a•Self::F<'a>                          TypeReference
             'a                                      LtIdentifier
                Self::F<'a>                          TypeCall
                Self::F                              TypePath
                        'a                           LtIdentifier
                                &'a•Self::F<'a>      TypeReference
                                 'a                  LtIdentifier
                                    Self::F<'a>      TypeCall
                                    Self::F          TypePath
                                            'a       LtIdentifier                                                                         */
fn f<T>() where T: S, for<'a> T::Item<'a>: Q {}                                                                                           /*
fn•f<T>()•where•T:•S,•for<'a>•T::Item<'a>:•Q•{}    FunctionDeclaration
     T                                             GenericTypeParameterDeclaration
                T:•S                               WhereTypeBoundDeclaration
                   S                               TypeTraitBound
                      for<'a>•T::Item<'a>:•Q       WhereTypeBoundDeclaration
                          'a                       GenericLtParameterDeclaration, LtIdentifier
                              T::Item<'a>          TypeCall
                              T::Item              TypePath
                                      'a           LtIdentifier
                                           Q       TypeTraitBound                                                                         */
fn f<'c, 'd>(s: Box<dyn X<Y = (&'c u32, &'d u32)>>) {}                                                                                    /*
fn•f<'c,•'d>(s:•Box<dyn•X<Y•=•(&'c•u32,•&'d•u32)>>)•{}    FunctionDeclaration
     'c                                                   GenericLtParameterDeclaration, LtIdentifier
         'd                                               GenericLtParameterDeclaration, LtIdentifier
             s:•Box<dyn•X<Y•=•(&'c•u32,•&'d•u32)>>        FunctionParameterDeclaration
                Box<dyn•X<Y•=•(&'c•u32,•&'d•u32)>>        TypeCall
                    dyn•X<Y•=•(&'c•u32,•&'d•u32)>         TypeDynBounds
                        X<Y•=•(&'c•u32,•&'d•u32)>         TypeTraitBound, TypeCall
                          Y•=•(&'c•u32,•&'d•u32)          TypeCallNamedArgument
                              (&'c•u32,•&'d•u32)          TypeTuple
                               &'c•u32                    TypeReference
                                'c                        LtIdentifier
                                        &'d•u32           TypeReference
                                         'd               LtIdentifier                                                                    */
fn f(e: &impl for<'a> X<Y<'a> = &'a ()>) -> &'static () {}                                                                                /*
fn•f(e:•&impl•for<'a>•X<Y<'a>•=•&'a•()>)•->•&'static•()•{}    FunctionDeclaration
     e:•&impl•for<'a>•X<Y<'a>•=•&'a•()>                       FunctionParameterDeclaration
        &impl•for<'a>•X<Y<'a>•=•&'a•()>                       TypeReference
         impl•for<'a>•X<Y<'a>•=•&'a•()>                       TypeImplBounds
              for<'a>•X<Y<'a>•=•&'a•()>                       TypeTraitBound
                  'a                                          GenericLtParameterDeclaration, LtIdentifier
                      X<Y<'a>•=•&'a•()>                       TypeCall
                        Y<'a>•=•&'a•()                        TypeCallNamedArgument
                        Y<'a>                                 TypeCall
                          'a                                  LtIdentifier
                                &'a•()                        TypeReference
                                 'a                           LtIdentifier
                                    ()                        TypeTuple
                                            &'static•()       TypeReference
                                             'static          LtStatic
                                                     ()       TypeTuple                                                                   */
fn f<T: for<'a> X<Y<'a> = &'a ()>>(x: &T) -> &'static () {}                                                                               /*
fn•f<T:•for<'a>•X<Y<'a>•=•&'a•()>>(x:•&T)•->•&'static•()•{}    FunctionDeclaration
     T:•for<'a>•X<Y<'a>•=•&'a•()>                              GenericTypeParameterDeclaration
        for<'a>•X<Y<'a>•=•&'a•()>                              TypeTraitBound
            'a                                                 GenericLtParameterDeclaration, LtIdentifier
                X<Y<'a>•=•&'a•()>                              TypeCall
                  Y<'a>•=•&'a•()                               TypeCallNamedArgument
                  Y<'a>                                        TypeCall
                    'a                                         LtIdentifier
                          &'a•()                               TypeReference
                           'a                                  LtIdentifier
                              ()                               TypeTuple
                                   x:•&T                       FunctionParameterDeclaration
                                      &T                       TypeReference
                                             &'static•()       TypeReference
                                              'static          LtStatic
                                                      ()       TypeTuple                                                                  */
fn f(x: &mut dyn for<'a> E<R<'a> = &'a i32>) -> usize {}                                                                                  /*
fn•f(x:•&mut•dyn•for<'a>•E<R<'a>•=•&'a•i32>)•->•usize•{}    FunctionDeclaration
     x:•&mut•dyn•for<'a>•E<R<'a>•=•&'a•i32>                 FunctionParameterDeclaration
        &mut•dyn•for<'a>•E<R<'a>•=•&'a•i32>                 TypeReference
             dyn•for<'a>•E<R<'a>•=•&'a•i32>                 TypeDynBounds
                 for<'a>•E<R<'a>•=•&'a•i32>                 TypeTraitBound
                     'a                                     GenericLtParameterDeclaration, LtIdentifier
                         E<R<'a>•=•&'a•i32>                 TypeCall
                           R<'a>•=•&'a•i32                  TypeCallNamedArgument
                           R<'a>                            TypeCall
                             'a                             LtIdentifier
                                   &'a•i32                  TypeReference
                                    'a                      LtIdentifier                                                                  */
fn f() where 'static: 'static, dyn 'static +: 'static + Copy,{}                                                                           /*
fn•f()•where•'static:•'static,•dyn•'static•+:•'static•+•Copy,{}    FunctionDeclaration
             'static:•'static                                      WhereLtBoundDeclaration
             'static                                               LtStatic
                      'static                                      LtStatic
                               dyn•'static•+:•'static•+•Copy       WhereTypeBoundDeclaration
                               dyn•'static•+                       TypeDynBounds
                                   'static                         LtStatic
                                              'static              LtStatic
                                                        Copy       TypeTraitBound                                                         */
fn f() where 'static: 'static, dyn 'static + ::Foo: 'static + Copy,{}                                                                     /*
fn•f()•where•'static:•'static,•dyn•'static•+•::Foo:•'static•+•Copy,{}    FunctionDeclaration
             'static:•'static                                            WhereLtBoundDeclaration
             'static                                                     LtStatic
                      'static                                            LtStatic
                               dyn•'static•+•::Foo:•'static•+•Copy       WhereTypeBoundDeclaration
                               dyn•'static•+•::Foo                       TypeDynBounds
                                   'static                               LtStatic
                                             ::Foo                       TypeTraitBound, TypePath
                                                    'static              LtStatic
                                                              Copy       TypeTraitBound                                                   */
fn f<F: A>() where F::B: Copy {}                                                                                                          /*
fn•f<F:•A>()•where•F::B:•Copy•{}    FunctionDeclaration
     F:•A                           GenericTypeParameterDeclaration
        A                           TypeTraitBound
                   F::B:•Copy       WhereTypeBoundDeclaration
                   F::B             TypePath
                         Copy       TypeTraitBound                                                                                        */
fn f<F: A>() where <F as A>::B: Copy {}                                                                                                   /*
fn•f<F:•A>()•where•<F•as•A>::B:•Copy•{}    FunctionDeclaration
     F:•A                                  GenericTypeParameterDeclaration
        A                                  TypeTraitBound
                   <F•as•A>::B:•Copy       WhereTypeBoundDeclaration
                   <F•as•A>::B             TypePath
                   <F•as•A>                ExpressionTypeSelector
                                Copy       TypeTraitBound                                                                                 */
fn f<F: A<B: A>>() where F::B: Copy {}                                                                                                    /*
fn•f<F:•A<B:•A>>()•where•F::B:•Copy•{}    FunctionDeclaration
     F:•A<B:•A>                           GenericTypeParameterDeclaration
        A<B:•A>                           TypeTraitBound, TypeCall
          B:•A                            TypeCallNamedBound
             A                            TypeTraitBound
                         F::B:•Copy       WhereTypeBoundDeclaration
                         F::B             TypePath
                               Copy       TypeTraitBound                                                                                  */
fn f<T: S<<Self as A>::Q>>(&self, r: &T) -> u64;                                                                                          /*
fn•f<T:•S<<Self•as•A>::Q>>(&self,•r:•&T)•->•u64;    FunctionDeclaration
     T:•S<<Self•as•A>::Q>                           GenericTypeParameterDeclaration
        S<<Self•as•A>::Q>                           TypeTraitBound, TypeCall
          <Self•as•A>::Q                            TypePath
          <Self•as•A>                               ExpressionTypeSelector
                           &self                    FunctionSelfParameterDeclaration
                                  r:•&T             FunctionParameterDeclaration
                                     &T             TypeReference                                                                         */
fn f() -> impl Default {}                                                                                                                 /*
fn•f()•->•impl•Default•{}    FunctionDeclaration
          impl•Default       TypeImplBounds
               Default       TypeTraitBound                                                                                               */
fn f(t: Box<dyn for<'a> Get<i32, i32>>) { }                                                                                               /*
fn•f(t:•Box<dyn•for<'a>•Get<i32,•i32>>)•{•}    FunctionDeclaration
     t:•Box<dyn•for<'a>•Get<i32,•i32>>         FunctionParameterDeclaration
        Box<dyn•for<'a>•Get<i32,•i32>>         TypeCall
            dyn•for<'a>•Get<i32,•i32>          TypeDynBounds
                for<'a>•Get<i32,•i32>          TypeTraitBound
                    'a                         GenericLtParameterDeclaration, LtIdentifier
                        Get<i32,•i32>          TypeCall                                                                                   */
fn f(t: Box<dyn for<'a> Fn(i32) -> i32>) { }                                                                                              /*
fn•f(t:•Box<dyn•for<'a>•Fn(i32)•->•i32>)•{•}    FunctionDeclaration
     t:•Box<dyn•for<'a>•Fn(i32)•->•i32>         FunctionParameterDeclaration
        Box<dyn•for<'a>•Fn(i32)•->•i32>         TypeCall
            dyn•for<'a>•Fn(i32)•->•i32          TypeDynBounds
                for<'a>•Fn(i32)•->•i32          TypeTraitBound
                    'a                          GenericLtParameterDeclaration, LtIdentifier
                        Fn(i32)•->•i32          TypeFunction                                                                              */
fn f(t: for<'a> fn(i32) -> i32) { }                                                                                                       /*
fn•f(t:•for<'a>•fn(i32)•->•i32)•{•}    FunctionDeclaration
     t:•for<'a>•fn(i32)•->•i32         FunctionParameterDeclaration
        for<'a>•fn(i32)•->•i32         TypeFnPointer
            'a                         GenericLtParameterDeclaration, LtIdentifier
                   i32                 TypeFnPointerParameter                                                                             */
fn f(t: for<'a> unsafe fn(i32) -> i32) { }                                                                                                /*
fn•f(t:•for<'a>•unsafe•fn(i32)•->•i32)•{•}    FunctionDeclaration
     t:•for<'a>•unsafe•fn(i32)•->•i32         FunctionParameterDeclaration
        for<'a>•unsafe•fn(i32)•->•i32         TypeFnPointer
            'a                                GenericLtParameterDeclaration, LtIdentifier
                          i32                 TypeFnPointerParameter                                                                      */
fn f(t: for<'a> extern "C" fn(i32) -> i32) { }                                                                                            /*
fn•f(t:•for<'a>•extern•"C"•fn(i32)•->•i32)•{•}    FunctionDeclaration
     t:•for<'a>•extern•"C"•fn(i32)•->•i32         FunctionParameterDeclaration
        for<'a>•extern•"C"•fn(i32)•->•i32         TypeFnPointer
            'a                                    GenericLtParameterDeclaration, LtIdentifier
                extern•"C"                        ExternSpecifier
                       "C"                        Literal
                              i32                 TypeFnPointerParameter                                                                  */
fn f(t: for<'a> unsafe extern "C" fn(i32) -> i32) { }                                                                                     /*
fn•f(t:•for<'a>•unsafe•extern•"C"•fn(i32)•->•i32)•{•}    FunctionDeclaration
     t:•for<'a>•unsafe•extern•"C"•fn(i32)•->•i32         FunctionParameterDeclaration
        for<'a>•unsafe•extern•"C"•fn(i32)•->•i32         TypeFnPointer
            'a                                           GenericLtParameterDeclaration, LtIdentifier
                       extern•"C"                        ExternSpecifier
                              "C"                        Literal
                                     i32                 TypeFnPointerParameter                                                           */
impl<T: Trait1, F: FnMut(<T as Trait1>::C)> Callback<T> for F {}                                                                          /*
impl<T:•Trait1,•F:•FnMut(<T•as•Trait1>::C)>•Callback<T>•for•F•{}    ImplDeclaration
     T:•Trait1                                                      GenericTypeParameterDeclaration
        Trait1                                                      TypeTraitBound
                F:•FnMut(<T•as•Trait1>::C)                          GenericTypeParameterDeclaration
                   FnMut(<T•as•Trait1>::C)                          TypeTraitBound, TypeFunction
                         <T•as•Trait1>::C                           TypePath
                         <T•as•Trait1>                              ExpressionTypeSelector
                                            Callback<T>             TypeCall                                                              */
impl Bar<N, M> for Foo<N, M> where A<{ N > 1 }>: B, A<{ M > 1 }>: B, {}                                                                   /*
impl•Bar<N,•M>•for•Foo<N,•M>•where•A<{•N•>•1•}>:•B,•A<{•M•>•1•}>:•B,•{}    ImplDeclaration
     Bar<N,•M>                                                             TypeCall
                   Foo<N,•M>                                               TypeCall
                                   A<{•N•>•1•}>:•B                         WhereTypeBoundDeclaration
                                   A<{•N•>•1•}>                            TypeCall
                                     {•N•>•1•}                             BlockExpression
                                       N•>•1                               ExpressionStatement, ComparisonExpression
                                           1                               Literal
                                                 B                         TypeTraitBound
                                                    A<{•M•>•1•}>:•B        WhereTypeBoundDeclaration
                                                    A<{•M•>•1•}>           TypeCall
                                                      {•M•>•1•}            BlockExpression
                                                        M•>•1              ExpressionStatement, ComparisonExpression
                                                            1              Literal
                                                                  B        TypeTraitBound                                                 */
async fn f( _: impl for<'a> Add<&'a u8>, _: impl for<'b> Add<&'b u8>, ) {}                                                                /*
async•fn•f(•_:•impl•for<'a>•Add<&'a•u8>,•_:•impl•for<'b>•Add<&'b•u8>,•)•{}    FunctionDeclaration
            _:•impl•for<'a>•Add<&'a•u8>                                       FunctionParameterDeclaration
            _                                                                 WildcardPattern
               impl•for<'a>•Add<&'a•u8>                                       TypeImplBounds
                    for<'a>•Add<&'a•u8>                                       TypeTraitBound
                        'a                                                    GenericLtParameterDeclaration, LtIdentifier
                            Add<&'a•u8>                                       TypeCall
                                &'a•u8                                        TypeReference
                                 'a                                           LtIdentifier
                                         _:•impl•for<'b>•Add<&'b•u8>          FunctionParameterDeclaration
                                         _                                    WildcardPattern
                                            impl•for<'b>•Add<&'b•u8>          TypeImplBounds
                                                 for<'b>•Add<&'b•u8>          TypeTraitBound
                                                     'b                       GenericLtParameterDeclaration, LtIdentifier
                                                         Add<&'b•u8>          TypeCall
                                                             &'b•u8           TypeReference
                                                              'b              LtIdentifier                                                */
async fn f<'a>(_: &'a ()) -> impl A<dyn B> {}                                                                                             /*
async•fn•f<'a>(_:•&'a•())•->•impl•A<dyn•B>•{}    FunctionDeclaration
           'a                                    GenericLtParameterDeclaration, LtIdentifier
               _:•&'a•()                         FunctionParameterDeclaration
               _                                 WildcardPattern
                  &'a•()                         TypeReference
                   'a                            LtIdentifier
                      ()                         TypeTuple
                             impl•A<dyn•B>       TypeImplBounds
                                  A<dyn•B>       TypeTraitBound, TypeCall
                                    dyn•B        TypeDynBounds
                                        B        TypeTraitBound                                                                           */
fn f<D: A>() where D::S: {}                                                                                                               /*
fn•f<D:•A>()•where•D::S:•{}    FunctionDeclaration
     D:•A                      GenericTypeParameterDeclaration
        A                      TypeTraitBound
                   D::S:       WhereTypeBoundDeclaration
                   D::S        TypePath                                                                                                   */
type T: Iterator<Item=<S as T>::T>;                                                                                                       /*
type•T:•Iterator<Item=<S•as•T>::T>;    TypeAliasDeclaration
        Iterator<Item=<S•as•T>::T>     TypeTraitBound, TypeCall
                 Item=<S•as•T>::T      TypeCallNamedArgument
                      <S•as•T>::T      TypePath
                      <S•as•T>         ExpressionTypeSelector                                                                             */
struct R<'a> { s: dyn for<'b> E<D<&'b ()>> + 'a, }                                                                                        /*
struct•R<'a>•{•s:•dyn•for<'b>•E<D<&'b•()>>•+•'a,•}    StructDeclaration
         'a                                           GenericLtParameterDeclaration, LtIdentifier
               s:•dyn•for<'b>•E<D<&'b•()>>•+•'a       StructPropertyDeclaration
                  dyn•for<'b>•E<D<&'b•()>>•+•'a       TypeDynBounds
                      for<'b>•E<D<&'b•()>>            TypeTraitBound
                          'b                          GenericLtParameterDeclaration, LtIdentifier
                              E<D<&'b•()>>            TypeCall
                                D<&'b•()>             TypeCall
                                  &'b•()              TypeReference
                                   'b                 LtIdentifier
                                      ()              TypeTuple
                                             'a       LtIdentifier                                                                        */
fn f() -> [u8; 4 * 1024 * 1024 * 1024 * 1024] {}                                                                                          /*
fn•f()•->•[u8;•4•*•1024•*•1024•*•1024•*•1024]•{}    FunctionDeclaration
          [u8;•4•*•1024•*•1024•*•1024•*•1024]       TypeSizedArray
               4•*•1024•*•1024•*•1024•*•1024        OperationExpression
               4•*•1024•*•1024•*•1024               OperationExpression
               4•*•1024•*•1024                      OperationExpression
               4•*•1024                             OperationExpression
               4                                    Literal
                   1024                             Literal
                          1024                      Literal
                                 1024               Literal
                                        1024        Literal                                                                               */
trait Foo where T: Borrow<U> + ?Sized, U: ?Sized + 'b, 'a: 'b, Box<T>:, { }                                                               /*
trait•Foo•where•T:•Borrow<U>•+•?Sized,•U:•?Sized•+•'b,•'a:•'b,•Box<T>:,•{•}    TraitDeclaration
                T:•Borrow<U>•+•?Sized                                          WhereTypeBoundDeclaration
                   Borrow<U>                                                   TypeTraitBound, TypeCall
                               ?Sized                                          TypeTraitBound
                                       U:•?Sized•+•'b                          WhereTypeBoundDeclaration
                                          ?Sized                               TypeTraitBound
                                                   'b                          LtIdentifier
                                                       'a:•'b                  WhereLtBoundDeclaration
                                                       'a                      LtIdentifier
                                                           'b                  LtIdentifier
                                                               Box<T>:         WhereTypeBoundDeclaration
                                                               Box<T>          TypeCall                                                   */
trait Map where for<'a> &'a Self: IntoIterator<Item = (&'a Self::Key, &'a Self::Value)>, {}                                               /*
trait•Map•where•for<'a>•&'a•Self:•IntoIterator<Item•=•(&'a•Self::Key,•&'a•Self::Value)>,•{}    TraitDeclaration
                for<'a>•&'a•Self:•IntoIterator<Item•=•(&'a•Self::Key,•&'a•Self::Value)>        WhereTypeBoundDeclaration
                    'a                                                                         GenericLtParameterDeclaration, LtIdentifier
                        &'a•Self                                                               TypeReference
                         'a                                                                    LtIdentifier
                                  IntoIterator<Item•=•(&'a•Self::Key,•&'a•Self::Value)>        TypeTraitBound, TypeCall
                                               Item•=•(&'a•Self::Key,•&'a•Self::Value)         TypeCallNamedArgument
                                                      (&'a•Self::Key,•&'a•Self::Value)         TypeTuple
                                                       &'a•Self::Key                           TypeReference
                                                        'a                                     LtIdentifier
                                                           Self::Key                           TypePath
                                                                      &'a•Self::Value          TypeReference
                                                                       'a                      LtIdentifier
                                                                          Self::Value          TypePath                                   */
trait S: A + AsRef<Self::B> {}                                                                                                            /*
trait•S:•A•+•AsRef<Self::B>•{}    TraitDeclaration
         A                        TypeTraitBound
             AsRef<Self::B>       TypeTraitBound, TypeCall
                   Self::B        TypePath                                                                                                */
struct Bar<const N: u8>([u8; (N + 2) as usize]) where [(); (N + 2) as usize]:;                                                            /*
struct•Bar<const•N:•u8>([u8;•(N•+•2)•as•usize])•where•[();•(N•+•2)•as•usize]:;    TupleStructDeclaration
           const•N:•u8                                                            ConstTypeParameterDeclaration
                        [u8;•(N•+•2)•as•usize]                                    TupleStructItemDeclaration, TypeSizedArray
                             (N•+•2)•as•usize                                     ExpressionAsTypeCast
                              N•+•2                                               OperationExpression
                                  2                                               Literal
                                                      [();•(N•+•2)•as•usize]:     WhereTypeBoundDeclaration
                                                      [();•(N•+•2)•as•usize]      TypeSizedArray
                                                       ()                         TypeTuple
                                                           (N•+•2)•as•usize       ExpressionAsTypeCast
                                                            N•+•2                 OperationExpression
                                                                2                 Literal                                                 */
fn f<const N: u8>() where D<{N as usize as u16 }>:{}                                                                                      /*
fn•f<const•N:•u8>()•where•D<{N•as•usize•as•u16•}>:{}    FunctionDeclaration
     const•N:•u8                                        ConstTypeParameterDeclaration
                          D<{N•as•usize•as•u16•}>:      WhereTypeBoundDeclaration
                          D<{N•as•usize•as•u16•}>       TypeCall
                            {N•as•usize•as•u16•}        BlockExpression
                             N•as•usize•as•u16          ExpressionStatement, ExpressionAsTypeCast
                             N•as•usize                 ExpressionAsTypeCast                                                              */
fn f<T>() where for<'a> T: TraitA<'a, AsA: for<'b> TraitB<'a, 'b, AsB: for<'c> TraitC<'a, 'b, 'c>>>, { }                                  /*
fn•f<T>()•where•for<'a>•T:•TraitA<'a,•AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>>,•{•}    FunctionDeclaration
     T                                                                                                      GenericTypeParameterDeclaration
                for<'a>•T:•TraitA<'a,•AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>>         WhereTypeBoundDeclaration
                    'a                                                                                      GenericLtParameterDeclaration, LtIdentifier
                           TraitA<'a,•AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>>         TypeTraitBound, TypeCall
                                  'a                                                                        LtIdentifier
                                      AsA:•for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>          TypeCallNamedBound
                                           for<'b>•TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>          TypeTraitBound
                                               'b                                                           GenericLtParameterDeclaration, LtIdentifier
                                                   TraitB<'a,•'b,•AsB:•for<'c>•TraitC<'a,•'b,•'c>>          TypeCall
                                                          'a                                                LtIdentifier
                                                              'b                                            LtIdentifier
                                                                  AsB:•for<'c>•TraitC<'a,•'b,•'c>           TypeCallNamedBound
                                                                       for<'c>•TraitC<'a,•'b,•'c>           TypeTraitBound
                                                                           'c                               GenericLtParameterDeclaration, LtIdentifier
                                                                               TraitC<'a,•'b,•'c>           TypeCall
                                                                                      'a                    LtIdentifier
                                                                                          'b                LtIdentifier
                                                                                              'c            LtIdentifier                  */
fn f<'u, 'a, F>() where for<'b> F: Iterator<Item: for<'c> B<'a, 'b, 'c> + for<'c> A<'a, 'c>>, { }                                         /*
fn•f<'u,•'a,•F>()•where•for<'b>•F:•Iterator<Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>>,•{•}    FunctionDeclaration
     'u                                                                                              GenericLtParameterDeclaration, LtIdentifier
         'a                                                                                          GenericLtParameterDeclaration, LtIdentifier
             F                                                                                       GenericTypeParameterDeclaration
                        for<'b>•F:•Iterator<Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>>         WhereTypeBoundDeclaration
                            'b                                                                       GenericLtParameterDeclaration, LtIdentifier
                                   Iterator<Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>>         TypeTraitBound, TypeCall
                                            Item:•for<'c>•B<'a,•'b,•'c>•+•for<'c>•A<'a,•'c>          TypeCallNamedBound
                                                  for<'c>•B<'a,•'b,•'c>                              TypeTraitBound
                                                      'c                                             GenericLtParameterDeclaration, LtIdentifier
                                                          B<'a,•'b,•'c>                              TypeCall
                                                            'a                                       LtIdentifier
                                                                'b                                   LtIdentifier
                                                                    'c                               LtIdentifier
                                                                          for<'c>•A<'a,•'c>          TypeTraitBound
                                                                              'c                     GenericLtParameterDeclaration, LtIdentifier
                                                                                  A<'a,•'c>          TypeCall
                                                                                    'a               LtIdentifier
                                                                                        'c           LtIdentifier                         */
fn f(&self, db: &<Q as QueryDb<'_>>::DynDb) {}                                                                                            /*
fn•f(&self,•db:•&<Q•as•QueryDb<'_>>::DynDb)•{}    FunctionDeclaration
     &self                                        FunctionSelfParameterDeclaration
            db:•&<Q•as•QueryDb<'_>>::DynDb        FunctionParameterDeclaration
                &<Q•as•QueryDb<'_>>::DynDb        TypeReference
                 <Q•as•QueryDb<'_>>::DynDb        TypePath
                 <Q•as•QueryDb<'_>>               ExpressionTypeSelector
                       QueryDb<'_>                TypeCall
                               '_                 LtElided                                                                                */
pub fn f<'a, I>() -> impl B<I, D = (), C = impl S + 'a> where I: A<E = &'a [()]>, {}                                                      /*
pub•fn•f<'a,•I>()•->•impl•B<I,•D•=•(),•C•=•impl•S•+•'a>•where•I:•A<E•=•&'a•[()]>,•{}    FunctionDeclaration
pub                                                                                     PubSpecifier
         'a                                                                             GenericLtParameterDeclaration, LtIdentifier
             I                                                                          GenericTypeParameterDeclaration
                     impl•B<I,•D•=•(),•C•=•impl•S•+•'a>                                 TypeImplBounds
                          B<I,•D•=•(),•C•=•impl•S•+•'a>                                 TypeTraitBound, TypeCall
                               D•=•()                                                   TypeCallNamedArgument
                                   ()                                                   TypeTuple
                                       C•=•impl•S•+•'a                                  TypeCallNamedArgument
                                           impl•S•+•'a                                  TypeImplBounds
                                                S                                       TypeTraitBound
                                                    'a                                  LtIdentifier
                                                              I:•A<E•=•&'a•[()]>        WhereTypeBoundDeclaration
                                                                 A<E•=•&'a•[()]>        TypeTraitBound, TypeCall
                                                                   E•=•&'a•[()]         TypeCallNamedArgument
                                                                       &'a•[()]         TypeReference
                                                                        'a              LtIdentifier
                                                                           [()]         TypeSlice
                                                                            ()          TypeTuple                                         */
type S<T: A<B: for<'a> C<&'a u8>>> = D;                                                                                                   /*
type•S<T:•A<B:•for<'a>•C<&'a•u8>>>•=•D;    TypeAliasDeclaration
       T:•A<B:•for<'a>•C<&'a•u8>>          GenericTypeParameterDeclaration
          A<B:•for<'a>•C<&'a•u8>>          TypeTraitBound, TypeCall
            B:•for<'a>•C<&'a•u8>           TypeCallNamedBound
               for<'a>•C<&'a•u8>           TypeTraitBound
                   'a                      GenericLtParameterDeclaration, LtIdentifier
                       C<&'a•u8>           TypeCall
                         &'a•u8            TypeReference
                          'a               LtIdentifier                                                                                   */
type S<T>;                                                                                                                                /*
type•S<T>;    TypeAliasDeclaration
       T      GenericTypeParameterDeclaration                                                                                             */
type A = a::b!();                                                                                                                         /*
type•A•=•a::b!();    TypeAliasDeclaration
         a::b!()     MacroInvocation
         a::b        TypePath                                                                                                             */
type S where Self: Sized;                                                                                                                 /*
type•S•where•Self:•Sized;    TypeAliasDeclaration
             Self:•Sized     WhereTypeBoundDeclaration
                   Sized     TypeTraitBound                                                                                               */
fn f(&self, a: &!){}                                                                                                                      /*
fn•f(&self,•a:•&!){}    FunctionDeclaration
     &self              FunctionSelfParameterDeclaration
            a:•&!       FunctionParameterDeclaration
               &!       TypeReference
                !       TypeNever                                                                                                         */
type S<T> where T: Display;                                                                                                               /*
type•S<T>•where•T:•Display;    TypeAliasDeclaration
       T                       GenericTypeParameterDeclaration
                T:•Display     WhereTypeBoundDeclaration
                   Display     TypeTraitBound                                                                                             */
type S<'a, T: Debug + 'a>: ?Sized = dyn Iterator<Item=T>;                                                                                 /*
type•S<'a,•T:•Debug•+•'a>:•?Sized•=•dyn•Iterator<Item=T>;    TypeAliasDeclaration
       'a                                                    GenericLtParameterDeclaration, LtIdentifier
           T:•Debug•+•'a                                     GenericTypeParameterDeclaration
              Debug                                          TypeTraitBound
                      'a                                     LtIdentifier
                           ?Sized                            TypeTraitBound
                                    dyn•Iterator<Item=T>     TypeDynBounds
                                        Iterator<Item=T>     TypeTraitBound, TypeCall
                                                 Item=T      TypeCallNamedArgument                                                        */
type S<'x> where T: 'x = (&'x ());                                                                                                        /*
type•S<'x>•where•T:•'x•=•(&'x•());    TypeAliasDeclaration
       'x                             GenericLtParameterDeclaration, LtIdentifier
                 T:•'x                WhereTypeBoundDeclaration
                    'x                LtIdentifier
                          &'x•()      TypeReference
                           'x         LtIdentifier
                              ()      TypeTuple                                                                                           */
type S<'u, 'v> where 'u: 'v = (&'v &'u ());                                                                                               /*
type•S<'u,•'v>•where•'u:•'v•=•(&'v•&'u•());    TypeAliasDeclaration
       'u                                      GenericLtParameterDeclaration, LtIdentifier
           'v                                  GenericLtParameterDeclaration, LtIdentifier
                     'u:•'v                    WhereLtBoundDeclaration
                     'u                        LtIdentifier
                         'v                    LtIdentifier
                               &'v•&'u•()      TypeReference
                                'v             LtIdentifier
                                   &'u•()      TypeReference
                                    'u         LtIdentifier
                                       ()      TypeTuple                                                                                  */
type S where Self: Q + S = E;                                                                                                             /*
type•S•where•Self:•Q•+•S•=•E;    TypeAliasDeclaration
             Self:•Q•+•S         WhereTypeBoundDeclaration
                   Q             TypeTraitBound
                       S         TypeTraitBound                                                                                           */
type S<'a: 'b, 'b> = (&'a(), &'b ());                                                                                                     /*
type•S<'a:•'b,•'b>•=•(&'a(),•&'b•());    TypeAliasDeclaration
       'a:•'b                            GenericLtParameterDeclaration
       'a                                LtIdentifier
           'b                            LtIdentifier
               'b                        GenericLtParameterDeclaration, LtIdentifier
                     (&'a(),•&'b•())     TypeTuple
                      &'a()              TypeReference
                       'a                LtIdentifier
                         ()              TypeTuple
                             &'b•()      TypeReference
                              'b         LtIdentifier
                                 ()      TypeTuple                                                                                        */
type S<'a> where Self: 'static = (&'a ());                                                                                                /*
type•S<'a>•where•Self:•'static•=•(&'a•());    TypeAliasDeclaration
       'a                                     GenericLtParameterDeclaration, LtIdentifier
                 Self:•'static                WhereTypeBoundDeclaration
                       'static                LtStatic
                                  &'a•()      TypeReference
                                   'a         LtIdentifier
                                      ()      TypeTuple                                                                                   */
type S<'a, 'b> where 'b: 'a = (&'a(), &'b ());                                                                                            /*
type•S<'a,•'b>•where•'b:•'a•=•(&'a(),•&'b•());    TypeAliasDeclaration
       'a                                         GenericLtParameterDeclaration, LtIdentifier
           'b                                     GenericLtParameterDeclaration, LtIdentifier
                     'b:•'a                       WhereLtBoundDeclaration
                     'b                           LtIdentifier
                         'a                       LtIdentifier
                              (&'a(),•&'b•())     TypeTuple
                               &'a()              TypeReference
                                'a                LtIdentifier
                                  ()              TypeTuple
                                      &'b•()      TypeReference
                                       'b         LtIdentifier
                                          ()      TypeTuple                                                                               */
type S<'a>: B<&'a [u8]>;                                                                                                                  /*
type•S<'a>:•B<&'a•[u8]>;    TypeAliasDeclaration
       'a                   GenericLtParameterDeclaration, LtIdentifier
            B<&'a•[u8]>     TypeTraitBound, TypeCall
              &'a•[u8]      TypeReference
               'a           LtIdentifier
                  [u8]      TypeSlice                                                                                                     */
type S<'a>: 'a;                                                                                                                           /*
type•S<'a>:•'a;    TypeAliasDeclaration
       'a          GenericLtParameterDeclaration, LtIdentifier
            'a     LtIdentifier                                                                                                           */
type S<'a: 'a>;                                                                                                                           /*
type•S<'a:•'a>;    TypeAliasDeclaration
       'a:•'a      GenericLtParameterDeclaration
       'a          LtIdentifier
           'a      LtIdentifier                                                                                                           */
type S<'a> = &'a ();                                                                                                                      /*
type•S<'a>•=•&'a•();    TypeAliasDeclaration
       'a               GenericLtParameterDeclaration, LtIdentifier
             &'a•()     TypeReference
              'a        LtIdentifier
                 ()     TypeTuple                                                                                                         */
type S<B>: S<A=B>;                                                                                                                        /*
type•S<B>:•S<A=B>;    TypeAliasDeclaration
       B              GenericTypeParameterDeclaration
           S<A=B>     TypeTraitBound, TypeCall
             A=B      TypeCallNamedArgument                                                                                               */
type S<'a, const N: usize>;                                                                                                               /*
type•S<'a,•const•N:•usize>;    TypeAliasDeclaration
       'a                      GenericLtParameterDeclaration, LtIdentifier
           const•N:•usize      ConstTypeParameterDeclaration                                                                              */
type S<'a> where <A as B>::T: 'a, <A as B>::T: 'a = R<&'a S::T, &'a E::T>;                                                                /*
type•S<'a>•where•<A•as•B>::T:•'a,•<A•as•B>::T:•'a•=•R<&'a•S::T,•&'a•E::T>;    TypeAliasDeclaration
       'a                                                                     GenericLtParameterDeclaration, LtIdentifier
                 <A•as•B>::T:•'a                                              WhereTypeBoundDeclaration
                 <A•as•B>::T                                                  TypePath
                 <A•as•B>                                                     ExpressionTypeSelector
                              'a                                              LtIdentifier
                                  <A•as•B>::T:•'a                             WhereTypeBoundDeclaration
                                  <A•as•B>::T                                 TypePath
                                  <A•as•B>                                    ExpressionTypeSelector
                                               'a                             LtIdentifier
                                                    R<&'a•S::T,•&'a•E::T>     TypeCall
                                                      &'a•S::T                TypeReference
                                                       'a                     LtIdentifier
                                                          S::T                TypePath
                                                                &'a•E::T      TypeReference
                                                                 'a           LtIdentifier
                                                                    E::T      TypePath                                                    */
type S<T> = Self::E<'static, T>;                                                                                                          /*
type•S<T>•=•Self::E<'static,•T>;    TypeAliasDeclaration
       T                            GenericTypeParameterDeclaration
            Self::E<'static,•T>     TypeCall
            Self::E                 TypePath
                    'static         LtStatic                                                                                              */
type S = Self::E<'static, 'static>;                                                                                                       /*
type•S•=•Self::E<'static,•'static>;    TypeAliasDeclaration
         Self::E<'static,•'static>     TypeCall
         Self::E                       TypePath
                 'static               LtStatic
                          'static      LtStatic                                                                                           */
impl<'b> ATy for &'b () {}                                                                                                                /*
impl<'b>•ATy•for•&'b•()•{}    ImplDeclaration
     'b                       GenericLtParameterDeclaration, LtIdentifier
                 &'b•()       TypeReference
                  'b          LtIdentifier
                     ()       TypeTuple                                                                                                   */
impl<T: Copy + std::ops::Deref> UnsafeCopy<T> for T {}                                                                                    /*
impl<T:•Copy•+•std::ops::Deref>•UnsafeCopy<T>•for•T•{}    ImplDeclaration
     T:•Copy•+•std::ops::Deref                            GenericTypeParameterDeclaration
        Copy                                              TypeTraitBound
               std::ops::Deref                            TypeTraitBound, TypePath
               std::ops                                   TypePath
                                UnsafeCopy<T>             TypeCall                                                                        */
impl<T: X<Y<i32> = i32>> M for T {}                                                                                                       /*
impl<T:•X<Y<i32>•=•i32>>•M•for•T•{}    ImplDeclaration
     T:•X<Y<i32>•=•i32>                GenericTypeParameterDeclaration
        X<Y<i32>•=•i32>                TypeTraitBound, TypeCall
          Y<i32>•=•i32                 TypeCallNamedArgument
          Y<i32>                       TypeCall                                                                                           */
type S: Sized where <Self as B>::C: Sized;                                                                                                /*
type•S:•Sized•where•<Self•as•B>::C:•Sized;    TypeAliasDeclaration
        Sized                                 TypeTraitBound
                    <Self•as•B>::C:•Sized     WhereTypeBoundDeclaration
                    <Self•as•B>::C            TypePath
                    <Self•as•B>               ExpressionTypeSelector
                                    Sized     TypeTraitBound                                                                              */
type S = Q<<T as R>::E>;                                                                                                                  /*
type•S•=•Q<<T•as•R>::E>;    TypeAliasDeclaration
         Q<<T•as•R>::E>     TypeCall
           <T•as•R>::E      TypePath
           <T•as•R>         ExpressionTypeSelector                                                                                        */
struct B<'a, T: for<'r> X<Y<'r> = &'r ()>> {f: <T as X>::Y<'a>}                                                                           /*
struct•B<'a,•T:•for<'r>•X<Y<'r>•=•&'r•()>>•{f:•<T•as•X>::Y<'a>}    StructDeclaration
         'a                                                        GenericLtParameterDeclaration, LtIdentifier
             T:•for<'r>•X<Y<'r>•=•&'r•()>                          GenericTypeParameterDeclaration
                for<'r>•X<Y<'r>•=•&'r•()>                          TypeTraitBound
                    'r                                             GenericLtParameterDeclaration, LtIdentifier
                        X<Y<'r>•=•&'r•()>                          TypeCall
                          Y<'r>•=•&'r•()                           TypeCallNamedArgument
                          Y<'r>                                    TypeCall
                            'r                                     LtIdentifier
                                  &'r•()                           TypeReference
                                   'r                              LtIdentifier
                                      ()                           TypeTuple
                                            f:•<T•as•X>::Y<'a>     StructPropertyDeclaration
                                               <T•as•X>::Y<'a>     TypeCall
                                               <T•as•X>::Y         TypePath
                                               <T•as•X>            ExpressionTypeSelector
                                                           'a      LtIdentifier                                                           */
enum E<'a> { S(<S as A>::B<'a>) }                                                                                                         /*
enum•E<'a>•{•S(<S•as•A>::B<'a>)•}    EnumDeclaration
       'a                            GenericLtParameterDeclaration, LtIdentifier
             S(<S•as•A>::B<'a>)      EnumMemberTupleDeclaration
               <S•as•A>::B<'a>       TupleStructItemDeclaration, TypeCall
               <S•as•A>::B           TypePath
               <S•as•A>              ExpressionTypeSelector
                           'a        LtIdentifier                                                                                         */
pub type T<P: Send + Send + Send> = P;                                                                                                    /*
pub•type•T<P:•Send•+•Send•+•Send>•=•P;    TypeAliasDeclaration
pub                                       PubSpecifier
           P:•Send•+•Send•+•Send          GenericTypeParameterDeclaration
              Send                        TypeTraitBound
                     Send                 TypeTraitBound
                            Send          TypeTraitBound                                                                                  */
type S<'b, 'a: 'b + 'b> = (&'b u32, Vec<&'a i32>);                                                                                        /*
type•S<'b,•'a:•'b•+•'b>•=•(&'b•u32,•Vec<&'a•i32>);    TypeAliasDeclaration
       'b                                             GenericLtParameterDeclaration, LtIdentifier
           'a:•'b•+•'b                                GenericLtParameterDeclaration
           'a                                         LtIdentifier
               'b                                     LtIdentifier
                    'b                                LtIdentifier
                          (&'b•u32,•Vec<&'a•i32>)     TypeTuple
                           &'b•u32                    TypeReference
                            'b                        LtIdentifier
                                    Vec<&'a•i32>      TypeCall
                                        &'a•i32       TypeReference
                                         'a           LtIdentifier                                                                        */
type S<'b, T: 'b + 'b> = (&'b u32, Vec<T>);                                                                                               /*
type•S<'b,•T:•'b•+•'b>•=•(&'b•u32,•Vec<T>);    TypeAliasDeclaration
       'b                                      GenericLtParameterDeclaration, LtIdentifier
           T:•'b•+•'b                          GenericTypeParameterDeclaration
              'b                               LtIdentifier
                   'b                          LtIdentifier
                         (&'b•u32,•Vec<T>)     TypeTuple
                          &'b•u32              TypeReference
                           'b                  LtIdentifier
                                   Vec<T>      TypeCall                                                                                   */
type S<'b, T> where T: 'b, T: 'b = (&'b u32, Vec<T>);                                                                                     /*
type•S<'b,•T>•where•T:•'b,•T:•'b•=•(&'b•u32,•Vec<T>);    TypeAliasDeclaration
       'b                                                GenericLtParameterDeclaration, LtIdentifier
           T                                             GenericTypeParameterDeclaration
                    T:•'b                                WhereTypeBoundDeclaration
                       'b                                LtIdentifier
                           T:•'b                         WhereTypeBoundDeclaration
                              'b                         LtIdentifier
                                   (&'b•u32,•Vec<T>)     TypeTuple
                                    &'b•u32              TypeReference
                                     'b                  LtIdentifier
                                             Vec<T>      TypeCall                                                                         */
type A = dyn S + ?Sized + ?Sized;                                                                                                         /*
type•A•=•dyn•S•+•?Sized•+•?Sized;    TypeAliasDeclaration
         dyn•S•+•?Sized•+•?Sized     TypeDynBounds
             S                       TypeTraitBound
                 ?Sized              TypeTraitBound
                          ?Sized     TypeTraitBound                                                                                       */
type R = dyn ?Sized + A;                                                                                                                  /*
type•R•=•dyn•?Sized•+•A;    TypeAliasDeclaration
         dyn•?Sized•+•A     TypeDynBounds
             ?Sized         TypeTraitBound
                      A     TypeTraitBound                                                                                                */
type Q = dyn for<'a> E<'a> + for<'b> R<'b>;                                                                                               /*
type•Q•=•dyn•for<'a>•E<'a>•+•for<'b>•R<'b>;    TypeAliasDeclaration
         dyn•for<'a>•E<'a>•+•for<'b>•R<'b>     TypeDynBounds
             for<'a>•E<'a>                     TypeTraitBound
                 'a                            GenericLtParameterDeclaration, LtIdentifier
                     E<'a>                     TypeCall
                       'a                      LtIdentifier
                             for<'b>•R<'b>     TypeTraitBound
                                 'b            GenericLtParameterDeclaration, LtIdentifier
                                     R<'b>     TypeCall
                                       'b      LtIdentifier                                                                               */
type S = dyn Q<for<'a> fn(&'a u8)> + G<for<'b> fn(&'b u8)>;                                                                               /*
type•S•=•dyn•Q<for<'a>•fn(&'a•u8)>•+•G<for<'b>•fn(&'b•u8)>;    TypeAliasDeclaration
         dyn•Q<for<'a>•fn(&'a•u8)>•+•G<for<'b>•fn(&'b•u8)>     TypeDynBounds
             Q<for<'a>•fn(&'a•u8)>                             TypeTraitBound, TypeCall
               for<'a>•fn(&'a•u8)                              TypeFnPointer
                   'a                                          GenericLtParameterDeclaration, LtIdentifier
                          &'a•u8                               TypeFnPointerParameter, TypeReference
                           'a                                  LtIdentifier
                                     G<for<'b>•fn(&'b•u8)>     TypeTraitBound, TypeCall
                                       for<'b>•fn(&'b•u8)      TypeFnPointer
                                           'b                  GenericLtParameterDeclaration, LtIdentifier
                                                  &'b•u8       TypeFnPointerParameter, TypeReference
                                                   'b          LtIdentifier                                                               */
type A = dyn ?Sized;                                                                                                                      /*
type•A•=•dyn•?Sized;    TypeAliasDeclaration
         dyn•?Sized     TypeDynBounds
             ?Sized     TypeTraitBound                                                                                                    */
type A = <S as Tr>::A::f<u8>;                                                                                                             /*
type•A•=•<S•as•Tr>::A::f<u8>;    TypeAliasDeclaration
         <S•as•Tr>::A::f<u8>     TypeCall
         <S•as•Tr>::A::f         TypePath
         <S•as•Tr>::A            TypePath
         <S•as•Tr>               ExpressionTypeSelector                                                                                   */
trait A: B<i32> + std::fmt::Debug + Send + Sync {}                                                                                        /*
trait•A:•B<i32>•+•std::fmt::Debug•+•Send•+•Sync•{}    TraitDeclaration
         B<i32>                                       TypeTraitBound, TypeCall
                  std::fmt::Debug                     TypeTraitBound, TypePath
                  std::fmt                            TypePath
                                    Send              TypeTraitBound
                                           Sync       TypeTraitBound                                                                      */
struct R<Z:?Sized = E<i32, i32>>(Z);                                                                                                      /*
struct•R<Z:?Sized•=•E<i32,•i32>>(Z);    TupleStructDeclaration
         Z:?Sized•=•E<i32,•i32>         GenericTypeParameterDeclaration
           ?Sized                       TypeTraitBound
                    E<i32,•i32>         TypeCall
                                 Z      TupleStructItemDeclaration                                                                        */
mod a {                                                                                                                                   /*
mod•a•{↲    <ModuleDeclaration>                                                                                                           */
    trait A {                                                                                                                             /*
    trait•A•{↲    <TraitDeclaration>                                                                                                      */
        const A: u8 = 0;                                                                                                                  /*
        const•A:•u8•=•0;    ConstVariableDeclaration
                      0     Literal                                                                                                       */
    }                                                                                                                                     /*
••••}    </TraitDeclaration>                                                                                                              */

    pub trait B {                                                                                                                         /*
    pub•trait•B•{↲    <TraitDeclaration>
    pub               PubSpecifier                                                                                                        */
        const B: u8 = 0;                                                                                                                  /*
        const•B:•u8•=•0;    ConstVariableDeclaration
                      0     Literal                                                                                                       */
    }                                                                                                                                     /*
••••}    </TraitDeclaration>                                                                                                              */

    pub trait C: A + B {                                                                                                                  /*
    pub•trait•C:•A•+•B•{↲    <TraitDeclaration>
    pub                      PubSpecifier
                 A           TypeTraitBound
                     B       TypeTraitBound                                                                                               */
        const C: u8 = 0;                                                                                                                  /*
        const•C:•u8•=•0;    ConstVariableDeclaration
                      0     Literal                                                                                                       */
    }                                                                                                                                     /*
••••}    </TraitDeclaration>                                                                                                              */

    impl A for ::S {}                                                                                                                     /*
    impl•A•for•::S•{}    ImplDeclaration
               ::S       TypePath                                                                                                         */
    impl B for ::S {}                                                                                                                     /*
    impl•B•for•::S•{}    ImplDeclaration
               ::S       TypePath                                                                                                         */
    impl C for ::S {}                                                                                                                     /*
    impl•C•for•::S•{}    ImplDeclaration
               ::S       TypePath                                                                                                         */
}                                                                                                                                         /*
}    </ModuleDeclaration>                                                                                                                 */

pub type b = Box<dyn t + sync::Send + sync::Sync + 'static>;                                                                              /*
pub•type•b•=•Box<dyn•t•+•sync::Send•+•sync::Sync•+•'static>;    TypeAliasDeclaration
pub                                                             PubSpecifier
             Box<dyn•t•+•sync::Send•+•sync::Sync•+•'static>     TypeCall
                 dyn•t•+•sync::Send•+•sync::Sync•+•'static      TypeDynBounds
                     t                                          TypeTraitBound
                         sync::Send                             TypeTraitBound, TypePath
                                      sync::Sync                TypeTraitBound, TypePath
                                                   'static      LtStatic                                                                  */
pub type b = Box<dyn for<'tcx> e<'tcx> + sync::Send + sync::Sync + 'static>;                                                              /*
pub•type•b•=•Box<dyn•for<'tcx>•e<'tcx>•+•sync::Send•+•sync::Sync•+•'static>;    TypeAliasDeclaration
pub                                                                             PubSpecifier
             Box<dyn•for<'tcx>•e<'tcx>•+•sync::Send•+•sync::Sync•+•'static>     TypeCall
                 dyn•for<'tcx>•e<'tcx>•+•sync::Send•+•sync::Sync•+•'static      TypeDynBounds
                     for<'tcx>•e<'tcx>                                          TypeTraitBound
                         'tcx                                                   GenericLtParameterDeclaration, LtIdentifier
                               e<'tcx>                                          TypeCall
                                 'tcx                                           LtIdentifier
                                         sync::Send                             TypeTraitBound, TypePath
                                                      sync::Sync                TypeTraitBound, TypePath
                                                                   'static      LtStatic                                                  */

// Discarded Nodes: 18
// Parsed Nodes: 4056
// state_rollbacks: 189
// Total '.charCodeAt()' calls: 19830 (49% re-reads)
// Unnecessary 'skip_whitespace()' calls: 3077
// source: "../../samples/types/types.rs"