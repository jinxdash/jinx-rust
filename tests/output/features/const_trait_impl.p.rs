#![feature(const_trait_impl)]                                                                                                             /*
#![feature(const_trait_impl)]↲    <Program>
#![feature(const_trait_impl)]     Attribute{inner}
  [feature(const_trait_impl)]     Attribute.segments{dk: "[]"}
          (const_trait_impl)      DelimGroup                                                                                              */

impl const T for S {}                                                                                                                     /*
impl•const•T•for•S•{}↲    <Program.ast{dk: "None"}>
impl•const•T•for•S•{}     ImplDeclaration{const}
                   {}     ImplDeclaration.body{dk: "{}"}                                                                                  */
impl<T> const T for S {}                                                                                                                  /*
impl<T>•const•T•for•S•{}    ImplDeclaration{const}
    <T>                     ImplDeclaration.generics{dk: "<>"}
     T                      GenericTypeParameterDeclaration
                      {}    ImplDeclaration.body{dk: "{}"}                                                                                */


impl const T for S {                                                                                                                      /*
impl•const•T•for•S•{↲    <ImplDeclaration{const}>
                   {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                 */
	#![attr]                                                                                                                              /*
	#![attr]    Attribute{inner}
	  [attr]    Attribute.segments{dk: "[]"}                                                                                              */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */

#[attr_0]                                                                                                                                 /*
#[attr_0]↲    <ImplDeclaration{const}>
#[attr_0]     Attribute{!inner}
 [attr_0]     Attribute.segments{dk: "[]"}                                                                                                */
impl const T for S {                                                                                                                      /*
impl•const•T•for•S•{↲    ImplDeclaration~ownStart
                   {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                 */
	#![attr_1] // comment
/*	#![attr_1]               Attribute{inner}
	  [attr_1]               Attribute.segments{dk: "[]"}                                                                                 */
	           //•comment    Comment{line}
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */

fn foo() -> u8 where Self: ~const Bar {}                                                                                                  /*
fn•foo()•->•u8•where•Self:•~const•Bar•{}    FunctionDeclaration
      ()                                    FunctionDeclaration.parameters{dk: "()"}
               where•Self:•~const•Bar       FunctionDeclaration.whereBounds{dk: "None"}
                     Self:•~const•Bar       WhereTypeBoundDeclaration
                           ~const•Bar       TypeTraitBound{maybeConst, !optional}
                                      {}    FunctionDeclaration.body{dk: "{}"}                                                            */
struct S {                                                                                                                                /*
struct•S•{↲    <StructDeclaration>
         {↲    <StructDeclaration.properties{dk: "{}"}>                                                                                   */
    D: ~const ?Q,                                                                                                                         /*
    D:•~const•?Q    StructPropertyDeclaration
       ~const•?Q    TypeDynBounds{!dyn}, TypeTraitBound{maybeConst, optional}                                                             */
    E: ~const Q + 'a,                                                                                                                     /*
    E:•~const•Q•+•'a    StructPropertyDeclaration
       ~const•Q•+•'a    TypeDynBounds{!dyn}
       ~const•Q         TypeTraitBound{maybeConst, !optional}
                  'a    LtIdentifier                                                                                                      */
    F: ~const Q,                                                                                                                          /*
    F:•~const•Q    StructPropertyDeclaration
       ~const•Q    TypeDynBounds{!dyn}, TypeTraitBound{maybeConst, !optional}                                                             */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */

struct S<                                                                                                                                 /*
struct•S<↲    <StructDeclaration>
        <↲    <StructDeclaration.generics{dk: "<>"}>                                                                                      */
    T: ~const ?for<'a> Tr<'a> + 'static + ~const std::ops::Add,                                                                           /*
    T:•~const•?for<'a>•Tr<'a>•+•'static•+•~const•std::ops::Add    GenericTypeParameterDeclaration
       ~const•?for<'a>•Tr<'a>                                     TypeTraitBound{maybeConst, optional}
               for<'a>                                            TypeTraitBound.ltParameters{dk: "<>"}
                   'a                                             GenericLtParameterDeclaration, LtIdentifier
                       Tr<'a>                                     TypeCall
                         <'a>                                     TypeCall.typeArguments{dk: "<>"}
                          'a                                      LtIdentifier
                                'static                           LtStatic
                                          ~const•std::ops::Add    TypeTraitBound{maybeConst, !optional}
                                                 std::ops::Add    TypePath
                                                 std::ops         TypePath                                                                */
    T: ~const ?for<'a: 'b> m::Trait<'a>,                                                                                                  /*
    T:•~const•?for<'a:•'b>•m::Trait<'a>    GenericTypeParameterDeclaration
       ~const•?for<'a:•'b>•m::Trait<'a>    TypeTraitBound{maybeConst, optional}
               for<'a:•'b>                 TypeTraitBound.ltParameters{dk: "<>"}
                   'a:•'b                  GenericLtParameterDeclaration
                   'a                      LtIdentifier
                       'b                  LtIdentifier
                           m::Trait<'a>    TypeCall
                           m::Trait        TypePath
                                   <'a>    TypeCall.typeArguments{dk: "<>"}
                                    'a     LtIdentifier                                                                                   */
>;                                                                                                                                        /*
>     </StructDeclaration.generics>
>;    </StructDeclaration>                                                                                                                */
trait F {                                                                                                                                 /*
trait•F•{↲    <TraitDeclaration>
        {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                           */
	fn bar() where Self: ~const Foo;                                                                                                      /*
	fn•bar()•where•Self:•~const•Foo;    FunctionDeclaration
	      ()                            FunctionDeclaration.parameters{dk: "()"}
	         where•Self:•~const•Foo     FunctionDeclaration.whereBounds{dk: "None"}
	               Self:•~const•Foo     WhereTypeBoundDeclaration
	                     ~const•Foo     TypeTraitBound{maybeConst, !optional}                                                             */
    fn c<T: ~const Bar>();                                                                                                                /*
    fn•c<T:•~const•Bar>();    FunctionDeclaration
        <T:•~const•Bar>       FunctionDeclaration.generics{dk: "<>"}
         T:•~const•Bar        GenericTypeParameterDeclaration
            ~const•Bar        TypeTraitBound{maybeConst, !optional}
                       ()     FunctionDeclaration.parameters{dk: "()"}                                                                    */
}                                                                                                                                         /*
}    </TraitDeclaration.body>
}    </TraitDeclaration>                                                                                                                  */
const fn qux<T: ~const Foo>() {}                                                                                                          /*
const•fn•qux<T:•~const•Foo>()•{}    FunctionDeclaration{const}
            <T:•~const•Foo>         FunctionDeclaration.generics{dk: "<>"}
             T:•~const•Foo          GenericTypeParameterDeclaration
                ~const•Foo          TypeTraitBound{maybeConst, !optional}
                           ()       FunctionDeclaration.parameters{dk: "()"}
                              {}    FunctionDeclaration.body{dk: "{}"}                                                                    */
const fn test1<T: ~const Foo + Bar>(){}                                                                                                   /*
const•fn•test1<T:•~const•Foo•+•Bar>(){}    FunctionDeclaration{const}
              <T:•~const•Foo•+•Bar>        FunctionDeclaration.generics{dk: "<>"}
               T:•~const•Foo•+•Bar         GenericTypeParameterDeclaration
                  ~const•Foo               TypeTraitBound{maybeConst, !optional}
                               Bar         TypeTraitBound{!maybeConst, !optional}
                                   ()      FunctionDeclaration.parameters{dk: "()"}
                                     {}    FunctionDeclaration.body{dk: "{}"}                                                             */
const fn test2<T: ~const Foo + ~const Bar>() {}                                                                                           /*
const•fn•test2<T:•~const•Foo•+•~const•Bar>()•{}    FunctionDeclaration{const}
              <T:•~const•Foo•+•~const•Bar>         FunctionDeclaration.generics{dk: "<>"}
               T:•~const•Foo•+•~const•Bar          GenericTypeParameterDeclaration
                  ~const•Foo                       TypeTraitBound{maybeConst, !optional}
                               ~const•Bar          TypeTraitBound{maybeConst, !optional}
                                          ()       FunctionDeclaration.parameters{dk: "()"}
                                             {}    FunctionDeclaration.body{dk: "{}"}
const•fn•test2<T:•~const•Foo•+•~const•Bar>()•{}    </Program.ast>
const•fn•test2<T:•~const•Foo•+•~const•Bar>()•{}    </Program>                                                                             */
// Discarded Nodes: 0
// Parsed Nodes: 116
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 821 (38% re-reads)
// Unnecessary 'skip_whitespace()' calls: 75
// source: "../../samples/features/const_trait_impl.rs"