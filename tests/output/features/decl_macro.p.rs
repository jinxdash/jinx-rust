#![feature(decl_macro)]                                                                                                                   /*
#![feature(decl_macro)]↲    <Program>
#![feature(decl_macro)]     Attribute{inner}
  [feature(decl_macro)]     Attribute.segments{dk: "[]"}
          (decl_macro)      DelimGroup                                                                                                    */

macro m() {}                                                                                                                              /*
macro•m()•{}    Program.ast{dk: "None"}
macro•m()•{}    MacroDeclaration
       ()•{}    MacroInlineRuleDeclaration
       ()       MacroInlineRuleDeclaration.match{dk: "()"}
          {}    MacroInlineRuleDeclaration.transform{dk: "{}"}
macro•m()•{}    </Program>                                                                                                                */
// Discarded Nodes: 0
// Parsed Nodes: 9
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 48 (30% re-reads)
// Unnecessary 'skip_whitespace()' calls: 4
// source: "../../samples/features/decl_macro.rs"