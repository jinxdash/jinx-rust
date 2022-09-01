type funky_func = extern "C" fn (unsafe extern "rust-call" fn(*const JSJitInfo, *mut JSContext, HandleObject, *mut libc::c_void, u32, *mut JSVal) -> u8);/*
type•funky_func•=•extern•"C"•fn•(unsafe•extern•"rust-call"•fn(*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)•->•u8);↲    <Program>
type•funky_func•=•extern•"C"•fn•(unsafe•extern•"rust-call"•fn(*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)•->•u8);↲    <Program.ast{dk: "None"}>
type•funky_func•=•extern•"C"•fn•(unsafe•extern•"rust-call"•fn(*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)•->•u8);     TypeAliasDeclaration
                  extern•"C"•fn•(unsafe•extern•"rust-call"•fn(*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)•->•u8)      TypeFnPointer
                  extern•"C"                                                                                                                                  ExternSpecifier
                         "C"                                                                                                                                  Literal{kind: String}
                                (unsafe•extern•"rust-call"•fn(*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)•->•u8)      TypeFnPointer.parameters{dk: "()"}
                                 unsafe•extern•"rust-call"•fn(*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)•->•u8       TypeFnPointerParameter, TypeFnPointer{unsafe}
                                        extern•"rust-call"                                                                                                    ExternSpecifier
                                               "rust-call"                                                                                                    Literal{kind: String}
                                                             (*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)             TypeFnPointer.parameters{dk: "()"}
                                                              *const•JSJitInfo                                                                                TypeFnPointerParameter, TypeDereferenceConst
                                                                                *mut•JSContext                                                                TypeFnPointerParameter, TypeDereferenceMut
                                                                                                HandleObject                                                  TypeFnPointerParameter
                                                                                                              *mut•libc::c_void                               TypeFnPointerParameter, TypeDereferenceMut
                                                                                                                   libc::c_void                               TypePath
                                                                                                                                 u32                          TypeFnPointerParameter
                                                                                                                                      *mut•JSVal              TypeFnPointerParameter, TypeDereferenceMut*/
extern "C" fn sup() { }                                                                                                                   /*
extern•"C"•fn•sup()•{•}    FunctionDeclaration
extern•"C"                 ExternSpecifier
       "C"                 Literal{kind: String}
                 ()        FunctionDeclaration.parameters{dk: "()"}
                    {•}    FunctionDeclaration.body{dk: "{}"}                                                                             */
extern  "C" { fn some_fn() -> (); }                                                                                                       /*
extern••"C"•{•fn•some_fn()•->•();•}    ExternBlockDeclaration
        "C"                            Literal{kind: String}
            {•fn•some_fn()•->•();•}    ExternBlockDeclaration.body{dk: "{}"}
              fn•some_fn()•->•();      FunctionDeclaration
                        ()             FunctionDeclaration.parameters{dk: "()"}
                              ()       TypeTuple                                                                                          */
extern {                                                                                                                                  /*
extern•{↲    <ExternBlockDeclaration>
       {↲    <ExternBlockDeclaration.body{dk: "{}"}>                                                                                      */
	fn quux() -> (); // Post comment
/*	fn•quux()•->•();                    FunctionDeclaration
	       ()                           FunctionDeclaration.parameters{dk: "()"}
	             ()                     TypeTuple                                                                                         */
	                 //•Post•comment    Comment{line}
	fn syscall(number: libc::c_long /* comment 1 */, /* comm 2 */ ... /* sup? */) -> libc::c_long;                                        /*
	fn•syscall(number:•libc::c_long•/*•comment•1•*/,•/*•comm•2•*/•...•/*•sup?•*/)•->•libc::c_long;    FunctionDeclaration
	          (number:•libc::c_long•/*•comment•1•*/,•/*•comm•2•*/•...•/*•sup?•*/)                     FunctionDeclaration.parameters{dk: "()"}
	           number:•libc::c_long                                                                   FunctionParameterDeclaration
	                   libc::c_long                                                                   TypePath
	                                /*•comment•1•*/                                                   Comment{!line}
	                                                 /*•comm•2•*/                                     Comment{!line}
	                                                              ...                                 FunctionSpread
	                                                                  /*•sup?•*/                      Comment{!line}
	                                                                                 libc::c_long     TypePath                            */
	unsafe fn foo( ) -> * mut Bar;                                                                                                        /*
	unsafe•fn•foo(•)•->•*•mut•Bar;    FunctionDeclaration{unsafe}
	             (•)                  FunctionDeclaration.parameters{dk: "()"}
	                    *•mut•Bar     TypeDereferenceMut                                                                                  */
	pub(super) const fn foo() -> *mut Bar;                                                                                                /*
	pub(super)•const•fn•foo()•->•*mut•Bar;    FunctionDeclaration{const}
	pub(super)                                PubSpecifier
	                       ()                 FunctionDeclaration.parameters{dk: "()"}
	                             *mut•Bar     TypeDereferenceMut                                                                          */
	pub(crate) unsafe fn foo() -> *mut Bar;                                                                                               /*
	pub(crate)•unsafe•fn•foo()•->•*mut•Bar;    FunctionDeclaration{unsafe}
	pub(crate)                                 PubSpecifier
	                        ()                 FunctionDeclaration.parameters{dk: "()"}
	                              *mut•Bar     TypeDereferenceMut                                                                         */
}                                                                                                                                         /*
}    </ExternBlockDeclaration.body>
}    </ExternBlockDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 74
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 595 (27% re-reads)
// Unnecessary 'skip_whitespace()' calls: 45
// source: "../../samples/specifiers/extern.rs"