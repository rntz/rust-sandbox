// interpreter for call-by-value untyped lambda calculus.
use io::{Reader,ReaderUtil,Writer,WriterUtil};
use to_str::{ToStr};
use option::{Option,map_consume};
use char::{is_whitespace,is_digit};

// ---------- Untyped lambda calculus language ----------
enum Term {
  Var(uint),
  Lam(@Term),
  App(@Term, @Term),
}

// ===== Stringification
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

pub fn is_val(t: @Term) -> bool {
  match *t { Lam(_) => true, _ => false }
}

fn term_to_str(t: @Term) -> ~str { (*t).to_str() }

// ===== Parsing
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


// ---------- Substitution-based evaluation ----------
mod subst {
  // Substitutes `t' for 0 in `body'.
  pub fn subst(t: @Term, body: @Term) -> @Term { subst_for(0, t, body) }

  // Substitutes `t' for `var' in `body'.
  pub fn subst_for(var: uint, t: @Term, body: @Term) -> @Term {
    match *body {
      Var(i) => if i == var {t} else {@Var(if i < var {i} else {i-1})},
      Lam(exp) => @Lam(subst_for(var+1, lift(t), exp)),
      App(e1, e2) => @App(subst_for(var, t, e1), subst_for(var, t, e2)),
    }
  }

  pub fn lift(t: @Term) -> @Term { lift_from(0, t) }

  pub fn lift_from(from: uint, t: @Term) -> @Term {
    @match *t {
      Var(i) => Var(if i < from {i} else {i+1}),
      Lam(e) => Lam(lift_from(from+1, e)),
      App(e1, e2) => App(lift_from(from, e1), lift_from(from, e2)),
    }
  }

  pub enum Step {
    Steps(@Term),   // Steps(t) where t is the term it steps to
    IsLam(@Term),   // IsLam(body) where body is the body of the Lam it steps to
  }

  pub fn step(t: @Term) -> Step {
    match *t {
      Var(_) => fail ~"cannot step open terms",
      Lam(body) => IsLam(body),
      App(e1, e2) => Steps(match step(e1) {
        Steps(e1s) => @App(e1s, e2),
        IsLam(body) => match step(e2) {
          Steps(e2s) => @App(e1, e2s),
          IsLam(_) => subst(e2, body),
        },
      }),
    }
  }

  pub fn eval(t: @Term) -> @Term {
    match *t {
      Var(_) => t,
      Lam(_) => t,
      App(e1, e2) => {
        match (eval(e1), eval(e2)) {
          (@Lam(body), e2v) => eval(subst(e2v, body)),
          // cannot evaluate further for some reason (eg. got a free var in
          // application position)
          (e1v, e2v) => @App(e1v, e2v),
        }
      }
    }
  }
}

// ---------- Closure-based evaluation ----------
mod closure {
  pub enum Val {
    Closure(~[@Val], @Term),
  }

  pub impl Val: ToStr {
    pure fn to_str() -> ~str {
      match self {
        Closure(cx, term) => fmt!("Closure(%s, %s)", cx.to_str(), term.to_str()),
      }
    }
  }

  pub fn eval(t: @Term) -> @Val {
    let x = ~[];
    let r = eval_in(x, t);
    log(info, x);               //FIXME
    r
  }

  fn lookup(cx: &[@Val], v: uint) -> @Val { cx[cx.len() - v - 1] }
  fn extend(cx: ~[@Val], v: @Val) -> ~[@Val] {
    let mut r = move cx;
    r.push(v);
    move r
  }

  pub fn eval_in(cx: &[@Val], t: @Term) -> @Val {
    match *t {
      Var(v) => lookup(cx, v),
      // is there a better way to copy cx?
      Lam(body) => @Closure(vec::from_slice(cx), body),
      App(e1, e2) => {
        let @Closure(fn_ctx, body) = eval_in(cx, e1);
        let arg = eval_in(cx, e2);
        eval_in(extend(move fn_ctx, arg), body)
      }
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
    //let v = closure::eval(t);
    let v = subst::eval(t);
    io::println((*v).to_str());
  }
}
