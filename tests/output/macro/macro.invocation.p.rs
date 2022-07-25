
declare_clippy_lint! {                                                                                                                    /*
declare_clippy_lint!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                       */
    /// ### q
    ///•###•q    Comment
    /// ```rust
    ///•```rust    Comment
    /// let b: Vec<&str> = vec![];
    ///•let•b:•Vec<&str>•=•vec![];    Comment
    /// if !b.is_empty() {
    ///•if•!b.is_empty()•{    Comment
    ///     panic!(q: {:?}", b);
    ///•••••panic!(q:•{:?}",•b);    Comment
    /// }
    ///•}    Comment
    /// ```
    ///•```    Comment
    /// Use instead:
    ///•Use•instead:    Comment
    /// ```rust
    ///•```rust    Comment
    /// let b: Vec<&str> = vec![];
    ///•let•b:•Vec<&str>•=•vec![];    Comment
    /// assert!(b.is_empty(), "there are sad people: {:?}", b);
    ///•assert!(b.is_empty(),•"there•are•sad•people:•{:?}",•b);    Comment
    /// ```
    ///•```    Comment
    #[clippy::version = "1.57.0"]                                                                                                         /*
    #                                PunctuationToken
     [clippy::version•=•"1.57.0"]    DelimGroup
            ::                       PunctuationToken
                      =              PunctuationToken
                        "1.57.0"     Literal                                                                                              */
    pub MANUAL_ASSERT,                                                                                                                    /*
                     ,    PunctuationToken                                                                                                */
    pedantic,                                                                                                                             /*
            ,    PunctuationToken                                                                                                         */
    "`panic!` and only a `panic!` in `if`-then statement"                                                                                 /*
    "`panic!`•and•only•a•`panic!`•in•`if`-then•statement"    Literal                                                                      */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */


fn aux<Xs: HList, Ys: HList>(xs: Xs, ys: Ys) -> Expr!(Xs + Ys) where                                                                      /*
fn•aux<Xs:•HList,•Ys:•HList>(xs:•Xs,•ys:•Ys)•->•Expr!(Xs•+•Ys)•where↲    <FunctionDeclaration>
       Xs:•HList                                                         GenericTypeParameterDeclaration
           HList                                                         TypeTraitBound
                  Ys:•HList                                              GenericTypeParameterDeclaration
                      HList                                              TypeTraitBound
                             xs:•Xs                                      FunctionParameterDeclaration
                                     ys:•Ys                              FunctionParameterDeclaration
                                                Expr!(Xs•+•Ys)           MacroInvocation
                                                         +               PunctuationToken                                                 */
Xs: Add<Ys> {                                                                                                                             /*
Xs:•Add<Ys>      WhereTypeBoundDeclaration
    Add<Ys>      TypeTraitBound, TypeCall                                                                                                 */
    xs + ys                                                                                                                               /*
    xs•+•ys    ExpressionStatement, OperationExpression                                                                                   */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

declare_lint_pass!(Attributes => [                                                                                                        /*
declare_lint_pass!(Attributes•=>•[↲    <ExpressionStatement>, <MacroInvocation>
                              =>       PunctuationToken
                                 [↲    <DelimGroup>                                                                                       */
    INLINE_ALWAYS,                                                                                                                        /*
                 ,    PunctuationToken                                                                                                    */
    DEPRECATED_SEMVER,                                                                                                                    /*
                     ,    PunctuationToken                                                                                                */
    USELESS_ATTRIBUTE,                                                                                                                    /*
                     ,    PunctuationToken                                                                                                */
    BLANKET_CLIPPY_RESTRICTION_LINTS,                                                                                                     /*
                                    ,    PunctuationToken                                                                                 */
]);                                                                                                                                       /*
]);    </ExpressionStatement>
])     </MacroInvocation>
]      </DelimGroup>                                                                                                                      */

fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>                                                                                                     */
    let xs: HList!(&str, bool, Vec<u64>) = hlist!("foo", false, vec![]);                                                                  /*
    let•xs:•HList!(&str,•bool,•Vec<u64>)•=•hlist!("foo",•false,•vec![]);    LetVariableDeclaration
            HList!(&str,•bool,•Vec<u64>)                                    MacroInvocation
                   &                                                        PunctuationToken
                       ,                                                    PunctuationToken
                             ,                                              PunctuationToken
                                  <                                         PunctuationToken
                                      >                                     PunctuationToken
                                           hlist!("foo",•false,•vec![])     MacroInvocation
                                                  "foo"                     Literal
                                                       ,                    PunctuationToken
                                                         false              Literal
                                                              ,             PunctuationToken
                                                                   !        PunctuationToken
                                                                    []      DelimGroup                                                    */
    let ys: HList!(u64, [u8; 3], ()) = hlist!(0, [0, 1, 2], ());                                                                          /*
    let•ys:•HList!(u64,•[u8;•3],•())•=•hlist!(0,•[0,•1,•2],•());    LetVariableDeclaration
            HList!(u64,•[u8;•3],•())                                MacroInvocation
                      ,                                             PunctuationToken
                        [u8;•3]                                     DelimGroup
                           ;                                        PunctuationToken
                             3                                      Literal
                               ,                                    PunctuationToken
                                 ()                                 DelimGroup
                                       hlist!(0,•[0,•1,•2],•())     MacroInvocation
                                              0                     Literal
                                               ,                    PunctuationToken
                                                 [0,•1,•2]          DelimGroup
                                                  0                 Literal
                                                   ,                PunctuationToken
                                                     1              Literal
                                                      ,             PunctuationToken
                                                        2           Literal
                                                          ,         PunctuationToken
                                                            ()      DelimGroup                                                            */
    let zs: Expr!((HList![&str] + HList![bool] + HList![Vec<u64>]) + (HList![u64] + HList![[u8; 3], ()]) + HList![]) = aux(xs, ys);       /*
    let•zs:•Expr!((HList![&str]•+•HList![bool]•+•HList![Vec<u64>])•+•(HList![u64]•+•HList![[u8;•3],•()])•+•HList![])•=•aux(xs,•ys);    LetVariableDeclaration
            Expr!((HList![&str]•+•HList![bool]•+•HList![Vec<u64>])•+•(HList![u64]•+•HList![[u8;•3],•()])•+•HList![])                   MacroInvocation
                  (HList![&str]•+•HList![bool]•+•HList![Vec<u64>])                                                                     DelimGroup
                        !                                                                                                              PunctuationToken
                         [&str]                                                                                                        DelimGroup
                          &                                                                                                            PunctuationToken
                                +                                                                                                      PunctuationToken
                                       !                                                                                               PunctuationToken
                                        [bool]                                                                                         DelimGroup
                                               +                                                                                       PunctuationToken
                                                      !                                                                                PunctuationToken
                                                       [Vec<u64>]                                                                      DelimGroup
                                                           <                                                                           PunctuationToken
                                                               >                                                                       PunctuationToken
                                                                   +                                                                   PunctuationToken
                                                                     (HList![u64]•+•HList![[u8;•3],•()])                               DelimGroup
                                                                           !                                                           PunctuationToken
                                                                            [u64]                                                      DelimGroup
                                                                                  +                                                    PunctuationToken
                                                                                         !                                             PunctuationToken
                                                                                          [[u8;•3],•()]                                DelimGroup
                                                                                           [u8;•3]                                     DelimGroup
                                                                                              ;                                        PunctuationToken
                                                                                                3                                      Literal
                                                                                                  ,                                    PunctuationToken
                                                                                                    ()                                 DelimGroup
                                                                                                         +                             PunctuationToken
                                                                                                                !                      PunctuationToken
                                                                                                                 []                    DelimGroup
                                                                                                                       aux(xs,•ys)     CallExpression*/
    higher_order!(subst ($x:expr, $y:expr, $foo:expr) => (($x + $y, $foo)));                                                              /*
    higher_order!(subst•($x:expr,•$y:expr,•$foo:expr)•=>•(($x•+•$y,•$foo)));    ExpressionStatement
    higher_order!(subst•($x:expr,•$y:expr,•$foo:expr)•=>•(($x•+•$y,•$foo)))     MacroInvocation
                        ($x:expr,•$y:expr,•$foo:expr)                           DelimGroup
                         $                                                      PunctuationToken
                           :                                                    PunctuationToken
                                ,                                               PunctuationToken
                                  $                                             PunctuationToken
                                    :                                           PunctuationToken
                                         ,                                      PunctuationToken
                                           $                                    PunctuationToken
                                               :                                PunctuationToken
                                                      =>                        PunctuationToken
                                                         (($x•+•$y,•$foo))      DelimGroup
                                                          ($x•+•$y,•$foo)       DelimGroup
                                                           $                    PunctuationToken
                                                              +                 PunctuationToken
                                                                $               PunctuationToken
                                                                  ,             PunctuationToken
                                                                    $           PunctuationToken                                          */
    assert_eq!(zs, hlist!["foo", false, vec![], 0, [0, 1, 2], ()]);                                                                       /*
    assert_eq!(zs,•hlist!["foo",•false,•vec![],•0,•[0,•1,•2],•()]);    ExpressionStatement
    assert_eq!(zs,•hlist!["foo",•false,•vec![],•0,•[0,•1,•2],•()])     MacroInvocation
                 ,                                                     PunctuationToken
                        !                                              PunctuationToken
                         ["foo",•false,•vec![],•0,•[0,•1,•2],•()]      DelimGroup
                          "foo"                                        Literal
                               ,                                       PunctuationToken
                                 false                                 Literal
                                      ,                                PunctuationToken
                                           !                           PunctuationToken
                                            []                         DelimGroup
                                              ,                        PunctuationToken
                                                0                      Literal
                                                 ,                     PunctuationToken
                                                   [0,•1,•2]           DelimGroup
                                                    0                  Literal
                                                     ,                 PunctuationToken
                                                       1               Literal
                                                        ,              PunctuationToken
                                                          2            Literal
                                                            ,          PunctuationToken
                                                              ()       DelimGroup                                                         */
	quote!(fn $name() -> bool { true });                                                                                                  /*
    quote!(fn•$name()•->•bool•{•true•});    ExpressionStatement
    quote!(fn•$name()•->•bool•{•true•})     MacroInvocation
              $                             PunctuationToken
                   ()                       DelimGroup
                      ->                    PunctuationToken
                              {•true•}      DelimGroup
                                true        Literal                                                                                       */
    impl_lint_pass!(Arithmetic => [INTEGER_ARITHMETIC, FLOAT_ARITHMETIC]);                                                                /*
    impl_lint_pass!(Arithmetic•=>•[INTEGER_ARITHMETIC,•FLOAT_ARITHMETIC]);    ExpressionStatement
    impl_lint_pass!(Arithmetic•=>•[INTEGER_ARITHMETIC,•FLOAT_ARITHMETIC])     MacroInvocation
                               =>                                             PunctuationToken
                                  [INTEGER_ARITHMETIC,•FLOAT_ARITHMETIC]      DelimGroup
                                                     ,                        PunctuationToken                                            */
	let result = quote! {                                                                                                                 /*
    let•result•=•quote!•{↲    <LetVariableDeclaration>
                 quote!•{↲    <MacroInvocation>                                                                                           */
        macro_rules! generated_foo {                                                                                                      /*
                   !                     PunctuationToken
                                   {↲    <DelimGroup>                                                                                     */
            (1 $$x:expr $$($$y:tt,)* $$(= $$z:tt)*) => {};                                                                                /*
            (1•$$x:expr•$$($$y:tt,)*•$$(=•$$z:tt)*)           DelimGroup
             1                                                Literal
               $                                              PunctuationToken
                $                                             PunctuationToken
                  :                                           PunctuationToken
                        $                                     PunctuationToken
                         $                                    PunctuationToken
                          ($$y:tt,)                           DelimGroup
                           $                                  PunctuationToken
                            $                                 PunctuationToken
                              :                               PunctuationToken
                                 ,                            PunctuationToken
                                   *                          PunctuationToken
                                     $                        PunctuationToken
                                      $                       PunctuationToken
                                       (=•$$z:tt)             DelimGroup
                                        =                     PunctuationToken
                                          $                   PunctuationToken
                                           $                  PunctuationToken
                                             :                PunctuationToken
                                                 *            PunctuationToken
                                                    =>        PunctuationToken
                                                       {}     DelimGroup
                                                         ;    PunctuationToken                                                            */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    };                                                                                                                                    /*
••••};    </LetVariableDeclaration>
••••}     </MacroInvocation>                                                                                                              */
    if !matches!(                                                                                                                         /*
    if•!matches!(↲    <ExpressionStatement>, <IfBlockExpression>
       !matches!(↲    <NotExpression>
        matches!(↲    <MacroInvocation>                                                                                                   */
        macro_name.as_str(),                                                                                                              /*
                  .             PunctuationToken
                         ()     DelimGroup
                           ,    PunctuationToken                                                                                          */
        "assert_eq" | "debug_assert_eq" | "assert_ne" | "debug_assert_ne"                                                                 /*
        "assert_eq"                                                          Literal
                    |                                                        PunctuationToken
                      "debug_assert_eq"                                      Literal
                                        |                                    PunctuationToken
                                          "assert_ne"                        Literal
                                                      |                      PunctuationToken
                                                        "debug_assert_ne"    Literal                                                      */
    ) {}                                                                                                                                  /*
••••)•{}    </ExpressionStatement>, </IfBlockExpression>
••••)       </NotExpression>, </MacroInvocation>                                                                                          */
    if matches!(cx.tcx.type_of(id).kind(), ty::Adt(adt, _) if ty_adt.did() == adt.did()) {}                                               /*
    if•matches!(cx.tcx.type_of(id).kind(),•ty::Adt(adt,•_)•if•ty_adt.did()•==•adt.did())•{}    ExpressionStatement, IfBlockExpression
       matches!(cx.tcx.type_of(id).kind(),•ty::Adt(adt,•_)•if•ty_adt.did()•==•adt.did())       MacroInvocation
                  .                                                                            PunctuationToken
                      .                                                                        PunctuationToken
                              (id)                                                             DelimGroup
                                  .                                                            PunctuationToken
                                       ()                                                      DelimGroup
                                         ,                                                     PunctuationToken
                                             ::                                                PunctuationToken
                                                  (adt,•_)                                     DelimGroup
                                                      ,                                        PunctuationToken
                                                        _                                      PunctuationToken
                                                                    .                          PunctuationToken
                                                                        ()                     DelimGroup
                                                                           ==                  PunctuationToken
                                                                                 .             PunctuationToken
                                                                                     ()        DelimGroup                                 */
    if !matches!(                                                                                                                         /*
    if•!matches!(↲    <ExpressionStatement>, <IfBlockExpression>
       !matches!(↲    <NotExpression>
        matches!(↲    <MacroInvocation>                                                                                                   */
        get_expr_use_or_unification_node(tcx, expr),                                                                                      /*
                                        (tcx,•expr)     DelimGroup
                                            ,           PunctuationToken
                                                   ,    PunctuationToken                                                                  */
        None | Some((                                                                                                                     /*
             |            PunctuationToken
                   ((↲    <DelimGroup>
                    (↲    <DelimGroup>                                                                                                    */
            Node::Stmt(Stmt {                                                                                                             /*
                ::                PunctuationToken
                      (Stmt•{↲    <DelimGroup>
                            {↲    <DelimGroup>                                                                                            */
                kind: StmtKind::Expr(_)                                                                                                   /*
                    :                      PunctuationToken
                              ::           PunctuationToken
                                    (_)    DelimGroup
                                     _     PunctuationToken                                                                               */
                    | StmtKind::Semi(_)                                                                                                   /*
                    |                      PunctuationToken
                              ::           PunctuationToken
                                    (_)    DelimGroup
                                     _     PunctuationToken                                                                               */
                    | StmtKind::Local(Local {                                                                                             /*
                    |                             PunctuationToken
                              ::                  PunctuationToken
                                     (Local•{↲    <DelimGroup>
                                            {↲    <DelimGroup>                                                                            */
                        pat: Pat {                                                                                                        /*
                           :           PunctuationToken
                                 {↲    <DelimGroup>                                                                                       */
                            kind: PatKind::Wild,                                                                                          /*
                                :                   PunctuationToken
                                         ::         PunctuationToken
                                               ,    PunctuationToken                                                                      */
                            ..                                                                                                            /*
                            ..    PunctuationToken                                                                                        */
                        },                                                                                                                /*
••••••••••••••••••••••••}     </DelimGroup>
                         ,    PunctuationToken                                                                                            */
                        ..                                                                                                                /*
                        ..    PunctuationToken                                                                                            */
                    }),                                                                                                                   /*
••••••••••••••••••••})     </DelimGroup>
••••••••••••••••••••}      </DelimGroup>
                      ,    PunctuationToken                                                                                               */
                ..                                                                                                                        /*
                ..    PunctuationToken                                                                                                    */
            }),                                                                                                                           /*
••••••••••••})     </DelimGroup>
••••••••••••}      </DelimGroup>
              ,    PunctuationToken                                                                                                       */
            _                                                                                                                             /*
            _    PunctuationToken                                                                                                         */
        ))                                                                                                                                /*
••••••••))    </DelimGroup>
••••••••)     </DelimGroup>                                                                                                               */
    ) {}                                                                                                                                  /*
••••)•{}    </ExpressionStatement>, </IfBlockExpression>
••••)       </NotExpression>, </MacroInvocation>                                                                                          */
    if_chain! {                                                                                                                           /*
    if_chain!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                              */
        if let ExprKind::If(cond, then, None) = expr.kind;                                                                                /*
                       ::                                     PunctuationToken
                           (cond,•then,•None)                 DelimGroup
                                ,                             PunctuationToken
                                      ,                       PunctuationToken
                                              =               PunctuationToken
                                                    .         PunctuationToken
                                                         ;    PunctuationToken                                                            */
        if !matches!(cond.kind, ExprKind::Let(_));                                                                                        /*
           !                                          PunctuationToken
                   !                                  PunctuationToken
                    (cond.kind,•ExprKind::Let(_))     DelimGroup
                         .                            PunctuationToken
                              ,                       PunctuationToken
                                        ::            PunctuationToken
                                             (_)      DelimGroup
                                              _       PunctuationToken
                                                 ;    PunctuationToken                                                                    */
        if !expr.span.from_expansion();                                                                                                   /*
           !                               PunctuationToken
                .                          PunctuationToken
                     .                     PunctuationToken
                                    ()     DelimGroup
                                      ;    PunctuationToken                                                                               */
        let then = peel_blocks_with_stmt(then);                                                                                           /*
                 =                                 PunctuationToken
                                        (then)     DelimGroup
                                              ;    PunctuationToken                                                                       */
        if let Some(macro_call) = root_macro_call(then.span);                                                                             /*
                   (macro_call)                                  DelimGroup
                                =                                PunctuationToken
                                                 (then.span)     DelimGroup
                                                      .          PunctuationToken
                                                            ;    PunctuationToken                                                         */
        if cx.tcx.item_name(macro_call.def_id) == sym::panic;                                                                             /*
             .                                                   PunctuationToken
                 .                                               PunctuationToken
                           (macro_call.def_id)                   DelimGroup
                                      .                          PunctuationToken
                                               ==                PunctuationToken
                                                     ::          PunctuationToken
                                                            ;    PunctuationToken                                                         */
        if !cx.tcx.sess.source_map().is_multiline(cond.span);                                                                             /*
           !                                                     PunctuationToken
              .                                                  PunctuationToken
                  .                                              PunctuationToken
                       .                                         PunctuationToken
                                  ()                             DelimGroup
                                    .                            PunctuationToken
                                                 (cond.span)     DelimGroup
                                                      .          PunctuationToken
                                                            ;    PunctuationToken                                                         */
        if let Some(format_args) = FormatArgsExpn::find_nested(cx, then, macro_call.expn);                                                /*
                   (format_args)                                                              DelimGroup
                                 =                                                            PunctuationToken
                                                 ::                                           PunctuationToken
                                                              (cx,•then,•macro_call.expn)     DelimGroup
                                                                 ,                            PunctuationToken
                                                                       ,                      PunctuationToken
                                                                                   .          PunctuationToken
                                                                                         ;    PunctuationToken                            */
        then {                                                                                                                            /*
             {↲    <DelimGroup>                                                                                                           */
            let mut applicability = Applicability::MachineApplicable;                                                                     /*
                                  =                                      PunctuationToken
                                                 ::                      PunctuationToken
                                                                    ;    PunctuationToken                                                 */
            let format_args_snip = snippet_with_applicability(cx, format_args.inputs_span(), "..", &mut applicability);                   /*
                                 =                                                                                         PunctuationToken
                                                             (cx,•format_args.inputs_span(),•"..",•&mut•applicability)     DelimGroup
                                                                ,                                                          PunctuationToken
                                                                             .                                             PunctuationToken
                                                                                         ()                                DelimGroup
                                                                                           ,                               PunctuationToken
                                                                                             ".."                          Literal
                                                                                                 ,                         PunctuationToken
                                                                                                   &                       PunctuationToken
                                                                                                                      ;    PunctuationToken*/
            let cond = cond.peel_drop_temps();                                                                                            /*
                     =                            PunctuationToken
                           .                      PunctuationToken
                                           ()     DelimGroup
                                             ;    PunctuationToken                                                                        */
            let (cond, not) = match cond.kind {                                                                                           /*
                (cond,•not)                         DelimGroup
                     ,                              PunctuationToken
                            =                       PunctuationToken
                                        .           PunctuationToken
                                              {↲    <DelimGroup>                                                                          */
                ExprKind::Unary(UnOp::Not, e) => (e, ""),                                                                                 /*
                        ::                                   PunctuationToken
                               (UnOp::Not,•e)                DelimGroup
                                    ::                       PunctuationToken
                                         ,                   PunctuationToken
                                              =>             PunctuationToken
                                                 (e,•"")     DelimGroup
                                                   ,         PunctuationToken
                                                     ""      Literal
                                                        ,    PunctuationToken                                                             */
                _ => (cond, "!"),                                                                                                         /*
                _                    PunctuationToken
                  =>                 PunctuationToken
                     (cond,•"!")     DelimGroup
                          ,          PunctuationToken
                            "!"      Literal
                                ,    PunctuationToken                                                                                     */
            };                                                                                                                            /*
••••••••••••}     </DelimGroup>
             ;    PunctuationToken                                                                                                        */
            let cond_sugg = sugg::Sugg::hir_with_applicability(cx, cond, "..", &mut applicability).maybe_par();                           /*
                          =                                                                                        PunctuationToken
                                ::                                                                                 PunctuationToken
                                      ::                                                                           PunctuationToken
                                                              (cx,•cond,•"..",•&mut•applicability)                 DelimGroup
                                                                 ,                                                 PunctuationToken
                                                                       ,                                           PunctuationToken
                                                                         ".."                                      Literal
                                                                             ,                                     PunctuationToken
                                                                               &                                   PunctuationToken
                                                                                                  .                PunctuationToken
                                                                                                            ()     DelimGroup
                                                                                                              ;    PunctuationToken       */
            let sugg = format!("assert!({not}{cond_sugg}, {format_args_snip});");                                                         /*
                     =                                                               PunctuationToken
                             !                                                       PunctuationToken
                              ("assert!({not}{cond_sugg},•{format_args_snip});")     DelimGroup
                               "assert!({not}{cond_sugg},•{format_args_snip});"      Literal
                                                                                ;    PunctuationToken                                     */
            span_lint_and_sugg(                                                                                                           /*
                              (↲    <DelimGroup>                                                                                          */
                cx,                                                                                                                       /*
                  ,    PunctuationToken                                                                                                   */
                MANUAL_ASSERT,                                                                                                            /*
                             ,    PunctuationToken                                                                                        */
                expr.span,                                                                                                                /*
                    .         PunctuationToken
                         ,    PunctuationToken                                                                                            */
                "only a `panic!` in `if`-then statement",                                                                                 /*
                "only•a•`panic!`•in•`if`-then•statement"     Literal
                                                        ,    PunctuationToken                                                             */
                "try",                                                                                                                    /*
                "try"     Literal
                     ,    PunctuationToken                                                                                                */
                sugg,                                                                                                                     /*
                    ,    PunctuationToken                                                                                                 */
                Applicability::MachineApplicable,                                                                                         /*
                             ::                      PunctuationToken
                                                ,    PunctuationToken                                                                     */
            );                                                                                                                            /*
••••••••••••)     </DelimGroup>
             ;    PunctuationToken                                                                                                        */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MacroInvocation>                                                                                       */
    if_chain! {                                                                                                                           /*
    if_chain!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                              */
        if meets_msrv(msrv.as_ref(), &msrvs::TOOL_ATTRIBUTES);                                                                            /*
                     (msrv.as_ref(),•&msrvs::TOOL_ATTRIBUTES)     DelimGroup
                          .                                       PunctuationToken
                                 ()                               DelimGroup
                                   ,                              PunctuationToken
                                     &                            PunctuationToken
                                           ::                     PunctuationToken
                                                             ;    PunctuationToken                                                        */
        // check cfg_attr
        //•check•cfg_attr    Comment
        if attr.has_name(sym::cfg_attr);                                                                                                  /*
               .                            PunctuationToken
                        (sym::cfg_attr)     DelimGroup
                            ::              PunctuationToken
                                       ;    PunctuationToken                                                                              */
        if let Some(items) = attr.meta_item_list();                                                                                       /*
                   (items)                             DelimGroup
                           =                           PunctuationToken
                                 .                     PunctuationToken
                                                ()     DelimGroup
                                                  ;    PunctuationToken                                                                   */
        if items.len() == 2;                                                                                                              /*
                .               PunctuationToken
                    ()          DelimGroup
                       ==       PunctuationToken
                          2     Literal
                           ;    PunctuationToken                                                                                          */
        // check for `rustfmt`
        //•check•for•`rustfmt`    Comment
        if let Some(feature_item) = items[0].meta_item();                                                                                 /*
                   (feature_item)                            DelimGroup
                                  =                          PunctuationToken
                                         [0]                 DelimGroup
                                          0                  Literal
                                            .                PunctuationToken
                                                      ()     DelimGroup
                                                        ;    PunctuationToken                                                             */
        if feature_item.has_name(sym::rustfmt);                                                                                           /*
                       .                           PunctuationToken
                                (sym::rustfmt)     DelimGroup
                                    ::             PunctuationToken
                                              ;    PunctuationToken                                                                       */
        // check for `rustfmt_skip` and `rustfmt::skip`
        //•check•for•`rustfmt_skip`•and•`rustfmt::skip`    Comment
        if let Some(skip_item) = &items[1].meta_item();                                                                                   /*
                   (skip_item)                             DelimGroup
                               =                           PunctuationToken
                                 &                         PunctuationToken
                                       [1]                 DelimGroup
                                        1                  Literal
                                          .                PunctuationToken
                                                    ()     DelimGroup
                                                      ;    PunctuationToken                                                               */
        if skip_item.has_name(sym!(rustfmt_skip)) ||                                                                                      /*
                    .                                   PunctuationToken
                             (sym!(rustfmt_skip))       DelimGroup
                                 !                      PunctuationToken
                                  (rustfmt_skip)        DelimGroup
                                                  ||    PunctuationToken                                                                  */
            skip_item.path.segments.last().expect("empty path in attribute").ident.name == sym::skip;                                     /*
                     .                                                                                   PunctuationToken
                          .                                                                              PunctuationToken
                                   .                                                                     PunctuationToken
                                        ()                                                               DelimGroup
                                          .                                                              PunctuationToken
                                                 ("empty•path•in•attribute")                             DelimGroup
                                                  "empty•path•in•attribute"                              Literal
                                                                            .                            PunctuationToken
                                                                                  .                      PunctuationToken
                                                                                        ==               PunctuationToken
                                                                                              ::         PunctuationToken
                                                                                                    ;    PunctuationToken                 */
        // Only lint outer attributes, because custom inner attributes are unstable
        //•Only•lint•outer•attributes,•because•custom•inner•attributes•are•unstable    Comment
        // Tracking issue: https://github.com/rust-lang/rust/issues/54726
        //•Tracking•issue:•https://github.com/rust-lang/rust/issues/54726    Comment
        if attr.style == AttrStyle::Outer;                                                                                                /*
               .                              PunctuationToken
                      ==                      PunctuationToken
                                  ::          PunctuationToken
                                         ;    PunctuationToken                                                                            */
        then {                                                                                                                            /*
             {↲    <DelimGroup>                                                                                                           */
            span_lint_and_sugg(                                                                                                           /*
                              (↲    <DelimGroup>                                                                                          */
                cx,                                                                                                                       /*
                  ,    PunctuationToken                                                                                                   */
                DEPRECATED_CFG_ATTR,                                                                                                      /*
                                   ,    PunctuationToken                                                                                  */
                attr.span,                                                                                                                /*
                    .         PunctuationToken
                         ,    PunctuationToken                                                                                            */
                "`cfg_attr` is deprecated for rustfmt and got replaced by tool attributes",                                               /*
                "`cfg_attr`•is•deprecated•for•rustfmt•and•got•replaced•by•tool•attributes"     Literal
                                                                                          ,    PunctuationToken                           */
                "use",                                                                                                                    /*
                "use"     Literal
                     ,    PunctuationToken                                                                                                */
                "#[rustfmt::skip]".to_string(),                                                                                           /*
                "#[rustfmt::skip]"                 Literal
                                  .                PunctuationToken
                                            ()     DelimGroup
                                              ,    PunctuationToken                                                                       */
                Applicability::MachineApplicable,                                                                                         /*
                             ::                      PunctuationToken
                                                ,    PunctuationToken                                                                     */
            );                                                                                                                            /*
••••••••••••)     </DelimGroup>
             ;    PunctuationToken                                                                                                        */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MacroInvocation>                                                                                       */
    if_chain! {                                                                                                                           /*
    if_chain!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                              */
        if let ExprKind::Binary(ref op, left, right) = &expr.kind;                                                                        /*
                       ::                                             PunctuationToken
                               (ref•op,•left,•right)                  DelimGroup
                                      ,                               PunctuationToken
                                            ,                         PunctuationToken
                                                     =                PunctuationToken
                                                       &              PunctuationToken
                                                            .         PunctuationToken
                                                                 ;    PunctuationToken                                                    */
        if let Some((candidate, check)) = normalize_le_ge(op, left, right);                                                               /*
                   ((candidate,•check))                                        DelimGroup
                    (candidate,•check)                                         DelimGroup
                              ,                                                PunctuationToken
                                        =                                      PunctuationToken
                                                         (op,•left,•right)     DelimGroup
                                                            ,                  PunctuationToken
                                                                  ,            PunctuationToken
                                                                          ;    PunctuationToken                                           */
        if let Some((from, to)) = get_types_from_cast(check, INTS, "max_value", "MAX");                                                   /*
                   ((from,•to))                                                            DelimGroup
                    (from,•to)                                                             DelimGroup
                         ,                                                                 PunctuationToken
                                =                                                          PunctuationToken
                                                     (check,•INTS,•"max_value",•"MAX")     DelimGroup
                                                           ,                               PunctuationToken
                                                                 ,                         PunctuationToken
                                                                   "max_value"             Literal
                                                                              ,            PunctuationToken
                                                                                "MAX"      Literal
                                                                                      ;    PunctuationToken                               */

        then {                                                                                                                            /*
             {↲    <DelimGroup>                                                                                                           */
            Conversion::try_new(candidate, from, to)                                                                                      /*
                      ::                                PunctuationToken
                               (candidate,•from,•to)    DelimGroup
                                         ,              PunctuationToken
                                               ,        PunctuationToken                                                                  */
        } else {                                                                                                                          /*
••••••••}            </DelimGroup>
               {↲    <DelimGroup>                                                                                                         */
            None
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MacroInvocation>                                                                                       */
    let help = format!(                                                                                                                   /*
    let•help•=•format!(↲    <LetVariableDeclaration>
               format!(↲    <MacroInvocation>                                                                                             */
        "because `{}` is the {} value for this type, {}",                                                                                 /*
        "because•`{}`•is•the•{}•value•for•this•type,•{}"     Literal
                                                        ,    PunctuationToken                                                             */
        snippet(cx, culprit.expr.span, "x"),                                                                                              /*
               (cx,•culprit.expr.span,•"x")     DelimGroup
                  ,                             PunctuationToken
                           .                    PunctuationToken
                                .               PunctuationToken
                                     ,          PunctuationToken
                                       "x"      Literal
                                           ,    PunctuationToken                                                                          */
        match culprit.which {                                                                                                             /*
                     .            PunctuationToken
                            {↲    <DelimGroup>                                                                                            */
            ExtremeType::Minimum => "minimum",                                                                                            /*
                       ::                         PunctuationToken
                                 =>               PunctuationToken
                                    "minimum"     Literal
                                             ,    PunctuationToken                                                                        */
            ExtremeType::Maximum => "maximum",                                                                                            /*
                       ::                         PunctuationToken
                                 =>               PunctuationToken
                                    "maximum"     Literal
                                             ,    PunctuationToken                                                                        */
        },                                                                                                                                /*
••••••••}     </DelimGroup>
         ,    PunctuationToken                                                                                                            */
        conclusion
    );                                                                                                                                    /*
••••);    </LetVariableDeclaration>
••••)     </MacroInvocation>                                                                                                              */

    let msg = format!(                                                                                                                    /*
    let•msg•=•format!(↲    <LetVariableDeclaration>
              format!(↲    <MacroInvocation>                                                                                              */
        "this `{}` can be collapsed into the outer `{}`",                                                                                 /*
        "this•`{}`•can•be•collapsed•into•the•outer•`{}`"     Literal
                                                        ,    PunctuationToken                                                             */
        if matches!(inner, IfLetOrMatch::Match(..)) { "match" } else { "if let" },                                                        /*
                  !                                                                   PunctuationToken
                   (inner,•IfLetOrMatch::Match(..))                                   DelimGroup
                         ,                                                            PunctuationToken
                                       ::                                             PunctuationToken
                                              (..)                                    DelimGroup
                                               ..                                     PunctuationToken
                                                    {•"match"•}                       DelimGroup
                                                      "match"                         Literal
                                                                     {•"if•let"•}     DelimGroup
                                                                       "if•let"       Literal
                                                                                 ,    PunctuationToken                                    */
        if outer_is_match { "match" } else { "if let" },                                                                                  /*
                          {•"match"•}                       DelimGroup
                            "match"                         Literal
                                           {•"if•let"•}     DelimGroup
                                             "if•let"       Literal
                                                       ,    PunctuationToken                                                              */
    );                                                                                                                                    /*
••••);    </LetVariableDeclaration>
••••)     </MacroInvocation>                                                                                                              */
    let mut contents = format!(                                                                                                           /*
    let•mut•contents•=•format!(↲    <LetVariableDeclaration>
        mut•contents                PatternVariableDeclaration
                       format!(↲    <MacroInvocation>                                                                                     */
        indoc! {"                                                                                                                        "/*
             !        PunctuationToken
               {"↲    <DelimGroup>
                "↲    <Literal>                                                                                                          */"
            #![warn(clippy::{})]

            fn main() {{
                // test code goes here
            }}
        "},                                                                                                                               /*
••••••••"}     </DelimGroup>
••••••••"      </Literal>
          ,    PunctuationToken                                                                                                           */
        lint_name
    );                                                                                                                                    /*
••••);    </LetVariableDeclaration>
••••)     </MacroInvocation>                                                                                                              */
    format!(                                                                                                                              /*
    format!(↲    <ExpressionStatement>, <MacroInvocation>                                                                                 */
        "store.register_{lint_pass}_pass(move || Box::new({module_name}::{camel_name}::new(msrv)));\n    ",                               /*
        "store.register_{lint_pass}_pass(move•||•Box::new({module_name}::{camel_name}::new(msrv)));\n••••"     Literal
                                                                                                          ,    PunctuationToken           */
        lint_pass = lint.pass,                                                                                                            /*
                  =               PunctuationToken
                        .         PunctuationToken
                             ,    PunctuationToken                                                                                        */
        module_name = lint.name,                                                                                                          /*
                    =               PunctuationToken
                          .         PunctuationToken
                               ,    PunctuationToken                                                                                      */
        camel_name = to_camel_case(lint.name),                                                                                            /*
                   =                              PunctuationToken
                                  (lint.name)     DelimGroup
                                       .          PunctuationToken
                                             ,    PunctuationToken                                                                        */
    );                                                                                                                                    /*
••••);    </ExpressionStatement>
••••)     </MacroInvocation>                                                                                                              */
    format!(                                                                                                                              /*
    format!(↲    <ExpressionStatement>, <MacroInvocation>                                                                                 */
        indoc! {r#"                                                                                                                     "#/*
             !          PunctuationToken
               {r#"↲    <DelimGroup>
                r#"↲    <Literal>                                                                                                      */r#"
            # {}

            [package]
            name = "{}"
            version = "0.1.0"
            publish = false

            [workspace]
        "#},                                                                                                                              /*
••••••••"#}     </DelimGroup>
••••••••"#      </Literal>
           ,    PunctuationToken                                                                                                          */
        hint, lint_name                                                                                                                   /*
            ,              PunctuationToken                                                                                               */
    );                                                                                                                                    /*
••••);    </ExpressionStatement>
••••)     </MacroInvocation>                                                                                                              */
    match iter.next() {                                                                                                                   /*
    match•iter.next()•{↲    <ExpressionStatement>, <MatchExpression>
          iter.next()       CallExpression                                                                                                */
        // #[clippy::version = "version"] pub
        //•#[clippy::version•=•"version"]•pub    Comment
        Some((TokenKind::Pound, _)) => {                                                                                                  /*
        Some((TokenKind::Pound,•_))•=>•{↲    <MatchExpressionCase>
        Some((TokenKind::Pound,•_))          TuplePattern
             (TokenKind::Pound,•_)           TuplePattern
              TokenKind::Pound               ExpressionPath
                                _            WildcardPattern
                                       {↲    <BlockExpression>                                                                            */
            match_tokens!(iter, OpenBracket Ident Colon Colon Ident Eq Literal{..} CloseBracket Ident);                                   /*
            match_tokens!(iter,•OpenBracket•Ident•Colon•Colon•Ident•Eq•Literal{..}•CloseBracket•Ident);    ExpressionStatement
            match_tokens!(iter,•OpenBracket•Ident•Colon•Colon•Ident•Eq•Literal{..}•CloseBracket•Ident)     MacroInvocation
                              ,                                                                            PunctuationToken
                                                                              {..}                         DelimGroup
                                                                               ..                          PunctuationToken               */
        },                                                                                                                                /*
••••••••}     </MatchExpressionCase>, </BlockExpression>                                                                                  */
        // pub
        //•pub    Comment
        Some((TokenKind::Ident, _)) => (),                                                                                                /*
        Some((TokenKind::Ident,•_))•=>•()     MatchExpressionCase
        Some((TokenKind::Ident,•_))           TuplePattern
             (TokenKind::Ident,•_)            TuplePattern
              TokenKind::Ident                ExpressionPath
                                _             WildcardPattern
                                       ()     TupleLiteral                                                                                */
        _ => continue,                                                                                                                    /*
        _•=>•continue     MatchExpressionCase
        _                 WildcardPattern
             continue     ContinueExpression                                                                                              */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </MatchExpression>                                                                                       */
    let (name, group, desc) = match_tokens!(                                                                                              /*
    let•(name,•group,•desc)•=•match_tokens!(↲    <LetVariableDeclaration>
        (name,•group,•desc)                      TuplePattern
                              match_tokens!(↲    <MacroInvocation>                                                                        */
        iter,                                                                                                                             /*
            ,    PunctuationToken                                                                                                         */
        // LINT_NAME
        //•LINT_NAME    Comment
        Ident(name) Comma                                                                                                                 /*
             (name)          DelimGroup                                                                                                   */
        // group,
        //•group,    Comment
        Ident(group) Comma                                                                                                                /*
             (group)          DelimGroup                                                                                                  */
        // "description" }
        //•"description"•}    Comment
        Literal{..}(desc) CloseBrace                                                                                                      /*
               {..}                     DelimGroup
                ..                      PunctuationToken
                   (desc)               DelimGroup                                                                                        */
        // #[clippy::version = "version"]
        //•#[clippy::version•=•"version"]    Comment
        Pound OpenBracket Ident Colon Colon Ident Eq Literal{..} CloseBracket                                                             /*
                                                            {..}                 DelimGroup
                                                             ..                  PunctuationToken                                         */
        // pub LINT_NAME,
        //•pub•LINT_NAME,    Comment
        Ident Ident(name) Comma                                                                                                           /*
                   (name)          DelimGroup                                                                                             */
        // "description"
        //•"description"    Comment
        Literal{kind: LiteralKind::Str{..},..}(reason)                                                                                    /*
               {kind:•LiteralKind::Str{..},..}            DelimGroup
                    :                                     PunctuationToken
                                 ::                       PunctuationToken
                                      {..}                DelimGroup
                                       ..                 PunctuationToken
                                          ,               PunctuationToken
                                           ..             PunctuationToken
                                              (reason)    DelimGroup                                                                      */
        // ("old_name",
        //•("old_name",    Comment
        Whitespace OpenParen Literal{kind: LiteralKind::Str{..},..}(old_name) Comma                                                       /*
                                    {kind:•LiteralKind::Str{..},..}                    DelimGroup
                                         :                                             PunctuationToken
                                                      ::                               PunctuationToken
                                                           {..}                        DelimGroup
                                                            ..                         PunctuationToken
                                                               ,                       PunctuationToken
                                                                ..                     PunctuationToken
                                                                   (old_name)          DelimGroup                                         */
        // "new_name"),
        //•"new_name"),    Comment
        Whitespace Literal{kind: LiteralKind::Str{..},..}(new_name) CloseParen Comma                                                      /*
                          {kind:•LiteralKind::Str{..},..}                               DelimGroup
                               :                                                        PunctuationToken
                                            ::                                          PunctuationToken
                                                 {..}                                   DelimGroup
                                                  ..                                    PunctuationToken
                                                     ,                                  PunctuationToken
                                                      ..                                PunctuationToken
                                                         (new_name)                     DelimGroup                                        */
    );                                                                                                                                    /*
••••);    </LetVariableDeclaration>
••••)     </MacroInvocation>                                                                                                              */

    info!(//debug
                                                                                                                                          /*
    info!(//debug↲    <ExpressionStatement>, <MacroInvocation>
          //debug     Comment                                                                                                             */
        "{}: sending function_code={:04x} data={:04x} crc=0x{:04X} data={:02X?}",                                                         /*
        "{}:•sending•function_code={:04x}•data={:04x}•crc=0x{:04X}•data={:02X?}"     Literal
                                                                                ,    PunctuationToken                                     */
        self.name, function_code, data, crc, output_cmd                                                                                   /*
            .                                              PunctuationToken
                 ,                                         PunctuationToken
                                ,                          PunctuationToken
                                      ,                    PunctuationToken
                                           ,               PunctuationToken                                                               */
    );                                                                                                                                    /*
••••);    </ExpressionStatement>
••••)     </MacroInvocation>                                                                                                              */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

cfg_if! {                                                                                                                                 /*
cfg_if!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                                    */
    if #[cfg(feature = "std_detect_file_io")] {                                                                                           /*
       #                                            PunctuationToken
        [cfg(feature•=•"std_detect_file_io")]       DelimGroup
            (feature•=•"std_detect_file_io")        DelimGroup
                     =                              PunctuationToken
                       "std_detect_file_io"         Literal
                                              {↲    <DelimGroup>                                                                          */
        #[cfg_attr(test, macro_use(println))]                                                                                             /*
        #                                        PunctuationToken
         [cfg_attr(test,•macro_use(println))]    DelimGroup
                  (test,•macro_use(println))     DelimGroup
                       ,                         PunctuationToken
                                  (println)      DelimGroup                                                                               */
        extern crate std;                                                                                                                 /*
                        ;    PunctuationToken                                                                                             */

        #[allow(unused_imports)]                                                                                                          /*
        #                           PunctuationToken
         [allow(unused_imports)]    DelimGroup
               (unused_imports)     DelimGroup                                                                                            */
        use std::{arch, fs, io, mem, sync};                                                                                               /*
               ::                              PunctuationToken
                 {arch,•fs,•io,•mem,•sync}     DelimGroup
                      ,                        PunctuationToken
                          ,                    PunctuationToken
                              ,                PunctuationToken
                                   ,           PunctuationToken
                                          ;    PunctuationToken                                                                           */
    } else {                                                                                                                              /*
••••}            </DelimGroup>
           {↲    <DelimGroup>                                                                                                             */
        #[cfg(test)]                                                                                                                      /*
        #               PunctuationToken
         [cfg(test)]    DelimGroup
             (test)     DelimGroup                                                                                                        */
        #[macro_use(println)]                                                                                                             /*
        #                        PunctuationToken
         [macro_use(println)]    DelimGroup
                   (println)     DelimGroup                                                                                               */
        extern crate std;                                                                                                                 /*
                        ;    PunctuationToken                                                                                             */

        #[allow(unused_imports)]                                                                                                          /*
        #                           PunctuationToken
         [allow(unused_imports)]    DelimGroup
               (unused_imports)     DelimGroup                                                                                            */
        use core::{arch, mem, sync};                                                                                                      /*
                ::                      PunctuationToken
                  {arch,•mem,•sync}     DelimGroup
                       ,                PunctuationToken
                            ,           PunctuationToken
                                   ;    PunctuationToken                                                                                  */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

op_utils! {                                                                                                                               /*
op_utils!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                                  */
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
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

msrv_aliases! {                                                                                                                           /*
msrv_aliases!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                              */
    1,53,0 { OR_PATTERNS, MANUAL_BITS }                                                                                                   /*
    1                                      Literal
     ,                                     PunctuationToken
      53                                   Literal
        ,                                  PunctuationToken
         0                                 Literal
           {•OR_PATTERNS,•MANUAL_BITS•}    DelimGroup
                        ,                  PunctuationToken                                                                               */
    1,52,0 { STR_SPLIT_ONCE }                                                                                                             /*
    1                            Literal
     ,                           PunctuationToken
      52                         Literal
        ,                        PunctuationToken
         0                       Literal
           {•STR_SPLIT_ONCE•}    DelimGroup                                                                                               */
    1,51,0 { BORROW_AS_PTR, UNSIGNED_ABS }                                                                                                /*
    1                                         Literal
     ,                                        PunctuationToken
      51                                      Literal
        ,                                     PunctuationToken
         0                                    Literal
           {•BORROW_AS_PTR,•UNSIGNED_ABS•}    DelimGroup
                          ,                   PunctuationToken                                                                            */
    1,50,0 { BOOL_THEN }                                                                                                                  /*
    1                       Literal
     ,                      PunctuationToken
      50                    Literal
        ,                   PunctuationToken
         0                  Literal
           {•BOOL_THEN•}    DelimGroup                                                                                                    */
    1,47,0 { TAU }                                                                                                                        /*
    1                 Literal
     ,                PunctuationToken
      47              Literal
        ,             PunctuationToken
         0            Literal
           {•TAU•}    DelimGroup                                                                                                          */
    1,46,0 { CONST_IF_MATCH }                                                                                                             /*
    1                            Literal
     ,                           PunctuationToken
      46                         Literal
        ,                        PunctuationToken
         0                       Literal
           {•CONST_IF_MATCH•}    DelimGroup                                                                                               */
    1,45,0 { STR_STRIP_PREFIX }                                                                                                           /*
    1                              Literal
     ,                             PunctuationToken
      45                           Literal
        ,                          PunctuationToken
         0                         Literal
           {•STR_STRIP_PREFIX•}    DelimGroup                                                                                             */
    1,43,0 { LOG2_10, LOG10_2 }                                                                                                           /*
    1                              Literal
     ,                             PunctuationToken
      43                           Literal
        ,                          PunctuationToken
         0                         Literal
           {•LOG2_10,•LOG10_2•}    DelimGroup
                    ,              PunctuationToken                                                                                       */
    1,42,0 { MATCHES_MACRO, SLICE_PATTERNS, PTR_SLICE_RAW_PARTS }                                                                         /*
    1                                                                Literal
     ,                                                               PunctuationToken
      42                                                             Literal
        ,                                                            PunctuationToken
         0                                                           Literal
           {•MATCHES_MACRO,•SLICE_PATTERNS,•PTR_SLICE_RAW_PARTS•}    DelimGroup
                          ,                                          PunctuationToken
                                          ,                          PunctuationToken                                                     */
    1,41,0 { RE_REBALANCING_COHERENCE, RESULT_MAP_OR_ELSE }                                                                               /*
    1                                                          Literal
     ,                                                         PunctuationToken
      41                                                       Literal
        ,                                                      PunctuationToken
         0                                                     Literal
           {•RE_REBALANCING_COHERENCE,•RESULT_MAP_OR_ELSE•}    DelimGroup
                                     ,                         PunctuationToken                                                           */
    1,40,0 { MEM_TAKE, NON_EXHAUSTIVE, OPTION_AS_DEREF }                                                                                  /*
    1                                                       Literal
     ,                                                      PunctuationToken
      40                                                    Literal
        ,                                                   PunctuationToken
         0                                                  Literal
           {•MEM_TAKE,•NON_EXHAUSTIVE,•OPTION_AS_DEREF•}    DelimGroup
                     ,                                      PunctuationToken
                                     ,                      PunctuationToken                                                              */
    1,38,0 { POINTER_CAST }                                                                                                               /*
    1                          Literal
     ,                         PunctuationToken
      38                       Literal
        ,                      PunctuationToken
         0                     Literal
           {•POINTER_CAST•}    DelimGroup                                                                                                 */
    1,37,0 { TYPE_ALIAS_ENUM_VARIANTS }                                                                                                   /*
    1                                      Literal
     ,                                     PunctuationToken
      37                                   Literal
        ,                                  PunctuationToken
         0                                 Literal
           {•TYPE_ALIAS_ENUM_VARIANTS•}    DelimGroup                                                                                     */
    1,36,0 { ITERATOR_COPIED }                                                                                                            /*
    1                             Literal
     ,                            PunctuationToken
      36                          Literal
        ,                         PunctuationToken
         0                        Literal
           {•ITERATOR_COPIED•}    DelimGroup                                                                                              */
    1,35,0 { OPTION_COPIED, RANGE_CONTAINS }                                                                                              /*
    1                                           Literal
     ,                                          PunctuationToken
      35                                        Literal
        ,                                       PunctuationToken
         0                                      Literal
           {•OPTION_COPIED,•RANGE_CONTAINS•}    DelimGroup
                          ,                     PunctuationToken                                                                          */
    1,34,0 { TRY_FROM }                                                                                                                   /*
    1                      Literal
     ,                     PunctuationToken
      34                   Literal
        ,                  PunctuationToken
         0                 Literal
           {•TRY_FROM•}    DelimGroup                                                                                                     */
    1,30,0 { ITERATOR_FIND_MAP, TOOL_ATTRIBUTES }                                                                                         /*
    1                                                Literal
     ,                                               PunctuationToken
      30                                             Literal
        ,                                            PunctuationToken
         0                                           Literal
           {•ITERATOR_FIND_MAP,•TOOL_ATTRIBUTES•}    DelimGroup
                              ,                      PunctuationToken                                                                     */
    1,28,0 { FROM_BOOL }                                                                                                                  /*
    1                       Literal
     ,                      PunctuationToken
      28                    Literal
        ,                   PunctuationToken
         0                  Literal
           {•FROM_BOOL•}    DelimGroup                                                                                                    */
    1,17,0 { FIELD_INIT_SHORTHAND, STATIC_IN_CONST, EXPECT_ERR }                                                                          /*
    1                                                               Literal
     ,                                                              PunctuationToken
      17                                                            Literal
        ,                                                           PunctuationToken
         0                                                          Literal
           {•FIELD_INIT_SHORTHAND,•STATIC_IN_CONST,•EXPECT_ERR•}    DelimGroup
                                 ,                                  PunctuationToken
                                                  ,                 PunctuationToken                                                      */
    1,16,0 { STR_REPEAT }                                                                                                                 /*
    1                        Literal
     ,                       PunctuationToken
      16                     Literal
        ,                    PunctuationToken
         0                   Literal
           {•STR_REPEAT•}    DelimGroup                                                                                                   */
    1,24,0 { IS_ASCII_DIGIT }                                                                                                             /*
    1                            Literal
     ,                           PunctuationToken
      24                         Literal
        ,                        PunctuationToken
         0                       Literal
           {•IS_ASCII_DIGIT•}    DelimGroup                                                                                               */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

kot! {                                                                                                                                    /*
kot!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                                       */
    struct W { foo : u8, bar : u16, } impl Clone for W                                                                                    /*
             {•foo•:•u8,•bar•:•u16,•}                     DelimGroup
                   :                                      PunctuationToken
                       ,                                  PunctuationToken
                             :                            PunctuationToken
                                  ,                       PunctuationToken                                                                */
    {                                                                                                                                     /*
    {↲    <DelimGroup>                                                                                                                    */
        fn clone() -> W                                                                                                                   /*
                ()         DelimGroup
                   ->      PunctuationToken                                                                                               */
        {                                                                                                                                 /*
        {↲    <DelimGroup>                                                                                                                */
            panic! () ;                                                                                                                   /*
                 !         PunctuationToken
                   ()      DelimGroup
                      ;    PunctuationToken                                                                                               */

        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

kot! {                                                                                                                                    /*
kot!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                                       */
    a(mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                             /*
     (mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <DelimGroup>                                            */
    mushkins mushkins) a                                                                                                                  /*
••••mushkins•mushkins)      </DelimGroup>                                                                                                 */
    [mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
    [mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <DelimGroup>                                             */
    mushkins mushkins] a                                                                                                                  /*
••••mushkins•mushkins]      </DelimGroup>                                                                                                 */
    {                                                                                                                                     /*
    {↲    <DelimGroup>                                                                                                                    */
        mushkins mushkins mushkins mushkins mushkins mushkins mushkins
        mushkins mushkins mushkins
    } a                                                                                                                                   /*
••••}      </DelimGroup>                                                                                                                  */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

kot!(mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
kot!(mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <ExpressionStatement>, <MacroInvocation>                 */
mushkins mushkins);                                                                                                                       /*
mushkins•mushkins);    </ExpressionStatement>
mushkins•mushkins)     </MacroInvocation>                                                                                                 */
kot![mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
kot![mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <ExpressionStatement>, <MacroInvocation>                 */
mushkins mushkins];                                                                                                                       /*
mushkins•mushkins];    </ExpressionStatement>
mushkins•mushkins]     </MacroInvocation>                                                                                                 */
kot! {                                                                                                                                    /*
kot!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                                       */
    mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins
    mushkins mushkins
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

#[rustc_dummy(mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
#[rustc_dummy(mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <ExpressionStatement>, <Attribute>
             (mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <DelimGroup>                                             */
mushkins mushkins mushkins)]                                                                                                              /*
mushkins•mushkins•mushkins)]    </Attribute>
mushkins•mushkins•mushkins)     </DelimGroup>                                                                                             */
#[rustc_dummy[mushkins mushkins mushkins mushkins mushkins mushkins mushkins                                                              /*
#[rustc_dummy[mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <Attribute>
             [mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins↲    <DelimGroup>                                             */
mushkins mushkins mushkins]]                                                                                                              /*
mushkins•mushkins•mushkins]]    </Attribute>
mushkins•mushkins•mushkins]     </DelimGroup>                                                                                             */
#[rustc_dummy {                                                                                                                           /*
#[rustc_dummy•{↲    <Attribute>
              {↲    <DelimGroup>                                                                                                          */
    mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins
    mushkins mushkins
}]                                                                                                                                        /*
}]    </Attribute>
}     </DelimGroup>                                                                                                                       */
#[rustc_dummy =                                                                                                                           /*
#[rustc_dummy•=↲    <Attribute>
              =     PunctuationToken                                                                                                      */
"mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins"]                                              /*
"mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins"]    </Attribute>
"mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins•mushkins"     Literal                                   */

match t.kind() {                                                                                                                          /*
match•t.kind()•{↲    <MatchExpression>
      t.kind()       CallExpression                                                                                                       */
    ty::Int(i) => find_fit!(i, val, negative,                                                                                             /*
    ty::Int(i)•=>•find_fit!(i,•val,•negative,↲    <MatchExpressionCase>
    ty::Int(i)                                    TuplePattern
    ty::Int                                       ExpressionPath
                  find_fit!(i,•val,•negative,↲    <MacroInvocation>
                             ,                    PunctuationToken
                                  ,               PunctuationToken
                                            ,     PunctuationToken                                                                        */
                  I8 => [U8] => [I16, I32, I64, I128],                                                                                    /*
                     =>                                   PunctuationToken
                        [U8]                              DelimGroup
                             =>                           PunctuationToken
                                [I16,•I32,•I64,•I128]     DelimGroup
                                    ,                     PunctuationToken
                                         ,                PunctuationToken
                                              ,           PunctuationToken
                                                     ,    PunctuationToken                                                                */
                  I16 => [U16] => [I32, I64, I128],                                                                                       /*
                      =>                               PunctuationToken
                         [U16]                         DelimGroup
                               =>                      PunctuationToken
                                  [I32,•I64,•I128]     DelimGroup
                                      ,                PunctuationToken
                                           ,           PunctuationToken
                                                  ,    PunctuationToken                                                                   */
                  I32 => [U32] => [I64, I128],                                                                                            /*
                      =>                          PunctuationToken
                         [U32]                    DelimGroup
                               =>                 PunctuationToken
                                  [I64,•I128]     DelimGroup
                                      ,           PunctuationToken
                                             ,    PunctuationToken                                                                        */
                  I64 => [U64] => [I128],                                                                                                 /*
                      =>                     PunctuationToken
                         [U64]               DelimGroup
                               =>            PunctuationToken
                                  [I128]     DelimGroup
                                        ,    PunctuationToken                                                                             */
                  I128 => [U128] => []),                                                                                                  /*
••••••••••••••••••I128•=>•[U128]•=>•[])     </MatchExpressionCase>, </MacroInvocation>
                       =>                   PunctuationToken
                          [U128]            DelimGroup
                                 =>         PunctuationToken
                                    []      DelimGroup                                                                                    */
    ty::Uint(u) => find_fit!(u, val, negative,                                                                                            /*
    ty::Uint(u)•=>•find_fit!(u,•val,•negative,↲    <MatchExpressionCase>
    ty::Uint(u)                                    TuplePattern
    ty::Uint                                       ExpressionPath
                   find_fit!(u,•val,•negative,↲    <MacroInvocation>
                              ,                    PunctuationToken
                                   ,               PunctuationToken
                                             ,     PunctuationToken                                                                       */
                  U8 => [U8, U16, U32, U64, U128] => [],                                                                                  /*
                     =>                                     PunctuationToken
                        [U8,•U16,•U32,•U64,•U128]           DelimGroup
                           ,                                PunctuationToken
                                ,                           PunctuationToken
                                     ,                      PunctuationToken
                                          ,                 PunctuationToken
                                                  =>        PunctuationToken
                                                     []     DelimGroup
                                                       ,    PunctuationToken                                                              */
                  U16 => [U16, U32, U64, U128] => [],                                                                                     /*
                      =>                                 PunctuationToken
                         [U16,•U32,•U64,•U128]           DelimGroup
                             ,                           PunctuationToken
                                  ,                      PunctuationToken
                                       ,                 PunctuationToken
                                               =>        PunctuationToken
                                                  []     DelimGroup
                                                    ,    PunctuationToken                                                                 */
                  U32 => [U32, U64, U128] => [],                                                                                          /*
                      =>                            PunctuationToken
                         [U32,•U64,•U128]           DelimGroup
                             ,                      PunctuationToken
                                  ,                 PunctuationToken
                                          =>        PunctuationToken
                                             []     DelimGroup
                                               ,    PunctuationToken                                                                      */
                  U64 => [U64, U128] => [],                                                                                               /*
                      =>                       PunctuationToken
                         [U64,•U128]           DelimGroup
                             ,                 PunctuationToken
                                     =>        PunctuationToken
                                        []     DelimGroup
                                          ,    PunctuationToken                                                                           */
                  U128 => [U128] => []),                                                                                                  /*
••••••••••••••••••U128•=>•[U128]•=>•[])     </MatchExpressionCase>, </MacroInvocation>
                       =>                   PunctuationToken
                          [U128]            DelimGroup
                                 =>         PunctuationToken
                                    []      DelimGroup                                                                                    */
    _ => None,                                                                                                                            /*
    _•=>•None     MatchExpressionCase
    _             WildcardPattern                                                                                                         */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MatchExpression>                                                                                           */
x! {                                                                                                                                      /*
x!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                                         */
    /// [d]: ../b.md#a
    ///•[d]:•../b.md#a    Comment
    pub G,                                                                                                                                /*
         ,    PunctuationToken                                                                                                            */
    d,                                                                                                                                    /*
     ,    PunctuationToken                                                                                                                */
    "c",                                                                                                                                  /*
    "c"     Literal
       ,    PunctuationToken                                                                                                              */
    @d = x {                                                                                                                              /*
    @            PunctuationToken
       =         PunctuationToken
           {↲    <DelimGroup>                                                                                                             */
        b: "a",                                                                                                                           /*
         :         PunctuationToken
           "a"     Literal
              ,    PunctuationToken                                                                                                       */
    };                                                                                                                                    /*
••••}     </DelimGroup>
     ;    PunctuationToken                                                                                                                */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */
throw_span_err!(                                                                                                                          /*
throw_span_err!(↲    <ExpressionStatement>, <MacroInvocation>                                                                             */
    attr.span().unwrap(),                                                                                                                 /*
        .                    PunctuationToken
             ()              DelimGroup
               .             PunctuationToken
                      ()     DelimGroup
                        ,    PunctuationToken                                                                                             */
    &format!(                                                                                                                             /*
    &             PunctuationToken
           !      PunctuationToken
            (↲    <DelimGroup>                                                                                                            */
        "the `#[{}{}]` attribute can only be applied to fields of type {}",                                                               /*
        "the•`#[{}{}]`•attribute•can•only•be•applied•to•fields•of•type•{}"     Literal
                                                                          ,    PunctuationToken                                           */
        name,                                                                                                                             /*
            ,    PunctuationToken                                                                                                         */
        match meta {                                                                                                                      /*
                   {↲    <DelimGroup>                                                                                                     */
            Meta::Path(_) => "",                                                                                                          /*
                ::                  PunctuationToken
                      (_)           DelimGroup
                       _            PunctuationToken
                          =>        PunctuationToken
                             ""     Literal
                               ,    PunctuationToken                                                                                      */
            Meta::NameValue(_) => " = ...",                                                                                               /*
                ::                             PunctuationToken
                           (_)                 DelimGroup
                            _                  PunctuationToken
                               =>              PunctuationToken
                                  "•=•..."     Literal
                                          ,    PunctuationToken                                                                           */
            Meta::List(_) => "(...)",                                                                                                     /*
                ::                       PunctuationToken
                      (_)                DelimGroup
                       _                 PunctuationToken
                          =>             PunctuationToken
                             "(...)"     Literal
                                    ,    PunctuationToken                                                                                 */
        },                                                                                                                                /*
••••••••}     </DelimGroup>
         ,    PunctuationToken                                                                                                            */
        ty_name
    )                                                                                                                                     /*
••••)    </DelimGroup>                                                                                                                    */
);                                                                                                                                        /*
);    </ExpressionStatement>
)     </MacroInvocation>                                                                                                                  */

provide! {                                                                                                                                /*
provide!•{•↲    <ExpressionStatement>, <MacroInvocation>                                                                                  */
    <'tcx> tcx, def_id, other, cdata,                                                                                                     /*
    <                                    PunctuationToken
     'tcx                                LtIdentifier
         >                               PunctuationToken
              ,                          PunctuationToken
                      ,                  PunctuationToken
                             ,           PunctuationToken
                                    ,    PunctuationToken                                                                                 */
    explicit_item_bounds => { table }                                                                                                     /*
                         =>              PunctuationToken
                            {•table•}    DelimGroup                                                                                       */
    explicit_predicates_of => { table }                                                                                                   /*
                           =>              PunctuationToken
                              {•table•}    DelimGroup                                                                                     */
    adt_destructor => {                                                                                                                   /*
                   =>       PunctuationToken
                      {↲    <DelimGroup>                                                                                                  */
        let _ = cdata;                                                                                                                    /*
            _             PunctuationToken
              =           PunctuationToken
                     ;    PunctuationToken                                                                                                */
        tcx.calculate_dtor(def_id, |_,_| Ok(()))                                                                                          /*
           .                                        PunctuationToken
                          (def_id,•|_,_|•Ok(()))    DelimGroup
                                 ,                  PunctuationToken
                                   |                PunctuationToken
                                    _               PunctuationToken
                                     ,              PunctuationToken
                                      _             PunctuationToken
                                       |            PunctuationToken
                                           (())     DelimGroup
                                            ()      DelimGroup                                                                            */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    extern_crate => {                                                                                                                     /*
                 =>       PunctuationToken
                    {↲    <DelimGroup>                                                                                                    */
        let r = *cdata.extern_crate.lock();                                                                                               /*
              =                                PunctuationToken
                *                              PunctuationToken
                      .                        PunctuationToken
                                   .           PunctuationToken
                                        ()     DelimGroup
                                          ;    PunctuationToken                                                                           */
        r.map(|c| &*tcx.arena.alloc(c))                                                                                                   /*
         .                                 PunctuationToken
             (|c|•&*tcx.arena.alloc(c))    DelimGroup
              |                            PunctuationToken
                |                          PunctuationToken
                  &                        PunctuationToken
                   *                       PunctuationToken
                       .                   PunctuationToken
                             .             PunctuationToken
                                   (c)     DelimGroup                                                                                     */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    reachable_non_generics => {                                                                                                           /*
                           =>       PunctuationToken
                              {↲    <DelimGroup>                                                                                          */
        let reachable_non_generics = tcx                                                                                                  /*
                                   =        PunctuationToken                                                                              */
            .exported_symbols(cdata.cnum)                                                                                                 /*
            .                                PunctuationToken
                             (cdata.cnum)    DelimGroup
                                   .         PunctuationToken                                                                             */
            .iter()                                                                                                                       /*
            .          PunctuationToken
                 ()    DelimGroup                                                                                                         */
            .filter_map(|&(exported_symbol, export_info)| {                                                                               /*
            .                                                   PunctuationToken
                       (|&(exported_symbol,•export_info)|•{↲    <DelimGroup>
                        |                                       PunctuationToken
                         &                                      PunctuationToken
                          (exported_symbol,•export_info)        DelimGroup
                                          ,                     PunctuationToken
                                                        |       PunctuationToken
                                                          {↲    <DelimGroup>                                                              */
                if let ExportedSymbol::NonGeneric(def_id) = exported_symbol {                                                             /*
                                     ::                                           PunctuationToken
                                                 (def_id)                         DelimGroup
                                                          =                       PunctuationToken
                                                                            {↲    <DelimGroup>                                            */
                    Some((def_id, export_info))                                                                                           /*
                        ((def_id,•export_info))    DelimGroup
                         (def_id,•export_info)     DelimGroup
                                ,                  PunctuationToken                                                                       */
                } else {                                                                                                                  /*
••••••••••••••••}            </DelimGroup>
                       {↲    <DelimGroup>                                                                                                 */
                    None
                }                                                                                                                         /*
••••••••••••••••}    </DelimGroup>                                                                                                        */
            })                                                                                                                            /*
••••••••••••})    </DelimGroup>
••••••••••••}     </DelimGroup>                                                                                                           */
            .collect();                                                                                                                   /*
            .              PunctuationToken
                    ()     DelimGroup
                      ;    PunctuationToken                                                                                               */

        reachable_non_generics
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    dep_kind => {                                                                                                                         /*
             =>       PunctuationToken
                {↲    <DelimGroup>                                                                                                        */
        let r = *cdata.dep_kind.lock();                                                                                                   /*
              =                            PunctuationToken
                *                          PunctuationToken
                      .                    PunctuationToken
                               .           PunctuationToken
                                    ()     DelimGroup
                                      ;    PunctuationToken                                                                               */
        r
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    module_children => {                                                                                                                  /*
                    =>       PunctuationToken
                       {↲    <DelimGroup>                                                                                                 */
        let mut result = SmallVec::<[_; 8]>::new();                                                                                       /*
                       =                               PunctuationToken
                                 ::                    PunctuationToken
                                   <                   PunctuationToken
                                    [_;•8]             DelimGroup
                                     _                 PunctuationToken
                                      ;                PunctuationToken
                                        8              Literal
                                          >            PunctuationToken
                                           ::          PunctuationToken
                                                ()     DelimGroup
                                                  ;    PunctuationToken                                                                   */
        cdata.for_each_module_child(def_id.index, |child| result.push(child), tcx.sess);                                                  /*
             .                                                                              PunctuationToken
                                   (def_id.index,•|child|•result.push(child),•tcx.sess)     DelimGroup
                                          .                                                 PunctuationToken
                                                ,                                           PunctuationToken
                                                  |                                         PunctuationToken
                                                        |                                   PunctuationToken
                                                                .                           PunctuationToken
                                                                     (child)                DelimGroup
                                                                            ,               PunctuationToken
                                                                                 .          PunctuationToken
                                                                                       ;    PunctuationToken                              */
        tcx.arena.alloc_slice(&result)                                                                                                    /*
           .                              PunctuationToken
                 .                        PunctuationToken
                             (&result)    DelimGroup
                              &           PunctuationToken                                                                                */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */

    missing_extern_crate_item => {                                                                                                        /*
                              =>       PunctuationToken
                                 {↲    <DelimGroup>                                                                                       */
        let r = matches!(*cdata.extern_crate.borrow(), Some(extern_crate) if !extern_crate.is_direct());                                  /*
              =                                                                                             PunctuationToken
                       !                                                                                    PunctuationToken
                        (*cdata.extern_crate.borrow(),•Some(extern_crate)•if•!extern_crate.is_direct())     DelimGroup
                         *                                                                                  PunctuationToken
                               .                                                                            PunctuationToken
                                            .                                                               PunctuationToken
                                                   ()                                                       DelimGroup
                                                     ,                                                      PunctuationToken
                                                           (extern_crate)                                   DelimGroup
                                                                             !                              PunctuationToken
                                                                                          .                 PunctuationToken
                                                                                                    ()      DelimGroup
                                                                                                       ;    PunctuationToken              */
        r
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */

    used_crate_source => { Lrc::clone(&cdata.source) }                                                                                    /*
                      =>                                  PunctuationToken
                         {•Lrc::clone(&cdata.source)•}    DelimGroup
                              ::                          PunctuationToken
                                     (&cdata.source)      DelimGroup
                                      &                   PunctuationToken
                                            .             PunctuationToken                                                                */
    debugger_visualizers => { cdata.get_debugger_visualizers() }                                                                          /*
                         =>                                         PunctuationToken
                            {•cdata.get_debugger_visualizers()•}    DelimGroup
                                   .                                PunctuationToken
                                                            ()      DelimGroup                                                            */

    exported_symbols => {                                                                                                                 /*
                     =>       PunctuationToken
                        {↲    <DelimGroup>                                                                                                */
        let syms = cdata.exported_symbols(tcx);                                                                                           /*
                 =                                 PunctuationToken
                        .                          PunctuationToken
                                         (tcx)     DelimGroup
                                              ;    PunctuationToken                                                                       */

        // a
        //•a    Comment

        syms
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

cfg_if::cfg_if! {                                                                                                                         /*
cfg_if::cfg_if!•{↲    <ExpressionStatement>, <MacroInvocation>
cfg_if::cfg_if        ExpressionPath                                                                                                      */
    if #[cfg(unix)] {                                                                                                                     /*
       #                  PunctuationToken
        [cfg(unix)]       DelimGroup
            (unix)        DelimGroup
                    {↲    <DelimGroup>                                                                                                    */
    } else if #[cfg(any(target_os = "hermit",                                                                                             /*
••••}                                             </DelimGroup>
              #                                   PunctuationToken
               [cfg(any(target_os•=•"hermit",↲    <DelimGroup>
                   (any(target_os•=•"hermit",↲    <DelimGroup>
                       (target_os•=•"hermit",↲    <DelimGroup>
                                  =               PunctuationToken
                                    "hermit"      Literal
                                            ,     PunctuationToken                                                                        */
                        target_os = "solid_asp3",                                                                                         /*
                                  =                  PunctuationToken
                                    "solid_asp3"     Literal
                                                ,    PunctuationToken                                                                     */
                        all(target_vendor = "fortanix", target_env = "sgx")                                                               /*
                           (target_vendor•=•"fortanix",•target_env•=•"sgx")    DelimGroup
                                          =                                    PunctuationToken
                                            "fortanix"                         Literal
                                                      ,                        PunctuationToken
                                                                   =           PunctuationToken
                                                                     "sgx"     Literal                                                    */
    ))] {                                                                                                                                 /*
••••))]       </DelimGroup>
••••))        </DelimGroup>
••••)         </DelimGroup>
        {↲    <DelimGroup>                                                                                                                */
    } else if #[cfg(all(windows, not(miri)))] {                                                                                           /*
••••}                                               </DelimGroup>
              #                                     PunctuationToken
               [cfg(all(windows,•not(miri)))]       DelimGroup
                   (all(windows,•not(miri)))        DelimGroup
                       (windows,•not(miri))         DelimGroup
                               ,                    PunctuationToken
                                    (miri)          DelimGroup
                                              {↲    <DelimGroup>                                                                          */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

ast_fragments! {                                                                                                                          /*
ast_fragments!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                             */
    Expr(P<ast::Expr>) { "expression"; one fn visit_expr; fn visit_expr; fn make_expr; }                                                  /*
        (P<ast::Expr>)                                                                      DelimGroup
          <                                                                                 PunctuationToken
              ::                                                                            PunctuationToken
                    >                                                                       PunctuationToken
                       {•"expression";•one•fn•visit_expr;•fn•visit_expr;•fn•make_expr;•}    DelimGroup
                         "expression"                                                       Literal
                                     ;                                                      PunctuationToken
                                                        ;                                   PunctuationToken
                                                                       ;                    PunctuationToken
                                                                                     ;      PunctuationToken                              */
    Pat(P<ast::Pat>) { "pattern"; one fn visit_pat; fn visit_pat; fn make_pat; }                                                          /*
       (P<ast::Pat>)                                                                DelimGroup
         <                                                                          PunctuationToken
             ::                                                                     PunctuationToken
                  >                                                                 PunctuationToken
                     {•"pattern";•one•fn•visit_pat;•fn•visit_pat;•fn•make_pat;•}    DelimGroup
                       "pattern"                                                    Literal
                                ;                                                   PunctuationToken
                                                  ;                                 PunctuationToken
                                                                ;                   PunctuationToken
                                                                             ;      PunctuationToken                                      */
    Ty(P<ast::Ty>) { "type"; one fn visit_ty; fn visit_ty; fn make_ty; }                                                                  /*
      (P<ast::Ty>)                                                          DelimGroup
        <                                                                   PunctuationToken
            ::                                                              PunctuationToken
                >                                                           PunctuationToken
                   {•"type";•one•fn•visit_ty;•fn•visit_ty;•fn•make_ty;•}    DelimGroup
                     "type"                                                 Literal
                           ;                                                PunctuationToken
                                            ;                               PunctuationToken
                                                         ;                  PunctuationToken
                                                                     ;      PunctuationToken                                              */
    Stmts(SmallVec<[ast::Stmt; 1]>) {                                                                                                     /*
         (SmallVec<[ast::Stmt;•1]>)       DelimGroup
                  <                       PunctuationToken
                   [ast::Stmt;•1]         DelimGroup
                       ::                 PunctuationToken
                             ;            PunctuationToken
                               1          Literal
                                 >        PunctuationToken
                                    {↲    <DelimGroup>                                                                                    */
        "statement"; many fn flat_map_stmt; fn visit_stmt(); fn make_stmts;                                                               /*
        "statement"                                                            Literal
                   ;                                                           PunctuationToken
                                          ;                                    PunctuationToken
                                                         ()                    DelimGroup
                                                           ;                   PunctuationToken
                                                                          ;    PunctuationToken                                           */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Items(SmallVec<[P<ast::Item>; 1]>) {                                                                                                  /*
         (SmallVec<[P<ast::Item>;•1]>)       DelimGroup
                  <                          PunctuationToken
                   [P<ast::Item>;•1]         DelimGroup
                     <                       PunctuationToken
                         ::                  PunctuationToken
                               >             PunctuationToken
                                ;            PunctuationToken
                                  1          Literal
                                    >        PunctuationToken
                                       {↲    <DelimGroup>                                                                                 */
        "item"; many fn flat_map_item; fn visit_item(); fn make_items;                                                                    /*
        "item"                                                            Literal
              ;                                                           PunctuationToken
                                     ;                                    PunctuationToken
                                                    ()                    DelimGroup
                                                      ;                   PunctuationToken
                                                                     ;    PunctuationToken                                                */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    TraitItems(SmallVec<[P<ast::AssocItem>; 1]>) {                                                                                        /*
              (SmallVec<[P<ast::AssocItem>;•1]>)       DelimGroup
                       <                               PunctuationToken
                        [P<ast::AssocItem>;•1]         DelimGroup
                          <                            PunctuationToken
                              ::                       PunctuationToken
                                         >             PunctuationToken
                                          ;            PunctuationToken
                                            1          Literal
                                              >        PunctuationToken
                                                 {↲    <DelimGroup>                                                                       */
        "trait item";                                                                                                                     /*
        "trait•item"     Literal
                    ;    PunctuationToken                                                                                                 */
        many fn flat_map_trait_item;                                                                                                      /*
                                   ;    PunctuationToken                                                                                  */
        fn visit_assoc_item(AssocCtxt::Trait);                                                                                            /*
                           (AssocCtxt::Trait)     DelimGroup
                                     ::           PunctuationToken
                                             ;    PunctuationToken                                                                        */
        fn make_trait_items;                                                                                                              /*
                           ;    PunctuationToken                                                                                          */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    ImplItems(SmallVec<[P<ast::AssocItem>; 1]>) {                                                                                         /*
             (SmallVec<[P<ast::AssocItem>;•1]>)       DelimGroup
                      <                               PunctuationToken
                       [P<ast::AssocItem>;•1]         DelimGroup
                         <                            PunctuationToken
                             ::                       PunctuationToken
                                        >             PunctuationToken
                                         ;            PunctuationToken
                                           1          Literal
                                             >        PunctuationToken
                                                {↲    <DelimGroup>                                                                        */
        "impl item";                                                                                                                      /*
        "impl•item"     Literal
                   ;    PunctuationToken                                                                                                  */
        many fn flat_map_impl_item;                                                                                                       /*
                                  ;    PunctuationToken                                                                                   */
        fn visit_assoc_item(AssocCtxt::Impl);                                                                                             /*
                           (AssocCtxt::Impl)     DelimGroup
                                     ::          PunctuationToken
                                            ;    PunctuationToken                                                                         */
        fn make_impl_items;                                                                                                               /*
                          ;    PunctuationToken                                                                                           */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    ForeignItems(SmallVec<[P<ast::ForeignItem>; 1]>) {                                                                                    /*
                (SmallVec<[P<ast::ForeignItem>;•1]>)       DelimGroup
                         <                                 PunctuationToken
                          [P<ast::ForeignItem>;•1]         DelimGroup
                            <                              PunctuationToken
                                ::                         PunctuationToken
                                             >             PunctuationToken
                                              ;            PunctuationToken
                                                1          Literal
                                                  >        PunctuationToken
                                                     {↲    <DelimGroup>                                                                   */
        "foreign item";                                                                                                                   /*
        "foreign•item"     Literal
                      ;    PunctuationToken                                                                                               */
        many fn flat_map_foreign_item;                                                                                                    /*
                                     ;    PunctuationToken                                                                                */
        fn visit_foreign_item();                                                                                                          /*
                             ()     DelimGroup
                               ;    PunctuationToken                                                                                      */
        fn make_foreign_items;                                                                                                            /*
                             ;    PunctuationToken                                                                                        */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Arms(SmallVec<[ast::Arm; 1]>) {                                                                                                       /*
        (SmallVec<[ast::Arm;•1]>)       DelimGroup
                 <                      PunctuationToken
                  [ast::Arm;•1]         DelimGroup
                      ::                PunctuationToken
                           ;            PunctuationToken
                             1          Literal
                               >        PunctuationToken
                                  {↲    <DelimGroup>                                                                                      */
        "match arm"; many fn flat_map_arm; fn visit_arm(); fn make_arms;                                                                  /*
        "match•arm"                                                         Literal
                   ;                                                        PunctuationToken
                                         ;                                  PunctuationToken
                                                       ()                   DelimGroup
                                                         ;                  PunctuationToken
                                                                       ;    PunctuationToken                                              */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    ExprFields(SmallVec<[ast::ExprField; 1]>) {                                                                                           /*
              (SmallVec<[ast::ExprField;•1]>)       DelimGroup
                       <                            PunctuationToken
                        [ast::ExprField;•1]         DelimGroup
                            ::                      PunctuationToken
                                       ;            PunctuationToken
                                         1          Literal
                                           >        PunctuationToken
                                              {↲    <DelimGroup>                                                                          */
        "field expression"; many fn flat_map_expr_field; fn visit_expr_field(); fn make_expr_fields;                                      /*
        "field•expression"                                                                              Literal
                          ;                                                                             PunctuationToken
                                                       ;                                                PunctuationToken
                                                                            ()                          DelimGroup
                                                                              ;                         PunctuationToken
                                                                                                   ;    PunctuationToken                  */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    PatFields(SmallVec<[ast::PatField; 1]>) {                                                                                             /*
             (SmallVec<[ast::PatField;•1]>)       DelimGroup
                      <                           PunctuationToken
                       [ast::PatField;•1]         DelimGroup
                           ::                     PunctuationToken
                                     ;            PunctuationToken
                                       1          Literal
                                         >        PunctuationToken
                                            {↲    <DelimGroup>                                                                            */
        "field pattern";                                                                                                                  /*
        "field•pattern"     Literal
                       ;    PunctuationToken                                                                                              */
        many fn flat_map_pat_field;                                                                                                       /*
                                  ;    PunctuationToken                                                                                   */
        fn visit_pat_field();                                                                                                             /*
                          ()     DelimGroup
                            ;    PunctuationToken                                                                                         */
        fn make_pat_fields;                                                                                                               /*
                          ;    PunctuationToken                                                                                           */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    GenericParams(SmallVec<[ast::GenericParam; 1]>) {                                                                                     /*
                 (SmallVec<[ast::GenericParam;•1]>)       DelimGroup
                          <                               PunctuationToken
                           [ast::GenericParam;•1]         DelimGroup
                               ::                         PunctuationToken
                                             ;            PunctuationToken
                                               1          Literal
                                                 >        PunctuationToken
                                                    {↲    <DelimGroup>                                                                    */
        "generic parameter";                                                                                                              /*
        "generic•parameter"     Literal
                           ;    PunctuationToken                                                                                          */
        many fn flat_map_generic_param;                                                                                                   /*
                                      ;    PunctuationToken                                                                               */
        fn visit_generic_param();                                                                                                         /*
                              ()     DelimGroup
                                ;    PunctuationToken                                                                                     */
        fn make_generic_params;                                                                                                           /*
                              ;    PunctuationToken                                                                                       */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Params(SmallVec<[ast::Param; 1]>) {                                                                                                   /*
          (SmallVec<[ast::Param;•1]>)       DelimGroup
                   <                        PunctuationToken
                    [ast::Param;•1]         DelimGroup
                        ::                  PunctuationToken
                               ;            PunctuationToken
                                 1          Literal
                                   >        PunctuationToken
                                      {↲    <DelimGroup>                                                                                  */
        "function parameter"; many fn flat_map_param; fn visit_param(); fn make_params;                                                   /*
        "function•parameter"                                                               Literal
                            ;                                                              PunctuationToken
                                                    ;                                      PunctuationToken
                                                                    ()                     DelimGroup
                                                                      ;                    PunctuationToken
                                                                                      ;    PunctuationToken                               */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    FieldDefs(SmallVec<[ast::FieldDef; 1]>) {                                                                                             /*
             (SmallVec<[ast::FieldDef;•1]>)       DelimGroup
                      <                           PunctuationToken
                       [ast::FieldDef;•1]         DelimGroup
                           ::                     PunctuationToken
                                     ;            PunctuationToken
                                       1          Literal
                                         >        PunctuationToken
                                            {↲    <DelimGroup>                                                                            */
        "field";                                                                                                                          /*
        "field"     Literal
               ;    PunctuationToken                                                                                                      */
        many fn flat_map_field_def;                                                                                                       /*
                                  ;    PunctuationToken                                                                                   */
        fn visit_field_def();                                                                                                             /*
                          ()     DelimGroup
                            ;    PunctuationToken                                                                                         */
        fn make_field_defs;                                                                                                               /*
                          ;    PunctuationToken                                                                                           */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Variants(SmallVec<[ast::Variant; 1]>) {                                                                                               /*
            (SmallVec<[ast::Variant;•1]>)       DelimGroup
                     <                          PunctuationToken
                      [ast::Variant;•1]         DelimGroup
                          ::                    PunctuationToken
                                   ;            PunctuationToken
                                     1          Literal
                                       >        PunctuationToken
                                          {↲    <DelimGroup>                                                                              */
        "variant"; many fn flat_map_variant; fn visit_variant(); fn make_variants;                                                        /*
        "variant"                                                                     Literal
                 ;                                                                    PunctuationToken
                                           ;                                          PunctuationToken
                                                             ()                       DelimGroup
                                                               ;                      PunctuationToken
                                                                                 ;    PunctuationToken                                    */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    Crate(ast::Crate) { "crate"; one fn visit_crate; fn visit_crate; fn make_crate; }                                                     /*
         (ast::Crate)                                                                    DelimGroup
             ::                                                                          PunctuationToken
                      {•"crate";•one•fn•visit_crate;•fn•visit_crate;•fn•make_crate;•}    DelimGroup
                        "crate"                                                          Literal
                               ;                                                         PunctuationToken
                                                   ;                                     PunctuationToken
                                                                   ;                     PunctuationToken
                                                                                  ;      PunctuationToken                                 */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

sides_mapping_methods! {                                                                                                                  /*
sides_mapping_methods!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                     */
    MainStartCrossStart::InlineStartBlockStart => {                                                                                       /*
                       ::                               PunctuationToken
                                               =>       PunctuationToken
                                                  {↲    <DelimGroup>                                                                      */
        main_start <=> inline_start,                                                                                                      /*
                   <=                   PunctuationToken
                     >                  PunctuationToken
                                   ,    PunctuationToken                                                                                  */
        cross_start <=> block_start,                                                                                                      /*
                    <=                  PunctuationToken
                      >                 PunctuationToken
                                   ,    PunctuationToken                                                                                  */
        main_end <=> inline_end,                                                                                                          /*
                 <=                 PunctuationToken
                   >                PunctuationToken
                               ,    PunctuationToken                                                                                      */
        cross_end <=> block_end,                                                                                                          /*
                  <=                PunctuationToken
                    >               PunctuationToken
                               ,    PunctuationToken                                                                                      */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

fuzz_target!(|data: &[u8]| if let Some(first) = data.first() {                                                                            /*
fuzz_target!(|data:•&[u8]|•if•let•Some(first)•=•data.first()•{↲    <ExpressionStatement>, <MacroInvocation>
             |                                                     PunctuationToken
                  :                                                PunctuationToken
                    &                                              PunctuationToken
                     [u8]                                          DelimGroup
                         |                                         PunctuationToken
                                      (first)                      DelimGroup
                                              =                    PunctuationToken
                                                    .              PunctuationToken
                                                          ()       DelimGroup
                                                             {↲    <DelimGroup>                                                           */
    let index = *first as usize;                                                                                                          /*
              =                     PunctuationToken
                *                   PunctuationToken
                               ;    PunctuationToken                                                                                      */
    if index >= ENCODINGS.len() {                                                                                                         /*
             >=                       PunctuationToken
                         .            PunctuationToken
                             ()       DelimGroup
                                {↲    <DelimGroup>                                                                                        */
        return;                                                                                                                           /*
              ;    PunctuationToken                                                                                                       */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
    let encoding = ENCODINGS[index];                                                                                                      /*
                 =                      PunctuationToken
                            [index]     DelimGroup
                                   ;    PunctuationToken                                                                                  */
    dispatch_test(encoding, &data[1..]);                                                                                                  /*
                 (encoding,•&data[1..])     DelimGroup
                          ,                 PunctuationToken
                            &               PunctuationToken
                                 [1..]      DelimGroup
                                  1         Literal
                                   ..       PunctuationToken
                                       ;    PunctuationToken                                                                              */
});                                                                                                                                       /*
});    </ExpressionStatement>
})     </MacroInvocation>
}      </DelimGroup>                                                                                                                      */

assert_eq!(count_compound_ops!(foo<=>bar <<<! -baz ++), 4);                                                                               /*
assert_eq!(count_compound_ops!(foo<=>bar•<<<!•-baz•++),•4);    ExpressionStatement
assert_eq!(count_compound_ops!(foo<=>bar•<<<!•-baz•++),•4)     MacroInvocation
                             !                                 PunctuationToken
                              (foo<=>bar•<<<!•-baz•++)         DelimGroup
                                  <=                           PunctuationToken
                                    >                          PunctuationToken
                                         <<                    PunctuationToken
                                           <                   PunctuationToken
                                            !                  PunctuationToken
                                              -                PunctuationToken
                                                   +           PunctuationToken
                                                    +          PunctuationToken
                                                      ,        PunctuationToken
                                                        4      Literal                                                                    */

print_bang! {                                                                                                                             /*
print_bang!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                                */

/**                                                                                                                                       /*
/**↲    <Comment>                                                                                                                         */
*******
* DOC *
* DOC *
* DOC *
*******
*/                                                                                                                                        /*
*/    </Comment>                                                                                                                          */
pub struct S;                                                                                                                             /*
            ;    PunctuationToken                                                                                                         */

}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

c! { pub const A: i32 = 0; }                                                                                                              /*
c!•{•pub•const•A:•i32•=•0;•}    ExpressionStatement, MacroInvocation
                :               PunctuationToken
                      =         PunctuationToken
                        0       Literal
                         ;      PunctuationToken                                                                                          */
c! { pub enum B {} }                                                                                                                      /*
c!•{•pub•enum•B•{}•}    ExpressionStatement, MacroInvocation
                {}      DelimGroup                                                                                                        */
c! { pub extern "C" fn c() {} }                                                                                                           /*
c!•{•pub•extern•"C"•fn•c()•{}•}    ExpressionStatement, MacroInvocation
                "C"                Literal
                        ()         DelimGroup
                           {}      DelimGroup                                                                                             */
c! { pub mod d {} }                                                                                                                       /*
c!•{•pub•mod•d•{}•}    ExpressionStatement, MacroInvocation
               {}      DelimGroup                                                                                                         */
c! { pub static E: i32 = 0; }                                                                                                             /*
c!•{•pub•static•E:•i32•=•0;•}    ExpressionStatement, MacroInvocation
                 :               PunctuationToken
                       =         PunctuationToken
                         0       Literal
                          ;      PunctuationToken                                                                                         */
c! { pub struct F; }                                                                                                                      /*
c!•{•pub•struct•F;•}    ExpressionStatement, MacroInvocation
                 ;      PunctuationToken                                                                                                  */
c! { pub trait G {} }                                                                                                                     /*
c!•{•pub•trait•G•{}•}    ExpressionStatement, MacroInvocation
                 {}      DelimGroup                                                                                                       */
c! { pub type H = i32; }                                                                                                                  /*
c!•{•pub•type•H•=•i32;•}    ExpressionStatement, MacroInvocation
                =           PunctuationToken
                     ;      PunctuationToken                                                                                              */
c! { pub use A as I; }                                                                                                                    /*
c!•{•pub•use•A•as•I;•}    ExpressionStatement, MacroInvocation
                   ;      PunctuationToken                                                                                                */
c! { const A: i32 = 0; }                                                                                                                  /*
c!•{•const•A:•i32•=•0;•}    ExpressionStatement, MacroInvocation
            :               PunctuationToken
                  =         PunctuationToken
                    0       Literal
                     ;      PunctuationToken                                                                                              */
c! { enum B {} }                                                                                                                          /*
c!•{•enum•B•{}•}    ExpressionStatement, MacroInvocation
            {}      DelimGroup                                                                                                            */
c! { extern "C" fn c() {} }                                                                                                               /*
c!•{•extern•"C"•fn•c()•{}•}    ExpressionStatement, MacroInvocation
            "C"                Literal
                    ()         DelimGroup
                       {}      DelimGroup                                                                                                 */
c! { mod d {} }                                                                                                                           /*
c!•{•mod•d•{}•}    ExpressionStatement, MacroInvocation
           {}      DelimGroup                                                                                                             */
c! { static E: i32 = 0; }                                                                                                                 /*
c!•{•static•E:•i32•=•0;•}    ExpressionStatement, MacroInvocation
             :               PunctuationToken
                   =         PunctuationToken
                     0       Literal
                      ;      PunctuationToken                                                                                             */
c! { struct F; }                                                                                                                          /*
c!•{•struct•F;•}    ExpressionStatement, MacroInvocation
             ;      PunctuationToken                                                                                                      */
c! { trait G {} }                                                                                                                         /*
c!•{•trait•G•{}•}    ExpressionStatement, MacroInvocation
             {}      DelimGroup                                                                                                           */
c! { type H = i32; }                                                                                                                      /*
c!•{•type•H•=•i32;•}    ExpressionStatement, MacroInvocation
            =           PunctuationToken
                 ;      PunctuationToken                                                                                                  */
c! { use A as I; }                                                                                                                        /*
c!•{•use•A•as•I;•}    ExpressionStatement, MacroInvocation
               ;      PunctuationToken                                                                                                    */
c! { pub(crate) const A: i32 = 0; }                                                                                                       /*
c!•{•pub(crate)•const•A:•i32•=•0;•}    ExpressionStatement, MacroInvocation
        (crate)                        DelimGroup
                       :               PunctuationToken
                             =         PunctuationToken
                               0       Literal
                                ;      PunctuationToken                                                                                   */
c! { pub(crate) enum B {} }                                                                                                               /*
c!•{•pub(crate)•enum•B•{}•}    ExpressionStatement, MacroInvocation
        (crate)                DelimGroup
                       {}      DelimGroup                                                                                                 */
c! { pub(crate) extern "C" fn c() {} }                                                                                                    /*
c!•{•pub(crate)•extern•"C"•fn•c()•{}•}    ExpressionStatement, MacroInvocation
        (crate)                           DelimGroup
                       "C"                Literal
                               ()         DelimGroup
                                  {}      DelimGroup                                                                                      */
c! { pub(crate) mod d {} }                                                                                                                /*
c!•{•pub(crate)•mod•d•{}•}    ExpressionStatement, MacroInvocation
        (crate)               DelimGroup
                      {}      DelimGroup                                                                                                  */
c! { pub(crate) static E: i32 = 0; }                                                                                                      /*
c!•{•pub(crate)•static•E:•i32•=•0;•}    ExpressionStatement, MacroInvocation
        (crate)                         DelimGroup
                        :               PunctuationToken
                              =         PunctuationToken
                                0       Literal
                                 ;      PunctuationToken                                                                                  */
c! { pub(crate) struct F; }                                                                                                               /*
c!•{•pub(crate)•struct•F;•}    ExpressionStatement, MacroInvocation
        (crate)                DelimGroup
                        ;      PunctuationToken                                                                                           */
c! { pub(crate) trait G {} }                                                                                                              /*
c!•{•pub(crate)•trait•G•{}•}    ExpressionStatement, MacroInvocation
        (crate)                 DelimGroup
                        {}      DelimGroup                                                                                                */
c! { pub(crate) type H = i32; }                                                                                                           /*
c!•{•pub(crate)•type•H•=•i32;•}    ExpressionStatement, MacroInvocation
        (crate)                    DelimGroup
                       =           PunctuationToken
                            ;      PunctuationToken                                                                                       */
c! { pub(crate) use A as I; }                                                                                                             /*
c!•{•pub(crate)•use•A•as•I;•}    ExpressionStatement, MacroInvocation
        (crate)                  DelimGroup
                          ;      PunctuationToken                                                                                         */
c! { crate const A: i32 = 0; }                                                                                                            /*
c!•{•crate•const•A:•i32•=•0;•}    ExpressionStatement, MacroInvocation
                  :               PunctuationToken
                        =         PunctuationToken
                          0       Literal
                           ;      PunctuationToken                                                                                        */
c! { crate enum B {} }                                                                                                                    /*
c!•{•crate•enum•B•{}•}    ExpressionStatement, MacroInvocation
                  {}      DelimGroup                                                                                                      */
c! { crate extern "C" fn c() {} }                                                                                                         /*
c!•{•crate•extern•"C"•fn•c()•{}•}    ExpressionStatement, MacroInvocation
                  "C"                Literal
                          ()         DelimGroup
                             {}      DelimGroup                                                                                           */
c! { crate mod d {} }                                                                                                                     /*
c!•{•crate•mod•d•{}•}    ExpressionStatement, MacroInvocation
                 {}      DelimGroup                                                                                                       */
c! { crate static E: i32 = 0; }                                                                                                           /*
c!•{•crate•static•E:•i32•=•0;•}    ExpressionStatement, MacroInvocation
                   :               PunctuationToken
                         =         PunctuationToken
                           0       Literal
                            ;      PunctuationToken                                                                                       */
c! { crate struct F; }                                                                                                                    /*
c!•{•crate•struct•F;•}    ExpressionStatement, MacroInvocation
                   ;      PunctuationToken                                                                                                */
c! { crate trait G {} }                                                                                                                   /*
c!•{•crate•trait•G•{}•}    ExpressionStatement, MacroInvocation
                   {}      DelimGroup                                                                                                     */
c! { crate type H = i32; }                                                                                                                /*
c!•{•crate•type•H•=•i32;•}    ExpressionStatement, MacroInvocation
                  =           PunctuationToken
                       ;      PunctuationToken                                                                                            */
c! { crate use A as I; }                                                                                                                  /*
c!•{•crate•use•A•as•I;•}    ExpressionStatement, MacroInvocation
                     ;      PunctuationToken                                                                                              */
c! { pub(in foo) const A: i32 = 0; }                                                                                                      /*
c!•{•pub(in•foo)•const•A:•i32•=•0;•}    ExpressionStatement, MacroInvocation
        (in•foo)                        DelimGroup
                        :               PunctuationToken
                              =         PunctuationToken
                                0       Literal
                                 ;      PunctuationToken                                                                                  */
c! { pub(in foo) enum B {} }                                                                                                              /*
c!•{•pub(in•foo)•enum•B•{}•}    ExpressionStatement, MacroInvocation
        (in•foo)                DelimGroup
                        {}      DelimGroup                                                                                                */
c! { pub(in foo) extern "C" fn c() {} }                                                                                                   /*
c!•{•pub(in•foo)•extern•"C"•fn•c()•{}•}    ExpressionStatement, MacroInvocation
        (in•foo)                           DelimGroup
                        "C"                Literal
                                ()         DelimGroup
                                   {}      DelimGroup                                                                                     */
c! { pub(in foo) mod d {} }                                                                                                               /*
c!•{•pub(in•foo)•mod•d•{}•}    ExpressionStatement, MacroInvocation
        (in•foo)               DelimGroup
                       {}      DelimGroup                                                                                                 */
c! { pub(in foo) static E: i32 = 0; }                                                                                                     /*
c!•{•pub(in•foo)•static•E:•i32•=•0;•}    ExpressionStatement, MacroInvocation
        (in•foo)                         DelimGroup
                         :               PunctuationToken
                               =         PunctuationToken
                                 0       Literal
                                  ;      PunctuationToken                                                                                 */
c! { pub(in foo) struct F; }                                                                                                              /*
c!•{•pub(in•foo)•struct•F;•}    ExpressionStatement, MacroInvocation
        (in•foo)                DelimGroup
                         ;      PunctuationToken                                                                                          */
c! { pub(in foo) trait G {} }                                                                                                             /*
c!•{•pub(in•foo)•trait•G•{}•}    ExpressionStatement, MacroInvocation
        (in•foo)                 DelimGroup
                         {}      DelimGroup                                                                                               */
c! { pub(in foo) type H = i32; }                                                                                                          /*
c!•{•pub(in•foo)•type•H•=•i32;•}    ExpressionStatement, MacroInvocation
        (in•foo)                    DelimGroup
                        =           PunctuationToken
                             ;      PunctuationToken                                                                                      */
c! { pub(in foo) use A as I; }                                                                                                            /*
c!•{•pub(in•foo)•use•A•as•I;•}    ExpressionStatement, MacroInvocation
        (in•foo)                  DelimGroup
                           ;      PunctuationToken                                                                                        */
c! { pub(crate) struct A { pub a: i32, b: i32, pub(crate) c: i32 } }                                                                      /*
c!•{•pub(crate)•struct•A•{•pub•a:•i32,•b:•i32,•pub(crate)•c:•i32•}•}    ExpressionStatement, MacroInvocation
        (crate)                                                         DelimGroup
                         {•pub•a:•i32,•b:•i32,•pub(crate)•c:•i32•}      DelimGroup
                                :                                       PunctuationToken
                                     ,                                  PunctuationToken
                                        :                               PunctuationToken
                                             ,                          PunctuationToken
                                                  (crate)               DelimGroup
                                                           :            PunctuationToken                                                  */
c! { pub struct B { a: i32, pub(crate) b: i32, pub c: i32 } }                                                                             /*
c!•{•pub•struct•B•{•a:•i32,•pub(crate)•b:•i32,•pub•c:•i32•}•}    ExpressionStatement, MacroInvocation
                  {•a:•i32,•pub(crate)•b:•i32,•pub•c:•i32•}      DelimGroup
                     :                                           PunctuationToken
                          ,                                      PunctuationToken
                               (crate)                           DelimGroup
                                        :                        PunctuationToken
                                             ,                   PunctuationToken
                                                    :            PunctuationToken                                                         */
c! { struct C { pub(crate) a: i32, pub b: i32, c: i32 } }                                                                                 /*
c!•{•struct•C•{•pub(crate)•a:•i32,•pub•b:•i32,•c:•i32•}•}    ExpressionStatement, MacroInvocation
              {•pub(crate)•a:•i32,•pub•b:•i32,•c:•i32•}      DelimGroup
                   (crate)                                   DelimGroup
                            :                                PunctuationToken
                                 ,                           PunctuationToken
                                        :                    PunctuationToken
                                             ,               PunctuationToken
                                                :            PunctuationToken                                                             */

c! { pub(crate) struct D (pub i32, i32, pub(crate) i32); }                                                                                /*
c!•{•pub(crate)•struct•D•(pub•i32,•i32,•pub(crate)•i32);•}    ExpressionStatement, MacroInvocation
        (crate)                                               DelimGroup
                         (pub•i32,•i32,•pub(crate)•i32)       DelimGroup
                                 ,                            PunctuationToken
                                      ,                       PunctuationToken
                                           (crate)            DelimGroup
                                                       ;      PunctuationToken                                                            */
c! { pub struct E (i32, pub(crate) i32, pub i32); }                                                                                       /*
c!•{•pub•struct•E•(i32,•pub(crate)•i32,•pub•i32);•}    ExpressionStatement, MacroInvocation
                  (i32,•pub(crate)•i32,•pub•i32)       DelimGroup
                      ,                                PunctuationToken
                           (crate)                     DelimGroup
                                      ,                PunctuationToken
                                                ;      PunctuationToken                                                                   */
c! { struct F (pub(crate) i32, pub i32, i32); }                                                                                           /*
c!•{•struct•F•(pub(crate)•i32,•pub•i32,•i32);•}    ExpressionStatement, MacroInvocation
              (pub(crate)•i32,•pub•i32,•i32)       DelimGroup
                  (crate)                          DelimGroup
                             ,                     PunctuationToken
                                      ,            PunctuationToken
                                            ;      PunctuationToken                                                                       */

c!(pub(crate);                                                                                                                            /*
c!(pub(crate);↲    <ExpressionStatement>, <MacroInvocation>
      (crate)      DelimGroup
             ;     PunctuationToken                                                                                                       */
c!(pub(self)));                                                                                                                           /*
c!(pub(self)));    </ExpressionStatement>
c!(pub(self)))     </MacroInvocation>
 !                 PunctuationToken
  (pub(self))      DelimGroup
      (self)       DelimGroup                                                                                                             */
c!(pub(super));                                                                                                                           /*
c!(pub(super));    ExpressionStatement
c!(pub(super))     MacroInvocation
      (super)      DelimGroup                                                                                                             */
c!(pub(in self));                                                                                                                         /*
c!(pub(in•self));    ExpressionStatement
c!(pub(in•self))     MacroInvocation
      (in•self)      DelimGroup                                                                                                           */
c!(pub(in super));                                                                                                                        /*
c!(pub(in•super));    ExpressionStatement
c!(pub(in•super))     MacroInvocation
      (in•super)      DelimGroup                                                                                                          */
c!(pub(in path::to));                                                                                                                     /*
c!(pub(in•path::to));    ExpressionStatement
c!(pub(in•path::to))     MacroInvocation
      (in•path::to)      DelimGroup
              ::         PunctuationToken                                                                                                 */
c!(pub(in ::path::to));                                                                                                                   /*
c!(pub(in•::path::to));    ExpressionStatement
c!(pub(in•::path::to))     MacroInvocation
      (in•::path::to)      DelimGroup
          ::               PunctuationToken
                ::         PunctuationToken                                                                                               */
c!(pub(in self::path::to));                                                                                                               /*
c!(pub(in•self::path::to));    ExpressionStatement
c!(pub(in•self::path::to))     MacroInvocation
      (in•self::path::to)      DelimGroup
              ::               PunctuationToken
                    ::         PunctuationToken                                                                                           */
c!(pub(in super::path::to));                                                                                                              /*
c!(pub(in•super::path::to));    ExpressionStatement
c!(pub(in•super::path::to))     MacroInvocation
      (in•super::path::to)      DelimGroup
               ::               PunctuationToken
                     ::         PunctuationToken                                                                                          */

// Discarded Nodes: 0
// Parsed Nodes: 3179
// state_rollbacks: 6
// Total '.charCodeAt()' calls: 20595 (18% re-reads)
// Unnecessary 'skip_whitespace()' calls: 1122
// source: "../../samples/macro/macro.invocation.rs"