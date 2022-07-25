#![feature(associated_type_bounds)]                                                                                                       /*
#![feature(associated_type_bounds)]    Attribute
          (associated_type_bounds)     DelimGroup                                                                                         */

type X = A<B: C>;                                                                                                                         /*
type•X•=•A<B:•C>;    TypeAliasDeclaration
         A<B:•C>     TypeCall
           B:•C      TypeCallNamedBound
              C      TypeTraitBound                                                                                                       */

fn f<F>(_: F) where F: for<'a> Trait<Output: 'a>, { }                                                                                     /*
fn•f<F>(_:•F)•where•F:•for<'a>•Trait<Output:•'a>,•{•}    FunctionDeclaration
     F                                                   GenericTypeParameterDeclaration
        _:•F                                             FunctionParameterDeclaration
        _                                                WildcardPattern
                    F:•for<'a>•Trait<Output:•'a>         WhereTypeBoundDeclaration
                       for<'a>•Trait<Output:•'a>         TypeTraitBound
                           'a                            GenericLtParameterDeclaration, LtIdentifier
                               Trait<Output:•'a>         TypeCall
                                     Output:•'a          TypeCallNamedBound
                                             'a          LtIdentifier                                                                     */
fn f<'b, F>() where for<'a> F: Iterator<Item: 'a> + 'b, { }                                                                               /*
fn•f<'b,•F>()•where•for<'a>•F:•Iterator<Item:•'a>•+•'b,•{•}    FunctionDeclaration
     'b                                                        GenericLtParameterDeclaration, LtIdentifier
         F                                                     GenericTypeParameterDeclaration
                    for<'a>•F:•Iterator<Item:•'a>•+•'b         WhereTypeBoundDeclaration
                        'a                                     GenericLtParameterDeclaration, LtIdentifier
                               Iterator<Item:•'a>              TypeTraitBound, TypeCall
                                        Item:•'a               TypeCallNamedBound
                                              'a               LtIdentifier
                                                    'b         LtIdentifier                                                               */

trait A: MP {                                                                                                                             /*
trait•A:•MP•{↲    <TraitDeclaration>
         MP       TypeTraitBound                                                                                                          */
    fn f<IM>(&self) -> i32 where for<'a> IM: T<T: U<<Self as MP>::T<'a>>>;                                                                /*
    fn•f<IM>(&self)•->•i32•where•for<'a>•IM:•T<T:•U<<Self•as•MP>::T<'a>>>;    FunctionDeclaration
         IM                                                                   GenericTypeParameterDeclaration
             &self                                                            FunctionSelfParameterDeclaration
                                 for<'a>•IM:•T<T:•U<<Self•as•MP>::T<'a>>>     WhereTypeBoundDeclaration
                                     'a                                       GenericLtParameterDeclaration, LtIdentifier
                                             T<T:•U<<Self•as•MP>::T<'a>>>     TypeTraitBound, TypeCall
                                               T:•U<<Self•as•MP>::T<'a>>      TypeCallNamedBound
                                                  U<<Self•as•MP>::T<'a>>      TypeTraitBound, TypeCall
                                                    <Self•as•MP>::T<'a>       TypeCall
                                                    <Self•as•MP>::T           TypePath
                                                    <Self•as•MP>              ExpressionTypeSelector
                                                                    'a        LtIdentifier                                                */
}                                                                                                                                         /*
}    </TraitDeclaration>                                                                                                                  */

// Discarded Nodes: 0
// Parsed Nodes: 77
// state_rollbacks: 7
// Total '.charCodeAt()' calls: 403 (54% re-reads)
// Unnecessary 'skip_whitespace()' calls: 69
// source: "../../samples/features/associated_type_bounds.rs"