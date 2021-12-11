/// Demonstrate usage of closure via different variant of syntaxs

fn my_fn(x: u64) -> u64 {
    x + 1
}

fn main() {
    // option 1 : as normal function
    let fn1 = my_fn;

    // option 2 : as normal function, define function at the same time of declaration
    fn fn2 (x: u64) -> u64 {
        x + 2
    }

    // more information on variant of closure syntax at
    // https://doc.rust-lang.org/book/ch13-01-closures.html
    // option 3 : as closure, syntax variant 1
    let fn3 = |x: u64| -> u64 { x + 3 };

    // option 4 : as closure, syntax variant 2
    let fn4 = |x| { x + 4 };

    // option 5 : as closure, syntax variant 3
    let fn5 = |x|  x + 5;

    println!("option 1: {}", fn1(1));
    println!("option 2: {}", fn2(1));
    println!("option 3: {}", fn3(1));
    println!("option 4: {}", fn4(1));
    println!("option 5: {}", fn5(1));
}
