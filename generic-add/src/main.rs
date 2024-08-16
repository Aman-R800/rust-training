use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    return a + b
}

fn main() {
    println!("{}", add(5, 7));
    println!("{}", add(8.23, 7.57));
}
