#![feature(generators)]

[
	static move || { let a = A::<Box<dyn D>> { b: E::r(), }; yield (); },
	|_| { a!("-> {}", yield); },
]