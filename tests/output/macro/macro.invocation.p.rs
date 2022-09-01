
declare_clippy_lint! {                                                                                                                    /*
declare_clippy_lint!•{↲    <Program>
declare_clippy_lint!•{↲    <Program.ast{dk: "None"}>
declare_clippy_lint!•{↲    <ExpressionStatement{!semi}>
declare_clippy_lint!•{↲    <MacroInvocation>
                     {↲    <MacroInvocation.segments{dk: "{}"}>                                                                           */
    /// ### q
    ///•###•q    Comment{line}
    /// ```rust
    ///•```rust    Comment{line}
    /// let b: Vec<&str> = vec![];
    ///•let•b:•Vec<&str>•=•vec![];    Comment{line}
    /// if !b.is_empty() {
    ///•if•!b.is_empty()•{    Comment{line}
    ///     panic!(q: {:?}", b);
    ///•••••panic!(q:•{:?}",•b);    Comment{line}
    /// }
    ///•}    Comment{line}
    /// ```
    ///•```    Comment{line}
    /// Use instead:
    ///•Use•instead:    Comment{line}
    /// ```rust
    ///•```rust    Comment{line}
    /// let b: Vec<&str> = vec![];
    ///•let•b:•Vec<&str>•=•vec![];    Comment{line}
    /// assert!(b.is_empty(), "there are sad people: {:?}", b);
    ///•assert!(b.is_empty(),•"there•are•sad•people:•{:?}",•b);    Comment{line}
    /// ```
    ///•```    Comment{line}
    #[clippy::version = "1.57.0"]                                                                                                         /*
    #                                PunctuationToken{tk: "#"}
     [clippy::version•=•"1.57.0"]    DelimGroup
            ::                       PunctuationToken{tk: "::"}
                      =              PunctuationToken{tk: "="}
                        "1.57.0"     Literal{kind: String}                                                                                */
    pub MANUAL_ASSERT,                                                                                                                    /*
                     ,    PunctuationToken{tk: ","}                                                                                       */
    pedantic,                                                                                                                             /*
            ,    PunctuationToken{tk: ","}                                                                                                */
    "`panic!` and only a `panic!` in `if`-then statement"                                                                                 /*
    "`panic!`•and•only•a•`panic!`•in•`if`-then•statement"    Literal{kind: String}                                                        */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */


fn aux<Xs: HList, Ys: HList>(xs: Xs, ys: Ys) -> Expr!(Xs + Ys) where                                                                      /*
fn•aux<Xs:•HList,•Ys:•HList>(xs:•Xs,•ys:•Ys)•->•Expr!(Xs•+•Ys)•where↲    <FunctionDeclaration>
      <Xs:•HList,•Ys:•HList>                                             FunctionDeclaration.generics{dk: "<>"}
       Xs:•HList                                                         GenericTypeParameterDeclaration
           HList                                                         TypeTraitBound{!maybeConst, !optional}
                  Ys:•HList                                              GenericTypeParameterDeclaration
                      HList                                              TypeTraitBound{!maybeConst, !optional}
                            (xs:•Xs,•ys:•Ys)                             FunctionDeclaration.parameters{dk: "()"}
                             xs:•Xs                                      FunctionParameterDeclaration
                                     ys:•Ys                              FunctionParameterDeclaration
                                                Expr!(Xs•+•Ys)           MacroInvocation
                                                     (Xs•+•Ys)           MacroInvocation.segments{dk: "()"}
                                                         +               PunctuationToken{tk: "+"}
                                                               where↲    <FunctionDeclaration.whereBounds{dk: "None"}>                    */
Xs: Add<Ys> {                                                                                                                             /*
Xs:•Add<Ys>       WhereTypeBoundDeclaration
    Add<Ys>       TypeTraitBound{!maybeConst, !optional}, TypeCall
       <Ys>       TypeCall.typeArguments{dk: "<>"}
Xs:•Add<Ys>       </FunctionDeclaration.whereBounds>
            {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                    */
    xs + ys                                                                                                                               /*
    xs•+•ys    ExpressionStatement{!semi}, OperationExpression{tk: "+"}                                                                   */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

declare_lint_pass!(Attributes => [                                                                                                        /*
declare_lint_pass!(Attributes•=>•[↲    <ExpressionStatement{semi}>
declare_lint_pass!(Attributes•=>•[↲    <MacroInvocation>
                  (Attributes•=>•[↲    <MacroInvocation.segments{dk: "()"}>
                              =>       PunctuationToken{tk: "=>"}
                                 [↲    <DelimGroup>                                                                                       */
    INLINE_ALWAYS,                                                                                                                        /*
                 ,    PunctuationToken{tk: ","}                                                                                           */
    DEPRECATED_SEMVER,                                                                                                                    /*
                     ,    PunctuationToken{tk: ","}                                                                                       */
    USELESS_ATTRIBUTE,                                                                                                                    /*
                     ,    PunctuationToken{tk: ","}                                                                                       */
    BLANKET_CLIPPY_RESTRICTION_LINTS,                                                                                                     /*
                                    ,    PunctuationToken{tk: ","}                                                                        */
]);                                                                                                                                       /*
]      </DelimGroup>
])     </MacroInvocation.segments>
])     </MacroInvocation>
]);    </ExpressionStatement>                                                                                                             */

fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>
       ()       FunctionDeclaration.parameters{dk: "()"}
          {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                      */
    let xs: HList!(&str, bool, Vec<u64>) = hlist!("foo", false, vec![]);                                                                  /*
    let•xs:•HList!(&str,•bool,•Vec<u64>)•=•hlist!("foo",•false,•vec![]);    LetVariableDeclaration
            HList!(&str,•bool,•Vec<u64>)                                    MacroInvocation
                  (&str,•bool,•Vec<u64>)                                    MacroInvocation.segments{dk: "()"}
                   &                                                        PunctuationToken{tk: "&"}
                       ,                                                    PunctuationToken{tk: ","}
                             ,                                              PunctuationToken{tk: ","}
                                  <                                         PunctuationToken{tk: "<"}
                                      >                                     PunctuationToken{tk: ">"}
                                           hlist!("foo",•false,•vec![])     MacroInvocation
                                                 ("foo",•false,•vec![])     MacroInvocation.segments{dk: "()"}
                                                  "foo"                     Literal{kind: String}
                                                       ,                    PunctuationToken{tk: ","}
                                                         false              Literal{kind: False}
                                                              ,             PunctuationToken{tk: ","}
                                                                   !        PunctuationToken{tk: "!"}
                                                                    []      DelimGroup                                                    */
    let ys: HList!(u64, [u8; 3], ()) = hlist!(0, [0, 1, 2], ());                                                                          /*
    let•ys:•HList!(u64,•[u8;•3],•())•=•hlist!(0,•[0,•1,•2],•());    LetVariableDeclaration
            HList!(u64,•[u8;•3],•())                                MacroInvocation
                  (u64,•[u8;•3],•())                                MacroInvocation.segments{dk: "()"}
                      ,                                             PunctuationToken{tk: ","}
                        [u8;•3]                                     DelimGroup
                           ;                                        PunctuationToken{tk: ";"}
                             3                                      Literal{kind: Integer}
                               ,                                    PunctuationToken{tk: ","}
                                 ()                                 DelimGroup
                                       hlist!(0,•[0,•1,•2],•())     MacroInvocation
                                             (0,•[0,•1,•2],•())     MacroInvocation.segments{dk: "()"}
                                              0                     Literal{kind: Integer}
                                               ,                    PunctuationToken{tk: ","}
                                                 [0,•1,•2]          DelimGroup
                                                  0                 Literal{kind: Integer}
                                                   ,                PunctuationToken{tk: ","}
                                                     1              Literal{kind: Integer}
                                                      ,             PunctuationToken{tk: ","}
                                                        2           Literal{kind: Integer}
                                                          ,         PunctuationToken{tk: ","}
                                                            ()      DelimGroup                                                            */
    let zs: Expr!((HList![&str] + HList![bool] + HList![Vec<u64>]) + (HList![u64] + HList![[u8; 3], ()]) + HList![]) = aux(xs, ys);       /*
    let•zs:•Expr!((HList![&str]•+•HList![bool]•+•HList![Vec<u64>])•+•(HList![u64]•+•HList![[u8;•3],•()])•+•HList![])•=•aux(xs,•ys);    LetVariableDeclaration
            Expr!((HList![&str]•+•HList![bool]•+•HList![Vec<u64>])•+•(HList![u64]•+•HList![[u8;•3],•()])•+•HList![])                   MacroInvocation
                 ((HList![&str]•+•HList![bool]•+•HList![Vec<u64>])•+•(HList![u64]•+•HList![[u8;•3],•()])•+•HList![])                   MacroInvocation.segments{dk: "()"}
                  (HList![&str]•+•HList![bool]•+•HList![Vec<u64>])                                                                     DelimGroup
                        !                                                                                                              PunctuationToken{tk: "!"}
                         [&str]                                                                                                        DelimGroup
                          &                                                                                                            PunctuationToken{tk: "&"}
                                +                                                                                                      PunctuationToken{tk: "+"}
                                       !                                                                                               PunctuationToken{tk: "!"}
                                        [bool]                                                                                         DelimGroup
                                               +                                                                                       PunctuationToken{tk: "+"}
                                                      !                                                                                PunctuationToken{tk: "!"}
                                                       [Vec<u64>]                                                                      DelimGroup
                                                           <                                                                           PunctuationToken{tk: "<"}
                                                               >                                                                       PunctuationToken{tk: ">"}
                                                                   +                                                                   PunctuationToken{tk: "+"}
                                                                     (HList![u64]•+•HList![[u8;•3],•()])                               DelimGroup
                                                                           !                                                           PunctuationToken{tk: "!"}
                                                                            [u64]                                                      DelimGroup
                                                                                  +                                                    PunctuationToken{tk: "+"}
                                                                                         !                                             PunctuationToken{tk: "!"}
                                                                                          [[u8;•3],•()]                                DelimGroup
                                                                                           [u8;•3]                                     DelimGroup
                                                                                              ;                                        PunctuationToken{tk: ";"}
                                                                                                3                                      Literal{kind: Integer}
                                                                                                  ,                                    PunctuationToken{tk: ","}
                                                                                                    ()                                 DelimGroup
                                                                                                         +                             PunctuationToken{tk: "+"}
                                                                                                                !                      PunctuationToken{tk: "!"}
                                                                                                                 []                    DelimGroup
                                                                                                                       aux(xs,•ys)     CallExpression
                                                                                                                          (xs,•ys)     CallExpression.arguments{dk: "()"}*/
    higher_order!(subst ($x:expr, $y:expr, $foo:expr) => (($x + $y, $foo)));                                                              /*
    higher_order!(subst•($x:expr,•$y:expr,•$foo:expr)•=>•(($x•+•$y,•$foo)));    ExpressionStatement{semi}
    higher_order!(subst•($x:expr,•$y:expr,•$foo:expr)•=>•(($x•+•$y,•$foo)))     MacroInvocation
                 (subst•($x:expr,•$y:expr,•$foo:expr)•=>•(($x•+•$y,•$foo)))     MacroInvocation.segments{dk: "()"}
                        ($x:expr,•$y:expr,•$foo:expr)                           DelimGroup
                         $                                                      PunctuationToken{tk: "$"}
                           :                                                    PunctuationToken{tk: ":"}
                                ,                                               PunctuationToken{tk: ","}
                                  $                                             PunctuationToken{tk: "$"}
                                    :                                           PunctuationToken{tk: ":"}
                                         ,                                      PunctuationToken{tk: ","}
                                           $                                    PunctuationToken{tk: "$"}
                                               :                                PunctuationToken{tk: ":"}
                                                      =>                        PunctuationToken{tk: "=>"}
                                                         (($x•+•$y,•$foo))      DelimGroup
                                                          ($x•+•$y,•$foo)       DelimGroup
                                                           $                    PunctuationToken{tk: "$"}
                                                              +                 PunctuationToken{tk: "+"}
                                                                $               PunctuationToken{tk: "$"}
                                                                  ,             PunctuationToken{tk: ","}
                                                                    $           PunctuationToken{tk: "$"}                                 */
    assert_eq!(zs, hlist!["foo", false, vec![], 0, [0, 1, 2], ()]);                                                                       /*
    assert_eq!(zs,•hlist!["foo",•false,•vec![],•0,•[0,•1,•2],•()]);    ExpressionStatement{semi}
    assert_eq!(zs,•hlist!["foo",•false,•vec![],•0,•[0,•1,•2],•()])     MacroInvocation
              (zs,•hlist!["foo",•false,•vec![],•0,•[0,•1,•2],•()])     MacroInvocation.segments{dk: "()"}
                 ,                                                     PunctuationToken{tk: ","}
                        !                                              PunctuationToken{tk: "!"}
                         ["foo",•false,•vec![],•0,•[0,•1,•2],•()]      DelimGroup
                          "foo"                                        Literal{kind: String}
                               ,                                       PunctuationToken{tk: ","}
                                 false                                 Literal{kind: False}
                                      ,                                PunctuationToken{tk: ","}
                                           !                           PunctuationToken{tk: "!"}
                                            []                         DelimGroup
                                              ,                        PunctuationToken{tk: ","}
                                                0                      Literal{kind: Integer}
                                                 ,                     PunctuationToken{tk: ","}
                                                   [0,•1,•2]           DelimGroup
                                                    0                  Literal{kind: Integer}
                                                     ,                 PunctuationToken{tk: ","}
                                                       1               Literal{kind: Integer}
                                                        ,              PunctuationToken{tk: ","}
                                                          2            Literal{kind: Integer}
                                                            ,          PunctuationToken{tk: ","}
                                                              ()       DelimGroup                                                         */
	quote!(fn $name() -> bool { true });                                                                                                  /*
	quote!(fn•$name()•->•bool•{•true•});    ExpressionStatement{semi}
	quote!(fn•$name()•->•bool•{•true•})     MacroInvocation
	      (fn•$name()•->•bool•{•true•})     MacroInvocation.segments{dk: "()"}
	          $                             PunctuationToken{tk: "$"}
	               ()                       DelimGroup
	                  ->                    PunctuationToken{tk: "->"}
	                          {•true•}      DelimGroup
	                            true        Literal{kind: True}                                                                           */
    impl_lint_pass!(Arithmetic => [INTEGER_ARITHMETIC, FLOAT_ARITHMETIC]);                                                                /*
    impl_lint_pass!(Arithmetic•=>•[INTEGER_ARITHMETIC,•FLOAT_ARITHMETIC]);    ExpressionStatement{semi}
    impl_lint_pass!(Arithmetic•=>•[INTEGER_ARITHMETIC,•FLOAT_ARITHMETIC])     MacroInvocation
                   (Arithmetic•=>•[INTEGER_ARITHMETIC,•FLOAT_ARITHMETIC])     MacroInvocation.segments{dk: "()"}
                               =>                                             PunctuationToken{tk: "=>"}
                                  [INTEGER_ARITHMETIC,•FLOAT_ARITHMETIC]      DelimGroup
                                                     ,                        PunctuationToken{tk: ","}                                   */
	let result = quote! {                                                                                                                 /*
	let•result•=•quote!•{↲    <LetVariableDeclaration>
	             quote!•{↲    <MacroInvocation>
	                    {↲    <MacroInvocation.segments{dk: "{}"}>                                                                        */
        macro_rules! generated_foo {                                                                                                      /*
                   !                     PunctuationToken{tk: "!"}
                                   {↲    <DelimGroup>                                                                                     */
            (1 $$x:expr $$($$y:tt,)* $$(= $$z:tt)*) => {};                                                                                /*
            (1•$$x:expr•$$($$y:tt,)*•$$(=•$$z:tt)*)           DelimGroup
             1                                                Literal{kind: Integer}
               $                                              PunctuationToken{tk: "$"}
                $                                             PunctuationToken{tk: "$"}
                  :                                           PunctuationToken{tk: ":"}
                        $                                     PunctuationToken{tk: "$"}
                         $                                    PunctuationToken{tk: "$"}
                          ($$y:tt,)                           DelimGroup
                           $                                  PunctuationToken{tk: "$"}
                            $                                 PunctuationToken{tk: "$"}
                              :                               PunctuationToken{tk: ":"}
                                 ,                            PunctuationToken{tk: ","}
                                   *                          PunctuationToken{tk: "*"}
                                     $                        PunctuationToken{tk: "$"}
                                      $                       PunctuationToken{tk: "$"}
                                       (=•$$z:tt)             DelimGroup
                                        =                     PunctuationToken{tk: "="}
                                          $                   PunctuationToken{tk: "$"}
                                           $                  PunctuationToken{tk: "$"}
                                             :                PunctuationToken{tk: ":"}
                                                 *            PunctuationToken{tk: "*"}
                                                    =>        PunctuationToken{tk: "=>"}
                                                       {}     DelimGroup
                                                         ;    PunctuationToken{tk: ";"}                                                   */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    };                                                                                                                                    /*
••••}     </MacroInvocation.segments>
••••}     </MacroInvocation>
••••};    </LetVariableDeclaration>                                                                                                       */
    if !matches!(                                                                                                                         /*
    if•!matches!(↲    <ExpressionStatement{!semi}>
    if•!matches!(↲    <IfBlockExpression>
       !matches!(↲    <NotExpression>
        matches!(↲    <MacroInvocation>
                (↲    <MacroInvocation.segments{dk: "()"}>                                                                                */
        macro_name.as_str(),                                                                                                              /*
                  .             PunctuationToken{tk: "."}
                         ()     DelimGroup
                           ,    PunctuationToken{tk: ","}                                                                                 */
        "assert_eq" | "debug_assert_eq" | "assert_ne" | "debug_assert_ne"                                                                 /*
        "assert_eq"                                                          Literal{kind: String}
                    |                                                        PunctuationToken{tk: "|"}
                      "debug_assert_eq"                                      Literal{kind: String}
                                        |                                    PunctuationToken{tk: "|"}
                                          "assert_ne"                        Literal{kind: String}
                                                      |                      PunctuationToken{tk: "|"}
                                                        "debug_assert_ne"    Literal{kind: String}                                        */
    ) {}                                                                                                                                  /*
••••)       </MacroInvocation.segments>
••••)       </MacroInvocation>
••••)       </NotExpression>
      {}    IfBlockExpression.body{dk: "{}"}
••••)•{}    </IfBlockExpression>
••••)•{}    </ExpressionStatement>                                                                                                        */
    if matches!(cx.tcx.type_of(id).kind(), ty::Adt(adt, _) if ty_adt.did() == adt.did()) {}                                               /*
    if•matches!(cx.tcx.type_of(id).kind(),•ty::Adt(adt,•_)•if•ty_adt.did()•==•adt.did())•{}    ExpressionStatement{!semi}, IfBlockExpression
       matches!(cx.tcx.type_of(id).kind(),•ty::Adt(adt,•_)•if•ty_adt.did()•==•adt.did())       MacroInvocation
               (cx.tcx.type_of(id).kind(),•ty::Adt(adt,•_)•if•ty_adt.did()•==•adt.did())       MacroInvocation.segments{dk: "()"}
                  .                                                                            PunctuationToken{tk: "."}
                      .                                                                        PunctuationToken{tk: "."}
                              (id)                                                             DelimGroup
                                  .                                                            PunctuationToken{tk: "."}
                                       ()                                                      DelimGroup
                                         ,                                                     PunctuationToken{tk: ","}
                                             ::                                                PunctuationToken{tk: "::"}
                                                  (adt,•_)                                     DelimGroup
                                                      ,                                        PunctuationToken{tk: ","}
                                                        _                                      PunctuationToken{tk: "_"}
                                                                    .                          PunctuationToken{tk: "."}
                                                                        ()                     DelimGroup
                                                                           ==                  PunctuationToken{tk: "=="}
                                                                                 .             PunctuationToken{tk: "."}
                                                                                     ()        DelimGroup
                                                                                         {}    IfBlockExpression.body{dk: "{}"}           */
    if !matches!(                                                                                                                         /*
    if•!matches!(↲    <ExpressionStatement{!semi}>
    if•!matches!(↲    <IfBlockExpression>
       !matches!(↲    <NotExpression>
        matches!(↲    <MacroInvocation>
                (↲    <MacroInvocation.segments{dk: "()"}>                                                                                */
        get_expr_use_or_unification_node(tcx, expr),                                                                                      /*
                                        (tcx,•expr)     DelimGroup
                                            ,           PunctuationToken{tk: ","}
                                                   ,    PunctuationToken{tk: ","}                                                         */
        None | Some((                                                                                                                     /*
             |            PunctuationToken{tk: "|"}
                   ((↲    <DelimGroup>
                    (↲    <DelimGroup>                                                                                                    */
            Node::Stmt(Stmt {                                                                                                             /*
                ::                PunctuationToken{tk: "::"}
                      (Stmt•{↲    <DelimGroup>
                            {↲    <DelimGroup>                                                                                            */
                kind: StmtKind::Expr(_)                                                                                                   /*
                    :                      PunctuationToken{tk: ":"}
                              ::           PunctuationToken{tk: "::"}
                                    (_)    DelimGroup
                                     _     PunctuationToken{tk: "_"}                                                                      */
                    | StmtKind::Semi(_)                                                                                                   /*
                    |                      PunctuationToken{tk: "|"}
                              ::           PunctuationToken{tk: "::"}
                                    (_)    DelimGroup
                                     _     PunctuationToken{tk: "_"}                                                                      */
                    | StmtKind::Local(Local {                                                                                             /*
                    |                             PunctuationToken{tk: "|"}
                              ::                  PunctuationToken{tk: "::"}
                                     (Local•{↲    <DelimGroup>
                                            {↲    <DelimGroup>                                                                            */
                        pat: Pat {                                                                                                        /*
                           :           PunctuationToken{tk: ":"}
                                 {↲    <DelimGroup>                                                                                       */
                            kind: PatKind::Wild,                                                                                          /*
                                :                   PunctuationToken{tk: ":"}
                                         ::         PunctuationToken{tk: "::"}
                                               ,    PunctuationToken{tk: ","}                                                             */
                            ..                                                                                                            /*
                            ..    PunctuationToken{tk: ".."}                                                                              */
                        },                                                                                                                /*
••••••••••••••••••••••••}     </DelimGroup>
                         ,    PunctuationToken{tk: ","}                                                                                   */
                        ..                                                                                                                /*
                        ..    PunctuationToken{tk: ".."}                                                                                  */
                    }),                                                                                                                   /*
••••••••••••••••••••}      </DelimGroup>
••••••••••••••••••••})     </DelimGroup>
                      ,    PunctuationToken{tk: ","}                                                                                      */
                ..                                                                                                                        /*
                ..    PunctuationToken{tk: ".."}                                                                                          */
            }),                                                                                                                           /*
••••••••••••}      </DelimGroup>
••••••••••••})     </DelimGroup>
              ,    PunctuationToken{tk: ","}                                                                                              */
            _                                                                                                                             /*
            _    PunctuationToken{tk: "_"}                                                                                                */
        ))                                                                                                                                /*
••••••••)     </DelimGroup>
••••••••))    </DelimGroup>                                                                                                               */
    ) {}                                                                                                                                  /*
••••)       </MacroInvocation.segments>
••••)       </MacroInvocation>
••••)       </NotExpression>
      {}    IfBlockExpression.body{dk: "{}"}
••••)•{}    </IfBlockExpression>
••••)•{}    </ExpressionStatement>                                                                                                        */
    if_chain! {                                                                                                                           /*
    if_chain!•{↲    <ExpressionStatement{!semi}>
    if_chain!•{↲    <MacroInvocation>
              {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                  */
        if let ExprKind::If(cond, then, None) = expr.kind;                                                                                /*
                       ::                                     PunctuationToken{tk: "::"}
                           (cond,•then,•None)                 DelimGroup
                                ,                             PunctuationToken{tk: ","}
                                      ,                       PunctuationToken{tk: ","}
                                              =               PunctuationToken{tk: "="}
                                                    .         PunctuationToken{tk: "."}
                                                         ;    PunctuationToken{tk: ";"}                                                   */
        if !matches!(cond.kind, ExprKind::Let(_));                                                                                        /*
           !                                          PunctuationToken{tk: "!"}
                   !                                  PunctuationToken{tk: "!"}
                    (cond.kind,•ExprKind::Let(_))     DelimGroup
                         .                            PunctuationToken{tk: "."}
                              ,                       PunctuationToken{tk: ","}
                                        ::            PunctuationToken{tk: "::"}
                                             (_)      DelimGroup
                                              _       PunctuationToken{tk: "_"}
                                                 ;    PunctuationToken{tk: ";"}                                                           */
        if !expr.span.from_expansion();                                                                                                   /*
           !                               PunctuationToken{tk: "!"}
                .                          PunctuationToken{tk: "."}
                     .                     PunctuationToken{tk: "."}
                                    ()     DelimGroup
                                      ;    PunctuationToken{tk: ";"}                                                                      */
        let then = peel_blocks_with_stmt(then);                                                                                           /*
                 =                                 PunctuationToken{tk: "="}
                                        (then)     DelimGroup
                                              ;    PunctuationToken{tk: ";"}                                                              */
        if let Some(macro_call) = root_macro_call(then.span);                                                                             /*
                   (macro_call)                                  DelimGroup
                                =                                PunctuationToken{tk: "="}
                                                 (then.span)     DelimGroup
                                                      .          PunctuationToken{tk: "."}
                                                            ;    PunctuationToken{tk: ";"}                                                */
        if cx.tcx.item_name(macro_call.def_id) == sym::panic;                                                                             /*
             .                                                   PunctuationToken{tk: "."}
                 .                                               PunctuationToken{tk: "."}
                           (macro_call.def_id)                   DelimGroup
                                      .                          PunctuationToken{tk: "."}
                                               ==                PunctuationToken{tk: "=="}
                                                     ::          PunctuationToken{tk: "::"}
                                                            ;    PunctuationToken{tk: ";"}                                                */
        if !cx.tcx.sess.source_map().is_multiline(cond.span);                                                                             /*
           !                                                     PunctuationToken{tk: "!"}
              .                                                  PunctuationToken{tk: "."}
                  .                                              PunctuationToken{tk: "."}
                       .                                         PunctuationToken{tk: "."}
                                  ()                             DelimGroup
                                    .                            PunctuationToken{tk: "."}
                                                 (cond.span)     DelimGroup
                                                      .          PunctuationToken{tk: "."}
                                                            ;    PunctuationToken{tk: ";"}                                                */
        if let Some(format_args) = FormatArgsExpn::find_nested(cx, then, macro_call.expn);                                                /*
                   (format_args)                                                              DelimGroup
                                 =                                                            PunctuationToken{tk: "="}
                                                 ::                                           PunctuationToken{tk: "::"}
                                                              (cx,•then,•macro_call.expn)     DelimGroup
                                                                 ,                            PunctuationToken{tk: ","}
                                                                       ,                      PunctuationToken{tk: ","}
                                                                                   .          PunctuationToken{tk: "."}
                                                                                         ;    PunctuationToken{tk: ";"}                   */
        then {                                                                                                                            /*
             {↲    <DelimGroup>                                                                                                           */
            let mut applicability = Applicability::MachineApplicable;                                                                     /*
                                  =                                      PunctuationToken{tk: "="}
                                                 ::                      PunctuationToken{tk: "::"}
                                                                    ;    PunctuationToken{tk: ";"}                                        */
            let format_args_snip = snippet_with_applicability(cx, format_args.inputs_span(), "..", &mut applicability);                   /*
                                 =                                                                                         PunctuationToken{tk: "="}
                                                             (cx,•format_args.inputs_span(),•"..",•&mut•applicability)     DelimGroup
                                                                ,                                                          PunctuationToken{tk: ","}
                                                                             .                                             PunctuationToken{tk: "."}
                                                                                         ()                                DelimGroup
                                                                                           ,                               PunctuationToken{tk: ","}
                                                                                             ".."                          Literal{kind: String}
                                                                                                 ,                         PunctuationToken{tk: ","}
                                                                                                   &                       PunctuationToken{tk: "&"}
                                                                                                                      ;    PunctuationToken{tk: ";"}*/
            let cond = cond.peel_drop_temps();                                                                                            /*
                     =                            PunctuationToken{tk: "="}
                           .                      PunctuationToken{tk: "."}
                                           ()     DelimGroup
                                             ;    PunctuationToken{tk: ";"}                                                               */
            let (cond, not) = match cond.kind {                                                                                           /*
                (cond,•not)                         DelimGroup
                     ,                              PunctuationToken{tk: ","}
                            =                       PunctuationToken{tk: "="}
                                        .           PunctuationToken{tk: "."}
                                              {↲    <DelimGroup>                                                                          */
                ExprKind::Unary(UnOp::Not, e) => (e, ""),                                                                                 /*
                        ::                                   PunctuationToken{tk: "::"}
                               (UnOp::Not,•e)                DelimGroup
                                    ::                       PunctuationToken{tk: "::"}
                                         ,                   PunctuationToken{tk: ","}
                                              =>             PunctuationToken{tk: "=>"}
                                                 (e,•"")     DelimGroup
                                                   ,         PunctuationToken{tk: ","}
                                                     ""      Literal{kind: String}
                                                        ,    PunctuationToken{tk: ","}                                                    */
                _ => (cond, "!"),                                                                                                         /*
                _                    PunctuationToken{tk: "_"}
                  =>                 PunctuationToken{tk: "=>"}
                     (cond,•"!")     DelimGroup
                          ,          PunctuationToken{tk: ","}
                            "!"      Literal{kind: String}
                                ,    PunctuationToken{tk: ","}                                                                            */
            };                                                                                                                            /*
••••••••••••}     </DelimGroup>
             ;    PunctuationToken{tk: ";"}                                                                                               */
            let cond_sugg = sugg::Sugg::hir_with_applicability(cx, cond, "..", &mut applicability).maybe_par();                           /*
                          =                                                                                        PunctuationToken{tk: "="}
                                ::                                                                                 PunctuationToken{tk: "::"}
                                      ::                                                                           PunctuationToken{tk: "::"}
                                                              (cx,•cond,•"..",•&mut•applicability)                 DelimGroup
                                                                 ,                                                 PunctuationToken{tk: ","}
                                                                       ,                                           PunctuationToken{tk: ","}
                                                                         ".."                                      Literal{kind: String}
                                                                             ,                                     PunctuationToken{tk: ","}
                                                                               &                                   PunctuationToken{tk: "&"}
                                                                                                  .                PunctuationToken{tk: "."}
                                                                                                            ()     DelimGroup
                                                                                                              ;    PunctuationToken{tk: ";"}*/
            let sugg = format!("assert!({not}{cond_sugg}, {format_args_snip});");                                                         /*
                     =                                                               PunctuationToken{tk: "="}
                             !                                                       PunctuationToken{tk: "!"}
                              ("assert!({not}{cond_sugg},•{format_args_snip});")     DelimGroup
                               "assert!({not}{cond_sugg},•{format_args_snip});"      Literal{kind: String}
                                                                                ;    PunctuationToken{tk: ";"}                            */
            span_lint_and_sugg(                                                                                                           /*
                              (↲    <DelimGroup>                                                                                          */
                cx,                                                                                                                       /*
                  ,    PunctuationToken{tk: ","}                                                                                          */
                MANUAL_ASSERT,                                                                                                            /*
                             ,    PunctuationToken{tk: ","}                                                                               */
                expr.span,                                                                                                                /*
                    .         PunctuationToken{tk: "."}
                         ,    PunctuationToken{tk: ","}                                                                                   */
                "only a `panic!` in `if`-then statement",                                                                                 /*
                "only•a•`panic!`•in•`if`-then•statement"     Literal{kind: String}
                                                        ,    PunctuationToken{tk: ","}                                                    */
                "try",                                                                                                                    /*
                "try"     Literal{kind: String}
                     ,    PunctuationToken{tk: ","}                                                                                       */
                sugg,                                                                                                                     /*
                    ,    PunctuationToken{tk: ","}                                                                                        */
                Applicability::MachineApplicable,                                                                                         /*
                             ::                      PunctuationToken{tk: "::"}
                                                ,    PunctuationToken{tk: ","}                                                            */
            );                                                                                                                            /*
••••••••••••)     </DelimGroup>
             ;    PunctuationToken{tk: ";"}                                                                                               */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </MacroInvocation.segments>
••••}    </MacroInvocation>
••••}    </ExpressionStatement>                                                                                                           */
    if_chain! {                                                                                                                           /*
    if_chain!•{↲    <ExpressionStatement{!semi}>
    if_chain!•{↲    <MacroInvocation>
              {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                  */
        if meets_msrv(msrv.as_ref(), &msrvs::TOOL_ATTRIBUTES);                                                                            /*
                     (msrv.as_ref(),•&msrvs::TOOL_ATTRIBUTES)     DelimGroup
                          .                                       PunctuationToken{tk: "."}
                                 ()                               DelimGroup
                                   ,                              PunctuationToken{tk: ","}
                                     &                            PunctuationToken{tk: "&"}
                                           ::                     PunctuationToken{tk: "::"}
                                                             ;    PunctuationToken{tk: ";"}                                               */
        // check cfg_attr
        //•check•cfg_attr    Comment{line}
        if attr.has_name(sym::cfg_attr);                                                                                                  /*
               .                            PunctuationToken{tk: "."}
                        (sym::cfg_attr)     DelimGroup
                            ::              PunctuationToken{tk: "::"}
                                       ;    PunctuationToken{tk: ";"}                                                                     */
        if let Some(items) = attr.meta_item_list();                                                                                       /*
                   (items)                             DelimGroup
                           =                           PunctuationToken{tk: "="}
                                 .                     PunctuationToken{tk: "."}
                                                ()     DelimGroup
                                                  ;    PunctuationToken{tk: ";"}                                                          */
        if items.len() == 2;                                                                                                              /*
                .               PunctuationToken{tk: "."}
                    ()          DelimGroup
                       ==       PunctuationToken{tk: "=="}
                          2     Literal{kind: Integer}
                           ;    PunctuationToken{tk: ";"}                                                                                 */
        // check for `rustfmt`
        //•check•for•`rustfmt`    Comment{line}
        if let Some(feature_item) = items[0].meta_item();                                                                                 /*
                   (feature_item)                            DelimGroup
                                  =                          PunctuationToken{tk: "="}
                                         [0]                 DelimGroup
                                          0                  Literal{kind: Integer}
                                            .                PunctuationToken{tk: "."}
                                                      ()     DelimGroup
                                                        ;    PunctuationToken{tk: ";"}                                                    */
        if feature_item.has_name(sym::rustfmt);                                                                                           /*
                       .                           PunctuationToken{tk: "."}
                                (sym::rustfmt)     DelimGroup
                                    ::             PunctuationToken{tk: "::"}
                                              ;    PunctuationToken{tk: ";"}                                                              */
        // check for `rustfmt_skip` and `rustfmt::skip`
        //•check•for•`rustfmt_skip`•and•`rustfmt::skip`    Comment{line}
        if let Some(skip_item) = &items[1].meta_item();                                                                                   /*
                   (skip_item)                             DelimGroup
                               =                           PunctuationToken{tk: "="}
                                 &                         PunctuationToken{tk: "&"}
                                       [1]                 DelimGroup
                                        1                  Literal{kind: Integer}
                                          .                PunctuationToken{tk: "."}
                                                    ()     DelimGroup
                                                      ;    PunctuationToken{tk: ";"}                                                      */
        if skip_item.has_name(sym!(rustfmt_skip)) ||                                                                                      /*
                    .                                   PunctuationToken{tk: "."}
                             (sym!(rustfmt_skip))       DelimGroup
                                 !                      PunctuationToken{tk: "!"}
                                  (rustfmt_skip)        DelimGroup
                                                  ||    PunctuationToken{tk: "||"}                                                        */
            skip_item.path.segments.last().expect("empty path in attribute").ident.name == sym::skip;                                     /*
                     .                                                                                   PunctuationToken{tk: "."}
                          .                                                                              PunctuationToken{tk: "."}
                                   .                                                                     PunctuationToken{tk: "."}
                                        ()                                                               DelimGroup
                                          .                                                              PunctuationToken{tk: "."}
                                                 ("empty•path•in•attribute")                             DelimGroup
                                                  "empty•path•in•attribute"                              Literal{kind: String}
                                                                            .                            PunctuationToken{tk: "."}
                                                                                  .                      PunctuationToken{tk: "."}
                                                                                        ==               PunctuationToken{tk: "=="}
                                                                                              ::         PunctuationToken{tk: "::"}
                                                                                                    ;    PunctuationToken{tk: ";"}        */
        // Only lint outer attributes, because custom inner attributes are unstable
        //•Only•lint•outer•attributes,•because•custom•inner•attributes•are•unstable    Comment{line}
        // Tracking issue: https://github.com/rust-lang/rust/issues/54726
        //•Tracking•issue:•https://github.com/rust-lang/rust/issues/54726    Comment{line}
        if attr.style == AttrStyle::Outer;                                                                                                /*
               .                              PunctuationToken{tk: "."}
                      ==                      PunctuationToken{tk: "=="}
                                  ::          PunctuationToken{tk: "::"}
                                         ;    PunctuationToken{tk: ";"}                                                                   */
        then {                                                                                                                            /*
             {↲    <DelimGroup>                                                                                                           */
            span_lint_and_sugg(                                                                                                           /*
                              (↲    <DelimGroup>                                                                                          */
                cx,                                                                                                                       /*
                  ,    PunctuationToken{tk: ","}                                                                                          */
                DEPRECATED_CFG_ATTR,                                                                                                      /*
                                   ,    PunctuationToken{tk: ","}                                                                         */
                attr.span,                                                                                                                /*
                    .         PunctuationToken{tk: "."}
                         ,    PunctuationToken{tk: ","}                                                                                   */
                "`cfg_attr` is deprecated for rustfmt and got replaced by tool attributes",                                               /*
                "`cfg_attr`•is•deprecated•for•rustfmt•and•got•replaced•by•tool•attributes"     Literal{kind: String}
                                                                                          ,    PunctuationToken{tk: ","}                  */
                "use",                                                                                                                    /*
                "use"     Literal{kind: String}
                     ,    PunctuationToken{tk: ","}                                                                                       */
                "#[rustfmt::skip]".to_string(),                                                                                           /*
                "#[rustfmt::skip]"                 Literal{kind: String}
                                  .                PunctuationToken{tk: "."}
                                            ()     DelimGroup
                                              ,    PunctuationToken{tk: ","}                                                              */
                Applicability::MachineApplicable,                                                                                         /*
                             ::                      PunctuationToken{tk: "::"}
                                                ,    PunctuationToken{tk: ","}                                                            */
            );                                                                                                                            /*
••••••••••••)     </DelimGroup>
             ;    PunctuationToken{tk: ";"}                                                                                               */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </MacroInvocation.segments>
••••}    </MacroInvocation>
••••}    </ExpressionStatement>                                                                                                           */
    if_chain! {                                                                                                                           /*
    if_chain!•{↲    <ExpressionStatement{!semi}>
    if_chain!•{↲    <MacroInvocation>
              {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                  */
        if let ExprKind::Binary(ref op, left, right) = &expr.kind;                                                                        /*
                       ::                                             PunctuationToken{tk: "::"}
                               (ref•op,•left,•right)                  DelimGroup
                                      ,                               PunctuationToken{tk: ","}
                                            ,                         PunctuationToken{tk: ","}
                                                     =                PunctuationToken{tk: "="}
                                                       &              PunctuationToken{tk: "&"}
                                                            .         PunctuationToken{tk: "."}
                                                                 ;    PunctuationToken{tk: ";"}                                           */
        if let Some((candidate, check)) = normalize_le_ge(op, left, right);                                                               /*
                   ((candidate,•check))                                        DelimGroup
                    (candidate,•check)                                         DelimGroup
                              ,                                                PunctuationToken{tk: ","}
                                        =                                      PunctuationToken{tk: "="}
                                                         (op,•left,•right)     DelimGroup
                                                            ,                  PunctuationToken{tk: ","}
                                                                  ,            PunctuationToken{tk: ","}
                                                                          ;    PunctuationToken{tk: ";"}                                  */
        if let Some((from, to)) = get_types_from_cast(check, INTS, "max_value", "MAX");                                                   /*
                   ((from,•to))                                                            DelimGroup
                    (from,•to)                                                             DelimGroup
                         ,                                                                 PunctuationToken{tk: ","}
                                =                                                          PunctuationToken{tk: "="}
                                                     (check,•INTS,•"max_value",•"MAX")     DelimGroup
                                                           ,                               PunctuationToken{tk: ","}
                                                                 ,                         PunctuationToken{tk: ","}
                                                                   "max_value"             Literal{kind: String}
                                                                              ,            PunctuationToken{tk: ","}
                                                                                "MAX"      Literal{kind: String}
                                                                                      ;    PunctuationToken{tk: ";"}                      */

        then {                                                                                                                            /*
             {↲    <DelimGroup>                                                                                                           */
            Conversion::try_new(candidate, from, to)                                                                                      /*
                      ::                                PunctuationToken{tk: "::"}
                               (candidate,•from,•to)    DelimGroup
                                         ,              PunctuationToken{tk: ","}
                                               ,        PunctuationToken{tk: ","}                                                         */
        } else {                                                                                                                          /*
••••••••}            </DelimGroup>
               {↲    <DelimGroup>                                                                                                         */
            None
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </MacroInvocation.segments>
••••}    </MacroInvocation>
••••}    </ExpressionStatement>                                                                                                           */
    let help = format!(                                                                                                                   /*
    let•help•=•format!(↲    <LetVariableDeclaration>
               format!(↲    <MacroInvocation>
                      (↲    <MacroInvocation.segments{dk: "()"}>                                                                          */
        "because `{}` is the {} value for this type, {}",                                                                                 /*
        "because•`{}`•is•the•{}•value•for•this•type,•{}"     Literal{kind: String}
                                                        ,    PunctuationToken{tk: ","}                                                    */
        snippet(cx, culprit.expr.span, "x"),                                                                                              /*
               (cx,•culprit.expr.span,•"x")     DelimGroup
                  ,                             PunctuationToken{tk: ","}
                           .                    PunctuationToken{tk: "."}
                                .               PunctuationToken{tk: "."}
                                     ,          PunctuationToken{tk: ","}
                                       "x"      Literal{kind: String}
                                           ,    PunctuationToken{tk: ","}                                                                 */
        match culprit.which {                                                                                                             /*
                     .            PunctuationToken{tk: "."}
                            {↲    <DelimGroup>                                                                                            */
            ExtremeType::Minimum => "minimum",                                                                                            /*
                       ::                         PunctuationToken{tk: "::"}
                                 =>               PunctuationToken{tk: "=>"}
                                    "minimum"     Literal{kind: String}
                                             ,    PunctuationToken{tk: ","}                                                               */
            ExtremeType::Maximum => "maximum",                                                                                            /*
                       ::                         PunctuationToken{tk: "::"}
                                 =>               PunctuationToken{tk: "=>"}
                                    "maximum"     Literal{kind: String}
                                             ,    PunctuationToken{tk: ","}                                                               */
        },                                                                                                                                /*
••••••••}     </DelimGroup>
         ,    PunctuationToken{tk: ","}                                                                                                   */
        conclusion
    );                                                                                                                                    /*
••••)     </MacroInvocation.segments>
••••)     </MacroInvocation>
••••);    </LetVariableDeclaration>                                                                                                       */
   
    let msg = format!(                                                                                                                    /*
    let•msg•=•format!(↲    <LetVariableDeclaration>
              format!(↲    <MacroInvocation>
                     (↲    <MacroInvocation.segments{dk: "()"}>                                                                           */
        "this `{}` can be collapsed into the outer `{}`",                                                                                 /*
        "this•`{}`•can•be•collapsed•into•the•outer•`{}`"     Literal{kind: String}
                                                        ,    PunctuationToken{tk: ","}                                                    */
        if matches!(inner, IfLetOrMatch::Match(..)) { "match" } else { "if let" },                                                        /*
                  !                                                                   PunctuationToken{tk: "!"}
                   (inner,•IfLetOrMatch::Match(..))                                   DelimGroup
                         ,                                                            PunctuationToken{tk: ","}
                                       ::                                             PunctuationToken{tk: "::"}
                                              (..)                                    DelimGroup
                                               ..                                     PunctuationToken{tk: ".."}
                                                    {•"match"•}                       DelimGroup
                                                      "match"                         Literal{kind: String}
                                                                     {•"if•let"•}     DelimGroup
                                                                       "if•let"       Literal{kind: String}
                                                                                 ,    PunctuationToken{tk: ","}                           */
        if outer_is_match { "match" } else { "if let" },                                                                                  /*
                          {•"match"•}                       DelimGroup
                            "match"                         Literal{kind: String}
                                           {•"if•let"•}     DelimGroup
                                             "if•let"       Literal{kind: String}
                                                       ,    PunctuationToken{tk: ","}                                                     */
    );                                                                                                                                    /*
••••)     </MacroInvocation.segments>
••••)     </MacroInvocation>
••••);    </LetVariableDeclaration>                                                                                                       */
    let mut contents = format!(                                                                                                           /*
    let•mut•contents•=•format!(↲    <LetVariableDeclaration>
        mut•contents                PatternVariableDeclaration{!ref, mut}
                       format!(↲    <MacroInvocation>
                              (↲    <MacroInvocation.segments{dk: "()"}>                                                                  */
        indoc! {"                                                                                                                        "/*
             !        PunctuationToken{tk: "!"}
               {"↲    <DelimGroup>
                "↲    <Literal{kind: String}>                                                                                            */"
            #![warn(clippy::{})]

            fn main() {{
                // test code goes here
            }}
        "},                                                                                                                               /*
••••••••"      </Literal>
••••••••"}     </DelimGroup>
          ,    PunctuationToken{tk: ","}                                                                                                  */
        lint_name
    );                                                                                                                                    /*
••••)     </MacroInvocation.segments>
••••)     </MacroInvocation>
••••);    </LetVariableDeclaration>                                                                                                       */
    format!(                                                                                                                              /*
    format!(↲    <ExpressionStatement{semi}>
    format!(↲    <MacroInvocation>
           (↲    <MacroInvocation.segments{dk: "()"}>                                                                                     */
        "store.register_{lint_pass}_pass(move || Box::new({module_name}::{camel_name}::new(msrv)));\n    ",                               /*
        "store.register_{lint_pass}_pass(move•||•Box::new({module_name}::{camel_name}::new(msrv)));\n••••"     Literal{kind: String}
                                                                                                          ,    PunctuationToken{tk: ","}  */
        lint_pass = lint.pass,                                                                                                            /*
                  =               PunctuationToken{tk: "="}
                        .         PunctuationToken{tk: "."}
                             ,    PunctuationToken{tk: ","}                                                                               */
        module_name = lint.name,                                                                                                          /*
                    =               PunctuationToken{tk: "="}
                          .         PunctuationToken{tk: "."}
                               ,    PunctuationToken{tk: ","}                                                                             */
        camel_name = to_camel_case(lint.name),                                                                                            /*
                   =                              PunctuationToken{tk: "="}
                                  (lint.name)     DelimGroup
                                       .          PunctuationToken{tk: "."}
                                             ,    PunctuationToken{tk: ","}                                                               */
    );                                                                                                                                    /*
••••)     </MacroInvocation.segments>
••••)     </MacroInvocation>
••••);    </ExpressionStatement>                                                                                                          */
    format!(                                                                                                                              /*
    format!(↲    <ExpressionStatement{semi}>
    format!(↲    <MacroInvocation>
           (↲    <MacroInvocation.segments{dk: "()"}>                                                                                     */
        indoc! {r#"                                                                                                                     "#/*
             !          PunctuationToken{tk: "!"}
               {r#"↲    <DelimGroup>
                r#"↲    <Literal{kind: rString}>                                                                                       */r#"
            # {}

            [package]
            name = "{}"
            version = "0.1.0"
            publish = false

            [workspace]
        "#},                                                                                                                              /*
••••••••"#      </Literal>
••••••••"#}     </DelimGroup>
           ,    PunctuationToken{tk: ","}                                                                                                 */
        hint, lint_name                                                                                                                   /*
            ,    PunctuationToken{tk: ","}                                                                                                */
    );                                                                                                                                    /*
••••)     </MacroInvocation.segments>
••••)     </MacroInvocation>
••••);    </ExpressionStatement>                                                                                                          */
    match iter.next() {                                                                                                                   /*
    match•iter.next()•{↲    <ExpressionStatement{!semi}>
    match•iter.next()•{↲    <MatchExpression>
          iter.next()       CallExpression
                   ()       CallExpression.arguments{dk: "()"}
                      {↲    <MatchExpression.cases{dk: "{}"}>                                                                             */
        // #[clippy::version = "version"] pub
        //•#[clippy::version•=•"version"]•pub    Comment{line}
        Some((TokenKind::Pound, _)) => {                                                                                                  /*
        Some((TokenKind::Pound,•_))•=>•{↲    <MatchExpressionCase>
        Some((TokenKind::Pound,•_))          TuplePattern
            ((TokenKind::Pound,•_))          TuplePattern.items{dk: "()"}
             (TokenKind::Pound,•_)           TuplePattern
              TokenKind::Pound               ExpressionPath
                                _            WildcardPattern
                                       {↲    <BlockExpression>                                                                            */
            match_tokens!(iter, OpenBracket Ident Colon Colon Ident Eq Literal{..} CloseBracket Ident);                                   /*
            match_tokens!(iter,•OpenBracket•Ident•Colon•Colon•Ident•Eq•Literal{..}•CloseBracket•Ident);    ExpressionStatement{semi}
            match_tokens!(iter,•OpenBracket•Ident•Colon•Colon•Ident•Eq•Literal{..}•CloseBracket•Ident)     MacroInvocation
                         (iter,•OpenBracket•Ident•Colon•Colon•Ident•Eq•Literal{..}•CloseBracket•Ident)     MacroInvocation.segments{dk: "()"}
                              ,                                                                            PunctuationToken{tk: ","}
                                                                              {..}                         DelimGroup
                                                                               ..                          PunctuationToken{tk: ".."}     */
        },                                                                                                                                /*
••••••••}    </BlockExpression>
••••••••}    </MatchExpressionCase>                                                                                                       */
        // pub
        //•pub    Comment{line}
        Some((TokenKind::Ident, _)) => (),                                                                                                /*
        Some((TokenKind::Ident,•_))•=>•()    MatchExpressionCase
        Some((TokenKind::Ident,•_))          TuplePattern
            ((TokenKind::Ident,•_))          TuplePattern.items{dk: "()"}
             (TokenKind::Ident,•_)           TuplePattern
              TokenKind::Ident               ExpressionPath
                                _            WildcardPattern
                                       ()    TupleLiteral                                                                                 */
        _ => continue,                                                                                                                    /*
        _•=>•continue    MatchExpressionCase
        _                WildcardPattern
             continue    ContinueExpression                                                                                               */
    }                                                                                                                                     /*
••••}    </MatchExpression.cases>
••••}    </MatchExpression>
••••}    </ExpressionStatement>                                                                                                           */
    let (name, group, desc) = match_tokens!(                                                                                              /*
    let•(name,•group,•desc)•=•match_tokens!(↲    <LetVariableDeclaration>
        (name,•group,•desc)                      TuplePattern
                              match_tokens!(↲    <MacroInvocation>
                                           (↲    <MacroInvocation.segments{dk: "()"}>                                                     */
        iter,                                                                                                                             /*
            ,    PunctuationToken{tk: ","}                                                                                                */
        // LINT_NAME
        //•LINT_NAME    Comment{line}
        Ident(name) Comma                                                                                                                 /*
             (name)    DelimGroup                                                                                                         */
        // group,
        //•group,    Comment{line}
        Ident(group) Comma                                                                                                                /*
             (group)    DelimGroup                                                                                                        */
        // "description" }
        //•"description"•}    Comment{line}
        Literal{..}(desc) CloseBrace                                                                                                      /*
               {..}          DelimGroup
                ..           PunctuationToken{tk: ".."}
                   (desc)    DelimGroup                                                                                                   */
        // #[clippy::version = "version"]
        //•#[clippy::version•=•"version"]    Comment{line}
        Pound OpenBracket Ident Colon Colon Ident Eq Literal{..} CloseBracket                                                             /*
                                                            {..}    DelimGroup
                                                             ..     PunctuationToken{tk: ".."}                                            */
        // pub LINT_NAME,
        //•pub•LINT_NAME,    Comment{line}
        Ident Ident(name) Comma                                                                                                           /*
                   (name)    DelimGroup                                                                                                   */
        // "description"
        //•"description"    Comment{line}
        Literal{kind: LiteralKind::Str{..},..}(reason)                                                                                    /*
               {kind:•LiteralKind::Str{..},..}            DelimGroup
                    :                                     PunctuationToken{tk: ":"}
                                 ::                       PunctuationToken{tk: "::"}
                                      {..}                DelimGroup
                                       ..                 PunctuationToken{tk: ".."}
                                          ,               PunctuationToken{tk: ","}
                                           ..             PunctuationToken{tk: ".."}
                                              (reason)    DelimGroup                                                                      */
        // ("old_name",
        //•("old_name",    Comment{line}
        Whitespace OpenParen Literal{kind: LiteralKind::Str{..},..}(old_name) Comma                                                       /*
                                    {kind:•LiteralKind::Str{..},..}              DelimGroup
                                         :                                       PunctuationToken{tk: ":"}
                                                      ::                         PunctuationToken{tk: "::"}
                                                           {..}                  DelimGroup
                                                            ..                   PunctuationToken{tk: ".."}
                                                               ,                 PunctuationToken{tk: ","}
                                                                ..               PunctuationToken{tk: ".."}
                                                                   (old_name)    DelimGroup                                               */
        // "new_name"),
        //•"new_name"),    Comment{line}
        Whitespace Literal{kind: LiteralKind::Str{..},..}(new_name) CloseParen Comma                                                      /*
                          {kind:•LiteralKind::Str{..},..}              DelimGroup
                               :                                       PunctuationToken{tk: ":"}
                                            ::                         PunctuationToken{tk: "::"}
                                                 {..}                  DelimGroup
                                                  ..                   PunctuationToken{tk: ".."}
                                                     ,                 PunctuationToken{tk: ","}
                                                      ..               PunctuationToken{tk: ".."}
                                                         (new_name)    DelimGroup                                                         */
    );                                                                                                                                    /*
••••)     </MacroInvocation.segments>
••••)     </MacroInvocation>
••••);    </LetVariableDeclaration>                                                                                                       */
    
    info!(//debug
/*  info!(//debug↲    <ExpressionStatement{semi}>
    info!(//debug↲    <MacroInvocation>
         (//debug↲    <MacroInvocation.segments{dk: "()"}>                                                                                */
          //debug     Comment{line}
        "{}: sending function_code={:04x} data={:04x} crc=0x{:04X} data={:02X?}",                                                         /*
        "{}:•sending•function_code={:04x}•data={:04x}•crc=0x{:04X}•data={:02X?}"     Literal{kind: String}
                                                                                ,    PunctuationToken{tk: ","}                            */
        self.name, function_code, data, crc, output_cmd                                                                                   /*
            .                                   PunctuationToken{tk: "."}
                 ,                              PunctuationToken{tk: ","}
                                ,               PunctuationToken{tk: ","}
                                      ,         PunctuationToken{tk: ","}
                                           ,    PunctuationToken{tk: ","}                                                                 */
    );                                                                                                                                    /*
••••)     </MacroInvocation.segments>
••••)     </MacroInvocation>
••••);    </ExpressionStatement>                                                                                                          */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */

cfg_if! {                                                                                                                                 /*
cfg_if!•{↲    <ExpressionStatement{!semi}>
cfg_if!•{↲    <MacroInvocation>
        {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                        */
    if #[cfg(feature = "std_detect_file_io")] {                                                                                           /*
       #                                            PunctuationToken{tk: "#"}
        [cfg(feature•=•"std_detect_file_io")]       DelimGroup
            (feature•=•"std_detect_file_io")        DelimGroup
                     =                              PunctuationToken{tk: "="}
                       "std_detect_file_io"         Literal{kind: String}
                                              {↲    <DelimGroup>                                                                          */
        #[cfg_attr(test, macro_use(println))]                                                                                             /*
        #                                        PunctuationToken{tk: "#"}
         [cfg_attr(test,•macro_use(println))]    DelimGroup
                  (test,•macro_use(println))     DelimGroup
                       ,                         PunctuationToken{tk: ","}
                                  (println)      DelimGroup                                                                               */
        extern crate std;                                                                                                                 /*
                        ;    PunctuationToken{tk: ";"}                                                                                    */

        #[allow(unused_imports)]                                                                                                          /*
        #                           PunctuationToken{tk: "#"}
         [allow(unused_imports)]    DelimGroup
               (unused_imports)     DelimGroup                                                                                            */
        use std::{arch, fs, io, mem, sync};                                                                                               /*
               ::                              PunctuationToken{tk: "::"}
                 {arch,•fs,•io,•mem,•sync}     DelimGroup
                      ,                        PunctuationToken{tk: ","}
                          ,                    PunctuationToken{tk: ","}
                              ,                PunctuationToken{tk: ","}
                                   ,           PunctuationToken{tk: ","}
                                          ;    PunctuationToken{tk: ";"}                                                                  */
    } else {                                                                                                                              /*
••••}            </DelimGroup>
           {↲    <DelimGroup>                                                                                                             */
        #[cfg(test)]                                                                                                                      /*
        #               PunctuationToken{tk: "#"}
         [cfg(test)]    DelimGroup
             (test)     DelimGroup                                                                                                        */
        #[macro_use(println)]                                                                                                             /*
        #                        PunctuationToken{tk: "#"}
         [macro_use(println)]    DelimGroup
                   (println)     DelimGroup                                                                                               */
        extern crate std;                                                                                                                 /*
                        ;    PunctuationToken{tk: ";"}                                                                                    */

        #[allow(unused_imports)]                                                                                                          /*
        #                           PunctuationToken{tk: "#"}
         [allow(unused_imports)]    DelimGroup
               (unused_imports)     DelimGroup                                                                                            */
        use core::{arch, mem, sync};                                                                                                      /*
                ::                      PunctuationToken{tk: "::"}
                  {arch,•mem,•sync}     DelimGroup
                       ,                PunctuationToken{tk: ","}
                            ,           PunctuationToken{tk: ","}
                                   ;    PunctuationToken{tk: ";"}                                                                         */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

op_utils! {                                                                                                                               /*
op_utils!•{↲    <ExpressionStatement{!semi}>
op_utils!•{↲    <MacroInvocation>
          {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                      */
    Add    AddAssign
    Sub    SubAssign
    Mul    MulAssign
    Div    DivAssign
    Rem    RemAssign
    BitXor BitXorAssign
    BitAnd BitAndAssign
    BitOr  BitOrAssign
    Shl    ShlAssign
    Shr    ShrAssign
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

msrv_aliases! {                                                                                                                           /*
msrv_aliases!•{↲    <ExpressionStatement{!semi}>
msrv_aliases!•{↲    <MacroInvocation>
              {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                  */
    1,53,0 { OR_PATTERNS, MANUAL_BITS }                                                                                                   /*
    1                                      Literal{kind: Integer}
     ,                                     PunctuationToken{tk: ","}
      53                                   Literal{kind: Integer}
        ,                                  PunctuationToken{tk: ","}
         0                                 Literal{kind: Integer}
           {•OR_PATTERNS,•MANUAL_BITS•}    DelimGroup
                        ,                  PunctuationToken{tk: ","}                                                                      */
    1,52,0 { STR_SPLIT_ONCE }                                                                                                             /*
    1                            Literal{kind: Integer}
     ,                           PunctuationToken{tk: ","}
      52                         Literal{kind: Integer}
        ,                        PunctuationToken{tk: ","}
         0                       Literal{kind: Integer}
           {•STR_SPLIT_ONCE•}    DelimGroup                                                                                               */
    1,51,0 { BORROW_AS_PTR, UNSIGNED_ABS }                                                                                                /*
    1                                         Literal{kind: Integer}
     ,                                        PunctuationToken{tk: ","}
      51                                      Literal{kind: Integer}
        ,                                     PunctuationToken{tk: ","}
         0                                    Literal{kind: Integer}
           {•BORROW_AS_PTR,•UNSIGNED_ABS•}    DelimGroup
                          ,                   PunctuationToken{tk: ","}                                                                   */
    1,50,0 { BOOL_THEN }                                                                                                                  /*
    1                       Literal{kind: Integer}
     ,                      PunctuationToken{tk: ","}
      50                    Literal{kind: Integer}
        ,                   PunctuationToken{tk: ","}
         0                  Literal{kind: Integer}
           {•BOOL_THEN•}    DelimGroup                                                                                                    */
    1,47,0 { TAU }                                                                                                                        /*
    1                 Literal{kind: Integer}
     ,                PunctuationToken{tk: ","}
      47              Literal{kind: Integer}
        ,             PunctuationToken{tk: ","}
         0            Literal{kind: Integer}
           {•TAU•}    DelimGroup                                                                                                          */
    1,46,0 { CONST_IF_MATCH }                                                                                                             /*
    1                            Literal{kind: Integer}
     ,                           PunctuationToken{tk: ","}
      46                         Literal{kind: Integer}
        ,                        PunctuationToken{tk: ","}
         0                       Literal{kind: Integer}
           {•CONST_IF_MATCH•}    DelimGroup                                                                                               */
    1,45,0 { STR_STRIP_PREFIX }                                                                                                           /*
    1                              Literal{kind: Integer}
     ,                             PunctuationToken{tk: ","}
      45                           Literal{kind: Integer}
        ,                          PunctuationToken{tk: ","}
         0                         Literal{kind: Integer}
           {•STR_STRIP_PREFIX•}    DelimGroup                                                                                             */
    1,43,0 { LOG2_10, LOG10_2 }                                                                                                           /*
    1                              Literal{kind: Integer}
     ,                             PunctuationToken{tk: ","}
      43                           Literal{kind: Integer}
        ,                          PunctuationToken{tk: ","}
         0                         Literal{kind: Integer}
           {•LOG2_10,•LOG10_2•}    DelimGroup
                    ,              PunctuationToken{tk: ","}                                                                              */
    1,42,0 { MATCHES_MACRO, SLICE_PATTERNS, PTR_SLICE_RAW_PARTS }                                                                         /*
    1                                                                Literal{kind: Integer}
     ,                                                               PunctuationToken{tk: ","}
      42                                                             Literal{kind: Integer}
        ,                                                            PunctuationToken{tk: ","}
         0                                                           Literal{kind: Integer}
           {•MATCHES_MACRO,•SLICE_PATTERNS,•PTR_SLICE_RAW_PARTS•}    DelimGroup
                          ,                                          PunctuationToken{tk: ","}
                                          ,                          PunctuationToken{tk: ","}                                            */
    1,41,0 { RE_REBALANCING_COHERENCE, RESULT_MAP_OR_ELSE }                                                                               /*
    1                                                          Literal{kind: Integer}
     ,                                                         PunctuationToken{tk: ","}
      41                                                       Literal{kind: Integer}
        ,                                                      PunctuationToken{tk: ","}
         0                                                     Literal{kind: Integer}
           {•RE_REBALANCING_COHERENCE,•RESULT_MAP_OR_ELSE•}    DelimGroup
                                     ,                         PunctuationToken{tk: ","}                                                  */
    1,40,0 { MEM_TAKE, NON_EXHAUSTIVE, OPTION_AS_DEREF }                                                                                  /*
    1                                                       Literal{kind: Integer}
     ,                                                      PunctuationToken{tk: ","}
      40                                                    Literal{kind: Integer}
        ,                                                   PunctuationToken{tk: ","}
         0                                                  Literal{kind: Integer}
           {•MEM_TAKE,•NON_EXHAUSTIVE,•OPTION_AS_DEREF•}    DelimGroup
                     ,                                      PunctuationToken{tk: ","}
                                     ,                      PunctuationToken{tk: ","}                                                     */
    1,38,0 { POINTER_CAST }                                                                                                               /*
    1                          Literal{kind: Integer}
     ,                         PunctuationToken{tk: ","}
      38                       Literal{kind: Integer}
        ,                      PunctuationToken{tk: ","}
         0                     Literal{kind: Integer}
           {•POINTER_CAST•}    DelimGroup                                                                                                 */
    1,37,0 { TYPE_ALIAS_ENUM_VARIANTS }                                                                                                   /*
    1                                      Literal{kind: Integer}
     ,                                     PunctuationToken{tk: ","}
      37                                   Literal{kind: Integer}
        ,                                  PunctuationToken{tk: ","}
         0                                 Literal{kind: Integer}
           {•TYPE_ALIAS_ENUM_VARIANTS•}    DelimGroup                                                                                     */
    1,36,0 { ITERATOR_COPIED }                                                                                                            /*
    1                             Literal{kind: Integer}
     ,                            PunctuationToken{tk: ","}
      36                          Literal{kind: Integer}
        ,                         PunctuationToken{tk: ","}
         0                        Literal{kind: Integer}
           {•ITERATOR_COPIED•}    DelimGroup                                                                                              */
    1,35,0 { OPTION_COPIED, RANGE_CONTAINS }                                                                                              /*
    1                                           Literal{kind: Integer}
     ,                                          PunctuationToken{tk: ","}
      35                                        Literal{kind: Integer}
        ,                                       PunctuationToken{tk: ","}
         0                                      Literal{kind: Integer}
           {•OPTION_COPIED,•RANGE_CONTAINS•}    DelimGroup
                          ,                     PunctuationToken{tk: ","}                                                                 */
    1,34,0 { TRY_FROM }                                                                                                                   /*
    1                      Literal{kind: Integer}
     ,                     PunctuationToken{tk: ","}
      34                   Literal{kind: Integer}
        ,                  PunctuationToken{tk: ","}
         0                 Literal{kind: Integer}
           {•TRY_FROM•}    DelimGroup                                                                                                     */
    1,30,0 { ITERATOR_FIND_MAP, TOOL_ATTRIBUTES }                                                                                         /*
    1                                                Literal{kind: Integer}
     ,                                               PunctuationToken{tk: ","}
      30                                             Literal{kind: Integer}
        ,                                            PunctuationToken{tk: ","}
         0                                           Literal{kind: Integer}
           {•ITERATOR_FIND_MAP,•TOOL_ATTRIBUTES•}    DelimGroup
                              ,                      PunctuationToken{tk: ","}                                                            */
    1,28,0 { FROM_BOOL }                                                                                                                  /*
    1                       Literal{kind: Integer}
     ,                      PunctuationToken{tk: ","}
      28                    Literal{kind: Integer}
        ,                   PunctuationToken{tk: ","}
         0                  Literal{kind: Integer}
           {•FROM_BOOL•}    DelimGroup                                                                                                    */
    1,17,0 { FIELD_INIT_SHORTHAND, STATIC_IN_CONST, EXPECT_ERR }                                                                          /*
    1                                                               Literal{kind: Integer}
     ,                                                              PunctuationToken{tk: ","}
      17                                                            Literal{kind: Integer}
        ,                                                           PunctuationToken{tk: ","}
         0                                                          Literal{kind: Integer}
           {•FIELD_INIT_SHORTHAND,•STATIC_IN_CONST,•EXPECT_ERR•}    DelimGroup
                                 ,                                  PunctuationToken{tk: ","}
                                                  ,                 PunctuationToken{tk: ","}                                             */
    1,16,0 { STR_REPEAT }                                                                                                                 /*
    1                        Literal{kind: Integer}
     ,                       PunctuationToken{tk: ","}
      16                     Literal{kind: Integer}
        ,                    PunctuationToken{tk: ","}
         0                   Literal{kind: Integer}
           {•STR_REPEAT•}    DelimGroup                                                                                                   */
    1,24,0 { IS_ASCII_DIGIT }                                                                                                             /*
    1                            Literal{kind: Integer}
     ,                           PunctuationToken{tk: ","}
      24                         Literal{kind: Integer}
        ,                        PunctuationToken{tk: ","}
         0                       Literal{kind: Integer}
           {•IS_ASCII_DIGIT•}    DelimGroup                                                                                               */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

kot! {                                                                                                                                    /*
kot!•{↲    <ExpressionStatement{!semi}>
kot!•{↲    <MacroInvocation>
     {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                           */
    struct W { foo : u8, bar : u16, } impl Clone for W                                                                                    /*
             {•foo•:•u8,•bar•:•u16,•}    DelimGroup
                   :                     PunctuationToken{tk: ":"}
                       ,                 PunctuationToken{tk: ","}
                             :           PunctuationToken{tk: ":"}
                                  ,      PunctuationToken{tk: ","}                                                                        */
    {                                                                                                                                     /*
    {↲    <DelimGroup>                                                                                                                    */
        fn clone() -> W                                                                                                                   /*
                ()       DelimGroup
                   ->    PunctuationToken{tk: "->"}                                                                                       */
        {                                                                                                                                 /*
        {↲    <DelimGroup>                                                                                                                */
            panic! () ;                                                                                                                   /*
                 !         PunctuationToken{tk: "!"}
                   ()      DelimGroup
                      ;    PunctuationToken{tk: ";"}                                                                                      */

        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

kot! {                                                                                                                                    /*
kot!•{↲    <ExpressionStatement{!semi}>
kot!•{↲    <MacroInvocation>
     {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                           */
    a(mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                             /*
     (mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <DelimGroup>                                            */
    mushkins mushkins) a                                                                                                                  /*
••••mushkins•mushkins)    </DelimGroup>                                                                                                   */
    [mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
    [mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <DelimGroup>                                             */
    mushkins mushkins] a                                                                                                                  /*
••••mushkins•mushkins]    </DelimGroup>                                                                                                   */
    {                                                                                                                                     /*
    {↲    <DelimGroup>                                                                                                                    */
        mushkins mushkins mushkins mushkins mushkins mushkins mushkins
        mushkins mushkins mushkins
    } a                                                                                                                                   /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

kot!(mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
kot!(mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <ExpressionStatement{semi}>
kot!(mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <MacroInvocation>
    (mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <MacroInvocation.segments{dk: "()"}>                     */
mushkins mushkins);                                                                                                                       /*
mushkins•mushkins)     </MacroInvocation.segments>
mushkins•mushkins)     </MacroInvocation>
mushkins•mushkins);    </ExpressionStatement>                                                                                             */
kot![mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
kot![mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <ExpressionStatement{semi}>
kot![mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <MacroInvocation>
    [mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <MacroInvocation.segments{dk: "[]"}>                     */
mushkins mushkins];                                                                                                                       /*
mushkins•mushkins]     </MacroInvocation.segments>
mushkins•mushkins]     </MacroInvocation>
mushkins•mushkins];    </ExpressionStatement>                                                                                             */
kot! {                                                                                                                                    /*
kot!•{↲    <ExpressionStatement{!semi}>
kot!•{↲    <MacroInvocation>
     {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                           */
    mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins
    mushkins mushkins
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

#[rustc_dummy(mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
#[rustc_dummy(mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <ExpressionStatement{!semi}>
#[rustc_dummy(mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <Attribute{!inner}>
 [rustc_dummy(mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <Attribute.segments{dk: "[]"}>
             (mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <DelimGroup>                                             */
mushkins mushkins mushkins)]                                                                                                              /*
mushkins•mushkins•mushkins)     </DelimGroup>
mushkins•mushkins•mushkins)]    </Attribute.segments>
mushkins•mushkins•mushkins)]    </Attribute>                                                                                              */
#[rustc_dummy[mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
#[rustc_dummy[mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <Attribute{!inner}>
 [rustc_dummy[mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <Attribute.segments{dk: "[]"}>
             [mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <DelimGroup>                                             */
mushkins mushkins mushkins]]                                                                                                              /*
mushkins•mushkins•mushkins]     </DelimGroup>
mushkins•mushkins•mushkins]]    </Attribute.segments>
mushkins•mushkins•mushkins]]    </Attribute>                                                                                              */
#[rustc_dummy {                                                                                                                           /*
#[rustc_dummy•{↲    <Attribute{!inner}>
 [rustc_dummy•{↲    <Attribute.segments{dk: "[]"}>
              {↲    <DelimGroup>                                                                                                          */
    mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins
    mushkins mushkins
}]                                                                                                                                        /*
}     </DelimGroup>
}]    </Attribute.segments>
}]    </Attribute>                                                                                                                        */
#[rustc_dummy =                                                                                                                           /*
#[rustc_dummy•=↲    <Attribute{!inner}>
 [rustc_dummy•=↲    <Attribute.segments{dk: "[]"}>
              =     PunctuationToken{tk: "="}                                                                                             */
"mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins"]                                              /*
"mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins"     Literal{kind: String}
"mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins"]    </Attribute.segments>
"mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins"]    </Attribute>                              */

match t.kind() {                                                                                                                          /*
match•t.kind()•{↲    ExpressionStatement~ownStart <MatchExpression>
      t.kind()       CallExpression
            ()       CallExpression.arguments{dk: "()"}
               {↲    <MatchExpression.cases{dk: "{}"}>                                                                                    */
    ty::Int(i) => find_fit!(i, val, negative,                                                                                             /*
    ty::Int(i)•=>•find_fit!(i,•val,•negative,↲    <MatchExpressionCase>
    ty::Int(i)                                    TuplePattern
    ty::Int                                       ExpressionPath
           (i)                                    TuplePattern.items{dk: "()"}
                  find_fit!(i,•val,•negative,↲    <MacroInvocation>
                           (i,•val,•negative,↲    <MacroInvocation.segments{dk: "()"}>
                             ,                    PunctuationToken{tk: ","}
                                  ,               PunctuationToken{tk: ","}
                                            ,     PunctuationToken{tk: ","}                                                               */
                  I8 => [U8] => [I16, I32, I64, I128],                                                                                    /*
                     =>                                   PunctuationToken{tk: "=>"}
                        [U8]                              DelimGroup
                             =>                           PunctuationToken{tk: "=>"}
                                [I16,•I32,•I64,•I128]     DelimGroup
                                    ,                     PunctuationToken{tk: ","}
                                         ,                PunctuationToken{tk: ","}
                                              ,           PunctuationToken{tk: ","}
                                                     ,    PunctuationToken{tk: ","}                                                       */
                  I16 => [U16] => [I32, I64, I128],                                                                                       /*
                      =>                               PunctuationToken{tk: "=>"}
                         [U16]                         DelimGroup
                               =>                      PunctuationToken{tk: "=>"}
                                  [I32,•I64,•I128]     DelimGroup
                                      ,                PunctuationToken{tk: ","}
                                           ,           PunctuationToken{tk: ","}
                                                  ,    PunctuationToken{tk: ","}                                                          */
                  I32 => [U32] => [I64, I128],                                                                                            /*
                      =>                          PunctuationToken{tk: "=>"}
                         [U32]                    DelimGroup
                               =>                 PunctuationToken{tk: "=>"}
                                  [I64,•I128]     DelimGroup
                                      ,           PunctuationToken{tk: ","}
                                             ,    PunctuationToken{tk: ","}                                                               */
                  I64 => [U64] => [I128],                                                                                                 /*
                      =>                     PunctuationToken{tk: "=>"}
                         [U64]               DelimGroup
                               =>            PunctuationToken{tk: "=>"}
                                  [I128]     DelimGroup
                                        ,    PunctuationToken{tk: ","}                                                                    */
                  I128 => [U128] => []),                                                                                                  /*
                       =>                  PunctuationToken{tk: "=>"}
                          [U128]           DelimGroup
                                 =>        PunctuationToken{tk: "=>"}
                                    []     DelimGroup
••••••••••••••••••I128•=>•[U128]•=>•[])    </MacroInvocation.segments>
••••••••••••••••••I128•=>•[U128]•=>•[])    </MacroInvocation>
••••••••••••••••••I128•=>•[U128]•=>•[])    </MatchExpressionCase>                                                                         */
    ty::Uint(u) => find_fit!(u, val, negative,                                                                                            /*
    ty::Uint(u)•=>•find_fit!(u,•val,•negative,↲    <MatchExpressionCase>
    ty::Uint(u)                                    TuplePattern
    ty::Uint                                       ExpressionPath
            (u)                                    TuplePattern.items{dk: "()"}
                   find_fit!(u,•val,•negative,↲    <MacroInvocation>
                            (u,•val,•negative,↲    <MacroInvocation.segments{dk: "()"}>
                              ,                    PunctuationToken{tk: ","}
                                   ,               PunctuationToken{tk: ","}
                                             ,     PunctuationToken{tk: ","}                                                              */
                  U8 => [U8, U16, U32, U64, U128] => [],                                                                                  /*
                     =>                                     PunctuationToken{tk: "=>"}
                        [U8,•U16,•U32,•U64,•U128]           DelimGroup
                           ,                                PunctuationToken{tk: ","}
                                ,                           PunctuationToken{tk: ","}
                                     ,                      PunctuationToken{tk: ","}
                                          ,                 PunctuationToken{tk: ","}
                                                  =>        PunctuationToken{tk: "=>"}
                                                     []     DelimGroup
                                                       ,    PunctuationToken{tk: ","}                                                     */
                  U16 => [U16, U32, U64, U128] => [],                                                                                     /*
                      =>                                 PunctuationToken{tk: "=>"}
                         [U16,•U32,•U64,•U128]           DelimGroup
                             ,                           PunctuationToken{tk: ","}
                                  ,                      PunctuationToken{tk: ","}
                                       ,                 PunctuationToken{tk: ","}
                                               =>        PunctuationToken{tk: "=>"}
                                                  []     DelimGroup
                                                    ,    PunctuationToken{tk: ","}                                                        */
                  U32 => [U32, U64, U128] => [],                                                                                          /*
                      =>                            PunctuationToken{tk: "=>"}
                         [U32,•U64,•U128]           DelimGroup
                             ,                      PunctuationToken{tk: ","}
                                  ,                 PunctuationToken{tk: ","}
                                          =>        PunctuationToken{tk: "=>"}
                                             []     DelimGroup
                                               ,    PunctuationToken{tk: ","}                                                             */
                  U64 => [U64, U128] => [],                                                                                               /*
                      =>                       PunctuationToken{tk: "=>"}
                         [U64,•U128]           DelimGroup
                             ,                 PunctuationToken{tk: ","}
                                     =>        PunctuationToken{tk: "=>"}
                                        []     DelimGroup
                                          ,    PunctuationToken{tk: ","}                                                                  */
                  U128 => [U128] => []),                                                                                                  /*
                       =>                  PunctuationToken{tk: "=>"}
                          [U128]           DelimGroup
                                 =>        PunctuationToken{tk: "=>"}
                                    []     DelimGroup
••••••••••••••••••U128•=>•[U128]•=>•[])    </MacroInvocation.segments>
••••••••••••••••••U128•=>•[U128]•=>•[])    </MacroInvocation>
••••••••••••••••••U128•=>•[U128]•=>•[])    </MatchExpressionCase>                                                                         */
    _ => None,                                                                                                                            /*
    _•=>•None    MatchExpressionCase
    _            WildcardPattern                                                                                                          */
}                                                                                                                                         /*
}    </MatchExpression.cases>
}    </MatchExpression>
}    </ExpressionStatement>                                                                                                               */
x! {                                                                                                                                      /*
x!•{↲    <ExpressionStatement{!semi}>
x!•{↲    <MacroInvocation>
   {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                             */
    /// [d]: ../b.md#a
    ///•[d]:•../b.md#a    Comment{line}
    pub G,                                                                                                                                /*
         ,    PunctuationToken{tk: ","}                                                                                                   */
    d,                                                                                                                                    /*
     ,    PunctuationToken{tk: ","}                                                                                                       */
    "c",                                                                                                                                  /*
    "c"     Literal{kind: String}
       ,    PunctuationToken{tk: ","}                                                                                                     */
    @d = x {                                                                                                                              /*
    @            PunctuationToken{tk: "@"}
       =         PunctuationToken{tk: "="}
           {↲    <DelimGroup>                                                                                                             */
        b: "a",                                                                                                                           /*
         :         PunctuationToken{tk: ":"}
           "a"     Literal{kind: String}
              ,    PunctuationToken{tk: ","}                                                                                              */
    };                                                                                                                                    /*
••••}     </DelimGroup>
     ;    PunctuationToken{tk: ";"}                                                                                                       */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */
throw_span_err!(                                                                                                                          /*
throw_span_err!(↲    <ExpressionStatement{semi}>
throw_span_err!(↲    <MacroInvocation>
               (↲    <MacroInvocation.segments{dk: "()"}>                                                                                 */
    attr.span().unwrap(),                                                                                                                 /*
        .                    PunctuationToken{tk: "."}
             ()              DelimGroup
               .             PunctuationToken{tk: "."}
                      ()     DelimGroup
                        ,    PunctuationToken{tk: ","}                                                                                    */
    &format!(                                                                                                                             /*
    &             PunctuationToken{tk: "&"}
           !      PunctuationToken{tk: "!"}
            (↲    <DelimGroup>                                                                                                            */
        "the `#[{}{}]` attribute can only be applied to fields of type {}",                                                               /*
        "the•`#[{}{}]`•attribute•can•only•be•applied•to•fields•of•type•{}"     Literal{kind: String}
                                                                          ,    PunctuationToken{tk: ","}                                  */
        name,                                                                                                                             /*
            ,    PunctuationToken{tk: ","}                                                                                                */
        match meta {                                                                                                                      /*
                   {↲    <DelimGroup>                                                                                                     */
            Meta::Path(_) => "",                                                                                                          /*
                ::                  PunctuationToken{tk: "::"}
                      (_)           DelimGroup
                       _            PunctuationToken{tk: "_"}
                          =>        PunctuationToken{tk: "=>"}
                             ""     Literal{kind: String}
                               ,    PunctuationToken{tk: ","}                                                                             */
            Meta::NameValue(_) => " = ...",                                                                                               /*
                ::                             PunctuationToken{tk: "::"}
                           (_)                 DelimGroup
                            _                  PunctuationToken{tk: "_"}
                               =>              PunctuationToken{tk: "=>"}
                                  "•=•..."     Literal{kind: String}
                                          ,    PunctuationToken{tk: ","}                                                                  */
            Meta::List(_) => "(...)",                                                                                                     /*
                ::                       PunctuationToken{tk: "::"}
                      (_)                DelimGroup
                       _                 PunctuationToken{tk: "_"}
                          =>             PunctuationToken{tk: "=>"}
                             "(...)"     Literal{kind: String}
                                    ,    PunctuationToken{tk: ","}                                                                        */
        },                                                                                                                                /*
••••••••}     </DelimGroup>
         ,    PunctuationToken{tk: ","}                                                                                                   */
        ty_name
    )                                                                                                                                     /*
••••)    </DelimGroup>                                                                                                                    */
);                                                                                                                                        /*
)     </MacroInvocation.segments>
)     </MacroInvocation>
);    </ExpressionStatement>                                                                                                              */

provide! {                                                                                                                                /*
provide!•{•↲    <ExpressionStatement{!semi}>
provide!•{•↲    <MacroInvocation>
         {•↲    <MacroInvocation.segments{dk: "{}"}>                                                                                      */
    <'tcx> tcx, def_id, other, cdata,                                                                                                     /*
    <                                    PunctuationToken{tk: "<"}
     'tcx                                LtIdentifier
         >                               PunctuationToken{tk: ">"}
              ,                          PunctuationToken{tk: ","}
                      ,                  PunctuationToken{tk: ","}
                             ,           PunctuationToken{tk: ","}
                                    ,    PunctuationToken{tk: ","}                                                                        */
    explicit_item_bounds => { table }                                                                                                     /*
                         =>              PunctuationToken{tk: "=>"}
                            {•table•}    DelimGroup                                                                                       */
    explicit_predicates_of => { table }                                                                                                   /*
                           =>              PunctuationToken{tk: "=>"}
                              {•table•}    DelimGroup                                                                                     */
    adt_destructor => {                                                                                                                   /*
                   =>       PunctuationToken{tk: "=>"}
                      {↲    <DelimGroup>                                                                                                  */
        let _ = cdata;                                                                                                                    /*
            _             PunctuationToken{tk: "_"}
              =           PunctuationToken{tk: "="}
                     ;    PunctuationToken{tk: ";"}                                                                                       */
        tcx.calculate_dtor(def_id, |_,_| Ok(()))                                                                                          /*
           .                                        PunctuationToken{tk: "."}
                          (def_id,•|_,_|•Ok(()))    DelimGroup
                                 ,                  PunctuationToken{tk: ","}
                                   |                PunctuationToken{tk: "|"}
                                    _               PunctuationToken{tk: "_"}
                                     ,              PunctuationToken{tk: ","}
                                      _             PunctuationToken{tk: "_"}
                                       |            PunctuationToken{tk: "|"}
                                           (())     DelimGroup
                                            ()      DelimGroup                                                                            */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    extern_crate => {                                                                                                                     /*
                 =>       PunctuationToken{tk: "=>"}
                    {↲    <DelimGroup>                                                                                                    */
        let r = *cdata.extern_crate.lock();                                                                                               /*
              =                                PunctuationToken{tk: "="}
                *                              PunctuationToken{tk: "*"}
                      .                        PunctuationToken{tk: "."}
                                   .           PunctuationToken{tk: "."}
                                        ()     DelimGroup
                                          ;    PunctuationToken{tk: ";"}                                                                  */
        r.map(|c| &*tcx.arena.alloc(c))                                                                                                   /*
         .                                 PunctuationToken{tk: "."}
             (|c|•&*tcx.arena.alloc(c))    DelimGroup
              |                            PunctuationToken{tk: "|"}
                |                          PunctuationToken{tk: "|"}
                  &                        PunctuationToken{tk: "&"}
                   *                       PunctuationToken{tk: "*"}
                       .                   PunctuationToken{tk: "."}
                             .             PunctuationToken{tk: "."}
                                   (c)     DelimGroup                                                                                     */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    reachable_non_generics => {                                                                                                           /*
                           =>       PunctuationToken{tk: "=>"}
                              {↲    <DelimGroup>                                                                                          */
        let reachable_non_generics = tcx                                                                                                  /*
                                   =    PunctuationToken{tk: "="}                                                                         */
            .exported_symbols(cdata.cnum)                                                                                                 /*
            .                                PunctuationToken{tk: "."}
                             (cdata.cnum)    DelimGroup
                                   .         PunctuationToken{tk: "."}                                                                    */
            .iter()                                                                                                                       /*
            .          PunctuationToken{tk: "."}
                 ()    DelimGroup                                                                                                         */
            .filter_map(|&(exported_symbol, export_info)| {                                                                               /*
            .                                                   PunctuationToken{tk: "."}
                       (|&(exported_symbol,•export_info)|•{↲    <DelimGroup>
                        |                                       PunctuationToken{tk: "|"}
                         &                                      PunctuationToken{tk: "&"}
                          (exported_symbol,•export_info)        DelimGroup
                                          ,                     PunctuationToken{tk: ","}
                                                        |       PunctuationToken{tk: "|"}
                                                          {↲    <DelimGroup>                                                              */
                if let ExportedSymbol::NonGeneric(def_id) = exported_symbol {                                                             /*
                                     ::                                           PunctuationToken{tk: "::"}
                                                 (def_id)                         DelimGroup
                                                          =                       PunctuationToken{tk: "="}
                                                                            {↲    <DelimGroup>                                            */
                    Some((def_id, export_info))                                                                                           /*
                        ((def_id,•export_info))    DelimGroup
                         (def_id,•export_info)     DelimGroup
                                ,                  PunctuationToken{tk: ","}                                                              */
                } else {                                                                                                                  /*
••••••••••••••••}            </DelimGroup>
                       {↲    <DelimGroup>                                                                                                 */
                    None
                }                                                                                                                         /*
••••••••••••••••}    </DelimGroup>                                                                                                        */
            })                                                                                                                            /*
••••••••••••}     </DelimGroup>
••••••••••••})    </DelimGroup>                                                                                                           */
            .collect();                                                                                                                   /*
            .              PunctuationToken{tk: "."}
                    ()     DelimGroup
                      ;    PunctuationToken{tk: ";"}                                                                                      */

        reachable_non_generics
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    dep_kind => {                                                                                                                         /*
             =>       PunctuationToken{tk: "=>"}
                {↲    <DelimGroup>                                                                                                        */
        let r = *cdata.dep_kind.lock();                                                                                                   /*
              =                            PunctuationToken{tk: "="}
                *                          PunctuationToken{tk: "*"}
                      .                    PunctuationToken{tk: "."}
                               .           PunctuationToken{tk: "."}
                                    ()     DelimGroup
                                      ;    PunctuationToken{tk: ";"}                                                                      */
        r
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    module_children => {                                                                                                                  /*
                    =>       PunctuationToken{tk: "=>"}
                       {↲    <DelimGroup>                                                                                                 */
        let mut result = SmallVec::<[_; 8]>::new();                                                                                       /*
                       =                               PunctuationToken{tk: "="}
                                 ::                    PunctuationToken{tk: "::"}
                                   <                   PunctuationToken{tk: "<"}
                                    [_;•8]             DelimGroup
                                     _                 PunctuationToken{tk: "_"}
                                      ;                PunctuationToken{tk: ";"}
                                        8              Literal{kind: Integer}
                                          >            PunctuationToken{tk: ">"}
                                           ::          PunctuationToken{tk: "::"}
                                                ()     DelimGroup
                                                  ;    PunctuationToken{tk: ";"}                                                          */
        cdata.for_each_module_child(def_id.index, |child| result.push(child), tcx.sess);                                                  /*
             .                                                                              PunctuationToken{tk: "."}
                                   (def_id.index,•|child|•result.push(child),•tcx.sess)     DelimGroup
                                          .                                                 PunctuationToken{tk: "."}
                                                ,                                           PunctuationToken{tk: ","}
                                                  |                                         PunctuationToken{tk: "|"}
                                                        |                                   PunctuationToken{tk: "|"}
                                                                .                           PunctuationToken{tk: "."}
                                                                     (child)                DelimGroup
                                                                            ,               PunctuationToken{tk: ","}
                                                                                 .          PunctuationToken{tk: "."}
                                                                                       ;    PunctuationToken{tk: ";"}                     */
        tcx.arena.alloc_slice(&result)                                                                                                    /*
           .                              PunctuationToken{tk: "."}
                 .                        PunctuationToken{tk: "."}
                             (&result)    DelimGroup
                              &           PunctuationToken{tk: "&"}                                                                       */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */

    missing_extern_crate_item => {                                                                                                        /*
                              =>       PunctuationToken{tk: "=>"}
                                 {↲    <DelimGroup>                                                                                       */
        let r = matches!(*cdata.extern_crate.borrow(), Some(extern_crate) if !extern_crate.is_direct());                                  /*
              =                                                                                             PunctuationToken{tk: "="}
                       !                                                                                    PunctuationToken{tk: "!"}
                        (*cdata.extern_crate.borrow(),•Some(extern_crate)•if•!extern_crate.is_direct())     DelimGroup
                         *                                                                                  PunctuationToken{tk: "*"}
                               .                                                                            PunctuationToken{tk: "."}
                                            .                                                               PunctuationToken{tk: "."}
                                                   ()                                                       DelimGroup
                                                     ,                                                      PunctuationToken{tk: ","}
                                                           (extern_crate)                                   DelimGroup
                                                                             !                              PunctuationToken{tk: "!"}
                                                                                          .                 PunctuationToken{tk: "."}
                                                                                                    ()      DelimGroup
                                                                                                       ;    PunctuationToken{tk: ";"}     */
        r
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */

    used_crate_source => { Lrc::clone(&cdata.source) }                                                                                    /*
                      =>                                  PunctuationToken{tk: "=>"}
                         {•Lrc::clone(&cdata.source)•}    DelimGroup
                              ::                          PunctuationToken{tk: "::"}
                                     (&cdata.source)      DelimGroup
                                      &                   PunctuationToken{tk: "&"}
                                            .             PunctuationToken{tk: "."}                                                       */
    debugger_visualizers => { cdata.get_debugger_visualizers() }                                                                          /*
                         =>                                         PunctuationToken{tk: "=>"}
                            {•cdata.get_debugger_visualizers()•}    DelimGroup
                                   .                                PunctuationToken{tk: "."}
                                                            ()      DelimGroup                                                            */

    exported_symbols => {                                                                                                                 /*
                     =>       PunctuationToken{tk: "=>"}
                        {↲    <DelimGroup>                                                                                                */
        let syms = cdata.exported_symbols(tcx);                                                                                           /*
                 =                                 PunctuationToken{tk: "="}
                        .                          PunctuationToken{tk: "."}
                                         (tcx)     DelimGroup
                                              ;    PunctuationToken{tk: ";"}                                                              */

        // a
        //•a    Comment{line}

        syms
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

cfg_if::cfg_if! {                                                                                                                         /*
cfg_if::cfg_if!•{↲    <ExpressionStatement{!semi}>
cfg_if::cfg_if!•{↲    <MacroInvocation>
cfg_if::cfg_if        ExpressionPath
                {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                */
    if #[cfg(unix)] {                                                                                                                     /*
       #                  PunctuationToken{tk: "#"}
        [cfg(unix)]       DelimGroup
            (unix)        DelimGroup
                    {↲    <DelimGroup>                                                                                                    */
    } else if #[cfg(any(target_os = "hermit",                                                                                             /*
••••}                                             </DelimGroup>
              #                                   PunctuationToken{tk: "#"}
               [cfg(any(target_os•=•"hermit",↲    <DelimGroup>
                   (any(target_os•=•"hermit",↲    <DelimGroup>
                       (target_os•=•"hermit",↲    <DelimGroup>
                                  =               PunctuationToken{tk: "="}
                                    "hermit"      Literal{kind: String}
                                            ,     PunctuationToken{tk: ","}                                                               */
                        target_os = "solid_asp3",                                                                                         /*
                                  =                  PunctuationToken{tk: "="}
                                    "solid_asp3"     Literal{kind: String}
                                                ,    PunctuationToken{tk: ","}                                                            */
                        all(target_vendor = "fortanix", target_env = "sgx")                                                               /*
                           (target_vendor•=•"fortanix",•target_env•=•"sgx")    DelimGroup
                                          =                                    PunctuationToken{tk: "="}
                                            "fortanix"                         Literal{kind: String}
                                                      ,                        PunctuationToken{tk: ","}
                                                                   =           PunctuationToken{tk: "="}
                                                                     "sgx"     Literal{kind: String}                                      */
    ))] {                                                                                                                                 /*
••••)         </DelimGroup>
••••))        </DelimGroup>
••••))]       </DelimGroup>
        {↲    <DelimGroup>                                                                                                                */
    } else if #[cfg(all(windows, not(miri)))] {                                                                                           /*
••••}                                               </DelimGroup>
              #                                     PunctuationToken{tk: "#"}
               [cfg(all(windows,•not(miri)))]       DelimGroup
                   (all(windows,•not(miri)))        DelimGroup
                       (windows,•not(miri))         DelimGroup
                               ,                    PunctuationToken{tk: ","}
                                    (miri)          DelimGroup
                                              {↲    <DelimGroup>                                                                          */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

ast_fragments! {                                                                                                                          /*
ast_fragments!•{↲    <ExpressionStatement{!semi}>
ast_fragments!•{↲    <MacroInvocation>
               {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                 */
    Expr(P<ast::Expr>) { "expression"; one fn visit_expr; fn visit_expr; fn make_expr; }                                                  /*
        (P<ast::Expr>)                                                                      DelimGroup
          <                                                                                 PunctuationToken{tk: "<"}
              ::                                                                            PunctuationToken{tk: "::"}
                    >                                                                       PunctuationToken{tk: ">"}
                       {•"expression";•one•fn•visit_expr;•fn•visit_expr;•fn•make_expr;•}    DelimGroup
                         "expression"                                                       Literal{kind: String}
                                     ;                                                      PunctuationToken{tk: ";"}
                                                        ;                                   PunctuationToken{tk: ";"}
                                                                       ;                    PunctuationToken{tk: ";"}
                                                                                     ;      PunctuationToken{tk: ";"}                     */
    Pat(P<ast::Pat>) { "pattern"; one fn visit_pat; fn visit_pat; fn make_pat; }                                                          /*
       (P<ast::Pat>)                                                                DelimGroup
         <                                                                          PunctuationToken{tk: "<"}
             ::                                                                     PunctuationToken{tk: "::"}
                  >                                                                 PunctuationToken{tk: ">"}
                     {•"pattern";•one•fn•visit_pat;•fn•visit_pat;•fn•make_pat;•}    DelimGroup
                       "pattern"                                                    Literal{kind: String}
                                ;                                                   PunctuationToken{tk: ";"}
                                                  ;                                 PunctuationToken{tk: ";"}
                                                                ;                   PunctuationToken{tk: ";"}
                                                                             ;      PunctuationToken{tk: ";"}                             */
    Ty(P<ast::Ty>) { "type"; one fn visit_ty; fn visit_ty; fn make_ty; }                                                                  /*
      (P<ast::Ty>)                                                          DelimGroup
        <                                                                   PunctuationToken{tk: "<"}
            ::                                                              PunctuationToken{tk: "::"}
                >                                                           PunctuationToken{tk: ">"}
                   {•"type";•one•fn•visit_ty;•fn•visit_ty;•fn•make_ty;•}    DelimGroup
                     "type"                                                 Literal{kind: String}
                           ;                                                PunctuationToken{tk: ";"}
                                            ;                               PunctuationToken{tk: ";"}
                                                         ;                  PunctuationToken{tk: ";"}
                                                                     ;      PunctuationToken{tk: ";"}                                     */
    Stmts(SmallVec<[ast::Stmt; 1]>) {                                                                                                     /*
         (SmallVec<[ast::Stmt;•1]>)       DelimGroup
                  <                       PunctuationToken{tk: "<"}
                   [ast::Stmt;•1]         DelimGroup
                       ::                 PunctuationToken{tk: "::"}
                             ;            PunctuationToken{tk: ";"}
                               1          Literal{kind: Integer}
                                 >        PunctuationToken{tk: ">"}
                                    {↲    <DelimGroup>                                                                                    */
        "statement"; many fn flat_map_stmt; fn visit_stmt(); fn make_stmts;                                                               /*
        "statement"                                                            Literal{kind: String}
                   ;                                                           PunctuationToken{tk: ";"}
                                          ;                                    PunctuationToken{tk: ";"}
                                                         ()                    DelimGroup
                                                           ;                   PunctuationToken{tk: ";"}
                                                                          ;    PunctuationToken{tk: ";"}                                  */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Items(SmallVec<[P<ast::Item>; 1]>) {                                                                                                  /*
         (SmallVec<[P<ast::Item>;•1]>)       DelimGroup
                  <                          PunctuationToken{tk: "<"}
                   [P<ast::Item>;•1]         DelimGroup
                     <                       PunctuationToken{tk: "<"}
                         ::                  PunctuationToken{tk: "::"}
                               >             PunctuationToken{tk: ">"}
                                ;            PunctuationToken{tk: ";"}
                                  1          Literal{kind: Integer}
                                    >        PunctuationToken{tk: ">"}
                                       {↲    <DelimGroup>                                                                                 */
        "item"; many fn flat_map_item; fn visit_item(); fn make_items;                                                                    /*
        "item"                                                            Literal{kind: String}
              ;                                                           PunctuationToken{tk: ";"}
                                     ;                                    PunctuationToken{tk: ";"}
                                                    ()                    DelimGroup
                                                      ;                   PunctuationToken{tk: ";"}
                                                                     ;    PunctuationToken{tk: ";"}                                       */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    TraitItems(SmallVec<[P<ast::AssocItem>; 1]>) {                                                                                        /*
              (SmallVec<[P<ast::AssocItem>;•1]>)       DelimGroup
                       <                               PunctuationToken{tk: "<"}
                        [P<ast::AssocItem>;•1]         DelimGroup
                          <                            PunctuationToken{tk: "<"}
                              ::                       PunctuationToken{tk: "::"}
                                         >             PunctuationToken{tk: ">"}
                                          ;            PunctuationToken{tk: ";"}
                                            1          Literal{kind: Integer}
                                              >        PunctuationToken{tk: ">"}
                                                 {↲    <DelimGroup>                                                                       */
        "trait item";                                                                                                                     /*
        "trait•item"     Literal{kind: String}
                    ;    PunctuationToken{tk: ";"}                                                                                        */
        many fn flat_map_trait_item;                                                                                                      /*
                                   ;    PunctuationToken{tk: ";"}                                                                         */
        fn visit_assoc_item(AssocCtxt::Trait);                                                                                            /*
                           (AssocCtxt::Trait)     DelimGroup
                                     ::           PunctuationToken{tk: "::"}
                                             ;    PunctuationToken{tk: ";"}                                                               */
        fn make_trait_items;                                                                                                              /*
                           ;    PunctuationToken{tk: ";"}                                                                                 */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    ImplItems(SmallVec<[P<ast::AssocItem>; 1]>) {                                                                                         /*
             (SmallVec<[P<ast::AssocItem>;•1]>)       DelimGroup
                      <                               PunctuationToken{tk: "<"}
                       [P<ast::AssocItem>;•1]         DelimGroup
                         <                            PunctuationToken{tk: "<"}
                             ::                       PunctuationToken{tk: "::"}
                                        >             PunctuationToken{tk: ">"}
                                         ;            PunctuationToken{tk: ";"}
                                           1          Literal{kind: Integer}
                                             >        PunctuationToken{tk: ">"}
                                                {↲    <DelimGroup>                                                                        */
        "impl item";                                                                                                                      /*
        "impl•item"     Literal{kind: String}
                   ;    PunctuationToken{tk: ";"}                                                                                         */
        many fn flat_map_impl_item;                                                                                                       /*
                                  ;    PunctuationToken{tk: ";"}                                                                          */
        fn visit_assoc_item(AssocCtxt::Impl);                                                                                             /*
                           (AssocCtxt::Impl)     DelimGroup
                                     ::          PunctuationToken{tk: "::"}
                                            ;    PunctuationToken{tk: ";"}                                                                */
        fn make_impl_items;                                                                                                               /*
                          ;    PunctuationToken{tk: ";"}                                                                                  */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    ForeignItems(SmallVec<[P<ast::ForeignItem>; 1]>) {                                                                                    /*
                (SmallVec<[P<ast::ForeignItem>;•1]>)       DelimGroup
                         <                                 PunctuationToken{tk: "<"}
                          [P<ast::ForeignItem>;•1]         DelimGroup
                            <                              PunctuationToken{tk: "<"}
                                ::                         PunctuationToken{tk: "::"}
                                             >             PunctuationToken{tk: ">"}
                                              ;            PunctuationToken{tk: ";"}
                                                1          Literal{kind: Integer}
                                                  >        PunctuationToken{tk: ">"}
                                                     {↲    <DelimGroup>                                                                   */
        "foreign item";                                                                                                                   /*
        "foreign•item"     Literal{kind: String}
                      ;    PunctuationToken{tk: ";"}                                                                                      */
        many fn flat_map_foreign_item;                                                                                                    /*
                                     ;    PunctuationToken{tk: ";"}                                                                       */
        fn visit_foreign_item();                                                                                                          /*
                             ()     DelimGroup
                               ;    PunctuationToken{tk: ";"}                                                                             */
        fn make_foreign_items;                                                                                                            /*
                             ;    PunctuationToken{tk: ";"}                                                                               */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Arms(SmallVec<[ast::Arm; 1]>) {                                                                                                       /*
        (SmallVec<[ast::Arm;•1]>)       DelimGroup
                 <                      PunctuationToken{tk: "<"}
                  [ast::Arm;•1]         DelimGroup
                      ::                PunctuationToken{tk: "::"}
                           ;            PunctuationToken{tk: ";"}
                             1          Literal{kind: Integer}
                               >        PunctuationToken{tk: ">"}
                                  {↲    <DelimGroup>                                                                                      */
        "match arm"; many fn flat_map_arm; fn visit_arm(); fn make_arms;                                                                  /*
        "match•arm"                                                         Literal{kind: String}
                   ;                                                        PunctuationToken{tk: ";"}
                                         ;                                  PunctuationToken{tk: ";"}
                                                       ()                   DelimGroup
                                                         ;                  PunctuationToken{tk: ";"}
                                                                       ;    PunctuationToken{tk: ";"}                                     */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    ExprFields(SmallVec<[ast::ExprField; 1]>) {                                                                                           /*
              (SmallVec<[ast::ExprField;•1]>)       DelimGroup
                       <                            PunctuationToken{tk: "<"}
                        [ast::ExprField;•1]         DelimGroup
                            ::                      PunctuationToken{tk: "::"}
                                       ;            PunctuationToken{tk: ";"}
                                         1          Literal{kind: Integer}
                                           >        PunctuationToken{tk: ">"}
                                              {↲    <DelimGroup>                                                                          */
        "field expression"; many fn flat_map_expr_field; fn visit_expr_field(); fn make_expr_fields;                                      /*
        "field•expression"                                                                              Literal{kind: String}
                          ;                                                                             PunctuationToken{tk: ";"}
                                                       ;                                                PunctuationToken{tk: ";"}
                                                                            ()                          DelimGroup
                                                                              ;                         PunctuationToken{tk: ";"}
                                                                                                   ;    PunctuationToken{tk: ";"}         */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    PatFields(SmallVec<[ast::PatField; 1]>) {                                                                                             /*
             (SmallVec<[ast::PatField;•1]>)       DelimGroup
                      <                           PunctuationToken{tk: "<"}
                       [ast::PatField;•1]         DelimGroup
                           ::                     PunctuationToken{tk: "::"}
                                     ;            PunctuationToken{tk: ";"}
                                       1          Literal{kind: Integer}
                                         >        PunctuationToken{tk: ">"}
                                            {↲    <DelimGroup>                                                                            */
        "field pattern";                                                                                                                  /*
        "field•pattern"     Literal{kind: String}
                       ;    PunctuationToken{tk: ";"}                                                                                     */
        many fn flat_map_pat_field;                                                                                                       /*
                                  ;    PunctuationToken{tk: ";"}                                                                          */
        fn visit_pat_field();                                                                                                             /*
                          ()     DelimGroup
                            ;    PunctuationToken{tk: ";"}                                                                                */
        fn make_pat_fields;                                                                                                               /*
                          ;    PunctuationToken{tk: ";"}                                                                                  */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    GenericParams(SmallVec<[ast::GenericParam; 1]>) {                                                                                     /*
                 (SmallVec<[ast::GenericParam;•1]>)       DelimGroup
                          <                               PunctuationToken{tk: "<"}
                           [ast::GenericParam;•1]         DelimGroup
                               ::                         PunctuationToken{tk: "::"}
                                             ;            PunctuationToken{tk: ";"}
                                               1          Literal{kind: Integer}
                                                 >        PunctuationToken{tk: ">"}
                                                    {↲    <DelimGroup>                                                                    */
        "generic parameter";                                                                                                              /*
        "generic•parameter"     Literal{kind: String}
                           ;    PunctuationToken{tk: ";"}                                                                                 */
        many fn flat_map_generic_param;                                                                                                   /*
                                      ;    PunctuationToken{tk: ";"}                                                                      */
        fn visit_generic_param();                                                                                                         /*
                              ()     DelimGroup
                                ;    PunctuationToken{tk: ";"}                                                                            */
        fn make_generic_params;                                                                                                           /*
                              ;    PunctuationToken{tk: ";"}                                                                              */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Params(SmallVec<[ast::Param; 1]>) {                                                                                                   /*
          (SmallVec<[ast::Param;•1]>)       DelimGroup
                   <                        PunctuationToken{tk: "<"}
                    [ast::Param;•1]         DelimGroup
                        ::                  PunctuationToken{tk: "::"}
                               ;            PunctuationToken{tk: ";"}
                                 1          Literal{kind: Integer}
                                   >        PunctuationToken{tk: ">"}
                                      {↲    <DelimGroup>                                                                                  */
        "function parameter"; many fn flat_map_param; fn visit_param(); fn make_params;                                                   /*
        "function•parameter"                                                               Literal{kind: String}
                            ;                                                              PunctuationToken{tk: ";"}
                                                    ;                                      PunctuationToken{tk: ";"}
                                                                    ()                     DelimGroup
                                                                      ;                    PunctuationToken{tk: ";"}
                                                                                      ;    PunctuationToken{tk: ";"}                      */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    FieldDefs(SmallVec<[ast::FieldDef; 1]>) {                                                                                             /*
             (SmallVec<[ast::FieldDef;•1]>)       DelimGroup
                      <                           PunctuationToken{tk: "<"}
                       [ast::FieldDef;•1]         DelimGroup
                           ::                     PunctuationToken{tk: "::"}
                                     ;            PunctuationToken{tk: ";"}
                                       1          Literal{kind: Integer}
                                         >        PunctuationToken{tk: ">"}
                                            {↲    <DelimGroup>                                                                            */
        "field";                                                                                                                          /*
        "field"     Literal{kind: String}
               ;    PunctuationToken{tk: ";"}                                                                                             */
        many fn flat_map_field_def;                                                                                                       /*
                                  ;    PunctuationToken{tk: ";"}                                                                          */
        fn visit_field_def();                                                                                                             /*
                          ()     DelimGroup
                            ;    PunctuationToken{tk: ";"}                                                                                */
        fn make_field_defs;                                                                                                               /*
                          ;    PunctuationToken{tk: ";"}                                                                                  */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Variants(SmallVec<[ast::Variant; 1]>) {                                                                                               /*
            (SmallVec<[ast::Variant;•1]>)       DelimGroup
                     <                          PunctuationToken{tk: "<"}
                      [ast::Variant;•1]         DelimGroup
                          ::                    PunctuationToken{tk: "::"}
                                   ;            PunctuationToken{tk: ";"}
                                     1          Literal{kind: Integer}
                                       >        PunctuationToken{tk: ">"}
                                          {↲    <DelimGroup>                                                                              */
        "variant"; many fn flat_map_variant; fn visit_variant(); fn make_variants;                                                        /*
        "variant"                                                                     Literal{kind: String}
                 ;                                                                    PunctuationToken{tk: ";"}
                                           ;                                          PunctuationToken{tk: ";"}
                                                             ()                       DelimGroup
                                                               ;                      PunctuationToken{tk: ";"}
                                                                                 ;    PunctuationToken{tk: ";"}                           */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Crate(ast::Crate) { "crate"; one fn visit_crate; fn visit_crate; fn make_crate; }                                                     /*
         (ast::Crate)                                                                    DelimGroup
             ::                                                                          PunctuationToken{tk: "::"}
                      {•"crate";•one•fn•visit_crate;•fn•visit_crate;•fn•make_crate;•}    DelimGroup
                        "crate"                                                          Literal{kind: String}
                               ;                                                         PunctuationToken{tk: ";"}
                                                   ;                                     PunctuationToken{tk: ";"}
                                                                   ;                     PunctuationToken{tk: ";"}
                                                                                  ;      PunctuationToken{tk: ";"}                        */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

sides_mapping_methods! {                                                                                                                  /*
sides_mapping_methods!•{↲    <ExpressionStatement{!semi}>
sides_mapping_methods!•{↲    <MacroInvocation>
                       {↲    <MacroInvocation.segments{dk: "{}"}>                                                                         */
    MainStartCrossStart::InlineStartBlockStart => {                                                                                       /*
                       ::                               PunctuationToken{tk: "::"}
                                               =>       PunctuationToken{tk: "=>"}
                                                  {↲    <DelimGroup>                                                                      */
        main_start <=> inline_start,                                                                                                      /*
                   <=                   PunctuationToken{tk: "<="}
                     >                  PunctuationToken{tk: ">"}
                                   ,    PunctuationToken{tk: ","}                                                                         */
        cross_start <=> block_start,                                                                                                      /*
                    <=                  PunctuationToken{tk: "<="}
                      >                 PunctuationToken{tk: ">"}
                                   ,    PunctuationToken{tk: ","}                                                                         */
        main_end <=> inline_end,                                                                                                          /*
                 <=                 PunctuationToken{tk: "<="}
                   >                PunctuationToken{tk: ">"}
                               ,    PunctuationToken{tk: ","}                                                                             */
        cross_end <=> block_end,                                                                                                          /*
                  <=                PunctuationToken{tk: "<="}
                    >               PunctuationToken{tk: ">"}
                               ,    PunctuationToken{tk: ","}                                                                             */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

fuzz_target!(|data: &[u8]| if let Some(first) = data.first() {                                                                            /*
fuzz_target!(|data:•&[u8]|•if•let•Some(first)•=•data.first()•{↲    <ExpressionStatement{semi}>
fuzz_target!(|data:•&[u8]|•if•let•Some(first)•=•data.first()•{↲    <MacroInvocation>
            (|data:•&[u8]|•if•let•Some(first)•=•data.first()•{↲    <MacroInvocation.segments{dk: "()"}>
             |                                                     PunctuationToken{tk: "|"}
                  :                                                PunctuationToken{tk: ":"}
                    &                                              PunctuationToken{tk: "&"}
                     [u8]                                          DelimGroup
                         |                                         PunctuationToken{tk: "|"}
                                      (first)                      DelimGroup
                                              =                    PunctuationToken{tk: "="}
                                                    .              PunctuationToken{tk: "."}
                                                          ()       DelimGroup
                                                             {↲    <DelimGroup>                                                           */
    let index = *first as usize;                                                                                                          /*
              =                     PunctuationToken{tk: "="}
                *                   PunctuationToken{tk: "*"}
                               ;    PunctuationToken{tk: ";"}                                                                             */
    if index >= ENCODINGS.len() {                                                                                                         /*
             >=                       PunctuationToken{tk: ">="}
                         .            PunctuationToken{tk: "."}
                             ()       DelimGroup
                                {↲    <DelimGroup>                                                                                        */
        return;                                                                                                                           /*
              ;    PunctuationToken{tk: ";"}                                                                                              */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    let encoding = ENCODINGS[index];                                                                                                      /*
                 =                      PunctuationToken{tk: "="}
                            [index]     DelimGroup
                                   ;    PunctuationToken{tk: ";"}                                                                         */
    dispatch_test(encoding, &data[1..]);                                                                                                  /*
                 (encoding,•&data[1..])     DelimGroup
                          ,                 PunctuationToken{tk: ","}
                            &               PunctuationToken{tk: "&"}
                                 [1..]      DelimGroup
                                  1         Literal{kind: Integer}
                                   ..       PunctuationToken{tk: ".."}
                                       ;    PunctuationToken{tk: ";"}                                                                     */
});                                                                                                                                       /*
}      </DelimGroup>
})     </MacroInvocation.segments>
})     </MacroInvocation>
});    </ExpressionStatement>                                                                                                             */

assert_eq!(count_compound_ops!(foo<=>bar <<<! -baz ++), 4);                                                                               /*
assert_eq!(count_compound_ops!(foo<=>bar•<<<!•-baz•++),•4);    ExpressionStatement{semi}
assert_eq!(count_compound_ops!(foo<=>bar•<<<!•-baz•++),•4)     MacroInvocation
          (count_compound_ops!(foo<=>bar•<<<!•-baz•++),•4)     MacroInvocation.segments{dk: "()"}
                             !                                 PunctuationToken{tk: "!"}
                              (foo<=>bar•<<<!•-baz•++)         DelimGroup
                                  <=                           PunctuationToken{tk: "<="}
                                    >                          PunctuationToken{tk: ">"}
                                         <<                    PunctuationToken{tk: "<<"}
                                           <                   PunctuationToken{tk: "<"}
                                            !                  PunctuationToken{tk: "!"}
                                              -                PunctuationToken{tk: "-"}
                                                   +           PunctuationToken{tk: "+"}
                                                    +          PunctuationToken{tk: "+"}
                                                      ,        PunctuationToken{tk: ","}
                                                        4      Literal{kind: Integer}                                                     */

print_bang! {                                                                                                                             /*
print_bang!•{↲    <ExpressionStatement{!semi}>
print_bang!•{↲    <MacroInvocation>
            {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                    */

/**                                                                                                                                       /*
/**↲    <Comment{!line}>                                                                                                                */*/
*******
* DOC *
* DOC *
* DOC *
*******
*/                                                                                                                                      /*/*
*/    </Comment>                                                                                                                          */
pub struct S;                                                                                                                             /*
            ;    PunctuationToken{tk: ";"}                                                                                                */

}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

c! { pub const A: i32 = 0; }                                                                                                              /*
c!•{•pub•const•A:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•const•A:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
                :               PunctuationToken{tk: ":"}
                      =         PunctuationToken{tk: "="}
                        0       Literal{kind: Integer}
                         ;      PunctuationToken{tk: ";"}                                                                                 */
c! { pub enum B {} }                                                                                                                      /*
c!•{•pub•enum•B•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•enum•B•{}•}    MacroInvocation.segments{dk: "{}"}
                {}      DelimGroup                                                                                                        */
c! { pub extern "C" fn c() {} }                                                                                                           /*
c!•{•pub•extern•"C"•fn•c()•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•extern•"C"•fn•c()•{}•}    MacroInvocation.segments{dk: "{}"}
                "C"                Literal{kind: String}
                        ()         DelimGroup
                           {}      DelimGroup                                                                                             */
c! { pub mod d {} }                                                                                                                       /*
c!•{•pub•mod•d•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•mod•d•{}•}    MacroInvocation.segments{dk: "{}"}
               {}      DelimGroup                                                                                                         */
c! { pub static E: i32 = 0; }                                                                                                             /*
c!•{•pub•static•E:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•static•E:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
                 :               PunctuationToken{tk: ":"}
                       =         PunctuationToken{tk: "="}
                         0       Literal{kind: Integer}
                          ;      PunctuationToken{tk: ";"}                                                                                */
c! { pub struct F; }                                                                                                                      /*
c!•{•pub•struct•F;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•struct•F;•}    MacroInvocation.segments{dk: "{}"}
                 ;      PunctuationToken{tk: ";"}                                                                                         */
c! { pub trait G {} }                                                                                                                     /*
c!•{•pub•trait•G•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•trait•G•{}•}    MacroInvocation.segments{dk: "{}"}
                 {}      DelimGroup                                                                                                       */
c! { pub type H = i32; }                                                                                                                  /*
c!•{•pub•type•H•=•i32;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•type•H•=•i32;•}    MacroInvocation.segments{dk: "{}"}
                =           PunctuationToken{tk: "="}
                     ;      PunctuationToken{tk: ";"}                                                                                     */
c! { pub use A as I; }                                                                                                                    /*
c!•{•pub•use•A•as•I;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•use•A•as•I;•}    MacroInvocation.segments{dk: "{}"}
                   ;      PunctuationToken{tk: ";"}                                                                                       */
c! { const A: i32 = 0; }                                                                                                                  /*
c!•{•const•A:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•const•A:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
            :               PunctuationToken{tk: ":"}
                  =         PunctuationToken{tk: "="}
                    0       Literal{kind: Integer}
                     ;      PunctuationToken{tk: ";"}                                                                                     */
c! { enum B {} }                                                                                                                          /*
c!•{•enum•B•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•enum•B•{}•}    MacroInvocation.segments{dk: "{}"}
            {}      DelimGroup                                                                                                            */
c! { extern "C" fn c() {} }                                                                                                               /*
c!•{•extern•"C"•fn•c()•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•extern•"C"•fn•c()•{}•}    MacroInvocation.segments{dk: "{}"}
            "C"                Literal{kind: String}
                    ()         DelimGroup
                       {}      DelimGroup                                                                                                 */
c! { mod d {} }                                                                                                                           /*
c!•{•mod•d•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•mod•d•{}•}    MacroInvocation.segments{dk: "{}"}
           {}      DelimGroup                                                                                                             */
c! { static E: i32 = 0; }                                                                                                                 /*
c!•{•static•E:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•static•E:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
             :               PunctuationToken{tk: ":"}
                   =         PunctuationToken{tk: "="}
                     0       Literal{kind: Integer}
                      ;      PunctuationToken{tk: ";"}                                                                                    */
c! { struct F; }                                                                                                                          /*
c!•{•struct•F;•}    ExpressionStatement{!semi}, MacroInvocation
   {•struct•F;•}    MacroInvocation.segments{dk: "{}"}
             ;      PunctuationToken{tk: ";"}                                                                                             */
c! { trait G {} }                                                                                                                         /*
c!•{•trait•G•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•trait•G•{}•}    MacroInvocation.segments{dk: "{}"}
             {}      DelimGroup                                                                                                           */
c! { type H = i32; }                                                                                                                      /*
c!•{•type•H•=•i32;•}    ExpressionStatement{!semi}, MacroInvocation
   {•type•H•=•i32;•}    MacroInvocation.segments{dk: "{}"}
            =           PunctuationToken{tk: "="}
                 ;      PunctuationToken{tk: ";"}                                                                                         */
c! { use A as I; }                                                                                                                        /*
c!•{•use•A•as•I;•}    ExpressionStatement{!semi}, MacroInvocation
   {•use•A•as•I;•}    MacroInvocation.segments{dk: "{}"}
               ;      PunctuationToken{tk: ";"}                                                                                           */
c! { pub(crate) const A: i32 = 0; }                                                                                                       /*
c!•{•pub(crate)•const•A:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•const•A:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
        (crate)                        DelimGroup
                       :               PunctuationToken{tk: ":"}
                             =         PunctuationToken{tk: "="}
                               0       Literal{kind: Integer}
                                ;      PunctuationToken{tk: ";"}                                                                          */
c! { pub(crate) enum B {} }                                                                                                               /*
c!•{•pub(crate)•enum•B•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•enum•B•{}•}    MacroInvocation.segments{dk: "{}"}
        (crate)                DelimGroup
                       {}      DelimGroup                                                                                                 */
c! { pub(crate) extern "C" fn c() {} }                                                                                                    /*
c!•{•pub(crate)•extern•"C"•fn•c()•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•extern•"C"•fn•c()•{}•}    MacroInvocation.segments{dk: "{}"}
        (crate)                           DelimGroup
                       "C"                Literal{kind: String}
                               ()         DelimGroup
                                  {}      DelimGroup                                                                                      */
c! { pub(crate) mod d {} }                                                                                                                /*
c!•{•pub(crate)•mod•d•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•mod•d•{}•}    MacroInvocation.segments{dk: "{}"}
        (crate)               DelimGroup
                      {}      DelimGroup                                                                                                  */
c! { pub(crate) static E: i32 = 0; }                                                                                                      /*
c!•{•pub(crate)•static•E:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•static•E:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
        (crate)                         DelimGroup
                        :               PunctuationToken{tk: ":"}
                              =         PunctuationToken{tk: "="}
                                0       Literal{kind: Integer}
                                 ;      PunctuationToken{tk: ";"}                                                                         */
c! { pub(crate) struct F; }                                                                                                               /*
c!•{•pub(crate)•struct•F;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•struct•F;•}    MacroInvocation.segments{dk: "{}"}
        (crate)                DelimGroup
                        ;      PunctuationToken{tk: ";"}                                                                                  */
c! { pub(crate) trait G {} }                                                                                                              /*
c!•{•pub(crate)•trait•G•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•trait•G•{}•}    MacroInvocation.segments{dk: "{}"}
        (crate)                 DelimGroup
                        {}      DelimGroup                                                                                                */
c! { pub(crate) type H = i32; }                                                                                                           /*
c!•{•pub(crate)•type•H•=•i32;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•type•H•=•i32;•}    MacroInvocation.segments{dk: "{}"}
        (crate)                    DelimGroup
                       =           PunctuationToken{tk: "="}
                            ;      PunctuationToken{tk: ";"}                                                                              */
c! { pub(crate) use A as I; }                                                                                                             /*
c!•{•pub(crate)•use•A•as•I;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•use•A•as•I;•}    MacroInvocation.segments{dk: "{}"}
        (crate)                  DelimGroup
                          ;      PunctuationToken{tk: ";"}                                                                                */
c! { crate const A: i32 = 0; }                                                                                                            /*
c!•{•crate•const•A:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•crate•const•A:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
                  :               PunctuationToken{tk: ":"}
                        =         PunctuationToken{tk: "="}
                          0       Literal{kind: Integer}
                           ;      PunctuationToken{tk: ";"}                                                                               */
c! { crate enum B {} }                                                                                                                    /*
c!•{•crate•enum•B•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•crate•enum•B•{}•}    MacroInvocation.segments{dk: "{}"}
                  {}      DelimGroup                                                                                                      */
c! { crate extern "C" fn c() {} }                                                                                                         /*
c!•{•crate•extern•"C"•fn•c()•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•crate•extern•"C"•fn•c()•{}•}    MacroInvocation.segments{dk: "{}"}
                  "C"                Literal{kind: String}
                          ()         DelimGroup
                             {}      DelimGroup                                                                                           */
c! { crate mod d {} }                                                                                                                     /*
c!•{•crate•mod•d•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•crate•mod•d•{}•}    MacroInvocation.segments{dk: "{}"}
                 {}      DelimGroup                                                                                                       */
c! { crate static E: i32 = 0; }                                                                                                           /*
c!•{•crate•static•E:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•crate•static•E:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
                   :               PunctuationToken{tk: ":"}
                         =         PunctuationToken{tk: "="}
                           0       Literal{kind: Integer}
                            ;      PunctuationToken{tk: ";"}                                                                              */
c! { crate struct F; }                                                                                                                    /*
c!•{•crate•struct•F;•}    ExpressionStatement{!semi}, MacroInvocation
   {•crate•struct•F;•}    MacroInvocation.segments{dk: "{}"}
                   ;      PunctuationToken{tk: ";"}                                                                                       */
c! { crate trait G {} }                                                                                                                   /*
c!•{•crate•trait•G•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•crate•trait•G•{}•}    MacroInvocation.segments{dk: "{}"}
                   {}      DelimGroup                                                                                                     */
c! { crate type H = i32; }                                                                                                                /*
c!•{•crate•type•H•=•i32;•}    ExpressionStatement{!semi}, MacroInvocation
   {•crate•type•H•=•i32;•}    MacroInvocation.segments{dk: "{}"}
                  =           PunctuationToken{tk: "="}
                       ;      PunctuationToken{tk: ";"}                                                                                   */
c! { crate use A as I; }                                                                                                                  /*
c!•{•crate•use•A•as•I;•}    ExpressionStatement{!semi}, MacroInvocation
   {•crate•use•A•as•I;•}    MacroInvocation.segments{dk: "{}"}
                     ;      PunctuationToken{tk: ";"}                                                                                     */
c! { pub(in foo) const A: i32 = 0; }                                                                                                      /*
c!•{•pub(in•foo)•const•A:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(in•foo)•const•A:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
        (in•foo)                        DelimGroup
                        :               PunctuationToken{tk: ":"}
                              =         PunctuationToken{tk: "="}
                                0       Literal{kind: Integer}
                                 ;      PunctuationToken{tk: ";"}                                                                         */
c! { pub(in foo) enum B {} }                                                                                                              /*
c!•{•pub(in•foo)•enum•B•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(in•foo)•enum•B•{}•}    MacroInvocation.segments{dk: "{}"}
        (in•foo)                DelimGroup
                        {}      DelimGroup                                                                                                */
c! { pub(in foo) extern "C" fn c() {} }                                                                                                   /*
c!•{•pub(in•foo)•extern•"C"•fn•c()•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(in•foo)•extern•"C"•fn•c()•{}•}    MacroInvocation.segments{dk: "{}"}
        (in•foo)                           DelimGroup
                        "C"                Literal{kind: String}
                                ()         DelimGroup
                                   {}      DelimGroup                                                                                     */
c! { pub(in foo) mod d {} }                                                                                                               /*
c!•{•pub(in•foo)•mod•d•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(in•foo)•mod•d•{}•}    MacroInvocation.segments{dk: "{}"}
        (in•foo)               DelimGroup
                       {}      DelimGroup                                                                                                 */
c! { pub(in foo) static E: i32 = 0; }                                                                                                     /*
c!•{•pub(in•foo)•static•E:•i32•=•0;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(in•foo)•static•E:•i32•=•0;•}    MacroInvocation.segments{dk: "{}"}
        (in•foo)                         DelimGroup
                         :               PunctuationToken{tk: ":"}
                               =         PunctuationToken{tk: "="}
                                 0       Literal{kind: Integer}
                                  ;      PunctuationToken{tk: ";"}                                                                        */
c! { pub(in foo) struct F; }                                                                                                              /*
c!•{•pub(in•foo)•struct•F;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(in•foo)•struct•F;•}    MacroInvocation.segments{dk: "{}"}
        (in•foo)                DelimGroup
                         ;      PunctuationToken{tk: ";"}                                                                                 */
c! { pub(in foo) trait G {} }                                                                                                             /*
c!•{•pub(in•foo)•trait•G•{}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(in•foo)•trait•G•{}•}    MacroInvocation.segments{dk: "{}"}
        (in•foo)                 DelimGroup
                         {}      DelimGroup                                                                                               */
c! { pub(in foo) type H = i32; }                                                                                                          /*
c!•{•pub(in•foo)•type•H•=•i32;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(in•foo)•type•H•=•i32;•}    MacroInvocation.segments{dk: "{}"}
        (in•foo)                    DelimGroup
                        =           PunctuationToken{tk: "="}
                             ;      PunctuationToken{tk: ";"}                                                                             */
c! { pub(in foo) use A as I; }                                                                                                            /*
c!•{•pub(in•foo)•use•A•as•I;•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(in•foo)•use•A•as•I;•}    MacroInvocation.segments{dk: "{}"}
        (in•foo)                  DelimGroup
                           ;      PunctuationToken{tk: ";"}                                                                               */
c! { pub(crate) struct A { pub a: i32, b: i32, pub(crate) c: i32 } }                                                                      /*
c!•{•pub(crate)•struct•A•{•pub•a:•i32,•b:•i32,•pub(crate)•c:•i32•}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•struct•A•{•pub•a:•i32,•b:•i32,•pub(crate)•c:•i32•}•}    MacroInvocation.segments{dk: "{}"}
        (crate)                                                         DelimGroup
                         {•pub•a:•i32,•b:•i32,•pub(crate)•c:•i32•}      DelimGroup
                                :                                       PunctuationToken{tk: ":"}
                                     ,                                  PunctuationToken{tk: ","}
                                        :                               PunctuationToken{tk: ":"}
                                             ,                          PunctuationToken{tk: ","}
                                                  (crate)               DelimGroup
                                                           :            PunctuationToken{tk: ":"}                                         */
c! { pub struct B { a: i32, pub(crate) b: i32, pub c: i32 } }                                                                             /*
c!•{•pub•struct•B•{•a:•i32,•pub(crate)•b:•i32,•pub•c:•i32•}•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•struct•B•{•a:•i32,•pub(crate)•b:•i32,•pub•c:•i32•}•}    MacroInvocation.segments{dk: "{}"}
                  {•a:•i32,•pub(crate)•b:•i32,•pub•c:•i32•}      DelimGroup
                     :                                           PunctuationToken{tk: ":"}
                          ,                                      PunctuationToken{tk: ","}
                               (crate)                           DelimGroup
                                        :                        PunctuationToken{tk: ":"}
                                             ,                   PunctuationToken{tk: ","}
                                                    :            PunctuationToken{tk: ":"}                                                */
c! { struct C { pub(crate) a: i32, pub b: i32, c: i32 } }                                                                                 /*
c!•{•struct•C•{•pub(crate)•a:•i32,•pub•b:•i32,•c:•i32•}•}    ExpressionStatement{!semi}, MacroInvocation
   {•struct•C•{•pub(crate)•a:•i32,•pub•b:•i32,•c:•i32•}•}    MacroInvocation.segments{dk: "{}"}
              {•pub(crate)•a:•i32,•pub•b:•i32,•c:•i32•}      DelimGroup
                   (crate)                                   DelimGroup
                            :                                PunctuationToken{tk: ":"}
                                 ,                           PunctuationToken{tk: ","}
                                        :                    PunctuationToken{tk: ":"}
                                             ,               PunctuationToken{tk: ","}
                                                :            PunctuationToken{tk: ":"}                                                    */

c! { pub(crate) struct D (pub i32, i32, pub(crate) i32); }                                                                                /*
c!•{•pub(crate)•struct•D•(pub•i32,•i32,•pub(crate)•i32);•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub(crate)•struct•D•(pub•i32,•i32,•pub(crate)•i32);•}    MacroInvocation.segments{dk: "{}"}
        (crate)                                               DelimGroup
                         (pub•i32,•i32,•pub(crate)•i32)       DelimGroup
                                 ,                            PunctuationToken{tk: ","}
                                      ,                       PunctuationToken{tk: ","}
                                           (crate)            DelimGroup
                                                       ;      PunctuationToken{tk: ";"}                                                   */
c! { pub struct E (i32, pub(crate) i32, pub i32); }                                                                                       /*
c!•{•pub•struct•E•(i32,•pub(crate)•i32,•pub•i32);•}    ExpressionStatement{!semi}, MacroInvocation
   {•pub•struct•E•(i32,•pub(crate)•i32,•pub•i32);•}    MacroInvocation.segments{dk: "{}"}
                  (i32,•pub(crate)•i32,•pub•i32)       DelimGroup
                      ,                                PunctuationToken{tk: ","}
                           (crate)                     DelimGroup
                                      ,                PunctuationToken{tk: ","}
                                                ;      PunctuationToken{tk: ";"}                                                          */
c! { struct F (pub(crate) i32, pub i32, i32); }                                                                                           /*
c!•{•struct•F•(pub(crate)•i32,•pub•i32,•i32);•}    ExpressionStatement{!semi}, MacroInvocation
   {•struct•F•(pub(crate)•i32,•pub•i32,•i32);•}    MacroInvocation.segments{dk: "{}"}
              (pub(crate)•i32,•pub•i32,•i32)       DelimGroup
                  (crate)                          DelimGroup
                             ,                     PunctuationToken{tk: ","}
                                      ,            PunctuationToken{tk: ","}
                                            ;      PunctuationToken{tk: ";"}                                                              */

c!(pub(crate);                                                                                                                            /*
c!(pub(crate);↲    <ExpressionStatement{semi}>
c!(pub(crate);↲    <MacroInvocation>
  (pub(crate);↲    <MacroInvocation.segments{dk: "()"}>
      (crate)      DelimGroup
             ;     PunctuationToken{tk: ";"}                                                                                              */
c!(pub(self)));                                                                                                                           /*
 !                 PunctuationToken{tk: "!"}
  (pub(self))      DelimGroup
      (self)       DelimGroup
c!(pub(self)))     </MacroInvocation.segments>
c!(pub(self)))     </MacroInvocation>
c!(pub(self)));    </ExpressionStatement>                                                                                                 */
c!(pub(super));                                                                                                                           /*
c!(pub(super));    ExpressionStatement{semi}
c!(pub(super))     MacroInvocation
  (pub(super))     MacroInvocation.segments{dk: "()"}
      (super)      DelimGroup                                                                                                             */
c!(pub(in self));                                                                                                                         /*
c!(pub(in•self));    ExpressionStatement{semi}
c!(pub(in•self))     MacroInvocation
  (pub(in•self))     MacroInvocation.segments{dk: "()"}
      (in•self)      DelimGroup                                                                                                           */
c!(pub(in super));                                                                                                                        /*
c!(pub(in•super));    ExpressionStatement{semi}
c!(pub(in•super))     MacroInvocation
  (pub(in•super))     MacroInvocation.segments{dk: "()"}
      (in•super)      DelimGroup                                                                                                          */
c!(pub(in path::to));                                                                                                                     /*
c!(pub(in•path::to));    ExpressionStatement{semi}
c!(pub(in•path::to))     MacroInvocation
  (pub(in•path::to))     MacroInvocation.segments{dk: "()"}
      (in•path::to)      DelimGroup
              ::         PunctuationToken{tk: "::"}                                                                                       */
c!(pub(in ::path::to));                                                                                                                   /*
c!(pub(in•::path::to));    ExpressionStatement{semi}
c!(pub(in•::path::to))     MacroInvocation
  (pub(in•::path::to))     MacroInvocation.segments{dk: "()"}
      (in•::path::to)      DelimGroup
          ::               PunctuationToken{tk: "::"}
                ::         PunctuationToken{tk: "::"}                                                                                     */
c!(pub(in self::path::to));                                                                                                               /*
c!(pub(in•self::path::to));    ExpressionStatement{semi}
c!(pub(in•self::path::to))     MacroInvocation
  (pub(in•self::path::to))     MacroInvocation.segments{dk: "()"}
      (in•self::path::to)      DelimGroup
              ::               PunctuationToken{tk: "::"}
                    ::         PunctuationToken{tk: "::"}                                                                                 */
c!(pub(in super::path::to));                                                                                                              /*
c!(pub(in•super::path::to));    ExpressionStatement{semi}
c!(pub(in•super::path::to))     MacroInvocation
  (pub(in•super::path::to))     MacroInvocation.segments{dk: "()"}
      (in•super::path::to)      DelimGroup
               ::               PunctuationToken{tk: "::"}
                     ::         PunctuationToken{tk: "::"}
c!(pub(in•super::path::to));    </Program.ast>
c!(pub(in•super::path::to));    </Program>                                                                                                */
// Discarded Nodes: 0
// Parsed Nodes: 3179
// state_rollbacks: 6
// Total '.charCodeAt()' calls: 20595 (18% re-reads)
// Unnecessary 'skip_whitespace()' calls: 1122
// source: "../../samples/macro/macro.invocation.rs"