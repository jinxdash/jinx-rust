fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>                                                                                                     */
	let _: &'static _ = &|| { let _ = 0; 0 };                                                                                             /*
    let•_:•&'static•_•=•&||•{•let•_•=•0;•0•};    LetVariableDeclaration
        _                                        WildcardPattern
           &'static•_                            TypeReference
            'static                              LtStatic
                    _                            TypeInferred
                        &||•{•let•_•=•0;•0•}     ReferenceExpression
                         ||•{•let•_•=•0;•0•}     ClosureFunctionExpression
                            {•let•_•=•0;•0•}     BlockExpression
                              let•_•=•0;         LetVariableDeclaration
                                  _              WildcardPattern
                                      0          Literal
                                         0       ExpressionStatement, Literal                                                             */
	let _ = match c(o.m(), o as T::T) {                                                                                                   /*
    let•_•=•match•c(o.m(),•o•as•T::T)•{↲    <LetVariableDeclaration>
        _                                   WildcardPattern
            match•c(o.m(),•o•as•T::T)•{↲    <ExpressionAsTypeCast>, <MatchExpression>
                  c(o.m(),•o•as•T::T)       CallExpression
                    o.m()                   CallExpression
                           o•as•T::T        ExpressionAsTypeCast
                                T::T        TypePath                                                                                      */
		0 if o::c() == 0 => 0,                                                                                                            /*
        0•if•o::c()•==•0•=>•0    MatchExpressionCase
        0                        Literal
             o::c()•==•0         ComparisonExpression
             o::c()              CallExpression
             o::c                ExpressionPath
                       0         Literal
                            0    Literal                                                                                                  */
		0 => return c(o::c()),                                                                                                            /*
        0•=>•return•c(o::c())    MatchExpressionCase
        0                        Literal
             return•c(o::c())    ReturnExpression
                    c(o::c())    CallExpression
                      o::c()     CallExpression
                      o::c       ExpressionPath                                                                                           */
	} as T;                                                                                                                               /*
   ╚}•as•T;    </LetVariableDeclaration>
   ╚}•as•T     </ExpressionAsTypeCast>
   ╚}          </MatchExpression>                                                                                                         */

	let _ = if  0 == 0 && 0 == 0 						{ 	0 == 0 && 0 == 0 						} else { 0 }                      /*
    let•_•=•if••0•==•0•&&•0•==•0•╚╚╚╚╚╚{•╚0•==•0•&&•0•==•0•╚╚╚╚╚╚}•else•{•0•}                                           LetVariableDeclaration
        _                                                                                                               WildcardPattern
            if••0•==•0•&&•0•==•0•╚╚╚╚╚╚{•╚0•==•0•&&•0•==•0•╚╚╚╚╚╚}•else•{•0•}                                           IfBlockExpression
                0•==•0•&&•0•==•0                                                                                        AndExpression
                0•==•0                                                                                                  ComparisonExpression
                0                                                                                                       Literal
                     0                                                                                                  Literal
                          0•==•0                                                                                        ComparisonExpression
                          0                                                                                             Literal
                               0                                                                                        Literal
                                                               0•==•0•&&•0•==•0                                         ExpressionStatement, AndExpression
                                                               0•==•0                                                   ComparisonExpression
                                                               0                                                        Literal
                                                                    0                                                   Literal
                                                                         0•==•0                                         ComparisonExpression
                                                                         0                                              Literal
                                                                              0                                         Literal
                                                                                                               {•0•}    BlockExpression
                                                                                                                 0      ExpressionStatement, Literal*/
	let _ = if  0 == 0 || 0 == 0 						{	0 == 0 || 0 == 0 						} else { 0 }                       /*
    let•_•=•if••0•==•0•||•0•==•0•╚╚╚╚╚╚{╚0•==•0•||•0•==•0•╚╚╚╚╚╚}•else•{•0•}                                           LetVariableDeclaration
        _                                                                                                              WildcardPattern
            if••0•==•0•||•0•==•0•╚╚╚╚╚╚{╚0•==•0•||•0•==•0•╚╚╚╚╚╚}•else•{•0•}                                           IfBlockExpression
                0•==•0•||•0•==•0                                                                                       OrExpression
                0•==•0                                                                                                 ComparisonExpression
                0                                                                                                      Literal
                     0                                                                                                 Literal
                          0•==•0                                                                                       ComparisonExpression
                          0                                                                                            Literal
                               0                                                                                       Literal
                                                              0•==•0•||•0•==•0                                         ExpressionStatement, OrExpression
                                                              0•==•0                                                   ComparisonExpression
                                                              0                                                        Literal
                                                                   0                                                   Literal
                                                                        0•==•0                                         ComparisonExpression
                                                                        0                                              Literal
                                                                             0                                         Literal
                                                                                                              {•0•}    BlockExpression
                                                                                                                0      ExpressionStatement, Literal*/
	let _ = if (0 == 0 || 0 == 0) && (0 == 0 || 0 == 0) {  (0 == 0 || 0 == 0) && (0 == 0 || 0 == 0) } else { 0 }                          /*
    let•_•=•if•(0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)•{••(0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)•}•else•{•0•}    LetVariableDeclaration
        _                                                                                                           WildcardPattern
            if•(0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)•{••(0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)•}•else•{•0•}    IfBlockExpression
               (0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)                                                             AndExpression
                0•==•0•||•0•==•0                                                                                    OrExpression
                0•==•0                                                                                              ComparisonExpression
                0                                                                                                   Literal
                     0                                                                                              Literal
                          0•==•0                                                                                    ComparisonExpression
                          0                                                                                         Literal
                               0                                                                                    Literal
                                      0•==•0•||•0•==•0                                                              OrExpression
                                      0•==•0                                                                        ComparisonExpression
                                      0                                                                             Literal
                                           0                                                                        Literal
                                                0•==•0                                                              ComparisonExpression
                                                0                                                                   Literal
                                                     0                                                              Literal
                                                           (0•==•0•||•0•==•0)•&&•(0•==•0•||•0•==•0)                 ExpressionStatement, AndExpression
                                                            0•==•0•||•0•==•0                                        OrExpression
                                                            0•==•0                                                  ComparisonExpression
                                                            0                                                       Literal
                                                                 0                                                  Literal
                                                                      0•==•0                                        ComparisonExpression
                                                                      0                                             Literal
                                                                           0                                        Literal
                                                                                  0•==•0•||•0•==•0                  OrExpression
                                                                                  0•==•0                            ComparisonExpression
                                                                                  0                                 Literal
                                                                                       0                            Literal
                                                                                            0•==•0                  ComparisonExpression
                                                                                            0                       Literal
                                                                                                 0                  Literal
                                                                                                           {•0•}    BlockExpression
                                                                                                             0      ExpressionStatement, Literal*/
	let _ = if  0 == 0 && 0 == 0  && (0 == 0 || 0 == 0) { 	0 == 0 && 0 == 0  && (0 == 0 || 0 == 0) } else { 0 }                        /*
    let•_•=•if••0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)•{•╚0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)•}•else•{•0•}       LetVariableDeclaration
        _                                                                                                             WildcardPattern
            if••0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)•{•╚0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)•}•else•{•0•}       IfBlockExpression
                0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)                                                               AndExpression
                0•==•0•&&•0•==•0                                                                                      AndExpression
                0•==•0                                                                                                ComparisonExpression
                0                                                                                                     Literal
                     0                                                                                                Literal
                          0•==•0                                                                                      ComparisonExpression
                          0                                                                                           Literal
                               0                                                                                      Literal
                                      0•==•0•||•0•==•0                                                                OrExpression
                                      0•==•0                                                                          ComparisonExpression
                                      0                                                                               Literal
                                           0                                                                          Literal
                                                0•==•0                                                                ComparisonExpression
                                                0                                                                     Literal
                                                     0                                                                Literal
                                                              0•==•0•&&•0•==•0••&&•(0•==•0•||•0•==•0)                 ExpressionStatement, AndExpression
                                                              0•==•0•&&•0•==•0                                        AndExpression
                                                              0•==•0                                                  ComparisonExpression
                                                              0                                                       Literal
                                                                   0                                                  Literal
                                                                        0•==•0                                        ComparisonExpression
                                                                        0                                             Literal
                                                                             0                                        Literal
                                                                                    0•==•0•||•0•==•0                  OrExpression
                                                                                    0•==•0                            ComparisonExpression
                                                                                    0                                 Literal
                                                                                         0                            Literal
                                                                                              0•==•0                  ComparisonExpression
                                                                                              0                       Literal
                                                                                                   0                  Literal
                                                                                                             {•0•}    BlockExpression
                                                                                                               0      ExpressionStatement, Literal*/
	let _ = if (0 == 0 || 0 == 0) &&  0 == 0 && 0 == 0  {  (0 == 0 || 0 == 0) &&  0 == 0 && 0 == 0  } else { 0 }                          /*
    let•_•=•if•(0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0••{••(0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0••}•else•{•0•}    LetVariableDeclaration
        _                                                                                                           WildcardPattern
            if•(0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0••{••(0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0••}•else•{•0•}    IfBlockExpression
               (0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0                                                              AndExpression
               (0•==•0•||•0•==•0)•&&••0•==•0                                                                        AndExpression
                0•==•0•||•0•==•0                                                                                    OrExpression
                0•==•0                                                                                              ComparisonExpression
                0                                                                                                   Literal
                     0                                                                                              Literal
                          0•==•0                                                                                    ComparisonExpression
                          0                                                                                         Literal
                               0                                                                                    Literal
                                      0•==•0                                                                        ComparisonExpression
                                      0                                                                             Literal
                                           0                                                                        Literal
                                                0•==•0                                                              ComparisonExpression
                                                0                                                                   Literal
                                                     0                                                              Literal
                                                           (0•==•0•||•0•==•0)•&&••0•==•0•&&•0•==•0                  ExpressionStatement, AndExpression
                                                           (0•==•0•||•0•==•0)•&&••0•==•0                            AndExpression
                                                            0•==•0•||•0•==•0                                        OrExpression
                                                            0•==•0                                                  ComparisonExpression
                                                            0                                                       Literal
                                                                 0                                                  Literal
                                                                      0•==•0                                        ComparisonExpression
                                                                      0                                             Literal
                                                                           0                                        Literal
                                                                                  0•==•0                            ComparisonExpression
                                                                                  0                                 Literal
                                                                                       0                            Literal
                                                                                            0•==•0                  ComparisonExpression
                                                                                            0                       Literal
                                                                                                 0                  Literal
                                                                                                           {•0•}    BlockExpression
                                                                                                             0      ExpressionStatement, Literal*/
	let _ = if  0 == 0 && 0 == 0  &&  0 == 0 && 0 == 0  { 	0 == 0 && 0 == 0  &&  0 == 0 && 0 == 0  } else { 0 }                        /*
    let•_•=•if••0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0••{•╚0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0••}•else•{•0•}       LetVariableDeclaration
        _                                                                                                             WildcardPattern
            if••0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0••{•╚0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0••}•else•{•0•}       IfBlockExpression
                0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0                                                                AndExpression
                0•==•0•&&•0•==•0••&&••0•==•0                                                                          AndExpression
                0•==•0•&&•0•==•0                                                                                      AndExpression
                0•==•0                                                                                                ComparisonExpression
                0                                                                                                     Literal
                     0                                                                                                Literal
                          0•==•0                                                                                      ComparisonExpression
                          0                                                                                           Literal
                               0                                                                                      Literal
                                      0•==•0                                                                          ComparisonExpression
                                      0                                                                               Literal
                                           0                                                                          Literal
                                                0•==•0                                                                ComparisonExpression
                                                0                                                                     Literal
                                                     0                                                                Literal
                                                              0•==•0•&&•0•==•0••&&••0•==•0•&&•0•==•0                  ExpressionStatement, AndExpression
                                                              0•==•0•&&•0•==•0••&&••0•==•0                            AndExpression
                                                              0•==•0•&&•0•==•0                                        AndExpression
                                                              0•==•0                                                  ComparisonExpression
                                                              0                                                       Literal
                                                                   0                                                  Literal
                                                                        0•==•0                                        ComparisonExpression
                                                                        0                                             Literal
                                                                             0                                        Literal
                                                                                    0•==•0                            ComparisonExpression
                                                                                    0                                 Literal
                                                                                         0                            Literal
                                                                                              0•==•0                  ComparisonExpression
                                                                                              0                       Literal
                                                                                                   0                  Literal
                                                                                                             {•0•}    BlockExpression
                                                                                                               0      ExpressionStatement, Literal*/
	let _ = if  0 == 0 && 0 != 0 						{ 	0 == 0 && 0 != 0 						} else { 0 }                      /*
    let•_•=•if••0•==•0•&&•0•!=•0•╚╚╚╚╚╚{•╚0•==•0•&&•0•!=•0•╚╚╚╚╚╚}•else•{•0•}                                           LetVariableDeclaration
        _                                                                                                               WildcardPattern
            if••0•==•0•&&•0•!=•0•╚╚╚╚╚╚{•╚0•==•0•&&•0•!=•0•╚╚╚╚╚╚}•else•{•0•}                                           IfBlockExpression
                0•==•0•&&•0•!=•0                                                                                        AndExpression
                0•==•0                                                                                                  ComparisonExpression
                0                                                                                                       Literal
                     0                                                                                                  Literal
                          0•!=•0                                                                                        ComparisonExpression
                          0                                                                                             Literal
                               0                                                                                        Literal
                                                               0•==•0•&&•0•!=•0                                         ExpressionStatement, AndExpression
                                                               0•==•0                                                   ComparisonExpression
                                                               0                                                        Literal
                                                                    0                                                   Literal
                                                                         0•!=•0                                         ComparisonExpression
                                                                         0                                              Literal
                                                                              0                                         Literal
                                                                                                               {•0•}    BlockExpression
                                                                                                                 0      ExpressionStatement, Literal*/
	let _ = if  c!() && c!()                            { 	c!() && c!()							} else { 0 }                        /*
    let•_•=•if••c!()•&&•c!()••••••••••••••••••••••••••••{•╚c!()•&&•c!()╚╚╚╚╚╚╚}•else•{•0•}                            LetVariableDeclaration
        _                                                                                                             WildcardPattern
            if••c!()•&&•c!()••••••••••••••••••••••••••••{•╚c!()•&&•c!()╚╚╚╚╚╚╚}•else•{•0•}                            IfBlockExpression
                c!()•&&•c!()                                                                                          AndExpression
                c!()                                                                                                  MacroInvocation
                        c!()                                                                                          MacroInvocation
                                                              c!()•&&•c!()                                            ExpressionStatement, AndExpression
                                                              c!()                                                    MacroInvocation
                                                                      c!()                                            MacroInvocation
                                                                                                             {•0•}    BlockExpression
                                                                                                               0      ExpressionStatement, Literal*/

	if let _ = 0..|| 0 {}                                                                                                                 /*
    if•let•_•=•0..||•0•{}    ExpressionStatement, IfBlockExpression
       let•_•=•0..||•0       LetScrutinee
           _                 WildcardPattern
               0..||•0       RangeLiteral
               0             Literal
                  ||•0       ClosureFunctionExpression
                     0       Literal                                                                                                      */
	if let _ = 0..&&0 {}                                                                                                                  /*
    if•let•_•=•0..&&0•{}    ExpressionStatement, IfBlockExpression
       let•_•=•0..&&0       LetScrutinee
           _                WildcardPattern
               0..&&0       RangeLiteral
               0            Literal
                  &&0       ReferenceExpression
                   &0       ReferenceExpression
                    0       Literal                                                                                                       */
	if let _ = 0..0 && 0 { }                                                                                                              /*
    if•let•_•=•0..0•&&•0•{•}    ExpressionStatement, IfBlockExpression
       let•_•=•0..0•&&•0        AndExpression
       let•_•=•0..0             LetScrutinee
           _                    WildcardPattern
               0..0             RangeLiteral
               0                Literal
                  0             Literal
                       0        Literal                                                                                                   */
	if let _ = break 0 && 0 {}                                                                                                            /*
    if•let•_•=•break•0•&&•0•{}    ExpressionStatement, IfBlockExpression
       let•_•=•break•0•&&•0       AndExpression
       let•_•=•break•0            LetScrutinee
           _                      WildcardPattern
               break•0            BreakExpression
                     0            Literal
                          0       Literal                                                                                                 */

	_ = if 0 { 0 } else { 0 };                                                                                                            /*
    _•=•if•0•{•0•}•else•{•0•};    ExpressionStatement
    _•=•if•0•{•0•}•else•{•0•}     ReassignmentExpression
    _                             UnassignedExpression
        if•0•{•0•}•else•{•0•}     IfBlockExpression
           0                      Literal
               0                  ExpressionStatement, Literal
                        {•0•}     BlockExpression
                          0       ExpressionStatement, Literal                                                                            */
	_ = if 0 { 0 } else { 0 }();                                                                                                          /*
    _•=•if•0•{•0•}•else•{•0•}();    ExpressionStatement
    _•=•if•0•{•0•}•else•{•0•}()     ReassignmentExpression
    _                               UnassignedExpression
        if•0•{•0•}•else•{•0•}()     CallExpression
        if•0•{•0•}•else•{•0•}       IfBlockExpression
           0                        Literal
               0                    ExpressionStatement, Literal
                        {•0•}       BlockExpression
                          0         ExpressionStatement, Literal                                                                          */
	_ = if 0 { 0 } else { 0 } as *mut _;                                                                                                  /*
    _•=•if•0•{•0•}•else•{•0•}•as•*mut•_;    ExpressionStatement
    _•=•if•0•{•0•}•else•{•0•}•as•*mut•_     ReassignmentExpression
    _                                       UnassignedExpression
        if•0•{•0•}•else•{•0•}•as•*mut•_     ExpressionAsTypeCast
        if•0•{•0•}•else•{•0•}               IfBlockExpression
           0                                Literal
               0                            ExpressionStatement, Literal
                        {•0•}               BlockExpression
                          0                 ExpressionStatement, Literal
                                 *mut•_     TypeDereferenceMut
                                      _     TypeInferred                                                                                  */

	for _ in &[c(1)] { c(0); }                                                                                                            /*
    for•_•in•&[c(1)]•{•c(0);•}    ExpressionStatement, ForInBlockExpression
        _                         WildcardPattern
             &[c(1)]              ReferenceExpression
              [c(1)]              ArrayLiteral
               c(1)               CallExpression
                 1                Literal
                       c(0);      ExpressionStatement
                       c(0)       CallExpression
                         0        Literal                                                                                                 */
	for _ in -0 + 0..=(0 - 0) {}                                                                                                          /*
    for•_•in•-0•+•0..=(0•-•0)•{}    ExpressionStatement, ForInBlockExpression
        _                           WildcardPattern
             -0•+•0..=(0•-•0)       RangeLiteral
             -0•+•0                 OperationExpression
             -0                     MinusExpression
              0                     Literal
                  0                 Literal
                       0•-•0        OperationExpression
                       0            Literal
                           0        Literal                                                                                               */
	for _ in 0..o.m() { o = o ^ o[o]; }                                                                                                   /*
    for•_•in•0..o.m()•{•o•=•o•^•o[o];•}    ExpressionStatement, ForInBlockExpression
        _                                  WildcardPattern
             0..o.m()                      RangeLiteral
             0                             Literal
                o.m()                      CallExpression
                        o•=•o•^•o[o];      ExpressionStatement
                        o•=•o•^•o[o]       ReassignmentExpression
                            o•^•o[o]       OperationExpression
                                o[o]       MemberExpression                                                                               */
	for i in 0..(chunk_d - 1) {}                                                                                                          /*
    for•i•in•0..(chunk_d•-•1)•{}    ExpressionStatement, ForInBlockExpression
             0..(chunk_d•-•1)       RangeLiteral
             0                      Literal
                 chunk_d•-•1        OperationExpression
                           1        Literal                                                                                               */

	unsafe { *o::<u8>() = 0; }                                                                                                            /*
    unsafe•{•*o::<u8>()•=•0;•}    ExpressionStatement, BlockExpression
             *o::<u8>()•=•0;      ExpressionStatement
             *o::<u8>()•=•0       ReassignmentExpression
             *o::<u8>()           DereferenceExpression
              o::<u8>()           CallExpression
                          0       Literal                                                                                                 */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */


type o = (                                                                                                                                /*
type•o•=•(↲    <TypeAliasDeclaration>
         (↲    <TypeTuple>                                                                                                                */
	Box<dyn FnMut(u8)->u8+'static>,                                                                                                       /*
    Box<dyn•FnMut(u8)->u8+'static>    TypeCall
        dyn•FnMut(u8)->u8+'static     TypeDynBounds
            FnMut(u8)->u8             TypeTraitBound, TypeFunction
                          'static     LtStatic                                                                                            */
	Box<dyn FnMut(u8)->dyn u8+'static>                                                                                                    /*
    Box<dyn•FnMut(u8)->dyn•u8+'static>    TypeCall
        dyn•FnMut(u8)->dyn•u8+'static     TypeDynBounds
            FnMut(u8)->dyn•u8+'static     TypeTraitBound, TypeFunction
                       dyn•u8+'static     TypeDynBounds
                           u8             TypeTraitBound
                              'static     LtStatic                                                                                        */
);                                                                                                                                        /*
);    </TypeAliasDeclaration>
)     </TypeTuple>                                                                                                                        */


pub extern "ABI" fn f() {                                                                                                                 /*
pub•extern•"ABI"•fn•f()•{↲    <FunctionDeclaration>
pub                           PubSpecifier
    extern•"ABI"              ExternSpecifier
           "ABI"              Literal                                                                                                     */
    c! { pub static T : T = T { } ; }                                                                                                     /*
    c!•{•pub•static•T•:•T•=•T•{•}•;•}    ExpressionStatement, MacroInvocation
                      :                  PunctuationToken
                          =              PunctuationToken
                              {•}        DelimGroup
                                  ;      PunctuationToken                                                                                 */
    #[attr(info)] { o.m(|a_0| ()) }                                                                                                       /*
    #[attr(info)]•{•o.m(|a_0|•())•}    ExpressionStatement
    #[attr(info)]                      Attribute
          (info)                       DelimGroup
                  {•o.m(|a_0|•())•}    BlockExpression
                    o.m(|a_0|•())      ExpressionStatement, CallExpression
                        |a_0|•()       ClosureFunctionExpression
                         a_0           ClosureFunctionParameterDeclaration
                              ()       TupleLiteral                                                                                       */
	c! () {}                                                                                                                              /*
    c!•()       ExpressionStatement, MacroInvocation
          {}    ExpressionStatement, BlockExpression                                                                                      */
	c! [] {}                                                                                                                              /*
    c!•[]       ExpressionStatement, MacroInvocation
          {}    ExpressionStatement, BlockExpression                                                                                      */
	c! {} {}                                                                                                                              /*
    c!•{}       ExpressionStatement, MacroInvocation
          {}    ExpressionStatement, BlockExpression                                                                                      */
	c! {}; {}                                                                                                                             /*
    c!•{};       ExpressionStatement
    c!•{}        MacroInvocation
           {}    ExpressionStatement, BlockExpression                                                                                     */
	c! (); {}                                                                                                                             /*
    c!•();       ExpressionStatement
    c!•()        MacroInvocation
           {}    ExpressionStatement, BlockExpression                                                                                     */
	c! []; {}                                                                                                                             /*
    c!•[];       ExpressionStatement
    c!•[]        MacroInvocation
           {}    ExpressionStatement, BlockExpression                                                                                     */
	{}{}{};                                                                                                                               /*
    {}         ExpressionStatement, BlockExpression
      {}       ExpressionStatement, BlockExpression
        {};    ExpressionStatement
        {}     BlockExpression                                                                                                            */
	(){}{};                                                                                                                               /*
    ()         ExpressionStatement, TupleLiteral
      {}       ExpressionStatement, BlockExpression
        {};    ExpressionStatement
        {}     BlockExpression                                                                                                            */
	{}(){};                                                                                                                               /*
    {}         ExpressionStatement, BlockExpression
      ()       ExpressionStatement, TupleLiteral
        {};    ExpressionStatement
        {}     BlockExpression                                                                                                            */
	{}{}();                                                                                                                               /*
    {}         ExpressionStatement, BlockExpression
      {}       ExpressionStatement, BlockExpression
        ();    ExpressionStatement
        ()     TupleLiteral                                                                                                               */
	[]{}();                                                                                                                               /*
    []         ExpressionStatement, ArrayLiteral
      {}       ExpressionStatement, BlockExpression
        ();    ExpressionStatement
        ()     TupleLiteral                                                                                                               */
	{}[]();                                                                                                                               /*
    {}         ExpressionStatement, BlockExpression
      []();    ExpressionStatement
      []()     CallExpression
      []       ArrayLiteral                                                                                                               */
	// {}()[];
    //•{}()[];    Comment
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */


// {   if a { 1 } else { 0 }   +   if b { 1 } else { 0 }   }
//•{•••if•a•{•1•}•else•{•0•}•••+•••if•b•{•1•}•else•{•0•}•••}    Comment
// {   if a { 1 } else { 0 }   + ( if b { 1 } else { 0 } ) }
//•{•••if•a•{•1•}•else•{•0•}•••+•(•if•b•{•1•}•else•{•0•}•)•}    Comment

[                                                                                                                                         /*
[↲    <ExpressionStatement>, <ArrayLiteral>                                                                                               */
	{ ({ 0 }) - 0 },                                                                                                                      /*
    {•({•0•})•-•0•}    BlockExpression
      ({•0•})•-•0      ExpressionStatement, OperationExpression
       {•0•}           BlockExpression
         0             ExpressionStatement, Literal
                0      Literal                                                                                                            */
	{ { 0 } || 0 },                                                                                                                       /*
    {•{•0•}•||•0•}    BlockExpression
      {•0•}           ExpressionStatement, BlockExpression
        0             ExpressionStatement, Literal
            ||•0      ExpressionStatement, ClosureFunctionExpression
               0      Literal                                                                                                             */
	{ { 0 } &&0 },                                                                                                                        /*
    {•{•0•}•&&0•}    BlockExpression
      {•0•}          ExpressionStatement, BlockExpression
        0            ExpressionStatement, Literal
            &&0      ExpressionStatement, ReferenceExpression
             &0      ReferenceExpression
              0      Literal                                                                                                              */
	{ { 0 } && if 0 { 0 } else { 0 } },                                                                                                   /*
    {•{•0•}•&&•if•0•{•0•}•else•{•0•}•}    BlockExpression
      {•0•}                               ExpressionStatement, BlockExpression
        0                                 ExpressionStatement, Literal
            &&•if•0•{•0•}•else•{•0•}      ExpressionStatement, ReferenceExpression
             &•if•0•{•0•}•else•{•0•}      ReferenceExpression
               if•0•{•0•}•else•{•0•}      IfBlockExpression
                  0                       Literal
                      0                   ExpressionStatement, Literal
                               {•0•}      BlockExpression
                                 0        ExpressionStatement, Literal                                                                    */
	{ { 0 } *0;},                                                                                                                         /*
    {•{•0•}•*0;}    BlockExpression
      {•0•}         ExpressionStatement, BlockExpression
        0           ExpressionStatement, Literal
            *0;     ExpressionStatement
            *0      DereferenceExpression
             0      Literal                                                                                                               */
	{ { 0 } *0 },                                                                                                                         /*
    {•{•0•}•*0•}    BlockExpression
      {•0•}         ExpressionStatement, BlockExpression
        0           ExpressionStatement, Literal
            *0      ExpressionStatement, DereferenceExpression
             0      Literal                                                                                                               */
	{ { 0 } (0, 0) },                                                                                                                     /*
    {•{•0•}•(0,•0)•}    BlockExpression
      {•0•}             ExpressionStatement, BlockExpression
        0               ExpressionStatement, Literal
            (0,•0)      ExpressionStatement, TupleLiteral
             0          Literal
                0       Literal                                                                                                           */
	{ { 0 } (0 || 0) && 0 },                                                                                                              /*
    {•{•0•}•(0•||•0)•&&•0•}    BlockExpression
      {•0•}                    ExpressionStatement, BlockExpression
        0                      ExpressionStatement, Literal
            (0•||•0)•&&•0      ExpressionStatement, AndExpression
             0•||•0            OrExpression
             0                 Literal
                  0            Literal
                        0      Literal                                                                                                    */
	{ if 0 {} !(0) },                                                                                                                     /*
    {•if•0•{}•!(0)•}    BlockExpression
      if•0•{}           ExpressionStatement, IfBlockExpression
         0              Literal
              !(0)      ExpressionStatement, NotExpression
                0       Literal                                                                                                           */
	{ if 0 {} vec![0] },                                                                                                                  /*
    {•if•0•{}•vec![0]•}    BlockExpression
      if•0•{}              ExpressionStatement, IfBlockExpression
         0                 Literal
              vec![0]      ExpressionStatement, MacroInvocation
                   0       Literal                                                                                                        */
	{ if 0 {} *0 || 0 },                                                                                                                  /*
    {•if•0•{}•*0•||•0•}    BlockExpression
      if•0•{}              ExpressionStatement, IfBlockExpression
         0                 Literal
              *0•||•0      ExpressionStatement, OrExpression
              *0           DereferenceExpression
               0           Literal
                    0      Literal                                                                                                        */
	{ if 0 {} -0 },                                                                                                                       /*
    {•if•0•{}•-0•}    BlockExpression
      if•0•{}         ExpressionStatement, IfBlockExpression
         0            Literal
              -0      ExpressionStatement, MinusExpression
               0      Literal                                                                                                             */
	{ if 0 {} .m() },                                                                                                                     /*
    {•if•0•{}•.m()•}    BlockExpression
      if•0•{}•.m()      ExpressionStatement, CallExpression
      if•0•{}           IfBlockExpression
         0              Literal                                                                                                           */
	{ if 0 {} ? },                                                                                                                        /*
    {•if•0•{}•?•}    BlockExpression
      if•0•{}•?      ExpressionStatement, UnwrapExpression
      if•0•{}        IfBlockExpression
         0           Literal                                                                                                              */
	{ if 0 {} || 0 },                                                                                                                     /*
    {•if•0•{}•||•0•}    BlockExpression
      if•0•{}           ExpressionStatement, IfBlockExpression
         0              Literal
              ||•0      ExpressionStatement, ClosureFunctionExpression
                 0      Literal                                                                                                           */
	{ if 0 {} (|| 0) },                                                                                                                   /*
    {•if•0•{}•(||•0)•}    BlockExpression
      if•0•{}             ExpressionStatement, IfBlockExpression
         0                Literal
              (||•0)      ExpressionStatement
               ||•0       ClosureFunctionExpression
                  0       Literal                                                                                                         */
	{ if 0 {} ? || 0 },                                                                                                                   /*
    {•if•0•{}•?•||•0•}    BlockExpression
      if•0•{}•?•||•0      ExpressionStatement, OrExpression
      if•0•{}•?           UnwrapExpression
      if•0•{}             IfBlockExpression
         0                Literal
                   0      Literal                                                                                                         */
	{ if 0 {} .m() || 0 },                                                                                                                /*
    {•if•0•{}•.m()•||•0•}    BlockExpression
      if•0•{}•.m()•||•0      ExpressionStatement, OrExpression
      if•0•{}•.m()           CallExpression
      if•0•{}                IfBlockExpression
         0                   Literal
                      0      Literal                                                                                                      */
	{ if 0 {} else {} |o| 0 },                                                                                                            /*
    {•if•0•{}•else•{}•|o|•0•}    BlockExpression
      if•0•{}•else•{}            ExpressionStatement, IfBlockExpression
         0                       Literal
                   {}            BlockExpression
                      |o|•0      ExpressionStatement, ClosureFunctionExpression
                       o         ClosureFunctionParameterDeclaration
                          0      Literal                                                                                                  */
	{ 0 + if 0 { 0 } else { 0 }[0] },                                                                                                     /*
    {•0•+•if•0•{•0•}•else•{•0•}[0]•}    BlockExpression
      0•+•if•0•{•0•}•else•{•0•}[0]      ExpressionStatement, OperationExpression
      0                                 Literal
          if•0•{•0•}•else•{•0•}[0]      MemberExpression
          if•0•{•0•}•else•{•0•}         IfBlockExpression
             0                          Literal
                 0                      ExpressionStatement, Literal
                          {•0•}         BlockExpression
                            0           ExpressionStatement, Literal
                                0       Literal                                                                                           */
	{ fn f<T>() { loop {} } o::<T> as T },                                                                                                /*
    {•fn•f<T>()•{•loop•{}•}•o::<T>•as•T•}    BlockExpression
      fn•f<T>()•{•loop•{}•}                  FunctionDeclaration
           T                                 GenericTypeParameterDeclaration
                  loop•{}                    ExpressionStatement, LoopBlockExpression
                            o::<T>•as•T      ExpressionStatement, ExpressionAsTypeCast
                            o::<T>           ExpressionTypeCast                                                                           */
	{ match 0 {} o[0] },                                                                                                                  /*
    {•match•0•{}•o[0]•}    BlockExpression
      match•0•{}           ExpressionStatement, MatchExpression
            0              Literal
                 o[0]      ExpressionStatement, MemberExpression
                   0       Literal                                                                                                        */
	{ match 0 {} (*0 < 0) as T },                                                                                                         /*
    {•match•0•{}•(*0•<•0)•as•T•}    BlockExpression
      match•0•{}                    ExpressionStatement, MatchExpression
            0                       Literal
                 (*0•<•0)•as•T      ExpressionStatement, ExpressionAsTypeCast
                  *0•<•0            ComparisonExpression
                  *0                DereferenceExpression
                   0                Literal
                       0            Literal                                                                                               */
	{ match 0 {} o.m(S { p }); },                                                                                                         /*
    {•match•0•{}•o.m(S•{•p•});•}    BlockExpression
      match•0•{}                    ExpressionStatement, MatchExpression
            0                       Literal
                 o.m(S•{•p•});      ExpressionStatement
                 o.m(S•{•p•})       CallExpression
                     S•{•p•}        StructLiteral
                         p          StructLiteralPropertyShorthand                                                                        */
	{ match 0 {} o.m(o.m() + 0); },                                                                                                       /*
    {•match•0•{}•o.m(o.m()•+•0);•}    BlockExpression
      match•0•{}                      ExpressionStatement, MatchExpression
            0                         Literal
                 o.m(o.m()•+•0);      ExpressionStatement
                 o.m(o.m()•+•0)       CallExpression
                     o.m()•+•0        OperationExpression
                     o.m()            CallExpression
                             0        Literal                                                                                             */
	{ match 0 {} if *0 < 0 { 0 } else { 0 } },                                                                                            /*
    {•match•0•{}•if•*0•<•0•{•0•}•else•{•0•}•}    BlockExpression
      match•0•{}                                 ExpressionStatement, MatchExpression
            0                                    Literal
                 if•*0•<•0•{•0•}•else•{•0•}      ExpressionStatement, IfBlockExpression
                    *0•<•0                       ComparisonExpression
                    *0                           DereferenceExpression
                     0                           Literal
                         0                       Literal
                             0                   ExpressionStatement, Literal
                                      {•0•}      BlockExpression
                                        0        ExpressionStatement, Literal                                                             */
	{ (0 .. 0) .m((0, 0), |a_0, _| { *o = (o.0, o.0 + o.0); c(*0) }) .m(&|(a_0, _)| 0) },                                                 /*
    {•(0•..•0)•.m((0,•0),•|a_0,•_|•{•*o•=•(o.0,•o.0•+•o.0);•c(*0)•})•.m(&|(a_0,•_)|•0)•}    BlockExpression
      (0•..•0)•.m((0,•0),•|a_0,•_|•{•*o•=•(o.0,•o.0•+•o.0);•c(*0)•})•.m(&|(a_0,•_)|•0)      ExpressionStatement, CallExpression
      (0•..•0)•.m((0,•0),•|a_0,•_|•{•*o•=•(o.0,•o.0•+•o.0);•c(*0)•})                        CallExpression
       0•..•0                                                                               RangeLiteral
       0                                                                                    Literal
            0                                                                               Literal
                  (0,•0)                                                                    TupleLiteral
                   0                                                                        Literal
                      0                                                                     Literal
                          |a_0,•_|•{•*o•=•(o.0,•o.0•+•o.0);•c(*0)•}                         ClosureFunctionExpression
                           a_0                                                              ClosureFunctionParameterDeclaration
                                _                                                           ClosureFunctionParameterDeclaration, WildcardPattern
                                   {•*o•=•(o.0,•o.0•+•o.0);•c(*0)•}                         BlockExpression
                                     *o•=•(o.0,•o.0•+•o.0);                                 ExpressionStatement
                                     *o•=•(o.0,•o.0•+•o.0)                                  ReassignmentExpression
                                     *o                                                     DereferenceExpression
                                          (o.0,•o.0•+•o.0)                                  TupleLiteral
                                           o.0                                              MemberExpression
                                             0                                              Index
                                                o.0•+•o.0                                   OperationExpression
                                                o.0                                         MemberExpression
                                                  0                                         Index
                                                      o.0                                   MemberExpression
                                                        0                                   Index
                                                            c(*0)                           ExpressionStatement, CallExpression
                                                              *0                            DereferenceExpression
                                                               0                            Literal
                                                                        &|(a_0,•_)|•0       ReferenceExpression
                                                                         |(a_0,•_)|•0       ClosureFunctionExpression
                                                                          (a_0,•_)          ClosureFunctionParameterDeclaration, TuplePattern
                                                                                _           WildcardPattern
                                                                                    0       Literal                                       */
	{ 'label: loop {}.p },                                                                                                                /*
    {•'label:•loop•{}.p•}    BlockExpression
      'label:•loop•{}.p      ExpressionStatement, MemberExpression
      'label:•loop•{}        LoopBlockExpression
      'label                 LbIdentifier                                                                                                 */
	{ async { 0 + 0 }.await },                                                                                                            /*
    {•async•{•0•+•0•}.await•}    BlockExpression
      async•{•0•+•0•}.await      ExpressionStatement, AwaitExpression
      async•{•0•+•0•}            BlockExpression
              0•+•0              ExpressionStatement, OperationExpression
              0                  Literal
                  0              Literal                                                                                                  */
	{ { o.p }.p = 0 },                                                                                                                    /*
    {•{•o.p•}.p•=•0•}    BlockExpression
      {•o.p•}.p•=•0      ExpressionStatement, ReassignmentExpression
      {•o.p•}.p          MemberExpression
      {•o.p•}            BlockExpression
        o.p              ExpressionStatement, MemberExpression
                  0      Literal                                                                                                          */
	{({ 0 } + 0)},                                                                                                                        /*
    {({•0•}•+•0)}    BlockExpression
     ({•0•}•+•0)     ExpressionStatement
      {•0•}•+•0      OperationExpression
      {•0•}          BlockExpression
        0            ExpressionStatement, Literal
              0      Literal                                                                                                              */
	{({ 0 }) + 0},                                                                                                                        /*
    {({•0•})•+•0}    BlockExpression
     ({•0•})•+•0     ExpressionStatement, OperationExpression
      {•0•}          BlockExpression
        0            ExpressionStatement, Literal
               0     Literal                                                                                                              */
	{({ 0 } + 0 + 0)},                                                                                                                    /*
    {({•0•}•+•0•+•0)}    BlockExpression
     ({•0•}•+•0•+•0)     ExpressionStatement
      {•0•}•+•0•+•0      OperationExpression
      {•0•}•+•0          OperationExpression
      {•0•}              BlockExpression
        0                ExpressionStatement, Literal
              0          Literal
                  0      Literal                                                                                                          */
	{ ( if 0 { 0 } else { 0 }   +   if 0 { 0 } else { 0 } ) },                                                                            /*
    {•(•if•0•{•0•}•else•{•0•}•••+•••if•0•{•0•}•else•{•0•}•)•}    BlockExpression
      (•if•0•{•0•}•else•{•0•}•••+•••if•0•{•0•}•else•{•0•}•)      ExpressionStatement
        if•0•{•0•}•else•{•0•}•••+•••if•0•{•0•}•else•{•0•}        OperationExpression
        if•0•{•0•}•else•{•0•}                                    IfBlockExpression
           0                                                     Literal
               0                                                 ExpressionStatement, Literal
                        {•0•}                                    BlockExpression
                          0                                      ExpressionStatement, Literal
                                    if•0•{•0•}•else•{•0•}        IfBlockExpression
                                       0                         Literal
                                           0                     ExpressionStatement, Literal
                                                    {•0•}        BlockExpression
                                                      0          ExpressionStatement, Literal                                             */
	{ ( if 0 { 0 } else { 0 } ) +   if 0 { 0 } else { 0 }   },                                                                            /*
    {•(•if•0•{•0•}•else•{•0•}•)•+•••if•0•{•0•}•else•{•0•}•••}    BlockExpression
      (•if•0•{•0•}•else•{•0•}•)•+•••if•0•{•0•}•else•{•0•}        ExpressionStatement, OperationExpression
        if•0•{•0•}•else•{•0•}                                    IfBlockExpression
           0                                                     Literal
               0                                                 ExpressionStatement, Literal
                        {•0•}                                    BlockExpression
                          0                                      ExpressionStatement, Literal
                                    if•0•{•0•}•else•{•0•}        IfBlockExpression
                                       0                         Literal
                                           0                     ExpressionStatement, Literal
                                                    {•0•}        BlockExpression
                                                      0          ExpressionStatement, Literal                                             */
	{ ( if 0 { 0 } else { 0 } ) + ( if 0 { 0 } else { 0 } ) },                                                                            /*
    {•(•if•0•{•0•}•else•{•0•}•)•+•(•if•0•{•0•}•else•{•0•}•)•}    BlockExpression
      (•if•0•{•0•}•else•{•0•}•)•+•(•if•0•{•0•}•else•{•0•}•)      ExpressionStatement, OperationExpression
        if•0•{•0•}•else•{•0•}                                    IfBlockExpression
           0                                                     Literal
               0                                                 ExpressionStatement, Literal
                        {•0•}                                    BlockExpression
                          0                                      ExpressionStatement, Literal
                                    if•0•{•0•}•else•{•0•}        IfBlockExpression
                                       0                         Literal
                                           0                     ExpressionStatement, Literal
                                                    {•0•}        BlockExpression
                                                      0          ExpressionStatement, Literal                                             */
	{ ( if 0 { 0 } else { 0 } ) () },                                                                                                     /*
    {•(•if•0•{•0•}•else•{•0•}•)•()•}    BlockExpression
      (•if•0•{•0•}•else•{•0•}•)•()      ExpressionStatement, CallExpression
        if•0•{•0•}•else•{•0•}           IfBlockExpression
           0                            Literal
               0                        ExpressionStatement, Literal
                        {•0•}           BlockExpression
                          0             ExpressionStatement, Literal                                                                      */
	{ ( if 0 { 0 } else { 0 } () ) },                                                                                                     /*
    {•(•if•0•{•0•}•else•{•0•}•()•)•}    BlockExpression
      (•if•0•{•0•}•else•{•0•}•()•)      ExpressionStatement
        if•0•{•0•}•else•{•0•}•()        CallExpression
        if•0•{•0•}•else•{•0•}           IfBlockExpression
           0                            Literal
               0                        ExpressionStatement, Literal
                        {•0•}           BlockExpression
                          0             ExpressionStatement, Literal                                                                      */
	{ ( match 0 {} ) () },                                                                                                                /*
    {•(•match•0•{}•)•()•}    BlockExpression
      (•match•0•{}•)•()      ExpressionStatement, CallExpression
        match•0•{}           MatchExpression
              0              Literal                                                                                                      */
	{ ( match 0 {} () ) },                                                                                                                /*
    {•(•match•0•{}•()•)•}    BlockExpression
      (•match•0•{}•()•)      ExpressionStatement
        match•0•{}•()        CallExpression
        match•0•{}           MatchExpression
              0              Literal                                                                                                      */
	{ ( { 0 } ) () },                                                                                                                     /*
    {•(•{•0•}•)•()•}    BlockExpression
      (•{•0•}•)•()      ExpressionStatement, CallExpression
        {•0•}           BlockExpression
          0             ExpressionStatement, Literal                                                                                      */
	{ ( { 0 } () ) },                                                                                                                     /*
    {•(•{•0•}•()•)•}    BlockExpression
      (•{•0•}•()•)      ExpressionStatement
        {•0•}•()        CallExpression
        {•0•}           BlockExpression
          0             ExpressionStatement, Literal                                                                                      */
	{({ 0 } as u8)},                                                                                                                      /*
    {({•0•}•as•u8)}    BlockExpression
     ({•0•}•as•u8)     ExpressionStatement
      {•0•}•as•u8      ExpressionAsTypeCast
      {•0•}            BlockExpression
        0              ExpressionStatement, Literal                                                                                       */
	{({ 0 }) as u8},                                                                                                                      /*
    {({•0•})•as•u8}    BlockExpression
     ({•0•})•as•u8     ExpressionStatement, ExpressionAsTypeCast
      {•0•}            BlockExpression
        0              ExpressionStatement, Literal                                                                                       */
	{if 0 { &0 } &0 as &u8},                                                                                                              /*
    {if•0•{•&0•}•&0•as•&u8}    BlockExpression
     if•0•{•&0•}               ExpressionStatement, IfBlockExpression
        0                      Literal
            &0                 ExpressionStatement, ReferenceExpression
             0                 Literal
                 &0•as•&u8     ExpressionStatement, ExpressionAsTypeCast
                 &0            ReferenceExpression
                  0            Literal
                       &u8     TypeReference                                                                                              */
	{(if 0 { 0 } else { 0 } as u8)},                                                                                                      /*
    {(if•0•{•0•}•else•{•0•}•as•u8)}    BlockExpression
     (if•0•{•0•}•else•{•0•}•as•u8)     ExpressionStatement
      if•0•{•0•}•else•{•0•}•as•u8      ExpressionAsTypeCast
      if•0•{•0•}•else•{•0•}            IfBlockExpression
         0                             Literal
             0                         ExpressionStatement, Literal
                      {•0•}            BlockExpression
                        0              ExpressionStatement, Literal                                                                       */
	{(if 0 { 0 } else { 0 }) as u8},                                                                                                      /*
    {(if•0•{•0•}•else•{•0•})•as•u8}    BlockExpression
     (if•0•{•0•}•else•{•0•})•as•u8     ExpressionStatement, ExpressionAsTypeCast
      if•0•{•0•}•else•{•0•}            IfBlockExpression
         0                             Literal
             0                         ExpressionStatement, Literal
                      {•0•}            BlockExpression
                        0              ExpressionStatement, Literal                                                                       */
	{if 0 { &0 } else { &0 } &0 as &u8},                                                                                                  /*
    {if•0•{•&0•}•else•{•&0•}•&0•as•&u8}    BlockExpression
     if•0•{•&0•}•else•{•&0•}               ExpressionStatement, IfBlockExpression
        0                                  Literal
            &0                             ExpressionStatement, ReferenceExpression
             0                             Literal
                      {•&0•}               BlockExpression
                        &0                 ExpressionStatement, ReferenceExpression
                         0                 Literal
                             &0•as•&u8     ExpressionStatement, ExpressionAsTypeCast
                             &0            ReferenceExpression
                              0            Literal
                                   &u8     TypeReference                                                                                  */
	{(match 0 {} as u8)},                                                                                                                 /*
    {(match•0•{}•as•u8)}    BlockExpression
     (match•0•{}•as•u8)     ExpressionStatement
      match•0•{}•as•u8      ExpressionAsTypeCast
      match•0•{}            MatchExpression
            0               Literal                                                                                                       */
	{(match 0 {}) as u8},                                                                                                                 /*
    {(match•0•{})•as•u8}    BlockExpression
     (match•0•{})•as•u8     ExpressionStatement, ExpressionAsTypeCast
      match•0•{}            MatchExpression
            0               Literal                                                                                                       */
	{({ o })[0]},                                                                                                                         /*
    {({•o•})[0]}    BlockExpression
     ({•o•})[0]     ExpressionStatement, MemberExpression
      {•o•}         BlockExpression
        o           ExpressionStatement
             0      Literal                                                                                                               */
	{({ o }[0])},                                                                                                                         /*
    {({•o•}[0])}    BlockExpression
     ({•o•}[0])     ExpressionStatement
      {•o•}[0]      MemberExpression
      {•o•}         BlockExpression
        o           ExpressionStatement
            0       Literal                                                                                                               */
	{(if 0 { 0 } else { 0 })[0]},                                                                                                         /*
    {(if•0•{•0•}•else•{•0•})[0]}    BlockExpression
     (if•0•{•0•}•else•{•0•})[0]     ExpressionStatement, MemberExpression
      if•0•{•0•}•else•{•0•}         IfBlockExpression
         0                          Literal
             0                      ExpressionStatement, Literal
                      {•0•}         BlockExpression
                        0           ExpressionStatement, Literal
                             0      Literal                                                                                               */
	{(if 0 { 0 } else { 0 }[0])},                                                                                                         /*
    {(if•0•{•0•}•else•{•0•}[0])}    BlockExpression
     (if•0•{•0•}•else•{•0•}[0])     ExpressionStatement
      if•0•{•0•}•else•{•0•}[0]      MemberExpression
      if•0•{•0•}•else•{•0•}         IfBlockExpression
         0                          Literal
             0                      ExpressionStatement, Literal
                      {•0•}         BlockExpression
                        0           ExpressionStatement, Literal
                            0       Literal                                                                                               */
	{(match 0 {})[0]},                                                                                                                    /*
    {(match•0•{})[0]}    BlockExpression
     (match•0•{})[0]     ExpressionStatement, MemberExpression
      match•0•{}         MatchExpression
            0            Literal
                  0      Literal                                                                                                          */
	{(match 0 {}[0])},                                                                                                                    /*
    {(match•0•{}[0])}    BlockExpression
     (match•0•{}[0])     ExpressionStatement
      match•0•{}[0]      MemberExpression
      match•0•{}         MatchExpression
            0            Literal
                 0       Literal                                                                                                          */
	{   c!()   &0 },                                                                                                                      /*
    {•••c!()•••&0•}    BlockExpression
        c!()•••&0      ExpressionStatement, OperationExpression
        c!()           MacroInvocation
                0      Literal                                                                                                            */
	{   c![]   &0 },                                                                                                                      /*
    {•••c![]•••&0•}    BlockExpression
        c![]•••&0      ExpressionStatement, OperationExpression
        c![]           MacroInvocation
                0      Literal                                                                                                            */
	{   c!{}   &0 },                                                                                                                      /*
    {•••c!{}•••&0•}    BlockExpression
        c!{}           ExpressionStatement, MacroInvocation
               &0      ExpressionStatement, ReferenceExpression
                0      Literal                                                                                                            */
	{ ( c!{} ) &0 },                                                                                                                      /*
    {•(•c!{}•)•&0•}    BlockExpression
      (•c!{}•)•&0      ExpressionStatement, OperationExpression
        c!{}           MacroInvocation
                0      Literal                                                                                                            */
	{   { 0 }   &  1   },                                                                                                                 /*
    {•••{•0•}•••&••1•••}    BlockExpression
        {•0•}               ExpressionStatement, BlockExpression
          0                 ExpressionStatement, Literal
                &••1        ExpressionStatement, ReferenceExpression
                   1        Literal                                                                                                       */
	{ ( { 0 } ) &  1   },                                                                                                                 /*
    {•(•{•0•}•)•&••1•••}    BlockExpression
      (•{•0•}•)•&••1        ExpressionStatement, OperationExpression
        {•0•}               BlockExpression
          0                 ExpressionStatement, Literal
                   1        Literal                                                                                                       */
	{   { 0 }   .. 1   },                                                                                                                 /*
    {•••{•0•}•••..•1•••}    BlockExpression
        {•0•}               ExpressionStatement, BlockExpression
          0                 ExpressionStatement, Literal
                ..•1        ExpressionStatement, RangeLiteral
                   1        Literal                                                                                                       */
	{ ( { 0 } ) .. 1   },                                                                                                                 /*
    {•(•{•0•}•)•..•1•••}    BlockExpression
      (•{•0•}•)•..•1        ExpressionStatement, RangeLiteral
        {•0•}               BlockExpression
          0                 ExpressionStatement, Literal
                   1        Literal                                                                                                       */

	a..b!(),                                                                                                                              /*
    a..b!()    RangeLiteral
       b!()    MacroInvocation                                                                                                            */

	o.p.0 = 0,                                                                                                                            /*
    o.p.0•=•0    ReassignmentExpression
    o.p.0        MemberExpression
    o.p          MemberExpression
        0        Index
            0    Literal                                                                                                                  */
	o.p.0.0 = 0,                                                                                                                          /*
    o.p.0.0•=•0    ReassignmentExpression
    o.p.0.0        MemberExpression
    o.p.0          MemberExpression
    o.p            MemberExpression
        0          Index
          0        Index
              0    Literal                                                                                                                */
	(*o.p).0 = 0,                                                                                                                         /*
    (*o.p).0•=•0    ReassignmentExpression
    (*o.p).0        MemberExpression
     *o.p           DereferenceExpression
      o.p           MemberExpression
           0        Index
               0    Literal                                                                                                               */
	(*o.p.0).0 = 0,                                                                                                                       /*
    (*o.p.0).0•=•0    ReassignmentExpression
    (*o.p.0).0        MemberExpression
     *o.p.0           DereferenceExpression
      o.p.0           MemberExpression
      o.p             MemberExpression
          0           Index
             0        Index
                 0    Literal                                                                                                             */
	&mut o.p.0,                                                                                                                           /*
    &mut•o.p.0    ReferenceExpression
         o.p.0    MemberExpression
         o.p      MemberExpression
             0    Index                                                                                                                   */
	&mut o.p.0.0,                                                                                                                         /*
    &mut•o.p.0.0    ReferenceExpression
         o.p.0.0    MemberExpression
         o.p.0      MemberExpression
         o.p        MemberExpression
             0      Index
               0    Index                                                                                                                 */
	&mut (*o.p).0,                                                                                                                        /*
    &mut•(*o.p).0    ReferenceExpression
         (*o.p).0    MemberExpression
          *o.p       DereferenceExpression
           o.p       MemberExpression
                0    Index                                                                                                                */
	&mut (*o.p.0).0,                                                                                                                      /*
    &mut•(*o.p.0).0    ReferenceExpression
         (*o.p.0).0    MemberExpression
          *o.p.0       DereferenceExpression
           o.p.0       MemberExpression
           o.p         MemberExpression
               0       Index
                  0    Index                                                                                                              */
	o.p.0.m(0),                                                                                                                           /*
    o.p.0.m(0)    CallExpression
    o.p.0         MemberExpression
    o.p           MemberExpression
        0         Index
            0     Literal                                                                                                                 */
	o.p.0.0.m(0),                                                                                                                         /*
    o.p.0.0.m(0)    CallExpression
    o.p.0.0         MemberExpression
    o.p.0           MemberExpression
    o.p             MemberExpression
        0           Index
          0         Index
              0     Literal                                                                                                               */
	(*o.p).0.m(0),                                                                                                                        /*
    (*o.p).0.m(0)    CallExpression
    (*o.p).0         MemberExpression
     *o.p            DereferenceExpression
      o.p            MemberExpression
           0         Index
               0     Literal                                                                                                              */
	(*o.p.0).0.m(0),                                                                                                                      /*
    (*o.p.0).0.m(0)    CallExpression
    (*o.p.0).0         MemberExpression
     *o.p.0            DereferenceExpression
      o.p.0            MemberExpression
      o.p              MemberExpression
          0            Index
             0         Index
                 0     Literal                                                                                                            */

	o.p + *o + o,                                                                                                                         /*
    o.p•+•*o•+•o    OperationExpression
    o.p•+•*o        OperationExpression
    o.p             MemberExpression
          *o        DereferenceExpression                                                                                                 */

	c(o.p..o.p + o.p),                                                                                                                    /*
    c(o.p..o.p•+•o.p)    CallExpression
      o.p..o.p•+•o.p     RangeLiteral
      o.p                MemberExpression
           o.p•+•o.p     OperationExpression
           o.p           MemberExpression
                 o.p     MemberExpression                                                                                                 */
	c(c(o.p, |_| o.p = 0), 0),                                                                                                            /*
    c(c(o.p,•|_|•o.p•=•0),•0)    CallExpression
      c(o.p,•|_|•o.p•=•0)        CallExpression
        o.p                      MemberExpression
             |_|•o.p•=•0         ClosureFunctionExpression
              _                  ClosureFunctionParameterDeclaration, WildcardPattern
                 o.p•=•0         ReassignmentExpression
                 o.p             MemberExpression
                       0         Literal
                           0     Literal                                                                                                  */

	(0 && o.m(o.m()) == 0) || ((!0 && o[0].p == 0) || 0 == 0),                                                                            /*
    (0•&&•o.m(o.m())•==•0)•||•((!0•&&•o[0].p•==•0)•||•0•==•0)    OrExpression
     0•&&•o.m(o.m())•==•0                                        AndExpression
     0                                                           Literal
          o.m(o.m())•==•0                                        ComparisonExpression
          o.m(o.m())                                             CallExpression
              o.m()                                              CallExpression
                        0                                        Literal
                               (!0•&&•o[0].p•==•0)•||•0•==•0     OrExpression
                                !0•&&•o[0].p•==•0                AndExpression
                                !0                               NotExpression
                                 0                               Literal
                                      o[0].p•==•0                ComparisonExpression
                                      o[0].p                     MemberExpression
                                      o[0]                       MemberExpression
                                        0                        Literal
                                                0                Literal
                                                      0•==•0     ComparisonExpression
                                                      0          Literal
                                                           0     Literal                                                                  */
	(0 && o.m(o.m()) == 0) ||  (!0 && o[0].p == 0) || 0 == 0,                                                                             /*
    (0•&&•o.m(o.m())•==•0)•||••(!0•&&•o[0].p•==•0)•||•0•==•0    OrExpression
    (0•&&•o.m(o.m())•==•0)•||••(!0•&&•o[0].p•==•0)              OrExpression
     0•&&•o.m(o.m())•==•0                                       AndExpression
     0                                                          Literal
          o.m(o.m())•==•0                                       ComparisonExpression
          o.m(o.m())                                            CallExpression
              o.m()                                             CallExpression
                        0                                       Literal
                                !0•&&•o[0].p•==•0               AndExpression
                                !0                              NotExpression
                                 0                              Literal
                                      o[0].p•==•0               ComparisonExpression
                                      o[0].p                    MemberExpression
                                      o[0]                      MemberExpression
                                        0                       Literal
                                                0               Literal
                                                      0•==•0    ComparisonExpression
                                                      0         Literal
                                                           0    Literal                                                                   */

	//  x  &  y  ==  0   input
    //••x••&••y••==••0•••input    Comment
	//  x  & (y  ==  0)  javascript precedence
    //••x••&•(y••==••0)••javascript•precedence    Comment
	// (x  &  y) ==  0   rust precedence
    //•(x••&••y)•==••0•••rust•precedence    Comment

	0  &  0  ==  0,                                                                                                                       /*
    0••&••0••==••0    ComparisonExpression
    0••&••0           OperationExpression
    0                 Literal
          0           Literal
                 0    Literal                                                                                                             */
	0  & (0  ==  0),                                                                                                                      /*
    0••&•(0••==••0)    OperationExpression
    0                  Literal
          0••==••0     ComparisonExpression
          0            Literal
                 0     Literal                                                                                                            */
   (0  &  0) ==  0,                                                                                                                       /*
   (0••&••0)•==••0     ComparisonExpression
    0••&••0            OperationExpression
    0                  Literal
          0            Literal
                 0     Literal                                                                                                            */

	0 as u8 * 0,                                                                                                                          /*
    0•as•u8•*•0    OperationExpression
    0•as•u8        ExpressionAsTypeCast
    0              Literal
              0    Literal                                                                                                                */
	0 as (u8) * 0,                                                                                                                        /*
    0•as•(u8)•*•0    OperationExpression
    0•as•(u8)        ExpressionAsTypeCast
    0                Literal
                0    Literal                                                                                                              */
	0 as (u8) / 0,                                                                                                                        /*
    0•as•(u8)•/•0    OperationExpression
    0•as•(u8)        ExpressionAsTypeCast
    0                Literal
                0    Literal                                                                                                              */
	0 as u8 + 0,                                                                                                                          /*
    0•as•u8•+•0    OperationExpression
    0•as•u8        ExpressionAsTypeCast
    0              Literal
              0    Literal                                                                                                                */
	0 as (u8) + 0,                                                                                                                        /*
    0•as•(u8)•+•0    OperationExpression
    0•as•(u8)        ExpressionAsTypeCast
    0                Literal
                0    Literal                                                                                                              */
	o = o = o = o,                                                                                                                        /*
    o•=•o•=•o•=•o    ReassignmentExpression
        o•=•o•=•o    ReassignmentExpression
            o•=•o    ReassignmentExpression                                                                                               */
	o -= 0 - 0,                                                                                                                           /*
    o•-=•0•-•0    ReassignmentOperationExpression
         0•-•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	o -= 0 - 0,                                                                                                                           /*
    o•-=•0•-•0    ReassignmentOperationExpression
         0•-•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	o *= 0 * 0,                                                                                                                           /*
    o•*=•0•*•0    ReassignmentOperationExpression
         0•*•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	o *= 0 * 0,                                                                                                                           /*
    o•*=•0•*•0    ReassignmentOperationExpression
         0•*•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	o *= 0 * 99,                                                                                                                          /*
    o•*=•0•*•99    ReassignmentOperationExpression
         0•*•99    OperationExpression
         0         Literal
             99    Literal                                                                                                                */
	o /= 0 / 0,                                                                                                                           /*
    o•/=•0•/•0    ReassignmentOperationExpression
         0•/•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	o /= 0 / 0,                                                                                                                           /*
    o•/=•0•/•0    ReassignmentOperationExpression
         0•/•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	o &= 0 & 0,                                                                                                                           /*
    o•&=•0•&•0    ReassignmentOperationExpression
         0•&•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	o %= 0 % 0,                                                                                                                           /*
    o•%=•0•%•0    ReassignmentOperationExpression
         0•%•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	o ^= 0,                                                                                                                               /*
    o•^=•0    ReassignmentOperationExpression
         0    Literal                                                                                                                     */
	o ^= 0,                                                                                                                               /*
    o•^=•0    ReassignmentOperationExpression
         0    Literal                                                                                                                     */
	o += *0,                                                                                                                              /*
    o•+=•*0    ReassignmentOperationExpression
         *0    DereferenceExpression
          0    Literal                                                                                                                    */
	o += 0 + 0,                                                                                                                           /*
    o•+=•0•+•0    ReassignmentOperationExpression
         0•+•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	o <<= 0 << 0,                                                                                                                         /*
    o•<<=•0•<<•0    ReassignmentOperationExpression
          0•<<•0    OperationExpression
          0         Literal
               0    Literal                                                                                                               */
	o = 0 - 0,                                                                                                                            /*
    o•=•0•-•0    ReassignmentExpression
        0•-•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0 * 0 * 0,                                                                                                                        /*
    o•=•0•*•0•*•0    ReassignmentExpression
        0•*•0•*•0    OperationExpression
        0•*•0        OperationExpression
        0            Literal
            0        Literal
                0    Literal                                                                                                              */
	o = 0 * 0 + 0,                                                                                                                        /*
    o•=•0•*•0•+•0    ReassignmentExpression
        0•*•0•+•0    OperationExpression
        0•*•0        OperationExpression
        0            Literal
            0        Literal
                0    Literal                                                                                                              */
	o = 0 * 0,                                                                                                                            /*
    o•=•0•*•0    ReassignmentExpression
        0•*•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0 / 0,                                                                                                                            /*
    o•=•0•/•0    ReassignmentExpression
        0•/•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0 & 0,                                                                                                                            /*
    o•=•0•&•0    ReassignmentExpression
        0•&•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0 ^ 0,                                                                                                                            /*
    o•=•0•^•0    ReassignmentExpression
        0•^•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0 + 0,                                                                                                                            /*
    o•=•0•+•0    ReassignmentExpression
        0•+•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0 << 0,                                                                                                                           /*
    o•=•0•<<•0    ReassignmentExpression
        0•<<•0    OperationExpression
        0         Literal
             0    Literal                                                                                                                 */
	0 != 0 != 0,                                                                                                                          /*
    0•!=•0•!=•0    ComparisonExpression
    0•!=•0         ComparisonExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 != 0 == 0,                                                                                                                          /*
    0•!=•0•==•0    ComparisonExpression
    0•!=•0         ComparisonExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 == 0 != 0,                                                                                                                          /*
    0•==•0•!=•0    ComparisonExpression
    0•==•0         ComparisonExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 == 0 == 0,                                                                                                                          /*
    0•==•0•==•0    ComparisonExpression
    0•==•0         ComparisonExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 + 0 / 0,                                                                                                                            /*
    0•+•0•/•0    OperationExpression
    0            Literal
        0•/•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	0 + 0 >> 0,                                                                                                                           /*
    0•+•0•>>•0    OperationExpression
    0•+•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 % 0 * 0,                                                                                                                            /*
    0•%•0•*•0    OperationExpression
    0•%•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 % 0 / 0,                                                                                                                            /*
    0•%•0•/•0    OperationExpression
    0•%•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 % 0 % 0,                                                                                                                            /*
    0•%•0•%•0    OperationExpression
    0•%•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 * 0 % 0,                                                                                                                            /*
    0•*•0•%•0    OperationExpression
    0•*•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 / 0 % 0,                                                                                                                            /*
    0•/•0•%•0    OperationExpression
    0•/•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 / 0 + 0,                                                                                                                            /*
    0•/•0•+•0    OperationExpression
    0•/•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 & 0 & 0,                                                                                                                            /*
    0•&•0•&•0    OperationExpression
    0•&•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 & 0 >> 0,                                                                                                                           /*
    0•&•0•>>•0    OperationExpression
    0             Literal
        0•>>•0    OperationExpression
        0         Literal
             0    Literal                                                                                                                 */
	0 & 0 | 0,                                                                                                                            /*
    0•&•0•|•0    OperationExpression
    0•&•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 ^ 0 ^ 0,                                                                                                                            /*
    0•^•0•^•0    OperationExpression
    0•^•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 << 0 | 0,                                                                                                                           /*
    0•<<•0•|•0    OperationExpression
    0•<<•0        OperationExpression
    0             Literal
         0        Literal
             0    Literal                                                                                                                 */
	0 << 0 >> 0,                                                                                                                          /*
    0•<<•0•>>•0    OperationExpression
    0•<<•0         OperationExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 >> 0 >> 0,                                                                                                                          /*
    0•>>•0•>>•0    OperationExpression
    0•>>•0         OperationExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 | 0 & 0,                                                                                                                            /*
    0•|•0•&•0    OperationExpression
    0            Literal
        0•&•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	0 | 0 | 0,                                                                                                                            /*
    0•|•0•|•0    OperationExpression
    0•|•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */

	// (0 x 0) x 0 ===========================================================
    //•(0•x•0)•x•0•===========================================================    Comment
	(0 && 0) && 0,                                                                                                                        /*
    (0•&&•0)•&&•0    AndExpression
     0•&&•0          AndExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 && 0) || 0,                                                                                                                        /*
    (0•&&•0)•||•0    OrExpression
     0•&&•0          AndExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 && 0) = 0,                                                                                                                         /*
    (0•&&•0)•=•0    ReassignmentExpression
     0•&&•0         AndExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 && 0) + 0,                                                                                                                         /*
    (0•&&•0)•+•0    OperationExpression
     0•&&•0         AndExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 && 0) * 0,                                                                                                                         /*
    (0•&&•0)•*•0    OperationExpression
     0•&&•0         AndExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 && 0) & 0,                                                                                                                         /*
    (0•&&•0)•&•0    OperationExpression
     0•&&•0         AndExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 && 0) << 0,                                                                                                                        /*
    (0•&&•0)•<<•0    OperationExpression
     0•&&•0          AndExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 && 0) == 0,                                                                                                                        /*
    (0•&&•0)•==•0    ComparisonExpression
     0•&&•0          AndExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 && 0) > 0,                                                                                                                         /*
    (0•&&•0)•>•0    ComparisonExpression
     0•&&•0         AndExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 && 0)..0,                                                                                                                          /*
    (0•&&•0)..0    RangeLiteral
     0•&&•0        AndExpression
     0             Literal
          0        Literal
              0    Literal                                                                                                                */
	(0 || 0) && 0,                                                                                                                        /*
    (0•||•0)•&&•0    AndExpression
     0•||•0          OrExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 || 0) || 0,                                                                                                                        /*
    (0•||•0)•||•0    OrExpression
     0•||•0          OrExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 || 0) = 0,                                                                                                                         /*
    (0•||•0)•=•0    ReassignmentExpression
     0•||•0         OrExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 || 0) + 0,                                                                                                                         /*
    (0•||•0)•+•0    OperationExpression
     0•||•0         OrExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 || 0) * 0,                                                                                                                         /*
    (0•||•0)•*•0    OperationExpression
     0•||•0         OrExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 || 0) & 0,                                                                                                                         /*
    (0•||•0)•&•0    OperationExpression
     0•||•0         OrExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 || 0) << 0,                                                                                                                        /*
    (0•||•0)•<<•0    OperationExpression
     0•||•0          OrExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 || 0) == 0,                                                                                                                        /*
    (0•||•0)•==•0    ComparisonExpression
     0•||•0          OrExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 || 0) > 0,                                                                                                                         /*
    (0•||•0)•>•0    ComparisonExpression
     0•||•0         OrExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 || 0)..0,                                                                                                                          /*
    (0•||•0)..0    RangeLiteral
     0•||•0        OrExpression
     0             Literal
          0        Literal
              0    Literal                                                                                                                */
	(o = 0) && 0,                                                                                                                         /*
    (o•=•0)•&&•0    AndExpression
     o•=•0          ReassignmentExpression
         0          Literal
               0    Literal                                                                                                               */
	(o = 0) || 0,                                                                                                                         /*
    (o•=•0)•||•0    OrExpression
     o•=•0          ReassignmentExpression
         0          Literal
               0    Literal                                                                                                               */
	// (o = 0) = 0,
    //•(o•=•0)•=•0,    Comment
	(o = 0) + 0,                                                                                                                          /*
    (o•=•0)•+•0    OperationExpression
     o•=•0         ReassignmentExpression
         0         Literal
              0    Literal                                                                                                                */
	(o = 0) * 0,                                                                                                                          /*
    (o•=•0)•*•0    OperationExpression
     o•=•0         ReassignmentExpression
         0         Literal
              0    Literal                                                                                                                */
	(o = 0) & 0,                                                                                                                          /*
    (o•=•0)•&•0    OperationExpression
     o•=•0         ReassignmentExpression
         0         Literal
              0    Literal                                                                                                                */
	(o = 0) << 0,                                                                                                                         /*
    (o•=•0)•<<•0    OperationExpression
     o•=•0          ReassignmentExpression
         0          Literal
               0    Literal                                                                                                               */
	(o = 0) == 0,                                                                                                                         /*
    (o•=•0)•==•0    ComparisonExpression
     o•=•0          ReassignmentExpression
         0          Literal
               0    Literal                                                                                                               */
	(o = 0) > 0,                                                                                                                          /*
    (o•=•0)•>•0    ComparisonExpression
     o•=•0         ReassignmentExpression
         0         Literal
              0    Literal                                                                                                                */
	(o = 0)..0,                                                                                                                           /*
    (o•=•0)..0    RangeLiteral
     o•=•0        ReassignmentExpression
         0        Literal
             0    Literal                                                                                                                 */
	(0 + 0) && 0,                                                                                                                         /*
    (0•+•0)•&&•0    AndExpression
     0•+•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 + 0) || 0,                                                                                                                         /*
    (0•+•0)•||•0    OrExpression
     0•+•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 + 0) = 0,                                                                                                                          /*
    (0•+•0)•=•0    ReassignmentExpression
     0•+•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 + 0) + 0,                                                                                                                          /*
    (0•+•0)•+•0    OperationExpression
     0•+•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 + 0) * 0,                                                                                                                          /*
    (0•+•0)•*•0    OperationExpression
     0•+•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 + 0) & 0,                                                                                                                          /*
    (0•+•0)•&•0    OperationExpression
     0•+•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 + 0) << 0,                                                                                                                         /*
    (0•+•0)•<<•0    OperationExpression
     0•+•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 + 0) == 0,                                                                                                                         /*
    (0•+•0)•==•0    ComparisonExpression
     0•+•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 + 0) > 0,                                                                                                                          /*
    (0•+•0)•>•0    ComparisonExpression
     0•+•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 + 0)..0,                                                                                                                           /*
    (0•+•0)..0    RangeLiteral
     0•+•0        OperationExpression
     0            Literal
         0        Literal
             0    Literal                                                                                                                 */
	(0 * 0) && 0,                                                                                                                         /*
    (0•*•0)•&&•0    AndExpression
     0•*•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 * 0) || 0,                                                                                                                         /*
    (0•*•0)•||•0    OrExpression
     0•*•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 * 0) = 0,                                                                                                                          /*
    (0•*•0)•=•0    ReassignmentExpression
     0•*•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 * 0) + 0,                                                                                                                          /*
    (0•*•0)•+•0    OperationExpression
     0•*•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 * 0) * 0,                                                                                                                          /*
    (0•*•0)•*•0    OperationExpression
     0•*•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 * 0) & 0,                                                                                                                          /*
    (0•*•0)•&•0    OperationExpression
     0•*•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 * 0) << 0,                                                                                                                         /*
    (0•*•0)•<<•0    OperationExpression
     0•*•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 * 0) == 0,                                                                                                                         /*
    (0•*•0)•==•0    ComparisonExpression
     0•*•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 * 0) > 0,                                                                                                                          /*
    (0•*•0)•>•0    ComparisonExpression
     0•*•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 * 0)..0,                                                                                                                           /*
    (0•*•0)..0    RangeLiteral
     0•*•0        OperationExpression
     0            Literal
         0        Literal
             0    Literal                                                                                                                 */
	(0 & 0) && 0,                                                                                                                         /*
    (0•&•0)•&&•0    AndExpression
     0•&•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 & 0) || 0,                                                                                                                         /*
    (0•&•0)•||•0    OrExpression
     0•&•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 & 0) = 0,                                                                                                                          /*
    (0•&•0)•=•0    ReassignmentExpression
     0•&•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 & 0) + 0,                                                                                                                          /*
    (0•&•0)•+•0    OperationExpression
     0•&•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 & 0) * 0,                                                                                                                          /*
    (0•&•0)•*•0    OperationExpression
     0•&•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 & 0) & 0,                                                                                                                          /*
    (0•&•0)•&•0    OperationExpression
     0•&•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 & 0) << 0,                                                                                                                         /*
    (0•&•0)•<<•0    OperationExpression
     0•&•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 & 0) == 0,                                                                                                                         /*
    (0•&•0)•==•0    ComparisonExpression
     0•&•0          OperationExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 & 0) > 0,                                                                                                                          /*
    (0•&•0)•>•0    ComparisonExpression
     0•&•0         OperationExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 & 0)..0,                                                                                                                           /*
    (0•&•0)..0    RangeLiteral
     0•&•0        OperationExpression
     0            Literal
         0        Literal
             0    Literal                                                                                                                 */
	(0 << 0) && 0,                                                                                                                        /*
    (0•<<•0)•&&•0    AndExpression
     0•<<•0          OperationExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 << 0) || 0,                                                                                                                        /*
    (0•<<•0)•||•0    OrExpression
     0•<<•0          OperationExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 << 0) = 0,                                                                                                                         /*
    (0•<<•0)•=•0    ReassignmentExpression
     0•<<•0         OperationExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 << 0) + 0,                                                                                                                         /*
    (0•<<•0)•+•0    OperationExpression
     0•<<•0         OperationExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 << 0) * 0,                                                                                                                         /*
    (0•<<•0)•*•0    OperationExpression
     0•<<•0         OperationExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 << 0) & 0,                                                                                                                         /*
    (0•<<•0)•&•0    OperationExpression
     0•<<•0         OperationExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 << 0) << 0,                                                                                                                        /*
    (0•<<•0)•<<•0    OperationExpression
     0•<<•0          OperationExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 << 0) == 0,                                                                                                                        /*
    (0•<<•0)•==•0    ComparisonExpression
     0•<<•0          OperationExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 << 0) > 0,                                                                                                                         /*
    (0•<<•0)•>•0    ComparisonExpression
     0•<<•0         OperationExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 << 0)..0,                                                                                                                          /*
    (0•<<•0)..0    RangeLiteral
     0•<<•0        OperationExpression
     0             Literal
          0        Literal
              0    Literal                                                                                                                */
	(0 == 0) && 0,                                                                                                                        /*
    (0•==•0)•&&•0    AndExpression
     0•==•0          ComparisonExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 == 0) || 0,                                                                                                                        /*
    (0•==•0)•||•0    OrExpression
     0•==•0          ComparisonExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 == 0) = 0,                                                                                                                         /*
    (0•==•0)•=•0    ReassignmentExpression
     0•==•0         ComparisonExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 == 0) + 0,                                                                                                                         /*
    (0•==•0)•+•0    OperationExpression
     0•==•0         ComparisonExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 == 0) * 0,                                                                                                                         /*
    (0•==•0)•*•0    OperationExpression
     0•==•0         ComparisonExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 == 0) & 0,                                                                                                                         /*
    (0•==•0)•&•0    OperationExpression
     0•==•0         ComparisonExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 == 0) << 0,                                                                                                                        /*
    (0•==•0)•<<•0    OperationExpression
     0•==•0          ComparisonExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 == 0) == 0,                                                                                                                        /*
    (0•==•0)•==•0    ComparisonExpression
     0•==•0          ComparisonExpression
     0               Literal
          0          Literal
                0    Literal                                                                                                              */
	(0 == 0) > 0,                                                                                                                         /*
    (0•==•0)•>•0    ComparisonExpression
     0•==•0         ComparisonExpression
     0              Literal
          0         Literal
               0    Literal                                                                                                               */
	(0 == 0)..0,                                                                                                                          /*
    (0•==•0)..0    RangeLiteral
     0•==•0        ComparisonExpression
     0             Literal
          0        Literal
              0    Literal                                                                                                                */
	(0 > 0) && 0,                                                                                                                         /*
    (0•>•0)•&&•0    AndExpression
     0•>•0          ComparisonExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 > 0) || 0,                                                                                                                         /*
    (0•>•0)•||•0    OrExpression
     0•>•0          ComparisonExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 > 0) = 0,                                                                                                                          /*
    (0•>•0)•=•0    ReassignmentExpression
     0•>•0         ComparisonExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 > 0) + 0,                                                                                                                          /*
    (0•>•0)•+•0    OperationExpression
     0•>•0         ComparisonExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 > 0) * 0,                                                                                                                          /*
    (0•>•0)•*•0    OperationExpression
     0•>•0         ComparisonExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 > 0) & 0,                                                                                                                          /*
    (0•>•0)•&•0    OperationExpression
     0•>•0         ComparisonExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 > 0) << 0,                                                                                                                         /*
    (0•>•0)•<<•0    OperationExpression
     0•>•0          ComparisonExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 > 0) == 0,                                                                                                                         /*
    (0•>•0)•==•0    ComparisonExpression
     0•>•0          ComparisonExpression
     0              Literal
         0          Literal
               0    Literal                                                                                                               */
	(0 > 0) > 0,                                                                                                                          /*
    (0•>•0)•>•0    ComparisonExpression
     0•>•0         ComparisonExpression
     0             Literal
         0         Literal
              0    Literal                                                                                                                */
	(0 > 0)..0,                                                                                                                           /*
    (0•>•0)..0    RangeLiteral
     0•>•0        ComparisonExpression
     0            Literal
         0        Literal
             0    Literal                                                                                                                 */
	(0..0) && 0,                                                                                                                          /*
    (0..0)•&&•0    AndExpression
     0..0          RangeLiteral
     0             Literal
        0          Literal
              0    Literal                                                                                                                */
	(0..0) || 0,                                                                                                                          /*
    (0..0)•||•0    OrExpression
     0..0          RangeLiteral
     0             Literal
        0          Literal
              0    Literal                                                                                                                */
	(0..0) = 0,                                                                                                                           /*
    (0..0)•=•0    ReassignmentExpression
    (0..0)        TupleLiteral
     0..0         RangeLiteral
     0            Literal
        0         Literal
             0    Literal                                                                                                                 */
	(0..0) + 0,                                                                                                                           /*
    (0..0)•+•0    OperationExpression
     0..0         RangeLiteral
     0            Literal
        0         Literal
             0    Literal                                                                                                                 */
	(0..0) * 0,                                                                                                                           /*
    (0..0)•*•0    OperationExpression
     0..0         RangeLiteral
     0            Literal
        0         Literal
             0    Literal                                                                                                                 */
	(0..0) & 0,                                                                                                                           /*
    (0..0)•&•0    OperationExpression
     0..0         RangeLiteral
     0            Literal
        0         Literal
             0    Literal                                                                                                                 */
	(0..0) << 0,                                                                                                                          /*
    (0..0)•<<•0    OperationExpression
     0..0          RangeLiteral
     0             Literal
        0          Literal
              0    Literal                                                                                                                */
	(0..0) == 0,                                                                                                                          /*
    (0..0)•==•0    ComparisonExpression
     0..0          RangeLiteral
     0             Literal
        0          Literal
              0    Literal                                                                                                                */
	(0..0) > 0,                                                                                                                           /*
    (0..0)•>•0    ComparisonExpression
     0..0         RangeLiteral
     0            Literal
        0         Literal
             0    Literal                                                                                                                 */
	(0..0)..0,                                                                                                                            /*
    (0..0)..0    RangeLiteral
     0..0        RangeLiteral
     0           Literal
        0        Literal
            0    Literal                                                                                                                  */
	// 0 x 0 x 0 ===========================================================
    //•0•x•0•x•0•===========================================================    Comment
	0 && 0 && 0,                                                                                                                          /*
    0•&&•0•&&•0    AndExpression
    0•&&•0         AndExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 && 0 || 0,                                                                                                                          /*
    0•&&•0•||•0    OrExpression
    0•&&•0         AndExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 && o = 0,                                                                                                                           /*
    0•&&•o•=•0    ReassignmentExpression
    0•&&•o        AndExpression
    0             Literal
             0    Literal                                                                                                                 */
	0 && 0 + 0,                                                                                                                           /*
    0•&&•0•+•0    AndExpression
    0             Literal
         0•+•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 && 0 * 0,                                                                                                                           /*
    0•&&•0•*•0    AndExpression
    0             Literal
         0•*•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 && 0 & 0,                                                                                                                           /*
    0•&&•0•&•0    AndExpression
    0             Literal
         0•&•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 && 0 << 0,                                                                                                                          /*
    0•&&•0•<<•0    AndExpression
    0              Literal
         0•<<•0    OperationExpression
         0         Literal
              0    Literal                                                                                                                */
	0 && 0 == 0,                                                                                                                          /*
    0•&&•0•==•0    AndExpression
    0              Literal
         0•==•0    ComparisonExpression
         0         Literal
              0    Literal                                                                                                                */
	0 && 0 > 0,                                                                                                                           /*
    0•&&•0•>•0    AndExpression
    0             Literal
         0•>•0    ComparisonExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 && 0..0,                                                                                                                            /*
    0•&&•0..0    RangeLiteral
    0•&&•0       AndExpression
    0            Literal
         0       Literal
            0    Literal                                                                                                                  */
	0 || 0 && 0,                                                                                                                          /*
    0•||•0•&&•0    OrExpression
    0              Literal
         0•&&•0    AndExpression
         0         Literal
              0    Literal                                                                                                                */
	0 || 0 || 0,                                                                                                                          /*
    0•||•0•||•0    OrExpression
    0•||•0         OrExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 || o = 0,                                                                                                                           /*
    0•||•o•=•0    ReassignmentExpression
    0•||•o        OrExpression
    0             Literal
             0    Literal                                                                                                                 */
	0 || 0 + 0,                                                                                                                           /*
    0•||•0•+•0    OrExpression
    0             Literal
         0•+•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 || 0 * 0,                                                                                                                           /*
    0•||•0•*•0    OrExpression
    0             Literal
         0•*•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 || 0 & 0,                                                                                                                           /*
    0•||•0•&•0    OrExpression
    0             Literal
         0•&•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 || 0 << 0,                                                                                                                          /*
    0•||•0•<<•0    OrExpression
    0              Literal
         0•<<•0    OperationExpression
         0         Literal
              0    Literal                                                                                                                */
	0 || 0 == 0,                                                                                                                          /*
    0•||•0•==•0    OrExpression
    0              Literal
         0•==•0    ComparisonExpression
         0         Literal
              0    Literal                                                                                                                */
	0 || 0 > 0,                                                                                                                           /*
    0•||•0•>•0    OrExpression
    0             Literal
         0•>•0    ComparisonExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 || 0..0,                                                                                                                            /*
    0•||•0..0    RangeLiteral
    0•||•0       OrExpression
    0            Literal
         0       Literal
            0    Literal                                                                                                                  */
	o = 0 && 0,                                                                                                                           /*
    o•=•0•&&•0    ReassignmentExpression
        0•&&•0    AndExpression
        0         Literal
             0    Literal                                                                                                                 */
	o = 0 || 0,                                                                                                                           /*
    o•=•0•||•0    ReassignmentExpression
        0•||•0    OrExpression
        0         Literal
             0    Literal                                                                                                                 */
	o = o = 0,                                                                                                                            /*
    o•=•o•=•0    ReassignmentExpression
        o•=•0    ReassignmentExpression
            0    Literal                                                                                                                  */
	o = 0 + 0,                                                                                                                            /*
    o•=•0•+•0    ReassignmentExpression
        0•+•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0 * 0,                                                                                                                            /*
    o•=•0•*•0    ReassignmentExpression
        0•*•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0 & 0,                                                                                                                            /*
    o•=•0•&•0    ReassignmentExpression
        0•&•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0 << 0,                                                                                                                           /*
    o•=•0•<<•0    ReassignmentExpression
        0•<<•0    OperationExpression
        0         Literal
             0    Literal                                                                                                                 */
	o = 0 == 0,                                                                                                                           /*
    o•=•0•==•0    ReassignmentExpression
        0•==•0    ComparisonExpression
        0         Literal
             0    Literal                                                                                                                 */
	o = 0 > 0,                                                                                                                            /*
    o•=•0•>•0    ReassignmentExpression
        0•>•0    ComparisonExpression
        0        Literal
            0    Literal                                                                                                                  */
	o = 0..0,                                                                                                                             /*
    o•=•0..0    ReassignmentExpression
        0..0    RangeLiteral
        0       Literal
           0    Literal                                                                                                                   */
	0 + 0 && 0,                                                                                                                           /*
    0•+•0•&&•0    AndExpression
    0•+•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 + 0 || 0,                                                                                                                           /*
    0•+•0•||•0    OrExpression
    0•+•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 + o = 0,                                                                                                                            /*
    0•+•o•=•0    ReassignmentExpression
    0•+•o        OperationExpression
    0            Literal
            0    Literal                                                                                                                  */
	0 + 0 + 0,                                                                                                                            /*
    0•+•0•+•0    OperationExpression
    0•+•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 + 0 * 0,                                                                                                                            /*
    0•+•0•*•0    OperationExpression
    0            Literal
        0•*•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	0 + 0 & 0,                                                                                                                            /*
    0•+•0•&•0    OperationExpression
    0•+•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 + 0 << 0,                                                                                                                           /*
    0•+•0•<<•0    OperationExpression
    0•+•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 + 0 == 0,                                                                                                                           /*
    0•+•0•==•0    ComparisonExpression
    0•+•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 + 0 > 0,                                                                                                                            /*
    0•+•0•>•0    ComparisonExpression
    0•+•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 + 0..0,                                                                                                                             /*
    0•+•0..0    RangeLiteral
    0•+•0       OperationExpression
    0           Literal
        0       Literal
           0    Literal                                                                                                                   */
	0 * 0 && 0,                                                                                                                           /*
    0•*•0•&&•0    AndExpression
    0•*•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 * 0 || 0,                                                                                                                           /*
    0•*•0•||•0    OrExpression
    0•*•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 * o = 0,                                                                                                                            /*
    0•*•o•=•0    ReassignmentExpression
    0•*•o        OperationExpression
    0            Literal
            0    Literal                                                                                                                  */
	0 * 0 + 0,                                                                                                                            /*
    0•*•0•+•0    OperationExpression
    0•*•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 * 0 * 0,                                                                                                                            /*
    0•*•0•*•0    OperationExpression
    0•*•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 * 0 & 0,                                                                                                                            /*
    0•*•0•&•0    OperationExpression
    0•*•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 * 0 << 0,                                                                                                                           /*
    0•*•0•<<•0    OperationExpression
    0•*•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 * 0 == 0,                                                                                                                           /*
    0•*•0•==•0    ComparisonExpression
    0•*•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 * 0 > 0,                                                                                                                            /*
    0•*•0•>•0    ComparisonExpression
    0•*•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 * 0..0,                                                                                                                             /*
    0•*•0..0    RangeLiteral
    0•*•0       OperationExpression
    0           Literal
        0       Literal
           0    Literal                                                                                                                   */
	0 & 0 && 0,                                                                                                                           /*
    0•&•0•&&•0    AndExpression
    0•&•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 & 0 || 0,                                                                                                                           /*
    0•&•0•||•0    OrExpression
    0•&•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 & o = 0,                                                                                                                            /*
    0•&•o•=•0    ReassignmentExpression
    0•&•o        OperationExpression
    0            Literal
            0    Literal                                                                                                                  */
	0 & 0 + 0,                                                                                                                            /*
    0•&•0•+•0    OperationExpression
    0            Literal
        0•+•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	0 & 0 * 0,                                                                                                                            /*
    0•&•0•*•0    OperationExpression
    0            Literal
        0•*•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	0 & 0 & 0,                                                                                                                            /*
    0•&•0•&•0    OperationExpression
    0•&•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 & 0 << 0,                                                                                                                           /*
    0•&•0•<<•0    OperationExpression
    0             Literal
        0•<<•0    OperationExpression
        0         Literal
             0    Literal                                                                                                                 */
	0 & 0 == 0,                                                                                                                           /*
    0•&•0•==•0    ComparisonExpression
    0•&•0         OperationExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 & 0 > 0,                                                                                                                            /*
    0•&•0•>•0    ComparisonExpression
    0•&•0        OperationExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 & 0..0,                                                                                                                             /*
    0•&•0..0    RangeLiteral
    0•&•0       OperationExpression
    0           Literal
        0       Literal
           0    Literal                                                                                                                   */
	0 << 0 && 0,                                                                                                                          /*
    0•<<•0•&&•0    AndExpression
    0•<<•0         OperationExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 << 0 || 0,                                                                                                                          /*
    0•<<•0•||•0    OrExpression
    0•<<•0         OperationExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 << o = 0,                                                                                                                           /*
    0•<<•o•=•0    ReassignmentExpression
    0•<<•o        OperationExpression
    0             Literal
             0    Literal                                                                                                                 */
	0 << 0 + 0,                                                                                                                           /*
    0•<<•0•+•0    OperationExpression
    0             Literal
         0•+•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 << 0 * 0,                                                                                                                           /*
    0•<<•0•*•0    OperationExpression
    0             Literal
         0•*•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 << 0 & 0,                                                                                                                           /*
    0•<<•0•&•0    OperationExpression
    0•<<•0        OperationExpression
    0             Literal
         0        Literal
             0    Literal                                                                                                                 */
	0 << 0 << 0,                                                                                                                          /*
    0•<<•0•<<•0    OperationExpression
    0•<<•0         OperationExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 << 0 == 0,                                                                                                                          /*
    0•<<•0•==•0    ComparisonExpression
    0•<<•0         OperationExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 << 0 > 0,                                                                                                                           /*
    0•<<•0•>•0    ComparisonExpression
    0•<<•0        OperationExpression
    0             Literal
         0        Literal
             0    Literal                                                                                                                 */
	0 << 0..0,                                                                                                                            /*
    0•<<•0..0    RangeLiteral
    0•<<•0       OperationExpression
    0            Literal
         0       Literal
            0    Literal                                                                                                                  */
	0 == 0 && 0,                                                                                                                          /*
    0•==•0•&&•0    AndExpression
    0•==•0         ComparisonExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 == 0 || 0,                                                                                                                          /*
    0•==•0•||•0    OrExpression
    0•==•0         ComparisonExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 == o = 0,                                                                                                                           /*
    0•==•o•=•0    ReassignmentExpression
    0•==•o        ComparisonExpression
    0             Literal
             0    Literal                                                                                                                 */
	0 == 0 + 0,                                                                                                                           /*
    0•==•0•+•0    ComparisonExpression
    0             Literal
         0•+•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 == 0 * 0,                                                                                                                           /*
    0•==•0•*•0    ComparisonExpression
    0             Literal
         0•*•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 == 0 & 0,                                                                                                                           /*
    0•==•0•&•0    ComparisonExpression
    0             Literal
         0•&•0    OperationExpression
         0        Literal
             0    Literal                                                                                                                 */
	0 == 0 << 0,                                                                                                                          /*
    0•==•0•<<•0    ComparisonExpression
    0              Literal
         0•<<•0    OperationExpression
         0         Literal
              0    Literal                                                                                                                */
	0 == 0 == 0,                                                                                                                          /*
    0•==•0•==•0    ComparisonExpression
    0•==•0         ComparisonExpression
    0              Literal
         0         Literal
              0    Literal                                                                                                                */
	0 == 0 > 0,                                                                                                                           /*
    0•==•0•>•0    ComparisonExpression
    0•==•0        ComparisonExpression
    0             Literal
         0        Literal
             0    Literal                                                                                                                 */
	0 == 0..0,                                                                                                                            /*
    0•==•0..0    RangeLiteral
    0•==•0       ComparisonExpression
    0            Literal
         0       Literal
            0    Literal                                                                                                                  */
	0 > 0 && 0,                                                                                                                           /*
    0•>•0•&&•0    AndExpression
    0•>•0         ComparisonExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 > 0 || 0,                                                                                                                           /*
    0•>•0•||•0    OrExpression
    0•>•0         ComparisonExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 > o = 0,                                                                                                                            /*
    0•>•o•=•0    ReassignmentExpression
    0•>•o        ComparisonExpression
    0            Literal
            0    Literal                                                                                                                  */
	0 > 0 + 0,                                                                                                                            /*
    0•>•0•+•0    ComparisonExpression
    0            Literal
        0•+•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	0 > 0 * 0,                                                                                                                            /*
    0•>•0•*•0    ComparisonExpression
    0            Literal
        0•*•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	0 > 0 & 0,                                                                                                                            /*
    0•>•0•&•0    ComparisonExpression
    0            Literal
        0•&•0    OperationExpression
        0        Literal
            0    Literal                                                                                                                  */
	0 > 0 << 0,                                                                                                                           /*
    0•>•0•<<•0    ComparisonExpression
    0             Literal
        0•<<•0    OperationExpression
        0         Literal
             0    Literal                                                                                                                 */
	0 > 0 == 0,                                                                                                                           /*
    0•>•0•==•0    ComparisonExpression
    0•>•0         ComparisonExpression
    0             Literal
        0         Literal
             0    Literal                                                                                                                 */
	0 > 0 > 0,                                                                                                                            /*
    0•>•0•>•0    ComparisonExpression
    0•>•0        ComparisonExpression
    0            Literal
        0        Literal
            0    Literal                                                                                                                  */
	0 > 0..0,                                                                                                                             /*
    0•>•0..0    RangeLiteral
    0•>•0       ComparisonExpression
    0           Literal
        0       Literal
           0    Literal                                                                                                                   */
	0..0 && 0,                                                                                                                            /*
    0..0•&&•0    RangeLiteral
    0            Literal
       0•&&•0    AndExpression
       0         Literal
            0    Literal                                                                                                                  */
	0..0 || 0,                                                                                                                            /*
    0..0•||•0    RangeLiteral
    0            Literal
       0•||•0    OrExpression
       0         Literal
            0    Literal                                                                                                                  */
	0..o = 0,                                                                                                                             /*
    0..o•=•0    ReassignmentExpression
    0..o        RangeLiteral
    0           Literal
           0    Literal                                                                                                                   */
	0..0 + 0,                                                                                                                             /*
    0..0•+•0    RangeLiteral
    0           Literal
       0•+•0    OperationExpression
       0        Literal
           0    Literal                                                                                                                   */
	0..0 * 0,                                                                                                                             /*
    0..0•*•0    RangeLiteral
    0           Literal
       0•*•0    OperationExpression
       0        Literal
           0    Literal                                                                                                                   */
	0..0 & 0,                                                                                                                             /*
    0..0•&•0    RangeLiteral
    0           Literal
       0•&•0    OperationExpression
       0        Literal
           0    Literal                                                                                                                   */
	0..0 << 0,                                                                                                                            /*
    0..0•<<•0    RangeLiteral
    0            Literal
       0•<<•0    OperationExpression
       0         Literal
            0    Literal                                                                                                                  */
	0..0 == 0,                                                                                                                            /*
    0..0•==•0    RangeLiteral
    0            Literal
       0•==•0    ComparisonExpression
       0         Literal
            0    Literal                                                                                                                  */
	0..0 > 0,                                                                                                                             /*
    0..0•>•0    RangeLiteral
    0           Literal
       0•>•0    ComparisonExpression
       0        Literal
           0    Literal                                                                                                                   */
	0..0..0,                                                                                                                              /*
    0..0..0    RangeLiteral
    0..0       RangeLiteral
    0          Literal
       0       Literal
          0    Literal                                                                                                                    */
	// 0 x (0 x 0) ===========================================================
    //•0•x•(0•x•0)•===========================================================    Comment
	0 && (0 && 0),                                                                                                                        /*
    0•&&•(0•&&•0)    AndExpression
    0                Literal
          0•&&•0     AndExpression
          0          Literal
               0     Literal                                                                                                              */
	0 && (0 || 0),                                                                                                                        /*
    0•&&•(0•||•0)    AndExpression
    0                Literal
          0•||•0     OrExpression
          0          Literal
               0     Literal                                                                                                              */
	0 && (o = 0),                                                                                                                         /*
    0•&&•(o•=•0)    AndExpression
    0               Literal
          o•=•0     ReassignmentExpression
              0     Literal                                                                                                               */
	0 && (0 + 0),                                                                                                                         /*
    0•&&•(0•+•0)    AndExpression
    0               Literal
          0•+•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 && (0 * 0),                                                                                                                         /*
    0•&&•(0•*•0)    AndExpression
    0               Literal
          0•*•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 && (0 & 0),                                                                                                                         /*
    0•&&•(0•&•0)    AndExpression
    0               Literal
          0•&•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 && (0 << 0),                                                                                                                        /*
    0•&&•(0•<<•0)    AndExpression
    0                Literal
          0•<<•0     OperationExpression
          0          Literal
               0     Literal                                                                                                              */
	0 && (0 == 0),                                                                                                                        /*
    0•&&•(0•==•0)    AndExpression
    0                Literal
          0•==•0     ComparisonExpression
          0          Literal
               0     Literal                                                                                                              */
	0 && (0 > 0),                                                                                                                         /*
    0•&&•(0•>•0)    AndExpression
    0               Literal
          0•>•0     ComparisonExpression
          0         Literal
              0     Literal                                                                                                               */
	0 && (0..0),                                                                                                                          /*
    0•&&•(0..0)    AndExpression
    0              Literal
          0..0     RangeLiteral
          0        Literal
             0     Literal                                                                                                                */
	0 || (0 && 0),                                                                                                                        /*
    0•||•(0•&&•0)    OrExpression
    0                Literal
          0•&&•0     AndExpression
          0          Literal
               0     Literal                                                                                                              */
	0 || (0 || 0),                                                                                                                        /*
    0•||•(0•||•0)    OrExpression
    0                Literal
          0•||•0     OrExpression
          0          Literal
               0     Literal                                                                                                              */
	0 || (o = 0),                                                                                                                         /*
    0•||•(o•=•0)    OrExpression
    0               Literal
          o•=•0     ReassignmentExpression
              0     Literal                                                                                                               */
	0 || (0 + 0),                                                                                                                         /*
    0•||•(0•+•0)    OrExpression
    0               Literal
          0•+•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 || (0 * 0),                                                                                                                         /*
    0•||•(0•*•0)    OrExpression
    0               Literal
          0•*•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 || (0 & 0),                                                                                                                         /*
    0•||•(0•&•0)    OrExpression
    0               Literal
          0•&•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 || (0 << 0),                                                                                                                        /*
    0•||•(0•<<•0)    OrExpression
    0                Literal
          0•<<•0     OperationExpression
          0          Literal
               0     Literal                                                                                                              */
	0 || (0 == 0),                                                                                                                        /*
    0•||•(0•==•0)    OrExpression
    0                Literal
          0•==•0     ComparisonExpression
          0          Literal
               0     Literal                                                                                                              */
	0 || (0 > 0),                                                                                                                         /*
    0•||•(0•>•0)    OrExpression
    0               Literal
          0•>•0     ComparisonExpression
          0         Literal
              0     Literal                                                                                                               */
	0 || (0..0),                                                                                                                          /*
    0•||•(0..0)    OrExpression
    0              Literal
          0..0     RangeLiteral
          0        Literal
             0     Literal                                                                                                                */
	o = (0 && 0),                                                                                                                         /*
    o•=•(0•&&•0)    ReassignmentExpression
         0•&&•0     AndExpression
         0          Literal
              0     Literal                                                                                                               */
	o = (0 || 0),                                                                                                                         /*
    o•=•(0•||•0)    ReassignmentExpression
         0•||•0     OrExpression
         0          Literal
              0     Literal                                                                                                               */
	o = (o = 0),                                                                                                                          /*
    o•=•(o•=•0)    ReassignmentExpression
         o•=•0     ReassignmentExpression
             0     Literal                                                                                                                */
	o = (0 + 0),                                                                                                                          /*
    o•=•(0•+•0)    ReassignmentExpression
         0•+•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	o = (0 * 0),                                                                                                                          /*
    o•=•(0•*•0)    ReassignmentExpression
         0•*•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	o = (0 & 0),                                                                                                                          /*
    o•=•(0•&•0)    ReassignmentExpression
         0•&•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	o = (0 << 0),                                                                                                                         /*
    o•=•(0•<<•0)    ReassignmentExpression
         0•<<•0     OperationExpression
         0          Literal
              0     Literal                                                                                                               */
	o = (0 == 0),                                                                                                                         /*
    o•=•(0•==•0)    ReassignmentExpression
         0•==•0     ComparisonExpression
         0          Literal
              0     Literal                                                                                                               */
	o = (0 > 0),                                                                                                                          /*
    o•=•(0•>•0)    ReassignmentExpression
         0•>•0     ComparisonExpression
         0         Literal
             0     Literal                                                                                                                */
	o = (0..0),                                                                                                                           /*
    o•=•(0..0)    ReassignmentExpression
         0..0     RangeLiteral
         0        Literal
            0     Literal                                                                                                                 */
	0 + (0 && 0),                                                                                                                         /*
    0•+•(0•&&•0)    OperationExpression
    0               Literal
         0•&&•0     AndExpression
         0          Literal
              0     Literal                                                                                                               */
	0 + (0 || 0),                                                                                                                         /*
    0•+•(0•||•0)    OperationExpression
    0               Literal
         0•||•0     OrExpression
         0          Literal
              0     Literal                                                                                                               */
	0 + (o = 0),                                                                                                                          /*
    0•+•(o•=•0)    OperationExpression
    0              Literal
         o•=•0     ReassignmentExpression
             0     Literal                                                                                                                */
	0 + (0 + 0),                                                                                                                          /*
    0•+•(0•+•0)    OperationExpression
    0              Literal
         0•+•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 + (0 * 0),                                                                                                                          /*
    0•+•(0•*•0)    OperationExpression
    0              Literal
         0•*•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 + (0 & 0),                                                                                                                          /*
    0•+•(0•&•0)    OperationExpression
    0              Literal
         0•&•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 + (0 << 0),                                                                                                                         /*
    0•+•(0•<<•0)    OperationExpression
    0               Literal
         0•<<•0     OperationExpression
         0          Literal
              0     Literal                                                                                                               */
	0 + (0 == 0),                                                                                                                         /*
    0•+•(0•==•0)    OperationExpression
    0               Literal
         0•==•0     ComparisonExpression
         0          Literal
              0     Literal                                                                                                               */
	0 + (0 > 0),                                                                                                                          /*
    0•+•(0•>•0)    OperationExpression
    0              Literal
         0•>•0     ComparisonExpression
         0         Literal
             0     Literal                                                                                                                */
	0 + (0..0),                                                                                                                           /*
    0•+•(0..0)    OperationExpression
    0             Literal
         0..0     RangeLiteral
         0        Literal
            0     Literal                                                                                                                 */
	0 * (0 && 0),                                                                                                                         /*
    0•*•(0•&&•0)    OperationExpression
    0               Literal
         0•&&•0     AndExpression
         0          Literal
              0     Literal                                                                                                               */
	0 * (0 || 0),                                                                                                                         /*
    0•*•(0•||•0)    OperationExpression
    0               Literal
         0•||•0     OrExpression
         0          Literal
              0     Literal                                                                                                               */
	0 * (o = 0),                                                                                                                          /*
    0•*•(o•=•0)    OperationExpression
    0              Literal
         o•=•0     ReassignmentExpression
             0     Literal                                                                                                                */
	0 * (0 + 0),                                                                                                                          /*
    0•*•(0•+•0)    OperationExpression
    0              Literal
         0•+•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 * (0 * 0),                                                                                                                          /*
    0•*•(0•*•0)    OperationExpression
    0              Literal
         0•*•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 * (0 & 0),                                                                                                                          /*
    0•*•(0•&•0)    OperationExpression
    0              Literal
         0•&•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 * (0 << 0),                                                                                                                         /*
    0•*•(0•<<•0)    OperationExpression
    0               Literal
         0•<<•0     OperationExpression
         0          Literal
              0     Literal                                                                                                               */
	0 * (0 == 0),                                                                                                                         /*
    0•*•(0•==•0)    OperationExpression
    0               Literal
         0•==•0     ComparisonExpression
         0          Literal
              0     Literal                                                                                                               */
	0 * (0 > 0),                                                                                                                          /*
    0•*•(0•>•0)    OperationExpression
    0              Literal
         0•>•0     ComparisonExpression
         0         Literal
             0     Literal                                                                                                                */
	0 * (0..0),                                                                                                                           /*
    0•*•(0..0)    OperationExpression
    0             Literal
         0..0     RangeLiteral
         0        Literal
            0     Literal                                                                                                                 */
	0 & (0 && 0),                                                                                                                         /*
    0•&•(0•&&•0)    OperationExpression
    0               Literal
         0•&&•0     AndExpression
         0          Literal
              0     Literal                                                                                                               */
	0 & (0 || 0),                                                                                                                         /*
    0•&•(0•||•0)    OperationExpression
    0               Literal
         0•||•0     OrExpression
         0          Literal
              0     Literal                                                                                                               */
	0 & (o = 0),                                                                                                                          /*
    0•&•(o•=•0)    OperationExpression
    0              Literal
         o•=•0     ReassignmentExpression
             0     Literal                                                                                                                */
	0 & (0 + 0),                                                                                                                          /*
    0•&•(0•+•0)    OperationExpression
    0              Literal
         0•+•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 & (0 * 0),                                                                                                                          /*
    0•&•(0•*•0)    OperationExpression
    0              Literal
         0•*•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 & (0 & 0),                                                                                                                          /*
    0•&•(0•&•0)    OperationExpression
    0              Literal
         0•&•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 & (0 << 0),                                                                                                                         /*
    0•&•(0•<<•0)    OperationExpression
    0               Literal
         0•<<•0     OperationExpression
         0          Literal
              0     Literal                                                                                                               */
	0 & (0 == 0),                                                                                                                         /*
    0•&•(0•==•0)    OperationExpression
    0               Literal
         0•==•0     ComparisonExpression
         0          Literal
              0     Literal                                                                                                               */
	0 & (0 > 0),                                                                                                                          /*
    0•&•(0•>•0)    OperationExpression
    0              Literal
         0•>•0     ComparisonExpression
         0         Literal
             0     Literal                                                                                                                */
	0 & (0..0),                                                                                                                           /*
    0•&•(0..0)    OperationExpression
    0             Literal
         0..0     RangeLiteral
         0        Literal
            0     Literal                                                                                                                 */
	0 << (0 && 0),                                                                                                                        /*
    0•<<•(0•&&•0)    OperationExpression
    0                Literal
          0•&&•0     AndExpression
          0          Literal
               0     Literal                                                                                                              */
	0 << (0 || 0),                                                                                                                        /*
    0•<<•(0•||•0)    OperationExpression
    0                Literal
          0•||•0     OrExpression
          0          Literal
               0     Literal                                                                                                              */
	0 << (o = 0),                                                                                                                         /*
    0•<<•(o•=•0)    OperationExpression
    0               Literal
          o•=•0     ReassignmentExpression
              0     Literal                                                                                                               */
	0 << (0 + 0),                                                                                                                         /*
    0•<<•(0•+•0)    OperationExpression
    0               Literal
          0•+•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 << (0 * 0),                                                                                                                         /*
    0•<<•(0•*•0)    OperationExpression
    0               Literal
          0•*•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 << (0 & 0),                                                                                                                         /*
    0•<<•(0•&•0)    OperationExpression
    0               Literal
          0•&•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 << (0 << 0),                                                                                                                        /*
    0•<<•(0•<<•0)    OperationExpression
    0                Literal
          0•<<•0     OperationExpression
          0          Literal
               0     Literal                                                                                                              */
	0 << (0 == 0),                                                                                                                        /*
    0•<<•(0•==•0)    OperationExpression
    0                Literal
          0•==•0     ComparisonExpression
          0          Literal
               0     Literal                                                                                                              */
	0 << (0 > 0),                                                                                                                         /*
    0•<<•(0•>•0)    OperationExpression
    0               Literal
          0•>•0     ComparisonExpression
          0         Literal
              0     Literal                                                                                                               */
	0 << (0..0),                                                                                                                          /*
    0•<<•(0..0)    OperationExpression
    0              Literal
          0..0     RangeLiteral
          0        Literal
             0     Literal                                                                                                                */
	0 == (0 && 0),                                                                                                                        /*
    0•==•(0•&&•0)    ComparisonExpression
    0                Literal
          0•&&•0     AndExpression
          0          Literal
               0     Literal                                                                                                              */
	0 == (0 || 0),                                                                                                                        /*
    0•==•(0•||•0)    ComparisonExpression
    0                Literal
          0•||•0     OrExpression
          0          Literal
               0     Literal                                                                                                              */
	0 == (o = 0),                                                                                                                         /*
    0•==•(o•=•0)    ComparisonExpression
    0               Literal
          o•=•0     ReassignmentExpression
              0     Literal                                                                                                               */
	0 == (0 + 0),                                                                                                                         /*
    0•==•(0•+•0)    ComparisonExpression
    0               Literal
          0•+•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 == (0 * 0),                                                                                                                         /*
    0•==•(0•*•0)    ComparisonExpression
    0               Literal
          0•*•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 == (0 & 0),                                                                                                                         /*
    0•==•(0•&•0)    ComparisonExpression
    0               Literal
          0•&•0     OperationExpression
          0         Literal
              0     Literal                                                                                                               */
	0 == (0 << 0),                                                                                                                        /*
    0•==•(0•<<•0)    ComparisonExpression
    0                Literal
          0•<<•0     OperationExpression
          0          Literal
               0     Literal                                                                                                              */
	0 == (0 == 0),                                                                                                                        /*
    0•==•(0•==•0)    ComparisonExpression
    0                Literal
          0•==•0     ComparisonExpression
          0          Literal
               0     Literal                                                                                                              */
	0 == (0 > 0),                                                                                                                         /*
    0•==•(0•>•0)    ComparisonExpression
    0               Literal
          0•>•0     ComparisonExpression
          0         Literal
              0     Literal                                                                                                               */
	0 == (0..0),                                                                                                                          /*
    0•==•(0..0)    ComparisonExpression
    0              Literal
          0..0     RangeLiteral
          0        Literal
             0     Literal                                                                                                                */
	0 > (0 && 0),                                                                                                                         /*
    0•>•(0•&&•0)    ComparisonExpression
    0               Literal
         0•&&•0     AndExpression
         0          Literal
              0     Literal                                                                                                               */
	0 > (0 || 0),                                                                                                                         /*
    0•>•(0•||•0)    ComparisonExpression
    0               Literal
         0•||•0     OrExpression
         0          Literal
              0     Literal                                                                                                               */
	0 > (o = 0),                                                                                                                          /*
    0•>•(o•=•0)    ComparisonExpression
    0              Literal
         o•=•0     ReassignmentExpression
             0     Literal                                                                                                                */
	0 > (0 + 0),                                                                                                                          /*
    0•>•(0•+•0)    ComparisonExpression
    0              Literal
         0•+•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 > (0 * 0),                                                                                                                          /*
    0•>•(0•*•0)    ComparisonExpression
    0              Literal
         0•*•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 > (0 & 0),                                                                                                                          /*
    0•>•(0•&•0)    ComparisonExpression
    0              Literal
         0•&•0     OperationExpression
         0         Literal
             0     Literal                                                                                                                */
	0 > (0 << 0),                                                                                                                         /*
    0•>•(0•<<•0)    ComparisonExpression
    0               Literal
         0•<<•0     OperationExpression
         0          Literal
              0     Literal                                                                                                               */
	0 > (0 == 0),                                                                                                                         /*
    0•>•(0•==•0)    ComparisonExpression
    0               Literal
         0•==•0     ComparisonExpression
         0          Literal
              0     Literal                                                                                                               */
	0 > (0 > 0),                                                                                                                          /*
    0•>•(0•>•0)    ComparisonExpression
    0              Literal
         0•>•0     ComparisonExpression
         0         Literal
             0     Literal                                                                                                                */
	0 > (0..0)                                                                                                                            /*
    0•>•(0..0)    ComparisonExpression
    0             Literal
         0..0     RangeLiteral
         0        Literal
            0     Literal                                                                                                                 */
];                                                                                                                                        /*
];    </ExpressionStatement>
]     </ArrayLiteral>                                                                                                                     */

// Discarded Nodes: 248
// Parsed Nodes: 3046
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 11763 (33% re-reads)
// Unnecessary 'skip_whitespace()' calls: 1062
// source: "../../samples/expressions/precedence.rs"