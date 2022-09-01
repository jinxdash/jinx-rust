#!/usr/bin/env bash                                                                                                                       /*
#!/usr/bin/env•bash    Shebang                                                                                                            */
#![forbid(unsafe_code)]/* This line is ignored by bash                                                                                    /*
#![forbid(unsafe_code)]/*•This•line•is•ignored•by•bash↲    <Program>
#![forbid(unsafe_code)]                                    Attribute{inner}
  [forbid(unsafe_code)]                                    Attribute.segments{dk: "[]"}
         (unsafe_code)                                     DelimGroup
                       /*•This•line•is•ignored•by•bash↲    <Comment{!line}>                                                           */*/*/
# This block is ignored by rustc
#*/                                                                                                                                     /*/*
#*/    </Comment>                                                                                                                         */

//! 
//!•    DocCommentAttribute{inner, line}

use std;                                                                                                                                  /*
use•std;    Program.ast{dk: "None"}
use•std;    UseStatement
    std     NamedImport
use•std;    </Program>                                                                                                                    */
// Discarded Nodes: 0
// Parsed Nodes: 12
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 139 (9% re-reads)
// Unnecessary 'skip_whitespace()' calls: 5
// source: "../../samples/miscellaneous/shebang-b.rs"