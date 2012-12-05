// this code is accepted but shouldn't be; it duplicates a unique pointer
fn dup<T>(x: T) -> (T,T) { *~(x,x) }
fn main() { dup(~2); }
