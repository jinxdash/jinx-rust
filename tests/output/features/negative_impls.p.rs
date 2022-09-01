#![feature(negative_impls)]                                                                                                               /*
#![feature(negative_impls)]↲    <Program>
#![feature(negative_impls)]     Attribute{inner}
  [feature(negative_impls)]     Attribute.segments{dk: "[]"}
          (negative_impls)      DelimGroup                                                                                                */

impl !Send for Test {}                                                                                                                    /*
impl•!Send•for•Test•{}↲    <Program.ast{dk: "None"}>
impl•!Send•for•Test•{}     NegativeImplDeclaration                                                                                        */
impl !Send for Foo {}                                                                                                                     /*
impl•!Send•for•Foo•{}    NegativeImplDeclaration                                                                                          */
impl !Sync for Foo {}                                                                                                                     /*
impl•!Sync•for•Foo•{}    NegativeImplDeclaration                                                                                          */
impl !std::marker::Unpin for Foo {}                                                                                                       /*
impl•!std::marker::Unpin•for•Foo•{}    NegativeImplDeclaration
      std::marker::Unpin               TypePath
      std::marker                      TypePath                                                                                           */
impl !std::panic::RefUnwindSafe for Foo {}                                                                                                /*
impl•!std::panic::RefUnwindSafe•for•Foo•{}    NegativeImplDeclaration
      std::panic::RefUnwindSafe               TypePath
      std::panic                              TypePath                                                                                    */
impl !std::panic::UnwindSafe for Foo {}                                                                                                   /*
impl•!std::panic::UnwindSafe•for•Foo•{}    NegativeImplDeclaration
      std::panic::UnwindSafe               TypePath
      std::panic                           TypePath                                                                                       */
impl<T: Send> !A::B for C where T: Copy {}                                                                                                /*
impl<T:•Send>•!A::B•for•C•where•T:•Copy•{}    NegativeImplDeclaration
    <T:•Send>                                 NegativeImplDeclaration.generics{dk: "<>"}
     T:•Send                                  GenericTypeParameterDeclaration
        Send                                  TypeTraitBound{!maybeConst, !optional}
               A::B                           TypePath
                          where•T:•Copy       NegativeImplDeclaration.whereBounds{dk: "None"}
                                T:•Copy       WhereTypeBoundDeclaration
                                   Copy       TypeTraitBound{!maybeConst, !optional}                                                      */
impl<T: Clone> !A for B where T: Sync {}                                                                                                  /*
impl<T:•Clone>•!A•for•B•where•T:•Sync•{}    NegativeImplDeclaration
    <T:•Clone>                              NegativeImplDeclaration.generics{dk: "<>"}
     T:•Clone                               GenericTypeParameterDeclaration
        Clone                               TypeTraitBound{!maybeConst, !optional}
                        where•T:•Sync       NegativeImplDeclaration.whereBounds{dk: "None"}
                              T:•Sync       WhereTypeBoundDeclaration
                                 Sync       TypeTraitBound{!maybeConst, !optional}                                                        */
impl !Send for A {}                                                                                                                       /*
impl•!Send•for•A•{}    NegativeImplDeclaration                                                                                            */
impl !Sync for A {}                                                                                                                       /*
impl•!Sync•for•A•{}    NegativeImplDeclaration                                                                                            */
impl<'a, T> !MyPredicate<'a> for &T where T: 'a {}                                                                                        /*
impl<'a,•T>•!MyPredicate<'a>•for•&T•where•T:•'a•{}    NegativeImplDeclaration
    <'a,•T>                                           NegativeImplDeclaration.generics{dk: "<>"}
     'a                                               GenericLtParameterDeclaration, LtIdentifier
         T                                            GenericTypeParameterDeclaration
             MyPredicate<'a>                          TypeCall
                        <'a>                          TypeCall.typeArguments{dk: "<>"}
                         'a                           LtIdentifier
                                 &T                   TypeReference{!mut}
                                    where•T:•'a       NegativeImplDeclaration.whereBounds{dk: "None"}
                                          T:•'a       WhereTypeBoundDeclaration
                                             'a       LtIdentifier                                                                        */
impl<T> !Foo for &T where T: 'static {}                                                                                                   /*
impl<T>•!Foo•for•&T•where•T:•'static•{}    NegativeImplDeclaration
    <T>                                    NegativeImplDeclaration.generics{dk: "<>"}
     T                                     GenericTypeParameterDeclaration
                 &T                        TypeReference{!mut}
                    where•T:•'static       NegativeImplDeclaration.whereBounds{dk: "None"}
                          T:•'static       WhereTypeBoundDeclaration
                             'static       LtStatic                                                                                       */
impl<E> !Future for Option<E> where E: Sized {}                                                                                           /*
impl<E>•!Future•for•Option<E>•where•E:•Sized•{}    NegativeImplDeclaration
    <E>                                            NegativeImplDeclaration.generics{dk: "<>"}
     E                                             GenericTypeParameterDeclaration
                    Option<E>                      TypeCall
                          <E>                      TypeCall.typeArguments{dk: "<>"}
                              where•E:•Sized       NegativeImplDeclaration.whereBounds{dk: "None"}
                                    E:•Sized       WhereTypeBoundDeclaration
                                       Sized       TypeTraitBound{!maybeConst, !optional}
impl<E>•!Future•for•Option<E>•where•E:•Sized•{}    </Program.ast>
impl<E>•!Future•for•Option<E>•where•E:•Sized•{}    </Program>                                                                             */
// Discarded Nodes: 0
// Parsed Nodes: 99
// state_rollbacks: 25
// Total '.charCodeAt()' calls: 796 (67% re-reads)
// Unnecessary 'skip_whitespace()' calls: 56
// source: "../../samples/features/negative_impls.rs"