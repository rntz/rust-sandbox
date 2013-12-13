macro_rules! matches(
  ($e:expr ~= $p:pat) => (
    match $e { $p => true, _ => false }
  )
)

macro_rules! and(
  ($($e:expr),*) => ($($e &&)* true)
)

macro_rules! case(
  ($e:expr, $p:pat $(where $cond:expr)* => $thn:expr, $els:expr) => (
    // FIXME: using option::Some here is an awful (and potentially
    // inefficiency-introducing) hack to make sure the underscore case isn't
    // "unreachable".
    match Some($e) {
      Some($p) => if and!($($cond),*) { $thn } else { $els },
      _ => $els
    }
  )
)

macro_rules! unwrap_or_return(
  ($e:expr, $re:expr) => (
    match $e {
      Some(x) => x,
      _ => return $re
    }
  )
)

fn main() {
  assert!( matches!(2 ~= 2..3));
  assert!(!matches!(4 ~= 2..3));
  assert!( and!(true, true, true));
  assert!(!and!(true, false));
  assert!(5 == case!(5, x where x > 3 => x, 3));
  assert!(3 == case!(2, x where x > 3 => x, 3));
  assert!(case!(4, x@1..10 where x % 2 == 0 => true, false));
}
