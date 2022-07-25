
fn b() {                                                                                                                                  /*
fn•b()•{↲    <FunctionDeclaration>                                                                                                        */
    // fn foo(..: u8) {}
    //•fn•foo(..:•u8)•{}    Comment
    let ..;                                                                                                                               /*
    let•..;    LetVariableDeclaration
        ..     RangePattern                                                                                                               */
    let box ..;                                                                                                                           /*
    let•box•..;    LetVariableDeclaration
        box•..     BoxPattern
            ..     RangePattern                                                                                                           */
    match x { .. | .. => {} }                                                                                                             /*
    match•x•{•..•|•..•=>•{}•}    ExpressionStatement, MatchExpression
              ..•|•..•=>•{}      MatchExpressionCase
              ..•|•..            UnionPattern
              ..                 RangePattern
                   ..            RangePattern
                         {}      BlockExpression                                                                                          */
    let &..;                                                                                                                              /*
    let•&..;    LetVariableDeclaration
        &..     ReferencePattern
         ..     RangePattern                                                                                                              */
    let &mut ..;                                                                                                                          /*
    let•&mut•..;    LetVariableDeclaration
        &mut•..     ReferencePattern
             ..     RangePattern                                                                                                          */
    let x @ ..;                                                                                                                           /*
    let•x•@•..;    LetVariableDeclaration
        x•@•..     PatternVariableDeclaration
            ..     RangePattern                                                                                                           */
    let ref x @ ..;                                                                                                                       /*
    let•ref•x•@•..;    LetVariableDeclaration
        ref•x•@•..     PatternVariableDeclaration
                ..     RangePattern                                                                                                       */
    let ref mut x @ ..;                                                                                                                   /*
    let•ref•mut•x•@•..;    LetVariableDeclaration
        ref•mut•x•@•..     PatternVariableDeclaration
                    ..     RangePattern                                                                                                   */
    let (..);                                                                                                                             /*
    let•(..);    LetVariableDeclaration
        (..)     TuplePattern
         ..      RestPattern                                                                                                              */
    let (..,);                                                                                                                            /*
    let•(..,);    LetVariableDeclaration
        (..,)     TuplePattern
         ..       RestPattern                                                                                                             */
    let (.., .., ..);                                                                                                                     /*
    let•(..,•..,•..);    LetVariableDeclaration
        (..,•..,•..)     TuplePattern
         ..              RestPattern
             ..          RestPattern
                 ..      RestPattern                                                                                                      */
    let (.., P, ..);                                                                                                                      /*
    let•(..,•P,•..);    LetVariableDeclaration
        (..,•P,•..)     TuplePattern
         ..             RestPattern
                ..      RestPattern                                                                                                       */
    let A(..);                                                                                                                            /*
    let•A(..);    LetVariableDeclaration
        A(..)     TuplePattern
          ..      RestPattern                                                                                                             */
    let A(..,);                                                                                                                           /*
    let•A(..,);    LetVariableDeclaration
        A(..,)     TuplePattern
          ..       RestPattern                                                                                                            */
    let A(.., .., ..);                                                                                                                    /*
    let•A(..,•..,•..);    LetVariableDeclaration
        A(..,•..,•..)     TuplePattern
          ..              RestPattern
              ..          RestPattern
                  ..      RestPattern                                                                                                     */
    let A(.., P, ..);                                                                                                                     /*
    let•A(..,•P,•..);    LetVariableDeclaration
        A(..,•P,•..)     TuplePattern
          ..             RestPattern
                 ..      RestPattern                                                                                                      */
    let [..];                                                                                                                             /*
    let•[..];    LetVariableDeclaration
        [..]     ArrayPattern
         ..      RestPattern                                                                                                              */
    let [..,];                                                                                                                            /*
    let•[..,];    LetVariableDeclaration
        [..,]     ArrayPattern
         ..       RestPattern                                                                                                             */
    let [.., .., ..];                                                                                                                     /*
    let•[..,•..,•..];    LetVariableDeclaration
        [..,•..,•..]     ArrayPattern
         ..              RestPattern
             ..          RestPattern
                 ..      RestPattern                                                                                                      */
    let [.., P, ..];                                                                                                                      /*
    let•[..,•P,•..];    LetVariableDeclaration
        [..,•P,•..]     ArrayPattern
         ..             RestPattern
                ..      RestPattern                                                                                                       */
    match x {                                                                                                                             /*
    match•x•{↲    <ExpressionStatement>, <MatchExpression>                                                                                */
        .. | [(box .., &(..), &mut .., x @ ..), ref x @ ..,] | ref mut x @ .. if x[..] => {}                                              /*
        ..•|•[(box•..,•&(..),•&mut•..,•x•@•..),•ref•x•@•..,]•|•ref•mut•x•@•..•if•x[..]•=>•{}    MatchExpressionCase
        ..•|•[(box•..,•&(..),•&mut•..,•x•@•..),•ref•x•@•..,]•|•ref•mut•x•@•..                   UnionPattern
        ..                                                                                      RangePattern
             [(box•..,•&(..),•&mut•..,•x•@•..),•ref•x•@•..,]                                    ArrayPattern
              (box•..,•&(..),•&mut•..,•x•@•..)                                                  TuplePattern
               box•..                                                                           BoxPattern
                   ..                                                                           RestPattern
                       &(..)                                                                    ReferencePattern
                        (..)                                                                    TuplePattern
                         ..                                                                     RestPattern
                              &mut•..                                                           ReferencePattern
                                   ..                                                           RestPattern
                                       x•@•..                                                   PatternVariableDeclaration
                                           ..                                                   RestPattern
                                                ref•x•@•..                                      PatternVariableDeclaration
                                                        ..                                      RestPattern
                                                               ref•mut•x•@•..                   PatternVariableDeclaration
                                                                           ..                   RangePattern
                                                                                 x[..]          MemberExpression
                                                                                   ..           RangeLiteral
                                                                                          {}    BlockExpression                           */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
    a!(..);                                                                                                                               /*
    a!(..);    ExpressionStatement
    a!(..)     MacroInvocation
       ..      PunctuationToken                                                                                                           */

	let [..]: &[u8];                                                                                                                      /*
    let•[..]:•&[u8];    LetVariableDeclaration
        [..]            ArrayPattern
         ..             RestPattern
              &[u8]     TypeReference
               [u8]     TypeSlice                                                                                                         */
	let [..,]: &[u8];                                                                                                                     /*
    let•[..,]:•&[u8];    LetVariableDeclaration
        [..,]            ArrayPattern
         ..              RestPattern
               &[u8]     TypeReference
                [u8]     TypeSlice                                                                                                        */
	let (..): (u8,);                                                                                                                      /*
    let•(..):•(u8,);    LetVariableDeclaration
        (..)            TuplePattern
         ..             RestPattern
              (u8,)     TypeTuple                                                                                                         */
	let (..,): (u8,);                                                                                                                     /*
    let•(..,):•(u8,);    LetVariableDeclaration
        (..,)            TuplePattern
         ..              RestPattern
               (u8,)     TypeTuple                                                                                                        */

	let (1, (Some(1), 2..=3)) = (1, (None, 2));                                                                                           /*
    let•(1,•(Some(1),•2..=3))•=•(1,•(None,•2));    LetVariableDeclaration
        (1,•(Some(1),•2..=3))                      TuplePattern
         1                                         Literal
            (Some(1),•2..=3)                       TuplePattern
             Some(1)                               TuplePattern
                  1                                Literal
                      2..=3                        RangePattern
                      2                            Literal
                          3                        Literal
                                (1,•(None,•2))     TupleLiteral
                                 1                 Literal
                                    (None,•2)      TupleLiteral
                                           2       Literal                                                                                */
	fn func((1, (Some(1), 2..=3)): (isize, (Option<isize>, isize))) {}                                                                    /*
    fn•func((1,•(Some(1),•2..=3)):•(isize,•(Option<isize>,•isize)))•{}    FunctionDeclaration
            (1,•(Some(1),•2..=3)):•(isize,•(Option<isize>,•isize))        FunctionParameterDeclaration
            (1,•(Some(1),•2..=3))                                         TuplePattern
             1                                                            Literal
                (Some(1),•2..=3)                                          TuplePattern
                 Some(1)                                                  TuplePattern
                      1                                                   Literal
                          2..=3                                           RangePattern
                          2                                               Literal
                              3                                           Literal
                                   (isize,•(Option<isize>,•isize))        TypeTuple
                                           (Option<isize>,•isize)         TypeTuple
                                            Option<isize>                 TypeCall                                                        */
	fn fun([a, ref mut b, ref xs @ .., ref c, d]: [X; 6]) {}                                                                              /*
    fn•fun([a,•ref•mut•b,•ref•xs•@•..,•ref•c,•d]:•[X;•6])•{}    FunctionDeclaration
           [a,•ref•mut•b,•ref•xs•@•..,•ref•c,•d]:•[X;•6]        FunctionParameterDeclaration
           [a,•ref•mut•b,•ref•xs•@•..,•ref•c,•d]                ArrayPattern
               ref•mut•b                                        PatternVariableDeclaration
                          ref•xs•@•..                           PatternVariableDeclaration
                                   ..                           RestPattern
                                       ref•c                    PatternVariableDeclaration
                                                  [X;•6]        TypeSizedArray
                                                      6         Literal                                                                   */
	fn foo(a @ [b, mid @ .., c]: [C; 3]) {}                                                                                               /*
    fn•foo(a•@•[b,•mid•@•..,•c]:•[C;•3])•{}    FunctionDeclaration
           a•@•[b,•mid•@•..,•c]:•[C;•3]        FunctionParameterDeclaration
           a•@•[b,•mid•@•..,•c]                PatternVariableDeclaration
               [b,•mid•@•..,•c]                ArrayPattern
                   mid•@•..                    PatternVariableDeclaration
                         ..                    RestPattern
                                 [C;•3]        TypeSizedArray
                                     3         Literal                                                                                    */

    let [..] = s;                                                                                                                         /*
    let•[..]•=•s;    LetVariableDeclaration
        [..]         ArrayPattern
         ..          RestPattern                                                                                                          */
    let [..] = s0;                                                                                                                        /*
    let•[..]•=•s0;    LetVariableDeclaration
        [..]          ArrayPattern
         ..           RestPattern                                                                                                         */
    let [..] = s1;                                                                                                                        /*
    let•[..]•=•s1;    LetVariableDeclaration
        [..]          ArrayPattern
         ..           RestPattern                                                                                                         */
    let [..] = s2;                                                                                                                        /*
    let•[..]•=•s2;    LetVariableDeclaration
        [..]          ArrayPattern
         ..           RestPattern                                                                                                         */
    let [_, ..] = s1;                                                                                                                     /*
    let•[_,•..]•=•s1;    LetVariableDeclaration
        [_,•..]          ArrayPattern
         _               WildcardPattern
            ..           RestPattern                                                                                                      */
    let [.., _] = s1;                                                                                                                     /*
    let•[..,•_]•=•s1;    LetVariableDeclaration
        [..,•_]          ArrayPattern
         ..              RestPattern
             _           WildcardPattern                                                                                                  */
    let [_, ..] = s2;                                                                                                                     /*
    let•[_,•..]•=•s2;    LetVariableDeclaration
        [_,•..]          ArrayPattern
         _               WildcardPattern
            ..           RestPattern                                                                                                      */
    let [.., _] = s2;                                                                                                                     /*
    let•[..,•_]•=•s2;    LetVariableDeclaration
        [..,•_]          ArrayPattern
         ..              RestPattern
             _           WildcardPattern                                                                                                  */
    let [_, _, ..] = s2;                                                                                                                  /*
    let•[_,•_,•..]•=•s2;    LetVariableDeclaration
        [_,•_,•..]          ArrayPattern
         _                  WildcardPattern
            _               WildcardPattern
               ..           RestPattern                                                                                                   */
    let [_, .., _] = s2;                                                                                                                  /*
    let•[_,•..,•_]•=•s2;    LetVariableDeclaration
        [_,•..,•_]          ArrayPattern
         _                  WildcardPattern
            ..              RestPattern
                _           WildcardPattern                                                                                               */
    let [.., _, _] = s2;                                                                                                                  /*
    let•[..,•_,•_]•=•s2;    LetVariableDeclaration
        [..,•_,•_]          ArrayPattern
         ..                 RestPattern
             _              WildcardPattern
                _           WildcardPattern                                                                                               */
	let Box { 0: _, .. }: Box<()>;                                                                                                        /*
    let•Box•{•0:•_,•..•}:•Box<()>;    LetVariableDeclaration
        Box•{•0:•_,•..•}              StructPattern
              0:•_                    StructPatternPropertyDestructured
              0                       Index
                 _                    WildcardPattern
                    ..                RestPattern
                          Box<()>     TypeCall
                              ()      TypeTuple                                                                                           */
	let Box { 1: _, .. }: Box<()>;                                                                                                        /*
    let•Box•{•1:•_,•..•}:•Box<()>;    LetVariableDeclaration
        Box•{•1:•_,•..•}              StructPattern
              1:•_                    StructPatternPropertyDestructured
              1                       Index
                 _                    WildcardPattern
                    ..                RestPattern
                          Box<()>     TypeCall
                              ()      TypeTuple                                                                                           */


	let [ref _x0, _x1, _, mut _x3, .., ref _x6, _x7] = arr;                                                                               /*
    let•[ref•_x0,•_x1,•_,•mut•_x3,•..,•ref•_x6,•_x7]•=•arr;    LetVariableDeclaration
        [ref•_x0,•_x1,•_,•mut•_x3,•..,•ref•_x6,•_x7]           ArrayPattern
         ref•_x0                                               PatternVariableDeclaration
                       _                                       WildcardPattern
                          mut•_x3                              PatternVariableDeclaration
                                   ..                          RestPattern
                                       ref•_x6                 PatternVariableDeclaration                                                 */
	let [ref _x0, ..] = arr;                                                                                                              /*
    let•[ref•_x0,•..]•=•arr;    LetVariableDeclaration
        [ref•_x0,•..]           ArrayPattern
         ref•_x0                PatternVariableDeclaration
                  ..            RestPattern                                                                                               */
	let [_x0, ..] = arr;                                                                                                                  /*
    let•[_x0,•..]•=•arr;    LetVariableDeclaration
        [_x0,•..]           ArrayPattern
              ..            RestPattern                                                                                                   */
	let (.., ref mut _x3) = tup;                                                                                                          /*
    let•(..,•ref•mut•_x3)•=•tup;    LetVariableDeclaration
        (..,•ref•mut•_x3)           TuplePattern
         ..                         RestPattern
             ref•mut•_x3            PatternVariableDeclaration                                                                            */
	let a @ [b, .., c] = [C, mk_c(), C];                                                                                                  /*
    let•a•@•[b,•..,•c]•=•[C,•mk_c(),•C];    LetVariableDeclaration
        a•@•[b,•..,•c]                      PatternVariableDeclaration
            [b,•..,•c]                      ArrayPattern
                ..                          RestPattern
                         [C,•mk_c(),•C]     ArrayLiteral
                             mk_c()         CallExpression                                                                                */
	let a @ [b, mid @ .., c] = [C, mk_c(), C];                                                                                            /*
    let•a•@•[b,•mid•@•..,•c]•=•[C,•mk_c(),•C];    LetVariableDeclaration
        a•@•[b,•mid•@•..,•c]                      PatternVariableDeclaration
            [b,•mid•@•..,•c]                      ArrayPattern
                mid•@•..                          PatternVariableDeclaration
                      ..                          RestPattern
                               [C,•mk_c(),•C]     ArrayLiteral
                                   mk_c()         CallExpression                                                                          */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 0
// Parsed Nodes: 336
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 1699 (25% re-reads)
// Unnecessary 'skip_whitespace()' calls: 188
// source: "../../samples/patterns/rest.rs"