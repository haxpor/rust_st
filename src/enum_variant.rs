/// Demonstrate the use of enum variant to mimick std::variant in C++
/// Note that Vec<T> has size of 24 bytes.

use std::mem;

// Variant enum to hold multiple types
enum GenericVar {
    Byte(u8),
    Short(u16),
    Str(String),
    Float(f64)
}

fn main() {
    let mut var_variant = Vec::new();

    var_variant.push(GenericVar::Byte(1));
    var_variant.push(GenericVar::Short(1024));

    // need to call to_owned(), basically to clone the string and owned it
    var_variant.push(GenericVar::Str("Hello world".to_owned()));
    var_variant.push(GenericVar::Float(10.4));

    println!("size of u8: {}", mem::size_of::<u8>());
    println!("size of u16: {}", mem::size_of::<u16>());
    println!("size of String: {}", mem::size_of::<String>());
    println!("size of f64: {}", mem::size_of::<f64>());
    println!("size of Vec<u8>: {}", mem::size_of::<Vec<u8>>());  // underlying data structure that String holds
                                                                 // See library/alloc/src/string.rs
    
    // to print the total size of Vec<T> we need to dereference first
    // to get to [T] then reference it.
    println!("size of var_variant: {}", mem::size_of_val(&*var_variant));

    for e in var_variant {
        match e {
            GenericVar::Byte(i) => println!("{} - u8", i),
            GenericVar::Short(i) => println!("{} - u16", i),
            GenericVar::Str(i) => println!("{} - String", i),
            GenericVar::Float(i) => println!("{} - f64", i),
        }
    }
}
