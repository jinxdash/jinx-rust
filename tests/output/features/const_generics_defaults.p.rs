#![feature(const_generics_defaults)]                                                                                                      /*
#![feature(const_generics_defaults)]↲    <Program>
#![feature(const_generics_defaults)]     Attribute{inner}
  [feature(const_generics_defaults)]     Attribute.segments{dk: "[]"}
          (const_generics_defaults)      DelimGroup                                                                                       */

struct Foo<const N: usize    =  1, const N2: usize =           2>;                                                                        /*
struct•Foo<const•N:•usize••••=••1,•const•N2:•usize•=•••••••••••2>;↲    <Program.ast{dk: "None"}>
struct•Foo<const•N:•usize••••=••1,•const•N2:•usize•=•••••••••••2>;     StructDeclaration
          <const•N:•usize••••=••1,•const•N2:•usize•=•••••••••••2>      StructDeclaration.generics{dk: "<>"}
           const•N:•usize••••=••1                                      ConstTypeParameterDeclaration
                                1                                      Literal{kind: Integer}
                                   const•N2:•usize•=•••••••••••2       ConstTypeParameterDeclaration
                                                               2       Literal{kind: Integer}                                             */
struct Bar<const N: usize, const N2: usize = {      N +                                                                                   /*
struct•Bar<const•N:•usize,•const•N2:•usize•=•{••••••N•+•↲    <StructDeclaration>
          <const•N:•usize,•const•N2:•usize•=•{••••••N•+•↲    <StructDeclaration.generics{dk: "<>"}>
           const•N:•usize                                    ConstTypeParameterDeclaration
                           const•N2:•usize•=•{••••••N•+•↲    <ConstTypeParameterDeclaration>
                                             {••••••N•+•↲    <BlockExpression>
                                                    N•+•↲    <ExpressionStatement{!semi}>
                                                    N•+•↲    <OperationExpression{tk: "+"}>                                               */
1 }>;                                                                                                                                     /*
1        Literal{kind: Integer}
1        </OperationExpression>
1        </ExpressionStatement>
1•}      </BlockExpression>
1•}      </ConstTypeParameterDeclaration>
1•}>     </StructDeclaration.generics>
1•}>;    </StructDeclaration>                                                                                                             */
struct Lots<const N1BlahFooUwU: usize = { 10 + 28 + 1872 / 10 * 3 },const N2SecondParamOhmyyy: usize = { N1BlahFooUwU / 2 + 10 * 2 },>;   /*
struct•Lots<const•N1BlahFooUwU:•usize•=•{•10•+•28•+•1872•/•10•*•3•},const•N2SecondParamOhmyyy:•usize•=•{•N1BlahFooUwU•/•2•+•10•*•2•},>;    StructDeclaration
           <const•N1BlahFooUwU:•usize•=•{•10•+•28•+•1872•/•10•*•3•},const•N2SecondParamOhmyyy:•usize•=•{•N1BlahFooUwU•/•2•+•10•*•2•},>     StructDeclaration.generics{dk: "<>"}
            const•N1BlahFooUwU:•usize•=•{•10•+•28•+•1872•/•10•*•3•}                                                                        ConstTypeParameterDeclaration
                                        {•10•+•28•+•1872•/•10•*•3•}                                                                        BlockExpression
                                          10•+•28•+•1872•/•10•*•3                                                                          ExpressionStatement{!semi}, OperationExpression{tk: "+"}
                                          10•+•28                                                                                          OperationExpression{tk: "+"}
                                          10                                                                                               Literal{kind: Integer}
                                               28                                                                                          Literal{kind: Integer}
                                                    1872•/•10•*•3                                                                          OperationExpression{tk: "*"}
                                                    1872•/•10                                                                              OperationExpression{tk: "/"}
                                                    1872                                                                                   Literal{kind: Integer}
                                                           10                                                                              Literal{kind: Integer}
                                                                3                                                                          Literal{kind: Integer}
                                                                    const•N2SecondParamOhmyyy:•usize•=•{•N1BlahFooUwU•/•2•+•10•*•2•}       ConstTypeParameterDeclaration
                                                                                                       {•N1BlahFooUwU•/•2•+•10•*•2•}       BlockExpression
                                                                                                         N1BlahFooUwU•/•2•+•10•*•2         ExpressionStatement{!semi}, OperationExpression{tk: "+"}
                                                                                                         N1BlahFooUwU•/•2                  OperationExpression{tk: "/"}
                                                                                                                        2                  Literal{kind: Integer}
                                                                                                                            10•*•2         OperationExpression{tk: "*"}
                                                                                                                            10             Literal{kind: Integer}
                                                                                                                                 2         Literal{kind: Integer}*/
struct Lott<const N1BlahFooUwU: usize = {                                                                                                 /*
struct•Lott<const•N1BlahFooUwU:•usize•=•{•↲    <StructDeclaration>
           <const•N1BlahFooUwU:•usize•=•{•↲    <StructDeclaration.generics{dk: "<>"}>
            const•N1BlahFooUwU:•usize•=•{•↲    <ConstTypeParameterDeclaration>
                                        {•↲    <BlockExpression>                                                                          */
    // 1
    //•1    Comment{line}
    1                                                                                                                                     /*
    1    ExpressionStatement{!semi}, Literal{kind: Integer}                                                                               */
 },const N2SecondParamOhmyyy: usize = { 2 },>;                                                                                            /*
•}                                                </BlockExpression>
•}                                                </ConstTypeParameterDeclaration>
   const•N2SecondParamOhmyyy:•usize•=•{•2•}       ConstTypeParameterDeclaration
                                      {•2•}       BlockExpression
                                        2         ExpressionStatement{!semi}, Literal{kind: Integer}
•},const•N2SecondParamOhmyyy:•usize•=•{•2•},>     </StructDeclaration.generics>
•},const•N2SecondParamOhmyyy:•usize•=•{•2•},>;    </StructDeclaration>                                                                    */
struct NamesRHard<const N: usize = { 1 + 1 + 1 + 1 + 1 + 1 }>;                                                                            /*
struct•NamesRHard<const•N:•usize•=•{•1•+•1•+•1•+•1•+•1•+•1•}>;    StructDeclaration
                 <const•N:•usize•=•{•1•+•1•+•1•+•1•+•1•+•1•}>     StructDeclaration.generics{dk: "<>"}
                  const•N:•usize•=•{•1•+•1•+•1•+•1•+•1•+•1•}      ConstTypeParameterDeclaration
                                   {•1•+•1•+•1•+•1•+•1•+•1•}      BlockExpression
                                     1•+•1•+•1•+•1•+•1•+•1        ExpressionStatement{!semi}, OperationExpression{tk: "+"}
                                     1•+•1•+•1•+•1•+•1            OperationExpression{tk: "+"}
                                     1•+•1•+•1•+•1                OperationExpression{tk: "+"}
                                     1•+•1•+•1                    OperationExpression{tk: "+"}
                                     1•+•1                        OperationExpression{tk: "+"}
                                     1                            Literal{kind: Integer}
                                         1                        Literal{kind: Integer}
                                             1                    Literal{kind: Integer}
                                                 1                Literal{kind: Integer}
                                                     1            Literal{kind: Integer}
                                                         1        Literal{kind: Integer}                                                  */
struct FooBar<                                                                                                                            /*
struct•FooBar<↲    <StructDeclaration>
             <↲    <StructDeclaration.generics{dk: "<>"}>                                                                                 */
    const LessThan100ButClose: usize = {1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1}                                                        /*
    const•LessThan100ButClose:•usize•=•{1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1}    ConstTypeParameterDeclaration
                                       {1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1}    BlockExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1     ExpressionStatement{!semi}, OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1       OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1         OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1           OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1             OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1               OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1                 OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1                   OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1+1+1                     OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1+1                       OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1+1                         OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1+1                           OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1+1                             OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1+1                               OperationExpression{tk: "+"}
                                        1+1+1+1+1+1+1                                 OperationExpression{tk: "+"}
                                        1+1+1+1+1+1                                   OperationExpression{tk: "+"}
                                        1+1+1+1+1                                     OperationExpression{tk: "+"}
                                        1+1+1+1                                       OperationExpression{tk: "+"}
                                        1+1+1                                         OperationExpression{tk: "+"}
                                        1+1                                           OperationExpression{tk: "+"}
                                        1                                             Literal{kind: Integer}
                                          1                                           Literal{kind: Integer}
                                            1                                         Literal{kind: Integer}
                                              1                                       Literal{kind: Integer}
                                                1                                     Literal{kind: Integer}
                                                  1                                   Literal{kind: Integer}
                                                    1                                 Literal{kind: Integer}
                                                      1                               Literal{kind: Integer}
                                                        1                             Literal{kind: Integer}
                                                          1                           Literal{kind: Integer}
                                                            1                         Literal{kind: Integer}
                                                              1                       Literal{kind: Integer}
                                                                1                     Literal{kind: Integer}
                                                                  1                   Literal{kind: Integer}
                                                                    1                 Literal{kind: Integer}
                                                                      1               Literal{kind: Integer}
                                                                        1             Literal{kind: Integer}
                                                                          1           Literal{kind: Integer}
                                                                            1         Literal{kind: Integer}
                                                                              1       Literal{kind: Integer}
                                                                                1     Literal{kind: Integer}                              */
>;                                                                                                                                        /*
>     </StructDeclaration.generics>
>;    </StructDeclaration>                                                                                                                */
struct FooBarrrrrrrr<const N: usize        =           {13478234326456456444323871+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+1+1+1 + 1},>;   /*
struct•FooBarrrrrrrr<const•N:•usize••••••••=•••••••••••{13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1},>;    StructDeclaration
                    <const•N:•usize••••••••=•••••••••••{13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1},>     StructDeclaration.generics{dk: "<>"}
                     const•N:•usize••••••••=•••••••••••{13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1}       ConstTypeParameterDeclaration
                                                       {13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1}       BlockExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1        ExpressionStatement{!semi}, OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1            OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1              OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1                OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1                  OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1                     OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1                        OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1                           OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1                              OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1                                 OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1                                    OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1                                       OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1+•1                                          OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1+•1                                             OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1+•1                                                OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1+•1                                                   OperationExpression{tk: "+"}
                                                        13478234326456456444323871+•1                                                      OperationExpression{tk: "+"}
                                                        13478234326456456444323871                                                         Literal{kind: Integer}
                                                                                    1                                                      Literal{kind: Integer}
                                                                                       1                                                   Literal{kind: Integer}
                                                                                          1                                                Literal{kind: Integer}
                                                                                             1                                             Literal{kind: Integer}
                                                                                                1                                          Literal{kind: Integer}
                                                                                                   1                                       Literal{kind: Integer}
                                                                                                      1                                    Literal{kind: Integer}
                                                                                                         1                                 Literal{kind: Integer}
                                                                                                            1                              Literal{kind: Integer}
                                                                                                               1                           Literal{kind: Integer}
                                                                                                                  1                        Literal{kind: Integer}
                                                                                                                     1                     Literal{kind: Integer}
                                                                                                                        1                  Literal{kind: Integer}
                                                                                                                          1                Literal{kind: Integer}
                                                                                                                            1              Literal{kind: Integer}
                                                                                                                              1            Literal{kind: Integer}
                                                                                                                                  1        Literal{kind: Integer}
struct•FooBarrrrrrrr<const•N:•usize••••••••=•••••••••••{13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1},>;    </Program.ast>
struct•FooBarrrrrrrr<const•N:•usize••••••••=•••••••••••{13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1},>;    </Program>*/
// Discarded Nodes: 0
// Parsed Nodes: 180
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 908 (28% re-reads)
// Unnecessary 'skip_whitespace()' calls: 77
// source: "../../samples/features/const_generics_defaults.rs"