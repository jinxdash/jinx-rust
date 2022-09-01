fn a() {                                                                                                                                  /*
fn•a()•{↲    <Program>
fn•a()•{↲    <Program.ast{dk: "None"}>
fn•a()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	match x {}                                                                                                                            /*
	match•x•{}    ExpressionStatement{!semi}, MatchExpression
	        {}    MatchExpression.cases{dk: "{}"}                                                                                         */
	match () {}                                                                                                                           /*
	match•()•{}    ExpressionStatement{!semi}, MatchExpression
	      ()       TupleLiteral
	         {}    MatchExpression.cases{dk: "{}"}                                                                                        */
	match (Sd { x: A, y: () }) {}                                                                                                         /*
	match•(Sd•{•x:•A,•y:•()•})•{}    ExpressionStatement{!semi}, MatchExpression
	       Sd•{•x:•A,•y:•()•}        StructLiteral
	          {•x:•A,•y:•()•}        StructLiteral.properties{dk: "{}"}
	            x:•A                 StructLiteralProperty
	                  y:•()          StructLiteralProperty
	                     ()          TupleLiteral
	                           {}    MatchExpression.cases{dk: "{}"}                                                                      */
	match *c {}                                                                                                                           /*
	match•*c•{}    ExpressionStatement{!semi}, MatchExpression
	      *c       DereferenceExpression
	         {}    MatchExpression.cases{dk: "{}"}                                                                                        */
	match ((A, ()), ()) {}                                                                                                                /*
	match•((A,•()),•())•{}    ExpressionStatement{!semi}, MatchExpression
	      ((A,•()),•())       TupleLiteral
	       (A,•())            TupleLiteral
	           ()             TupleLiteral
	                ()        TupleLiteral
	                    {}    MatchExpression.cases{dk: "{}"}                                                                             */
	match [0u8; LARGE_SIZE] {}                                                                                                            /*
	match•[0u8;•LARGE_SIZE]•{}    ExpressionStatement{!semi}, MatchExpression
	      [0u8;•LARGE_SIZE]       SizedArrayLiteral
	       0u8                    Literal{kind: Integer}
	        u8                    Identifier
	                        {}    MatchExpression.cases{dk: "{}"}                                                                         */
	match na.kind {}                                                                                                                      /*
	match•na.kind•{}    ExpressionStatement{!semi}, MatchExpression
	      na.kind       MemberExpression{!computed}
	              {}    MatchExpression.cases{dk: "{}"}                                                                                   */
	match (T::T1(()), V::V2(true)) {}                                                                                                     /*
	match•(T::T1(()),•V::V2(true))•{}    ExpressionStatement{!semi}, MatchExpression
	      (T::T1(()),•V::V2(true))       TupleLiteral
	       T::T1(())                     CallExpression
	       T::T1                         ExpressionPath
	            (())                     CallExpression.arguments{dk: "()"}
	             ()                      TupleLiteral
	                  V::V2(true)        CallExpression
	                  V::V2              ExpressionPath
	                       (true)        CallExpression.arguments{dk: "()"}
	                        true         Literal{kind: True}
	                               {}    MatchExpression.cases{dk: "{}"}                                                                  */
	match (Sd { x: A, y: () }) {}                                                                                                         /*
	match•(Sd•{•x:•A,•y:•()•})•{}    ExpressionStatement{!semi}, MatchExpression
	       Sd•{•x:•A,•y:•()•}        StructLiteral
	          {•x:•A,•y:•()•}        StructLiteral.properties{dk: "{}"}
	            x:•A                 StructLiteralProperty
	                  y:•()          StructLiteralProperty
	                     ()          TupleLiteral
	                           {}    MatchExpression.cases{dk: "{}"}                                                                      */
	match "a" {}                                                                                                                          /*
	match•"a"•{}    ExpressionStatement{!semi}, MatchExpression
	      "a"       Literal{kind: String}
	          {}    MatchExpression.cases{dk: "{}"}                                                                                       */
	match (&"foo", "bar") {}                                                                                                              /*
	match•(&"foo",•"bar")•{}    ExpressionStatement{!semi}, MatchExpression
	      (&"foo",•"bar")       TupleLiteral
	       &"foo"               ReferenceExpression{!mut}
	        "foo"               Literal{kind: String}
	               "bar"        Literal{kind: String}
	                      {}    MatchExpression.cases{dk: "{}"}                                                                           */
	match (Foo{foo: true, bar: Some(10), baz: 20}) {}                                                                                     /*
	match•(Foo{foo:•true,•bar:•Some(10),•baz:•20})•{}    ExpressionStatement{!semi}, MatchExpression
	       Foo{foo:•true,•bar:•Some(10),•baz:•20}        StructLiteral
	          {foo:•true,•bar:•Some(10),•baz:•20}        StructLiteral.properties{dk: "{}"}
	           foo:•true                                 StructLiteralProperty
	                true                                 Literal{kind: True}
	                      bar:•Some(10)                  StructLiteralProperty
	                           Some(10)                  CallExpression
	                               (10)                  CallExpression.arguments{dk: "()"}
	                                10                   Literal{kind: Integer}
	                                     baz:•20         StructLiteralProperty
	                                          20         Literal{kind: Integer}
	                                               {}    MatchExpression.cases{dk: "{}"}                                                  */
	match (l1, l2) {}                                                                                                                     /*
	match•(l1,•l2)•{}    ExpressionStatement{!semi}, MatchExpression
	      (l1,•l2)       TupleLiteral
	               {}    MatchExpression.cases{dk: "{}"}                                                                                  */

	match 0 { 0 if false => () }                                                                                                          /*
	match•0•{•0•if•false•=>•()•}    ExpressionStatement{!semi}, MatchExpression
	      0                         Literal{kind: Integer}
	        {•0•if•false•=>•()•}    MatchExpression.cases{dk: "{}"}
	          0•if•false•=>•()      MatchExpressionCase
	          0                     Literal{kind: Integer}
	               false            Literal{kind: False}
	                        ()      TupleLiteral                                                                                          */
	match true { true => true }                                                                                                           /*
	match•true•{•true•=>•true•}    ExpressionStatement{!semi}, MatchExpression
	      true                     Literal{kind: True}
	           {•true•=>•true•}    MatchExpression.cases{dk: "{}"}
	             true•=>•true      MatchExpressionCase
	             true              Literal{kind: True}
	                     true      Literal{kind: True}                                                                                    */
	
	let v: isize = match &*sl {};                                                                                                         /*
	let•v:•isize•=•match•&*sl•{};    LetVariableDeclaration
	               match•&*sl•{}     MatchExpression
	                     &*sl        ReferenceExpression{!mut}
	                      *sl        DereferenceExpression
	                          {}     MatchExpression.cases{dk: "{}"}                                                                      */
	let a: isize = match 1 {                                                                                                              /*
	let•a:•isize•=•match•1•{•↲    <LetVariableDeclaration>
	               match•1•{•↲    <MatchExpression>
	                     1        Literal{kind: Integer}
	                       {•↲    <MatchExpression.cases{dk: "{}"}>                                                                       */
		x if x < 2 => { 3 }                                                                                                               /*
		x•if•x•<•2•=>•{•3•}    MatchExpressionCase
		     x•<•2             ComparisonExpression{tk: "<"}
		         2             Literal{kind: Integer}
		              {•3•}    BlockExpression
		                3      ExpressionStatement{!semi}, Literal{kind: Integer}                                                         */
		x if x < 4 => { 5 }                                                                                                               /*
		x•if•x•<•4•=>•{•5•}    MatchExpressionCase
		     x•<•4             ComparisonExpression{tk: "<"}
		         4             Literal{kind: Integer}
		              {•5•}    BlockExpression
		                5      ExpressionStatement{!semi}, Literal{kind: Integer}                                                         */
		6 => { 7 }                                                                                                                        /*
		6•=>•{•7•}    MatchExpressionCase
		6             Literal{kind: Integer}
		     {•7•}    BlockExpression
		       7      ExpressionStatement{!semi}, Literal{kind: Integer}                                                                  */
		_ => { 8 }                                                                                                                        /*
		_•=>•{•8•}    MatchExpressionCase
		_             WildcardPattern
		     {•8•}    BlockExpression
		       8      ExpressionStatement{!semi}, Literal{kind: Integer}                                                                  */
	};                                                                                                                                    /*
   ╚}     </MatchExpression.cases>
   ╚}     </MatchExpression>
   ╚};    </LetVariableDeclaration>                                                                                                       */
	let val = match match match match match () { () => () } { () => () } { () => () } { () => () } { () => () };                          /*
	let•val•=•match•match•match•match•match•()•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•};    LetVariableDeclaration
	          match•match•match•match•match•()•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}     MatchExpression
	                match•match•match•match•()•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}                  MatchExpression
	                      match•match•match•()•{•()•=>•()•}•{•()•=>•()•}•{•()•=>•()•}                               MatchExpression
	                            match•match•()•{•()•=>•()•}•{•()•=>•()•}                                            MatchExpression
	                                  match•()•{•()•=>•()•}                                                         MatchExpression
	                                        ()                                                                      TupleLiteral
	                                           {•()•=>•()•}                                                         MatchExpression.cases{dk: "{}"}
	                                             ()•=>•()                                                           MatchExpressionCase
	                                             ()                                                                 TuplePattern
	                                                   ()                                                           TupleLiteral
	                                                        {•()•=>•()•}                                            MatchExpression.cases{dk: "{}"}
	                                                          ()•=>•()                                              MatchExpressionCase
	                                                          ()                                                    TuplePattern
	                                                                ()                                              TupleLiteral
	                                                                     {•()•=>•()•}                               MatchExpression.cases{dk: "{}"}
	                                                                       ()•=>•()                                 MatchExpressionCase
	                                                                       ()                                       TuplePattern
	                                                                             ()                                 TupleLiteral
	                                                                                  {•()•=>•()•}                  MatchExpression.cases{dk: "{}"}
	                                                                                    ()•=>•()                    MatchExpressionCase
	                                                                                    ()                          TuplePattern
	                                                                                          ()                    TupleLiteral
	                                                                                               {•()•=>•()•}     MatchExpression.cases{dk: "{}"}
	                                                                                                 ()•=>•()       MatchExpressionCase
	                                                                                                 ()             TuplePattern
	                                                                                                       ()       TupleLiteral          */
	let b: isize =                                                                                                                        /*
	let•b:•isize•=↲    <LetVariableDeclaration>                                                                                           */
		match (A {x: 10, y: 20}) {                                                                                                        /*
		match•(A•{x:•10,•y:•20})•{↲    <MatchExpression>
		       A•{x:•10,•y:•20}        StructLiteral
		         {x:•10,•y:•20}        StructLiteral.properties{dk: "{}"}
		          x:•10                StructLiteralProperty
		             10                Literal{kind: Integer}
		                 y:•20         StructLiteralProperty
		                    20         Literal{kind: Integer}
		                         {↲    <MatchExpression.cases{dk: "{}"}>                                                                  */
			x if x.x < 5 && x.y < 5 => { 1 }                                                                                              /*
			x•if•x.x•<•5•&&•x.y•<•5•=>•{•1•}    MatchExpressionCase
			     x.x•<•5•&&•x.y•<•5             AndExpression{tk: "&&"}
			     x.x•<•5                        ComparisonExpression{tk: "<"}
			     x.x                            MemberExpression{!computed}
			           5                        Literal{kind: Integer}
			                x.y•<•5             ComparisonExpression{tk: "<"}
			                x.y                 MemberExpression{!computed}
			                      5             Literal{kind: Integer}
			                           {•1•}    BlockExpression
			                             1      ExpressionStatement{!semi}, Literal{kind: Integer}                                        */
			A {..} if x == 10 && y == 20 => { 2 }                                                                                         /*
			A•{..}•if•x•==•10•&&•y•==•20•=>•{•2•}    MatchExpressionCase
			A•{..}                                   StructPattern
			  {..}                                   StructPattern.properties{dk: "{}"}
			   ..                                    RestPattern
			          x•==•10•&&•y•==•20             AndExpression{tk: "&&"}
			          x•==•10                        ComparisonExpression{tk: "=="}
			               10                        Literal{kind: Integer}
			                     y•==•20             ComparisonExpression{tk: "=="}
			                          20             Literal{kind: Integer}
			                                {•2•}    BlockExpression
			                                  2      ExpressionStatement{!semi}, Literal{kind: Integer}                                   */
			A {..} => { 3 }                                                                                                               /*
			A•{..}•=>•{•3•}    MatchExpressionCase
			A•{..}             StructPattern
			  {..}             StructPattern.properties{dk: "{}"}
			   ..              RestPattern
			          {•3•}    BlockExpression
			            3      ExpressionStatement{!semi}, Literal{kind: Integer}                                                         */
		};                                                                                                                                /*
      ╚╚}     </MatchExpression.cases>
      ╚╚}     </MatchExpression>
      ╚╚};    </LetVariableDeclaration>                                                                                                   */

	match true {                                                                                                                          /*
	match•true•{↲    <ExpressionStatement{!semi}>
	match•true•{↲    <MatchExpression>
	      true       Literal{kind: True}
	           {↲    <MatchExpression.cases{dk: "{}"}>                                                                                    */
        true if true => (),                                                                                                               /*
        true•if•true•=>•()    MatchExpressionCase
        true                  Literal{kind: True}
                true          Literal{kind: True}
                        ()    TupleLiteral                                                                                                */
        false if false => unsafe { },                                                                                                     /*
        false•if•false•=>•unsafe•{•}    MatchExpressionCase
        false                           Literal{kind: False}
                 false                  Literal{kind: False}
                          unsafe•{•}    BlockExpression{unsafe}
                                 {•}    BlockExpression.body{dk: "{}"}                                                                    */
        true => { }                                                                                                                       /*
        true•=>•{•}    MatchExpressionCase
        true           Literal{kind: True}
                {•}    BlockExpression                                                                                                    */
        false => (),                                                                                                                      /*
        false•=>•()    MatchExpressionCase
        false          Literal{kind: False}
                 ()    TupleLiteral                                                                                                       */
		&[] => 0,                                                                                                                         /*
		&[]•=>•0    MatchExpressionCase
		&[]         ReferencePattern{!mut}
		 []         ArrayPattern
		       0    Literal{kind: Integer}                                                                                                */
        &[a,b,c] => 3,                                                                                                                    /*
        &[a,b,c]•=>•3    MatchExpressionCase
        &[a,b,c]         ReferencePattern{!mut}
         [a,b,c]         ArrayPattern
                    3    Literal{kind: Integer}                                                                                           */
        &[a, ref d @ ..] => a,                                                                                                            /*
        &[a,•ref•d•@•..]•=>•a    MatchExpressionCase
        &[a,•ref•d•@•..]         ReferencePattern{!mut}
         [a,•ref•d•@•..]         ArrayPattern
             ref•d•@•..          PatternVariableDeclaration{ref, !mut}
                     ..          RestPattern                                                                                              */
        &[10,a, ref d @ ..] => 10,                                                                                                        /*
        &[10,a,•ref•d•@•..]•=>•10    MatchExpressionCase
        &[10,a,•ref•d•@•..]          ReferencePattern{!mut}
         [10,a,•ref•d•@•..]          ArrayPattern
          10                         Literal{kind: Integer}
                ref•d•@•..           PatternVariableDeclaration{ref, !mut}
                        ..           RestPattern
                               10    Literal{kind: Integer}                                                                               */
        [h, ..] if h > n => 0,                                                                                                            /*
        [h,•..]•if•h•>•n•=>•0    MatchExpressionCase
        [h,•..]                  ArrayPattern
            ..                   RestPattern
                   h•>•n         ComparisonExpression{tk: ">"}
                            0    Literal{kind: Integer}                                                                                   */
        [h, ..] if h == n => 1,                                                                                                           /*
        [h,•..]•if•h•==•n•=>•1    MatchExpressionCase
        [h,•..]                   ArrayPattern
            ..                    RestPattern
                   h•==•n         ComparisonExpression{tk: "=="}
                             1    Literal{kind: Integer}                                                                                  */
        [h, ref ts] => foo(c, n - h) + foo(ts, n),                                                                                        /*
        [h,•ref•ts]•=>•foo(c,•n•-•h)•+•foo(ts,•n)    MatchExpressionCase
        [h,•ref•ts]                                  ArrayPattern
            ref•ts                                   PatternVariableDeclaration{ref, !mut}
                       foo(c,•n•-•h)•+•foo(ts,•n)    OperationExpression{tk: "+"}
                       foo(c,•n•-•h)                 CallExpression
                          (c,•n•-•h)                 CallExpression.arguments{dk: "()"}
                              n•-•h                  OperationExpression{tk: "-"}
                                       foo(ts,•n)    CallExpression
                                          (ts,•n)    CallExpression.arguments{dk: "()"}                                                   */
        [] => 0,                                                                                                                          /*
        []•=>•0    MatchExpressionCase
        []         ArrayPattern
              0    Literal{kind: Integer}                                                                                                 */
		&A::C(v, box ref a) => tail(e),                                                                                                   /*
		&A::C(v,•box•ref•a)•=>•tail(e)    MatchExpressionCase
		&A::C(v,•box•ref•a)               ReferencePattern{!mut}
		 A::C(v,•box•ref•a)               TuplePattern
		 A::C                             ExpressionPath
		     (v,•box•ref•a)               TuplePattern.items{dk: "()"}
		         box•ref•a                BoxPattern
		             ref•a                PatternVariableDeclaration{ref, !mut}
		                       tail(e)    CallExpression
		                           (e)    CallExpression.arguments{dk: "()"}                                                              */
        &A::C(x, box A::S)  => A::C(c, box A::R),                                                                                         /*
        &A::C(x,•box•A::S)••=>•A::C(c,•box•A::R)    MatchExpressionCase
        &A::C(x,•box•A::S)                          ReferencePattern{!mut}
         A::C(x,•box•A::S)                          TuplePattern
         A::C                                       ExpressionPath
             (x,•box•A::S)                          TuplePattern.items{dk: "()"}
                 box•A::S                           BoxPattern
                     A::S                           ExpressionPath
                               A::C(c,•box•A::R)    CallExpression
                               A::C                 ExpressionPath
                                   (c,•box•A::R)    CallExpression.arguments{dk: "()"}
                                       box•A::R     BoxExpression
                                           A::R     ExpressionPath                                                                        */
		0 => return e(j::h::r(a::e::d, "")),                                                                                              /*
		0•=>•return•e(j::h::r(a::e::d,•""))    MatchExpressionCase
		0                                      Literal{kind: Integer}
		     return•e(j::h::r(a::e::d,•""))    ReturnExpression
		            e(j::h::r(a::e::d,•""))    CallExpression
		             (j::h::r(a::e::d,•""))    CallExpression.arguments{dk: "()"}
		              j::h::r(a::e::d,•"")     CallExpression
		              j::h::r                  ExpressionPath
		              j::h                     ExpressionPath
		                     (a::e::d,•"")     CallExpression.arguments{dk: "()"}
		                      a::e::d          ExpressionPath
		                      a::e             ExpressionPath
		                               ""      Literal{kind: String}                                                                      */
		n => r = &mut a::d(&mut e, &mut [])[n..],                                                                                         /*
		n•=>•r•=•&mut•a::d(&mut•e,•&mut•[])[n..]    MatchExpressionCase
		     r•=•&mut•a::d(&mut•e,•&mut•[])[n..]    ReassignmentExpression{tk: "="}
		         &mut•a::d(&mut•e,•&mut•[])[n..]    ReferenceExpression{mut}
		              a::d(&mut•e,•&mut•[])[n..]    MemberExpression{computed}
		              a::d(&mut•e,•&mut•[])         CallExpression
		              a::d                          ExpressionPath
		                  (&mut•e,•&mut•[])         CallExpression.arguments{dk: "()"}
		                   &mut•e                   ReferenceExpression{mut}
		                           &mut•[]          ReferenceExpression{mut}
		                                []          ArrayLiteral
		                                    n..     RangeLiteral{!last}                                                                   */
        box Q::V(ed) =>                                                                                                                   /*
        box•Q::V(ed)•=>•↲    <MatchExpressionCase>
        box•Q::V(ed)         BoxPattern
            Q::V(ed)         TuplePattern
            Q::V             ExpressionPath
                (ed)         TuplePattern.items{dk: "()"}                                                                                 */
			match ed.q {                                                                                                                  /*
			match•ed.q•{↲    <MatchExpression>
			      ed.q       MemberExpression{!computed}
			           {↲    <MatchExpression.cases{dk: "{}"}>                                                                            */
           		box R::E(ref d) if d.d.r() => { true }                                                                                    /*
           		box•R::E(ref•d)•if•d.d.r()•=>•{•true•}    MatchExpressionCase
           		box•R::E(ref•d)                           BoxPattern
           		    R::E(ref•d)                           TuplePattern
           		    R::E                                  ExpressionPath
           		        (ref•d)                           TuplePattern.items{dk: "()"}
           		         ref•d                            PatternVariableDeclaration{ref, !mut}
           		                   d.d.r()                CallExpression
           		                   d.d                    MemberExpression{!computed}
           		                        ()                CallExpression.arguments{dk: "()"}
           		                              {•true•}    BlockExpression
           		                                true      ExpressionStatement{!semi}, Literal{kind: True}                                 */
        	},                                                                                                                            /*
••••••••╚}    </MatchExpression.cases>
••••••••╚}    </MatchExpression>
••••••••╚}    </MatchExpressionCase>                                                                                                      */
		_ => panic!(),                                                                                                                    /*
		_•=>•panic!()    MatchExpressionCase
		_                WildcardPattern
		     panic!()    MacroInvocation
		           ()    MacroInvocation.segments{dk: "()"}                                                                               */
		ref _x => unreachable!(),                                                                                                         /*
		ref•_x•=>•unreachable!()    MatchExpressionCase
		ref•_x                      PatternVariableDeclaration{ref, !mut}
		          unreachable!()    MacroInvocation
		                      ()    MacroInvocation.segments{dk: "()"}                                                                    */
        0 => return,                                                                                                                      /*
        0•=>•return    MatchExpressionCase
        0              Literal{kind: Integer}
             return    ReturnExpression                                                                                                   */
		A { a: v } if *v.clone() == 42 => v,                                                                                              /*
		A•{•a:•v•}•if•*v.clone()•==•42•=>•v    MatchExpressionCase
		A•{•a:•v•}                             StructPattern
		  {•a:•v•}                             StructPattern.properties{dk: "{}"}
		    a:•v                               StructPatternPropertyDestructured
		              *v.clone()•==•42         ComparisonExpression{tk: "=="}
		              *v.clone()               DereferenceExpression
		               v.clone()               CallExpression
		                      ()               CallExpression.arguments{dk: "()"}
		                            42         Literal{kind: Integer}                                                                     */
		A((a,)) => *a = 0,                                                                                                                /*
		A((a,))•=>•*a•=•0    MatchExpressionCase
		A((a,))              TuplePattern
		 ((a,))              TuplePattern.items{dk: "()"}
		  (a,)               TuplePattern
		           *a•=•0    ReassignmentExpression{tk: "="}
		           *a        DereferenceExpression
		                0    Literal{kind: Integer}                                                                                       */
		Some(x) if let Some(y) = x => (x, y),                                                                                             /*
		Some(x)•if•let•Some(y)•=•x•=>•(x,•y)    MatchExpressionCase
		Some(x)                                 TuplePattern
		    (x)                                 TuplePattern.items{dk: "()"}
		           let•Some(y)•=•x              LetScrutinee
		               Some(y)                  TuplePattern
		                   (y)                  TuplePattern.items{dk: "()"}
		                              (x,•y)    TupleLiteral                                                                              */
		Some((x, _)) if let Foo::Bar = bar(x) => panic!(),                                                                                /*
		Some((x,•_))•if•let•Foo::Bar•=•bar(x)•=>•panic!()    MatchExpressionCase
		Some((x,•_))                                         TuplePattern
		    ((x,•_))                                         TuplePattern.items{dk: "()"}
		     (x,•_)                                          TuplePattern
		         _                                           WildcardPattern
		                let•Foo::Bar•=•bar(x)                LetScrutinee
		                    Foo::Bar                         ExpressionPath
		                               bar(x)                CallExpression
		                                  (x)                CallExpression.arguments{dk: "()"}
		                                         panic!()    MacroInvocation
		                                               ()    MacroInvocation.segments{dk: "()"}                                           */
        Some((_, x)) if let Foo::Baz = baz(x) => {},                                                                                      /*
        Some((_,•x))•if•let•Foo::Baz•=•baz(x)•=>•{}    MatchExpressionCase
        Some((_,•x))                                   TuplePattern
            ((_,•x))                                   TuplePattern.items{dk: "()"}
             (_,•x)                                    TuplePattern
              _                                        WildcardPattern
                        let•Foo::Baz•=•baz(x)          LetScrutinee
                            Foo::Baz                   ExpressionPath
                                       baz(x)          CallExpression
                                          (x)          CallExpression.arguments{dk: "()"}
                                                 {}    BlockExpression                                                                    */
        Some(x) if let Foo::Qux(y) = qux(x) => assert_eq!(y, 84),                                                                         /*
        Some(x)•if•let•Foo::Qux(y)•=•qux(x)•=>•assert_eq!(y,•84)    MatchExpressionCase
        Some(x)                                                     TuplePattern
            (x)                                                     TuplePattern.items{dk: "()"}
                   let•Foo::Qux(y)•=•qux(x)                         LetScrutinee
                       Foo::Qux(y)                                  TuplePattern
                       Foo::Qux                                     ExpressionPath
                               (y)                                  TuplePattern.items{dk: "()"}
                                     qux(x)                         CallExpression
                                        (x)                         CallExpression.arguments{dk: "()"}
                                               assert_eq!(y,•84)    MacroInvocation
                                                         (y,•84)    MacroInvocation.segments{dk: "()"}
                                                           ,        PunctuationToken{tk: ","}
                                                             84     Literal{kind: Integer}                                                */
        Ok(mut r) | Err(mut r) if true => r = 1,                                                                                          /*
        Ok(mut•r)•|•Err(mut•r)•if•true•=>•r•=•1    MatchExpressionCase
        Ok(mut•r)•|•Err(mut•r)                     UnionPattern
        Ok(mut•r)                                  TuplePattern
          (mut•r)                                  TuplePattern.items{dk: "()"}
           mut•r                                   PatternVariableDeclaration{!ref, mut}
                    Err(mut•r)                     TuplePattern
                       (mut•r)                     TuplePattern.items{dk: "()"}
                        mut•r                      PatternVariableDeclaration{!ref, mut}
                                  true             Literal{kind: True}
                                          r•=•1    ReassignmentExpression{tk: "="}
                                              1    Literal{kind: Integer}                                                                 */
        Color::Rgb(r, g, b) => r | g == 0 || r | b == 0 || g | b == 0,                                                                    /*
        Color::Rgb(r,•g,•b)•=>•r•|•g•==•0•||•r•|•b•==•0•||•g•|•b•==•0    MatchExpressionCase
        Color::Rgb(r,•g,•b)                                              TuplePattern
        Color::Rgb                                                       ExpressionPath
                  (r,•g,•b)                                              TuplePattern.items{dk: "()"}
                               r•|•g•==•0•||•r•|•b•==•0•||•g•|•b•==•0    OrExpression{tk: "||"}
                               r•|•g•==•0•||•r•|•b•==•0                  OrExpression{tk: "||"}
                               r•|•g•==•0                                ComparisonExpression{tk: "=="}
                               r•|•g                                     OperationExpression{tk: "|"}
                                        0                                Literal{kind: Integer}
                                             r•|•b•==•0                  ComparisonExpression{tk: "=="}
                                             r•|•b                       OperationExpression{tk: "|"}
                                                      0                  Literal{kind: Integer}
                                                           g•|•b•==•0    ComparisonExpression{tk: "=="}
                                                           g•|•b         OperationExpression{tk: "|"}
                                                                    0    Literal{kind: Integer}                                           */
        not_red @ Color::Green | not_red @ Color::Blue | not_red @ Color::Rgb(..) | not_red @ Color::Cyan => format!("{:?}", not_red),    /*
        not_red•@•Color::Green•|•not_red•@•Color::Blue•|•not_red•@•Color::Rgb(..)•|•not_red•@•Color::Cyan•=>•format!("{:?}",•not_red)    MatchExpressionCase
        not_red•@•Color::Green•|•not_red•@•Color::Blue•|•not_red•@•Color::Rgb(..)•|•not_red•@•Color::Cyan                                UnionPattern
        not_red•@•Color::Green                                                                                                           PatternVariableDeclaration{!ref, !mut}
                  Color::Green                                                                                                           ExpressionPath
                                 not_red•@•Color::Blue                                                                                   PatternVariableDeclaration{!ref, !mut}
                                           Color::Blue                                                                                   ExpressionPath
                                                         not_red•@•Color::Rgb(..)                                                        PatternVariableDeclaration{!ref, !mut}
                                                                   Color::Rgb(..)                                                        TuplePattern
                                                                   Color::Rgb                                                            ExpressionPath
                                                                             (..)                                                        TuplePattern.items{dk: "()"}
                                                                              ..                                                         RestPattern
                                                                                    not_red•@•Color::Cyan                                PatternVariableDeclaration{!ref, !mut}
                                                                                              Color::Cyan                                ExpressionPath
                                                                                                             format!("{:?}",•not_red)    MacroInvocation
                                                                                                                    ("{:?}",•not_red)    MacroInvocation.segments{dk: "()"}
                                                                                                                     "{:?}"              Literal{kind: String}
                                                                                                                           ,             PunctuationToken{tk: ","}*/
        Ok(x) if let Err(_) = x => {},                                                                                                    /*
        Ok(x)•if•let•Err(_)•=•x•=>•{}    MatchExpressionCase
        Ok(x)                            TuplePattern
          (x)                            TuplePattern.items{dk: "()"}
                 let•Err(_)•=•x          LetScrutinee
                     Err(_)              TuplePattern
                        (_)              TuplePattern.items{dk: "()"}
                         _               WildcardPattern
                                   {}    BlockExpression                                                                                  */
		// _ if let _ = !Foo{ a: 1 } => {},
		//•_•if•let•_•=•!Foo{•a:•1•}•=>•{},    Comment{line}
		_ if !Foo{ a: 1 } => {}                                                                                                           /*
		_•if•!Foo{•a:•1•}•=>•{}    MatchExpressionCase
		_                          WildcardPattern
		     !Foo{•a:•1•}          NotExpression
		      Foo{•a:•1•}          StructLiteral
		         {•a:•1•}          StructLiteral.properties{dk: "{}"}
		           a:•1            StructLiteralProperty
		              1            Literal{kind: Integer}
		                     {}    BlockExpression                                                                                        */
		E { x: A, y: _ } => {}                                                                                                            /*
		E•{•x:•A,•y:•_•}•=>•{}    MatchExpressionCase
		E•{•x:•A,•y:•_•}          StructPattern
		  {•x:•A,•y:•_•}          StructPattern.properties{dk: "{}"}
		    x:•A                  StructPatternPropertyDestructured
		          y:•_            StructPatternPropertyDestructured
		             _            WildcardPattern
		                    {}    BlockExpression                                                                                         */
        D { a: _a } | C { a: _a } if true => {}                                                                                           /*
        D•{•a:•_a•}•|•C•{•a:•_a•}•if•true•=>•{}    MatchExpressionCase
        D•{•a:•_a•}•|•C•{•a:•_a•}                  UnionPattern
        D•{•a:•_a•}                                StructPattern
          {•a:•_a•}                                StructPattern.properties{dk: "{}"}
            a:•_a                                  StructPatternPropertyDestructured
                      C•{•a:•_a•}                  StructPattern
                        {•a:•_a•}                  StructPattern.properties{dk: "{}"}
                          a:•_a                    StructPatternPropertyDestructured
                                     true          Literal{kind: True}
                                             {}    BlockExpression                                                                        */
		Some(a::B { misc: false, .. }) => {}                                                                                              /*
		Some(a::B•{•misc:•false,•..•})•=>•{}    MatchExpressionCase
		Some(a::B•{•misc:•false,•..•})          TuplePattern
		    (a::B•{•misc:•false,•..•})          TuplePattern.items{dk: "()"}
		     a::B•{•misc:•false,•..•}           StructPattern
		     a::B                               ExpressionPath
		          {•misc:•false,•..•}           StructPattern.properties{dk: "{}"}
		            misc:•false                 StructPatternPropertyDestructured
		                  false                 Literal{kind: False}
		                         ..             RestPattern
		                                  {}    BlockExpression                                                                           */
		ref _x if false => {}                                                                                                             /*
		ref•_x•if•false•=>•{}    MatchExpressionCase
		ref•_x                   PatternVariableDeclaration{ref, !mut}
		          false          Literal{kind: False}
		                   {}    BlockExpression                                                                                          */
        "b" => {}                                                                                                                         /*
        "b"•=>•{}    MatchExpressionCase
        "b"          Literal{kind: String}
               {}    BlockExpression                                                                                                      */
        "b" => {}                                                                                                                         /*
        "b"•=>•{}    MatchExpressionCase
        "b"          Literal{kind: String}
               {}    BlockExpression                                                                                                      */
        _ => {},                                                                                                                          /*
        _•=>•{}    MatchExpressionCase
        _          WildcardPattern
             {}    BlockExpression                                                                                                        */
		() if f == Foo { x: 42 } => {}                                                                                                    /*
		()•if•f•==•Foo•{•x:•42•}•=>•{}    MatchExpressionCase
		()                                TuplePattern
		      f•==•Foo•{•x:•42•}          ComparisonExpression{tk: "=="}
		           Foo•{•x:•42•}          StructLiteral
		               {•x:•42•}          StructLiteral.properties{dk: "{}"}
		                 x:•42            StructLiteralProperty
		                    42            Literal{kind: Integer}
		                            {}    BlockExpression                                                                                 */
        _ => {}                                                                                                                           /*
        _•=>•{}    MatchExpressionCase
        _          WildcardPattern
             {}    BlockExpression                                                                                                        */
        0 => {}                                                                                                                           /*
        0•=>•{}    MatchExpressionCase
        0          Literal{kind: Integer}
             {}    BlockExpression                                                                                                        */
        a => {}                                                                                                                           /*
        a•=>•{}    MatchExpressionCase
             {}    BlockExpression                                                                                                        */
        a::X => {}                                                                                                                        /*
        a::X•=>•{}    MatchExpressionCase
        a::X          ExpressionPath
                {}    BlockExpression                                                                                                     */
        _ => {}                                                                                                                           /*
        _•=>•{}    MatchExpressionCase
        _          WildcardPattern
             {}    BlockExpression                                                                                                        */
        (a, ..,) => {}                                                                                                                    /*
        (a,•..,)•=>•{}    MatchExpressionCase
        (a,•..,)          TuplePattern
            ..            RestPattern
                    {}    BlockExpression                                                                                                 */
		0 .. 128 => {}                                                                                                                    /*
		0•..•128•=>•{}    MatchExpressionCase
		0•..•128          RangePattern{!last}
		0                 Literal{kind: Integer}
		     128          Literal{kind: Integer}
		            {}    BlockExpression                                                                                                 */
        128 ..= 255 => {}                                                                                                                 /*
        128•..=•255•=>•{}    MatchExpressionCase
        128•..=•255          RangePattern{last}
        128                  Literal{kind: Integer}
                255          Literal{kind: Integer}
                       {}    BlockExpression                                                                                              */
		128 ..= 255 if 1 => {}                                                                                                            /*
		128•..=•255•if•1•=>•{}    MatchExpressionCase
		128•..=•255               RangePattern{last}
		128                       Literal{kind: Integer}
		        255               Literal{kind: Integer}
		               1          Literal{kind: Integer}
		                    {}    BlockExpression                                                                                         */
		(Some(_), None) | (None, Some(_)) => {}                                                                                           /*
		(Some(_),•None)•|•(None,•Some(_))•=>•{}    MatchExpressionCase
		(Some(_),•None)•|•(None,•Some(_))          UnionPattern
		(Some(_),•None)                            TuplePattern
		 Some(_)                                   TuplePattern
		     (_)                                   TuplePattern.items{dk: "()"}
		      _                                    WildcardPattern
		                  (None,•Some(_))          TuplePattern
		                         Some(_)           TuplePattern
		                             (_)           TuplePattern.items{dk: "()"}
		                              _            WildcardPattern
		                                     {}    BlockExpression                                                                        */
		S::<{a()}> => {}                                                                                                                  /*
		S::<{a()}>•=>•{}    MatchExpressionCase
		S::<{a()}>          ExpressionTypeCast
		   <{a()}>          ExpressionTypeCast.typeArguments{dk: "<>"}
		    {a()}           BlockExpression
		     a()            ExpressionStatement{!semi}, CallExpression
		      ()            CallExpression.arguments{dk: "()"}
		              {}    BlockExpression                                                                                               */
		((A, _), _) => {}                                                                                                                 /*
		((A,•_),•_)•=>•{}    MatchExpressionCase
		((A,•_),•_)          TuplePattern
		 (A,•_)              TuplePattern
		     _               WildcardPattern
		         _           WildcardPattern
		               {}    BlockExpression                                                                                              */
		[..] => {}                                                                                                                        /*
		[..]•=>•{}    MatchExpressionCase
		[..]          ArrayPattern
		 ..           RestPattern
		        {}    BlockExpression                                                                                                     */
        &[] => {}                                                                                                                         /*
        &[]•=>•{}    MatchExpressionCase
        &[]          ReferencePattern{!mut}
         []          ArrayPattern
               {}    BlockExpression                                                                                                      */
        &[1..=255] => {}                                                                                                                  /*
        &[1..=255]•=>•{}    MatchExpressionCase
        &[1..=255]          ReferencePattern{!mut}
         [1..=255]          ArrayPattern
          1..=255           RangePattern{last}
          1                 Literal{kind: Integer}
              255           Literal{kind: Integer}
                      {}    BlockExpression                                                                                               */
        C0 => {}                                                                                                                          /*
        C0•=>•{}    MatchExpressionCase
              {}    BlockExpression                                                                                                       */
		T::A {} => {}                                                                                                                     /*
		T::A•{}•=>•{}    MatchExpressionCase
		T::A•{}          StructPattern
		T::A             ExpressionPath
		     {}          StructPattern.properties{dk: "{}"}
		           {}    BlockExpression                                                                                                  */
        &[_, _, ..] => {}                                                                                                                 /*
        &[_,•_,•..]•=>•{}    MatchExpressionCase
        &[_,•_,•..]          ReferencePattern{!mut}
         [_,•_,•..]          ArrayPattern
          _                  WildcardPattern
             _               WildcardPattern
                ..           RestPattern
                       {}    BlockExpression                                                                                              */
		[Some(..), None, ref tail @ ..] => {}                                                                                             /*
		[Some(..),•None,•ref•tail•@•..]•=>•{}    MatchExpressionCase
		[Some(..),•None,•ref•tail•@•..]          ArrayPattern
		 Some(..)                                TuplePattern
		     (..)                                TuplePattern.items{dk: "()"}
		      ..                                 RestPattern
		                 ref•tail•@•..           PatternVariableDeclaration{ref, !mut}
		                            ..           RestPattern
		                                   {}    BlockExpression                                                                          */
		[Some(..), Some(..), ref tail @ ..] => {}                                                                                         /*
		[Some(..),•Some(..),•ref•tail•@•..]•=>•{}    MatchExpressionCase
		[Some(..),•Some(..),•ref•tail•@•..]          ArrayPattern
		 Some(..)                                    TuplePattern
		     (..)                                    TuplePattern.items{dk: "()"}
		      ..                                     RestPattern
		           Some(..)                          TuplePattern
		               (..)                          TuplePattern.items{dk: "()"}
		                ..                           RestPattern
		                     ref•tail•@•..           PatternVariableDeclaration{ref, !mut}
		                                ..           RestPattern
		                                       {}    BlockExpression                                                                      */
		[None, None, ref tail @ ..] => {}                                                                                                 /*
		[None,•None,•ref•tail•@•..]•=>•{}    MatchExpressionCase
		[None,•None,•ref•tail•@•..]          ArrayPattern
		             ref•tail•@•..           PatternVariableDeclaration{ref, !mut}
		                        ..           RestPattern
		                               {}    BlockExpression                                                                              */
		[None, Some(..), ref tail @ ..] => {}                                                                                             /*
		[None,•Some(..),•ref•tail•@•..]•=>•{}    MatchExpressionCase
		[None,•Some(..),•ref•tail•@•..]          ArrayPattern
		       Some(..)                          TuplePattern
		           (..)                          TuplePattern.items{dk: "()"}
		            ..                           RestPattern
		                 ref•tail•@•..           PatternVariableDeclaration{ref, !mut}
		                            ..           RestPattern
		                                   {}    BlockExpression                                                                          */
		[_, _, ref tail @ .., _] => {}                                                                                                    /*
		[_,•_,•ref•tail•@•..,•_]•=>•{}    MatchExpressionCase
		[_,•_,•ref•tail•@•..,•_]          ArrayPattern
		 _                                WildcardPattern
		    _                             WildcardPattern
		       ref•tail•@•..              PatternVariableDeclaration{ref, !mut}
		                  ..              RestPattern
		                      _           WildcardPattern
		                            {}    BlockExpression                                                                                 */
		(&"foo", &_) => {}                                                                                                                /*
		(&"foo",•&_)•=>•{}    MatchExpressionCase
		(&"foo",•&_)          TuplePattern
		 &"foo"               ReferencePattern{!mut}
		  "foo"               Literal{kind: String}
		         &_           ReferencePattern{!mut}
		          _           WildcardPattern
		                {}    BlockExpression                                                                                             */
		(&&_, &_) => {}                                                                                                                   /*
		(&&_,•&_)•=>•{}    MatchExpressionCase
		(&&_,•&_)          TuplePattern
		 &&_               ReferencePattern{!mut}
		  &_               ReferencePattern{!mut}
		   _               WildcardPattern
		      &_           ReferencePattern{!mut}
		       _           WildcardPattern
		             {}    BlockExpression                                                                                                */
		Foo{foo: true, bar: Some(_), ..} => {}                                                                                            /*
		Foo{foo:•true,•bar:•Some(_),•..}•=>•{}    MatchExpressionCase
		Foo{foo:•true,•bar:•Some(_),•..}          StructPattern
		   {foo:•true,•bar:•Some(_),•..}          StructPattern.properties{dk: "{}"}
		    foo:•true                             StructPatternPropertyDestructured
		         true                             Literal{kind: True}
		               bar:•Some(_)               StructPatternPropertyDestructured
		                    Some(_)               TuplePattern
		                        (_)               TuplePattern.items{dk: "()"}
		                         _                WildcardPattern
		                             ..           RestPattern
		                                    {}    BlockExpression                                                                         */
		Foo{foo: false, bar: None, ..} => {}                                                                                              /*
		Foo{foo:•false,•bar:•None,•..}•=>•{}    MatchExpressionCase
		Foo{foo:•false,•bar:•None,•..}          StructPattern
		   {foo:•false,•bar:•None,•..}          StructPattern.properties{dk: "{}"}
		    foo:•false                          StructPatternPropertyDestructured
		         false                          Literal{kind: False}
		                bar:•None               StructPatternPropertyDestructured
		                           ..           RestPattern
		                                  {}    BlockExpression                                                                           */
		Foo{foo: true, bar: None, ..} => {}                                                                                               /*
		Foo{foo:•true,•bar:•None,•..}•=>•{}    MatchExpressionCase
		Foo{foo:•true,•bar:•None,•..}          StructPattern
		   {foo:•true,•bar:•None,•..}          StructPattern.properties{dk: "{}"}
		    foo:•true                          StructPatternPropertyDestructured
		         true                          Literal{kind: True}
		               bar:•None               StructPatternPropertyDestructured
		                          ..           RestPattern
		                                 {}    BlockExpression                                                                            */
		Foo{foo: false, bar: Some(_), ..} => {}                                                                                           /*
		Foo{foo:•false,•bar:•Some(_),•..}•=>•{}    MatchExpressionCase
		Foo{foo:•false,•bar:•Some(_),•..}          StructPattern
		   {foo:•false,•bar:•Some(_),•..}          StructPattern.properties{dk: "{}"}
		    foo:•false                             StructPatternPropertyDestructured
		         false                             Literal{kind: False}
		                bar:•Some(_)               StructPatternPropertyDestructured
		                     Some(_)               TuplePattern
		                         (_)               TuplePattern.items{dk: "()"}
		                          _                WildcardPattern
		                              ..           RestPattern
		                                     {}    BlockExpression                                                                        */
		(Some(&[]), Ok(&[])) => {}                                                                                                        /*
		(Some(&[]),•Ok(&[]))•=>•{}    MatchExpressionCase
		(Some(&[]),•Ok(&[]))          TuplePattern
		 Some(&[])                    TuplePattern
		     (&[])                    TuplePattern.items{dk: "()"}
		      &[]                     ReferencePattern{!mut}
		       []                     ArrayPattern
		            Ok(&[])           TuplePattern
		              (&[])           TuplePattern.items{dk: "()"}
		               &[]            ReferencePattern{!mut}
		                []            ArrayPattern
		                        {}    BlockExpression                                                                                     */
		(Some(&[_, ..]), Ok(_)) | (Some(&[_, ..]), Err(())) => {},                                                                        /*
		(Some(&[_,•..]),•Ok(_))•|•(Some(&[_,•..]),•Err(()))•=>•{}    MatchExpressionCase
		(Some(&[_,•..]),•Ok(_))•|•(Some(&[_,•..]),•Err(()))          UnionPattern
		(Some(&[_,•..]),•Ok(_))                                      TuplePattern
		 Some(&[_,•..])                                              TuplePattern
		     (&[_,•..])                                              TuplePattern.items{dk: "()"}
		      &[_,•..]                                               ReferencePattern{!mut}
		       [_,•..]                                               ArrayPattern
		        _                                                    WildcardPattern
		           ..                                                RestPattern
		                 Ok(_)                                       TuplePattern
		                   (_)                                       TuplePattern.items{dk: "()"}
		                    _                                        WildcardPattern
		                          (Some(&[_,•..]),•Err(()))          TuplePattern
		                           Some(&[_,•..])                    TuplePattern
		                               (&[_,•..])                    TuplePattern.items{dk: "()"}
		                                &[_,•..]                     ReferencePattern{!mut}
		                                 [_,•..]                     ArrayPattern
		                                  _                          WildcardPattern
		                                     ..                      RestPattern
		                                           Err(())           TuplePattern
		                                              (())           TuplePattern.items{dk: "()"}
		                                               ()            TuplePattern
		                                                       {}    BlockExpression                                                      */
		(None, Ok(&[])) | (None, Err(())) | (None, Ok(&[_])) => {}                                                                        /*
		(None,•Ok(&[]))•|•(None,•Err(()))•|•(None,•Ok(&[_]))•=>•{}    MatchExpressionCase
		(None,•Ok(&[]))•|•(None,•Err(()))•|•(None,•Ok(&[_]))          UnionPattern
		(None,•Ok(&[]))                                               TuplePattern
		       Ok(&[])                                                TuplePattern
		         (&[])                                                TuplePattern.items{dk: "()"}
		          &[]                                                 ReferencePattern{!mut}
		           []                                                 ArrayPattern
		                  (None,•Err(()))                             TuplePattern
		                         Err(())                              TuplePattern
		                            (())                              TuplePattern.items{dk: "()"}
		                             ()                               TuplePattern
		                                    (None,•Ok(&[_]))          TuplePattern
		                                           Ok(&[_])           TuplePattern
		                                             (&[_])           TuplePattern.items{dk: "()"}
		                                              &[_]            ReferencePattern{!mut}
		                                               [_]            ArrayPattern
		                                                _             WildcardPattern
		                                                        {}    BlockExpression                                                     */
		(None, Ok(&[_, _, ..])) => {}                                                                                                     /*
		(None,•Ok(&[_,•_,•..]))•=>•{}    MatchExpressionCase
		(None,•Ok(&[_,•_,•..]))          TuplePattern
		       Ok(&[_,•_,•..])           TuplePattern
		         (&[_,•_,•..])           TuplePattern.items{dk: "()"}
		          &[_,•_,•..]            ReferencePattern{!mut}
		           [_,•_,•..]            ArrayPattern
		            _                    WildcardPattern
		               _                 WildcardPattern
		                  ..             RestPattern
		                           {}    BlockExpression                                                                                  */
		(T::T1(()), V::V1(i)) => {}                                                                                                       /*
		(T::T1(()),•V::V1(i))•=>•{}    MatchExpressionCase
		(T::T1(()),•V::V1(i))          TuplePattern
		 T::T1(())                     TuplePattern
		 T::T1                         ExpressionPath
		      (())                     TuplePattern.items{dk: "()"}
		       ()                      TuplePattern
		            V::V1(i)           TuplePattern
		            V::V1              ExpressionPath
		                 (i)           TuplePattern.items{dk: "()"}
		                         {}    BlockExpression                                                                                    */
        (T::T2(()), V::V2(b)) => {}                                                                                                       /*
        (T::T2(()),•V::V2(b))•=>•{}    MatchExpressionCase
        (T::T2(()),•V::V2(b))          TuplePattern
         T::T2(())                     TuplePattern
         T::T2                         ExpressionPath
              (())                     TuplePattern.items{dk: "()"}
               ()                      TuplePattern
                    V::V2(b)           TuplePattern
                    V::V2              ExpressionPath
                         (b)           TuplePattern.items{dk: "()"}
                                 {}    BlockExpression                                                                                    */
		Foo::Bar { bar: Bar::A, .. } => {}                                                                                                /*
		Foo::Bar•{•bar:•Bar::A,•..•}•=>•{}    MatchExpressionCase
		Foo::Bar•{•bar:•Bar::A,•..•}          StructPattern
		Foo::Bar                              ExpressionPath
		         {•bar:•Bar::A,•..•}          StructPattern.properties{dk: "{}"}
		           bar:•Bar::A                StructPatternPropertyDestructured
		                Bar::A                ExpressionPath
		                        ..            RestPattern
		                                {}    BlockExpression                                                                             */
		::A::B(3) => {}                                                                                                                   /*
		::A::B(3)•=>•{}    MatchExpressionCase
		::A::B(3)          TuplePattern
		::A::B             ExpressionPath
		::A                ExpressionPath
		      (3)          TuplePattern.items{dk: "()"}
		       3           Literal{kind: Integer}
		             {}    BlockExpression                                                                                                */
		::A::B(_) if false => {}                                                                                                          /*
		::A::B(_)•if•false•=>•{}    MatchExpressionCase
		::A::B(_)                   TuplePattern
		::A::B                      ExpressionPath
		::A                         ExpressionPath
		      (_)                   TuplePattern.items{dk: "()"}
		       _                    WildcardPattern
		             false          Literal{kind: False}
		                      {}    BlockExpression                                                                                       */
		::A::B(..) if false => {}                                                                                                         /*
		::A::B(..)•if•false•=>•{}    MatchExpressionCase
		::A::B(..)                   TuplePattern
		::A::B                       ExpressionPath
		::A                          ExpressionPath
		      (..)                   TuplePattern.items{dk: "()"}
		       ..                    RestPattern
		              false          Literal{kind: False}
		                       {}    BlockExpression                                                                                      */
		::A::B(_n) => {}                                                                                                                  /*
		::A::B(_n)•=>•{}    MatchExpressionCase
		::A::B(_n)          TuplePattern
		::A::B              ExpressionPath
		::A                 ExpressionPath
		      (_n)          TuplePattern.items{dk: "()"}
		              {}    BlockExpression                                                                                               */
		::A::B => {}                                                                                                                      /*
		::A::B•=>•{}    MatchExpressionCase
		::A::B          ExpressionPath
		::A             ExpressionPath
		          {}    BlockExpression                                                                                                   */
		::A::B(::A::B) => {}                                                                                                              /*
		::A::B(::A::B)•=>•{}    MatchExpressionCase
		::A::B(::A::B)          TuplePattern
		::A::B                  ExpressionPath
		::A                     ExpressionPath
		      (::A::B)          TuplePattern.items{dk: "()"}
		       ::A::B           ExpressionPath
		       ::A              ExpressionPath
		                  {}    BlockExpression                                                                                           */
		::A::B(::A::B(_)) => {}                                                                                                           /*
		::A::B(::A::B(_))•=>•{}    MatchExpressionCase
		::A::B(::A::B(_))          TuplePattern
		::A::B                     ExpressionPath
		::A                        ExpressionPath
		      (::A::B(_))          TuplePattern.items{dk: "()"}
		       ::A::B(_)           TuplePattern
		       ::A::B              ExpressionPath
		       ::A                 ExpressionPath
		             (_)           TuplePattern.items{dk: "()"}
		              _            WildcardPattern
		                     {}    BlockExpression                                                                                        */
		::A::B(::A::B, ::A::B(_)) => {}                                                                                                   /*
		::A::B(::A::B,•::A::B(_))•=>•{}    MatchExpressionCase
		::A::B(::A::B,•::A::B(_))          TuplePattern
		::A::B                             ExpressionPath
		::A                                ExpressionPath
		      (::A::B,•::A::B(_))          TuplePattern.items{dk: "()"}
		       ::A::B                      ExpressionPath
		       ::A                         ExpressionPath
		               ::A::B(_)           TuplePattern
		               ::A::B              ExpressionPath
		               ::A                 ExpressionPath
		                     (_)           TuplePattern.items{dk: "()"}
		                      _            WildcardPattern
		                             {}    BlockExpression                                                                                */
		::A::B(::A::B(..), ::A::B) => {}                                                                                                  /*
		::A::B(::A::B(..),•::A::B)•=>•{}    MatchExpressionCase
		::A::B(::A::B(..),•::A::B)          TuplePattern
		::A::B                              ExpressionPath
		::A                                 ExpressionPath
		      (::A::B(..),•::A::B)          TuplePattern.items{dk: "()"}
		       ::A::B(..)                   TuplePattern
		       ::A::B                       ExpressionPath
		       ::A                          ExpressionPath
		             (..)                   TuplePattern.items{dk: "()"}
		              ..                    RestPattern
		                   ::A::B           ExpressionPath
		                   ::A              ExpressionPath
		                              {}    BlockExpression                                                                               */
		::A::B(..) => {}                                                                                                                  /*
		::A::B(..)•=>•{}    MatchExpressionCase
		::A::B(..)          TuplePattern
		::A::B              ExpressionPath
		::A                 ExpressionPath
		      (..)          TuplePattern.items{dk: "()"}
		       ..           RestPattern
		              {}    BlockExpression                                                                                               */
		A::<A<u8>> { x: A(10, 11) } => {}                                                                                                 /*
		A::<A<u8>>•{•x:•A(10,•11)•}•=>•{}    MatchExpressionCase
		A::<A<u8>>•{•x:•A(10,•11)•}          StructPattern
		A::<A<u8>>                           ExpressionTypeCast
		   <A<u8>>                           ExpressionTypeCast.typeArguments{dk: "<>"}
		    A<u8>                            TypeCall
		     <u8>                            TypeCall.typeArguments{dk: "<>"}
		           {•x:•A(10,•11)•}          StructPattern.properties{dk: "{}"}
		             x:•A(10,•11)            StructPatternPropertyDestructured
		                A(10,•11)            TuplePattern
		                 (10,•11)            TuplePattern.items{dk: "()"}
		                  10                 Literal{kind: Integer}
		                      11             Literal{kind: Integer}
		                               {}    BlockExpression                                                                              */
		::B::<<A<_> as C>::U> { x: A::<u8>(11, 16) } => {}                                                                                /*
		::B::<<A<_>•as•C>::U>•{•x:•A::<u8>(11,•16)•}•=>•{}    MatchExpressionCase
		::B::<<A<_>•as•C>::U>•{•x:•A::<u8>(11,•16)•}          StructPattern
		::B::<<A<_>•as•C>::U>                                 ExpressionTypeCast
		::B                                                   ExpressionPath
		     <<A<_>•as•C>::U>                                 ExpressionTypeCast.typeArguments{dk: "<>"}
		      <A<_>•as•C>::U                                  TypePath
		      <A<_>•as•C>                                     ExpressionTypeSelector
		       A<_>                                           TypeCall
		        <_>                                           TypeCall.typeArguments{dk: "<>"}
		         _                                            TypeInferred
		                      {•x:•A::<u8>(11,•16)•}          StructPattern.properties{dk: "{}"}
		                        x:•A::<u8>(11,•16)            StructPatternPropertyDestructured
		                           A::<u8>(11,•16)            TuplePattern
		                           A::<u8>                    ExpressionTypeCast
		                              <u8>                    ExpressionTypeCast.typeArguments{dk: "<>"}
		                                  (11,•16)            TuplePattern.items{dk: "()"}
		                                   11                 Literal{kind: Integer}
		                                       16             Literal{kind: Integer}
		                                                {}    BlockExpression                                                             */
		isize::MIN..5 | 5..=isize::MAX => {}                                                                                              /*
		isize::MIN..5•|•5..=isize::MAX•=>•{}    MatchExpressionCase
		isize::MIN..5•|•5..=isize::MAX          UnionPattern
		isize::MIN..5                           RangePattern{!last}
		isize::MIN                              ExpressionPath
		            5                           Literal{kind: Integer}
		                5..=isize::MAX          RangePattern{last}
		                5                       Literal{kind: Integer}
		                    isize::MAX          ExpressionPath
		                                  {}    BlockExpression                                                                           */
		0..5 | 5..=usize::MAX => {}                                                                                                       /*
		0..5•|•5..=usize::MAX•=>•{}    MatchExpressionCase
		0..5•|•5..=usize::MAX          UnionPattern
		0..5                           RangePattern{!last}
		0                              Literal{kind: Integer}
		   5                           Literal{kind: Integer}
		       5..=usize::MAX          RangePattern{last}
		       5                       Literal{kind: Integer}
		           usize::MAX          ExpressionPath
		                         {}    BlockExpression                                                                                    */
		(0..5, true) | (5..=usize::MAX, true) | (0..=usize::MAX, false) => {}                                                             /*
		(0..5,•true)•|•(5..=usize::MAX,•true)•|•(0..=usize::MAX,•false)•=>•{}    MatchExpressionCase
		(0..5,•true)•|•(5..=usize::MAX,•true)•|•(0..=usize::MAX,•false)          UnionPattern
		(0..5,•true)                                                             TuplePattern
		 0..5                                                                    RangePattern{!last}
		 0                                                                       Literal{kind: Integer}
		    5                                                                    Literal{kind: Integer}
		       true                                                              Literal{kind: True}
		               (5..=usize::MAX,•true)                                    TuplePattern
		                5..=usize::MAX                                           RangePattern{last}
		                5                                                        Literal{kind: Integer}
		                    usize::MAX                                           ExpressionPath
		                                true                                     Literal{kind: True}
		                                        (0..=usize::MAX,•false)          TuplePattern
		                                         0..=usize::MAX                  RangePattern{last}
		                                         0                               Literal{kind: Integer}
		                                             usize::MAX                  ExpressionPath
		                                                         false           Literal{kind: False}
		                                                                   {}    BlockExpression                                          */
		[Ok(box ref a), ref xs @ .., Err(box b), Err(box ref mut c)] => {}                                                                /*
		[Ok(box•ref•a),•ref•xs•@•..,•Err(box•b),•Err(box•ref•mut•c)]•=>•{}    MatchExpressionCase
		[Ok(box•ref•a),•ref•xs•@•..,•Err(box•b),•Err(box•ref•mut•c)]          ArrayPattern
		 Ok(box•ref•a)                                                        TuplePattern
		   (box•ref•a)                                                        TuplePattern.items{dk: "()"}
		    box•ref•a                                                         BoxPattern
		        ref•a                                                         PatternVariableDeclaration{ref, !mut}
		                ref•xs•@•..                                           PatternVariableDeclaration{ref, !mut}
		                         ..                                           RestPattern
		                             Err(box•b)                               TuplePattern
		                                (box•b)                               TuplePattern.items{dk: "()"}
		                                 box•b                                BoxPattern
		                                         Err(box•ref•mut•c)           TuplePattern
		                                            (box•ref•mut•c)           TuplePattern.items{dk: "()"}
		                                             box•ref•mut•c            BoxPattern
		                                                 ref•mut•c            PatternVariableDeclaration{ref, mut}
		                                                                {}    BlockExpression                                             */
		[Ok(box a), ref xs @ .., Err(box ref b), Err(box ref c)] => {}                                                                    /*
		[Ok(box•a),•ref•xs•@•..,•Err(box•ref•b),•Err(box•ref•c)]•=>•{}    MatchExpressionCase
		[Ok(box•a),•ref•xs•@•..,•Err(box•ref•b),•Err(box•ref•c)]          ArrayPattern
		 Ok(box•a)                                                        TuplePattern
		   (box•a)                                                        TuplePattern.items{dk: "()"}
		    box•a                                                         BoxPattern
		            ref•xs•@•..                                           PatternVariableDeclaration{ref, !mut}
		                     ..                                           RestPattern
		                         Err(box•ref•b)                           TuplePattern
		                            (box•ref•b)                           TuplePattern.items{dk: "()"}
		                             box•ref•b                            BoxPattern
		                                 ref•b                            PatternVariableDeclaration{ref, !mut}
		                                         Err(box•ref•c)           TuplePattern
		                                            (box•ref•c)           TuplePattern.items{dk: "()"}
		                                             box•ref•c            BoxPattern
		                                                 ref•c            PatternVariableDeclaration{ref, !mut}
		                                                            {}    BlockExpression                                                 */
		box a => {Foo(box 1)}                                                                                                             /*
		box•a•=>•{Foo(box•1)}    MatchExpressionCase
		box•a                    BoxPattern
		         {Foo(box•1)}    BlockExpression
		          Foo(box•1)     ExpressionStatement{!semi}, CallExpression
		             (box•1)     CallExpression.arguments{dk: "()"}
		              box•1      BoxExpression
		                  1      Literal{kind: Integer}                                                                                   */
		box [Ok(a), ref xs @ .., Err(ref b)] => {}                                                                                        /*
		box•[Ok(a),•ref•xs•@•..,•Err(ref•b)]•=>•{}    MatchExpressionCase
		box•[Ok(a),•ref•xs•@•..,•Err(ref•b)]          BoxPattern
		    [Ok(a),•ref•xs•@•..,•Err(ref•b)]          ArrayPattern
		     Ok(a)                                    TuplePattern
		       (a)                                    TuplePattern.items{dk: "()"}
		            ref•xs•@•..                       PatternVariableDeclaration{ref, !mut}
		                     ..                       RestPattern
		                         Err(ref•b)           TuplePattern
		                            (ref•b)           TuplePattern.items{dk: "()"}
		                             ref•b            PatternVariableDeclaration{ref, !mut}
		                                        {}    BlockExpression                                                                     */
		ref a @ box b => {}                                                                                                               /*
		ref•a•@•box•b•=>•{}    MatchExpressionCase
		ref•a•@•box•b          PatternVariableDeclaration{ref, !mut}
		        box•b          BoxPattern
		                 {}    BlockExpression                                                                                            */
		ref a @ box ref b => {}                                                                                                           /*
		ref•a•@•box•ref•b•=>•{}    MatchExpressionCase
		ref•a•@•box•ref•b          PatternVariableDeclaration{ref, !mut}
		        box•ref•b          BoxPattern
		            ref•b          PatternVariableDeclaration{ref, !mut}
		                     {}    BlockExpression                                                                                        */
		Ok(ref a @ b) | Err(b @ ref a) => {}                                                                                              /*
		Ok(ref•a•@•b)•|•Err(b•@•ref•a)•=>•{}    MatchExpressionCase
		Ok(ref•a•@•b)•|•Err(b•@•ref•a)          UnionPattern
		Ok(ref•a•@•b)                           TuplePattern
		  (ref•a•@•b)                           TuplePattern.items{dk: "()"}
		   ref•a•@•b                            PatternVariableDeclaration{ref, !mut}
		                Err(b•@•ref•a)          TuplePattern
		                   (b•@•ref•a)          TuplePattern.items{dk: "()"}
		                    b•@•ref•a           PatternVariableDeclaration{!ref, !mut}
		                        ref•a           PatternVariableDeclaration{ref, !mut}
		                                  {}    BlockExpression                                                                           */
		ref a @ Ok(ref b) | ref a @ Err(ref b) => {}                                                                                      /*
		ref•a•@•Ok(ref•b)•|•ref•a•@•Err(ref•b)•=>•{}    MatchExpressionCase
		ref•a•@•Ok(ref•b)•|•ref•a•@•Err(ref•b)          UnionPattern
		ref•a•@•Ok(ref•b)                               PatternVariableDeclaration{ref, !mut}
		        Ok(ref•b)                               TuplePattern
		          (ref•b)                               TuplePattern.items{dk: "()"}
		           ref•b                                PatternVariableDeclaration{ref, !mut}
		                    ref•a•@•Err(ref•b)          PatternVariableDeclaration{ref, !mut}
		                            Err(ref•b)          TuplePattern
		                               (ref•b)          TuplePattern.items{dk: "()"}
		                                ref•b           PatternVariableDeclaration{ref, !mut}
		                                          {}    BlockExpression                                                                   */
		ref a @ Ok(ref mut b) | ref a @ Err(ref mut b) if { *b = U; false } => {}                                                         /*
		ref•a•@•Ok(ref•mut•b)•|•ref•a•@•Err(ref•mut•b)•if•{•*b•=•U;•false•}•=>•{}    MatchExpressionCase
		ref•a•@•Ok(ref•mut•b)•|•ref•a•@•Err(ref•mut•b)                               UnionPattern
		ref•a•@•Ok(ref•mut•b)                                                        PatternVariableDeclaration{ref, !mut}
		        Ok(ref•mut•b)                                                        TuplePattern
		          (ref•mut•b)                                                        TuplePattern.items{dk: "()"}
		           ref•mut•b                                                         PatternVariableDeclaration{ref, mut}
		                        ref•a•@•Err(ref•mut•b)                               PatternVariableDeclaration{ref, !mut}
		                                Err(ref•mut•b)                               TuplePattern
		                                   (ref•mut•b)                               TuplePattern.items{dk: "()"}
		                                    ref•mut•b                                PatternVariableDeclaration{ref, mut}
		                                                  {•*b•=•U;•false•}          BlockExpression
		                                                    *b•=•U;                  ExpressionStatement{semi}
		                                                    *b•=•U                   ReassignmentExpression{tk: "="}
		                                                    *b                       DereferenceExpression
		                                                            false            ExpressionStatement{!semi}, Literal{kind: False}
		                                                                       {}    BlockExpression                                      */
		ref mut a @ Ok(ref b) | ref mut a @ Err(ref b) if { *a = Err(U); false } => {}                                                    /*
		ref•mut•a•@•Ok(ref•b)•|•ref•mut•a•@•Err(ref•b)•if•{•*a•=•Err(U);•false•}•=>•{}    MatchExpressionCase
		ref•mut•a•@•Ok(ref•b)•|•ref•mut•a•@•Err(ref•b)                                    UnionPattern
		ref•mut•a•@•Ok(ref•b)                                                             PatternVariableDeclaration{ref, mut}
		            Ok(ref•b)                                                             TuplePattern
		              (ref•b)                                                             TuplePattern.items{dk: "()"}
		               ref•b                                                              PatternVariableDeclaration{ref, !mut}
		                        ref•mut•a•@•Err(ref•b)                                    PatternVariableDeclaration{ref, mut}
		                                    Err(ref•b)                                    TuplePattern
		                                       (ref•b)                                    TuplePattern.items{dk: "()"}
		                                        ref•b                                     PatternVariableDeclaration{ref, !mut}
		                                                  {•*a•=•Err(U);•false•}          BlockExpression
		                                                    *a•=•Err(U);                  ExpressionStatement{semi}
		                                                    *a•=•Err(U)                   ReassignmentExpression{tk: "="}
		                                                    *a                            DereferenceExpression
		                                                         Err(U)                   CallExpression
		                                                            (U)                   CallExpression.arguments{dk: "()"}
		                                                                 false            ExpressionStatement{!semi}, Literal{kind: False}
		                                                                            {}    BlockExpression                                 */
		a @ Some((mut b @ ref mut c, d @ ref e)) => {}                                                                                    /*
		a•@•Some((mut•b•@•ref•mut•c,•d•@•ref•e))•=>•{}    MatchExpressionCase
		a•@•Some((mut•b•@•ref•mut•c,•d•@•ref•e))          PatternVariableDeclaration{!ref, !mut}
		    Some((mut•b•@•ref•mut•c,•d•@•ref•e))          TuplePattern
		        ((mut•b•@•ref•mut•c,•d•@•ref•e))          TuplePattern.items{dk: "()"}
		         (mut•b•@•ref•mut•c,•d•@•ref•e)           TuplePattern
		          mut•b•@•ref•mut•c                       PatternVariableDeclaration{!ref, mut}
		                  ref•mut•c                       PatternVariableDeclaration{ref, mut}
		                             d•@•ref•e            PatternVariableDeclaration{!ref, !mut}
		                                 ref•e            PatternVariableDeclaration{ref, !mut}
		                                            {}    BlockExpression                                                                 */
		mut a @ Some([ref b, ref mut c]) => {}                                                                                            /*
		mut•a•@•Some([ref•b,•ref•mut•c])•=>•{}    MatchExpressionCase
		mut•a•@•Some([ref•b,•ref•mut•c])          PatternVariableDeclaration{!ref, mut}
		        Some([ref•b,•ref•mut•c])          TuplePattern
		            ([ref•b,•ref•mut•c])          TuplePattern.items{dk: "()"}
		             [ref•b,•ref•mut•c]           ArrayPattern
		              ref•b                       PatternVariableDeclaration{ref, !mut}
		                     ref•mut•c            PatternVariableDeclaration{ref, mut}
		                                    {}    BlockExpression                                                                         */
		ref mut a @ Some([b, mut c]) => {}                                                                                                /*
		ref•mut•a•@•Some([b,•mut•c])•=>•{}    MatchExpressionCase
		ref•mut•a•@•Some([b,•mut•c])          PatternVariableDeclaration{ref, mut}
		            Some([b,•mut•c])          TuplePattern
		                ([b,•mut•c])          TuplePattern.items{dk: "()"}
		                 [b,•mut•c]           ArrayPattern
		                     mut•c            PatternVariableDeclaration{!ref, mut}
		                                {}    BlockExpression                                                                             */
		ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {}                                                                      /*
		ref•mut•a•@•Ok(ref•mut•b)•|•ref•mut•a•@•Err(ref•mut•b)•=>•{}    MatchExpressionCase
		ref•mut•a•@•Ok(ref•mut•b)•|•ref•mut•a•@•Err(ref•mut•b)          UnionPattern
		ref•mut•a•@•Ok(ref•mut•b)                                       PatternVariableDeclaration{ref, mut}
		            Ok(ref•mut•b)                                       TuplePattern
		              (ref•mut•b)                                       TuplePattern.items{dk: "()"}
		               ref•mut•b                                        PatternVariableDeclaration{ref, mut}
		                            ref•mut•a•@•Err(ref•mut•b)          PatternVariableDeclaration{ref, mut}
		                                        Err(ref•mut•b)          TuplePattern
		                                           (ref•mut•b)          TuplePattern.items{dk: "()"}
		                                            ref•mut•b           PatternVariableDeclaration{ref, mut}
		                                                          {}    BlockExpression                                                   */
		ref bar @ Some(box n) if n > 0 => {}                                                                                              /*
		ref•bar•@•Some(box•n)•if•n•>•0•=>•{}    MatchExpressionCase
		ref•bar•@•Some(box•n)                   PatternVariableDeclaration{ref, !mut}
		          Some(box•n)                   TuplePattern
		              (box•n)                   TuplePattern.items{dk: "()"}
		               box•n                    BoxPattern
		                         n•>•0          ComparisonExpression{tk: ">"}
		                             0          Literal{kind: Integer}
		                                  {}    BlockExpression                                                                           */
		Some(ref bar @ box n) if n < 0 => {}                                                                                              /*
		Some(ref•bar•@•box•n)•if•n•<•0•=>•{}    MatchExpressionCase
		Some(ref•bar•@•box•n)                   TuplePattern
		    (ref•bar•@•box•n)                   TuplePattern.items{dk: "()"}
		     ref•bar•@•box•n                    PatternVariableDeclaration{ref, !mut}
		               box•n                    BoxPattern
		                         n•<•0          ComparisonExpression{tk: "<"}
		                             0          Literal{kind: Integer}
		                                  {}    BlockExpression                                                                           */
		ref x @ A { ref a, b: 20 } => {}                                                                                                  /*
		ref•x•@•A•{•ref•a,•b:•20•}•=>•{}    MatchExpressionCase
		ref•x•@•A•{•ref•a,•b:•20•}          PatternVariableDeclaration{ref, !mut}
		        A•{•ref•a,•b:•20•}          StructPattern
		          {•ref•a,•b:•20•}          StructPattern.properties{dk: "{}"}
		            ref•a                   StructPatternPropertyShorthand{!box, ref, !mut}
		                   b:•20            StructPatternPropertyDestructured
		                      20            Literal{kind: Integer}
		                              {}    BlockExpression                                                                               */
        (a,_) | (_,a) if a > 10 => 0,                                                                                                     /*
        (a,_)•|•(_,a)•if•a•>•10•=>•0    MatchExpressionCase
        (a,_)•|•(_,a)                   UnionPattern
        (a,_)                           TuplePattern
           _                            WildcardPattern
                (_,a)                   TuplePattern
                 _                      WildcardPattern
                         a•>•10         ComparisonExpression{tk: ">"}
                             10         Literal{kind: Integer}
                                   0    Literal{kind: Integer}                                                                            */
        e @ &(1..=2) | e @ &(3..=4) => {}                                                                                                 /*
        e•@•&(1..=2)•|•e•@•&(3..=4)•=>•{}    MatchExpressionCase
        e•@•&(1..=2)•|•e•@•&(3..=4)          UnionPattern
        e•@•&(1..=2)                         PatternVariableDeclaration{!ref, !mut}
            &(1..=2)                         ReferencePattern{!mut}
              1..=2                          RangePattern{last}
              1                              Literal{kind: Integer}
                  2                          Literal{kind: Integer}
                       e•@•&(3..=4)          PatternVariableDeclaration{!ref, !mut}
                           &(3..=4)          ReferencePattern{!mut}
                             3..=4           RangePattern{last}
                             3               Literal{kind: Integer}
                                 4           Literal{kind: Integer}
                                       {}    BlockExpression                                                                              */
        0 | &1 => {}                                                                                                                      /*
        0•|•&1•=>•{}    MatchExpressionCase
        0•|•&1          UnionPattern
        0               Literal{kind: Integer}
            &1          ReferencePattern{!mut}
             1          Literal{kind: Integer}
                  {}    BlockExpression                                                                                                   */
        Ok(x) | Err(x) => 0,                                                                                                              /*
        Ok(x)•|•Err(x)•=>•0    MatchExpressionCase
        Ok(x)•|•Err(x)         UnionPattern
        Ok(x)                  TuplePattern
          (x)                  TuplePattern.items{dk: "()"}
                Err(x)         TuplePattern
                   (x)         TuplePattern.items{dk: "()"}
                          0    Literal{kind: Integer}                                                                                     */
        &(Ok(x) | Err(x)) => 0,                                                                                                           /*
        &(Ok(x)•|•Err(x))•=>•0    MatchExpressionCase
        &(Ok(x)•|•Err(x))         ReferencePattern{!mut}
          Ok(x)•|•Err(x)          UnionPattern
          Ok(x)                   TuplePattern
            (x)                   TuplePattern.items{dk: "()"}
                  Err(x)          TuplePattern
                     (x)          TuplePattern.items{dk: "()"}
                             0    Literal{kind: Integer}                                                                                  */
        Ok(mut x) | &Err(mut x) => 0,                                                                                                     /*
        Ok(mut•x)•|•&Err(mut•x)•=>•0    MatchExpressionCase
        Ok(mut•x)•|•&Err(mut•x)         UnionPattern
        Ok(mut•x)                       TuplePattern
          (mut•x)                       TuplePattern.items{dk: "()"}
           mut•x                        PatternVariableDeclaration{!ref, mut}
                    &Err(mut•x)         ReferencePattern{!mut}
                     Err(mut•x)         TuplePattern
                        (mut•x)         TuplePattern.items{dk: "()"}
                         mut•x          PatternVariableDeclaration{!ref, mut}
                                   0    Literal{kind: Integer}                                                                            */
        Some((a, _) | (_, a)) if a > 10 => 0,                                                                                             /*
        Some((a,•_)•|•(_,•a))•if•a•>•10•=>•0    MatchExpressionCase
        Some((a,•_)•|•(_,•a))                   TuplePattern
            ((a,•_)•|•(_,•a))                   TuplePattern.items{dk: "()"}
             (a,•_)•|•(_,•a)                    UnionPattern
             (a,•_)                             TuplePattern
                 _                              WildcardPattern
                      (_,•a)                    TuplePattern
                       _                        WildcardPattern
                                 a•>•10         ComparisonExpression{tk: ">"}
                                     10         Literal{kind: Integer}
                                           0    Literal{kind: Integer}                                                                    */
		Some((a, _)) | Some((_, a)) if a > 10 => 0,                                                                                       /*
		Some((a,•_))•|•Some((_,•a))•if•a•>•10•=>•0    MatchExpressionCase
		Some((a,•_))•|•Some((_,•a))                   UnionPattern
		Some((a,•_))                                  TuplePattern
		    ((a,•_))                                  TuplePattern.items{dk: "()"}
		     (a,•_)                                   TuplePattern
		         _                                    WildcardPattern
		               Some((_,•a))                   TuplePattern
		                   ((_,•a))                   TuplePattern.items{dk: "()"}
		                    (_,•a)                    TuplePattern
		                     _                        WildcardPattern
		                               a•>•10         ComparisonExpression{tk: ">"}
		                                   10         Literal{kind: Integer}
		                                         0    Literal{kind: Integer}                                                              */
		Some(ref bar @ box Test::Baz | ref bar @ box Test::Qux) => 0,                                                                     /*
		Some(ref•bar•@•box•Test::Baz•|•ref•bar•@•box•Test::Qux)•=>•0    MatchExpressionCase
		Some(ref•bar•@•box•Test::Baz•|•ref•bar•@•box•Test::Qux)         TuplePattern
		    (ref•bar•@•box•Test::Baz•|•ref•bar•@•box•Test::Qux)         TuplePattern.items{dk: "()"}
		     ref•bar•@•box•Test::Baz•|•ref•bar•@•box•Test::Qux          UnionPattern
		     ref•bar•@•box•Test::Baz                                    PatternVariableDeclaration{ref, !mut}
		               box•Test::Baz                                    BoxPattern
		                   Test::Baz                                    ExpressionPath
		                               ref•bar•@•box•Test::Qux          PatternVariableDeclaration{ref, !mut}
		                                         box•Test::Qux          BoxPattern
		                                             Test::Qux          ExpressionPath
		                                                           0    Literal{kind: Integer}                                            */
		Some(x) if let Foo::Qux(y) = qux(x) => 0,                                                                                         /*
		Some(x)•if•let•Foo::Qux(y)•=•qux(x)•=>•0    MatchExpressionCase
		Some(x)                                     TuplePattern
		    (x)                                     TuplePattern.items{dk: "()"}
		           let•Foo::Qux(y)•=•qux(x)         LetScrutinee
		               Foo::Qux(y)                  TuplePattern
		               Foo::Qux                     ExpressionPath
		                       (y)                  TuplePattern.items{dk: "()"}
		                             qux(x)         CallExpression
		                                (x)         CallExpression.arguments{dk: "()"}
		                                       0    Literal{kind: Integer}                                                                */
		[bar @ .., n] if n == &5 => {}                                                                                                    /*
		[bar•@•..,•n]•if•n•==•&5•=>•{}    MatchExpressionCase
		[bar•@•..,•n]                     ArrayPattern
		 bar•@•..                         PatternVariableDeclaration{!ref, !mut}
		       ..                         RestPattern
		                 n•==•&5          ComparisonExpression{tk: "=="}
		                      &5          ReferenceExpression{!mut}
		                       5          Literal{kind: Integer}
		                            {}    BlockExpression                                                                                 */
		&A { a: 2 } if a.b().c() => {}                                                                                                    /*
		&A•{•a:•2•}•if•a.b().c()•=>•{}    MatchExpressionCase
		&A•{•a:•2•}                       ReferencePattern{!mut}
		 A•{•a:•2•}                       StructPattern
		   {•a:•2•}                       StructPattern.properties{dk: "{}"}
		     a:•2                         StructPatternPropertyDestructured
		        2                         Literal{kind: Integer}
		               a.b().c()          CallExpression
		               a.b()              CallExpression
		                  ()              CallExpression.arguments{dk: "()"}
		                      ()          CallExpression.arguments{dk: "()"}
		                            {}    BlockExpression                                                                                 */
		A::B { a } => {}                                                                                                                  /*
		A::B•{•a•}•=>•{}    MatchExpressionCase
		A::B•{•a•}          StructPattern
		A::B                ExpressionPath
		     {•a•}          StructPattern.properties{dk: "{}"}
		       a            StructPatternPropertyShorthand{!box, !ref, !mut}
		              {}    BlockExpression                                                                                               */
		&A::B { a } => {}                                                                                                                 /*
		&A::B•{•a•}•=>•{}    MatchExpressionCase
		&A::B•{•a•}          ReferencePattern{!mut}
		 A::B•{•a•}          StructPattern
		 A::B                ExpressionPath
		      {•a•}          StructPattern.properties{dk: "{}"}
		        a            StructPatternPropertyShorthand{!box, !ref, !mut}
		               {}    BlockExpression                                                                                              */
		box A::B { a } => {}                                                                                                              /*
		box•A::B•{•a•}•=>•{}    MatchExpressionCase
		box•A::B•{•a•}          BoxPattern
		    A::B•{•a•}          StructPattern
		    A::B                ExpressionPath
		         {•a•}          StructPattern.properties{dk: "{}"}
		           a            StructPatternPropertyShorthand{!box, !ref, !mut}
		                  {}    BlockExpression                                                                                           */
		(A::B { a },) => {}                                                                                                               /*
		(A::B•{•a•},)•=>•{}    MatchExpressionCase
		(A::B•{•a•},)          TuplePattern
		 A::B•{•a•}            StructPattern
		 A::B                  ExpressionPath
		      {•a•}            StructPattern.properties{dk: "{}"}
		        a              StructPatternPropertyShorthand{!box, !ref, !mut}
		                 {}    BlockExpression                                                                                            */
		[A::B { a }] => {}                                                                                                                /*
		[A::B•{•a•}]•=>•{}    MatchExpressionCase
		[A::B•{•a•}]          ArrayPattern
		 A::B•{•a•}           StructPattern
		 A::B                 ExpressionPath
		      {•a•}           StructPattern.properties{dk: "{}"}
		        a             StructPatternPropertyShorthand{!box, !ref, !mut}
		                {}    BlockExpression                                                                                             */
		C(A::B { a }, ()) => {}                                                                                                           /*
		C(A::B•{•a•},•())•=>•{}    MatchExpressionCase
		C(A::B•{•a•},•())          TuplePattern
		 (A::B•{•a•},•())          TuplePattern.items{dk: "()"}
		  A::B•{•a•}               StructPattern
		  A::B                     ExpressionPath
		       {•a•}               StructPattern.properties{dk: "{}"}
		         a                 StructPatternPropertyShorthand{!box, !ref, !mut}
		              ()           TuplePattern
		                     {}    BlockExpression                                                                                        */
        ((0 | 1,) | (2 | 3,),) => {}                                                                                                      /*
        ((0•|•1,)•|•(2•|•3,),)•=>•{}    MatchExpressionCase
        ((0•|•1,)•|•(2•|•3,),)          TuplePattern
         (0•|•1,)•|•(2•|•3,)            UnionPattern
         (0•|•1,)                       TuplePattern
          0•|•1                         UnionPattern
          0                             Literal{kind: Integer}
              1                         Literal{kind: Integer}
                    (2•|•3,)            TuplePattern
                     2•|•3              UnionPattern
                     2                  Literal{kind: Integer}
                         3              Literal{kind: Integer}
                                  {}    BlockExpression                                                                                   */
        (Some(2..=255),) => {}                                                                                                            /*
        (Some(2..=255),)•=>•{}    MatchExpressionCase
        (Some(2..=255),)          TuplePattern
         Some(2..=255)            TuplePattern
             (2..=255)            TuplePattern.items{dk: "()"}
              2..=255             RangePattern{last}
              2                   Literal{kind: Integer}
                  255             Literal{kind: Integer}
                            {}    BlockExpression                                                                                         */
        (None | Some(0 | 1),) => {}                                                                                                       /*
        (None•|•Some(0•|•1),)•=>•{}    MatchExpressionCase
        (None•|•Some(0•|•1),)          TuplePattern
         None•|•Some(0•|•1)            UnionPattern
                Some(0•|•1)            TuplePattern
                    (0•|•1)            TuplePattern.items{dk: "()"}
                     0•|•1             UnionPattern
                     0                 Literal{kind: Integer}
                         1             Literal{kind: Integer}
                                 {}    BlockExpression                                                                                    */
        (1 | 2,) => {}                                                                                                                    /*
        (1•|•2,)•=>•{}    MatchExpressionCase
        (1•|•2,)          TuplePattern
         1•|•2            UnionPattern
         1                Literal{kind: Integer}
             2            Literal{kind: Integer}
                    {}    BlockExpression                                                                                                 */
        (1 | 2, 3 | 4) => {}                                                                                                              /*
        (1•|•2,•3•|•4)•=>•{}    MatchExpressionCase
        (1•|•2,•3•|•4)          TuplePattern
         1•|•2                  UnionPattern
         1                      Literal{kind: Integer}
             2                  Literal{kind: Integer}
                3•|•4           UnionPattern
                3               Literal{kind: Integer}
                    4           Literal{kind: Integer}
                          {}    BlockExpression                                                                                           */
        ([] | [0 | 1..=255] | [_, ..],) => {}                                                                                             /*
        ([]•|•[0•|•1..=255]•|•[_,•..],)•=>•{}    MatchExpressionCase
        ([]•|•[0•|•1..=255]•|•[_,•..],)          TuplePattern
         []•|•[0•|•1..=255]•|•[_,•..]            UnionPattern
         []                                      ArrayPattern
              [0•|•1..=255]                      ArrayPattern
               0•|•1..=255                       UnionPattern
               0                                 Literal{kind: Integer}
                   1..=255                       RangePattern{last}
                   1                             Literal{kind: Integer}
                       255                       Literal{kind: Integer}
                              [_,•..]            ArrayPattern
                               _                 WildcardPattern
                                  ..             RestPattern
                                           {}    BlockExpression                                                                          */
        ((0, 0) | (0, 1),) => {}                                                                                                          /*
        ((0,•0)•|•(0,•1),)•=>•{}    MatchExpressionCase
        ((0,•0)•|•(0,•1),)          TuplePattern
         (0,•0)•|•(0,•1)            UnionPattern
         (0,•0)                     TuplePattern
          0                         Literal{kind: Integer}
             0                      Literal{kind: Integer}
                  (0,•1)            TuplePattern
                   0                Literal{kind: Integer}
                      1             Literal{kind: Integer}
                              {}    BlockExpression                                                                                       */
        ((0, 0) | (1, 0),) => {}                                                                                                          /*
        ((0,•0)•|•(1,•0),)•=>•{}    MatchExpressionCase
        ((0,•0)•|•(1,•0),)          TuplePattern
         (0,•0)•|•(1,•0)            UnionPattern
         (0,•0)                     TuplePattern
          0                         Literal{kind: Integer}
             0                      Literal{kind: Integer}
                  (1,•0)            TuplePattern
                   1                Literal{kind: Integer}
                      0             Literal{kind: Integer}
                              {}    BlockExpression                                                                                       */
		Tri::A(Ok(mut x) | Err(mut x)) | Tri::B(&Ok(mut x) | Err(mut x)) | &Tri::C(Ok(mut x) | Err(mut x)) => 0,                          /*
		Tri::A(Ok(mut•x)•|•Err(mut•x))•|•Tri::B(&Ok(mut•x)•|•Err(mut•x))•|•&Tri::C(Ok(mut•x)•|•Err(mut•x))•=>•0    MatchExpressionCase
		Tri::A(Ok(mut•x)•|•Err(mut•x))•|•Tri::B(&Ok(mut•x)•|•Err(mut•x))•|•&Tri::C(Ok(mut•x)•|•Err(mut•x))         UnionPattern
		Tri::A(Ok(mut•x)•|•Err(mut•x))                                                                             TuplePattern
		Tri::A                                                                                                     ExpressionPath
		      (Ok(mut•x)•|•Err(mut•x))                                                                             TuplePattern.items{dk: "()"}
		       Ok(mut•x)•|•Err(mut•x)                                                                              UnionPattern
		       Ok(mut•x)                                                                                           TuplePattern
		         (mut•x)                                                                                           TuplePattern.items{dk: "()"}
		          mut•x                                                                                            PatternVariableDeclaration{!ref, mut}
		                   Err(mut•x)                                                                              TuplePattern
		                      (mut•x)                                                                              TuplePattern.items{dk: "()"}
		                       mut•x                                                                               PatternVariableDeclaration{!ref, mut}
		                                 Tri::B(&Ok(mut•x)•|•Err(mut•x))                                           TuplePattern
		                                 Tri::B                                                                    ExpressionPath
		                                       (&Ok(mut•x)•|•Err(mut•x))                                           TuplePattern.items{dk: "()"}
		                                        &Ok(mut•x)•|•Err(mut•x)                                            UnionPattern
		                                        &Ok(mut•x)                                                         ReferencePattern{!mut}
		                                         Ok(mut•x)                                                         TuplePattern
		                                           (mut•x)                                                         TuplePattern.items{dk: "()"}
		                                            mut•x                                                          PatternVariableDeclaration{!ref, mut}
		                                                     Err(mut•x)                                            TuplePattern
		                                                        (mut•x)                                            TuplePattern.items{dk: "()"}
		                                                         mut•x                                             PatternVariableDeclaration{!ref, mut}
		                                                                   &Tri::C(Ok(mut•x)•|•Err(mut•x))         ReferencePattern{!mut}
		                                                                    Tri::C(Ok(mut•x)•|•Err(mut•x))         TuplePattern
		                                                                    Tri::C                                 ExpressionPath
		                                                                          (Ok(mut•x)•|•Err(mut•x))         TuplePattern.items{dk: "()"}
		                                                                           Ok(mut•x)•|•Err(mut•x)          UnionPattern
		                                                                           Ok(mut•x)                       TuplePattern
		                                                                             (mut•x)                       TuplePattern.items{dk: "()"}
		                                                                              mut•x                        PatternVariableDeclaration{!ref, mut}
		                                                                                       Err(mut•x)          TuplePattern
		                                                                                          (mut•x)          TuplePattern.items{dk: "()"}
		                                                                                           mut•x           PatternVariableDeclaration{!ref, mut}
		                                                                                                      0    Literal{kind: Integer} */
		Wrap(Ok(mut x) | &Err(mut x)) => 0,                                                                                               /*
		Wrap(Ok(mut•x)•|•&Err(mut•x))•=>•0    MatchExpressionCase
		Wrap(Ok(mut•x)•|•&Err(mut•x))         TuplePattern
		    (Ok(mut•x)•|•&Err(mut•x))         TuplePattern.items{dk: "()"}
		     Ok(mut•x)•|•&Err(mut•x)          UnionPattern
		     Ok(mut•x)                        TuplePattern
		       (mut•x)                        TuplePattern.items{dk: "()"}
		        mut•x                         PatternVariableDeclaration{!ref, mut}
		                 &Err(mut•x)          ReferencePattern{!mut}
		                  Err(mut•x)          TuplePattern
		                     (mut•x)          TuplePattern.items{dk: "()"}
		                      mut•x           PatternVariableDeclaration{!ref, mut}
		                                 0    Literal{kind: Integer}                                                                      */
		Wrap(&(Ok(x) | Err(x))) => 0,                                                                                                     /*
		Wrap(&(Ok(x)•|•Err(x)))•=>•0    MatchExpressionCase
		Wrap(&(Ok(x)•|•Err(x)))         TuplePattern
		    (&(Ok(x)•|•Err(x)))         TuplePattern.items{dk: "()"}
		     &(Ok(x)•|•Err(x))          ReferencePattern{!mut}
		       Ok(x)•|•Err(x)           UnionPattern
		       Ok(x)                    TuplePattern
		         (x)                    TuplePattern.items{dk: "()"}
		               Err(x)           TuplePattern
		                  (x)           TuplePattern.items{dk: "()"}
		                           0    Literal{kind: Integer}                                                                            */
		Wrap(Ok(x) | Err(x)) => 0,                                                                                                        /*
		Wrap(Ok(x)•|•Err(x))•=>•0    MatchExpressionCase
		Wrap(Ok(x)•|•Err(x))         TuplePattern
		    (Ok(x)•|•Err(x))         TuplePattern.items{dk: "()"}
		     Ok(x)•|•Err(x)          UnionPattern
		     Ok(x)                   TuplePattern
		       (x)                   TuplePattern.items{dk: "()"}
		             Err(x)          TuplePattern
		                (x)          TuplePattern.items{dk: "()"}
		                        0    Literal{kind: Integer}                                                                               */
		() if if if if 0 {0} else {0} {0} else {0} {0} else {0} => 0,                                                                     /*
		()•if•if•if•if•0•{0}•else•{0}•{0}•else•{0}•{0}•else•{0}•=>•0    MatchExpressionCase
		()                                                              TuplePattern
		      if•if•if•0•{0}•else•{0}•{0}•else•{0}•{0}•else•{0}         IfBlockExpression
		         if•if•0•{0}•else•{0}•{0}•else•{0}                      IfBlockExpression
		            if•0•{0}•else•{0}                                   IfBlockExpression
		               0                                                Literal{kind: Integer}
		                 {0}                                            IfBlockExpression.body{dk: "{}"}
		                  0                                             ExpressionStatement{!semi}, Literal{kind: Integer}
		                          {0}                                   BlockExpression
		                           0                                    ExpressionStatement{!semi}, Literal{kind: Integer}
		                              {0}                               IfBlockExpression.body{dk: "{}"}
		                               0                                ExpressionStatement{!semi}, Literal{kind: Integer}
		                                       {0}                      BlockExpression
		                                        0                       ExpressionStatement{!semi}, Literal{kind: Integer}
		                                           {0}                  IfBlockExpression.body{dk: "{}"}
		                                            0                   ExpressionStatement{!semi}, Literal{kind: Integer}
		                                                    {0}         BlockExpression
		                                                     0          ExpressionStatement{!semi}, Literal{kind: Integer}
		                                                           0    Literal{kind: Integer}                                            */
		Add | Mul | And | Or | BitXor | BitAnd | BitOr | Eq | Ne => 0,                                                                    /*
		Add•|•Mul•|•And•|•Or•|•BitXor•|•BitAnd•|•BitOr•|•Eq•|•Ne•=>•0    MatchExpressionCase
		Add•|•Mul•|•And•|•Or•|•BitXor•|•BitAnd•|•BitOr•|•Eq•|•Ne         UnionPattern
		                                                            0    Literal{kind: Integer}                                           */
        Sub | Div | Rem | Shl | Shr | Lt | Le | Ge | Gt => 0,                                                                             /*
        Sub•|•Div•|•Rem•|•Shl•|•Shr•|•Lt•|•Le•|•Ge•|•Gt•=>•0    MatchExpressionCase
        Sub•|•Div•|•Rem•|•Shl•|•Shr•|•Lt•|•Le•|•Ge•|•Gt         UnionPattern
                                                           0    Literal{kind: Integer}                                                    */
		ThisIsA::ReallyLongPatternNameToHelpOverflowTheNextValueOntoTheNextLine | ThisIsA::SecondValueSeparatedByAPipe | ThisIsA::ThirdValueSeparatedByAPipe => 0,/*
		ThisIsA::ReallyLongPatternNameToHelpOverflowTheNextValueOntoTheNextLine•|•ThisIsA::SecondValueSeparatedByAPipe•|•ThisIsA::ThirdValueSeparatedByAPipe•=>•0    MatchExpressionCase
		ThisIsA::ReallyLongPatternNameToHelpOverflowTheNextValueOntoTheNextLine•|•ThisIsA::SecondValueSeparatedByAPipe•|•ThisIsA::ThirdValueSeparatedByAPipe         UnionPattern
		ThisIsA::ReallyLongPatternNameToHelpOverflowTheNextValueOntoTheNextLine                                                                                      ExpressionPath
		                                                                          ThisIsA::SecondValueSeparatedByAPipe                                               ExpressionPath
		                                                                                                                 ThisIsA::ThirdValueSeparatedByAPipe         ExpressionPath
		                                                                                                                                                        0    Literal{kind: Integer}*/
		MyEnum::Option1 if cfg!(target_os="windows") =>                                                                                   /*
		MyEnum::Option1•if•cfg!(target_os="windows")•=>↲    <MatchExpressionCase>
		MyEnum::Option1                                     ExpressionPath
		                   cfg!(target_os="windows")        MacroInvocation
		                       (target_os="windows")        MacroInvocation.segments{dk: "()"}
		                                 =                  PunctuationToken{tk: "="}
		                                  "windows"         Literal{kind: String}                                                         */
            #[cfg(target_os="windows")]{                                                                                                  /*
            #[cfg(target_os="windows")]      (dangling) Attribute{!inner}
             [cfg(target_os="windows")]      (dangling) Attribute.segments{dk: "[]"}
                 (target_os="windows")       DelimGroup
                           =                 PunctuationToken{tk: "="}
                            "windows"        Literal{kind: String}
                                       {↲    <BlockExpression>                                                                            */
                1                                                                                                                         /*
                1    ExpressionStatement{!semi}, Literal{kind: Integer}                                                                   */
            }                                                                                                                             /*
••••••••••••}    </BlockExpression>
••••••••••••}    </MatchExpressionCase>                                                                                                   */
        MyEnum::Option1 if cfg!(target_os="windows") =>                                                                                   /*
        MyEnum::Option1•if•cfg!(target_os="windows")•=>↲    <MatchExpressionCase>
        MyEnum::Option1                                     ExpressionPath
                           cfg!(target_os="windows")        MacroInvocation
                               (target_os="windows")        MacroInvocation.segments{dk: "()"}
                                         =                  PunctuationToken{tk: "="}
                                          "windows"         Literal{kind: String}                                                         */
            #[cfg(target_os="windows")]                                                                                                   /*
            #[cfg(target_os="windows")]    (dangling) Attribute{!inner}
             [cfg(target_os="windows")]    (dangling) Attribute.segments{dk: "[]"}
                 (target_os="windows")     DelimGroup
                           =               PunctuationToken{tk: "="}
                            "windows"      Literal{kind: String}                                                                          */
                2,                                                                                                                        /*
                2    Literal{kind: Integer}
••••••••••••••••2    </MatchExpressionCase>                                                                                               */
		Some(RegionResolutionError::SubSupConflict(                                                                                       /*
		Some(RegionResolutionError::SubSupConflict(↲    <MatchExpressionCase>
		Some(RegionResolutionError::SubSupConflict(↲    <TuplePattern>
		    (RegionResolutionError::SubSupConflict(↲    <TuplePattern.items{dk: "()"}>
		     RegionResolutionError::SubSupConflict(↲    <TuplePattern>
		     RegionResolutionError::SubSupConflict      ExpressionPath
		                                          (↲    <TuplePattern.items{dk: "()"}>                                                    */
                vid,
                _,                                                                                                                        /*
                _    WildcardPattern                                                                                                      */
                SubregionOrigin::Subtype(box TypeTrace { cause, values }),                                                                /*
                SubregionOrigin::Subtype(box•TypeTrace•{•cause,•values•})    TuplePattern
                SubregionOrigin::Subtype                                     ExpressionPath
                                        (box•TypeTrace•{•cause,•values•})    TuplePattern.items{dk: "()"}
                                         box•TypeTrace•{•cause,•values•}     BoxPattern
                                             TypeTrace•{•cause,•values•}     StructPattern
                                                       {•cause,•values•}     StructPattern.properties{dk: "{}"}
                                                         cause               StructPatternPropertyShorthand{!box, !ref, !mut}
                                                                values       StructPatternPropertyShorthand{!box, !ref, !mut}             */
                sub_placeholder @ Region(Interned(RePlaceholder(_), _)),                                                                  /*
                sub_placeholder•@•Region(Interned(RePlaceholder(_),•_))    PatternVariableDeclaration{!ref, !mut}
                                  Region(Interned(RePlaceholder(_),•_))    TuplePattern
                                        (Interned(RePlaceholder(_),•_))    TuplePattern.items{dk: "()"}
                                         Interned(RePlaceholder(_),•_)     TuplePattern
                                                 (RePlaceholder(_),•_)     TuplePattern.items{dk: "()"}
                                                  RePlaceholder(_)         TuplePattern
                                                               (_)         TuplePattern.items{dk: "()"}
                                                                _          WildcardPattern
                                                                    _      WildcardPattern                                                */
                _,                                                                                                                        /*
                _    WildcardPattern                                                                                                      */
                sup_placeholder @ Region(Interned(RePlaceholder(_), _)),                                                                  /*
                sup_placeholder•@•Region(Interned(RePlaceholder(_),•_))    PatternVariableDeclaration{!ref, !mut}
                                  Region(Interned(RePlaceholder(_),•_))    TuplePattern
                                        (Interned(RePlaceholder(_),•_))    TuplePattern.items{dk: "()"}
                                         Interned(RePlaceholder(_),•_)     TuplePattern
                                                 (RePlaceholder(_),•_)     TuplePattern.items{dk: "()"}
                                                  RePlaceholder(_)         TuplePattern
                                                               (_)         TuplePattern.items{dk: "()"}
                                                                _          WildcardPattern
                                                                    _      WildcardPattern                                                */
                _,                                                                                                                        /*
                _    WildcardPattern                                                                                                      */
            )) => self.try_report_trait_placeholder_mismatch(                                                                             /*
••••••••••••)                                                     </TuplePattern.items>
••••••••••••)                                                     </TuplePattern>
••••••••••••))                                                    </TuplePattern.items>
••••••••••••))                                                    </TuplePattern>
                  self.try_report_trait_placeholder_mismatch(↲    <CallExpression>
                                                            (↲    <CallExpression.arguments{dk: "()"}>                                    */
                Some(self.tcx().mk_region(ReVar(*vid))),                                                                                  /*
                Some(self.tcx().mk_region(ReVar(*vid)))    CallExpression
                    (self.tcx().mk_region(ReVar(*vid)))    CallExpression.arguments{dk: "()"}
                     self.tcx().mk_region(ReVar(*vid))     CallExpression
                     self.tcx()                            CallExpression
                             ()                            CallExpression.arguments{dk: "()"}
                                         (ReVar(*vid))     CallExpression.arguments{dk: "()"}
                                          ReVar(*vid)      CallExpression
                                               (*vid)      CallExpression.arguments{dk: "()"}
                                                *vid       DereferenceExpression                                                          */
                cause,
                Some(*sub_placeholder),                                                                                                   /*
                Some(*sub_placeholder)    CallExpression
                    (*sub_placeholder)    CallExpression.arguments{dk: "()"}
                     *sub_placeholder     DereferenceExpression                                                                           */
                Some(*sup_placeholder),                                                                                                   /*
                Some(*sup_placeholder)    CallExpression
                    (*sup_placeholder)    CallExpression.arguments{dk: "()"}
                     *sup_placeholder     DereferenceExpression                                                                           */
                values,
            ),                                                                                                                            /*
••••••••••••)    </CallExpression.arguments>
••••••••••••)    </CallExpression>
••••••••••••)    </MatchExpressionCase>                                                                                                   */
		GenericParamKind::Const { kw_span, default: Some(default), .. } => {                                                              /*
		GenericParamKind::Const•{•kw_span,•default:•Some(default),•..•}•=>•{↲    <MatchExpressionCase>
		GenericParamKind::Const•{•kw_span,•default:•Some(default),•..•}          StructPattern
		GenericParamKind::Const                                                  ExpressionPath
		                        {•kw_span,•default:•Some(default),•..•}          StructPattern.properties{dk: "{}"}
		                          kw_span                                        StructPatternPropertyShorthand{!box, !ref, !mut}
		                                   default:•Some(default)                StructPatternPropertyDestructured
		                                            Some(default)                TuplePattern
		                                                (default)                TuplePattern.items{dk: "()"}
		                                                           ..            RestPattern
		                                                                   {↲    <BlockExpression>                                        */
            kw_span.to(default.value.span)                                                                                                /*
            kw_span.to(default.value.span)    ExpressionStatement{!semi}, CallExpression
                      (default.value.span)    CallExpression.arguments{dk: "()"}
                       default.value.span     MemberExpression{!computed}
                       default.value          MemberExpression{!computed}                                                                 */
        }                                                                                                                                 /*
••••••••}    </BlockExpression>
••••••••}    </MatchExpressionCase>                                                                                                       */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 8
// Parsed Nodes: 1913
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 8987 (28% re-reads)
// Unnecessary 'skip_whitespace()' calls: 870
// source: "../../samples/expressions/match.rs"