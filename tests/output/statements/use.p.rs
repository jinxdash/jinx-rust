pub use self::bb::{aa, bb};                                                                                                               /*
pub•use•self::bb::{aa,•bb};↲    <Program>
pub•use•self::bb::{aa,•bb};↲    <Program.ast{dk: "None"}>
pub•use•self::bb::{aa,•bb};     UseStatement
pub                             PubSpecifier
        self::bb::{aa,•bb}      DestructuredImport
        self::bb                ItemPath
                  {aa,•bb}      DestructuredImport.specifiers{dk: "{}"}
                   aa           NamedImport
                       bb       NamedImport                                                                                               */
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
             {S,•Z}     DestructuredImport.specifiers{dk: "{}"}
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
             {self,•Error•as•IoError}     DestructuredImport.specifiers{dk: "{}"}
              self                        NamedImport
                    Error•as•IoError      NamedImport                                                                                     */
use std::net::{self as stdnet, TcpStream};                                                                                                /*
use•std::net::{self•as•stdnet,•TcpStream};    UseStatement
    std::net::{self•as•stdnet,•TcpStream}     DestructuredImport
    std::net                                  ItemPath
              {self•as•stdnet,•TcpStream}     DestructuredImport.specifiers{dk: "{}"}
               self•as•stdnet                 NamedImport
                               TcpStream      NamedImport                                                                                 */
use foo::{Foo, bar::{baz::{}, foobar::*}, *};                                                                                             /*
use•foo::{Foo,•bar::{baz::{},•foobar::*},•*};    UseStatement
    foo::{Foo,•bar::{baz::{},•foobar::*},•*}     DestructuredImport
         {Foo,•bar::{baz::{},•foobar::*},•*}     DestructuredImport.specifiers{dk: "{}"}
          Foo                                    NamedImport
               bar::{baz::{},•foobar::*}         DestructuredImport
                    {baz::{},•foobar::*}         DestructuredImport.specifiers{dk: "{}"}
                     baz::{}                     DestructuredImport
                          {}                     DestructuredImport.specifiers{dk: "{}"}
                              foobar::*          AmbientImport
                                          *      AmbientImport                                                                            */
use foo::bar::baz::{*, *};                                                                                                                /*
use•foo::bar::baz::{*,•*};    UseStatement
    foo::bar::baz::{*,•*}     DestructuredImport
    foo::bar::baz             ItemPath
    foo::bar                  ItemPath
                   {*,•*}     DestructuredImport.specifiers{dk: "{}"}
                    *         AmbientImport
                       *      AmbientImport                                                                                               */
use foo::{};                                                                                                                              /*
use•foo::{};    UseStatement
    foo::{}     DestructuredImport
         {}     DestructuredImport.specifiers{dk: "{}"}                                                                                   */
mod m {                                                                                                                                   /*
mod•m•{↲    <ModuleDeclaration>
      {↲    <ModuleDeclaration.body{dk: "{}"}>                                                                                            */
	use S;                                                                                                                                /*
	use•S;    UseStatement
	    S     NamedImport                                                                                                                 */
	use self::{self};                                                                                                                     /*
	use•self::{self};    UseStatement
	    self::{self}     DestructuredImport
	          {self}     DestructuredImport.specifiers{dk: "{}"}
	           self      NamedImport                                                                                                      */
	use super::{self};                                                                                                                    /*
	use•super::{self};    UseStatement
	    super::{self}     DestructuredImport
	           {self}     DestructuredImport.specifiers{dk: "{}"}
	            self      NamedImport                                                                                                     */
}                                                                                                                                         /*
}    </ModuleDeclaration.body>
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
                {self}     DestructuredImport.specifiers{dk: "{}"}
                 self      NamedImport                                                                                                    */
use std::{                                                                                                                                /*
use•std::{↲    <UseStatement>
    std::{↲    <DestructuredImport>
         {↲    <DestructuredImport.specifiers{dk: "{}"}>                                                                                  */
    ops::A,                                                                                                                               /*
    ops::A    NamedImport, ItemPath                                                                                                       */
    marker::{C, B},                                                                                                                       /*
    marker::{C,•B}    DestructuredImport
            {C,•B}    DestructuredImport.specifiers{dk: "{}"}
             C        NamedImport
                B     NamedImport                                                                                                         */
};                                                                                                                                        /*
}     </DestructuredImport.specifiers>
}     </DestructuredImport>
};    </UseStatement>                                                                                                                     */
mod bar {                                                                                                                                 /*
mod•bar•{↲    <ModuleDeclaration>
        {↲    <ModuleDeclaration.body{dk: "{}"}>                                                                                          */
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
}    </ModuleDeclaration.body>
}    </ModuleDeclaration>                                                                                                                 */
use rustc_hir::BinOpKind::{ Add, And, BitAnd, BitOr, BitXor, Div, Eq, Ge, Gtab };                                                         /*
use•rustc_hir::BinOpKind::{•Add,•And,•BitAnd,•BitOr,•BitXor,•Div,•Eq,•Ge,•Gtab•};    UseStatement
    rustc_hir::BinOpKind::{•Add,•And,•BitAnd,•BitOr,•BitXor,•Div,•Eq,•Ge,•Gtab•}     DestructuredImport
    rustc_hir::BinOpKind                                                             ItemPath
                          {•Add,•And,•BitAnd,•BitOr,•BitXor,•Div,•Eq,•Ge,•Gtab•}     DestructuredImport.specifiers{dk: "{}"}
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
                    {ItemForeignMod,•ItemImpl,•ItemMac,•ItemMod,•ItemStatic,•ItemDefaultImpl}     DestructuredImport.specifiers{dk: "{}"}
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
                                                                                               {ItemA,•ItemB}     DestructuredImport.specifiers{dk: "{}"}
                                                                                                ItemA             NamedImport
                                                                                                       ItemB      NamedImport             */
use exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA, ItemB};                               /*
use•exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA,•ItemB};    UseStatement
    exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA,•ItemB}     DestructuredImport
    exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path                     ItemPath
    exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import                           ItemPath
    exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong                                   ItemPath
                                                                                            {ItemA,•ItemB}     DestructuredImport.specifiers{dk: "{}"}
                                                                                             ItemA             NamedImport
                                                                                                    ItemB      NamedImport                */

use list::{                                                                                                                               /*
use•list::{↲    <UseStatement>
    list::{↲    <DestructuredImport>
          {↲    <DestructuredImport.specifiers{dk: "{}"}>                                                                                 */
    // Some item
    //•Some•item    Comment{line}
    SomeItem /* Comment */, /* Another item */ AnotherItem /* Another Comment */, // Last Item
/*  SomeItem                                                                                      NamedImport
             /*•Comment•*/                                                                        Comment{!line}
                            /*•Another•item•*/                                                    Comment{!line}
                                               AnotherItem                                        NamedImport
                                                           /*•Another•Comment•*/                  Comment{!line}                          */
                                                                                  //•Last•Item    Comment{line}
    LastItem                                                                                                                              /*
    LastItem    NamedImport                                                                                                               */
};                                                                                                                                        /*
}     </DestructuredImport.specifiers>
}     </DestructuredImport>
};    </UseStatement>                                                                                                                     */

use test::{  Other          /* C   */  , /*   A   */ self  /*    B     */    };                                                           /*
use•test::{••Other••••••••••/*•C•••*/••,•/*•••A•••*/•self••/*••••B•••••*/••••};    UseStatement
    test::{••Other••••••••••/*•C•••*/••,•/*•••A•••*/•self••/*••••B•••••*/••••}     DestructuredImport
          {••Other••••••••••/*•C•••*/••,•/*•••A•••*/•self••/*••••B•••••*/••••}     DestructuredImport.specifiers{dk: "{}"}
             Other                                                                 NamedImport
                            /*•C•••*/                                              Comment{!line}
                                         /*•••A•••*/                               Comment{!line}
                                                     self                          NamedImport
                                                           /*••••B•••••*/          Comment{!line}                                         */

use rustc_ast::{self};                                                                                                                    /*
use•rustc_ast::{self};    UseStatement
    rustc_ast::{self}     DestructuredImport
               {self}     DestructuredImport.specifiers{dk: "{}"}
                self      NamedImport                                                                                                     */
use {/* Pre-comment! */                                                                                                                   /*
use•{/*•Pre-comment!•*/↲    <UseStatement>
    {/*•Pre-comment!•*/↲    <DestructuredImport>
     /*•Pre-comment!•*/     Comment{!line}                                                                                                */
     Foo, Bar /* comment */};                                                                                                             /*
     Foo                         NamedImport
          Bar                    NamedImport
              /*•comment•*/      Comment{!line}
•••••Foo,•Bar•/*•comment•*/}     </DestructuredImport>
•••••Foo,•Bar•/*•comment•*/};    </UseStatement>                                                                                          */
use Foo::{Bar, Baz};                                                                                                                      /*
use•Foo::{Bar,•Baz};    UseStatement
    Foo::{Bar,•Baz}     DestructuredImport
         {Bar,•Baz}     DestructuredImport.specifiers{dk: "{}"}
          Bar           NamedImport
               Baz      NamedImport                                                                                                       */
pub use rustc_ast::ast::{Expr_, Expr, ExprAssign, ExprCall, ExprMethodCall, ExprPath};                                                    /*
pub•use•rustc_ast::ast::{Expr_,•Expr,•ExprAssign,•ExprCall,•ExprMethodCall,•ExprPath};    UseStatement
pub                                                                                       PubSpecifier
        rustc_ast::ast::{Expr_,•Expr,•ExprAssign,•ExprCall,•ExprMethodCall,•ExprPath}     DestructuredImport
        rustc_ast::ast                                                                    ItemPath
                        {Expr_,•Expr,•ExprAssign,•ExprCall,•ExprMethodCall,•ExprPath}     DestructuredImport.specifiers{dk: "{}"}
                         Expr_                                                            NamedImport
                                Expr                                                      NamedImport
                                      ExprAssign                                          NamedImport
                                                  ExprCall                                NamedImport
                                                            ExprMethodCall                NamedImport
                                                                            ExprPath      NamedImport                                     */

use rustc_ast::some::{};                                                                                                                  /*
use•rustc_ast::some::{};    UseStatement
    rustc_ast::some::{}     DestructuredImport
    rustc_ast::some         ItemPath
                     {}     DestructuredImport.specifiers{dk: "{}"}                                                                       */

use self;                                                                                                                                 /*
use•self;    UseStatement
    self     NamedImport                                                                                                                  */
use std::io::{self};                                                                                                                      /*
use•std::io::{self};    UseStatement
    std::io::{self}     DestructuredImport
    std::io             ItemPath
             {self}     DestructuredImport.specifiers{dk: "{}"}
              self      NamedImport                                                                                                       */
use std::io::self;                                                                                                                        /*
use•std::io::self;    UseStatement
    std::io::self     NamedImport, ItemPath
    std::io           ItemPath                                                                                                            */

mod Foo {                                                                                                                                 /*
mod•Foo•{↲    <ModuleDeclaration>
        {↲    <ModuleDeclaration.body{dk: "{}"}>                                                                                          */
    pub use rustc_ast::ast::{ A };                                                                                                        /*
    pub•use•rustc_ast::ast::{•A•};    UseStatement
    pub                               PubSpecifier
            rustc_ast::ast::{•A•}     DestructuredImport
            rustc_ast::ast            ItemPath
                            {•A•}     DestructuredImport.specifiers{dk: "{}"}
                              A       NamedImport                                                                                         */
    mod Foo2 { pub use rustc_ast::ast::{A, self, B}; }                                                                                    /*
    mod•Foo2•{•pub•use•rustc_ast::ast::{A,•self,•B};•}    ModuleDeclaration
             {•pub•use•rustc_ast::ast::{A,•self,•B};•}    ModuleDeclaration.body{dk: "{}"}
               pub•use•rustc_ast::ast::{A,•self,•B};      UseStatement
               pub                                        PubSpecifier
                       rustc_ast::ast::{A,•self,•B}       DestructuredImport
                       rustc_ast::ast                     ItemPath
                                       {A,•self,•B}       DestructuredImport.specifiers{dk: "{}"}
                                        A                 NamedImport
                                           self           NamedImport
                                                 B        NamedImport                                                                     */
}                                                                                                                                         /*
}    </ModuleDeclaration.body>
}    </ModuleDeclaration>                                                                                                                 */

fn test() { use Baz::*; use Qux; }                                                                                                        /*
fn•test()•{•use•Baz::*;•use•Qux;•}    FunctionDeclaration
       ()                             FunctionDeclaration.parameters{dk: "()"}
          {•use•Baz::*;•use•Qux;•}    FunctionDeclaration.body{dk: "{}"}
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
         {self•as•bar,•baz}     DestructuredImport.specifiers{dk: "{}"}
          self•as•bar           NamedImport
                       baz      NamedImport                                                                                               */
use foo::{self as bar};                                                                                                                   /*
use•foo::{self•as•bar};    UseStatement
    foo::{self•as•bar}     DestructuredImport
         {self•as•bar}     DestructuredImport.specifiers{dk: "{}"}
          self•as•bar      NamedImport                                                                                                    */
use foo::{qux as bar};                                                                                                                    /*
use•foo::{qux•as•bar};    UseStatement
    foo::{qux•as•bar}     DestructuredImport
         {qux•as•bar}     DestructuredImport.specifiers{dk: "{}"}
          qux•as•bar      NamedImport                                                                                                     */
use foo::{baz, qux as bar};                                                                                                               /*
use•foo::{baz,•qux•as•bar};    UseStatement
    foo::{baz,•qux•as•bar}     DestructuredImport
         {baz,•qux•as•bar}     DestructuredImport.specifiers{dk: "{}"}
          baz                  NamedImport
               qux•as•bar      NamedImport                                                                                                */
use ::foo;                                                                                                                                /*
use•::foo;    UseStatement
    ::foo     NamedImport, ItemPath                                                                                                       */
use ::foo::{Bar};                                                                                                                         /*
use•::foo::{Bar};    UseStatement
    ::foo::{Bar}     DestructuredImport
    ::foo            ItemPath
           {Bar}     DestructuredImport.specifiers{dk: "{}"}
            Bar      NamedImport                                                                                                          */
use ::foo::{Bar, Baz};                                                                                                                    /*
use•::foo::{Bar,•Baz};    UseStatement
    ::foo::{Bar,•Baz}     DestructuredImport
    ::foo                 ItemPath
           {Bar,•Baz}     DestructuredImport.specifiers{dk: "{}"}
            Bar           NamedImport
                 Baz      NamedImport                                                                                                     */
use ::{Foo};                                                                                                                              /*
use•::{Foo};    UseStatement
    ::{Foo}     DestructuredImport
      {Foo}     DestructuredImport.specifiers{dk: "{}"}
       Foo      NamedImport                                                                                                               */
use ::{Bar, Baz};                                                                                                                         /*
use•::{Bar,•Baz};    UseStatement
    ::{Bar,•Baz}     DestructuredImport
      {Bar,•Baz}     DestructuredImport.specifiers{dk: "{}"}
       Bar           NamedImport
            Baz      NamedImport                                                                                                          */
use *;                                                                                                                                    /*
use•*;    UseStatement
    *     AmbientImport                                                                                                                   */
use ::*; (error)                                                                                                                          /*
use•::*;            UseStatement
    ::*             AmbientImport
         (error)    ExpressionStatement{!semi}                                                                                            */
use super:: * ;                                                                                                                           /*
use•super::•*•;    UseStatement
    super::•*      AmbientImport                                                                                                          */
use foo::issue_1356:: * ;                                                                                                                 /*
use•foo::issue_1356::•*•;    UseStatement
    foo::issue_1356::•*      AmbientImport
    foo::issue_1356          ItemPath                                                                                                     */
#[cfg(unix)]                                                                                                                              /*
#[cfg(unix)]↲    <UseStatement>
#[cfg(unix)]     Attribute{!inner}
 [cfg(unix)]     Attribute.segments{dk: "[]"}
     (unix)      DelimGroup                                                                                                               */
use self::unix::{};                                                                                                                       /*
use•self::unix::{};    UseStatement~ownStart
    self::unix::{}     DestructuredImport
    self::unix         ItemPath
                {}     DestructuredImport.specifiers{dk: "{}"}
use•self::unix::{};    </UseStatement>                                                                                                    */
use foo::{a, bar::{baz, qux, xxxxxxxxxxx, yyyyyyyyyyyyy, zzzzzzzzzzzzzzzz, foo::{a, b, cxxxxxxxxxxxxx, yyyyyyyyyyyyyy, zzzzzzzzzzzzzzzz}}, b, boo, c,};/*
use•foo::{a,•bar::{baz,•qux,•xxxxxxxxxxx,•yyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz,•foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}},•b,•boo,•c,};    UseStatement
    foo::{a,•bar::{baz,•qux,•xxxxxxxxxxx,•yyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz,•foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}},•b,•boo,•c,}     DestructuredImport
         {a,•bar::{baz,•qux,•xxxxxxxxxxx,•yyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz,•foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}},•b,•boo,•c,}     DestructuredImport.specifiers{dk: "{}"}
          a                                                                                                                                                NamedImport
             bar::{baz,•qux,•xxxxxxxxxxx,•yyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz,•foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}}                  DestructuredImport
                  {baz,•qux,•xxxxxxxxxxx,•yyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz,•foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}}                  DestructuredImport.specifiers{dk: "{}"}
                   baz                                                                                                                                     NamedImport
                        qux                                                                                                                                NamedImport
                             xxxxxxxxxxx                                                                                                                   NamedImport
                                          yyyyyyyyyyyyy                                                                                                    NamedImport
                                                         zzzzzzzzzzzzzzzz                                                                                  NamedImport
                                                                           foo::{a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}                   DestructuredImport
                                                                                {a,•b,•cxxxxxxxxxxxxx,•yyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzz}                   DestructuredImport.specifiers{dk: "{}"}
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
          {baar::{foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,•yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}},•z,•bar,•bar::*,•x,•y}     DestructuredImport.specifiers{dk: "{}"}
           baar::{foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,•yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}}                            DestructuredImport
                 {foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,•yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}}                            DestructuredImport.specifiers{dk: "{}"}
                  foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,•yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}                             DestructuredImport
                          {xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,•yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,•zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz}                             DestructuredImport.specifiers{dk: "{}"}
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
            {api::{Api,•ApiError},•blockchain::{self,•BlockProof,•Blockchain,•Transaction,•TransactionSet},•crypto::{Hash,•PublicKey},•helpers::Height,•node::TransactionSend,•storage::{ListProof,•MapProof}}     DestructuredImport.specifiers{dk: "{}"}
             api::{Api,•ApiError}                                                                                                                                                                                  DestructuredImport
                  {Api,•ApiError}                                                                                                                                                                                  DestructuredImport.specifiers{dk: "{}"}
                   Api                                                                                                                                                                                             NamedImport
                        ApiError                                                                                                                                                                                   NamedImport
                                   blockchain::{self,•BlockProof,•Blockchain,•Transaction,•TransactionSet}                                                                                                         DestructuredImport
                                               {self,•BlockProof,•Blockchain,•Transaction,•TransactionSet}                                                                                                         DestructuredImport.specifiers{dk: "{}"}
                                                self                                                                                                                                                               NamedImport
                                                      BlockProof                                                                                                                                                   NamedImport
                                                                  Blockchain                                                                                                                                       NamedImport
                                                                              Transaction                                                                                                                          NamedImport
                                                                                           TransactionSet                                                                                                          NamedImport
                                                                                                            crypto::{Hash,•PublicKey}                                                                              DestructuredImport
                                                                                                                    {Hash,•PublicKey}                                                                              DestructuredImport.specifiers{dk: "{}"}
                                                                                                                     Hash                                                                                          NamedImport
                                                                                                                           PublicKey                                                                               NamedImport
                                                                                                                                       helpers::Height                                                             NamedImport, ItemPath
                                                                                                                                                        node::TransactionSend                                      NamedImport, ItemPath
                                                                                                                                                                               storage::{ListProof,•MapProof}      DestructuredImport
                                                                                                                                                                                        {ListProof,•MapProof}      DestructuredImport.specifiers{dk: "{}"}
                                                                                                                                                                                         ListProof                 NamedImport
                                                                                                                                                                                                    MapProof       NamedImport*/
use a::{b::{c::*}};                                                                                                                       /*
use•a::{b::{c::*}};    UseStatement
    a::{b::{c::*}}     DestructuredImport
       {b::{c::*}}     DestructuredImport.specifiers{dk: "{}"}
        b::{c::*}      DestructuredImport
           {c::*}      DestructuredImport.specifiers{dk: "{}"}
            c::*       AmbientImport                                                                                                      */
use a::{b::{c::{}}};                                                                                                                      /*
use•a::{b::{c::{}}};    UseStatement
    a::{b::{c::{}}}     DestructuredImport
       {b::{c::{}}}     DestructuredImport.specifiers{dk: "{}"}
        b::{c::{}}      DestructuredImport
           {c::{}}      DestructuredImport.specifiers{dk: "{}"}
            c::{}       DestructuredImport
               {}       DestructuredImport.specifiers{dk: "{}"}                                                                           */
use a::{b::{c::d}};                                                                                                                       /*
use•a::{b::{c::d}};    UseStatement
    a::{b::{c::d}}     DestructuredImport
       {b::{c::d}}     DestructuredImport.specifiers{dk: "{}"}
        b::{c::d}      DestructuredImport
           {c::d}      DestructuredImport.specifiers{dk: "{}"}
            c::d       NamedImport, ItemPath                                                                                              */
use a::{b::{c::{xxx, yyy, zzz}}};                                                                                                         /*
use•a::{b::{c::{xxx,•yyy,•zzz}}};    UseStatement
    a::{b::{c::{xxx,•yyy,•zzz}}}     DestructuredImport
       {b::{c::{xxx,•yyy,•zzz}}}     DestructuredImport.specifiers{dk: "{}"}
        b::{c::{xxx,•yyy,•zzz}}      DestructuredImport
           {c::{xxx,•yyy,•zzz}}      DestructuredImport.specifiers{dk: "{}"}
            c::{xxx,•yyy,•zzz}       DestructuredImport
               {xxx,•yyy,•zzz}       DestructuredImport.specifiers{dk: "{}"}
                xxx                  NamedImport
                     yyy             NamedImport
                          zzz        NamedImport                                                                                          */
/// a
///•a↲    <UseStatement>
///•a     DocCommentAttribute{!inner, line}
// b
//•b    Comment{line}
use c;                                                                                                                                    /*
use•c;    UseStatement~ownStart
    c     NamedImport
use•c;    </UseStatement>                                                                                                                 */
#[macro_use]                                                                                                                              /*
#[macro_use]↲    <UseStatement>
#[macro_use]     Attribute{!inner}
 [macro_use]     Attribute.segments{dk: "[]"}                                                                                             */
use imports_with_attr;                                                                                                                    /*
use•imports_with_attr;    UseStatement~ownStart
    imports_with_attr     NamedImport
use•imports_with_attr;    </UseStatement>                                                                                                 */
use std::f64::consts::{SQRT_2, E, PI};                                                                                                    /*
use•std::f64::consts::{SQRT_2,•E,•PI};    UseStatement
    std::f64::consts::{SQRT_2,•E,•PI}     DestructuredImport
    std::f64::consts                      ItemPath
    std::f64                              ItemPath
                      {SQRT_2,•E,•PI}     DestructuredImport.specifiers{dk: "{}"}
                       SQRT_2             NamedImport
                               E          NamedImport
                                  PI      NamedImport                                                                                     */
#[rustfmt::skip]                                                                                                                          /*
#[rustfmt::skip]↲    <UseStatement>
#[rustfmt::skip]     Attribute{!inner}
 [rustfmt::skip]     Attribute.segments{dk: "[]"}
         ::          PunctuationToken{tk: "::"}                                                                                           */
use std::fmt::{self, {Display, Formatter}};                                                                                               /*
use•std::fmt::{self,•{Display,•Formatter}};    UseStatement~ownStart
    std::fmt::{self,•{Display,•Formatter}}     DestructuredImport
    std::fmt                                   ItemPath
              {self,•{Display,•Formatter}}     DestructuredImport.specifiers{dk: "{}"}
               self                            NamedImport
                     {Display,•Formatter}      DestructuredImport
                      Display                  NamedImport
                               Formatter       NamedImport
use•std::fmt::{self,•{Display,•Formatter}};    </UseStatement>
use•std::fmt::{self,•{Display,•Formatter}};    </Program.ast>
use•std::fmt::{self,•{Display,•Formatter}};    </Program>                                                                                 */
// Discarded Nodes: 1
// Parsed Nodes: 647
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 3680 (27% re-reads)
// Unnecessary 'skip_whitespace()' calls: 495
// source: "../../samples/statements/use.rs"