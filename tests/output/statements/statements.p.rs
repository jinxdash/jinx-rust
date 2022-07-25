const _: () = {                                                                                                                           /*
const•_:•()•=•{↲    <ConstVariableDeclaration>
      _             WildcardPattern
         ()         TypeTuple
              {↲    <BlockExpression>                                                                                                     */
    pub trait A {                                                                                                                         /*
    pub•trait•A•{↲    <TraitDeclaration>
    pub               PubSpecifier                                                                                                        */
        const _: () = ();                                                                                                                 /*
        const•_:•()•=•();    ConstVariableDeclaration
              _              WildcardPattern
                 ()          TypeTuple
                      ()     TupleLiteral                                                                                                 */
    }                                                                                                                                     /*
••••}    </TraitDeclaration>                                                                                                              */
    impl A for () {                                                                                                                       /*
    impl•A•for•()•{↲    <ImplDeclaration>
               ()       TypeTuple                                                                                                         */
        const _: () = ();                                                                                                                 /*
        const•_:•()•=•();    ConstVariableDeclaration
              _              WildcardPattern
                 ()          TypeTuple
                      ()     TupleLiteral                                                                                                 */
    }                                                                                                                                     /*
••••}    </ImplDeclaration>                                                                                                               */
    impl dyn A {                                                                                                                          /*
    impl•dyn•A•{↲    <ImplDeclaration>
         dyn•A       TypeDynBounds
             A       TypeTraitBound                                                                                                       */
        const _: () = ();                                                                                                                 /*
        const•_:•()•=•();    ConstVariableDeclaration
              _              WildcardPattern
                 ()          TypeTuple
                      ()     TupleLiteral                                                                                                 */
    }                                                                                                                                     /*
••••}    </ImplDeclaration>                                                                                                               */
};                                                                                                                                        /*
};    </ConstVariableDeclaration>
}     </BlockExpression>                                                                                                                  */


extern r#"C"# { fn bar(); }                                                                                                               /*
extern•r#"C"#•{•fn•bar();•}    ExternBlockDeclaration
       r#"C"#                  Literal
                fn•bar();      FunctionDeclaration                                                                                        */
extern r#"C"# fn foo() {}                                                                                                                 /*
extern•r#"C"#•fn•foo()•{}    FunctionDeclaration
extern•r#"C"#                ExternSpecifier
       r#"C"#                Literal                                                                                                      */
type T = extern r#"C"# fn();                                                                                                              /*
type•T•=•extern•r#"C"#•fn();    TypeAliasDeclaration
         extern•r#"C"#•fn()     TypeFnPointer
         extern•r#"C"#          ExternSpecifier
                r#"C"#          Literal                                                                                                   */
extern "\x43" fn foo() {}                                                                                                                 /*
extern•"\x43"•fn•foo()•{}    FunctionDeclaration
extern•"\x43"                ExternSpecifier
       "\x43"                Literal                                                                                                      */
extern "\x43" { fn bar(); }                                                                                                               /*
extern•"\x43"•{•fn•bar();•}    ExternBlockDeclaration
       "\x43"                  Literal
                fn•bar();      FunctionDeclaration                                                                                        */
type T = extern "\x43" fn();                                                                                                              /*
type•T•=•extern•"\x43"•fn();    TypeAliasDeclaration
         extern•"\x43"•fn()     TypeFnPointer
         extern•"\x43"          ExternSpecifier
                "\x43"          Literal                                                                                                   */

// extern crate async;
//•extern•crate•async;    Comment
// extern crate async as something_else;
//•extern•crate•async•as•something_else;    Comment

fn f1();                                                                                                                                  /*
fn•f1();    FunctionDeclaration                                                                                                           */
fn f2() {};                                                                                                                               /*
fn•f2()•{}     FunctionDeclaration
          ;    ExpressionStatement                                                                                                        */
fn f3();                                                                                                                                  /*
fn•f3();    FunctionDeclaration                                                                                                           */

trait X {                                                                                                                                 /*
trait•X•{↲    <TraitDeclaration>                                                                                                          */
	fn f();                                                                                                                               /*
    fn•f();    FunctionDeclaration                                                                                                        */
	fn f() {}                                                                                                                             /*
    fn•f()•{}    FunctionDeclaration                                                                                                      */
	const Y: u8;                                                                                                                          /*
    const•Y:•u8;    ConstVariableDeclaration                                                                                              */
}                                                                                                                                         /*
}    </TraitDeclaration>                                                                                                                  */


extern "C" {                                                                                                                              /*
extern•"C"•{↲    <ExternBlockDeclaration>
       "C"       Literal                                                                                                                  */
	fn f();                                                                                                                               /*
    fn•f();    FunctionDeclaration                                                                                                        */
	fn f();                                                                                                                               /*
    fn•f();    FunctionDeclaration                                                                                                        */
    static X: u8;                                                                                                                         /*
    static•X:•u8;    StaticVariableDeclaration                                                                                            */
    static mut Y: u8;                                                                                                                     /*
    static•mut•Y:•u8;    StaticVariableDeclaration
           mut•Y         PatternVariableDeclaration                                                                                       */
	// type E: where;
    //•type•E:•where;    Comment
	type A: Ord;                                                                                                                          /*
    type•A:•Ord;    TypeAliasDeclaration
            Ord     TypeTraitBound                                                                                                        */
    type A<'a> where 'a: 'static;                                                                                                         /*
    type•A<'a>•where•'a:•'static;    TypeAliasDeclaration
           'a                        GenericLtParameterDeclaration, LtIdentifier
                     'a:•'static     WhereLtBoundDeclaration
                     'a              LtIdentifier
                         'static     LtStatic                                                                                             */
    type A<T: Ord> where T: 'static;                                                                                                      /*
    type•A<T:•Ord>•where•T:•'static;    TypeAliasDeclaration
           T:•Ord                       GenericTypeParameterDeclaration
              Ord                       TypeTraitBound
                         T:•'static     WhereTypeBoundDeclaration
                            'static     LtStatic                                                                                          */
    type A = u8;                                                                                                                          /*
    type•A•=•u8;    TypeAliasDeclaration                                                                                                  */
    type A<'a: 'static, T: Ord + 'static>: Eq + PartialEq where T: 'static + Copy = Vec<u8>;                                              /*
    type•A<'a:•'static,•T:•Ord•+•'static>:•Eq•+•PartialEq•where•T:•'static•+•Copy•=•Vec<u8>;    TypeAliasDeclaration
           'a:•'static                                                                          GenericLtParameterDeclaration
           'a                                                                                   LtIdentifier
               'static                                                                          LtStatic
                        T:•Ord•+•'static                                                        GenericTypeParameterDeclaration
                           Ord                                                                  TypeTraitBound
                                 'static                                                        LtStatic
                                           Eq                                                   TypeTraitBound
                                                PartialEq                                       TypeTraitBound
                                                                T:•'static•+•Copy               WhereTypeBoundDeclaration
                                                                   'static                      LtStatic
                                                                             Copy               TypeTraitBound
                                                                                    Vec<u8>     TypeCall                                  */
}                                                                                                                                         /*
}    </ExternBlockDeclaration>                                                                                                            */
const async const fn test() {}                                                                                                            /*
const•async•const•fn•test()•{}    FunctionDeclaration                                                                                     */
unsafe async fn test() {}                                                                                                                 /*
unsafe•async•fn•test()•{}    FunctionDeclaration                                                                                          */
unsafe const fn test() {}                                                                                                                 /*
unsafe•const•fn•test()•{}    FunctionDeclaration                                                                                          */
extern unsafe fn test() {}                                                                                                                /*
extern•unsafe•fn•test()•{}    FunctionDeclaration
extern                        ExternSpecifier                                                                                             */
fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>                                                                                                        */
    async fn f();                                                                                                                         /*
    async•fn•f();    FunctionDeclaration                                                                                                  */
    unsafe fn f();                                                                                                                        /*
    unsafe•fn•f();    FunctionDeclaration                                                                                                 */
    const fn f();                                                                                                                         /*
    const•fn•f();    FunctionDeclaration                                                                                                  */
    extern "C" fn f();                                                                                                                    /*
    extern•"C"•fn•f();    FunctionDeclaration
    extern•"C"            ExternSpecifier
           "C"            Literal                                                                                                         */
    const async unsafe extern "C" fn f();                                                                                                 /*
    const•async•unsafe•extern•"C"•fn•f();    FunctionDeclaration
                       extern•"C"            ExternSpecifier
                              "C"            Literal                                                                                      */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
const fn f(a: *const i32, b: i32) -> bool {}                                                                                              /*
const•fn•f(a:•*const•i32,•b:•i32)•->•bool•{}    FunctionDeclaration
           a:•*const•i32                        FunctionParameterDeclaration
              *const•i32                        TypeDereferenceConst
                          b:•i32                FunctionParameterDeclaration                                                              */
unsafe fn f(&self) -> u32;                                                                                                                /*
unsafe•fn•f(&self)•->•u32;    FunctionDeclaration
            &self             FunctionSelfParameterDeclaration                                                                            */
const unsafe fn f(v: u32) -> u32 {}                                                                                                       /*
const•unsafe•fn•f(v:•u32)•->•u32•{}    FunctionDeclaration
                  v:•u32               FunctionParameterDeclaration                                                                       */
unsafe fn f(func: unsafe fn() -> ()) -> () {}                                                                                             /*
unsafe•fn•f(func:•unsafe•fn()•->•())•->•()•{}    FunctionDeclaration
            func:•unsafe•fn()•->•()              FunctionParameterDeclaration
                  unsafe•fn()•->•()              TypeFnPointer
                                 ()              TypeTuple
                                        ()       TypeTuple                                                                                */
struct Range<const FROM: usize = 0, const LEN: usize = 0, const TO: usize = FROM>;                                                        /*
struct•Range<const•FROM:•usize•=•0,•const•LEN:•usize•=•0,•const•TO:•usize•=•FROM>;    StructDeclaration
             const•FROM:•usize•=•0                                                    ConstTypeParameterDeclaration
                                 0                                                    Literal
                                    const•LEN:•usize•=•0                              ConstTypeParameterDeclaration
                                                       0                              Literal
                                                          const•TO:•usize•=•FROM      ConstTypeParameterDeclaration                       */
impl<T> From<[u8; 1 + 1]> for Foo<T, [u8; 1 + 1]> {}                                                                                      /*
impl<T>•From<[u8;•1•+•1]>•for•Foo<T,•[u8;•1•+•1]>•{}    ImplDeclaration
     T                                                  GenericTypeParameterDeclaration
        From<[u8;•1•+•1]>                               TypeCall
             [u8;•1•+•1]                                TypeSizedArray
                  1•+•1                                 OperationExpression
                  1                                     Literal
                      1                                 Literal
                              Foo<T,•[u8;•1•+•1]>       TypeCall
                                     [u8;•1•+•1]        TypeSizedArray
                                          1•+•1         OperationExpression
                                          1             Literal
                                              1         Literal                                                                           */
fn f(d: [u8; 1 + 1]) -> A<T, [u8; 1 + 1]>                                                                                                 /*
fn•f(d:•[u8;•1•+•1])•->•A<T,•[u8;•1•+•1]>•↲    <FunctionDeclaration>
     d:•[u8;•1•+•1]                            FunctionParameterDeclaration
        [u8;•1•+•1]                            TypeSizedArray
             1•+•1                             OperationExpression
             1                                 Literal
                 1                             Literal
                        A<T,•[u8;•1•+•1]>      TypeCall
                             [u8;•1•+•1]       TypeSizedArray
                                  1•+•1        OperationExpression
                                  1            Literal
                                      1        Literal                                                                                    */
where [u8; 1 + 1]: From<[u8; 1 + 1]> {}                                                                                                   /*
where•[u8;•1•+•1]:•From<[u8;•1•+•1]>•{}    </FunctionDeclaration>
      [u8;•1•+•1]:•From<[u8;•1•+•1]>       WhereTypeBoundDeclaration
      [u8;•1•+•1]                          TypeSizedArray
           1•+•1                           OperationExpression
           1                               Literal
               1                           Literal
                   From<[u8;•1•+•1]>       TypeTraitBound, TypeCall
                        [u8;•1•+•1]        TypeSizedArray
                             1•+•1         OperationExpression
                             1             Literal
                                 1         Literal                                                                                        */
fn f<'a, 'b, 'c, T>(x: foo::X<'a, T, 'b, 'c>) {}                                                                                          /*
fn•f<'a,•'b,•'c,•T>(x:•foo::X<'a,•T,•'b,•'c>)•{}    FunctionDeclaration
     'a                                             GenericLtParameterDeclaration, LtIdentifier
         'b                                         GenericLtParameterDeclaration, LtIdentifier
             'c                                     GenericLtParameterDeclaration, LtIdentifier
                 T                                  GenericTypeParameterDeclaration
                    x:•foo::X<'a,•T,•'b,•'c>        FunctionParameterDeclaration
                       foo::X<'a,•T,•'b,•'c>        TypeCall
                       foo::X                       TypePath
                              'a                    LtIdentifier
                                     'b             LtIdentifier
                                         'c         LtIdentifier                                                                          */
fn f() -> Option<fn() -> Option<bool>> { Some(|| Some(true)) }                                                                            /*
fn•f()•->•Option<fn()•->•Option<bool>>•{•Some(||•Some(true))•}    FunctionDeclaration
          Option<fn()•->•Option<bool>>                            TypeCall
                 fn()•->•Option<bool>                             TypeFnPointer
                         Option<bool>                             TypeCall
                                         Some(||•Some(true))      ExpressionStatement, CallExpression
                                              ||•Some(true)       ClosureFunctionExpression
                                                 Some(true)       CallExpression
                                                      true        Literal                                                                 */
fn a() {                                                                                                                                  /*
fn•a()•{↲    <FunctionDeclaration>                                                                                                        */
	let a = 0;                                                                                                                            /*
    let•a•=•0;    LetVariableDeclaration
            0     Literal                                                                                                                 */
    let _b = 0;                                                                                                                           /*
    let•_b•=•0;    LetVariableDeclaration
             0     Literal                                                                                                                */
    let _ = 0;                                                                                                                            /*
    let•_•=•0;    LetVariableDeclaration
        _         WildcardPattern
            0     Literal                                                                                                                 */
    let mut b = 0;                                                                                                                        /*
    let•mut•b•=•0;    LetVariableDeclaration
        mut•b         PatternVariableDeclaration
                0     Literal                                                                                                             */
    let mut _b = 0;                                                                                                                       /*
    let•mut•_b•=•0;    LetVariableDeclaration
        mut•_b         PatternVariableDeclaration
                 0     Literal                                                                                                            */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

enum Test3 {                                                                                                                              /*
enum•Test3•{↲    <EnumDeclaration>                                                                                                        */
	Var1,                                                                                                                                 /*
    Var1    EnumMemberDeclaration                                                                                                         */
	Var2(String),                                                                                                                         /*
    Var2(String)    EnumMemberTupleDeclaration
         String     TupleStructItemDeclaration                                                                                            */
	StillFine {                                                                                                                           /*
    StillFine•{↲    <EnumMemberStructDeclaration>                                                                                         */
		def: i32,                                                                                                                         /*
        def:•i32    StructPropertyDeclaration                                                                                             */
	},                                                                                                                                    /*
   ╚}    </EnumMemberStructDeclaration>                                                                                                   */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */
enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>                                                                                                            */
    UnitVariant,                                                                                                                          /*
    UnitVariant     EnumMemberDeclaration                                                                                                 */
    TupleVariant(),                                                                                                                       /*
    TupleVariant()     EnumMemberTupleDeclaration                                                                                         */
    BracedVariant{},                                                                                                                      /*
    BracedVariant{}     EnumMemberStructDeclaration                                                                                       */
	T(T, [!; 0]),                                                                                                                         /*
    T(T,•[!;•0])    EnumMemberTupleDeclaration
      T             TupleStructItemDeclaration
         [!;•0]     TupleStructItemDeclaration, TypeSizedArray
          !         TypeNever
             0      Literal                                                                                                               */
    #[allow(dead_code)]                                                                                                                   /*
    #[allow(dead_code)]↲    <EnumMemberTupleDeclaration>
    #[allow(dead_code)]     Attribute
           (dead_code)      DelimGroup                                                                                                    */
    U(U),                                                                                                                                 /*
••••U(U)     </EnumMemberTupleDeclaration>
      U      TupleStructItemDeclaration                                                                                                   */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */
fn foobar<F, G>() -> usize                                                                                                                /*
fn•foobar<F,•G>()•->•usize↲    <FunctionDeclaration>
          F                    GenericTypeParameterDeclaration
             G                 GenericTypeParameterDeclaration                                                                            */
where
    (): Foobar<F, G>,{}                                                                                                                   /*
••••():•Foobar<F,•G>,{}    </FunctionDeclaration>
    ():•Foobar<F,•G>       WhereTypeBoundDeclaration
    ()                     TypeTuple
        Foobar<F,•G>       TypeTraitBound, TypeCall                                                                                       */

mod a {                                                                                                                                   /*
mod•a•{↲    <ModuleDeclaration>                                                                                                           */
	extern "C" {                                                                                                                          /*
    extern•"C"•{↲    <ExternBlockDeclaration>
           "C"       Literal                                                                                                              */
		pub fn free(x: *const u8);                                                                                                        /*
        pub•fn•free(x:•*const•u8);    FunctionDeclaration
        pub                           PubSpecifier
                    x:•*const•u8      FunctionParameterDeclaration
                       *const•u8      TypeDereferenceConst                                                                                */
	}                                                                                                                                     /*
   ╚}    </ExternBlockDeclaration>                                                                                                        */
}                                                                                                                                         /*
}    </ModuleDeclaration>                                                                                                                 */
pub union U {                                                                                                                             /*
pub•union•U•{↲    <UnionDeclaration>
pub               PubSpecifier                                                                                                            */
	pub a: u8,                                                                                                                            /*
    pub•a:•u8    StructPropertyDeclaration
    pub          PubSpecifier                                                                                                             */
	pub(super) b: u8,                                                                                                                     /*
    pub(super)•b:•u8    StructPropertyDeclaration
    pub(super)          PubSpecifier                                                                                                      */
	c: u8,                                                                                                                                /*
    c:•u8    StructPropertyDeclaration                                                                                                    */
}                                                                                                                                         /*
}    </UnionDeclaration>                                                                                                                  */


trait C<A> {                                                                                                                              /*
trait•C<A>•{↲    <TraitDeclaration>
        A        GenericTypeParameterDeclaration                                                                                          */
    fn D<B, F>(&self, f: F) where F: FnMut(A) -> Q<B>;                                                                                    /*
    fn•D<B,•F>(&self,•f:•F)•where•F:•FnMut(A)•->•Q<B>;    FunctionDeclaration
         B                                                GenericTypeParameterDeclaration
            F                                             GenericTypeParameterDeclaration
               &self                                      FunctionSelfParameterDeclaration
                      f:•F                                FunctionParameterDeclaration
                                  F:•FnMut(A)•->•Q<B>     WhereTypeBoundDeclaration
                                     FnMut(A)•->•Q<B>     TypeTraitBound, TypeFunction
                                                 Q<B>     TypeCall                                                                        */
}                                                                                                                                         /*
}    </TraitDeclaration>                                                                                                                  */
trait A<T: B=Self> {}                                                                                                                     /*
trait•A<T:•B=Self>•{}    TraitDeclaration
        T:•B=Self        GenericTypeParameterDeclaration
           B             TypeTraitBound                                                                                                   */
trait A: B::C {}                                                                                                                          /*
trait•A:•B::C•{}    TraitDeclaration
         B::C       TypeTraitBound, TypePath                                                                                              */
trait A<T>: B::C {}                                                                                                                       /*
trait•A<T>:•B::C•{}    TraitDeclaration
        T              GenericTypeParameterDeclaration
            B::C       TypeTraitBound, TypePath                                                                                           */
fn f() -> () { }                                                                                                                          /*
fn•f()•->•()•{•}    FunctionDeclaration
          ()        TypeTuple                                                                                                             */
fn f<T: X<Y<()> = i32>>() {}                                                                                                              /*
fn•f<T:•X<Y<()>•=•i32>>()•{}    FunctionDeclaration
     T:•X<Y<()>•=•i32>          GenericTypeParameterDeclaration
        X<Y<()>•=•i32>          TypeTraitBound, TypeCall
          Y<()>•=•i32           TypeCallNamedArgument
          Y<()>                 TypeCall
            ()                  TypeTuple                                                                                                 */
fn f<F>(mut f: F) where F: FnMut(&mut R, bool) {}                                                                                         /*
fn•f<F>(mut•f:•F)•where•F:•FnMut(&mut•R,•bool)•{}    FunctionDeclaration
     F                                               GenericTypeParameterDeclaration
        mut•f:•F                                     FunctionParameterDeclaration
        mut•f                                        PatternVariableDeclaration
                        F:•FnMut(&mut•R,•bool)       WhereTypeBoundDeclaration
                           FnMut(&mut•R,•bool)       TypeTraitBound, TypeFunction
                                 &mut•R              TypeReference                                                                        */
fn f<F>(f: F) where F: for<'a> Fn(&'a isize, &'a isize) -> isize { }                                                                      /*
fn•f<F>(f:•F)•where•F:•for<'a>•Fn(&'a•isize,•&'a•isize)•->•isize•{•}    FunctionDeclaration
     F                                                                  GenericTypeParameterDeclaration
        f:•F                                                            FunctionParameterDeclaration
                    F:•for<'a>•Fn(&'a•isize,•&'a•isize)•->•isize        WhereTypeBoundDeclaration
                       for<'a>•Fn(&'a•isize,•&'a•isize)•->•isize        TypeTraitBound
                           'a                                           GenericLtParameterDeclaration, LtIdentifier
                               Fn(&'a•isize,•&'a•isize)•->•isize        TypeFunction
                                  &'a•isize                             TypeReference
                                   'a                                   LtIdentifier
                                             &'a•isize                  TypeReference
                                              'a                        LtIdentifier                                                      */
fn f<F>(f: F) -> isize where F: Fn() -> isize { f() }                                                                                     /*
fn•f<F>(f:•F)•->•isize•where•F:•Fn()•->•isize•{•f()•}    FunctionDeclaration
     F                                                   GenericTypeParameterDeclaration
        f:•F                                             FunctionParameterDeclaration
                             F:•Fn()•->•isize            WhereTypeBoundDeclaration
                                Fn()•->•isize            TypeTraitBound, TypeFunction
                                                f()      ExpressionStatement, CallExpression                                              */
async fn g(((ref a, ref mut b), (ref mut c, ref d)): ((A, A), (A, A))) {}                                                                 /*
async•fn•g(((ref•a,•ref•mut•b),•(ref•mut•c,•ref•d)):•((A,•A),•(A,•A)))•{}    FunctionDeclaration
           ((ref•a,•ref•mut•b),•(ref•mut•c,•ref•d)):•((A,•A),•(A,•A))        FunctionParameterDeclaration
           ((ref•a,•ref•mut•b),•(ref•mut•c,•ref•d))                          TuplePattern
            (ref•a,•ref•mut•b)                                               TuplePattern
             ref•a                                                           PatternVariableDeclaration
                    ref•mut•b                                                PatternVariableDeclaration
                                (ref•mut•c,•ref•d)                           TuplePattern
                                 ref•mut•c                                   PatternVariableDeclaration
                                            ref•d                            PatternVariableDeclaration
                                                     ((A,•A),•(A,•A))        TypeTuple
                                                      (A,•A)                 TypeTuple
                                                              (A,•A)         TypeTuple                                                    */
pub unsafe extern "C" fn bar(_: i32, mut ap: ...) -> usize {}                                                                             /*
pub•unsafe•extern•"C"•fn•bar(_:•i32,•mut•ap:•...)•->•usize•{}    FunctionDeclaration
pub                                                              PubSpecifier
           extern•"C"                                            ExternSpecifier
                  "C"                                            Literal
                             _:•i32                              FunctionParameterDeclaration
                             _                                   WildcardPattern
                                     mut•ap:•...                 FunctionParameterDeclaration
                                     mut•ap                      PatternVariableDeclaration
                                             ...                 FunctionSpread                                                           */
unsafe fn f(&self, x: &usize) { *self + *x; }                                                                                             /*
unsafe•fn•f(&self,•x:•&usize)•{•*self•+•*x;•}    FunctionDeclaration
            &self                                FunctionSelfParameterDeclaration
                   x:•&usize                     FunctionParameterDeclaration
                      &usize                     TypeReference
                                *self•+•*x;      ExpressionStatement
                                *self•+•*x       OperationExpression
                                *self            DereferenceExpression
                                        *x       DereferenceExpression                                                                    */
fn f<B:?Sized+Q>() { }                                                                                                                    /*
fn•f<B:?Sized+Q>()•{•}    FunctionDeclaration
     B:?Sized+Q           GenericTypeParameterDeclaration
       ?Sized             TypeTraitBound
              Q           TypeTraitBound                                                                                                  */
fn f<A,F:for<'a> F<(&'a A,)>>(_: F) { }                                                                                                   /*
fn•f<A,F:for<'a>•F<(&'a•A,)>>(_:•F)•{•}    FunctionDeclaration
     A                                     GenericTypeParameterDeclaration
       F:for<'a>•F<(&'a•A,)>               GenericTypeParameterDeclaration
         for<'a>•F<(&'a•A,)>               TypeTraitBound
             'a                            GenericLtParameterDeclaration, LtIdentifier
                 F<(&'a•A,)>               TypeCall
                   (&'a•A,)                TypeTuple
                    &'a•A                  TypeReference
                     'a                    LtIdentifier
                              _:•F         FunctionParameterDeclaration
                              _            WildcardPattern                                                                                */
fn f<M>(a: M) where M: A, M::B: C, {}                                                                                                     /*
fn•f<M>(a:•M)•where•M:•A,•M::B:•C,•{}    FunctionDeclaration
     M                                   GenericTypeParameterDeclaration
        a:•M                             FunctionParameterDeclaration
                    M:•A                 WhereTypeBoundDeclaration
                       A                 TypeTraitBound
                          M::B:•C        WhereTypeBoundDeclaration
                          M::B           TypePath
                                C        TypeTraitBound                                                                                   */
fn f<T,A>(t: fn(&A)) where fn(&A) : for<'a> F<(&'a A,)>,{}                                                                                /*
fn•f<T,A>(t:•fn(&A))•where•fn(&A)•:•for<'a>•F<(&'a•A,)>,{}    FunctionDeclaration
     T                                                        GenericTypeParameterDeclaration
       A                                                      GenericTypeParameterDeclaration
          t:•fn(&A)                                           FunctionParameterDeclaration
             fn(&A)                                           TypeFnPointer
                &A                                            TypeFnPointerParameter, TypeReference
                           fn(&A)•:•for<'a>•F<(&'a•A,)>       WhereTypeBoundDeclaration
                           fn(&A)                             TypeFnPointer
                              &A                              TypeFnPointerParameter, TypeReference
                                    for<'a>•F<(&'a•A,)>       TypeTraitBound
                                        'a                    GenericLtParameterDeclaration, LtIdentifier
                                            F<(&'a•A,)>       TypeCall
                                              (&'a•A,)        TypeTuple
                                               &'a•A          TypeReference
                                                'a            LtIdentifier                                                                */
#[no_mangle]                                                                                                                              /*
#[no_mangle]↲    <FunctionDeclaration>
#[no_mangle]     Attribute                                                                                                                */
pub extern "C" fn rust_no_mangle() -> i32 {}                                                                                              /*
pub•extern•"C"•fn•rust_no_mangle()•->•i32•{}    </FunctionDeclaration>
pub                                             PubSpecifier
    extern•"C"                                  ExternSpecifier
           "C"                                  Literal                                                                                   */
pub fn foo<'a, 'b>(x: Foo<'a, 'b>, _o: Option<&   &   ()>) { let _y = x.foo; }                                                            /*
pub•fn•foo<'a,•'b>(x:•Foo<'a,•'b>,•_o:•Option<&•••&•••()>)•{•let•_y•=•x.foo;•}    FunctionDeclaration
pub                                                                               PubSpecifier
           'a                                                                     GenericLtParameterDeclaration, LtIdentifier
               'b                                                                 GenericLtParameterDeclaration, LtIdentifier
                   x:•Foo<'a,•'b>                                                 FunctionParameterDeclaration
                      Foo<'a,•'b>                                                 TypeCall
                          'a                                                      LtIdentifier
                              'b                                                  LtIdentifier
                                   _o:•Option<&•••&•••()>                         FunctionParameterDeclaration
                                       Option<&•••&•••()>                         TypeCall
                                              &•••&•••()                          TypeReference
                                                  &•••()                          TypeReference
                                                      ()                          TypeTuple
                                                             let•_y•=•x.foo;      LetVariableDeclaration
                                                                      x.foo       MemberExpression                                        */
const x: &'static dyn Fn() = &|| e!("q");                                                                                                 /*
const•x:•&'static•dyn•Fn()•=•&||•e!("q");    ConstVariableDeclaration
         &'static•dyn•Fn()                   TypeReference
          'static                            LtStatic
                  dyn•Fn()                   TypeDynBounds
                      Fn()                   TypeTraitBound, TypeFunction
                             &||•e!("q")     ReferenceExpression
                              ||•e!("q")     ClosureFunctionExpression
                                 e!("q")     MacroInvocation
                                    "q"      Literal                                                                                      */
const fn foo() -> i32 {}                                                                                                                  /*
const•fn•foo()•->•i32•{}    FunctionDeclaration                                                                                           */
extern "\x43" fn foo() {}                                                                                                                 /*
extern•"\x43"•fn•foo()•{}    FunctionDeclaration
extern•"\x43"                ExternSpecifier
       "\x43"                Literal                                                                                                      */

extern "\x43" {                                                                                                                           /*
extern•"\x43"•{↲    <ExternBlockDeclaration>
       "\x43"       Literal                                                                                                               */
    fn bar();                                                                                                                             /*
    fn•bar();    FunctionDeclaration                                                                                                      */
}                                                                                                                                         /*
}    </ExternBlockDeclaration>                                                                                                            */

type T = extern "\x43" fn();                                                                                                              /*
type•T•=•extern•"\x43"•fn();    TypeAliasDeclaration
         extern•"\x43"•fn()     TypeFnPointer
         extern•"\x43"          ExternSpecifier
                "\x43"          Literal                                                                                                   */
extern r#"C"# fn foo() {}                                                                                                                 /*
extern•r#"C"#•fn•foo()•{}    FunctionDeclaration
extern•r#"C"#                ExternSpecifier
       r#"C"#                Literal                                                                                                      */

extern r#"C"# {                                                                                                                           /*
extern•r#"C"#•{↲    <ExternBlockDeclaration>
       r#"C"#       Literal                                                                                                               */
    fn bar();                                                                                                                             /*
    fn•bar();    FunctionDeclaration                                                                                                      */
}                                                                                                                                         /*
}    </ExternBlockDeclaration>                                                                                                            */

type T = extern r#"C"# fn();                                                                                                              /*
type•T•=•extern•r#"C"#•fn();    TypeAliasDeclaration
         extern•r#"C"#•fn()     TypeFnPointer
         extern•r#"C"#          ExternSpecifier
                r#"C"#          Literal                                                                                                   */

// Discarded Nodes: 0
// Parsed Nodes: 681
// state_rollbacks: 44
// Total '.charCodeAt()' calls: 4272 (42% re-reads)
// Unnecessary 'skip_whitespace()' calls: 454
// source: "../../samples/statements/statements.rs"