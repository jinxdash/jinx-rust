#![feature(destructuring_assignment)]                                                                                                     /*
#![feature(destructuring_assignment)]    Attribute
          (destructuring_assignment)     DelimGroup                                                                                       */

fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>                                                                                                     */
	_ = 1;                                                                                                                                /*
    _•=•1;    ExpressionStatement
    _•=•1     ReassignmentExpression
    _         UnassignedExpression
        1     Literal                                                                                                                     */
	_ = DropRecorder(1);                                                                                                                  /*
    _•=•DropRecorder(1);    ExpressionStatement
    _•=•DropRecorder(1)     ReassignmentExpression
    _                       UnassignedExpression
        DropRecorder(1)     CallExpression
                     1      Literal                                                                                                       */
	_val = DropRecorder(2);                                                                                                               /*
    _val•=•DropRecorder(2);    ExpressionStatement
    _val•=•DropRecorder(2)     ReassignmentExpression
           DropRecorder(2)     CallExpression
                        2      Literal                                                                                                    */
	(_,) = (1, 2);                                                                                                                        /*
    (_,)•=•(1,•2);    ExpressionStatement
    (_,)•=•(1,•2)     ReassignmentExpression
    (_,)              TupleLiteral
     _                UnassignedExpression
           (1,•2)     TupleLiteral
            1         Literal
               2      Literal                                                                                                             */
	(.., a) = (1, 2);                                                                                                                     /*
    (..,•a)•=•(1,•2);    ExpressionStatement
    (..,•a)•=•(1,•2)     ReassignmentExpression
    (..,•a)              TupleLiteral
     ..                  RangeLiteral
              (1,•2)     TupleLiteral
               1         Literal
                  2      Literal                                                                                                          */
	(..) = (3, 4);                                                                                                                        /*
    (..)•=•(3,•4);    ExpressionStatement
    (..)•=•(3,•4)     ReassignmentExpression
    (..)              TupleLiteral
     ..               RangeLiteral
           (3,•4)     TupleLiteral
            3         Literal
               4      Literal                                                                                                             */
	(((a, b)), (c)) = ((2, 3), d);                                                                                                        /*
    (((a,•b)),•(c))•=•((2,•3),•d);    ExpressionStatement
    (((a,•b)),•(c))•=•((2,•3),•d)     ReassignmentExpression
    (((a,•b)),•(c))                   TupleLiteral
      (a,•b)                          TupleLiteral
                      ((2,•3),•d)     TupleLiteral
                       (2,•3)         TupleLiteral
                        2             Literal
                           3          Literal                                                                                             */
	((a, .., b), .., (..)) = ((4, 5), ());                                                                                                /*
    ((a,•..,•b),•..,•(..))•=•((4,•5),•());    ExpressionStatement
    ((a,•..,•b),•..,•(..))•=•((4,•5),•())     ReassignmentExpression
    ((a,•..,•b),•..,•(..))                    TupleLiteral
     (a,•..,•b)                               TupleLiteral
         ..                                   RangeLiteral
                 ..                           RangeLiteral
                      ..                      RangeLiteral
                             ((4,•5),•())     TupleLiteral
                              (4,•5)          TupleLiteral
                               4              Literal
                                  5           Literal
                                      ()      TupleLiteral                                                                                */
	((a, b)) = (0, 1);                                                                                                                    /*
    ((a,•b))•=•(0,•1);    ExpressionStatement
    ((a,•b))•=•(0,•1)     ReassignmentExpression
     (a,•b)               TupleLiteral
               (0,•1)     TupleLiteral
                0         Literal
                   1      Literal                                                                                                         */
	(*foo(&mut x), *foo(&mut x)) = (5, 6);                                                                                                /*
    (*foo(&mut•x),•*foo(&mut•x))•=•(5,•6);    ExpressionStatement
    (*foo(&mut•x),•*foo(&mut•x))•=•(5,•6)     ReassignmentExpression
    (*foo(&mut•x),•*foo(&mut•x))              TupleLiteral
     *foo(&mut•x)                             DereferenceExpression
      foo(&mut•x)                             CallExpression
          &mut•x                              ReferenceExpression
                   *foo(&mut•x)               DereferenceExpression
                    foo(&mut•x)               CallExpression
                        &mut•x                ReferenceExpression
                                   (5,•6)     TupleLiteral
                                    5         Literal
                                       6      Literal                                                                                     */
	(a, _) = (8, 9);                                                                                                                      /*
    (a,•_)•=•(8,•9);    ExpressionStatement
    (a,•_)•=•(8,•9)     ReassignmentExpression
    (a,•_)              TupleLiteral
        _               UnassignedExpression
             (8,•9)     TupleLiteral
              8         Literal
                 9      Literal                                                                                                           */
	(a, .., b) = (1, 2);                                                                                                                  /*
    (a,•..,•b)•=•(1,•2);    ExpressionStatement
    (a,•..,•b)•=•(1,•2)     ReassignmentExpression
    (a,•..,•b)              TupleLiteral
        ..                  RangeLiteral
                 (1,•2)     TupleLiteral
                  1         Literal
                     2      Literal                                                                                                       */
	(a, a, b) = (1, 2);                                                                                                                   /*
    (a,•a,•b)•=•(1,•2);    ExpressionStatement
    (a,•a,•b)•=•(1,•2)     ReassignmentExpression
    (a,•a,•b)              TupleLiteral
                (1,•2)     TupleLiteral
                 1         Literal
                    2      Literal                                                                                                        */
	(a, b) += (3, 4);                                                                                                                     /*
    (a,•b)•+=•(3,•4);    ExpressionStatement
    (a,•b)•+=•(3,•4)     ReassignmentOperationExpression
    (a,•b)               TupleLiteral
              (3,•4)     TupleLiteral
               3         Literal
                  4      Literal                                                                                                          */
	(a, b) = (0, 1);                                                                                                                      /*
    (a,•b)•=•(0,•1);    ExpressionStatement
    (a,•b)•=•(0,•1)     ReassignmentExpression
    (a,•b)              TupleLiteral
             (0,•1)     TupleLiteral
              0         Literal
                 1      Literal                                                                                                           */
	(a, b) = (3, 4);                                                                                                                      /*
    (a,•b)•=•(3,•4);    ExpressionStatement
    (a,•b)•=•(3,•4)     ReassignmentExpression
    (a,•b)              TupleLiteral
             (3,•4)     TupleLiteral
              3         Literal
                 4      Literal                                                                                                           */
	(b, ..) = (5, 6, 7);                                                                                                                  /*
    (b,•..)•=•(5,•6,•7);    ExpressionStatement
    (b,•..)•=•(5,•6,•7)     ReassignmentExpression
    (b,•..)                 TupleLiteral
        ..                  RangeLiteral
              (5,•6,•7)     TupleLiteral
               5            Literal
                  6         Literal
                     7      Literal                                                                                                       */
	(b, a) = (a, b);                                                                                                                      /*
    (b,•a)•=•(a,•b);    ExpressionStatement
    (b,•a)•=•(a,•b)     ReassignmentExpression
    (b,•a)              TupleLiteral
             (a,•b)     TupleLiteral                                                                                                      */
	(C, ..) = (0,1);                                                                                                                      /*
    (C,•..)•=•(0,1);    ExpressionStatement
    (C,•..)•=•(0,1)     ReassignmentExpression
    (C,•..)             TupleLiteral
        ..              RangeLiteral
              (0,1)     TupleLiteral
               0        Literal
                 1      Literal                                                                                                           */
	(c, d) = ("c".to_owned(), "d".to_owned());                                                                                            /*
    (c,•d)•=•("c".to_owned(),•"d".to_owned());    ExpressionStatement
    (c,•d)•=•("c".to_owned(),•"d".to_owned())     ReassignmentExpression
    (c,•d)                                        TupleLiteral
             ("c".to_owned(),•"d".to_owned())     TupleLiteral
              "c".to_owned()                      CallExpression
              "c"                                 Literal
                              "d".to_owned()      CallExpression
                              "d"                 Literal                                                                                 */
	(d, c) = (c, d);                                                                                                                      /*
    (d,•c)•=•(c,•d);    ExpressionStatement
    (d,•c)•=•(c,•d)     ReassignmentExpression
    (d,•c)              TupleLiteral
             (c,•d)     TupleLiteral                                                                                                      */
	(test)() = TupleStruct(0, 0);                                                                                                         /*
    (test)()•=•TupleStruct(0,•0);    ExpressionStatement
    (test)()•=•TupleStruct(0,•0)     ReassignmentExpression
    (test)()                         CallExpression
               TupleStruct(0,•0)     CallExpression
                           0         Literal
                              0      Literal                                                                                              */
	(x, _) = (DropRecorder(3), DropRecorder(4));                                                                                          /*
    (x,•_)•=•(DropRecorder(3),•DropRecorder(4));    ExpressionStatement
    (x,•_)•=•(DropRecorder(3),•DropRecorder(4))     ReassignmentExpression
    (x,•_)                                          TupleLiteral
        _                                           UnassignedExpression
             (DropRecorder(3),•DropRecorder(4))     TupleLiteral
              DropRecorder(3)                       CallExpression
                           3                        Literal
                               DropRecorder(4)      CallExpression
                                            4       Literal                                                                               */
	[_, a, _] = [1, 2, 3];                                                                                                                /*
    [_,•a,•_]•=•[1,•2,•3];    ExpressionStatement
    [_,•a,•_]•=•[1,•2,•3]     ReassignmentExpression
    [_,•a,•_]                 ArrayLiteral
     _                        UnassignedExpression
           _                  UnassignedExpression
                [1,•2,•3]     ArrayLiteral
                 1            Literal
                    2         Literal
                       3      Literal                                                                                                     */
	[_] = [1, 2];                                                                                                                         /*
    [_]•=•[1,•2];    ExpressionStatement
    [_]•=•[1,•2]     ReassignmentExpression
    [_]              ArrayLiteral
     _               UnassignedExpression
          [1,•2]     ArrayLiteral
           1         Literal
              2      Literal                                                                                                              */
	[..] = [1, 2, 3];                                                                                                                     /*
    [..]•=•[1,•2,•3];    ExpressionStatement
    [..]•=•[1,•2,•3]     ReassignmentExpression
    [..]                 ArrayLiteral
     ..                  RangeLiteral
           [1,•2,•3]     ArrayLiteral
            1            Literal
               2         Literal
                  3      Literal                                                                                                          */
	[a, .., b, ..] = [0, 1];                                                                                                              /*
    [a,•..,•b,•..]•=•[0,•1];    ExpressionStatement
    [a,•..,•b,•..]•=•[0,•1]     ReassignmentExpression
    [a,•..,•b,•..]              ArrayLiteral
        ..                      RangeLiteral
               ..               RangeLiteral
                     [0,•1]     ArrayLiteral
                      0         Literal
                         1      Literal                                                                                                   */
	[a, .., b, c] = [1, 2, 3, 4, 5];                                                                                                      /*
    [a,•..,•b,•c]•=•[1,•2,•3,•4,•5];    ExpressionStatement
    [a,•..,•b,•c]•=•[1,•2,•3,•4,•5]     ReassignmentExpression
    [a,•..,•b,•c]                       ArrayLiteral
        ..                              RangeLiteral
                    [1,•2,•3,•4,•5]     ArrayLiteral
                     1                  Literal
                        2               Literal
                           3            Literal
                              4         Literal
                                 5      Literal                                                                                           */
	[a, a, b] = [1, 2];                                                                                                                   /*
    [a,•a,•b]•=•[1,•2];    ExpressionStatement
    [a,•a,•b]•=•[1,•2]     ReassignmentExpression
    [a,•a,•b]              ArrayLiteral
                [1,•2]     ArrayLiteral
                 1         Literal
                    2      Literal                                                                                                        */
	[a, b] += [3, 4];                                                                                                                     /*
    [a,•b]•+=•[3,•4];    ExpressionStatement
    [a,•b]•+=•[3,•4]     ReassignmentOperationExpression
    [a,•b]               ArrayLiteral
              [3,•4]     ArrayLiteral
               3         Literal
                  4      Literal                                                                                                          */
	[a, b] = [0, 1];                                                                                                                      /*
    [a,•b]•=•[0,•1];    ExpressionStatement
    [a,•b]•=•[0,•1]     ReassignmentExpression
    [a,•b]              ArrayLiteral
             [0,•1]     ArrayLiteral
              0         Literal
                 1      Literal                                                                                                           */
	[a, b] = [3, 4];                                                                                                                      /*
    [a,•b]•=•[3,•4];    ExpressionStatement
    [a,•b]•=•[3,•4]     ReassignmentExpression
    [a,•b]              ArrayLiteral
             [3,•4]     ArrayLiteral
              3         Literal
                 4      Literal                                                                                                           */
	[c, ..] = [5, 6, 6];                                                                                                                  /*
    [c,•..]•=•[5,•6,•6];    ExpressionStatement
    [c,•..]•=•[5,•6,•6]     ReassignmentExpression
    [c,•..]                 ArrayLiteral
        ..                  RangeLiteral
              [5,•6,•6]     ArrayLiteral
               5            Literal
                  6         Literal
                     6      Literal                                                                                                       */
	<Alias::<isize> as Test>::test() = TupleStruct(0, 0);                                                                                 /*
    <Alias::<isize>•as•Test>::test()•=•TupleStruct(0,•0);    ExpressionStatement
    <Alias::<isize>•as•Test>::test()•=•TupleStruct(0,•0)     ReassignmentExpression
    <Alias::<isize>•as•Test>::test()                         CallExpression
    <Alias::<isize>•as•Test>::test                           ExpressionPath
    <Alias::<isize>•as•Test>                                 ExpressionTypeSelector
     Alias::<isize>                                          TypeCall
                                       TupleStruct(0,•0)     CallExpression
                                                   0         Literal
                                                      0      Literal                                                                      */
	Alias::SingleVariant(a, b) = Alias::SingleVariant(9, 10);                                                                             /*
    Alias::SingleVariant(a,•b)•=•Alias::SingleVariant(9,•10);    ExpressionStatement
    Alias::SingleVariant(a,•b)•=•Alias::SingleVariant(9,•10)     ReassignmentExpression
    Alias::SingleVariant(a,•b)                                   CallExpression
    Alias::SingleVariant                                         ExpressionPath
                                 Alias::SingleVariant(9,•10)     CallExpression
                                 Alias::SingleVariant            ExpressionPath
                                                      9          Literal
                                                         10      Literal                                                                  */
	Enum::SingleVariant(_) = Enum::SingleVariant(1, 2);                                                                                   /*
    Enum::SingleVariant(_)•=•Enum::SingleVariant(1,•2);    ExpressionStatement
    Enum::SingleVariant(_)•=•Enum::SingleVariant(1,•2)     ReassignmentExpression
    Enum::SingleVariant(_)                                 CallExpression
    Enum::SingleVariant                                    ExpressionPath
                        _                                  UnassignedExpression
                             Enum::SingleVariant(1,•2)     CallExpression
                             Enum::SingleVariant           ExpressionPath
                                                 1         Literal
                                                    2      Literal                                                                        */
	Enum::SingleVariant(a, .., b, ..) = Enum::SingleVariant(0, 1);                                                                        /*
    Enum::SingleVariant(a,•..,•b,•..)•=•Enum::SingleVariant(0,•1);    ExpressionStatement
    Enum::SingleVariant(a,•..,•b,•..)•=•Enum::SingleVariant(0,•1)     ReassignmentExpression
    Enum::SingleVariant(a,•..,•b,•..)                                 CallExpression
    Enum::SingleVariant                                               ExpressionPath
                           ..                                         RangeLiteral
                                  ..                                  RangeLiteral
                                        Enum::SingleVariant(0,•1)     CallExpression
                                        Enum::SingleVariant           ExpressionPath
                                                            0         Literal
                                                               1      Literal                                                             */
	Enum::SingleVariant(a, a, b) = Enum::SingleVariant(1, 2);                                                                             /*
    Enum::SingleVariant(a,•a,•b)•=•Enum::SingleVariant(1,•2);    ExpressionStatement
    Enum::SingleVariant(a,•a,•b)•=•Enum::SingleVariant(1,•2)     ReassignmentExpression
    Enum::SingleVariant(a,•a,•b)                                 CallExpression
    Enum::SingleVariant                                          ExpressionPath
                                   Enum::SingleVariant(1,•2)     CallExpression
                                   Enum::SingleVariant           ExpressionPath
                                                       1         Literal
                                                          2      Literal                                                                  */
	Enum::SingleVariant(a, b) = Enum::SingleVariant(7, 8);                                                                                /*
    Enum::SingleVariant(a,•b)•=•Enum::SingleVariant(7,•8);    ExpressionStatement
    Enum::SingleVariant(a,•b)•=•Enum::SingleVariant(7,•8)     ReassignmentExpression
    Enum::SingleVariant(a,•b)                                 CallExpression
    Enum::SingleVariant                                       ExpressionPath
                                Enum::SingleVariant(7,•8)     CallExpression
                                Enum::SingleVariant           ExpressionPath
                                                    7         Literal
                                                       8      Literal                                                                     */
	S { x: a, ..s } = S { x: 3, y: 4 };                                                                                                   /*
    S•{•x:•a,•..s•}•=•S•{•x:•3,•y:•4•};    ExpressionStatement
    S•{•x:•a,•..s•}•=•S•{•x:•3,•y:•4•}     ReassignmentExpression
    S•{•x:•a,•..s•}                        StructLiteral
        x:•a                               StructLiteralProperty
              ..s                          StructLiteralPropertySpread
                      S•{•x:•3,•y:•4•}     StructLiteral
                          x:•3             StructLiteralProperty
                             3             Literal
                                y:•4       StructLiteralProperty
                                   4       Literal                                                                                        */
	S { x: a, y: b } += s;                                                                                                                /*
    S•{•x:•a,•y:•b•}•+=•s;    ExpressionStatement
    S•{•x:•a,•y:•b•}•+=•s     ReassignmentOperationExpression
    S•{•x:•a,•y:•b•}          StructLiteral
        x:•a                  StructLiteralProperty
              y:•b            StructLiteralProperty                                                                                       */
	S { x: a, y: b } = s;                                                                                                                 /*
    S•{•x:•a,•y:•b•}•=•s;    ExpressionStatement
    S•{•x:•a,•y:•b•}•=•s     ReassignmentExpression
    S•{•x:•a,•y:•b•}         StructLiteral
        x:•a                 StructLiteralProperty
              y:•b           StructLiteralProperty                                                                                        */
	Struct { .. } = Struct { a: 1, b: 4 };                                                                                                /*
    Struct•{•..•}•=•Struct•{•a:•1,•b:•4•};    ExpressionStatement
    Struct•{•..•}•=•Struct•{•a:•1,•b:•4•}     ReassignmentExpression
    Struct•{•..•}                             StructLiteral
             ..                               StructLiteralRestUnassigned
                    Struct•{•a:•1,•b:•4•}     StructLiteral
                             a:•1             StructLiteralProperty
                                1             Literal
                                   b:•4       StructLiteralProperty
                                      4       Literal                                                                                     */
	Struct { a, .. } = Struct { a: 1, b: 3 };                                                                                             /*
    Struct•{•a,•..•}•=•Struct•{•a:•1,•b:•3•};    ExpressionStatement
    Struct•{•a,•..•}•=•Struct•{•a:•1,•b:•3•}     ReassignmentExpression
    Struct•{•a,•..•}                             StructLiteral
             a                                   StructLiteralPropertyShorthand
                ..                               StructLiteralRestUnassigned
                       Struct•{•a:•1,•b:•3•}     StructLiteral
                                a:•1             StructLiteralProperty
                                   1             Literal
                                      b:•3       StructLiteralProperty
                                         3       Literal                                                                                  */
	Struct { a, .. };                                                                                                                     /*
    Struct•{•a,•..•};    ExpressionStatement
    Struct•{•a,•..•}     StructLiteral
             a           StructLiteralPropertyShorthand
                ..       StructLiteralRestUnassigned                                                                                      */
	Struct { a, ..d } = Struct { a: 1, b: 2 };                                                                                            /*
    Struct•{•a,•..d•}•=•Struct•{•a:•1,•b:•2•};    ExpressionStatement
    Struct•{•a,•..d•}•=•Struct•{•a:•1,•b:•2•}     ReassignmentExpression
    Struct•{•a,•..d•}                             StructLiteral
             a                                    StructLiteralPropertyShorthand
                ..d                               StructLiteralPropertySpread
                        Struct•{•a:•1,•b:•2•}     StructLiteral
                                 a:•1             StructLiteralProperty
                                    1             Literal
                                       b:•2       StructLiteralProperty
                                          2       Literal                                                                                 */
	Struct { a, b } = Struct { a: 0, b: 1 };                                                                                              /*
    Struct•{•a,•b•}•=•Struct•{•a:•0,•b:•1•};    ExpressionStatement
    Struct•{•a,•b•}•=•Struct•{•a:•0,•b:•1•}     ReassignmentExpression
    Struct•{•a,•b•}                             StructLiteral
             a                                  StructLiteralPropertyShorthand
                b                               StructLiteralPropertyShorthand
                      Struct•{•a:•0,•b:•1•}     StructLiteral
                               a:•0             StructLiteralProperty
                                  0             Literal
                                     b:•1       StructLiteralProperty
                                        1       Literal                                                                                   */
	Struct { a, b, c } = Struct { a: 0, b: 1 };                                                                                           /*
    Struct•{•a,•b,•c•}•=•Struct•{•a:•0,•b:•1•};    ExpressionStatement
    Struct•{•a,•b,•c•}•=•Struct•{•a:•0,•b:•1•}     ReassignmentExpression
    Struct•{•a,•b,•c•}                             StructLiteral
             a                                     StructLiteralPropertyShorthand
                b                                  StructLiteralPropertyShorthand
                   c                               StructLiteralPropertyShorthand
                         Struct•{•a:•0,•b:•1•}     StructLiteral
                                  a:•0             StructLiteralProperty
                                     0             Literal
                                        b:•1       StructLiteralProperty
                                           1       Literal                                                                                */
	Struct { a: _, b } = Struct { a: 1, b: 2 };                                                                                           /*
    Struct•{•a:•_,•b•}•=•Struct•{•a:•1,•b:•2•};    ExpressionStatement
    Struct•{•a:•_,•b•}•=•Struct•{•a:•1,•b:•2•}     ReassignmentExpression
    Struct•{•a:•_,•b•}                             StructLiteral
             a:•_                                  StructLiteralProperty
                _                                  UnassignedExpression
                   b                               StructLiteralPropertyShorthand
                         Struct•{•a:•1,•b:•2•}     StructLiteral
                                  a:•1             StructLiteralProperty
                                     1             Literal
                                        b:•2       StructLiteralProperty
                                           2       Literal                                                                                */
	Struct { a: b, b: a }  = Struct { a: 1, b: 2 };                                                                                       /*
    Struct•{•a:•b,•b:•a•}••=•Struct•{•a:•1,•b:•2•};    ExpressionStatement
    Struct•{•a:•b,•b:•a•}••=•Struct•{•a:•1,•b:•2•}     ReassignmentExpression
    Struct•{•a:•b,•b:•a•}                              StructLiteral
             a:•b                                      StructLiteralProperty
                   b:•a                                StructLiteralProperty
                             Struct•{•a:•1,•b:•2•}     StructLiteral
                                      a:•1             StructLiteralProperty
                                         1             Literal
                                            b:•2       StructLiteralProperty
                                               2       Literal                                                                            */
	Struct { a: TupleStruct((a, b), c), b: [d] } = Struct { a: TupleStruct((0, 1), 2), b: [3] };                                          /*
    Struct•{•a:•TupleStruct((a,•b),•c),•b:•[d]•}•=•Struct•{•a:•TupleStruct((0,•1),•2),•b:•[3]•};    ExpressionStatement
    Struct•{•a:•TupleStruct((a,•b),•c),•b:•[d]•}•=•Struct•{•a:•TupleStruct((0,•1),•2),•b:•[3]•}     ReassignmentExpression
    Struct•{•a:•TupleStruct((a,•b),•c),•b:•[d]•}                                                    StructLiteral
             a:•TupleStruct((a,•b),•c)                                                              StructLiteralProperty
                TupleStruct((a,•b),•c)                                                              CallExpression
                            (a,•b)                                                                  TupleLiteral
                                        b:•[d]                                                      StructLiteralProperty
                                           [d]                                                      ArrayLiteral
                                                   Struct•{•a:•TupleStruct((0,•1),•2),•b:•[3]•}     StructLiteral
                                                            a:•TupleStruct((0,•1),•2)               StructLiteralProperty
                                                               TupleStruct((0,•1),•2)               CallExpression
                                                                           (0,•1)                   TupleLiteral
                                                                            0                       Literal
                                                                               1                    Literal
                                                                                   2                Literal
                                                                                       b:•[3]       StructLiteralProperty
                                                                                          [3]       ArrayLiteral
                                                                                           3        Literal                               */
	test() = TupleStruct(0, 0);                                                                                                           /*
    test()•=•TupleStruct(0,•0);    ExpressionStatement
    test()•=•TupleStruct(0,•0)     ReassignmentExpression
    test()                         CallExpression
             TupleStruct(0,•0)     CallExpression
                         0         Literal
                            0      Literal                                                                                                */
	TupleStruct(_, a) = TupleStruct(2, 2);                                                                                                /*
    TupleStruct(_,•a)•=•TupleStruct(2,•2);    ExpressionStatement
    TupleStruct(_,•a)•=•TupleStruct(2,•2)     ReassignmentExpression
    TupleStruct(_,•a)                         CallExpression
                _                             UnassignedExpression
                        TupleStruct(2,•2)     CallExpression
                                    2         Literal
                                       2      Literal                                                                                     */
	TupleStruct(_) = TupleStruct(1, 2);                                                                                                   /*
    TupleStruct(_)•=•TupleStruct(1,•2);    ExpressionStatement
    TupleStruct(_)•=•TupleStruct(1,•2)     ReassignmentExpression
    TupleStruct(_)                         CallExpression
                _                          UnassignedExpression
                     TupleStruct(1,•2)     CallExpression
                                 1         Literal
                                    2      Literal                                                                                        */
	TupleStruct(..) = TupleStruct(3, 4);                                                                                                  /*
    TupleStruct(..)•=•TupleStruct(3,•4);    ExpressionStatement
    TupleStruct(..)•=•TupleStruct(3,•4)     ReassignmentExpression
    TupleStruct(..)                         CallExpression
                ..                          RangeLiteral
                      TupleStruct(3,•4)     CallExpression
                                  3         Literal
                                     4      Literal                                                                                       */
	TupleStruct(5,6).assign(&mut a, &mut b);                                                                                              /*
    TupleStruct(5,6).assign(&mut•a,•&mut•b);    ExpressionStatement
    TupleStruct(5,6).assign(&mut•a,•&mut•b)     CallExpression
    TupleStruct(5,6)                            CallExpression
                5                               Literal
                  6                             Literal
                            &mut•a              ReferenceExpression
                                    &mut•b      ReferenceExpression                                                                       */
	TupleStruct(a, .., b, ..) = TupleStruct(0, 1);                                                                                        /*
    TupleStruct(a,•..,•b,•..)•=•TupleStruct(0,•1);    ExpressionStatement
    TupleStruct(a,•..,•b,•..)•=•TupleStruct(0,•1)     ReassignmentExpression
    TupleStruct(a,•..,•b,•..)                         CallExpression
                   ..                                 RangeLiteral
                          ..                          RangeLiteral
                                TupleStruct(0,•1)     CallExpression
                                            0         Literal
                                               1      Literal                                                                             */
	TupleStruct(a, .., b) = TupleStruct(1, 2);                                                                                            /*
    TupleStruct(a,•..,•b)•=•TupleStruct(1,•2);    ExpressionStatement
    TupleStruct(a,•..,•b)•=•TupleStruct(1,•2)     ReassignmentExpression
    TupleStruct(a,•..,•b)                         CallExpression
                   ..                             RangeLiteral
                            TupleStruct(1,•2)     CallExpression
                                        1         Literal
                                           2      Literal                                                                                 */
	TupleStruct(a, a, b) = TupleStruct(1, 2);                                                                                             /*
    TupleStruct(a,•a,•b)•=•TupleStruct(1,•2);    ExpressionStatement
    TupleStruct(a,•a,•b)•=•TupleStruct(1,•2)     ReassignmentExpression
    TupleStruct(a,•a,•b)                         CallExpression
                           TupleStruct(1,•2)     CallExpression
                                       1         Literal
                                          2      Literal                                                                                  */
	TupleStruct(a, b) = TupleStruct(0, 1);                                                                                                /*
    TupleStruct(a,•b)•=•TupleStruct(0,•1);    ExpressionStatement
    TupleStruct(a,•b)•=•TupleStruct(0,•1)     ReassignmentExpression
    TupleStruct(a,•b)                         CallExpression
                        TupleStruct(0,•1)     CallExpression
                                    0         Literal
                                       1      Literal                                                                                     */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

// Discarded Nodes: 5
// Parsed Nodes: 666
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 2562 (26% re-reads)
// Unnecessary 'skip_whitespace()' calls: 339
// source: "../../samples/features/destructuring_assignment.rs"