use cmp::{Eq, Ord};

enum Order { LT, EQ, GT }

impl Order: ToStr {
    pure fn to_str() -> ~str {
        match self {
            LT => ~"LT",
            EQ => ~"EQ",
            GT => ~"GT"
        }
    }
}

impl Order: Eq {
    pure fn eq(x: &Order) -> bool {
        match (self, *x) {
            (LT,LT) => true,
            (EQ,EQ) => true,
            (GT,GT) => true,
            _ => false
        }
    }

    pure fn ne(x: &Order) -> bool { !self.eq(x) }

}

fn compare<T:Eq Ord>(x: &const T, y: &const T) -> Order {
    if x == y { EQ } else if x < y { LT } else { GT }
}

impl Order: Ord {
    pure fn le(x: &Order) -> bool {
        match (self, *x) {
            (LT,_) => true,
            (EQ,EQ) => true,
            (EQ,GT) => true,
            (GT,GT) => true,
            _ => false
        }
    }
    pure fn ge(x: &Order) -> bool { (*x).le(&self) }
    pure fn lt(x: &Order) -> bool { !self.ge(x) }
    pure fn gt(x: &Order) -> bool { !self.le(x) }
}

fn main() {}
