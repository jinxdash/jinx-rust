#![feature(generators)]                                                                                                                   /*
#![feature(generators)]    Attribute
          (generators)     DelimGroup                                                                                                     */

[                                                                                                                                         /*
[↲    <ExpressionStatement>, <ArrayLiteral>                                                                                               */
	static move || { let a = A::<Box<dyn D>> { b: E::r(), }; yield (); },                                                                 /*
    static•move•||•{•let•a•=•A::<Box<dyn•D>>•{•b:•E::r(),•};•yield•();•}    ClosureFunctionExpression
                   {•let•a•=•A::<Box<dyn•D>>•{•b:•E::r(),•};•yield•();•}    BlockExpression
                     let•a•=•A::<Box<dyn•D>>•{•b:•E::r(),•};                LetVariableDeclaration
                             A::<Box<dyn•D>>•{•b:•E::r(),•}                 StructLiteral
                             A::<Box<dyn•D>>                                ExpressionTypeCast
                                 Box<dyn•D>                                 TypeCall
                                     dyn•D                                  TypeDynBounds
                                         D                                  TypeTraitBound
                                               b:•E::r()                    StructLiteralProperty
                                                  E::r()                    CallExpression
                                                  E::r                      ExpressionPath
                                                             yield•();      ExpressionStatement
                                                             yield•()       YieldExpression
                                                                   ()       TupleLiteral                                                  */
	|_| { a!("-> {}", yield); },                                                                                                          /*
    |_|•{•a!("->•{}",•yield);•}    ClosureFunctionExpression
     _                             ClosureFunctionParameterDeclaration, WildcardPattern
        {•a!("->•{}",•yield);•}    BlockExpression
          a!("->•{}",•yield);      ExpressionStatement
          a!("->•{}",•yield)       MacroInvocation
             "->•{}"               Literal
                    ,              PunctuationToken                                                                                       */
]                                                                                                                                         /*
]    </ExpressionStatement>, </ArrayLiteral>                                                                                              */

// Discarded Nodes: 0
// Parsed Nodes: 39
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 168 (30% re-reads)
// Unnecessary 'skip_whitespace()' calls: 26
// source: "../../samples/features/generators.rs"