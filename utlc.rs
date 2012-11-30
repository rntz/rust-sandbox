// interpreter for call-by-value untyped lambda calculus.
use io::{Reader,ReaderUtil,Writer,WriterUtil};
use to_str::{ToStr};
use option::{Option,map_consume};
use char::{is_whitespace,is_digit};
use tuple::{CopyableTuple};

// binding is encoded using DeBruijn indices
enum Term {
  Var(uint),
  Lam(@Term),
  App(@Term, @Term),
}

impl Term: ToStr {
  pure fn to_str() -> ~str {
    match self {
      Var(i) => i.to_str(),
      Lam(@body) => ~"\\" + body.to_str(),
      App(e1, e2) => {
        pure fn parg(e: &Term) -> ~str {
          match *e {
            Lam(_) => ~"(" + (*e).to_str() + ")",
            _ => (*e).to_str()
          }
        }

        pure fn papp(e1: &Term, e2: &Term) -> ~str {
          fmt!("%s %s",
               match *e1 {
                 App(x,y) => papp(x,y),
                 _ => parg(e1),
               },
               parg(e2))
        }

        papp(e1,e2)
      }
    }
  }
}

fn term_to_str(t: @Term) -> ~str { (*t).to_str() }

fn term_from_str(s: ~str) -> Option<@Term> {

  fn puint(s: &[char], i: uint) -> Option<(uint,uint)> {
    let mut j = i;
    assert i < s.len() && is_digit(s[i]);
    while j < s.len() && is_digit(s[j]) { j += 1; }
    do map_consume(uint::from_str(str::from_chars(vec::slice(s, i, j)))) |x| {
      (x,j)
    }
  }

  fn papp(s: &[char], i_: uint) -> Option<(@Term, uint)> {
    let mut (e, i) = match pterm(s, i_) {
      Some(x) => x,
      None => { return None; }
    };
    loop {
      match pterm(s,i) {
        None => { return Some((e,i)); }
        Some((er, j)) => {
          e = @App(e, er);
          i = j;
        }
      }
    }
  }

  fn pterm(s: &[char], i_: uint) -> Option<(@Term,uint)> {
    let mut i = i_;
    while i < s.len() && is_whitespace(s[i]) { i += 1; }
    if i >= s.len() { return None; }

    let c = s[i];
    match c {
      '\\' => do map_consume(papp(s, i+1)) |(x,y)| { (@Lam(x), y) },
      '0'..'9' => do map_consume(puint(s,i)) |(x,y)| { (@Var(x), y) },
      '(' => match papp(s,i+1) {
        None => None,
        Some((e,j_)) => {
          let mut j = j_;
          while j < s.len() && is_whitespace(s[j]) { j += 1; }
          if j >= s.len() || s[j] != ')' { None }
          else { Some((e,j+1)) }
        }
      },
      ')' => None,
      _ => fail ~"unrecognized character",
    }
  }

  match papp(str::chars(s), 0) {
    Some((t,_)) => Some(t),
    None => None,
  }
}

// For closure-based interpretation
enum Val {
  Closure(~[@Val], @Term),
}

impl Val: ToStr {
  pure fn to_str() -> ~str {
    match self {
      Closure(cx, term) => fmt!("Closure(%s, %s)", cx.to_str(), term.to_str()),
    }
  }
}

// Evaluation
fn evaluate(t: @Term) -> @Val {
  let x = ~[];
  let r = eval(x, t);
  log(info, x);
  r
}

fn lookup(cx: &[@Val], v: uint) -> @Val { cx[cx.len() - v - 1] }
fn extend(cx: ~[@Val], v: @Val) -> ~[@Val] {
  let mut r = move cx;
  r.push(v);
  move r
}

fn eval(cx: &[@Val], t: @Term) -> @Val {
  match *t {
    Var(v) => lookup(cx, v),
    // is there a better way to copy cx?
    Lam(body) => @Closure(vec::from_slice(cx), body),
    App(e1, e2) => {
      let @Closure(fn_ctx, body) = eval(cx, e1);
      let arg = eval(cx, e2);
      eval(extend(move fn_ctx, arg), body)
    }
  }
}

fn main() {
  loop {
    let s = io::stdin().read_line();
    let t = match term_from_str(move s) {
      Some(t) => t,
      None => {break}
    };
    let v = evaluate(t);
    io::println(v.to_str());
  }
}
