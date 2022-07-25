pub use self::bb::{aa, bb};                                                                                                               /*
pub•use•self::bb::{aa,•bb};    UseStatement
pub                            PubSpecifier
        self::bb::{aa,•bb}     DestructuredImport
        self::bb               ItemPath
                   aa          NamedImport
                       bb      NamedImport                                                                                                */
pub use self::cc::*;                                                                                                                      /*
pub•use•self::cc::*;    UseStatement
pub                     PubSpecifier
        self::cc::*     AmbientImport
        self::cc        ItemPath                                                                                                          */
use Self::f;                                                                                                                              /*
use•Self::f;    UseStatement
    Self::f     NamedImport, ItemPath                                                                                                     */
use ::super::{S, Z};                                                                                                                      /*
use•::super::{S,•Z};    UseStatement
    ::super::{S,•Z}     DestructuredImport
    ::super             ItemPath
              S         NamedImport
                 Z      NamedImport                                                                                                       */
use ::super::main;                                                                                                                        /*
use•::super::main;    UseStatement
    ::super::main     NamedImport, ItemPath
    ::super           ItemPath                                                                                                            */
use a::*;                                                                                                                                 /*
use•a::*;    UseStatement
    a::*     AmbientImport                                                                                                                */
use m::S;                                                                                                                                 /*
use•m::S;    UseStatement
    m::S     NamedImport, ItemPath                                                                                                        */
use {{}, {}};                                                                                                                             /*
use•{{},•{}};    UseStatement
    {{},•{}}     DestructuredImport
     {}          DestructuredImport
         {}      DestructuredImport                                                                                                       */
extern crate x;                                                                                                                           /*
extern•crate•x;    ExternCrateStatement
             x     NamedImport                                                                                                            */
use std::mem::self;                                                                                                                       /*
use•std::mem::self;    UseStatement
    std::mem::self     NamedImport, ItemPath
    std::mem           ItemPath                                                                                                           */
use foo::bar::self as abc;                                                                                                                /*
use•foo::bar::self•as•abc;    UseStatement
    foo::bar::self•as•abc     NamedImport
    foo::bar::self            ItemPath
    foo::bar                  ItemPath                                                                                                    */
extern crate priv_impl_prim_ty as bar;                                                                                                    /*
extern•crate•priv_impl_prim_ty•as•bar;    ExternCrateStatement
             priv_impl_prim_ty•as•bar     NamedImport                                                                                     */
extern crate crate_method_reexport_grrrrrrr2;                                                                                             /*
extern•crate•crate_method_reexport_grrrrrrr2;    ExternCrateStatement
             crate_method_reexport_grrrrrrr2     NamedImport                                                                              */
use std::io::{self, Error as IoError};                                                                                                    /*
use•std::io::{self,•Error•as•IoError};    UseStatement
    std::io::{self,•Error•as•IoError}     DestructuredImport
    std::io                               ItemPath
              self                        NamedImport
                    Error•as•IoError      NamedImport                                                                                     */
use std::net::{self as stdnet, TcpStream};                                                                                                /*
use•std::net::{self•as•stdnet,•TcpStream};    UseStatement
    std::net::{self•as•stdnet,•TcpStream}     DestructuredImport
    std::net                                  ItemPath
               self•as•stdnet                 NamedImport
                               TcpStream      NamedImport                                                                                 */
use foo::{Foo, bar::{baz::{}, foobar::*}, *};                                                                                             /*
use•foo::{Foo,•bar::{baz::{},•foobar::*},•*};    UseStatement
    foo::{Foo,•bar::{baz::{},•foobar::*},•*}     DestructuredImport
          Foo                                    NamedImport
               bar::{baz::{},•foobar::*}         DestructuredImport
                     baz::{}                     DestructuredImport
                              foobar::*          AmbientImport
                                          *      AmbientImport                                                                            */
use foo::bar::baz::{*, *};                                                                                                                /*
use•foo::bar::baz::{*,•*};    UseStatement
    foo::bar::baz::{*,•*}     DestructuredImport
    foo::bar::baz             ItemPath
    foo::bar                  ItemPath
                    *         AmbientImport
                       *      AmbientImport                                                                                               */
use foo::{};                                                                                                                              /*
use•foo::{};    UseStatement
    foo::{}     DestructuredImport                                                                                                        */
mod m {                                                                                                                                   /*
mod•m•{↲    <ModuleDeclaration>                                                                                                           */
	use S;                                                                                                                                /*
    use•S;    UseStatement
        S     NamedImport                                                                                                                 */
	use self::{self};                                                                                                                     /*
    use•self::{self};    UseStatement
        self::{self}     DestructuredImport
               self      NamedImport                                                                                                      */
	use super::{self};                                                                                                                    /*
    use•super::{self};    UseStatement
        super::{self}     DestructuredImport
                self      NamedImport                                                                                                     */
}                                                                                                                                         /*
}    </ModuleDeclaration>                                                                                                                 */
pub use ::E::*;                                                                                                                           /*
pub•use•::E::*;    UseStatement
pub                PubSpecifier
        ::E::*     AmbientImport
        ::E        ItemPath                                                                                                               */
use crate as _;                                                                                                                           /*
use•crate•as•_;    UseStatement
    crate•as•_     AnonymousImport                                                                                                        */
pub use ::E::V::{self};                                                                                                                   /*
pub•use•::E::V::{self};    UseStatement
pub                        PubSpecifier
        ::E::V::{self}     DestructuredImport
        ::E::V             ItemPath
        ::E                ItemPath
                 self      NamedImport                                                                                                    */
use std::{                                                                                                                                /*
use•std::{↲    <UseStatement>
    std::{↲    <DestructuredImport>                                                                                                       */
    ops::A,                                                                                                                               /*
    ops::A     NamedImport, ItemPath                                                                                                      */
    marker::{C, B},                                                                                                                       /*
    marker::{C,•B}     DestructuredImport
             C         NamedImport
                B      NamedImport                                                                                                        */
};                                                                                                                                        /*
};    </UseStatement>
}     </DestructuredImport>                                                                                                               */
mod bar {                                                                                                                                 /*
mod•bar•{↲    <ModuleDeclaration>                                                                                                         */
	pub use bar::*;                                                                                                                       /*
    pub•use•bar::*;    UseStatement
    pub                PubSpecifier
            bar::*     AmbientImport                                                                                                      */
    pub use main as f;                                                                                                                    /*
    pub•use•main•as•f;    UseStatement
    pub                   PubSpecifier
            main•as•f     NamedImport                                                                                                     */
	pub use super::*;                                                                                                                     /*
    pub•use•super::*;    UseStatement
    pub                  PubSpecifier
            super::*     AmbientImport                                                                                                    */
	use ::std::mem;                                                                                                                       /*
    use•::std::mem;    UseStatement
        ::std::mem     NamedImport, ItemPath
        ::std          ItemPath                                                                                                           */
	use crate_method_reexport_grrrrrrr2::rust::add;                                                                                       /*
    use•crate_method_reexport_grrrrrrr2::rust::add;    UseStatement
        crate_method_reexport_grrrrrrr2::rust::add     NamedImport, ItemPath
        crate_method_reexport_grrrrrrr2::rust          ItemPath                                                                           */
    crate struct Foo;                                                                                                                     /*
    crate•struct•Foo;    StructDeclaration
    crate                PubSpecifier                                                                                                     */
}                                                                                                                                         /*
}    </ModuleDeclaration>                                                                                                                 */
use rustc_hir::BinOpKind::{ Add, And, BitAnd, BitOr, BitXor, Div, Eq, Ge, Gtab };                                                         /*
use•rustc_hir::BinOpKind::{•Add,•And,•BitAnd,•BitOr,•BitXor,•Div,•Eq,•Ge,•Gtab•};    UseStatement
    rustc_hir::BinOpKind::{•Add,•And,•BitAnd,•BitOr,•BitXor,•Div,•Eq,•Ge,•Gtab•}     DestructuredImport
    rustc_hir::BinOpKind                                                             ItemPath
                            Add                                                      NamedImport
                                 And                                                 NamedImport
                                      BitAnd                                         NamedImport
                                              BitOr                                  NamedImport
                                                     BitXor                          NamedImport
                                                             Div                     NamedImport
                                                                  Eq                 NamedImport
                                                                      Ge             NamedImport
                                                                          Gtab       NamedImport                                          */
use rustc_ast::ast::{ItemForeignMod, ItemImpl, ItemMac, ItemMod, ItemStatic, ItemDefaultImpl};                                            /*
use•rustc_ast::ast::{ItemForeignMod,•ItemImpl,•ItemMac,•ItemMod,•ItemStatic,•ItemDefaultImpl};    UseStatement
    rustc_ast::ast::{ItemForeignMod,•ItemImpl,•ItemMac,•ItemMod,•ItemStatic,•ItemDefaultImpl}     DestructuredImport
    rustc_ast::ast                                                                                ItemPath
                     ItemForeignMod                                                               NamedImport
                                     ItemImpl                                                     NamedImport
                                               ItemMac                                            NamedImport
                                                        ItemMod                                   NamedImport
                                                                 ItemStatic                       NamedImport
                                                                             ItemDefaultImpl      NamedImport                             */
use exceedingly::looooooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA, ItemB};                            /*
use•exceedingly::looooooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA,•ItemB};    UseStatement
    exceedingly::looooooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA,•ItemB}     DestructuredImport
    exceedingly::looooooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path                     ItemPath
    exceedingly::looooooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import                           ItemPath
    exceedingly::looooooooooooooooooooooooooooooooooooooooooooooooooooooooooong                                   ItemPath
                                                                                                ItemA             NamedImport
                                                                                                       ItemB      NamedImport             */
use exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA, ItemB};                               /*
use•exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA,•ItemB};    UseStatement
    exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA,•ItemB}     DestructuredImport
    exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path                     ItemPath
    exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import                           ItemPath
    exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong                                   ItemPath
                                                                                             ItemA             NamedImport
                                                                                                    ItemB      NamedImport                */

use list::{                                                                                                                               /*
use•list::{↲    <UseStatement>
    list::{↲    <DestructuredImport>                                                                                                      */
    // Some item
    //•Some•item    Comment
    SomeItem /* Comment */, /* Another item */ AnotherItem /* Another Comment */, // Last Item
                                                                                                                                          /*
    SomeItem                                                                                      NamedImport
             /*•Comment•*/                                                                        Comment
                            /*•Another•item•*/                                                    Comment
                                               AnotherItem                                        NamedImport
                                                           /*•Another•Comment•*/                  Comment
                                                                                  //•Last•Item    Comment                                 */
    LastItem                                                                                                                              /*
    LastItem    NamedImport                                                                                                               */
};                                                                                                                                        /*
};    </UseStatement>
}     </DestructuredImport>                                                                                                               */

use test::{  Other          /* C   */  , /*   A   */ self  /*    B     */    };                                                           /*
use•test::{••Other••••••••••/*•C•••*/••,•/*•••A•••*/•self••/*••••B•••••*/••••};    UseStatement
    test::{••Other••••••••••/*•C•••*/••,•/*•••A•••*/•self••/*••••B•••••*/••••}     DestructuredImport
             Other                                                                 NamedImport
                            /*•C•••*/                                              Comment
                                         /*•••A•••*/                               Comment
                                                     self                          NamedImport
                                                           /*••••B•••••*/          Comment                                                */

use rustc_ast::{self};                                                                                                                    /*
use•rustc_ast::{self};    UseStatement
    rustc_ast::{self}     DestructuredImport
                self      NamedImport                                                                                                     */
use {/* Pre-comment! */                                                                                                                   /*
use•{/*•Pre-comment!•*/↲    <UseStatement>
    {/*•Pre-comment!•*/↲    <DestructuredImport>
     /*•Pre-comment!•*/     Comment                                                                                                       */
     Foo, Bar /* comment */};                                                                                                             /*
•••••Foo,•Bar•/*•comment•*/};    </UseStatement>
•••••Foo,•Bar•/*•comment•*/}     </DestructuredImport>
     Foo                         NamedImport
          Bar                    NamedImport
              /*•comment•*/      Comment                                                                                                  */
use Foo::{Bar, Baz};                                                                                                                      /*
use•Foo::{Bar,•Baz};    UseStatement
    Foo::{Bar,•Baz}     DestructuredImport
          Bar           NamedImport
               Baz      NamedImport                                                                                                       */
pub use rustc_ast::ast::{Expr_, Expr, ExprAssign, ExprCall, ExprMethodCall, ExprPath};                                                    /*
pub•use•rustc_ast::ast::{Expr_,•Expr,•ExprAssign,•ExprCall,•ExprMethodCall,•ExprPath};    UseStatement
pub                                                                                       PubSpecifier
        rustc_ast::ast::{Expr_,•Expr,•ExprAssign,•ExprCall,•ExprMethodCall,•ExprPath}     DestructuredImport
        rustc_ast::ast                                                                    ItemPath
                         Expr_                                                            NamedImport
                                Expr                                                      NamedImport
                                      ExprAssign                                          NamedImport
                                                  ExprCall                                NamedImport
                                                            ExprMethodCall                NamedImport
                                                                            ExprPath      NamedImport                                     */

use rustc_ast::some::{};                                                                                                                  /*
use•rustc_ast::some::{};    UseStatement
    rustc_ast::some::{}     DestructuredImport
    rustc_ast::some         ItemPath                                                                                                      */

use self;                                                                                                                                 /*
use•self;    UseStatement
    self     NamedImport                                                                                                                  */
use std::io::{self};                                                                                                                      /*
use•std::io::{self};    UseStatement
    std::io::{self}     DestructuredImport
    std::io             ItemPath
              self      NamedImport                                                                                                       */
use std::io::self;                                                                                                                        /*
use•std::io::self;    UseStatement
    std::io::self     NamedImport, ItemPath
    std::io           ItemPath                                                                                                            */

mod Foo {                                                                                                                                 /*
mod•Foo•{↲    <ModuleDeclaration>                                                                                                         */
    pub use rustc_ast::ast::{ A };                                                                                                        /*
    pub•use•rustc_ast::ast::{•A•};    UseStatement
    pub                               PubSpecifier
            rustc_ast::ast::{•A•}     DestructuredImport
            rustc_ast::ast            ItemPath
                              A       NamedImport                                                                                         */
    mod Foo2 { pub use rustc_ast::ast::{A, self, B}; }                                                                                    /*
    mod•Foo2•{•pub•use•rustc_ast::ast::{A,•self,•B};•}    ModuleDeclaration
               pub•use•rustc_ast::ast::{A,•self,•B};      UseStatement
               pub                                        PubSpecifier
                       rustc_ast::ast::{A,•self,•B}       DestructuredImport
                       rustc_ast::ast                     ItemPath
                                        A                 NamedImport
                                           self           NamedImport
                                                 B        NamedImport                                                                     */
}                                                                                                                                         /*
}    </ModuleDeclaration>                                                                                                                 */

fn test() { use Baz::*; use Qux; }                                                                                                        /*
fn•test()•{•use•Baz::*;•use•Qux;•}    FunctionDeclaration
            use•Baz::*;               UseStatement
                Baz::*                AmbientImport
                        use•Qux;      UseStatement
                            Qux       NamedImport                                                                                         */
use  foo::bar::baz as baz ;                                                                                                               /*
use••foo::bar::baz•as•baz•;    UseStatement
     foo::bar::baz•as•baz      NamedImport
     foo::bar::baz             ItemPath
     foo::bar                  ItemPath                                                                                                   */
use bar::quux  as    kaas;                                                                                                                /*
use•bar::quux••as••••kaas;    UseStatement
    bar::quux••as••••kaas     NamedImport
    bar::quux                 ItemPath                                                                                                    */
use  foo;                                                                                                                                 /*
use••foo;    UseStatement
     foo     NamedImport                                                                                                                  */
use foo::{self as bar, baz};                                                                                                              /*
use•foo::{self•as•bar,•baz};    UseStatement
    foo::{self•as•bar,•baz}     DestructuredImport
          self•as•bar           NamedImport
                       baz      NamedImport                                                                                               */
use foo::{self as bar};                                                                                                                   /*
use•foo::{self•as•bar};    UseStatement
    foo::{self•as•bar}     DestructuredImport
          self•as•bar      NamedImport                                                                                                    */
use foo::{qux as bar};                                                                                                                    /*
use•foo::{qux•as•bar};    UseStatement
    foo::{qux•as•bar}     DestructuredImport
          qux•as•bar      NamedImport                                                                                                     */
use foo::{baz, qux as bar};                                                                                                               /*
use•foo::{baz,•qux•as•bar};    UseStatement
    foo::{baz,•qux•as•bar}     DestructuredImport
          baz                  NamedImport
               qux•as•bar      NamedImport                                                                                                */
use ::foo;                                                                                                                                /*
use•::foo;    UseStatement
    ::foo     NamedImport, ItemPath                                                                                                       */
use ::foo::{Bar};                                                                                                                         /*
use•::foo::{Bar};    UseStatement
    ::foo::{Bar}     DestructuredImport
    ::foo            ItemPath
            Bar      NamedImport                                                                                                          */
use ::foo::{Bar, Baz};                                                                                                                    /*
use•::foo::{Bar,•Baz};    UseStatement
    ::foo::{Bar,•Baz}     DestructuredImport
    ::foo                 ItemPath
            Bar           NamedImport
                 Baz      NamedImport                                                                                                     */
use ::{Foo};                                                                                                                              /*
use•::{Foo};    UseStatement
    ::{Foo}     DestructuredImport
       Foo      NamedImport                                                                                                               */
use ::{Bar, Baz};                                                                                                                         /*
use•::{Bar,•Baz};    UseStatement
    ::{Bar,•Baz}     DestructuredImport
       Bar           NamedImport
            Baz      NamedImport                                                                                                          */
use *;                                                                                                                                    /*
use•*;    UseStatement
    *     AmbientImport                                                                                                                   */
use ::*; (error)                                                                                                                          /*
use•::*;            UseStatement
    ::*             AmbientImport
         (error)    ExpressionStatement                                                                                                   */
use super:: * ;                                                                                                                           /*
use•super::•*•;    UseStatement
    super::•*      AmbientImport                                                                                                          */
use foo::issue_1356:: * ;                                                                                                                 /*
use•foo::issue_1356::•*•;    UseStatement
    foo::issue_1356::•*      AmbientImport
    foo::issue_1356          ItemPath                                                                                                     */
#[cfg(unix)]                                                                                                                              /*
#[cfg(unix)]↲    <UseStatement>
#[cfg(unix)]     Attribute
     (unix)      DelimGroup                                                                                                               */
use self::unix::{};                                                                                                                       /*
use•self::unix::{};    </UseStatement>
    self::unix::{}     DestructuredImport
    self::unix         ItemPath                                                                                                           */
use foo::{a, bar::{baz, qux, xxxxxxxxxxx, yyyyyyyyyyyyy, zzzzzzzzzzzzzzzz, foo::{a, b, cxxxxxxxxxxxxx, yyyyyyyyyyyyyy, zzzzzzzzzzzzzzzz}}, b, boo, c,};/*
use•foo::{a,•bar::{baz,•qux,•xxxxxxxxxxx,•yyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz,•foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}},•b,•boo,•c,};    UseStatement
    foo::{a,•bar::{baz,•qux,•xxxxxxxxxxx,•yyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz,•foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}},•b,•boo,•c,}     DestructuredImport
          a                                                                                                                                                NamedImport
             bar::{baz,•qux,•xxxxxxxxxxx,•yyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz,•foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}}                  DestructuredImport
                   baz                                                                                                                                     NamedImport
                        qux                                                                                                                                NamedImport
                             xxxxxxxxxxx                                                                                                                   NamedImport
                                          yyyyyyyyyyyyy                                                                                                    NamedImport
                                                         zzzzzzzzzzzzzzzz                                                                                  NamedImport
                                                                           foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}                   DestructuredImport
                                                                                 a                                                                         NamedImport
                                                                                    b                                                                      NamedImport
                                                                                       cxxxxxxxxxxxxx                                                      NamedImport
                                                                                                       yyyyyyyyyyyyyy                                      NamedImport
                                                                                                                       zzzzzzzzzzzzzzzz                    NamedImport
                                                                                                                                           b               NamedImport
                                                                                                                                              boo          NamedImport
                                                                                                                                                   c       NamedImport*/
use fooo::{baar::{foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx, yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy, zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}}, z, bar, bar::*, x, y};/*
use•fooo::{baar::{foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,•yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}},•z,•bar,•bar::*,•x,•y};    UseStatement
    fooo::{baar::{foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,•yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}},•z,•bar,•bar::*,•x,•y}     DestructuredImport
           baar::{foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,•yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}}                            DestructuredImport
                  foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,•yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}                             DestructuredImport
                           xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx                                                                                                  NamedImport
                                                             yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy                                                                NamedImport
                                                                                               zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz                              NamedImport
                                                                                                                                   z                         NamedImport
                                                                                                                                      bar                    NamedImport
                                                                                                                                           bar::*            AmbientImport
                                                                                                                                                   x         NamedImport
                                                                                                                                                      y      NamedImport*/
use exonum::{api::{Api, ApiError}, blockchain::{self, BlockProof, Blockchain, Transaction, TransactionSet}, crypto::{Hash, PublicKey}, helpers::Height, node::TransactionSend, storage::{ListProof, MapProof}};/*
use•exonum::{api::{Api,•ApiError},•blockchain::{self,•BlockProof,•Blockchain,•Transaction,•TransactionSet},•crypto::{Hash,•PublicKey},•helpers::Height,•node::TransactionSend,•storage::{ListProof,•MapProof}};    UseStatement
    exonum::{api::{Api,•ApiError},•blockchain::{self,•BlockProof,•Blockchain,•Transaction,•TransactionSet},•crypto::{Hash,•PublicKey},•helpers::Height,•node::TransactionSend,•storage::{ListProof,•MapProof}}     DestructuredImport
             api::{Api,•ApiError}                                                                                                                                                                                  DestructuredImport
                   Api                                                                                                                                                                                             NamedImport
                        ApiError                                                                                                                                                                                   NamedImport
                                   blockchain::{self,•BlockProof,•Blockchain,•Transaction,•TransactionSet}                                                                                                         DestructuredImport
                                                self                                                                                                                                                               NamedImport
                                                      BlockProof                                                                                                                                                   NamedImport
                                                                  Blockchain                                                                                                                                       NamedImport
                                                                              Transaction                                                                                                                          NamedImport
                                                                                           TransactionSet                                                                                                          NamedImport
                                                                                                            crypto::{Hash,•PublicKey}                                                                              DestructuredImport
                                                                                                                     Hash                                                                                          NamedImport
                                                                                                                           PublicKey                                                                               NamedImport
                                                                                                                                       helpers::Height                                                             NamedImport, ItemPath
                                                                                                                                                        node::TransactionSend                                      NamedImport, ItemPath
                                                                                                                                                                               storage::{ListProof,•MapProof}      DestructuredImport
                                                                                                                                                                                         ListProof                 NamedImport
                                                                                                                                                                                                    MapProof       NamedImport*/
use a::{b::{c::*}};                                                                                                                       /*
use•a::{b::{c::*}};    UseStatement
    a::{b::{c::*}}     DestructuredImport
        b::{c::*}      DestructuredImport
            c::*       AmbientImport                                                                                                      */
use a::{b::{c::{}}};                                                                                                                      /*
use•a::{b::{c::{}}};    UseStatement
    a::{b::{c::{}}}     DestructuredImport
        b::{c::{}}      DestructuredImport
            c::{}       DestructuredImport                                                                                                */
use a::{b::{c::d}};                                                                                                                       /*
use•a::{b::{c::d}};    UseStatement
    a::{b::{c::d}}     DestructuredImport
        b::{c::d}      DestructuredImport
            c::d       NamedImport, ItemPath                                                                                              */
use a::{b::{c::{xxx, yyy, zzz}}};                                                                                                         /*
use•a::{b::{c::{xxx,•yyy,•zzz}}};    UseStatement
    a::{b::{c::{xxx,•yyy,•zzz}}}     DestructuredImport
        b::{c::{xxx,•yyy,•zzz}}      DestructuredImport
            c::{xxx,•yyy,•zzz}       DestructuredImport
                xxx                  NamedImport
                     yyy             NamedImport
                          zzz        NamedImport                                                                                          */
/// a
                                                                                                                                          /*
///•a↲    <UseStatement>
///•a     DocCommentAttribute                                                                                                             */
// b
//•b    Comment
use c;                                                                                                                                    /*
use•c;    </UseStatement>
    c     NamedImport                                                                                                                     */
#[macro_use]                                                                                                                              /*
#[macro_use]↲    <UseStatement>
#[macro_use]     Attribute                                                                                                                */
use imports_with_attr;                                                                                                                    /*
use•imports_with_attr;    </UseStatement>
    imports_with_attr     NamedImport                                                                                                     */
use std::f64::consts::{SQRT_2, E, PI};                                                                                                    /*
use•std::f64::consts::{SQRT_2,•E,•PI};    UseStatement
    std::f64::consts::{SQRT_2,•E,•PI}     DestructuredImport
    std::f64::consts                      ItemPath
    std::f64                              ItemPath
                       SQRT_2             NamedImport
                               E          NamedImport
                                  PI      NamedImport                                                                                     */
#[rustfmt::skip]                                                                                                                          /*
#[rustfmt::skip]↲    <UseStatement>
#[rustfmt::skip]     Attribute
         ::          PunctuationToken                                                                                                     */
use std::fmt::{self, {Display, Formatter}};                                                                                               /*
use•std::fmt::{self,•{Display,•Formatter}};    </UseStatement>
    std::fmt::{self,•{Display,•Formatter}}     DestructuredImport
    std::fmt                                   ItemPath
               self                            NamedImport
                     {Display,•Formatter}      DestructuredImport
                      Display                  NamedImport
                               Formatter       NamedImport                                                                                */

// Discarded Nodes: 1
// Parsed Nodes: 647
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 3680 (27% re-reads)
// Unnecessary 'skip_whitespace()' calls: 495
// source: "../../samples/statements/use.rs"