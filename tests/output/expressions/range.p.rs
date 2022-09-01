fn q() {                                                                                                                                  /*
fn•q()•{↲    <Program>
fn•q()•{↲    <Program.ast{dk: "None"}>
fn•q()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	if let 0..3 = 0 {}                                                                                                                    /*
	if•let•0..3•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0..3•=•0       LetScrutinee
	       0..3           RangePattern{!last}
	       0              Literal{kind: Integer}
	          3           Literal{kind: Integer}
	              0       Literal{kind: Integer}
	                {}    IfBlockExpression.body{dk: "{}"}                                                                                */
	if let 0..Y = 0 {}                                                                                                                    /*
	if•let•0..Y•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0..Y•=•0       LetScrutinee
	       0..Y           RangePattern{!last}
	       0              Literal{kind: Integer}
	              0       Literal{kind: Integer}
	                {}    IfBlockExpression.body{dk: "{}"}                                                                                */
	if let X..3 = 0 {}                                                                                                                    /*
	if•let•X..3•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•X..3•=•0       LetScrutinee
	       X..3           RangePattern{!last}
	          3           Literal{kind: Integer}
	              0       Literal{kind: Integer}
	                {}    IfBlockExpression.body{dk: "{}"}                                                                                */
	if let X..Y = 0 {}                                                                                                                    /*
	if•let•X..Y•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•X..Y•=•0       LetScrutinee
	       X..Y           RangePattern{!last}
	              0       Literal{kind: Integer}
	                {}    IfBlockExpression.body{dk: "{}"}                                                                                */

	if let 0..=3 = 0 {}                                                                                                                   /*
	if•let•0..=3•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0..=3•=•0       LetScrutinee
	       0..=3           RangePattern{last}
	       0               Literal{kind: Integer}
	           3           Literal{kind: Integer}
	               0       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let 0..=Y = 0 {}                                                                                                                   /*
	if•let•0..=Y•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0..=Y•=•0       LetScrutinee
	       0..=Y           RangePattern{last}
	       0               Literal{kind: Integer}
	               0       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let X..=3 = 0 {}                                                                                                                   /*
	if•let•X..=3•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•X..=3•=•0       LetScrutinee
	       X..=3           RangePattern{last}
	           3           Literal{kind: Integer}
	               0       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let X..=Y = 0 {}                                                                                                                   /*
	if•let•X..=Y•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•X..=Y•=•0       LetScrutinee
	       X..=Y           RangePattern{last}
	               0       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */

	if let 0...3 = 0 {}                                                                                                                   /*
	if•let•0...3•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0...3•=•0       LetScrutinee
	       0...3           RangePattern{last}
	       0               Literal{kind: Integer}
	           3           Literal{kind: Integer}
	               0       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let 0...Y = 0 {}                                                                                                                   /*
	if•let•0...Y•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0...Y•=•0       LetScrutinee
	       0...Y           RangePattern{last}
	       0               Literal{kind: Integer}
	               0       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let X...3 = 0 {}                                                                                                                   /*
	if•let•X...3•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•X...3•=•0       LetScrutinee
	       X...3           RangePattern{last}
	           3           Literal{kind: Integer}
	               0       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let X...Y = 0 {}                                                                                                                   /*
	if•let•X...Y•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•X...Y•=•0       LetScrutinee
	       X...Y           RangePattern{last}
	               0       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */

	if let 0.. = 0 {}                                                                                                                     /*
	if•let•0..•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0..•=•0       LetScrutinee
	       0..           RangePattern{!last}
	       0             Literal{kind: Integer}
	             0       Literal{kind: Integer}
	               {}    IfBlockExpression.body{dk: "{}"}                                                                                 */
    if let X.. = 0 {}                                                                                                                     /*
    if•let•X..•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
       let•X..•=•0       LetScrutinee
           X..           RangePattern{!last}
                 0       Literal{kind: Integer}
                   {}    IfBlockExpression.body{dk: "{}"}                                                                                 */

	if let ..0 = 0 {}                                                                                                                     /*
	if•let•..0•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•..0•=•0       LetScrutinee
	       ..0           RangePattern{!last}
	         0           Literal{kind: Integer}
	             0       Literal{kind: Integer}
	               {}    IfBlockExpression.body{dk: "{}"}                                                                                 */
	if let ..Y = 0 {}                                                                                                                     /*
	if•let•..Y•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•..Y•=•0       LetScrutinee
	       ..Y           RangePattern{!last}
	             0       Literal{kind: Integer}
	               {}    IfBlockExpression.body{dk: "{}"}                                                                                 */

	if let ..=3 = 0 {}                                                                                                                    /*
	if•let•..=3•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•..=3•=•0       LetScrutinee
	       ..=3           RangePattern{last}
	          3           Literal{kind: Integer}
	              0       Literal{kind: Integer}
	                {}    IfBlockExpression.body{dk: "{}"}                                                                                */
    if let ..=Y = 0 {}                                                                                                                    /*
    if•let•..=Y•=•0•{}    ExpressionStatement{!semi}, IfBlockExpression
       let•..=Y•=•0       LetScrutinee
           ..=Y           RangePattern{last}
                  0       Literal{kind: Integer}
                    {}    IfBlockExpression.body{dk: "{}"}                                                                                */

	let 0..1;                                                                                                                             /*
	let•0..1;    LetVariableDeclaration
	    0..1     RangePattern{!last}
	    0        Literal{kind: Integer}
	       1     Literal{kind: Integer}                                                                                                   */
	let 0...1;                                                                                                                            /*
	let•0...1;    LetVariableDeclaration
	    0...1     RangePattern{last}
	    0         Literal{kind: Integer}
	        1     Literal{kind: Integer}                                                                                                  */
	let 0..=1;                                                                                                                            /*
	let•0..=1;    LetVariableDeclaration
	    0..=1     RangePattern{last}
	    0         Literal{kind: Integer}
	        1     Literal{kind: Integer}                                                                                                  */
	
	let ..0;                                                                                                                              /*
	let•..0;    LetVariableDeclaration
	    ..0     RangePattern{!last}
	      0     Literal{kind: Integer}                                                                                                    */
	let ..=0;                                                                                                                             /*
	let•..=0;    LetVariableDeclaration
	    ..=0     RangePattern{last}
	       0     Literal{kind: Integer}                                                                                                   */
	let 0..;                                                                                                                              /*
	let•0..;    LetVariableDeclaration
	    0..     RangePattern{!last}
	    0       Literal{kind: Integer}                                                                                                    */

	for _ in [0..1] {}                                                                                                                    /*
	for•_•in•[0..1]•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                 WildcardPattern
	         [0..1]       ArrayLiteral
	          0..1        RangeLiteral{!last}
	          0           Literal{kind: Integer}
	             1        Literal{kind: Integer}
	                {}    ForInBlockExpression.body{dk: "{}"}                                                                             */
    for _ in [0..=1] {}                                                                                                                   /*
    for•_•in•[0..=1]•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                  WildcardPattern
             [0..=1]       ArrayLiteral
              0..=1        RangeLiteral{last}
              0            Literal{kind: Integer}
                  1        Literal{kind: Integer}
                     {}    ForInBlockExpression.body{dk: "{}"}                                                                            */
    for _ in [0..] {}                                                                                                                     /*
    for•_•in•[0..]•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                WildcardPattern
             [0..]       ArrayLiteral
              0..        RangeLiteral{!last}
              0          Literal{kind: Integer}
                   {}    ForInBlockExpression.body{dk: "{}"}                                                                              */
    for _ in [..1] {}                                                                                                                     /*
    for•_•in•[..1]•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                WildcardPattern
             [..1]       ArrayLiteral
              ..1        RangeLiteral{!last}
                1        Literal{kind: Integer}
                   {}    ForInBlockExpression.body{dk: "{}"}                                                                              */
    for _ in [..=1] {}                                                                                                                    /*
    for•_•in•[..=1]•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                 WildcardPattern
             [..=1]       ArrayLiteral
              ..=1        RangeLiteral{last}
                 1        Literal{kind: Integer}
                    {}    ForInBlockExpression.body{dk: "{}"}                                                                             */
    for _ in [b..c] {}                                                                                                                    /*
    for•_•in•[b..c]•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                 WildcardPattern
             [b..c]       ArrayLiteral
              b..c        RangeLiteral{!last}
                    {}    ForInBlockExpression.body{dk: "{}"}                                                                             */
    for _ in [0..1, 2..3] {}                                                                                                              /*
    for•_•in•[0..1,•2..3]•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                       WildcardPattern
             [0..1,•2..3]       ArrayLiteral
              0..1              RangeLiteral{!last}
              0                 Literal{kind: Integer}
                 1              Literal{kind: Integer}
                    2..3        RangeLiteral{!last}
                    2           Literal{kind: Integer}
                       3        Literal{kind: Integer}
                          {}    ForInBlockExpression.body{dk: "{}"}                                                                       */
    for _ in [0..=1] {}                                                                                                                   /*
    for•_•in•[0..=1]•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                  WildcardPattern
             [0..=1]       ArrayLiteral
              0..=1        RangeLiteral{last}
              0            Literal{kind: Integer}
                  1        Literal{kind: Integer}
                     {}    ForInBlockExpression.body{dk: "{}"}                                                                            */
	for _ in 0..2 {}                                                                                                                      /*
	for•_•in•0..2•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _               WildcardPattern
	         0..2       RangeLiteral{!last}
	         0          Literal{kind: Integer}
	            2       Literal{kind: Integer}
	              {}    ForInBlockExpression.body{dk: "{}"}                                                                               */
	for _ in 0..=2 {}                                                                                                                     /*
	for•_•in•0..=2•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                WildcardPattern
	         0..=2       RangeLiteral{last}
	         0           Literal{kind: Integer}
	             2       Literal{kind: Integer}
	               {}    ForInBlockExpression.body{dk: "{}"}                                                                              */
	for _ in 0..=3 {}                                                                                                                     /*
	for•_•in•0..=3•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                WildcardPattern
	         0..=3       RangeLiteral{last}
	         0           Literal{kind: Integer}
	             3       Literal{kind: Integer}
	               {}    ForInBlockExpression.body{dk: "{}"}                                                                              */
	for _ in 0..=3 + 1 {}                                                                                                                 /*
	for•_•in•0..=3•+•1•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                    WildcardPattern
	         0..=3•+•1       RangeLiteral{last}
	         0               Literal{kind: Integer}
	             3•+•1       OperationExpression{tk: "+"}
	             3           Literal{kind: Integer}
	                 1       Literal{kind: Integer}
	                   {}    ForInBlockExpression.body{dk: "{}"}                                                                          */
	for _ in 0..=5 {}                                                                                                                     /*
	for•_•in•0..=5•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                WildcardPattern
	         0..=5       RangeLiteral{last}
	         0           Literal{kind: Integer}
	             5       Literal{kind: Integer}
	               {}    ForInBlockExpression.body{dk: "{}"}                                                                              */
	for _ in 0..=1 + 5 {}                                                                                                                 /*
	for•_•in•0..=1•+•5•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                    WildcardPattern
	         0..=1•+•5       RangeLiteral{last}
	         0               Literal{kind: Integer}
	             1•+•5       OperationExpression{tk: "+"}
	             1           Literal{kind: Integer}
	                 5       Literal{kind: Integer}
	                   {}    ForInBlockExpression.body{dk: "{}"}                                                                          */
	for _ in 1..=1 {}                                                                                                                     /*
	for•_•in•1..=1•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                WildcardPattern
	         1..=1       RangeLiteral{last}
	         1           Literal{kind: Integer}
	             1       Literal{kind: Integer}
	               {}    ForInBlockExpression.body{dk: "{}"}                                                                              */
	for _ in 1..=1 + 1 {}                                                                                                                 /*
	for•_•in•1..=1•+•1•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                    WildcardPattern
	         1..=1•+•1       RangeLiteral{last}
	         1               Literal{kind: Integer}
	             1•+•1       OperationExpression{tk: "+"}
	             1           Literal{kind: Integer}
	                 1       Literal{kind: Integer}
	                   {}    ForInBlockExpression.body{dk: "{}"}                                                                          */
	for _ in 0..13 + 13 {}                                                                                                                /*
	for•_•in•0..13•+•13•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                     WildcardPattern
	         0..13•+•13       RangeLiteral{!last}
	         0                Literal{kind: Integer}
	            13•+•13       OperationExpression{tk: "+"}
	            13            Literal{kind: Integer}
	                 13       Literal{kind: Integer}
	                    {}    ForInBlockExpression.body{dk: "{}"}                                                                         */
	for _ in 0..=13 - 7 {}                                                                                                                /*
	for•_•in•0..=13•-•7•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                     WildcardPattern
	         0..=13•-•7       RangeLiteral{last}
	         0                Literal{kind: Integer}
	             13•-•7       OperationExpression{tk: "-"}
	             13           Literal{kind: Integer}
	                  7       Literal{kind: Integer}
	                    {}    ForInBlockExpression.body{dk: "{}"}                                                                         */
	for _ in 0..=f() {}                                                                                                                   /*
	for•_•in•0..=f()•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                  WildcardPattern
	         0..=f()       RangeLiteral{last}
	         0             Literal{kind: Integer}
	             f()       CallExpression
	              ()       CallExpression.arguments{dk: "()"}
	                 {}    ForInBlockExpression.body{dk: "{}"}                                                                            */
	for _ in 0..=(1 + f()) {}                                                                                                             /*
	for•_•in•0..=(1•+•f())•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                        WildcardPattern
	         0..=(1•+•f())       RangeLiteral{last}
	         0                   Literal{kind: Integer}
	              1•+•f()        OperationExpression{tk: "+"}
	              1              Literal{kind: Integer}
	                  f()        CallExpression
	                   ()        CallExpression.arguments{dk: "()"}
	                       {}    ForInBlockExpression.body{dk: "{}"}                                                                      */
	let _ = ..11 - 1;                                                                                                                     /*
	let•_•=•..11•-•1;    LetVariableDeclaration
	    _                WildcardPattern
	        ..11•-•1     RangeLiteral{!last}
	          11•-•1     OperationExpression{tk: "-"}
	          11         Literal{kind: Integer}
	               1     Literal{kind: Integer}                                                                                           */
	let _ = ..11;                                                                                                                         /*
	let•_•=•..11;    LetVariableDeclaration
	    _            WildcardPattern
	        ..11     RangeLiteral{!last}
	          11     Literal{kind: Integer}                                                                                               */
	let _ = ..11;                                                                                                                         /*
	let•_•=•..11;    LetVariableDeclaration
	    _            WildcardPattern
	        ..11     RangeLiteral{!last}
	          11     Literal{kind: Integer}                                                                                               */
	let _ = (1..=11);                                                                                                                     /*
	let•_•=•(1..=11);    LetVariableDeclaration
	    _                WildcardPattern
	         1..=11      RangeLiteral{last}
	         1           Literal{kind: Integer}
	             11      Literal{kind: Integer}                                                                                           */
	let _ = ((f() + 1)..=f());                                                                                                            /*
	let•_•=•((f()•+•1)..=f());    LetVariableDeclaration
	    _                         WildcardPattern
	         (f()•+•1)..=f()      RangeLiteral{last}
	          f()•+•1             OperationExpression{tk: "+"}
	          f()                 CallExpression
	           ()                 CallExpression.arguments{dk: "()"}
	                1             Literal{kind: Integer}
	                     f()      CallExpression
	                      ()      CallExpression.arguments{dk: "()"}                                                                      */
    for _ in 1..=ONE {}                                                                                                                   /*
    for•_•in•1..=ONE•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                  WildcardPattern
             1..=ONE       RangeLiteral{last}
             1             Literal{kind: Integer}
                     {}    ForInBlockExpression.body{dk: "{}"}                                                                            */

	let a = 0.0..1.1;                                                                                                                     /*
	let•a•=•0.0..1.1;    LetVariableDeclaration
	        0.0..1.1     RangeLiteral{!last}
	        0.0          Literal{kind: Float}
	             1.1     Literal{kind: Float}                                                                                             */
	if let 2...0 = 3 {}                                                                                                                   /*
	if•let•2...0•=•3•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•2...0•=•3       LetScrutinee
	       2...0           RangePattern{last}
	       2               Literal{kind: Integer}
	           0           Literal{kind: Integer}
	               3       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let 2..=0 = 3 {}                                                                                                                   /*
	if•let•2..=0•=•3•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•2..=0•=•3       LetScrutinee
	       2..=0           RangePattern{last}
	       2               Literal{kind: Integer}
	           0           Literal{kind: Integer}
	               3       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let 2..0 = 3 {}                                                                                                                    /*
	if•let•2..0•=•3•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•2..0•=•3       LetScrutinee
	       2..0           RangePattern{!last}
	       2              Literal{kind: Integer}
	          0           Literal{kind: Integer}
	              3       Literal{kind: Integer}
	                {}    IfBlockExpression.body{dk: "{}"}                                                                                */
	if let ..0 = 3 {}                                                                                                                     /*
	if•let•..0•=•3•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•..0•=•3       LetScrutinee
	       ..0           RangePattern{!last}
	         0           Literal{kind: Integer}
	             3       Literal{kind: Integer}
	               {}    IfBlockExpression.body{dk: "{}"}                                                                                 */
	if let ..=0 = 3 {}                                                                                                                    /*
	if•let•..=0•=•3•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•..=0•=•3       LetScrutinee
	       ..=0           RangePattern{last}
	          0           Literal{kind: Integer}
	              3       Literal{kind: Integer}
	                {}    IfBlockExpression.body{dk: "{}"}                                                                                */
	if let 0.. = 5 {}                                                                                                                     /*
	if•let•0..•=•5•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0..•=•5       LetScrutinee
	       0..           RangePattern{!last}
	       0             Literal{kind: Integer}
	             5       Literal{kind: Integer}
	               {}    IfBlockExpression.body{dk: "{}"}                                                                                 */
	if let 0..5 = 4 {}                                                                                                                    /*
	if•let•0..5•=•4•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0..5•=•4       LetScrutinee
	       0..5           RangePattern{!last}
	       0              Literal{kind: Integer}
	          5           Literal{kind: Integer}
	              4       Literal{kind: Integer}
	                {}    IfBlockExpression.body{dk: "{}"}                                                                                */
	if let 0..=5 = 4 {}                                                                                                                   /*
	if•let•0..=5•=•4•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0..=5•=•4       LetScrutinee
	       0..=5           RangePattern{last}
	       0               Literal{kind: Integer}
	           5           Literal{kind: Integer}
	               4       Literal{kind: Integer}
	                 {}    IfBlockExpression.body{dk: "{}"}                                                                               */
	if let -1..=0 | 2..3 | 4 = x {}                                                                                                       /*
	if•let•-1..=0•|•2..3•|•4•=•x•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•-1..=0•|•2..3•|•4•=•x       LetScrutinee
	       -1..=0•|•2..3•|•4           UnionPattern
	       -1..=0                      RangePattern{last}
	       -1                          MinusPattern
	        1                          Literal{kind: Integer}
	            0                      Literal{kind: Integer}
	                2..3               RangePattern{!last}
	                2                  Literal{kind: Integer}
	                   3               Literal{kind: Integer}
	                       4           Literal{kind: Integer}
	                             {}    IfBlockExpression.body{dk: "{}"}                                                                   */
	for x in -9 + 1..=(9 - 2) {}                                                                                                          /*
	for•x•in•-9•+•1..=(9•-•2)•{}    ExpressionStatement{!semi}, ForInBlockExpression
	         -9•+•1..=(9•-•2)       RangeLiteral{last}
	         -9•+•1                 OperationExpression{tk: "+"}
	         -9                     MinusExpression
	          9                     Literal{kind: Integer}
	              1                 Literal{kind: Integer}
	                   9•-•2        OperationExpression{tk: "-"}
	                   9            Literal{kind: Integer}
	                       2        Literal{kind: Integer}
	                          {}    ForInBlockExpression.body{dk: "{}"}                                                                   */
	if let [3..=14, ..] = xs {}                                                                                                           /*
	if•let•[3..=14,•..]•=•xs•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•[3..=14,•..]•=•xs       LetScrutinee
	       [3..=14,•..]            ArrayPattern
	        3..=14                 RangePattern{last}
	        3                      Literal{kind: Integer}
	            14                 Literal{kind: Integer}
	                ..             RestPattern
	                         {}    IfBlockExpression.body{dk: "{}"}                                                                       */
	match 0 {                                                                                                                             /*
	match•0•{↲    <ExpressionStatement{!semi}>
	match•0•{↲    <MatchExpression>
	      0       Literal{kind: Integer}
	        {↲    <MatchExpression.cases{dk: "{}"}>                                                                                       */
		X.. | 0.. | 'a'.. | 0.0f32.. => {}                                                                                                /*
		X..•|•0..•|•'a'..•|•0.0f32..•=>•{}    MatchExpressionCase
		X..•|•0..•|•'a'..•|•0.0f32..          UnionPattern
		X..                                   RangePattern{!last}
		      0..                             RangePattern{!last}
		      0                               Literal{kind: Integer}
		            'a'..                     RangePattern{!last}
		            'a'                       Literal{kind: Char}
		                    0.0f32..          RangePattern{!last}
		                    0.0f32            Literal{kind: Float}
		                       f32            Identifier
		                                {}    BlockExpression                                                                             */
        ..=X | ..X => {}                                                                                                                  /*
        ..=X•|•..X•=>•{}    MatchExpressionCase
        ..=X•|•..X          UnionPattern
        ..=X                RangePattern{last}
               ..X          RangePattern{!last}
                      {}    BlockExpression                                                                                               */
        ..=0 | ..0 => {}                                                                                                                  /*
        ..=0•|•..0•=>•{}    MatchExpressionCase
        ..=0•|•..0          UnionPattern
        ..=0                RangePattern{last}
           0                Literal{kind: Integer}
               ..0          RangePattern{!last}
                 0          Literal{kind: Integer}
                      {}    BlockExpression                                                                                               */
        ..='a' | ..'a' => {}                                                                                                              /*
        ..='a'•|•..'a'•=>•{}    MatchExpressionCase
        ..='a'•|•..'a'          UnionPattern
        ..='a'                  RangePattern{last}
           'a'                  Literal{kind: Char}
                 ..'a'          RangePattern{!last}
                   'a'          Literal{kind: Char}
                          {}    BlockExpression                                                                                           */
        ..=0.0f32 | ..0.0f32 => {}                                                                                                        /*
        ..=0.0f32•|•..0.0f32•=>•{}    MatchExpressionCase
        ..=0.0f32•|•..0.0f32          UnionPattern
        ..=0.0f32                     RangePattern{last}
           0.0f32                     Literal{kind: Float}
              f32                     Identifier
                    ..0.0f32          RangePattern{!last}
                      0.0f32          Literal{kind: Float}
                         f32          Identifier
                                {}    BlockExpression                                                                                     */
		(..a) => {}                                                                                                                       /*
		(..a)•=>•{}    MatchExpressionCase
		 ..a           RangePattern{!last}
		         {}    BlockExpression                                                                                                    */
		(a..) => {}                                                                                                                       /*
		(a..)•=>•{}    MatchExpressionCase
		 a..           RangePattern{!last}
		         {}    BlockExpression                                                                                                    */
		1 | -3..0 => {}                                                                                                                   /*
		1•|•-3..0•=>•{}    MatchExpressionCase
		1•|•-3..0          UnionPattern
		1                  Literal{kind: Integer}
		    -3..0          RangePattern{!last}
		    -3             MinusPattern
		     3             Literal{kind: Integer}
		        0          Literal{kind: Integer}
		             {}    BlockExpression                                                                                                */
		y @ (0..5 | 6) => {}                                                                                                              /*
		y•@•(0..5•|•6)•=>•{}    MatchExpressionCase
		y•@•(0..5•|•6)          PatternVariableDeclaration{!ref, !mut}
		     0..5•|•6           UnionPattern
		     0..5               RangePattern{!last}
		     0                  Literal{kind: Integer}
		        5               Literal{kind: Integer}
		            6           Literal{kind: Integer}
		                  {}    BlockExpression                                                                                           */
		y @ -5.. => {}                                                                                                                    /*
		y•@•-5..•=>•{}    MatchExpressionCase
		y•@•-5..          PatternVariableDeclaration{!ref, !mut}
		    -5..          RangePattern{!last}
		    -5            MinusPattern
		     5            Literal{kind: Integer}
		            {}    BlockExpression                                                                                                 */
		y @ ..-7 => {}                                                                                                                    /*
		y•@•..-7•=>•{}    MatchExpressionCase
		y•@•..-7          PatternVariableDeclaration{!ref, !mut}
		    ..-7          RangePattern{!last}
		      -7          MinusPattern
		       7          Literal{kind: Integer}
		            {}    BlockExpression                                                                                                 */
		box 0...9 => {}                                                                                                                   /*
		box•0...9•=>•{}    MatchExpressionCase
		box•0...9          BoxPattern
		    0...9          RangePattern{last}
		    0              Literal{kind: Integer}
		        9          Literal{kind: Integer}
		             {}    BlockExpression                                                                                                */
		box 10..=15 => {}                                                                                                                 /*
		box•10..=15•=>•{}    MatchExpressionCase
		box•10..=15          BoxPattern
		    10..=15          RangePattern{last}
		    10               Literal{kind: Integer}
		         15          Literal{kind: Integer}
		               {}    BlockExpression                                                                                              */
		box (16..=20) => {}                                                                                                               /*
		box•(16..=20)•=>•{}    MatchExpressionCase
		box•(16..=20)          BoxPattern
		     16..=20           RangePattern{last}
		     16                Literal{kind: Integer}
		          20           Literal{kind: Integer}
		                 {}    BlockExpression                                                                                            */
		0..1 => {}                                                                                                                        /*
		0..1•=>•{}    MatchExpressionCase
		0..1          RangePattern{!last}
		0             Literal{kind: Integer}
		   1          Literal{kind: Integer}
		        {}    BlockExpression                                                                                                     */
		(0)..1 => {}                                                                                                                      /*
		(0)..1•=>•{}    MatchExpressionCase
		(0)..1          RangePattern{!last}
		 0              Literal{kind: Integer}
		     1          Literal{kind: Integer}
		          {}    BlockExpression                                                                                                   */
		0..(1) => {}                                                                                                                      /*
		0..(1)•=>•{}    MatchExpressionCase
		0..(1)          RangePattern{!last}
		0               Literal{kind: Integer}
		    1           Literal{kind: Integer}
		          {}    BlockExpression                                                                                                   */
	}                                                                                                                                     /*
   ╚}    </MatchExpression.cases>
   ╚}    </MatchExpression>
   ╚}    </ExpressionStatement>                                                                                                           */
	[                                                                                                                                     /*
	[↲    <ExpressionStatement{semi}>
	[↲    <ArrayLiteral>                                                                                                                  */
		0..1,                                                                                                                             /*
		0..1    RangeLiteral{!last}
		0       Literal{kind: Integer}
		   1    Literal{kind: Integer}                                                                                                    */
		(0)..1,                                                                                                                           /*
		(0)..1    RangeLiteral{!last}
		 0        Literal{kind: Integer}
		     1    Literal{kind: Integer}                                                                                                  */
		0..(1),                                                                                                                           /*
		0..(1)    RangeLiteral{!last}
		0         Literal{kind: Integer}
		    1     Literal{kind: Integer}                                                                                                  */
		(0)..(1),                                                                                                                         /*
		(0)..(1)    RangeLiteral{!last}
		 0          Literal{kind: Integer}
		      1     Literal{kind: Integer}                                                                                                */
		1..,                                                                                                                              /*
		1..    RangeLiteral{!last}
		1      Literal{kind: Integer}                                                                                                     */
		..,                                                                                                                               /*
		..    RangeLiteral{!last}                                                                                                         */
		0..=1,                                                                                                                            /*
		0..=1    RangeLiteral{last}
		0        Literal{kind: Integer}
		    1    Literal{kind: Integer}                                                                                                   */
		0 ..=1,                                                                                                                           /*
		0•..=1    RangeLiteral{last}
		0         Literal{kind: Integer}
		     1    Literal{kind: Integer}                                                                                                  */
		0 ..=1,                                                                                                                           /*
		0•..=1    RangeLiteral{last}
		0         Literal{kind: Integer}
		     1    Literal{kind: Integer}                                                                                                  */
		// 0...1,
		//•0...1,    Comment{line}
		// &0...9,
		//•&0...9,    Comment{line}
		&10..=15,                                                                                                                         /*
		&10..=15    RangeLiteral{last}
		&10         ReferenceExpression{!mut}
		 10         Literal{kind: Integer}
		      15    Literal{kind: Integer}                                                                                                */
		// box 0...9,
		//•box•0...9,    Comment{line}
		box 0..=9,                                                                                                                        /*
		box•0..=9    RangeLiteral{last}
		box•0        BoxExpression
		    0        Literal{kind: Integer}
		        9    Literal{kind: Integer}                                                                                               */
		..1,                                                                                                                              /*
		..1    RangeLiteral{!last}
		  1    Literal{kind: Integer}                                                                                                     */
		..=1,                                                                                                                             /*
		..=1    RangeLiteral{last}
		   1    Literal{kind: Integer}                                                                                                    */
		0u32..10i32,                                                                                                                      /*
		0u32..10i32    RangeLiteral{!last}
		0u32           Literal{kind: Integer}
		 u32           Identifier
		      10i32    Literal{kind: Integer}
		        i32    Identifier                                                                                                         */
		*a..                                                                                                                              /*
		*a..    RangeLiteral{!last}
		*a      DereferenceExpression                                                                                                     */
	];                                                                                                                                    /*
   ╚]     </ArrayLiteral>
   ╚];    </ExpressionStatement>                                                                                                          */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
fn f() { a.. }                                                                                                                            /*
fn•f()•{•a..•}    FunctionDeclaration
    ()            FunctionDeclaration.parameters{dk: "()"}
       {•a..•}    FunctionDeclaration.body{dk: "{}"}
         a..      ExpressionStatement{!semi}, RangeLiteral{!last}                                                                         */
fn f() { a..b }                                                                                                                           /*
fn•f()•{•a..b•}    FunctionDeclaration
    ()             FunctionDeclaration.parameters{dk: "()"}
       {•a..b•}    FunctionDeclaration.body{dk: "{}"}
         a..b      ExpressionStatement{!semi}, RangeLiteral{!last}                                                                        */
fn f() { a()..b() }                                                                                                                       /*
fn•f()•{•a()..b()•}    FunctionDeclaration
    ()                 FunctionDeclaration.parameters{dk: "()"}
       {•a()..b()•}    FunctionDeclaration.body{dk: "{}"}
         a()..b()      ExpressionStatement{!semi}, RangeLiteral{!last}
         a()           CallExpression
          ()           CallExpression.arguments{dk: "()"}
              b()      CallExpression
               ()      CallExpression.arguments{dk: "()"}                                                                                 */
fn foo(-128..=127: i8) {}                                                                                                                 /*
fn•foo(-128..=127:•i8)•{}    FunctionDeclaration
      (-128..=127:•i8)       FunctionDeclaration.parameters{dk: "()"}
       -128..=127:•i8        FunctionParameterDeclaration
       -128..=127            RangePattern{last}
       -128                  MinusPattern
        128                  Literal{kind: Integer}
              127            Literal{kind: Integer}
                       {}    FunctionDeclaration.body{dk: "{}"}
fn•foo(-128..=127:•i8)•{}    </Program.ast>
fn•foo(-128..=127:•i8)•{}    </Program>                                                                                                   */
// Discarded Nodes: 14
// Parsed Nodes: 605
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 2563 (33% re-reads)
// Unnecessary 'skip_whitespace()' calls: 92
// source: "../../samples/expressions/range.rs"