fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>                                                                                                        */
	{ foo.0 }.0 = 0;                                                                                                                      /*
    {•foo.0•}.0•=•0;    ExpressionStatement
    {•foo.0•}.0•=•0     ReassignmentExpression
    {•foo.0•}.0         MemberExpression
    {•foo.0•}           BlockExpression
      foo.0             ExpressionStatement, MemberExpression
          0             Index
              0         Index
                  0     Literal                                                                                                           */
	async {}.await;                                                                                                                       /*
    async•{}.await;    ExpressionStatement
    async•{}.await     AwaitExpression
    async•{}           BlockExpression                                                                                                    */
	async move {}                                                                                                                         /*
    async•move•{}    ExpressionStatement, BlockExpression                                                                                 */
	{ a }.0 += { 0 };                                                                                                                     /*
    {•a•}.0•+=•{•0•};    ExpressionStatement
    {•a•}.0•+=•{•0•}     ReassignmentOperationExpression
    {•a•}.0              MemberExpression
    {•a•}                BlockExpression
      a                  ExpressionStatement
          0              Index
               {•0•}     BlockExpression
                 0       ExpressionStatement, Literal                                                                                     */
	try {}                                                                                                                                /*
    try•{}    ExpressionStatement, TryBlockExpression                                                                                     */
	match try {} {}                                                                                                                       /*
    match•try•{}•{}    ExpressionStatement, MatchExpression
          try•{}       TryBlockExpression                                                                                                 */
	for lhs in &mut self.0 { *lhs += rhs; }                                                                                               /*
    for•lhs•in•&mut•self.0•{•*lhs•+=•rhs;•}    ExpressionStatement, ForInBlockExpression
               &mut•self.0                     ReferenceExpression
                    self.0                     MemberExpression
                         0                     Index
                             *lhs•+=•rhs;      ExpressionStatement
                             *lhs•+=•rhs       ReassignmentOperationExpression
                             *lhs              DereferenceExpression                                                                      */
	for lhs in self.0.iter_mut() {}                                                                                                       /*
    for•lhs•in•self.0.iter_mut()•{}    ExpressionStatement, ForInBlockExpression
               self.0.iter_mut()       CallExpression
               self.0                  MemberExpression
                    0                  Index                                                                                              */
    for _ in [1, 2, 3].into_iter() {}                                                                                                     /*
    for•_•in•[1,•2,•3].into_iter()•{}    ExpressionStatement, ForInBlockExpression
        _                                WildcardPattern
             [1,•2,•3].into_iter()       CallExpression
             [1,•2,•3]                   ArrayLiteral
              1                          Literal
                 2                       Literal
                    3                    Literal                                                                                          */
	for elt in self { r = r + f(*elt); }                                                                                                  /*
    for•elt•in•self•{•r•=•r•+•f(*elt);•}    ExpressionStatement, ForInBlockExpression
                      r•=•r•+•f(*elt);      ExpressionStatement
                      r•=•r•+•f(*elt)       ReassignmentExpression
                          r•+•f(*elt)       OperationExpression
                              f(*elt)       CallExpression
                                *elt        DereferenceExpression                                                                         */
	for _ in [1, 2] {}                                                                                                                    /*
    for•_•in•[1,•2]•{}    ExpressionStatement, ForInBlockExpression
        _                 WildcardPattern
             [1,•2]       ArrayLiteral
              1           Literal
                 2        Literal                                                                                                         */
    for _ in [1.0, 2.0] {}                                                                                                                /*
    for•_•in•[1.0,•2.0]•{}    ExpressionStatement, ForInBlockExpression
        _                     WildcardPattern
             [1.0,•2.0]       ArrayLiteral
              1.0             Literal
                   2.0        Literal                                                                                                     */
	if loop {} {}                                                                                                                         /*
    if•loop•{}•{}    ExpressionStatement, IfBlockExpression
       loop•{}       LoopBlockExpression                                                                                                  */
	if let 0 = 1 {}                                                                                                                       /*
    if•let•0•=•1•{}    ExpressionStatement, IfBlockExpression
       let•0•=•1       LetScrutinee
           0           Literal
               1       Literal                                                                                                            */
	if a % 5 == 0 {}                                                                                                                      /*
    if•a•%•5•==•0•{}    ExpressionStatement, IfBlockExpression
       a•%•5•==•0       ComparisonExpression
       a•%•5            OperationExpression
           5            Literal
                0       Literal                                                                                                           */
	let x: A = if a % 5 == 0 {};                                                                                                          /*
    let•x:•A•=•if•a•%•5•==•0•{};    LetVariableDeclaration
               if•a•%•5•==•0•{}     IfBlockExpression
                  a•%•5•==•0        ComparisonExpression
                  a•%•5             OperationExpression
                      5             Literal
                           0        Literal                                                                                               */
	let x = if let 0 = 1 {};                                                                                                              /*
    let•x•=•if•let•0•=•1•{};    LetVariableDeclaration
            if•let•0•=•1•{}     IfBlockExpression
               let•0•=•1        LetScrutinee
                   0            Literal
                       1        Literal                                                                                                   */
	if let 0 = 1 { 3 }                                                                                                                    /*
    if•let•0•=•1•{•3•}    ExpressionStatement, IfBlockExpression
       let•0•=•1          LetScrutinee
           0              Literal
               1          Literal
                   3      ExpressionStatement, Literal                                                                                    */
	if y = (Foo { foo: x }) {}                                                                                                            /*
    if•y•=•(Foo•{•foo:•x•})•{}    ExpressionStatement, IfBlockExpression
       y•=•(Foo•{•foo:•x•})       ReassignmentExpression
            Foo•{•foo:•x•}        StructLiteral
                  foo:•x          StructLiteralProperty                                                                                   */
	if q == "" {}                                                                                                                         /*
    if•q•==•""•{}    ExpressionStatement, IfBlockExpression
       q•==•""       ComparisonExpression
            ""       Literal                                                                                                              */
    if ('x' as char) < ('y' as char) {} else {}                                                                                           /*
    if•('x'•as•char)•<•('y'•as•char)•{}•else•{}    ExpressionStatement, IfBlockExpression
       ('x'•as•char)•<•('y'•as•char)               ComparisonExpression
        'x'•as•char                                ExpressionAsTypeCast
        'x'                                        Literal
                        'y'•as•char                ExpressionAsTypeCast
                        'y'                        Literal
                                             {}    BlockExpression                                                                        */
	let a = if 1 { 1 };                                                                                                                   /*
    let•a•=•if•1•{•1•};    LetVariableDeclaration
            if•1•{•1•}     IfBlockExpression
               1           Literal
                   1       ExpressionStatement, Literal                                                                                   */
	let a = if 1 { 0 } else if 1 { 0 };                                                                                                   /*
    let•a•=•if•1•{•0•}•else•if•1•{•0•};    LetVariableDeclaration
            if•1•{•0•}•else•if•1•{•0•}     IfBlockExpression
               1                           Literal
                   0                       ExpressionStatement, Literal
                            if•1•{•0•}     IfBlockExpression
                               1           Literal
                                   0       ExpressionStatement, Literal                                                                   */
	let a = if if 0 { 0 } else { 0 } { 0 } else { 0 };                                                                                    /*
    let•a•=•if•if•0•{•0•}•else•{•0•}•{•0•}•else•{•0•};    LetVariableDeclaration
            if•if•0•{•0•}•else•{•0•}•{•0•}•else•{•0•}     IfBlockExpression
               if•0•{•0•}•else•{•0•}                      IfBlockExpression
                  0                                       Literal
                      0                                   ExpressionStatement, Literal
                               {•0•}                      BlockExpression
                                 0                        ExpressionStatement, Literal
                                       0                  ExpressionStatement, Literal
                                                {•0•}     BlockExpression
                                                  0       ExpressionStatement, Literal                                                    */
	let a = if 0 { if 0 { 0 } else { 0 } } else { 0 };                                                                                    /*
    let•a•=•if•0•{•if•0•{•0•}•else•{•0•}•}•else•{•0•};    LetVariableDeclaration
            if•0•{•if•0•{•0•}•else•{•0•}•}•else•{•0•}     IfBlockExpression
               0                                          Literal
                   if•0•{•0•}•else•{•0•}                  ExpressionStatement, IfBlockExpression
                      0                                   Literal
                          0                               ExpressionStatement, Literal
                                   {•0•}                  BlockExpression
                                     0                    ExpressionStatement, Literal
                                                {•0•}     BlockExpression
                                                  0       ExpressionStatement, Literal                                                    */
	let a = if 0 { 0 } else if 0 { 0 } else { 1 };                                                                                        /*
    let•a•=•if•0•{•0•}•else•if•0•{•0•}•else•{•1•};    LetVariableDeclaration
            if•0•{•0•}•else•if•0•{•0•}•else•{•1•}     IfBlockExpression
               0                                      Literal
                   0                                  ExpressionStatement, Literal
                            if•0•{•0•}•else•{•1•}     IfBlockExpression
                               0                      Literal
                                   0                  ExpressionStatement, Literal
                                            {•1•}     BlockExpression
                                              1       ExpressionStatement, Literal                                                        */
	let a = if let 0 = if let 0 = 0 { 0} else { 0 } { 0 } else { 0 };                                                                     /*
    let•a•=•if•let•0•=•if•let•0•=•0•{•0}•else•{•0•}•{•0•}•else•{•0•};    LetVariableDeclaration
            if•let•0•=•if•let•0•=•0•{•0}•else•{•0•}•{•0•}•else•{•0•}     IfBlockExpression
               let•0•=•if•let•0•=•0•{•0}•else•{•0•}                      LetScrutinee
                   0                                                     Literal
                       if•let•0•=•0•{•0}•else•{•0•}                      IfBlockExpression
                          let•0•=•0                                      LetScrutinee
                              0                                          Literal
                                  0                                      Literal
                                      0                                  ExpressionStatement, Literal
                                              {•0•}                      BlockExpression
                                                0                        ExpressionStatement, Literal
                                                      0                  ExpressionStatement, Literal
                                                               {•0•}     BlockExpression
                                                                 0       ExpressionStatement, Literal                                     */
	for x in 0..10 { async { Some(x) }.await.unwrap(); }                                                                                  /*
    for•x•in•0..10•{•async•{•Some(x)•}.await.unwrap();•}    ExpressionStatement, ForInBlockExpression
             0..10                                          RangeLiteral
             0                                              Literal
                10                                          Literal
                     async•{•Some(x)•}.await.unwrap();      ExpressionStatement
                     async•{•Some(x)•}.await.unwrap()       CallExpression
                     async•{•Some(x)•}.await                AwaitExpression
                     async•{•Some(x)•}                      BlockExpression
                             Some(x)                        ExpressionStatement, CallExpression                                           */
	for _  in  1 ..{ call_forever(); }                                                                                                    /*
    for•_••in••1•..{•call_forever();•}    ExpressionStatement, ForInBlockExpression
        _                                 WildcardPattern
               1•..                       RangeLiteral
               1                          Literal
                     call_forever();      ExpressionStatement
                     call_forever()       CallExpression                                                                                  */
	unsafe { Foo { b: () }.a }                                                                                                            /*
    unsafe•{•Foo•{•b:•()•}.a•}    ExpressionStatement, BlockExpression
             Foo•{•b:•()•}.a      ExpressionStatement, MemberExpression
             Foo•{•b:•()•}        StructLiteral
                   b:•()          StructLiteralProperty
                      ()          TupleLiteral                                                                                            */
	if 1 { } else if let Some(a) = Some(1) { }                                                                                            /*
    if•1•{•}•else•if•let•Some(a)•=•Some(1)•{•}    ExpressionStatement, IfBlockExpression
       1                                          Literal
                  if•let•Some(a)•=•Some(1)•{•}    IfBlockExpression
                     let•Some(a)•=•Some(1)        LetScrutinee
                         Some(a)                  TuplePattern
                                   Some(1)        CallExpression
                                        1         Literal                                                                                 */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
[                                                                                                                                         /*
[↲    <ExpressionStatement>, <ArrayLiteral>                                                                                               */
	m::Pub { 0: loop {} },                                                                                                                /*
    m::Pub•{•0:•loop•{}•}    StructLiteral
    m::Pub                   ExpressionPath
             0:•loop•{}      StructLiteralProperty
             0               Index
                loop•{}      LoopBlockExpression                                                                                          */
	2_usize + (loop {}),                                                                                                                  /*
    2_usize•+•(loop•{})    OperationExpression
    2_usize                Literal
      usize                Identifier
               loop•{}     LoopBlockExpression                                                                                            */
	[(); & { loop { continue } } ],                                                                                                       /*
    [();•&•{•loop•{•continue•}•}•]    SizedArrayLiteral
     ()                               TupleLiteral
         &•{•loop•{•continue•}•}      ReferenceExpression
           {•loop•{•continue•}•}      BlockExpression
             loop•{•continue•}        ExpressionStatement, LoopBlockExpression
                    continue          ExpressionStatement, ContinueExpression                                                             */
	[(); loop { break }],                                                                                                                 /*
    [();•loop•{•break•}]    SizedArrayLiteral
     ()                     TupleLiteral
         loop•{•break•}     LoopBlockExpression
                break       ExpressionStatement, BreakExpression                                                                          */
	[(); {while true {break}; 0}],                                                                                                        /*
    [();•{while•true•{break};•0}]    SizedArrayLiteral
     ()                              TupleLiteral
         {while•true•{break};•0}     BlockExpression
          while•true•{break};        ExpressionStatement
          while•true•{break}         WhileBlockExpression
                true                 Literal
                      break          ExpressionStatement, BreakExpression
                              0      ExpressionStatement, Literal                                                                         */
	[(); { for _ in 0usize.. {}; 0}],                                                                                                     /*
    [();•{•for•_•in•0usize..•{};•0}]    SizedArrayLiteral
     ()                                 TupleLiteral
         {•for•_•in•0usize..•{};•0}     BlockExpression
           for•_•in•0usize..•{};        ExpressionStatement
           for•_•in•0usize..•{}         ForInBlockExpression
               _                        WildcardPattern
                    0usize..            RangeLiteral
                    0usize              Literal
                     usize              Identifier
                                 0      ExpressionStatement, Literal                                                                      */
	unsafe { *(&raw mut (y)) },                                                                                                           /*
    unsafe•{•*(&raw•mut•(y))•}    BlockExpression
             *(&raw•mut•(y))      ExpressionStatement, DereferenceExpression
               &raw•mut•(y)       RawReferenceExpression                                                                                  */
];                                                                                                                                        /*
];    </ExpressionStatement>
]     </ArrayLiteral>                                                                                                                     */

a::b(async move {                                                                                                                         /*
a::b(async•move•{↲    <ExpressionStatement>, <CallExpression>
a::b                  ExpressionPath
     async•move•{↲    <BlockExpression>                                                                                                   */
	if let Err(e) = c(d).await {                                                                                                          /*
    if•let•Err(e)•=•c(d).await•{↲    <ExpressionStatement>, <IfBlockExpression>
       let•Err(e)•=•c(d).await       LetScrutinee
           Err(e)                    TuplePattern
                    c(d).await       AwaitExpression
                    c(d)             CallExpression                                                                                       */
		f!("g: {}", h);                                                                                                                   /*
        f!("g:•{}",•h);    ExpressionStatement
        f!("g:•{}",•h)     MacroInvocation
           "g:•{}"         Literal
                  ,        PunctuationToken                                                                                               */
	}                                                                                                                                     /*
   ╚}    </ExpressionStatement>, </IfBlockExpression>                                                                                     */
});                                                                                                                                       /*
});    </ExpressionStatement>
})     </CallExpression>
}      </BlockExpression>                                                                                                                 */

// Discarded Nodes: 6
// Parsed Nodes: 345
// state_rollbacks: 4
// Total '.charCodeAt()' calls: 1650 (34% re-reads)
// Unnecessary 'skip_whitespace()' calls: 92
// source: "../../samples/expressions/block.rs"