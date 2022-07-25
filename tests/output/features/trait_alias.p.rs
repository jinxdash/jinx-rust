#![feature(trait_alias)]                                                                                                                  /*
#![feature(trait_alias)]    Attribute
          (trait_alias)     DelimGroup                                                                                                    */

trait A =;                                                                                                                                /*
trait•A•=     TraitAliasDeclaration
         ;    ExpressionStatement                                                                                                         */
trait A = std::fmt::Debug + Send;                                                                                                         /*
trait•A•=•std::fmt::Debug•+•Send     TraitAliasDeclaration
          std::fmt::Debug            TypeTraitBound, TypePath
          std::fmt                   TypePath
                            Send     TypeTraitBound
                                ;    ExpressionStatement                                                                                  */

// Discarded Nodes: 0
// Parsed Nodes: 20
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 93 (31% re-reads)
// Unnecessary 'skip_whitespace()' calls: 10
// source: "../../samples/features/trait_alias.rs"