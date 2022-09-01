trait A {}                                                                                                                                /*
trait•A•{}↲    <Program>
trait•A•{}↲    <Program.ast{dk: "None"}>
trait•A•{}     TraitDeclaration
        {}     TraitDeclaration.body{dk: "{}"}                                                                                            */
trait A<B:C>{}                                                                                                                            /*
trait•A<B:C>{}    TraitDeclaration
       <B:C>      TraitDeclaration.generics{dk: "<>"}
        B:C       GenericTypeParameterDeclaration
          C       TypeTraitBound{!maybeConst, !optional}
            {}    TraitDeclaration.body{dk: "{}"}                                                                                         */
trait T2<'x, 'y> : T1<'x> {}                                                                                                              /*
trait•T2<'x,•'y>•:•T1<'x>•{}    TraitDeclaration
        <'x,•'y>                TraitDeclaration.generics{dk: "<>"}
         'x                     GenericLtParameterDeclaration, LtIdentifier
             'y                 GenericLtParameterDeclaration, LtIdentifier
                   T1<'x>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                     <'x>       TypeCall.typeArguments{dk: "<>"}
                      'x        LtIdentifier
                          {}    TraitDeclaration.body{dk: "{}"}                                                                           */
trait T<E, R> = S<A> where <Self as B<D>>::T: H<R>;                                                                                       /*
trait•T<E,•R>•=•S<A>•where•<Self•as•B<D>>::T:•H<R>     TraitAliasDeclaration
       <E,•R>                                          TraitAliasDeclaration.generics{dk: "<>"}
        E                                              GenericTypeParameterDeclaration
           R                                           GenericTypeParameterDeclaration
                S<A>                                   TypeTraitBound{!maybeConst, !optional}, TypeCall
                 <A>                                   TypeCall.typeArguments{dk: "<>"}
                     where•<Self•as•B<D>>::T:•H<R>     TraitAliasDeclaration.whereBounds{dk: "None"}
                           <Self•as•B<D>>::T:•H<R>     WhereTypeBoundDeclaration
                           <Self•as•B<D>>::T           TypePath
                           <Self•as•B<D>>              ExpressionTypeSelector
                                    B<D>               TypeCall
                                     <D>               TypeCall.typeArguments{dk: "<>"}
                                              H<R>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                                               <R>     TypeCall.typeArguments{dk: "<>"}
                                                  ;    ExpressionStatement{semi}                                                          */
trait A<B: C>: B<B::S> + E<()> {}                                                                                                         /*
trait•A<B:•C>:•B<B::S>•+•E<()>•{}    TraitDeclaration
       <B:•C>                        TraitDeclaration.generics{dk: "<>"}
        B:•C                         GenericTypeParameterDeclaration
           C                         TypeTraitBound{!maybeConst, !optional}
               B<B::S>               TypeTraitBound{!maybeConst, !optional}, TypeCall
                <B::S>               TypeCall.typeArguments{dk: "<>"}
                 B::S                TypePath
                         E<()>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                          <()>       TypeCall.typeArguments{dk: "<>"}
                           ()        TypeTuple
                               {}    TraitDeclaration.body{dk: "{}"}                                                                      */
trait A<S: B<C=S>>: D<S> {}                                                                                                               /*
trait•A<S:•B<C=S>>:•D<S>•{}    TraitDeclaration
       <S:•B<C=S>>             TraitDeclaration.generics{dk: "<>"}
        S:•B<C=S>              GenericTypeParameterDeclaration
           B<C=S>              TypeTraitBound{!maybeConst, !optional}, TypeCall
            <C=S>              TypeCall.typeArguments{dk: "<>"}
             C=S               TypeCallNamedArgument
                    D<S>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                     <S>       TypeCall.typeArguments{dk: "<>"}
                         {}    TraitDeclaration.body{dk: "{}"}                                                                            */
trait A = B<C=D>;                                                                                                                         /*
trait•A•=•B<C=D>     TraitAliasDeclaration
          B<C=D>     TypeTraitBound{!maybeConst, !optional}, TypeCall
           <C=D>     TypeCall.typeArguments{dk: "<>"}
            C=D      TypeCallNamedArgument
                ;    ExpressionStatement{semi}                                                                                            */
trait A<T> = B where T: C<D>;                                                                                                             /*
trait•A<T>•=•B•where•T:•C<D>     TraitAliasDeclaration
       <T>                       TraitAliasDeclaration.generics{dk: "<>"}
        T                        GenericTypeParameterDeclaration
             B                   TypeTraitBound{!maybeConst, !optional}
               where•T:•C<D>     TraitAliasDeclaration.whereBounds{dk: "None"}
                     T:•C<D>     WhereTypeBoundDeclaration
                        C<D>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                         <D>     TypeCall.typeArguments{dk: "<>"}
                            ;    ExpressionStatement{semi}                                                                                */
trait A: B<B = u32> + C<Self::A> {}                                                                                                       /*
trait•A:•B<B•=•u32>•+•C<Self::A>•{}    TraitDeclaration
         B<B•=•u32>                    TypeTraitBound{!maybeConst, !optional}, TypeCall
          <B•=•u32>                    TypeCall.typeArguments{dk: "<>"}
           B•=•u32                     TypeCallNamedArgument
                      C<Self::A>       TypeTraitBound{!maybeConst, !optional}, TypeCall
                       <Self::A>       TypeCall.typeArguments{dk: "<>"}
                        Self::A        TypePath
                                 {}    TraitDeclaration.body{dk: "{}"}                                                                    */
trait A = std::fmt::Display + std::fmt::Debug;                                                                                            /*
trait•A•=•std::fmt::Display•+•std::fmt::Debug     TraitAliasDeclaration
          std::fmt::Display                       TypeTraitBound{!maybeConst, !optional}, TypePath
          std::fmt                                TypePath
                              std::fmt::Debug     TypeTraitBound{!maybeConst, !optional}, TypePath
                              std::fmt            TypePath
                                             ;    ExpressionStatement{semi}                                                               */
trait B = std::fmt::Display + std::fmt::Debug;                                                                                            /*
trait•B•=•std::fmt::Display•+•std::fmt::Debug     TraitAliasDeclaration
          std::fmt::Display                       TypeTraitBound{!maybeConst, !optional}, TypePath
          std::fmt                                TypePath
                              std::fmt::Debug     TypeTraitBound{!maybeConst, !optional}, TypePath
                              std::fmt            TypePath
                                             ;    ExpressionStatement{semi}                                                               */
trait A = Default;                                                                                                                        /*
trait•A•=•Default     TraitAliasDeclaration
          Default     TypeTraitBound{!maybeConst, !optional}
                 ;    ExpressionStatement{semi}                                                                                           */
trait A<T> = B<C = D>;                                                                                                                    /*
trait•A<T>•=•B<C•=•D>     TraitAliasDeclaration
       <T>                TraitAliasDeclaration.generics{dk: "<>"}
        T                 GenericTypeParameterDeclaration
             B<C•=•D>     TypeTraitBound{!maybeConst, !optional}, TypeCall
              <C•=•D>     TypeCall.typeArguments{dk: "<>"}
               C•=•D      TypeCallNamedArgument
                     ;    ExpressionStatement{semi}                                                                                       */
trait A = B<C>;                                                                                                                           /*
trait•A•=•B<C>     TraitAliasDeclaration
          B<C>     TypeTraitBound{!maybeConst, !optional}, TypeCall
           <C>     TypeCall.typeArguments{dk: "<>"}
              ;    ExpressionStatement{semi}                                                                                              */
trait A<'a, T: 'a> = B<&'a D>;                                                                                                            /*
trait•A<'a,•T:•'a>•=•B<&'a•D>     TraitAliasDeclaration
       <'a,•T:•'a>                TraitAliasDeclaration.generics{dk: "<>"}
        'a                        GenericLtParameterDeclaration, LtIdentifier
            T:•'a                 GenericTypeParameterDeclaration
               'a                 LtIdentifier
                     B<&'a•D>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                      <&'a•D>     TypeCall.typeArguments{dk: "<>"}
                       &'a•D      TypeReference{!mut}
                        'a        LtIdentifier
                             ;    ExpressionStatement{semi}                                                                               */
trait A = 'static;                                                                                                                        /*
trait•A•=•'static     TraitAliasDeclaration
          'static     LtStatic
                 ;    ExpressionStatement{semi}                                                                                           */
trait A<B, C> = D + E where F<(G, H)>: I;                                                                                                 /*
trait•A<B,•C>•=•D•+•E•where•F<(G,•H)>:•I     TraitAliasDeclaration
       <B,•C>                                TraitAliasDeclaration.generics{dk: "<>"}
        B                                    GenericTypeParameterDeclaration
           C                                 GenericTypeParameterDeclaration
                D                            TypeTraitBound{!maybeConst, !optional}
                    E                        TypeTraitBound{!maybeConst, !optional}
                      where•F<(G,•H)>:•I     TraitAliasDeclaration.whereBounds{dk: "None"}
                            F<(G,•H)>:•I     WhereTypeBoundDeclaration
                            F<(G,•H)>        TypeCall
                             <(G,•H)>        TypeCall.typeArguments{dk: "<>"}
                              (G,•H)         TypeTuple
                                       I     TypeTraitBound{!maybeConst, !optional}
                                        ;    ExpressionStatement{semi}                                                                    */
trait A<B, C> = where D<E>: F<G>;                                                                                                         /*
trait•A<B,•C>•=•where•D<E>:•F<G>     TraitAliasDeclaration
       <B,•C>                        TraitAliasDeclaration.generics{dk: "<>"}
        B                            GenericTypeParameterDeclaration
           C                         GenericTypeParameterDeclaration
                where•D<E>:•F<G>     TraitAliasDeclaration.whereBounds{dk: "None"}
                      D<E>:•F<G>     WhereTypeBoundDeclaration
                      D<E>           TypeCall
                       <E>           TypeCall.typeArguments{dk: "<>"}
                            F<G>     TypeTraitBound{!maybeConst, !optional}, TypeCall
                             <G>     TypeCall.typeArguments{dk: "<>"}
                                ;    ExpressionStatement{semi}                                                                            */
trait A<X: T1> : T2 {}                                                                                                                    /*
trait•A<X:•T1>•:•T2•{}    TraitDeclaration
       <X:•T1>            TraitDeclaration.generics{dk: "<>"}
        X:•T1             GenericTypeParameterDeclaration
           T1             TypeTraitBound{!maybeConst, !optional}
                 T2       TypeTraitBound{!maybeConst, !optional}
                    {}    TraitDeclaration.body{dk: "{}"}                                                                                 */
trait A<X: ?Sized> {}                                                                                                                     /*
trait•A<X:•?Sized>•{}    TraitDeclaration
       <X:•?Sized>       TraitDeclaration.generics{dk: "<>"}
        X:•?Sized        GenericTypeParameterDeclaration
           ?Sized        TypeTraitBound{!maybeConst, optional}
                   {}    TraitDeclaration.body{dk: "{}"}                                                                                  */
trait A<X: ?Sized, Y> {}                                                                                                                  /*
trait•A<X:•?Sized,•Y>•{}    TraitDeclaration
       <X:•?Sized,•Y>       TraitDeclaration.generics{dk: "<>"}
        X:•?Sized           GenericTypeParameterDeclaration
           ?Sized           TypeTraitBound{!maybeConst, optional}
                   Y        GenericTypeParameterDeclaration
                      {}    TraitDeclaration.body{dk: "{}"}                                                                               */
trait A<Y, X: ?Sized> {}                                                                                                                  /*
trait•A<Y,•X:•?Sized>•{}    TraitDeclaration
       <Y,•X:•?Sized>       TraitDeclaration.generics{dk: "<>"}
        Y                   GenericTypeParameterDeclaration
           X:•?Sized        GenericTypeParameterDeclaration
              ?Sized        TypeTraitBound{!maybeConst, optional}
                      {}    TraitDeclaration.body{dk: "{}"}                                                                               */
trait A<X: ?Sized, Y: ?Sized> {}                                                                                                          /*
trait•A<X:•?Sized,•Y:•?Sized>•{}    TraitDeclaration
       <X:•?Sized,•Y:•?Sized>       TraitDeclaration.generics{dk: "<>"}
        X:•?Sized                   GenericTypeParameterDeclaration
           ?Sized                   TypeTraitBound{!maybeConst, optional}
                   Y:•?Sized        GenericTypeParameterDeclaration
                      ?Sized        TypeTraitBound{!maybeConst, optional}
                              {}    TraitDeclaration.body{dk: "{}"}                                                                       */
trait A<X: ?Sized+T2> {}                                                                                                                  /*
trait•A<X:•?Sized+T2>•{}    TraitDeclaration
       <X:•?Sized+T2>       TraitDeclaration.generics{dk: "<>"}
        X:•?Sized+T2        GenericTypeParameterDeclaration
           ?Sized           TypeTraitBound{!maybeConst, optional}
                  T2        TypeTraitBound{!maybeConst, !optional}
                      {}    TraitDeclaration.body{dk: "{}"}                                                                               */
trait A<X: T2 + ?Sized> {}                                                                                                                /*
trait•A<X:•T2•+•?Sized>•{}    TraitDeclaration
       <X:•T2•+•?Sized>       TraitDeclaration.generics{dk: "<>"}
        X:•T2•+•?Sized        GenericTypeParameterDeclaration
           T2                 TypeTraitBound{!maybeConst, !optional}
                ?Sized        TypeTraitBound{!maybeConst, optional}
                        {}    TraitDeclaration.body{dk: "{}"}                                                                             */

pub trait A {}                                                                                                                            /*
pub•trait•A•{}    TraitDeclaration
pub               PubSpecifier
            {}    TraitDeclaration.body{dk: "{}"}                                                                                         */
pub trait C = A + B;                                                                                                                      /*
pub•trait•C•=•A•+•B     TraitAliasDeclaration
pub                     PubSpecifier
              A         TypeTraitBound{!maybeConst, !optional}
                  B     TypeTraitBound{!maybeConst, !optional}
                   ;    ExpressionStatement{semi}                                                                                         */
pub trait A<R: D>: Sized {}                                                                                                               /*
pub•trait•A<R:•D>:•Sized•{}    TraitDeclaration
pub                            PubSpecifier
           <R:•D>              TraitDeclaration.generics{dk: "<>"}
            R:•D               GenericTypeParameterDeclaration
               D               TypeTraitBound{!maybeConst, !optional}
                   Sized       TypeTraitBound{!maybeConst, !optional}
                         {}    TraitDeclaration.body{dk: "{}"}
pub•trait•A<R:•D>:•Sized•{}    </Program.ast>
pub•trait•A<R:•D>:•Sized•{}    </Program>                                                                                                 */
// Discarded Nodes: 0
// Parsed Nodes: 290
// state_rollbacks: 4
// Total '.charCodeAt()' calls: 1157 (49% re-reads)
// Unnecessary 'skip_whitespace()' calls: 212
// source: "../../samples/statements/trait.rs"