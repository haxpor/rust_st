/// Demonstrate
/// - using slice
/// - simple template function
/// - printing address of variable
/// - convertion of variable into numeric address

fn get_address<T>(ptr: &T) -> usize 
{
    let raw_ptr = ptr as *const T;
    raw_ptr as usize
}
fn main() {
    let str = String::from("hello world");

    // define a slice that starts from the first index, to the 3rd character
    // structure of slice contains
    // - a typed pointer
    // - length
    // See https://doc.rust-lang.org/book/ch04-03-slices.html
    let slice = &str[0..2];

    println!("original: {}", str);
    println!("slice:    {}", slice);

    println!("address of str (in numeric direct): {:p}", &str);
    // or get_address(&str)
    // this will print in hexadecimal style
    // see https://stackoverflow.com/a/56485170/571227
    println!("address of str (in numeric): 0x{:02x}", get_address::<String>(&str));

    // assert that get_address() function is in fact performs correctly
    assert!(((&str as *const String) as usize) == get_address(&str));
}
