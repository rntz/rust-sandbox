macro_rules! matches(
  ($e:expr, $p:pat) => (
    match $e { $p => true, _ => false }
  )
)

fn main() {
  assert  matches!(2, 2..3);
  assert !matches!(4, 2..3);
}
