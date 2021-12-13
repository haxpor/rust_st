/// Express
/// - in Rust, there is no static data field for struct, but we can declare
///   static variable inside a function. Use this as workaround.
/// - in the same file, it regards as the same module. So we can still access
///   private function. In order to have such effect in the same file, introduce
///   mod scope.
mod incrementer {
    use std::borrow::BorrowMut;

    pub struct StaticVarIncrementer { }

    impl<'a> StaticVarIncrementer {
        fn get_static_var_as_mut() -> &'a mut u32 {
            static mut STATIC_VAR: u32 = 0;

            // either commented line, or the next
            //unsafe { &mut STATIC_VAR as &mut u32 }
            unsafe { STATIC_VAR.borrow_mut() }
        }

        pub fn increment() {
            *StaticVarIncrementer::get_static_var_as_mut() += 1
        }

        pub fn get() -> &'a u32 {
            StaticVarIncrementer::get_static_var_as_mut()
        }
    }
}

fn main() {
    use incrementer::*;

    println!("{}", StaticVarIncrementer::get());
    StaticVarIncrementer::increment();
    println!("{}", StaticVarIncrementer::get());
}
