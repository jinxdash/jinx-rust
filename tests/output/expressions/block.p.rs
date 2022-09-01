fn f() {                                                                                                                                  /*
fn•f()•{↲    <Program>
fn•f()•{↲    <Program.ast{dk: "None"}>
fn•f()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	{ foo.0 }.0 = 0;                                                                                                                      /*
	{•foo.0•}.0•=•0;    ExpressionStatement{semi}
	{•foo.0•}.0•=•0     ReassignmentExpression{tk: "="}
	{•foo.0•}.0         MemberExpression{!computed}
	{•foo.0•}           BlockExpression
	  foo.0             ExpressionStatement{!semi}, MemberExpression{!computed}
	      0             Index
	          0         Index
	              0     Literal{kind: Integer}                                                                                            */
	async {}.await;                                                                                                                       /*
	async•{}.await;    ExpressionStatement{semi}
	async•{}.await     AwaitExpression
	async•{}           BlockExpression{async}
	      {}           BlockExpression.body{dk: "{}"}                                                                                     */
	async move {}                                                                                                                         /*
	async•move•{}    ExpressionStatement{!semi}, BlockExpression{move, async}
	           {}    BlockExpression.body{dk: "{}"}                                                                                       */
	{ a }.0 += { 0 };                                                                                                                     /*
	{•a•}.0•+=•{•0•};    ExpressionStatement{semi}
	{•a•}.0•+=•{•0•}     ReassignmentOperationExpression{tk: "+="}
	{•a•}.0              MemberExpression{!computed}
	{•a•}                BlockExpression
	  a                  ExpressionStatement{!semi}
	      0              Index
	           {•0•}     BlockExpression
	             0       ExpressionStatement{!semi}, Literal{kind: Integer}                                                               */
	try {}                                                                                                                                /*
	try•{}    ExpressionStatement{!semi}, TryBlockExpression
	    {}    TryBlockExpression.body{dk: "{}"}                                                                                           */
	match try {} {}                                                                                                                       /*
	match•try•{}•{}    ExpressionStatement{!semi}, MatchExpression
	      try•{}       TryBlockExpression
	          {}       TryBlockExpression.body{dk: "{}"}
	             {}    MatchExpression.cases{dk: "{}"}                                                                                    */
	for lhs in &mut self.0 { *lhs += rhs; }                                                                                               /*
	for•lhs•in•&mut•self.0•{•*lhs•+=•rhs;•}    ExpressionStatement{!semi}, ForInBlockExpression
	           &mut•self.0                     ReferenceExpression{mut}
	                self.0                     MemberExpression{!computed}
	                     0                     Index
	                       {•*lhs•+=•rhs;•}    ForInBlockExpression.body{dk: "{}"}
	                         *lhs•+=•rhs;      ExpressionStatement{semi}
	                         *lhs•+=•rhs       ReassignmentOperationExpression{tk: "+="}
	                         *lhs              DereferenceExpression                                                                      */
	for lhs in self.0.iter_mut() {}                                                                                                       /*
	for•lhs•in•self.0.iter_mut()•{}    ExpressionStatement{!semi}, ForInBlockExpression
	           self.0.iter_mut()       CallExpression
	           self.0                  MemberExpression{!computed}
	                0                  Index
	                          ()       CallExpression.arguments{dk: "()"}
	                             {}    ForInBlockExpression.body{dk: "{}"}                                                                */
    for _ in [1, 2, 3].into_iter() {}                                                                                                     /*
    for•_•in•[1,•2,•3].into_iter()•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                                WildcardPattern
             [1,•2,•3].into_iter()       CallExpression
             [1,•2,•3]                   ArrayLiteral
              1                          Literal{kind: Integer}
                 2                       Literal{kind: Integer}
                    3                    Literal{kind: Integer}
                                ()       CallExpression.arguments{dk: "()"}
                                   {}    ForInBlockExpression.body{dk: "{}"}                                                              */
	for elt in self { r = r + f(*elt); }                                                                                                  /*
	for•elt•in•self•{•r•=•r•+•f(*elt);•}    ExpressionStatement{!semi}, ForInBlockExpression
	                {•r•=•r•+•f(*elt);•}    ForInBlockExpression.body{dk: "{}"}
	                  r•=•r•+•f(*elt);      ExpressionStatement{semi}
	                  r•=•r•+•f(*elt)       ReassignmentExpression{tk: "="}
	                      r•+•f(*elt)       OperationExpression{tk: "+"}
	                          f(*elt)       CallExpression
	                           (*elt)       CallExpression.arguments{dk: "()"}
	                            *elt        DereferenceExpression                                                                         */
	for _ in [1, 2] {}                                                                                                                    /*
	for•_•in•[1,•2]•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                 WildcardPattern
	         [1,•2]       ArrayLiteral
	          1           Literal{kind: Integer}
	             2        Literal{kind: Integer}
	                {}    ForInBlockExpression.body{dk: "{}"}                                                                             */
    for _ in [1.0, 2.0] {}                                                                                                                /*
    for•_•in•[1.0,•2.0]•{}    ExpressionStatement{!semi}, ForInBlockExpression
        _                     WildcardPattern
             [1.0,•2.0]       ArrayLiteral
              1.0             Literal{kind: Float}
                   2.0        Literal{kind: Float}
                        {}    ForInBlockExpression.body{dk: "{}"}                                                                         */
	if loop {} {}                                                                                                                         /*
	if•loop•{}•{}    ExpressionStatement{!semi}, IfBlockExpression
	   loop•{}       LoopBlockExpression
	        {}       LoopBlockExpression.body{dk: "{}"}
	           {}    IfBlockExpression.body{dk: "{}"}                                                                                     */
	if let 0 = 1 {}                                                                                                                       /*
	if•let•0•=•1•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0•=•1       LetScrutinee
	       0           Literal{kind: Integer}
	           1       Literal{kind: Integer}
	             {}    IfBlockExpression.body{dk: "{}"}                                                                                   */
	if a % 5 == 0 {}                                                                                                                      /*
	if•a•%•5•==•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   a•%•5•==•0       ComparisonExpression{tk: "=="}
	   a•%•5            OperationExpression{tk: "%"}
	       5            Literal{kind: Integer}
	            0       Literal{kind: Integer}
	              {}    IfBlockExpression.body{dk: "{}"}                                                                                  */
	let x: A = if a % 5 == 0 {};                                                                                                          /*
	let•x:•A•=•if•a•%•5•==•0•{};    LetVariableDeclaration
	           if•a•%•5•==•0•{}     IfBlockExpression
	              a•%•5•==•0        ComparisonExpression{tk: "=="}
	              a•%•5             OperationExpression{tk: "%"}
	                  5             Literal{kind: Integer}
	                       0        Literal{kind: Integer}
	                         {}     IfBlockExpression.body{dk: "{}"}                                                                      */
	let x = if let 0 = 1 {};                                                                                                              /*
	let•x•=•if•let•0•=•1•{};    LetVariableDeclaration
	        if•let•0•=•1•{}     IfBlockExpression
	           let•0•=•1        LetScrutinee
	               0            Literal{kind: Integer}
	                   1        Literal{kind: Integer}
	                     {}     IfBlockExpression.body{dk: "{}"}                                                                          */
	if let 0 = 1 { 3 }                                                                                                                    /*
	if•let•0•=•1•{•3•}    ExpressionStatement{!semi}, IfBlockExpression
	   let•0•=•1          LetScrutinee
	       0              Literal{kind: Integer}
	           1          Literal{kind: Integer}
	             {•3•}    IfBlockExpression.body{dk: "{}"}
	               3      ExpressionStatement{!semi}, Literal{kind: Integer}                                                              */
	if y = (Foo { foo: x }) {}                                                                                                            /*
	if•y•=•(Foo•{•foo:•x•})•{}    ExpressionStatement{!semi}, IfBlockExpression
	   y•=•(Foo•{•foo:•x•})       ReassignmentExpression{tk: "="}
	        Foo•{•foo:•x•}        StructLiteral
	            {•foo:•x•}        StructLiteral.properties{dk: "{}"}
	              foo:•x          StructLiteralProperty
	                        {}    IfBlockExpression.body{dk: "{}"}                                                                        */
	if q == "" {}                                                                                                                         /*
	if•q•==•""•{}    ExpressionStatement{!semi}, IfBlockExpression
	   q•==•""       ComparisonExpression{tk: "=="}
	        ""       Literal{kind: String}
	           {}    IfBlockExpression.body{dk: "{}"}                                                                                     */
    if ('x' as char) < ('y' as char) {} else {}                                                                                           /*
    if•('x'•as•char)•<•('y'•as•char)•{}•else•{}    ExpressionStatement{!semi}, IfBlockExpression
       ('x'•as•char)•<•('y'•as•char)               ComparisonExpression{tk: "<"}
        'x'•as•char                                ExpressionAsTypeCast
        'x'                                        Literal{kind: Char}
                        'y'•as•char                ExpressionAsTypeCast
                        'y'                        Literal{kind: Char}
                                     {}            IfBlockExpression.body{dk: "{}"}
                                             {}    BlockExpression                                                                        */
	let a = if 1 { 1 };                                                                                                                   /*
	let•a•=•if•1•{•1•};    LetVariableDeclaration
	        if•1•{•1•}     IfBlockExpression
	           1           Literal{kind: Integer}
	             {•1•}     IfBlockExpression.body{dk: "{}"}
	               1       ExpressionStatement{!semi}, Literal{kind: Integer}                                                             */
	let a = if 1 { 0 } else if 1 { 0 };                                                                                                   /*
	let•a•=•if•1•{•0•}•else•if•1•{•0•};    LetVariableDeclaration
	        if•1•{•0•}•else•if•1•{•0•}     IfBlockExpression
	           1                           Literal{kind: Integer}
	             {•0•}                     IfBlockExpression.body{dk: "{}"}
	               0                       ExpressionStatement{!semi}, Literal{kind: Integer}
	                        if•1•{•0•}     IfBlockExpression
	                           1           Literal{kind: Integer}
	                             {•0•}     IfBlockExpression.body{dk: "{}"}
	                               0       ExpressionStatement{!semi}, Literal{kind: Integer}                                             */
	let a = if if 0 { 0 } else { 0 } { 0 } else { 0 };                                                                                    /*
	let•a•=•if•if•0•{•0•}•else•{•0•}•{•0•}•else•{•0•};    LetVariableDeclaration
	        if•if•0•{•0•}•else•{•0•}•{•0•}•else•{•0•}     IfBlockExpression
	           if•0•{•0•}•else•{•0•}                      IfBlockExpression
	              0                                       Literal{kind: Integer}
	                {•0•}                                 IfBlockExpression.body{dk: "{}"}
	                  0                                   ExpressionStatement{!semi}, Literal{kind: Integer}
	                           {•0•}                      BlockExpression
	                             0                        ExpressionStatement{!semi}, Literal{kind: Integer}
	                                 {•0•}                IfBlockExpression.body{dk: "{}"}
	                                   0                  ExpressionStatement{!semi}, Literal{kind: Integer}
	                                            {•0•}     BlockExpression
	                                              0       ExpressionStatement{!semi}, Literal{kind: Integer}                              */
	let a = if 0 { if 0 { 0 } else { 0 } } else { 0 };                                                                                    /*
	let•a•=•if•0•{•if•0•{•0•}•else•{•0•}•}•else•{•0•};    LetVariableDeclaration
	        if•0•{•if•0•{•0•}•else•{•0•}•}•else•{•0•}     IfBlockExpression
	           0                                          Literal{kind: Integer}
	             {•if•0•{•0•}•else•{•0•}•}                IfBlockExpression.body{dk: "{}"}
	               if•0•{•0•}•else•{•0•}                  ExpressionStatement{!semi}, IfBlockExpression
	                  0                                   Literal{kind: Integer}
	                    {•0•}                             IfBlockExpression.body{dk: "{}"}
	                      0                               ExpressionStatement{!semi}, Literal{kind: Integer}
	                               {•0•}                  BlockExpression
	                                 0                    ExpressionStatement{!semi}, Literal{kind: Integer}
	                                            {•0•}     BlockExpression
	                                              0       ExpressionStatement{!semi}, Literal{kind: Integer}                              */
	let a = if 0 { 0 } else if 0 { 0 } else { 1 };                                                                                        /*
	let•a•=•if•0•{•0•}•else•if•0•{•0•}•else•{•1•};    LetVariableDeclaration
	        if•0•{•0•}•else•if•0•{•0•}•else•{•1•}     IfBlockExpression
	           0                                      Literal{kind: Integer}
	             {•0•}                                IfBlockExpression.body{dk: "{}"}
	               0                                  ExpressionStatement{!semi}, Literal{kind: Integer}
	                        if•0•{•0•}•else•{•1•}     IfBlockExpression
	                           0                      Literal{kind: Integer}
	                             {•0•}                IfBlockExpression.body{dk: "{}"}
	                               0                  ExpressionStatement{!semi}, Literal{kind: Integer}
	                                        {•1•}     BlockExpression
	                                          1       ExpressionStatement{!semi}, Literal{kind: Integer}                                  */
	let a = if let 0 = if let 0 = 0 { 0} else { 0 } { 0 } else { 0 };                                                                     /*
	let•a•=•if•let•0•=•if•let•0•=•0•{•0}•else•{•0•}•{•0•}•else•{•0•};    LetVariableDeclaration
	        if•let•0•=•if•let•0•=•0•{•0}•else•{•0•}•{•0•}•else•{•0•}     IfBlockExpression
	           let•0•=•if•let•0•=•0•{•0}•else•{•0•}                      LetScrutinee
	               0                                                     Literal{kind: Integer}
	                   if•let•0•=•0•{•0}•else•{•0•}                      IfBlockExpression
	                      let•0•=•0                                      LetScrutinee
	                          0                                          Literal{kind: Integer}
	                              0                                      Literal{kind: Integer}
	                                {•0}                                 IfBlockExpression.body{dk: "{}"}
	                                  0                                  ExpressionStatement{!semi}, Literal{kind: Integer}
	                                          {•0•}                      BlockExpression
	                                            0                        ExpressionStatement{!semi}, Literal{kind: Integer}
	                                                {•0•}                IfBlockExpression.body{dk: "{}"}
	                                                  0                  ExpressionStatement{!semi}, Literal{kind: Integer}
	                                                           {•0•}     BlockExpression
	                                                             0       ExpressionStatement{!semi}, Literal{kind: Integer}               */
	for x in 0..10 { async { Some(x) }.await.unwrap(); }                                                                                  /*
	for•x•in•0..10•{•async•{•Some(x)•}.await.unwrap();•}    ExpressionStatement{!semi}, ForInBlockExpression
	         0..10                                          RangeLiteral{!last}
	         0                                              Literal{kind: Integer}
	            10                                          Literal{kind: Integer}
	               {•async•{•Some(x)•}.await.unwrap();•}    ForInBlockExpression.body{dk: "{}"}
	                 async•{•Some(x)•}.await.unwrap();      ExpressionStatement{semi}
	                 async•{•Some(x)•}.await.unwrap()       CallExpression
	                 async•{•Some(x)•}.await                AwaitExpression
	                 async•{•Some(x)•}                      BlockExpression{async}
	                       {•Some(x)•}                      BlockExpression.body{dk: "{}"}
	                         Some(x)                        ExpressionStatement{!semi}, CallExpression
	                             (x)                        CallExpression.arguments{dk: "()"}
	                                               ()       CallExpression.arguments{dk: "()"}                                            */
	for _  in  1 ..{ call_forever(); }                                                                                                    /*
	for•_••in••1•..{•call_forever();•}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                                 WildcardPattern
	           1•..                       RangeLiteral{!last}
	           1                          Literal{kind: Integer}
	               {•call_forever();•}    ForInBlockExpression.body{dk: "{}"}
	                 call_forever();      ExpressionStatement{semi}
	                 call_forever()       CallExpression
	                             ()       CallExpression.arguments{dk: "()"}                                                              */
	unsafe { Foo { b: () }.a }                                                                                                            /*
	unsafe•{•Foo•{•b:•()•}.a•}    ExpressionStatement{!semi}, BlockExpression{unsafe}
	       {•Foo•{•b:•()•}.a•}    BlockExpression.body{dk: "{}"}
	         Foo•{•b:•()•}.a      ExpressionStatement{!semi}, MemberExpression{!computed}
	         Foo•{•b:•()•}        StructLiteral
	             {•b:•()•}        StructLiteral.properties{dk: "{}"}
	               b:•()          StructLiteralProperty
	                  ()          TupleLiteral                                                                                            */
	if 1 { } else if let Some(a) = Some(1) { }                                                                                            /*
	if•1•{•}•else•if•let•Some(a)•=•Some(1)•{•}    ExpressionStatement{!semi}, IfBlockExpression
	   1                                          Literal{kind: Integer}
	     {•}                                      IfBlockExpression.body{dk: "{}"}
	              if•let•Some(a)•=•Some(1)•{•}    IfBlockExpression
	                 let•Some(a)•=•Some(1)        LetScrutinee
	                     Some(a)                  TuplePattern
	                         (a)                  TuplePattern.items{dk: "()"}
	                               Some(1)        CallExpression
	                                   (1)        CallExpression.arguments{dk: "()"}
	                                    1         Literal{kind: Integer}
	                                       {•}    IfBlockExpression.body{dk: "{}"}                                                        */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
[                                                                                                                                         /*
[↲    <ExpressionStatement{semi}>
[↲    <ArrayLiteral>                                                                                                                      */
	m::Pub { 0: loop {} },                                                                                                                /*
	m::Pub•{•0:•loop•{}•}    StructLiteral
	m::Pub                   ExpressionPath
	       {•0:•loop•{}•}    StructLiteral.properties{dk: "{}"}
	         0:•loop•{}      StructLiteralProperty
	         0               Index
	            loop•{}      LoopBlockExpression
	                 {}      LoopBlockExpression.body{dk: "{}"}                                                                           */
	2_usize + (loop {}),                                                                                                                  /*
	2_usize•+•(loop•{})    OperationExpression{tk: "+"}
	2_usize                Literal{kind: Integer}
	  usize                Identifier
	           loop•{}     LoopBlockExpression
	                {}     LoopBlockExpression.body{dk: "{}"}                                                                             */
	[(); & { loop { continue } } ],                                                                                                       /*
	[();•&•{•loop•{•continue•}•}•]    SizedArrayLiteral
	 ()                               TupleLiteral
	     &•{•loop•{•continue•}•}      ReferenceExpression{!mut}
	       {•loop•{•continue•}•}      BlockExpression
	         loop•{•continue•}        ExpressionStatement{!semi}, LoopBlockExpression
	              {•continue•}        LoopBlockExpression.body{dk: "{}"}
	                continue          ExpressionStatement{!semi}, ContinueExpression                                                      */
	[(); loop { break }],                                                                                                                 /*
	[();•loop•{•break•}]    SizedArrayLiteral
	 ()                     TupleLiteral
	     loop•{•break•}     LoopBlockExpression
	          {•break•}     LoopBlockExpression.body{dk: "{}"}
	            break       ExpressionStatement{!semi}, BreakExpression                                                                   */
	[(); {while true {break}; 0}],                                                                                                        /*
	[();•{while•true•{break};•0}]    SizedArrayLiteral
	 ()                              TupleLiteral
	     {while•true•{break};•0}     BlockExpression
	      while•true•{break};        ExpressionStatement{semi}
	      while•true•{break}         WhileBlockExpression
	            true                 Literal{kind: True}
	                 {break}         WhileBlockExpression.body{dk: "{}"}
	                  break          ExpressionStatement{!semi}, BreakExpression
	                          0      ExpressionStatement{!semi}, Literal{kind: Integer}                                                   */
	[(); { for _ in 0usize.. {}; 0}],                                                                                                     /*
	[();•{•for•_•in•0usize..•{};•0}]    SizedArrayLiteral
	 ()                                 TupleLiteral
	     {•for•_•in•0usize..•{};•0}     BlockExpression
	       for•_•in•0usize..•{};        ExpressionStatement{semi}
	       for•_•in•0usize..•{}         ForInBlockExpression
	           _                        WildcardPattern
	                0usize..            RangeLiteral{!last}
	                0usize              Literal{kind: Integer}
	                 usize              Identifier
	                         {}         ForInBlockExpression.body{dk: "{}"}
	                             0      ExpressionStatement{!semi}, Literal{kind: Integer}                                                */
	unsafe { *(&raw mut (y)) },                                                                                                           /*
	unsafe•{•*(&raw•mut•(y))•}    BlockExpression{unsafe}
	       {•*(&raw•mut•(y))•}    BlockExpression.body{dk: "{}"}
	         *(&raw•mut•(y))      ExpressionStatement{!semi}, DereferenceExpression
	           &raw•mut•(y)       RawReferenceExpression{kind: "mut"}                                                                     */
];                                                                                                                                        /*
]     </ArrayLiteral>
];    </ExpressionStatement>                                                                                                              */

a::b(async move {                                                                                                                         /*
a::b(async•move•{↲    <ExpressionStatement{semi}>
a::b(async•move•{↲    <CallExpression>
a::b                  ExpressionPath
    (async•move•{↲    <CallExpression.arguments{dk: "()"}>
     async•move•{↲    <BlockExpression{move, async}>
                {↲    <BlockExpression.body{dk: "{}"}>                                                                                    */
	if let Err(e) = c(d).await {                                                                                                          /*
	if•let•Err(e)•=•c(d).await•{↲    <ExpressionStatement{!semi}>
	if•let•Err(e)•=•c(d).await•{↲    <IfBlockExpression>
	   let•Err(e)•=•c(d).await       LetScrutinee
	       Err(e)                    TuplePattern
	          (e)                    TuplePattern.items{dk: "()"}
	                c(d).await       AwaitExpression
	                c(d)             CallExpression
	                 (d)             CallExpression.arguments{dk: "()"}
	                           {↲    <IfBlockExpression.body{dk: "{}"}>                                                                   */
		f!("g: {}", h);                                                                                                                   /*
		f!("g:•{}",•h);    ExpressionStatement{semi}
		f!("g:•{}",•h)     MacroInvocation
		  ("g:•{}",•h)     MacroInvocation.segments{dk: "()"}
		   "g:•{}"         Literal{kind: String}
		          ,        PunctuationToken{tk: ","}                                                                                      */
	}                                                                                                                                     /*
   ╚}    </IfBlockExpression.body>
   ╚}    </IfBlockExpression>
   ╚}    </ExpressionStatement>                                                                                                           */
});                                                                                                                                       /*
}      </BlockExpression.body>
}      </BlockExpression>
})     </CallExpression.arguments>
})     </CallExpression>
});    </ExpressionStatement>
});    </Program.ast>
});    </Program>                                                                                                                         */
// Discarded Nodes: 6
// Parsed Nodes: 345
// state_rollbacks: 4
// Total '.charCodeAt()' calls: 1650 (34% re-reads)
// Unnecessary 'skip_whitespace()' calls: 92
// source: "../../samples/expressions/block.rs"