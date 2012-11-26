iface A { fn foo() -> bool; }
iface B { fn foo() -> int; }

// These compile
fn aa<T:A B>(x: T) -> bool { x.foo() }
fn bb<T:B A>(x: T) -> int { x.foo() }

// These don't
fn ab<T:A B>(x: T) -> int { x.foo() }
fn ba<T:B A>(x: T) -> bool { x.foo() }

// As for implementations
impl of A for int { fn foo() -> bool { self > 0 } }
impl of B for int { fn foo() -> int { self } }

fn foo(x: int) -> bool { x.foo() }

fn main() {}
