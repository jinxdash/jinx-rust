fn bare_crate(_: crate::a);                                                                                                               /*
fn•bare_crate(_:•crate::a);    FunctionDeclaration
              _:•crate::a      FunctionParameterDeclaration
              _                WildcardPattern
                 crate::a      TypePath                                                                                                   */
fn bare_global(_: ::a);                                                                                                                   /*
fn•bare_global(_:•::a);    FunctionDeclaration
               _:•::a      FunctionParameterDeclaration
               _           WildcardPattern
                  ::a      TypePath                                                                                                       */
fn u8(u8: u8) {                                                                                                                           /*
fn•u8(u8:•u8)•{↲    <FunctionDeclaration>
      u8:•u8        FunctionParameterDeclaration                                                                                          */
    if u8 != 0u8 {}                                                                                                                       /*
    if•u8•!=•0u8•{}    ExpressionStatement, IfBlockExpression
       u8•!=•0u8       ComparisonExpression
             0u8       Literal
              u8       Identifier                                                                                                         */
    assert_eq!(8u8, {                                                                                                                     /*
    assert_eq!(8u8,•{↲    <ExpressionStatement>, <MacroInvocation>
               8u8        Literal
                u8        Identifier
                  ,       PunctuationToken
                    {↲    <DelimGroup>                                                                                                    */
        macro_rules! u8 { (u8) => { mod u8 { pub fn u8<'u8: 'u8 + 'u8>(u8: &'u8 u8) -> &'u8 u8 { "u8"; u8 } } }; }                        /*
                   !                                                                                                  PunctuationToken
                        {•(u8)•=>•{•mod•u8•{•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}•};•}    DelimGroup
                          (u8)                                                                                        DelimGroup
                               =>                                                                                     PunctuationToken
                                  {•mod•u8•{•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}•}       DelimGroup
                                           {•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}         DelimGroup
                                                      <                                                               PunctuationToken
                                                       'u8                                                            LtIdentifier
                                                          :                                                           PunctuationToken
                                                            'u8                                                       LtIdentifier
                                                                +                                                     PunctuationToken
                                                                  'u8                                                 LtIdentifier
                                                                     >                                                PunctuationToken
                                                                      (u8:•&'u8•u8)                                   DelimGroup
                                                                         :                                            PunctuationToken
                                                                           &                                          PunctuationToken
                                                                            'u8                                       LtIdentifier
                                                                                    ->                                PunctuationToken
                                                                                       &                              PunctuationToken
                                                                                        'u8                           LtIdentifier
                                                                                               {•"u8";•u8•}           DelimGroup
                                                                                                 "u8"                 Literal
                                                                                                     ;                PunctuationToken
                                                                                                               ;      PunctuationToken    */
        let &u8: &u8 = u8::u8(&8u8);                                                                                                      /*
            &                           PunctuationToken
               :                        PunctuationToken
                 &                      PunctuationToken
                     =                  PunctuationToken
                         ::             PunctuationToken
                             (&8u8)     DelimGroup
                              &         PunctuationToken
                               8u8      Literal
                                u8      Identifier
                                   ;    PunctuationToken                                                                                  */
        ::u8(0u8);                                                                                                                        /*
        ::            PunctuationToken
            (0u8)     DelimGroup
             0u8      Literal
              u8      Identifier
                 ;    PunctuationToken                                                                                                    */
        u8!(u8);                                                                                                                          /*
          !         PunctuationToken
           (u8)     DelimGroup
               ;    PunctuationToken                                                                                                      */
        u8
    });                                                                                                                                   /*
••••});    </ExpressionStatement>
••••})     </MacroInvocation>
••••}      </DelimGroup>                                                                                                                  */
    let &u8: &u8 = u8::u8(&8u8);                                                                                                          /*
    let•&u8:•&u8•=•u8::u8(&8u8);    LetVariableDeclaration
        &u8                         ReferencePattern
             &u8                    TypeReference
                   u8::u8(&8u8)     CallExpression
                   u8::u8           ExpressionPath
                          &8u8      ReferenceExpression
                           8u8      Literal
                            u8      Identifier                                                                                            */
    ::u8(0u8);                                                                                                                            /*
    ::u8(0u8);    ExpressionStatement
    ::u8(0u8)     CallExpression
    ::u8          ExpressionPath
         0u8      Literal
          u8      Identifier                                                                                                              */
    u8!(u8);                                                                                                                              /*
    u8!(u8);    ExpressionStatement
    u8!(u8)     MacroInvocation                                                                                                           */
    u8;                                                                                                                                   /*
    u8;    ExpressionStatement                                                                                                            */
    let µ = 1.0;                                                                                                                          /*
    let•µ•=•1.0;    LetVariableDeclaration
            1.0     Literal                                                                                                               */
    µ;                                                                                                                                    /*
    µ;    ExpressionStatement                                                                                                             */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

mod u8 { pub fn u8<'u8: 'u8 + 'u8>(u8: &'u8 u8) -> &'u8 u8 { "u8"; u8 } }                                                                 /*
mod•u8•{•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}    ModuleDeclaration
         pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}      FunctionDeclaration
         pub                                                                 PubSpecifier
                   'u8:•'u8•+•'u8                                            GenericLtParameterDeclaration
                   'u8                                                       LtIdentifier
                        'u8                                                  LtIdentifier
                              'u8                                            LtIdentifier
                                   u8:•&'u8•u8                               FunctionParameterDeclaration
                                       &'u8•u8                               TypeReference
                                        'u8                                  LtIdentifier
                                                   &'u8•u8                   TypeReference
                                                    'u8                      LtIdentifier
                                                             "u8";           ExpressionStatement
                                                             "u8"            Literal
                                                                   u8        ExpressionStatement                                          */

// Discarded Nodes: 0
// Parsed Nodes: 146
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 668 (37% re-reads)
// Unnecessary 'skip_whitespace()' calls: 69
// source: "../../samples/expressions/ident.rs"