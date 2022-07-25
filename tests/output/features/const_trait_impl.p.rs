#![feature(const_trait_impl)]                                                                                                             /*
#![feature(const_trait_impl)]    Attribute
          (const_trait_impl)     DelimGroup                                                                                               */

impl const T for S {}                                                                                                                     /*
impl•const•T•for•S•{}    ImplDeclaration                                                                                                  */
impl<T> const T for S {}                                                                                                                  /*
impl<T>•const•T•for•S•{}    ImplDeclaration
     T                      GenericTypeParameterDeclaration                                                                               */


impl const T for S {                                                                                                                      /*
impl•const•T•for•S•{↲    <ImplDeclaration>                                                                                                */
	#![attr]                                                                                                                              /*
    #![attr]    Attribute                                                                                                                 */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */

#[attr_0]                                                                                                                                 /*
#[attr_0]↲    <ImplDeclaration>
#[attr_0]     Attribute                                                                                                                   */
impl const T for S {
	#![attr_1] // comment
                                                                                                                                          /*
    #![attr_1]               Attribute
               //•comment    Comment                                                                                                      */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */

fn foo() -> u8 where Self: ~const Bar {}                                                                                                  /*
fn•foo()•->•u8•where•Self:•~const•Bar•{}    FunctionDeclaration
                     Self:•~const•Bar       WhereTypeBoundDeclaration
                           ~const•Bar       TypeTraitBound                                                                                */
struct S {                                                                                                                                /*
struct•S•{↲    <StructDeclaration>                                                                                                        */
    D: ~const ?Q,                                                                                                                         /*
    D:•~const•?Q     StructPropertyDeclaration
       ~const•?Q     TypeDynBounds, TypeTraitBound                                                                                        */
    E: ~const Q + 'a,                                                                                                                     /*
    E:•~const•Q•+•'a     StructPropertyDeclaration
       ~const•Q•+•'a     TypeDynBounds
       ~const•Q          TypeTraitBound
                  'a     LtIdentifier                                                                                                     */
    F: ~const Q,                                                                                                                          /*
    F:•~const•Q     StructPropertyDeclaration
       ~const•Q     TypeDynBounds, TypeTraitBound                                                                                         */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */

struct S<                                                                                                                                 /*
struct•S<↲    <StructDeclaration>                                                                                                         */
    T: ~const ?for<'a> Tr<'a> + 'static + ~const std::ops::Add,                                                                           /*
    T:•~const•?for<'a>•Tr<'a>•+•'static•+•~const•std::ops::Add     GenericTypeParameterDeclaration
       ~const•?for<'a>•Tr<'a>                                      TypeTraitBound
                   'a                                              GenericLtParameterDeclaration, LtIdentifier
                       Tr<'a>                                      TypeCall
                          'a                                       LtIdentifier
                                'static                            LtStatic
                                          ~const•std::ops::Add     TypeTraitBound
                                                 std::ops::Add     TypePath
                                                 std::ops          TypePath                                                               */
    T: ~const ?for<'a: 'b> m::Trait<'a>,                                                                                                  /*
    T:•~const•?for<'a:•'b>•m::Trait<'a>     GenericTypeParameterDeclaration
       ~const•?for<'a:•'b>•m::Trait<'a>     TypeTraitBound
                   'a:•'b                   GenericLtParameterDeclaration
                   'a                       LtIdentifier
                       'b                   LtIdentifier
                           m::Trait<'a>     TypeCall
                           m::Trait         TypePath
                                    'a      LtIdentifier                                                                                  */
>;                                                                                                                                        /*
>;    </StructDeclaration>                                                                                                                */
trait F {                                                                                                                                 /*
trait•F•{↲    <TraitDeclaration>                                                                                                          */
	fn bar() where Self: ~const Foo;                                                                                                      /*
    fn•bar()•where•Self:•~const•Foo;    FunctionDeclaration
                   Self:•~const•Foo     WhereTypeBoundDeclaration
                         ~const•Foo     TypeTraitBound                                                                                    */
    fn c<T: ~const Bar>();                                                                                                                /*
    fn•c<T:•~const•Bar>();    FunctionDeclaration
         T:•~const•Bar        GenericTypeParameterDeclaration
            ~const•Bar        TypeTraitBound                                                                                              */
}                                                                                                                                         /*
}    </TraitDeclaration>                                                                                                                  */
const fn qux<T: ~const Foo>() {}                                                                                                          /*
const•fn•qux<T:•~const•Foo>()•{}    FunctionDeclaration
             T:•~const•Foo          GenericTypeParameterDeclaration
                ~const•Foo          TypeTraitBound                                                                                        */
const fn test1<T: ~const Foo + Bar>(){}                                                                                                   /*
const•fn•test1<T:•~const•Foo•+•Bar>(){}    FunctionDeclaration
               T:•~const•Foo•+•Bar         GenericTypeParameterDeclaration
                  ~const•Foo               TypeTraitBound
                               Bar         TypeTraitBound                                                                                 */
const fn test2<T: ~const Foo + ~const Bar>() {}                                                                                           /*
const•fn•test2<T:•~const•Foo•+•~const•Bar>()•{}    FunctionDeclaration
               T:•~const•Foo•+•~const•Bar          GenericTypeParameterDeclaration
                  ~const•Foo                       TypeTraitBound
                               ~const•Bar          TypeTraitBound                                                                         */

// Discarded Nodes: 0
// Parsed Nodes: 116
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 821 (38% re-reads)
// Unnecessary 'skip_whitespace()' calls: 75
// source: "../../samples/features/const_trait_impl.rs"