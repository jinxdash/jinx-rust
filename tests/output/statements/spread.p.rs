fn main() {}                                                                                                                              /*
fn•main()•{}↲    <Program>
fn•main()•{}↲    <Program.ast{dk: "None"}>
fn•main()•{}     FunctionDeclaration
       ()        FunctionDeclaration.parameters{dk: "()"}
          {}     FunctionDeclaration.body{dk: "{}"}                                                                                       */
fn f1_1(x: isize, ...) {}                                                                                                                 /*
fn•f1_1(x:•isize,•...)•{}    FunctionDeclaration
       (x:•isize,•...)       FunctionDeclaration.parameters{dk: "()"}
        x:•isize             FunctionParameterDeclaration
                  ...        FunctionSpread
                       {}    FunctionDeclaration.body{dk: "{}"}                                                                           */
fn f1_2(...) {}                                                                                                                           /*
fn•f1_2(...)•{}    FunctionDeclaration
       (...)       FunctionDeclaration.parameters{dk: "()"}
        ...        FunctionSpread
             {}    FunctionDeclaration.body{dk: "{}"}                                                                                     */
extern "C" fn f2_1(x: isize, ...) {}                                                                                                      /*
extern•"C"•fn•f2_1(x:•isize,•...)•{}    FunctionDeclaration
extern•"C"                              ExternSpecifier
       "C"                              Literal{kind: String}
                  (x:•isize,•...)       FunctionDeclaration.parameters{dk: "()"}
                   x:•isize             FunctionParameterDeclaration
                             ...        FunctionSpread
                                  {}    FunctionDeclaration.body{dk: "{}"}                                                                */
extern "C" fn f2_2(...) {}                                                                                                                /*
extern•"C"•fn•f2_2(...)•{}    FunctionDeclaration
extern•"C"                    ExternSpecifier
       "C"                    Literal{kind: String}
                  (...)       FunctionDeclaration.parameters{dk: "()"}
                   ...        FunctionSpread
                        {}    FunctionDeclaration.body{dk: "{}"}                                                                          */
extern "C" fn f2_3(..., x: isize) {}                                                                                                      /*
extern•"C"•fn•f2_3(...,•x:•isize)•{}    FunctionDeclaration
extern•"C"                              ExternSpecifier
       "C"                              Literal{kind: String}
                  (...,•x:•isize)       FunctionDeclaration.parameters{dk: "()"}
                   ...                  FunctionSpread
                        x:•isize        FunctionParameterDeclaration
                                  {}    FunctionDeclaration.body{dk: "{}"}                                                                */
extern fn f3_1(x: isize, ...) {}                                                                                                          /*
extern•fn•f3_1(x:•isize,•...)•{}    FunctionDeclaration
extern                              ExternSpecifier
              (x:•isize,•...)       FunctionDeclaration.parameters{dk: "()"}
               x:•isize             FunctionParameterDeclaration
                         ...        FunctionSpread
                              {}    FunctionDeclaration.body{dk: "{}"}                                                                    */
extern fn f3_2(...) {}                                                                                                                    /*
extern•fn•f3_2(...)•{}    FunctionDeclaration
extern                    ExternSpecifier
              (...)       FunctionDeclaration.parameters{dk: "()"}
               ...        FunctionSpread
                    {}    FunctionDeclaration.body{dk: "{}"}                                                                              */
extern fn f3_3(..., x: isize) {}                                                                                                          /*
extern•fn•f3_3(...,•x:•isize)•{}    FunctionDeclaration
extern                              ExternSpecifier
              (...,•x:•isize)       FunctionDeclaration.parameters{dk: "()"}
               ...                  FunctionSpread
                    x:•isize        FunctionParameterDeclaration
                              {}    FunctionDeclaration.body{dk: "{}"}                                                                    */
extern {                                                                                                                                  /*
extern•{↲    <ExternBlockDeclaration>
       {↲    <ExternBlockDeclaration.body{dk: "{}"}>                                                                                      */
    fn e_f1(...);                                                                                                                         /*
    fn•e_f1(...);    FunctionDeclaration
           (...)     FunctionDeclaration.parameters{dk: "()"}
            ...      FunctionSpread                                                                                                       */
    fn e_f2(..., x: isize);                                                                                                               /*
    fn•e_f2(...,•x:•isize);    FunctionDeclaration
           (...,•x:•isize)     FunctionDeclaration.parameters{dk: "()"}
            ...                FunctionSpread
                 x:•isize      FunctionParameterDeclaration                                                                               */
}                                                                                                                                         /*
}    </ExternBlockDeclaration.body>
}    </ExternBlockDeclaration>                                                                                                            */
struct X;                                                                                                                                 /*
struct•X;    StructDeclaration                                                                                                            */
impl X {                                                                                                                                  /*
impl•X•{↲    <ImplDeclaration{!const}>
       {↲    <ImplDeclaration.body{dk: "{}"}>                                                                                             */
    fn i_f1(x: isize, ...) {}                                                                                                             /*
    fn•i_f1(x:•isize,•...)•{}    FunctionDeclaration
           (x:•isize,•...)       FunctionDeclaration.parameters{dk: "()"}
            x:•isize             FunctionParameterDeclaration
                      ...        FunctionSpread
                           {}    FunctionDeclaration.body{dk: "{}"}                                                                       */
    fn i_f2(...) {}                                                                                                                       /*
    fn•i_f2(...)•{}    FunctionDeclaration
           (...)       FunctionDeclaration.parameters{dk: "()"}
            ...        FunctionSpread
                 {}    FunctionDeclaration.body{dk: "{}"}                                                                                 */
    fn i_f3(..., x: isize, ...) {}                                                                                                        /*
    fn•i_f3(...,•x:•isize,•...)•{}    FunctionDeclaration
           (...,•x:•isize,•...)       FunctionDeclaration.parameters{dk: "()"}
            ...                       FunctionSpread
                 x:•isize             FunctionParameterDeclaration
                           ...        FunctionSpread
                                {}    FunctionDeclaration.body{dk: "{}"}                                                                  */
    fn i_f4(..., x: isize, ...) {}                                                                                                        /*
    fn•i_f4(...,•x:•isize,•...)•{}    FunctionDeclaration
           (...,•x:•isize,•...)       FunctionDeclaration.parameters{dk: "()"}
            ...                       FunctionSpread
                 x:•isize             FunctionParameterDeclaration
                           ...        FunctionSpread
                                {}    FunctionDeclaration.body{dk: "{}"}                                                                  */
}                                                                                                                                         /*
}    </ImplDeclaration.body>
}    </ImplDeclaration>                                                                                                                   */
trait T {                                                                                                                                 /*
trait•T•{↲    <TraitDeclaration>
        {↲    <TraitDeclaration.body{dk: "{}"}>                                                                                           */
    fn t_f1(x: isize, ...) {}                                                                                                             /*
    fn•t_f1(x:•isize,•...)•{}    FunctionDeclaration
           (x:•isize,•...)       FunctionDeclaration.parameters{dk: "()"}
            x:•isize             FunctionParameterDeclaration
                      ...        FunctionSpread
                           {}    FunctionDeclaration.body{dk: "{}"}                                                                       */
    fn t_f2(x: isize, ...);                                                                                                               /*
    fn•t_f2(x:•isize,•...);    FunctionDeclaration
           (x:•isize,•...)     FunctionDeclaration.parameters{dk: "()"}
            x:•isize           FunctionParameterDeclaration
                      ...      FunctionSpread                                                                                             */
    fn t_f3(...) {}                                                                                                                       /*
    fn•t_f3(...)•{}    FunctionDeclaration
           (...)       FunctionDeclaration.parameters{dk: "()"}
            ...        FunctionSpread
                 {}    FunctionDeclaration.body{dk: "{}"}                                                                                 */
    fn t_f4(...);                                                                                                                         /*
    fn•t_f4(...);    FunctionDeclaration
           (...)     FunctionDeclaration.parameters{dk: "()"}
            ...      FunctionSpread                                                                                                       */
    fn t_f5(..., x: isize) {}                                                                                                             /*
    fn•t_f5(...,•x:•isize)•{}    FunctionDeclaration
           (...,•x:•isize)       FunctionDeclaration.parameters{dk: "()"}
            ...                  FunctionSpread
                 x:•isize        FunctionParameterDeclaration
                           {}    FunctionDeclaration.body{dk: "{}"}                                                                       */
    fn t_f6(..., x: isize);                                                                                                               /*
    fn•t_f6(...,•x:•isize);    FunctionDeclaration
           (...,•x:•isize)     FunctionDeclaration.parameters{dk: "()"}
            ...                FunctionSpread
                 x:•isize      FunctionParameterDeclaration                                                                               */
}                                                                                                                                         /*
}    </TraitDeclaration.body>
}    </TraitDeclaration>                                                                                                                  */

extern "C" {                                                                                                                              /*
extern•"C"•{↲    <ExternBlockDeclaration>
       "C"       Literal{kind: String}
           {↲    <ExternBlockDeclaration.body{dk: "{}"}>                                                                                  */
    pub fn foo(x: i32, ...);                                                                                                              /*
    pub•fn•foo(x:•i32,•...);    FunctionDeclaration
    pub                         PubSpecifier
              (x:•i32,•...)     FunctionDeclaration.parameters{dk: "()"}
               x:•i32           FunctionParameterDeclaration
                       ...      FunctionSpread                                                                                            */
}                                                                                                                                         /*
}    </ExternBlockDeclaration.body>
}    </ExternBlockDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 130
// state_rollbacks: 14
// Total '.charCodeAt()' calls: 884 (35% re-reads)
// Unnecessary 'skip_whitespace()' calls: 79
// source: "../../samples/statements/spread.rs"