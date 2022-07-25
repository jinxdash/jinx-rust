#![feature(auto_traits)]                                                                                                                  /*
#![feature(auto_traits)]    Attribute
          (auto_traits)     DelimGroup                                                                                                    */

auto trait T {}                                                                                                                           /*
auto•trait•T•{}    AutoTraitDeclaration                                                                                                   */
unsafe auto trait T {}                                                                                                                    /*
unsafe•auto•trait•T•{}    AutoTraitDeclaration                                                                                            */
pub auto trait T {}                                                                                                                       /*
pub•auto•trait•T•{}    AutoTraitDeclaration
pub                    PubSpecifier                                                                                                       */
pub unsafe auto trait T {}                                                                                                                /*
pub•unsafe•auto•trait•T•{}    AutoTraitDeclaration
pub                           PubSpecifier                                                                                                */

auto trait T {                                                                                                                            /*
auto•trait•T•{↲    <AutoTraitDeclaration>                                                                                                 */
	#![attr]                                                                                                                              /*
    #![attr]    Attribute                                                                                                                 */
}                                                                                                                                         /*
}    </AutoTraitDeclaration>                                                                                                              */

#[attr_0]                                                                                                                                 /*
#[attr_0]↲    <AutoTraitDeclaration>
#[attr_0]     Attribute                                                                                                                   */
auto trait T {
	#![attr_1] // comment
                                                                                                                                          /*
    #![attr_1]               Attribute
               //•comment    Comment                                                                                                      */
}                                                                                                                                         /*
}    </AutoTraitDeclaration>                                                                                                              */

// Discarded Nodes: 0
// Parsed Nodes: 27
// state_rollbacks: 8
// Total '.charCodeAt()' calls: 310 (62% re-reads)
// Unnecessary 'skip_whitespace()' calls: 14
// source: "../../samples/features/auto_traits.rs"