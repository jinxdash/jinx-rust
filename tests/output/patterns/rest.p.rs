
fn b() {                                                                                                                                  /*
fn•b()•{↲    <Program>
fn•b()•{↲    <Program.ast{dk: "None"}>
fn•b()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    // fn foo(..: u8) {}
    //•fn•foo(..:•u8)•{}    Comment{line}
    let ..;                                                                                                                               /*
    let•..;    LetVariableDeclaration
        ..     RangePattern{!last}                                                                                                        */
    let box ..;                                                                                                                           /*
    let•box•..;    LetVariableDeclaration
        box•..     BoxPattern
            ..     RangePattern{!last}                                                                                                    */
    match x { .. | .. => {} }                                                                                                             /*
    match•x•{•..•|•..•=>•{}•}    ExpressionStatement{!semi}, MatchExpression
            {•..•|•..•=>•{}•}    MatchExpression.cases{dk: "{}"}
              ..•|•..•=>•{}      MatchExpressionCase
              ..•|•..            UnionPattern
              ..                 RangePattern{!last}
                   ..            RangePattern{!last}
                         {}      BlockExpression                                                                                          */
    let &..;                                                                                                                              /*
    let•&..;    LetVariableDeclaration
        &..     ReferencePattern{!mut}
         ..     RangePattern{!last}                                                                                                       */
    let &mut ..;                                                                                                                          /*
    let•&mut•..;    LetVariableDeclaration
        &mut•..     ReferencePattern{mut}
             ..     RangePattern{!last}                                                                                                   */
    let x @ ..;                                                                                                                           /*
    let•x•@•..;    LetVariableDeclaration
        x•@•..     PatternVariableDeclaration{!ref, !mut}
            ..     RangePattern{!last}                                                                                                    */
    let ref x @ ..;                                                                                                                       /*
    let•ref•x•@•..;    LetVariableDeclaration
        ref•x•@•..     PatternVariableDeclaration{ref, !mut}
                ..     RangePattern{!last}                                                                                                */
    let ref mut x @ ..;                                                                                                                   /*
    let•ref•mut•x•@•..;    LetVariableDeclaration
        ref•mut•x•@•..     PatternVariableDeclaration{ref, mut}
                    ..     RangePattern{!last}                                                                                            */
    let (..);                                                                                                                             /*
    let•(..);    LetVariableDeclaration
        (..)     TuplePattern
         ..      RestPattern                                                                                                              */
    let (..,);                                                                                                                            /*
    let•(..,);    LetVariableDeclaration
        (..,)     TuplePattern
         ..       RestPattern                                                                                                             */
    let (.., .., ..);                                                                                                                     /*
    let•(..,•..,•..);    LetVariableDeclaration
        (..,•..,•..)     TuplePattern
         ..              RestPattern
             ..          RestPattern
                 ..      RestPattern                                                                                                      */
    let (.., P, ..);                                                                                                                      /*
    let•(..,•P,•..);    LetVariableDeclaration
        (..,•P,•..)     TuplePattern
         ..             RestPattern
                ..      RestPattern                                                                                                       */
    let A(..);                                                                                                                            /*
    let•A(..);    LetVariableDeclaration
        A(..)     TuplePattern
         (..)     TuplePattern.items{dk: "()"}
          ..      RestPattern                                                                                                             */
    let A(..,);                                                                                                                           /*
    let•A(..,);    LetVariableDeclaration
        A(..,)     TuplePattern
         (..,)     TuplePattern.items{dk: "()"}
          ..       RestPattern                                                                                                            */
    let A(.., .., ..);                                                                                                                    /*
    let•A(..,•..,•..);    LetVariableDeclaration
        A(..,•..,•..)     TuplePattern
         (..,•..,•..)     TuplePattern.items{dk: "()"}
          ..              RestPattern
              ..          RestPattern
                  ..      RestPattern                                                                                                     */
    let A(.., P, ..);                                                                                                                     /*
    let•A(..,•P,•..);    LetVariableDeclaration
        A(..,•P,•..)     TuplePattern
         (..,•P,•..)     TuplePattern.items{dk: "()"}
          ..             RestPattern
                 ..      RestPattern                                                                                                      */
    let [..];                                                                                                                             /*
    let•[..];    LetVariableDeclaration
        [..]     ArrayPattern
         ..      RestPattern                                                                                                              */
    let [..,];                                                                                                                            /*
    let•[..,];    LetVariableDeclaration
        [..,]     ArrayPattern
         ..       RestPattern                                                                                                             */
    let [.., .., ..];                                                                                                                     /*
    let•[..,•..,•..];    LetVariableDeclaration
        [..,•..,•..]     ArrayPattern
         ..              RestPattern
             ..          RestPattern
                 ..      RestPattern                                                                                                      */
    let [.., P, ..];                                                                                                                      /*
    let•[..,•P,•..];    LetVariableDeclaration
        [..,•P,•..]     ArrayPattern
         ..             RestPattern
                ..      RestPattern                                                                                                       */
    match x {                                                                                                                             /*
    match•x•{↲    <ExpressionStatement{!semi}>
    match•x•{↲    <MatchExpression>
            {↲    <MatchExpression.cases{dk: "{}"}>                                                                                       */
        .. | [(box .., &(..), &mut .., x @ ..), ref x @ ..,] | ref mut x @ .. if x[..] => {}                                              /*
        ..•|•[(box•..,•&(..),•&mut•..,•x•@•..),•ref•x•@•..,]•|•ref•mut•x•@•..•if•x[..]•=>•{}    MatchExpressionCase
        ..•|•[(box•..,•&(..),•&mut•..,•x•@•..),•ref•x•@•..,]•|•ref•mut•x•@•..                   UnionPattern
        ..                                                                                      RangePattern{!last}
             [(box•..,•&(..),•&mut•..,•x•@•..),•ref•x•@•..,]                                    ArrayPattern
              (box•..,•&(..),•&mut•..,•x•@•..)                                                  TuplePattern
               box•..                                                                           BoxPattern
                   ..                                                                           RestPattern
                       &(..)                                                                    ReferencePattern{!mut}
                        (..)                                                                    TuplePattern
                         ..                                                                     RestPattern
                              &mut•..                                                           ReferencePattern{mut}
                                   ..                                                           RestPattern
                                       x•@•..                                                   PatternVariableDeclaration{!ref, !mut}
                                           ..                                                   RestPattern
                                                ref•x•@•..                                      PatternVariableDeclaration{ref, !mut}
                                                        ..                                      RestPattern
                                                               ref•mut•x•@•..                   PatternVariableDeclaration{ref, mut}
                                                                           ..                   RangePattern{!last}
                                                                                 x[..]          MemberExpression{computed}
                                                                                   ..           RangeLiteral{!last}
                                                                                          {}    BlockExpression                           */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
    a!(..);                                                                                                                               /*
    a!(..);    ExpressionStatement{semi}
    a!(..)     MacroInvocation
      (..)     MacroInvocation.segments{dk: "()"}
       ..      PunctuationToken{tk: ".."}                                                                                                 */

	let [..]: &[u8];                                                                                                                      /*
	let•[..]:•&[u8];    LetVariableDeclaration
	    [..]            ArrayPattern
	     ..             RestPattern
	          &[u8]     TypeReference{!mut}
	           [u8]     TypeSlice                                                                                                         */
	let [..,]: &[u8];                                                                                                                     /*
	let•[..,]:•&[u8];    LetVariableDeclaration
	    [..,]            ArrayPattern
	     ..              RestPattern
	           &[u8]     TypeReference{!mut}
	            [u8]     TypeSlice                                                                                                        */
	let (..): (u8,);                                                                                                                      /*
	let•(..):•(u8,);    LetVariableDeclaration
	    (..)            TuplePattern
	     ..             RestPattern
	          (u8,)     TypeTuple                                                                                                         */
	let (..,): (u8,);                                                                                                                     /*
	let•(..,):•(u8,);    LetVariableDeclaration
	    (..,)            TuplePattern
	     ..              RestPattern
	           (u8,)     TypeTuple                                                                                                        */
    
	let (1, (Some(1), 2..=3)) = (1, (None, 2));                                                                                           /*
	let•(1,•(Some(1),•2..=3))•=•(1,•(None,•2));    LetVariableDeclaration
	    (1,•(Some(1),•2..=3))                      TuplePattern
	     1                                         Literal{kind: Integer}
	        (Some(1),•2..=3)                       TuplePattern
	         Some(1)                               TuplePattern
	             (1)                               TuplePattern.items{dk: "()"}
	              1                                Literal{kind: Integer}
	                  2..=3                        RangePattern{last}
	                  2                            Literal{kind: Integer}
	                      3                        Literal{kind: Integer}
	                            (1,•(None,•2))     TupleLiteral
	                             1                 Literal{kind: Integer}
	                                (None,•2)      TupleLiteral
	                                       2       Literal{kind: Integer}                                                                 */
	fn func((1, (Some(1), 2..=3)): (isize, (Option<isize>, isize))) {}                                                                    /*
	fn•func((1,•(Some(1),•2..=3)):•(isize,•(Option<isize>,•isize)))•{}    FunctionDeclaration
	       ((1,•(Some(1),•2..=3)):•(isize,•(Option<isize>,•isize)))       FunctionDeclaration.parameters{dk: "()"}
	        (1,•(Some(1),•2..=3)):•(isize,•(Option<isize>,•isize))        FunctionParameterDeclaration
	        (1,•(Some(1),•2..=3))                                         TuplePattern
	         1                                                            Literal{kind: Integer}
	            (Some(1),•2..=3)                                          TuplePattern
	             Some(1)                                                  TuplePattern
	                 (1)                                                  TuplePattern.items{dk: "()"}
	                  1                                                   Literal{kind: Integer}
	                      2..=3                                           RangePattern{last}
	                      2                                               Literal{kind: Integer}
	                          3                                           Literal{kind: Integer}
	                               (isize,•(Option<isize>,•isize))        TypeTuple
	                                       (Option<isize>,•isize)         TypeTuple
	                                        Option<isize>                 TypeCall
	                                              <isize>                 TypeCall.typeArguments{dk: "<>"}
	                                                                {}    FunctionDeclaration.body{dk: "{}"}                              */
	fn fun([a, ref mut b, ref xs @ .., ref c, d]: [X; 6]) {}                                                                              /*
	fn•fun([a,•ref•mut•b,•ref•xs•@•..,•ref•c,•d]:•[X;•6])•{}    FunctionDeclaration
	      ([a,•ref•mut•b,•ref•xs•@•..,•ref•c,•d]:•[X;•6])       FunctionDeclaration.parameters{dk: "()"}
	       [a,•ref•mut•b,•ref•xs•@•..,•ref•c,•d]:•[X;•6]        FunctionParameterDeclaration
	       [a,•ref•mut•b,•ref•xs•@•..,•ref•c,•d]                ArrayPattern
	           ref•mut•b                                        PatternVariableDeclaration{ref, mut}
	                      ref•xs•@•..                           PatternVariableDeclaration{ref, !mut}
	                               ..                           RestPattern
	                                   ref•c                    PatternVariableDeclaration{ref, !mut}
	                                              [X;•6]        TypeSizedArray
	                                                  6         Literal{kind: Integer}
	                                                      {}    FunctionDeclaration.body{dk: "{}"}                                        */
	fn foo(a @ [b, mid @ .., c]: [C; 3]) {}                                                                                               /*
	fn•foo(a•@•[b,•mid•@•..,•c]:•[C;•3])•{}    FunctionDeclaration
	      (a•@•[b,•mid•@•..,•c]:•[C;•3])       FunctionDeclaration.parameters{dk: "()"}
	       a•@•[b,•mid•@•..,•c]:•[C;•3]        FunctionParameterDeclaration
	       a•@•[b,•mid•@•..,•c]                PatternVariableDeclaration{!ref, !mut}
	           [b,•mid•@•..,•c]                ArrayPattern
	               mid•@•..                    PatternVariableDeclaration{!ref, !mut}
	                     ..                    RestPattern
	                             [C;•3]        TypeSizedArray
	                                 3         Literal{kind: Integer}
	                                     {}    FunctionDeclaration.body{dk: "{}"}                                                         */
    
    let [..] = s;                                                                                                                         /*
    let•[..]•=•s;    LetVariableDeclaration
        [..]         ArrayPattern
         ..          RestPattern                                                                                                          */
    let [..] = s0;                                                                                                                        /*
    let•[..]•=•s0;    LetVariableDeclaration
        [..]          ArrayPattern
         ..           RestPattern                                                                                                         */
    let [..] = s1;                                                                                                                        /*
    let•[..]•=•s1;    LetVariableDeclaration
        [..]          ArrayPattern
         ..           RestPattern                                                                                                         */
    let [..] = s2;                                                                                                                        /*
    let•[..]•=•s2;    LetVariableDeclaration
        [..]          ArrayPattern
         ..           RestPattern                                                                                                         */
    let [_, ..] = s1;                                                                                                                     /*
    let•[_,•..]•=•s1;    LetVariableDeclaration
        [_,•..]          ArrayPattern
         _               WildcardPattern
            ..           RestPattern                                                                                                      */
    let [.., _] = s1;                                                                                                                     /*
    let•[..,•_]•=•s1;    LetVariableDeclaration
        [..,•_]          ArrayPattern
         ..              RestPattern
             _           WildcardPattern                                                                                                  */
    let [_, ..] = s2;                                                                                                                     /*
    let•[_,•..]•=•s2;    LetVariableDeclaration
        [_,•..]          ArrayPattern
         _               WildcardPattern
            ..           RestPattern                                                                                                      */
    let [.., _] = s2;                                                                                                                     /*
    let•[..,•_]•=•s2;    LetVariableDeclaration
        [..,•_]          ArrayPattern
         ..              RestPattern
             _           WildcardPattern                                                                                                  */
    let [_, _, ..] = s2;                                                                                                                  /*
    let•[_,•_,•..]•=•s2;    LetVariableDeclaration
        [_,•_,•..]          ArrayPattern
         _                  WildcardPattern
            _               WildcardPattern
               ..           RestPattern                                                                                                   */
    let [_, .., _] = s2;                                                                                                                  /*
    let•[_,•..,•_]•=•s2;    LetVariableDeclaration
        [_,•..,•_]          ArrayPattern
         _                  WildcardPattern
            ..              RestPattern
                _           WildcardPattern                                                                                               */
    let [.., _, _] = s2;                                                                                                                  /*
    let•[..,•_,•_]•=•s2;    LetVariableDeclaration
        [..,•_,•_]          ArrayPattern
         ..                 RestPattern
             _              WildcardPattern
                _           WildcardPattern                                                                                               */
	let Box { 0: _, .. }: Box<()>;                                                                                                        /*
	let•Box•{•0:•_,•..•}:•Box<()>;    LetVariableDeclaration
	    Box•{•0:•_,•..•}              StructPattern
	        {•0:•_,•..•}              StructPattern.properties{dk: "{}"}
	          0:•_                    StructPatternPropertyDestructured
	          0                       Index
	             _                    WildcardPattern
	                ..                RestPattern
	                      Box<()>     TypeCall
	                         <()>     TypeCall.typeArguments{dk: "<>"}
	                          ()      TypeTuple                                                                                           */
	let Box { 1: _, .. }: Box<()>;                                                                                                        /*
	let•Box•{•1:•_,•..•}:•Box<()>;    LetVariableDeclaration
	    Box•{•1:•_,•..•}              StructPattern
	        {•1:•_,•..•}              StructPattern.properties{dk: "{}"}
	          1:•_                    StructPatternPropertyDestructured
	          1                       Index
	             _                    WildcardPattern
	                ..                RestPattern
	                      Box<()>     TypeCall
	                         <()>     TypeCall.typeArguments{dk: "<>"}
	                          ()      TypeTuple                                                                                           */

    
	let [ref _x0, _x1, _, mut _x3, .., ref _x6, _x7] = arr;                                                                               /*
	let•[ref•_x0,•_x1,•_,•mut•_x3,•..,•ref•_x6,•_x7]•=•arr;    LetVariableDeclaration
	    [ref•_x0,•_x1,•_,•mut•_x3,•..,•ref•_x6,•_x7]           ArrayPattern
	     ref•_x0                                               PatternVariableDeclaration{ref, !mut}
	                   _                                       WildcardPattern
	                      mut•_x3                              PatternVariableDeclaration{!ref, mut}
	                               ..                          RestPattern
	                                   ref•_x6                 PatternVariableDeclaration{ref, !mut}                                      */
	let [ref _x0, ..] = arr;                                                                                                              /*
	let•[ref•_x0,•..]•=•arr;    LetVariableDeclaration
	    [ref•_x0,•..]           ArrayPattern
	     ref•_x0                PatternVariableDeclaration{ref, !mut}
	              ..            RestPattern                                                                                               */
	let [_x0, ..] = arr;                                                                                                                  /*
	let•[_x0,•..]•=•arr;    LetVariableDeclaration
	    [_x0,•..]           ArrayPattern
	          ..            RestPattern                                                                                                   */
	let (.., ref mut _x3) = tup;                                                                                                          /*
	let•(..,•ref•mut•_x3)•=•tup;    LetVariableDeclaration
	    (..,•ref•mut•_x3)           TuplePattern
	     ..                         RestPattern
	         ref•mut•_x3            PatternVariableDeclaration{ref, mut}                                                                  */
	let a @ [b, .., c] = [C, mk_c(), C];                                                                                                  /*
	let•a•@•[b,•..,•c]•=•[C,•mk_c(),•C];    LetVariableDeclaration
	    a•@•[b,•..,•c]                      PatternVariableDeclaration{!ref, !mut}
	        [b,•..,•c]                      ArrayPattern
	            ..                          RestPattern
	                     [C,•mk_c(),•C]     ArrayLiteral
	                         mk_c()         CallExpression
	                             ()         CallExpression.arguments{dk: "()"}                                                            */
	let a @ [b, mid @ .., c] = [C, mk_c(), C];                                                                                            /*
	let•a•@•[b,•mid•@•..,•c]•=•[C,•mk_c(),•C];    LetVariableDeclaration
	    a•@•[b,•mid•@•..,•c]                      PatternVariableDeclaration{!ref, !mut}
	        [b,•mid•@•..,•c]                      ArrayPattern
	            mid•@•..                          PatternVariableDeclaration{!ref, !mut}
	                  ..                          RestPattern
	                           [C,•mk_c(),•C]     ArrayLiteral
	                               mk_c()         CallExpression
	                                   ()         CallExpression.arguments{dk: "()"}                                                      */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 336
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 1699 (25% re-reads)
// Unnecessary 'skip_whitespace()' calls: 188
// source: "../../samples/patterns/rest.rs"