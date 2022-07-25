// a
//•a    Comment

/// b
                                                                                                                                          /*
///•b↲    <FunctionDeclaration>
///•b     DocCommentAttribute                                                                                                             */
fn a() { }                                                                                                                                /*
fn•a()•{•}    </FunctionDeclaration>                                                                                                      */

fn b() {                                                                                                                                  /*
fn•b()•{↲    <FunctionDeclaration>                                                                                                        */
    //! c
    //!•c    DocCommentAttribute
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

//////////////////////////////////
//////////////////////////////////    Comment
// d
//•d    Comment
/// let heart = '❤️';
                                                                                                                                          /*
///•let•heart•=•'❤️';↲    <FunctionDeclaration>
///•let•heart•=•'❤️';     DocCommentAttribute                                                                                             */
//////////////////////////////////
//////////////////////////////////    Comment
/// e
///•e    DocCommentAttribute
/// f
///•f••    DocCommentAttribute
fn c() { }                                                                                                                                /*
fn•c()•{•}    </FunctionDeclaration>                                                                                                      */

/*                                                                                                                                        /*
/*↲    <Comment>                                                                                                                          */
 * g
 */                                                                                                                                       /*
•*/    </Comment>                                                                                                                         */

/**                                                                                                                                       /*
/**↲    <FunctionDeclaration>, <DocCommentAttribute>                                                                                      */
 * h
 */                                                                                                                                       /*
•*/    </DocCommentAttribute>                                                                                                             */
fn d() { }                                                                                                                                /*
fn•d()•{•}    </FunctionDeclaration>                                                                                                      */

fn e() {                                                                                                                                  /*
fn•e()•{↲    <FunctionDeclaration>                                                                                                        */
    /*!                                                                                                                                   /*
    /*!↲    <DocCommentAttribute>                                                                                                         */
     * i
     */                                                                                                                                   /*
•••••*/    </DocCommentAttribute>                                                                                                         */
	//! ```a
    //!•```a    DocCommentAttribute
    //! b
    //!•b    DocCommentAttribute
    //! ```
    //!•```    DocCommentAttribute
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

/********************************/                                                                                                        /*
/********************************/    Comment                                                                                             */
/*                                                                                                                                        /*
/*↲    <Comment>                                                                                                                          */
 * j
 */                                                                                                                                       /*
•*/    </Comment>                                                                                                                         */

/********************************/                                                                                                        /*
/********************************/    Comment                                                                                             */
/**                                                                                                                                       /*
/**↲    <FunctionDeclaration>, <DocCommentAttribute>                                                                                      */
 * k
 */                                                                                                                                       /*
•*/    </DocCommentAttribute>                                                                                                             */
fn f() { }                                                                                                                                /*
fn•f()•{•}    </FunctionDeclaration>                                                                                                      */


// before
//•before    Comment
#[macro_use]                                                                                                                              /*
#[macro_use]↲    <ExternCrateStatement>
#[macro_use]     Attribute                                                                                                                */
// after
//•after    Comment
extern crate x;                                                                                                                           /*
extern•crate•x;    </ExternCrateStatement>
             x     NamedImport                                                                                                            */

a! {                                                                                                                                      /*
a!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                                         */
    /// line 2
    ///•line•2    Comment
    /// line 3
    ///•line•3    Comment
    #[a(b,c(d(e(f="g",h="i"))))]                                                                                                          /*
    #                               PunctuationToken
     [a(b,c(d(e(f="g",h="i"))))]    DelimGroup
       (b,c(d(e(f="g",h="i"))))     DelimGroup
         ,                          PunctuationToken
           (d(e(f="g",h="i")))      DelimGroup
             (e(f="g",h="i"))       DelimGroup
               (f="g",h="i")        DelimGroup
                 =                  PunctuationToken
                  "g"               Literal
                     ,              PunctuationToken
                       =            PunctuationToken
                        "i"         Literal                                                                                               */
    pub enum   X {                                                                                                                        /*
                 {↲    <DelimGroup>                                                                                                       */
        /// line 6
        ///•line•6    Comment
        /// line 7
        ///•line•7    Comment
        A(  B),                                                                                                                           /*
         (••B)     DelimGroup
              ,    PunctuationToken                                                                                                       */

        /// line 10
        ///•line•10    Comment
        C(D  ),                                                                                                                           /*
         (D••)     DelimGroup
              ,    PunctuationToken                                                                                                       */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */
// 5 comments inside a!{}
//•5•comments•inside•a!{}    Comment

/// ____
                                                                                                                                          /*
///•____↲    <TypeAliasDeclaration>
///•____     DocCommentAttribute                                                                                                          */
/// ____
///•____    DocCommentAttribute
#[attr_0]                                                                                                                                 /*
#[attr_0]    Attribute                                                                                                                    */
pub type A = Vec<B>;                                                                                                                      /*
pub•type•A•=•Vec<B>;    </TypeAliasDeclaration>
pub                     PubSpecifier
             Vec<B>     TypeCall                                                                                                          */

#[attr_1]                                                                                                                                 /*
#[attr_1]↲    <ExternCrateStatement>
#[attr_1]     Attribute                                                                                                                   */
extern crate b as d;                                                                                                                      /*
extern•crate•b•as•d;    </ExternCrateStatement>
             b•as•d     NamedImport                                                                                                       */

#![attr_2]                                                                                                                                /*
#![attr_2]    Attribute                                                                                                                   */
/**                                                                                                                                       /*
/**↲    <FunctionDeclaration>, <DocCommentAttribute>                                                                                      */
 * directly below attr_2 and 1 line above attr_3
 */                                                                                                                                       /*
•*/    </DocCommentAttribute>                                                                                                             */

#![attr_3]                                                                                                                                /*
#![attr_3]    Attribute                                                                                                                   */
fn attr_3_3a_target() {
    #![attr_3a]                                                                                                                           /*
    #![attr_3a]    Attribute                                                                                                              */
    #[attr_3b]                                                                                                                            /*
    #[attr_3b]↲    <FunctionDeclaration>
    #[attr_3b]     Attribute                                                                                                              */
    fn attr_3b_target() {}                                                                                                                /*
••••fn•attr_3b_target()•{}    </FunctionDeclaration>                                                                                      */

    #[attr_3c]                                                                                                                            /*
    #[attr_3c]↲    <FunctionDeclaration>
    #[attr_3c]     Attribute                                                                                                              */
    fn attr_3c_target() {}                                                                                                                /*
••••fn•attr_3c_target()•{}    </FunctionDeclaration>                                                                                      */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */


trait foo_C {                                                                                                                             /*
trait•foo_C•{↲    <TraitDeclaration>                                                                                                      */
    #![attr_4]                                                                                                                            /*
    #![attr_4]    Attribute                                                                                                               */
}                                                                                                                                         /*
}    </TraitDeclaration>                                                                                                                  */

#[attr_5]                                                                                                                                 /*
#[attr_5]↲    <FunctionDeclaration>
#[attr_5]     Attribute                                                                                                                   */

#![attr_6]                                                                                                                                /*
#![attr_6]    Attribute                                                                                                                   */
fn main() {
    // comment
    //•comment    Comment
	#[attr] ();                                                                                                                           /*
    #[attr]•();    ExpressionStatement
    #[attr]        Attribute
            ()     TupleLiteral                                                                                                           */
	#[attr] [1; 4];                                                                                                                       /*
    #[attr]•[1;•4];    ExpressionStatement
    #[attr]            Attribute
            [1;•4]     SizedArrayLiteral
             1         Literal
                4      Literal                                                                                                            */
	#[attr] Foo{data: (),};                                                                                                               /*
    #[attr]•Foo{data:•(),};    ExpressionStatement
    #[attr]                    Attribute
            Foo{data:•(),}     StructLiteral
                data:•()       StructLiteralProperty
                      ()       TupleLiteral                                                                                               */
	#[attr] if let Some(_) = Some(true) {}                                                                                                /*
    #[attr]•if•let•Some(_)•=•Some(true)•{}    ExpressionStatement
    #[attr]                                   Attribute
            if•let•Some(_)•=•Some(true)•{}    IfBlockExpression
               let•Some(_)•=•Some(true)       LetScrutinee
                   Some(_)                    TuplePattern
                        _                     WildcardPattern
                             Some(true)       CallExpression
                                  true        Literal                                                                                     */
	#[attr] if let Some(_) = Some(true) {} else if let Some(false) = Some(true) {}                                                        /*
    #[attr]•if•let•Some(_)•=•Some(true)•{}•else•if•let•Some(false)•=•Some(true)•{}    ExpressionStatement
    #[attr]                                                                           Attribute
            if•let•Some(_)•=•Some(true)•{}•else•if•let•Some(false)•=•Some(true)•{}    IfBlockExpression
               let•Some(_)•=•Some(true)                                               LetScrutinee
                   Some(_)                                                            TuplePattern
                        _                                                             WildcardPattern
                             Some(true)                                               CallExpression
                                  true                                                Literal
                                                if•let•Some(false)•=•Some(true)•{}    IfBlockExpression
                                                   let•Some(false)•=•Some(true)       LetScrutinee
                                                       Some(false)                    TuplePattern
                                                            false                     Literal
                                                                     Some(true)       CallExpression
                                                                          true        Literal                                             */
	#[attr] if true {}                                                                                                                    /*
    #[attr]•if•true•{}    ExpressionStatement
    #[attr]               Attribute
            if•true•{}    IfBlockExpression
               true       Literal                                                                                                         */
	#[attr] let _ = 0;                                                                                                                    /*
    #[attr]•let•_•=•0;    LetVariableDeclaration
    #[attr]               Attribute
                _         WildcardPattern
                    0     Literal                                                                                                         */
	#[attr] if true { } else if false { } else { }                                                                                        /*
    #[attr]•if•true•{•}•else•if•false•{•}•else•{•}    ExpressionStatement
    #[attr]                                           Attribute
            if•true•{•}•else•if•false•{•}•else•{•}    IfBlockExpression
               true                                   Literal
                             if•false•{•}•else•{•}    IfBlockExpression
                                false                 Literal
                                               {•}    BlockExpression                                                                     */
    { #![attr] foo() }                                                                                                                    /*
    {•#![attr]•foo()•}    ExpressionStatement, BlockExpression
      #![attr]            Attribute
               foo()      ExpressionStatement, CallExpression                                                                             */
    #[attr] if true {}                                                                                                                    /*
    #[attr]•if•true•{}    ExpressionStatement
    #[attr]               Attribute
            if•true•{}    IfBlockExpression
               true       Literal                                                                                                         */
    #[attr] (0, 1);                                                                                                                       /*
    #[attr]•(0,•1);    ExpressionStatement
    #[attr]            Attribute
            (0,•1)     TupleLiteral
             0         Literal
                1      Literal                                                                                                            */
    #[attr] (0,);                                                                                                                         /*
    #[attr]•(0,);    ExpressionStatement
    #[attr]          Attribute
            (0,)     TupleLiteral
             0       Literal                                                                                                              */
    #[attr] (0);                                                                                                                          /*
    #[attr]•(0);    ExpressionStatement
    #[attr]         Attribute
             0      Literal                                                                                                               */
    #[attr] [1, 2, 3];                                                                                                                    /*
    #[attr]•[1,•2,•3];    ExpressionStatement
    #[attr]               Attribute
            [1,•2,•3]     ArrayLiteral
             1            Literal
                2         Literal
                   3      Literal                                                                                                         */
    #[attr] { #![attr] }                                                                                                                  /*
    #[attr]•{•#![attr]•}    ExpressionStatement
    #[attr]                 Attribute
            {•#![attr]•}    BlockExpression
              #![attr]      Attribute                                                                                                     */
    #[attr] { foo(); }                                                                                                                    /*
    #[attr]•{•foo();•}    ExpressionStatement
    #[attr]               Attribute
            {•foo();•}    BlockExpression
              foo();      ExpressionStatement
              foo()       CallExpression                                                                                                  */
    #[attr] 0;                                                                                                                            /*
    #[attr]•0;    ExpressionStatement
    #[attr]       Attribute
            0     Literal                                                                                                                 */
    #[attr] expr_mac!();                                                                                                                  /*
    #[attr]•expr_mac!();    ExpressionStatement
    #[attr]                 Attribute
            expr_mac!()     MacroInvocation                                                                                               */
    #[attr] foo();                                                                                                                        /*
    #[attr]•foo();    ExpressionStatement
    #[attr]           Attribute
            foo()     CallExpression                                                                                                      */
    #[attr] let x = 1;                                                                                                                    /*
    #[attr]•let•x•=•1;    LetVariableDeclaration
    #[attr]               Attribute
                    1     Literal                                                                                                         */
    #[attr] match () { _ => { } }                                                                                                         /*
    #[attr]•match•()•{•_•=>•{•}•}    ExpressionStatement
    #[attr]                          Attribute
            match•()•{•_•=>•{•}•}    MatchExpression
                  ()                 TupleLiteral
                       _•=>•{•}      MatchExpressionCase
                       _             WildcardPattern
                            {•}      BlockExpression                                                                                      */
    #[attr] match () { #![attr] _ => (), }                                                                                                /*
    #[attr]•match•()•{•#![attr]•_•=>•(),•}    ExpressionStatement
    #[attr]                                   Attribute
            match•()•{•#![attr]•_•=>•(),•}    MatchExpression
                  ()                          TupleLiteral
                       #![attr]               Attribute
                                _•=>•()       MatchExpressionCase
                                _             WildcardPattern
                                     ()       TupleLiteral                                                                                */
    #[attr] unsafe { /**/ }                                                                                                               /*
    #[attr]•unsafe•{•/**/•}    ExpressionStatement
    #[attr]                    Attribute
            unsafe•{•/**/•}    BlockExpression
                     /**/      Comment                                                                                                    */
    || #[attr] return;                                                                                                                    /*
    ||•#[attr]•return;    ExpressionStatement
    ||•#[attr]•return     ClosureFunctionExpression
       #[attr]            Attribute (dangling)
               return     ReturnExpression                                                                                                */
	let a = #[attr] [1; 4];                                                                                                               /*
    let•a•=•#[attr]•[1;•4];    LetVariableDeclaration
            #[attr]•[1;•4]     SizedArrayLiteral
            #[attr]            Attribute
                     1         Literal
                        4      Literal                                                                                                    */
	let a = #[attr] box 0;                                                                                                                /*
    let•a•=•#[attr]•box•0;    LetVariableDeclaration
            #[attr]•box•0     BoxExpression
            #[attr]           Attribute
                        0     Literal                                                                                                     */
    let a = #[attr] [1, 2, 3];                                                                                                            /*
    let•a•=•#[attr]•[1,•2,•3];    LetVariableDeclaration
            #[attr]•[1,•2,•3]     ArrayLiteral
            #[attr]               Attribute
                     1            Literal
                        2         Literal
                           3      Literal                                                                                                 */
    let a = #[attr] Foo{data: (),};                                                                                                       /*
    let•a•=•#[attr]•Foo{data:•(),};    LetVariableDeclaration
            #[attr]•Foo{data:•(),}     StructLiteral
            #[attr]                    Attribute
                        data:•()       StructLiteralProperty
                              ()       TupleLiteral                                                                                       */
    let a = (#[attr] s).data;                                                                                                             /*
    let•a•=•(#[attr]•s).data;    LetVariableDeclaration
            (#[attr]•s).data     MemberExpression
             #[attr]             Attribute (dangling)                                                                                     */
    let a = (#[attr] t).0;                                                                                                                /*
    let•a•=•(#[attr]•t).0;    LetVariableDeclaration
            (#[attr]•t).0     MemberExpression
             #[attr]          Attribute (dangling)
                        0     Index                                                                                                       */
    let a = (#[attr] v)[0];                                                                                                               /*
    let•a•=•(#[attr]•v)[0];    LetVariableDeclaration
            (#[attr]•v)[0]     MemberExpression
             #[attr]           Attribute (dangling)
                        0      Literal                                                                                                    */
    let a = #[attr] -0i32;                                                                                                                /*
    let•a•=•#[attr]•-0i32;    LetVariableDeclaration
            #[attr]•-0i32     MinusExpression
            #[attr]           Attribute
                     0i32     Literal
                      i32     Identifier                                                                                                  */
    let a = #[attr] !0;                                                                                                                   /*
    let•a•=•#[attr]•!0;    LetVariableDeclaration
            #[attr]•!0     NotExpression
            #[attr]        Attribute
                     0     Literal                                                                                                        */
    let a = #[attr] 'c';                                                                                                                  /*
    let•a•=•#[attr]•'c';    LetVariableDeclaration
            #[attr]•'c'     Literal
            #[attr]         Attribute                                                                                                     */
    let a = #[attr] (..);                                                                                                                 /*
    let•a•=•#[attr]•(..);    LetVariableDeclaration
            #[attr]•(..      RangeLiteral
            #[attr]          Attribute                                                                                                    */
    let a = #[attr] (..0);                                                                                                                /*
    let•a•=•#[attr]•(..0);    LetVariableDeclaration
            #[attr]•(..0      RangeLiteral
            #[attr]           Attribute
                       0      Literal                                                                                                     */
    let a = #[attr] (#[attr] 0,);                                                                                                         /*
    let•a•=•#[attr]•(#[attr]•0,);    LetVariableDeclaration
            #[attr]•(#[attr]•0,)     TupleLiteral
            #[attr]                  Attribute
                     #[attr]•0       Literal
                     #[attr]         Attribute                                                                                            */
    let a = #[attr] (#[attr] 0, 0);                                                                                                       /*
    let•a•=•#[attr]•(#[attr]•0,•0);    LetVariableDeclaration
            #[attr]•(#[attr]•0,•0)     TupleLiteral
            #[attr]                    Attribute
                     #[attr]•0         Literal
                     #[attr]           Attribute
                                0      Literal                                                                                            */
    let a = #[attr] ();                                                                                                                   /*
    let•a•=•#[attr]•();    LetVariableDeclaration
            #[attr]•()     TupleLiteral
            #[attr]        Attribute                                                                                                      */
    let a = #[attr] (0, 0);                                                                                                               /*
    let•a•=•#[attr]•(0,•0);    LetVariableDeclaration
            #[attr]•(0,•0)     TupleLiteral
            #[attr]            Attribute
                     0         Literal
                        0      Literal                                                                                                    */
    let a = #[attr] (0,);                                                                                                                 /*
    let•a•=•#[attr]•(0,);    LetVariableDeclaration
            #[attr]•(0,)     TupleLiteral
            #[attr]          Attribute
                     0       Literal                                                                                                      */
    let a = #[attr] (0..);                                                                                                                /*
    let•a•=•#[attr]•(0..);    LetVariableDeclaration
            #[attr]•(0..      RangeLiteral
            #[attr]           Attribute
                     0        Literal                                                                                                     */
    let a = #[attr] (0..0);                                                                                                               /*
    let•a•=•#[attr]•(0..0);    LetVariableDeclaration
            #[attr]•(0..0      RangeLiteral
            #[attr]            Attribute
                     0         Literal
                        0      Literal                                                                                                    */
    let a = #[attr] (0);                                                                                                                  /*
    let•a•=•#[attr]•(0);    LetVariableDeclaration
            #[attr]•(0      Literal
            #[attr]         Attribute                                                                                                     */
    let a = #[attr] (0);                                                                                                                  /*
    let•a•=•#[attr]•(0);    LetVariableDeclaration
            #[attr]•(0      Literal
            #[attr]         Attribute                                                                                                     */
    let a = #[attr] [0, 0];                                                                                                               /*
    let•a•=•#[attr]•[0,•0];    LetVariableDeclaration
            #[attr]•[0,•0]     ArrayLiteral
            #[attr]            Attribute
                     0         Literal
                        0      Literal                                                                                                    */
    let a = #[attr] [0; 0];                                                                                                               /*
    let•a•=•#[attr]•[0;•0];    LetVariableDeclaration
            #[attr]•[0;•0]     SizedArrayLiteral
            #[attr]            Attribute
                     0         Literal
                        0      Literal                                                                                                    */
    let a = #[attr] { #![attr] };                                                                                                         /*
    let•a•=•#[attr]•{•#![attr]•};    LetVariableDeclaration
            #[attr]•{•#![attr]•}     BlockExpression
            #[attr]                  Attribute
                      #![attr]       Attribute                                                                                            */
    let a = #[attr] { #![attr] let _ = (); ()  };                                                                                         /*
    let•a•=•#[attr]•{•#![attr]•let•_•=•();•()••};    LetVariableDeclaration
            #[attr]•{•#![attr]•let•_•=•();•()••}     BlockExpression
            #[attr]                                  Attribute
                      #![attr]                       Attribute
                               let•_•=•();           LetVariableDeclaration
                                   _                 WildcardPattern
                                       ()            TupleLiteral
                                           ()        ExpressionStatement, TupleLiteral                                                    */
    let a = #[attr] { #![attr] let _ = (); };                                                                                             /*
    let•a•=•#[attr]•{•#![attr]•let•_•=•();•};    LetVariableDeclaration
            #[attr]•{•#![attr]•let•_•=•();•}     BlockExpression
            #[attr]                              Attribute
                      #![attr]                   Attribute
                               let•_•=•();       LetVariableDeclaration
                                   _             WildcardPattern
                                       ()        TupleLiteral                                                                             */
    let a = #[attr] &#[attr] 0;                                                                                                           /*
    let•a•=•#[attr]•&#[attr]•0;    LetVariableDeclaration
            #[attr]•&#[attr]•0     ReferenceExpression
            #[attr]                Attribute
                     #[attr]       Attribute (dangling)
                             0     Literal                                                                                                */
    let a = #[attr] &0;                                                                                                                   /*
    let•a•=•#[attr]•&0;    LetVariableDeclaration
            #[attr]•&0     ReferenceExpression
            #[attr]        Attribute
                     0     Literal                                                                                                        */
    let a = #[attr] &mut #[attr] 0;                                                                                                       /*
    let•a•=•#[attr]•&mut•#[attr]•0;    LetVariableDeclaration
            #[attr]•&mut•#[attr]•0     ReferenceExpression
            #[attr]                    Attribute
                         #[attr]       Attribute (dangling)
                                 0     Literal                                                                                            */
    let a = #[attr] &mut 0;                                                                                                               /*
    let•a•=•#[attr]•&mut•0;    LetVariableDeclaration
            #[attr]•&mut•0     ReferenceExpression
            #[attr]            Attribute
                         0     Literal                                                                                                    */
    let a = #[attr] || { #![attr] #[attr] () };                                                                                           /*
    let•a•=•#[attr]•||•{•#![attr]•#[attr]•()•};    LetVariableDeclaration
            #[attr]•||•{•#![attr]•#[attr]•()•}     ClosureFunctionExpression
            #[attr]                                Attribute
                       {•#![attr]•#[attr]•()•}     BlockExpression
                         #![attr]                  Attribute
                                  #[attr]•()       ExpressionStatement
                                  #[attr]          Attribute
                                          ()       TupleLiteral                                                                           */
    let a = #[attr] || #[attr] ();                                                                                                        /*
    let•a•=•#[attr]•||•#[attr]•();    LetVariableDeclaration
            #[attr]•||•#[attr]•()     ClosureFunctionExpression
            #[attr]                   Attribute
                       #[attr]        Attribute (dangling)
                               ()     TupleLiteral                                                                                        */
    let a = #[attr] 0 + #[attr] 0;                                                                                                        /*
    let•a•=•#[attr]•0•+•#[attr]•0;    LetVariableDeclaration
            #[attr]•0•+•#[attr]•0     OperationExpression
            #[attr]                   Attribute
                    0                 Literal
                        #[attr]       Attribute (dangling)
                                0     Literal                                                                                             */
    let a = #[attr] 0 as usize;                                                                                                           /*
    let•a•=•#[attr]•0•as•usize;    LetVariableDeclaration
            #[attr]•0•as•usize     ExpressionAsTypeCast
            #[attr]                Attribute
                    0              Literal                                                                                                */
    let a = #[attr] 0;                                                                                                                    /*
    let•a•=•#[attr]•0;    LetVariableDeclaration
            #[attr]•0     Literal
            #[attr]       Attribute                                                                                                       */
    let a = #[attr] 0..;                                                                                                                  /*
    let•a•=•#[attr]•0..;    LetVariableDeclaration
            #[attr]•0..     RangeLiteral
            #[attr]         Attribute
                    0       Literal                                                                                                       */
    let a = #[attr] 0..#[attr] 0;                                                                                                         /*
    let•a•=•#[attr]•0..#[attr]•0;    LetVariableDeclaration
            #[attr]•0..#[attr]•0     RangeLiteral
            #[attr]                  Attribute
                    0                Literal
                       #[attr]       Attribute (dangling)
                               0     Literal                                                                                              */
    let a = #[attr] 1i32.clone();                                                                                                         /*
    let•a•=•#[attr]•1i32.clone();    LetVariableDeclaration
            #[attr]•1i32.clone()     CallExpression
            #[attr]                  Attribute
                    1i32             Literal
                     i32             Identifier                                                                                           */
    let a = #[attr] x! { };                                                                                                               /*
    let•a•=•#[attr]•x!•{•};    LetVariableDeclaration
            #[attr]•x!•{•}     MacroInvocation
            #[attr]            Attribute                                                                                                  */
    let a = #[attr] x!();                                                                                                                 /*
    let•a•=•#[attr]•x!();    LetVariableDeclaration
            #[attr]•x!()     MacroInvocation
            #[attr]          Attribute                                                                                                    */
    let a = #[attr] x![];                                                                                                                 /*
    let•a•=•#[attr]•x![];    LetVariableDeclaration
            #[attr]•x![]     MacroInvocation
            #[attr]          Attribute                                                                                                    */
    let a = #[attr] false;                                                                                                                /*
    let•a•=•#[attr]•false;    LetVariableDeclaration
            #[attr]•false     Literal
            #[attr]           Attribute                                                                                                   */
    let a = #[attr] x();                                                                                                                  /*
    let•a•=•#[attr]•x();    LetVariableDeclaration
            #[attr]•x()     CallExpression
            #[attr]         Attribute                                                                                                     */
    let a = #[attr] x!();                                                                                                                 /*
    let•a•=•#[attr]•x!();    LetVariableDeclaration
            #[attr]•x!()     MacroInvocation
            #[attr]          Attribute                                                                                                    */
    let a = #[attr] x!(#! [attr]);                                                                                                        /*
    let•a•=•#[attr]•x!(#!•[attr]);    LetVariableDeclaration
            #[attr]•x!(#!•[attr])     MacroInvocation
            #[attr]                   Attribute
                       #              PunctuationToken
                        !             PunctuationToken
                          [attr]      DelimGroup                                                                                          */
    let a = #[attr] x![];                                                                                                                 /*
    let•a•=•#[attr]•x![];    LetVariableDeclaration
            #[attr]•x![]     MacroInvocation
            #[attr]          Attribute                                                                                                    */
    let a = #[attr] x![#! [attr]];                                                                                                        /*
    let•a•=•#[attr]•x![#!•[attr]];    LetVariableDeclaration
            #[attr]•x![#!•[attr]]     MacroInvocation
            #[attr]                   Attribute
                       #              PunctuationToken
                        !             PunctuationToken
                          [attr]      DelimGroup                                                                                          */
    let a = #[attr] x! {};                                                                                                                /*
    let•a•=•#[attr]•x!•{};    LetVariableDeclaration
            #[attr]•x!•{}     MacroInvocation
            #[attr]           Attribute                                                                                                   */
    let a = #[attr] x! { #! [attr] };                                                                                                     /*
    let•a•=•#[attr]•x!•{•#!•[attr]•};    LetVariableDeclaration
            #[attr]•x!•{•#!•[attr]•}     MacroInvocation
            #[attr]                      Attribute
                         #               PunctuationToken
                          !              PunctuationToken
                            [attr]       DelimGroup                                                                                       */
    let a = #[attr] Foo{..s};                                                                                                             /*
    let•a•=•#[attr]•Foo{..s};    LetVariableDeclaration
            #[attr]•Foo{..s}     StructLiteral
            #[attr]              Attribute
                        ..s      StructLiteralPropertySpread                                                                              */
    let a = #[attr] Foo{data: (), ..s};                                                                                                   /*
    let•a•=•#[attr]•Foo{data:•(),•..s};    LetVariableDeclaration
            #[attr]•Foo{data:•(),•..s}     StructLiteral
            #[attr]                        Attribute
                        data:•()           StructLiteralProperty
                              ()           TupleLiteral
                                  ..s      StructLiteralPropertySpread                                                                    */
    let a = #[attr] Foo{data: (),};                                                                                                       /*
    let•a•=•#[attr]•Foo{data:•(),};    LetVariableDeclaration
            #[attr]•Foo{data:•(),}     StructLiteral
            #[attr]                    Attribute
                        data:•()       StructLiteralProperty
                              ()       TupleLiteral                                                                                       */
    let a = #[attr] for _ in 0..0 { #![attr] };                                                                                           /*
    let•a•=•#[attr]•for•_•in•0..0•{•#![attr]•};    LetVariableDeclaration
            #[attr]•for•_•in•0..0•{•#![attr]•}     ForInBlockExpression
            #[attr]                                Attribute
                        _                          WildcardPattern
                             0..0                  RangeLiteral
                             0                     Literal
                                0                  Literal
                                    #![attr]       Attribute                                                                              */
    let a = #[attr] loop { #![attr] };                                                                                                    /*
    let•a•=•#[attr]•loop•{•#![attr]•};    LetVariableDeclaration
            #[attr]•loop•{•#![attr]•}     LoopBlockExpression
            #[attr]                       Attribute
                           #![attr]       Attribute                                                                                       */
    let a = #[attr] match 0 { #![attr] () => (), };                                                                                       /*
    let•a•=•#[attr]•match•0•{•#![attr]•()•=>•(),•};    LetVariableDeclaration
            #[attr]•match•0•{•#![attr]•()•=>•(),•}     MatchExpression
            #[attr]                                    Attribute
                          0                            Literal
                              #![attr]                 Attribute
                                       ()•=>•()        MatchExpressionCase
                                       ()              TuplePattern
                                             ()        TupleLiteral                                                                       */
    let a = #[attr] match 0 { #![attr] _ => (), };                                                                                        /*
    let•a•=•#[attr]•match•0•{•#![attr]•_•=>•(),•};    LetVariableDeclaration
            #[attr]•match•0•{•#![attr]•_•=>•(),•}     MatchExpression
            #[attr]                                   Attribute
                          0                           Literal
                              #![attr]                Attribute
                                       _•=>•()        MatchExpressionCase
                                       _              WildcardPattern
                                            ()        TupleLiteral                                                                        */
    let a = #[attr] move || { #![attr] #[attr] () };                                                                                      /*
    let•a•=•#[attr]•move•||•{•#![attr]•#[attr]•()•};    LetVariableDeclaration
            #[attr]•move•||•{•#![attr]•#[attr]•()•}     ClosureFunctionExpression
            #[attr]                                     Attribute
                            {•#![attr]•#[attr]•()•}     BlockExpression
                              #![attr]                  Attribute
                                       #[attr]•()       ExpressionStatement
                                       #[attr]          Attribute
                                               ()       TupleLiteral                                                                      */
    let a = #[attr] move || #[attr] ();                                                                                                   /*
    let•a•=•#[attr]•move•||•#[attr]•();    LetVariableDeclaration
            #[attr]•move•||•#[attr]•()     ClosureFunctionExpression
            #[attr]                        Attribute
                            #[attr]        Attribute (dangling)
                                    ()     TupleLiteral                                                                                   */
    let a = #[attr] s.data;                                                                                                               /*
    let•a•=•#[attr]•s.data;    LetVariableDeclaration
            #[attr]•s.data     MemberExpression
            #[attr]            Attribute                                                                                                  */
    let a = #[attr] t.0;                                                                                                                  /*
    let•a•=•#[attr]•t.0;    LetVariableDeclaration
            #[attr]•t.0     MemberExpression
            #[attr]         Attribute
                      0     Index                                                                                                         */
    let a = #[attr] v[0];                                                                                                                 /*
    let•a•=•#[attr]•v[0];    LetVariableDeclaration
            #[attr]•v[0]     MemberExpression
            #[attr]          Attribute
                      0      Literal                                                                                                      */
    let a = #[attr] while false { #![attr] };                                                                                             /*
    let•a•=•#[attr]•while•false•{•#![attr]•};    LetVariableDeclaration
            #[attr]•while•false•{•#![attr]•}     WhileBlockExpression
            #[attr]                              Attribute
                          false                  Literal
                                  #![attr]       Attribute                                                                                */
    let a = #[attr] while let None = Some(()) { #![attr] };                                                                               /*
    let•a•=•#[attr]•while•let•None•=•Some(())•{•#![attr]•};    LetVariableDeclaration
            #[attr]•while•let•None•=•Some(())•{•#![attr]•}     WhileBlockExpression
            #[attr]                                            Attribute
                          let•None•=•Some(())                  LetScrutinee
                                     Some(())                  CallExpression
                                          ()                   TupleLiteral
                                                #![attr]       Attribute                                                                  */
    let a = #[attr] x += 15;                                                                                                              /*
    let•a•=•#[attr]•x•+=•15;    LetVariableDeclaration
            #[attr]•x•+=•15     ReassignmentOperationExpression
            #[attr]             Attribute
                         15     Literal                                                                                                   */
    let a = #[attr] (x += 15);                                                                                                            /*
    let•a•=•#[attr]•(x•+=•15);    LetVariableDeclaration
            #[attr]•(x•+=•15      ReassignmentOperationExpression
            #[attr]               Attribute
                          15      Literal                                                                                                 */
    let a = #[attr] x = 15;                                                                                                               /*
    let•a•=•#[attr]•x•=•15;    LetVariableDeclaration
            #[attr]•x•=•15     ReassignmentExpression
            #[attr]            Attribute
                        15     Literal                                                                                                    */
    let a = #[attr] (x = 15);                                                                                                             /*
    let•a•=•#[attr]•(x•=•15);    LetVariableDeclaration
            #[attr]•(x•=•15      ReassignmentExpression
            #[attr]              Attribute
                         15      Literal                                                                                                  */
    let a: [(); 0] = #[attr] [];                                                                                                          /*
    let•a:•[();•0]•=•#[attr]•[];    LetVariableDeclaration
           [();•0]                  TypeSizedArray
            ()                      TypeTuple
                0                   Literal
                     #[attr]•[]     ArrayLiteral
                     #[attr]        Attribute                                                                                             */
    let a: fn(&u32) -> u32 = #[attr] std::clone::Clone::clone;                                                                            /*
    let•a:•fn(&u32)•->•u32•=•#[attr]•std::clone::Clone::clone;    LetVariableDeclaration
           fn(&u32)•->•u32                                        TypeFnPointer
              &u32                                                TypeFnPointerParameter, TypeReference
                             #[attr]•std::clone::Clone::clone     ExpressionPath
                             #[attr]                              Attribute
                                     std::clone::Clone            ExpressionPath
                                     std::clone                   ExpressionPath                                                          */
    let a = #[attr] 1;                                                                                                                    /*
    let•a•=•#[attr]•1;    LetVariableDeclaration
            #[attr]•1     Literal
            #[attr]       Attribute                                                                                                       */
    let a = |                                                                                                                             /*
    let•a•=•|↲    <LetVariableDeclaration>
            |↲    <ClosureFunctionExpression>                                                                                             */
    #[allow(C)]a: u32,                                                                                                                    /*
    #[allow(C)]a:•u32     ClosureFunctionParameterDeclaration
    #[allow(C)]           Attribute
           (C)            DelimGroup                                                                                                      */
    #[cfg(something)] b: i32,                                                                                                             /*
    #[cfg(something)]•b:•i32     ClosureFunctionParameterDeclaration
    #[cfg(something)]            Attribute
         (something)             DelimGroup                                                                                               */
    #[cfg_attr(something, cfg(nothing))]#[deny(C)] c: i32,                                                                                /*
    #[cfg_attr(something,•cfg(nothing))]#[deny(C)]•c:•i32     ClosureFunctionParameterDeclaration
    #[cfg_attr(something,•cfg(nothing))]                      Attribute
              (something,•cfg(nothing))                       DelimGroup
                        ,                                     PunctuationToken
                             (nothing)                        DelimGroup
                                        #[deny(C)]            Attribute
                                              (C)             DelimGroup                                                                  */
    | {};                                                                                                                                 /*
••••|•{};    </LetVariableDeclaration>
••••|•{}     </ClosureFunctionExpression>
      {}     BlockExpression                                                                                                              */
    qux(3 + #[attr] 2);                                                                                                                   /*
    qux(3•+•#[attr]•2);    ExpressionStatement
    qux(3•+•#[attr]•2)     CallExpression
        3•+•#[attr]•2      OperationExpression
        3                  Literal
            #[attr]        Attribute (dangling)
                    2      Literal                                                                                                        */
    foo3(x, #[attr] y, z);                                                                                                                /*
    foo3(x,•#[attr]•y,•z);    ExpressionStatement
    foo3(x,•#[attr]•y,•z)     CallExpression
            #[attr]•y         Identifier
            #[attr]           Attribute                                                                                                   */
    while false { let _ = #[attr] continue ; }                                                                                            /*
    while•false•{•let•_•=•#[attr]•continue•;•}    ExpressionStatement, WhileBlockExpression
          false                                   Literal
                  let•_•=•#[attr]•continue•;      LetVariableDeclaration
                      _                           WildcardPattern
                          #[attr]•continue        ContinueExpression
                          #[attr]                 Attribute                                                                               */
    while true { let _ = #[attr] break ; }                                                                                                /*
    while•true•{•let•_•=•#[attr]•break•;•}    ExpressionStatement, WhileBlockExpression
          true                                Literal
                 let•_•=•#[attr]•break•;      LetVariableDeclaration
                     _                        WildcardPattern
                         #[attr]•break        BreakExpression
                         #[attr]              Attribute                                                                                   */
    match (Q {#[attr] C: 1 }) {                                                                                                           /*
    match•(Q•{#[attr]•C:•1•})•{↲    <ExpressionStatement>, <MatchExpression>
           Q•{#[attr]•C:•1•}        StructLiteral
              #[attr]•C:•1          StructLiteralProperty
              #[attr]               Attribute
                         1          Literal                                                                                               */
        Q { #[attr] C } => {}                                                                                                             /*
        Q•{•#[attr]•C•}•=>•{}    MatchExpressionCase
        Q•{•#[attr]•C•}          StructPattern
            #[attr]•C            StructPatternPropertyShorthand
            #[attr]              Attribute
                           {}    BlockExpression                                                                                          */
        Q ( #[attr] C ) => {}                                                                                                             /*
        Q•(•#[attr]•C•)•=>•{}    MatchExpressionCase
        Q•(•#[attr]•C•)          TuplePattern
            #[attr]•C            Identifier
            #[attr]              Attribute
                           {}    BlockExpression                                                                                          */
		#[attr] _ => {}                                                                                                                   /*
        #[attr]•_•=>•{}    MatchExpressionCase
        #[attr]            Attribute
                _          WildcardPattern
                     {}    BlockExpression                                                                                                */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
static X: &[Y] = &[                                                                                                                       /*
static•X:•&[Y]•=•&[↲    <StaticVariableDeclaration>
          &[Y]          TypeReference
           [Y]          TypeSlice
                 &[↲    <ReferenceExpression>
                  [↲    <ArrayLiteral>                                                                                                    */
	#[a(b = "c")]                                                                                                                         /*
    #[a(b•=•"c")]↲    <Literal>
    #[a(b•=•"c")]     Attribute
       (b•=•"c")      DelimGroup
          =           PunctuationToken
            "c"       Literal                                                                                                             */
	0,                                                                                                                                    /*
   ╚0    </Literal>                                                                                                                       */
];                                                                                                                                        /*
];    </StaticVariableDeclaration>
]     </ReferenceExpression>, </ArrayLiteral>                                                                                             */
#[attr] const C: C = C { #[attr] field: 0, #[attr] field: 1 };                                                                            /*
#[attr]•const•C:•C•=•C•{•#[attr]•field:•0,•#[attr]•field:•1•};    ConstVariableDeclaration
#[attr]                                                           Attribute
                     C•{•#[attr]•field:•0,•#[attr]•field:•1•}     StructLiteral
                         #[attr]•field:•0                         StructLiteralProperty
                         #[attr]                                  Attribute
                                        0                         Literal
                                           #[attr]•field:•1       StructLiteralProperty
                                           #[attr]                Attribute
                                                          1       Literal                                                                 */
#[attr] struct S;                                                                                                                         /*
#[attr]•struct•S;    StructDeclaration
#[attr]              Attribute                                                                                                            */
#[attr] struct I { #[attr] i: u8, #[attr] pub i: u8, }                                                                                    /*
#[attr]•struct•I•{•#[attr]•i:•u8,•#[attr]•pub•i:•u8,•}    StructDeclaration
#[attr]                                                   Attribute
                   #[attr]•i:•u8                          StructPropertyDeclaration
                   #[attr]                                Attribute
                                  #[attr]•pub•i:•u8       StructPropertyDeclaration
                                  #[attr]                 Attribute
                                          pub             PubSpecifier                                                                    */
#[attr] struct I ( #[attr] u8, #[attr] pub u8, )                                                                                          /*
#[attr]•struct•I•(•#[attr]•u8,•#[attr]•pub•u8,•)    TupleStructDeclaration
#[attr]                                             Attribute
                   #[attr]•u8                       TupleStructItemDeclaration
                   #[attr]                          Attribute
                               #[attr]•pub•u8       TupleStructItemDeclaration
                               #[attr]              Attribute
                                       pub          PubSpecifier                                                                          */
#[attr] struct BreaksWithComment ( #[attr] u8, #[attr] // comment
                                                                                                                                          /*
#[attr]•struct•BreaksWithComment•(•#[attr]•u8,•#[attr]•//•comment↲    <TupleStructDeclaration>
#[attr]                                                               Attribute
                                   #[attr]•u8                         TupleStructItemDeclaration
                                   #[attr]                            Attribute
                                               #[attr]•//•comment↲    <TupleStructItemDeclaration>
                                               #[attr]                Attribute
                                                       //•comment     Comment                                                             */
    pub u8, )                                                                                                                             /*
••••pub•u8,•)    </TupleStructDeclaration>
••••pub•u8       </TupleStructItemDeclaration>
    pub          PubSpecifier                                                                                                             */
struct C {                                                                                                                                /*
struct•C•{↲    <StructDeclaration>                                                                                                        */
    #[attr]                                                                                                                               /*
    #[attr]↲    <StructPropertyDeclaration>
    #[attr]     Attribute                                                                                                                 */
    /// below attr
    ///•below•attr    DocCommentAttribute
    a: b,                                                                                                                                 /*
••••a:•b     </StructPropertyDeclaration>                                                                                                 */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */
struct Q { #[attr] C: i32, }                                                                                                              /*
struct•Q•{•#[attr]•C:•i32,•}    StructDeclaration
           #[attr]•C:•i32       StructPropertyDeclaration
           #[attr]              Attribute                                                                                                 */
struct A<#[attr] 'a>();                                                                                                                   /*
struct•A<#[attr]•'a>();    TupleStructDeclaration
         #[attr]•'a        GenericLtParameterDeclaration
         #[attr]           Attribute
                 'a        LtIdentifier                                                                                                   */
struct A<#[attr] I>(I);                                                                                                                   /*
struct•A<#[attr]•I>(I);    TupleStructDeclaration
         #[attr]•I         GenericTypeParameterDeclaration
         #[attr]           Attribute
                    I      TupleStructItemDeclaration                                                                                     */
enum E { #[attr] C(i32), }                                                                                                                /*
enum•E•{•#[attr]•C(i32),•}    EnumDeclaration
         #[attr]•C(i32)       EnumMemberTupleDeclaration
         #[attr]              Attribute
                   i32        TupleStructItemDeclaration                                                                                  */
enum E<#[attr] 'b> {}                                                                                                                     /*
enum•E<#[attr]•'b>•{}    EnumDeclaration
       #[attr]•'b        GenericLtParameterDeclaration
       #[attr]           Attribute
               'b        LtIdentifier                                                                                                     */
enum E<#[attr] J> {}                                                                                                                      /*
enum•E<#[attr]•J>•{}    EnumDeclaration
       #[attr]•J        GenericTypeParameterDeclaration
       #[attr]          Attribute                                                                                                         */
trait T { #![attr] }                                                                                                                      /*
trait•T•{•#![attr]•}    TraitDeclaration
          #![attr]      Attribute                                                                                                         */
trait T<#[attr] 'c> {}                                                                                                                    /*
trait•T<#[attr]•'c>•{}    TraitDeclaration
        #[attr]•'c        GenericLtParameterDeclaration
        #[attr]           Attribute
                'c        LtIdentifier                                                                                                    */
trait T<#[attr] K> {}                                                                                                                     /*
trait•T<#[attr]•K>•{}    TraitDeclaration
        #[attr]•K        GenericTypeParameterDeclaration
        #[attr]          Attribute                                                                                                        */
type Y<#[attr] 'd> = ();                                                                                                                  /*
type•Y<#[attr]•'d>•=•();    TypeAliasDeclaration
       #[attr]•'d           GenericLtParameterDeclaration
       #[attr]              Attribute
               'd           LtIdentifier
                     ()     TypeTuple                                                                                                     */
type Y<#[attr] L> = ();                                                                                                                   /*
type•Y<#[attr]•L>•=•();    TypeAliasDeclaration
       #[attr]•L           GenericTypeParameterDeclaration
       #[attr]             Attribute
                    ()     TypeTuple                                                                                                      */
#[attr] type A = fn(#[a1] u8, #[a2] ...);                                                                                                 /*
#[attr]•type•A•=•fn(#[a1]•u8,•#[a2]•...);    TypeAliasDeclaration
#[attr]                                      Attribute
                 fn(#[a1]•u8,•#[a2]•...)     TypeFnPointer
                    #[a1]•u8                 TypeFnPointerParameter
                    #[a1]                    Attribute
                              #[a2]•...      FunctionSpread
                              #[a2]          Attribute                                                                                    */
impl<#[attr] 'e> A<'e> {}                                                                                                                 /*
impl<#[attr]•'e>•A<'e>•{}    ImplDeclaration
     #[attr]•'e              GenericLtParameterDeclaration
     #[attr]                 Attribute
             'e              LtIdentifier
                 A<'e>       TypeCall
                   'e        LtIdentifier                                                                                                 */
impl<#[attr] M> A<M> {}                                                                                                                   /*
impl<#[attr]•M>•A<M>•{}    ImplDeclaration
     #[attr]•M             GenericTypeParameterDeclaration
     #[attr]               Attribute
                A<M>       TypeCall                                                                                                       */
impl<#[attr] 'f> T<'f> for A<'f> {}                                                                                                       /*
impl<#[attr]•'f>•T<'f>•for•A<'f>•{}    ImplDeclaration
     #[attr]•'f                        GenericLtParameterDeclaration
     #[attr]                           Attribute
             'f                        LtIdentifier
                 T<'f>                 TypeCall
                   'f                  LtIdentifier
                           A<'f>       TypeCall
                             'f        LtIdentifier                                                                                       */
impl<#[attr] N> T<N> for A<N> {}                                                                                                          /*
impl<#[attr]•N>•T<N>•for•A<N>•{}    ImplDeclaration
     #[attr]•N                      GenericTypeParameterDeclaration
     #[attr]                        Attribute
                T<N>                TypeCall
                         A<N>       TypeCall                                                                                              */
#[attr] #[attr] macro_rules! m_e {}                                                                                                       /*
#[attr]•#[attr]•macro_rules!•m_e•{}    MacroRulesDeclaration
#[attr]                                Attribute
        #[attr]                        Attribute                                                                                          */
#[attr] macro_rules! m {}                                                                                                                 /*
#[attr]•macro_rules!•m•{}    MacroRulesDeclaration
#[attr]                      Attribute                                                                                                    */

#[attr] fn f() {}                                                                                                                         /*
#[attr]•fn•f()•{}    FunctionDeclaration
#[attr]              Attribute                                                                                                            */
#[attr] fn f(#[a1] a: u8) { let f = |#[a2] W(x), #[a3] y| (); }                                                                           /*
#[attr]•fn•f(#[a1]•a:•u8)•{•let•f•=•|#[a2]•W(x),•#[a3]•y|•();•}    FunctionDeclaration
#[attr]                                                            Attribute
             #[a1]•a:•u8                                           FunctionParameterDeclaration
             #[a1]                                                 Attribute
                            let•f•=•|#[a2]•W(x),•#[a3]•y|•();      LetVariableDeclaration
                                    |#[a2]•W(x),•#[a3]•y|•()       ClosureFunctionExpression
                                     #[a2]•W(x)                    ClosureFunctionParameterDeclaration
                                     #[a2]                         Attribute
                                           W(x)                    TuplePattern
                                                 #[a3]•y           ClosureFunctionParameterDeclaration
                                                 #[a3]             Attribute
                                                          ()       TupleLiteral                                                           */
fn f<#[attr] 'g>() {}                                                                                                                     /*
fn•f<#[attr]•'g>()•{}    FunctionDeclaration
     #[attr]•'g          GenericLtParameterDeclaration
     #[attr]             Attribute
             'g          LtIdentifier                                                                                                     */
fn f<'e, #[attr] 'g>() {}                                                                                                                 /*
fn•f<'e,•#[attr]•'g>()•{}    FunctionDeclaration
     'e                      GenericLtParameterDeclaration, LtIdentifier
         #[attr]•'g          GenericLtParameterDeclaration
         #[attr]             Attribute
                 'g          LtIdentifier                                                                                                 */
fn f<#[attr] G>() {}                                                                                                                      /*
fn•f<#[attr]•G>()•{}    FunctionDeclaration
     #[attr]•G          GenericTypeParameterDeclaration
     #[attr]            Attribute                                                                                                         */
fn f<E, #[attr] G>() {}                                                                                                                   /*
fn•f<E,•#[attr]•G>()•{}    FunctionDeclaration
     E                     GenericTypeParameterDeclaration
        #[attr]•G          GenericTypeParameterDeclaration
        #[attr]            Attribute                                                                                                      */
fn f() where for<#[attr] 'i> X: for<#[attr] 'i> Y {}                                                                                      /*
fn•f()•where•for<#[attr]•'i>•X:•for<#[attr]•'i>•Y•{}    FunctionDeclaration
             for<#[attr]•'i>•X:•for<#[attr]•'i>•Y       WhereTypeBoundDeclaration
                 #[attr]•'i                             GenericLtParameterDeclaration
                 #[attr]                                Attribute
                         'i                             LtIdentifier
                                for<#[attr]•'i>•Y       TypeTraitBound
                                    #[attr]•'i          GenericLtParameterDeclaration
                                    #[attr]             Attribute
                                            'i          LtIdentifier                                                                      */
fn f(#[d(true)] a: i32, #[a2] b: i32, #[what = "how"] c: u32) {}                                                                          /*
fn•f(#[d(true)]•a:•i32,•#[a2]•b:•i32,•#[what•=•"how"]•c:•u32)•{}    FunctionDeclaration
     #[d(true)]•a:•i32                                              FunctionParameterDeclaration
     #[d(true)]                                                     Attribute
        (true)                                                      DelimGroup
         true                                                       Literal
                        #[a2]•b:•i32                                FunctionParameterDeclaration
                        #[a2]                                       Attribute
                                      #[what•=•"how"]•c:•u32        FunctionParameterDeclaration
                                      #[what•=•"how"]               Attribute
                                             =                      PunctuationToken
                                               "how"                Literal                                                               */
fn f(#[a1] #[a2] a: i32, #[what = "how"] b: i32, #[e(true)] c: u32) {}                                                                    /*
fn•f(#[a1]•#[a2]•a:•i32,•#[what•=•"how"]•b:•i32,•#[e(true)]•c:•u32)•{}    FunctionDeclaration
     #[a1]•#[a2]•a:•i32                                                   FunctionParameterDeclaration
     #[a1]                                                                Attribute
           #[a2]                                                          Attribute
                         #[what•=•"how"]•b:•i32                           FunctionParameterDeclaration
                         #[what•=•"how"]                                  Attribute
                                =                                         PunctuationToken
                                  "how"                                   Literal
                                                 #[e(true)]•c:•u32        FunctionParameterDeclaration
                                                 #[e(true)]               Attribute
                                                    (true)                DelimGroup
                                                     true                 Literal                                                         */
fn b(#[cfg(x)]x: i32, y: i32) -> i32 {}                                                                                                   /*
fn•b(#[cfg(x)]x:•i32,•y:•i32)•->•i32•{}    FunctionDeclaration
     #[cfg(x)]x:•i32                       FunctionParameterDeclaration
     #[cfg(x)]                             Attribute
          (x)                              DelimGroup
                      y:•i32               FunctionParameterDeclaration                                                                   */
fn f( #[a1] #[a2] &self, #[a1] #[a2] a: i32, #[what = "how"] b: i32, #[f(true)] c: u32 ) {}                                               /*
fn•f(•#[a1]•#[a2]•&self,•#[a1]•#[a2]•a:•i32,•#[what•=•"how"]•b:•i32,•#[f(true)]•c:•u32•)•{}    FunctionDeclaration
      #[a1]•#[a2]•&self                                                                        FunctionSelfParameterDeclaration
      #[a1]                                                                                    Attribute
            #[a2]                                                                              Attribute
                         #[a1]•#[a2]•a:•i32                                                    FunctionParameterDeclaration
                         #[a1]                                                                 Attribute
                               #[a2]                                                           Attribute
                                             #[what•=•"how"]•b:•i32                            FunctionParameterDeclaration
                                             #[what•=•"how"]                                   Attribute
                                                    =                                          PunctuationToken
                                                      "how"                                    Literal
                                                                     #[f(true)]•c:•u32         FunctionParameterDeclaration
                                                                     #[f(true)]                Attribute
                                                                        (true)                 DelimGroup
                                                                         true                  Literal                                    */
fn c( #[cfg(foo)]&mut self,#[deny(C)] b: i32, ) {}                                                                                        /*
fn•c(•#[cfg(foo)]&mut•self,#[deny(C)]•b:•i32,•)•{}    FunctionDeclaration
      #[cfg(foo)]&mut•self                            FunctionSelfParameterDeclaration
      #[cfg(foo)]                                     Attribute
           (foo)                                      DelimGroup
                           #[deny(C)]•b:•i32          FunctionParameterDeclaration
                           #[deny(C)]                 Attribute
                                 (C)                  DelimGroup                                                                          */

#[a = "q"] #[b = "q"] mod a {#![c = "q"]}                                                                                                 /*
#[a•=•"q"]•#[b•=•"q"]•mod•a•{#![c•=•"q"]}    ModuleDeclaration
#[a•=•"q"]                                   Attribute
    =                                        PunctuationToken
      "q"                                    Literal
           #[b•=•"q"]                        Attribute
               =                             PunctuationToken
                 "q"                         Literal
                             #![c•=•"q"]     Attribute
                                  =          PunctuationToken
                                    "q"      Literal                                                                                      */
#[a = "q"] pub static X: u8 = ();                                                                                                         /*
#[a•=•"q"]•pub•static•X:•u8•=•();    StaticVariableDeclaration
#[a•=•"q"]                           Attribute
    =                                PunctuationToken
      "q"                            Literal
           pub                       PubSpecifier
                              ()     TupleLiteral                                                                                         */
#[a = "q"] pub fn f() {}                                                                                                                  /*
#[a•=•"q"]•pub•fn•f()•{}    FunctionDeclaration
#[a•=•"q"]                  Attribute
    =                       PunctuationToken
      "q"                   Literal
           pub              PubSpecifier                                                                                                  */
#[a = "q"] pub mod a {}                                                                                                                   /*
#[a•=•"q"]•pub•mod•a•{}    ModuleDeclaration
#[a•=•"q"]                 Attribute
    =                      PunctuationToken
      "q"                  Literal
           pub             PubSpecifier                                                                                                   */
#[a = "q"] extern "C" {}                                                                                                                  /*
#[a•=•"q"]•extern•"C"•{}    ExternBlockDeclaration
#[a•=•"q"]                  Attribute
    =                       PunctuationToken
      "q"                   Literal
                  "C"       Literal                                                                                                       */

//(#[b(c(#(d = #e),*))]);
//(#[b(c(#(d•=•#e),*))]);    Comment
a!(#[b(c(#(d = #e),*))]);                                                                                                                 /*
a!(#[b(c(#(d•=•#e),*))]);    ExpressionStatement
a!(#[b(c(#(d•=•#e),*))])     MacroInvocation
   #                         PunctuationToken
    [b(c(#(d•=•#e),*))]      DelimGroup
      (c(#(d•=•#e),*))       DelimGroup
        (#(d•=•#e),*)        DelimGroup
         #                   PunctuationToken
          (d•=•#e)           DelimGroup
             =               PunctuationToken
               #             PunctuationToken
                  ,          PunctuationToken
                   *         PunctuationToken                                                                                             */

//(#![b(c(#(d = #e),*))]);
//(#![b(c(#(d•=•#e),*))]);    Comment
a!(#![b(c(#(d = #e),*))]);                                                                                                                /*
a!(#![b(c(#(d•=•#e),*))]);    ExpressionStatement
a!(#![b(c(#(d•=•#e),*))])     MacroInvocation
   #                          PunctuationToken
    !                         PunctuationToken
     [b(c(#(d•=•#e),*))]      DelimGroup
       (c(#(d•=•#e),*))       DelimGroup
         (#(d•=•#e),*)        DelimGroup
          #                   PunctuationToken
           (d•=•#e)           DelimGroup
              =               PunctuationToken
                #             PunctuationToken
                   ,          PunctuationToken
                    *         PunctuationToken                                                                                            */

#[attr] extern "C" { fn ffi(#[a1] a: i32, #[a2] ...); }                                                                                   /*
#[attr]•extern•"C"•{•fn•ffi(#[a1]•a:•i32,•#[a2]•...);•}    ExternBlockDeclaration
#[attr]                                                    Attribute
               "C"                                         Literal
                     fn•ffi(#[a1]•a:•i32,•#[a2]•...);      FunctionDeclaration
                            #[a1]•a:•i32                   FunctionParameterDeclaration
                            #[a1]                          Attribute
                                          #[a2]•...        FunctionSpread
                                          #[a2]            Attribute                                                                      */
#[attr] unsafe extern "C" fn f(a: i32, #[a1] mut args: ...) {}                                                                            /*
#[attr]•unsafe•extern•"C"•fn•f(a:•i32,•#[a1]•mut•args:•...)•{}    FunctionDeclaration
#[attr]                                                           Attribute
               extern•"C"                                         ExternSpecifier
                      "C"                                         Literal
                               a:•i32                             FunctionParameterDeclaration
                                       #[a1]•mut•args:•...        FunctionParameterDeclaration
                                       #[a1]                      Attribute
                                             mut•args             PatternVariableDeclaration
                                                       ...        FunctionSpread                                                          */

impl W {                                                                                                                                  /*
impl•W•{↲    <ImplDeclaration>                                                                                                            */
    #[attr] fn f(#[a1] self, #[a2] a: u8) {}                                                                                              /*
    #[attr]•fn•f(#[a1]•self,•#[a2]•a:•u8)•{}    FunctionDeclaration
    #[attr]                                     Attribute
                 #[a1]•self                     FunctionSelfParameterDeclaration
                 #[a1]                          Attribute
                             #[a2]•a:•u8        FunctionParameterDeclaration
                             #[a2]              Attribute                                                                                 */
    #[attr] fn f(#[a1] &self, #[a2] a: u8) {}                                                                                             /*
    #[attr]•fn•f(#[a1]•&self,•#[a2]•a:•u8)•{}    FunctionDeclaration
    #[attr]                                      Attribute
                 #[a1]•&self                     FunctionSelfParameterDeclaration
                 #[a1]                           Attribute
                              #[a2]•a:•u8        FunctionParameterDeclaration
                              #[a2]              Attribute                                                                                */
    #[attr] fn f<'a>(#[a1] &'a mut self, #[a2] a: u8) {}                                                                                  /*
    #[attr]•fn•f<'a>(#[a1]•&'a•mut•self,•#[a2]•a:•u8)•{}    FunctionDeclaration
    #[attr]                                                 Attribute
                 'a                                         GenericLtParameterDeclaration, LtIdentifier
                     #[a1]•&'a•mut•self                     FunctionSelfParameterDeclaration
                     #[a1]                                  Attribute
                            'a                              LtIdentifier
                                         #[a2]•a:•u8        FunctionParameterDeclaration
                                         #[a2]              Attribute                                                                     */
    #[attr] fn f<'a>(#[a1] self: Box<Self>, #[a2] a: u8) {}                                                                               /*
    #[attr]•fn•f<'a>(#[a1]•self:•Box<Self>,•#[a2]•a:•u8)•{}    FunctionDeclaration
    #[attr]                                                    Attribute
                 'a                                            GenericLtParameterDeclaration, LtIdentifier
                     #[a1]•self:•Box<Self>                     FunctionSelfParameterDeclaration
                     #[a1]                                     Attribute
                                 Box<Self>                     TypeCall
                                            #[a2]•a:•u8        FunctionParameterDeclaration
                                            #[a2]              Attribute                                                                  */
    #[attr] fn f(#[a1] #[a2] a: u8, #[a3] b: u8) {}                                                                                       /*
    #[attr]•fn•f(#[a1]•#[a2]•a:•u8,•#[a3]•b:•u8)•{}    FunctionDeclaration
    #[attr]                                            Attribute
                 #[a1]•#[a2]•a:•u8                     FunctionParameterDeclaration
                 #[a1]                                 Attribute
                       #[a2]                           Attribute
                                    #[a3]•b:•u8        FunctionParameterDeclaration
                                    #[a3]              Attribute                                                                          */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */

trait A {                                                                                                                                 /*
trait•A•{↲    <TraitDeclaration>                                                                                                          */
    #[attr] fn f(#[a1] self, #[a2] a: u8);                                                                                                /*
    #[attr]•fn•f(#[a1]•self,•#[a2]•a:•u8);    FunctionDeclaration
    #[attr]                                   Attribute
                 #[a1]•self                   FunctionSelfParameterDeclaration
                 #[a1]                        Attribute
                             #[a2]•a:•u8      FunctionParameterDeclaration
                             #[a2]            Attribute                                                                                   */
    #[attr] fn f(#[a1] &self, #[a2] a: u8);                                                                                               /*
    #[attr]•fn•f(#[a1]•&self,•#[a2]•a:•u8);    FunctionDeclaration
    #[attr]                                    Attribute
                 #[a1]•&self                   FunctionSelfParameterDeclaration
                 #[a1]                         Attribute
                              #[a2]•a:•u8      FunctionParameterDeclaration
                              #[a2]            Attribute                                                                                  */
    #[attr] fn f<'a>(#[a1] &'a mut self, #[a2] a: u8);                                                                                    /*
    #[attr]•fn•f<'a>(#[a1]•&'a•mut•self,•#[a2]•a:•u8);    FunctionDeclaration
    #[attr]                                               Attribute
                 'a                                       GenericLtParameterDeclaration, LtIdentifier
                     #[a1]•&'a•mut•self                   FunctionSelfParameterDeclaration
                     #[a1]                                Attribute
                            'a                            LtIdentifier
                                         #[a2]•a:•u8      FunctionParameterDeclaration
                                         #[a2]            Attribute                                                                       */
    #[attr] fn f<'a>(#[a1] self: Box<Self>, #[a2] a: u8, #[a3] b: Vec<u8>);                                                               /*
    #[attr]•fn•f<'a>(#[a1]•self:•Box<Self>,•#[a2]•a:•u8,•#[a3]•b:•Vec<u8>);    FunctionDeclaration
    #[attr]                                                                    Attribute
                 'a                                                            GenericLtParameterDeclaration, LtIdentifier
                     #[a1]•self:•Box<Self>                                     FunctionSelfParameterDeclaration
                     #[a1]                                                     Attribute
                                 Box<Self>                                     TypeCall
                                            #[a2]•a:•u8                        FunctionParameterDeclaration
                                            #[a2]                              Attribute
                                                         #[a3]•b:•Vec<u8>      FunctionParameterDeclaration
                                                         #[a3]                 Attribute
                                                                  Vec<u8>      TypeCall                                                   */
    #[attr] fn f(#[a1] #[a2] a: u8, #[a3] b: u8);                                                                                         /*
    #[attr]•fn•f(#[a1]•#[a2]•a:•u8,•#[a3]•b:•u8);    FunctionDeclaration
    #[attr]                                          Attribute
                 #[a1]•#[a2]•a:•u8                   FunctionParameterDeclaration
                 #[a1]                               Attribute
                       #[a2]                         Attribute
                                    #[a3]•b:•u8      FunctionParameterDeclaration
                                    #[a3]            Attribute                                                                            */
}                                                                                                                                         /*
}    </TraitDeclaration>                                                                                                                  */



// directly above #![doc(...
//•directly•above•#![doc(...    Comment
#![doc(___ = "____________________________________________________________",                                                              /*
#![doc(___•=•"____________________________________________________________",↲    <Attribute>
      (___•=•"____________________________________________________________",↲    <DelimGroup>
           =                                                                     PunctuationToken
             "____________________________________________________________"      Literal
                                                                           ,     PunctuationToken                                         */
       ___________________ = "_________________________________________________",                                                         /*
                           =                                                         PunctuationToken
                             "_________________________________________________"     Literal
                                                                                ,    PunctuationToken                                     */
       ______________ = "_________________________________________________",                                                              /*
                      =                                                         PunctuationToken
                        "_________________________________________________"     Literal
                                                                           ,    PunctuationToken                                          */
       ___________________ = "_________________________________________________", a(b(c(d))))]                                            /*
•••••••___________________•=•"_________________________________________________",•a(b(c(d))))]    </Attribute>
•••••••___________________•=•"_________________________________________________",•a(b(c(d))))     </DelimGroup>
                           =                                                                      PunctuationToken
                             "_________________________________________________"                  Literal
                                                                                ,                 PunctuationToken
                                                                                   (b(c(d)))      DelimGroup
                                                                                     (c(d))       DelimGroup
                                                                                       (d)        DelimGroup                              */

//! 1 line below #![doc(...
//!•1•line•below•#![doc(...    DocCommentAttribute

#![attr]                                                                                                                                  /*
#![attr]    Attribute                                                                                                                     */

//! aaa
//!•aaa    DocCommentAttribute

// bbb
//•bbb    Comment

// ccc
//•ccc    Comment
#![attr(b)]                                                                                                                               /*
#![attr(b)]    Attribute
       (b)     DelimGroup                                                                                                                 */

// ddd
//•ddd    Comment

/// eee
                                                                                                                                          /*
///•eee↲    <ImplDeclaration>
///•eee     DocCommentAttribute                                                                                                           */
/// eee
///•eee    DocCommentAttribute
/// eee
///•eee    DocCommentAttribute
/// eee
///•eee    DocCommentAttribute

/// fff
///•fff    DocCommentAttribute
impl Bar {
    /// 0
                                                                                                                                          /*
    ///•0↲    <FunctionDeclaration>
    ///•0     DocCommentAttribute                                                                                                         */
    /// 1
    ///•1    DocCommentAttribute
    /// 2
    ///•2    DocCommentAttribute
    /// 3
    ///•3    DocCommentAttribute
    #[attr]                                                                                                                               /*
    #[attr]    Attribute                                                                                                                  */
    #[doc = " ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ "]                                                              /*
    #[doc•=•"•___•___•___•___•___•___•___•___•___•___•___•___•___•___•___•"]    Attribute
          =                                                                     PunctuationToken
            "•___•___•___•___•___•___•___•___•___•___•___•___•___•___•___•"     Literal                                                   */
    fn foo(&mut self) -> isize {                                                                                                          /*
           &mut•self                FunctionSelfParameterDeclaration                                                                      */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration>                                                                                                           */

    /// \n 4
                                                                                                                                          /*
    ///•\n•4↲    <FunctionDeclaration>
    ///•\n•4     DocCommentAttribute                                                                                                      */
    /// 5
    ///•5    DocCommentAttribute
    /// 6
    ///•6    DocCommentAttribute


    /// \n\n 7
    ///•\n\n•7    DocCommentAttribute
    /// 8
    ///•8    DocCommentAttribute
    /// 9
    ///•9    DocCommentAttribute
    pub fn f2(self) {                                                                                                                     /*
    pub                  PubSpecifier
              self       FunctionSelfParameterDeclaration                                                                                 */
        (foo, bar)                                                                                                                        /*
        (foo,•bar)    ExpressionStatement, TupleLiteral                                                                                   */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration>                                                                                                           */

    #[attr]                                                                                                                               /*
    #[attr]↲    <FunctionDeclaration>
    #[attr]     Attribute                                                                                                                 */
    fn f3(self) -> Q {                                                                                                                    /*
          self            FunctionSelfParameterDeclaration                                                                                */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration>                                                                                                           */

    /// \n 10 \n
                                                                                                                                          /*
    ///•\n•10•\n↲    <FunctionDeclaration>
    ///•\n•10•\n     DocCommentAttribute                                                                                                  */

    #[attrib1]                                                                                                                            /*
    #[attrib1]    Attribute                                                                                                               */
    /// 11
    ///•11    DocCommentAttribute
    #[attrib2]                                                                                                                            /*
    #[attrib2]    Attribute                                                                                                               */
    // 12
    //•12    Comment
    /// 13
    ///•13    DocCommentAttribute
    fn f4(self) -> A {                                                                                                                    /*
          self            FunctionSelfParameterDeclaration                                                                                */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration>                                                                                                           */

    // \n 14
    //•\n•14    Comment
    #[a(b="c")]                                                                                                                           /*
    #[a(b="c")]↲    <FunctionDeclaration>
    #[a(b="c")]     Attribute
       (b="c")      DelimGroup
         =          PunctuationToken
          "c"       Literal                                                                                                               */
    fn f5(self) -> X {}                                                                                                                   /*
••••fn•f5(self)•->•X•{}    </FunctionDeclaration>
          self             FunctionSelfParameterDeclaration                                                                               */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */

struct Foo {                                                                                                                              /*
struct•Foo•{↲    <StructDeclaration>                                                                                                      */
    # [ derive ( A , B , C , D , E ) ]                                                                                                    /*
    #•[•derive•(•A•,•B•,•C•,•D•,•E•)•]↲    <StructPropertyDeclaration>
    #•[•derive•(•A•,•B•,•C•,•D•,•E•)•]     Attribute
               (•A•,•B•,•C•,•D•,•E•)       DelimGroup
                   ,                       PunctuationToken
                       ,                   PunctuationToken
                           ,               PunctuationToken
                               ,           PunctuationToken                                                                               */
    foo: usize,                                                                                                                           /*
••••foo:•usize     </StructPropertyDeclaration>                                                                                           */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */

// expected_{x} comments inside each attribute
//•expected_{x}•comments•inside•each•attribute    Comment

#[should_panic]                                                                                                                           /*
#[should_panic]↲    <FunctionDeclaration>
#[should_panic]     Attribute                                                                                                             */
#[should_panic(expected_0 = "(")]                                                                                                         /*
#[should_panic(expected_0•=•"(")]    Attribute
              (expected_0•=•"(")     DelimGroup
                          =          PunctuationToken
                            "("      Literal                                                                                              */
#[should_panic(expected_1 = /* ( */ "(")]                                                                                                 /*
#[should_panic(expected_1•=•/*•(•*/•"(")]    Attribute
              (expected_1•=•/*•(•*/•"(")     DelimGroup
                          =                  PunctuationToken
                            /*•(•*/          Comment
                                    "("      Literal                                                                                      */
#[should_panic(/* ((((( */expected_4 /* ((((( */= /* ((((( */ "("/* ((((( */)]                                                            /*
#[should_panic(/*•(((((•*/expected_4•/*•(((((•*/=•/*•(((((•*/•"("/*•(((((•*/)]    Attribute
              (/*•(((((•*/expected_4•/*•(((((•*/=•/*•(((((•*/•"("/*•(((((•*/)     DelimGroup
               /*•(((((•*/                                                        Comment
                                     /*•(((((•*/                                  Comment
                                                =                                 PunctuationToken
                                                  /*•(((((•*/                     Comment
                                                              "("                 Literal
                                                                 /*•(((((•*/      Comment                                                 */
#[should_panic(                                                                                                                           /*
#[should_panic(↲    <Attribute>
              (↲    <DelimGroup>                                                                                                          */
    /* (((((((( *//*                                                                                                                      /*
    /*•((((((((•*/       Comment
                  /*↲    <Comment>                                                                                                        */
    (((((((((()(((((((( */                                                                                                                /*
••••(((((((((()((((((((•*/    </Comment>                                                                                                  */
    expected_3 = "("                                                                                                                      /*
               =        PunctuationToken
                 "("    Literal                                                                                                           */
    // ((((((((
    //•((((((((    Comment
)]                                                                                                                                        /*
)]    </Attribute>
)     </DelimGroup>                                                                                                                       */
fn f() {}                                                                                                                                 /*
fn•f()•{}    </FunctionDeclaration>                                                                                                       */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>                                                                                                        */
    #[allow(unreachable_code)] // right of attr
                                                                                                                                          /*
    #[allow(unreachable_code)]•//•right•of•attr↲    <ExpressionStatement>
    #[allow(unreachable_code)]                      Attribute
           (unreachable_code)                       DelimGroup
                               //•right•of•attr     Comment                                                                               */
    Some( Err(error) ) ;                                                                                                                  /*
••••Some(•Err(error)•)•;    </ExpressionStatement>
    Some(•Err(error)•)      CallExpression
          Err(error)        CallExpression                                                                                                */

    #[allow(unreachable_code)]                                                                                                            /*
    #[allow(unreachable_code)]↲    <ExpressionStatement>
    #[allow(unreachable_code)]     Attribute
           (unreachable_code)      DelimGroup                                                                                             */
    // below attr
    //•below•attr    Comment
    Some( Err(error) ) ;                                                                                                                  /*
••••Some(•Err(error)•)•;    </ExpressionStatement>
    Some(•Err(error)•)      CallExpression
          Err(error)        CallExpression                                                                                                */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>                                                                                                        */
    #![ this_is_an_inner_attribute ( foo ) ]                                                                                              /*
    #![•this_is_an_inner_attribute•(•foo•)•]    Attribute
                                   (•foo•)      DelimGroup                                                                                */

    foo();                                                                                                                                /*
    foo();    ExpressionStatement
    foo()     CallExpression                                                                                                              */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

impl InnerAttributes() {                                                                                                                  /*
impl•InnerAttributes()•{↲    <ImplDeclaration>
     InnerAttributes()       TypeFunction                                                                                                 */
    #![ this_is_an_inner_attribute ( foo ) ]                                                                                              /*
    #![•this_is_an_inner_attribute•(•foo•)•]    Attribute
                                   (•foo•)      DelimGroup                                                                                */

    fn f() {}                                                                                                                             /*
    fn•f()•{}    FunctionDeclaration                                                                                                      */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */

mod InnerAttributes {                                                                                                                     /*
mod•InnerAttributes•{↲    <ModuleDeclaration>                                                                                             */
    #![ this_is_an_inner_attribute ( foo ) ]                                                                                              /*
    #![•this_is_an_inner_attribute•(•foo•)•]    Attribute
                                   (•foo•)      DelimGroup                                                                                */
}                                                                                                                                         /*
}    </ModuleDeclaration>                                                                                                                 */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>                                                                                                        */
    // Local
    //•Local    Comment
    # [ attr ( on ( local ) ) ]                                                                                                           /*
    #•[•attr•(•on•(•local•)•)•]↲    <LetVariableDeclaration>
    #•[•attr•(•on•(•local•)•)•]     Attribute
             (•on•(•local•)•)       DelimGroup
                  (•local•)         DelimGroup                                                                                            */
    let x = 3;                                                                                                                            /*
••••let•x•=•3;    </LetVariableDeclaration>
            3     Literal                                                                                                                 */

    // Item
    //•Item    Comment
    # [ attr ( on ( item ) ) ]                                                                                                            /*
    #•[•attr•(•on•(•item•)•)•]↲    <UseStatement>
    #•[•attr•(•on•(•item•)•)•]     Attribute
             (•on•(•item•)•)       DelimGroup
                  (•item•)         DelimGroup                                                                                             */
    use foo;                                                                                                                              /*
••••use•foo;    </UseStatement>
        foo     NamedImport                                                                                                               */

    // Expr
    //•Expr    Comment
    # [ attr ( on ( expr ) ) ]                                                                                                            /*
    #•[•attr•(•on•(•expr•)•)•]↲    <ExpressionStatement>
    #•[•attr•(•on•(•expr•)•)•]     Attribute
             (•on•(•expr•)•)       DelimGroup
                  (•expr•)         DelimGroup                                                                                             */
    {}                                                                                                                                    /*
••••{}    </ExpressionStatement>
    {}    BlockExpression                                                                                                                 */

    // Semi
    //•Semi    Comment
    # [ attr ( on ( semi ) ) ]                                                                                                            /*
    #•[•attr•(•on•(•semi•)•)•]↲    <ExpressionStatement>
    #•[•attr•(•on•(•semi•)•)•]     Attribute
             (•on•(•semi•)•)       DelimGroup
                  (•semi•)         DelimGroup                                                                                             */
    foo();                                                                                                                                /*
••••foo();    </ExpressionStatement>
    foo()     CallExpression                                                                                                              */

    // Mac
    //•Mac    Comment
    # [ attr ( on ( mac ) ) ]                                                                                                             /*
    #•[•attr•(•on•(•mac•)•)•]↲    <ExpressionStatement>
    #•[•attr•(•on•(•mac•)•)•]     Attribute
             (•on•(•mac•)•)       DelimGroup
                  (•mac•)         DelimGroup                                                                                              */
    foo!();                                                                                                                               /*
••••foo!();    </ExpressionStatement>
    foo!()     MacroInvocation                                                                                                            */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

#[derive(Add, Sub, Mul, Div, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Mul)]                                   /*
#[derive(Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Mul)]↲    <TupleStructDeclaration>
#[derive(Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Mul)]     Attribute
        (Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Mul)      DelimGroup
            ,                                                                                               PunctuationToken
                 ,                                                                                          PunctuationToken
                      ,                                                                                     PunctuationToken
                           ,                                                                                PunctuationToken
                                  ,                                                                         PunctuationToken
                                        ,                                                                   PunctuationToken
                                            ,                                                               PunctuationToken
                                                       ,                                                    PunctuationToken
                                                            ,                                               PunctuationToken
                                                                        ,                                   PunctuationToken
                                                                               ,                            PunctuationToken
                                                                                     ,                      PunctuationToken
                                                                                                ,           PunctuationToken              */


///
///    DocCommentAttribute


#[derive(Add, Sub, Mul, Div, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]                           /*
#[derive(Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Deserialize)]    Attribute
        (Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Deserialize)     DelimGroup
            ,                                                                                                      PunctuationToken
                 ,                                                                                                 PunctuationToken
                      ,                                                                                            PunctuationToken
                           ,                                                                                       PunctuationToken
                                  ,                                                                                PunctuationToken
                                        ,                                                                          PunctuationToken
                                            ,                                                                      PunctuationToken
                                                       ,                                                           PunctuationToken
                                                            ,                                                      PunctuationToken
                                                                        ,                                          PunctuationToken
                                                                               ,                                   PunctuationToken
                                                                                     ,                             PunctuationToken
                                                                                                ,                  PunctuationToken       */
pub struct HP(pub u8);                                                                                                                    /*
pub•struct•HP(pub•u8);    </TupleStructDeclaration>
pub                       PubSpecifier
              pub•u8      TupleStructItemDeclaration
              pub         PubSpecifier                                                                                                    */

//
//•    Comment
struct A { #[doc = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"] b: i32 }                     /*
struct•A•{•#[doc•=•"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]•b:•i32•}    StructDeclaration
           #[doc•=•"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]•b:•i32      StructPropertyDeclaration
           #[doc•=•"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]             Attribute
                 =                                                                                                       PunctuationToken
                   "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"              Literal          */

#[cfg(feature = "this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")]                                     /*
#[cfg(feature•=•"this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")]↲    <FunctionDeclaration>
#[cfg(feature•=•"this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")]     Attribute
     (feature•=•"this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")      DelimGroup
              =                                                                                           PunctuationToken
                "this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"       Literal                         */
pub fn foo() {}                                                                                                                           /*
pub•fn•foo()•{}    </FunctionDeclaration>
pub                PubSpecifier                                                                                                           */

//
//•    Comment
#[clippy::bar]                                                                                                                            /*
#[clippy::bar]↲    <FunctionDeclaration>
#[clippy::bar]     Attribute
        ::         PunctuationToken                                                                                                       */
#[clippy::bar(a, b, c)]                                                                                                                   /*
#[clippy::bar(a,•b,•c)]    Attribute
        ::                 PunctuationToken
             (a,•b,•c)     DelimGroup
               ,           PunctuationToken
                  ,        PunctuationToken                                                                                               */
pub fn foo() {}                                                                                                                           /*
pub•fn•foo()•{}    </FunctionDeclaration>
pub                PubSpecifier                                                                                                           */

mod v {                                                                                                                                   /*
mod•v•{↲    <ModuleDeclaration>                                                                                                           */
    #[derive(Debug, StructOpt)]                                                                                                           /*
    #[derive(Debug,•StructOpt)]↲    <StructDeclaration>
    #[derive(Debug,•StructOpt)]     Attribute
            (Debug,•StructOpt)      DelimGroup
                  ,                 PunctuationToken                                                                                      */
    #[structopt(about = "x")]                                                                                                             /*
    #[structopt(about•=•"x")]    Attribute
               (about•=•"x")     DelimGroup
                      =          PunctuationToken
                        "x"      Literal                                                                                                  */
    pub struct Params {                                                                                                                   /*
    pub                    PubSpecifier                                                                                                   */
        #[structopt(help = "x")]                                                                                                          /*
        #[structopt(help•=•"x")]↲    <StructPropertyDeclaration>
        #[structopt(help•=•"x")]     Attribute
                   (help•=•"x")      DelimGroup
                         =           PunctuationToken
                           "x"       Literal                                                                                              */
        server: String,                                                                                                                   /*
••••••••server:•String     </StructPropertyDeclaration>                                                                                   */
        #[structopt(help = "x")]                                                                                                          /*
        #[structopt(help•=•"x")]↲    <StructPropertyDeclaration>
        #[structopt(help•=•"x")]     Attribute
                   (help•=•"x")      DelimGroup
                         =           PunctuationToken
                           "x"       Literal                                                                                              */
        first_name: String,                                                                                                               /*
••••••••first_name:•String     </StructPropertyDeclaration>                                                                               */
        #[structopt(help = "x")]                                                                                                          /*
        #[structopt(help•=•"x")]↲    <StructPropertyDeclaration>
        #[structopt(help•=•"x")]     Attribute
                   (help•=•"x")      DelimGroup
                         =           PunctuationToken
                           "x"       Literal                                                                                              */
        last_name: String,                                                                                                                /*
••••••••last_name:•String     </StructPropertyDeclaration>                                                                                */
        #[structopt(                                                                                                                      /*
        #[structopt(↲    <StructPropertyDeclaration>, <Attribute>
                   (↲    <DelimGroup>                                                                                                     */
            short = "j",                                                                                                                  /*
                  =         PunctuationToken
                    "j"     Literal
                       ,    PunctuationToken                                                                                              */
            long = "job",                                                                                                                 /*
                 =           PunctuationToken
                   "job"     Literal
                        ,    PunctuationToken                                                                                             */
            help = "The job to look at",                                                                                                  /*
                 =                          PunctuationToken
                   "The•job•to•look•at"     Literal
                                       ,    PunctuationToken                                                                              */
            parse(try_from_str)                                                                                                           /*
                 (try_from_str)    DelimGroup                                                                                             */
        )]                                                                                                                                /*
••••••••)]    </Attribute>
••••••••)     </DelimGroup>                                                                                                               */
        job: Option<Job>                                                                                                                  /*
••••••••job:•Option<Job>    </StructPropertyDeclaration>
             Option<Job>    TypeCall                                                                                                      */
    }                                                                                                                                     /*
••••}    </StructDeclaration>                                                                                                             */
}                                                                                                                                         /*
}    </ModuleDeclaration>                                                                                                                 */

#[cfg(not(all(feature="std",                                                                                                              /*
#[cfg(not(all(feature="std",↲    <TypeAliasDeclaration>, <Attribute>
     (not(all(feature="std",↲    <DelimGroup>
         (all(feature="std",↲    <DelimGroup>
             (feature="std",↲    <DelimGroup>
                     =           PunctuationToken
                      "std"      Literal
                           ,     PunctuationToken                                                                                         */
              any(target_os = "linux", target_os = "android",                                                                             /*
                 (target_os•=•"linux",•target_os•=•"android",↲    <DelimGroup>
                            =                                     PunctuationToken
                              "linux"                             Literal
                                     ,                            PunctuationToken
                                                 =                PunctuationToken
                                                   "android"      Literal
                                                            ,     PunctuationToken                                                        */
                  target_os = "netbsd",                                                                                                   /*
                            =              PunctuationToken
                              "netbsd"     Literal
                                      ,    PunctuationToken                                                                               */
                  target_os = "dragonfly",                                                                                                /*
                            =                 PunctuationToken
                              "dragonfly"     Literal
                                         ,    PunctuationToken                                                                            */
                  target_os = "haiku",                                                                                                    /*
                            =             PunctuationToken
                              "haiku"     Literal
                                     ,    PunctuationToken                                                                                */
                  target_os = "emscripten",                                                                                               /*
                            =                  PunctuationToken
                              "emscripten"     Literal
                                          ,    PunctuationToken                                                                           */
                  target_os = "solaris",                                                                                                  /*
                            =               PunctuationToken
                              "solaris"     Literal
                                       ,    PunctuationToken                                                                              */
                  target_os = "cloudabi",                                                                                                 /*
                            =                PunctuationToken
                              "cloudabi"     Literal
                                        ,    PunctuationToken                                                                             */
                  target_os = "macos", target_os = "ios",                                                                                 /*
                            =                                PunctuationToken
                              "macos"                        Literal
                                     ,                       PunctuationToken
                                                 =           PunctuationToken
                                                   "ios"     Literal
                                                        ,    PunctuationToken                                                             */
                  target_os = "freebsd",                                                                                                  /*
                            =               PunctuationToken
                              "freebsd"     Literal
                                       ,    PunctuationToken                                                                              */
                  target_os = "openbsd",                                                                                                  /*
                            =               PunctuationToken
                              "openbsd"     Literal
                                       ,    PunctuationToken                                                                              */
                  target_os = "redox",                                                                                                    /*
                            =             PunctuationToken
                              "redox"     Literal
                                     ,    PunctuationToken                                                                                */
                  target_os = "fuchsia",                                                                                                  /*
                            =               PunctuationToken
                              "fuchsia"     Literal
                                       ,    PunctuationToken                                                                              */
                  windows,                                                                                                                /*
                         ,    PunctuationToken                                                                                            */
                  all(target_arch = "wasm32", feature = "stdweb"),                                                                        /*
                     (target_arch•=•"wasm32",•feature•=•"stdweb")     DelimGroup
                                  =                                   PunctuationToken
                                    "wasm32"                          Literal
                                            ,                         PunctuationToken
                                                      =               PunctuationToken
                                                        "stdweb"      Literal
                                                                 ,    PunctuationToken                                                    */
                  all(target_arch = "wasm32", feature = "wasm-bindgen"),                                                                  /*
                     (target_arch•=•"wasm32",•feature•=•"wasm-bindgen")     DelimGroup
                                  =                                         PunctuationToken
                                    "wasm32"                                Literal
                                            ,                               PunctuationToken
                                                      =                     PunctuationToken
                                                        "wasm-bindgen"      Literal
                                                                       ,    PunctuationToken                                              */
              ))))]                                                                                                                       /*
••••••••••••••))))]    </Attribute>
••••••••••••••))))     </DelimGroup>
••••••••••••••)))      </DelimGroup>
••••••••••••••))       </DelimGroup>
••••••••••••••)        </DelimGroup>                                                                                                      */
type Os = NoSource;                                                                                                                       /*
type•Os•=•NoSource;    </TypeAliasDeclaration>                                                                                            */

fn stmt_expr_attributes() {                                                                                                               /*
fn•stmt_expr_attributes()•{↲    <FunctionDeclaration>                                                                                     */
    let foo ;                                                                                                                             /*
    let•foo•;    LetVariableDeclaration                                                                                                   */
    #[must_use]                                                                                                                           /*
    #[must_use]↲    <ExpressionStatement>
    #[must_use]     Attribute                                                                                                             */
   foo = false ;                                                                                                                          /*
•••foo•=•false•;    </ExpressionStatement>
   foo•=•false      ReassignmentExpression
         false      Literal                                                                                                               */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

fn a() {                                                                                                                                  /*
fn•a()•{↲    <FunctionDeclaration>                                                                                                        */
    match () {                                                                                                                            /*
    match•()•{↲    <ExpressionStatement>, <MatchExpression>
          ()       TupleLiteral                                                                                                           */
        #![attr]                                                                                                                          /*
        #![attr]    Attribute                                                                                                             */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
    match () {                                                                                                                            /*
    match•()•{↲    <ExpressionStatement>, <MatchExpression>
          ()       TupleLiteral                                                                                                           */
        #[attr]                                                                                                                           /*
        #[attr]    Attribute (dangling)                                                                                                   */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

fn x() {                                                                                                                                  /*
fn•x()•{↲    <FunctionDeclaration>                                                                                                        */
    match MyEnum {                                                                                                                        /*
    match•MyEnum•{↲    <ExpressionStatement>, <MatchExpression>                                                                           */

    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// 2 comments inside this attribute
//•2•comments•inside•this•attribute    Comment
#[derive(                                                                                                                                 /*
#[derive(↲    <StructDeclaration>, <Attribute>
        (↲    <DelimGroup>                                                                                                                */
/* ---------- ------------------------------------------------------------------- --------- */                                            /*
/*•----------•-------------------------------------------------------------------•---------•*/    Comment                                 */
Debug, Clone,/* --------------- */Eq, PartialEq,                                                                                          /*
     ,                                              PunctuationToken
            ,                                       PunctuationToken
             /*•---------------•*/                  Comment
                                    ,               PunctuationToken
                                               ,    PunctuationToken                                                                      */
)]                                                                                                                                        /*
)]    </Attribute>
)     </DelimGroup>                                                                                                                       */
struct Foo {
    a: i32,                                                                                                                               /*
    a:•i32     StructPropertyDeclaration                                                                                                  */
    b: T,                                                                                                                                 /*
    b:•T     StructPropertyDeclaration                                                                                                    */
}                                                                                                                                         /*
}    </StructDeclaration>                                                                                                                 */

// 1 comment inside this attribute
//•1•comment•inside•this•attribute    Comment
#[derive(/*Debug, */Clone)]                                                                                                               /*
#[derive(/*Debug,•*/Clone)]↲    <StructDeclaration>
#[derive(/*Debug,•*/Clone)]     Attribute
        (/*Debug,•*/Clone)      DelimGroup
         /*Debug,•*/            Comment                                                                                                   */
struct Foo;                                                                                                                               /*
struct•Foo;    </StructDeclaration>                                                                                                       */

#[cfg(all(any(                                                                                                                            /*
#[cfg(all(any(↲    <ConstVariableDeclaration>, <Attribute>
     (all(any(↲    <DelimGroup>
         (any(↲    <DelimGroup>
             (↲    <DelimGroup>                                                                                                           */
    target_arch = "x86",                                                                                                                  /*
                =           PunctuationToken
                  "x86"     Literal
                       ,    PunctuationToken                                                                                              */
    target_arch = "x86_64",                                                                                                               /*
                =              PunctuationToken
                  "x86_64"     Literal
                          ,    PunctuationToken                                                                                           */
    target_arch = "aarch64",                                                                                                              /*
                =               PunctuationToken
                  "aarch64"     Literal
                           ,    PunctuationToken                                                                                          */
    target_arch = "powerpc64",                                                                                                            /*
                =                 PunctuationToken
                  "powerpc64"     Literal
                             ,    PunctuationToken                                                                                        */
    target_arch = "powerpc64le",                                                                                                          /*
                =                   PunctuationToken
                  "powerpc64le"     Literal
                               ,    PunctuationToken                                                                                      */
    target_arch = "mips64",                                                                                                               /*
                =              PunctuationToken
                  "mips64"     Literal
                          ,    PunctuationToken                                                                                           */
    target_arch = "s390x",                                                                                                                /*
                =             PunctuationToken
                  "s390x"     Literal
                         ,    PunctuationToken                                                                                            */
    target_arch = "sparc64"                                                                                                               /*
                =              PunctuationToken
                  "sparc64"    Literal                                                                                                    */
)))]                                                                                                                                      /*
)))]    </Attribute>
)))     </DelimGroup>
))      </DelimGroup>
)       </DelimGroup>                                                                                                                     */
const MIN_ALIGN: usize = 16;                                                                                                              /*
const•MIN_ALIGN:•usize•=•16;    </ConstVariableDeclaration>
                         16     Literal                                                                                                   */

// Discarded Nodes: 21
// Parsed Nodes: 2181
// state_rollbacks: 45
// Total '.charCodeAt()' calls: 14021 (24% re-reads)
// Unnecessary 'skip_whitespace()' calls: 951
// source: "../../samples/macro/attributes.rs"