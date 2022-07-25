type funky_func = extern "C" fn (unsafe extern "rust-call" fn(*const JSJitInfo, *mut JSContext, HandleObject, *mut libc::c_void, u32, *mut JSVal) -> u8);/*
type•funky_func•=•extern•"C"•fn•(unsafe•extern•"rust-call"•fn(*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)•->•u8);    TypeAliasDeclaration
                  extern•"C"•fn•(unsafe•extern•"rust-call"•fn(*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)•->•u8)     TypeFnPointer
                  extern•"C"                                                                                                                                 ExternSpecifier
                         "C"                                                                                                                                 Literal
                                 unsafe•extern•"rust-call"•fn(*const•JSJitInfo,•*mut•JSContext,•HandleObject,•*mut•libc::c_void,•u32,•*mut•JSVal)•->•u8      TypeFnPointerParameter, TypeFnPointer
                                        extern•"rust-call"                                                                                                   ExternSpecifier
                                               "rust-call"                                                                                                   Literal
                                                              *const•JSJitInfo                                                                               TypeFnPointerParameter, TypeDereferenceConst
                                                                                *mut•JSContext                                                               TypeFnPointerParameter, TypeDereferenceMut
                                                                                                HandleObject                                                 TypeFnPointerParameter
                                                                                                              *mut•libc::c_void                              TypeFnPointerParameter, TypeDereferenceMut
                                                                                                                   libc::c_void                              TypePath
                                                                                                                                 u32                         TypeFnPointerParameter
                                                                                                                                      *mut•JSVal             TypeFnPointerParameter, TypeDereferenceMut*/
extern "C" fn sup() { }                                                                                                                   /*
extern•"C"•fn•sup()•{•}    FunctionDeclaration
extern•"C"                 ExternSpecifier
       "C"                 Literal                                                                                                        */
extern  "C" { fn some_fn() -> (); }                                                                                                       /*
extern••"C"•{•fn•some_fn()•->•();•}    ExternBlockDeclaration
        "C"                            Literal
              fn•some_fn()•->•();      FunctionDeclaration
                              ()       TypeTuple                                                                                          */
extern {                                                                                                                                  /*
extern•{↲    <ExternBlockDeclaration>                                                                                                     */
	fn quux() -> (); // Post comment
                                                                                                                                          /*
    fn•quux()•->•();                    FunctionDeclaration
                 ()                     TypeTuple
                     //•Post•comment    Comment                                                                                           */
	fn syscall(number: libc::c_long /* comment 1 */, /* comm 2 */ ... /* sup? */) -> libc::c_long;                                        /*
    fn•syscall(number:•libc::c_long•/*•comment•1•*/,•/*•comm•2•*/•...•/*•sup?•*/)•->•libc::c_long;    FunctionDeclaration
               number:•libc::c_long                                                                   FunctionParameterDeclaration
                       libc::c_long                                                                   TypePath
                                    /*•comment•1•*/                                                   Comment
                                                     /*•comm•2•*/                                     Comment
                                                                  ...                                 FunctionSpread
                                                                      /*•sup?•*/                      Comment
                                                                                     libc::c_long     TypePath                            */
	unsafe fn foo( ) -> * mut Bar;                                                                                                        /*
    unsafe•fn•foo(•)•->•*•mut•Bar;    FunctionDeclaration
                        *•mut•Bar     TypeDereferenceMut                                                                                  */
	pub(super) const fn foo() -> *mut Bar;                                                                                                /*
    pub(super)•const•fn•foo()•->•*mut•Bar;    FunctionDeclaration
    pub(super)                                PubSpecifier
                                 *mut•Bar     TypeDereferenceMut                                                                          */
	pub(crate) unsafe fn foo() -> *mut Bar;                                                                                               /*
    pub(crate)•unsafe•fn•foo()•->•*mut•Bar;    FunctionDeclaration
    pub(crate)                                 PubSpecifier
                                  *mut•Bar     TypeDereferenceMut                                                                         */
}                                                                                                                                         /*
}    </ExternBlockDeclaration>                                                                                                            */

// Discarded Nodes: 0
// Parsed Nodes: 74
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 595 (27% re-reads)
// Unnecessary 'skip_whitespace()' calls: 45
// source: "../../samples/specifiers/extern.rs"