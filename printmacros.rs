macro_rules! print(
    ($s:expr $(, $e:expr )* ) => (
        io::print(#fmt[$s $(, $e )*])
    )
)

macro_rules! println(
    ($s:expr $(, $e:expr )* ) => (
        io::println(#fmt[$s $(, $e )*])
    )
)

fn main() {
    print!("%s: ", "numbers");
    println!("%d %s", 0, 1.to_str());
}
