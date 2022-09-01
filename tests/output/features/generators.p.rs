#![feature(generators)]                                                                                                                   /*
#![feature(generators)]↲    <Program>
#![feature(generators)]     Attribute{inner}
  [feature(generators)]     Attribute.segments{dk: "[]"}
          (generators)      DelimGroup                                                                                                    */

[                                                                                                                                         /*
[↲    <Program.ast{dk: "None"}>
[↲    <ExpressionStatement{!semi}>
[↲    <ArrayLiteral>                                                                                                                      */
	static move || { let a = A::<Box<dyn D>> { b: E::r(), }; yield (); },                                                                 /*
	static•move•||•{•let•a•=•A::<Box<dyn•D>>•{•b:•E::r(),•};•yield•();•}    ClosureFunctionExpression{move, static}
	            ||                                                          ClosureFunctionExpression.parameters{dk: "||"}
	               {•let•a•=•A::<Box<dyn•D>>•{•b:•E::r(),•};•yield•();•}    BlockExpression
	                 let•a•=•A::<Box<dyn•D>>•{•b:•E::r(),•};                LetVariableDeclaration
	                         A::<Box<dyn•D>>•{•b:•E::r(),•}                 StructLiteral
	                         A::<Box<dyn•D>>                                ExpressionTypeCast
	                            <Box<dyn•D>>                                ExpressionTypeCast.typeArguments{dk: "<>"}
	                             Box<dyn•D>                                 TypeCall
	                                <dyn•D>                                 TypeCall.typeArguments{dk: "<>"}
	                                 dyn•D                                  TypeDynBounds{dyn}
	                                     D                                  TypeTraitBound{!maybeConst, !optional}
	                                         {•b:•E::r(),•}                 StructLiteral.properties{dk: "{}"}
	                                           b:•E::r()                    StructLiteralProperty
	                                              E::r()                    CallExpression
	                                              E::r                      ExpressionPath
	                                                  ()                    CallExpression.arguments{dk: "()"}
	                                                         yield•();      ExpressionStatement{semi}
	                                                         yield•()       YieldExpression
	                                                               ()       TupleLiteral                                                  */
	|_| { a!("-> {}", yield); },                                                                                                          /*
	|_|•{•a!("->•{}",•yield);•}    ClosureFunctionExpression
	|_|                            ClosureFunctionExpression.parameters{dk: "||"}
	 _                             ClosureFunctionParameterDeclaration, WildcardPattern
	    {•a!("->•{}",•yield);•}    BlockExpression
	      a!("->•{}",•yield);      ExpressionStatement{semi}
	      a!("->•{}",•yield)       MacroInvocation
	        ("->•{}",•yield)       MacroInvocation.segments{dk: "()"}
	         "->•{}"               Literal{kind: String}
	                ,              PunctuationToken{tk: ","}                                                                              */
]                                                                                                                                         /*
]    </ArrayLiteral>
]    </ExpressionStatement>
]    </Program.ast>
]    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 39
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 168 (30% re-reads)
// Unnecessary 'skip_whitespace()' calls: 26
// source: "../../samples/features/generators.rs"