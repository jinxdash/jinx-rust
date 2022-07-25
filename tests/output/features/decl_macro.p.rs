#![feature(decl_macro)]                                                                                                                   /*
#![feature(decl_macro)]    Attribute
          (decl_macro)     DelimGroup                                                                                                     */

macro m() {}                                                                                                                              /*
macro•m()•{}    MacroDeclaration
       ()•{}    MacroInlineRuleDeclaration                                                                                                */

// Discarded Nodes: 0
// Parsed Nodes: 9
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 48 (30% re-reads)
// Unnecessary 'skip_whitespace()' calls: 4
// source: "../../samples/features/decl_macro.rs"