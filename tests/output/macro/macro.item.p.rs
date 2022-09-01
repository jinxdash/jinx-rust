macro_rules ! spaced {}                                                                                                                   /*
macro_rules•!•spaced•{}↲    <Program>
macro_rules•!•spaced•{}↲    <Program.ast{dk: "None"}>
macro_rules•!•spaced•{}     MacroRulesDeclaration
                     {}     MacroRulesDeclaration.rules{dk: "{}"}                                                                         */
macro_rules! brace { () => { } ; }                                                                                                        /*
macro_rules!•brace•{•()•=>•{•}•;•}    MacroRulesDeclaration
                   {•()•=>•{•}•;•}    MacroRulesDeclaration.rules{dk: "{}"}
                     ()•=>•{•}        MacroRuleDeclaration
                     ()               MacroRuleDeclaration.match{dk: "()"}
                           {•}        MacroRuleDeclaration.transform{dk: "{}"}                                                            */
macro_rules! bracket[() => { } ;];                                                                                                        /*
macro_rules!•bracket[()•=>•{•}•;];    MacroRulesDeclaration
                    [()•=>•{•}•;]     MacroRulesDeclaration.rules{dk: "[]"}
                     ()•=>•{•}        MacroRuleDeclaration
                     ()               MacroRuleDeclaration.match{dk: "()"}
                           {•}        MacroRuleDeclaration.transform{dk: "{}"}                                                            */
macro_rules! paren(() => { } ;);                                                                                                          /*
macro_rules!•paren(()•=>•{•}•;);    MacroRulesDeclaration
                  (()•=>•{•}•;)     MacroRulesDeclaration.rules{dk: "()"}
                   ()•=>•{•}        MacroRuleDeclaration
                   ()               MacroRuleDeclaration.match{dk: "()"}
                         {•}        MacroRuleDeclaration.transform{dk: "{}"}                                                              */
macro_rules! macro_rules { () => {} }                                                                                                     /*
macro_rules!•macro_rules•{•()•=>•{}•}    MacroRulesDeclaration
                         {•()•=>•{}•}    MacroRulesDeclaration.rules{dk: "{}"}
                           ()•=>•{}      MacroRuleDeclaration
                           ()            MacroRuleDeclaration.match{dk: "()"}
                                 {}      MacroRuleDeclaration.transform{dk: "{}"}                                                         */
macro_rules! {}                                                                                                                           /*
macro_rules!•{}    ExpressionStatement{!semi}, MacroInvocation
             {}    MacroInvocation.segments{dk: "{}"}                                                                                     */

macro m($S:ident, $x:ident) {}                                                                                                            /*
macro•m($S:ident,•$x:ident)•{}    MacroDeclaration
       ($S:ident,•$x:ident)•{}    MacroInlineRuleDeclaration
       ($S:ident,•$x:ident)       MacroInlineRuleDeclaration.match{dk: "()"}
        $S:ident                  MacroParameterDeclaration
        $S                        McIdentifier
                ,                 PunctuationToken{tk: ","}
                  $x:ident        MacroParameterDeclaration
                  $x              McIdentifier
                            {}    MacroInlineRuleDeclaration.transform{dk: "{}"}                                                          */
pub macro create_struct($a:ident) {}                                                                                                      /*
pub•macro•create_struct($a:ident)•{}    MacroDeclaration
pub                                     PubSpecifier
                       ($a:ident)•{}    MacroInlineRuleDeclaration
                       ($a:ident)       MacroInlineRuleDeclaration.match{dk: "()"}
                        $a:ident        MacroParameterDeclaration
                        $a              McIdentifier
                                  {}    MacroInlineRuleDeclaration.transform{dk: "{}"}                                                    */
pub(crate) macro mac { ($arg : expr) => { $arg + $arg } }                                                                                 /*
pub(crate)•macro•mac•{•($arg•:•expr)•=>•{•$arg•+•$arg•}•}    MacroDeclaration
pub(crate)                                                   PubSpecifier
                     {•($arg•:•expr)•=>•{•$arg•+•$arg•}•}    MacroDeclaration.rules{dk: "{}"}
                       ($arg•:•expr)•=>•{•$arg•+•$arg•}      MacroRuleDeclaration
                       ($arg•:•expr)                         MacroRuleDeclaration.match{dk: "()"}
                        $arg•:•expr                          MacroParameterDeclaration
                        $arg                                 McIdentifier
                                        {•$arg•+•$arg•}      MacroRuleDeclaration.transform{dk: "{}"}
                                          $arg               McIdentifier
                                               +             PunctuationToken{tk: "+"}
                                                 $arg        McIdentifier
pub(crate)•macro•mac•{•($arg•:•expr)•=>•{•$arg•+•$arg•}•}    </Program.ast>
pub(crate)•macro•mac•{•($arg•:•expr)•=>•{•$arg•+•$arg•}•}    </Program>                                                                   */
// Discarded Nodes: 0
// Parsed Nodes: 47
// state_rollbacks: 8
// Total '.charCodeAt()' calls: 388 (25% re-reads)
// Unnecessary 'skip_whitespace()' calls: 16
// source: "../../samples/macro/macro.item.rs"