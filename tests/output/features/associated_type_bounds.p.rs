#![feature(associated_type_bounds)]                                                                                                       /*
#![feature(associated_type_bounds)]↲    <Program>
#![feature(associated_type_bounds)]     Attribute{inner}
  [feature(associated_type_bounds)]     Attribute.segments{dk: "[]"}
          (associated_type_bounds)      DelimGroup                                                                                        */

type X = A<B: C>;                                                                                                                         /*
type•X•=•A<B:•C>;↲    <Program.ast{dk: "None"}>
type•X•=•A<B:•C>;     TypeAliasDeclaration
         A<B:•C>      TypeCall
          <B:•C>      TypeCall.typeArguments{dk: "<>"}
           B:•C       TypeCallNamedBound
              C       TypeTraitBound{!maybeConst, !optional}                                                                              */

fn f<F>(_: F) where F: for<'a> Trait<Output: 'a>, { }                                                                                     /*
fn•f<F>(_:•F)•where•F:•for<'a>•Trait<Output:•'a>,•{•}    FunctionDeclaration
    <F>                                                  FunctionDeclaration.generics{dk: "<>"}
     F                                                   GenericTypeParameterDeclaration
       (_:•F)                                            FunctionDeclaration.parameters{dk: "()"}
        _:•F                                             FunctionParameterDeclaration
        _                                                WildcardPattern
              where•F:•for<'a>•Trait<Output:•'a>,        FunctionDeclaration.whereBounds{dk: "None"}
                    F:•for<'a>•Trait<Output:•'a>         WhereTypeBoundDeclaration
                       for<'a>•Trait<Output:•'a>         TypeTraitBound{!maybeConst, !optional}
                       for<'a>                           TypeTraitBound.ltParameters{dk: "<>"}
                           'a                            GenericLtParameterDeclaration, LtIdentifier
                               Trait<Output:•'a>         TypeCall
                                    <Output:•'a>         TypeCall.typeArguments{dk: "<>"}
                                     Output:•'a          TypeCallNamedBound
                                             'a          LtIdentifier
                                                  {•}    FunctionDeclaration.body{dk: "{}"}                                               */
fn f<'b, F>() where for<'a> F: Iterator<Item: 'a> + 'b, { }                                                                               /*
fn•f<'b,•F>()•where•for<'a>•F:•Iterator<Item:•'a>•+•'b,•{•}    FunctionDeclaration
    <'b,•F>                                                    FunctionDeclaration.generics{dk: "<>"}
     'b                                                        GenericLtParameterDeclaration, LtIdentifier
         F                                                     GenericTypeParameterDeclaration
           ()                                                  FunctionDeclaration.parameters{dk: "()"}
              where•for<'a>•F:•Iterator<Item:•'a>•+•'b,        FunctionDeclaration.whereBounds{dk: "None"}
                    for<'a>•F:•Iterator<Item:•'a>•+•'b         WhereTypeBoundDeclaration
                    for<'a>                                    WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                        'a                                     GenericLtParameterDeclaration, LtIdentifier
                               Iterator<Item:•'a>              TypeTraitBound{!maybeConst, !optional}, TypeCall
                                       <Item:•'a>              TypeCall.typeArguments{dk: "<>"}
                                        Item:•'a               TypeCallNamedBound
                                              'a               LtIdentifier
                                                    'b         LtIdentifier
                                                        {•}    FunctionDeclaration.body{dk: "{}"}                                         */

trait A: MP {                                                                                                                             /*
trait•A:•MP•{↲    <TraitDeclaration>
         MP       TypeTraitBound{!maybeConst, !optional}
            {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                       */
    fn f<IM>(&self) -> i32 where for<'a> IM: T<T: U<<Self as MP>::T<'a>>>;                                                                /*
    fn•f<IM>(&self)•->•i32•where•for<'a>•IM:•T<T:•U<<Self•as•MP>::T<'a>>>;    FunctionDeclaration
        <IM>                                                                  FunctionDeclaration.generics{dk: "<>"}
         IM                                                                   GenericTypeParameterDeclaration
            (&self)                                                           FunctionDeclaration.parameters{dk: "()"}
             &self                                                            FunctionSelfParameterDeclaration{ref, !mut}
                           where•for<'a>•IM:•T<T:•U<<Self•as•MP>::T<'a>>>     FunctionDeclaration.whereBounds{dk: "None"}
                                 for<'a>•IM:•T<T:•U<<Self•as•MP>::T<'a>>>     WhereTypeBoundDeclaration
                                 for<'a>                                      WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                                     'a                                       GenericLtParameterDeclaration, LtIdentifier
                                             T<T:•U<<Self•as•MP>::T<'a>>>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                                              <T:•U<<Self•as•MP>::T<'a>>>     TypeCall.typeArguments{dk: "<>"}
                                               T:•U<<Self•as•MP>::T<'a>>      TypeCallNamedBound
                                                  U<<Self•as•MP>::T<'a>>      TypeTraitBound{!maybeConst, !optional}, TypeCall
                                                   <<Self•as•MP>::T<'a>>      TypeCall.typeArguments{dk: "<>"}
                                                    <Self•as•MP>::T<'a>       TypeCall
                                                    <Self•as•MP>::T           TypePath
                                                    <Self•as•MP>              ExpressionTypeSelector
                                                                   <'a>       TypeCall.typeArguments{dk: "<>"}
                                                                    'a        LtIdentifier                                                */
}                                                                                                                                         /*
}    </TraitDeclaration.body>
}    </TraitDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 77
// state_rollbacks: 7
// Total '.charCodeAt()' calls: 403 (54% re-reads)
// Unnecessary 'skip_whitespace()' calls: 69
// source: "../../samples/features/associated_type_bounds.rs"