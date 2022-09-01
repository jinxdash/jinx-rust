fn a() {                                                                                                                                  /*
fn•a()•{↲    <Program>
fn•a()•{↲    <Program.ast{dk: "None"}>
fn•a()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	a::<!>();                                                                                                                             /*
	a::<!>();    ExpressionStatement{semi}
	a::<!>()     CallExpression
	   <!>       CallExpression.typeArguments{dk: "<>"}
	    !        TypeNever
	      ()     CallExpression.arguments{dk: "()"}                                                                                       */
	let x: ! = a!();                                                                                                                      /*
	let•x:•!•=•a!();    LetVariableDeclaration
	       !            TypeNever
	           a!()     MacroInvocation
	             ()     MacroInvocation.segments{dk: "()"}                                                                                */
	let x: ! = unsafe { a::<B, !>(C) };                                                                                                   /*
	let•x:•!•=•unsafe•{•a::<B,•!>(C)•};    LetVariableDeclaration
	       !                               TypeNever
	           unsafe•{•a::<B,•!>(C)•}     BlockExpression{unsafe}
	                  {•a::<B,•!>(C)•}     BlockExpression.body{dk: "{}"}
	                    a::<B,•!>(C)       ExpressionStatement{!semi}, CallExpression
	                       <B,•!>          CallExpression.typeArguments{dk: "<>"}
	                           !           TypeNever
	                             (C)       CallExpression.arguments{dk: "()"}                                                             */
	<E as From<!>>::from(never);                                                                                                          /*
	<E•as•From<!>>::from(never);    ExpressionStatement{semi}
	<E•as•From<!>>::from(never)     CallExpression
	<E•as•From<!>>::from            ExpressionPath
	<E•as•From<!>>                  ExpressionTypeSelector
	      From<!>                   TypeCall
	          <!>                   TypeCall.typeArguments{dk: "<>"}
	           !                    TypeNever
	                    (never)     CallExpression.arguments{dk: "()"}                                                                    */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
fn a(x: !) -> ! { x }                                                                                                                     /*
fn•a(x:•!)•->•!•{•x•}    FunctionDeclaration
    (x:•!)               FunctionDeclaration.parameters{dk: "()"}
     x:•!                FunctionParameterDeclaration
        !                TypeNever
              !          TypeNever
                {•x•}    FunctionDeclaration.body{dk: "{}"}
                  x      ExpressionStatement{!semi}                                                                                       */
fn foo(never: !) {}                                                                                                                       /*
fn•foo(never:•!)•{}    FunctionDeclaration
      (never:•!)       FunctionDeclaration.parameters{dk: "()"}
       never:•!        FunctionParameterDeclaration
              !        TypeNever
                 {}    FunctionDeclaration.body{dk: "{}"}                                                                                 */
fn a(x: !) {}                                                                                                                             /*
fn•a(x:•!)•{}    FunctionDeclaration
    (x:•!)       FunctionDeclaration.parameters{dk: "()"}
     x:•!        FunctionParameterDeclaration
        !        TypeNever
           {}    FunctionDeclaration.body{dk: "{}"}                                                                                       */
fn a(ref x: !) {}                                                                                                                         /*
fn•a(ref•x:•!)•{}    FunctionDeclaration
    (ref•x:•!)       FunctionDeclaration.parameters{dk: "()"}
     ref•x:•!        FunctionParameterDeclaration
     ref•x           PatternVariableDeclaration{ref, !mut}
            !        TypeNever
               {}    FunctionDeclaration.body{dk: "{}"}                                                                                   */
fn a(x: &[!]) {}                                                                                                                          /*
fn•a(x:•&[!])•{}    FunctionDeclaration
    (x:•&[!])       FunctionDeclaration.parameters{dk: "()"}
     x:•&[!]        FunctionParameterDeclaration
        &[!]        TypeReference{!mut}
         [!]        TypeSlice
          !         TypeNever
              {}    FunctionDeclaration.body{dk: "{}"}                                                                                    */
fn a(x: B<(), !>) {}                                                                                                                      /*
fn•a(x:•B<(),•!>)•{}    FunctionDeclaration
    (x:•B<(),•!>)       FunctionDeclaration.parameters{dk: "()"}
     x:•B<(),•!>        FunctionParameterDeclaration
        B<(),•!>        TypeCall
         <(),•!>        TypeCall.typeArguments{dk: "<>"}
          ()            TypeTuple
              !         TypeNever
                  {}    FunctionDeclaration.body{dk: "{}"}                                                                                */

impl A<!> for B {                                                                                                                         /*
impl•A<!>•for•B•{↲    <ImplDeclaration{!const}>
     A<!>             TypeCall
      <!>             TypeCall.typeArguments{dk: "<>"}
       !              TypeNever
                {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                    */
    fn c(&self, d: &!) -> E {}                                                                                                            /*
    fn•c(&self,•d:•&!)•->•E•{}    FunctionDeclaration
        (&self,•d:•&!)            FunctionDeclaration.parameters{dk: "()"}
         &self                    FunctionSelfParameterDeclaration{ref, !mut}
                d:•&!             FunctionParameterDeclaration
                   &!             TypeReference{!mut}
                    !             TypeNever
                            {}    FunctionDeclaration.body{dk: "{}"}                                                                      */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */
impl A for ! {}                                                                                                                           /*
impl•A•for•!•{}    ImplDeclaration{!const}
           !       TypeNever
             {}    ImplDeclaration.body{dk: "{}"}                                                                                         */
type A = !;                                                                                                                               /*
type•A•=•!;    TypeAliasDeclaration
         !     TypeNever
type•A•=•!;    </Program.ast>
type•A•=•!;    </Program>                                                                                                                 */
// Discarded Nodes: 0
// Parsed Nodes: 91
// state_rollbacks: 8
// Total '.charCodeAt()' calls: 439 (47% re-reads)
// Unnecessary 'skip_whitespace()' calls: 67
// source: "../../samples/types/never.rs"