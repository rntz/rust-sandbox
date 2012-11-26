import option::option;

fn main() {
    io::println("hello world");
}

type Z = ();
enum S<T> { zero, succ(T) }

iface finite {
    fn value() -> uint;
    fn max(self) -> self;
}

impl of finite for Z {
    fn value() -> uint { 0u }
    fn max(_q : Z) -> Z { () }
}

impl<T:finite copy> of finite for S<T> {
    fn value() -> uint {
        alt self {
          zero { 0u }
          succ(n) { 1u + n.value() }
        }
    }

    fn max(x : S<T>) -> S<T> {
        alt (self, x) {
          (zero, _) {x}
          (_, zero) {self}
          (succ(a), succ(b)) { succ(a.max(b)) }
        }
    }
}
