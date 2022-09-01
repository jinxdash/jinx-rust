fn main() {                                                                                                                               /*
fn•main()•{↲    <Program>
fn•main()•{↲    <Program.ast{dk: "None"}>
fn•main()•{↲    <FunctionDeclaration>
       ()       FunctionDeclaration.parameters{dk: "()"}
          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                      */
	let _: &'static _ = &|| { let _ = 0; 0 };                                                                                             /*
	let•_:•&'static•_•=•&||•{•let•_•=•0;•0•};    LetVariableDeclaration
	    _                                        WildcardPattern
	       &'static•_                            TypeReference{!mut}
	        'static                              LtStatic
	                _                            TypeInferred
	                    &||•{•let•_•=•0;•0•}     ReferenceExpression{!mut}
	                     ||•{•let•_•=•0;•0•}     ClosureFunctionExpression
	                     ||                      ClosureFunctionExpression.parameters{dk: "||"}
	                        {•let•_•=•0;•0•}     BlockExpression
	                          let•_•=•0;         LetVariableDeclaration
	                              _              WildcardPattern
	                                  0          Literal{kind: Integer}
	                                     0       ExpressionStatement{!semi}, Literal{kind: Integer}                                       */
	let _ = match c(o.m(), o as T::T) {                                                                                                   /*
	let•_•=•match•c(o.m(),•o•as•T::T)•{↲    <LetVariableDeclaration>
	    _                                   WildcardPattern
	        match•c(o.m(),•o•as•T::T)•{↲    <ExpressionAsTypeCast>
	        match•c(o.m(),•o•as•T::T)•{↲    <MatchExpression>
	              c(o.m(),•o•as•T::T)       CallExpression
	               (o.m(),•o•as•T::T)       CallExpression.arguments{dk: "()"}
	                o.m()                   CallExpression
	                   ()                   CallExpression.arguments{dk: "()"}
	                       o•as•T::T        ExpressionAsTypeCast
	                            T::T        TypePath
	                                  {↲    <MatchExpression.cases{dk: "{}"}>                                                             */
		0 if o::c() == 0 => 0,                                                                                                            /*
		0•if•o::c()•==•0•=>•0    MatchExpressionCase
		0                        Literal{kind: Integer}
		     o::c()•==•0         ComparisonExpression{tk: "=="}
		     o::c()              CallExpression
		     o::c                ExpressionPath
		         ()              CallExpression.arguments{dk: "()"}
		               0         Literal{kind: Integer}
		                    0    Literal{kind: Integer}                                                                                   */
		0 => return c(o::c()),                                                                                                            /*
		0•=>•return•c(o::c())    MatchExpressionCase
		0                        Literal{kind: Integer}
		     return•c(o::c())    ReturnExpression
		            c(o::c())    CallExpression
		             (o::c())    CallExpression.arguments{dk: "()"}
		              o::c()     CallExpression
		              o::c       ExpressionPath
		                  ()     CallExpression.arguments{dk: "()"}                                                                       */
	} as T;                                                                                                                               /*
   ╚}          </MatchExpression.cases>
   ╚}          </MatchExpression>
   ╚}•as•T     </ExpressionAsTypeCast>
   ╚}•as•T;    </LetVariableDeclaration>                                                                                                  */

	let _ = if  0 == 0 && 0 == 0 						{ 	0 == 0 && 0 == 0 						} else { 0 }                          /*
	let•_•=•if••0•==•0•&&•0•==•0•╚╚╚╚╚╚{•╚0•==•0•&&•0•==•0•╚╚╚╚╚╚}•else•{•0•}                                       LetVariableDeclaration
	    _                                                                                                           WildcardPattern
	        if••0•==•0•&&•0•==•0•╚╚╚╚╚╚{•╚0•==•0•&&•0•==•0•╚╚╚╚╚╚}•else•{•0•}                                       IfBlockExpression
	            0•==•0•&&•0•==•0                                                                                    AndExpression{tk: "&&"}
	            0•==•0                                                                                              ComparisonExpression{tk: "=="}
	            0                                                                                                   Literal{kind: Integer}
	                 0                                                                                              Literal{kind: Integer}
	                      0•==•0                                                                                    ComparisonExpression{tk: "=="}
	                      0                                                                                         Literal{kind: Integer}
	                           0                                                                                    Literal{kind: Integer}
	                             						{•╚0•==•0•&&•0•==•0•╚╚╚╚╚╚}                                 IfBlockExpression.body{dk: "{}"}
	                             						  	0•==•0•&&•0•==•0                                        ExpressionStatement{!semi}, AndExpression{tk: "&&"}
	                             						  	0•==•0                                                  ComparisonExpression{tk: "=="}
	                             						  	0                                                       Literal{kind: Integer}
	                             						  	     0                                                  Literal{kind: Integer}
	                             						  	          0•==•0                                        ComparisonExpression{tk: "=="}
	                             						  	          0                                             Literal{kind: Integer}
	                             						  	               0                                        Literal{kind: Integer}
	                             						  	                 						       {•0•}    BlockExpression
	                             						  	                 						         0      ExpressionStatement{!semi}, Literal{kind: Integer}*/
	let _ = if  0 == 0 || 0 == 0 						{	0 == 0 || 0 == 0 						} else { 0 }                          /*
	let•_•=•if••0•==•0•||•0•==•0•╚╚╚╚╚╚{╚0•==•0•||•0•==•0•╚╚╚╚╚╚}•else•{•0•}                                        LetVariableDeclaration
	    _                                                                                                           WildcardPattern
	        if••0•==•0•||•0•==•0•╚╚╚╚╚╚{╚0•==•0•||•0•==•0•╚╚╚╚╚╚}•else•{•0•}                                        IfBlockExpression
	            0•==•0•||•0•==•0                                                                                    OrExpression{tk: "||"}
	            0•==•0                                                                                              ComparisonExpression{tk: "=="}
	            0                                                                                                   Literal{kind: Integer}
	                 0                                                                                              Literal{kind: Integer}
	                      0•==•0                                                                                    ComparisonExpression{tk: "=="}
	                      0                                                                                         Literal{kind: Integer}
	                           0                                                                                    Literal{kind: Integer}
	                             						{╚0•==•0•||•0•==•0•╚╚╚╚╚╚}                                  IfBlockExpression.body{dk: "{}"}
	                             						 	0•==•0•||•0•==•0                                        ExpressionStatement{!semi}, OrExpression{tk: "||"}
	                             						 	0•==•0                                                  ComparisonExpression{tk: "=="}
	                             						 	0                                                       Literal{kind: Integer}
	                             						 	     0                                                  Literal{kind: Integer}
	                             						 	          0•==•0                                        ComparisonExpression{tk: "=="}
	                             						 	          0                                             Literal{kind: Integer}
	                             						 	               0                                        Literal{kind: Integer}
	                             						 	                 						       {•0•}    BlockExpression
	                             						 	                 						         0      ExpressionStatement{!semi}, Literal{kind: Integer}*/
	let _ = if (0 == 0 || 0 == 0) && (0 == 0 || 0 == 0) {  (0 == 0 || 0 == 0) && (0 == 0 || 0 == 0) } else { 0 }                          /*
	let•_•=•if•(0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)•{••(0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)•}•else•{•0•}    LetVariableDeclaration
	    _                                                                                                           WildcardPattern
	        if•(0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)•{••(0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)•}•else•{•0•}    IfBlockExpression
	           (0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)                                                             AndExpression{tk: "&&"}
	            0•==•0•||•0•==•0                                                                                    OrExpression{tk: "||"}
	            0•==•0                                                                                              ComparisonExpression{tk: "=="}
	            0                                                                                                   Literal{kind: Integer}
	                 0                                                                                              Literal{kind: Integer}
	                      0•==•0                                                                                    ComparisonExpression{tk: "=="}
	                      0                                                                                         Literal{kind: Integer}
	                           0                                                                                    Literal{kind: Integer}
	                                  0•==•0•||•0•==•0                                                              OrExpression{tk: "||"}
	                                  0•==•0                                                                        ComparisonExpression{tk: "=="}
	                                  0                                                                             Literal{kind: Integer}
	                                       0                                                                        Literal{kind: Integer}
	                                            0•==•0                                                              ComparisonExpression{tk: "=="}
	                                            0                                                                   Literal{kind: Integer}
	                                                 0                                                              Literal{kind: Integer}
	                                                    {••(0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)•}               IfBlockExpression.body{dk: "{}"}
	                                                       (0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)                 ExpressionStatement{!semi}, AndExpression{tk: "&&"}
	                                                        0•==•0•||•0•==•0                                        OrExpression{tk: "||"}
	                                                        0•==•0                                                  ComparisonExpression{tk: "=="}
	                                                        0                                                       Literal{kind: Integer}
	                                                             0                                                  Literal{kind: Integer}
	                                                                  0•==•0                                        ComparisonExpression{tk: "=="}
	                                                                  0                                             Literal{kind: Integer}
	                                                                       0                                        Literal{kind: Integer}
	                                                                              0•==•0•||•0•==•0                  OrExpression{tk: "||"}
	                                                                              0•==•0                            ComparisonExpression{tk: "=="}
	                                                                              0                                 Literal{kind: Integer}
	                                                                                   0                            Literal{kind: Integer}
	                                                                                        0•==•0                  ComparisonExpression{tk: "=="}
	                                                                                        0                       Literal{kind: Integer}
	                                                                                             0                  Literal{kind: Integer}
	                                                                                                       {•0•}    BlockExpression
	                                                                                                         0      ExpressionStatement{!semi}, Literal{kind: Integer}*/
	let _ = if  0 == 0 && 0 == 0  && (0 == 0 || 0 == 0) { 	0 == 0 && 0 == 0  && (0 == 0 || 0 == 0) } else { 0 }                          /*
	let•_•=•if••0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)•{•╚0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)•}•else•{•0•}     LetVariableDeclaration
	    _                                                                                                           WildcardPattern
	        if••0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)•{•╚0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)•}•else•{•0•}     IfBlockExpression
	            0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)                                                             AndExpression{tk: "&&"}
	            0•==•0•&&•0•==•0                                                                                    AndExpression{tk: "&&"}
	            0•==•0                                                                                              ComparisonExpression{tk: "=="}
	            0                                                                                                   Literal{kind: Integer}
	                 0                                                                                              Literal{kind: Integer}
	                      0•==•0                                                                                    ComparisonExpression{tk: "=="}
	                      0                                                                                         Literal{kind: Integer}
	                           0                                                                                    Literal{kind: Integer}
	                                  0•==•0•||•0•==•0                                                              OrExpression{tk: "||"}
	                                  0•==•0                                                                        ComparisonExpression{tk: "=="}
	                                  0                                                                             Literal{kind: Integer}
	                                       0                                                                        Literal{kind: Integer}
	                                            0•==•0                                                              ComparisonExpression{tk: "=="}
	                                            0                                                                   Literal{kind: Integer}
	                                                 0                                                              Literal{kind: Integer}
	                                                    {•╚0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)•}                IfBlockExpression.body{dk: "{}"}
	                                                      	0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)                 ExpressionStatement{!semi}, AndExpression{tk: "&&"}
	                                                      	0•==•0•&&•0•==•0                                        AndExpression{tk: "&&"}
	                                                      	0•==•0                                                  ComparisonExpression{tk: "=="}
	                                                      	0                                                       Literal{kind: Integer}
	                                                      	     0                                                  Literal{kind: Integer}
	                                                      	          0•==•0                                        ComparisonExpression{tk: "=="}
	                                                      	          0                                             Literal{kind: Integer}
	                                                      	               0                                        Literal{kind: Integer}
	                                                      	                      0•==•0•||•0•==•0                  OrExpression{tk: "||"}
	                                                      	                      0•==•0                            ComparisonExpression{tk: "=="}
	                                                      	                      0                                 Literal{kind: Integer}
	                                                      	                           0                            Literal{kind: Integer}
	                                                      	                                0•==•0                  ComparisonExpression{tk: "=="}
	                                                      	                                0                       Literal{kind: Integer}
	                                                      	                                     0                  Literal{kind: Integer}
	                                                      	                                               {•0•}    BlockExpression
	                                                      	                                                 0      ExpressionStatement{!semi}, Literal{kind: Integer}*/
	let _ = if (0 == 0 || 0 == 0) &&  0 == 0 && 0 == 0  {  (0 == 0 || 0 == 0) &&  0 == 0 && 0 == 0  } else { 0 }                          /*
	let•_•=•if•(0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0••{••(0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0••}•else•{•0•}    LetVariableDeclaration
	    _                                                                                                           WildcardPattern
	        if•(0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0••{••(0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0••}•else•{•0•}    IfBlockExpression
	           (0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0                                                              AndExpression{tk: "&&"}
	           (0•==•0•||•0•==•0)•&&••0•==•0                                                                        AndExpression{tk: "&&"}
	            0•==•0•||•0•==•0                                                                                    OrExpression{tk: "||"}
	            0•==•0                                                                                              ComparisonExpression{tk: "=="}
	            0                                                                                                   Literal{kind: Integer}
	                 0                                                                                              Literal{kind: Integer}
	                      0•==•0                                                                                    ComparisonExpression{tk: "=="}
	                      0                                                                                         Literal{kind: Integer}
	                           0                                                                                    Literal{kind: Integer}
	                                  0•==•0                                                                        ComparisonExpression{tk: "=="}
	                                  0                                                                             Literal{kind: Integer}
	                                       0                                                                        Literal{kind: Integer}
	                                            0•==•0                                                              ComparisonExpression{tk: "=="}
	                                            0                                                                   Literal{kind: Integer}
	                                                 0                                                              Literal{kind: Integer}
	                                                    {••(0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0••}               IfBlockExpression.body{dk: "{}"}
	                                                       (0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0                  ExpressionStatement{!semi}, AndExpression{tk: "&&"}
	                                                       (0•==•0•||•0•==•0)•&&••0•==•0                            AndExpression{tk: "&&"}
	                                                        0•==•0•||•0•==•0                                        OrExpression{tk: "||"}
	                                                        0•==•0                                                  ComparisonExpression{tk: "=="}
	                                                        0                                                       Literal{kind: Integer}
	                                                             0                                                  Literal{kind: Integer}
	                                                                  0•==•0                                        ComparisonExpression{tk: "=="}
	                                                                  0                                             Literal{kind: Integer}
	                                                                       0                                        Literal{kind: Integer}
	                                                                              0•==•0                            ComparisonExpression{tk: "=="}
	                                                                              0                                 Literal{kind: Integer}
	                                                                                   0                            Literal{kind: Integer}
	                                                                                        0•==•0                  ComparisonExpression{tk: "=="}
	                                                                                        0                       Literal{kind: Integer}
	                                                                                             0                  Literal{kind: Integer}
	                                                                                                       {•0•}    BlockExpression
	                                                                                                         0      ExpressionStatement{!semi}, Literal{kind: Integer}*/
	let _ = if  0 == 0 && 0 == 0  &&  0 == 0 && 0 == 0  { 	0 == 0 && 0 == 0  &&  0 == 0 && 0 == 0  } else { 0 }                          /*
	let•_•=•if••0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0••{•╚0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0••}•else•{•0•}     LetVariableDeclaration
	    _                                                                                                           WildcardPattern
	        if••0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0••{•╚0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0••}•else•{•0•}     IfBlockExpression
	            0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0                                                              AndExpression{tk: "&&"}
	            0•==•0•&&•0•==•0••&&••0•==•0                                                                        AndExpression{tk: "&&"}
	            0•==•0•&&•0•==•0                                                                                    AndExpression{tk: "&&"}
	            0•==•0                                                                                              ComparisonExpression{tk: "=="}
	            0                                                                                                   Literal{kind: Integer}
	                 0                                                                                              Literal{kind: Integer}
	                      0•==•0                                                                                    ComparisonExpression{tk: "=="}
	                      0                                                                                         Literal{kind: Integer}
	                           0                                                                                    Literal{kind: Integer}
	                                  0•==•0                                                                        ComparisonExpression{tk: "=="}
	                                  0                                                                             Literal{kind: Integer}
	                                       0                                                                        Literal{kind: Integer}
	                                            0•==•0                                                              ComparisonExpression{tk: "=="}
	                                            0                                                                   Literal{kind: Integer}
	                                                 0                                                              Literal{kind: Integer}
	                                                    {•╚0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0••}                IfBlockExpression.body{dk: "{}"}
	                                                      	0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0                  ExpressionStatement{!semi}, AndExpression{tk: "&&"}
	                                                      	0•==•0•&&•0•==•0••&&••0•==•0                            AndExpression{tk: "&&"}
	                                                      	0•==•0•&&•0•==•0                                        AndExpression{tk: "&&"}
	                                                      	0•==•0                                                  ComparisonExpression{tk: "=="}
	                                                      	0                                                       Literal{kind: Integer}
	                                                      	     0                                                  Literal{kind: Integer}
	                                                      	          0•==•0                                        ComparisonExpression{tk: "=="}
	                                                      	          0                                             Literal{kind: Integer}
	                                                      	               0                                        Literal{kind: Integer}
	                                                      	                      0•==•0                            ComparisonExpression{tk: "=="}
	                                                      	                      0                                 Literal{kind: Integer}
	                                                      	                           0                            Literal{kind: Integer}
	                                                      	                                0•==•0                  ComparisonExpression{tk: "=="}
	                                                      	                                0                       Literal{kind: Integer}
	                                                      	                                     0                  Literal{kind: Integer}
	                                                      	                                               {•0•}    BlockExpression
	                                                      	                                                 0      ExpressionStatement{!semi}, Literal{kind: Integer}*/
	let _ = if  0 == 0 && 0 != 0 						{ 	0 == 0 && 0 != 0 						} else { 0 }                          /*
	let•_•=•if••0•==•0•&&•0•!=•0•╚╚╚╚╚╚{•╚0•==•0•&&•0•!=•0•╚╚╚╚╚╚}•else•{•0•}                                       LetVariableDeclaration
	    _                                                                                                           WildcardPattern
	        if••0•==•0•&&•0•!=•0•╚╚╚╚╚╚{•╚0•==•0•&&•0•!=•0•╚╚╚╚╚╚}•else•{•0•}                                       IfBlockExpression
	            0•==•0•&&•0•!=•0                                                                                    AndExpression{tk: "&&"}
	            0•==•0                                                                                              ComparisonExpression{tk: "=="}
	            0                                                                                                   Literal{kind: Integer}
	                 0                                                                                              Literal{kind: Integer}
	                      0•!=•0                                                                                    ComparisonExpression{tk: "!="}
	                      0                                                                                         Literal{kind: Integer}
	                           0                                                                                    Literal{kind: Integer}
	                             						{•╚0•==•0•&&•0•!=•0•╚╚╚╚╚╚}                                 IfBlockExpression.body{dk: "{}"}
	                             						  	0•==•0•&&•0•!=•0                                        ExpressionStatement{!semi}, AndExpression{tk: "&&"}
	                             						  	0•==•0                                                  ComparisonExpression{tk: "=="}
	                             						  	0                                                       Literal{kind: Integer}
	                             						  	     0                                                  Literal{kind: Integer}
	                             						  	          0•!=•0                                        ComparisonExpression{tk: "!="}
	                             						  	          0                                             Literal{kind: Integer}
	                             						  	               0                                        Literal{kind: Integer}
	                             						  	                 						       {•0•}    BlockExpression
	                             						  	                 						         0      ExpressionStatement{!semi}, Literal{kind: Integer}*/
	let _ = if  c!() && c!()                            { 	c!() && c!()							} else { 0 }                          /*
	let•_•=•if••c!()•&&•c!()••••••••••••••••••••••••••••{•╚c!()•&&•c!()╚╚╚╚╚╚╚}•else•{•0•}                          LetVariableDeclaration
	    _                                                                                                           WildcardPattern
	        if••c!()•&&•c!()••••••••••••••••••••••••••••{•╚c!()•&&•c!()╚╚╚╚╚╚╚}•else•{•0•}                          IfBlockExpression
	            c!()•&&•c!()                                                                                        AndExpression{tk: "&&"}
	            c!()                                                                                                MacroInvocation
	              ()                                                                                                MacroInvocation.segments{dk: "()"}
	                    c!()                                                                                        MacroInvocation
	                      ()                                                                                        MacroInvocation.segments{dk: "()"}
	                                                    {•╚c!()•&&•c!()╚╚╚╚╚╚╚}                                     IfBlockExpression.body{dk: "{}"}
	                                                      	c!()•&&•c!()                                            ExpressionStatement{!semi}, AndExpression{tk: "&&"}
	                                                      	c!()                                                    MacroInvocation
	                                                      	  ()                                                    MacroInvocation.segments{dk: "()"}
	                                                      	        c!()                                            MacroInvocation
	                                                      	          ()                                            MacroInvocation.segments{dk: "()"}
	                                                      	            							       {•0•}    BlockExpression
	                                                      	            							         0      ExpressionStatement{!semi}, Literal{kind: Integer}*/

	if let _ = 0..|| 0 {}                                                                                                                 /*
	if•let•_•=•0..||•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•_•=•0..||•0       LetScrutinee
	       _                 WildcardPattern
	           0..||•0       RangeLiteral{!last}
	           0             Literal{kind: Integer}
	              ||•0       ClosureFunctionExpression
	              ||         ClosureFunctionExpression.parameters{dk: "||"}
	                 0       Literal{kind: Integer}
	                   {}    IfBlockExpression.body{dk: "{}"}                                                                             */
	if let _ = 0..&&0 {}                                                                                                                  /*
	if•let•_•=•0..&&0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•_•=•0..&&0       LetScrutinee
	       _                WildcardPattern
	           0..&&0       RangeLiteral{!last}
	           0            Literal{kind: Integer}
	              &&0       ReferenceExpression{!mut}
	               &0       ReferenceExpression{!mut}
	                0       Literal{kind: Integer}
	                  {}    IfBlockExpression.body{dk: "{}"}                                                                              */
	if let _ = 0..0 && 0 { }                                                                                                              /*
	if•let•_•=•0..0•&&•0•{•}    ExpressionStatement{!semi}, IfBlockExpression
	   let•_•=•0..0•&&•0        AndExpression{tk: "&&"}
	   let•_•=•0..0             LetScrutinee
	       _                    WildcardPattern
	           0..0             RangeLiteral{!last}
	           0                Literal{kind: Integer}
	              0             Literal{kind: Integer}
	                   0        Literal{kind: Integer}
	                     {•}    IfBlockExpression.body{dk: "{}"}                                                                          */
	if let _ = break 0 && 0 {}                                                                                                            /*
	if•let•_•=•break•0•&&•0•{}    ExpressionStatement{!semi}, IfBlockExpression
	   let•_•=•break•0•&&•0       AndExpression{tk: "&&"}
	   let•_•=•break•0            LetScrutinee
	       _                      WildcardPattern
	           break•0            BreakExpression
	                 0            Literal{kind: Integer}
	                      0       Literal{kind: Integer}
	                        {}    IfBlockExpression.body{dk: "{}"}                                                                        */

	_ = if 0 { 0 } else { 0 };                                                                                                            /*
	_•=•if•0•{•0•}•else•{•0•};    ExpressionStatement{semi}
	_•=•if•0•{•0•}•else•{•0•}     ReassignmentExpression{tk: "="}
	_                             UnassignedExpression
	    if•0•{•0•}•else•{•0•}     IfBlockExpression
	       0                      Literal{kind: Integer}
	         {•0•}                IfBlockExpression.body{dk: "{}"}
	           0                  ExpressionStatement{!semi}, Literal{kind: Integer}
	                    {•0•}     BlockExpression
	                      0       ExpressionStatement{!semi}, Literal{kind: Integer}                                                      */
	_ = if 0 { 0 } else { 0 }();                                                                                                          /*
	_•=•if•0•{•0•}•else•{•0•}();    ExpressionStatement{semi}
	_•=•if•0•{•0•}•else•{•0•}()     ReassignmentExpression{tk: "="}
	_                               UnassignedExpression
	    if•0•{•0•}•else•{•0•}()     CallExpression
	    if•0•{•0•}•else•{•0•}       IfBlockExpression
	       0                        Literal{kind: Integer}
	         {•0•}                  IfBlockExpression.body{dk: "{}"}
	           0                    ExpressionStatement{!semi}, Literal{kind: Integer}
	                    {•0•}       BlockExpression
	                      0         ExpressionStatement{!semi}, Literal{kind: Integer}
	                         ()     CallExpression.arguments{dk: "()"}                                                                    */
	_ = if 0 { 0 } else { 0 } as *mut _;                                                                                                  /*
	_•=•if•0•{•0•}•else•{•0•}•as•*mut•_;    ExpressionStatement{semi}
	_•=•if•0•{•0•}•else•{•0•}•as•*mut•_     ReassignmentExpression{tk: "="}
	_                                       UnassignedExpression
	    if•0•{•0•}•else•{•0•}•as•*mut•_     ExpressionAsTypeCast
	    if•0•{•0•}•else•{•0•}               IfBlockExpression
	       0                                Literal{kind: Integer}
	         {•0•}                          IfBlockExpression.body{dk: "{}"}
	           0                            ExpressionStatement{!semi}, Literal{kind: Integer}
	                    {•0•}               BlockExpression
	                      0                 ExpressionStatement{!semi}, Literal{kind: Integer}
	                             *mut•_     TypeDereferenceMut
	                                  _     TypeInferred                                                                                  */

	for _ in &[c(1)] { c(0); }                                                                                                            /*
	for•_•in•&[c(1)]•{•c(0);•}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                         WildcardPattern
	         &[c(1)]              ReferenceExpression{!mut}
	          [c(1)]              ArrayLiteral
	           c(1)               CallExpression
	            (1)               CallExpression.arguments{dk: "()"}
	             1                Literal{kind: Integer}
	                 {•c(0);•}    ForInBlockExpression.body{dk: "{}"}
	                   c(0);      ExpressionStatement{semi}
	                   c(0)       CallExpression
	                    (0)       CallExpression.arguments{dk: "()"}
	                     0        Literal{kind: Integer}                                                                                  */
	for _ in -0 + 0..=(0 - 0) {}                                                                                                          /*
	for•_•in•-0•+•0..=(0•-•0)•{}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                           WildcardPattern
	         -0•+•0..=(0•-•0)       RangeLiteral{last}
	         -0•+•0                 OperationExpression{tk: "+"}
	         -0                     MinusExpression
	          0                     Literal{kind: Integer}
	              0                 Literal{kind: Integer}
	                   0•-•0        OperationExpression{tk: "-"}
	                   0            Literal{kind: Integer}
	                       0        Literal{kind: Integer}
	                          {}    ForInBlockExpression.body{dk: "{}"}                                                                   */
	for _ in 0..o.m() { o = o ^ o[o]; }                                                                                                   /*
	for•_•in•0..o.m()•{•o•=•o•^•o[o];•}    ExpressionStatement{!semi}, ForInBlockExpression
	    _                                  WildcardPattern
	         0..o.m()                      RangeLiteral{!last}
	         0                             Literal{kind: Integer}
	            o.m()                      CallExpression
	               ()                      CallExpression.arguments{dk: "()"}
	                  {•o•=•o•^•o[o];•}    ForInBlockExpression.body{dk: "{}"}
	                    o•=•o•^•o[o];      ExpressionStatement{semi}
	                    o•=•o•^•o[o]       ReassignmentExpression{tk: "="}
	                        o•^•o[o]       OperationExpression{tk: "^"}
	                            o[o]       MemberExpression{computed}                                                                     */
	for i in 0..(chunk_d - 1) {}                                                                                                          /*
	for•i•in•0..(chunk_d•-•1)•{}    ExpressionStatement{!semi}, ForInBlockExpression
	         0..(chunk_d•-•1)       RangeLiteral{!last}
	         0                      Literal{kind: Integer}
	             chunk_d•-•1        OperationExpression{tk: "-"}
	                       1        Literal{kind: Integer}
	                          {}    ForInBlockExpression.body{dk: "{}"}                                                                   */

	unsafe { *o::<u8>() = 0; }                                                                                                            /*
	unsafe•{•*o::<u8>()•=•0;•}    ExpressionStatement{!semi}, BlockExpression{unsafe}
	       {•*o::<u8>()•=•0;•}    BlockExpression.body{dk: "{}"}
	         *o::<u8>()•=•0;      ExpressionStatement{semi}
	         *o::<u8>()•=•0       ReassignmentExpression{tk: "="}
	         *o::<u8>()           DereferenceExpression
	          o::<u8>()           CallExpression
	             <u8>             CallExpression.typeArguments{dk: "<>"}
	                 ()           CallExpression.arguments{dk: "()"}
	                      0       Literal{kind: Integer}                                                                                  */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */


type o = (                                                                                                                                /*
type•o•=•(↲    <TypeAliasDeclaration>
         (↲    <TypeTuple>                                                                                                                */
	Box<dyn FnMut(u8)->u8+'static>,                                                                                                       /*
	Box<dyn•FnMut(u8)->u8+'static>    TypeCall
	   <dyn•FnMut(u8)->u8+'static>    TypeCall.typeArguments{dk: "<>"}
	    dyn•FnMut(u8)->u8+'static     TypeDynBounds{dyn}
	        FnMut(u8)->u8             TypeTraitBound{!maybeConst, !optional}, TypeFunction
	             (u8)                 TypeFunction.parameters{dk: "()"}
	                      'static     LtStatic                                                                                            */
	Box<dyn FnMut(u8)->dyn u8+'static>                                                                                                    /*
	Box<dyn•FnMut(u8)->dyn•u8+'static>    TypeCall
	   <dyn•FnMut(u8)->dyn•u8+'static>    TypeCall.typeArguments{dk: "<>"}
	    dyn•FnMut(u8)->dyn•u8+'static     TypeDynBounds{dyn}
	        FnMut(u8)->dyn•u8+'static     TypeTraitBound{!maybeConst, !optional}, TypeFunction
	             (u8)                     TypeFunction.parameters{dk: "()"}
	                   dyn•u8+'static     TypeDynBounds{dyn}
	                       u8             TypeTraitBound{!maybeConst, !optional}
	                          'static     LtStatic                                                                                        */
);                                                                                                                                        /*
)     </TypeTuple>
);    </TypeAliasDeclaration>                                                                                                             */


pub extern "ABI" fn f() {                                                                                                                 /*
pub•extern•"ABI"•fn•f()•{↲    <FunctionDeclaration>
pub                           PubSpecifier
    extern•"ABI"              ExternSpecifier
           "ABI"              Literal{kind: String}
                     ()       FunctionDeclaration.parameters{dk: "()"}
                        {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                        */
    c! { pub static T : T = T { } ; }                                                                                                     /*
    c!•{•pub•static•T•:•T•=•T•{•}•;•}    ExpressionStatement{!semi}, MacroInvocation
       {•pub•static•T•:•T•=•T•{•}•;•}    MacroInvocation.segments{dk: "{}"}
                      :                  PunctuationToken{tk: ":"}
                          =              PunctuationToken{tk: "="}
                              {•}        DelimGroup
                                  ;      PunctuationToken{tk: ";"}                                                                        */
    #[attr(info)] { o.m(|a_0| ()) }                                                                                                       /*
    #[attr(info)]•{•o.m(|a_0|•())•}    ExpressionStatement{!semi}
                  {•o.m(|a_0|•())•}    ExpressionStatement~ownStart
    #[attr(info)]                      Attribute{!inner}
     [attr(info)]                      Attribute.segments{dk: "[]"}
          (info)                       DelimGroup
                  {•o.m(|a_0|•())•}    BlockExpression
                    o.m(|a_0|•())      ExpressionStatement{!semi}, CallExpression
                       (|a_0|•())      CallExpression.arguments{dk: "()"}
                        |a_0|•()       ClosureFunctionExpression
                        |a_0|          ClosureFunctionExpression.parameters{dk: "||"}
                         a_0           ClosureFunctionParameterDeclaration
                              ()       TupleLiteral                                                                                       */
	c! () {}                                                                                                                              /*
	c!•()       ExpressionStatement{!semi}, MacroInvocation
	   ()       MacroInvocation.segments{dk: "()"}
	      {}    ExpressionStatement{!semi}, BlockExpression                                                                               */
	c! [] {}                                                                                                                              /*
	c!•[]       ExpressionStatement{!semi}, MacroInvocation
	   []       MacroInvocation.segments{dk: "[]"}
	      {}    ExpressionStatement{!semi}, BlockExpression                                                                               */
	c! {} {}                                                                                                                              /*
	c!•{}       ExpressionStatement{!semi}, MacroInvocation
	   {}       MacroInvocation.segments{dk: "{}"}
	      {}    ExpressionStatement{!semi}, BlockExpression                                                                               */
	c! {}; {}                                                                                                                             /*
	c!•{};       ExpressionStatement{semi}
	c!•{}        MacroInvocation
	   {}        MacroInvocation.segments{dk: "{}"}
	       {}    ExpressionStatement{!semi}, BlockExpression                                                                              */
	c! (); {}                                                                                                                             /*
	c!•();       ExpressionStatement{semi}
	c!•()        MacroInvocation
	   ()        MacroInvocation.segments{dk: "()"}
	       {}    ExpressionStatement{!semi}, BlockExpression                                                                              */
	c! []; {}                                                                                                                             /*
	c!•[];       ExpressionStatement{semi}
	c!•[]        MacroInvocation
	   []        MacroInvocation.segments{dk: "[]"}
	       {}    ExpressionStatement{!semi}, BlockExpression                                                                              */
	{}{}{};                                                                                                                               /*
	{}         ExpressionStatement{!semi}, BlockExpression
	  {}       ExpressionStatement{!semi}, BlockExpression
	    {};    ExpressionStatement{semi}
	    {}     BlockExpression                                                                                                            */
	(){}{};                                                                                                                               /*
	()         ExpressionStatement{!semi}, TupleLiteral
	  {}       ExpressionStatement{!semi}, BlockExpression
	    {};    ExpressionStatement{semi}
	    {}     BlockExpression                                                                                                            */
	{}(){};                                                                                                                               /*
	{}         ExpressionStatement{!semi}, BlockExpression
	  ()       ExpressionStatement{!semi}, TupleLiteral
	    {};    ExpressionStatement{semi}
	    {}     BlockExpression                                                                                                            */
	{}{}();                                                                                                                               /*
	{}         ExpressionStatement{!semi}, BlockExpression
	  {}       ExpressionStatement{!semi}, BlockExpression
	    ();    ExpressionStatement{semi}
	    ()     TupleLiteral                                                                                                               */
	[]{}();                                                                                                                               /*
	[]         ExpressionStatement{!semi}, ArrayLiteral
	  {}       ExpressionStatement{!semi}, BlockExpression
	    ();    ExpressionStatement{semi}
	    ()     TupleLiteral                                                                                                               */
	{}[]();                                                                                                                               /*
	{}         ExpressionStatement{!semi}, BlockExpression
	  []();    ExpressionStatement{semi}
	  []()     CallExpression
	  []       ArrayLiteral
	    ()     CallExpression.arguments{dk: "()"}                                                                                         */
	// {}()[];
	//•{}()[];    Comment{line}
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */


// {   if a { 1 } else { 0 }   +   if b { 1 } else { 0 }   }
//•{•••if•a•{•1•}•else•{•0•}•••+•••if•b•{•1•}•else•{•0•}•••}    Comment{line}
// {   if a { 1 } else { 0 }   + ( if b { 1 } else { 0 } ) }
//•{•••if•a•{•1•}•else•{•0•}•••+•(•if•b•{•1•}•else•{•0•}•)•}    Comment{line}

[                                                                                                                                         /*
[↲    <ExpressionStatement{semi}>
[↲    <ArrayLiteral>                                                                                                                      */
	{ ({ 0 }) - 0 },                                                                                                                      /*
	{•({•0•})•-•0•}    BlockExpression
	  ({•0•})•-•0      ExpressionStatement{!semi}, OperationExpression{tk: "-"}
	   {•0•}           BlockExpression
	     0             ExpressionStatement{!semi}, Literal{kind: Integer}
	            0      Literal{kind: Integer}                                                                                             */
	{ { 0 } || 0 },                                                                                                                       /*
	{•{•0•}•||•0•}    BlockExpression
	  {•0•}           ExpressionStatement{!semi}, BlockExpression
	    0             ExpressionStatement{!semi}, Literal{kind: Integer}
	        ||•0      ExpressionStatement{!semi}, ClosureFunctionExpression
	        ||        ClosureFunctionExpression.parameters{dk: "||"}
	           0      Literal{kind: Integer}                                                                                              */
	{ { 0 } &&0 },                                                                                                                        /*
	{•{•0•}•&&0•}    BlockExpression
	  {•0•}          ExpressionStatement{!semi}, BlockExpression
	    0            ExpressionStatement{!semi}, Literal{kind: Integer}
	        &&0      ExpressionStatement{!semi}, ReferenceExpression{!mut}
	         &0      ReferenceExpression{!mut}
	          0      Literal{kind: Integer}                                                                                               */
	{ { 0 } && if 0 { 0 } else { 0 } },                                                                                                   /*
	{•{•0•}•&&•if•0•{•0•}•else•{•0•}•}    BlockExpression
	  {•0•}                               ExpressionStatement{!semi}, BlockExpression
	    0                                 ExpressionStatement{!semi}, Literal{kind: Integer}
	        &&•if•0•{•0•}•else•{•0•}      ExpressionStatement{!semi}, ReferenceExpression{!mut}
	         &•if•0•{•0•}•else•{•0•}      ReferenceExpression{!mut}
	           if•0•{•0•}•else•{•0•}      IfBlockExpression
	              0                       Literal{kind: Integer}
	                {•0•}                 IfBlockExpression.body{dk: "{}"}
	                  0                   ExpressionStatement{!semi}, Literal{kind: Integer}
	                           {•0•}      BlockExpression
	                             0        ExpressionStatement{!semi}, Literal{kind: Integer}                                              */
	{ { 0 } *0;},                                                                                                                         /*
	{•{•0•}•*0;}    BlockExpression
	  {•0•}         ExpressionStatement{!semi}, BlockExpression
	    0           ExpressionStatement{!semi}, Literal{kind: Integer}
	        *0;     ExpressionStatement{semi}
	        *0      DereferenceExpression
	         0      Literal{kind: Integer}                                                                                                */
	{ { 0 } *0 },                                                                                                                         /*
	{•{•0•}•*0•}    BlockExpression
	  {•0•}         ExpressionStatement{!semi}, BlockExpression
	    0           ExpressionStatement{!semi}, Literal{kind: Integer}
	        *0      ExpressionStatement{!semi}, DereferenceExpression
	         0      Literal{kind: Integer}                                                                                                */
	{ { 0 } (0, 0) },                                                                                                                     /*
	{•{•0•}•(0,•0)•}    BlockExpression
	  {•0•}             ExpressionStatement{!semi}, BlockExpression
	    0               ExpressionStatement{!semi}, Literal{kind: Integer}
	        (0,•0)      ExpressionStatement{!semi}, TupleLiteral
	         0          Literal{kind: Integer}
	            0       Literal{kind: Integer}                                                                                            */
	{ { 0 } (0 || 0) && 0 },                                                                                                              /*
	{•{•0•}•(0•||•0)•&&•0•}    BlockExpression
	  {•0•}                    ExpressionStatement{!semi}, BlockExpression
	    0                      ExpressionStatement{!semi}, Literal{kind: Integer}
	        (0•||•0)•&&•0      ExpressionStatement{!semi}, AndExpression{tk: "&&"}
	         0•||•0            OrExpression{tk: "||"}
	         0                 Literal{kind: Integer}
	              0            Literal{kind: Integer}
	                    0      Literal{kind: Integer}                                                                                     */
	{ if 0 {} !(0) },                                                                                                                     /*
	{•if•0•{}•!(0)•}    BlockExpression
	  if•0•{}           ExpressionStatement{!semi}, IfBlockExpression
	     0              Literal{kind: Integer}
	       {}           IfBlockExpression.body{dk: "{}"}
	          !(0)      ExpressionStatement{!semi}, NotExpression
	            0       Literal{kind: Integer}                                                                                            */
	{ if 0 {} vec![0] },                                                                                                                  /*
	{•if•0•{}•vec![0]•}    BlockExpression
	  if•0•{}              ExpressionStatement{!semi}, IfBlockExpression
	     0                 Literal{kind: Integer}
	       {}              IfBlockExpression.body{dk: "{}"}
	          vec![0]      ExpressionStatement{!semi}, MacroInvocation
	              [0]      MacroInvocation.segments{dk: "[]"}
	               0       Literal{kind: Integer}                                                                                         */
	{ if 0 {} *0 || 0 },                                                                                                                  /*
	{•if•0•{}•*0•||•0•}    BlockExpression
	  if•0•{}              ExpressionStatement{!semi}, IfBlockExpression
	     0                 Literal{kind: Integer}
	       {}              IfBlockExpression.body{dk: "{}"}
	          *0•||•0      ExpressionStatement{!semi}, OrExpression{tk: "||"}
	          *0           DereferenceExpression
	           0           Literal{kind: Integer}
	                0      Literal{kind: Integer}                                                                                         */
	{ if 0 {} -0 },                                                                                                                       /*
	{•if•0•{}•-0•}    BlockExpression
	  if•0•{}         ExpressionStatement{!semi}, IfBlockExpression
	     0            Literal{kind: Integer}
	       {}         IfBlockExpression.body{dk: "{}"}
	          -0      ExpressionStatement{!semi}, MinusExpression
	           0      Literal{kind: Integer}                                                                                              */
	{ if 0 {} .m() },                                                                                                                     /*
	{•if•0•{}•.m()•}    BlockExpression
	  if•0•{}•.m()      ExpressionStatement{!semi}, CallExpression
	  if•0•{}           IfBlockExpression
	     0              Literal{kind: Integer}
	       {}           IfBlockExpression.body{dk: "{}"}
	            ()      CallExpression.arguments{dk: "()"}                                                                                */
	{ if 0 {} ? },                                                                                                                        /*
	{•if•0•{}•?•}    BlockExpression
	  if•0•{}•?      ExpressionStatement{!semi}, UnwrapExpression
	  if•0•{}        IfBlockExpression
	     0           Literal{kind: Integer}
	       {}        IfBlockExpression.body{dk: "{}"}                                                                                     */
	{ if 0 {} || 0 },                                                                                                                     /*
	{•if•0•{}•||•0•}    BlockExpression
	  if•0•{}           ExpressionStatement{!semi}, IfBlockExpression
	     0              Literal{kind: Integer}
	       {}           IfBlockExpression.body{dk: "{}"}
	          ||•0      ExpressionStatement{!semi}, ClosureFunctionExpression
	          ||        ClosureFunctionExpression.parameters{dk: "||"}
	             0      Literal{kind: Integer}                                                                                            */
	{ if 0 {} (|| 0) },                                                                                                                   /*
	{•if•0•{}•(||•0)•}    BlockExpression
	  if•0•{}             ExpressionStatement{!semi}, IfBlockExpression
	     0                Literal{kind: Integer}
	       {}             IfBlockExpression.body{dk: "{}"}
	          (||•0)      ExpressionStatement{!semi}
	           ||•0       ClosureFunctionExpression
	           ||         ClosureFunctionExpression.parameters{dk: "||"}
	              0       Literal{kind: Integer}                                                                                          */
	{ if 0 {} ? || 0 },                                                                                                                   /*
	{•if•0•{}•?•||•0•}    BlockExpression
	  if•0•{}•?•||•0      ExpressionStatement{!semi}, OrExpression{tk: "||"}
	  if•0•{}•?           UnwrapExpression
	  if•0•{}             IfBlockExpression
	     0                Literal{kind: Integer}
	       {}             IfBlockExpression.body{dk: "{}"}
	               0      Literal{kind: Integer}                                                                                          */
	{ if 0 {} .m() || 0 },                                                                                                                /*
	{•if•0•{}•.m()•||•0•}    BlockExpression
	  if•0•{}•.m()•||•0      ExpressionStatement{!semi}, OrExpression{tk: "||"}
	  if•0•{}•.m()           CallExpression
	  if•0•{}                IfBlockExpression
	     0                   Literal{kind: Integer}
	       {}                IfBlockExpression.body{dk: "{}"}
	            ()           CallExpression.arguments{dk: "()"}
	                  0      Literal{kind: Integer}                                                                                       */
	{ if 0 {} else {} |o| 0 },                                                                                                            /*
	{•if•0•{}•else•{}•|o|•0•}    BlockExpression
	  if•0•{}•else•{}            ExpressionStatement{!semi}, IfBlockExpression
	     0                       Literal{kind: Integer}
	       {}                    IfBlockExpression.body{dk: "{}"}
	               {}            BlockExpression
	                  |o|•0      ExpressionStatement{!semi}, ClosureFunctionExpression
	                  |o|        ClosureFunctionExpression.parameters{dk: "||"}
	                   o         ClosureFunctionParameterDeclaration
	                      0      Literal{kind: Integer}                                                                                   */
	{ 0 + if 0 { 0 } else { 0 }[0] },                                                                                                     /*
	{•0•+•if•0•{•0•}•else•{•0•}[0]•}    BlockExpression
	  0•+•if•0•{•0•}•else•{•0•}[0]      ExpressionStatement{!semi}, OperationExpression{tk: "+"}
	  0                                 Literal{kind: Integer}
	      if•0•{•0•}•else•{•0•}[0]      MemberExpression{computed}
	      if•0•{•0•}•else•{•0•}         IfBlockExpression
	         0                          Literal{kind: Integer}
	           {•0•}                    IfBlockExpression.body{dk: "{}"}
	             0                      ExpressionStatement{!semi}, Literal{kind: Integer}
	                      {•0•}         BlockExpression
	                        0           ExpressionStatement{!semi}, Literal{kind: Integer}
	                            0       Literal{kind: Integer}                                                                            */
	{ fn f<T>() { loop {} } o::<T> as T },                                                                                                /*
	{•fn•f<T>()•{•loop•{}•}•o::<T>•as•T•}    BlockExpression
	  fn•f<T>()•{•loop•{}•}                  FunctionDeclaration
	      <T>                                FunctionDeclaration.generics{dk: "<>"}
	       T                                 GenericTypeParameterDeclaration
	         ()                              FunctionDeclaration.parameters{dk: "()"}
	            {•loop•{}•}                  FunctionDeclaration.body{dk: "{}"}
	              loop•{}                    ExpressionStatement{!semi}, LoopBlockExpression
	                   {}                    LoopBlockExpression.body{dk: "{}"}
	                        o::<T>•as•T      ExpressionStatement{!semi}, ExpressionAsTypeCast
	                        o::<T>           ExpressionTypeCast
	                           <T>           ExpressionTypeCast.typeArguments{dk: "<>"}                                                   */
	{ match 0 {} o[0] },                                                                                                                  /*
	{•match•0•{}•o[0]•}    BlockExpression
	  match•0•{}           ExpressionStatement{!semi}, MatchExpression
	        0              Literal{kind: Integer}
	          {}           MatchExpression.cases{dk: "{}"}
	             o[0]      ExpressionStatement{!semi}, MemberExpression{computed}
	               0       Literal{kind: Integer}                                                                                         */
	{ match 0 {} (*0 < 0) as T },                                                                                                         /*
	{•match•0•{}•(*0•<•0)•as•T•}    BlockExpression
	  match•0•{}                    ExpressionStatement{!semi}, MatchExpression
	        0                       Literal{kind: Integer}
	          {}                    MatchExpression.cases{dk: "{}"}
	             (*0•<•0)•as•T      ExpressionStatement{!semi}, ExpressionAsTypeCast
	              *0•<•0            ComparisonExpression{tk: "<"}
	              *0                DereferenceExpression
	               0                Literal{kind: Integer}
	                   0            Literal{kind: Integer}                                                                                */
	{ match 0 {} o.m(S { p }); },                                                                                                         /*
	{•match•0•{}•o.m(S•{•p•});•}    BlockExpression
	  match•0•{}                    ExpressionStatement{!semi}, MatchExpression
	        0                       Literal{kind: Integer}
	          {}                    MatchExpression.cases{dk: "{}"}
	             o.m(S•{•p•});      ExpressionStatement{semi}
	             o.m(S•{•p•})       CallExpression
	                (S•{•p•})       CallExpression.arguments{dk: "()"}
	                 S•{•p•}        StructLiteral
	                   {•p•}        StructLiteral.properties{dk: "{}"}
	                     p          StructLiteralPropertyShorthand                                                                        */
	{ match 0 {} o.m(o.m() + 0); },                                                                                                       /*
	{•match•0•{}•o.m(o.m()•+•0);•}    BlockExpression
	  match•0•{}                      ExpressionStatement{!semi}, MatchExpression
	        0                         Literal{kind: Integer}
	          {}                      MatchExpression.cases{dk: "{}"}
	             o.m(o.m()•+•0);      ExpressionStatement{semi}
	             o.m(o.m()•+•0)       CallExpression
	                (o.m()•+•0)       CallExpression.arguments{dk: "()"}
	                 o.m()•+•0        OperationExpression{tk: "+"}
	                 o.m()            CallExpression
	                    ()            CallExpression.arguments{dk: "()"}
	                         0        Literal{kind: Integer}                                                                              */
	{ match 0 {} if *0 < 0 { 0 } else { 0 } },                                                                                            /*
	{•match•0•{}•if•*0•<•0•{•0•}•else•{•0•}•}    BlockExpression
	  match•0•{}                                 ExpressionStatement{!semi}, MatchExpression
	        0                                    Literal{kind: Integer}
	          {}                                 MatchExpression.cases{dk: "{}"}
	             if•*0•<•0•{•0•}•else•{•0•}      ExpressionStatement{!semi}, IfBlockExpression
	                *0•<•0                       ComparisonExpression{tk: "<"}
	                *0                           DereferenceExpression
	                 0                           Literal{kind: Integer}
	                     0                       Literal{kind: Integer}
	                       {•0•}                 IfBlockExpression.body{dk: "{}"}
	                         0                   ExpressionStatement{!semi}, Literal{kind: Integer}
	                                  {•0•}      BlockExpression
	                                    0        ExpressionStatement{!semi}, Literal{kind: Integer}                                       */
	{ (0 .. 0) .m((0, 0), |a_0, _| { *o = (o.0, o.0 + o.0); c(*0) }) .m(&|(a_0, _)| 0) },                                                 /*
	{•(0•..•0)•.m((0,•0),•|a_0,•_|•{•*o•=•(o.0,•o.0•+•o.0);•c(*0)•})•.m(&|(a_0,•_)|•0)•}    BlockExpression
	  (0•..•0)•.m((0,•0),•|a_0,•_|•{•*o•=•(o.0,•o.0•+•o.0);•c(*0)•})•.m(&|(a_0,•_)|•0)      ExpressionStatement{!semi}, CallExpression
	  (0•..•0)•.m((0,•0),•|a_0,•_|•{•*o•=•(o.0,•o.0•+•o.0);•c(*0)•})                        CallExpression
	   0•..•0                                                                               RangeLiteral{!last}
	   0                                                                                    Literal{kind: Integer}
	        0                                                                               Literal{kind: Integer}
	             ((0,•0),•|a_0,•_|•{•*o•=•(o.0,•o.0•+•o.0);•c(*0)•})                        CallExpression.arguments{dk: "()"}
	              (0,•0)                                                                    TupleLiteral
	               0                                                                        Literal{kind: Integer}
	                  0                                                                     Literal{kind: Integer}
	                      |a_0,•_|•{•*o•=•(o.0,•o.0•+•o.0);•c(*0)•}                         ClosureFunctionExpression
	                      |a_0,•_|                                                          ClosureFunctionExpression.parameters{dk: "||"}
	                       a_0                                                              ClosureFunctionParameterDeclaration
	                            _                                                           ClosureFunctionParameterDeclaration, WildcardPattern
	                               {•*o•=•(o.0,•o.0•+•o.0);•c(*0)•}                         BlockExpression
	                                 *o•=•(o.0,•o.0•+•o.0);                                 ExpressionStatement{semi}
	                                 *o•=•(o.0,•o.0•+•o.0)                                  ReassignmentExpression{tk: "="}
	                                 *o                                                     DereferenceExpression
	                                      (o.0,•o.0•+•o.0)                                  TupleLiteral
	                                       o.0                                              MemberExpression{!computed}
	                                         0                                              Index
	                                            o.0•+•o.0                                   OperationExpression{tk: "+"}
	                                            o.0                                         MemberExpression{!computed}
	                                              0                                         Index
	                                                  o.0                                   MemberExpression{!computed}
	                                                    0                                   Index
	                                                        c(*0)                           ExpressionStatement{!semi}, CallExpression
	                                                         (*0)                           CallExpression.arguments{dk: "()"}
	                                                          *0                            DereferenceExpression
	                                                           0                            Literal{kind: Integer}
	                                                                   (&|(a_0,•_)|•0)      CallExpression.arguments{dk: "()"}
	                                                                    &|(a_0,•_)|•0       ReferenceExpression{!mut}
	                                                                     |(a_0,•_)|•0       ClosureFunctionExpression
	                                                                     |(a_0,•_)|         ClosureFunctionExpression.parameters{dk: "||"}
	                                                                      (a_0,•_)          ClosureFunctionParameterDeclaration, TuplePattern
	                                                                            _           WildcardPattern
	                                                                                0       Literal{kind: Integer}                        */
	{ 'label: loop {}.p },                                                                                                                /*
	{•'label:•loop•{}.p•}    BlockExpression
	  'label:•loop•{}.p      ExpressionStatement{!semi}, MemberExpression{!computed}
	  'label:•loop•{}        LoopBlockExpression
	  'label                 LbIdentifier
	               {}        LoopBlockExpression.body{dk: "{}"}                                                                           */
	{ async { 0 + 0 }.await },                                                                                                            /*
	{•async•{•0•+•0•}.await•}    BlockExpression
	  async•{•0•+•0•}.await      ExpressionStatement{!semi}, AwaitExpression
	  async•{•0•+•0•}            BlockExpression{async}
	        {•0•+•0•}            BlockExpression.body{dk: "{}"}
	          0•+•0              ExpressionStatement{!semi}, OperationExpression{tk: "+"}
	          0                  Literal{kind: Integer}
	              0              Literal{kind: Integer}                                                                                   */
	{ { o.p }.p = 0 },                                                                                                                    /*
	{•{•o.p•}.p•=•0•}    BlockExpression
	  {•o.p•}.p•=•0      ExpressionStatement{!semi}, ReassignmentExpression{tk: "="}
	  {•o.p•}.p          MemberExpression{!computed}
	  {•o.p•}            BlockExpression
	    o.p              ExpressionStatement{!semi}, MemberExpression{!computed}
	              0      Literal{kind: Integer}                                                                                           */
	{({ 0 } + 0)},                                                                                                                        /*
	{({•0•}•+•0)}    BlockExpression
	 ({•0•}•+•0)     ExpressionStatement{!semi}
	  {•0•}•+•0      OperationExpression{tk: "+"}
	  {•0•}          BlockExpression
	    0            ExpressionStatement{!semi}, Literal{kind: Integer}
	          0      Literal{kind: Integer}                                                                                               */
	{({ 0 }) + 0},                                                                                                                        /*
	{({•0•})•+•0}    BlockExpression
	 ({•0•})•+•0     ExpressionStatement{!semi}, OperationExpression{tk: "+"}
	  {•0•}          BlockExpression
	    0            ExpressionStatement{!semi}, Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	{({ 0 } + 0 + 0)},                                                                                                                    /*
	{({•0•}•+•0•+•0)}    BlockExpression
	 ({•0•}•+•0•+•0)     ExpressionStatement{!semi}
	  {•0•}•+•0•+•0      OperationExpression{tk: "+"}
	  {•0•}•+•0          OperationExpression{tk: "+"}
	  {•0•}              BlockExpression
	    0                ExpressionStatement{!semi}, Literal{kind: Integer}
	          0          Literal{kind: Integer}
	              0      Literal{kind: Integer}                                                                                           */
	{ ( if 0 { 0 } else { 0 }   +   if 0 { 0 } else { 0 } ) },                                                                            /*
	{•(•if•0•{•0•}•else•{•0•}•••+•••if•0•{•0•}•else•{•0•}•)•}    BlockExpression
	  (•if•0•{•0•}•else•{•0•}•••+•••if•0•{•0•}•else•{•0•}•)      ExpressionStatement{!semi}
	    if•0•{•0•}•else•{•0•}•••+•••if•0•{•0•}•else•{•0•}        OperationExpression{tk: "+"}
	    if•0•{•0•}•else•{•0•}                                    IfBlockExpression
	       0                                                     Literal{kind: Integer}
	         {•0•}                                               IfBlockExpression.body{dk: "{}"}
	           0                                                 ExpressionStatement{!semi}, Literal{kind: Integer}
	                    {•0•}                                    BlockExpression
	                      0                                      ExpressionStatement{!semi}, Literal{kind: Integer}
	                                if•0•{•0•}•else•{•0•}        IfBlockExpression
	                                   0                         Literal{kind: Integer}
	                                     {•0•}                   IfBlockExpression.body{dk: "{}"}
	                                       0                     ExpressionStatement{!semi}, Literal{kind: Integer}
	                                                {•0•}        BlockExpression
	                                                  0          ExpressionStatement{!semi}, Literal{kind: Integer}                       */
	{ ( if 0 { 0 } else { 0 } ) +   if 0 { 0 } else { 0 }   },                                                                            /*
	{•(•if•0•{•0•}•else•{•0•}•)•+•••if•0•{•0•}•else•{•0•}•••}    BlockExpression
	  (•if•0•{•0•}•else•{•0•}•)•+•••if•0•{•0•}•else•{•0•}        ExpressionStatement{!semi}, OperationExpression{tk: "+"}
	    if•0•{•0•}•else•{•0•}                                    IfBlockExpression
	       0                                                     Literal{kind: Integer}
	         {•0•}                                               IfBlockExpression.body{dk: "{}"}
	           0                                                 ExpressionStatement{!semi}, Literal{kind: Integer}
	                    {•0•}                                    BlockExpression
	                      0                                      ExpressionStatement{!semi}, Literal{kind: Integer}
	                                if•0•{•0•}•else•{•0•}        IfBlockExpression
	                                   0                         Literal{kind: Integer}
	                                     {•0•}                   IfBlockExpression.body{dk: "{}"}
	                                       0                     ExpressionStatement{!semi}, Literal{kind: Integer}
	                                                {•0•}        BlockExpression
	                                                  0          ExpressionStatement{!semi}, Literal{kind: Integer}                       */
	{ ( if 0 { 0 } else { 0 } ) + ( if 0 { 0 } else { 0 } ) },                                                                            /*
	{•(•if•0•{•0•}•else•{•0•}•)•+•(•if•0•{•0•}•else•{•0•}•)•}    BlockExpression
	  (•if•0•{•0•}•else•{•0•}•)•+•(•if•0•{•0•}•else•{•0•}•)      ExpressionStatement{!semi}, OperationExpression{tk: "+"}
	    if•0•{•0•}•else•{•0•}                                    IfBlockExpression
	       0                                                     Literal{kind: Integer}
	         {•0•}                                               IfBlockExpression.body{dk: "{}"}
	           0                                                 ExpressionStatement{!semi}, Literal{kind: Integer}
	                    {•0•}                                    BlockExpression
	                      0                                      ExpressionStatement{!semi}, Literal{kind: Integer}
	                                if•0•{•0•}•else•{•0•}        IfBlockExpression
	                                   0                         Literal{kind: Integer}
	                                     {•0•}                   IfBlockExpression.body{dk: "{}"}
	                                       0                     ExpressionStatement{!semi}, Literal{kind: Integer}
	                                                {•0•}        BlockExpression
	                                                  0          ExpressionStatement{!semi}, Literal{kind: Integer}                       */
	{ ( if 0 { 0 } else { 0 } ) () },                                                                                                     /*
	{•(•if•0•{•0•}•else•{•0•}•)•()•}    BlockExpression
	  (•if•0•{•0•}•else•{•0•}•)•()      ExpressionStatement{!semi}, CallExpression
	    if•0•{•0•}•else•{•0•}           IfBlockExpression
	       0                            Literal{kind: Integer}
	         {•0•}                      IfBlockExpression.body{dk: "{}"}
	           0                        ExpressionStatement{!semi}, Literal{kind: Integer}
	                    {•0•}           BlockExpression
	                      0             ExpressionStatement{!semi}, Literal{kind: Integer}
	                            ()      CallExpression.arguments{dk: "()"}                                                                */
	{ ( if 0 { 0 } else { 0 } () ) },                                                                                                     /*
	{•(•if•0•{•0•}•else•{•0•}•()•)•}    BlockExpression
	  (•if•0•{•0•}•else•{•0•}•()•)      ExpressionStatement{!semi}
	    if•0•{•0•}•else•{•0•}•()        CallExpression
	    if•0•{•0•}•else•{•0•}           IfBlockExpression
	       0                            Literal{kind: Integer}
	         {•0•}                      IfBlockExpression.body{dk: "{}"}
	           0                        ExpressionStatement{!semi}, Literal{kind: Integer}
	                    {•0•}           BlockExpression
	                      0             ExpressionStatement{!semi}, Literal{kind: Integer}
	                          ()        CallExpression.arguments{dk: "()"}                                                                */
	{ ( match 0 {} ) () },                                                                                                                /*
	{•(•match•0•{}•)•()•}    BlockExpression
	  (•match•0•{}•)•()      ExpressionStatement{!semi}, CallExpression
	    match•0•{}           MatchExpression
	          0              Literal{kind: Integer}
	            {}           MatchExpression.cases{dk: "{}"}
	                 ()      CallExpression.arguments{dk: "()"}                                                                           */
	{ ( match 0 {} () ) },                                                                                                                /*
	{•(•match•0•{}•()•)•}    BlockExpression
	  (•match•0•{}•()•)      ExpressionStatement{!semi}
	    match•0•{}•()        CallExpression
	    match•0•{}           MatchExpression
	          0              Literal{kind: Integer}
	            {}           MatchExpression.cases{dk: "{}"}
	               ()        CallExpression.arguments{dk: "()"}                                                                           */
	{ ( { 0 } ) () },                                                                                                                     /*
	{•(•{•0•}•)•()•}    BlockExpression
	  (•{•0•}•)•()      ExpressionStatement{!semi}, CallExpression
	    {•0•}           BlockExpression
	      0             ExpressionStatement{!semi}, Literal{kind: Integer}
	            ()      CallExpression.arguments{dk: "()"}                                                                                */
	{ ( { 0 } () ) },                                                                                                                     /*
	{•(•{•0•}•()•)•}    BlockExpression
	  (•{•0•}•()•)      ExpressionStatement{!semi}
	    {•0•}•()        CallExpression
	    {•0•}           BlockExpression
	      0             ExpressionStatement{!semi}, Literal{kind: Integer}
	          ()        CallExpression.arguments{dk: "()"}                                                                                */
	{({ 0 } as u8)},                                                                                                                      /*
	{({•0•}•as•u8)}    BlockExpression
	 ({•0•}•as•u8)     ExpressionStatement{!semi}
	  {•0•}•as•u8      ExpressionAsTypeCast
	  {•0•}            BlockExpression
	    0              ExpressionStatement{!semi}, Literal{kind: Integer}                                                                 */
	{({ 0 }) as u8},                                                                                                                      /*
	{({•0•})•as•u8}    BlockExpression
	 ({•0•})•as•u8     ExpressionStatement{!semi}, ExpressionAsTypeCast
	  {•0•}            BlockExpression
	    0              ExpressionStatement{!semi}, Literal{kind: Integer}                                                                 */
	{if 0 { &0 } &0 as &u8},                                                                                                              /*
	{if•0•{•&0•}•&0•as•&u8}    BlockExpression
	 if•0•{•&0•}               ExpressionStatement{!semi}, IfBlockExpression
	    0                      Literal{kind: Integer}
	      {•&0•}               IfBlockExpression.body{dk: "{}"}
	        &0                 ExpressionStatement{!semi}, ReferenceExpression{!mut}
	         0                 Literal{kind: Integer}
	             &0•as•&u8     ExpressionStatement{!semi}, ExpressionAsTypeCast
	             &0            ReferenceExpression{!mut}
	              0            Literal{kind: Integer}
	                   &u8     TypeReference{!mut}                                                                                        */
	{(if 0 { 0 } else { 0 } as u8)},                                                                                                      /*
	{(if•0•{•0•}•else•{•0•}•as•u8)}    BlockExpression
	 (if•0•{•0•}•else•{•0•}•as•u8)     ExpressionStatement{!semi}
	  if•0•{•0•}•else•{•0•}•as•u8      ExpressionAsTypeCast
	  if•0•{•0•}•else•{•0•}            IfBlockExpression
	     0                             Literal{kind: Integer}
	       {•0•}                       IfBlockExpression.body{dk: "{}"}
	         0                         ExpressionStatement{!semi}, Literal{kind: Integer}
	                  {•0•}            BlockExpression
	                    0              ExpressionStatement{!semi}, Literal{kind: Integer}                                                 */
	{(if 0 { 0 } else { 0 }) as u8},                                                                                                      /*
	{(if•0•{•0•}•else•{•0•})•as•u8}    BlockExpression
	 (if•0•{•0•}•else•{•0•})•as•u8     ExpressionStatement{!semi}, ExpressionAsTypeCast
	  if•0•{•0•}•else•{•0•}            IfBlockExpression
	     0                             Literal{kind: Integer}
	       {•0•}                       IfBlockExpression.body{dk: "{}"}
	         0                         ExpressionStatement{!semi}, Literal{kind: Integer}
	                  {•0•}            BlockExpression
	                    0              ExpressionStatement{!semi}, Literal{kind: Integer}                                                 */
	{if 0 { &0 } else { &0 } &0 as &u8},                                                                                                  /*
	{if•0•{•&0•}•else•{•&0•}•&0•as•&u8}    BlockExpression
	 if•0•{•&0•}•else•{•&0•}               ExpressionStatement{!semi}, IfBlockExpression
	    0                                  Literal{kind: Integer}
	      {•&0•}                           IfBlockExpression.body{dk: "{}"}
	        &0                             ExpressionStatement{!semi}, ReferenceExpression{!mut}
	         0                             Literal{kind: Integer}
	                  {•&0•}               BlockExpression
	                    &0                 ExpressionStatement{!semi}, ReferenceExpression{!mut}
	                     0                 Literal{kind: Integer}
	                         &0•as•&u8     ExpressionStatement{!semi}, ExpressionAsTypeCast
	                         &0            ReferenceExpression{!mut}
	                          0            Literal{kind: Integer}
	                               &u8     TypeReference{!mut}                                                                            */
	{(match 0 {} as u8)},                                                                                                                 /*
	{(match•0•{}•as•u8)}    BlockExpression
	 (match•0•{}•as•u8)     ExpressionStatement{!semi}
	  match•0•{}•as•u8      ExpressionAsTypeCast
	  match•0•{}            MatchExpression
	        0               Literal{kind: Integer}
	          {}            MatchExpression.cases{dk: "{}"}                                                                               */
	{(match 0 {}) as u8},                                                                                                                 /*
	{(match•0•{})•as•u8}    BlockExpression
	 (match•0•{})•as•u8     ExpressionStatement{!semi}, ExpressionAsTypeCast
	  match•0•{}            MatchExpression
	        0               Literal{kind: Integer}
	          {}            MatchExpression.cases{dk: "{}"}                                                                               */
	{({ o })[0]},                                                                                                                         /*
	{({•o•})[0]}    BlockExpression
	 ({•o•})[0]     ExpressionStatement{!semi}, MemberExpression{computed}
	  {•o•}         BlockExpression
	    o           ExpressionStatement{!semi}
	         0      Literal{kind: Integer}                                                                                                */
	{({ o }[0])},                                                                                                                         /*
	{({•o•}[0])}    BlockExpression
	 ({•o•}[0])     ExpressionStatement{!semi}
	  {•o•}[0]      MemberExpression{computed}
	  {•o•}         BlockExpression
	    o           ExpressionStatement{!semi}
	        0       Literal{kind: Integer}                                                                                                */
	{(if 0 { 0 } else { 0 })[0]},                                                                                                         /*
	{(if•0•{•0•}•else•{•0•})[0]}    BlockExpression
	 (if•0•{•0•}•else•{•0•})[0]     ExpressionStatement{!semi}, MemberExpression{computed}
	  if•0•{•0•}•else•{•0•}         IfBlockExpression
	     0                          Literal{kind: Integer}
	       {•0•}                    IfBlockExpression.body{dk: "{}"}
	         0                      ExpressionStatement{!semi}, Literal{kind: Integer}
	                  {•0•}         BlockExpression
	                    0           ExpressionStatement{!semi}, Literal{kind: Integer}
	                         0      Literal{kind: Integer}                                                                                */
	{(if 0 { 0 } else { 0 }[0])},                                                                                                         /*
	{(if•0•{•0•}•else•{•0•}[0])}    BlockExpression
	 (if•0•{•0•}•else•{•0•}[0])     ExpressionStatement{!semi}
	  if•0•{•0•}•else•{•0•}[0]      MemberExpression{computed}
	  if•0•{•0•}•else•{•0•}         IfBlockExpression
	     0                          Literal{kind: Integer}
	       {•0•}                    IfBlockExpression.body{dk: "{}"}
	         0                      ExpressionStatement{!semi}, Literal{kind: Integer}
	                  {•0•}         BlockExpression
	                    0           ExpressionStatement{!semi}, Literal{kind: Integer}
	                        0       Literal{kind: Integer}                                                                                */
	{(match 0 {})[0]},                                                                                                                    /*
	{(match•0•{})[0]}    BlockExpression
	 (match•0•{})[0]     ExpressionStatement{!semi}, MemberExpression{computed}
	  match•0•{}         MatchExpression
	        0            Literal{kind: Integer}
	          {}         MatchExpression.cases{dk: "{}"}
	              0      Literal{kind: Integer}                                                                                           */
	{(match 0 {}[0])},                                                                                                                    /*
	{(match•0•{}[0])}    BlockExpression
	 (match•0•{}[0])     ExpressionStatement{!semi}
	  match•0•{}[0]      MemberExpression{computed}
	  match•0•{}         MatchExpression
	        0            Literal{kind: Integer}
	          {}         MatchExpression.cases{dk: "{}"}
	             0       Literal{kind: Integer}                                                                                           */
	{   c!()   &0 },                                                                                                                      /*
	{•••c!()•••&0•}    BlockExpression
	    c!()•••&0      ExpressionStatement{!semi}, OperationExpression{tk: "&"}
	    c!()           MacroInvocation
	      ()           MacroInvocation.segments{dk: "()"}
	            0      Literal{kind: Integer}                                                                                             */
	{   c![]   &0 },                                                                                                                      /*
	{•••c![]•••&0•}    BlockExpression
	    c![]•••&0      ExpressionStatement{!semi}, OperationExpression{tk: "&"}
	    c![]           MacroInvocation
	      []           MacroInvocation.segments{dk: "[]"}
	            0      Literal{kind: Integer}                                                                                             */
	{   c!{}   &0 },                                                                                                                      /*
	{•••c!{}•••&0•}    BlockExpression
	    c!{}           ExpressionStatement{!semi}, MacroInvocation
	      {}           MacroInvocation.segments{dk: "{}"}
	           &0      ExpressionStatement{!semi}, ReferenceExpression{!mut}
	            0      Literal{kind: Integer}                                                                                             */
	{ ( c!{} ) &0 },                                                                                                                      /*
	{•(•c!{}•)•&0•}    BlockExpression
	  (•c!{}•)•&0      ExpressionStatement{!semi}, OperationExpression{tk: "&"}
	    c!{}           MacroInvocation
	      {}           MacroInvocation.segments{dk: "{}"}
	            0      Literal{kind: Integer}                                                                                             */
	{   { 0 }   &  1   },                                                                                                                 /*
	{•••{•0•}•••&••1•••}    BlockExpression
	    {•0•}               ExpressionStatement{!semi}, BlockExpression
	      0                 ExpressionStatement{!semi}, Literal{kind: Integer}
	            &••1        ExpressionStatement{!semi}, ReferenceExpression{!mut}
	               1        Literal{kind: Integer}                                                                                        */
	{ ( { 0 } ) &  1   },                                                                                                                 /*
	{•(•{•0•}•)•&••1•••}    BlockExpression
	  (•{•0•}•)•&••1        ExpressionStatement{!semi}, OperationExpression{tk: "&"}
	    {•0•}               BlockExpression
	      0                 ExpressionStatement{!semi}, Literal{kind: Integer}
	               1        Literal{kind: Integer}                                                                                        */
	{   { 0 }   .. 1   },                                                                                                                 /*
	{•••{•0•}•••..•1•••}    BlockExpression
	    {•0•}               ExpressionStatement{!semi}, BlockExpression
	      0                 ExpressionStatement{!semi}, Literal{kind: Integer}
	            ..•1        ExpressionStatement{!semi}, RangeLiteral{!last}
	               1        Literal{kind: Integer}                                                                                        */
	{ ( { 0 } ) .. 1   },                                                                                                                 /*
	{•(•{•0•}•)•..•1•••}    BlockExpression
	  (•{•0•}•)•..•1        ExpressionStatement{!semi}, RangeLiteral{!last}
	    {•0•}               BlockExpression
	      0                 ExpressionStatement{!semi}, Literal{kind: Integer}
	               1        Literal{kind: Integer}                                                                                        */
	
	a..b!(),                                                                                                                              /*
	a..b!()    RangeLiteral{!last}
	   b!()    MacroInvocation
	     ()    MacroInvocation.segments{dk: "()"}                                                                                         */

	o.p.0 = 0,                                                                                                                            /*
	o.p.0•=•0    ReassignmentExpression{tk: "="}
	o.p.0        MemberExpression{!computed}
	o.p          MemberExpression{!computed}
	    0        Index
	        0    Literal{kind: Integer}                                                                                                   */
	o.p.0.0 = 0,                                                                                                                          /*
	o.p.0.0•=•0    ReassignmentExpression{tk: "="}
	o.p.0.0        MemberExpression{!computed}
	o.p.0          MemberExpression{!computed}
	o.p            MemberExpression{!computed}
	    0          Index
	      0        Index
	          0    Literal{kind: Integer}                                                                                                 */
	(*o.p).0 = 0,                                                                                                                         /*
	(*o.p).0•=•0    ReassignmentExpression{tk: "="}
	(*o.p).0        MemberExpression{!computed}
	 *o.p           DereferenceExpression
	  o.p           MemberExpression{!computed}
	       0        Index
	           0    Literal{kind: Integer}                                                                                                */
	(*o.p.0).0 = 0,                                                                                                                       /*
	(*o.p.0).0•=•0    ReassignmentExpression{tk: "="}
	(*o.p.0).0        MemberExpression{!computed}
	 *o.p.0           DereferenceExpression
	  o.p.0           MemberExpression{!computed}
	  o.p             MemberExpression{!computed}
	      0           Index
	         0        Index
	             0    Literal{kind: Integer}                                                                                              */
	&mut o.p.0,                                                                                                                           /*
	&mut•o.p.0    ReferenceExpression{mut}
	     o.p.0    MemberExpression{!computed}
	     o.p      MemberExpression{!computed}
	         0    Index                                                                                                                   */
	&mut o.p.0.0,                                                                                                                         /*
	&mut•o.p.0.0    ReferenceExpression{mut}
	     o.p.0.0    MemberExpression{!computed}
	     o.p.0      MemberExpression{!computed}
	     o.p        MemberExpression{!computed}
	         0      Index
	           0    Index                                                                                                                 */
	&mut (*o.p).0,                                                                                                                        /*
	&mut•(*o.p).0    ReferenceExpression{mut}
	     (*o.p).0    MemberExpression{!computed}
	      *o.p       DereferenceExpression
	       o.p       MemberExpression{!computed}
	            0    Index                                                                                                                */
	&mut (*o.p.0).0,                                                                                                                      /*
	&mut•(*o.p.0).0    ReferenceExpression{mut}
	     (*o.p.0).0    MemberExpression{!computed}
	      *o.p.0       DereferenceExpression
	       o.p.0       MemberExpression{!computed}
	       o.p         MemberExpression{!computed}
	           0       Index
	              0    Index                                                                                                              */
	o.p.0.m(0),                                                                                                                           /*
	o.p.0.m(0)    CallExpression
	o.p.0         MemberExpression{!computed}
	o.p           MemberExpression{!computed}
	    0         Index
	       (0)    CallExpression.arguments{dk: "()"}
	        0     Literal{kind: Integer}                                                                                                  */
	o.p.0.0.m(0),                                                                                                                         /*
	o.p.0.0.m(0)    CallExpression
	o.p.0.0         MemberExpression{!computed}
	o.p.0           MemberExpression{!computed}
	o.p             MemberExpression{!computed}
	    0           Index
	      0         Index
	         (0)    CallExpression.arguments{dk: "()"}
	          0     Literal{kind: Integer}                                                                                                */
	(*o.p).0.m(0),                                                                                                                        /*
	(*o.p).0.m(0)    CallExpression
	(*o.p).0         MemberExpression{!computed}
	 *o.p            DereferenceExpression
	  o.p            MemberExpression{!computed}
	       0         Index
	          (0)    CallExpression.arguments{dk: "()"}
	           0     Literal{kind: Integer}                                                                                               */
	(*o.p.0).0.m(0),                                                                                                                      /*
	(*o.p.0).0.m(0)    CallExpression
	(*o.p.0).0         MemberExpression{!computed}
	 *o.p.0            DereferenceExpression
	  o.p.0            MemberExpression{!computed}
	  o.p              MemberExpression{!computed}
	      0            Index
	         0         Index
	            (0)    CallExpression.arguments{dk: "()"}
	             0     Literal{kind: Integer}                                                                                             */

	o.p + *o + o,                                                                                                                         /*
	o.p•+•*o•+•o    OperationExpression{tk: "+"}
	o.p•+•*o        OperationExpression{tk: "+"}
	o.p             MemberExpression{!computed}
	      *o        DereferenceExpression                                                                                                 */

	c(o.p..o.p + o.p),                                                                                                                    /*
	c(o.p..o.p•+•o.p)    CallExpression
	 (o.p..o.p•+•o.p)    CallExpression.arguments{dk: "()"}
	  o.p..o.p•+•o.p     RangeLiteral{!last}
	  o.p                MemberExpression{!computed}
	       o.p•+•o.p     OperationExpression{tk: "+"}
	       o.p           MemberExpression{!computed}
	             o.p     MemberExpression{!computed}                                                                                      */
	c(c(o.p, |_| o.p = 0), 0),                                                                                                            /*
	c(c(o.p,•|_|•o.p•=•0),•0)    CallExpression
	 (c(o.p,•|_|•o.p•=•0),•0)    CallExpression.arguments{dk: "()"}
	  c(o.p,•|_|•o.p•=•0)        CallExpression
	   (o.p,•|_|•o.p•=•0)        CallExpression.arguments{dk: "()"}
	    o.p                      MemberExpression{!computed}
	         |_|•o.p•=•0         ClosureFunctionExpression
	         |_|                 ClosureFunctionExpression.parameters{dk: "||"}
	          _                  ClosureFunctionParameterDeclaration, WildcardPattern
	             o.p•=•0         ReassignmentExpression{tk: "="}
	             o.p             MemberExpression{!computed}
	                   0         Literal{kind: Integer}
	                       0     Literal{kind: Integer}                                                                                   */

	(0 && o.m(o.m()) == 0) || ((!0 && o[0].p == 0) || 0 == 0),                                                                            /*
	(0•&&•o.m(o.m())•==•0)•||•((!0•&&•o[0].p•==•0)•||•0•==•0)    OrExpression{tk: "||"}
	 0•&&•o.m(o.m())•==•0                                        AndExpression{tk: "&&"}
	 0                                                           Literal{kind: Integer}
	      o.m(o.m())•==•0                                        ComparisonExpression{tk: "=="}
	      o.m(o.m())                                             CallExpression
	         (o.m())                                             CallExpression.arguments{dk: "()"}
	          o.m()                                              CallExpression
	             ()                                              CallExpression.arguments{dk: "()"}
	                    0                                        Literal{kind: Integer}
	                           (!0•&&•o[0].p•==•0)•||•0•==•0     OrExpression{tk: "||"}
	                            !0•&&•o[0].p•==•0                AndExpression{tk: "&&"}
	                            !0                               NotExpression
	                             0                               Literal{kind: Integer}
	                                  o[0].p•==•0                ComparisonExpression{tk: "=="}
	                                  o[0].p                     MemberExpression{!computed}
	                                  o[0]                       MemberExpression{computed}
	                                    0                        Literal{kind: Integer}
	                                            0                Literal{kind: Integer}
	                                                  0•==•0     ComparisonExpression{tk: "=="}
	                                                  0          Literal{kind: Integer}
	                                                       0     Literal{kind: Integer}                                                   */
	(0 && o.m(o.m()) == 0) ||  (!0 && o[0].p == 0) || 0 == 0,                                                                             /*
	(0•&&•o.m(o.m())•==•0)•||••(!0•&&•o[0].p•==•0)•||•0•==•0    OrExpression{tk: "||"}
	(0•&&•o.m(o.m())•==•0)•||••(!0•&&•o[0].p•==•0)              OrExpression{tk: "||"}
	 0•&&•o.m(o.m())•==•0                                       AndExpression{tk: "&&"}
	 0                                                          Literal{kind: Integer}
	      o.m(o.m())•==•0                                       ComparisonExpression{tk: "=="}
	      o.m(o.m())                                            CallExpression
	         (o.m())                                            CallExpression.arguments{dk: "()"}
	          o.m()                                             CallExpression
	             ()                                             CallExpression.arguments{dk: "()"}
	                    0                                       Literal{kind: Integer}
	                            !0•&&•o[0].p•==•0               AndExpression{tk: "&&"}
	                            !0                              NotExpression
	                             0                              Literal{kind: Integer}
	                                  o[0].p•==•0               ComparisonExpression{tk: "=="}
	                                  o[0].p                    MemberExpression{!computed}
	                                  o[0]                      MemberExpression{computed}
	                                    0                       Literal{kind: Integer}
	                                            0               Literal{kind: Integer}
	                                                  0•==•0    ComparisonExpression{tk: "=="}
	                                                  0         Literal{kind: Integer}
	                                                       0    Literal{kind: Integer}                                                    */

	//  x  &  y  ==  0   input
	//••x••&••y••==••0•••input    Comment{line}
	//  x  & (y  ==  0)  javascript precedence
	//••x••&•(y••==••0)••javascript•precedence    Comment{line}
	// (x  &  y) ==  0   rust precedence
	//•(x••&••y)•==••0•••rust•precedence    Comment{line}
	
	0  &  0  ==  0,                                                                                                                       /*
	0••&••0••==••0    ComparisonExpression{tk: "=="}
	0••&••0           OperationExpression{tk: "&"}
	0                 Literal{kind: Integer}
	      0           Literal{kind: Integer}
	             0    Literal{kind: Integer}                                                                                              */
	0  & (0  ==  0),                                                                                                                      /*
	0••&•(0••==••0)    OperationExpression{tk: "&"}
	0                  Literal{kind: Integer}
	      0••==••0     ComparisonExpression{tk: "=="}
	      0            Literal{kind: Integer}
	             0     Literal{kind: Integer}                                                                                             */
   (0  &  0) ==  0,                                                                                                                       /*
   (0••&••0)•==••0    ComparisonExpression{tk: "=="}
    0••&••0           OperationExpression{tk: "&"}
    0                 Literal{kind: Integer}
          0           Literal{kind: Integer}
                 0    Literal{kind: Integer}                                                                                              */

	0 as u8 * 0,                                                                                                                          /*
	0•as•u8•*•0    OperationExpression{tk: "*"}
	0•as•u8        ExpressionAsTypeCast
	0              Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 as (u8) * 0,                                                                                                                        /*
	0•as•(u8)•*•0    OperationExpression{tk: "*"}
	0•as•(u8)        ExpressionAsTypeCast
	0                Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	0 as (u8) / 0,                                                                                                                        /*
	0•as•(u8)•/•0    OperationExpression{tk: "/"}
	0•as•(u8)        ExpressionAsTypeCast
	0                Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	0 as u8 + 0,                                                                                                                          /*
	0•as•u8•+•0    OperationExpression{tk: "+"}
	0•as•u8        ExpressionAsTypeCast
	0              Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 as (u8) + 0,                                                                                                                        /*
	0•as•(u8)•+•0    OperationExpression{tk: "+"}
	0•as•(u8)        ExpressionAsTypeCast
	0                Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	o = o = o = o,                                                                                                                        /*
	o•=•o•=•o•=•o    ReassignmentExpression{tk: "="}
	    o•=•o•=•o    ReassignmentExpression{tk: "="}
	        o•=•o    ReassignmentExpression{tk: "="}                                                                                      */
	o -= 0 - 0,                                                                                                                           /*
	o•-=•0•-•0    ReassignmentOperationExpression{tk: "-="}
	     0•-•0    OperationExpression{tk: "-"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o -= 0 - 0,                                                                                                                           /*
	o•-=•0•-•0    ReassignmentOperationExpression{tk: "-="}
	     0•-•0    OperationExpression{tk: "-"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o *= 0 * 0,                                                                                                                           /*
	o•*=•0•*•0    ReassignmentOperationExpression{tk: "*="}
	     0•*•0    OperationExpression{tk: "*"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o *= 0 * 0,                                                                                                                           /*
	o•*=•0•*•0    ReassignmentOperationExpression{tk: "*="}
	     0•*•0    OperationExpression{tk: "*"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o *= 0 * 99,                                                                                                                          /*
	o•*=•0•*•99    ReassignmentOperationExpression{tk: "*="}
	     0•*•99    OperationExpression{tk: "*"}
	     0         Literal{kind: Integer}
	         99    Literal{kind: Integer}                                                                                                 */
	o /= 0 / 0,                                                                                                                           /*
	o•/=•0•/•0    ReassignmentOperationExpression{tk: "/="}
	     0•/•0    OperationExpression{tk: "/"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o /= 0 / 0,                                                                                                                           /*
	o•/=•0•/•0    ReassignmentOperationExpression{tk: "/="}
	     0•/•0    OperationExpression{tk: "/"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o &= 0 & 0,                                                                                                                           /*
	o•&=•0•&•0    ReassignmentOperationExpression{tk: "&="}
	     0•&•0    OperationExpression{tk: "&"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o %= 0 % 0,                                                                                                                           /*
	o•%=•0•%•0    ReassignmentOperationExpression{tk: "%="}
	     0•%•0    OperationExpression{tk: "%"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o ^= 0,                                                                                                                               /*
	o•^=•0    ReassignmentOperationExpression{tk: "^="}
	     0    Literal{kind: Integer}                                                                                                      */
	o ^= 0,                                                                                                                               /*
	o•^=•0    ReassignmentOperationExpression{tk: "^="}
	     0    Literal{kind: Integer}                                                                                                      */
	o += *0,                                                                                                                              /*
	o•+=•*0    ReassignmentOperationExpression{tk: "+="}
	     *0    DereferenceExpression
	      0    Literal{kind: Integer}                                                                                                     */
	o += 0 + 0,                                                                                                                           /*
	o•+=•0•+•0    ReassignmentOperationExpression{tk: "+="}
	     0•+•0    OperationExpression{tk: "+"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o <<= 0 << 0,                                                                                                                         /*
	o•<<=•0•<<•0    ReassignmentOperationExpression{tk: "<<="}
	      0•<<•0    OperationExpression{tk: "<<"}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	o = 0 - 0,                                                                                                                            /*
	o•=•0•-•0    ReassignmentExpression{tk: "="}
	    0•-•0    OperationExpression{tk: "-"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 * 0 * 0,                                                                                                                        /*
	o•=•0•*•0•*•0    ReassignmentExpression{tk: "="}
	    0•*•0•*•0    OperationExpression{tk: "*"}
	    0•*•0        OperationExpression{tk: "*"}
	    0            Literal{kind: Integer}
	        0        Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	o = 0 * 0 + 0,                                                                                                                        /*
	o•=•0•*•0•+•0    ReassignmentExpression{tk: "="}
	    0•*•0•+•0    OperationExpression{tk: "+"}
	    0•*•0        OperationExpression{tk: "*"}
	    0            Literal{kind: Integer}
	        0        Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	o = 0 * 0,                                                                                                                            /*
	o•=•0•*•0    ReassignmentExpression{tk: "="}
	    0•*•0    OperationExpression{tk: "*"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 / 0,                                                                                                                            /*
	o•=•0•/•0    ReassignmentExpression{tk: "="}
	    0•/•0    OperationExpression{tk: "/"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 & 0,                                                                                                                            /*
	o•=•0•&•0    ReassignmentExpression{tk: "="}
	    0•&•0    OperationExpression{tk: "&"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 ^ 0,                                                                                                                            /*
	o•=•0•^•0    ReassignmentExpression{tk: "="}
	    0•^•0    OperationExpression{tk: "^"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 + 0,                                                                                                                            /*
	o•=•0•+•0    ReassignmentExpression{tk: "="}
	    0•+•0    OperationExpression{tk: "+"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 << 0,                                                                                                                           /*
	o•=•0•<<•0    ReassignmentExpression{tk: "="}
	    0•<<•0    OperationExpression{tk: "<<"}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 != 0 != 0,                                                                                                                          /*
	0•!=•0•!=•0    ComparisonExpression{tk: "!="}
	0•!=•0         ComparisonExpression{tk: "!="}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 != 0 == 0,                                                                                                                          /*
	0•!=•0•==•0    ComparisonExpression{tk: "=="}
	0•!=•0         ComparisonExpression{tk: "!="}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 == 0 != 0,                                                                                                                          /*
	0•==•0•!=•0    ComparisonExpression{tk: "!="}
	0•==•0         ComparisonExpression{tk: "=="}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 == 0 == 0,                                                                                                                          /*
	0•==•0•==•0    ComparisonExpression{tk: "=="}
	0•==•0         ComparisonExpression{tk: "=="}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 + 0 / 0,                                                                                                                            /*
	0•+•0•/•0    OperationExpression{tk: "+"}
	0            Literal{kind: Integer}
	    0•/•0    OperationExpression{tk: "/"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 + 0 >> 0,                                                                                                                           /*
	0•+•0•>>•0    OperationExpression{tk: ">>"}
	0•+•0         OperationExpression{tk: "+"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 % 0 * 0,                                                                                                                            /*
	0•%•0•*•0    OperationExpression{tk: "*"}
	0•%•0        OperationExpression{tk: "%"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 % 0 / 0,                                                                                                                            /*
	0•%•0•/•0    OperationExpression{tk: "/"}
	0•%•0        OperationExpression{tk: "%"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 % 0 % 0,                                                                                                                            /*
	0•%•0•%•0    OperationExpression{tk: "%"}
	0•%•0        OperationExpression{tk: "%"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 * 0 % 0,                                                                                                                            /*
	0•*•0•%•0    OperationExpression{tk: "%"}
	0•*•0        OperationExpression{tk: "*"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 / 0 % 0,                                                                                                                            /*
	0•/•0•%•0    OperationExpression{tk: "%"}
	0•/•0        OperationExpression{tk: "/"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 / 0 + 0,                                                                                                                            /*
	0•/•0•+•0    OperationExpression{tk: "+"}
	0•/•0        OperationExpression{tk: "/"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 & 0 & 0,                                                                                                                            /*
	0•&•0•&•0    OperationExpression{tk: "&"}
	0•&•0        OperationExpression{tk: "&"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 & 0 >> 0,                                                                                                                           /*
	0•&•0•>>•0    OperationExpression{tk: "&"}
	0             Literal{kind: Integer}
	    0•>>•0    OperationExpression{tk: ">>"}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 & 0 | 0,                                                                                                                            /*
	0•&•0•|•0    OperationExpression{tk: "|"}
	0•&•0        OperationExpression{tk: "&"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 ^ 0 ^ 0,                                                                                                                            /*
	0•^•0•^•0    OperationExpression{tk: "^"}
	0•^•0        OperationExpression{tk: "^"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 << 0 | 0,                                                                                                                           /*
	0•<<•0•|•0    OperationExpression{tk: "|"}
	0•<<•0        OperationExpression{tk: "<<"}
	0             Literal{kind: Integer}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 << 0 >> 0,                                                                                                                          /*
	0•<<•0•>>•0    OperationExpression{tk: ">>"}
	0•<<•0         OperationExpression{tk: "<<"}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 >> 0 >> 0,                                                                                                                          /*
	0•>>•0•>>•0    OperationExpression{tk: ">>"}
	0•>>•0         OperationExpression{tk: ">>"}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 | 0 & 0,                                                                                                                            /*
	0•|•0•&•0    OperationExpression{tk: "|"}
	0            Literal{kind: Integer}
	    0•&•0    OperationExpression{tk: "&"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 | 0 | 0,                                                                                                                            /*
	0•|•0•|•0    OperationExpression{tk: "|"}
	0•|•0        OperationExpression{tk: "|"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */

	// (0 x 0) x 0 ===========================================================
	//•(0•x•0)•x•0•===========================================================    Comment{line}
	(0 && 0) && 0,                                                                                                                        /*
	(0•&&•0)•&&•0    AndExpression{tk: "&&"}
	 0•&&•0          AndExpression{tk: "&&"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 && 0) || 0,                                                                                                                        /*
	(0•&&•0)•||•0    OrExpression{tk: "||"}
	 0•&&•0          AndExpression{tk: "&&"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 && 0) = 0,                                                                                                                         /*
	(0•&&•0)•=•0    ReassignmentExpression{tk: "="}
	 0•&&•0         AndExpression{tk: "&&"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 && 0) + 0,                                                                                                                         /*
	(0•&&•0)•+•0    OperationExpression{tk: "+"}
	 0•&&•0         AndExpression{tk: "&&"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 && 0) * 0,                                                                                                                         /*
	(0•&&•0)•*•0    OperationExpression{tk: "*"}
	 0•&&•0         AndExpression{tk: "&&"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 && 0) & 0,                                                                                                                         /*
	(0•&&•0)•&•0    OperationExpression{tk: "&"}
	 0•&&•0         AndExpression{tk: "&&"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 && 0) << 0,                                                                                                                        /*
	(0•&&•0)•<<•0    OperationExpression{tk: "<<"}
	 0•&&•0          AndExpression{tk: "&&"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 && 0) == 0,                                                                                                                        /*
	(0•&&•0)•==•0    ComparisonExpression{tk: "=="}
	 0•&&•0          AndExpression{tk: "&&"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 && 0) > 0,                                                                                                                         /*
	(0•&&•0)•>•0    ComparisonExpression{tk: ">"}
	 0•&&•0         AndExpression{tk: "&&"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 && 0)..0,                                                                                                                          /*
	(0•&&•0)..0    RangeLiteral{!last}
	 0•&&•0        AndExpression{tk: "&&"}
	 0             Literal{kind: Integer}
	      0        Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 || 0) && 0,                                                                                                                        /*
	(0•||•0)•&&•0    AndExpression{tk: "&&"}
	 0•||•0          OrExpression{tk: "||"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 || 0) || 0,                                                                                                                        /*
	(0•||•0)•||•0    OrExpression{tk: "||"}
	 0•||•0          OrExpression{tk: "||"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 || 0) = 0,                                                                                                                         /*
	(0•||•0)•=•0    ReassignmentExpression{tk: "="}
	 0•||•0         OrExpression{tk: "||"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 || 0) + 0,                                                                                                                         /*
	(0•||•0)•+•0    OperationExpression{tk: "+"}
	 0•||•0         OrExpression{tk: "||"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 || 0) * 0,                                                                                                                         /*
	(0•||•0)•*•0    OperationExpression{tk: "*"}
	 0•||•0         OrExpression{tk: "||"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 || 0) & 0,                                                                                                                         /*
	(0•||•0)•&•0    OperationExpression{tk: "&"}
	 0•||•0         OrExpression{tk: "||"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 || 0) << 0,                                                                                                                        /*
	(0•||•0)•<<•0    OperationExpression{tk: "<<"}
	 0•||•0          OrExpression{tk: "||"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 || 0) == 0,                                                                                                                        /*
	(0•||•0)•==•0    ComparisonExpression{tk: "=="}
	 0•||•0          OrExpression{tk: "||"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 || 0) > 0,                                                                                                                         /*
	(0•||•0)•>•0    ComparisonExpression{tk: ">"}
	 0•||•0         OrExpression{tk: "||"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 || 0)..0,                                                                                                                          /*
	(0•||•0)..0    RangeLiteral{!last}
	 0•||•0        OrExpression{tk: "||"}
	 0             Literal{kind: Integer}
	      0        Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(o = 0) && 0,                                                                                                                         /*
	(o•=•0)•&&•0    AndExpression{tk: "&&"}
	 o•=•0          ReassignmentExpression{tk: "="}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(o = 0) || 0,                                                                                                                         /*
	(o•=•0)•||•0    OrExpression{tk: "||"}
	 o•=•0          ReassignmentExpression{tk: "="}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	// (o = 0) = 0,
	//•(o•=•0)•=•0,    Comment{line}
	(o = 0) + 0,                                                                                                                          /*
	(o•=•0)•+•0    OperationExpression{tk: "+"}
	 o•=•0         ReassignmentExpression{tk: "="}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(o = 0) * 0,                                                                                                                          /*
	(o•=•0)•*•0    OperationExpression{tk: "*"}
	 o•=•0         ReassignmentExpression{tk: "="}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(o = 0) & 0,                                                                                                                          /*
	(o•=•0)•&•0    OperationExpression{tk: "&"}
	 o•=•0         ReassignmentExpression{tk: "="}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(o = 0) << 0,                                                                                                                         /*
	(o•=•0)•<<•0    OperationExpression{tk: "<<"}
	 o•=•0          ReassignmentExpression{tk: "="}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(o = 0) == 0,                                                                                                                         /*
	(o•=•0)•==•0    ComparisonExpression{tk: "=="}
	 o•=•0          ReassignmentExpression{tk: "="}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(o = 0) > 0,                                                                                                                          /*
	(o•=•0)•>•0    ComparisonExpression{tk: ">"}
	 o•=•0         ReassignmentExpression{tk: "="}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(o = 0)..0,                                                                                                                           /*
	(o•=•0)..0    RangeLiteral{!last}
	 o•=•0        ReassignmentExpression{tk: "="}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0 + 0) && 0,                                                                                                                         /*
	(0•+•0)•&&•0    AndExpression{tk: "&&"}
	 0•+•0          OperationExpression{tk: "+"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 + 0) || 0,                                                                                                                         /*
	(0•+•0)•||•0    OrExpression{tk: "||"}
	 0•+•0          OperationExpression{tk: "+"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 + 0) = 0,                                                                                                                          /*
	(0•+•0)•=•0    ReassignmentExpression{tk: "="}
	 0•+•0         OperationExpression{tk: "+"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 + 0) + 0,                                                                                                                          /*
	(0•+•0)•+•0    OperationExpression{tk: "+"}
	 0•+•0         OperationExpression{tk: "+"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 + 0) * 0,                                                                                                                          /*
	(0•+•0)•*•0    OperationExpression{tk: "*"}
	 0•+•0         OperationExpression{tk: "+"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 + 0) & 0,                                                                                                                          /*
	(0•+•0)•&•0    OperationExpression{tk: "&"}
	 0•+•0         OperationExpression{tk: "+"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 + 0) << 0,                                                                                                                         /*
	(0•+•0)•<<•0    OperationExpression{tk: "<<"}
	 0•+•0          OperationExpression{tk: "+"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 + 0) == 0,                                                                                                                         /*
	(0•+•0)•==•0    ComparisonExpression{tk: "=="}
	 0•+•0          OperationExpression{tk: "+"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 + 0) > 0,                                                                                                                          /*
	(0•+•0)•>•0    ComparisonExpression{tk: ">"}
	 0•+•0         OperationExpression{tk: "+"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 + 0)..0,                                                                                                                           /*
	(0•+•0)..0    RangeLiteral{!last}
	 0•+•0        OperationExpression{tk: "+"}
	 0            Literal{kind: Integer}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0 * 0) && 0,                                                                                                                         /*
	(0•*•0)•&&•0    AndExpression{tk: "&&"}
	 0•*•0          OperationExpression{tk: "*"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 * 0) || 0,                                                                                                                         /*
	(0•*•0)•||•0    OrExpression{tk: "||"}
	 0•*•0          OperationExpression{tk: "*"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 * 0) = 0,                                                                                                                          /*
	(0•*•0)•=•0    ReassignmentExpression{tk: "="}
	 0•*•0         OperationExpression{tk: "*"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 * 0) + 0,                                                                                                                          /*
	(0•*•0)•+•0    OperationExpression{tk: "+"}
	 0•*•0         OperationExpression{tk: "*"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 * 0) * 0,                                                                                                                          /*
	(0•*•0)•*•0    OperationExpression{tk: "*"}
	 0•*•0         OperationExpression{tk: "*"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 * 0) & 0,                                                                                                                          /*
	(0•*•0)•&•0    OperationExpression{tk: "&"}
	 0•*•0         OperationExpression{tk: "*"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 * 0) << 0,                                                                                                                         /*
	(0•*•0)•<<•0    OperationExpression{tk: "<<"}
	 0•*•0          OperationExpression{tk: "*"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 * 0) == 0,                                                                                                                         /*
	(0•*•0)•==•0    ComparisonExpression{tk: "=="}
	 0•*•0          OperationExpression{tk: "*"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 * 0) > 0,                                                                                                                          /*
	(0•*•0)•>•0    ComparisonExpression{tk: ">"}
	 0•*•0         OperationExpression{tk: "*"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 * 0)..0,                                                                                                                           /*
	(0•*•0)..0    RangeLiteral{!last}
	 0•*•0        OperationExpression{tk: "*"}
	 0            Literal{kind: Integer}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0 & 0) && 0,                                                                                                                         /*
	(0•&•0)•&&•0    AndExpression{tk: "&&"}
	 0•&•0          OperationExpression{tk: "&"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 & 0) || 0,                                                                                                                         /*
	(0•&•0)•||•0    OrExpression{tk: "||"}
	 0•&•0          OperationExpression{tk: "&"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 & 0) = 0,                                                                                                                          /*
	(0•&•0)•=•0    ReassignmentExpression{tk: "="}
	 0•&•0         OperationExpression{tk: "&"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 & 0) + 0,                                                                                                                          /*
	(0•&•0)•+•0    OperationExpression{tk: "+"}
	 0•&•0         OperationExpression{tk: "&"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 & 0) * 0,                                                                                                                          /*
	(0•&•0)•*•0    OperationExpression{tk: "*"}
	 0•&•0         OperationExpression{tk: "&"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 & 0) & 0,                                                                                                                          /*
	(0•&•0)•&•0    OperationExpression{tk: "&"}
	 0•&•0         OperationExpression{tk: "&"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 & 0) << 0,                                                                                                                         /*
	(0•&•0)•<<•0    OperationExpression{tk: "<<"}
	 0•&•0          OperationExpression{tk: "&"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 & 0) == 0,                                                                                                                         /*
	(0•&•0)•==•0    ComparisonExpression{tk: "=="}
	 0•&•0          OperationExpression{tk: "&"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 & 0) > 0,                                                                                                                          /*
	(0•&•0)•>•0    ComparisonExpression{tk: ">"}
	 0•&•0         OperationExpression{tk: "&"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 & 0)..0,                                                                                                                           /*
	(0•&•0)..0    RangeLiteral{!last}
	 0•&•0        OperationExpression{tk: "&"}
	 0            Literal{kind: Integer}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0 << 0) && 0,                                                                                                                        /*
	(0•<<•0)•&&•0    AndExpression{tk: "&&"}
	 0•<<•0          OperationExpression{tk: "<<"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 << 0) || 0,                                                                                                                        /*
	(0•<<•0)•||•0    OrExpression{tk: "||"}
	 0•<<•0          OperationExpression{tk: "<<"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 << 0) = 0,                                                                                                                         /*
	(0•<<•0)•=•0    ReassignmentExpression{tk: "="}
	 0•<<•0         OperationExpression{tk: "<<"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 << 0) + 0,                                                                                                                         /*
	(0•<<•0)•+•0    OperationExpression{tk: "+"}
	 0•<<•0         OperationExpression{tk: "<<"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 << 0) * 0,                                                                                                                         /*
	(0•<<•0)•*•0    OperationExpression{tk: "*"}
	 0•<<•0         OperationExpression{tk: "<<"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 << 0) & 0,                                                                                                                         /*
	(0•<<•0)•&•0    OperationExpression{tk: "&"}
	 0•<<•0         OperationExpression{tk: "<<"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 << 0) << 0,                                                                                                                        /*
	(0•<<•0)•<<•0    OperationExpression{tk: "<<"}
	 0•<<•0          OperationExpression{tk: "<<"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 << 0) == 0,                                                                                                                        /*
	(0•<<•0)•==•0    ComparisonExpression{tk: "=="}
	 0•<<•0          OperationExpression{tk: "<<"}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 << 0) > 0,                                                                                                                         /*
	(0•<<•0)•>•0    ComparisonExpression{tk: ">"}
	 0•<<•0         OperationExpression{tk: "<<"}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 << 0)..0,                                                                                                                          /*
	(0•<<•0)..0    RangeLiteral{!last}
	 0•<<•0        OperationExpression{tk: "<<"}
	 0             Literal{kind: Integer}
	      0        Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 == 0) && 0,                                                                                                                        /*
	(0•==•0)•&&•0    AndExpression{tk: "&&"}
	 0•==•0          ComparisonExpression{tk: "=="}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 == 0) || 0,                                                                                                                        /*
	(0•==•0)•||•0    OrExpression{tk: "||"}
	 0•==•0          ComparisonExpression{tk: "=="}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 == 0) = 0,                                                                                                                         /*
	(0•==•0)•=•0    ReassignmentExpression{tk: "="}
	 0•==•0         ComparisonExpression{tk: "=="}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 == 0) + 0,                                                                                                                         /*
	(0•==•0)•+•0    OperationExpression{tk: "+"}
	 0•==•0         ComparisonExpression{tk: "=="}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 == 0) * 0,                                                                                                                         /*
	(0•==•0)•*•0    OperationExpression{tk: "*"}
	 0•==•0         ComparisonExpression{tk: "=="}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 == 0) & 0,                                                                                                                         /*
	(0•==•0)•&•0    OperationExpression{tk: "&"}
	 0•==•0         ComparisonExpression{tk: "=="}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 == 0) << 0,                                                                                                                        /*
	(0•==•0)•<<•0    OperationExpression{tk: "<<"}
	 0•==•0          ComparisonExpression{tk: "=="}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 == 0) == 0,                                                                                                                        /*
	(0•==•0)•==•0    ComparisonExpression{tk: "=="}
	 0•==•0          ComparisonExpression{tk: "=="}
	 0               Literal{kind: Integer}
	      0          Literal{kind: Integer}
	            0    Literal{kind: Integer}                                                                                               */
	(0 == 0) > 0,                                                                                                                         /*
	(0•==•0)•>•0    ComparisonExpression{tk: ">"}
	 0•==•0         ComparisonExpression{tk: "=="}
	 0              Literal{kind: Integer}
	      0         Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 == 0)..0,                                                                                                                          /*
	(0•==•0)..0    RangeLiteral{!last}
	 0•==•0        ComparisonExpression{tk: "=="}
	 0             Literal{kind: Integer}
	      0        Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 > 0) && 0,                                                                                                                         /*
	(0•>•0)•&&•0    AndExpression{tk: "&&"}
	 0•>•0          ComparisonExpression{tk: ">"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 > 0) || 0,                                                                                                                         /*
	(0•>•0)•||•0    OrExpression{tk: "||"}
	 0•>•0          ComparisonExpression{tk: ">"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 > 0) = 0,                                                                                                                          /*
	(0•>•0)•=•0    ReassignmentExpression{tk: "="}
	 0•>•0         ComparisonExpression{tk: ">"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 > 0) + 0,                                                                                                                          /*
	(0•>•0)•+•0    OperationExpression{tk: "+"}
	 0•>•0         ComparisonExpression{tk: ">"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 > 0) * 0,                                                                                                                          /*
	(0•>•0)•*•0    OperationExpression{tk: "*"}
	 0•>•0         ComparisonExpression{tk: ">"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 > 0) & 0,                                                                                                                          /*
	(0•>•0)•&•0    OperationExpression{tk: "&"}
	 0•>•0         ComparisonExpression{tk: ">"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 > 0) << 0,                                                                                                                         /*
	(0•>•0)•<<•0    OperationExpression{tk: "<<"}
	 0•>•0          ComparisonExpression{tk: ">"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 > 0) == 0,                                                                                                                         /*
	(0•>•0)•==•0    ComparisonExpression{tk: "=="}
	 0•>•0          ComparisonExpression{tk: ">"}
	 0              Literal{kind: Integer}
	     0          Literal{kind: Integer}
	           0    Literal{kind: Integer}                                                                                                */
	(0 > 0) > 0,                                                                                                                          /*
	(0•>•0)•>•0    ComparisonExpression{tk: ">"}
	 0•>•0         ComparisonExpression{tk: ">"}
	 0             Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0 > 0)..0,                                                                                                                           /*
	(0•>•0)..0    RangeLiteral{!last}
	 0•>•0        ComparisonExpression{tk: ">"}
	 0            Literal{kind: Integer}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0..0) && 0,                                                                                                                          /*
	(0..0)•&&•0    AndExpression{tk: "&&"}
	 0..0          RangeLiteral{!last}
	 0             Literal{kind: Integer}
	    0          Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0..0) || 0,                                                                                                                          /*
	(0..0)•||•0    OrExpression{tk: "||"}
	 0..0          RangeLiteral{!last}
	 0             Literal{kind: Integer}
	    0          Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0..0) = 0,                                                                                                                           /*
	(0..0)•=•0    ReassignmentExpression{tk: "="}
	(0..0)        TupleLiteral
	 0..0         RangeLiteral{!last}
	 0            Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0..0) + 0,                                                                                                                           /*
	(0..0)•+•0    OperationExpression{tk: "+"}
	 0..0         RangeLiteral{!last}
	 0            Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0..0) * 0,                                                                                                                           /*
	(0..0)•*•0    OperationExpression{tk: "*"}
	 0..0         RangeLiteral{!last}
	 0            Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0..0) & 0,                                                                                                                           /*
	(0..0)•&•0    OperationExpression{tk: "&"}
	 0..0         RangeLiteral{!last}
	 0            Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0..0) << 0,                                                                                                                          /*
	(0..0)•<<•0    OperationExpression{tk: "<<"}
	 0..0          RangeLiteral{!last}
	 0             Literal{kind: Integer}
	    0          Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0..0) == 0,                                                                                                                          /*
	(0..0)•==•0    ComparisonExpression{tk: "=="}
	 0..0          RangeLiteral{!last}
	 0             Literal{kind: Integer}
	    0          Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	(0..0) > 0,                                                                                                                           /*
	(0..0)•>•0    ComparisonExpression{tk: ">"}
	 0..0         RangeLiteral{!last}
	 0            Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	(0..0)..0,                                                                                                                            /*
	(0..0)..0    RangeLiteral{!last}
	 0..0        RangeLiteral{!last}
	 0           Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	// 0 x 0 x 0 ===========================================================
	//•0•x•0•x•0•===========================================================    Comment{line}
	0 && 0 && 0,                                                                                                                          /*
	0•&&•0•&&•0    AndExpression{tk: "&&"}
	0•&&•0         AndExpression{tk: "&&"}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 && 0 || 0,                                                                                                                          /*
	0•&&•0•||•0    OrExpression{tk: "||"}
	0•&&•0         AndExpression{tk: "&&"}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 && o = 0,                                                                                                                           /*
	0•&&•o•=•0    ReassignmentExpression{tk: "="}
	0•&&•o        AndExpression{tk: "&&"}
	0             Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 && 0 + 0,                                                                                                                           /*
	0•&&•0•+•0    AndExpression{tk: "&&"}
	0             Literal{kind: Integer}
	     0•+•0    OperationExpression{tk: "+"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 && 0 * 0,                                                                                                                           /*
	0•&&•0•*•0    AndExpression{tk: "&&"}
	0             Literal{kind: Integer}
	     0•*•0    OperationExpression{tk: "*"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 && 0 & 0,                                                                                                                           /*
	0•&&•0•&•0    AndExpression{tk: "&&"}
	0             Literal{kind: Integer}
	     0•&•0    OperationExpression{tk: "&"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 && 0 << 0,                                                                                                                          /*
	0•&&•0•<<•0    AndExpression{tk: "&&"}
	0              Literal{kind: Integer}
	     0•<<•0    OperationExpression{tk: "<<"}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 && 0 == 0,                                                                                                                          /*
	0•&&•0•==•0    AndExpression{tk: "&&"}
	0              Literal{kind: Integer}
	     0•==•0    ComparisonExpression{tk: "=="}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 && 0 > 0,                                                                                                                           /*
	0•&&•0•>•0    AndExpression{tk: "&&"}
	0             Literal{kind: Integer}
	     0•>•0    ComparisonExpression{tk: ">"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 && 0..0,                                                                                                                            /*
	0•&&•0..0    RangeLiteral{!last}
	0•&&•0       AndExpression{tk: "&&"}
	0            Literal{kind: Integer}
	     0       Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 || 0 && 0,                                                                                                                          /*
	0•||•0•&&•0    OrExpression{tk: "||"}
	0              Literal{kind: Integer}
	     0•&&•0    AndExpression{tk: "&&"}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 || 0 || 0,                                                                                                                          /*
	0•||•0•||•0    OrExpression{tk: "||"}
	0•||•0         OrExpression{tk: "||"}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 || o = 0,                                                                                                                           /*
	0•||•o•=•0    ReassignmentExpression{tk: "="}
	0•||•o        OrExpression{tk: "||"}
	0             Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 || 0 + 0,                                                                                                                           /*
	0•||•0•+•0    OrExpression{tk: "||"}
	0             Literal{kind: Integer}
	     0•+•0    OperationExpression{tk: "+"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 || 0 * 0,                                                                                                                           /*
	0•||•0•*•0    OrExpression{tk: "||"}
	0             Literal{kind: Integer}
	     0•*•0    OperationExpression{tk: "*"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 || 0 & 0,                                                                                                                           /*
	0•||•0•&•0    OrExpression{tk: "||"}
	0             Literal{kind: Integer}
	     0•&•0    OperationExpression{tk: "&"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 || 0 << 0,                                                                                                                          /*
	0•||•0•<<•0    OrExpression{tk: "||"}
	0              Literal{kind: Integer}
	     0•<<•0    OperationExpression{tk: "<<"}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 || 0 == 0,                                                                                                                          /*
	0•||•0•==•0    OrExpression{tk: "||"}
	0              Literal{kind: Integer}
	     0•==•0    ComparisonExpression{tk: "=="}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 || 0 > 0,                                                                                                                           /*
	0•||•0•>•0    OrExpression{tk: "||"}
	0             Literal{kind: Integer}
	     0•>•0    ComparisonExpression{tk: ">"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 || 0..0,                                                                                                                            /*
	0•||•0..0    RangeLiteral{!last}
	0•||•0       OrExpression{tk: "||"}
	0            Literal{kind: Integer}
	     0       Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 && 0,                                                                                                                           /*
	o•=•0•&&•0    ReassignmentExpression{tk: "="}
	    0•&&•0    AndExpression{tk: "&&"}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o = 0 || 0,                                                                                                                           /*
	o•=•0•||•0    ReassignmentExpression{tk: "="}
	    0•||•0    OrExpression{tk: "||"}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o = o = 0,                                                                                                                            /*
	o•=•o•=•0    ReassignmentExpression{tk: "="}
	    o•=•0    ReassignmentExpression{tk: "="}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 + 0,                                                                                                                            /*
	o•=•0•+•0    ReassignmentExpression{tk: "="}
	    0•+•0    OperationExpression{tk: "+"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 * 0,                                                                                                                            /*
	o•=•0•*•0    ReassignmentExpression{tk: "="}
	    0•*•0    OperationExpression{tk: "*"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 & 0,                                                                                                                            /*
	o•=•0•&•0    ReassignmentExpression{tk: "="}
	    0•&•0    OperationExpression{tk: "&"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0 << 0,                                                                                                                           /*
	o•=•0•<<•0    ReassignmentExpression{tk: "="}
	    0•<<•0    OperationExpression{tk: "<<"}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o = 0 == 0,                                                                                                                           /*
	o•=•0•==•0    ReassignmentExpression{tk: "="}
	    0•==•0    ComparisonExpression{tk: "=="}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	o = 0 > 0,                                                                                                                            /*
	o•=•0•>•0    ReassignmentExpression{tk: "="}
	    0•>•0    ComparisonExpression{tk: ">"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	o = 0..0,                                                                                                                             /*
	o•=•0..0    ReassignmentExpression{tk: "="}
	    0..0    RangeLiteral{!last}
	    0       Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0 + 0 && 0,                                                                                                                           /*
	0•+•0•&&•0    AndExpression{tk: "&&"}
	0•+•0         OperationExpression{tk: "+"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 + 0 || 0,                                                                                                                           /*
	0•+•0•||•0    OrExpression{tk: "||"}
	0•+•0         OperationExpression{tk: "+"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 + o = 0,                                                                                                                            /*
	0•+•o•=•0    ReassignmentExpression{tk: "="}
	0•+•o        OperationExpression{tk: "+"}
	0            Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 + 0 + 0,                                                                                                                            /*
	0•+•0•+•0    OperationExpression{tk: "+"}
	0•+•0        OperationExpression{tk: "+"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 + 0 * 0,                                                                                                                            /*
	0•+•0•*•0    OperationExpression{tk: "+"}
	0            Literal{kind: Integer}
	    0•*•0    OperationExpression{tk: "*"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 + 0 & 0,                                                                                                                            /*
	0•+•0•&•0    OperationExpression{tk: "&"}
	0•+•0        OperationExpression{tk: "+"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 + 0 << 0,                                                                                                                           /*
	0•+•0•<<•0    OperationExpression{tk: "<<"}
	0•+•0         OperationExpression{tk: "+"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 + 0 == 0,                                                                                                                           /*
	0•+•0•==•0    ComparisonExpression{tk: "=="}
	0•+•0         OperationExpression{tk: "+"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 + 0 > 0,                                                                                                                            /*
	0•+•0•>•0    ComparisonExpression{tk: ">"}
	0•+•0        OperationExpression{tk: "+"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 + 0..0,                                                                                                                             /*
	0•+•0..0    RangeLiteral{!last}
	0•+•0       OperationExpression{tk: "+"}
	0           Literal{kind: Integer}
	    0       Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0 * 0 && 0,                                                                                                                           /*
	0•*•0•&&•0    AndExpression{tk: "&&"}
	0•*•0         OperationExpression{tk: "*"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 * 0 || 0,                                                                                                                           /*
	0•*•0•||•0    OrExpression{tk: "||"}
	0•*•0         OperationExpression{tk: "*"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 * o = 0,                                                                                                                            /*
	0•*•o•=•0    ReassignmentExpression{tk: "="}
	0•*•o        OperationExpression{tk: "*"}
	0            Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 * 0 + 0,                                                                                                                            /*
	0•*•0•+•0    OperationExpression{tk: "+"}
	0•*•0        OperationExpression{tk: "*"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 * 0 * 0,                                                                                                                            /*
	0•*•0•*•0    OperationExpression{tk: "*"}
	0•*•0        OperationExpression{tk: "*"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 * 0 & 0,                                                                                                                            /*
	0•*•0•&•0    OperationExpression{tk: "&"}
	0•*•0        OperationExpression{tk: "*"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 * 0 << 0,                                                                                                                           /*
	0•*•0•<<•0    OperationExpression{tk: "<<"}
	0•*•0         OperationExpression{tk: "*"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 * 0 == 0,                                                                                                                           /*
	0•*•0•==•0    ComparisonExpression{tk: "=="}
	0•*•0         OperationExpression{tk: "*"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 * 0 > 0,                                                                                                                            /*
	0•*•0•>•0    ComparisonExpression{tk: ">"}
	0•*•0        OperationExpression{tk: "*"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 * 0..0,                                                                                                                             /*
	0•*•0..0    RangeLiteral{!last}
	0•*•0       OperationExpression{tk: "*"}
	0           Literal{kind: Integer}
	    0       Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0 & 0 && 0,                                                                                                                           /*
	0•&•0•&&•0    AndExpression{tk: "&&"}
	0•&•0         OperationExpression{tk: "&"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 & 0 || 0,                                                                                                                           /*
	0•&•0•||•0    OrExpression{tk: "||"}
	0•&•0         OperationExpression{tk: "&"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 & o = 0,                                                                                                                            /*
	0•&•o•=•0    ReassignmentExpression{tk: "="}
	0•&•o        OperationExpression{tk: "&"}
	0            Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 & 0 + 0,                                                                                                                            /*
	0•&•0•+•0    OperationExpression{tk: "&"}
	0            Literal{kind: Integer}
	    0•+•0    OperationExpression{tk: "+"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 & 0 * 0,                                                                                                                            /*
	0•&•0•*•0    OperationExpression{tk: "&"}
	0            Literal{kind: Integer}
	    0•*•0    OperationExpression{tk: "*"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 & 0 & 0,                                                                                                                            /*
	0•&•0•&•0    OperationExpression{tk: "&"}
	0•&•0        OperationExpression{tk: "&"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 & 0 << 0,                                                                                                                           /*
	0•&•0•<<•0    OperationExpression{tk: "&"}
	0             Literal{kind: Integer}
	    0•<<•0    OperationExpression{tk: "<<"}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 & 0 == 0,                                                                                                                           /*
	0•&•0•==•0    ComparisonExpression{tk: "=="}
	0•&•0         OperationExpression{tk: "&"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 & 0 > 0,                                                                                                                            /*
	0•&•0•>•0    ComparisonExpression{tk: ">"}
	0•&•0        OperationExpression{tk: "&"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 & 0..0,                                                                                                                             /*
	0•&•0..0    RangeLiteral{!last}
	0•&•0       OperationExpression{tk: "&"}
	0           Literal{kind: Integer}
	    0       Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0 << 0 && 0,                                                                                                                          /*
	0•<<•0•&&•0    AndExpression{tk: "&&"}
	0•<<•0         OperationExpression{tk: "<<"}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 << 0 || 0,                                                                                                                          /*
	0•<<•0•||•0    OrExpression{tk: "||"}
	0•<<•0         OperationExpression{tk: "<<"}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 << o = 0,                                                                                                                           /*
	0•<<•o•=•0    ReassignmentExpression{tk: "="}
	0•<<•o        OperationExpression{tk: "<<"}
	0             Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 << 0 + 0,                                                                                                                           /*
	0•<<•0•+•0    OperationExpression{tk: "<<"}
	0             Literal{kind: Integer}
	     0•+•0    OperationExpression{tk: "+"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 << 0 * 0,                                                                                                                           /*
	0•<<•0•*•0    OperationExpression{tk: "<<"}
	0             Literal{kind: Integer}
	     0•*•0    OperationExpression{tk: "*"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 << 0 & 0,                                                                                                                           /*
	0•<<•0•&•0    OperationExpression{tk: "&"}
	0•<<•0        OperationExpression{tk: "<<"}
	0             Literal{kind: Integer}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 << 0 << 0,                                                                                                                          /*
	0•<<•0•<<•0    OperationExpression{tk: "<<"}
	0•<<•0         OperationExpression{tk: "<<"}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 << 0 == 0,                                                                                                                          /*
	0•<<•0•==•0    ComparisonExpression{tk: "=="}
	0•<<•0         OperationExpression{tk: "<<"}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 << 0 > 0,                                                                                                                           /*
	0•<<•0•>•0    ComparisonExpression{tk: ">"}
	0•<<•0        OperationExpression{tk: "<<"}
	0             Literal{kind: Integer}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 << 0..0,                                                                                                                            /*
	0•<<•0..0    RangeLiteral{!last}
	0•<<•0       OperationExpression{tk: "<<"}
	0            Literal{kind: Integer}
	     0       Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 == 0 && 0,                                                                                                                          /*
	0•==•0•&&•0    AndExpression{tk: "&&"}
	0•==•0         ComparisonExpression{tk: "=="}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 == 0 || 0,                                                                                                                          /*
	0•==•0•||•0    OrExpression{tk: "||"}
	0•==•0         ComparisonExpression{tk: "=="}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 == o = 0,                                                                                                                           /*
	0•==•o•=•0    ReassignmentExpression{tk: "="}
	0•==•o        ComparisonExpression{tk: "=="}
	0             Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 == 0 + 0,                                                                                                                           /*
	0•==•0•+•0    ComparisonExpression{tk: "=="}
	0             Literal{kind: Integer}
	     0•+•0    OperationExpression{tk: "+"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 == 0 * 0,                                                                                                                           /*
	0•==•0•*•0    ComparisonExpression{tk: "=="}
	0             Literal{kind: Integer}
	     0•*•0    OperationExpression{tk: "*"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 == 0 & 0,                                                                                                                           /*
	0•==•0•&•0    ComparisonExpression{tk: "=="}
	0             Literal{kind: Integer}
	     0•&•0    OperationExpression{tk: "&"}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 == 0 << 0,                                                                                                                          /*
	0•==•0•<<•0    ComparisonExpression{tk: "=="}
	0              Literal{kind: Integer}
	     0•<<•0    OperationExpression{tk: "<<"}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 == 0 == 0,                                                                                                                          /*
	0•==•0•==•0    ComparisonExpression{tk: "=="}
	0•==•0         ComparisonExpression{tk: "=="}
	0              Literal{kind: Integer}
	     0         Literal{kind: Integer}
	          0    Literal{kind: Integer}                                                                                                 */
	0 == 0 > 0,                                                                                                                           /*
	0•==•0•>•0    ComparisonExpression{tk: ">"}
	0•==•0        ComparisonExpression{tk: "=="}
	0             Literal{kind: Integer}
	     0        Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 == 0..0,                                                                                                                            /*
	0•==•0..0    RangeLiteral{!last}
	0•==•0       ComparisonExpression{tk: "=="}
	0            Literal{kind: Integer}
	     0       Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 > 0 && 0,                                                                                                                           /*
	0•>•0•&&•0    AndExpression{tk: "&&"}
	0•>•0         ComparisonExpression{tk: ">"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 > 0 || 0,                                                                                                                           /*
	0•>•0•||•0    OrExpression{tk: "||"}
	0•>•0         ComparisonExpression{tk: ">"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 > o = 0,                                                                                                                            /*
	0•>•o•=•0    ReassignmentExpression{tk: "="}
	0•>•o        ComparisonExpression{tk: ">"}
	0            Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 > 0 + 0,                                                                                                                            /*
	0•>•0•+•0    ComparisonExpression{tk: ">"}
	0            Literal{kind: Integer}
	    0•+•0    OperationExpression{tk: "+"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 > 0 * 0,                                                                                                                            /*
	0•>•0•*•0    ComparisonExpression{tk: ">"}
	0            Literal{kind: Integer}
	    0•*•0    OperationExpression{tk: "*"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 > 0 & 0,                                                                                                                            /*
	0•>•0•&•0    ComparisonExpression{tk: ">"}
	0            Literal{kind: Integer}
	    0•&•0    OperationExpression{tk: "&"}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 > 0 << 0,                                                                                                                           /*
	0•>•0•<<•0    ComparisonExpression{tk: ">"}
	0             Literal{kind: Integer}
	    0•<<•0    OperationExpression{tk: "<<"}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 > 0 == 0,                                                                                                                           /*
	0•>•0•==•0    ComparisonExpression{tk: "=="}
	0•>•0         ComparisonExpression{tk: ">"}
	0             Literal{kind: Integer}
	    0         Literal{kind: Integer}
	         0    Literal{kind: Integer}                                                                                                  */
	0 > 0 > 0,                                                                                                                            /*
	0•>•0•>•0    ComparisonExpression{tk: ">"}
	0•>•0        ComparisonExpression{tk: ">"}
	0            Literal{kind: Integer}
	    0        Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0 > 0..0,                                                                                                                             /*
	0•>•0..0    RangeLiteral{!last}
	0•>•0       ComparisonExpression{tk: ">"}
	0           Literal{kind: Integer}
	    0       Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0..0 && 0,                                                                                                                            /*
	0..0•&&•0    RangeLiteral{!last}
	0            Literal{kind: Integer}
	   0•&&•0    AndExpression{tk: "&&"}
	   0         Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0..0 || 0,                                                                                                                            /*
	0..0•||•0    RangeLiteral{!last}
	0            Literal{kind: Integer}
	   0•||•0    OrExpression{tk: "||"}
	   0         Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0..o = 0,                                                                                                                             /*
	0..o•=•0    ReassignmentExpression{tk: "="}
	0..o        RangeLiteral{!last}
	0           Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0..0 + 0,                                                                                                                             /*
	0..0•+•0    RangeLiteral{!last}
	0           Literal{kind: Integer}
	   0•+•0    OperationExpression{tk: "+"}
	   0        Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0..0 * 0,                                                                                                                             /*
	0..0•*•0    RangeLiteral{!last}
	0           Literal{kind: Integer}
	   0•*•0    OperationExpression{tk: "*"}
	   0        Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0..0 & 0,                                                                                                                             /*
	0..0•&•0    RangeLiteral{!last}
	0           Literal{kind: Integer}
	   0•&•0    OperationExpression{tk: "&"}
	   0        Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0..0 << 0,                                                                                                                            /*
	0..0•<<•0    RangeLiteral{!last}
	0            Literal{kind: Integer}
	   0•<<•0    OperationExpression{tk: "<<"}
	   0         Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0..0 == 0,                                                                                                                            /*
	0..0•==•0    RangeLiteral{!last}
	0            Literal{kind: Integer}
	   0•==•0    ComparisonExpression{tk: "=="}
	   0         Literal{kind: Integer}
	        0    Literal{kind: Integer}                                                                                                   */
	0..0 > 0,                                                                                                                             /*
	0..0•>•0    RangeLiteral{!last}
	0           Literal{kind: Integer}
	   0•>•0    ComparisonExpression{tk: ">"}
	   0        Literal{kind: Integer}
	       0    Literal{kind: Integer}                                                                                                    */
	0..0..0,                                                                                                                              /*
	0..0..0    RangeLiteral{!last}
	0..0       RangeLiteral{!last}
	0          Literal{kind: Integer}
	   0       Literal{kind: Integer}
	      0    Literal{kind: Integer}                                                                                                     */
	// 0 x (0 x 0) ===========================================================
	//•0•x•(0•x•0)•===========================================================    Comment{line}
	0 && (0 && 0),                                                                                                                        /*
	0•&&•(0•&&•0)    AndExpression{tk: "&&"}
	0                Literal{kind: Integer}
	      0•&&•0     AndExpression{tk: "&&"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 && (0 || 0),                                                                                                                        /*
	0•&&•(0•||•0)    AndExpression{tk: "&&"}
	0                Literal{kind: Integer}
	      0•||•0     OrExpression{tk: "||"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 && (o = 0),                                                                                                                         /*
	0•&&•(o•=•0)    AndExpression{tk: "&&"}
	0               Literal{kind: Integer}
	      o•=•0     ReassignmentExpression{tk: "="}
	          0     Literal{kind: Integer}                                                                                                */
	0 && (0 + 0),                                                                                                                         /*
	0•&&•(0•+•0)    AndExpression{tk: "&&"}
	0               Literal{kind: Integer}
	      0•+•0     OperationExpression{tk: "+"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 && (0 * 0),                                                                                                                         /*
	0•&&•(0•*•0)    AndExpression{tk: "&&"}
	0               Literal{kind: Integer}
	      0•*•0     OperationExpression{tk: "*"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 && (0 & 0),                                                                                                                         /*
	0•&&•(0•&•0)    AndExpression{tk: "&&"}
	0               Literal{kind: Integer}
	      0•&•0     OperationExpression{tk: "&"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 && (0 << 0),                                                                                                                        /*
	0•&&•(0•<<•0)    AndExpression{tk: "&&"}
	0                Literal{kind: Integer}
	      0•<<•0     OperationExpression{tk: "<<"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 && (0 == 0),                                                                                                                        /*
	0•&&•(0•==•0)    AndExpression{tk: "&&"}
	0                Literal{kind: Integer}
	      0•==•0     ComparisonExpression{tk: "=="}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 && (0 > 0),                                                                                                                         /*
	0•&&•(0•>•0)    AndExpression{tk: "&&"}
	0               Literal{kind: Integer}
	      0•>•0     ComparisonExpression{tk: ">"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 && (0..0),                                                                                                                          /*
	0•&&•(0..0)    AndExpression{tk: "&&"}
	0              Literal{kind: Integer}
	      0..0     RangeLiteral{!last}
	      0        Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 || (0 && 0),                                                                                                                        /*
	0•||•(0•&&•0)    OrExpression{tk: "||"}
	0                Literal{kind: Integer}
	      0•&&•0     AndExpression{tk: "&&"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 || (0 || 0),                                                                                                                        /*
	0•||•(0•||•0)    OrExpression{tk: "||"}
	0                Literal{kind: Integer}
	      0•||•0     OrExpression{tk: "||"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 || (o = 0),                                                                                                                         /*
	0•||•(o•=•0)    OrExpression{tk: "||"}
	0               Literal{kind: Integer}
	      o•=•0     ReassignmentExpression{tk: "="}
	          0     Literal{kind: Integer}                                                                                                */
	0 || (0 + 0),                                                                                                                         /*
	0•||•(0•+•0)    OrExpression{tk: "||"}
	0               Literal{kind: Integer}
	      0•+•0     OperationExpression{tk: "+"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 || (0 * 0),                                                                                                                         /*
	0•||•(0•*•0)    OrExpression{tk: "||"}
	0               Literal{kind: Integer}
	      0•*•0     OperationExpression{tk: "*"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 || (0 & 0),                                                                                                                         /*
	0•||•(0•&•0)    OrExpression{tk: "||"}
	0               Literal{kind: Integer}
	      0•&•0     OperationExpression{tk: "&"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 || (0 << 0),                                                                                                                        /*
	0•||•(0•<<•0)    OrExpression{tk: "||"}
	0                Literal{kind: Integer}
	      0•<<•0     OperationExpression{tk: "<<"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 || (0 == 0),                                                                                                                        /*
	0•||•(0•==•0)    OrExpression{tk: "||"}
	0                Literal{kind: Integer}
	      0•==•0     ComparisonExpression{tk: "=="}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 || (0 > 0),                                                                                                                         /*
	0•||•(0•>•0)    OrExpression{tk: "||"}
	0               Literal{kind: Integer}
	      0•>•0     ComparisonExpression{tk: ">"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 || (0..0),                                                                                                                          /*
	0•||•(0..0)    OrExpression{tk: "||"}
	0              Literal{kind: Integer}
	      0..0     RangeLiteral{!last}
	      0        Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	o = (0 && 0),                                                                                                                         /*
	o•=•(0•&&•0)    ReassignmentExpression{tk: "="}
	     0•&&•0     AndExpression{tk: "&&"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	o = (0 || 0),                                                                                                                         /*
	o•=•(0•||•0)    ReassignmentExpression{tk: "="}
	     0•||•0     OrExpression{tk: "||"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	o = (o = 0),                                                                                                                          /*
	o•=•(o•=•0)    ReassignmentExpression{tk: "="}
	     o•=•0     ReassignmentExpression{tk: "="}
	         0     Literal{kind: Integer}                                                                                                 */
	o = (0 + 0),                                                                                                                          /*
	o•=•(0•+•0)    ReassignmentExpression{tk: "="}
	     0•+•0     OperationExpression{tk: "+"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	o = (0 * 0),                                                                                                                          /*
	o•=•(0•*•0)    ReassignmentExpression{tk: "="}
	     0•*•0     OperationExpression{tk: "*"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	o = (0 & 0),                                                                                                                          /*
	o•=•(0•&•0)    ReassignmentExpression{tk: "="}
	     0•&•0     OperationExpression{tk: "&"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	o = (0 << 0),                                                                                                                         /*
	o•=•(0•<<•0)    ReassignmentExpression{tk: "="}
	     0•<<•0     OperationExpression{tk: "<<"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	o = (0 == 0),                                                                                                                         /*
	o•=•(0•==•0)    ReassignmentExpression{tk: "="}
	     0•==•0     ComparisonExpression{tk: "=="}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	o = (0 > 0),                                                                                                                          /*
	o•=•(0•>•0)    ReassignmentExpression{tk: "="}
	     0•>•0     ComparisonExpression{tk: ">"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	o = (0..0),                                                                                                                           /*
	o•=•(0..0)    ReassignmentExpression{tk: "="}
	     0..0     RangeLiteral{!last}
	     0        Literal{kind: Integer}
	        0     Literal{kind: Integer}                                                                                                  */
	0 + (0 && 0),                                                                                                                         /*
	0•+•(0•&&•0)    OperationExpression{tk: "+"}
	0               Literal{kind: Integer}
	     0•&&•0     AndExpression{tk: "&&"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 + (0 || 0),                                                                                                                         /*
	0•+•(0•||•0)    OperationExpression{tk: "+"}
	0               Literal{kind: Integer}
	     0•||•0     OrExpression{tk: "||"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 + (o = 0),                                                                                                                          /*
	0•+•(o•=•0)    OperationExpression{tk: "+"}
	0              Literal{kind: Integer}
	     o•=•0     ReassignmentExpression{tk: "="}
	         0     Literal{kind: Integer}                                                                                                 */
	0 + (0 + 0),                                                                                                                          /*
	0•+•(0•+•0)    OperationExpression{tk: "+"}
	0              Literal{kind: Integer}
	     0•+•0     OperationExpression{tk: "+"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 + (0 * 0),                                                                                                                          /*
	0•+•(0•*•0)    OperationExpression{tk: "+"}
	0              Literal{kind: Integer}
	     0•*•0     OperationExpression{tk: "*"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 + (0 & 0),                                                                                                                          /*
	0•+•(0•&•0)    OperationExpression{tk: "+"}
	0              Literal{kind: Integer}
	     0•&•0     OperationExpression{tk: "&"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 + (0 << 0),                                                                                                                         /*
	0•+•(0•<<•0)    OperationExpression{tk: "+"}
	0               Literal{kind: Integer}
	     0•<<•0     OperationExpression{tk: "<<"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 + (0 == 0),                                                                                                                         /*
	0•+•(0•==•0)    OperationExpression{tk: "+"}
	0               Literal{kind: Integer}
	     0•==•0     ComparisonExpression{tk: "=="}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 + (0 > 0),                                                                                                                          /*
	0•+•(0•>•0)    OperationExpression{tk: "+"}
	0              Literal{kind: Integer}
	     0•>•0     ComparisonExpression{tk: ">"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 + (0..0),                                                                                                                           /*
	0•+•(0..0)    OperationExpression{tk: "+"}
	0             Literal{kind: Integer}
	     0..0     RangeLiteral{!last}
	     0        Literal{kind: Integer}
	        0     Literal{kind: Integer}                                                                                                  */
	0 * (0 && 0),                                                                                                                         /*
	0•*•(0•&&•0)    OperationExpression{tk: "*"}
	0               Literal{kind: Integer}
	     0•&&•0     AndExpression{tk: "&&"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 * (0 || 0),                                                                                                                         /*
	0•*•(0•||•0)    OperationExpression{tk: "*"}
	0               Literal{kind: Integer}
	     0•||•0     OrExpression{tk: "||"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 * (o = 0),                                                                                                                          /*
	0•*•(o•=•0)    OperationExpression{tk: "*"}
	0              Literal{kind: Integer}
	     o•=•0     ReassignmentExpression{tk: "="}
	         0     Literal{kind: Integer}                                                                                                 */
	0 * (0 + 0),                                                                                                                          /*
	0•*•(0•+•0)    OperationExpression{tk: "*"}
	0              Literal{kind: Integer}
	     0•+•0     OperationExpression{tk: "+"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 * (0 * 0),                                                                                                                          /*
	0•*•(0•*•0)    OperationExpression{tk: "*"}
	0              Literal{kind: Integer}
	     0•*•0     OperationExpression{tk: "*"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 * (0 & 0),                                                                                                                          /*
	0•*•(0•&•0)    OperationExpression{tk: "*"}
	0              Literal{kind: Integer}
	     0•&•0     OperationExpression{tk: "&"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 * (0 << 0),                                                                                                                         /*
	0•*•(0•<<•0)    OperationExpression{tk: "*"}
	0               Literal{kind: Integer}
	     0•<<•0     OperationExpression{tk: "<<"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 * (0 == 0),                                                                                                                         /*
	0•*•(0•==•0)    OperationExpression{tk: "*"}
	0               Literal{kind: Integer}
	     0•==•0     ComparisonExpression{tk: "=="}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 * (0 > 0),                                                                                                                          /*
	0•*•(0•>•0)    OperationExpression{tk: "*"}
	0              Literal{kind: Integer}
	     0•>•0     ComparisonExpression{tk: ">"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 * (0..0),                                                                                                                           /*
	0•*•(0..0)    OperationExpression{tk: "*"}
	0             Literal{kind: Integer}
	     0..0     RangeLiteral{!last}
	     0        Literal{kind: Integer}
	        0     Literal{kind: Integer}                                                                                                  */
	0 & (0 && 0),                                                                                                                         /*
	0•&•(0•&&•0)    OperationExpression{tk: "&"}
	0               Literal{kind: Integer}
	     0•&&•0     AndExpression{tk: "&&"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 & (0 || 0),                                                                                                                         /*
	0•&•(0•||•0)    OperationExpression{tk: "&"}
	0               Literal{kind: Integer}
	     0•||•0     OrExpression{tk: "||"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 & (o = 0),                                                                                                                          /*
	0•&•(o•=•0)    OperationExpression{tk: "&"}
	0              Literal{kind: Integer}
	     o•=•0     ReassignmentExpression{tk: "="}
	         0     Literal{kind: Integer}                                                                                                 */
	0 & (0 + 0),                                                                                                                          /*
	0•&•(0•+•0)    OperationExpression{tk: "&"}
	0              Literal{kind: Integer}
	     0•+•0     OperationExpression{tk: "+"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 & (0 * 0),                                                                                                                          /*
	0•&•(0•*•0)    OperationExpression{tk: "&"}
	0              Literal{kind: Integer}
	     0•*•0     OperationExpression{tk: "*"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 & (0 & 0),                                                                                                                          /*
	0•&•(0•&•0)    OperationExpression{tk: "&"}
	0              Literal{kind: Integer}
	     0•&•0     OperationExpression{tk: "&"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 & (0 << 0),                                                                                                                         /*
	0•&•(0•<<•0)    OperationExpression{tk: "&"}
	0               Literal{kind: Integer}
	     0•<<•0     OperationExpression{tk: "<<"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 & (0 == 0),                                                                                                                         /*
	0•&•(0•==•0)    OperationExpression{tk: "&"}
	0               Literal{kind: Integer}
	     0•==•0     ComparisonExpression{tk: "=="}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 & (0 > 0),                                                                                                                          /*
	0•&•(0•>•0)    OperationExpression{tk: "&"}
	0              Literal{kind: Integer}
	     0•>•0     ComparisonExpression{tk: ">"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 & (0..0),                                                                                                                           /*
	0•&•(0..0)    OperationExpression{tk: "&"}
	0             Literal{kind: Integer}
	     0..0     RangeLiteral{!last}
	     0        Literal{kind: Integer}
	        0     Literal{kind: Integer}                                                                                                  */
	0 << (0 && 0),                                                                                                                        /*
	0•<<•(0•&&•0)    OperationExpression{tk: "<<"}
	0                Literal{kind: Integer}
	      0•&&•0     AndExpression{tk: "&&"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 << (0 || 0),                                                                                                                        /*
	0•<<•(0•||•0)    OperationExpression{tk: "<<"}
	0                Literal{kind: Integer}
	      0•||•0     OrExpression{tk: "||"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 << (o = 0),                                                                                                                         /*
	0•<<•(o•=•0)    OperationExpression{tk: "<<"}
	0               Literal{kind: Integer}
	      o•=•0     ReassignmentExpression{tk: "="}
	          0     Literal{kind: Integer}                                                                                                */
	0 << (0 + 0),                                                                                                                         /*
	0•<<•(0•+•0)    OperationExpression{tk: "<<"}
	0               Literal{kind: Integer}
	      0•+•0     OperationExpression{tk: "+"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 << (0 * 0),                                                                                                                         /*
	0•<<•(0•*•0)    OperationExpression{tk: "<<"}
	0               Literal{kind: Integer}
	      0•*•0     OperationExpression{tk: "*"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 << (0 & 0),                                                                                                                         /*
	0•<<•(0•&•0)    OperationExpression{tk: "<<"}
	0               Literal{kind: Integer}
	      0•&•0     OperationExpression{tk: "&"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 << (0 << 0),                                                                                                                        /*
	0•<<•(0•<<•0)    OperationExpression{tk: "<<"}
	0                Literal{kind: Integer}
	      0•<<•0     OperationExpression{tk: "<<"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 << (0 == 0),                                                                                                                        /*
	0•<<•(0•==•0)    OperationExpression{tk: "<<"}
	0                Literal{kind: Integer}
	      0•==•0     ComparisonExpression{tk: "=="}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 << (0 > 0),                                                                                                                         /*
	0•<<•(0•>•0)    OperationExpression{tk: "<<"}
	0               Literal{kind: Integer}
	      0•>•0     ComparisonExpression{tk: ">"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 << (0..0),                                                                                                                          /*
	0•<<•(0..0)    OperationExpression{tk: "<<"}
	0              Literal{kind: Integer}
	      0..0     RangeLiteral{!last}
	      0        Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 == (0 && 0),                                                                                                                        /*
	0•==•(0•&&•0)    ComparisonExpression{tk: "=="}
	0                Literal{kind: Integer}
	      0•&&•0     AndExpression{tk: "&&"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 == (0 || 0),                                                                                                                        /*
	0•==•(0•||•0)    ComparisonExpression{tk: "=="}
	0                Literal{kind: Integer}
	      0•||•0     OrExpression{tk: "||"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 == (o = 0),                                                                                                                         /*
	0•==•(o•=•0)    ComparisonExpression{tk: "=="}
	0               Literal{kind: Integer}
	      o•=•0     ReassignmentExpression{tk: "="}
	          0     Literal{kind: Integer}                                                                                                */
	0 == (0 + 0),                                                                                                                         /*
	0•==•(0•+•0)    ComparisonExpression{tk: "=="}
	0               Literal{kind: Integer}
	      0•+•0     OperationExpression{tk: "+"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 == (0 * 0),                                                                                                                         /*
	0•==•(0•*•0)    ComparisonExpression{tk: "=="}
	0               Literal{kind: Integer}
	      0•*•0     OperationExpression{tk: "*"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 == (0 & 0),                                                                                                                         /*
	0•==•(0•&•0)    ComparisonExpression{tk: "=="}
	0               Literal{kind: Integer}
	      0•&•0     OperationExpression{tk: "&"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 == (0 << 0),                                                                                                                        /*
	0•==•(0•<<•0)    ComparisonExpression{tk: "=="}
	0                Literal{kind: Integer}
	      0•<<•0     OperationExpression{tk: "<<"}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 == (0 == 0),                                                                                                                        /*
	0•==•(0•==•0)    ComparisonExpression{tk: "=="}
	0                Literal{kind: Integer}
	      0•==•0     ComparisonExpression{tk: "=="}
	      0          Literal{kind: Integer}
	           0     Literal{kind: Integer}                                                                                               */
	0 == (0 > 0),                                                                                                                         /*
	0•==•(0•>•0)    ComparisonExpression{tk: "=="}
	0               Literal{kind: Integer}
	      0•>•0     ComparisonExpression{tk: ">"}
	      0         Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 == (0..0),                                                                                                                          /*
	0•==•(0..0)    ComparisonExpression{tk: "=="}
	0              Literal{kind: Integer}
	      0..0     RangeLiteral{!last}
	      0        Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 > (0 && 0),                                                                                                                         /*
	0•>•(0•&&•0)    ComparisonExpression{tk: ">"}
	0               Literal{kind: Integer}
	     0•&&•0     AndExpression{tk: "&&"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 > (0 || 0),                                                                                                                         /*
	0•>•(0•||•0)    ComparisonExpression{tk: ">"}
	0               Literal{kind: Integer}
	     0•||•0     OrExpression{tk: "||"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 > (o = 0),                                                                                                                          /*
	0•>•(o•=•0)    ComparisonExpression{tk: ">"}
	0              Literal{kind: Integer}
	     o•=•0     ReassignmentExpression{tk: "="}
	         0     Literal{kind: Integer}                                                                                                 */
	0 > (0 + 0),                                                                                                                          /*
	0•>•(0•+•0)    ComparisonExpression{tk: ">"}
	0              Literal{kind: Integer}
	     0•+•0     OperationExpression{tk: "+"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 > (0 * 0),                                                                                                                          /*
	0•>•(0•*•0)    ComparisonExpression{tk: ">"}
	0              Literal{kind: Integer}
	     0•*•0     OperationExpression{tk: "*"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 > (0 & 0),                                                                                                                          /*
	0•>•(0•&•0)    ComparisonExpression{tk: ">"}
	0              Literal{kind: Integer}
	     0•&•0     OperationExpression{tk: "&"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 > (0 << 0),                                                                                                                         /*
	0•>•(0•<<•0)    ComparisonExpression{tk: ">"}
	0               Literal{kind: Integer}
	     0•<<•0     OperationExpression{tk: "<<"}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 > (0 == 0),                                                                                                                         /*
	0•>•(0•==•0)    ComparisonExpression{tk: ">"}
	0               Literal{kind: Integer}
	     0•==•0     ComparisonExpression{tk: "=="}
	     0          Literal{kind: Integer}
	          0     Literal{kind: Integer}                                                                                                */
	0 > (0 > 0),                                                                                                                          /*
	0•>•(0•>•0)    ComparisonExpression{tk: ">"}
	0              Literal{kind: Integer}
	     0•>•0     ComparisonExpression{tk: ">"}
	     0         Literal{kind: Integer}
	         0     Literal{kind: Integer}                                                                                                 */
	0 > (0..0)                                                                                                                            /*
	0•>•(0..0)    ComparisonExpression{tk: ">"}
	0             Literal{kind: Integer}
	     0..0     RangeLiteral{!last}
	     0        Literal{kind: Integer}
	        0     Literal{kind: Integer}                                                                                                  */
];                                                                                                                                        /*
]     </ArrayLiteral>
];    </ExpressionStatement>
];    </Program.ast>
];    </Program>                                                                                                                          */
// Discarded Nodes: 248
// Parsed Nodes: 3046
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 11763 (33% re-reads)
// Unnecessary 'skip_whitespace()' calls: 1062
// source: "../../samples/expressions/precedence.rs"