macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

macro_rules! addition {
    ($left:expr, $right:expr) => {
        $left + $right
    };
}

// set the closure variable names as well as defining the closure body
macro_rules! block_define_closures_noinput {
    ($left_block_var_name:ident, $left_block:block; $right_block_var_name:ident, $right_block:block) => {
        let $left_block_var_name = || $left_block;
        let $right_block_var_name = || $right_block;
    };
}

fn main() {
    foo();
    bar();
    
    print_result!(1u32 + 2);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
    
    println!("addition(1,2) = {}", addition!(1, 2));
    
    block_define_closures_noinput!(left_closure_var, {1+1}; right_closure_var, {5+5});
    
    println!("{} and {}", left_closure_var(), right_closure_var());
}
