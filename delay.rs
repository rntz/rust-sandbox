fn convert<T:Send>(f: fn~() -> T) -> ~fn() -> T {
  move f
}

macro_rules! delay(
  ($e:expr) => ( convert(|| $e) )
)

fn main() {
  let x: fn~() -> ~str = delay!(~"foo" + "bar");
  io::println(x().to_str())
}
