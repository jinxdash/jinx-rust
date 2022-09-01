fn a() {                                                                                                                                  /*
fn•a()•{↲    <Program>
fn•a()•{↲    <Program.ast{dk: "None"}>
fn•a()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	if 5u64 as i32 as u16 == 0u16 {}                                                                                                      /*
	if•5u64•as•i32•as•u16•==•0u16•{}    ExpressionStatement{!semi}, IfBlockExpression
	   5u64•as•i32•as•u16•==•0u16       ComparisonExpression{tk: "=="}
	   5u64•as•i32•as•u16               ExpressionAsTypeCast
	   5u64•as•i32                      ExpressionAsTypeCast
	   5u64                             Literal{kind: Integer}
	    u64                             Identifier
	                         0u16       Literal{kind: Integer}
	                          u16       Identifier
	                              {}    IfBlockExpression.body{dk: "{}"}                                                                  */
	[                                                                                                                                     /*
	[↲    <ExpressionStatement{!semi}>
	[↲    <ArrayLiteral>                                                                                                                  */
		u64 as u8 as i8 == 9i8,                                                                                                           /*
		u64•as•u8•as•i8•==•9i8    ComparisonExpression{tk: "=="}
		u64•as•u8•as•i8           ExpressionAsTypeCast
		u64•as•u8                 ExpressionAsTypeCast
		                   9i8    Literal{kind: Integer}
		                    i8    Identifier                                                                                              */
		&([1,2,3]) as *const _ as *const [i32; 3],                                                                                        /*
		&([1,2,3])•as•*const•_•as•*const•[i32;•3]    ExpressionAsTypeCast
		&([1,2,3])•as•*const•_                       ExpressionAsTypeCast
		&([1,2,3])                                   ReferenceExpression{!mut}
		  [1,2,3]                                    ArrayLiteral
		   1                                         Literal{kind: Integer}
		     2                                       Literal{kind: Integer}
		       3                                     Literal{kind: Integer}
		              *const•_                       TypeDereferenceConst
		                     _                       TypeInferred
		                          *const•[i32;•3]    TypeDereferenceConst
		                                 [i32;•3]    TypeSizedArray
		                                       3     Literal{kind: Integer}                                                               */
		(-0i16) as i8,                                                                                                                    /*
		(-0i16)•as•i8    ExpressionAsTypeCast
		 -0i16           MinusExpression
		  0i16           Literal{kind: Integer}
		   i16           Identifier                                                                                                       */
		(!0u16) as u8,                                                                                                                    /*
		(!0u16)•as•u8    ExpressionAsTypeCast
		 !0u16           NotExpression
		  0u16           Literal{kind: Integer}
		   u16           Identifier                                                                                                       */
		(0u16 << 15) as u8,                                                                                                               /*
		(0u16•<<•15)•as•u8    ExpressionAsTypeCast
		 0u16•<<•15           OperationExpression{tk: "<<"}
		 0u16                 Literal{kind: Integer}
		  u16                 Identifier
		         15           Literal{kind: Integer}                                                                                      */
		(0u32 << 31) as u16,                                                                                                              /*
		(0u32•<<•31)•as•u16    ExpressionAsTypeCast
		 0u32•<<•31            OperationExpression{tk: "<<"}
		 0u32                  Literal{kind: Integer}
		  u32                  Identifier
		         31            Literal{kind: Integer}                                                                                     */
		(Foo::Bar) as i8,                                                                                                                 /*
		(Foo::Bar)•as•i8    ExpressionAsTypeCast
		 Foo::Bar           ExpressionPath                                                                                                */
		0 as i32 as i32,                                                                                                                  /*
		0•as•i32•as•i32    ExpressionAsTypeCast
		0•as•i32           ExpressionAsTypeCast
		0                  Literal{kind: Integer}                                                                                         */
		// 0 as i32: i32,
		//•0•as•i32:•i32,    Comment{line}
		// 0i32: i32 as i32,
		//•0i32:•i32•as•i32,    Comment{line}
		// 0i32: i32: i32 as u32 as i32,
		//•0i32:•i32:•i32•as•u32•as•i32,    Comment{line}
		// 0i32: i32: i32,
		//•0i32:•i32:•i32,    Comment{line}
		0u8 as u32,                                                                                                                       /*
		0u8•as•u32    ExpressionAsTypeCast
		0u8           Literal{kind: Integer}
		 u8           Identifier                                                                                                          */
		a as fn(u8),                                                                                                                      /*
		a•as•fn(u8)    ExpressionAsTypeCast
		     fn(u8)    TypeFnPointer
		       (u8)    TypeFnPointer.parameters{dk: "()"}
		        u8     TypeFnPointerParameter                                                                                             */
		// <T as Foo>::Assoc<3>,
		//•<T•as•Foo>::Assoc<3>,    Comment{line}
		drop as fn(u8),                                                                                                                   /*
		drop•as•fn(u8)    ExpressionAsTypeCast
		        fn(u8)    TypeFnPointer
		          (u8)    TypeFnPointer.parameters{dk: "()"}
		           u8     TypeFnPointerParameter                                                                                          */
		&x as *const _,                                                                                                                   /*
		&x•as•*const•_    ExpressionAsTypeCast
		&x                ReferenceExpression{!mut}
		      *const•_    TypeDereferenceConst
		             _    TypeInferred                                                                                                    */
		Box::new(A) as &dyn B<C=usize>,                                                                                                   /*
		Box::new(A)•as•&dyn•B<C=usize>    ExpressionAsTypeCast
		Box::new(A)                       CallExpression
		Box::new                          ExpressionPath
		        (A)                       CallExpression.arguments{dk: "()"}
		               &dyn•B<C=usize>    TypeReference{!mut}
		                dyn•B<C=usize>    TypeDynBounds{dyn}
		                    B<C=usize>    TypeTraitBound{!maybeConst, !optional}, TypeCall
		                     <C=usize>    TypeCall.typeArguments{dk: "<>"}
		                      C=usize     TypeCallNamedArgument                                                                           */
		(box move |y: i32| -> i32 { x + y }) as Box<dyn FnMut(i32)->i32+'static>,                                                         /*
		(box•move•|y:•i32|•->•i32•{•x•+•y•})•as•Box<dyn•FnMut(i32)->i32+'static>    ExpressionAsTypeCast
		 box•move•|y:•i32|•->•i32•{•x•+•y•}                                         BoxExpression
		     move•|y:•i32|•->•i32•{•x•+•y•}                                         ClosureFunctionExpression{move}
		          |y:•i32|                                                          ClosureFunctionExpression.parameters{dk: "||"}
		           y:•i32                                                           ClosureFunctionParameterDeclaration
		                          {•x•+•y•}                                         BlockExpression
		                            x•+•y                                           ExpressionStatement{!semi}, OperationExpression{tk: "+"}
		                                        Box<dyn•FnMut(i32)->i32+'static>    TypeCall
		                                           <dyn•FnMut(i32)->i32+'static>    TypeCall.typeArguments{dk: "<>"}
		                                            dyn•FnMut(i32)->i32+'static     TypeDynBounds{dyn}
		                                                FnMut(i32)->i32             TypeTraitBound{!maybeConst, !optional}, TypeFunction
		                                                     (i32)                  TypeFunction.parameters{dk: "()"}
		                                                                'static     LtStatic                                              */
		((&x) as *const i16) as f32,                                                                                                      /*
		((&x)•as•*const•i16)•as•f32    ExpressionAsTypeCast
		 (&x)•as•*const•i16            ExpressionAsTypeCast
		  &x                           ReferenceExpression{!mut}
		         *const•i16            TypeDereferenceConst                                                                               */
		(&|_| ()) as &dyn for<'x> Fn(<u32 as T<'x>>::V),                                                                                  /*
		(&|_|•())•as•&dyn•for<'x>•Fn(<u32•as•T<'x>>::V)    ExpressionAsTypeCast
		 &|_|•()                                           ReferenceExpression{!mut}
		  |_|•()                                           ClosureFunctionExpression
		  |_|                                              ClosureFunctionExpression.parameters{dk: "||"}
		   _                                               ClosureFunctionParameterDeclaration, WildcardPattern
		      ()                                           TupleLiteral
		             &dyn•for<'x>•Fn(<u32•as•T<'x>>::V)    TypeReference{!mut}
		              dyn•for<'x>•Fn(<u32•as•T<'x>>::V)    TypeDynBounds{dyn}
		                  for<'x>•Fn(<u32•as•T<'x>>::V)    TypeTraitBound{!maybeConst, !optional}
		                  for<'x>                          TypeTraitBound.ltParameters{dk: "<>"}
		                      'x                           GenericLtParameterDeclaration, LtIdentifier
		                          Fn(<u32•as•T<'x>>::V)    TypeFunction
		                            (<u32•as•T<'x>>::V)    TypeFunction.parameters{dk: "()"}
		                             <u32•as•T<'x>>::V     TypePath
		                             <u32•as•T<'x>>        ExpressionTypeSelector
		                                     T<'x>         TypeCall
		                                      <'x>         TypeCall.typeArguments{dk: "<>"}
		                                       'x          LtIdentifier                                                                   */
		TestStruct{x: 0x1234 as *const [isize; 2]},                                                                                       /*
		TestStruct{x:•0x1234•as•*const•[isize;•2]}    StructLiteral
		          {x:•0x1234•as•*const•[isize;•2]}    StructLiteral.properties{dk: "{}"}
		           x:•0x1234•as•*const•[isize;•2]     StructLiteralProperty
		              0x1234•as•*const•[isize;•2]     ExpressionAsTypeCast
		              0x1234                          Literal{kind: Hex}
		                        *const•[isize;•2]     TypeDereferenceConst
		                               [isize;•2]     TypeSizedArray
		                                       2      Literal{kind: Integer}                                                              */
		!(FOO as *const usize).a(),                                                                                                       /*
		!(FOO•as•*const•usize).a()    NotExpression
		 (FOO•as•*const•usize).a()    CallExpression
		  FOO•as•*const•usize         ExpressionAsTypeCast
		         *const•usize         TypeDereferenceConst
		                        ()    CallExpression.arguments{dk: "()"}                                                                  */
		!(42 as *const usize).a(),                                                                                                        /*
		!(42•as•*const•usize).a()    NotExpression
		 (42•as•*const•usize).a()    CallExpression
		  42•as•*const•usize         ExpressionAsTypeCast
		  42                         Literal{kind: Integer}
		        *const•usize         TypeDereferenceConst
		                       ()    CallExpression.arguments{dk: "()"}                                                                   */
		(0 as *const usize).a(),                                                                                                          /*
		(0•as•*const•usize).a()    CallExpression
		 0•as•*const•usize         ExpressionAsTypeCast
		 0                         Literal{kind: Integer}
		      *const•usize         TypeDereferenceConst
		                     ()    CallExpression.arguments{dk: "()"}                                                                     */
		!("foo" as *const str).a(),                                                                                                       /*
		!("foo"•as•*const•str).a()    NotExpression
		 ("foo"•as•*const•str).a()    CallExpression
		  "foo"•as•*const•str         ExpressionAsTypeCast
		  "foo"                       Literal{kind: String}
		           *const•str         TypeDereferenceConst
		                        ()    CallExpression.arguments{dk: "()"}                                                                  */
		&x as T[0],                                                                                                                       /*
		&x•as•T[0]    MemberExpression{computed}
		&x•as•T       ExpressionAsTypeCast
		&x            ReferenceExpression{!mut}
		        0     Literal{kind: Integer}                                                                                              */
	]                                                                                                                                     /*
   ╚]    </ArrayLiteral>
   ╚]    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
const A: *const u8 = &0 as *const _ as *const Q as *const u8;                                                                             /*
const•A:•*const•u8•=•&0•as•*const•_•as•*const•Q•as•*const•u8;    ConstVariableDeclaration
         *const•u8                                               TypeDereferenceConst
                     &0•as•*const•_•as•*const•Q•as•*const•u8     ExpressionAsTypeCast
                     &0•as•*const•_•as•*const•Q                  ExpressionAsTypeCast
                     &0•as•*const•_                              ExpressionAsTypeCast
                     &0                                          ReferenceExpression{!mut}
                      0                                          Literal{kind: Integer}
                           *const•_                              TypeDereferenceConst
                                  _                              TypeInferred
                                       *const•Q                  TypeDereferenceConst
                                                   *const•u8     TypeDereferenceConst
const•A:•*const•u8•=•&0•as•*const•_•as•*const•Q•as•*const•u8;    </Program.ast>
const•A:•*const•u8•=•&0•as•*const•_•as•*const•Q•as•*const•u8;    </Program>                                                               */
// Discarded Nodes: 14
// Parsed Nodes: 214
// state_rollbacks: 2
// Total '.charCodeAt()' calls: 1163 (35% re-reads)
// Unnecessary 'skip_whitespace()' calls: 138
// source: "../../samples/types/cast.rs"