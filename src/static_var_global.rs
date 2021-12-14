/// Express how to define global static variables.
/// For mutable global static variable, we would need synchronized primitives to
/// prevent data race thus allow only once thread at a time to make change to the
/// variable, or we can use `thread_local!` macro to make a copy of it per thread.
/// Thus there is no worry for data race to happen.
///
/// So in short, this program demonstrates the following
/// 1. Define static global variables
/// 2. Wrap those static global variables inside thread_local! for safety in
///    prevention of data race.
/// 3. How to use thread_local! (which is LocalKey<T>) to access or call its
///    method.

use std::cell::RefCell;

// NOTE: we cannot do .to_owned() as it's not const function.
// For safety in using static variables in global scope, sync them via
// any synchronized primitives, or in our case use `thread_local!` (for simplicity)
static STATIC_STRING: &str = "This is static variable string";

thread_local! {
    #[allow(unused)]
    pub static STATIC_NUM: RefCell<u32> = RefCell::new(1);

    // We need lifetime parameter `'static` here.
    // You can inspect the executable binary file with objdump for instance
    // in section .rodata, you will see the content of "My string" there.
    #[allow(unused)]
    pub static STATIC_ANOTHER_STRING: RefCell<&'static str> = RefCell::new("My string");
}

fn main() {
    println!("STATIC_STRING = {}", STATIC_STRING);

    // to access variable defined inside `thread_local!` scope
    // we need to use `with()` or `try_with()`
    STATIC_NUM.with(|f| {
        println!("STATIC_NUM = {}", *f.borrow());
        f.replace(2);
        println!("STATIC_NUM = {}", *f.borrow());
    });

    println!("---");

    STATIC_ANOTHER_STRING.with(|f| {
        println!("STATIC_ANOTHER_STRING = {}", *f.borrow());

        // take the value out, and leave it with Defau;t::default() value
        let str_val = f.take();

        println!("str_val = {}", str_val);
        println!("STATIC_ANOTHER_STRING = {}", *f.borrow());
    });

    // We cannot do `into_inner()` here as LocalKey<T>::with() accept the borrowed
    // argument, and we cannot move out from borrowed content.
    //STATIC_NUM.with(|f| {
    //    let v = f.into_inner();
    //    println!("STATIC_NUM = {}", v);
    //});
}
