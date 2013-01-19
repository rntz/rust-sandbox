use cmp::{Eq, Ord};

pub enum Order { LT, EQ, GT }

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
    pure fn eq(&self, x: &Order) -> bool {
        match (*self, *x) {
            (LT,LT) => true,
            (EQ,EQ) => true,
            (GT,GT) => true,
            _ => false
        }
    }

    pure fn ne(&self, x: &Order) -> bool { !self.eq(x) }

}

pub pure fn compare<T:Eq Ord>(x: &const T, y: &const T) -> Order {
    if x == y { EQ } else if x < y { LT } else { GT }
}

impl Order: Ord {
    pure fn le(&self, x: &Order) -> bool {
        match (*self, *x) {
            (LT,_) => true,
            (EQ,EQ) => true,
            (EQ,GT) => true,
            (GT,GT) => true,
            _ => false
        }
    }
    pure fn ge(&self, x: &Order) -> bool { x.le(self) }
    pure fn lt(&self, x: &Order) -> bool { !self.ge(x) }
    pure fn gt(&self, x: &Order) -> bool { !self.le(x) }
}
