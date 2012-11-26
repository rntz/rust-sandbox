use cmp::{Eq, Ord};

type Le<T> = pure fn&(&T, &T) -> bool;

struct SortParams<T> {
    le: Option<Le<T>>,
    find_pivot: Option<pure fn&(&[const T]) -> uint>,
}

enum Rev<T> = T;

impl<T:Eq> Rev<T> : Eq {
    pure fn eq(other: &Rev<T>) -> bool { cmp::eq(&*self, &**other) }
    pure fn ne(other: &Rev<T>) -> bool { cmp::ne(&*self, &**other) }
}

impl<T:Eq Ord> Rev<T>: Ord {
    pure fn lt(other: &Rev<T>) -> bool { cmp::gt(&*self, &**other) }
    pure fn le(other: &Rev<T>) -> bool { cmp::ge(&*self, &**other) }
    pure fn ge(other: &Rev<T>) -> bool { cmp::le(&*self, &**other) }
    pure fn gt(other: &Rev<T>) -> bool { cmp::lt(&*self, &**other) }
}


// The actual sorting routine
// An unstable in-place quicksort using median-of-three.
fn sort_by<T>(array: &[mut T], le: Le<T>) {
    let len = array.len();
    if len < 2 { return }
    if len == 2 {
        if !le(&array[0], &array[1]) {
            vec::swap(array, 0, 1);
        }
        return;
    }

    let split_idx = partition_by(array, median_of_three(array, le), le);

    // Sort the two halves.
    sort_by(vec::mut_view(array, 0, split_idx), le);
    sort_by(vec::mut_view(array, split_idx+1, len), le);
}

fn partition_by<T>(array: &[mut T], pivot_idx: uint, le: Le<T>) -> uint {
    array[0] <-> array[pivot_idx];
    let mut end = 1u, i = 1u;
    loop {
        while i < array.len() && le(&array[0], &array[i]) {
            i += 1;
        }
        if i >= array.len() { break; }
        array[end] <-> array[i];
        assert le(&array[end], &array[0]);
        assert i == end || le(&array[0], &array[i]);
        i += 1; end += 1;
    }
    array[0] <-> array[end-1u];
    end - 1u
}

// Finds the index of the median of the first, middle, and last element.
pure fn median_of_three<T>(array: &[const T], le: Le<T>) -> uint {
    let len = array.len();
    assert len >= 3;
    let fst = &array[0], mid = &array[len/2], lst = &array[len-1];
    match (le(fst, mid), le(mid, lst)) {
        (true, true) => len/2,
        (false, false) => len/2,
        // fst <= mid, lst < mid
        (true, false) => { if le(fst, lst) { len-1 } else { 0 } }
        // mid < fst, mid <= lst
        (false, true) => { if le(fst, lst) { 0 } else { len-1 } }
    }
}


// Some utilities
fn sort<T: Eq Ord>(array: &[mut T]) {
    sort_by(array, cmp::le::<T>);
}

fn sort_on<T,U: Eq Ord>(array: &[mut T], proj: pure fn&(&T) -> U) {
    do sort_by(array) |x,y| { cmp::le(&proj(x), &proj(y)) }
}

fn main() {
    let x: ~[mut int] = ~[mut 3,2,7,12,100,7];
    sort(x);
    // Need to from_slice to get rid of the mut :/
    log(info, x);
}
