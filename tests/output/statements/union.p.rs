union {}                                                                                                                                  /*
union•{}↲    <Program>
union•{}↲    <Program.ast{dk: "None"}>
union•{}     ExpressionStatement{!semi}, StructLiteral
      {}     StructLiteral.properties{dk: "{}"}                                                                                           */
union::b {}                                                                                                                               /*
union::b•{}    ExpressionStatement{!semi}, StructLiteral
union::b       ExpressionPath
         {}    StructLiteral.properties{dk: "{}"}                                                                                         */
union union<'union> { union: &'union union<'union>, }                                                                                     /*
union•union<'union>•{•union:•&'union•union<'union>,•}    UnionDeclaration
           <'union>                                      UnionDeclaration.generics{dk: "<>"}
            'union                                       GenericLtParameterDeclaration, LtIdentifier
                    {•union:•&'union•union<'union>,•}    UnionDeclaration.properties{dk: "{}"}
                      union:•&'union•union<'union>       StructPropertyDeclaration
                             &'union•union<'union>       TypeReference{!mut}
                              'union                     LtIdentifier
                                     union<'union>       TypeCall
                                          <'union>       TypeCall.typeArguments{dk: "<>"}
                                           'union        LtIdentifier                                                                     */
struct union;                                                                                                                             /*
struct•union;    StructDeclaration                                                                                                        */

impl union {                                                                                                                              /*
impl•union•{↲    <ImplDeclaration{!const}>
           {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                         */
    pub fn new() -> Self {                                                                                                                /*
    pub•fn•new()•->•Self•{↲    <FunctionDeclaration>
    pub                        PubSpecifier
              ()               FunctionDeclaration.parameters{dk: "()"}
                         {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                       */
        union { }                                                                                                                         /*
        union•{•}    ExpressionStatement{!semi}, StructLiteral
              {•}    StructLiteral.properties{dk: "{}"}                                                                                   */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration.body>
••••}    </FunctionDeclaration>                                                                                                           */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */

fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>
       ()       FunctionDeclaration.parameters{dk: "()"}
          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                      */
    let _u = union::new();                                                                                                                /*
    let•_u•=•union::new();    LetVariableDeclaration
             union::new()     CallExpression
             union::new       ExpressionPath
                       ()     CallExpression.arguments{dk: "()"}                                                                          */
	let mut r#async = 1;                                                                                                                  /*
	let•mut•r#async•=•1;    LetVariableDeclaration
	    mut•r#async         PatternVariableDeclaration{!ref, mut}
	                  1     Literal{kind: Integer}                                                                                        */
	union as T;                                                                                                                           /*
	union•as•T;    ExpressionStatement{semi}
	union•as•T     ExpressionAsTypeCast                                                                                                   */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 48
// state_rollbacks: 5
// Total '.charCodeAt()' calls: 322 (39% re-reads)
// Unnecessary 'skip_whitespace()' calls: 24
// source: "../../samples/statements/union.rs"