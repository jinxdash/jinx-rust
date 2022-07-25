trait A {}                                                                                                                                /*
trait•A•{}    TraitDeclaration                                                                                                            */
trait A<B:C>{}                                                                                                                            /*
trait•A<B:C>{}    TraitDeclaration
        B:C       GenericTypeParameterDeclaration
          C       TypeTraitBound                                                                                                          */
trait T2<'x, 'y> : T1<'x> {}                                                                                                              /*
trait•T2<'x,•'y>•:•T1<'x>•{}    TraitDeclaration
         'x                     GenericLtParameterDeclaration, LtIdentifier
             'y                 GenericLtParameterDeclaration, LtIdentifier
                   T1<'x>       TypeTraitBound, TypeCall
                      'x        LtIdentifier                                                                                              */
trait T<E, R> = S<A> where <Self as B<D>>::T: H<R>;                                                                                       /*
trait•T<E,•R>•=•S<A>•where•<Self•as•B<D>>::T:•H<R>     TraitAliasDeclaration
        E                                              GenericTypeParameterDeclaration
           R                                           GenericTypeParameterDeclaration
                S<A>                                   TypeTraitBound, TypeCall
                           <Self•as•B<D>>::T:•H<R>     WhereTypeBoundDeclaration
                           <Self•as•B<D>>::T           TypePath
                           <Self•as•B<D>>              ExpressionTypeSelector
                                    B<D>               TypeCall
                                              H<R>     TypeTraitBound, TypeCall
                                                  ;    ExpressionStatement                                                                */
trait A<B: C>: B<B::S> + E<()> {}                                                                                                         /*
trait•A<B:•C>:•B<B::S>•+•E<()>•{}    TraitDeclaration
        B:•C                         GenericTypeParameterDeclaration
           C                         TypeTraitBound
               B<B::S>               TypeTraitBound, TypeCall
                 B::S                TypePath
                         E<()>       TypeTraitBound, TypeCall
                           ()        TypeTuple                                                                                            */
trait A<S: B<C=S>>: D<S> {}                                                                                                               /*
trait•A<S:•B<C=S>>:•D<S>•{}    TraitDeclaration
        S:•B<C=S>              GenericTypeParameterDeclaration
           B<C=S>              TypeTraitBound, TypeCall
             C=S               TypeCallNamedArgument
                    D<S>       TypeTraitBound, TypeCall                                                                                   */
trait A = B<C=D>;                                                                                                                         /*
trait•A•=•B<C=D>     TraitAliasDeclaration
          B<C=D>     TypeTraitBound, TypeCall
            C=D      TypeCallNamedArgument
                ;    ExpressionStatement                                                                                                  */
trait A<T> = B where T: C<D>;                                                                                                             /*
trait•A<T>•=•B•where•T:•C<D>     TraitAliasDeclaration
        T                        GenericTypeParameterDeclaration
             B                   TypeTraitBound
                     T:•C<D>     WhereTypeBoundDeclaration
                        C<D>     TypeTraitBound, TypeCall
                            ;    ExpressionStatement                                                                                      */
trait A: B<B = u32> + C<Self::A> {}                                                                                                       /*
trait•A:•B<B•=•u32>•+•C<Self::A>•{}    TraitDeclaration
         B<B•=•u32>                    TypeTraitBound, TypeCall
           B•=•u32                     TypeCallNamedArgument
                      C<Self::A>       TypeTraitBound, TypeCall
                        Self::A        TypePath                                                                                           */
trait A = std::fmt::Display + std::fmt::Debug;                                                                                            /*
trait•A•=•std::fmt::Display•+•std::fmt::Debug     TraitAliasDeclaration
          std::fmt::Display                       TypeTraitBound, TypePath
          std::fmt                                TypePath
                              std::fmt::Debug     TypeTraitBound, TypePath
                              std::fmt            TypePath
                                             ;    ExpressionStatement                                                                     */
trait B = std::fmt::Display + std::fmt::Debug;                                                                                            /*
trait•B•=•std::fmt::Display•+•std::fmt::Debug     TraitAliasDeclaration
          std::fmt::Display                       TypeTraitBound, TypePath
          std::fmt                                TypePath
                              std::fmt::Debug     TypeTraitBound, TypePath
                              std::fmt            TypePath
                                             ;    ExpressionStatement                                                                     */
trait A = Default;                                                                                                                        /*
trait•A•=•Default     TraitAliasDeclaration
          Default     TypeTraitBound
                 ;    ExpressionStatement                                                                                                 */
trait A<T> = B<C = D>;                                                                                                                    /*
trait•A<T>•=•B<C•=•D>     TraitAliasDeclaration
        T                 GenericTypeParameterDeclaration
             B<C•=•D>     TypeTraitBound, TypeCall
               C•=•D      TypeCallNamedArgument
                     ;    ExpressionStatement                                                                                             */
trait A = B<C>;                                                                                                                           /*
trait•A•=•B<C>     TraitAliasDeclaration
          B<C>     TypeTraitBound, TypeCall
              ;    ExpressionStatement                                                                                                    */
trait A<'a, T: 'a> = B<&'a D>;                                                                                                            /*
trait•A<'a,•T:•'a>•=•B<&'a•D>     TraitAliasDeclaration
        'a                        GenericLtParameterDeclaration, LtIdentifier
            T:•'a                 GenericTypeParameterDeclaration
               'a                 LtIdentifier
                     B<&'a•D>     TypeTraitBound, TypeCall
                       &'a•D      TypeReference
                        'a        LtIdentifier
                             ;    ExpressionStatement                                                                                     */
trait A = 'static;                                                                                                                        /*
trait•A•=•'static     TraitAliasDeclaration
          'static     LtStatic
                 ;    ExpressionStatement                                                                                                 */
trait A<B, C> = D + E where F<(G, H)>: I;                                                                                                 /*
trait•A<B,•C>•=•D•+•E•where•F<(G,•H)>:•I     TraitAliasDeclaration
        B                                    GenericTypeParameterDeclaration
           C                                 GenericTypeParameterDeclaration
                D                            TypeTraitBound
                    E                        TypeTraitBound
                            F<(G,•H)>:•I     WhereTypeBoundDeclaration
                            F<(G,•H)>        TypeCall
                              (G,•H)         TypeTuple
                                       I     TypeTraitBound
                                        ;    ExpressionStatement                                                                          */
trait A<B, C> = where D<E>: F<G>;                                                                                                         /*
trait•A<B,•C>•=•where•D<E>:•F<G>     TraitAliasDeclaration
        B                            GenericTypeParameterDeclaration
           C                         GenericTypeParameterDeclaration
                      D<E>:•F<G>     WhereTypeBoundDeclaration
                      D<E>           TypeCall
                            F<G>     TypeTraitBound, TypeCall
                                ;    ExpressionStatement                                                                                  */
trait A<X: T1> : T2 {}                                                                                                                    /*
trait•A<X:•T1>•:•T2•{}    TraitDeclaration
        X:•T1             GenericTypeParameterDeclaration
           T1             TypeTraitBound
                 T2       TypeTraitBound                                                                                                  */
trait A<X: ?Sized> {}                                                                                                                     /*
trait•A<X:•?Sized>•{}    TraitDeclaration
        X:•?Sized        GenericTypeParameterDeclaration
           ?Sized        TypeTraitBound                                                                                                   */
trait A<X: ?Sized, Y> {}                                                                                                                  /*
trait•A<X:•?Sized,•Y>•{}    TraitDeclaration
        X:•?Sized           GenericTypeParameterDeclaration
           ?Sized           TypeTraitBound
                   Y        GenericTypeParameterDeclaration                                                                               */
trait A<Y, X: ?Sized> {}                                                                                                                  /*
trait•A<Y,•X:•?Sized>•{}    TraitDeclaration
        Y                   GenericTypeParameterDeclaration
           X:•?Sized        GenericTypeParameterDeclaration
              ?Sized        TypeTraitBound                                                                                                */
trait A<X: ?Sized, Y: ?Sized> {}                                                                                                          /*
trait•A<X:•?Sized,•Y:•?Sized>•{}    TraitDeclaration
        X:•?Sized                   GenericTypeParameterDeclaration
           ?Sized                   TypeTraitBound
                   Y:•?Sized        GenericTypeParameterDeclaration
                      ?Sized        TypeTraitBound                                                                                        */
trait A<X: ?Sized+T2> {}                                                                                                                  /*
trait•A<X:•?Sized+T2>•{}    TraitDeclaration
        X:•?Sized+T2        GenericTypeParameterDeclaration
           ?Sized           TypeTraitBound
                  T2        TypeTraitBound                                                                                                */
trait A<X: T2 + ?Sized> {}                                                                                                                /*
trait•A<X:•T2•+•?Sized>•{}    TraitDeclaration
        X:•T2•+•?Sized        GenericTypeParameterDeclaration
           T2                 TypeTraitBound
                ?Sized        TypeTraitBound                                                                                              */

pub trait A {}                                                                                                                            /*
pub•trait•A•{}    TraitDeclaration
pub               PubSpecifier                                                                                                            */
pub trait C = A + B;                                                                                                                      /*
pub•trait•C•=•A•+•B     TraitAliasDeclaration
pub                     PubSpecifier
              A         TypeTraitBound
                  B     TypeTraitBound
                   ;    ExpressionStatement                                                                                               */
pub trait A<R: D>: Sized {}                                                                                                               /*
pub•trait•A<R:•D>:•Sized•{}    TraitDeclaration
pub                            PubSpecifier
            R:•D               GenericTypeParameterDeclaration
               D               TypeTraitBound
                   Sized       TypeTraitBound                                                                                             */

// Discarded Nodes: 0
// Parsed Nodes: 290
// state_rollbacks: 4
// Total '.charCodeAt()' calls: 1157 (49% re-reads)
// Unnecessary 'skip_whitespace()' calls: 212
// source: "../../samples/statements/trait.rs"