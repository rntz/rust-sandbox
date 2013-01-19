use option::Option;

// an owned, purely functional deque

// eventually will be based on Chris Okasaki's "Simple and Efficient Purely
// Functional Queues and Deques".
//
// Current implementation is horribly inefficient.

pub struct Deque<T>(~[T]);

pub fn push<T>(deq: Deque<T>, elem: T) -> Deque<T> {
  let mut deq = deq;
  (*deq).push(elem);
  deq
}

pub fn pop<T>(deq: Deque<T>) -> (Deque<T>, Option<T>) {
  if (*deq).is_empty() { (deq, None) }
  else {
    let mut deq = deq;
    let elem = (*deq).pop();
    (deq, Some(elem))
  }
}
