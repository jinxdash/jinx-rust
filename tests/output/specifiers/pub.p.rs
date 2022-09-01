enum E {                                                                                                                                  /*
enum•E•{↲    <Program>
enum•E•{↲    <Program.ast{dk: "None"}>
enum•E•{↲    <EnumDeclaration>
       {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                          */
    pub U,                                                                                                                                /*
    pub•U    EnumMemberDeclaration
    pub      PubSpecifier                                                                                                                 */
    pub(crate) T(u8),                                                                                                                     /*
    pub(crate)•T(u8)    EnumMemberTupleDeclaration
    pub(crate)          PubSpecifier
                (u8)    EnumMemberTupleDeclaration.items{dk: "()"}
                 u8     TupleStructItemDeclaration                                                                                        */
    pub(super) T { f: String }                                                                                                            /*
    pub(super)•T•{•f:•String•}    EnumMemberStructDeclaration
    pub(super)                    PubSpecifier
                 {•f:•String•}    EnumMemberStructDeclaration.properties{dk: "{}"}
                   f:•String      StructPropertyDeclaration                                                                               */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
pub impl Tr for S {                                                                                                                       /*
pub•impl•Tr•for•S•{•↲    <ImplDeclaration{!const}>
pub                      PubSpecifier
                  {•↲    <ImplDeclaration.body{dk: "{}"}>                                                                                 */
    pub fn f() {}                                                                                                                         /*
    pub•fn•f()•{}    FunctionDeclaration
    pub              PubSpecifier
            ()       FunctionDeclaration.parameters{dk: "()"}
               {}    FunctionDeclaration.body{dk: "{}"}                                                                                   */
    pub const C: u8 = 0;                                                                                                                  /*
    pub•const•C:•u8•=•0;    ConstVariableDeclaration
    pub                     PubSpecifier
                      0     Literal{kind: Integer}                                                                                        */
    pub type T = u8;                                                                                                                      /*
    pub•type•T•=•u8;    TypeAliasDeclaration
    pub                 PubSpecifier                                                                                                      */
	pub(in foo) fn f(&self) -> i32 { 0 }                                                                                                  /*
	pub(in•foo)•fn•f(&self)•->•i32•{•0•}    FunctionDeclaration
	pub(in•foo)                             PubSpecifier
	                (&self)                 FunctionDeclaration.parameters{dk: "()"}
	                 &self                  FunctionSelfParameterDeclaration{ref, !mut}
	                               {•0•}    FunctionDeclaration.body{dk: "{}"}
	                                 0      ExpressionStatement{!semi}, Literal{kind: Integer}                                            */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */
pub struct Pub(Priv2);                                                                                                                    /*
pub•struct•Pub(Priv2);    TupleStructDeclaration
pub                       PubSpecifier
              (Priv2)     TupleStructDeclaration.items{dk: "()"}
               Priv2      TupleStructItemDeclaration                                                                                      */
mod bar { pub use ::*; }                                                                                                                  /*
mod•bar•{•pub•use•::*;•}    ModuleDeclaration
        {•pub•use•::*;•}    ModuleDeclaration.body{dk: "{}"}
          pub•use•::*;      UseStatement
          pub               PubSpecifier
                  ::*       AmbientImport                                                                                                 */
pub trait Sized {}                                                                                                                        /*
pub•trait•Sized•{}    TraitDeclaration
pub                   PubSpecifier
                {}    TraitDeclaration.body{dk: "{}"}                                                                                     */
const MAIN: u8 = {                                                                                                                        /*
const•MAIN:•u8•=•{↲    <ConstVariableDeclaration>
                 {↲    <BlockExpression>                                                                                                  */
    pub trait Tr {                                                                                                                        /*
    pub•trait•Tr•{↲    <TraitDeclaration>
    pub                PubSpecifier
                 {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                  */
        fn f();                                                                                                                           /*
        fn•f();    FunctionDeclaration
            ()     FunctionDeclaration.parameters{dk: "()"}                                                                               */
        const C: u8;                                                                                                                      /*
        const•C:•u8;    ConstVariableDeclaration                                                                                          */
        type T;                                                                                                                           /*
        type•T;    TypeAliasDeclaration                                                                                                   */
    }                                                                                                                                     /*
••••}    </TraitDeclaration.body>
••••}    </TraitDeclaration>                                                                                                              */
    pub struct S {                                                                                                                        /*
    pub•struct•S•{↲    <StructDeclaration>
    pub                PubSpecifier
                 {↲    <StructDeclaration.properties{dk: "{}"}>                                                                           */
        pub a: u8                                                                                                                         /*
        pub•a:•u8    StructPropertyDeclaration
        pub          PubSpecifier                                                                                                         */
    }                                                                                                                                     /*
••••}    </StructDeclaration.properties>
••••}    </StructDeclaration>                                                                                                             */
    struct Ts(pub u8);                                                                                                                    /*
    struct•Ts(pub•u8);    TupleStructDeclaration
             (pub•u8)     TupleStructDeclaration.items{dk: "()"}
              pub•u8      TupleStructItemDeclaration
              pub         PubSpecifier                                                                                                    */
    pub impl Tr for S {                                                                                                                   /*
    pub•impl•Tr•for•S•{•↲    <ImplDeclaration{!const}>
    pub                      PubSpecifier
                      {•↲    <ImplDeclaration.body{dk: "{}"}>                                                                             */
        pub fn f() {}                                                                                                                     /*
        pub•fn•f()•{}    FunctionDeclaration
        pub              PubSpecifier
                ()       FunctionDeclaration.parameters{dk: "()"}
                   {}    FunctionDeclaration.body{dk: "{}"}                                                                               */
        pub const C: u8 = 0;                                                                                                              /*
        pub•const•C:•u8•=•0;    ConstVariableDeclaration
        pub                     PubSpecifier
                          0     Literal{kind: Integer}                                                                                    */
        pub type T = u8;                                                                                                                  /*
        pub•type•T•=•u8;    TypeAliasDeclaration
        pub                 PubSpecifier                                                                                                  */
    }                                                                                                                                     /*
••••}    </ImplDeclaration.body>
••••}    </ImplDeclaration>                                                                                                               */
    pub impl S {                                                                                                                          /*
    pub•impl•S•{↲    <ImplDeclaration{!const}>
    pub              PubSpecifier
               {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                     */
        pub fn f() {}                                                                                                                     /*
        pub•fn•f()•{}    FunctionDeclaration
        pub              PubSpecifier
                ()       FunctionDeclaration.parameters{dk: "()"}
                   {}    FunctionDeclaration.body{dk: "{}"}                                                                               */
        pub const C: u8 = 0;                                                                                                              /*
        pub•const•C:•u8•=•0;    ConstVariableDeclaration
        pub                     PubSpecifier
                          0     Literal{kind: Integer}                                                                                    */
        // pub type T = u8;
        //•pub•type•T•=•u8;    Comment{line}
    }                                                                                                                                     /*
••••}    </ImplDeclaration.body>
••••}    </ImplDeclaration>                                                                                                               */
    pub extern "C" {                                                                                                                      /*
    pub•extern•"C"•{↲    <ExternBlockDeclaration>
    pub                  PubSpecifier
               "C"       Literal{kind: String}
                   {↲    <ExternBlockDeclaration.body{dk: "{}"}>                                                                          */
        pub fn f();                                                                                                                       /*
        pub•fn•f();    FunctionDeclaration
        pub            PubSpecifier
                ()     FunctionDeclaration.parameters{dk: "()"}                                                                           */
        pub static St: u8;                                                                                                                /*
        pub•static•St:•u8;    StaticVariableDeclaration
        pub                   PubSpecifier                                                                                                */
    }                                                                                                                                     /*
••••}    </ExternBlockDeclaration.body>
••••}    </ExternBlockDeclaration>                                                                                                        */
    ()                                                                                                                                    /*
    ()    ExpressionStatement{!semi}, TupleLiteral                                                                                        */
};                                                                                                                                        /*
}     </BlockExpression>
};    </ConstVariableDeclaration>                                                                                                         */
pub(super) fn f(_: Priv) {}                                                                                                               /*
pub(super)•fn•f(_:•Priv)•{}    FunctionDeclaration
pub(super)                     PubSpecifier
               (_:•Priv)       FunctionDeclaration.parameters{dk: "()"}
                _:•Priv        FunctionParameterDeclaration
                _              WildcardPattern
                         {}    FunctionDeclaration.body{dk: "{}"}                                                                         */
pub(crate) fn g(_: Priv) {}                                                                                                               /*
pub(crate)•fn•g(_:•Priv)•{}    FunctionDeclaration
pub(crate)                     PubSpecifier
               (_:•Priv)       FunctionDeclaration.parameters{dk: "()"}
                _:•Priv        FunctionParameterDeclaration
                _              WildcardPattern
                         {}    FunctionDeclaration.body{dk: "{}"}                                                                         */
crate fn h(_: Priv) {}                                                                                                                    /*
crate•fn•h(_:•Priv)•{}    FunctionDeclaration
crate                     PubSpecifier
          (_:•Priv)       FunctionDeclaration.parameters{dk: "()"}
           _:•Priv        FunctionParameterDeclaration
           _              WildcardPattern
                    {}    FunctionDeclaration.body{dk: "{}"}                                                                              */
pub(in crate) struct S1;                                                                                                                  /*
pub(in•crate)•struct•S1;    StructDeclaration
pub(in•crate)               PubSpecifier                                                                                                  */
pub(in super) struct S2;                                                                                                                  /*
pub(in•super)•struct•S2;    StructDeclaration
pub(in•super)               PubSpecifier                                                                                                  */
pub(in self) struct S3;                                                                                                                   /*
pub(in•self)•struct•S3;    StructDeclaration
pub(in•self)               PubSpecifier                                                                                                   */
pub(in ::core) struct S4;                                                                                                                 /*
pub(in•::core)•struct•S4;    StructDeclaration
pub(in•::core)               PubSpecifier
       ::core                ItemPath                                                                                                     */
pub(in a::b) struct S5;                                                                                                                   /*
pub(in•a::b)•struct•S5;    StructDeclaration
pub(in•a::b)               PubSpecifier
       a::b                ItemPath                                                                                                       */
pub type A;                                                                                                                               /*
pub•type•A;    TypeAliasDeclaration
pub            PubSpecifier                                                                                                               */
pub static b: Q;                                                                                                                          /*
pub•static•b:•Q;    StaticVariableDeclaration
pub                 PubSpecifier                                                                                                          */
pub extern crate core;                                                                                                                    /*
pub•extern•crate•core;    ExternCrateStatement
pub                       PubSpecifier
                 core     NamedImport                                                                                                     */
struct Bar(pub(()));                                                                                                                      /*
struct•Bar(pub(()));    TupleStructDeclaration
          (pub(()))     TupleStructDeclaration.items{dk: "()"}
           pub(())      TupleStructItemDeclaration
           pub          PubSpecifier
               ()       TypeTuple                                                                                                         */
pub struct C(pub isize, isize);                                                                                                           /*
pub•struct•C(pub•isize,•isize);    TupleStructDeclaration
pub                                PubSpecifier
            (pub•isize,•isize)     TupleStructDeclaration.items{dk: "()"}
             pub•isize             TupleStructItemDeclaration
             pub                   PubSpecifier
                        isize      TupleStructItemDeclaration                                                                             */
pub struct D(pub isize);                                                                                                                  /*
pub•struct•D(pub•isize);    TupleStructDeclaration
pub                         PubSpecifier
            (pub•isize)     TupleStructDeclaration.items{dk: "()"}
             pub•isize      TupleStructItemDeclaration
             pub            PubSpecifier                                                                                                  */
pub struct bool;                                                                                                                          /*
pub•struct•bool;    StructDeclaration
pub                 PubSpecifier                                                                                                          */
pub struct Pub<T = Alias>(pub T);                                                                                                         /*
pub•struct•Pub<T•=•Alias>(pub•T);    TupleStructDeclaration
pub                                  PubSpecifier
              <T•=•Alias>            TupleStructDeclaration.generics{dk: "<>"}
               T•=•Alias             GenericTypeParameterDeclaration
                         (pub•T)     TupleStructDeclaration.items{dk: "()"}
                          pub•T      TupleStructItemDeclaration
                          pub        PubSpecifier                                                                                         */
pub type A;                                                                                                                               /*
pub•type•A;    TypeAliasDeclaration
pub            PubSpecifier                                                                                                               */
pub mod bar {                                                                                                                             /*
pub•mod•bar•{↲    <ModuleDeclaration>
pub               PubSpecifier
            {↲    <ModuleDeclaration.body{dk: "{}"}>                                                                                      */
	pub use a::b::c;                                                                                                                      /*
	pub•use•a::b::c;    UseStatement
	pub                 PubSpecifier
	        a::b::c     NamedImport, ItemPath
	        a::b        ItemPath                                                                                                          */
	pub mod b {}                                                                                                                          /*
	pub•mod•b•{}    ModuleDeclaration
	pub             PubSpecifier
	          {}    ModuleDeclaration.body{dk: "{}"}                                                                                      */
	pub struct S {                                                                                                                        /*
	pub•struct•S•{↲    <StructDeclaration>
	pub                PubSpecifier
	             {↲    <StructDeclaration.properties{dk: "{}"}>                                                                           */
		pub(in foo) x: i32,                                                                                                               /*
		pub(in•foo)•x:•i32    StructPropertyDeclaration
		pub(in•foo)           PubSpecifier                                                                                                */
	}                                                                                                                                     /*
   ╚}    </StructDeclaration.properties>
   ╚}    </StructDeclaration>                                                                                                             */
}                                                                                                                                         /*
}    </ModuleDeclaration.body>
}    </ModuleDeclaration>                                                                                                                 */
pub macro m() {}                                                                                                                          /*
pub•macro•m()•{}    MacroDeclaration
pub                 PubSpecifier
           ()•{}    MacroInlineRuleDeclaration
           ()       MacroInlineRuleDeclaration.match{dk: "()"}
              {}    MacroInlineRuleDeclaration.transform{dk: "{}"}                                                                        */
pub(in Self::f) struct Z;                                                                                                                 /*
pub(in•Self::f)•struct•Z;    StructDeclaration
pub(in•Self::f)              PubSpecifier
       Self::f               ItemPath                                                                                                     */
pub extern crate self as name;                                                                                                            /*
pub•extern•crate•self•as•name;    ExternCrateStatement
pub                               PubSpecifier
                 self•as•name     NamedImport                                                                                             */
pub use name::name as bug;                                                                                                                /*
pub•use•name::name•as•bug;    UseStatement
pub                           PubSpecifier
        name::name•as•bug     NamedImport
        name::name            ItemPath
pub•use•name::name•as•bug;    </Program.ast>
pub•use•name::name•as•bug;    </Program>                                                                                                  */
// Discarded Nodes: 1
// Parsed Nodes: 256
// state_rollbacks: 9
// Total '.charCodeAt()' calls: 1807 (38% re-reads)
// Unnecessary 'skip_whitespace()' calls: 115
// source: "../../samples/specifiers/pub.rs"