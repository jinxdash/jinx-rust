fn main() {}                                                                                                                              /*
fn•main()•{}    FunctionDeclaration                                                                                                       */
fn f1_1(x: isize, ...) {}                                                                                                                 /*
fn•f1_1(x:•isize,•...)•{}    FunctionDeclaration
        x:•isize             FunctionParameterDeclaration
                  ...        FunctionSpread                                                                                               */
fn f1_2(...) {}                                                                                                                           /*
fn•f1_2(...)•{}    FunctionDeclaration
        ...        FunctionSpread                                                                                                         */
extern "C" fn f2_1(x: isize, ...) {}                                                                                                      /*
extern•"C"•fn•f2_1(x:•isize,•...)•{}    FunctionDeclaration
extern•"C"                              ExternSpecifier
       "C"                              Literal
                   x:•isize             FunctionParameterDeclaration
                             ...        FunctionSpread                                                                                    */
extern "C" fn f2_2(...) {}                                                                                                                /*
extern•"C"•fn•f2_2(...)•{}    FunctionDeclaration
extern•"C"                    ExternSpecifier
       "C"                    Literal
                   ...        FunctionSpread                                                                                              */
extern "C" fn f2_3(..., x: isize) {}                                                                                                      /*
extern•"C"•fn•f2_3(...,•x:•isize)•{}    FunctionDeclaration
extern•"C"                              ExternSpecifier
       "C"                              Literal
                   ...                  FunctionSpread
                        x:•isize        FunctionParameterDeclaration                                                                      */
extern fn f3_1(x: isize, ...) {}                                                                                                          /*
extern•fn•f3_1(x:•isize,•...)•{}    FunctionDeclaration
extern                              ExternSpecifier
               x:•isize             FunctionParameterDeclaration
                         ...        FunctionSpread                                                                                        */
extern fn f3_2(...) {}                                                                                                                    /*
extern•fn•f3_2(...)•{}    FunctionDeclaration
extern                    ExternSpecifier
               ...        FunctionSpread                                                                                                  */
extern fn f3_3(..., x: isize) {}                                                                                                          /*
extern•fn•f3_3(...,•x:•isize)•{}    FunctionDeclaration
extern                              ExternSpecifier
               ...                  FunctionSpread
                    x:•isize        FunctionParameterDeclaration                                                                          */
extern {                                                                                                                                  /*
extern•{↲    <ExternBlockDeclaration>                                                                                                     */
    fn e_f1(...);                                                                                                                         /*
    fn•e_f1(...);    FunctionDeclaration
            ...      FunctionSpread                                                                                                       */
    fn e_f2(..., x: isize);                                                                                                               /*
    fn•e_f2(...,•x:•isize);    FunctionDeclaration
            ...                FunctionSpread
                 x:•isize      FunctionParameterDeclaration                                                                               */
}                                                                                                                                         /*
}    </ExternBlockDeclaration>                                                                                                            */
struct X;                                                                                                                                 /*
struct•X;    StructDeclaration                                                                                                            */
impl X {                                                                                                                                  /*
impl•X•{↲    <ImplDeclaration>                                                                                                            */
    fn i_f1(x: isize, ...) {}                                                                                                             /*
    fn•i_f1(x:•isize,•...)•{}    FunctionDeclaration
            x:•isize             FunctionParameterDeclaration
                      ...        FunctionSpread                                                                                           */
    fn i_f2(...) {}                                                                                                                       /*
    fn•i_f2(...)•{}    FunctionDeclaration
            ...        FunctionSpread                                                                                                     */
    fn i_f3(..., x: isize, ...) {}                                                                                                        /*
    fn•i_f3(...,•x:•isize,•...)•{}    FunctionDeclaration
            ...                       FunctionSpread
                 x:•isize             FunctionParameterDeclaration
                           ...        FunctionSpread                                                                                      */
    fn i_f4(..., x: isize, ...) {}                                                                                                        /*
    fn•i_f4(...,•x:•isize,•...)•{}    FunctionDeclaration
            ...                       FunctionSpread
                 x:•isize             FunctionParameterDeclaration
                           ...        FunctionSpread                                                                                      */
}                                                                                                                                         /*
}    </ImplDeclaration>                                                                                                                   */
trait T {                                                                                                                                 /*
trait•T•{↲    <TraitDeclaration>                                                                                                          */
    fn t_f1(x: isize, ...) {}                                                                                                             /*
    fn•t_f1(x:•isize,•...)•{}    FunctionDeclaration
            x:•isize             FunctionParameterDeclaration
                      ...        FunctionSpread                                                                                           */
    fn t_f2(x: isize, ...);                                                                                                               /*
    fn•t_f2(x:•isize,•...);    FunctionDeclaration
            x:•isize           FunctionParameterDeclaration
                      ...      FunctionSpread                                                                                             */
    fn t_f3(...) {}                                                                                                                       /*
    fn•t_f3(...)•{}    FunctionDeclaration
            ...        FunctionSpread                                                                                                     */
    fn t_f4(...);                                                                                                                         /*
    fn•t_f4(...);    FunctionDeclaration
            ...      FunctionSpread                                                                                                       */
    fn t_f5(..., x: isize) {}                                                                                                             /*
    fn•t_f5(...,•x:•isize)•{}    FunctionDeclaration
            ...                  FunctionSpread
                 x:•isize        FunctionParameterDeclaration                                                                             */
    fn t_f6(..., x: isize);                                                                                                               /*
    fn•t_f6(...,•x:•isize);    FunctionDeclaration
            ...                FunctionSpread
                 x:•isize      FunctionParameterDeclaration                                                                               */
}                                                                                                                                         /*
}    </TraitDeclaration>                                                                                                                  */

extern "C" {                                                                                                                              /*
extern•"C"•{↲    <ExternBlockDeclaration>
       "C"       Literal                                                                                                                  */
    pub fn foo(x: i32, ...);                                                                                                              /*
    pub•fn•foo(x:•i32,•...);    FunctionDeclaration
    pub                         PubSpecifier
               x:•i32           FunctionParameterDeclaration
                       ...      FunctionSpread                                                                                            */
}                                                                                                                                         /*
}    </ExternBlockDeclaration>                                                                                                            */

// Discarded Nodes: 0
// Parsed Nodes: 130
// state_rollbacks: 14
// Total '.charCodeAt()' calls: 884 (35% re-reads)
// Unnecessary 'skip_whitespace()' calls: 79
// source: "../../samples/statements/spread.rs"