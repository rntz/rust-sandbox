fn forl<T:Copy>(init: T, next: fn(T)->T, end: fn(T)->bool, block: fn(T))
{
    let mut x = init;
    loop {
        if end(x) {break}
        block(x);
        x = next(x);
    }
}

fn repeat(n: uint, f: fn(uint)) {
    let mut i = n;
    while i < n { f(i); i += 1u; }
}

fn range(from: uint, to: uint, by: int, f: fn(uint)) {
    if from == to { return }
    assert if from < to {by > 0} else {by < 0};

    let mut i = from;
    while if from < to {i < to} else {i > to} {
        f(i);
        if from < to { i += 1u; }
        else { i -= 1u; }
    }
}

fn rangei(from: int, to: int, by: int, f: fn(int)) {
    if from == to { return }
    assert if from < to {by > 0} else {by < 0};

    let mut i = from;
    while if from < to {i < to} else {i > to} {
        f(i);
        i += if from < to {1} else {-1};
    }
}

fn up(from: uint, to: uint, f: fn(uint)) { range(from, to, 1, f) }
fn down(from: uint, to: uint, f: fn(uint)) { range(from, to, -1, f) }

fn upi(from: int, to: int, f: fn(int)) { rangei(from, to, 1, f) }
fn downi(from: int, to: int, f: fn(int)) { rangei(from, to, -1, f) }

// fn from_to(from: int, to: int, by: int, f: block(int)) {
//     assert (if from < to then {by > 0}
//             else if from > to {by < 0}
//             else {true});
//     let mut i = from;
// }
