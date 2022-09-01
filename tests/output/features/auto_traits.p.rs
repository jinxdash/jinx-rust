#![feature(auto_traits)]                                                                                                                  /*
#![feature(auto_traits)]↲    <Program>
#![feature(auto_traits)]     Attribute{inner}
  [feature(auto_traits)]     Attribute.segments{dk: "[]"}
          (auto_traits)      DelimGroup                                                                                                   */

auto trait T {}                                                                                                                           /*
auto•trait•T•{}↲    <Program.ast{dk: "None"}>
auto•trait•T•{}     AutoTraitDeclaration                                                                                                  */
unsafe auto trait T {}                                                                                                                    /*
unsafe•auto•trait•T•{}    AutoTraitDeclaration{unsafe}                                                                                    */
pub auto trait T {}                                                                                                                       /*
pub•auto•trait•T•{}    AutoTraitDeclaration
pub                    PubSpecifier                                                                                                       */
pub unsafe auto trait T {}                                                                                                                /*
pub•unsafe•auto•trait•T•{}    AutoTraitDeclaration{unsafe}
pub                           PubSpecifier                                                                                                */

auto trait T {                                                                                                                            /*
auto•trait•T•{↲    <AutoTraitDeclaration>                                                                                                 */
	#![attr]                                                                                                                              /*
	#![attr]    Attribute{inner}
	  [attr]    Attribute.segments{dk: "[]"}                                                                                              */
}                                                                                                                                         /*
}    </AutoTraitDeclaration>                                                                                                              */

#[attr_0]                                                                                                                                 /*
#[attr_0]↲    <AutoTraitDeclaration>
#[attr_0]     Attribute{!inner}
 [attr_0]     Attribute.segments{dk: "[]"}                                                                                                */
auto trait T {                                                                                                                            /*
auto•trait•T•{↲    AutoTraitDeclaration~ownStart                                                                                          */
	#![attr_1] // comment
/*	#![attr_1]               Attribute{inner}
	  [attr_1]               Attribute.segments{dk: "[]"}                                                                                 */
	           //•comment    Comment{line}
}                                                                                                                                         /*
}    </AutoTraitDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 27
// state_rollbacks: 8
// Total '.charCodeAt()' calls: 310 (62% re-reads)
// Unnecessary 'skip_whitespace()' calls: 14
// source: "../../samples/features/auto_traits.rs"