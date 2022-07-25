fn q() {                                                                                                                                  /*
fn•q()•{↲    <FunctionDeclaration>                                                                                                        */
	if let 0..3 = 0 {}                                                                                                                    /*
    if•let•0..3•=•0•{}    ExpressionStatement, IfBlockExpression
       let•0..3•=•0       LetScrutinee
           0..3           RangePattern
           0              Literal
              3           Literal
                  0       Literal                                                                                                         */
	if let 0..Y = 0 {}                                                                                                                    /*
    if•let•0..Y•=•0•{}    ExpressionStatement, IfBlockExpression
       let•0..Y•=•0       LetScrutinee
           0..Y           RangePattern
           0              Literal
                  0       Literal                                                                                                         */
	if let X..3 = 0 {}                                                                                                                    /*
    if•let•X..3•=•0•{}    ExpressionStatement, IfBlockExpression
       let•X..3•=•0       LetScrutinee
           X..3           RangePattern
              3           Literal
                  0       Literal                                                                                                         */
	if let X..Y = 0 {}                                                                                                                    /*
    if•let•X..Y•=•0•{}    ExpressionStatement, IfBlockExpression
       let•X..Y•=•0       LetScrutinee
           X..Y           RangePattern
                  0       Literal                                                                                                         */

	if let 0..=3 = 0 {}                                                                                                                   /*
    if•let•0..=3•=•0•{}    ExpressionStatement, IfBlockExpression
       let•0..=3•=•0       LetScrutinee
           0..=3           RangePattern
           0               Literal
               3           Literal
                   0       Literal                                                                                                        */
	if let 0..=Y = 0 {}                                                                                                                   /*
    if•let•0..=Y•=•0•{}    ExpressionStatement, IfBlockExpression
       let•0..=Y•=•0       LetScrutinee
           0..=Y           RangePattern
           0               Literal
                   0       Literal                                                                                                        */
	if let X..=3 = 0 {}                                                                                                                   /*
    if•let•X..=3•=•0•{}    ExpressionStatement, IfBlockExpression
       let•X..=3•=•0       LetScrutinee
           X..=3           RangePattern
               3           Literal
                   0       Literal                                                                                                        */
	if let X..=Y = 0 {}                                                                                                                   /*
    if•let•X..=Y•=•0•{}    ExpressionStatement, IfBlockExpression
       let•X..=Y•=•0       LetScrutinee
           X..=Y           RangePattern
                   0       Literal                                                                                                        */

	if let 0...3 = 0 {}                                                                                                                   /*
    if•let•0...3•=•0•{}    ExpressionStatement, IfBlockExpression
       let•0...3•=•0       LetScrutinee
           0...3           RangePattern
           0               Literal
               3           Literal
                   0       Literal                                                                                                        */
	if let 0...Y = 0 {}                                                                                                                   /*
    if•let•0...Y•=•0•{}    ExpressionStatement, IfBlockExpression
       let•0...Y•=•0       LetScrutinee
           0...Y           RangePattern
           0               Literal
                   0       Literal                                                                                                        */
	if let X...3 = 0 {}                                                                                                                   /*
    if•let•X...3•=•0•{}    ExpressionStatement, IfBlockExpression
       let•X...3•=•0       LetScrutinee
           X...3           RangePattern
               3           Literal
                   0       Literal                                                                                                        */
	if let X...Y = 0 {}                                                                                                                   /*
    if•let•X...Y•=•0•{}    ExpressionStatement, IfBlockExpression
       let•X...Y•=•0       LetScrutinee
           X...Y           RangePattern
                   0       Literal                                                                                                        */

	if let 0.. = 0 {}                                                                                                                     /*
    if•let•0..•=•0•{}    ExpressionStatement, IfBlockExpression
       let•0..•=•0       LetScrutinee
           0..           RangePattern
           0             Literal
                 0       Literal                                                                                                          */
    if let X.. = 0 {}                                                                                                                     /*
    if•let•X..•=•0•{}    ExpressionStatement, IfBlockExpression
       let•X..•=•0       LetScrutinee
           X..           RangePattern
                 0       Literal                                                                                                          */

	if let ..0 = 0 {}                                                                                                                     /*
    if•let•..0•=•0•{}    ExpressionStatement, IfBlockExpression
       let•..0•=•0       LetScrutinee
           ..0           RangePattern
             0           Literal
                 0       Literal                                                                                                          */
	if let ..Y = 0 {}                                                                                                                     /*
    if•let•..Y•=•0•{}    ExpressionStatement, IfBlockExpression
       let•..Y•=•0       LetScrutinee
           ..Y           RangePattern
                 0       Literal                                                                                                          */

	if let ..=3 = 0 {}                                                                                                                    /*
    if•let•..=3•=•0•{}    ExpressionStatement, IfBlockExpression
       let•..=3•=•0       LetScrutinee
           ..=3           RangePattern
              3           Literal
                  0       Literal                                                                                                         */
    if let ..=Y = 0 {}                                                                                                                    /*
    if•let•..=Y•=•0•{}    ExpressionStatement, IfBlockExpression
       let•..=Y•=•0       LetScrutinee
           ..=Y           RangePattern
                  0       Literal                                                                                                         */

	let 0..1;                                                                                                                             /*
    let•0..1;    LetVariableDeclaration
        0..1     RangePattern
        0        Literal
           1     Literal                                                                                                                  */
	let 0...1;                                                                                                                            /*
    let•0...1;    LetVariableDeclaration
        0...1     RangePattern
        0         Literal
            1     Literal                                                                                                                 */
	let 0..=1;                                                                                                                            /*
    let•0..=1;    LetVariableDeclaration
        0..=1     RangePattern
        0         Literal
            1     Literal                                                                                                                 */

	let ..0;                                                                                                                              /*
    let•..0;    LetVariableDeclaration
        ..0     RangePattern
          0     Literal                                                                                                                   */
	let ..=0;                                                                                                                             /*
    let•..=0;    LetVariableDeclaration
        ..=0     RangePattern
           0     Literal                                                                                                                  */
	let 0..;                                                                                                                              /*
    let•0..;    LetVariableDeclaration
        0..     RangePattern
        0       Literal                                                                                                                   */

	for _ in [0..1] {}                                                                                                                    /*
    for•_•in•[0..1]•{}    ExpressionStatement, ForInBlockExpression
        _                 WildcardPattern
             [0..1]       ArrayLiteral
              0..1        RangeLiteral
              0           Literal
                 1        Literal                                                                                                         */
    for _ in [0..=1] {}                                                                                                                   /*
    for•_•in•[0..=1]•{}    ExpressionStatement, ForInBlockExpression
        _                  WildcardPattern
             [0..=1]       ArrayLiteral
              0..=1        RangeLiteral
              0            Literal
                  1        Literal                                                                                                        */
    for _ in [0..] {}                                                                                                                     /*
    for•_•in•[0..]•{}    ExpressionStatement, ForInBlockExpression
        _                WildcardPattern
             [0..]       ArrayLiteral
              0..        RangeLiteral
              0          Literal                                                                                                          */
    for _ in [..1] {}                                                                                                                     /*
    for•_•in•[..1]•{}    ExpressionStatement, ForInBlockExpression
        _                WildcardPattern
             [..1]       ArrayLiteral
              ..1        RangeLiteral
                1        Literal                                                                                                          */
    for _ in [..=1] {}                                                                                                                    /*
    for•_•in•[..=1]•{}    ExpressionStatement, ForInBlockExpression
        _                 WildcardPattern
             [..=1]       ArrayLiteral
              ..=1        RangeLiteral
                 1        Literal                                                                                                         */
    for _ in [b..c] {}                                                                                                                    /*
    for•_•in•[b..c]•{}    ExpressionStatement, ForInBlockExpression
        _                 WildcardPattern
             [b..c]       ArrayLiteral
              b..c        RangeLiteral                                                                                                    */
    for _ in [0..1, 2..3] {}                                                                                                              /*
    for•_•in•[0..1,•2..3]•{}    ExpressionStatement, ForInBlockExpression
        _                       WildcardPattern
             [0..1,•2..3]       ArrayLiteral
              0..1              RangeLiteral
              0                 Literal
                 1              Literal
                    2..3        RangeLiteral
                    2           Literal
                       3        Literal                                                                                                   */
    for _ in [0..=1] {}                                                                                                                   /*
    for•_•in•[0..=1]•{}    ExpressionStatement, ForInBlockExpression
        _                  WildcardPattern
             [0..=1]       ArrayLiteral
              0..=1        RangeLiteral
              0            Literal
                  1        Literal                                                                                                        */
	for _ in 0..2 {}                                                                                                                      /*
    for•_•in•0..2•{}    ExpressionStatement, ForInBlockExpression
        _               WildcardPattern
             0..2       RangeLiteral
             0          Literal
                2       Literal                                                                                                           */
	for _ in 0..=2 {}                                                                                                                     /*
    for•_•in•0..=2•{}    ExpressionStatement, ForInBlockExpression
        _                WildcardPattern
             0..=2       RangeLiteral
             0           Literal
                 2       Literal                                                                                                          */
	for _ in 0..=3 {}                                                                                                                     /*
    for•_•in•0..=3•{}    ExpressionStatement, ForInBlockExpression
        _                WildcardPattern
             0..=3       RangeLiteral
             0           Literal
                 3       Literal                                                                                                          */
	for _ in 0..=3 + 1 {}                                                                                                                 /*
    for•_•in•0..=3•+•1•{}    ExpressionStatement, ForInBlockExpression
        _                    WildcardPattern
             0..=3•+•1       RangeLiteral
             0               Literal
                 3•+•1       OperationExpression
                 3           Literal
                     1       Literal                                                                                                      */
	for _ in 0..=5 {}                                                                                                                     /*
    for•_•in•0..=5•{}    ExpressionStatement, ForInBlockExpression
        _                WildcardPattern
             0..=5       RangeLiteral
             0           Literal
                 5       Literal                                                                                                          */
	for _ in 0..=1 + 5 {}                                                                                                                 /*
    for•_•in•0..=1•+•5•{}    ExpressionStatement, ForInBlockExpression
        _                    WildcardPattern
             0..=1•+•5       RangeLiteral
             0               Literal
                 1•+•5       OperationExpression
                 1           Literal
                     5       Literal                                                                                                      */
	for _ in 1..=1 {}                                                                                                                     /*
    for•_•in•1..=1•{}    ExpressionStatement, ForInBlockExpression
        _                WildcardPattern
             1..=1       RangeLiteral
             1           Literal
                 1       Literal                                                                                                          */
	for _ in 1..=1 + 1 {}                                                                                                                 /*
    for•_•in•1..=1•+•1•{}    ExpressionStatement, ForInBlockExpression
        _                    WildcardPattern
             1..=1•+•1       RangeLiteral
             1               Literal
                 1•+•1       OperationExpression
                 1           Literal
                     1       Literal                                                                                                      */
	for _ in 0..13 + 13 {}                                                                                                                /*
    for•_•in•0..13•+•13•{}    ExpressionStatement, ForInBlockExpression
        _                     WildcardPattern
             0..13•+•13       RangeLiteral
             0                Literal
                13•+•13       OperationExpression
                13            Literal
                     13       Literal                                                                                                     */
	for _ in 0..=13 - 7 {}                                                                                                                /*
    for•_•in•0..=13•-•7•{}    ExpressionStatement, ForInBlockExpression
        _                     WildcardPattern
             0..=13•-•7       RangeLiteral
             0                Literal
                 13•-•7       OperationExpression
                 13           Literal
                      7       Literal                                                                                                     */
	for _ in 0..=f() {}                                                                                                                   /*
    for•_•in•0..=f()•{}    ExpressionStatement, ForInBlockExpression
        _                  WildcardPattern
             0..=f()       RangeLiteral
             0             Literal
                 f()       CallExpression                                                                                                 */
	for _ in 0..=(1 + f()) {}                                                                                                             /*
    for•_•in•0..=(1•+•f())•{}    ExpressionStatement, ForInBlockExpression
        _                        WildcardPattern
             0..=(1•+•f())       RangeLiteral
             0                   Literal
                  1•+•f()        OperationExpression
                  1              Literal
                      f()        CallExpression                                                                                           */
	let _ = ..11 - 1;                                                                                                                     /*
    let•_•=•..11•-•1;    LetVariableDeclaration
        _                WildcardPattern
            ..11•-•1     RangeLiteral
              11•-•1     OperationExpression
              11         Literal
                   1     Literal                                                                                                          */
	let _ = ..11;                                                                                                                         /*
    let•_•=•..11;    LetVariableDeclaration
        _            WildcardPattern
            ..11     RangeLiteral
              11     Literal                                                                                                              */
	let _ = ..11;                                                                                                                         /*
    let•_•=•..11;    LetVariableDeclaration
        _            WildcardPattern
            ..11     RangeLiteral
              11     Literal                                                                                                              */
	let _ = (1..=11);                                                                                                                     /*
    let•_•=•(1..=11);    LetVariableDeclaration
        _                WildcardPattern
             1..=11      RangeLiteral
             1           Literal
                 11      Literal                                                                                                          */
	let _ = ((f() + 1)..=f());                                                                                                            /*
    let•_•=•((f()•+•1)..=f());    LetVariableDeclaration
        _                         WildcardPattern
             (f()•+•1)..=f()      RangeLiteral
              f()•+•1             OperationExpression
              f()                 CallExpression
                    1             Literal
                         f()      CallExpression                                                                                          */
    for _ in 1..=ONE {}                                                                                                                   /*
    for•_•in•1..=ONE•{}    ExpressionStatement, ForInBlockExpression
        _                  WildcardPattern
             1..=ONE       RangeLiteral
             1             Literal                                                                                                        */

	let a = 0.0..1.1;                                                                                                                     /*
    let•a•=•0.0..1.1;    LetVariableDeclaration
            0.0..1.1     RangeLiteral
            0.0          Literal
                 1.1     Literal                                                                                                          */
	if let 2...0 = 3 {}                                                                                                                   /*
    if•let•2...0•=•3•{}    ExpressionStatement, IfBlockExpression
       let•2...0•=•3       LetScrutinee
           2...0           RangePattern
           2               Literal
               0           Literal
                   3       Literal                                                                                                        */
	if let 2..=0 = 3 {}                                                                                                                   /*
    if•let•2..=0•=•3•{}    ExpressionStatement, IfBlockExpression
       let•2..=0•=•3       LetScrutinee
           2..=0           RangePattern
           2               Literal
               0           Literal
                   3       Literal                                                                                                        */
	if let 2..0 = 3 {}                                                                                                                    /*
    if•let•2..0•=•3•{}    ExpressionStatement, IfBlockExpression
       let•2..0•=•3       LetScrutinee
           2..0           RangePattern
           2              Literal
              0           Literal
                  3       Literal                                                                                                         */
	if let ..0 = 3 {}                                                                                                                     /*
    if•let•..0•=•3•{}    ExpressionStatement, IfBlockExpression
       let•..0•=•3       LetScrutinee
           ..0           RangePattern
             0           Literal
                 3       Literal                                                                                                          */
	if let ..=0 = 3 {}                                                                                                                    /*
    if•let•..=0•=•3•{}    ExpressionStatement, IfBlockExpression
       let•..=0•=•3       LetScrutinee
           ..=0           RangePattern
              0           Literal
                  3       Literal                                                                                                         */
	if let 0.. = 5 {}                                                                                                                     /*
    if•let•0..•=•5•{}    ExpressionStatement, IfBlockExpression
       let•0..•=•5       LetScrutinee
           0..           RangePattern
           0             Literal
                 5       Literal                                                                                                          */
	if let 0..5 = 4 {}                                                                                                                    /*
    if•let•0..5•=•4•{}    ExpressionStatement, IfBlockExpression
       let•0..5•=•4       LetScrutinee
           0..5           RangePattern
           0              Literal
              5           Literal
                  4       Literal                                                                                                         */
	if let 0..=5 = 4 {}                                                                                                                   /*
    if•let•0..=5•=•4•{}    ExpressionStatement, IfBlockExpression
       let•0..=5•=•4       LetScrutinee
           0..=5           RangePattern
           0               Literal
               5           Literal
                   4       Literal                                                                                                        */
	if let -1..=0 | 2..3 | 4 = x {}                                                                                                       /*
    if•let•-1..=0•|•2..3•|•4•=•x•{}    ExpressionStatement, IfBlockExpression
       let•-1..=0•|•2..3•|•4•=•x       LetScrutinee
           -1..=0•|•2..3•|•4           UnionPattern
           -1..=0                      RangePattern
           -1                          MinusPattern
            1                          Literal
                0                      Literal
                    2..3               RangePattern
                    2                  Literal
                       3               Literal
                           4           Literal                                                                                            */
	for x in -9 + 1..=(9 - 2) {}                                                                                                          /*
    for•x•in•-9•+•1..=(9•-•2)•{}    ExpressionStatement, ForInBlockExpression
             -9•+•1..=(9•-•2)       RangeLiteral
             -9•+•1                 OperationExpression
             -9                     MinusExpression
              9                     Literal
                  1                 Literal
                       9•-•2        OperationExpression
                       9            Literal
                           2        Literal                                                                                               */
	if let [3..=14, ..] = xs {}                                                                                                           /*
    if•let•[3..=14,•..]•=•xs•{}    ExpressionStatement, IfBlockExpression
       let•[3..=14,•..]•=•xs       LetScrutinee
           [3..=14,•..]            ArrayPattern
            3..=14                 RangePattern
            3                      Literal
                14                 Literal
                    ..             RestPattern                                                                                            */
	match 0 {                                                                                                                             /*
    match•0•{↲    <ExpressionStatement>, <MatchExpression>
          0       Literal                                                                                                                 */
		X.. | 0.. | 'a'.. | 0.0f32.. => {}                                                                                                /*
        X..•|•0..•|•'a'..•|•0.0f32..•=>•{}    MatchExpressionCase
        X..•|•0..•|•'a'..•|•0.0f32..          UnionPattern
        X..                                   RangePattern
              0..                             RangePattern
              0                               Literal
                    'a'..                     RangePattern
                    'a'                       Literal
                            0.0f32..          RangePattern
                            0.0f32            Literal
                               f32            Identifier
                                        {}    BlockExpression                                                                             */
        ..=X | ..X => {}                                                                                                                  /*
        ..=X•|•..X•=>•{}    MatchExpressionCase
        ..=X•|•..X          UnionPattern
        ..=X                RangePattern
               ..X          RangePattern
                      {}    BlockExpression                                                                                               */
        ..=0 | ..0 => {}                                                                                                                  /*
        ..=0•|•..0•=>•{}    MatchExpressionCase
        ..=0•|•..0          UnionPattern
        ..=0                RangePattern
           0                Literal
               ..0          RangePattern
                 0          Literal
                      {}    BlockExpression                                                                                               */
        ..='a' | ..'a' => {}                                                                                                              /*
        ..='a'•|•..'a'•=>•{}    MatchExpressionCase
        ..='a'•|•..'a'          UnionPattern
        ..='a'                  RangePattern
           'a'                  Literal
                 ..'a'          RangePattern
                   'a'          Literal
                          {}    BlockExpression                                                                                           */
        ..=0.0f32 | ..0.0f32 => {}                                                                                                        /*
        ..=0.0f32•|•..0.0f32•=>•{}    MatchExpressionCase
        ..=0.0f32•|•..0.0f32          UnionPattern
        ..=0.0f32                     RangePattern
           0.0f32                     Literal
              f32                     Identifier
                    ..0.0f32          RangePattern
                      0.0f32          Literal
                         f32          Identifier
                                {}    BlockExpression                                                                                     */
		(..a) => {}                                                                                                                       /*
        (..a)•=>•{}    MatchExpressionCase
         ..a           RangePattern
                 {}    BlockExpression                                                                                                    */
		(a..) => {}                                                                                                                       /*
        (a..)•=>•{}    MatchExpressionCase
         a..           RangePattern
                 {}    BlockExpression                                                                                                    */
		1 | -3..0 => {}                                                                                                                   /*
        1•|•-3..0•=>•{}    MatchExpressionCase
        1•|•-3..0          UnionPattern
        1                  Literal
            -3..0          RangePattern
            -3             MinusPattern
             3             Literal
                0          Literal
                     {}    BlockExpression                                                                                                */
		y @ (0..5 | 6) => {}                                                                                                              /*
        y•@•(0..5•|•6)•=>•{}    MatchExpressionCase
        y•@•(0..5•|•6)          PatternVariableDeclaration
             0..5•|•6           UnionPattern
             0..5               RangePattern
             0                  Literal
                5               Literal
                    6           Literal
                          {}    BlockExpression                                                                                           */
		y @ -5.. => {}                                                                                                                    /*
        y•@•-5..•=>•{}    MatchExpressionCase
        y•@•-5..          PatternVariableDeclaration
            -5..          RangePattern
            -5            MinusPattern
             5            Literal
                    {}    BlockExpression                                                                                                 */
		y @ ..-7 => {}                                                                                                                    /*
        y•@•..-7•=>•{}    MatchExpressionCase
        y•@•..-7          PatternVariableDeclaration
            ..-7          RangePattern
              -7          MinusPattern
               7          Literal
                    {}    BlockExpression                                                                                                 */
		box 0...9 => {}                                                                                                                   /*
        box•0...9•=>•{}    MatchExpressionCase
        box•0...9          BoxPattern
            0...9          RangePattern
            0              Literal
                9          Literal
                     {}    BlockExpression                                                                                                */
		box 10..=15 => {}                                                                                                                 /*
        box•10..=15•=>•{}    MatchExpressionCase
        box•10..=15          BoxPattern
            10..=15          RangePattern
            10               Literal
                 15          Literal
                       {}    BlockExpression                                                                                              */
		box (16..=20) => {}                                                                                                               /*
        box•(16..=20)•=>•{}    MatchExpressionCase
        box•(16..=20)          BoxPattern
             16..=20           RangePattern
             16                Literal
                  20           Literal
                         {}    BlockExpression                                                                                            */
		0..1 => {}                                                                                                                        /*
        0..1•=>•{}    MatchExpressionCase
        0..1          RangePattern
        0             Literal
           1          Literal
                {}    BlockExpression                                                                                                     */
		(0)..1 => {}                                                                                                                      /*
        (0)..1•=>•{}    MatchExpressionCase
        (0)..1          RangePattern
         0              Literal
             1          Literal
                  {}    BlockExpression                                                                                                   */
		0..(1) => {}                                                                                                                      /*
        0..(1)•=>•{}    MatchExpressionCase
        0..(1)          RangePattern
        0               Literal
            1           Literal
                  {}    BlockExpression                                                                                                   */
	}                                                                                                                                     /*
   ╚}    </ExpressionStatement>, </MatchExpression>                                                                                       */
	[                                                                                                                                     /*
    [↲    <ExpressionStatement>, <ArrayLiteral>                                                                                           */
		0..1,                                                                                                                             /*
        0..1    RangeLiteral
        0       Literal
           1    Literal                                                                                                                   */
		(0)..1,                                                                                                                           /*
        (0)..1    RangeLiteral
         0        Literal
             1    Literal                                                                                                                 */
		0..(1),                                                                                                                           /*
        0..(1)    RangeLiteral
        0         Literal
            1     Literal                                                                                                                 */
		(0)..(1),                                                                                                                         /*
        (0)..(1)    RangeLiteral
         0          Literal
              1     Literal                                                                                                               */
		1..,                                                                                                                              /*
        1..    RangeLiteral
        1      Literal                                                                                                                    */
		..,                                                                                                                               /*
        ..    RangeLiteral                                                                                                                */
		0..=1,                                                                                                                            /*
        0..=1    RangeLiteral
        0        Literal
            1    Literal                                                                                                                  */
		0 ..=1,                                                                                                                           /*
        0•..=1    RangeLiteral
        0         Literal
             1    Literal                                                                                                                 */
		0 ..=1,                                                                                                                           /*
        0•..=1    RangeLiteral
        0         Literal
             1    Literal                                                                                                                 */
		// 0...1,
        //•0...1,    Comment
		// &0...9,
        //•&0...9,    Comment
		&10..=15,                                                                                                                         /*
        &10..=15    RangeLiteral
        &10         ReferenceExpression
         10         Literal
              15    Literal                                                                                                               */
		// box 0...9,
        //•box•0...9,    Comment
		box 0..=9,                                                                                                                        /*
        box•0..=9    RangeLiteral
        box•0        BoxExpression
            0        Literal
                9    Literal                                                                                                              */
		..1,                                                                                                                              /*
        ..1    RangeLiteral
          1    Literal                                                                                                                    */
		..=1,                                                                                                                             /*
        ..=1    RangeLiteral
           1    Literal                                                                                                                   */
		0u32..10i32,                                                                                                                      /*
        0u32..10i32    RangeLiteral
        0u32           Literal
         u32           Identifier
              10i32    Literal
                i32    Identifier                                                                                                         */
		*a..                                                                                                                              /*
        *a..    RangeLiteral
        *a      DereferenceExpression                                                                                                     */
	];                                                                                                                                    /*
   ╚];    </ExpressionStatement>
   ╚]     </ArrayLiteral>                                                                                                                 */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
fn f() { a.. }                                                                                                                            /*
fn•f()•{•a..•}    FunctionDeclaration
         a..      ExpressionStatement, RangeLiteral                                                                                       */
fn f() { a..b }                                                                                                                           /*
fn•f()•{•a..b•}    FunctionDeclaration
         a..b      ExpressionStatement, RangeLiteral                                                                                      */
fn f() { a()..b() }                                                                                                                       /*
fn•f()•{•a()..b()•}    FunctionDeclaration
         a()..b()      ExpressionStatement, RangeLiteral
         a()           CallExpression
              b()      CallExpression                                                                                                     */
fn foo(-128..=127: i8) {}                                                                                                                 /*
fn•foo(-128..=127:•i8)•{}    FunctionDeclaration
       -128..=127:•i8        FunctionParameterDeclaration
       -128..=127            RangePattern
       -128                  MinusPattern
        128                  Literal
              127            Literal                                                                                                      */

// Discarded Nodes: 14
// Parsed Nodes: 605
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 2563 (33% re-reads)
// Unnecessary 'skip_whitespace()' calls: 92
// source: "../../samples/expressions/range.rs"