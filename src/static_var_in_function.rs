/// Express
/// - in Rust, there is no static data field for struct, but we can declare
///   static variable inside a function. Use this as workaround.
/// - in the same file, it regards as the same module. So we can still access
///   private function. In order to have such effect in the same file, introduce
///   mod scope.
mod incrementer {
    use std::borrow::BorrowMut;

    /// This depends on the type of Ii.
    /// If Ii changes its type, we need to reflect this as well.
    use std::sync::atomic::{AtomicU32, Ordering};

    type Ii = u32;
    type AtomicType = AtomicU32;

    pub trait Incrementable {
        fn increment();
        fn get() -> Ii;
    }

    /// Non-thread-safe version for a container to increment a managed static variable
    pub struct StaticVarIncrementer { }

    impl<'a> StaticVarIncrementer {
        fn get_static_var_as_mut() -> &'a mut Ii {
            static mut STATIC_VAR: Ii = 0;

            // NOTE: borrow_mut() is available for primitive types as well
            // either commented line, or the next
            //unsafe { &mut STATIC_VAR as &mut Ii }
            unsafe { STATIC_VAR.borrow_mut() }
        }
    }

    impl<'a> Incrementable for StaticVarIncrementer {
        fn increment() {
            *StaticVarIncrementer::get_static_var_as_mut() += 1
        }

        fn get() -> Ii {
            *StaticVarIncrementer::get_static_var_as_mut()
        }
    }

    /// Thread-safe version of StaticVarIncrementer
    pub struct ThreadSafeStaticVarIncrementer { }

    impl<'a> ThreadSafeStaticVarIncrementer {
        fn get_static_var_as_mut() -> &'a mut AtomicType {
            static mut STATIC_VAR: AtomicType = AtomicType::new(0);

            unsafe { STATIC_VAR.borrow_mut() }
        }
    }

    impl<'a> Incrementable for ThreadSafeStaticVarIncrementer {
        fn increment() {
            (*ThreadSafeStaticVarIncrementer::get_static_var_as_mut()).fetch_add(1, Ordering::Relaxed);
        }

        fn get() -> Ii {
            (*ThreadSafeStaticVarIncrementer::get_static_var_as_mut()).load(Ordering::Relaxed)
        }
    }
}

fn main() {
    use incrementer::*;
    use std::thread;

    // StaticVarIncrementer
    println!("{}", StaticVarIncrementer::get());
    StaticVarIncrementer::increment();
    println!("{}", StaticVarIncrementer::get());

    // ThreadStaticVarIncrementer
    let mut threads = vec![];

    const NUM_LOOP: u32 = 1000;
    // spawn NUM_LOOP threads to increment the value managed by ThreadSafeStaticVarIncrementer
    // [start, end) for the range.
    for _ in 1..NUM_LOOP+1 {
        threads.push(thread::spawn(|| {
            ThreadSafeStaticVarIncrementer::increment();
        }));
    }
    
    for t in threads {
        match t.join() {
            Ok(_) => {},
            Err(e) => eprintln!("{:?}", e),
        }
    }

    assert_eq!(NUM_LOOP, ThreadSafeStaticVarIncrementer::get());
    println!("Done");
}
