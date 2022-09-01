#!shebang                                                                                                                                 /*
#!shebang    Shebang                                                                                                                      */
// comment
//•comment↲    <Program>
//•comment     Comment{line}
#[attr]                                                                                                                                   /*
#[attr]↲    <Program.ast{dk: "None"}>
#[attr]↲    <StructDeclaration>
#[attr]     Attribute{!inner}
 [attr]     Attribute.segments{dk: "[]"}                                                                                                  */
struct T;                                                                                                                                 /*
struct•T;    StructDeclaration~ownStart
struct•T;    </StructDeclaration>
struct•T;    </Program.ast>                                                                                                               */
// comment
//•comment    Comment{line}
//•comment    </Program>
// Discarded Nodes: 0
// Parsed Nodes: 9
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 59 (20% re-reads)
// Unnecessary 'skip_whitespace()' calls: 2
// source: "../../samples/miscellaneous/ast-program-locs-attr.rs"