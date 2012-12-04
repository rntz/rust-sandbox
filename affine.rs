// Playing around with affine typs.

trait Test {
  static fn s_add(int, self) -> self;
  fn m_add(&mut self, int);
  fn i_add(self, int) -> self;
}

impl ~[int]: Test {
  static fn s_add(x: int, v_: ~[int]) -> ~[int] {
    let mut v = move v_;
    v.push(x);
    move v
  }

  fn m_add(&mut self, x: int) {
    self.push(x)
  }

  fn i_add(self, _x: int) -> ~[int] {
    fail;
    // // Can't move out of self yet, so this doesn't work.
    // self.push(x);
    // move self
  }
}

impl @[int]: Test {
  static fn s_add(x: int, v: @[int]) -> @[int] { v + [x] }
  fn m_add(&mut self, x: int) { *self = *self + [x]; }
  fn i_add(self, x: int) -> @[int] { self + [x] }
}

fn main() {}
