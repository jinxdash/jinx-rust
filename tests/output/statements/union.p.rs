union {}                                                                                                                                  /*
union•{}    ExpressionStatement, StructLiteral                                                                                            */
union::b {}                                                                                                                               /*
union::b•{}    ExpressionStatement, StructLiteral
union::b       ExpressionPath                                                                                                             */
union union<'union> { union: &'union union<'union>, }                                                                                     /*
union•union<'union>•{•union:•&'union•union<'union>,•}    UnionDeclaration
            'union                                       GenericLtParameterDeclaration, LtIdentifier
                      union:•&'union•union<'union>       StructPropertyDeclaration
                             &'union•union<'union>       TypeReference
                              'union                     LtIdentifier
                                     union<'union>       TypeCall
                                           'union        LtIdentifier                                                                     */
struct union;                                                                                                                             /*
struct•union;    StructDeclaration                                                                                                        */

impl union {                                                                                                                              /*
impl•union•{↲    <ImplDeclaration>                                                                                                        */
    pub fn new() -> Self {                                                                                                                /*
    pub•fn•new()•->•Self•{↲    <FunctionDeclaration>
    pub                        PubSpecifier                                                                                               */
        union { }                                                                                                                         /*
        union•{•}    ExpressionStatement, StructLiteral                                                                                   */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration>                                                                                                           */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */

fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>                                                                                                     */
    let _u = union::new();                                                                                                                /*
    let•_u•=•union::new();    LetVariableDeclaration
             union::new()     CallExpression
             union::new       ExpressionPath                                                                                              */
	let mut r#async = 1;                                                                                                                  /*
    let•mut•r#async•=•1;    LetVariableDeclaration
        mut•r#async         PatternVariableDeclaration
                      1     Literal                                                                                                       */
	union as T;                                                                                                                           /*
    union•as•T;    ExpressionStatement
    union•as•T     ExpressionAsTypeCast                                                                                                   */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 0
// Parsed Nodes: 48
// state_rollbacks: 5
// Total '.charCodeAt()' calls: 322 (39% re-reads)
// Unnecessary 'skip_whitespace()' calls: 24
// source: "../../samples/statements/union.rs"