#![feature(trait_alias)]                                                                                                                  /*
#![feature(trait_alias)]↲    <Program>
#![feature(trait_alias)]     Attribute{inner}
  [feature(trait_alias)]     Attribute.segments{dk: "[]"}
          (trait_alias)      DelimGroup                                                                                                   */

trait A =;                                                                                                                                /*
trait•A•=;↲    <Program.ast{dk: "None"}>
trait•A•=      TraitAliasDeclaration
         ;     ExpressionStatement{semi}                                                                                                  */
trait A = std::fmt::Debug + Send;                                                                                                         /*
trait•A•=•std::fmt::Debug•+•Send     TraitAliasDeclaration
          std::fmt::Debug            TypeTraitBound{!maybeConst, !optional}, TypePath
          std::fmt                   TypePath
                            Send     TypeTraitBound{!maybeConst, !optional}
                                ;    ExpressionStatement{semi}
trait•A•=•std::fmt::Debug•+•Send;    </Program.ast>
trait•A•=•std::fmt::Debug•+•Send;    </Program>                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 20
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 93 (31% re-reads)
// Unnecessary 'skip_whitespace()' calls: 10
// source: "../../samples/features/trait_alias.rs"