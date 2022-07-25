#!/usr/bin/env bash                                                                                                                       /*
#!/usr/bin/env•bash    Shebang                                                                                                            */
#![forbid(unsafe_code)]/* This line is ignored by bash                                                                                    /*
#![forbid(unsafe_code)]                                    Attribute
         (unsafe_code)                                     DelimGroup
                       /*•This•line•is•ignored•by•bash↲    <Comment>                                                                      */
# This block is ignored by rustc
#*/                                                                                                                                       /*
#*/    </Comment>                                                                                                                         */

//!
//!•    DocCommentAttribute

use std;                                                                                                                                  /*
use•std;    UseStatement
    std     NamedImport                                                                                                                   */

// Discarded Nodes: 0
// Parsed Nodes: 12
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 139 (9% re-reads)
// Unnecessary 'skip_whitespace()' calls: 5
// source: "../../samples/miscellaneous/shebang-b.rs"