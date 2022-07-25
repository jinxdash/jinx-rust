#![feature(const_generics_defaults)]                                                                                                      /*
#![feature(const_generics_defaults)]    Attribute
          (const_generics_defaults)     DelimGroup                                                                                        */

struct Foo<const N: usize    =  1, const N2: usize =           2>;                                                                        /*
struct•Foo<const•N:•usize••••=••1,•const•N2:•usize•=•••••••••••2>;    StructDeclaration
           const•N:•usize••••=••1                                     ConstTypeParameterDeclaration
                                1                                     Literal
                                   const•N2:•usize•=•••••••••••2      ConstTypeParameterDeclaration
                                                               2      Literal                                                             */
struct Bar<const N: usize, const N2: usize = {      N +                                                                                   /*
struct•Bar<const•N:•usize,•const•N2:•usize•=•{••••••N•+•↲    <StructDeclaration>
           const•N:•usize                                    ConstTypeParameterDeclaration
                           const•N2:•usize•=•{••••••N•+•↲    <ConstTypeParameterDeclaration>
                                             {••••••N•+•↲    <BlockExpression>
                                                    N•+•↲    <ExpressionStatement>, <OperationExpression>                                 */
1 }>;                                                                                                                                     /*
1•}>;    </StructDeclaration>
1•}      </ConstTypeParameterDeclaration>, </BlockExpression>
1        </ExpressionStatement>, </OperationExpression>                                                                                   */
struct Lots<const N1BlahFooUwU: usize = { 10 + 28 + 1872 / 10 * 3 },const N2SecondParamOhmyyy: usize = { N1BlahFooUwU / 2 + 10 * 2 },>;   /*
struct•Lots<const•N1BlahFooUwU:•usize•=•{•10•+•28•+•1872•/•10•*•3•},const•N2SecondParamOhmyyy:•usize•=•{•N1BlahFooUwU•/•2•+•10•*•2•},>;    StructDeclaration
            const•N1BlahFooUwU:•usize•=•{•10•+•28•+•1872•/•10•*•3•}                                                                        ConstTypeParameterDeclaration
                                        {•10•+•28•+•1872•/•10•*•3•}                                                                        BlockExpression
                                          10•+•28•+•1872•/•10•*•3                                                                          ExpressionStatement, OperationExpression
                                          10•+•28                                                                                          OperationExpression
                                          10                                                                                               Literal
                                               28                                                                                          Literal
                                                    1872•/•10•*•3                                                                          OperationExpression
                                                    1872•/•10                                                                              OperationExpression
                                                    1872                                                                                   Literal
                                                           10                                                                              Literal
                                                                3                                                                          Literal
                                                                    const•N2SecondParamOhmyyy:•usize•=•{•N1BlahFooUwU•/•2•+•10•*•2•}       ConstTypeParameterDeclaration
                                                                                                       {•N1BlahFooUwU•/•2•+•10•*•2•}       BlockExpression
                                                                                                         N1BlahFooUwU•/•2•+•10•*•2         ExpressionStatement, OperationExpression
                                                                                                         N1BlahFooUwU•/•2                  OperationExpression
                                                                                                                        2                  Literal
                                                                                                                            10•*•2         OperationExpression
                                                                                                                            10             Literal
                                                                                                                                 2         Literal*/
struct Lott<const N1BlahFooUwU: usize = {                                                                                                 /*
struct•Lott<const•N1BlahFooUwU:•usize•=•{•↲    <StructDeclaration>
            const•N1BlahFooUwU:•usize•=•{•↲    <ConstTypeParameterDeclaration>
                                        {•↲    <BlockExpression>                                                                          */
    // 1
    //•1    Comment
    1                                                                                                                                     /*
    1    ExpressionStatement, Literal                                                                                                     */
 },const N2SecondParamOhmyyy: usize = { 2 },>;                                                                                            /*
•},const•N2SecondParamOhmyyy:•usize•=•{•2•},>;    </StructDeclaration>
•}                                                </ConstTypeParameterDeclaration>, </BlockExpression>
   const•N2SecondParamOhmyyy:•usize•=•{•2•}       ConstTypeParameterDeclaration
                                      {•2•}       BlockExpression
                                        2         ExpressionStatement, Literal                                                            */
struct NamesRHard<const N: usize = { 1 + 1 + 1 + 1 + 1 + 1 }>;                                                                            /*
struct•NamesRHard<const•N:•usize•=•{•1•+•1•+•1•+•1•+•1•+•1•}>;    StructDeclaration
                  const•N:•usize•=•{•1•+•1•+•1•+•1•+•1•+•1•}      ConstTypeParameterDeclaration
                                   {•1•+•1•+•1•+•1•+•1•+•1•}      BlockExpression
                                     1•+•1•+•1•+•1•+•1•+•1        ExpressionStatement, OperationExpression
                                     1•+•1•+•1•+•1•+•1            OperationExpression
                                     1•+•1•+•1•+•1                OperationExpression
                                     1•+•1•+•1                    OperationExpression
                                     1•+•1                        OperationExpression
                                     1                            Literal
                                         1                        Literal
                                             1                    Literal
                                                 1                Literal
                                                     1            Literal
                                                         1        Literal                                                                 */
struct FooBar<                                                                                                                            /*
struct•FooBar<↲    <StructDeclaration>                                                                                                    */
    const LessThan100ButClose: usize = {1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1}                                                        /*
    const•LessThan100ButClose:•usize•=•{1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1}    ConstTypeParameterDeclaration
                                       {1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1}    BlockExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1     ExpressionStatement, OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1       OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1         OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1           OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1             OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1               OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1+1                 OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1+1                   OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1+1                     OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1+1                       OperationExpression
                                        1+1+1+1+1+1+1+1+1+1+1                         OperationExpression
                                        1+1+1+1+1+1+1+1+1+1                           OperationExpression
                                        1+1+1+1+1+1+1+1+1                             OperationExpression
                                        1+1+1+1+1+1+1+1                               OperationExpression
                                        1+1+1+1+1+1+1                                 OperationExpression
                                        1+1+1+1+1+1                                   OperationExpression
                                        1+1+1+1+1                                     OperationExpression
                                        1+1+1+1                                       OperationExpression
                                        1+1+1                                         OperationExpression
                                        1+1                                           OperationExpression
                                        1                                             Literal
                                          1                                           Literal
                                            1                                         Literal
                                              1                                       Literal
                                                1                                     Literal
                                                  1                                   Literal
                                                    1                                 Literal
                                                      1                               Literal
                                                        1                             Literal
                                                          1                           Literal
                                                            1                         Literal
                                                              1                       Literal
                                                                1                     Literal
                                                                  1                   Literal
                                                                    1                 Literal
                                                                      1               Literal
                                                                        1             Literal
                                                                          1           Literal
                                                                            1         Literal
                                                                              1       Literal
                                                                                1     Literal                                             */
>;                                                                                                                                        /*
>;    </StructDeclaration>                                                                                                                */
struct FooBarrrrrrrr<const N: usize        =           {13478234326456456444323871+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+ 1+1+1+1 + 1},>;   /*
struct•FooBarrrrrrrr<const•N:•usize••••••••=•••••••••••{13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1},>;    StructDeclaration
                     const•N:•usize••••••••=•••••••••••{13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1}       ConstTypeParameterDeclaration
                                                       {13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1}       BlockExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1•+•1        ExpressionStatement, OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1+1            OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1+1              OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+1                OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1                  OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1                     OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1                        OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1+•1                           OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1+•1                              OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1+•1                                 OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1+•1                                    OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1+•1                                       OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1+•1                                          OperationExpression
                                                        13478234326456456444323871+•1+•1+•1+•1                                             OperationExpression
                                                        13478234326456456444323871+•1+•1+•1                                                OperationExpression
                                                        13478234326456456444323871+•1+•1                                                   OperationExpression
                                                        13478234326456456444323871+•1                                                      OperationExpression
                                                        13478234326456456444323871                                                         Literal
                                                                                    1                                                      Literal
                                                                                       1                                                   Literal
                                                                                          1                                                Literal
                                                                                             1                                             Literal
                                                                                                1                                          Literal
                                                                                                   1                                       Literal
                                                                                                      1                                    Literal
                                                                                                         1                                 Literal
                                                                                                            1                              Literal
                                                                                                               1                           Literal
                                                                                                                  1                        Literal
                                                                                                                     1                     Literal
                                                                                                                        1                  Literal
                                                                                                                          1                Literal
                                                                                                                            1              Literal
                                                                                                                              1            Literal
                                                                                                                                  1        Literal*/

// Discarded Nodes: 0
// Parsed Nodes: 180
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 908 (28% re-reads)
// Unnecessary 'skip_whitespace()' calls: 77
// source: "../../samples/features/const_generics_defaults.rs"