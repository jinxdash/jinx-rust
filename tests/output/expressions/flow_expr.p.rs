pub fn main() {                                                                                                                           /*
pub•fn•main()•{↲    <FunctionDeclaration>
pub                 PubSpecifier                                                                                                          */
    loop { return break as (); }                                                                                                          /*
    loop•{•return•break•as•();•}    ExpressionStatement, LoopBlockExpression
           return•break•as•();      ExpressionStatement
           return•break•as•()       ReturnExpression
                  break•as•()       ExpressionAsTypeCast
                  break             BreakExpression
                           ()       TypeTuple                                                                                             */
    return ();                                                                                                                            /*
    return•();    ExpressionStatement
    return•()     ReturnExpression
           ()     TupleLiteral                                                                                                            */
    return as ();                                                                                                                         /*
    return•as•();    ExpressionStatement
    return•as•()     ExpressionAsTypeCast
    return           ReturnExpression
              ()     TypeTuple                                                                                                            */
    return if 1 {()} else {()};                                                                                                           /*
    return•if•1•{()}•else•{()};    ExpressionStatement
    return•if•1•{()}•else•{()}     ReturnExpression
           if•1•{()}•else•{()}     IfBlockExpression
              1                    Literal
                 ()                ExpressionStatement, TupleLiteral
                          {()}     BlockExpression
                           ()      ExpressionStatement, TupleLiteral                                                                      */
    return return as ();                                                                                                                  /*
    return•return•as•();    ExpressionStatement
    return•return•as•()     ReturnExpression
           return•as•()     ExpressionAsTypeCast
           return           ReturnExpression
                     ()     TypeTuple                                                                                                     */
    return return return;                                                                                                                 /*
    return•return•return;    ExpressionStatement
    return•return•return     ReturnExpression
           return•return     ReturnExpression
                  return     ReturnExpression                                                                                             */
	return try {4};                                                                                                                       /*
    return•try•{4};    ExpressionStatement
    return•try•{4}     ReturnExpression
           try•{4}     TryBlockExpression
                4      ExpressionStatement, Literal                                                                                       */
    return;                                                                                                                               /*
    return;    ExpressionStatement
    return     ReturnExpression                                                                                                           */
	return 'aaa: loop { break 'aaa 1; };                                                                                                  /*
    return•'aaa:•loop•{•break•'aaa•1;•};    ExpressionStatement
    return•'aaa:•loop•{•break•'aaa•1;•}     ReturnExpression
           'aaa:•loop•{•break•'aaa•1;•}     LoopBlockExpression
           'aaa                             LbIdentifier
                        break•'aaa•1;       ExpressionStatement
                        break•'aaa•1        BreakExpression
                              'aaa          LbIdentifier
                                   1        Literal                                                                                       */
	loop { break ('aaa: loop { break 'aaa 1; }); }                                                                                        /*
    loop•{•break•('aaa:•loop•{•break•'aaa•1;•});•}    ExpressionStatement, LoopBlockExpression
           break•('aaa:•loop•{•break•'aaa•1;•});      ExpressionStatement
           break•('aaa:•loop•{•break•'aaa•1;•})       BreakExpression
                  'aaa:•loop•{•break•'aaa•1;•}        LoopBlockExpression
                  'aaa                                LbIdentifier
                               break•'aaa•1;          ExpressionStatement
                               break•'aaa•1           BreakExpression
                                     'aaa             LbIdentifier
                                          1           Literal                                                                             */
	'aaa: loop { break 'aaa 'bbb: loop { break 1; }; }                                                                                    /*
    'aaa:•loop•{•break•'aaa•'bbb:•loop•{•break•1;•};•}    ExpressionStatement, LoopBlockExpression
    'aaa                                                  LbIdentifier
                 break•'aaa•'bbb:•loop•{•break•1;•};      ExpressionStatement
                 break•'aaa•'bbb:•loop•{•break•1;•}       BreakExpression
                       'aaa                               LbIdentifier
                            'bbb:•loop•{•break•1;•}       LoopBlockExpression
                            'bbb                          LbIdentifier
                                         break•1;         ExpressionStatement
                                         break•1          BreakExpression
                                               1          Literal                                                                         */
	let a = 'a: loop { break 'a 1; };                                                                                                     /*
    let•a•=•'a:•loop•{•break•'a•1;•};    LetVariableDeclaration
            'a:•loop•{•break•'a•1;•}     LoopBlockExpression
            'a                           LbIdentifier
                       break•'a•1;       ExpressionStatement
                       break•'a•1        BreakExpression
                             'a          LbIdentifier
                                1        Literal                                                                                          */
	[(); return || { let tx; }];                                                                                                          /*
    [();•return•||•{•let•tx;•}];    ExpressionStatement
    [();•return•||•{•let•tx;•}]     SizedArrayLiteral
     ()                             TupleLiteral
         return•||•{•let•tx;•}      ReturnExpression
                ||•{•let•tx;•}      ClosureFunctionExpression
                   {•let•tx;•}      BlockExpression
                     let•tx;        LetVariableDeclaration                                                                                */
	[(); return];                                                                                                                         /*
    [();•return];    ExpressionStatement
    [();•return]     SizedArrayLiteral
     ()              TupleLiteral
         return      ReturnExpression                                                                                                     */
	[(); return match 0 { n => n }];                                                                                                      /*
    [();•return•match•0•{•n•=>•n•}];    ExpressionStatement
    [();•return•match•0•{•n•=>•n•}]     SizedArrayLiteral
     ()                                 TupleLiteral
         return•match•0•{•n•=>•n•}      ReturnExpression
                match•0•{•n•=>•n•}      MatchExpression
                      0                 Literal
                          n•=>•n        MatchExpressionCase                                                                               */
	[(); return match () { 'a' => 0, _ => 0 }];                                                                                           /*
    [();•return•match•()•{•'a'•=>•0,•_•=>•0•}];    ExpressionStatement
    [();•return•match•()•{•'a'•=>•0,•_•=>•0•}]     SizedArrayLiteral
     ()                                            TupleLiteral
         return•match•()•{•'a'•=>•0,•_•=>•0•}      ReturnExpression
                match•()•{•'a'•=>•0,•_•=>•0•}      MatchExpression
                      ()                           TupleLiteral
                           'a'•=>•0                MatchExpressionCase
                           'a'                     Literal
                                  0                Literal
                                     _•=>•0        MatchExpressionCase
                                     _             WildcardPattern
                                          0        Literal                                                                                */
	let a = loop { break { return 0; () } };                                                                                              /*
    let•a•=•loop•{•break•{•return•0;•()•}•};    LetVariableDeclaration
            loop•{•break•{•return•0;•()•}•}     LoopBlockExpression
                   break•{•return•0;•()•}       ExpressionStatement, BreakExpression
                         {•return•0;•()•}       BlockExpression
                           return•0;            ExpressionStatement
                           return•0             ReturnExpression
                                  0             Literal
                                     ()         ExpressionStatement, TupleLiteral                                                         */
	let a = loop { break break };                                                                                                         /*
    let•a•=•loop•{•break•break•};    LetVariableDeclaration
            loop•{•break•break•}     LoopBlockExpression
                   break•break       ExpressionStatement, BreakExpression
                         break       BreakExpression                                                                                      */
	let a = loop { break loop {} };                                                                                                       /*
    let•a•=•loop•{•break•loop•{}•};    LetVariableDeclaration
            loop•{•break•loop•{}•}     LoopBlockExpression
                   break•loop•{}       ExpressionStatement, BreakExpression
                         loop•{}       LoopBlockExpression                                                                                */
	let a = loop { break return 0 };                                                                                                      /*
    let•a•=•loop•{•break•return•0•};    LetVariableDeclaration
            loop•{•break•return•0•}     LoopBlockExpression
                   break•return•0       ExpressionStatement, BreakExpression
                         return•0       ReturnExpression
                                0       Literal                                                                                           */
	loop { if break { } }                                                                                                                 /*
    loop•{•if•break•{•}•}    ExpressionStatement, LoopBlockExpression
           if•break•{•}      ExpressionStatement, IfBlockExpression
              break          BreakExpression                                                                                              */
	for _ in { return (); 0..3 } {};                                                                                                      /*
    for•_•in•{•return•();•0..3•}•{};    ExpressionStatement
    for•_•in•{•return•();•0..3•}•{}     ForInBlockExpression
        _                               WildcardPattern
             {•return•();•0..3•}        BlockExpression
               return•();               ExpressionStatement
               return•()                ReturnExpression
                      ()                TupleLiteral
                          0..3          ExpressionStatement, RangeLiteral
                          0             Literal
                             3          Literal                                                                                           */
	loop {                                                                                                                                /*
    loop•{↲    <ExpressionStatement>, <LoopBlockExpression>                                                                               */
        while (return) { if (return) { match (return) { 1 => { if (return) { return } else { return } } _ => { return } }; } else if (return) { return; } }/*
        while•(return)•{•if•(return)•{•match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•};•}•else•if•(return)•{•return;•}•}    ExpressionStatement, WhileBlockExpression
               return                                                                                                                                          ReturnExpression
                         if•(return)•{•match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•};•}•else•if•(return)•{•return;•}      ExpressionStatement, IfBlockExpression
                             return                                                                                                                            ReturnExpression
                                       match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•};                                     ExpressionStatement
                                       match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•}                                      MatchExpression
                                              return                                                                                                           ReturnExpression
                                                        1•=>•{•if•(return)•{•return•}•else•{•return•}•}                                                        MatchExpressionCase
                                                        1                                                                                                      Literal
                                                             {•if•(return)•{•return•}•else•{•return•}•}                                                        BlockExpression
                                                               if•(return)•{•return•}•else•{•return•}                                                          ExpressionStatement, IfBlockExpression
                                                                   return                                                                                      ReturnExpression
                                                                             return                                                                            ExpressionStatement, ReturnExpression
                                                                                           {•return•}                                                          BlockExpression
                                                                                             return                                                            ExpressionStatement, ReturnExpression
                                                                                                        _•=>•{•return•}                                        MatchExpressionCase
                                                                                                        _                                                      WildcardPattern
                                                                                                             {•return•}                                        BlockExpression
                                                                                                               return                                          ExpressionStatement, ReturnExpression
                                                                                                                                  if•(return)•{•return;•}      IfBlockExpression
                                                                                                                                      return                   ReturnExpression
                                                                                                                                                return;        ExpressionStatement
                                                                                                                                                return         ReturnExpression*/
        if (return) { break; }                                                                                                            /*
        if•(return)•{•break;•}    ExpressionStatement, IfBlockExpression
            return                ReturnExpression
                      break;      ExpressionStatement
                      break       BreakExpression                                                                                         */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </LoopBlockExpression>                                                                                   */
	let () = if 0 {} else { return }                                                                                                      /*
    let•()•=•if•0•{}•else•{•return•}    LetVariableDeclaration
        ()                              TuplePattern
             if•0•{}•else•{•return•}    IfBlockExpression
                0                       Literal
                          {•return•}    BlockExpression
                            return      ExpressionStatement, ReturnExpression                                                             */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 7
// Parsed Nodes: 203
// state_rollbacks: 1
// Total '.charCodeAt()' calls: 1164 (26% re-reads)
// Unnecessary 'skip_whitespace()' calls: 57
// source: "../../samples/expressions/flow_expr.rs"