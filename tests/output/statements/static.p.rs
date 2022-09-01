static A: u8;                                                                                                                             /*
static•A:•u8;↲    <Program>
static•A:•u8;↲    <Program.ast{dk: "None"}>
static•A:•u8;     StaticVariableDeclaration                                                                                               */
static B;                                                                                                                                 /*
static•B;    StaticVariableDeclaration
        ^    MissingNode (StaticVariableDeclaration.typeAnnotation)                                                                       */

static mut C: u8;                                                                                                                         /*
static•mut•C:•u8;    StaticVariableDeclaration
       mut•C         PatternVariableDeclaration{!ref, mut}                                                                                */
static mut D;                                                                                                                             /*
static•mut•D;    StaticVariableDeclaration
       mut•D     PatternVariableDeclaration{!ref, mut}
            ^    MissingNode (StaticVariableDeclaration.typeAnnotation)                                                                   */
static X: u8;                                                                                                                             /*
static•X:•u8;    StaticVariableDeclaration                                                                                                */
pub static B: &'static a = unsafe { &q };                                                                                                 /*
pub•static•B:•&'static•a•=•unsafe•{•&q•};    StaticVariableDeclaration
pub                                          PubSpecifier
              &'static•a                     TypeReference{!mut}
               'static                       LtStatic
                           unsafe•{•&q•}     BlockExpression{unsafe}
                                  {•&q•}     BlockExpression.body{dk: "{}"}
                                    &q       ExpressionStatement{!semi}, ReferenceExpression{!mut}                                        */
static A: fn(_) -> u8 = |_| 8;                                                                                                            /*
static•A:•fn(_)•->•u8•=•|_|•8;    StaticVariableDeclaration
          fn(_)•->•u8             TypeFnPointer
            (_)                   TypeFnPointer.parameters{dk: "()"}
             _                    TypeFnPointerParameter, TypeInferred
                        |_|•8     ClosureFunctionExpression
                        |_|       ClosureFunctionExpression.parameters{dk: "||"}
                         _        ClosureFunctionParameterDeclaration, WildcardPattern
                            8     Literal{kind: Integer}                                                                                  */
static BOO: dyn Fn() -> _ = "";                                                                                                           /*
static•BOO:•dyn•Fn()•->•_•=•"";    StaticVariableDeclaration
            dyn•Fn()•->•_          TypeDynBounds{dyn}
                Fn()•->•_          TypeTraitBound{!maybeConst, !optional}, TypeFunction
                  ()               TypeFunction.parameters{dk: "()"}
                        _          TypeInferred
                            ""     Literal{kind: String}                                                                                  */
static x: _;                                                                                                                              /*
static•x:•_;    StaticVariableDeclaration
          _     TypeInferred
static•x:•_;    </Program.ast>
static•x:•_;    </Program>                                                                                                                */
// Discarded Nodes: 0
// Parsed Nodes: 50
// state_rollbacks: 9
// Total '.charCodeAt()' calls: 315 (66% re-reads)
// Unnecessary 'skip_whitespace()' calls: 31
// source: "../../samples/statements/static.rs"