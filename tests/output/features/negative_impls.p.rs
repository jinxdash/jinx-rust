#![feature(negative_impls)]                                                                                                               /*
#![feature(negative_impls)]    Attribute
          (negative_impls)     DelimGroup                                                                                                 */

impl !Send for Test {}                                                                                                                    /*
impl•!Send•for•Test•{}    NegativeImplDeclaration                                                                                         */
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
     T:•Send                                  GenericTypeParameterDeclaration
        Send                                  TypeTraitBound
               A::B                           TypePath
                                T:•Copy       WhereTypeBoundDeclaration
                                   Copy       TypeTraitBound                                                                              */
impl<T: Clone> !A for B where T: Sync {}                                                                                                  /*
impl<T:•Clone>•!A•for•B•where•T:•Sync•{}    NegativeImplDeclaration
     T:•Clone                               GenericTypeParameterDeclaration
        Clone                               TypeTraitBound
                              T:•Sync       WhereTypeBoundDeclaration
                                 Sync       TypeTraitBound                                                                                */
impl !Send for A {}                                                                                                                       /*
impl•!Send•for•A•{}    NegativeImplDeclaration                                                                                            */
impl !Sync for A {}                                                                                                                       /*
impl•!Sync•for•A•{}    NegativeImplDeclaration                                                                                            */
impl<'a, T> !MyPredicate<'a> for &T where T: 'a {}                                                                                        /*
impl<'a,•T>•!MyPredicate<'a>•for•&T•where•T:•'a•{}    NegativeImplDeclaration
     'a                                               GenericLtParameterDeclaration, LtIdentifier
         T                                            GenericTypeParameterDeclaration
             MyPredicate<'a>                          TypeCall
                         'a                           LtIdentifier
                                 &T                   TypeReference
                                          T:•'a       WhereTypeBoundDeclaration
                                             'a       LtIdentifier                                                                        */
impl<T> !Foo for &T where T: 'static {}                                                                                                   /*
impl<T>•!Foo•for•&T•where•T:•'static•{}    NegativeImplDeclaration
     T                                     GenericTypeParameterDeclaration
                 &T                        TypeReference
                          T:•'static       WhereTypeBoundDeclaration
                             'static       LtStatic                                                                                       */
impl<E> !Future for Option<E> where E: Sized {}                                                                                           /*
impl<E>•!Future•for•Option<E>•where•E:•Sized•{}    NegativeImplDeclaration
     E                                             GenericTypeParameterDeclaration
                    Option<E>                      TypeCall
                                    E:•Sized       WhereTypeBoundDeclaration
                                       Sized       TypeTraitBound                                                                         */

// Discarded Nodes: 0
// Parsed Nodes: 99
// state_rollbacks: 25
// Total '.charCodeAt()' calls: 796 (67% re-reads)
// Unnecessary 'skip_whitespace()' calls: 56
// source: "../../samples/features/negative_impls.rs"