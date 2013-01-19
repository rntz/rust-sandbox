use io::{Reader,ReaderUtil,Writer,WriterUtil};
use to_str::{ToStr};
use option::{Option};
use char::{is_whitespace,is_digit};
use result::{Ok,Err,chain};

extern mod std;
use map = std::map;
use map::HashMap;

fn main() {}

// is there a better representation for symbols?
type Sym = @str;

enum Val {
  Nil,
  Cons(@Val, @Val),
  Num(int),
  Str(@str),
  Sym(Sym),
  Fn(@Scope, @Val, @Val),
  Builtin(Builtin),
}

enum Builtin {
  BCons, BCar, BCdr, BPrint,
}

type Var = @mut Val;            //named Var solely to annoy Bob Harper

enum Scope {
  WithLocal(Sym, Var, @Scope),
  Globals(@Globals),
}

type Globals = HashMap<Sym, Var>;

impl Scope {
  fn find(&self, var: Sym) -> Option<Var> {
    match *self {
      WithLocal(v, cell, rest) =>
        if var == v { Some(cell) }
        else { rest.find(var) },
      Globals(globs) => globs.find(var),
    }
  }

  fn with(@self, var: Sym, val: Val) -> @Scope {
    @WithLocal(var, @mut val, self)
  }
}

// ---------- STRINGIFYING & PARSING ----------
impl Val: ToStr {
  pure fn to_str() -> ~str {
    pure fn list_to_str(v: Val) -> ~str {
      match v {
        Cons(@x, @Nil) => x.to_str(),
        Cons(@x, @y) => x.to_str() + " " + list_to_str(y),
        Nil => ~"",
        _ => ~". " + v.to_str(),
      }
    }
    match self {
      Nil => ~"nil",
      Cons(*) => ~"(" + list_to_str(self) + ")",
      Num(i) => i.to_str(),
      Str(s) => fmt!("\"%s\"", s.escape_default()),
      Sym(s) => s.to_str(),
      Fn(*) => ~"<function>",
      Builtin(_) => ~"<builtin>",
    }
  }
}

// ---------- EVALUATOR ----------
struct Engine {}                //global state

type Result = result::Result<Val, Error>;
enum Error {
  UnboundVar(Sym, @Scope),
  Error(~str)
}

impl Engine {
  fn eval(scope: @Scope, exp: Val) -> Result {
    match exp {
      Sym(sym) => match scope.find(sym) {
        Some(@v) => Ok(v),
        None => Err(UnboundVar(sym, scope)),
      },
      Cons(@f, @args) => self.eval_cons(scope, f, args),
      // everything else evaluates to itself
      _ => Ok(exp)
    }
  }

  fn eval_body(scope: @Scope, body: Val) -> Result {
    match body {
      Nil => Err(Error(~"empty body in fn")),
      Cons(@x, @Nil) => self.eval(scope, x),
      Cons(@x, @rest) => {
        self.eval(scope, x);
        self.eval_body(scope, rest)
      }
      _ => Err(Error(~"invalid body")),
    }
  }

  fn eval_cons(scope: @Scope, fexp: Val, argexps: Val) -> Result {
    match fexp {
      Sym(s) => {
        if s == @"fn" {
          match argexps {
            Cons(argspat, body) => Ok(Fn(scope, argspat, body)),
            _ => Err(Error(~"malformed use of fn")),
          }
        } else if s == @"let" {
          fail                  //TODO
        } else if s == @"assign" {
          fail                  //TODO
        } else if s == @"quote" {
          // Quote first argument
          match argexps {
            Cons(@sexp, @Nil) => Ok(sexp),
            _ => Err(Error(~"wrong number of arguments to quote")),
          }
        } else {
          self.eval_call(scope, fexp, argexps)
        }
      }
      _ => self.eval_call(scope, fexp, argexps)
    }
  }

  fn eval_call(scope: @Scope, fexp: Val, argexps: Val) -> Result {
    // It is a function application
    do chain(self.eval(scope, fexp)) |f| {
      do chain(self.eval_args(scope, argexps)) |args| {
        self.apply(f, args)
      }
    }
  }

  fn eval_args(scope: @Scope, argexps: Val) -> Result {
    match argexps {
      Nil => Ok(Nil),
      Cons(@car, @cdr) => {
        do chain(self.eval(scope, car)) |carv| {
          do chain(self.eval(scope, cdr)) |cdrv| {
            Ok(Cons(@carv, @cdrv))
          }
        }
      }
      _ => Err(Error(~"cannot evaluate in dotted position")),
    }
  }

  fn apply(func: Val, args: Val) -> Result {
    match func {
      // builtins
      Builtin(b) => match b {
        BCons => match args {
          Cons(x, @Cons(y, @Nil)) => Ok(Cons(x,y)),
          _ => Err(Error(~"wrong number of arguments to cons")),
        },
        BCar => match args {
          Cons(@Cons(@x,_), @Nil) => Ok(x),
          _ => Err(Error(~"bad arguments to car")),
        },
        BCdr => match args {
          Cons(@Cons(_,@x), @Nil) => Ok(x),
          _ => Err(Error(~"bad arguments to car")),
        },
        BPrint => match args {
          Cons(x, @Nil) => {
            // FIXME: better printing
            io::println(fmt!("%?", x));
            Ok(Nil)
          }
          _ => Err(Error(~"wrong number of arguments to print")),
        },
      },

      // closures
      Fn(clos, @argpat, @body) => {
        do chain(self.bind_args(clos, argpat, args)) |scope| {
          self.eval_body(scope, body)
        }
      }

      _ => Err(Error(~"applied a non-function, non-builtin"))
    }
  }

  fn bind_args(scope: @Scope, pat: Val, arg: Val)
    -> result::Result<@Scope, Error>
  {
    match (pat, arg) {
      (Sym(s), v) => Ok(scope.with(s, v)),
      (Nil, Nil) => Ok(scope),
      (Nil, _) => Err(Error(~"bad arguments: not nil")),
      (Cons(@xp, @yp), Cons(@xv, @yv)) => {
        do chain(self.bind_args(scope, xp, xv)) |scope| {
          self.bind_args(scope, yp, yv)
        }
      },
      (Cons(*), _) => Err(Error(~"bad arguments: not a cons")),
      (Num(_), _) => Err(Error(~"numbers are not valid patterns")),
      (Str(_), _) => Err(Error(~"strings are not valid patterns")),
      (Fn(*), _) => Err(Error(~"functions are not valid patterns")),
      (Builtin(_), _) => Err(Error(~"builtins are not valid patterns")),
    }
  }
}
