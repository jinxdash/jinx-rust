const _: () = {                                                                                                                           /*
const•_:•()•=•{↲    <Program>
const•_:•()•=•{↲    <Program.ast{dk: "None"}>
const•_:•()•=•{↲    <ConstVariableDeclaration>
      _             WildcardPattern
         ()         TypeTuple
              {↲    <BlockExpression>                                                                                                     */
    pub trait A {                                                                                                                         /*
    pub•trait•A•{↲    <TraitDeclaration>
    pub               PubSpecifier
                {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                   */
        const _: () = ();                                                                                                                 /*
        const•_:•()•=•();    ConstVariableDeclaration
              _              WildcardPattern
                 ()          TypeTuple
                      ()     TupleLiteral                                                                                                 */
    }                                                                                                                                     /*
••••}    </TraitDeclaration.body>
••••}    </TraitDeclaration>                                                                                                              */
    impl A for () {                                                                                                                       /*
    impl•A•for•()•{↲    <ImplDeclaration{!const}>
               ()       TypeTuple
                  {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                  */
        const _: () = ();                                                                                                                 /*
        const•_:•()•=•();    ConstVariableDeclaration
              _              WildcardPattern
                 ()          TypeTuple
                      ()     TupleLiteral                                                                                                 */
    }                                                                                                                                     /*
••••}    </ImplDeclaration.body>
••••}    </ImplDeclaration>                                                                                                               */
    impl dyn A {                                                                                                                          /*
    impl•dyn•A•{↲    <ImplDeclaration{!const}>
         dyn•A       TypeDynBounds{dyn}
             A       TypeTraitBound{!maybeConst, !optional}
               {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                     */
        const _: () = ();                                                                                                                 /*
        const•_:•()•=•();    ConstVariableDeclaration
              _              WildcardPattern
                 ()          TypeTuple
                      ()     TupleLiteral                                                                                                 */
    }                                                                                                                                     /*
••••}    </ImplDeclaration.body>
••••}    </ImplDeclaration>                                                                                                               */
};                                                                                                                                        /*
}     </BlockExpression>
};    </ConstVariableDeclaration>                                                                                                         */


extern r#"C"# { fn bar(); }                                                                                                               /*
extern•r#"C"#•{•fn•bar();•}    ExternBlockDeclaration
       r#"C"#                  Literal{kind: rString}
              {•fn•bar();•}    ExternBlockDeclaration.body{dk: "{}"}
                fn•bar();      FunctionDeclaration
                      ()       FunctionDeclaration.parameters{dk: "()"}                                                                   */
extern r#"C"# fn foo() {}                                                                                                                 /*
extern•r#"C"#•fn•foo()•{}    FunctionDeclaration
extern•r#"C"#                ExternSpecifier
       r#"C"#                Literal{kind: rString}
                    ()       FunctionDeclaration.parameters{dk: "()"}
                       {}    FunctionDeclaration.body{dk: "{}"}                                                                           */
type T = extern r#"C"# fn();                                                                                                              /*
type•T•=•extern•r#"C"#•fn();    TypeAliasDeclaration
         extern•r#"C"#•fn()     TypeFnPointer
         extern•r#"C"#          ExternSpecifier
                r#"C"#          Literal{kind: rString}
                         ()     TypeFnPointer.parameters{dk: "()"}                                                                        */
extern "\x43" fn foo() {}                                                                                                                 /*
extern•"\x43"•fn•foo()•{}    FunctionDeclaration
extern•"\x43"                ExternSpecifier
       "\x43"                Literal{kind: String}
                    ()       FunctionDeclaration.parameters{dk: "()"}
                       {}    FunctionDeclaration.body{dk: "{}"}                                                                           */
extern "\x43" { fn bar(); }                                                                                                               /*
extern•"\x43"•{•fn•bar();•}    ExternBlockDeclaration
       "\x43"                  Literal{kind: String}
              {•fn•bar();•}    ExternBlockDeclaration.body{dk: "{}"}
                fn•bar();      FunctionDeclaration
                      ()       FunctionDeclaration.parameters{dk: "()"}                                                                   */
type T = extern "\x43" fn();                                                                                                              /*
type•T•=•extern•"\x43"•fn();    TypeAliasDeclaration
         extern•"\x43"•fn()     TypeFnPointer
         extern•"\x43"          ExternSpecifier
                "\x43"          Literal{kind: String}
                         ()     TypeFnPointer.parameters{dk: "()"}                                                                        */

// extern crate async;
//•extern•crate•async;    Comment{line}
// extern crate async as something_else;
//•extern•crate•async•as•something_else;    Comment{line}

fn f1();                                                                                                                                  /*
fn•f1();    FunctionDeclaration
     ()     FunctionDeclaration.parameters{dk: "()"}                                                                                      */
fn f2() {};                                                                                                                               /*
fn•f2()•{}     FunctionDeclaration
     ()        FunctionDeclaration.parameters{dk: "()"}
        {}     FunctionDeclaration.body{dk: "{}"}
          ;    ExpressionStatement{semi}                                                                                                  */
fn f3();                                                                                                                                  /*
fn•f3();    FunctionDeclaration
     ()     FunctionDeclaration.parameters{dk: "()"}                                                                                      */

trait X {                                                                                                                                 /*
trait•X•{↲    <TraitDeclaration>
        {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                           */
	fn f();                                                                                                                               /*
	fn•f();    FunctionDeclaration
	    ()     FunctionDeclaration.parameters{dk: "()"}                                                                                   */
	fn f() {}                                                                                                                             /*
	fn•f()•{}    FunctionDeclaration
	    ()       FunctionDeclaration.parameters{dk: "()"}
	       {}    FunctionDeclaration.body{dk: "{}"}                                                                                       */
	const Y: u8;                                                                                                                          /*
	const•Y:•u8;    ConstVariableDeclaration                                                                                              */
}                                                                                                                                         /*
}    </TraitDeclaration.body>
}    </TraitDeclaration>                                                                                                                  */


extern "C" {                                                                                                                              /*
extern•"C"•{↲    <ExternBlockDeclaration>
       "C"       Literal{kind: String}
           {↲    <ExternBlockDeclaration.body{dk: "{}"}>                                                                                  */
	fn f();                                                                                                                               /*
	fn•f();    FunctionDeclaration
	    ()     FunctionDeclaration.parameters{dk: "()"}                                                                                   */
	fn f();                                                                                                                               /*
	fn•f();    FunctionDeclaration
	    ()     FunctionDeclaration.parameters{dk: "()"}                                                                                   */
    static X: u8;                                                                                                                         /*
    static•X:•u8;    StaticVariableDeclaration                                                                                            */
    static mut Y: u8;                                                                                                                     /*
    static•mut•Y:•u8;    StaticVariableDeclaration
           mut•Y         PatternVariableDeclaration{!ref, mut}                                                                            */
	// type E: where;
	//•type•E:•where;    Comment{line}
	type A: Ord;                                                                                                                          /*
	type•A:•Ord;    TypeAliasDeclaration
	        Ord     TypeTraitBound{!maybeConst, !optional}                                                                                */
    type A<'a> where 'a: 'static;                                                                                                         /*
    type•A<'a>•where•'a:•'static;    TypeAliasDeclaration
          <'a>                       TypeAliasDeclaration.generics{dk: "<>"}
           'a                        GenericLtParameterDeclaration, LtIdentifier
               where•'a:•'static     TypeAliasDeclaration.whereBounds{dk: "None"}
                     'a:•'static     WhereLtBoundDeclaration
                     'a              LtIdentifier
                         'static     LtStatic                                                                                             */
    type A<T: Ord> where T: 'static;                                                                                                      /*
    type•A<T:•Ord>•where•T:•'static;    TypeAliasDeclaration
          <T:•Ord>                      TypeAliasDeclaration.generics{dk: "<>"}
           T:•Ord                       GenericTypeParameterDeclaration
              Ord                       TypeTraitBound{!maybeConst, !optional}
                   where•T:•'static     TypeAliasDeclaration.whereBounds{dk: "None"}
                         T:•'static     WhereTypeBoundDeclaration
                            'static     LtStatic                                                                                          */
    type A = u8;                                                                                                                          /*
    type•A•=•u8;    TypeAliasDeclaration                                                                                                  */
    type A<'a: 'static, T: Ord + 'static>: Eq + PartialEq where T: 'static + Copy = Vec<u8>;                                              /*
    type•A<'a:•'static,•T:•Ord•+•'static>:•Eq•+•PartialEq•where•T:•'static•+•Copy•=•Vec<u8>;    TypeAliasDeclaration
          <'a:•'static,•T:•Ord•+•'static>                                                       TypeAliasDeclaration.generics{dk: "<>"}
           'a:•'static                                                                          GenericLtParameterDeclaration
           'a                                                                                   LtIdentifier
               'static                                                                          LtStatic
                        T:•Ord•+•'static                                                        GenericTypeParameterDeclaration
                           Ord                                                                  TypeTraitBound{!maybeConst, !optional}
                                 'static                                                        LtStatic
                                           Eq                                                   TypeTraitBound{!maybeConst, !optional}
                                                PartialEq                                       TypeTraitBound{!maybeConst, !optional}
                                                          where•T:•'static•+•Copy               TypeAliasDeclaration.whereBounds{dk: "None"}
                                                                T:•'static•+•Copy               WhereTypeBoundDeclaration
                                                                   'static                      LtStatic
                                                                             Copy               TypeTraitBound{!maybeConst, !optional}
                                                                                    Vec<u8>     TypeCall
                                                                                       <u8>     TypeCall.typeArguments{dk: "<>"}          */
}                                                                                                                                         /*
}    </ExternBlockDeclaration.body>
}    </ExternBlockDeclaration>                                                                                                            */
const async const fn test() {}                                                                                                            /*
const•async•const•fn•test()•{}    FunctionDeclaration{const, async}
                         ()       FunctionDeclaration.parameters{dk: "()"}
                            {}    FunctionDeclaration.body{dk: "{}"}                                                                      */
unsafe async fn test() {}                                                                                                                 /*
unsafe•async•fn•test()•{}    FunctionDeclaration{async, unsafe}
                    ()       FunctionDeclaration.parameters{dk: "()"}
                       {}    FunctionDeclaration.body{dk: "{}"}                                                                           */
unsafe const fn test() {}                                                                                                                 /*
unsafe•const•fn•test()•{}    FunctionDeclaration{const, unsafe}
                    ()       FunctionDeclaration.parameters{dk: "()"}
                       {}    FunctionDeclaration.body{dk: "{}"}                                                                           */
extern unsafe fn test() {}                                                                                                                /*
extern•unsafe•fn•test()•{}    FunctionDeclaration{unsafe}
extern                        ExternSpecifier
                     ()       FunctionDeclaration.parameters{dk: "()"}
                        {}    FunctionDeclaration.body{dk: "{}"}                                                                          */
fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    async fn f();                                                                                                                         /*
    async•fn•f();    FunctionDeclaration{async}
              ()     FunctionDeclaration.parameters{dk: "()"}                                                                             */
    unsafe fn f();                                                                                                                        /*
    unsafe•fn•f();    FunctionDeclaration{unsafe}
               ()     FunctionDeclaration.parameters{dk: "()"}                                                                            */
    const fn f();                                                                                                                         /*
    const•fn•f();    FunctionDeclaration{const}
              ()     FunctionDeclaration.parameters{dk: "()"}                                                                             */
    extern "C" fn f();                                                                                                                    /*
    extern•"C"•fn•f();    FunctionDeclaration
    extern•"C"            ExternSpecifier
           "C"            Literal{kind: String}
                   ()     FunctionDeclaration.parameters{dk: "()"}                                                                        */
    const async unsafe extern "C" fn f();                                                                                                 /*
    const•async•unsafe•extern•"C"•fn•f();    FunctionDeclaration{unsafe, async, const}
                       extern•"C"            ExternSpecifier
                              "C"            Literal{kind: String}
                                      ()     FunctionDeclaration.parameters{dk: "()"}                                                     */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
const fn f(a: *const i32, b: i32) -> bool {}                                                                                              /*
const•fn•f(a:•*const•i32,•b:•i32)•->•bool•{}    FunctionDeclaration{const}
          (a:•*const•i32,•b:•i32)               FunctionDeclaration.parameters{dk: "()"}
           a:•*const•i32                        FunctionParameterDeclaration
              *const•i32                        TypeDereferenceConst
                          b:•i32                FunctionParameterDeclaration
                                          {}    FunctionDeclaration.body{dk: "{}"}                                                        */
unsafe fn f(&self) -> u32;                                                                                                                /*
unsafe•fn•f(&self)•->•u32;    FunctionDeclaration{unsafe}
           (&self)            FunctionDeclaration.parameters{dk: "()"}
            &self             FunctionSelfParameterDeclaration{ref, !mut}                                                                 */
const unsafe fn f(v: u32) -> u32 {}                                                                                                       /*
const•unsafe•fn•f(v:•u32)•->•u32•{}    FunctionDeclaration{unsafe, const}
                 (v:•u32)              FunctionDeclaration.parameters{dk: "()"}
                  v:•u32               FunctionParameterDeclaration
                                 {}    FunctionDeclaration.body{dk: "{}"}                                                                 */
unsafe fn f(func: unsafe fn() -> ()) -> () {}                                                                                             /*
unsafe•fn•f(func:•unsafe•fn()•->•())•->•()•{}    FunctionDeclaration{unsafe}
           (func:•unsafe•fn()•->•())             FunctionDeclaration.parameters{dk: "()"}
            func:•unsafe•fn()•->•()              FunctionParameterDeclaration
                  unsafe•fn()•->•()              TypeFnPointer{unsafe}
                           ()                    TypeFnPointer.parameters{dk: "()"}
                                 ()              TypeTuple
                                        ()       TypeTuple
                                           {}    FunctionDeclaration.body{dk: "{}"}                                                       */
struct Range<const FROM: usize = 0, const LEN: usize = 0, const TO: usize = FROM>;                                                        /*
struct•Range<const•FROM:•usize•=•0,•const•LEN:•usize•=•0,•const•TO:•usize•=•FROM>;    StructDeclaration
            <const•FROM:•usize•=•0,•const•LEN:•usize•=•0,•const•TO:•usize•=•FROM>     StructDeclaration.generics{dk: "<>"}
             const•FROM:•usize•=•0                                                    ConstTypeParameterDeclaration
                                 0                                                    Literal{kind: Integer}
                                    const•LEN:•usize•=•0                              ConstTypeParameterDeclaration
                                                       0                              Literal{kind: Integer}
                                                          const•TO:•usize•=•FROM      ConstTypeParameterDeclaration                       */
impl<T> From<[u8; 1 + 1]> for Foo<T, [u8; 1 + 1]> {}                                                                                      /*
impl<T>•From<[u8;•1•+•1]>•for•Foo<T,•[u8;•1•+•1]>•{}    ImplDeclaration{!const}
    <T>                                                 ImplDeclaration.generics{dk: "<>"}
     T                                                  GenericTypeParameterDeclaration
        From<[u8;•1•+•1]>                               TypeCall
            <[u8;•1•+•1]>                               TypeCall.typeArguments{dk: "<>"}
             [u8;•1•+•1]                                TypeSizedArray
                  1•+•1                                 OperationExpression{tk: "+"}
                  1                                     Literal{kind: Integer}
                      1                                 Literal{kind: Integer}
                              Foo<T,•[u8;•1•+•1]>       TypeCall
                                 <T,•[u8;•1•+•1]>       TypeCall.typeArguments{dk: "<>"}
                                     [u8;•1•+•1]        TypeSizedArray
                                          1•+•1         OperationExpression{tk: "+"}
                                          1             Literal{kind: Integer}
                                              1         Literal{kind: Integer}
                                                  {}    ImplDeclaration.body{dk: "{}"}                                                    */
fn f(d: [u8; 1 + 1]) -> A<T, [u8; 1 + 1]>                                                                                                 /*
fn•f(d:•[u8;•1•+•1])•->•A<T,•[u8;•1•+•1]>•↲    <FunctionDeclaration>
    (d:•[u8;•1•+•1])                           FunctionDeclaration.parameters{dk: "()"}
     d:•[u8;•1•+•1]                            FunctionParameterDeclaration
        [u8;•1•+•1]                            TypeSizedArray
             1•+•1                             OperationExpression{tk: "+"}
             1                                 Literal{kind: Integer}
                 1                             Literal{kind: Integer}
                        A<T,•[u8;•1•+•1]>      TypeCall
                         <T,•[u8;•1•+•1]>      TypeCall.typeArguments{dk: "<>"}
                             [u8;•1•+•1]       TypeSizedArray
                                  1•+•1        OperationExpression{tk: "+"}
                                  1            Literal{kind: Integer}
                                      1        Literal{kind: Integer}                                                                     */
where [u8; 1 + 1]: From<[u8; 1 + 1]> {}                                                                                                   /*
where•[u8;•1•+•1]:•From<[u8;•1•+•1]>       FunctionDeclaration.whereBounds{dk: "None"}
      [u8;•1•+•1]:•From<[u8;•1•+•1]>       WhereTypeBoundDeclaration
      [u8;•1•+•1]                          TypeSizedArray
           1•+•1                           OperationExpression{tk: "+"}
           1                               Literal{kind: Integer}
               1                           Literal{kind: Integer}
                   From<[u8;•1•+•1]>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                       <[u8;•1•+•1]>       TypeCall.typeArguments{dk: "<>"}
                        [u8;•1•+•1]        TypeSizedArray
                             1•+•1         OperationExpression{tk: "+"}
                             1             Literal{kind: Integer}
                                 1         Literal{kind: Integer}
                                     {}    FunctionDeclaration.body{dk: "{}"}
where•[u8;•1•+•1]:•From<[u8;•1•+•1]>•{}    </FunctionDeclaration>                                                                         */
fn f<'a, 'b, 'c, T>(x: foo::X<'a, T, 'b, 'c>) {}                                                                                          /*
fn•f<'a,•'b,•'c,•T>(x:•foo::X<'a,•T,•'b,•'c>)•{}    FunctionDeclaration
    <'a,•'b,•'c,•T>                                 FunctionDeclaration.generics{dk: "<>"}
     'a                                             GenericLtParameterDeclaration, LtIdentifier
         'b                                         GenericLtParameterDeclaration, LtIdentifier
             'c                                     GenericLtParameterDeclaration, LtIdentifier
                 T                                  GenericTypeParameterDeclaration
                   (x:•foo::X<'a,•T,•'b,•'c>)       FunctionDeclaration.parameters{dk: "()"}
                    x:•foo::X<'a,•T,•'b,•'c>        FunctionParameterDeclaration
                       foo::X<'a,•T,•'b,•'c>        TypeCall
                       foo::X                       TypePath
                             <'a,•T,•'b,•'c>        TypeCall.typeArguments{dk: "<>"}
                              'a                    LtIdentifier
                                     'b             LtIdentifier
                                         'c         LtIdentifier
                                              {}    FunctionDeclaration.body{dk: "{}"}                                                    */
fn f() -> Option<fn() -> Option<bool>> { Some(|| Some(true)) }                                                                            /*
fn•f()•->•Option<fn()•->•Option<bool>>•{•Some(||•Some(true))•}    FunctionDeclaration
    ()                                                            FunctionDeclaration.parameters{dk: "()"}
          Option<fn()•->•Option<bool>>                            TypeCall
                <fn()•->•Option<bool>>                            TypeCall.typeArguments{dk: "<>"}
                 fn()•->•Option<bool>                             TypeFnPointer
                   ()                                             TypeFnPointer.parameters{dk: "()"}
                         Option<bool>                             TypeCall
                               <bool>                             TypeCall.typeArguments{dk: "<>"}
                                       {•Some(||•Some(true))•}    FunctionDeclaration.body{dk: "{}"}
                                         Some(||•Some(true))      ExpressionStatement{!semi}, CallExpression
                                             (||•Some(true))      CallExpression.arguments{dk: "()"}
                                              ||•Some(true)       ClosureFunctionExpression
                                              ||                  ClosureFunctionExpression.parameters{dk: "||"}
                                                 Some(true)       CallExpression
                                                     (true)       CallExpression.arguments{dk: "()"}
                                                      true        Literal{kind: True}                                                     */
fn a() {                                                                                                                                  /*
fn•a()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	let a = 0;                                                                                                                            /*
	let•a•=•0;    LetVariableDeclaration
	        0     Literal{kind: Integer}                                                                                                  */
    let _b = 0;                                                                                                                           /*
    let•_b•=•0;    LetVariableDeclaration
             0     Literal{kind: Integer}                                                                                                 */
    let _ = 0;                                                                                                                            /*
    let•_•=•0;    LetVariableDeclaration
        _         WildcardPattern
            0     Literal{kind: Integer}                                                                                                  */
    let mut b = 0;                                                                                                                        /*
    let•mut•b•=•0;    LetVariableDeclaration
        mut•b         PatternVariableDeclaration{!ref, mut}
                0     Literal{kind: Integer}                                                                                              */
    let mut _b = 0;                                                                                                                       /*
    let•mut•_b•=•0;    LetVariableDeclaration
        mut•_b         PatternVariableDeclaration{!ref, mut}
                 0     Literal{kind: Integer}                                                                                             */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

enum Test3 {                                                                                                                              /*
enum•Test3•{↲    <EnumDeclaration>
           {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                      */
	Var1,                                                                                                                                 /*
	Var1    EnumMemberDeclaration                                                                                                         */
	Var2(String),                                                                                                                         /*
	Var2(String)    EnumMemberTupleDeclaration
	    (String)    EnumMemberTupleDeclaration.items{dk: "()"}
	     String     TupleStructItemDeclaration                                                                                            */
	StillFine {                                                                                                                           /*
	StillFine•{↲    <EnumMemberStructDeclaration>
	          {↲    <EnumMemberStructDeclaration.properties{dk: "{}"}>                                                                    */
		def: i32,                                                                                                                         /*
		def:•i32    StructPropertyDeclaration                                                                                             */
	},                                                                                                                                    /*
   ╚}    </EnumMemberStructDeclaration.properties>
   ╚}    </EnumMemberStructDeclaration>                                                                                                   */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
enum E {                                                                                                                                  /*
enum•E•{↲    <EnumDeclaration>
       {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                          */
    UnitVariant,                                                                                                                          /*
    UnitVariant    EnumMemberDeclaration                                                                                                  */
    TupleVariant(),                                                                                                                       /*
    TupleVariant()    EnumMemberTupleDeclaration
                ()    EnumMemberTupleDeclaration.items{dk: "()"}                                                                          */
    BracedVariant{},                                                                                                                      /*
    BracedVariant{}    EnumMemberStructDeclaration
                 {}    EnumMemberStructDeclaration.properties{dk: "{}"}                                                                   */
	T(T, [!; 0]),                                                                                                                         /*
	T(T,•[!;•0])    EnumMemberTupleDeclaration
	 (T,•[!;•0])    EnumMemberTupleDeclaration.items{dk: "()"}
	  T             TupleStructItemDeclaration
	     [!;•0]     TupleStructItemDeclaration, TypeSizedArray
	      !         TypeNever
	         0      Literal{kind: Integer}                                                                                                */
    #[allow(dead_code)]                                                                                                                   /*
    #[allow(dead_code)]↲    <EnumMemberTupleDeclaration>
    #[allow(dead_code)]     Attribute{!inner}
     [allow(dead_code)]     Attribute.segments{dk: "[]"}
           (dead_code)      DelimGroup                                                                                                    */
    U(U),                                                                                                                                 /*
    U(U)    EnumMemberTupleDeclaration~ownStart
     (U)    EnumMemberTupleDeclaration.items{dk: "()"}
      U     TupleStructItemDeclaration
••••U(U)    </EnumMemberTupleDeclaration>                                                                                                 */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
fn foobar<F, G>() -> usize                                                                                                                /*
fn•foobar<F,•G>()•->•usize↲    <FunctionDeclaration>
         <F,•G>                FunctionDeclaration.generics{dk: "<>"}
          F                    GenericTypeParameterDeclaration
             G                 GenericTypeParameterDeclaration
               ()              FunctionDeclaration.parameters{dk: "()"}                                                                   */
where                                                                                                                                     /*
where↲    <FunctionDeclaration.whereBounds{dk: "None"}>                                                                                   */
    (): Foobar<F, G>,{}                                                                                                                   /*
    ():•Foobar<F,•G>       WhereTypeBoundDeclaration
    ()                     TypeTuple
        Foobar<F,•G>       TypeTraitBound{!maybeConst, !optional}, TypeCall
              <F,•G>       TypeCall.typeArguments{dk: "<>"}
••••():•Foobar<F,•G>,      </FunctionDeclaration.whereBounds>
                     {}    FunctionDeclaration.body{dk: "{}"}
••••():•Foobar<F,•G>,{}    </FunctionDeclaration>                                                                                         */

mod a {                                                                                                                                   /*
mod•a•{↲    <ModuleDeclaration>
      {↲    <ModuleDeclaration.body{dk: "{}"}>                                                                                            */
	extern "C" {                                                                                                                          /*
	extern•"C"•{↲    <ExternBlockDeclaration>
	       "C"       Literal{kind: String}
	           {↲    <ExternBlockDeclaration.body{dk: "{}"}>                                                                              */
		pub fn free(x: *const u8);                                                                                                        /*
		pub•fn•free(x:•*const•u8);    FunctionDeclaration
		pub                           PubSpecifier
		           (x:•*const•u8)     FunctionDeclaration.parameters{dk: "()"}
		            x:•*const•u8      FunctionParameterDeclaration
		               *const•u8      TypeDereferenceConst                                                                                */
	}                                                                                                                                     /*
   ╚}    </ExternBlockDeclaration.body>
   ╚}    </ExternBlockDeclaration>                                                                                                        */
}                                                                                                                                         /*
}    </ModuleDeclaration.body>
}    </ModuleDeclaration>                                                                                                                 */
pub union U {                                                                                                                             /*
pub•union•U•{↲    <UnionDeclaration>
pub               PubSpecifier
            {↲    <UnionDeclaration.properties{dk: "{}"}>                                                                                 */
	pub a: u8,                                                                                                                            /*
	pub•a:•u8    StructPropertyDeclaration
	pub          PubSpecifier                                                                                                             */
	pub(super) b: u8,                                                                                                                     /*
	pub(super)•b:•u8    StructPropertyDeclaration
	pub(super)          PubSpecifier                                                                                                      */
	c: u8,                                                                                                                                /*
	c:•u8    StructPropertyDeclaration                                                                                                    */
}                                                                                                                                         /*
}    </UnionDeclaration.properties>
}    </UnionDeclaration>                                                                                                                  */


trait C<A> {                                                                                                                              /*
trait•C<A>•{↲    <TraitDeclaration>
       <A>       TraitDeclaration.generics{dk: "<>"}
        A        GenericTypeParameterDeclaration
           {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                        */
    fn D<B, F>(&self, f: F) where F: FnMut(A) -> Q<B>;                                                                                    /*
    fn•D<B,•F>(&self,•f:•F)•where•F:•FnMut(A)•->•Q<B>;    FunctionDeclaration
        <B,•F>                                            FunctionDeclaration.generics{dk: "<>"}
         B                                                GenericTypeParameterDeclaration
            F                                             GenericTypeParameterDeclaration
              (&self,•f:•F)                               FunctionDeclaration.parameters{dk: "()"}
               &self                                      FunctionSelfParameterDeclaration{ref, !mut}
                      f:•F                                FunctionParameterDeclaration
                            where•F:•FnMut(A)•->•Q<B>     FunctionDeclaration.whereBounds{dk: "None"}
                                  F:•FnMut(A)•->•Q<B>     WhereTypeBoundDeclaration
                                     FnMut(A)•->•Q<B>     TypeTraitBound{!maybeConst, !optional}, TypeFunction
                                          (A)             TypeFunction.parameters{dk: "()"}
                                                 Q<B>     TypeCall
                                                  <B>     TypeCall.typeArguments{dk: "<>"}                                                */
}                                                                                                                                         /*
}    </TraitDeclaration.body>
}    </TraitDeclaration>                                                                                                                  */
trait A<T: B=Self> {}                                                                                                                     /*
trait•A<T:•B=Self>•{}    TraitDeclaration
       <T:•B=Self>       TraitDeclaration.generics{dk: "<>"}
        T:•B=Self        GenericTypeParameterDeclaration
           B             TypeTraitBound{!maybeConst, !optional}
                   {}    TraitDeclaration.body{dk: "{}"}                                                                                  */
trait A: B::C {}                                                                                                                          /*
trait•A:•B::C•{}    TraitDeclaration
         B::C       TypeTraitBound{!maybeConst, !optional}, TypePath
              {}    TraitDeclaration.body{dk: "{}"}                                                                                       */
trait A<T>: B::C {}                                                                                                                       /*
trait•A<T>:•B::C•{}    TraitDeclaration
       <T>             TraitDeclaration.generics{dk: "<>"}
        T              GenericTypeParameterDeclaration
            B::C       TypeTraitBound{!maybeConst, !optional}, TypePath
                 {}    TraitDeclaration.body{dk: "{}"}                                                                                    */
fn f() -> () { }                                                                                                                          /*
fn•f()•->•()•{•}    FunctionDeclaration
    ()              FunctionDeclaration.parameters{dk: "()"}
          ()        TypeTuple
             {•}    FunctionDeclaration.body{dk: "{}"}                                                                                    */
fn f<T: X<Y<()> = i32>>() {}                                                                                                              /*
fn•f<T:•X<Y<()>•=•i32>>()•{}    FunctionDeclaration
    <T:•X<Y<()>•=•i32>>         FunctionDeclaration.generics{dk: "<>"}
     T:•X<Y<()>•=•i32>          GenericTypeParameterDeclaration
        X<Y<()>•=•i32>          TypeTraitBound{!maybeConst, !optional}, TypeCall
         <Y<()>•=•i32>          TypeCall.typeArguments{dk: "<>"}
          Y<()>•=•i32           TypeCallNamedArgument
          Y<()>                 TypeCall
           <()>                 TypeCall.typeArguments{dk: "<>"}
            ()                  TypeTuple
                       ()       FunctionDeclaration.parameters{dk: "()"}
                          {}    FunctionDeclaration.body{dk: "{}"}                                                                        */
fn f<F>(mut f: F) where F: FnMut(&mut R, bool) {}                                                                                         /*
fn•f<F>(mut•f:•F)•where•F:•FnMut(&mut•R,•bool)•{}    FunctionDeclaration
    <F>                                              FunctionDeclaration.generics{dk: "<>"}
     F                                               GenericTypeParameterDeclaration
       (mut•f:•F)                                    FunctionDeclaration.parameters{dk: "()"}
        mut•f:•F                                     FunctionParameterDeclaration
        mut•f                                        PatternVariableDeclaration{!ref, mut}
                  where•F:•FnMut(&mut•R,•bool)       FunctionDeclaration.whereBounds{dk: "None"}
                        F:•FnMut(&mut•R,•bool)       WhereTypeBoundDeclaration
                           FnMut(&mut•R,•bool)       TypeTraitBound{!maybeConst, !optional}, TypeFunction
                                (&mut•R,•bool)       TypeFunction.parameters{dk: "()"}
                                 &mut•R              TypeReference{mut}
                                               {}    FunctionDeclaration.body{dk: "{}"}                                                   */
fn f<F>(f: F) where F: for<'a> Fn(&'a isize, &'a isize) -> isize { }                                                                      /*
fn•f<F>(f:•F)•where•F:•for<'a>•Fn(&'a•isize,•&'a•isize)•->•isize•{•}    FunctionDeclaration
    <F>                                                                 FunctionDeclaration.generics{dk: "<>"}
     F                                                                  GenericTypeParameterDeclaration
       (f:•F)                                                           FunctionDeclaration.parameters{dk: "()"}
        f:•F                                                            FunctionParameterDeclaration
              where•F:•for<'a>•Fn(&'a•isize,•&'a•isize)•->•isize        FunctionDeclaration.whereBounds{dk: "None"}
                    F:•for<'a>•Fn(&'a•isize,•&'a•isize)•->•isize        WhereTypeBoundDeclaration
                       for<'a>•Fn(&'a•isize,•&'a•isize)•->•isize        TypeTraitBound{!maybeConst, !optional}
                       for<'a>                                          TypeTraitBound.ltParameters{dk: "<>"}
                           'a                                           GenericLtParameterDeclaration, LtIdentifier
                               Fn(&'a•isize,•&'a•isize)•->•isize        TypeFunction
                                 (&'a•isize,•&'a•isize)                 TypeFunction.parameters{dk: "()"}
                                  &'a•isize                             TypeReference{!mut}
                                   'a                                   LtIdentifier
                                             &'a•isize                  TypeReference{!mut}
                                              'a                        LtIdentifier
                                                                 {•}    FunctionDeclaration.body{dk: "{}"}                                */
fn f<F>(f: F) -> isize where F: Fn() -> isize { f() }                                                                                     /*
fn•f<F>(f:•F)•->•isize•where•F:•Fn()•->•isize•{•f()•}    FunctionDeclaration
    <F>                                                  FunctionDeclaration.generics{dk: "<>"}
     F                                                   GenericTypeParameterDeclaration
       (f:•F)                                            FunctionDeclaration.parameters{dk: "()"}
        f:•F                                             FunctionParameterDeclaration
                       where•F:•Fn()•->•isize            FunctionDeclaration.whereBounds{dk: "None"}
                             F:•Fn()•->•isize            WhereTypeBoundDeclaration
                                Fn()•->•isize            TypeTraitBound{!maybeConst, !optional}, TypeFunction
                                  ()                     TypeFunction.parameters{dk: "()"}
                                              {•f()•}    FunctionDeclaration.body{dk: "{}"}
                                                f()      ExpressionStatement{!semi}, CallExpression
                                                 ()      CallExpression.arguments{dk: "()"}                                               */
async fn g(((ref a, ref mut b), (ref mut c, ref d)): ((A, A), (A, A))) {}                                                                 /*
async•fn•g(((ref•a,•ref•mut•b),•(ref•mut•c,•ref•d)):•((A,•A),•(A,•A)))•{}    FunctionDeclaration{async}
          (((ref•a,•ref•mut•b),•(ref•mut•c,•ref•d)):•((A,•A),•(A,•A)))       FunctionDeclaration.parameters{dk: "()"}
           ((ref•a,•ref•mut•b),•(ref•mut•c,•ref•d)):•((A,•A),•(A,•A))        FunctionParameterDeclaration
           ((ref•a,•ref•mut•b),•(ref•mut•c,•ref•d))                          TuplePattern
            (ref•a,•ref•mut•b)                                               TuplePattern
             ref•a                                                           PatternVariableDeclaration{ref, !mut}
                    ref•mut•b                                                PatternVariableDeclaration{ref, mut}
                                (ref•mut•c,•ref•d)                           TuplePattern
                                 ref•mut•c                                   PatternVariableDeclaration{ref, mut}
                                            ref•d                            PatternVariableDeclaration{ref, !mut}
                                                     ((A,•A),•(A,•A))        TypeTuple
                                                      (A,•A)                 TypeTuple
                                                              (A,•A)         TypeTuple
                                                                       {}    FunctionDeclaration.body{dk: "{}"}                           */
pub unsafe extern "C" fn bar(_: i32, mut ap: ...) -> usize {}                                                                             /*
pub•unsafe•extern•"C"•fn•bar(_:•i32,•mut•ap:•...)•->•usize•{}    FunctionDeclaration{unsafe}
pub                                                              PubSpecifier
           extern•"C"                                            ExternSpecifier
                  "C"                                            Literal{kind: String}
                            (_:•i32,•mut•ap:•...)                FunctionDeclaration.parameters{dk: "()"}
                             _:•i32                              FunctionParameterDeclaration
                             _                                   WildcardPattern
                                     mut•ap:•...                 FunctionParameterDeclaration
                                     mut•ap                      PatternVariableDeclaration{!ref, mut}
                                             ...                 FunctionSpread
                                                           {}    FunctionDeclaration.body{dk: "{}"}                                       */
unsafe fn f(&self, x: &usize) { *self + *x; }                                                                                             /*
unsafe•fn•f(&self,•x:•&usize)•{•*self•+•*x;•}    FunctionDeclaration{unsafe}
           (&self,•x:•&usize)                    FunctionDeclaration.parameters{dk: "()"}
            &self                                FunctionSelfParameterDeclaration{ref, !mut}
                   x:•&usize                     FunctionParameterDeclaration
                      &usize                     TypeReference{!mut}
                              {•*self•+•*x;•}    FunctionDeclaration.body{dk: "{}"}
                                *self•+•*x;      ExpressionStatement{semi}
                                *self•+•*x       OperationExpression{tk: "+"}
                                *self            DereferenceExpression
                                        *x       DereferenceExpression                                                                    */
fn f<B:?Sized+Q>() { }                                                                                                                    /*
fn•f<B:?Sized+Q>()•{•}    FunctionDeclaration
    <B:?Sized+Q>          FunctionDeclaration.generics{dk: "<>"}
     B:?Sized+Q           GenericTypeParameterDeclaration
       ?Sized             TypeTraitBound{!maybeConst, optional}
              Q           TypeTraitBound{!maybeConst, !optional}
                ()        FunctionDeclaration.parameters{dk: "()"}
                   {•}    FunctionDeclaration.body{dk: "{}"}                                                                              */
fn f<A,F:for<'a> F<(&'a A,)>>(_: F) { }                                                                                                   /*
fn•f<A,F:for<'a>•F<(&'a•A,)>>(_:•F)•{•}    FunctionDeclaration
    <A,F:for<'a>•F<(&'a•A,)>>              FunctionDeclaration.generics{dk: "<>"}
     A                                     GenericTypeParameterDeclaration
       F:for<'a>•F<(&'a•A,)>               GenericTypeParameterDeclaration
         for<'a>•F<(&'a•A,)>               TypeTraitBound{!maybeConst, !optional}
         for<'a>                           TypeTraitBound.ltParameters{dk: "<>"}
             'a                            GenericLtParameterDeclaration, LtIdentifier
                 F<(&'a•A,)>               TypeCall
                  <(&'a•A,)>               TypeCall.typeArguments{dk: "<>"}
                   (&'a•A,)                TypeTuple
                    &'a•A                  TypeReference{!mut}
                     'a                    LtIdentifier
                             (_:•F)        FunctionDeclaration.parameters{dk: "()"}
                              _:•F         FunctionParameterDeclaration
                              _            WildcardPattern
                                    {•}    FunctionDeclaration.body{dk: "{}"}                                                             */
fn f<M>(a: M) where M: A, M::B: C, {}                                                                                                     /*
fn•f<M>(a:•M)•where•M:•A,•M::B:•C,•{}    FunctionDeclaration
    <M>                                  FunctionDeclaration.generics{dk: "<>"}
     M                                   GenericTypeParameterDeclaration
       (a:•M)                            FunctionDeclaration.parameters{dk: "()"}
        a:•M                             FunctionParameterDeclaration
              where•M:•A,•M::B:•C,       FunctionDeclaration.whereBounds{dk: "None"}
                    M:•A                 WhereTypeBoundDeclaration
                       A                 TypeTraitBound{!maybeConst, !optional}
                          M::B:•C        WhereTypeBoundDeclaration
                          M::B           TypePath
                                C        TypeTraitBound{!maybeConst, !optional}
                                   {}    FunctionDeclaration.body{dk: "{}"}                                                               */
fn f<T,A>(t: fn(&A)) where fn(&A) : for<'a> F<(&'a A,)>,{}                                                                                /*
fn•f<T,A>(t:•fn(&A))•where•fn(&A)•:•for<'a>•F<(&'a•A,)>,{}    FunctionDeclaration
    <T,A>                                                     FunctionDeclaration.generics{dk: "<>"}
     T                                                        GenericTypeParameterDeclaration
       A                                                      GenericTypeParameterDeclaration
         (t:•fn(&A))                                          FunctionDeclaration.parameters{dk: "()"}
          t:•fn(&A)                                           FunctionParameterDeclaration
             fn(&A)                                           TypeFnPointer
               (&A)                                           TypeFnPointer.parameters{dk: "()"}
                &A                                            TypeFnPointerParameter, TypeReference{!mut}
                     where•fn(&A)•:•for<'a>•F<(&'a•A,)>,      FunctionDeclaration.whereBounds{dk: "None"}
                           fn(&A)•:•for<'a>•F<(&'a•A,)>       WhereTypeBoundDeclaration
                           fn(&A)                             TypeFnPointer
                             (&A)                             TypeFnPointer.parameters{dk: "()"}
                              &A                              TypeFnPointerParameter, TypeReference{!mut}
                                    for<'a>•F<(&'a•A,)>       TypeTraitBound{!maybeConst, !optional}
                                    for<'a>                   TypeTraitBound.ltParameters{dk: "<>"}
                                        'a                    GenericLtParameterDeclaration, LtIdentifier
                                            F<(&'a•A,)>       TypeCall
                                             <(&'a•A,)>       TypeCall.typeArguments{dk: "<>"}
                                              (&'a•A,)        TypeTuple
                                               &'a•A          TypeReference{!mut}
                                                'a            LtIdentifier
                                                        {}    FunctionDeclaration.body{dk: "{}"}                                          */
#[no_mangle]                                                                                                                              /*
#[no_mangle]↲    <FunctionDeclaration>
#[no_mangle]     Attribute{!inner}
 [no_mangle]     Attribute.segments{dk: "[]"}                                                                                             */
pub extern "C" fn rust_no_mangle() -> i32 {}                                                                                              /*
pub•extern•"C"•fn•rust_no_mangle()•->•i32•{}    FunctionDeclaration~ownStart
pub                                             PubSpecifier
    extern•"C"                                  ExternSpecifier
           "C"                                  Literal{kind: String}
                                ()              FunctionDeclaration.parameters{dk: "()"}
                                          {}    FunctionDeclaration.body{dk: "{}"}
pub•extern•"C"•fn•rust_no_mangle()•->•i32•{}    </FunctionDeclaration>                                                                    */
pub fn foo<'a, 'b>(x: Foo<'a, 'b>, _o: Option<&   &   ()>) { let _y = x.foo; }                                                            /*
pub•fn•foo<'a,•'b>(x:•Foo<'a,•'b>,•_o:•Option<&•••&•••()>)•{•let•_y•=•x.foo;•}    FunctionDeclaration
pub                                                                               PubSpecifier
          <'a,•'b>                                                                FunctionDeclaration.generics{dk: "<>"}
           'a                                                                     GenericLtParameterDeclaration, LtIdentifier
               'b                                                                 GenericLtParameterDeclaration, LtIdentifier
                  (x:•Foo<'a,•'b>,•_o:•Option<&•••&•••()>)                        FunctionDeclaration.parameters{dk: "()"}
                   x:•Foo<'a,•'b>                                                 FunctionParameterDeclaration
                      Foo<'a,•'b>                                                 TypeCall
                         <'a,•'b>                                                 TypeCall.typeArguments{dk: "<>"}
                          'a                                                      LtIdentifier
                              'b                                                  LtIdentifier
                                   _o:•Option<&•••&•••()>                         FunctionParameterDeclaration
                                       Option<&•••&•••()>                         TypeCall
                                             <&•••&•••()>                         TypeCall.typeArguments{dk: "<>"}
                                              &•••&•••()                          TypeReference{!mut}
                                                  &•••()                          TypeReference{!mut}
                                                      ()                          TypeTuple
                                                           {•let•_y•=•x.foo;•}    FunctionDeclaration.body{dk: "{}"}
                                                             let•_y•=•x.foo;      LetVariableDeclaration
                                                                      x.foo       MemberExpression{!computed}                             */
const x: &'static dyn Fn() = &|| e!("q");                                                                                                 /*
const•x:•&'static•dyn•Fn()•=•&||•e!("q");    ConstVariableDeclaration
         &'static•dyn•Fn()                   TypeReference{!mut}
          'static                            LtStatic
                  dyn•Fn()                   TypeDynBounds{dyn}
                      Fn()                   TypeTraitBound{!maybeConst, !optional}, TypeFunction
                        ()                   TypeFunction.parameters{dk: "()"}
                             &||•e!("q")     ReferenceExpression{!mut}
                              ||•e!("q")     ClosureFunctionExpression
                              ||             ClosureFunctionExpression.parameters{dk: "||"}
                                 e!("q")     MacroInvocation
                                   ("q")     MacroInvocation.segments{dk: "()"}
                                    "q"      Literal{kind: String}                                                                        */
const fn foo() -> i32 {}                                                                                                                  /*
const•fn•foo()•->•i32•{}    FunctionDeclaration{const}
            ()              FunctionDeclaration.parameters{dk: "()"}
                      {}    FunctionDeclaration.body{dk: "{}"}                                                                            */
extern "\x43" fn foo() {}                                                                                                                 /*
extern•"\x43"•fn•foo()•{}    FunctionDeclaration
extern•"\x43"                ExternSpecifier
       "\x43"                Literal{kind: String}
                    ()       FunctionDeclaration.parameters{dk: "()"}
                       {}    FunctionDeclaration.body{dk: "{}"}                                                                           */

extern "\x43" {                                                                                                                           /*
extern•"\x43"•{↲    <ExternBlockDeclaration>
       "\x43"       Literal{kind: String}
              {↲    <ExternBlockDeclaration.body{dk: "{}"}>                                                                               */
    fn bar();                                                                                                                             /*
    fn•bar();    FunctionDeclaration
          ()     FunctionDeclaration.parameters{dk: "()"}                                                                                 */
}                                                                                                                                         /*
}    </ExternBlockDeclaration.body>
}    </ExternBlockDeclaration>                                                                                                            */

type T = extern "\x43" fn();                                                                                                              /*
type•T•=•extern•"\x43"•fn();    TypeAliasDeclaration
         extern•"\x43"•fn()     TypeFnPointer
         extern•"\x43"          ExternSpecifier
                "\x43"          Literal{kind: String}
                         ()     TypeFnPointer.parameters{dk: "()"}                                                                        */
extern r#"C"# fn foo() {}                                                                                                                 /*
extern•r#"C"#•fn•foo()•{}    FunctionDeclaration
extern•r#"C"#                ExternSpecifier
       r#"C"#                Literal{kind: rString}
                    ()       FunctionDeclaration.parameters{dk: "()"}
                       {}    FunctionDeclaration.body{dk: "{}"}                                                                           */

extern r#"C"# {                                                                                                                           /*
extern•r#"C"#•{↲    <ExternBlockDeclaration>
       r#"C"#       Literal{kind: rString}
              {↲    <ExternBlockDeclaration.body{dk: "{}"}>                                                                               */
    fn bar();                                                                                                                             /*
    fn•bar();    FunctionDeclaration
          ()     FunctionDeclaration.parameters{dk: "()"}                                                                                 */
}                                                                                                                                         /*
}    </ExternBlockDeclaration.body>
}    </ExternBlockDeclaration>                                                                                                            */

type T = extern r#"C"# fn();                                                                                                              /*
type•T•=•extern•r#"C"#•fn();    TypeAliasDeclaration
         extern•r#"C"#•fn()     TypeFnPointer
         extern•r#"C"#          ExternSpecifier
                r#"C"#          Literal{kind: rString}
                         ()     TypeFnPointer.parameters{dk: "()"}
type•T•=•extern•r#"C"#•fn();    </Program.ast>
type•T•=•extern•r#"C"#•fn();    </Program>                                                                                                */
// Discarded Nodes: 0
// Parsed Nodes: 681
// state_rollbacks: 44
// Total '.charCodeAt()' calls: 4272 (42% re-reads)
// Unnecessary 'skip_whitespace()' calls: 454
// source: "../../samples/statements/statements.rs"