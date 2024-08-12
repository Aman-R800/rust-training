unsafe fn dangerous() {
    println!("This is an unsafe function!");
}

fn main() {
    unsafe {
        dangerous();
    }

    static mut COUNTER: i32 = 0;

    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }

    unsafe trait UnsafeTrait {
        fn unsafe_method(&self);
    }

    struct MyStruct;

    unsafe impl UnsafeTrait for MyStruct {
        fn unsafe_method(&self) {
            println!("Unsafe method called!");
        }
    }

    let s = MyStruct;
    unsafe {
        s.unsafe_method();
    }

    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: i32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    while let Some(n) = counter.next() {
        println!("Counter: {}", n);
    }

    fn diverges() -> ! {
        panic!("This function never returns!");
    }

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let f: fn(i32) -> i32 = add_one;
    println!("Function pointer: {}", f(5));

    let closure = |x: i32| -> i32 { x + 1 };
    println!("Closure: {}", closure(5));

    fn returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    let closure = returns_closure();
    println!("Returned closure: {}", closure(5));

    macro_rules! say_hello {
        () => {
            println!("Hello, world!");
        };
    }

    say_hello!();

    macro_rules! x_and_y {
        (x => $e:expr) => {
            println!("X: {}", $e);
        };
        (y => $e:expr) => {
            println!("Y: {}", $e);
        };
    }

    x_and_y!(x => 5);
    x_and_y!(y => 10);

    macro_rules! create_function {
        ($func_name:ident) => {
            fn $func_name() {
                println!("You called {:?}()", stringify!($func_name));
            }
        };
    }

    create_function!(foo);
    create_function!(bar);

    foo();
    bar();

    macro_rules! find_min {
        ($x:expr) => ($x);
        ($x:expr, $($y:expr),+) => {
            std::cmp::min($x, find_min!($($y),+))
        }
    }

    println!("Min: {}", find_min!(3, 5, 1, 2, 4));

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    struct Wrapper(Vec<String>);

    impl Wrapper {
        fn new() -> Self {
            Wrapper(Vec::new())
        }

        fn add(&mut self, s: String) {
            self.0.push(s);
        }

        fn print(&self) {
            for s in &self.0 {
                println!("{}", s);
            }
        }
    }

    let mut wrapper = Wrapper::new();
    wrapper.add(String::from("Hello"));
    wrapper.add(String::from("World"));
    wrapper.print();
}
