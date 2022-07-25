fn a() {                                                                                                                                  /*
fn•a()•{↲    <FunctionDeclaration>                                                                                                        */
	a::<!>();                                                                                                                             /*
    a::<!>();    ExpressionStatement
    a::<!>()     CallExpression
        !        TypeNever                                                                                                                */
	let x: ! = a!();                                                                                                                      /*
    let•x:•!•=•a!();    LetVariableDeclaration
           !            TypeNever
               a!()     MacroInvocation                                                                                                   */
	let x: ! = unsafe { a::<B, !>(C) };                                                                                                   /*
    let•x:•!•=•unsafe•{•a::<B,•!>(C)•};    LetVariableDeclaration
           !                               TypeNever
               unsafe•{•a::<B,•!>(C)•}     BlockExpression
                        a::<B,•!>(C)       ExpressionStatement, CallExpression
                               !           TypeNever                                                                                      */
	<E as From<!>>::from(never);                                                                                                          /*
    <E•as•From<!>>::from(never);    ExpressionStatement
    <E•as•From<!>>::from(never)     CallExpression
    <E•as•From<!>>::from            ExpressionPath
    <E•as•From<!>>                  ExpressionTypeSelector
          From<!>                   TypeCall
               !                    TypeNever                                                                                             */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
fn a(x: !) -> ! { x }                                                                                                                     /*
fn•a(x:•!)•->•!•{•x•}    FunctionDeclaration
     x:•!                FunctionParameterDeclaration
        !                TypeNever
              !          TypeNever
                  x      ExpressionStatement                                                                                              */
fn foo(never: !) {}                                                                                                                       /*
fn•foo(never:•!)•{}    FunctionDeclaration
       never:•!        FunctionParameterDeclaration
              !        TypeNever                                                                                                          */
fn a(x: !) {}                                                                                                                             /*
fn•a(x:•!)•{}    FunctionDeclaration
     x:•!        FunctionParameterDeclaration
        !        TypeNever                                                                                                                */
fn a(ref x: !) {}                                                                                                                         /*
fn•a(ref•x:•!)•{}    FunctionDeclaration
     ref•x:•!        FunctionParameterDeclaration
     ref•x           PatternVariableDeclaration
            !        TypeNever                                                                                                            */
fn a(x: &[!]) {}                                                                                                                          /*
fn•a(x:•&[!])•{}    FunctionDeclaration
     x:•&[!]        FunctionParameterDeclaration
        &[!]        TypeReference
         [!]        TypeSlice
          !         TypeNever                                                                                                             */
fn a(x: B<(), !>) {}                                                                                                                      /*
fn•a(x:•B<(),•!>)•{}    FunctionDeclaration
     x:•B<(),•!>        FunctionParameterDeclaration
        B<(),•!>        TypeCall
          ()            TypeTuple
              !         TypeNever                                                                                                         */

impl A<!> for B {                                                                                                                         /*
impl•A<!>•for•B•{↲    <ImplDeclaration>
     A<!>             TypeCall
       !              TypeNever                                                                                                           */
    fn c(&self, d: &!) -> E {}                                                                                                            /*
    fn•c(&self,•d:•&!)•->•E•{}    FunctionDeclaration
         &self                    FunctionSelfParameterDeclaration
                d:•&!             FunctionParameterDeclaration
                   &!             TypeReference
                    !             TypeNever                                                                                               */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */
impl A for ! {}                                                                                                                           /*
impl•A•for•!•{}    ImplDeclaration
           !       TypeNever                                                                                                              */
type A = !;                                                                                                                               /*
type•A•=•!;    TypeAliasDeclaration
         !     TypeNever                                                                                                                  */

// Discarded Nodes: 0
// Parsed Nodes: 91
// state_rollbacks: 8
// Total '.charCodeAt()' calls: 439 (47% re-reads)
// Unnecessary 'skip_whitespace()' calls: 67
// source: "../../samples/types/never.rs"