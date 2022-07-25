const X: u8;                                                                                                                              /*
const•X:•u8;    ConstVariableDeclaration                                                                                                  */
const B;                                                                                                                                  /*
const•B;    ConstVariableDeclaration
       ^    MissingNode                                                                                                                   */
const A: u8;                                                                                                                              /*
const•A:•u8;    ConstVariableDeclaration                                                                                                  */
pub const A: Self::AssocTy = 1;                                                                                                           /*
pub•const•A:•Self::AssocTy•=•1;    ConstVariableDeclaration
pub                                PubSpecifier
             Self::AssocTy         TypePath
                             1     Literal                                                                                                */
const FOO: dyn Fn() -> _ = "";                                                                                                            /*
const•FOO:•dyn•Fn()•->•_•=•"";    ConstVariableDeclaration
           dyn•Fn()•->•_          TypeDynBounds
               Fn()•->•_          TypeTraitBound, TypeFunction
                       _          TypeInferred
                           ""     Literal                                                                                                 */
pub const FOO: &'static *const i32 = &(&0 as _);                                                                                          /*
pub•const•FOO:•&'static•*const•i32•=•&(&0•as•_);    ConstVariableDeclaration
pub                                                 PubSpecifier
               &'static•*const•i32                  TypeReference
                'static                             LtStatic
                        *const•i32                  TypeDereferenceConst
                                     &(&0•as•_)     ReferenceExpression
                                       &0•as•_      ExpressionAsTypeCast
                                       &0           ReferenceExpression
                                        0           Literal
                                             _      TypeInferred                                                                          */
const TEST: fn() -> _ = 1;                                                                                                                /*
const•TEST:•fn()•->•_•=•1;    ConstVariableDeclaration
            fn()•->•_         TypeFnPointer
                    _         TypeInferred
                        1     Literal                                                                                                     */
const MY_A: A = A { e: |s, a, b| { if s { let _ = (); } else if let Q(s) = b.r(|p| p.d()) { let _ = (); } }, };                           /*
const•MY_A:•A•=•A•{•e:•|s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•},•};    ConstVariableDeclaration
                A•{•e:•|s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•},•}     StructLiteral
                    e:•|s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•}        StructLiteralProperty
                       |s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•}        ClosureFunctionExpression
                        s                                                                                          ClosureFunctionParameterDeclaration
                           a                                                                                       ClosureFunctionParameterDeclaration
                              b                                                                                    ClosureFunctionParameterDeclaration
                                 {•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•}        BlockExpression
                                   if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}          ExpressionStatement, IfBlockExpression
                                          let•_•=•();                                                              LetVariableDeclaration
                                              _                                                                    WildcardPattern
                                                  ()                                                               TupleLiteral
                                                             if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}          IfBlockExpression
                                                                let•Q(s)•=•b.r(|p|•p.d())                          LetScrutinee
                                                                    Q(s)                                           TuplePattern
                                                                           b.r(|p|•p.d())                          CallExpression
                                                                               |p|•p.d()                           ClosureFunctionExpression
                                                                                p                                  ClosureFunctionParameterDeclaration
                                                                                   p.d()                           CallExpression
                                                                                            let•_•=•();            LetVariableDeclaration
                                                                                                _                  WildcardPattern
                                                                                                    ()             TupleLiteral           */

// Discarded Nodes: 1
// Parsed Nodes: 81
// state_rollbacks: 7
// Total '.charCodeAt()' calls: 426 (49% re-reads)
// Unnecessary 'skip_whitespace()' calls: 43
// source: "../../samples/statements/const.rs"