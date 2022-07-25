macro_rules! brace { () => { } ; }                                                                                                        /*
macro_rules!•brace•{•()•=>•{•}•;•}    MacroRulesDeclaration
                     ()•=>•{•}        MacroRuleDeclaration                                                                                */
macro_rules! bracket[() => { } ;];                                                                                                        /*
macro_rules!•bracket[()•=>•{•}•;];    MacroRulesDeclaration
                     ()•=>•{•}        MacroRuleDeclaration                                                                                */
macro_rules! paren(() => { } ;);                                                                                                          /*
macro_rules!•paren(()•=>•{•}•;);    MacroRulesDeclaration
                   ()•=>•{•}        MacroRuleDeclaration                                                                                  */
macro_rules! macro_rules { () => {} }                                                                                                     /*
macro_rules!•macro_rules•{•()•=>•{}•}    MacroRulesDeclaration
                           ()•=>•{}      MacroRuleDeclaration                                                                             */
macro_rules! {}                                                                                                                           /*
macro_rules!•{}    ExpressionStatement, MacroInvocation                                                                                   */

macro m($S:ident, $x:ident) {}                                                                                                            /*
macro•m($S:ident,•$x:ident)•{}    MacroDeclaration
       ($S:ident,•$x:ident)•{}    MacroInlineRuleDeclaration
        $S:ident                  MacroParameterDeclaration
        $S                        McIdentifier
                ,                 PunctuationToken
                  $x:ident        MacroParameterDeclaration
                  $x              McIdentifier                                                                                            */
pub macro create_struct($a:ident) {}                                                                                                      /*
pub•macro•create_struct($a:ident)•{}    MacroDeclaration
pub                                     PubSpecifier
                       ($a:ident)•{}    MacroInlineRuleDeclaration
                        $a:ident        MacroParameterDeclaration
                        $a              McIdentifier                                                                                      */
pub(crate) macro mac { ($arg : expr) => { $arg + $arg } }                                                                                 /*
pub(crate)•macro•mac•{•($arg•:•expr)•=>•{•$arg•+•$arg•}•}    MacroDeclaration
pub(crate)                                                   PubSpecifier
                       ($arg•:•expr)•=>•{•$arg•+•$arg•}      MacroRuleDeclaration
                        $arg•:•expr                          MacroParameterDeclaration
                        $arg                                 McIdentifier
                                          $arg               McIdentifier
                                               +             PunctuationToken
                                                 $arg        McIdentifier                                                                 */

// Discarded Nodes: 0
// Parsed Nodes: 45
// state_rollbacks: 5
// Total '.charCodeAt()' calls: 351 (23% re-reads)
// Unnecessary 'skip_whitespace()' calls: 14
// source: "../../samples/macro/macro.item.rs"