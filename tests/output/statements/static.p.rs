static A: u8;                                                                                                                             /*
static•A:•u8;    StaticVariableDeclaration                                                                                                */
static B;                                                                                                                                 /*
static•B;    StaticVariableDeclaration
        ^    MissingNode                                                                                                                  */

static mut C: u8;                                                                                                                         /*
static•mut•C:•u8;    StaticVariableDeclaration
       mut•C         PatternVariableDeclaration                                                                                           */
static mut D;                                                                                                                             /*
static•mut•D;    StaticVariableDeclaration
       mut•D     PatternVariableDeclaration
            ^    MissingNode                                                                                                              */
static X: u8;                                                                                                                             /*
static•X:•u8;    StaticVariableDeclaration                                                                                                */
pub static B: &'static a = unsafe { &q };                                                                                                 /*
pub•static•B:•&'static•a•=•unsafe•{•&q•};    StaticVariableDeclaration
pub                                          PubSpecifier
              &'static•a                     TypeReference
               'static                       LtStatic
                           unsafe•{•&q•}     BlockExpression
                                    &q       ExpressionStatement, ReferenceExpression                                                     */
static A: fn(_) -> u8 = |_| 8;                                                                                                            /*
static•A:•fn(_)•->•u8•=•|_|•8;    StaticVariableDeclaration
          fn(_)•->•u8             TypeFnPointer
             _                    TypeFnPointerParameter, TypeInferred
                        |_|•8     ClosureFunctionExpression
                         _        ClosureFunctionParameterDeclaration, WildcardPattern
                            8     Literal                                                                                                 */
static BOO: dyn Fn() -> _ = "";                                                                                                           /*
static•BOO:•dyn•Fn()•->•_•=•"";    StaticVariableDeclaration
            dyn•Fn()•->•_          TypeDynBounds
                Fn()•->•_          TypeTraitBound, TypeFunction
                        _          TypeInferred
                            ""     Literal                                                                                                */
static x: _;                                                                                                                              /*
static•x:•_;    StaticVariableDeclaration
          _     TypeInferred                                                                                                              */

// Discarded Nodes: 0
// Parsed Nodes: 50
// state_rollbacks: 9
// Total '.charCodeAt()' calls: 315 (66% re-reads)
// Unnecessary 'skip_whitespace()' calls: 31
// source: "../../samples/statements/static.rs"