#![feature(const_trait_impl)]

impl const T for S {}
impl<T> const T for S {}


impl const T for S {
	#![attr]
}

#[attr_0]
impl const T for S {
	#![attr_1] // comment
}

fn foo() -> u8 where Self: ~const Bar {}
struct S {
    D: ~const ?Q, 
    E: ~const Q + 'a, 
    F: ~const Q, 
}

struct S<
    T: ~const ?for<'a> Tr<'a> + 'static + ~const std::ops::Add,
    T: ~const ?for<'a: 'b> m::Trait<'a>,
>;
trait F {
	fn bar() where Self: ~const Foo;
    fn c<T: ~const Bar>();
}
const fn qux<T: ~const Foo>() {}
const fn test1<T: ~const Foo + Bar>(){}
const fn test2<T: ~const Foo + ~const Bar>() {}