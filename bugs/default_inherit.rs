fn main() {}

trait Foo {
  pure fn foo(&self) -> int;
}

trait Bar: Foo {
  pure fn bar(&self) -> uint { self.foo() }
}
