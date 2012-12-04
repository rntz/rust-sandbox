fn main() {}

trait Flub {
  static pure fn foo(uint, self) -> uint;
  static pure fn bar(x: self) -> uint { foo(0, x) }
}
