// a
//•a↲    <Program>
//•a     Comment{line}

/// b
///•b↲    <Program.ast{dk: "None"}>
///•b↲    <FunctionDeclaration>
///•b     DocCommentAttribute{!inner, line}
fn a() { }                                                                                                                                /*
fn•a()•{•}    FunctionDeclaration~ownStart
    ()        FunctionDeclaration.parameters{dk: "()"}
       {•}    FunctionDeclaration.body{dk: "{}"}
fn•a()•{•}    </FunctionDeclaration>                                                                                                      */

fn b() {                                                                                                                                  /*
fn•b()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    //! c
    //!•c    DocCommentAttribute{inner, line}
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

//////////////////////////////////
//////////////////////////////////    Comment{line}
// d
//•d    Comment{line}
/// let heart = '❤️';
///•let•heart•=•'❤️';↲    <FunctionDeclaration>
///•let•heart•=•'❤️';     DocCommentAttribute{!inner, line}
//////////////////////////////////
//////////////////////////////////    Comment{line}
/// e
///•e    DocCommentAttribute{!inner, line}
/// f  
///•f••    DocCommentAttribute{!inner, line}
fn c() { }                                                                                                                                /*
fn•c()•{•}    FunctionDeclaration~ownStart
    ()        FunctionDeclaration.parameters{dk: "()"}
       {•}    FunctionDeclaration.body{dk: "{}"}
fn•c()•{•}    </FunctionDeclaration>                                                                                                      */

/*                                                                                                                                        /*
/*↲    <Comment{!line}>                                                                                                                 */*/
 * g
 */                                                                                                                                     /*/*
•*/    </Comment>                                                                                                                         */

/**                                                                                                                                       /*
/**↲    <FunctionDeclaration>
/**↲    <DocCommentAttribute{!inner, !line}>                                                                                          */*/*/
 * h
 */                                                                                                                                     /*/*
•*/    </DocCommentAttribute>                                                                                                             */
fn d() { }                                                                                                                                /*
fn•d()•{•}    FunctionDeclaration~ownStart
    ()        FunctionDeclaration.parameters{dk: "()"}
       {•}    FunctionDeclaration.body{dk: "{}"}
fn•d()•{•}    </FunctionDeclaration>                                                                                                      */

fn e() {                                                                                                                                  /*
fn•e()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    /*!                                                                                                                                   /*
    /*!↲    <DocCommentAttribute{inner, !line}>                                                                                         */*/
     * i
     */                                                                                                                                 /*/*
•••••*/    </DocCommentAttribute>                                                                                                         */
	//! ```a
	//!•```a    DocCommentAttribute{inner, line}
    //! b
    //!•b    DocCommentAttribute{inner, line}
    //! ```
    //!•```    DocCommentAttribute{inner, line}
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

/********************************/                                                                                                        /*
/********************************/    Comment{!line}                                                                                      */
/*                                                                                                                                        /*
/*↲    <Comment{!line}>                                                                                                                 */*/
 * j
 */                                                                                                                                     /*/*
•*/    </Comment>                                                                                                                         */

/********************************/                                                                                                        /*
/********************************/    Comment{!line}                                                                                      */
/**                                                                                                                                       /*
/**↲    <FunctionDeclaration>
/**↲    <DocCommentAttribute{!inner, !line}>                                                                                          */*/*/
 * k
 */                                                                                                                                     /*/*
•*/    </DocCommentAttribute>                                                                                                             */
fn f() { }                                                                                                                                /*
fn•f()•{•}    FunctionDeclaration~ownStart
    ()        FunctionDeclaration.parameters{dk: "()"}
       {•}    FunctionDeclaration.body{dk: "{}"}
fn•f()•{•}    </FunctionDeclaration>                                                                                                      */


// before
//•before    Comment{line}
#[macro_use]                                                                                                                              /*
#[macro_use]↲    <ExternCrateStatement>
#[macro_use]     Attribute{!inner}
 [macro_use]     Attribute.segments{dk: "[]"}                                                                                             */
// after
//•after    Comment{line}
extern crate x;                                                                                                                           /*
extern•crate•x;    ExternCrateStatement~ownStart
             x     NamedImport
extern•crate•x;    </ExternCrateStatement>                                                                                                */

a! {                                                                                                                                      /*
a!•{↲    <ExpressionStatement{!semi}>
a!•{↲    <MacroInvocation>
   {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                             */
    /// line 2
    ///•line•2    Comment{line}
    /// line 3
    ///•line•3    Comment{line}
    #[a(b,c(d(e(f="g",h="i"))))]                                                                                                          /*
    #                               PunctuationToken{tk: "#"}
     [a(b,c(d(e(f="g",h="i"))))]    DelimGroup
       (b,c(d(e(f="g",h="i"))))     DelimGroup
         ,                          PunctuationToken{tk: ","}
           (d(e(f="g",h="i")))      DelimGroup
             (e(f="g",h="i"))       DelimGroup
               (f="g",h="i")        DelimGroup
                 =                  PunctuationToken{tk: "="}
                  "g"               Literal{kind: String}
                     ,              PunctuationToken{tk: ","}
                       =            PunctuationToken{tk: "="}
                        "i"         Literal{kind: String}                                                                                 */
    pub enum   X {                                                                                                                        /*
                 {↲    <DelimGroup>                                                                                                       */
        /// line 6
        ///•line•6    Comment{line}
        /// line 7
        ///•line•7    Comment{line}
        A(  B),                                                                                                                           /*
         (••B)     DelimGroup
              ,    PunctuationToken{tk: ","}                                                                                              */

        /// line 10
        ///•line•10    Comment{line}
        C(D  ),                                                                                                                           /*
         (D••)     DelimGroup
              ,    PunctuationToken{tk: ","}                                                                                              */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */
// 5 comments inside a!{}
//•5•comments•inside•a!{}    Comment{line}

/// ____
///•____↲    <TypeAliasDeclaration>
///•____     DocCommentAttribute{!inner, line}
/// ____
///•____    DocCommentAttribute{!inner, line}
#[attr_0]                                                                                                                                 /*
#[attr_0]    Attribute{!inner}
 [attr_0]    Attribute.segments{dk: "[]"}                                                                                                 */
pub type A = Vec<B>;                                                                                                                      /*
pub•type•A•=•Vec<B>;    TypeAliasDeclaration~ownStart
pub                     PubSpecifier
             Vec<B>     TypeCall
                <B>     TypeCall.typeArguments{dk: "<>"}
pub•type•A•=•Vec<B>;    </TypeAliasDeclaration>                                                                                           */

#[attr_1]                                                                                                                                 /*
#[attr_1]↲    <ExternCrateStatement>
#[attr_1]     Attribute{!inner}
 [attr_1]     Attribute.segments{dk: "[]"}                                                                                                */
extern crate b as d;                                                                                                                      /*
extern•crate•b•as•d;    ExternCrateStatement~ownStart
             b•as•d     NamedImport
extern•crate•b•as•d;    </ExternCrateStatement>                                                                                           */

#![attr_2]                                                                                                                                /*
#![attr_2]    Attribute{inner}
  [attr_2]    Attribute.segments{dk: "[]"}                                                                                                */
/**                                                                                                                                       /*
/**↲    <FunctionDeclaration>
/**↲    <DocCommentAttribute{!inner, !line}>                                                                                          */*/*/
 * directly below attr_2 and 1 line above attr_3
 */                                                                                                                                     /*/*
•*/    </DocCommentAttribute>                                                                                                             */

#![attr_3]                                                                                                                                /*
#![attr_3]    Attribute{inner}
  [attr_3]    Attribute.segments{dk: "[]"}                                                                                                */
fn attr_3_3a_target() {                                                                                                                   /*
fn•attr_3_3a_target()•{↲    <FunctionDeclaration~ownStart>
                   ()       FunctionDeclaration.parameters{dk: "()"}
                      {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                          */
    #![attr_3a]                                                                                                                           /*
    #![attr_3a]    Attribute{inner}
      [attr_3a]    Attribute.segments{dk: "[]"}                                                                                           */
    #[attr_3b]                                                                                                                            /*
    #[attr_3b]↲    <FunctionDeclaration>
    #[attr_3b]     Attribute{!inner}
     [attr_3b]     Attribute.segments{dk: "[]"}                                                                                           */
    fn attr_3b_target() {}                                                                                                                /*
    fn•attr_3b_target()•{}    FunctionDeclaration~ownStart
                     ()       FunctionDeclaration.parameters{dk: "()"}
                        {}    FunctionDeclaration.body{dk: "{}"}
••••fn•attr_3b_target()•{}    </FunctionDeclaration>                                                                                      */

    #[attr_3c]                                                                                                                            /*
    #[attr_3c]↲    <FunctionDeclaration>
    #[attr_3c]     Attribute{!inner}
     [attr_3c]     Attribute.segments{dk: "[]"}                                                                                           */
    fn attr_3c_target() {}                                                                                                                /*
    fn•attr_3c_target()•{}    FunctionDeclaration~ownStart
                     ()       FunctionDeclaration.parameters{dk: "()"}
                        {}    FunctionDeclaration.body{dk: "{}"}
••••fn•attr_3c_target()•{}    </FunctionDeclaration>                                                                                      */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */


trait foo_C {                                                                                                                             /*
trait•foo_C•{↲    <TraitDeclaration>
            {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                       */
    #![attr_4]                                                                                                                            /*
    #![attr_4]    Attribute{inner}
      [attr_4]    Attribute.segments{dk: "[]"}                                                                                            */
}                                                                                                                                         /*
}    </TraitDeclaration.body>
}    </TraitDeclaration>                                                                                                                  */

#[attr_5]                                                                                                                                 /*
#[attr_5]↲    <FunctionDeclaration>
#[attr_5]     Attribute{!inner}
 [attr_5]     Attribute.segments{dk: "[]"}                                                                                                */

#![attr_6]                                                                                                                                /*
#![attr_6]    Attribute{inner}
  [attr_6]    Attribute.segments{dk: "[]"}                                                                                                */
fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration~ownStart>
       ()       FunctionDeclaration.parameters{dk: "()"}
          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                      */
    // comment
    //•comment    Comment{line}
	#[attr] ();                                                                                                                           /*
	#[attr]•();    ExpressionStatement{semi}
	        ();    ExpressionStatement~ownStart
	#[attr]        Attribute{!inner}
	 [attr]        Attribute.segments{dk: "[]"}
	        ()     TupleLiteral                                                                                                           */
	#[attr] [1; 4];                                                                                                                       /*
	#[attr]•[1;•4];    ExpressionStatement{semi}
	        [1;•4];    ExpressionStatement~ownStart
	#[attr]            Attribute{!inner}
	 [attr]            Attribute.segments{dk: "[]"}
	        [1;•4]     SizedArrayLiteral
	         1         Literal{kind: Integer}
	            4      Literal{kind: Integer}                                                                                             */
	#[attr] Foo{data: (),};                                                                                                               /*
	#[attr]•Foo{data:•(),};    ExpressionStatement{semi}
	        Foo{data:•(),};    ExpressionStatement~ownStart
	#[attr]                    Attribute{!inner}
	 [attr]                    Attribute.segments{dk: "[]"}
	        Foo{data:•(),}     StructLiteral
	           {data:•(),}     StructLiteral.properties{dk: "{}"}
	            data:•()       StructLiteralProperty
	                  ()       TupleLiteral                                                                                               */
	#[attr] if let Some(_) = Some(true) {}                                                                                                /*
	#[attr]•if•let•Some(_)•=•Some(true)•{}    ExpressionStatement{!semi}
	        if•let•Some(_)•=•Some(true)•{}    ExpressionStatement~ownStart
	#[attr]                                   Attribute{!inner}
	 [attr]                                   Attribute.segments{dk: "[]"}
	        if•let•Some(_)•=•Some(true)•{}    IfBlockExpression
	           let•Some(_)•=•Some(true)       LetScrutinee
	               Some(_)                    TuplePattern
	                   (_)                    TuplePattern.items{dk: "()"}
	                    _                     WildcardPattern
	                         Some(true)       CallExpression
	                             (true)       CallExpression.arguments{dk: "()"}
	                              true        Literal{kind: True}
	                                    {}    IfBlockExpression.body{dk: "{}"}                                                            */
	#[attr] if let Some(_) = Some(true) {} else if let Some(false) = Some(true) {}                                                        /*
	#[attr]•if•let•Some(_)•=•Some(true)•{}•else•if•let•Some(false)•=•Some(true)•{}    ExpressionStatement{!semi}
	        if•let•Some(_)•=•Some(true)•{}•else•if•let•Some(false)•=•Some(true)•{}    ExpressionStatement~ownStart
	#[attr]                                                                           Attribute{!inner}
	 [attr]                                                                           Attribute.segments{dk: "[]"}
	        if•let•Some(_)•=•Some(true)•{}•else•if•let•Some(false)•=•Some(true)•{}    IfBlockExpression
	           let•Some(_)•=•Some(true)                                               LetScrutinee
	               Some(_)                                                            TuplePattern
	                   (_)                                                            TuplePattern.items{dk: "()"}
	                    _                                                             WildcardPattern
	                         Some(true)                                               CallExpression
	                             (true)                                               CallExpression.arguments{dk: "()"}
	                              true                                                Literal{kind: True}
	                                    {}                                            IfBlockExpression.body{dk: "{}"}
	                                            if•let•Some(false)•=•Some(true)•{}    IfBlockExpression
	                                               let•Some(false)•=•Some(true)       LetScrutinee
	                                                   Some(false)                    TuplePattern
	                                                       (false)                    TuplePattern.items{dk: "()"}
	                                                        false                     Literal{kind: False}
	                                                                 Some(true)       CallExpression
	                                                                     (true)       CallExpression.arguments{dk: "()"}
	                                                                      true        Literal{kind: True}
	                                                                            {}    IfBlockExpression.body{dk: "{}"}                    */
	#[attr] if true {}                                                                                                                    /*
	#[attr]•if•true•{}    ExpressionStatement{!semi}
	        if•true•{}    ExpressionStatement~ownStart
	#[attr]               Attribute{!inner}
	 [attr]               Attribute.segments{dk: "[]"}
	        if•true•{}    IfBlockExpression
	           true       Literal{kind: True}
	                {}    IfBlockExpression.body{dk: "{}"}                                                                                */
	#[attr] let _ = 0;                                                                                                                    /*
	#[attr]•let•_•=•0;    LetVariableDeclaration
	        let•_•=•0;    LetVariableDeclaration~ownStart
	#[attr]               Attribute{!inner}
	 [attr]               Attribute.segments{dk: "[]"}
	            _         WildcardPattern
	                0     Literal{kind: Integer}                                                                                          */
	#[attr] if true { } else if false { } else { }                                                                                        /*
	#[attr]•if•true•{•}•else•if•false•{•}•else•{•}    ExpressionStatement{!semi}
	        if•true•{•}•else•if•false•{•}•else•{•}    ExpressionStatement~ownStart
	#[attr]                                           Attribute{!inner}
	 [attr]                                           Attribute.segments{dk: "[]"}
	        if•true•{•}•else•if•false•{•}•else•{•}    IfBlockExpression
	           true                                   Literal{kind: True}
	                {•}                               IfBlockExpression.body{dk: "{}"}
	                         if•false•{•}•else•{•}    IfBlockExpression
	                            false                 Literal{kind: False}
	                                  {•}             IfBlockExpression.body{dk: "{}"}
	                                           {•}    BlockExpression                                                                     */
    { #![attr] foo() }                                                                                                                    /*
    {•#![attr]•foo()•}    ExpressionStatement{!semi}, BlockExpression
      #![attr]            Attribute{inner}
        [attr]            Attribute.segments{dk: "[]"}
               foo()      ExpressionStatement{!semi}, CallExpression
                  ()      CallExpression.arguments{dk: "()"}                                                                              */
    #[attr] if true {}                                                                                                                    /*
    #[attr]•if•true•{}    ExpressionStatement{!semi}
            if•true•{}    ExpressionStatement~ownStart
    #[attr]               Attribute{!inner}
     [attr]               Attribute.segments{dk: "[]"}
            if•true•{}    IfBlockExpression
               true       Literal{kind: True}
                    {}    IfBlockExpression.body{dk: "{}"}                                                                                */
    #[attr] (0, 1);                                                                                                                       /*
    #[attr]•(0,•1);    ExpressionStatement{semi}
            (0,•1);    ExpressionStatement~ownStart
    #[attr]            Attribute{!inner}
     [attr]            Attribute.segments{dk: "[]"}
            (0,•1)     TupleLiteral
             0         Literal{kind: Integer}
                1      Literal{kind: Integer}                                                                                             */
    #[attr] (0,);                                                                                                                         /*
    #[attr]•(0,);    ExpressionStatement{semi}
            (0,);    ExpressionStatement~ownStart
    #[attr]          Attribute{!inner}
     [attr]          Attribute.segments{dk: "[]"}
            (0,)     TupleLiteral
             0       Literal{kind: Integer}                                                                                               */
    #[attr] (0);                                                                                                                          /*
    #[attr]•(0);    ExpressionStatement{semi}
            (0);    ExpressionStatement~ownStart
    #[attr]         Attribute{!inner}
     [attr]         Attribute.segments{dk: "[]"}
             0      Literal{kind: Integer}                                                                                                */
    #[attr] [1, 2, 3];                                                                                                                    /*
    #[attr]•[1,•2,•3];    ExpressionStatement{semi}
            [1,•2,•3];    ExpressionStatement~ownStart
    #[attr]               Attribute{!inner}
     [attr]               Attribute.segments{dk: "[]"}
            [1,•2,•3]     ArrayLiteral
             1            Literal{kind: Integer}
                2         Literal{kind: Integer}
                   3      Literal{kind: Integer}                                                                                          */
    #[attr] { #![attr] }                                                                                                                  /*
    #[attr]•{•#![attr]•}    ExpressionStatement{!semi}
            {•#![attr]•}    ExpressionStatement~ownStart
    #[attr]                 Attribute{!inner}
     [attr]                 Attribute.segments{dk: "[]"}
            {•#![attr]•}    BlockExpression
              #![attr]      Attribute{inner}
                [attr]      Attribute.segments{dk: "[]"}                                                                                  */
    #[attr] { foo(); }                                                                                                                    /*
    #[attr]•{•foo();•}    ExpressionStatement{!semi}
            {•foo();•}    ExpressionStatement~ownStart
    #[attr]               Attribute{!inner}
     [attr]               Attribute.segments{dk: "[]"}
            {•foo();•}    BlockExpression
              foo();      ExpressionStatement{semi}
              foo()       CallExpression
                 ()       CallExpression.arguments{dk: "()"}                                                                              */
    #[attr] 0;                                                                                                                            /*
    #[attr]•0;    ExpressionStatement{semi}
            0;    ExpressionStatement~ownStart
    #[attr]       Attribute{!inner}
     [attr]       Attribute.segments{dk: "[]"}
            0     Literal{kind: Integer}                                                                                                  */
    #[attr] expr_mac!();                                                                                                                  /*
    #[attr]•expr_mac!();    ExpressionStatement{semi}
            expr_mac!();    ExpressionStatement~ownStart
    #[attr]                 Attribute{!inner}
     [attr]                 Attribute.segments{dk: "[]"}
            expr_mac!()     MacroInvocation
                     ()     MacroInvocation.segments{dk: "()"}                                                                            */
    #[attr] foo();                                                                                                                        /*
    #[attr]•foo();    ExpressionStatement{semi}
            foo();    ExpressionStatement~ownStart
    #[attr]           Attribute{!inner}
     [attr]           Attribute.segments{dk: "[]"}
            foo()     CallExpression
               ()     CallExpression.arguments{dk: "()"}                                                                                  */
    #[attr] let x = 1;                                                                                                                    /*
    #[attr]•let•x•=•1;    LetVariableDeclaration
            let•x•=•1;    LetVariableDeclaration~ownStart
    #[attr]               Attribute{!inner}
     [attr]               Attribute.segments{dk: "[]"}
                    1     Literal{kind: Integer}                                                                                          */
    #[attr] match () { _ => { } }                                                                                                         /*
    #[attr]•match•()•{•_•=>•{•}•}    ExpressionStatement{!semi}
            match•()•{•_•=>•{•}•}    ExpressionStatement~ownStart
    #[attr]                          Attribute{!inner}
     [attr]                          Attribute.segments{dk: "[]"}
            match•()•{•_•=>•{•}•}    MatchExpression
                  ()                 TupleLiteral
                     {•_•=>•{•}•}    MatchExpression.cases{dk: "{}"}
                       _•=>•{•}      MatchExpressionCase
                       _             WildcardPattern
                            {•}      BlockExpression                                                                                      */
    #[attr] match () { #![attr] _ => (), }                                                                                                /*
    #[attr]•match•()•{•#![attr]•_•=>•(),•}    ExpressionStatement{!semi}
            match•()•{•#![attr]•_•=>•(),•}    ExpressionStatement~ownStart
    #[attr]                                   Attribute{!inner}
     [attr]                                   Attribute.segments{dk: "[]"}
            match•()•{•#![attr]•_•=>•(),•}    MatchExpression
                  ()                          TupleLiteral
                     {•#![attr]•_•=>•(),•}    MatchExpression.cases{dk: "{}"}
                       #![attr]               Attribute{inner}
                         [attr]               Attribute.segments{dk: "[]"}
                                _•=>•()       MatchExpressionCase
                                _             WildcardPattern
                                     ()       TupleLiteral                                                                                */
    #[attr] unsafe { /**/ }                                                                                                               /*
    #[attr]•unsafe•{•/**/•}    ExpressionStatement{!semi}
            unsafe•{•/**/•}    ExpressionStatement~ownStart
    #[attr]                    Attribute{!inner}
     [attr]                    Attribute.segments{dk: "[]"}
            unsafe•{•/**/•}    BlockExpression{unsafe}
                   {•/**/•}    BlockExpression.body{dk: "{}"}
                     /**/      Comment{!line}                                                                                             */
    || #[attr] return;                                                                                                                    /*
    ||•#[attr]•return;    ExpressionStatement{semi}
    ||•#[attr]•return     ClosureFunctionExpression
    ||                    ClosureFunctionExpression.parameters{dk: "||"}
       #[attr]            Attribute#DANGLING{!inner}
        [attr]            Attribute#DANGLING.segments{dk: "[]"}
               return     ReturnExpression                                                                                                */
	let a = #[attr] [1; 4];                                                                                                               /*
	let•a•=•#[attr]•[1;•4];    LetVariableDeclaration
	        #[attr]•[1;•4]     SizedArrayLiteral
	                [1;•4]     SizedArrayLiteral~ownStart
	        #[attr]            Attribute{!inner}
	         [attr]            Attribute.segments{dk: "[]"}
	                 1         Literal{kind: Integer}
	                    4      Literal{kind: Integer}                                                                                     */
	let a = #[attr] box 0;                                                                                                                /*
	let•a•=•#[attr]•box•0;    LetVariableDeclaration
	        #[attr]•box•0     BoxExpression
	                box•0     BoxExpression~ownStart
	        #[attr]           Attribute{!inner}
	         [attr]           Attribute.segments{dk: "[]"}
	                    0     Literal{kind: Integer}                                                                                      */
    let a = #[attr] [1, 2, 3];                                                                                                            /*
    let•a•=•#[attr]•[1,•2,•3];    LetVariableDeclaration
            #[attr]•[1,•2,•3]     ArrayLiteral
                    [1,•2,•3]     ArrayLiteral~ownStart
            #[attr]               Attribute{!inner}
             [attr]               Attribute.segments{dk: "[]"}
                    [1,•2,•3]     ArrayLiteral.items{dk: "[]"}
                     1            Literal{kind: Integer}
                        2         Literal{kind: Integer}
                           3      Literal{kind: Integer}                                                                                  */
    let a = #[attr] Foo{data: (),};                                                                                                       /*
    let•a•=•#[attr]•Foo{data:•(),};    LetVariableDeclaration
            #[attr]•Foo{data:•(),}     StructLiteral
                    Foo{data:•(),}     StructLiteral~ownStart
            #[attr]                    Attribute{!inner}
             [attr]                    Attribute.segments{dk: "[]"}
                       {data:•(),}     StructLiteral.properties{dk: "{}"}
                        data:•()       StructLiteralProperty
                              ()       TupleLiteral                                                                                       */
    let a = (#[attr] s).data;                                                                                                             /*
    let•a•=•(#[attr]•s).data;    LetVariableDeclaration
            (#[attr]•s).data     MemberExpression{!computed}
             #[attr]             Attribute#DANGLING{!inner}
              [attr]             Attribute#DANGLING.segments{dk: "[]"}                                                                    */
    let a = (#[attr] t).0;                                                                                                                /*
    let•a•=•(#[attr]•t).0;    LetVariableDeclaration
            (#[attr]•t).0     MemberExpression{!computed}
             #[attr]          Attribute#DANGLING{!inner}
              [attr]          Attribute#DANGLING.segments{dk: "[]"}
                        0     Index                                                                                                       */
    let a = (#[attr] v)[0];                                                                                                               /*
    let•a•=•(#[attr]•v)[0];    LetVariableDeclaration
            (#[attr]•v)[0]     MemberExpression{computed}
             #[attr]           Attribute#DANGLING{!inner}
              [attr]           Attribute#DANGLING.segments{dk: "[]"}
                        0      Literal{kind: Integer}                                                                                     */
    let a = #[attr] -0i32;                                                                                                                /*
    let•a•=•#[attr]•-0i32;    LetVariableDeclaration
            #[attr]•-0i32     MinusExpression
                    -0i32     MinusExpression~ownStart
            #[attr]           Attribute{!inner}
             [attr]           Attribute.segments{dk: "[]"}
                     0i32     Literal{kind: Integer}
                      i32     Identifier                                                                                                  */
    let a = #[attr] !0;                                                                                                                   /*
    let•a•=•#[attr]•!0;    LetVariableDeclaration
            #[attr]•!0     NotExpression
                    !0     NotExpression~ownStart
            #[attr]        Attribute{!inner}
             [attr]        Attribute.segments{dk: "[]"}
                     0     Literal{kind: Integer}                                                                                         */
    let a = #[attr] 'c';                                                                                                                  /*
    let•a•=•#[attr]•'c';    LetVariableDeclaration
            #[attr]•'c'     Literal{kind: Char}
                    'c'     Literal~ownStart
            #[attr]         Attribute{!inner}
             [attr]         Attribute.segments{dk: "[]"}                                                                                  */
    let a = #[attr] (..);                                                                                                                 /*
    let•a•=•#[attr]•(..);    LetVariableDeclaration
            #[attr]•(..      RangeLiteral{!last}
                     ..      RangeLiteral~ownStart
            #[attr]          Attribute{!inner}
             [attr]          Attribute.segments{dk: "[]"}                                                                                 */
    let a = #[attr] (..0);                                                                                                                /*
    let•a•=•#[attr]•(..0);    LetVariableDeclaration
            #[attr]•(..0      RangeLiteral{!last}
                     ..0      RangeLiteral~ownStart
            #[attr]           Attribute{!inner}
             [attr]           Attribute.segments{dk: "[]"}
                       0      Literal{kind: Integer}                                                                                      */
    let a = #[attr] (#[attr] 0,);                                                                                                         /*
    let•a•=•#[attr]•(#[attr]•0,);    LetVariableDeclaration
            #[attr]•(#[attr]•0,)     TupleLiteral
                    (#[attr]•0,)     TupleLiteral~ownStart
            #[attr]                  Attribute{!inner}
             [attr]                  Attribute.segments{dk: "[]"}
                    (#[attr]•0,)     TupleLiteral.items{dk: "()"}
                     #[attr]•0       Literal{kind: Integer}
                             0       Literal~ownStart
                     #[attr]         Attribute{!inner}
                      [attr]         Attribute.segments{dk: "[]"}                                                                         */
    let a = #[attr] (#[attr] 0, 0);                                                                                                       /*
    let•a•=•#[attr]•(#[attr]•0,•0);    LetVariableDeclaration
            #[attr]•(#[attr]•0,•0)     TupleLiteral
                    (#[attr]•0,•0)     TupleLiteral~ownStart
            #[attr]                    Attribute{!inner}
             [attr]                    Attribute.segments{dk: "[]"}
                    (#[attr]•0,•0)     TupleLiteral.items{dk: "()"}
                     #[attr]•0         Literal{kind: Integer}
                             0         Literal~ownStart
                     #[attr]           Attribute{!inner}
                      [attr]           Attribute.segments{dk: "[]"}
                                0      Literal{kind: Integer}                                                                             */
    let a = #[attr] ();                                                                                                                   /*
    let•a•=•#[attr]•();    LetVariableDeclaration
            #[attr]•()     TupleLiteral
                    ()     TupleLiteral~ownStart
            #[attr]        Attribute{!inner}
             [attr]        Attribute.segments{dk: "[]"}
                    ()     TupleLiteral.items{dk: "()"}                                                                                   */
    let a = #[attr] (0, 0);                                                                                                               /*
    let•a•=•#[attr]•(0,•0);    LetVariableDeclaration
            #[attr]•(0,•0)     TupleLiteral
                    (0,•0)     TupleLiteral~ownStart
            #[attr]            Attribute{!inner}
             [attr]            Attribute.segments{dk: "[]"}
                    (0,•0)     TupleLiteral.items{dk: "()"}
                     0         Literal{kind: Integer}
                        0      Literal{kind: Integer}                                                                                     */
    let a = #[attr] (0,);                                                                                                                 /*
    let•a•=•#[attr]•(0,);    LetVariableDeclaration
            #[attr]•(0,)     TupleLiteral
                    (0,)     TupleLiteral~ownStart
            #[attr]          Attribute{!inner}
             [attr]          Attribute.segments{dk: "[]"}
                    (0,)     TupleLiteral.items{dk: "()"}
                     0       Literal{kind: Integer}                                                                                       */
    let a = #[attr] (0..);                                                                                                                /*
    let•a•=•#[attr]•(0..);    LetVariableDeclaration
            #[attr]•(0..      RangeLiteral{!last}
                     0..      RangeLiteral~ownStart
            #[attr]           Attribute{!inner}
             [attr]           Attribute.segments{dk: "[]"}
                     0        Literal{kind: Integer}                                                                                      */
    let a = #[attr] (0..0);                                                                                                               /*
    let•a•=•#[attr]•(0..0);    LetVariableDeclaration
            #[attr]•(0..0      RangeLiteral{!last}
                     0..0      RangeLiteral~ownStart
            #[attr]            Attribute{!inner}
             [attr]            Attribute.segments{dk: "[]"}
                     0         Literal{kind: Integer}
                        0      Literal{kind: Integer}                                                                                     */
    let a = #[attr] (0);                                                                                                                  /*
    let•a•=•#[attr]•(0);    LetVariableDeclaration
            #[attr]•(0      Literal{kind: Integer}
                     0      Literal~ownStart
            #[attr]         Attribute{!inner}
             [attr]         Attribute.segments{dk: "[]"}                                                                                  */
    let a = #[attr] (0);                                                                                                                  /*
    let•a•=•#[attr]•(0);    LetVariableDeclaration
            #[attr]•(0      Literal{kind: Integer}
                     0      Literal~ownStart
            #[attr]         Attribute{!inner}
             [attr]         Attribute.segments{dk: "[]"}                                                                                  */
    let a = #[attr] [0, 0];                                                                                                               /*
    let•a•=•#[attr]•[0,•0];    LetVariableDeclaration
            #[attr]•[0,•0]     ArrayLiteral
                    [0,•0]     ArrayLiteral~ownStart
            #[attr]            Attribute{!inner}
             [attr]            Attribute.segments{dk: "[]"}
                    [0,•0]     ArrayLiteral.items{dk: "[]"}
                     0         Literal{kind: Integer}
                        0      Literal{kind: Integer}                                                                                     */
    let a = #[attr] [0; 0];                                                                                                               /*
    let•a•=•#[attr]•[0;•0];    LetVariableDeclaration
            #[attr]•[0;•0]     SizedArrayLiteral
                    [0;•0]     SizedArrayLiteral~ownStart
            #[attr]            Attribute{!inner}
             [attr]            Attribute.segments{dk: "[]"}
                     0         Literal{kind: Integer}
                        0      Literal{kind: Integer}                                                                                     */
    let a = #[attr] { #![attr] };                                                                                                         /*
    let•a•=•#[attr]•{•#![attr]•};    LetVariableDeclaration
            #[attr]•{•#![attr]•}     BlockExpression
                    {•#![attr]•}     BlockExpression~ownStart
            #[attr]                  Attribute{!inner}
             [attr]                  Attribute.segments{dk: "[]"}
                    {•#![attr]•}     BlockExpression.body{dk: "{}"}
                      #![attr]       Attribute{inner}
                        [attr]       Attribute.segments{dk: "[]"}                                                                         */
    let a = #[attr] { #![attr] let _ = (); ()  };                                                                                         /*
    let•a•=•#[attr]•{•#![attr]•let•_•=•();•()••};    LetVariableDeclaration
            #[attr]•{•#![attr]•let•_•=•();•()••}     BlockExpression
                    {•#![attr]•let•_•=•();•()••}     BlockExpression~ownStart
            #[attr]                                  Attribute{!inner}
             [attr]                                  Attribute.segments{dk: "[]"}
                    {•#![attr]•let•_•=•();•()••}     BlockExpression.body{dk: "{}"}
                      #![attr]                       Attribute{inner}
                        [attr]                       Attribute.segments{dk: "[]"}
                               let•_•=•();           LetVariableDeclaration
                                   _                 WildcardPattern
                                       ()            TupleLiteral
                                           ()        ExpressionStatement{!semi}, TupleLiteral                                             */
    let a = #[attr] { #![attr] let _ = (); };                                                                                             /*
    let•a•=•#[attr]•{•#![attr]•let•_•=•();•};    LetVariableDeclaration
            #[attr]•{•#![attr]•let•_•=•();•}     BlockExpression
                    {•#![attr]•let•_•=•();•}     BlockExpression~ownStart
            #[attr]                              Attribute{!inner}
             [attr]                              Attribute.segments{dk: "[]"}
                    {•#![attr]•let•_•=•();•}     BlockExpression.body{dk: "{}"}
                      #![attr]                   Attribute{inner}
                        [attr]                   Attribute.segments{dk: "[]"}
                               let•_•=•();       LetVariableDeclaration
                                   _             WildcardPattern
                                       ()        TupleLiteral                                                                             */
    let a = #[attr] &#[attr] 0;                                                                                                           /*
    let•a•=•#[attr]•&#[attr]•0;    LetVariableDeclaration
            #[attr]•&#[attr]•0     ReferenceExpression{!mut}
                    &#[attr]•0     ReferenceExpression~ownStart
            #[attr]                Attribute{!inner}
             [attr]                Attribute.segments{dk: "[]"}
                     #[attr]       Attribute#DANGLING{!inner}
                      [attr]       Attribute#DANGLING.segments{dk: "[]"}
                             0     Literal{kind: Integer}                                                                                 */
    let a = #[attr] &0;                                                                                                                   /*
    let•a•=•#[attr]•&0;    LetVariableDeclaration
            #[attr]•&0     ReferenceExpression{!mut}
                    &0     ReferenceExpression~ownStart
            #[attr]        Attribute{!inner}
             [attr]        Attribute.segments{dk: "[]"}
                     0     Literal{kind: Integer}                                                                                         */
    let a = #[attr] &mut #[attr] 0;                                                                                                       /*
    let•a•=•#[attr]•&mut•#[attr]•0;    LetVariableDeclaration
            #[attr]•&mut•#[attr]•0     ReferenceExpression{mut}
                    &mut•#[attr]•0     ReferenceExpression~ownStart
            #[attr]                    Attribute{!inner}
             [attr]                    Attribute.segments{dk: "[]"}
                         #[attr]       Attribute#DANGLING{!inner}
                          [attr]       Attribute#DANGLING.segments{dk: "[]"}
                                 0     Literal{kind: Integer}                                                                             */
    let a = #[attr] &mut 0;                                                                                                               /*
    let•a•=•#[attr]•&mut•0;    LetVariableDeclaration
            #[attr]•&mut•0     ReferenceExpression{mut}
                    &mut•0     ReferenceExpression~ownStart
            #[attr]            Attribute{!inner}
             [attr]            Attribute.segments{dk: "[]"}
                         0     Literal{kind: Integer}                                                                                     */
    let a = #[attr] || { #![attr] #[attr] () };                                                                                           /*
    let•a•=•#[attr]•||•{•#![attr]•#[attr]•()•};    LetVariableDeclaration
            #[attr]•||•{•#![attr]•#[attr]•()•}     ClosureFunctionExpression
                    ||•{•#![attr]•#[attr]•()•}     ClosureFunctionExpression~ownStart
            #[attr]                                Attribute{!inner}
             [attr]                                Attribute.segments{dk: "[]"}
                    ||                             ClosureFunctionExpression.parameters{dk: "||"}
                       {•#![attr]•#[attr]•()•}     BlockExpression
                         #![attr]                  Attribute{inner}
                           [attr]                  Attribute.segments{dk: "[]"}
                                  #[attr]•()       ExpressionStatement{!semi}
                                          ()       ExpressionStatement~ownStart
                                  #[attr]          Attribute{!inner}
                                   [attr]          Attribute.segments{dk: "[]"}
                                          ()       TupleLiteral                                                                           */
    let a = #[attr] || #[attr] ();                                                                                                        /*
    let•a•=•#[attr]•||•#[attr]•();    LetVariableDeclaration
            #[attr]•||•#[attr]•()     ClosureFunctionExpression
                    ||•#[attr]•()     ClosureFunctionExpression~ownStart
            #[attr]                   Attribute{!inner}
             [attr]                   Attribute.segments{dk: "[]"}
                    ||                ClosureFunctionExpression.parameters{dk: "||"}
                       #[attr]        Attribute#DANGLING{!inner}
                        [attr]        Attribute#DANGLING.segments{dk: "[]"}
                               ()     TupleLiteral                                                                                        */
    let a = #[attr] 0 + #[attr] 0;                                                                                                        /*
    let•a•=•#[attr]•0•+•#[attr]•0;    LetVariableDeclaration
            #[attr]•0•+•#[attr]•0     OperationExpression{tk: "+"}
                    0•+•#[attr]•0     OperationExpression~ownStart
            #[attr]                   Attribute{!inner}
             [attr]                   Attribute.segments{dk: "[]"}
                    0                 Literal{kind: Integer}
                        #[attr]       Attribute#DANGLING{!inner}
                         [attr]       Attribute#DANGLING.segments{dk: "[]"}
                                0     Literal{kind: Integer}                                                                              */
    let a = #[attr] 0 as usize;                                                                                                           /*
    let•a•=•#[attr]•0•as•usize;    LetVariableDeclaration
            #[attr]•0•as•usize     ExpressionAsTypeCast
                    0•as•usize     ExpressionAsTypeCast~ownStart
            #[attr]                Attribute{!inner}
             [attr]                Attribute.segments{dk: "[]"}
                    0              Literal{kind: Integer}                                                                                 */
    let a = #[attr] 0;                                                                                                                    /*
    let•a•=•#[attr]•0;    LetVariableDeclaration
            #[attr]•0     Literal{kind: Integer}
                    0     Literal~ownStart
            #[attr]       Attribute{!inner}
             [attr]       Attribute.segments{dk: "[]"}                                                                                    */
    let a = #[attr] 0..;                                                                                                                  /*
    let•a•=•#[attr]•0..;    LetVariableDeclaration
            #[attr]•0..     RangeLiteral{!last}
                    0..     RangeLiteral~ownStart
            #[attr]         Attribute{!inner}
             [attr]         Attribute.segments{dk: "[]"}
                    0       Literal{kind: Integer}                                                                                        */
    let a = #[attr] 0..#[attr] 0;                                                                                                         /*
    let•a•=•#[attr]•0..#[attr]•0;    LetVariableDeclaration
            #[attr]•0..#[attr]•0     RangeLiteral{!last}
                    0..#[attr]•0     RangeLiteral~ownStart
            #[attr]                  Attribute{!inner}
             [attr]                  Attribute.segments{dk: "[]"}
                    0                Literal{kind: Integer}
                       #[attr]       Attribute#DANGLING{!inner}
                        [attr]       Attribute#DANGLING.segments{dk: "[]"}
                               0     Literal{kind: Integer}                                                                               */
    let a = #[attr] 1i32.clone();                                                                                                         /*
    let•a•=•#[attr]•1i32.clone();    LetVariableDeclaration
            #[attr]•1i32.clone()     CallExpression
                    1i32.clone()     CallExpression~ownStart
            #[attr]                  Attribute{!inner}
             [attr]                  Attribute.segments{dk: "[]"}
                    1i32             Literal{kind: Integer}
                     i32             Identifier
                              ()     CallExpression.arguments{dk: "()"}                                                                   */
    let a = #[attr] x! { };                                                                                                               /*
    let•a•=•#[attr]•x!•{•};    LetVariableDeclaration
            #[attr]•x!•{•}     MacroInvocation
                    x!•{•}     MacroInvocation~ownStart
            #[attr]            Attribute{!inner}
             [attr]            Attribute.segments{dk: "[]"}
                       {•}     MacroInvocation.segments{dk: "{}"}                                                                         */
    let a = #[attr] x!();                                                                                                                 /*
    let•a•=•#[attr]•x!();    LetVariableDeclaration
            #[attr]•x!()     MacroInvocation
                    x!()     MacroInvocation~ownStart
            #[attr]          Attribute{!inner}
             [attr]          Attribute.segments{dk: "[]"}
                      ()     MacroInvocation.segments{dk: "()"}                                                                           */
    let a = #[attr] x![];                                                                                                                 /*
    let•a•=•#[attr]•x![];    LetVariableDeclaration
            #[attr]•x![]     MacroInvocation
                    x![]     MacroInvocation~ownStart
            #[attr]          Attribute{!inner}
             [attr]          Attribute.segments{dk: "[]"}
                      []     MacroInvocation.segments{dk: "[]"}                                                                           */
    let a = #[attr] false;                                                                                                                /*
    let•a•=•#[attr]•false;    LetVariableDeclaration
            #[attr]•false     Literal{kind: False}
                    false     Literal~ownStart
            #[attr]           Attribute{!inner}
             [attr]           Attribute.segments{dk: "[]"}                                                                                */
    let a = #[attr] x();                                                                                                                  /*
    let•a•=•#[attr]•x();    LetVariableDeclaration
            #[attr]•x()     CallExpression
                    x()     CallExpression~ownStart
            #[attr]         Attribute{!inner}
             [attr]         Attribute.segments{dk: "[]"}
                     ()     CallExpression.arguments{dk: "()"}                                                                            */
    let a = #[attr] x!();                                                                                                                 /*
    let•a•=•#[attr]•x!();    LetVariableDeclaration
            #[attr]•x!()     MacroInvocation
                    x!()     MacroInvocation~ownStart
            #[attr]          Attribute{!inner}
             [attr]          Attribute.segments{dk: "[]"}
                      ()     MacroInvocation.segments{dk: "()"}                                                                           */
    let a = #[attr] x!(#! [attr]);                                                                                                        /*
    let•a•=•#[attr]•x!(#!•[attr]);    LetVariableDeclaration
            #[attr]•x!(#!•[attr])     MacroInvocation
                    x!(#!•[attr])     MacroInvocation~ownStart
            #[attr]                   Attribute{!inner}
             [attr]                   Attribute.segments{dk: "[]"}
                      (#!•[attr])     MacroInvocation.segments{dk: "()"}
                       #              PunctuationToken{tk: "#"}
                        !             PunctuationToken{tk: "!"}
                          [attr]      DelimGroup                                                                                          */
    let a = #[attr] x![];                                                                                                                 /*
    let•a•=•#[attr]•x![];    LetVariableDeclaration
            #[attr]•x![]     MacroInvocation
                    x![]     MacroInvocation~ownStart
            #[attr]          Attribute{!inner}
             [attr]          Attribute.segments{dk: "[]"}
                      []     MacroInvocation.segments{dk: "[]"}                                                                           */
    let a = #[attr] x![#! [attr]];                                                                                                        /*
    let•a•=•#[attr]•x![#!•[attr]];    LetVariableDeclaration
            #[attr]•x![#!•[attr]]     MacroInvocation
                    x![#!•[attr]]     MacroInvocation~ownStart
            #[attr]                   Attribute{!inner}
             [attr]                   Attribute.segments{dk: "[]"}
                      [#!•[attr]]     MacroInvocation.segments{dk: "[]"}
                       #              PunctuationToken{tk: "#"}
                        !             PunctuationToken{tk: "!"}
                          [attr]      DelimGroup                                                                                          */
    let a = #[attr] x! {};                                                                                                                /*
    let•a•=•#[attr]•x!•{};    LetVariableDeclaration
            #[attr]•x!•{}     MacroInvocation
                    x!•{}     MacroInvocation~ownStart
            #[attr]           Attribute{!inner}
             [attr]           Attribute.segments{dk: "[]"}
                       {}     MacroInvocation.segments{dk: "{}"}                                                                          */
    let a = #[attr] x! { #! [attr] };                                                                                                     /*
    let•a•=•#[attr]•x!•{•#!•[attr]•};    LetVariableDeclaration
            #[attr]•x!•{•#!•[attr]•}     MacroInvocation
                    x!•{•#!•[attr]•}     MacroInvocation~ownStart
            #[attr]                      Attribute{!inner}
             [attr]                      Attribute.segments{dk: "[]"}
                       {•#!•[attr]•}     MacroInvocation.segments{dk: "{}"}
                         #               PunctuationToken{tk: "#"}
                          !              PunctuationToken{tk: "!"}
                            [attr]       DelimGroup                                                                                       */
    let a = #[attr] Foo{..s};                                                                                                             /*
    let•a•=•#[attr]•Foo{..s};    LetVariableDeclaration
            #[attr]•Foo{..s}     StructLiteral
                    Foo{..s}     StructLiteral~ownStart
            #[attr]              Attribute{!inner}
             [attr]              Attribute.segments{dk: "[]"}
                       {..s}     StructLiteral.properties{dk: "{}"}
                        ..s      StructLiteralPropertySpread                                                                              */
    let a = #[attr] Foo{data: (), ..s};                                                                                                   /*
    let•a•=•#[attr]•Foo{data:•(),•..s};    LetVariableDeclaration
            #[attr]•Foo{data:•(),•..s}     StructLiteral
                    Foo{data:•(),•..s}     StructLiteral~ownStart
            #[attr]                        Attribute{!inner}
             [attr]                        Attribute.segments{dk: "[]"}
                       {data:•(),•..s}     StructLiteral.properties{dk: "{}"}
                        data:•()           StructLiteralProperty
                              ()           TupleLiteral
                                  ..s      StructLiteralPropertySpread                                                                    */
    let a = #[attr] Foo{data: (),};                                                                                                       /*
    let•a•=•#[attr]•Foo{data:•(),};    LetVariableDeclaration
            #[attr]•Foo{data:•(),}     StructLiteral
                    Foo{data:•(),}     StructLiteral~ownStart
            #[attr]                    Attribute{!inner}
             [attr]                    Attribute.segments{dk: "[]"}
                       {data:•(),}     StructLiteral.properties{dk: "{}"}
                        data:•()       StructLiteralProperty
                              ()       TupleLiteral                                                                                       */
    let a = #[attr] for _ in 0..0 { #![attr] };                                                                                           /*
    let•a•=•#[attr]•for•_•in•0..0•{•#![attr]•};    LetVariableDeclaration
            #[attr]•for•_•in•0..0•{•#![attr]•}     ForInBlockExpression
                    for•_•in•0..0•{•#![attr]•}     ForInBlockExpression~ownStart
            #[attr]                                Attribute{!inner}
             [attr]                                Attribute.segments{dk: "[]"}
                        _                          WildcardPattern
                             0..0                  RangeLiteral{!last}
                             0                     Literal{kind: Integer}
                                0                  Literal{kind: Integer}
                                  {•#![attr]•}     ForInBlockExpression.body{dk: "{}"}
                                    #![attr]       Attribute{inner}
                                      [attr]       Attribute.segments{dk: "[]"}                                                           */
    let a = #[attr] loop { #![attr] };                                                                                                    /*
    let•a•=•#[attr]•loop•{•#![attr]•};    LetVariableDeclaration
            #[attr]•loop•{•#![attr]•}     LoopBlockExpression
                    loop•{•#![attr]•}     LoopBlockExpression~ownStart
            #[attr]                       Attribute{!inner}
             [attr]                       Attribute.segments{dk: "[]"}
                         {•#![attr]•}     LoopBlockExpression.body{dk: "{}"}
                           #![attr]       Attribute{inner}
                             [attr]       Attribute.segments{dk: "[]"}                                                                    */
    let a = #[attr] match 0 { #![attr] () => (), };                                                                                       /*
    let•a•=•#[attr]•match•0•{•#![attr]•()•=>•(),•};    LetVariableDeclaration
            #[attr]•match•0•{•#![attr]•()•=>•(),•}     MatchExpression
                    match•0•{•#![attr]•()•=>•(),•}     MatchExpression~ownStart
            #[attr]                                    Attribute{!inner}
             [attr]                                    Attribute.segments{dk: "[]"}
                          0                            Literal{kind: Integer}
                            {•#![attr]•()•=>•(),•}     MatchExpression.cases{dk: "{}"}
                              #![attr]                 Attribute{inner}
                                [attr]                 Attribute.segments{dk: "[]"}
                                       ()•=>•()        MatchExpressionCase
                                       ()              TuplePattern
                                             ()        TupleLiteral                                                                       */
    let a = #[attr] match 0 { #![attr] _ => (), };                                                                                        /*
    let•a•=•#[attr]•match•0•{•#![attr]•_•=>•(),•};    LetVariableDeclaration
            #[attr]•match•0•{•#![attr]•_•=>•(),•}     MatchExpression
                    match•0•{•#![attr]•_•=>•(),•}     MatchExpression~ownStart
            #[attr]                                   Attribute{!inner}
             [attr]                                   Attribute.segments{dk: "[]"}
                          0                           Literal{kind: Integer}
                            {•#![attr]•_•=>•(),•}     MatchExpression.cases{dk: "{}"}
                              #![attr]                Attribute{inner}
                                [attr]                Attribute.segments{dk: "[]"}
                                       _•=>•()        MatchExpressionCase
                                       _              WildcardPattern
                                            ()        TupleLiteral                                                                        */
    let a = #[attr] move || { #![attr] #[attr] () };                                                                                      /*
    let•a•=•#[attr]•move•||•{•#![attr]•#[attr]•()•};    LetVariableDeclaration
            #[attr]•move•||•{•#![attr]•#[attr]•()•}     ClosureFunctionExpression{move}
                    move•||•{•#![attr]•#[attr]•()•}     ClosureFunctionExpression~ownStart
            #[attr]                                     Attribute{!inner}
             [attr]                                     Attribute.segments{dk: "[]"}
                         ||                             ClosureFunctionExpression.parameters{dk: "||"}
                            {•#![attr]•#[attr]•()•}     BlockExpression
                              #![attr]                  Attribute{inner}
                                [attr]                  Attribute.segments{dk: "[]"}
                                       #[attr]•()       ExpressionStatement{!semi}
                                               ()       ExpressionStatement~ownStart
                                       #[attr]          Attribute{!inner}
                                        [attr]          Attribute.segments{dk: "[]"}
                                               ()       TupleLiteral                                                                      */
    let a = #[attr] move || #[attr] ();                                                                                                   /*
    let•a•=•#[attr]•move•||•#[attr]•();    LetVariableDeclaration
            #[attr]•move•||•#[attr]•()     ClosureFunctionExpression{move}
                    move•||•#[attr]•()     ClosureFunctionExpression~ownStart
            #[attr]                        Attribute{!inner}
             [attr]                        Attribute.segments{dk: "[]"}
                         ||                ClosureFunctionExpression.parameters{dk: "||"}
                            #[attr]        Attribute#DANGLING{!inner}
                             [attr]        Attribute#DANGLING.segments{dk: "[]"}
                                    ()     TupleLiteral                                                                                   */
    let a = #[attr] s.data;                                                                                                               /*
    let•a•=•#[attr]•s.data;    LetVariableDeclaration
            #[attr]•s.data     MemberExpression{!computed}
                    s.data     MemberExpression~ownStart
            #[attr]            Attribute{!inner}
             [attr]            Attribute.segments{dk: "[]"}                                                                               */
    let a = #[attr] t.0;                                                                                                                  /*
    let•a•=•#[attr]•t.0;    LetVariableDeclaration
            #[attr]•t.0     MemberExpression{!computed}
                    t.0     MemberExpression~ownStart
            #[attr]         Attribute{!inner}
             [attr]         Attribute.segments{dk: "[]"}
                      0     Index                                                                                                         */
    let a = #[attr] v[0];                                                                                                                 /*
    let•a•=•#[attr]•v[0];    LetVariableDeclaration
            #[attr]•v[0]     MemberExpression{computed}
                    v[0]     MemberExpression~ownStart
            #[attr]          Attribute{!inner}
             [attr]          Attribute.segments{dk: "[]"}
                      0      Literal{kind: Integer}                                                                                       */
    let a = #[attr] while false { #![attr] };                                                                                             /*
    let•a•=•#[attr]•while•false•{•#![attr]•};    LetVariableDeclaration
            #[attr]•while•false•{•#![attr]•}     WhileBlockExpression
                    while•false•{•#![attr]•}     WhileBlockExpression~ownStart
            #[attr]                              Attribute{!inner}
             [attr]                              Attribute.segments{dk: "[]"}
                          false                  Literal{kind: False}
                                {•#![attr]•}     WhileBlockExpression.body{dk: "{}"}
                                  #![attr]       Attribute{inner}
                                    [attr]       Attribute.segments{dk: "[]"}                                                             */
    let a = #[attr] while let None = Some(()) { #![attr] };                                                                               /*
    let•a•=•#[attr]•while•let•None•=•Some(())•{•#![attr]•};    LetVariableDeclaration
            #[attr]•while•let•None•=•Some(())•{•#![attr]•}     WhileBlockExpression
                    while•let•None•=•Some(())•{•#![attr]•}     WhileBlockExpression~ownStart
            #[attr]                                            Attribute{!inner}
             [attr]                                            Attribute.segments{dk: "[]"}
                          let•None•=•Some(())                  LetScrutinee
                                     Some(())                  CallExpression
                                         (())                  CallExpression.arguments{dk: "()"}
                                          ()                   TupleLiteral
                                              {•#![attr]•}     WhileBlockExpression.body{dk: "{}"}
                                                #![attr]       Attribute{inner}
                                                  [attr]       Attribute.segments{dk: "[]"}                                               */
    let a = #[attr] x += 15;                                                                                                              /*
    let•a•=•#[attr]•x•+=•15;    LetVariableDeclaration
            #[attr]•x•+=•15     ReassignmentOperationExpression{tk: "+="}
                    x•+=•15     ReassignmentOperationExpression~ownStart
            #[attr]             Attribute{!inner}
             [attr]             Attribute.segments{dk: "[]"}
                         15     Literal{kind: Integer}                                                                                    */
    let a = #[attr] (x += 15);                                                                                                            /*
    let•a•=•#[attr]•(x•+=•15);    LetVariableDeclaration
            #[attr]•(x•+=•15      ReassignmentOperationExpression{tk: "+="}
                     x•+=•15      ReassignmentOperationExpression~ownStart
            #[attr]               Attribute{!inner}
             [attr]               Attribute.segments{dk: "[]"}
                          15      Literal{kind: Integer}                                                                                  */
    let a = #[attr] x = 15;                                                                                                               /*
    let•a•=•#[attr]•x•=•15;    LetVariableDeclaration
            #[attr]•x•=•15     ReassignmentExpression{tk: "="}
                    x•=•15     ReassignmentExpression~ownStart
            #[attr]            Attribute{!inner}
             [attr]            Attribute.segments{dk: "[]"}
                        15     Literal{kind: Integer}                                                                                     */
    let a = #[attr] (x = 15);                                                                                                             /*
    let•a•=•#[attr]•(x•=•15);    LetVariableDeclaration
            #[attr]•(x•=•15      ReassignmentExpression{tk: "="}
                     x•=•15      ReassignmentExpression~ownStart
            #[attr]              Attribute{!inner}
             [attr]              Attribute.segments{dk: "[]"}
                         15      Literal{kind: Integer}                                                                                   */
    let a: [(); 0] = #[attr] [];                                                                                                          /*
    let•a:•[();•0]•=•#[attr]•[];    LetVariableDeclaration
           [();•0]                  TypeSizedArray
            ()                      TypeTuple
                0                   Literal{kind: Integer}
                     #[attr]•[]     ArrayLiteral
                             []     ArrayLiteral~ownStart
                     #[attr]        Attribute{!inner}
                      [attr]        Attribute.segments{dk: "[]"}
                             []     ArrayLiteral.items{dk: "[]"}                                                                          */
    let a: fn(&u32) -> u32 = #[attr] std::clone::Clone::clone;                                                                            /*
    let•a:•fn(&u32)•->•u32•=•#[attr]•std::clone::Clone::clone;    LetVariableDeclaration
           fn(&u32)•->•u32                                        TypeFnPointer
             (&u32)                                               TypeFnPointer.parameters{dk: "()"}
              &u32                                                TypeFnPointerParameter, TypeReference{!mut}
                             #[attr]•std::clone::Clone::clone     ExpressionPath
                                     std::clone::Clone::clone     ExpressionPath~ownStart
                             #[attr]                              Attribute{!inner}
                              [attr]                              Attribute.segments{dk: "[]"}
                                     std::clone::Clone            ExpressionPath
                                     std::clone                   ExpressionPath                                                          */
    let a = #[attr] 1;                                                                                                                    /*
    let•a•=•#[attr]•1;    LetVariableDeclaration
            #[attr]•1     Literal{kind: Integer}
                    1     Literal~ownStart
            #[attr]       Attribute{!inner}
             [attr]       Attribute.segments{dk: "[]"}                                                                                    */
    let a = |                                                                                                                             /*
    let•a•=•|↲    <LetVariableDeclaration>
            |↲    <ClosureFunctionExpression>
            |↲    <ClosureFunctionExpression.parameters{dk: "||"}>                                                                        */
    #[allow(C)]a: u32,                                                                                                                    /*
    #[allow(C)]a:•u32    ClosureFunctionParameterDeclaration
               a:•u32    ClosureFunctionParameterDeclaration~ownStart
    #[allow(C)]          Attribute{!inner}
     [allow(C)]          Attribute.segments{dk: "[]"}
           (C)           DelimGroup                                                                                                       */
    #[cfg(something)] b: i32,                                                                                                             /*
    #[cfg(something)]•b:•i32    ClosureFunctionParameterDeclaration
                      b:•i32    ClosureFunctionParameterDeclaration~ownStart
    #[cfg(something)]           Attribute{!inner}
     [cfg(something)]           Attribute.segments{dk: "[]"}
         (something)            DelimGroup                                                                                                */
    #[cfg_attr(something, cfg(nothing))]#[deny(C)] c: i32,                                                                                /*
    #[cfg_attr(something,•cfg(nothing))]#[deny(C)]•c:•i32    ClosureFunctionParameterDeclaration
                                                   c:•i32    ClosureFunctionParameterDeclaration~ownStart
    #[cfg_attr(something,•cfg(nothing))]                     Attribute{!inner}
     [cfg_attr(something,•cfg(nothing))]                     Attribute.segments{dk: "[]"}
              (something,•cfg(nothing))                      DelimGroup
                        ,                                    PunctuationToken{tk: ","}
                             (nothing)                       DelimGroup
                                        #[deny(C)]           Attribute{!inner}
                                         [deny(C)]           Attribute.segments{dk: "[]"}
                                              (C)            DelimGroup                                                                   */
    | {};                                                                                                                                 /*
••••|        </ClosureFunctionExpression.parameters>
      {}     BlockExpression
••••|•{}     </ClosureFunctionExpression>
••••|•{};    </LetVariableDeclaration>                                                                                                    */
    qux(3 + #[attr] 2);                                                                                                                   /*
    qux(3•+•#[attr]•2);    ExpressionStatement{semi}
    qux(3•+•#[attr]•2)     CallExpression
       (3•+•#[attr]•2)     CallExpression.arguments{dk: "()"}
        3•+•#[attr]•2      OperationExpression{tk: "+"}
        3                  Literal{kind: Integer}
            #[attr]        Attribute#DANGLING{!inner}
             [attr]        Attribute#DANGLING.segments{dk: "[]"}
                    2      Literal{kind: Integer}                                                                                         */
    foo3(x, #[attr] y, z);                                                                                                                /*
    foo3(x,•#[attr]•y,•z);    ExpressionStatement{semi}
    foo3(x,•#[attr]•y,•z)     CallExpression
        (x,•#[attr]•y,•z)     CallExpression.arguments{dk: "()"}
            #[attr]•y         Identifier
                    y         Identifier~ownStart
            #[attr]           Attribute{!inner}
             [attr]           Attribute.segments{dk: "[]"}                                                                                */
    while false { let _ = #[attr] continue ; }                                                                                            /*
    while•false•{•let•_•=•#[attr]•continue•;•}    ExpressionStatement{!semi}, WhileBlockExpression
          false                                   Literal{kind: False}
                {•let•_•=•#[attr]•continue•;•}    WhileBlockExpression.body{dk: "{}"}
                  let•_•=•#[attr]•continue•;      LetVariableDeclaration
                      _                           WildcardPattern
                          #[attr]•continue        ContinueExpression
                                  continue        ContinueExpression~ownStart
                          #[attr]                 Attribute{!inner}
                           [attr]                 Attribute.segments{dk: "[]"}                                                            */
    while true { let _ = #[attr] break ; }                                                                                                /*
    while•true•{•let•_•=•#[attr]•break•;•}    ExpressionStatement{!semi}, WhileBlockExpression
          true                                Literal{kind: True}
               {•let•_•=•#[attr]•break•;•}    WhileBlockExpression.body{dk: "{}"}
                 let•_•=•#[attr]•break•;      LetVariableDeclaration
                     _                        WildcardPattern
                         #[attr]•break        BreakExpression
                                 break        BreakExpression~ownStart
                         #[attr]              Attribute{!inner}
                          [attr]              Attribute.segments{dk: "[]"}                                                                */
    match (Q {#[attr] C: 1 }) {                                                                                                           /*
    match•(Q•{#[attr]•C:•1•})•{↲    <ExpressionStatement{!semi}>
    match•(Q•{#[attr]•C:•1•})•{↲    <MatchExpression>
           Q•{#[attr]•C:•1•}        StructLiteral
             {#[attr]•C:•1•}        StructLiteral.properties{dk: "{}"}
              #[attr]•C:•1          StructLiteralProperty
                      C:•1          StructLiteralProperty~ownStart
              #[attr]               Attribute{!inner}
               [attr]               Attribute.segments{dk: "[]"}
                         1          Literal{kind: Integer}
                              {↲    <MatchExpression.cases{dk: "{}"}>                                                                     */
        Q { #[attr] C } => {}                                                                                                             /*
        Q•{•#[attr]•C•}•=>•{}    MatchExpressionCase
        Q•{•#[attr]•C•}          StructPattern
          {•#[attr]•C•}          StructPattern.properties{dk: "{}"}
            #[attr]•C            StructPatternPropertyShorthand{!box, !ref, !mut}
                    C            StructPatternPropertyShorthand~ownStart
            #[attr]              Attribute{!inner}
             [attr]              Attribute.segments{dk: "[]"}
                           {}    BlockExpression                                                                                          */
        Q ( #[attr] C ) => {}                                                                                                             /*
        Q•(•#[attr]•C•)•=>•{}    MatchExpressionCase
        Q•(•#[attr]•C•)          TuplePattern
          (•#[attr]•C•)          TuplePattern.items{dk: "()"}
            #[attr]•C            Identifier
                    C            Identifier~ownStart
            #[attr]              Attribute{!inner}
             [attr]              Attribute.segments{dk: "[]"}
                           {}    BlockExpression                                                                                          */
		#[attr] _ => {}                                                                                                                   /*
		#[attr]•_•=>•{}    MatchExpressionCase
		        _•=>•{}    MatchExpressionCase~ownStart
		#[attr]            Attribute{!inner}
		 [attr]            Attribute.segments{dk: "[]"}
		        _          WildcardPattern
		             {}    BlockExpression                                                                                                */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
static X: &[Y] = &[                                                                                                                       /*
static•X:•&[Y]•=•&[↲    <StaticVariableDeclaration>
          &[Y]          TypeReference{!mut}
           [Y]          TypeSlice
                 &[↲    <ReferenceExpression{!mut}>
                  [↲    <ArrayLiteral>                                                                                                    */
	#[a(b = "c")]                                                                                                                         /*
	#[a(b•=•"c")]↲    <Literal{kind: Integer}>
	#[a(b•=•"c")]     Attribute{!inner}
	 [a(b•=•"c")]     Attribute.segments{dk: "[]"}
	   (b•=•"c")      DelimGroup
	      =           PunctuationToken{tk: "="}
	        "c"       Literal{kind: String}                                                                                               */
	0,                                                                                                                                    /*
	0    Literal~ownStart
   ╚0    </Literal>                                                                                                                       */
];                                                                                                                                        /*
]     </ArrayLiteral>
]     </ReferenceExpression>
];    </StaticVariableDeclaration>                                                                                                        */
#[attr] const C: C = C { #[attr] field: 0, #[attr] field: 1 };                                                                            /*
#[attr]•const•C:•C•=•C•{•#[attr]•field:•0,•#[attr]•field:•1•};    ConstVariableDeclaration
        const•C:•C•=•C•{•#[attr]•field:•0,•#[attr]•field:•1•};    ConstVariableDeclaration~ownStart
#[attr]                                                           Attribute{!inner}
 [attr]                                                           Attribute.segments{dk: "[]"}
                     C•{•#[attr]•field:•0,•#[attr]•field:•1•}     StructLiteral
                       {•#[attr]•field:•0,•#[attr]•field:•1•}     StructLiteral.properties{dk: "{}"}
                         #[attr]•field:•0                         StructLiteralProperty
                                 field:•0                         StructLiteralProperty~ownStart
                         #[attr]                                  Attribute{!inner}
                          [attr]                                  Attribute.segments{dk: "[]"}
                                        0                         Literal{kind: Integer}
                                           #[attr]•field:•1       StructLiteralProperty
                                                   field:•1       StructLiteralProperty~ownStart
                                           #[attr]                Attribute{!inner}
                                            [attr]                Attribute.segments{dk: "[]"}
                                                          1       Literal{kind: Integer}                                                  */
#[attr] struct S;                                                                                                                         /*
#[attr]•struct•S;    StructDeclaration
        struct•S;    StructDeclaration~ownStart
#[attr]              Attribute{!inner}
 [attr]              Attribute.segments{dk: "[]"}                                                                                         */
#[attr] struct I { #[attr] i: u8, #[attr] pub i: u8, }                                                                                    /*
#[attr]•struct•I•{•#[attr]•i:•u8,•#[attr]•pub•i:•u8,•}    StructDeclaration
        struct•I•{•#[attr]•i:•u8,•#[attr]•pub•i:•u8,•}    StructDeclaration~ownStart
#[attr]                                                   Attribute{!inner}
 [attr]                                                   Attribute.segments{dk: "[]"}
                 {•#[attr]•i:•u8,•#[attr]•pub•i:•u8,•}    StructDeclaration.properties{dk: "{}"}
                   #[attr]•i:•u8                          StructPropertyDeclaration
                           i:•u8                          StructPropertyDeclaration~ownStart
                   #[attr]                                Attribute{!inner}
                    [attr]                                Attribute.segments{dk: "[]"}
                                  #[attr]•pub•i:•u8       StructPropertyDeclaration
                                          pub•i:•u8       StructPropertyDeclaration~ownStart
                                  #[attr]                 Attribute{!inner}
                                   [attr]                 Attribute.segments{dk: "[]"}
                                          pub             PubSpecifier                                                                    */
#[attr] struct I ( #[attr] u8, #[attr] pub u8, )                                                                                          /*
#[attr]•struct•I•(•#[attr]•u8,•#[attr]•pub•u8,•)    TupleStructDeclaration
        struct•I•(•#[attr]•u8,•#[attr]•pub•u8,•)    TupleStructDeclaration~ownStart
#[attr]                                             Attribute{!inner}
 [attr]                                             Attribute.segments{dk: "[]"}
                 (•#[attr]•u8,•#[attr]•pub•u8,•)    TupleStructDeclaration.items{dk: "()"}
                   #[attr]•u8                       TupleStructItemDeclaration
                           u8                       TupleStructItemDeclaration~ownStart
                   #[attr]                          Attribute{!inner}
                    [attr]                          Attribute.segments{dk: "[]"}
                               #[attr]•pub•u8       TupleStructItemDeclaration
                                       pub•u8       TupleStructItemDeclaration~ownStart
                               #[attr]              Attribute{!inner}
                                [attr]              Attribute.segments{dk: "[]"}
                                       pub          PubSpecifier                                                                          */
#[attr] struct BreaksWithComment ( #[attr] u8, #[attr] // comment
                                                                                                                                          /*
#[attr]•struct•BreaksWithComment•(•#[attr]•u8,•#[attr]•//•comment↲    <TupleStructDeclaration>
        struct•BreaksWithComment•(•#[attr]•u8,•#[attr]•//•comment↲    <TupleStructDeclaration~ownStart>
#[attr]                                                               Attribute{!inner}
 [attr]                                                               Attribute.segments{dk: "[]"}
                                 (•#[attr]•u8,•#[attr]•//•comment↲    <TupleStructDeclaration.items{dk: "()"}>
                                   #[attr]•u8                         TupleStructItemDeclaration
                                           u8                         TupleStructItemDeclaration~ownStart
                                   #[attr]                            Attribute{!inner}
                                    [attr]                            Attribute.segments{dk: "[]"}
                                               #[attr]•//•comment↲    <TupleStructItemDeclaration>
                                               #[attr]                Attribute{!inner}
                                                [attr]                Attribute.segments{dk: "[]"}                                        */
                                                       //•comment     Comment{line}
    pub u8, )                                                                                                                             /*
    pub•u8       TupleStructItemDeclaration~ownStart
    pub          PubSpecifier
••••pub•u8       </TupleStructItemDeclaration>
••••pub•u8,•)    </TupleStructDeclaration.items>
••••pub•u8,•)    </TupleStructDeclaration>                                                                                                */
struct C {                                                                                                                                /*
struct•C•{↲    <StructDeclaration>
         {↲    <StructDeclaration.properties{dk: "{}"}>                                                                                   */
    #[attr]                                                                                                                               /*
    #[attr]↲    <StructPropertyDeclaration>
    #[attr]     Attribute{!inner}
     [attr]     Attribute.segments{dk: "[]"}                                                                                              */
    /// below attr
    ///•below•attr    DocCommentAttribute{!inner, line}
    a: b,                                                                                                                                 /*
    a:•b    StructPropertyDeclaration~ownStart
••••a:•b    </StructPropertyDeclaration>                                                                                                  */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */
struct Q { #[attr] C: i32, }                                                                                                              /*
struct•Q•{•#[attr]•C:•i32,•}    StructDeclaration
         {•#[attr]•C:•i32,•}    StructDeclaration.properties{dk: "{}"}
           #[attr]•C:•i32       StructPropertyDeclaration
                   C:•i32       StructPropertyDeclaration~ownStart
           #[attr]              Attribute{!inner}
            [attr]              Attribute.segments{dk: "[]"}                                                                              */
struct A<#[attr] 'a>();                                                                                                                   /*
struct•A<#[attr]•'a>();    TupleStructDeclaration
        <#[attr]•'a>       TupleStructDeclaration.generics{dk: "<>"}
         #[attr]•'a        GenericLtParameterDeclaration
                 'a        GenericLtParameterDeclaration~ownStart
         #[attr]           Attribute{!inner}
          [attr]           Attribute.segments{dk: "[]"}
                 'a        LtIdentifier
                    ()     TupleStructDeclaration.items{dk: "()"}                                                                         */
struct A<#[attr] I>(I);                                                                                                                   /*
struct•A<#[attr]•I>(I);    TupleStructDeclaration
        <#[attr]•I>        TupleStructDeclaration.generics{dk: "<>"}
         #[attr]•I         GenericTypeParameterDeclaration
                 I         GenericTypeParameterDeclaration~ownStart
         #[attr]           Attribute{!inner}
          [attr]           Attribute.segments{dk: "[]"}
                   (I)     TupleStructDeclaration.items{dk: "()"}
                    I      TupleStructItemDeclaration                                                                                     */
enum E { #[attr] C(i32), }                                                                                                                /*
enum•E•{•#[attr]•C(i32),•}    EnumDeclaration
       {•#[attr]•C(i32),•}    EnumDeclaration.members{dk: "{}"}
         #[attr]•C(i32)       EnumMemberTupleDeclaration
                 C(i32)       EnumMemberTupleDeclaration~ownStart
         #[attr]              Attribute{!inner}
          [attr]              Attribute.segments{dk: "[]"}
                  (i32)       EnumMemberTupleDeclaration.items{dk: "()"}
                   i32        TupleStructItemDeclaration                                                                                  */
enum E<#[attr] 'b> {}                                                                                                                     /*
enum•E<#[attr]•'b>•{}    EnumDeclaration
      <#[attr]•'b>       EnumDeclaration.generics{dk: "<>"}
       #[attr]•'b        GenericLtParameterDeclaration
               'b        GenericLtParameterDeclaration~ownStart
       #[attr]           Attribute{!inner}
        [attr]           Attribute.segments{dk: "[]"}
               'b        LtIdentifier
                   {}    EnumDeclaration.members{dk: "{}"}                                                                                */
enum E<#[attr] J> {}                                                                                                                      /*
enum•E<#[attr]•J>•{}    EnumDeclaration
      <#[attr]•J>       EnumDeclaration.generics{dk: "<>"}
       #[attr]•J        GenericTypeParameterDeclaration
               J        GenericTypeParameterDeclaration~ownStart
       #[attr]          Attribute{!inner}
        [attr]          Attribute.segments{dk: "[]"}
                  {}    EnumDeclaration.members{dk: "{}"}                                                                                 */
trait T { #![attr] }                                                                                                                      /*
trait•T•{•#![attr]•}    TraitDeclaration
        {•#![attr]•}    TraitDeclaration.body{dk: "{}"}
          #![attr]      Attribute{inner}
            [attr]      Attribute.segments{dk: "[]"}                                                                                      */
trait T<#[attr] 'c> {}                                                                                                                    /*
trait•T<#[attr]•'c>•{}    TraitDeclaration
       <#[attr]•'c>       TraitDeclaration.generics{dk: "<>"}
        #[attr]•'c        GenericLtParameterDeclaration
                'c        GenericLtParameterDeclaration~ownStart
        #[attr]           Attribute{!inner}
         [attr]           Attribute.segments{dk: "[]"}
                'c        LtIdentifier
                    {}    TraitDeclaration.body{dk: "{}"}                                                                                 */
trait T<#[attr] K> {}                                                                                                                     /*
trait•T<#[attr]•K>•{}    TraitDeclaration
       <#[attr]•K>       TraitDeclaration.generics{dk: "<>"}
        #[attr]•K        GenericTypeParameterDeclaration
                K        GenericTypeParameterDeclaration~ownStart
        #[attr]          Attribute{!inner}
         [attr]          Attribute.segments{dk: "[]"}
                   {}    TraitDeclaration.body{dk: "{}"}                                                                                  */
type Y<#[attr] 'd> = ();                                                                                                                  /*
type•Y<#[attr]•'d>•=•();    TypeAliasDeclaration
      <#[attr]•'d>          TypeAliasDeclaration.generics{dk: "<>"}
       #[attr]•'d           GenericLtParameterDeclaration
               'd           GenericLtParameterDeclaration~ownStart
       #[attr]              Attribute{!inner}
        [attr]              Attribute.segments{dk: "[]"}
               'd           LtIdentifier
                     ()     TypeTuple                                                                                                     */
type Y<#[attr] L> = ();                                                                                                                   /*
type•Y<#[attr]•L>•=•();    TypeAliasDeclaration
      <#[attr]•L>          TypeAliasDeclaration.generics{dk: "<>"}
       #[attr]•L           GenericTypeParameterDeclaration
               L           GenericTypeParameterDeclaration~ownStart
       #[attr]             Attribute{!inner}
        [attr]             Attribute.segments{dk: "[]"}
                    ()     TypeTuple                                                                                                      */
#[attr] type A = fn(#[a1] u8, #[a2] ...);                                                                                                 /*
#[attr]•type•A•=•fn(#[a1]•u8,•#[a2]•...);    TypeAliasDeclaration
        type•A•=•fn(#[a1]•u8,•#[a2]•...);    TypeAliasDeclaration~ownStart
#[attr]                                      Attribute{!inner}
 [attr]                                      Attribute.segments{dk: "[]"}
                 fn(#[a1]•u8,•#[a2]•...)     TypeFnPointer
                   (#[a1]•u8,•#[a2]•...)     TypeFnPointer.parameters{dk: "()"}
                    #[a1]•u8                 TypeFnPointerParameter
                          u8                 TypeFnPointerParameter~ownStart
                    #[a1]                    Attribute{!inner}
                     [a1]                    Attribute.segments{dk: "[]"}
                              #[a2]•...      FunctionSpread
                                    ...      FunctionSpread~ownStart
                              #[a2]          Attribute{!inner}
                               [a2]          Attribute.segments{dk: "[]"}                                                                 */
impl<#[attr] 'e> A<'e> {}                                                                                                                 /*
impl<#[attr]•'e>•A<'e>•{}    ImplDeclaration{!const}
    <#[attr]•'e>             ImplDeclaration.generics{dk: "<>"}
     #[attr]•'e              GenericLtParameterDeclaration
             'e              GenericLtParameterDeclaration~ownStart
     #[attr]                 Attribute{!inner}
      [attr]                 Attribute.segments{dk: "[]"}
             'e              LtIdentifier
                 A<'e>       TypeCall
                  <'e>       TypeCall.typeArguments{dk: "<>"}
                   'e        LtIdentifier
                       {}    ImplDeclaration.body{dk: "{}"}                                                                               */
impl<#[attr] M> A<M> {}                                                                                                                   /*
impl<#[attr]•M>•A<M>•{}    ImplDeclaration{!const}
    <#[attr]•M>            ImplDeclaration.generics{dk: "<>"}
     #[attr]•M             GenericTypeParameterDeclaration
             M             GenericTypeParameterDeclaration~ownStart
     #[attr]               Attribute{!inner}
      [attr]               Attribute.segments{dk: "[]"}
                A<M>       TypeCall
                 <M>       TypeCall.typeArguments{dk: "<>"}
                     {}    ImplDeclaration.body{dk: "{}"}                                                                                 */
impl<#[attr] 'f> T<'f> for A<'f> {}                                                                                                       /*
impl<#[attr]•'f>•T<'f>•for•A<'f>•{}    ImplDeclaration{!const}
    <#[attr]•'f>                       ImplDeclaration.generics{dk: "<>"}
     #[attr]•'f                        GenericLtParameterDeclaration
             'f                        GenericLtParameterDeclaration~ownStart
     #[attr]                           Attribute{!inner}
      [attr]                           Attribute.segments{dk: "[]"}
             'f                        LtIdentifier
                 T<'f>                 TypeCall
                  <'f>                 TypeCall.typeArguments{dk: "<>"}
                   'f                  LtIdentifier
                           A<'f>       TypeCall
                            <'f>       TypeCall.typeArguments{dk: "<>"}
                             'f        LtIdentifier
                                 {}    ImplDeclaration.body{dk: "{}"}                                                                     */
impl<#[attr] N> T<N> for A<N> {}                                                                                                          /*
impl<#[attr]•N>•T<N>•for•A<N>•{}    ImplDeclaration{!const}
    <#[attr]•N>                     ImplDeclaration.generics{dk: "<>"}
     #[attr]•N                      GenericTypeParameterDeclaration
             N                      GenericTypeParameterDeclaration~ownStart
     #[attr]                        Attribute{!inner}
      [attr]                        Attribute.segments{dk: "[]"}
                T<N>                TypeCall
                 <N>                TypeCall.typeArguments{dk: "<>"}
                         A<N>       TypeCall
                          <N>       TypeCall.typeArguments{dk: "<>"}
                              {}    ImplDeclaration.body{dk: "{}"}                                                                        */
#[attr] #[attr] macro_rules! m_e {}                                                                                                       /*
#[attr]•#[attr]•macro_rules!•m_e•{}    MacroRulesDeclaration
                macro_rules!•m_e•{}    MacroRulesDeclaration~ownStart
#[attr]                                Attribute{!inner}
 [attr]                                Attribute.segments{dk: "[]"}
        #[attr]                        Attribute{!inner}
         [attr]                        Attribute.segments{dk: "[]"}
                                 {}    MacroRulesDeclaration.rules{dk: "{}"}                                                              */
#[attr] macro_rules! m {}                                                                                                                 /*
#[attr]•macro_rules!•m•{}    MacroRulesDeclaration
        macro_rules!•m•{}    MacroRulesDeclaration~ownStart
#[attr]                      Attribute{!inner}
 [attr]                      Attribute.segments{dk: "[]"}
                       {}    MacroRulesDeclaration.rules{dk: "{}"}                                                                        */

#[attr] fn f() {}                                                                                                                         /*
#[attr]•fn•f()•{}    FunctionDeclaration
        fn•f()•{}    FunctionDeclaration~ownStart
#[attr]              Attribute{!inner}
 [attr]              Attribute.segments{dk: "[]"}
            ()       FunctionDeclaration.parameters{dk: "()"}
               {}    FunctionDeclaration.body{dk: "{}"}                                                                                   */
#[attr] fn f(#[a1] a: u8) { let f = |#[a2] W(x), #[a3] y| (); }                                                                           /*
#[attr]•fn•f(#[a1]•a:•u8)•{•let•f•=•|#[a2]•W(x),•#[a3]•y|•();•}    FunctionDeclaration
        fn•f(#[a1]•a:•u8)•{•let•f•=•|#[a2]•W(x),•#[a3]•y|•();•}    FunctionDeclaration~ownStart
#[attr]                                                            Attribute{!inner}
 [attr]                                                            Attribute.segments{dk: "[]"}
            (#[a1]•a:•u8)                                          FunctionDeclaration.parameters{dk: "()"}
             #[a1]•a:•u8                                           FunctionParameterDeclaration
                   a:•u8                                           FunctionParameterDeclaration~ownStart
             #[a1]                                                 Attribute{!inner}
              [a1]                                                 Attribute.segments{dk: "[]"}
                          {•let•f•=•|#[a2]•W(x),•#[a3]•y|•();•}    FunctionDeclaration.body{dk: "{}"}
                            let•f•=•|#[a2]•W(x),•#[a3]•y|•();      LetVariableDeclaration
                                    |#[a2]•W(x),•#[a3]•y|•()       ClosureFunctionExpression
                                    |#[a2]•W(x),•#[a3]•y|          ClosureFunctionExpression.parameters{dk: "||"}
                                     #[a2]•W(x)                    ClosureFunctionParameterDeclaration
                                           W(x)                    ClosureFunctionParameterDeclaration~ownStart
                                     #[a2]                         Attribute{!inner}
                                      [a2]                         Attribute.segments{dk: "[]"}
                                           W(x)                    TuplePattern
                                            (x)                    TuplePattern.items{dk: "()"}
                                                 #[a3]•y           ClosureFunctionParameterDeclaration
                                                       y           ClosureFunctionParameterDeclaration~ownStart
                                                 #[a3]             Attribute{!inner}
                                                  [a3]             Attribute.segments{dk: "[]"}
                                                          ()       TupleLiteral                                                           */
fn f<#[attr] 'g>() {}                                                                                                                     /*
fn•f<#[attr]•'g>()•{}    FunctionDeclaration
    <#[attr]•'g>         FunctionDeclaration.generics{dk: "<>"}
     #[attr]•'g          GenericLtParameterDeclaration
             'g          GenericLtParameterDeclaration~ownStart
     #[attr]             Attribute{!inner}
      [attr]             Attribute.segments{dk: "[]"}
             'g          LtIdentifier
                ()       FunctionDeclaration.parameters{dk: "()"}
                   {}    FunctionDeclaration.body{dk: "{}"}                                                                               */
fn f<'e, #[attr] 'g>() {}                                                                                                                 /*
fn•f<'e,•#[attr]•'g>()•{}    FunctionDeclaration
    <'e,•#[attr]•'g>         FunctionDeclaration.generics{dk: "<>"}
     'e                      GenericLtParameterDeclaration, LtIdentifier
         #[attr]•'g          GenericLtParameterDeclaration
                 'g          GenericLtParameterDeclaration~ownStart
         #[attr]             Attribute{!inner}
          [attr]             Attribute.segments{dk: "[]"}
                 'g          LtIdentifier
                    ()       FunctionDeclaration.parameters{dk: "()"}
                       {}    FunctionDeclaration.body{dk: "{}"}                                                                           */
fn f<#[attr] G>() {}                                                                                                                      /*
fn•f<#[attr]•G>()•{}    FunctionDeclaration
    <#[attr]•G>         FunctionDeclaration.generics{dk: "<>"}
     #[attr]•G          GenericTypeParameterDeclaration
             G          GenericTypeParameterDeclaration~ownStart
     #[attr]            Attribute{!inner}
      [attr]            Attribute.segments{dk: "[]"}
               ()       FunctionDeclaration.parameters{dk: "()"}
                  {}    FunctionDeclaration.body{dk: "{}"}                                                                                */
fn f<E, #[attr] G>() {}                                                                                                                   /*
fn•f<E,•#[attr]•G>()•{}    FunctionDeclaration
    <E,•#[attr]•G>         FunctionDeclaration.generics{dk: "<>"}
     E                     GenericTypeParameterDeclaration
        #[attr]•G          GenericTypeParameterDeclaration
                G          GenericTypeParameterDeclaration~ownStart
        #[attr]            Attribute{!inner}
         [attr]            Attribute.segments{dk: "[]"}
                  ()       FunctionDeclaration.parameters{dk: "()"}
                     {}    FunctionDeclaration.body{dk: "{}"}                                                                             */
fn f() where for<#[attr] 'i> X: for<#[attr] 'i> Y {}                                                                                      /*
fn•f()•where•for<#[attr]•'i>•X:•for<#[attr]•'i>•Y•{}    FunctionDeclaration
    ()                                                  FunctionDeclaration.parameters{dk: "()"}
       where•for<#[attr]•'i>•X:•for<#[attr]•'i>•Y       FunctionDeclaration.whereBounds{dk: "None"}
             for<#[attr]•'i>•X:•for<#[attr]•'i>•Y       WhereTypeBoundDeclaration
             for<#[attr]•'i>                            WhereTypeBoundDeclaration.ltParameters{dk: "<>"}
                 #[attr]•'i                             GenericLtParameterDeclaration
                         'i                             GenericLtParameterDeclaration~ownStart
                 #[attr]                                Attribute{!inner}
                  [attr]                                Attribute.segments{dk: "[]"}
                         'i                             LtIdentifier
                                for<#[attr]•'i>•Y       TypeTraitBound{!maybeConst, !optional}
                                for<#[attr]•'i>         TypeTraitBound.ltParameters{dk: "<>"}
                                    #[attr]•'i          GenericLtParameterDeclaration
                                            'i          GenericLtParameterDeclaration~ownStart
                                    #[attr]             Attribute{!inner}
                                     [attr]             Attribute.segments{dk: "[]"}
                                            'i          LtIdentifier
                                                  {}    FunctionDeclaration.body{dk: "{}"}                                                */
fn f(#[d(true)] a: i32, #[a2] b: i32, #[what = "how"] c: u32) {}                                                                          /*
fn•f(#[d(true)]•a:•i32,•#[a2]•b:•i32,•#[what•=•"how"]•c:•u32)•{}    FunctionDeclaration
    (#[d(true)]•a:•i32,•#[a2]•b:•i32,•#[what•=•"how"]•c:•u32)       FunctionDeclaration.parameters{dk: "()"}
     #[d(true)]•a:•i32                                              FunctionParameterDeclaration
                a:•i32                                              FunctionParameterDeclaration~ownStart
     #[d(true)]                                                     Attribute{!inner}
      [d(true)]                                                     Attribute.segments{dk: "[]"}
        (true)                                                      DelimGroup
         true                                                       Literal{kind: True}
                        #[a2]•b:•i32                                FunctionParameterDeclaration
                              b:•i32                                FunctionParameterDeclaration~ownStart
                        #[a2]                                       Attribute{!inner}
                         [a2]                                       Attribute.segments{dk: "[]"}
                                      #[what•=•"how"]•c:•u32        FunctionParameterDeclaration
                                                      c:•u32        FunctionParameterDeclaration~ownStart
                                      #[what•=•"how"]               Attribute{!inner}
                                       [what•=•"how"]               Attribute.segments{dk: "[]"}
                                             =                      PunctuationToken{tk: "="}
                                               "how"                Literal{kind: String}
                                                              {}    FunctionDeclaration.body{dk: "{}"}                                    */
fn f(#[a1] #[a2] a: i32, #[what = "how"] b: i32, #[e(true)] c: u32) {}                                                                    /*
fn•f(#[a1]•#[a2]•a:•i32,•#[what•=•"how"]•b:•i32,•#[e(true)]•c:•u32)•{}    FunctionDeclaration
    (#[a1]•#[a2]•a:•i32,•#[what•=•"how"]•b:•i32,•#[e(true)]•c:•u32)       FunctionDeclaration.parameters{dk: "()"}
     #[a1]•#[a2]•a:•i32                                                   FunctionParameterDeclaration
                 a:•i32                                                   FunctionParameterDeclaration~ownStart
     #[a1]                                                                Attribute{!inner}
      [a1]                                                                Attribute.segments{dk: "[]"}
           #[a2]                                                          Attribute{!inner}
            [a2]                                                          Attribute.segments{dk: "[]"}
                         #[what•=•"how"]•b:•i32                           FunctionParameterDeclaration
                                         b:•i32                           FunctionParameterDeclaration~ownStart
                         #[what•=•"how"]                                  Attribute{!inner}
                          [what•=•"how"]                                  Attribute.segments{dk: "[]"}
                                =                                         PunctuationToken{tk: "="}
                                  "how"                                   Literal{kind: String}
                                                 #[e(true)]•c:•u32        FunctionParameterDeclaration
                                                            c:•u32        FunctionParameterDeclaration~ownStart
                                                 #[e(true)]               Attribute{!inner}
                                                  [e(true)]               Attribute.segments{dk: "[]"}
                                                    (true)                DelimGroup
                                                     true                 Literal{kind: True}
                                                                    {}    FunctionDeclaration.body{dk: "{}"}                              */
fn b(#[cfg(x)]x: i32, y: i32) -> i32 {}                                                                                                   /*
fn•b(#[cfg(x)]x:•i32,•y:•i32)•->•i32•{}    FunctionDeclaration
    (#[cfg(x)]x:•i32,•y:•i32)              FunctionDeclaration.parameters{dk: "()"}
     #[cfg(x)]x:•i32                       FunctionParameterDeclaration
              x:•i32                       FunctionParameterDeclaration~ownStart
     #[cfg(x)]                             Attribute{!inner}
      [cfg(x)]                             Attribute.segments{dk: "[]"}
          (x)                              DelimGroup
                      y:•i32               FunctionParameterDeclaration
                                     {}    FunctionDeclaration.body{dk: "{}"}                                                             */
fn f( #[a1] #[a2] &self, #[a1] #[a2] a: i32, #[what = "how"] b: i32, #[f(true)] c: u32 ) {}                                               /*
fn•f(•#[a1]•#[a2]•&self,•#[a1]•#[a2]•a:•i32,•#[what•=•"how"]•b:•i32,•#[f(true)]•c:•u32•)•{}    FunctionDeclaration
    (•#[a1]•#[a2]•&self,•#[a1]•#[a2]•a:•i32,•#[what•=•"how"]•b:•i32,•#[f(true)]•c:•u32•)       FunctionDeclaration.parameters{dk: "()"}
      #[a1]•#[a2]•&self                                                                        FunctionSelfParameterDeclaration{ref, !mut}
                  &self                                                                        FunctionSelfParameterDeclaration~ownStart
      #[a1]                                                                                    Attribute{!inner}
       [a1]                                                                                    Attribute.segments{dk: "[]"}
            #[a2]                                                                              Attribute{!inner}
             [a2]                                                                              Attribute.segments{dk: "[]"}
                         #[a1]•#[a2]•a:•i32                                                    FunctionParameterDeclaration
                                     a:•i32                                                    FunctionParameterDeclaration~ownStart
                         #[a1]                                                                 Attribute{!inner}
                          [a1]                                                                 Attribute.segments{dk: "[]"}
                               #[a2]                                                           Attribute{!inner}
                                [a2]                                                           Attribute.segments{dk: "[]"}
                                             #[what•=•"how"]•b:•i32                            FunctionParameterDeclaration
                                                             b:•i32                            FunctionParameterDeclaration~ownStart
                                             #[what•=•"how"]                                   Attribute{!inner}
                                              [what•=•"how"]                                   Attribute.segments{dk: "[]"}
                                                    =                                          PunctuationToken{tk: "="}
                                                      "how"                                    Literal{kind: String}
                                                                     #[f(true)]•c:•u32         FunctionParameterDeclaration
                                                                                c:•u32         FunctionParameterDeclaration~ownStart
                                                                     #[f(true)]                Attribute{!inner}
                                                                      [f(true)]                Attribute.segments{dk: "[]"}
                                                                        (true)                 DelimGroup
                                                                         true                  Literal{kind: True}
                                                                                         {}    FunctionDeclaration.body{dk: "{}"}         */
fn c( #[cfg(foo)]&mut self,#[deny(C)] b: i32, ) {}                                                                                        /*
fn•c(•#[cfg(foo)]&mut•self,#[deny(C)]•b:•i32,•)•{}    FunctionDeclaration
    (•#[cfg(foo)]&mut•self,#[deny(C)]•b:•i32,•)       FunctionDeclaration.parameters{dk: "()"}
      #[cfg(foo)]&mut•self                            FunctionSelfParameterDeclaration{ref, mut}
                 &mut•self                            FunctionSelfParameterDeclaration~ownStart
      #[cfg(foo)]                                     Attribute{!inner}
       [cfg(foo)]                                     Attribute.segments{dk: "[]"}
           (foo)                                      DelimGroup
                           #[deny(C)]•b:•i32          FunctionParameterDeclaration
                                      b:•i32          FunctionParameterDeclaration~ownStart
                           #[deny(C)]                 Attribute{!inner}
                            [deny(C)]                 Attribute.segments{dk: "[]"}
                                 (C)                  DelimGroup
                                                {}    FunctionDeclaration.body{dk: "{}"}                                                  */

#[a = "q"] #[b = "q"] mod a {#![c = "q"]}                                                                                                 /*
#[a•=•"q"]•#[b•=•"q"]•mod•a•{#![c•=•"q"]}    ModuleDeclaration
                      mod•a•{#![c•=•"q"]}    ModuleDeclaration~ownStart
#[a•=•"q"]                                   Attribute{!inner}
 [a•=•"q"]                                   Attribute.segments{dk: "[]"}
    =                                        PunctuationToken{tk: "="}
      "q"                                    Literal{kind: String}
           #[b•=•"q"]                        Attribute{!inner}
            [b•=•"q"]                        Attribute.segments{dk: "[]"}
               =                             PunctuationToken{tk: "="}
                 "q"                         Literal{kind: String}
                            {#![c•=•"q"]}    ModuleDeclaration.body{dk: "{}"}
                             #![c•=•"q"]     Attribute{inner}
                               [c•=•"q"]     Attribute.segments{dk: "[]"}
                                  =          PunctuationToken{tk: "="}
                                    "q"      Literal{kind: String}                                                                        */
#[a = "q"] pub static X: u8 = ();                                                                                                         /*
#[a•=•"q"]•pub•static•X:•u8•=•();    StaticVariableDeclaration
           pub•static•X:•u8•=•();    StaticVariableDeclaration~ownStart
#[a•=•"q"]                           Attribute{!inner}
 [a•=•"q"]                           Attribute.segments{dk: "[]"}
    =                                PunctuationToken{tk: "="}
      "q"                            Literal{kind: String}
           pub                       PubSpecifier
                              ()     TupleLiteral                                                                                         */
#[a = "q"] pub fn f() {}                                                                                                                  /*
#[a•=•"q"]•pub•fn•f()•{}    FunctionDeclaration
           pub•fn•f()•{}    FunctionDeclaration~ownStart
#[a•=•"q"]                  Attribute{!inner}
 [a•=•"q"]                  Attribute.segments{dk: "[]"}
    =                       PunctuationToken{tk: "="}
      "q"                   Literal{kind: String}
           pub              PubSpecifier
                   ()       FunctionDeclaration.parameters{dk: "()"}
                      {}    FunctionDeclaration.body{dk: "{}"}                                                                            */
#[a = "q"] pub mod a {}                                                                                                                   /*
#[a•=•"q"]•pub•mod•a•{}    ModuleDeclaration
           pub•mod•a•{}    ModuleDeclaration~ownStart
#[a•=•"q"]                 Attribute{!inner}
 [a•=•"q"]                 Attribute.segments{dk: "[]"}
    =                      PunctuationToken{tk: "="}
      "q"                  Literal{kind: String}
           pub             PubSpecifier
                     {}    ModuleDeclaration.body{dk: "{}"}                                                                               */
#[a = "q"] extern "C" {}                                                                                                                  /*
#[a•=•"q"]•extern•"C"•{}    ExternBlockDeclaration
           extern•"C"•{}    ExternBlockDeclaration~ownStart
#[a•=•"q"]                  Attribute{!inner}
 [a•=•"q"]                  Attribute.segments{dk: "[]"}
    =                       PunctuationToken{tk: "="}
      "q"                   Literal{kind: String}
                  "C"       Literal{kind: String}
                      {}    ExternBlockDeclaration.body{dk: "{}"}                                                                         */

//(#[b(c(#(d = #e),*))]);
//(#[b(c(#(d•=•#e),*))]);    Comment{line}
a!(#[b(c(#(d = #e),*))]);                                                                                                                 /*
a!(#[b(c(#(d•=•#e),*))]);    ExpressionStatement{semi}
a!(#[b(c(#(d•=•#e),*))])     MacroInvocation
  (#[b(c(#(d•=•#e),*))])     MacroInvocation.segments{dk: "()"}
   #                         PunctuationToken{tk: "#"}
    [b(c(#(d•=•#e),*))]      DelimGroup
      (c(#(d•=•#e),*))       DelimGroup
        (#(d•=•#e),*)        DelimGroup
         #                   PunctuationToken{tk: "#"}
          (d•=•#e)           DelimGroup
             =               PunctuationToken{tk: "="}
               #             PunctuationToken{tk: "#"}
                  ,          PunctuationToken{tk: ","}
                   *         PunctuationToken{tk: "*"}                                                                                    */

//(#![b(c(#(d = #e),*))]);
//(#![b(c(#(d•=•#e),*))]);    Comment{line}
a!(#![b(c(#(d = #e),*))]);                                                                                                                /*
a!(#![b(c(#(d•=•#e),*))]);    ExpressionStatement{semi}
a!(#![b(c(#(d•=•#e),*))])     MacroInvocation
  (#![b(c(#(d•=•#e),*))])     MacroInvocation.segments{dk: "()"}
   #                          PunctuationToken{tk: "#"}
    !                         PunctuationToken{tk: "!"}
     [b(c(#(d•=•#e),*))]      DelimGroup
       (c(#(d•=•#e),*))       DelimGroup
         (#(d•=•#e),*)        DelimGroup
          #                   PunctuationToken{tk: "#"}
           (d•=•#e)           DelimGroup
              =               PunctuationToken{tk: "="}
                #             PunctuationToken{tk: "#"}
                   ,          PunctuationToken{tk: ","}
                    *         PunctuationToken{tk: "*"}                                                                                   */

#[attr] extern "C" { fn ffi(#[a1] a: i32, #[a2] ...); }                                                                                   /*
#[attr]•extern•"C"•{•fn•ffi(#[a1]•a:•i32,•#[a2]•...);•}    ExternBlockDeclaration
        extern•"C"•{•fn•ffi(#[a1]•a:•i32,•#[a2]•...);•}    ExternBlockDeclaration~ownStart
#[attr]                                                    Attribute{!inner}
 [attr]                                                    Attribute.segments{dk: "[]"}
               "C"                                         Literal{kind: String}
                   {•fn•ffi(#[a1]•a:•i32,•#[a2]•...);•}    ExternBlockDeclaration.body{dk: "{}"}
                     fn•ffi(#[a1]•a:•i32,•#[a2]•...);      FunctionDeclaration
                           (#[a1]•a:•i32,•#[a2]•...)       FunctionDeclaration.parameters{dk: "()"}
                            #[a1]•a:•i32                   FunctionParameterDeclaration
                                  a:•i32                   FunctionParameterDeclaration~ownStart
                            #[a1]                          Attribute{!inner}
                             [a1]                          Attribute.segments{dk: "[]"}
                                          #[a2]•...        FunctionSpread
                                                ...        FunctionSpread~ownStart
                                          #[a2]            Attribute{!inner}
                                           [a2]            Attribute.segments{dk: "[]"}                                                   */
#[attr] unsafe extern "C" fn f(a: i32, #[a1] mut args: ...) {}                                                                            /*
#[attr]•unsafe•extern•"C"•fn•f(a:•i32,•#[a1]•mut•args:•...)•{}    FunctionDeclaration{unsafe}
        unsafe•extern•"C"•fn•f(a:•i32,•#[a1]•mut•args:•...)•{}    FunctionDeclaration~ownStart
#[attr]                                                           Attribute{!inner}
 [attr]                                                           Attribute.segments{dk: "[]"}
               extern•"C"                                         ExternSpecifier
                      "C"                                         Literal{kind: String}
                              (a:•i32,•#[a1]•mut•args:•...)       FunctionDeclaration.parameters{dk: "()"}
                               a:•i32                             FunctionParameterDeclaration
                                       #[a1]•mut•args:•...        FunctionParameterDeclaration
                                             mut•args:•...        FunctionParameterDeclaration~ownStart
                                       #[a1]                      Attribute{!inner}
                                        [a1]                      Attribute.segments{dk: "[]"}
                                             mut•args             PatternVariableDeclaration{!ref, mut}
                                                       ...        FunctionSpread
                                                            {}    FunctionDeclaration.body{dk: "{}"}                                      */

impl W {                                                                                                                                  /*
impl•W•{↲    <ImplDeclaration{!const}>
       {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                             */
    #[attr] fn f(#[a1] self, #[a2] a: u8) {}                                                                                              /*
    #[attr]•fn•f(#[a1]•self,•#[a2]•a:•u8)•{}    FunctionDeclaration
            fn•f(#[a1]•self,•#[a2]•a:•u8)•{}    FunctionDeclaration~ownStart
    #[attr]                                     Attribute{!inner}
     [attr]                                     Attribute.segments{dk: "[]"}
                (#[a1]•self,•#[a2]•a:•u8)       FunctionDeclaration.parameters{dk: "()"}
                 #[a1]•self                     FunctionSelfParameterDeclaration{!ref, !mut}
                       self                     FunctionSelfParameterDeclaration~ownStart
                 #[a1]                          Attribute{!inner}
                  [a1]                          Attribute.segments{dk: "[]"}
                             #[a2]•a:•u8        FunctionParameterDeclaration
                                   a:•u8        FunctionParameterDeclaration~ownStart
                             #[a2]              Attribute{!inner}
                              [a2]              Attribute.segments{dk: "[]"}
                                          {}    FunctionDeclaration.body{dk: "{}"}                                                        */
    #[attr] fn f(#[a1] &self, #[a2] a: u8) {}                                                                                             /*
    #[attr]•fn•f(#[a1]•&self,•#[a2]•a:•u8)•{}    FunctionDeclaration
            fn•f(#[a1]•&self,•#[a2]•a:•u8)•{}    FunctionDeclaration~ownStart
    #[attr]                                      Attribute{!inner}
     [attr]                                      Attribute.segments{dk: "[]"}
                (#[a1]•&self,•#[a2]•a:•u8)       FunctionDeclaration.parameters{dk: "()"}
                 #[a1]•&self                     FunctionSelfParameterDeclaration{ref, !mut}
                       &self                     FunctionSelfParameterDeclaration~ownStart
                 #[a1]                           Attribute{!inner}
                  [a1]                           Attribute.segments{dk: "[]"}
                              #[a2]•a:•u8        FunctionParameterDeclaration
                                    a:•u8        FunctionParameterDeclaration~ownStart
                              #[a2]              Attribute{!inner}
                               [a2]              Attribute.segments{dk: "[]"}
                                           {}    FunctionDeclaration.body{dk: "{}"}                                                       */
    #[attr] fn f<'a>(#[a1] &'a mut self, #[a2] a: u8) {}                                                                                  /*
    #[attr]•fn•f<'a>(#[a1]•&'a•mut•self,•#[a2]•a:•u8)•{}    FunctionDeclaration
            fn•f<'a>(#[a1]•&'a•mut•self,•#[a2]•a:•u8)•{}    FunctionDeclaration~ownStart
    #[attr]                                                 Attribute{!inner}
     [attr]                                                 Attribute.segments{dk: "[]"}
                <'a>                                        FunctionDeclaration.generics{dk: "<>"}
                 'a                                         GenericLtParameterDeclaration, LtIdentifier
                    (#[a1]•&'a•mut•self,•#[a2]•a:•u8)       FunctionDeclaration.parameters{dk: "()"}
                     #[a1]•&'a•mut•self                     FunctionSelfParameterDeclaration{ref, mut}
                           &'a•mut•self                     FunctionSelfParameterDeclaration~ownStart
                     #[a1]                                  Attribute{!inner}
                      [a1]                                  Attribute.segments{dk: "[]"}
                            'a                              LtIdentifier
                                         #[a2]•a:•u8        FunctionParameterDeclaration
                                               a:•u8        FunctionParameterDeclaration~ownStart
                                         #[a2]              Attribute{!inner}
                                          [a2]              Attribute.segments{dk: "[]"}
                                                      {}    FunctionDeclaration.body{dk: "{}"}                                            */
    #[attr] fn f<'a>(#[a1] self: Box<Self>, #[a2] a: u8) {}                                                                               /*
    #[attr]•fn•f<'a>(#[a1]•self:•Box<Self>,•#[a2]•a:•u8)•{}    FunctionDeclaration
            fn•f<'a>(#[a1]•self:•Box<Self>,•#[a2]•a:•u8)•{}    FunctionDeclaration~ownStart
    #[attr]                                                    Attribute{!inner}
     [attr]                                                    Attribute.segments{dk: "[]"}
                <'a>                                           FunctionDeclaration.generics{dk: "<>"}
                 'a                                            GenericLtParameterDeclaration, LtIdentifier
                    (#[a1]•self:•Box<Self>,•#[a2]•a:•u8)       FunctionDeclaration.parameters{dk: "()"}
                     #[a1]•self:•Box<Self>                     FunctionSelfParameterDeclaration{!ref, !mut}
                           self:•Box<Self>                     FunctionSelfParameterDeclaration~ownStart
                     #[a1]                                     Attribute{!inner}
                      [a1]                                     Attribute.segments{dk: "[]"}
                                 Box<Self>                     TypeCall
                                    <Self>                     TypeCall.typeArguments{dk: "<>"}
                                            #[a2]•a:•u8        FunctionParameterDeclaration
                                                  a:•u8        FunctionParameterDeclaration~ownStart
                                            #[a2]              Attribute{!inner}
                                             [a2]              Attribute.segments{dk: "[]"}
                                                         {}    FunctionDeclaration.body{dk: "{}"}                                         */
    #[attr] fn f(#[a1] #[a2] a: u8, #[a3] b: u8) {}                                                                                       /*
    #[attr]•fn•f(#[a1]•#[a2]•a:•u8,•#[a3]•b:•u8)•{}    FunctionDeclaration
            fn•f(#[a1]•#[a2]•a:•u8,•#[a3]•b:•u8)•{}    FunctionDeclaration~ownStart
    #[attr]                                            Attribute{!inner}
     [attr]                                            Attribute.segments{dk: "[]"}
                (#[a1]•#[a2]•a:•u8,•#[a3]•b:•u8)       FunctionDeclaration.parameters{dk: "()"}
                 #[a1]•#[a2]•a:•u8                     FunctionParameterDeclaration
                             a:•u8                     FunctionParameterDeclaration~ownStart
                 #[a1]                                 Attribute{!inner}
                  [a1]                                 Attribute.segments{dk: "[]"}
                       #[a2]                           Attribute{!inner}
                        [a2]                           Attribute.segments{dk: "[]"}
                                    #[a3]•b:•u8        FunctionParameterDeclaration
                                          b:•u8        FunctionParameterDeclaration~ownStart
                                    #[a3]              Attribute{!inner}
                                     [a3]              Attribute.segments{dk: "[]"}
                                                 {}    FunctionDeclaration.body{dk: "{}"}                                                 */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */

trait A {                                                                                                                                 /*
trait•A•{↲    <TraitDeclaration>
        {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                           */
    #[attr] fn f(#[a1] self, #[a2] a: u8);                                                                                                /*
    #[attr]•fn•f(#[a1]•self,•#[a2]•a:•u8);    FunctionDeclaration
            fn•f(#[a1]•self,•#[a2]•a:•u8);    FunctionDeclaration~ownStart
    #[attr]                                   Attribute{!inner}
     [attr]                                   Attribute.segments{dk: "[]"}
                (#[a1]•self,•#[a2]•a:•u8)     FunctionDeclaration.parameters{dk: "()"}
                 #[a1]•self                   FunctionSelfParameterDeclaration{!ref, !mut}
                       self                   FunctionSelfParameterDeclaration~ownStart
                 #[a1]                        Attribute{!inner}
                  [a1]                        Attribute.segments{dk: "[]"}
                             #[a2]•a:•u8      FunctionParameterDeclaration
                                   a:•u8      FunctionParameterDeclaration~ownStart
                             #[a2]            Attribute{!inner}
                              [a2]            Attribute.segments{dk: "[]"}                                                                */
    #[attr] fn f(#[a1] &self, #[a2] a: u8);                                                                                               /*
    #[attr]•fn•f(#[a1]•&self,•#[a2]•a:•u8);    FunctionDeclaration
            fn•f(#[a1]•&self,•#[a2]•a:•u8);    FunctionDeclaration~ownStart
    #[attr]                                    Attribute{!inner}
     [attr]                                    Attribute.segments{dk: "[]"}
                (#[a1]•&self,•#[a2]•a:•u8)     FunctionDeclaration.parameters{dk: "()"}
                 #[a1]•&self                   FunctionSelfParameterDeclaration{ref, !mut}
                       &self                   FunctionSelfParameterDeclaration~ownStart
                 #[a1]                         Attribute{!inner}
                  [a1]                         Attribute.segments{dk: "[]"}
                              #[a2]•a:•u8      FunctionParameterDeclaration
                                    a:•u8      FunctionParameterDeclaration~ownStart
                              #[a2]            Attribute{!inner}
                               [a2]            Attribute.segments{dk: "[]"}                                                               */
    #[attr] fn f<'a>(#[a1] &'a mut self, #[a2] a: u8);                                                                                    /*
    #[attr]•fn•f<'a>(#[a1]•&'a•mut•self,•#[a2]•a:•u8);    FunctionDeclaration
            fn•f<'a>(#[a1]•&'a•mut•self,•#[a2]•a:•u8);    FunctionDeclaration~ownStart
    #[attr]                                               Attribute{!inner}
     [attr]                                               Attribute.segments{dk: "[]"}
                <'a>                                      FunctionDeclaration.generics{dk: "<>"}
                 'a                                       GenericLtParameterDeclaration, LtIdentifier
                    (#[a1]•&'a•mut•self,•#[a2]•a:•u8)     FunctionDeclaration.parameters{dk: "()"}
                     #[a1]•&'a•mut•self                   FunctionSelfParameterDeclaration{ref, mut}
                           &'a•mut•self                   FunctionSelfParameterDeclaration~ownStart
                     #[a1]                                Attribute{!inner}
                      [a1]                                Attribute.segments{dk: "[]"}
                            'a                            LtIdentifier
                                         #[a2]•a:•u8      FunctionParameterDeclaration
                                               a:•u8      FunctionParameterDeclaration~ownStart
                                         #[a2]            Attribute{!inner}
                                          [a2]            Attribute.segments{dk: "[]"}                                                    */
    #[attr] fn f<'a>(#[a1] self: Box<Self>, #[a2] a: u8, #[a3] b: Vec<u8>);                                                               /*
    #[attr]•fn•f<'a>(#[a1]•self:•Box<Self>,•#[a2]•a:•u8,•#[a3]•b:•Vec<u8>);    FunctionDeclaration
            fn•f<'a>(#[a1]•self:•Box<Self>,•#[a2]•a:•u8,•#[a3]•b:•Vec<u8>);    FunctionDeclaration~ownStart
    #[attr]                                                                    Attribute{!inner}
     [attr]                                                                    Attribute.segments{dk: "[]"}
                <'a>                                                           FunctionDeclaration.generics{dk: "<>"}
                 'a                                                            GenericLtParameterDeclaration, LtIdentifier
                    (#[a1]•self:•Box<Self>,•#[a2]•a:•u8,•#[a3]•b:•Vec<u8>)     FunctionDeclaration.parameters{dk: "()"}
                     #[a1]•self:•Box<Self>                                     FunctionSelfParameterDeclaration{!ref, !mut}
                           self:•Box<Self>                                     FunctionSelfParameterDeclaration~ownStart
                     #[a1]                                                     Attribute{!inner}
                      [a1]                                                     Attribute.segments{dk: "[]"}
                                 Box<Self>                                     TypeCall
                                    <Self>                                     TypeCall.typeArguments{dk: "<>"}
                                            #[a2]•a:•u8                        FunctionParameterDeclaration
                                                  a:•u8                        FunctionParameterDeclaration~ownStart
                                            #[a2]                              Attribute{!inner}
                                             [a2]                              Attribute.segments{dk: "[]"}
                                                         #[a3]•b:•Vec<u8>      FunctionParameterDeclaration
                                                               b:•Vec<u8>      FunctionParameterDeclaration~ownStart
                                                         #[a3]                 Attribute{!inner}
                                                          [a3]                 Attribute.segments{dk: "[]"}
                                                                  Vec<u8>      TypeCall
                                                                     <u8>      TypeCall.typeArguments{dk: "<>"}                           */
    #[attr] fn f(#[a1] #[a2] a: u8, #[a3] b: u8);                                                                                         /*
    #[attr]•fn•f(#[a1]•#[a2]•a:•u8,•#[a3]•b:•u8);    FunctionDeclaration
            fn•f(#[a1]•#[a2]•a:•u8,•#[a3]•b:•u8);    FunctionDeclaration~ownStart
    #[attr]                                          Attribute{!inner}
     [attr]                                          Attribute.segments{dk: "[]"}
                (#[a1]•#[a2]•a:•u8,•#[a3]•b:•u8)     FunctionDeclaration.parameters{dk: "()"}
                 #[a1]•#[a2]•a:•u8                   FunctionParameterDeclaration
                             a:•u8                   FunctionParameterDeclaration~ownStart
                 #[a1]                               Attribute{!inner}
                  [a1]                               Attribute.segments{dk: "[]"}
                       #[a2]                         Attribute{!inner}
                        [a2]                         Attribute.segments{dk: "[]"}
                                    #[a3]•b:•u8      FunctionParameterDeclaration
                                          b:•u8      FunctionParameterDeclaration~ownStart
                                    #[a3]            Attribute{!inner}
                                     [a3]            Attribute.segments{dk: "[]"}                                                         */
}                                                                                                                                         /*
}    </TraitDeclaration.body>
}    </TraitDeclaration>                                                                                                                  */



// directly above #![doc(...
//•directly•above•#![doc(...    Comment{line}
#![doc(___ = "____________________________________________________________",                                                              /*
#![doc(___•=•"____________________________________________________________",↲    <Attribute{inner}>
  [doc(___•=•"____________________________________________________________",↲    <Attribute.segments{dk: "[]"}>
      (___•=•"____________________________________________________________",↲    <DelimGroup>
           =                                                                     PunctuationToken{tk: "="}
             "____________________________________________________________"      Literal{kind: String}
                                                                           ,     PunctuationToken{tk: ","}                                */
       ___________________ = "_________________________________________________",                                                         /*
                           =                                                         PunctuationToken{tk: "="}
                             "_________________________________________________"     Literal{kind: String}
                                                                                ,    PunctuationToken{tk: ","}                            */
       ______________ = "_________________________________________________",                                                              /*
                      =                                                         PunctuationToken{tk: "="}
                        "_________________________________________________"     Literal{kind: String}
                                                                           ,    PunctuationToken{tk: ","}                                 */
       ___________________ = "_________________________________________________", a(b(c(d))))]                                            /*
                           =                                                                      PunctuationToken{tk: "="}
                             "_________________________________________________"                  Literal{kind: String}
                                                                                ,                 PunctuationToken{tk: ","}
                                                                                   (b(c(d)))      DelimGroup
                                                                                     (c(d))       DelimGroup
                                                                                       (d)        DelimGroup
•••••••___________________•=•"_________________________________________________",•a(b(c(d))))     </DelimGroup>
•••••••___________________•=•"_________________________________________________",•a(b(c(d))))]    </Attribute.segments>
•••••••___________________•=•"_________________________________________________",•a(b(c(d))))]    </Attribute>                            */

//! 1 line below #![doc(...
//!•1•line•below•#![doc(...    DocCommentAttribute{inner, line}

#![attr]                                                                                                                                  /*
#![attr]    Attribute{inner}
  [attr]    Attribute.segments{dk: "[]"}                                                                                                  */

//! aaa
//!•aaa    DocCommentAttribute{inner, line}

// bbb
//•bbb    Comment{line}

// ccc
//•ccc    Comment{line}
#![attr(b)]                                                                                                                               /*
#![attr(b)]    Attribute{inner}
  [attr(b)]    Attribute.segments{dk: "[]"}
       (b)     DelimGroup                                                                                                                 */

// ddd
//•ddd    Comment{line}

/// eee
///•eee↲    <ImplDeclaration{!const}>
///•eee     DocCommentAttribute{!inner, line}
/// eee
///•eee    DocCommentAttribute{!inner, line}
/// eee
///•eee    DocCommentAttribute{!inner, line}
/// eee
///•eee    DocCommentAttribute{!inner, line}

/// fff
///•fff    DocCommentAttribute{!inner, line}
impl Bar {                                                                                                                                /*
impl•Bar•{↲    <ImplDeclaration~ownStart>
         {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                           */
    /// 0
    ///•0↲    <FunctionDeclaration>
    ///•0     DocCommentAttribute{!inner, line}
    /// 1
    ///•1    DocCommentAttribute{!inner, line}
    /// 2
    ///•2    DocCommentAttribute{!inner, line}
    /// 3
    ///•3    DocCommentAttribute{!inner, line}
    #[attr]                                                                                                                               /*
    #[attr]    Attribute{!inner}
     [attr]    Attribute.segments{dk: "[]"}                                                                                               */
    #[doc = " ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ "]                                                              /*
    #[doc•=•"•___•___•___•___•___•___•___•___•___•___•___•___•___•___•___•"]    Attribute{!inner}
     [doc•=•"•___•___•___•___•___•___•___•___•___•___•___•___•___•___•___•"]    Attribute.segments{dk: "[]"}
          =                                                                     PunctuationToken{tk: "="}
            "•___•___•___•___•___•___•___•___•___•___•___•___•___•___•___•"     Literal{kind: String}                                     */
    fn foo(&mut self) -> isize {                                                                                                          /*
    fn•foo(&mut•self)•->•isize•{↲    <FunctionDeclaration~ownStart>
          (&mut•self)                FunctionDeclaration.parameters{dk: "()"}
           &mut•self                 FunctionSelfParameterDeclaration{ref, mut}
                               {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                 */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration.body>
••••}    </FunctionDeclaration>                                                                                                           */

    /// \n 4
    ///•\n•4↲    <FunctionDeclaration>
    ///•\n•4     DocCommentAttribute{!inner, line}
    /// 5
    ///•5    DocCommentAttribute{!inner, line}
    /// 6
    ///•6    DocCommentAttribute{!inner, line}


    /// \n\n 7
    ///•\n\n•7    DocCommentAttribute{!inner, line}
    /// 8
    ///•8    DocCommentAttribute{!inner, line}
    /// 9
    ///•9    DocCommentAttribute{!inner, line}
    pub fn f2(self) {                                                                                                                     /*
    pub•fn•f2(self)•{↲    <FunctionDeclaration~ownStart>
    pub                   PubSpecifier
             (self)       FunctionDeclaration.parameters{dk: "()"}
              self        FunctionSelfParameterDeclaration{!ref, !mut}
                    {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                            */
        (foo, bar)                                                                                                                        /*
        (foo,•bar)    ExpressionStatement{!semi}, TupleLiteral                                                                            */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration.body>
••••}    </FunctionDeclaration>                                                                                                           */

    #[attr]                                                                                                                               /*
    #[attr]↲    <FunctionDeclaration>
    #[attr]     Attribute{!inner}
     [attr]     Attribute.segments{dk: "[]"}                                                                                              */
    fn f3(self) -> Q {                                                                                                                    /*
    fn•f3(self)•->•Q•{↲    <FunctionDeclaration~ownStart>
         (self)            FunctionDeclaration.parameters{dk: "()"}
          self             FunctionSelfParameterDeclaration{!ref, !mut}
                     {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                           */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration.body>
••••}    </FunctionDeclaration>                                                                                                           */

    /// \n 10 \n
    ///•\n•10•\n↲    <FunctionDeclaration>
    ///•\n•10•\n     DocCommentAttribute{!inner, line}

    #[attrib1]                                                                                                                            /*
    #[attrib1]    Attribute{!inner}
     [attrib1]    Attribute.segments{dk: "[]"}                                                                                            */
    /// 11
    ///•11    DocCommentAttribute{!inner, line}
    #[attrib2]                                                                                                                            /*
    #[attrib2]    Attribute{!inner}
     [attrib2]    Attribute.segments{dk: "[]"}                                                                                            */
    // 12
    //•12    Comment{line}
    /// 13
    ///•13    DocCommentAttribute{!inner, line}
    fn f4(self) -> A {                                                                                                                    /*
    fn•f4(self)•->•A•{↲    <FunctionDeclaration~ownStart>
         (self)            FunctionDeclaration.parameters{dk: "()"}
          self             FunctionSelfParameterDeclaration{!ref, !mut}
                     {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                           */
    }                                                                                                                                     /*
••••}    </FunctionDeclaration.body>
••••}    </FunctionDeclaration>                                                                                                           */

    // \n 14
    //•\n•14    Comment{line}
    #[a(b="c")]                                                                                                                           /*
    #[a(b="c")]↲    <FunctionDeclaration>
    #[a(b="c")]     Attribute{!inner}
     [a(b="c")]     Attribute.segments{dk: "[]"}
       (b="c")      DelimGroup
         =          PunctuationToken{tk: "="}
          "c"       Literal{kind: String}                                                                                                 */
    fn f5(self) -> X {}                                                                                                                   /*
    fn•f5(self)•->•X•{}    FunctionDeclaration~ownStart
         (self)            FunctionDeclaration.parameters{dk: "()"}
          self             FunctionSelfParameterDeclaration{!ref, !mut}
                     {}    FunctionDeclaration.body{dk: "{}"}
••••fn•f5(self)•->•X•{}    </FunctionDeclaration>                                                                                         */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */

struct Foo {                                                                                                                              /*
struct•Foo•{↲    <StructDeclaration>
           {↲    <StructDeclaration.properties{dk: "{}"}>                                                                                 */
    # [ derive ( A , B , C , D , E ) ]                                                                                                    /*
    #•[•derive•(•A•,•B•,•C•,•D•,•E•)•]↲    <StructPropertyDeclaration>
    #•[•derive•(•A•,•B•,•C•,•D•,•E•)•]     Attribute{!inner}
      [•derive•(•A•,•B•,•C•,•D•,•E•)•]     Attribute.segments{dk: "[]"}
               (•A•,•B•,•C•,•D•,•E•)       DelimGroup
                   ,                       PunctuationToken{tk: ","}
                       ,                   PunctuationToken{tk: ","}
                           ,               PunctuationToken{tk: ","}
                               ,           PunctuationToken{tk: ","}                                                                      */
    foo: usize,                                                                                                                           /*
    foo:•usize    StructPropertyDeclaration~ownStart
••••foo:•usize    </StructPropertyDeclaration>                                                                                            */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */

// expected_{x} comments inside each attribute
//•expected_{x}•comments•inside•each•attribute    Comment{line}

#[should_panic]                                                                                                                           /*
#[should_panic]↲    <FunctionDeclaration>
#[should_panic]     Attribute{!inner}
 [should_panic]     Attribute.segments{dk: "[]"}                                                                                          */
#[should_panic(expected_0 = "(")]                                                                                                         /*
#[should_panic(expected_0•=•"(")]    Attribute{!inner}
 [should_panic(expected_0•=•"(")]    Attribute.segments{dk: "[]"}
              (expected_0•=•"(")     DelimGroup
                          =          PunctuationToken{tk: "="}
                            "("      Literal{kind: String}                                                                                */
#[should_panic(expected_1 = /* ( */ "(")]                                                                                                 /*
#[should_panic(expected_1•=•/*•(•*/•"(")]    Attribute{!inner}
 [should_panic(expected_1•=•/*•(•*/•"(")]    Attribute.segments{dk: "[]"}
              (expected_1•=•/*•(•*/•"(")     DelimGroup
                          =                  PunctuationToken{tk: "="}
                            /*•(•*/          Comment{!line}
                                    "("      Literal{kind: String}                                                                        */
#[should_panic(/* ((((( */expected_4 /* ((((( */= /* ((((( */ "("/* ((((( */)]                                                            /*
#[should_panic(/*•(((((•*/expected_4•/*•(((((•*/=•/*•(((((•*/•"("/*•(((((•*/)]    Attribute{!inner}
 [should_panic(/*•(((((•*/expected_4•/*•(((((•*/=•/*•(((((•*/•"("/*•(((((•*/)]    Attribute.segments{dk: "[]"}
              (/*•(((((•*/expected_4•/*•(((((•*/=•/*•(((((•*/•"("/*•(((((•*/)     DelimGroup
               /*•(((((•*/                                                        Comment{!line}
                                     /*•(((((•*/                                  Comment{!line}
                                                =                                 PunctuationToken{tk: "="}
                                                  /*•(((((•*/                     Comment{!line}
                                                              "("                 Literal{kind: String}
                                                                 /*•(((((•*/      Comment{!line}                                          */
#[should_panic(                                                                                                                           /*
#[should_panic(↲    <Attribute{!inner}>
 [should_panic(↲    <Attribute.segments{dk: "[]"}>
              (↲    <DelimGroup>                                                                                                          */
    /* (((((((( *//*                                                                                                                      /*
    /*•((((((((•*/       Comment{!line}
                  /*↲    <Comment{!line}>                                                                                               */*/
    (((((((((()(((((((( */                                                                                                              /*/*
••••(((((((((()((((((((•*/    </Comment>                                                                                                  */
    expected_3 = "("                                                                                                                      /*
               =        PunctuationToken{tk: "="}
                 "("    Literal{kind: String}                                                                                             */
    // ((((((((
    //•((((((((    Comment{line}
)]                                                                                                                                        /*
)     </DelimGroup>
)]    </Attribute.segments>
)]    </Attribute>                                                                                                                        */
fn f() {}                                                                                                                                 /*
fn•f()•{}    FunctionDeclaration~ownStart
    ()       FunctionDeclaration.parameters{dk: "()"}
       {}    FunctionDeclaration.body{dk: "{}"}
fn•f()•{}    </FunctionDeclaration>                                                                                                       */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    #[allow(unreachable_code)] // right of attr
/*  #[allow(unreachable_code)]•//•right•of•attr↲    <ExpressionStatement{semi}>
    #[allow(unreachable_code)]                      Attribute{!inner}
     [allow(unreachable_code)]                      Attribute.segments{dk: "[]"}
           (unreachable_code)                       DelimGroup                                                                            */
                               //•right•of•attr     Comment{line}
    Some( Err(error) ) ;                                                                                                                  /*
    Some(•Err(error)•)•;    ExpressionStatement~ownStart
    Some(•Err(error)•)      CallExpression
        (•Err(error)•)      CallExpression.arguments{dk: "()"}
          Err(error)        CallExpression
             (error)        CallExpression.arguments{dk: "()"}
••••Some(•Err(error)•)•;    </ExpressionStatement>                                                                                        */

    #[allow(unreachable_code)]                                                                                                            /*
    #[allow(unreachable_code)]↲    <ExpressionStatement{semi}>
    #[allow(unreachable_code)]     Attribute{!inner}
     [allow(unreachable_code)]     Attribute.segments{dk: "[]"}
           (unreachable_code)      DelimGroup                                                                                             */
    // below attr
    //•below•attr    Comment{line}
    Some( Err(error) ) ;                                                                                                                  /*
    Some(•Err(error)•)•;    ExpressionStatement~ownStart
    Some(•Err(error)•)      CallExpression
        (•Err(error)•)      CallExpression.arguments{dk: "()"}
          Err(error)        CallExpression
             (error)        CallExpression.arguments{dk: "()"}
••••Some(•Err(error)•)•;    </ExpressionStatement>                                                                                        */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    #![ this_is_an_inner_attribute ( foo ) ]                                                                                              /*
    #![•this_is_an_inner_attribute•(•foo•)•]    Attribute{inner}
      [•this_is_an_inner_attribute•(•foo•)•]    Attribute.segments{dk: "[]"}
                                   (•foo•)      DelimGroup                                                                                */

    foo();                                                                                                                                /*
    foo();    ExpressionStatement{semi}
    foo()     CallExpression
       ()     CallExpression.arguments{dk: "()"}                                                                                          */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

impl InnerAttributes() {                                                                                                                  /*
impl•InnerAttributes()•{↲    <ImplDeclaration{!const}>
     InnerAttributes()       TypeFunction
                    ()       TypeFunction.parameters{dk: "()"}
                       {↲    <ImplDeclaration.body{dk: "{}"}>                                                                             */
    #![ this_is_an_inner_attribute ( foo ) ]                                                                                              /*
    #![•this_is_an_inner_attribute•(•foo•)•]    Attribute{inner}
      [•this_is_an_inner_attribute•(•foo•)•]    Attribute.segments{dk: "[]"}
                                   (•foo•)      DelimGroup                                                                                */

    fn f() {}                                                                                                                             /*
    fn•f()•{}    FunctionDeclaration
        ()       FunctionDeclaration.parameters{dk: "()"}
           {}    FunctionDeclaration.body{dk: "{}"}                                                                                       */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */

mod InnerAttributes {                                                                                                                     /*
mod•InnerAttributes•{↲    <ModuleDeclaration>
                    {↲    <ModuleDeclaration.body{dk: "{}"}>                                                                              */
    #![ this_is_an_inner_attribute ( foo ) ]                                                                                              /*
    #![•this_is_an_inner_attribute•(•foo•)•]    Attribute{inner}
      [•this_is_an_inner_attribute•(•foo•)•]    Attribute.segments{dk: "[]"}
                                   (•foo•)      DelimGroup                                                                                */
}                                                                                                                                         /*
}    </ModuleDeclaration.body>
}    </ModuleDeclaration>                                                                                                                 */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    // Local
    //•Local    Comment{line}
    # [ attr ( on ( local ) ) ]                                                                                                           /*
    #•[•attr•(•on•(•local•)•)•]↲    <LetVariableDeclaration>
    #•[•attr•(•on•(•local•)•)•]     Attribute{!inner}
      [•attr•(•on•(•local•)•)•]     Attribute.segments{dk: "[]"}
             (•on•(•local•)•)       DelimGroup
                  (•local•)         DelimGroup                                                                                            */
    let x = 3;                                                                                                                            /*
    let•x•=•3;    LetVariableDeclaration~ownStart
            3     Literal{kind: Integer}
••••let•x•=•3;    </LetVariableDeclaration>                                                                                               */

    // Item
    //•Item    Comment{line}
    # [ attr ( on ( item ) ) ]                                                                                                            /*
    #•[•attr•(•on•(•item•)•)•]↲    <UseStatement>
    #•[•attr•(•on•(•item•)•)•]     Attribute{!inner}
      [•attr•(•on•(•item•)•)•]     Attribute.segments{dk: "[]"}
             (•on•(•item•)•)       DelimGroup
                  (•item•)         DelimGroup                                                                                             */
    use foo;                                                                                                                              /*
    use•foo;    UseStatement~ownStart
        foo     NamedImport
••••use•foo;    </UseStatement>                                                                                                           */

    // Expr
    //•Expr    Comment{line}
    # [ attr ( on ( expr ) ) ]                                                                                                            /*
    #•[•attr•(•on•(•expr•)•)•]↲    <ExpressionStatement{!semi}>
    #•[•attr•(•on•(•expr•)•)•]     Attribute{!inner}
      [•attr•(•on•(•expr•)•)•]     Attribute.segments{dk: "[]"}
             (•on•(•expr•)•)       DelimGroup
                  (•expr•)         DelimGroup                                                                                             */
    {}                                                                                                                                    /*
    {}    ExpressionStatement~ownStart
    {}    BlockExpression
••••{}    </ExpressionStatement>                                                                                                          */

    // Semi
    //•Semi    Comment{line}
    # [ attr ( on ( semi ) ) ]                                                                                                            /*
    #•[•attr•(•on•(•semi•)•)•]↲    <ExpressionStatement{semi}>
    #•[•attr•(•on•(•semi•)•)•]     Attribute{!inner}
      [•attr•(•on•(•semi•)•)•]     Attribute.segments{dk: "[]"}
             (•on•(•semi•)•)       DelimGroup
                  (•semi•)         DelimGroup                                                                                             */
    foo();                                                                                                                                /*
    foo();    ExpressionStatement~ownStart
    foo()     CallExpression
       ()     CallExpression.arguments{dk: "()"}
••••foo();    </ExpressionStatement>                                                                                                      */

    // Mac
    //•Mac    Comment{line}
    # [ attr ( on ( mac ) ) ]                                                                                                             /*
    #•[•attr•(•on•(•mac•)•)•]↲    <ExpressionStatement{semi}>
    #•[•attr•(•on•(•mac•)•)•]     Attribute{!inner}
      [•attr•(•on•(•mac•)•)•]     Attribute.segments{dk: "[]"}
             (•on•(•mac•)•)       DelimGroup
                  (•mac•)         DelimGroup                                                                                              */
    foo!();                                                                                                                               /*
    foo!();    ExpressionStatement~ownStart
    foo!()     MacroInvocation
        ()     MacroInvocation.segments{dk: "()"}
••••foo!();    </ExpressionStatement>                                                                                                     */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

#[derive(Add, Sub, Mul, Div, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Mul)]                                   /*
#[derive(Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Mul)]↲    <TupleStructDeclaration>
#[derive(Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Mul)]     Attribute{!inner}
 [derive(Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Mul)]     Attribute.segments{dk: "[]"}
        (Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Mul)      DelimGroup
            ,                                                                                               PunctuationToken{tk: ","}
                 ,                                                                                          PunctuationToken{tk: ","}
                      ,                                                                                     PunctuationToken{tk: ","}
                           ,                                                                                PunctuationToken{tk: ","}
                                  ,                                                                         PunctuationToken{tk: ","}
                                        ,                                                                   PunctuationToken{tk: ","}
                                            ,                                                               PunctuationToken{tk: ","}
                                                       ,                                                    PunctuationToken{tk: ","}
                                                            ,                                               PunctuationToken{tk: ","}
                                                                        ,                                   PunctuationToken{tk: ","}
                                                                               ,                            PunctuationToken{tk: ","}
                                                                                     ,                      PunctuationToken{tk: ","}
                                                                                                ,           PunctuationToken{tk: ","}     */


///
///    DocCommentAttribute{!inner, line}


#[derive(Add, Sub, Mul, Div, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]                           /*
#[derive(Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Deserialize)]    Attribute{!inner}
 [derive(Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Deserialize)]    Attribute.segments{dk: "[]"}
        (Add,•Sub,•Mul,•Div,•Clone,•Copy,•Eq,•PartialEq,•Ord,•PartialOrd,•Debug,•Hash,•Serialize,•Deserialize)     DelimGroup
            ,                                                                                                      PunctuationToken{tk: ","}
                 ,                                                                                                 PunctuationToken{tk: ","}
                      ,                                                                                            PunctuationToken{tk: ","}
                           ,                                                                                       PunctuationToken{tk: ","}
                                  ,                                                                                PunctuationToken{tk: ","}
                                        ,                                                                          PunctuationToken{tk: ","}
                                            ,                                                                      PunctuationToken{tk: ","}
                                                       ,                                                           PunctuationToken{tk: ","}
                                                            ,                                                      PunctuationToken{tk: ","}
                                                                        ,                                          PunctuationToken{tk: ","}
                                                                               ,                                   PunctuationToken{tk: ","}
                                                                                     ,                             PunctuationToken{tk: ","}
                                                                                                ,                  PunctuationToken{tk: ","}*/
pub struct HP(pub u8);                                                                                                                    /*
pub•struct•HP(pub•u8);    TupleStructDeclaration~ownStart
pub                       PubSpecifier
             (pub•u8)     TupleStructDeclaration.items{dk: "()"}
              pub•u8      TupleStructItemDeclaration
              pub         PubSpecifier
pub•struct•HP(pub•u8);    </TupleStructDeclaration>                                                                                       */

// 
//•    Comment{line}
struct A { #[doc = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"] b: i32 }                     /*
struct•A•{•#[doc•=•"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]•b:•i32•}    StructDeclaration
         {•#[doc•=•"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]•b:•i32•}    StructDeclaration.properties{dk: "{}"}
           #[doc•=•"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]•b:•i32      StructPropertyDeclaration
                                                                                                             b:•i32      StructPropertyDeclaration~ownStart
           #[doc•=•"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]             Attribute{!inner}
            [doc•=•"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]             Attribute.segments{dk: "[]"}
                 =                                                                                                       PunctuationToken{tk: "="}
                   "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"              Literal{kind: String}*/

#[cfg(feature = "this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")]                                     /*
#[cfg(feature•=•"this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")]↲    <FunctionDeclaration>
#[cfg(feature•=•"this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")]     Attribute{!inner}
 [cfg(feature•=•"this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")]     Attribute.segments{dk: "[]"}
     (feature•=•"this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")      DelimGroup
              =                                                                                           PunctuationToken{tk: "="}
                "this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"       Literal{kind: String}           */
pub fn foo() {}                                                                                                                           /*
pub•fn•foo()•{}    FunctionDeclaration~ownStart
pub                PubSpecifier
          ()       FunctionDeclaration.parameters{dk: "()"}
             {}    FunctionDeclaration.body{dk: "{}"}
pub•fn•foo()•{}    </FunctionDeclaration>                                                                                                 */

// 
//•    Comment{line}
#[clippy::bar]                                                                                                                            /*
#[clippy::bar]↲    <FunctionDeclaration>
#[clippy::bar]     Attribute{!inner}
 [clippy::bar]     Attribute.segments{dk: "[]"}
        ::         PunctuationToken{tk: "::"}                                                                                             */
#[clippy::bar(a, b, c)]                                                                                                                   /*
#[clippy::bar(a,•b,•c)]    Attribute{!inner}
 [clippy::bar(a,•b,•c)]    Attribute.segments{dk: "[]"}
        ::                 PunctuationToken{tk: "::"}
             (a,•b,•c)     DelimGroup
               ,           PunctuationToken{tk: ","}
                  ,        PunctuationToken{tk: ","}                                                                                      */
pub fn foo() {}                                                                                                                           /*
pub•fn•foo()•{}    FunctionDeclaration~ownStart
pub                PubSpecifier
          ()       FunctionDeclaration.parameters{dk: "()"}
             {}    FunctionDeclaration.body{dk: "{}"}
pub•fn•foo()•{}    </FunctionDeclaration>                                                                                                 */

mod v {                                                                                                                                   /*
mod•v•{↲    <ModuleDeclaration>
      {↲    <ModuleDeclaration.body{dk: "{}"}>                                                                                            */
    #[derive(Debug, StructOpt)]                                                                                                           /*
    #[derive(Debug,•StructOpt)]↲    <StructDeclaration>
    #[derive(Debug,•StructOpt)]     Attribute{!inner}
     [derive(Debug,•StructOpt)]     Attribute.segments{dk: "[]"}
            (Debug,•StructOpt)      DelimGroup
                  ,                 PunctuationToken{tk: ","}                                                                             */
    #[structopt(about = "x")]                                                                                                             /*
    #[structopt(about•=•"x")]    Attribute{!inner}
     [structopt(about•=•"x")]    Attribute.segments{dk: "[]"}
               (about•=•"x")     DelimGroup
                      =          PunctuationToken{tk: "="}
                        "x"      Literal{kind: String}                                                                                    */
    pub struct Params {                                                                                                                   /*
    pub•struct•Params•{↲    <StructDeclaration~ownStart>
    pub                     PubSpecifier
                      {↲    <StructDeclaration.properties{dk: "{}"}>                                                                      */
        #[structopt(help = "x")]                                                                                                          /*
        #[structopt(help•=•"x")]↲    <StructPropertyDeclaration>
        #[structopt(help•=•"x")]     Attribute{!inner}
         [structopt(help•=•"x")]     Attribute.segments{dk: "[]"}
                   (help•=•"x")      DelimGroup
                         =           PunctuationToken{tk: "="}
                           "x"       Literal{kind: String}                                                                                */
        server: String,                                                                                                                   /*
        server:•String    StructPropertyDeclaration~ownStart
••••••••server:•String    </StructPropertyDeclaration>                                                                                    */
        #[structopt(help = "x")]                                                                                                          /*
        #[structopt(help•=•"x")]↲    <StructPropertyDeclaration>
        #[structopt(help•=•"x")]     Attribute{!inner}
         [structopt(help•=•"x")]     Attribute.segments{dk: "[]"}
                   (help•=•"x")      DelimGroup
                         =           PunctuationToken{tk: "="}
                           "x"       Literal{kind: String}                                                                                */
        first_name: String,                                                                                                               /*
        first_name:•String    StructPropertyDeclaration~ownStart
••••••••first_name:•String    </StructPropertyDeclaration>                                                                                */
        #[structopt(help = "x")]                                                                                                          /*
        #[structopt(help•=•"x")]↲    <StructPropertyDeclaration>
        #[structopt(help•=•"x")]     Attribute{!inner}
         [structopt(help•=•"x")]     Attribute.segments{dk: "[]"}
                   (help•=•"x")      DelimGroup
                         =           PunctuationToken{tk: "="}
                           "x"       Literal{kind: String}                                                                                */
        last_name: String,                                                                                                                /*
        last_name:•String    StructPropertyDeclaration~ownStart
••••••••last_name:•String    </StructPropertyDeclaration>                                                                                 */
        #[structopt(                                                                                                                      /*
        #[structopt(↲    <StructPropertyDeclaration>
        #[structopt(↲    <Attribute{!inner}>
         [structopt(↲    <Attribute.segments{dk: "[]"}>
                   (↲    <DelimGroup>                                                                                                     */
            short = "j",                                                                                                                  /*
                  =         PunctuationToken{tk: "="}
                    "j"     Literal{kind: String}
                       ,    PunctuationToken{tk: ","}                                                                                     */
            long = "job",                                                                                                                 /*
                 =           PunctuationToken{tk: "="}
                   "job"     Literal{kind: String}
                        ,    PunctuationToken{tk: ","}                                                                                    */
            help = "The job to look at",                                                                                                  /*
                 =                          PunctuationToken{tk: "="}
                   "The•job•to•look•at"     Literal{kind: String}
                                       ,    PunctuationToken{tk: ","}                                                                     */
            parse(try_from_str)                                                                                                           /*
                 (try_from_str)    DelimGroup                                                                                             */
        )]                                                                                                                                /*
••••••••)     </DelimGroup>
••••••••)]    </Attribute.segments>
••••••••)]    </Attribute>                                                                                                                */
        job: Option<Job>                                                                                                                  /*
        job:•Option<Job>    StructPropertyDeclaration~ownStart
             Option<Job>    TypeCall
                   <Job>    TypeCall.typeArguments{dk: "<>"}
••••••••job:•Option<Job>    </StructPropertyDeclaration>                                                                                  */
    }                                                                                                                                     /*
••••}    </StructDeclaration.properties>
••••}    </StructDeclaration>                                                                                                             */
}                                                                                                                                         /*
}    </ModuleDeclaration.body>
}    </ModuleDeclaration>                                                                                                                 */

#[cfg(not(all(feature="std",                                                                                                              /*
#[cfg(not(all(feature="std",↲    <TypeAliasDeclaration>
#[cfg(not(all(feature="std",↲    <Attribute{!inner}>
 [cfg(not(all(feature="std",↲    <Attribute.segments{dk: "[]"}>
     (not(all(feature="std",↲    <DelimGroup>
         (all(feature="std",↲    <DelimGroup>
             (feature="std",↲    <DelimGroup>
                     =           PunctuationToken{tk: "="}
                      "std"      Literal{kind: String}
                           ,     PunctuationToken{tk: ","}                                                                                */
              any(target_os = "linux", target_os = "android",                                                                             /*
                 (target_os•=•"linux",•target_os•=•"android",↲    <DelimGroup>
                            =                                     PunctuationToken{tk: "="}
                              "linux"                             Literal{kind: String}
                                     ,                            PunctuationToken{tk: ","}
                                                 =                PunctuationToken{tk: "="}
                                                   "android"      Literal{kind: String}
                                                            ,     PunctuationToken{tk: ","}                                               */
                  target_os = "netbsd",                                                                                                   /*
                            =              PunctuationToken{tk: "="}
                              "netbsd"     Literal{kind: String}
                                      ,    PunctuationToken{tk: ","}                                                                      */
                  target_os = "dragonfly",                                                                                                /*
                            =                 PunctuationToken{tk: "="}
                              "dragonfly"     Literal{kind: String}
                                         ,    PunctuationToken{tk: ","}                                                                   */
                  target_os = "haiku",                                                                                                    /*
                            =             PunctuationToken{tk: "="}
                              "haiku"     Literal{kind: String}
                                     ,    PunctuationToken{tk: ","}                                                                       */
                  target_os = "emscripten",                                                                                               /*
                            =                  PunctuationToken{tk: "="}
                              "emscripten"     Literal{kind: String}
                                          ,    PunctuationToken{tk: ","}                                                                  */
                  target_os = "solaris",                                                                                                  /*
                            =               PunctuationToken{tk: "="}
                              "solaris"     Literal{kind: String}
                                       ,    PunctuationToken{tk: ","}                                                                     */
                  target_os = "cloudabi",                                                                                                 /*
                            =                PunctuationToken{tk: "="}
                              "cloudabi"     Literal{kind: String}
                                        ,    PunctuationToken{tk: ","}                                                                    */
                  target_os = "macos", target_os = "ios",                                                                                 /*
                            =                                PunctuationToken{tk: "="}
                              "macos"                        Literal{kind: String}
                                     ,                       PunctuationToken{tk: ","}
                                                 =           PunctuationToken{tk: "="}
                                                   "ios"     Literal{kind: String}
                                                        ,    PunctuationToken{tk: ","}                                                    */
                  target_os = "freebsd",                                                                                                  /*
                            =               PunctuationToken{tk: "="}
                              "freebsd"     Literal{kind: String}
                                       ,    PunctuationToken{tk: ","}                                                                     */
                  target_os = "openbsd",                                                                                                  /*
                            =               PunctuationToken{tk: "="}
                              "openbsd"     Literal{kind: String}
                                       ,    PunctuationToken{tk: ","}                                                                     */
                  target_os = "redox",                                                                                                    /*
                            =             PunctuationToken{tk: "="}
                              "redox"     Literal{kind: String}
                                     ,    PunctuationToken{tk: ","}                                                                       */
                  target_os = "fuchsia",                                                                                                  /*
                            =               PunctuationToken{tk: "="}
                              "fuchsia"     Literal{kind: String}
                                       ,    PunctuationToken{tk: ","}                                                                     */
                  windows,                                                                                                                /*
                         ,    PunctuationToken{tk: ","}                                                                                   */
                  all(target_arch = "wasm32", feature = "stdweb"),                                                                        /*
                     (target_arch•=•"wasm32",•feature•=•"stdweb")     DelimGroup
                                  =                                   PunctuationToken{tk: "="}
                                    "wasm32"                          Literal{kind: String}
                                            ,                         PunctuationToken{tk: ","}
                                                      =               PunctuationToken{tk: "="}
                                                        "stdweb"      Literal{kind: String}
                                                                 ,    PunctuationToken{tk: ","}                                           */
                  all(target_arch = "wasm32", feature = "wasm-bindgen"),                                                                  /*
                     (target_arch•=•"wasm32",•feature•=•"wasm-bindgen")     DelimGroup
                                  =                                         PunctuationToken{tk: "="}
                                    "wasm32"                                Literal{kind: String}
                                            ,                               PunctuationToken{tk: ","}
                                                      =                     PunctuationToken{tk: "="}
                                                        "wasm-bindgen"      Literal{kind: String}
                                                                       ,    PunctuationToken{tk: ","}                                     */
              ))))]                                                                                                                       /*
••••••••••••••)        </DelimGroup>
••••••••••••••))       </DelimGroup>
••••••••••••••)))      </DelimGroup>
••••••••••••••))))     </DelimGroup>
••••••••••••••))))]    </Attribute.segments>
••••••••••••••))))]    </Attribute>                                                                                                       */
type Os = NoSource;                                                                                                                       /*
type•Os•=•NoSource;    TypeAliasDeclaration~ownStart
type•Os•=•NoSource;    </TypeAliasDeclaration>                                                                                            */

fn stmt_expr_attributes() {                                                                                                               /*
fn•stmt_expr_attributes()•{↲    <FunctionDeclaration>
                       ()       FunctionDeclaration.parameters{dk: "()"}
                          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                      */
    let foo ;                                                                                                                             /*
    let•foo•;    LetVariableDeclaration                                                                                                   */
    #[must_use]                                                                                                                           /*
    #[must_use]↲    <ExpressionStatement{semi}>
    #[must_use]     Attribute{!inner}
     [must_use]     Attribute.segments{dk: "[]"}                                                                                          */
   foo = false ;                                                                                                                          /*
   foo•=•false•;    ExpressionStatement~ownStart
   foo•=•false      ReassignmentExpression{tk: "="}
         false      Literal{kind: False}
•••foo•=•false•;    </ExpressionStatement>                                                                                                */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

fn a() {                                                                                                                                  /*
fn•a()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    match () {                                                                                                                            /*
    match•()•{↲    <ExpressionStatement{!semi}>
    match•()•{↲    <MatchExpression>
          ()       TupleLiteral
             {↲    <MatchExpression.cases{dk: "{}"}>                                                                                      */
        #![attr]                                                                                                                          /*
        #![attr]    Attribute{inner}
          [attr]    Attribute.segments{dk: "[]"}                                                                                          */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
    match () {                                                                                                                            /*
    match•()•{↲    <ExpressionStatement{!semi}>
    match•()•{↲    <MatchExpression>
          ()       TupleLiteral
             {↲    <MatchExpression.cases{dk: "{}"}>                                                                                      */
        #[attr]                                                                                                                           /*
        #[attr]    Attribute#DANGLING{!inner}
         [attr]    Attribute#DANGLING.segments{dk: "[]"}                                                                                  */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

fn x() {                                                                                                                                  /*
fn•x()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    match MyEnum {                                                                                                                        /*
    match•MyEnum•{↲    <ExpressionStatement{!semi}>
    match•MyEnum•{↲    <MatchExpression>
                 {↲    <MatchExpression.cases{dk: "{}"}>                                                                                  */

    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

// 2 comments inside this attribute
//•2•comments•inside•this•attribute    Comment{line}
#[derive(                                                                                                                                 /*
#[derive(↲    <StructDeclaration>
#[derive(↲    <Attribute{!inner}>
 [derive(↲    <Attribute.segments{dk: "[]"}>
        (↲    <DelimGroup>                                                                                                                */
/* ---------- ------------------------------------------------------------------- --------- */                                            /*
/*•----------•-------------------------------------------------------------------•---------•*/    Comment{!line}                          */
Debug, Clone,/* --------------- */Eq, PartialEq,                                                                                          /*
     ,                                              PunctuationToken{tk: ","}
            ,                                       PunctuationToken{tk: ","}
             /*•---------------•*/                  Comment{!line}
                                    ,               PunctuationToken{tk: ","}
                                               ,    PunctuationToken{tk: ","}                                                             */
)]                                                                                                                                        /*
)     </DelimGroup>
)]    </Attribute.segments>
)]    </Attribute>                                                                                                                        */
struct Foo {                                                                                                                              /*
struct•Foo•{↲    <StructDeclaration~ownStart>
           {↲    <StructDeclaration.properties{dk: "{}"}>                                                                                 */
    a: i32,                                                                                                                               /*
    a:•i32    StructPropertyDeclaration                                                                                                   */
    b: T,                                                                                                                                 /*
    b:•T    StructPropertyDeclaration                                                                                                     */
}                                                                                                                                         /*
}    </StructDeclaration.properties>
}    </StructDeclaration>                                                                                                                 */

// 1 comment inside this attribute
//•1•comment•inside•this•attribute    Comment{line}
#[derive(/*Debug, */Clone)]                                                                                                               /*
#[derive(/*Debug,•*/Clone)]↲    <StructDeclaration>
#[derive(/*Debug,•*/Clone)]     Attribute{!inner}
 [derive(/*Debug,•*/Clone)]     Attribute.segments{dk: "[]"}
        (/*Debug,•*/Clone)      DelimGroup
         /*Debug,•*/            Comment{!line}                                                                                            */
struct Foo;                                                                                                                               /*
struct•Foo;    StructDeclaration~ownStart
struct•Foo;    </StructDeclaration>                                                                                                       */

#[cfg(all(any(                                                                                                                            /*
#[cfg(all(any(↲    <ConstVariableDeclaration>
#[cfg(all(any(↲    <Attribute{!inner}>
 [cfg(all(any(↲    <Attribute.segments{dk: "[]"}>
     (all(any(↲    <DelimGroup>
         (any(↲    <DelimGroup>
             (↲    <DelimGroup>                                                                                                           */
    target_arch = "x86",                                                                                                                  /*
                =           PunctuationToken{tk: "="}
                  "x86"     Literal{kind: String}
                       ,    PunctuationToken{tk: ","}                                                                                     */
    target_arch = "x86_64",                                                                                                               /*
                =              PunctuationToken{tk: "="}
                  "x86_64"     Literal{kind: String}
                          ,    PunctuationToken{tk: ","}                                                                                  */
    target_arch = "aarch64",                                                                                                              /*
                =               PunctuationToken{tk: "="}
                  "aarch64"     Literal{kind: String}
                           ,    PunctuationToken{tk: ","}                                                                                 */
    target_arch = "powerpc64",                                                                                                            /*
                =                 PunctuationToken{tk: "="}
                  "powerpc64"     Literal{kind: String}
                             ,    PunctuationToken{tk: ","}                                                                               */
    target_arch = "powerpc64le",                                                                                                          /*
                =                   PunctuationToken{tk: "="}
                  "powerpc64le"     Literal{kind: String}
                               ,    PunctuationToken{tk: ","}                                                                             */
    target_arch = "mips64",                                                                                                               /*
                =              PunctuationToken{tk: "="}
                  "mips64"     Literal{kind: String}
                          ,    PunctuationToken{tk: ","}                                                                                  */
    target_arch = "s390x",                                                                                                                /*
                =             PunctuationToken{tk: "="}
                  "s390x"     Literal{kind: String}
                         ,    PunctuationToken{tk: ","}                                                                                   */
    target_arch = "sparc64"                                                                                                               /*
                =              PunctuationToken{tk: "="}
                  "sparc64"    Literal{kind: String}                                                                                      */
)))]                                                                                                                                      /*
)       </DelimGroup>
))      </DelimGroup>
)))     </DelimGroup>
)))]    </Attribute.segments>
)))]    </Attribute>                                                                                                                      */
const MIN_ALIGN: usize = 16;                                                                                                              /*
const•MIN_ALIGN:•usize•=•16;    ConstVariableDeclaration~ownStart
                         16     Literal{kind: Integer}
const•MIN_ALIGN:•usize•=•16;    </ConstVariableDeclaration>
const•MIN_ALIGN:•usize•=•16;    </Program.ast>
const•MIN_ALIGN:•usize•=•16;    </Program>                                                                                                */
// Discarded Nodes: 21
// Parsed Nodes: 2181
// state_rollbacks: 45
// Total '.charCodeAt()' calls: 14021 (24% re-reads)
// Unnecessary 'skip_whitespace()' calls: 951
// source: "../../samples/macro/attributes.rs"