struct Foo { x: int }

fn main() {
  let _ = Foo { x: 0, ..fail };
}
