pub fn main() {                                                                                                                           /*
pub•fn•main()•{↲    <Program>
pub•fn•main()•{↲    <Program.ast{dk: "None"}>
pub•fn•main()•{↲    <FunctionDeclaration>
pub                 PubSpecifier
           ()       FunctionDeclaration.parameters{dk: "()"}
              {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                  */
    loop { return break as (); }                                                                                                          /*
    loop•{•return•break•as•();•}    ExpressionStatement{!semi}, LoopBlockExpression
         {•return•break•as•();•}    LoopBlockExpression.body{dk: "{}"}
           return•break•as•();      ExpressionStatement{semi}
           return•break•as•()       ReturnExpression
                  break•as•()       ExpressionAsTypeCast
                  break             BreakExpression
                           ()       TypeTuple                                                                                             */
    return ();                                                                                                                            /*
    return•();    ExpressionStatement{semi}
    return•()     ReturnExpression
           ()     TupleLiteral                                                                                                            */
    return as ();                                                                                                                         /*
    return•as•();    ExpressionStatement{semi}
    return•as•()     ExpressionAsTypeCast
    return           ReturnExpression
              ()     TypeTuple                                                                                                            */
    return if 1 {()} else {()};                                                                                                           /*
    return•if•1•{()}•else•{()};    ExpressionStatement{semi}
    return•if•1•{()}•else•{()}     ReturnExpression
           if•1•{()}•else•{()}     IfBlockExpression
              1                    Literal{kind: Integer}
                {()}               IfBlockExpression.body{dk: "{}"}
                 ()                ExpressionStatement{!semi}, TupleLiteral
                          {()}     BlockExpression
                           ()      ExpressionStatement{!semi}, TupleLiteral                                                               */
    return return as ();                                                                                                                  /*
    return•return•as•();    ExpressionStatement{semi}
    return•return•as•()     ReturnExpression
           return•as•()     ExpressionAsTypeCast
           return           ReturnExpression
                     ()     TypeTuple                                                                                                     */
    return return return;                                                                                                                 /*
    return•return•return;    ExpressionStatement{semi}
    return•return•return     ReturnExpression
           return•return     ReturnExpression
                  return     ReturnExpression                                                                                             */
	return try {4};                                                                                                                       /*
	return•try•{4};    ExpressionStatement{semi}
	return•try•{4}     ReturnExpression
	       try•{4}     TryBlockExpression
	           {4}     TryBlockExpression.body{dk: "{}"}
	            4      ExpressionStatement{!semi}, Literal{kind: Integer}                                                                 */
    return;                                                                                                                               /*
    return;    ExpressionStatement{semi}
    return     ReturnExpression                                                                                                           */
	return 'aaa: loop { break 'aaa 1; };                                                                                                  /*
	return•'aaa:•loop•{•break•'aaa•1;•};    ExpressionStatement{semi}
	return•'aaa:•loop•{•break•'aaa•1;•}     ReturnExpression
	       'aaa:•loop•{•break•'aaa•1;•}     LoopBlockExpression
	       'aaa                             LbIdentifier
	                  {•break•'aaa•1;•}     LoopBlockExpression.body{dk: "{}"}
	                    break•'aaa•1;       ExpressionStatement{semi}
	                    break•'aaa•1        BreakExpression
	                          'aaa          LbIdentifier
	                               1        Literal{kind: Integer}                                                                        */
	loop { break ('aaa: loop { break 'aaa 1; }); }                                                                                        /*
	loop•{•break•('aaa:•loop•{•break•'aaa•1;•});•}    ExpressionStatement{!semi}, LoopBlockExpression
	     {•break•('aaa:•loop•{•break•'aaa•1;•});•}    LoopBlockExpression.body{dk: "{}"}
	       break•('aaa:•loop•{•break•'aaa•1;•});      ExpressionStatement{semi}
	       break•('aaa:•loop•{•break•'aaa•1;•})       BreakExpression
	              'aaa:•loop•{•break•'aaa•1;•}        LoopBlockExpression
	              'aaa                                LbIdentifier
	                         {•break•'aaa•1;•}        LoopBlockExpression.body{dk: "{}"}
	                           break•'aaa•1;          ExpressionStatement{semi}
	                           break•'aaa•1           BreakExpression
	                                 'aaa             LbIdentifier
	                                      1           Literal{kind: Integer}                                                              */
	'aaa: loop { break 'aaa 'bbb: loop { break 1; }; }                                                                                    /*
	'aaa:•loop•{•break•'aaa•'bbb:•loop•{•break•1;•};•}    ExpressionStatement{!semi}, LoopBlockExpression
	'aaa                                                  LbIdentifier
	           {•break•'aaa•'bbb:•loop•{•break•1;•};•}    LoopBlockExpression.body{dk: "{}"}
	             break•'aaa•'bbb:•loop•{•break•1;•};      ExpressionStatement{semi}
	             break•'aaa•'bbb:•loop•{•break•1;•}       BreakExpression
	                   'aaa                               LbIdentifier
	                        'bbb:•loop•{•break•1;•}       LoopBlockExpression
	                        'bbb                          LbIdentifier
	                                   {•break•1;•}       LoopBlockExpression.body{dk: "{}"}
	                                     break•1;         ExpressionStatement{semi}
	                                     break•1          BreakExpression
	                                           1          Literal{kind: Integer}                                                          */
	let a = 'a: loop { break 'a 1; };                                                                                                     /*
	let•a•=•'a:•loop•{•break•'a•1;•};    LetVariableDeclaration
	        'a:•loop•{•break•'a•1;•}     LoopBlockExpression
	        'a                           LbIdentifier
	                 {•break•'a•1;•}     LoopBlockExpression.body{dk: "{}"}
	                   break•'a•1;       ExpressionStatement{semi}
	                   break•'a•1        BreakExpression
	                         'a          LbIdentifier
	                            1        Literal{kind: Integer}                                                                           */
	[(); return || { let tx; }];                                                                                                          /*
	[();•return•||•{•let•tx;•}];    ExpressionStatement{semi}
	[();•return•||•{•let•tx;•}]     SizedArrayLiteral
	 ()                             TupleLiteral
	     return•||•{•let•tx;•}      ReturnExpression
	            ||•{•let•tx;•}      ClosureFunctionExpression
	            ||                  ClosureFunctionExpression.parameters{dk: "||"}
	               {•let•tx;•}      BlockExpression
	                 let•tx;        LetVariableDeclaration                                                                                */
	[(); return];                                                                                                                         /*
	[();•return];    ExpressionStatement{semi}
	[();•return]     SizedArrayLiteral
	 ()              TupleLiteral
	     return      ReturnExpression                                                                                                     */
	[(); return match 0 { n => n }];                                                                                                      /*
	[();•return•match•0•{•n•=>•n•}];    ExpressionStatement{semi}
	[();•return•match•0•{•n•=>•n•}]     SizedArrayLiteral
	 ()                                 TupleLiteral
	     return•match•0•{•n•=>•n•}      ReturnExpression
	            match•0•{•n•=>•n•}      MatchExpression
	                  0                 Literal{kind: Integer}
	                    {•n•=>•n•}      MatchExpression.cases{dk: "{}"}
	                      n•=>•n        MatchExpressionCase                                                                               */
	[(); return match () { 'a' => 0, _ => 0 }];                                                                                           /*
	[();•return•match•()•{•'a'•=>•0,•_•=>•0•}];    ExpressionStatement{semi}
	[();•return•match•()•{•'a'•=>•0,•_•=>•0•}]     SizedArrayLiteral
	 ()                                            TupleLiteral
	     return•match•()•{•'a'•=>•0,•_•=>•0•}      ReturnExpression
	            match•()•{•'a'•=>•0,•_•=>•0•}      MatchExpression
	                  ()                           TupleLiteral
	                     {•'a'•=>•0,•_•=>•0•}      MatchExpression.cases{dk: "{}"}
	                       'a'•=>•0                MatchExpressionCase
	                       'a'                     Literal{kind: Char}
	                              0                Literal{kind: Integer}
	                                 _•=>•0        MatchExpressionCase
	                                 _             WildcardPattern
	                                      0        Literal{kind: Integer}                                                                 */
	let a = loop { break { return 0; () } };                                                                                              /*
	let•a•=•loop•{•break•{•return•0;•()•}•};    LetVariableDeclaration
	        loop•{•break•{•return•0;•()•}•}     LoopBlockExpression
	             {•break•{•return•0;•()•}•}     LoopBlockExpression.body{dk: "{}"}
	               break•{•return•0;•()•}       ExpressionStatement{!semi}, BreakExpression
	                     {•return•0;•()•}       BlockExpression
	                       return•0;            ExpressionStatement{semi}
	                       return•0             ReturnExpression
	                              0             Literal{kind: Integer}
	                                 ()         ExpressionStatement{!semi}, TupleLiteral                                                  */
	let a = loop { break break };                                                                                                         /*
	let•a•=•loop•{•break•break•};    LetVariableDeclaration
	        loop•{•break•break•}     LoopBlockExpression
	             {•break•break•}     LoopBlockExpression.body{dk: "{}"}
	               break•break       ExpressionStatement{!semi}, BreakExpression
	                     break       BreakExpression                                                                                      */
	let a = loop { break loop {} };                                                                                                       /*
	let•a•=•loop•{•break•loop•{}•};    LetVariableDeclaration
	        loop•{•break•loop•{}•}     LoopBlockExpression
	             {•break•loop•{}•}     LoopBlockExpression.body{dk: "{}"}
	               break•loop•{}       ExpressionStatement{!semi}, BreakExpression
	                     loop•{}       LoopBlockExpression
	                          {}       LoopBlockExpression.body{dk: "{}"}                                                                 */
	let a = loop { break return 0 };                                                                                                      /*
	let•a•=•loop•{•break•return•0•};    LetVariableDeclaration
	        loop•{•break•return•0•}     LoopBlockExpression
	             {•break•return•0•}     LoopBlockExpression.body{dk: "{}"}
	               break•return•0       ExpressionStatement{!semi}, BreakExpression
	                     return•0       ReturnExpression
	                            0       Literal{kind: Integer}                                                                            */
	loop { if break { } }                                                                                                                 /*
	loop•{•if•break•{•}•}    ExpressionStatement{!semi}, LoopBlockExpression
	     {•if•break•{•}•}    LoopBlockExpression.body{dk: "{}"}
	       if•break•{•}      ExpressionStatement{!semi}, IfBlockExpression
	          break          BreakExpression
	                {•}      IfBlockExpression.body{dk: "{}"}                                                                             */
	for _ in { return (); 0..3 } {};                                                                                                      /*
	for•_•in•{•return•();•0..3•}•{};    ExpressionStatement{semi}
	for•_•in•{•return•();•0..3•}•{}     ForInBlockExpression
	    _                               WildcardPattern
	         {•return•();•0..3•}        BlockExpression
	           return•();               ExpressionStatement{semi}
	           return•()                ReturnExpression
	                  ()                TupleLiteral
	                      0..3          ExpressionStatement{!semi}, RangeLiteral{!last}
	                      0             Literal{kind: Integer}
	                         3          Literal{kind: Integer}
	                             {}     ForInBlockExpression.body{dk: "{}"}                                                               */
	loop {                                                                                                                                /*
	loop•{↲    <ExpressionStatement{!semi}>
	loop•{↲    <LoopBlockExpression>
	     {↲    <LoopBlockExpression.body{dk: "{}"}>                                                                                       */
        while (return) { if (return) { match (return) { 1 => { if (return) { return } else { return } } _ => { return } }; } else if (return) { return; } }/*
        while•(return)•{•if•(return)•{•match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•};•}•else•if•(return)•{•return;•}•}    ExpressionStatement{!semi}, WhileBlockExpression
               return                                                                                                                                          ReturnExpression
                       {•if•(return)•{•match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•};•}•else•if•(return)•{•return;•}•}    WhileBlockExpression.body{dk: "{}"}
                         if•(return)•{•match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•};•}•else•if•(return)•{•return;•}      ExpressionStatement{!semi}, IfBlockExpression
                             return                                                                                                                            ReturnExpression
                                     {•match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•};•}                                   IfBlockExpression.body{dk: "{}"}
                                       match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•};                                     ExpressionStatement{semi}
                                       match•(return)•{•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•}                                      MatchExpression
                                              return                                                                                                           ReturnExpression
                                                      {•1•=>•{•if•(return)•{•return•}•else•{•return•}•}•_•=>•{•return•}•}                                      MatchExpression.cases{dk: "{}"}
                                                        1•=>•{•if•(return)•{•return•}•else•{•return•}•}                                                        MatchExpressionCase
                                                        1                                                                                                      Literal{kind: Integer}
                                                             {•if•(return)•{•return•}•else•{•return•}•}                                                        BlockExpression
                                                               if•(return)•{•return•}•else•{•return•}                                                          ExpressionStatement{!semi}, IfBlockExpression
                                                                   return                                                                                      ReturnExpression
                                                                           {•return•}                                                                          IfBlockExpression.body{dk: "{}"}
                                                                             return                                                                            ExpressionStatement{!semi}, ReturnExpression
                                                                                           {•return•}                                                          BlockExpression
                                                                                             return                                                            ExpressionStatement{!semi}, ReturnExpression
                                                                                                        _•=>•{•return•}                                        MatchExpressionCase
                                                                                                        _                                                      WildcardPattern
                                                                                                             {•return•}                                        BlockExpression
                                                                                                               return                                          ExpressionStatement{!semi}, ReturnExpression
                                                                                                                                  if•(return)•{•return;•}      IfBlockExpression
                                                                                                                                      return                   ReturnExpression
                                                                                                                                              {•return;•}      IfBlockExpression.body{dk: "{}"}
                                                                                                                                                return;        ExpressionStatement{semi}
                                                                                                                                                return         ReturnExpression*/
        if (return) { break; }                                                                                                            /*
        if•(return)•{•break;•}    ExpressionStatement{!semi}, IfBlockExpression
            return                ReturnExpression
                    {•break;•}    IfBlockExpression.body{dk: "{}"}
                      break;      ExpressionStatement{semi}
                      break       BreakExpression                                                                                         */
    }                                                                                                                                     /*
••••}    </LoopBlockExpression.body>
••••}    </LoopBlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
	let () = if 0 {} else { return }                                                                                                      /*
	let•()•=•if•0•{}•else•{•return•}    LetVariableDeclaration
	    ()                              TuplePattern
	         if•0•{}•else•{•return•}    IfBlockExpression
	            0                       Literal{kind: Integer}
	              {}                    IfBlockExpression.body{dk: "{}"}
	                      {•return•}    BlockExpression
	                        return      ExpressionStatement{!semi}, ReturnExpression                                                      */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 7
// Parsed Nodes: 203
// state_rollbacks: 1
// Total '.charCodeAt()' calls: 1164 (26% re-reads)
// Unnecessary 'skip_whitespace()' calls: 57
// source: "../../samples/expressions/flow_expr.rs"