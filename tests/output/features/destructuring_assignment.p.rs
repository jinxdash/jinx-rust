#![feature(destructuring_assignment)]                                                                                                     /*
#![feature(destructuring_assignment)]↲    <Program>
#![feature(destructuring_assignment)]     Attribute{inner}
  [feature(destructuring_assignment)]     Attribute.segments{dk: "[]"}
          (destructuring_assignment)      DelimGroup                                                                                      */

fn main() {                                                                                                                               /*
fn•main()•{↲    <Program.ast{dk: "None"}>
fn•main()•{↲    <FunctionDeclaration>
       ()       FunctionDeclaration.parameters{dk: "()"}
          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                      */
	_ = 1;                                                                                                                                /*
	_•=•1;    ExpressionStatement{semi}
	_•=•1     ReassignmentExpression{tk: "="}
	_         UnassignedExpression
	    1     Literal{kind: Integer}                                                                                                      */
	_ = DropRecorder(1);                                                                                                                  /*
	_•=•DropRecorder(1);    ExpressionStatement{semi}
	_•=•DropRecorder(1)     ReassignmentExpression{tk: "="}
	_                       UnassignedExpression
	    DropRecorder(1)     CallExpression
	                (1)     CallExpression.arguments{dk: "()"}
	                 1      Literal{kind: Integer}                                                                                        */
	_val = DropRecorder(2);                                                                                                               /*
	_val•=•DropRecorder(2);    ExpressionStatement{semi}
	_val•=•DropRecorder(2)     ReassignmentExpression{tk: "="}
	       DropRecorder(2)     CallExpression
	                   (2)     CallExpression.arguments{dk: "()"}
	                    2      Literal{kind: Integer}                                                                                     */
	(_,) = (1, 2);                                                                                                                        /*
	(_,)•=•(1,•2);    ExpressionStatement{semi}
	(_,)•=•(1,•2)     ReassignmentExpression{tk: "="}
	(_,)              TupleLiteral
	 _                UnassignedExpression
	       (1,•2)     TupleLiteral
	        1         Literal{kind: Integer}
	           2      Literal{kind: Integer}                                                                                              */
	(.., a) = (1, 2);                                                                                                                     /*
	(..,•a)•=•(1,•2);    ExpressionStatement{semi}
	(..,•a)•=•(1,•2)     ReassignmentExpression{tk: "="}
	(..,•a)              TupleLiteral
	 ..                  RangeLiteral{!last}
	          (1,•2)     TupleLiteral
	           1         Literal{kind: Integer}
	              2      Literal{kind: Integer}                                                                                           */
	(..) = (3, 4);                                                                                                                        /*
	(..)•=•(3,•4);    ExpressionStatement{semi}
	(..)•=•(3,•4)     ReassignmentExpression{tk: "="}
	(..)              TupleLiteral
	 ..               RangeLiteral{!last}
	       (3,•4)     TupleLiteral
	        3         Literal{kind: Integer}
	           4      Literal{kind: Integer}                                                                                              */
	(((a, b)), (c)) = ((2, 3), d);                                                                                                        /*
	(((a,•b)),•(c))•=•((2,•3),•d);    ExpressionStatement{semi}
	(((a,•b)),•(c))•=•((2,•3),•d)     ReassignmentExpression{tk: "="}
	(((a,•b)),•(c))                   TupleLiteral
	  (a,•b)                          TupleLiteral
	                  ((2,•3),•d)     TupleLiteral
	                   (2,•3)         TupleLiteral
	                    2             Literal{kind: Integer}
	                       3          Literal{kind: Integer}                                                                              */
	((a, .., b), .., (..)) = ((4, 5), ());                                                                                                /*
	((a,•..,•b),•..,•(..))•=•((4,•5),•());    ExpressionStatement{semi}
	((a,•..,•b),•..,•(..))•=•((4,•5),•())     ReassignmentExpression{tk: "="}
	((a,•..,•b),•..,•(..))                    TupleLiteral
	 (a,•..,•b)                               TupleLiteral
	     ..                                   RangeLiteral{!last}
	             ..                           RangeLiteral{!last}
	                  ..                      RangeLiteral{!last}
	                         ((4,•5),•())     TupleLiteral
	                          (4,•5)          TupleLiteral
	                           4              Literal{kind: Integer}
	                              5           Literal{kind: Integer}
	                                  ()      TupleLiteral                                                                                */
	((a, b)) = (0, 1);                                                                                                                    /*
	((a,•b))•=•(0,•1);    ExpressionStatement{semi}
	((a,•b))•=•(0,•1)     ReassignmentExpression{tk: "="}
	 (a,•b)               TupleLiteral
	           (0,•1)     TupleLiteral
	            0         Literal{kind: Integer}
	               1      Literal{kind: Integer}                                                                                          */
	(*foo(&mut x), *foo(&mut x)) = (5, 6);                                                                                                /*
	(*foo(&mut•x),•*foo(&mut•x))•=•(5,•6);    ExpressionStatement{semi}
	(*foo(&mut•x),•*foo(&mut•x))•=•(5,•6)     ReassignmentExpression{tk: "="}
	(*foo(&mut•x),•*foo(&mut•x))              TupleLiteral
	 *foo(&mut•x)                             DereferenceExpression
	  foo(&mut•x)                             CallExpression
	     (&mut•x)                             CallExpression.arguments{dk: "()"}
	      &mut•x                              ReferenceExpression{mut}
	               *foo(&mut•x)               DereferenceExpression
	                foo(&mut•x)               CallExpression
	                   (&mut•x)               CallExpression.arguments{dk: "()"}
	                    &mut•x                ReferenceExpression{mut}
	                               (5,•6)     TupleLiteral
	                                5         Literal{kind: Integer}
	                                   6      Literal{kind: Integer}                                                                      */
	(a, _) = (8, 9);                                                                                                                      /*
	(a,•_)•=•(8,•9);    ExpressionStatement{semi}
	(a,•_)•=•(8,•9)     ReassignmentExpression{tk: "="}
	(a,•_)              TupleLiteral
	    _               UnassignedExpression
	         (8,•9)     TupleLiteral
	          8         Literal{kind: Integer}
	             9      Literal{kind: Integer}                                                                                            */
	(a, .., b) = (1, 2);                                                                                                                  /*
	(a,•..,•b)•=•(1,•2);    ExpressionStatement{semi}
	(a,•..,•b)•=•(1,•2)     ReassignmentExpression{tk: "="}
	(a,•..,•b)              TupleLiteral
	    ..                  RangeLiteral{!last}
	             (1,•2)     TupleLiteral
	              1         Literal{kind: Integer}
	                 2      Literal{kind: Integer}                                                                                        */
	(a, a, b) = (1, 2);                                                                                                                   /*
	(a,•a,•b)•=•(1,•2);    ExpressionStatement{semi}
	(a,•a,•b)•=•(1,•2)     ReassignmentExpression{tk: "="}
	(a,•a,•b)              TupleLiteral
	            (1,•2)     TupleLiteral
	             1         Literal{kind: Integer}
	                2      Literal{kind: Integer}                                                                                         */
	(a, b) += (3, 4);                                                                                                                     /*
	(a,•b)•+=•(3,•4);    ExpressionStatement{semi}
	(a,•b)•+=•(3,•4)     ReassignmentOperationExpression{tk: "+="}
	(a,•b)               TupleLiteral
	          (3,•4)     TupleLiteral
	           3         Literal{kind: Integer}
	              4      Literal{kind: Integer}                                                                                           */
	(a, b) = (0, 1);                                                                                                                      /*
	(a,•b)•=•(0,•1);    ExpressionStatement{semi}
	(a,•b)•=•(0,•1)     ReassignmentExpression{tk: "="}
	(a,•b)              TupleLiteral
	         (0,•1)     TupleLiteral
	          0         Literal{kind: Integer}
	             1      Literal{kind: Integer}                                                                                            */
	(a, b) = (3, 4);                                                                                                                      /*
	(a,•b)•=•(3,•4);    ExpressionStatement{semi}
	(a,•b)•=•(3,•4)     ReassignmentExpression{tk: "="}
	(a,•b)              TupleLiteral
	         (3,•4)     TupleLiteral
	          3         Literal{kind: Integer}
	             4      Literal{kind: Integer}                                                                                            */
	(b, ..) = (5, 6, 7);                                                                                                                  /*
	(b,•..)•=•(5,•6,•7);    ExpressionStatement{semi}
	(b,•..)•=•(5,•6,•7)     ReassignmentExpression{tk: "="}
	(b,•..)                 TupleLiteral
	    ..                  RangeLiteral{!last}
	          (5,•6,•7)     TupleLiteral
	           5            Literal{kind: Integer}
	              6         Literal{kind: Integer}
	                 7      Literal{kind: Integer}                                                                                        */
	(b, a) = (a, b);                                                                                                                      /*
	(b,•a)•=•(a,•b);    ExpressionStatement{semi}
	(b,•a)•=•(a,•b)     ReassignmentExpression{tk: "="}
	(b,•a)              TupleLiteral
	         (a,•b)     TupleLiteral                                                                                                      */
	(C, ..) = (0,1);                                                                                                                      /*
	(C,•..)•=•(0,1);    ExpressionStatement{semi}
	(C,•..)•=•(0,1)     ReassignmentExpression{tk: "="}
	(C,•..)             TupleLiteral
	    ..              RangeLiteral{!last}
	          (0,1)     TupleLiteral
	           0        Literal{kind: Integer}
	             1      Literal{kind: Integer}                                                                                            */
	(c, d) = ("c".to_owned(), "d".to_owned());                                                                                            /*
	(c,•d)•=•("c".to_owned(),•"d".to_owned());    ExpressionStatement{semi}
	(c,•d)•=•("c".to_owned(),•"d".to_owned())     ReassignmentExpression{tk: "="}
	(c,•d)                                        TupleLiteral
	         ("c".to_owned(),•"d".to_owned())     TupleLiteral
	          "c".to_owned()                      CallExpression
	          "c"                                 Literal{kind: String}
	                      ()                      CallExpression.arguments{dk: "()"}
	                          "d".to_owned()      CallExpression
	                          "d"                 Literal{kind: String}
	                                      ()      CallExpression.arguments{dk: "()"}                                                      */
	(d, c) = (c, d);                                                                                                                      /*
	(d,•c)•=•(c,•d);    ExpressionStatement{semi}
	(d,•c)•=•(c,•d)     ReassignmentExpression{tk: "="}
	(d,•c)              TupleLiteral
	         (c,•d)     TupleLiteral                                                                                                      */
	(test)() = TupleStruct(0, 0);                                                                                                         /*
	(test)()•=•TupleStruct(0,•0);    ExpressionStatement{semi}
	(test)()•=•TupleStruct(0,•0)     ReassignmentExpression{tk: "="}
	(test)()                         CallExpression
	      ()                         CallExpression.arguments{dk: "()"}
	           TupleStruct(0,•0)     CallExpression
	                      (0,•0)     CallExpression.arguments{dk: "()"}
	                       0         Literal{kind: Integer}
	                          0      Literal{kind: Integer}                                                                               */
	(x, _) = (DropRecorder(3), DropRecorder(4));                                                                                          /*
	(x,•_)•=•(DropRecorder(3),•DropRecorder(4));    ExpressionStatement{semi}
	(x,•_)•=•(DropRecorder(3),•DropRecorder(4))     ReassignmentExpression{tk: "="}
	(x,•_)                                          TupleLiteral
	    _                                           UnassignedExpression
	         (DropRecorder(3),•DropRecorder(4))     TupleLiteral
	          DropRecorder(3)                       CallExpression
	                      (3)                       CallExpression.arguments{dk: "()"}
	                       3                        Literal{kind: Integer}
	                           DropRecorder(4)      CallExpression
	                                       (4)      CallExpression.arguments{dk: "()"}
	                                        4       Literal{kind: Integer}                                                                */
	[_, a, _] = [1, 2, 3];                                                                                                                /*
	[_,•a,•_]•=•[1,•2,•3];    ExpressionStatement{semi}
	[_,•a,•_]•=•[1,•2,•3]     ReassignmentExpression{tk: "="}
	[_,•a,•_]                 ArrayLiteral
	 _                        UnassignedExpression
	       _                  UnassignedExpression
	            [1,•2,•3]     ArrayLiteral
	             1            Literal{kind: Integer}
	                2         Literal{kind: Integer}
	                   3      Literal{kind: Integer}                                                                                      */
	[_] = [1, 2];                                                                                                                         /*
	[_]•=•[1,•2];    ExpressionStatement{semi}
	[_]•=•[1,•2]     ReassignmentExpression{tk: "="}
	[_]              ArrayLiteral
	 _               UnassignedExpression
	      [1,•2]     ArrayLiteral
	       1         Literal{kind: Integer}
	          2      Literal{kind: Integer}                                                                                               */
	[..] = [1, 2, 3];                                                                                                                     /*
	[..]•=•[1,•2,•3];    ExpressionStatement{semi}
	[..]•=•[1,•2,•3]     ReassignmentExpression{tk: "="}
	[..]                 ArrayLiteral
	 ..                  RangeLiteral{!last}
	       [1,•2,•3]     ArrayLiteral
	        1            Literal{kind: Integer}
	           2         Literal{kind: Integer}
	              3      Literal{kind: Integer}                                                                                           */
	[a, .., b, ..] = [0, 1];                                                                                                              /*
	[a,•..,•b,•..]•=•[0,•1];    ExpressionStatement{semi}
	[a,•..,•b,•..]•=•[0,•1]     ReassignmentExpression{tk: "="}
	[a,•..,•b,•..]              ArrayLiteral
	    ..                      RangeLiteral{!last}
	           ..               RangeLiteral{!last}
	                 [0,•1]     ArrayLiteral
	                  0         Literal{kind: Integer}
	                     1      Literal{kind: Integer}                                                                                    */
	[a, .., b, c] = [1, 2, 3, 4, 5];                                                                                                      /*
	[a,•..,•b,•c]•=•[1,•2,•3,•4,•5];    ExpressionStatement{semi}
	[a,•..,•b,•c]•=•[1,•2,•3,•4,•5]     ReassignmentExpression{tk: "="}
	[a,•..,•b,•c]                       ArrayLiteral
	    ..                              RangeLiteral{!last}
	                [1,•2,•3,•4,•5]     ArrayLiteral
	                 1                  Literal{kind: Integer}
	                    2               Literal{kind: Integer}
	                       3            Literal{kind: Integer}
	                          4         Literal{kind: Integer}
	                             5      Literal{kind: Integer}                                                                            */
	[a, a, b] = [1, 2];                                                                                                                   /*
	[a,•a,•b]•=•[1,•2];    ExpressionStatement{semi}
	[a,•a,•b]•=•[1,•2]     ReassignmentExpression{tk: "="}
	[a,•a,•b]              ArrayLiteral
	            [1,•2]     ArrayLiteral
	             1         Literal{kind: Integer}
	                2      Literal{kind: Integer}                                                                                         */
	[a, b] += [3, 4];                                                                                                                     /*
	[a,•b]•+=•[3,•4];    ExpressionStatement{semi}
	[a,•b]•+=•[3,•4]     ReassignmentOperationExpression{tk: "+="}
	[a,•b]               ArrayLiteral
	          [3,•4]     ArrayLiteral
	           3         Literal{kind: Integer}
	              4      Literal{kind: Integer}                                                                                           */
	[a, b] = [0, 1];                                                                                                                      /*
	[a,•b]•=•[0,•1];    ExpressionStatement{semi}
	[a,•b]•=•[0,•1]     ReassignmentExpression{tk: "="}
	[a,•b]              ArrayLiteral
	         [0,•1]     ArrayLiteral
	          0         Literal{kind: Integer}
	             1      Literal{kind: Integer}                                                                                            */
	[a, b] = [3, 4];                                                                                                                      /*
	[a,•b]•=•[3,•4];    ExpressionStatement{semi}
	[a,•b]•=•[3,•4]     ReassignmentExpression{tk: "="}
	[a,•b]              ArrayLiteral
	         [3,•4]     ArrayLiteral
	          3         Literal{kind: Integer}
	             4      Literal{kind: Integer}                                                                                            */
	[c, ..] = [5, 6, 6];                                                                                                                  /*
	[c,•..]•=•[5,•6,•6];    ExpressionStatement{semi}
	[c,•..]•=•[5,•6,•6]     ReassignmentExpression{tk: "="}
	[c,•..]                 ArrayLiteral
	    ..                  RangeLiteral{!last}
	          [5,•6,•6]     ArrayLiteral
	           5            Literal{kind: Integer}
	              6         Literal{kind: Integer}
	                 6      Literal{kind: Integer}                                                                                        */
	<Alias::<isize> as Test>::test() = TupleStruct(0, 0);                                                                                 /*
	<Alias::<isize>•as•Test>::test()•=•TupleStruct(0,•0);    ExpressionStatement{semi}
	<Alias::<isize>•as•Test>::test()•=•TupleStruct(0,•0)     ReassignmentExpression{tk: "="}
	<Alias::<isize>•as•Test>::test()                         CallExpression
	<Alias::<isize>•as•Test>::test                           ExpressionPath
	<Alias::<isize>•as•Test>                                 ExpressionTypeSelector
	 Alias::<isize>                                          TypeCall
	        <isize>                                          TypeCall.typeArguments{dk: "<>"}
	                              ()                         CallExpression.arguments{dk: "()"}
	                                   TupleStruct(0,•0)     CallExpression
	                                              (0,•0)     CallExpression.arguments{dk: "()"}
	                                               0         Literal{kind: Integer}
	                                                  0      Literal{kind: Integer}                                                       */
	Alias::SingleVariant(a, b) = Alias::SingleVariant(9, 10);                                                                             /*
	Alias::SingleVariant(a,•b)•=•Alias::SingleVariant(9,•10);    ExpressionStatement{semi}
	Alias::SingleVariant(a,•b)•=•Alias::SingleVariant(9,•10)     ReassignmentExpression{tk: "="}
	Alias::SingleVariant(a,•b)                                   CallExpression
	Alias::SingleVariant                                         ExpressionPath
	                    (a,•b)                                   CallExpression.arguments{dk: "()"}
	                             Alias::SingleVariant(9,•10)     CallExpression
	                             Alias::SingleVariant            ExpressionPath
	                                                 (9,•10)     CallExpression.arguments{dk: "()"}
	                                                  9          Literal{kind: Integer}
	                                                     10      Literal{kind: Integer}                                                   */
	Enum::SingleVariant(_) = Enum::SingleVariant(1, 2);                                                                                   /*
	Enum::SingleVariant(_)•=•Enum::SingleVariant(1,•2);    ExpressionStatement{semi}
	Enum::SingleVariant(_)•=•Enum::SingleVariant(1,•2)     ReassignmentExpression{tk: "="}
	Enum::SingleVariant(_)                                 CallExpression
	Enum::SingleVariant                                    ExpressionPath
	                   (_)                                 CallExpression.arguments{dk: "()"}
	                    _                                  UnassignedExpression
	                         Enum::SingleVariant(1,•2)     CallExpression
	                         Enum::SingleVariant           ExpressionPath
	                                            (1,•2)     CallExpression.arguments{dk: "()"}
	                                             1         Literal{kind: Integer}
	                                                2      Literal{kind: Integer}                                                         */
	Enum::SingleVariant(a, .., b, ..) = Enum::SingleVariant(0, 1);                                                                        /*
	Enum::SingleVariant(a,•..,•b,•..)•=•Enum::SingleVariant(0,•1);    ExpressionStatement{semi}
	Enum::SingleVariant(a,•..,•b,•..)•=•Enum::SingleVariant(0,•1)     ReassignmentExpression{tk: "="}
	Enum::SingleVariant(a,•..,•b,•..)                                 CallExpression
	Enum::SingleVariant                                               ExpressionPath
	                   (a,•..,•b,•..)                                 CallExpression.arguments{dk: "()"}
	                       ..                                         RangeLiteral{!last}
	                              ..                                  RangeLiteral{!last}
	                                    Enum::SingleVariant(0,•1)     CallExpression
	                                    Enum::SingleVariant           ExpressionPath
	                                                       (0,•1)     CallExpression.arguments{dk: "()"}
	                                                        0         Literal{kind: Integer}
	                                                           1      Literal{kind: Integer}                                              */
	Enum::SingleVariant(a, a, b) = Enum::SingleVariant(1, 2);                                                                             /*
	Enum::SingleVariant(a,•a,•b)•=•Enum::SingleVariant(1,•2);    ExpressionStatement{semi}
	Enum::SingleVariant(a,•a,•b)•=•Enum::SingleVariant(1,•2)     ReassignmentExpression{tk: "="}
	Enum::SingleVariant(a,•a,•b)                                 CallExpression
	Enum::SingleVariant                                          ExpressionPath
	                   (a,•a,•b)                                 CallExpression.arguments{dk: "()"}
	                               Enum::SingleVariant(1,•2)     CallExpression
	                               Enum::SingleVariant           ExpressionPath
	                                                  (1,•2)     CallExpression.arguments{dk: "()"}
	                                                   1         Literal{kind: Integer}
	                                                      2      Literal{kind: Integer}                                                   */
	Enum::SingleVariant(a, b) = Enum::SingleVariant(7, 8);                                                                                /*
	Enum::SingleVariant(a,•b)•=•Enum::SingleVariant(7,•8);    ExpressionStatement{semi}
	Enum::SingleVariant(a,•b)•=•Enum::SingleVariant(7,•8)     ReassignmentExpression{tk: "="}
	Enum::SingleVariant(a,•b)                                 CallExpression
	Enum::SingleVariant                                       ExpressionPath
	                   (a,•b)                                 CallExpression.arguments{dk: "()"}
	                            Enum::SingleVariant(7,•8)     CallExpression
	                            Enum::SingleVariant           ExpressionPath
	                                               (7,•8)     CallExpression.arguments{dk: "()"}
	                                                7         Literal{kind: Integer}
	                                                   8      Literal{kind: Integer}                                                      */
	S { x: a, ..s } = S { x: 3, y: 4 };                                                                                                   /*
	S•{•x:•a,•..s•}•=•S•{•x:•3,•y:•4•};    ExpressionStatement{semi}
	S•{•x:•a,•..s•}•=•S•{•x:•3,•y:•4•}     ReassignmentExpression{tk: "="}
	S•{•x:•a,•..s•}                        StructLiteral
	  {•x:•a,•..s•}                        StructLiteral.properties{dk: "{}"}
	    x:•a                               StructLiteralProperty
	          ..s                          StructLiteralPropertySpread
	                  S•{•x:•3,•y:•4•}     StructLiteral
	                    {•x:•3,•y:•4•}     StructLiteral.properties{dk: "{}"}
	                      x:•3             StructLiteralProperty
	                         3             Literal{kind: Integer}
	                            y:•4       StructLiteralProperty
	                               4       Literal{kind: Integer}                                                                         */
	S { x: a, y: b } += s;                                                                                                                /*
	S•{•x:•a,•y:•b•}•+=•s;    ExpressionStatement{semi}
	S•{•x:•a,•y:•b•}•+=•s     ReassignmentOperationExpression{tk: "+="}
	S•{•x:•a,•y:•b•}          StructLiteral
	  {•x:•a,•y:•b•}          StructLiteral.properties{dk: "{}"}
	    x:•a                  StructLiteralProperty
	          y:•b            StructLiteralProperty                                                                                       */
	S { x: a, y: b } = s;                                                                                                                 /*
	S•{•x:•a,•y:•b•}•=•s;    ExpressionStatement{semi}
	S•{•x:•a,•y:•b•}•=•s     ReassignmentExpression{tk: "="}
	S•{•x:•a,•y:•b•}         StructLiteral
	  {•x:•a,•y:•b•}         StructLiteral.properties{dk: "{}"}
	    x:•a                 StructLiteralProperty
	          y:•b           StructLiteralProperty                                                                                        */
	Struct { .. } = Struct { a: 1, b: 4 };                                                                                                /*
	Struct•{•..•}•=•Struct•{•a:•1,•b:•4•};    ExpressionStatement{semi}
	Struct•{•..•}•=•Struct•{•a:•1,•b:•4•}     ReassignmentExpression{tk: "="}
	Struct•{•..•}                             StructLiteral
	       {•..•}                             StructLiteral.properties{dk: "{}"}
	         ..                               StructLiteralRestUnassigned
	                Struct•{•a:•1,•b:•4•}     StructLiteral
	                       {•a:•1,•b:•4•}     StructLiteral.properties{dk: "{}"}
	                         a:•1             StructLiteralProperty
	                            1             Literal{kind: Integer}
	                               b:•4       StructLiteralProperty
	                                  4       Literal{kind: Integer}                                                                      */
	Struct { a, .. } = Struct { a: 1, b: 3 };                                                                                             /*
	Struct•{•a,•..•}•=•Struct•{•a:•1,•b:•3•};    ExpressionStatement{semi}
	Struct•{•a,•..•}•=•Struct•{•a:•1,•b:•3•}     ReassignmentExpression{tk: "="}
	Struct•{•a,•..•}                             StructLiteral
	       {•a,•..•}                             StructLiteral.properties{dk: "{}"}
	         a                                   StructLiteralPropertyShorthand
	            ..                               StructLiteralRestUnassigned
	                   Struct•{•a:•1,•b:•3•}     StructLiteral
	                          {•a:•1,•b:•3•}     StructLiteral.properties{dk: "{}"}
	                            a:•1             StructLiteralProperty
	                               1             Literal{kind: Integer}
	                                  b:•3       StructLiteralProperty
	                                     3       Literal{kind: Integer}                                                                   */
	Struct { a, .. };                                                                                                                     /*
	Struct•{•a,•..•};    ExpressionStatement{semi}
	Struct•{•a,•..•}     StructLiteral
	       {•a,•..•}     StructLiteral.properties{dk: "{}"}
	         a           StructLiteralPropertyShorthand
	            ..       StructLiteralRestUnassigned                                                                                      */
	Struct { a, ..d } = Struct { a: 1, b: 2 };                                                                                            /*
	Struct•{•a,•..d•}•=•Struct•{•a:•1,•b:•2•};    ExpressionStatement{semi}
	Struct•{•a,•..d•}•=•Struct•{•a:•1,•b:•2•}     ReassignmentExpression{tk: "="}
	Struct•{•a,•..d•}                             StructLiteral
	       {•a,•..d•}                             StructLiteral.properties{dk: "{}"}
	         a                                    StructLiteralPropertyShorthand
	            ..d                               StructLiteralPropertySpread
	                    Struct•{•a:•1,•b:•2•}     StructLiteral
	                           {•a:•1,•b:•2•}     StructLiteral.properties{dk: "{}"}
	                             a:•1             StructLiteralProperty
	                                1             Literal{kind: Integer}
	                                   b:•2       StructLiteralProperty
	                                      2       Literal{kind: Integer}                                                                  */
	Struct { a, b } = Struct { a: 0, b: 1 };                                                                                              /*
	Struct•{•a,•b•}•=•Struct•{•a:•0,•b:•1•};    ExpressionStatement{semi}
	Struct•{•a,•b•}•=•Struct•{•a:•0,•b:•1•}     ReassignmentExpression{tk: "="}
	Struct•{•a,•b•}                             StructLiteral
	       {•a,•b•}                             StructLiteral.properties{dk: "{}"}
	         a                                  StructLiteralPropertyShorthand
	            b                               StructLiteralPropertyShorthand
	                  Struct•{•a:•0,•b:•1•}     StructLiteral
	                         {•a:•0,•b:•1•}     StructLiteral.properties{dk: "{}"}
	                           a:•0             StructLiteralProperty
	                              0             Literal{kind: Integer}
	                                 b:•1       StructLiteralProperty
	                                    1       Literal{kind: Integer}                                                                    */
	Struct { a, b, c } = Struct { a: 0, b: 1 };                                                                                           /*
	Struct•{•a,•b,•c•}•=•Struct•{•a:•0,•b:•1•};    ExpressionStatement{semi}
	Struct•{•a,•b,•c•}•=•Struct•{•a:•0,•b:•1•}     ReassignmentExpression{tk: "="}
	Struct•{•a,•b,•c•}                             StructLiteral
	       {•a,•b,•c•}                             StructLiteral.properties{dk: "{}"}
	         a                                     StructLiteralPropertyShorthand
	            b                                  StructLiteralPropertyShorthand
	               c                               StructLiteralPropertyShorthand
	                     Struct•{•a:•0,•b:•1•}     StructLiteral
	                            {•a:•0,•b:•1•}     StructLiteral.properties{dk: "{}"}
	                              a:•0             StructLiteralProperty
	                                 0             Literal{kind: Integer}
	                                    b:•1       StructLiteralProperty
	                                       1       Literal{kind: Integer}                                                                 */
	Struct { a: _, b } = Struct { a: 1, b: 2 };                                                                                           /*
	Struct•{•a:•_,•b•}•=•Struct•{•a:•1,•b:•2•};    ExpressionStatement{semi}
	Struct•{•a:•_,•b•}•=•Struct•{•a:•1,•b:•2•}     ReassignmentExpression{tk: "="}
	Struct•{•a:•_,•b•}                             StructLiteral
	       {•a:•_,•b•}                             StructLiteral.properties{dk: "{}"}
	         a:•_                                  StructLiteralProperty
	            _                                  UnassignedExpression
	               b                               StructLiteralPropertyShorthand
	                     Struct•{•a:•1,•b:•2•}     StructLiteral
	                            {•a:•1,•b:•2•}     StructLiteral.properties{dk: "{}"}
	                              a:•1             StructLiteralProperty
	                                 1             Literal{kind: Integer}
	                                    b:•2       StructLiteralProperty
	                                       2       Literal{kind: Integer}                                                                 */
	Struct { a: b, b: a }  = Struct { a: 1, b: 2 };                                                                                       /*
	Struct•{•a:•b,•b:•a•}••=•Struct•{•a:•1,•b:•2•};    ExpressionStatement{semi}
	Struct•{•a:•b,•b:•a•}••=•Struct•{•a:•1,•b:•2•}     ReassignmentExpression{tk: "="}
	Struct•{•a:•b,•b:•a•}                              StructLiteral
	       {•a:•b,•b:•a•}                              StructLiteral.properties{dk: "{}"}
	         a:•b                                      StructLiteralProperty
	               b:•a                                StructLiteralProperty
	                         Struct•{•a:•1,•b:•2•}     StructLiteral
	                                {•a:•1,•b:•2•}     StructLiteral.properties{dk: "{}"}
	                                  a:•1             StructLiteralProperty
	                                     1             Literal{kind: Integer}
	                                        b:•2       StructLiteralProperty
	                                           2       Literal{kind: Integer}                                                             */
	Struct { a: TupleStruct((a, b), c), b: [d] } = Struct { a: TupleStruct((0, 1), 2), b: [3] };                                          /*
	Struct•{•a:•TupleStruct((a,•b),•c),•b:•[d]•}•=•Struct•{•a:•TupleStruct((0,•1),•2),•b:•[3]•};    ExpressionStatement{semi}
	Struct•{•a:•TupleStruct((a,•b),•c),•b:•[d]•}•=•Struct•{•a:•TupleStruct((0,•1),•2),•b:•[3]•}     ReassignmentExpression{tk: "="}
	Struct•{•a:•TupleStruct((a,•b),•c),•b:•[d]•}                                                    StructLiteral
	       {•a:•TupleStruct((a,•b),•c),•b:•[d]•}                                                    StructLiteral.properties{dk: "{}"}
	         a:•TupleStruct((a,•b),•c)                                                              StructLiteralProperty
	            TupleStruct((a,•b),•c)                                                              CallExpression
	                       ((a,•b),•c)                                                              CallExpression.arguments{dk: "()"}
	                        (a,•b)                                                                  TupleLiteral
	                                    b:•[d]                                                      StructLiteralProperty
	                                       [d]                                                      ArrayLiteral
	                                               Struct•{•a:•TupleStruct((0,•1),•2),•b:•[3]•}     StructLiteral
	                                                      {•a:•TupleStruct((0,•1),•2),•b:•[3]•}     StructLiteral.properties{dk: "{}"}
	                                                        a:•TupleStruct((0,•1),•2)               StructLiteralProperty
	                                                           TupleStruct((0,•1),•2)               CallExpression
	                                                                      ((0,•1),•2)               CallExpression.arguments{dk: "()"}
	                                                                       (0,•1)                   TupleLiteral
	                                                                        0                       Literal{kind: Integer}
	                                                                           1                    Literal{kind: Integer}
	                                                                               2                Literal{kind: Integer}
	                                                                                   b:•[3]       StructLiteralProperty
	                                                                                      [3]       ArrayLiteral
	                                                                                       3        Literal{kind: Integer}                */
	test() = TupleStruct(0, 0);                                                                                                           /*
	test()•=•TupleStruct(0,•0);    ExpressionStatement{semi}
	test()•=•TupleStruct(0,•0)     ReassignmentExpression{tk: "="}
	test()                         CallExpression
	    ()                         CallExpression.arguments{dk: "()"}
	         TupleStruct(0,•0)     CallExpression
	                    (0,•0)     CallExpression.arguments{dk: "()"}
	                     0         Literal{kind: Integer}
	                        0      Literal{kind: Integer}                                                                                 */
	TupleStruct(_, a) = TupleStruct(2, 2);                                                                                                /*
	TupleStruct(_,•a)•=•TupleStruct(2,•2);    ExpressionStatement{semi}
	TupleStruct(_,•a)•=•TupleStruct(2,•2)     ReassignmentExpression{tk: "="}
	TupleStruct(_,•a)                         CallExpression
	           (_,•a)                         CallExpression.arguments{dk: "()"}
	            _                             UnassignedExpression
	                    TupleStruct(2,•2)     CallExpression
	                               (2,•2)     CallExpression.arguments{dk: "()"}
	                                2         Literal{kind: Integer}
	                                   2      Literal{kind: Integer}                                                                      */
	TupleStruct(_) = TupleStruct(1, 2);                                                                                                   /*
	TupleStruct(_)•=•TupleStruct(1,•2);    ExpressionStatement{semi}
	TupleStruct(_)•=•TupleStruct(1,•2)     ReassignmentExpression{tk: "="}
	TupleStruct(_)                         CallExpression
	           (_)                         CallExpression.arguments{dk: "()"}
	            _                          UnassignedExpression
	                 TupleStruct(1,•2)     CallExpression
	                            (1,•2)     CallExpression.arguments{dk: "()"}
	                             1         Literal{kind: Integer}
	                                2      Literal{kind: Integer}                                                                         */
	TupleStruct(..) = TupleStruct(3, 4);                                                                                                  /*
	TupleStruct(..)•=•TupleStruct(3,•4);    ExpressionStatement{semi}
	TupleStruct(..)•=•TupleStruct(3,•4)     ReassignmentExpression{tk: "="}
	TupleStruct(..)                         CallExpression
	           (..)                         CallExpression.arguments{dk: "()"}
	            ..                          RangeLiteral{!last}
	                  TupleStruct(3,•4)     CallExpression
	                             (3,•4)     CallExpression.arguments{dk: "()"}
	                              3         Literal{kind: Integer}
	                                 4      Literal{kind: Integer}                                                                        */
	TupleStruct(5,6).assign(&mut a, &mut b);                                                                                              /*
	TupleStruct(5,6).assign(&mut•a,•&mut•b);    ExpressionStatement{semi}
	TupleStruct(5,6).assign(&mut•a,•&mut•b)     CallExpression
	TupleStruct(5,6)                            CallExpression
	           (5,6)                            CallExpression.arguments{dk: "()"}
	            5                               Literal{kind: Integer}
	              6                             Literal{kind: Integer}
	                       (&mut•a,•&mut•b)     CallExpression.arguments{dk: "()"}
	                        &mut•a              ReferenceExpression{mut}
	                                &mut•b      ReferenceExpression{mut}                                                                  */
	TupleStruct(a, .., b, ..) = TupleStruct(0, 1);                                                                                        /*
	TupleStruct(a,•..,•b,•..)•=•TupleStruct(0,•1);    ExpressionStatement{semi}
	TupleStruct(a,•..,•b,•..)•=•TupleStruct(0,•1)     ReassignmentExpression{tk: "="}
	TupleStruct(a,•..,•b,•..)                         CallExpression
	           (a,•..,•b,•..)                         CallExpression.arguments{dk: "()"}
	               ..                                 RangeLiteral{!last}
	                      ..                          RangeLiteral{!last}
	                            TupleStruct(0,•1)     CallExpression
	                                       (0,•1)     CallExpression.arguments{dk: "()"}
	                                        0         Literal{kind: Integer}
	                                           1      Literal{kind: Integer}                                                              */
	TupleStruct(a, .., b) = TupleStruct(1, 2);                                                                                            /*
	TupleStruct(a,•..,•b)•=•TupleStruct(1,•2);    ExpressionStatement{semi}
	TupleStruct(a,•..,•b)•=•TupleStruct(1,•2)     ReassignmentExpression{tk: "="}
	TupleStruct(a,•..,•b)                         CallExpression
	           (a,•..,•b)                         CallExpression.arguments{dk: "()"}
	               ..                             RangeLiteral{!last}
	                        TupleStruct(1,•2)     CallExpression
	                                   (1,•2)     CallExpression.arguments{dk: "()"}
	                                    1         Literal{kind: Integer}
	                                       2      Literal{kind: Integer}                                                                  */
	TupleStruct(a, a, b) = TupleStruct(1, 2);                                                                                             /*
	TupleStruct(a,•a,•b)•=•TupleStruct(1,•2);    ExpressionStatement{semi}
	TupleStruct(a,•a,•b)•=•TupleStruct(1,•2)     ReassignmentExpression{tk: "="}
	TupleStruct(a,•a,•b)                         CallExpression
	           (a,•a,•b)                         CallExpression.arguments{dk: "()"}
	                       TupleStruct(1,•2)     CallExpression
	                                  (1,•2)     CallExpression.arguments{dk: "()"}
	                                   1         Literal{kind: Integer}
	                                      2      Literal{kind: Integer}                                                                   */
	TupleStruct(a, b) = TupleStruct(0, 1);                                                                                                /*
	TupleStruct(a,•b)•=•TupleStruct(0,•1);    ExpressionStatement{semi}
	TupleStruct(a,•b)•=•TupleStruct(0,•1)     ReassignmentExpression{tk: "="}
	TupleStruct(a,•b)                         CallExpression
	           (a,•b)                         CallExpression.arguments{dk: "()"}
	                    TupleStruct(0,•1)     CallExpression
	                               (0,•1)     CallExpression.arguments{dk: "()"}
	                                0         Literal{kind: Integer}
	                                   1      Literal{kind: Integer}                                                                      */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 5
// Parsed Nodes: 666
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 2562 (26% re-reads)
// Unnecessary 'skip_whitespace()' calls: 339
// source: "../../samples/features/destructuring_assignment.rs"