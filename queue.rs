use option::*;

pub type Queue<T> = Queue_<T>;

struct Queue_<T> {
  mut queue: ~[T]
}

pub impl<T> Queue<T> {
  fn push(self, elem: T) {
    vec::unshift(&mut self.queue, elem)
  }

  pure fn is_empty(&self) -> bool {
    self.queue.is_empty()
  }

  fn pop(self) -> T {
    assert !self.is_empty();
    vec::pop(&mut self.queue)
  }

  fn try_pop(self) -> Option<T> {
    if self.is_empty() { None }
    else { Some(self.pop()) }
  }
}
