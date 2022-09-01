const X: u8;                                                                                                                              /*
const•X:•u8;↲    <Program>
const•X:•u8;↲    <Program.ast{dk: "None"}>
const•X:•u8;     ConstVariableDeclaration                                                                                                 */
const B;                                                                                                                                  /*
const•B;    ConstVariableDeclaration
       ^    MissingNode (ConstVariableDeclaration.typeAnnotation)                                                                         */
const A: u8;                                                                                                                              /*
const•A:•u8;    ConstVariableDeclaration                                                                                                  */
pub const A: Self::AssocTy = 1;                                                                                                           /*
pub•const•A:•Self::AssocTy•=•1;    ConstVariableDeclaration
pub                                PubSpecifier
             Self::AssocTy         TypePath
                             1     Literal{kind: Integer}                                                                                 */
const FOO: dyn Fn() -> _ = "";                                                                                                            /*
const•FOO:•dyn•Fn()•->•_•=•"";    ConstVariableDeclaration
           dyn•Fn()•->•_          TypeDynBounds{dyn}
               Fn()•->•_          TypeTraitBound{!maybeConst, !optional}, TypeFunction
                 ()               TypeFunction.parameters{dk: "()"}
                       _          TypeInferred
                           ""     Literal{kind: String}                                                                                   */
pub const FOO: &'static *const i32 = &(&0 as _);                                                                                          /*
pub•const•FOO:•&'static•*const•i32•=•&(&0•as•_);    ConstVariableDeclaration
pub                                                 PubSpecifier
               &'static•*const•i32                  TypeReference{!mut}
                'static                             LtStatic
                        *const•i32                  TypeDereferenceConst
                                     &(&0•as•_)     ReferenceExpression{!mut}
                                       &0•as•_      ExpressionAsTypeCast
                                       &0           ReferenceExpression{!mut}
                                        0           Literal{kind: Integer}
                                             _      TypeInferred                                                                          */
const TEST: fn() -> _ = 1;                                                                                                                /*
const•TEST:•fn()•->•_•=•1;    ConstVariableDeclaration
            fn()•->•_         TypeFnPointer
              ()              TypeFnPointer.parameters{dk: "()"}
                    _         TypeInferred
                        1     Literal{kind: Integer}                                                                                      */
const MY_A: A = A { e: |s, a, b| { if s { let _ = (); } else if let Q(s) = b.r(|p| p.d()) { let _ = (); } }, };                           /*
const•MY_A:•A•=•A•{•e:•|s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•},•};    ConstVariableDeclaration
                A•{•e:•|s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•},•}     StructLiteral
                  {•e:•|s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•},•}     StructLiteral.properties{dk: "{}"}
                    e:•|s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•}        StructLiteralProperty
                       |s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•}        ClosureFunctionExpression
                       |s,•a,•b|                                                                                   ClosureFunctionExpression.parameters{dk: "||"}
                        s                                                                                          ClosureFunctionParameterDeclaration
                           a                                                                                       ClosureFunctionParameterDeclaration
                              b                                                                                    ClosureFunctionParameterDeclaration
                                 {•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•}        BlockExpression
                                   if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}          ExpressionStatement{!semi}, IfBlockExpression
                                        {•let•_•=•();•}                                                            IfBlockExpression.body{dk: "{}"}
                                          let•_•=•();                                                              LetVariableDeclaration
                                              _                                                                    WildcardPattern
                                                  ()                                                               TupleLiteral
                                                             if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}          IfBlockExpression
                                                                let•Q(s)•=•b.r(|p|•p.d())                          LetScrutinee
                                                                    Q(s)                                           TuplePattern
                                                                     (s)                                           TuplePattern.items{dk: "()"}
                                                                           b.r(|p|•p.d())                          CallExpression
                                                                              (|p|•p.d())                          CallExpression.arguments{dk: "()"}
                                                                               |p|•p.d()                           ClosureFunctionExpression
                                                                               |p|                                 ClosureFunctionExpression.parameters{dk: "||"}
                                                                                p                                  ClosureFunctionParameterDeclaration
                                                                                   p.d()                           CallExpression
                                                                                      ()                           CallExpression.arguments{dk: "()"}
                                                                                          {•let•_•=•();•}          IfBlockExpression.body{dk: "{}"}
                                                                                            let•_•=•();            LetVariableDeclaration
                                                                                                _                  WildcardPattern
                                                                                                    ()             TupleLiteral
const•MY_A:•A•=•A•{•e:•|s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•},•};    </Program.ast>
const•MY_A:•A•=•A•{•e:•|s,•a,•b|•{•if•s•{•let•_•=•();•}•else•if•let•Q(s)•=•b.r(|p|•p.d())•{•let•_•=•();•}•},•};    </Program>             */
// Discarded Nodes: 1
// Parsed Nodes: 81
// state_rollbacks: 7
// Total '.charCodeAt()' calls: 426 (49% re-reads)
// Unnecessary 'skip_whitespace()' calls: 43
// source: "../../samples/statements/const.rs"