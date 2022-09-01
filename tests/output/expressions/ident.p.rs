fn bare_crate(_: crate::a);                                                                                                               /*
fn•bare_crate(_:•crate::a);↲    <Program>
fn•bare_crate(_:•crate::a);↲    <Program.ast{dk: "None"}>
fn•bare_crate(_:•crate::a);     FunctionDeclaration
             (_:•crate::a)      FunctionDeclaration.parameters{dk: "()"}
              _:•crate::a       FunctionParameterDeclaration
              _                 WildcardPattern
                 crate::a       TypePath                                                                                                  */
fn bare_global(_: ::a);                                                                                                                   /*
fn•bare_global(_:•::a);    FunctionDeclaration
              (_:•::a)     FunctionDeclaration.parameters{dk: "()"}
               _:•::a      FunctionParameterDeclaration
               _           WildcardPattern
                  ::a      TypePath                                                                                                       */
fn u8(u8: u8) {                                                                                                                           /*
fn•u8(u8:•u8)•{↲    <FunctionDeclaration>
     (u8:•u8)       FunctionDeclaration.parameters{dk: "()"}
      u8:•u8        FunctionParameterDeclaration
              {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                  */
    if u8 != 0u8 {}                                                                                                                       /*
    if•u8•!=•0u8•{}    ExpressionStatement{!semi}, IfBlockExpression
       u8•!=•0u8       ComparisonExpression{tk: "!="}
             0u8       Literal{kind: Integer}
              u8       Identifier
                 {}    IfBlockExpression.body{dk: "{}"}                                                                                   */
    assert_eq!(8u8, {                                                                                                                     /*
    assert_eq!(8u8,•{↲    <ExpressionStatement{semi}>
    assert_eq!(8u8,•{↲    <MacroInvocation>
              (8u8,•{↲    <MacroInvocation.segments{dk: "()"}>
               8u8        Literal{kind: Integer}
                u8        Identifier
                  ,       PunctuationToken{tk: ","}
                    {↲    <DelimGroup>                                                                                                    */
        macro_rules! u8 { (u8) => { mod u8 { pub fn u8<'u8: 'u8 + 'u8>(u8: &'u8 u8) -> &'u8 u8 { "u8"; u8 } } }; }                        /*
                   !                                                                                                  PunctuationToken{tk: "!"}
                        {•(u8)•=>•{•mod•u8•{•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}•};•}    DelimGroup
                          (u8)                                                                                        DelimGroup
                               =>                                                                                     PunctuationToken{tk: "=>"}
                                  {•mod•u8•{•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}•}       DelimGroup
                                           {•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}         DelimGroup
                                                      <                                                               PunctuationToken{tk: "<"}
                                                       'u8                                                            LtIdentifier
                                                          :                                                           PunctuationToken{tk: ":"}
                                                            'u8                                                       LtIdentifier
                                                                +                                                     PunctuationToken{tk: "+"}
                                                                  'u8                                                 LtIdentifier
                                                                     >                                                PunctuationToken{tk: ">"}
                                                                      (u8:•&'u8•u8)                                   DelimGroup
                                                                         :                                            PunctuationToken{tk: ":"}
                                                                           &                                          PunctuationToken{tk: "&"}
                                                                            'u8                                       LtIdentifier
                                                                                    ->                                PunctuationToken{tk: "->"}
                                                                                       &                              PunctuationToken{tk: "&"}
                                                                                        'u8                           LtIdentifier
                                                                                               {•"u8";•u8•}           DelimGroup
                                                                                                 "u8"                 Literal{kind: String}
                                                                                                     ;                PunctuationToken{tk: ";"}
                                                                                                               ;      PunctuationToken{tk: ";"}*/
        let &u8: &u8 = u8::u8(&8u8);                                                                                                      /*
            &                           PunctuationToken{tk: "&"}
               :                        PunctuationToken{tk: ":"}
                 &                      PunctuationToken{tk: "&"}
                     =                  PunctuationToken{tk: "="}
                         ::             PunctuationToken{tk: "::"}
                             (&8u8)     DelimGroup
                              &         PunctuationToken{tk: "&"}
                               8u8      Literal{kind: Integer}
                                u8      Identifier
                                   ;    PunctuationToken{tk: ";"}                                                                         */
        ::u8(0u8);                                                                                                                        /*
        ::            PunctuationToken{tk: "::"}
            (0u8)     DelimGroup
             0u8      Literal{kind: Integer}
              u8      Identifier
                 ;    PunctuationToken{tk: ";"}                                                                                           */
        u8!(u8);                                                                                                                          /*
          !         PunctuationToken{tk: "!"}
           (u8)     DelimGroup
               ;    PunctuationToken{tk: ";"}                                                                                             */
        u8
    });                                                                                                                                   /*
••••}      </DelimGroup>
••••})     </MacroInvocation.segments>
••••})     </MacroInvocation>
••••});    </ExpressionStatement>                                                                                                         */
    let &u8: &u8 = u8::u8(&8u8);                                                                                                          /*
    let•&u8:•&u8•=•u8::u8(&8u8);    LetVariableDeclaration
        &u8                         ReferencePattern{!mut}
             &u8                    TypeReference{!mut}
                   u8::u8(&8u8)     CallExpression
                   u8::u8           ExpressionPath
                         (&8u8)     CallExpression.arguments{dk: "()"}
                          &8u8      ReferenceExpression{!mut}
                           8u8      Literal{kind: Integer}
                            u8      Identifier                                                                                            */
    ::u8(0u8);                                                                                                                            /*
    ::u8(0u8);    ExpressionStatement{semi}
    ::u8(0u8)     CallExpression
    ::u8          ExpressionPath
        (0u8)     CallExpression.arguments{dk: "()"}
         0u8      Literal{kind: Integer}
          u8      Identifier                                                                                                              */
    u8!(u8);                                                                                                                              /*
    u8!(u8);    ExpressionStatement{semi}
    u8!(u8)     MacroInvocation
       (u8)     MacroInvocation.segments{dk: "()"}                                                                                        */
    u8;                                                                                                                                   /*
    u8;    ExpressionStatement{semi}                                                                                                      */
    let µ = 1.0;                                                                                                                          /*
    let•µ•=•1.0;    LetVariableDeclaration
            1.0     Literal{kind: Float}                                                                                                  */
    µ;                                                                                                                                    /*
    µ;    ExpressionStatement{semi}                                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

mod u8 { pub fn u8<'u8: 'u8 + 'u8>(u8: &'u8 u8) -> &'u8 u8 { "u8"; u8 } }                                                                 /*
mod•u8•{•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}    ModuleDeclaration
       {•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}    ModuleDeclaration.body{dk: "{}"}
         pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}      FunctionDeclaration
         pub                                                                 PubSpecifier
                  <'u8:•'u8•+•'u8>                                           FunctionDeclaration.generics{dk: "<>"}
                   'u8:•'u8•+•'u8                                            GenericLtParameterDeclaration
                   'u8                                                       LtIdentifier
                        'u8                                                  LtIdentifier
                              'u8                                            LtIdentifier
                                  (u8:•&'u8•u8)                              FunctionDeclaration.parameters{dk: "()"}
                                   u8:•&'u8•u8                               FunctionParameterDeclaration
                                       &'u8•u8                               TypeReference{!mut}
                                        'u8                                  LtIdentifier
                                                   &'u8•u8                   TypeReference{!mut}
                                                    'u8                      LtIdentifier
                                                           {•"u8";•u8•}      FunctionDeclaration.body{dk: "{}"}
                                                             "u8";           ExpressionStatement{semi}
                                                             "u8"            Literal{kind: String}
                                                                   u8        ExpressionStatement{!semi}
mod•u8•{•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}    </Program.ast>
mod•u8•{•pub•fn•u8<'u8:•'u8•+•'u8>(u8:•&'u8•u8)•->•&'u8•u8•{•"u8";•u8•}•}    </Program>                                                   */
// Discarded Nodes: 0
// Parsed Nodes: 146
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 668 (37% re-reads)
// Unnecessary 'skip_whitespace()' calls: 69
// source: "../../samples/expressions/ident.rs"