use unittest_proj::common;
use unittest_proj::utils;

fn main() {
    println!("{}", common::dummy_add(1, 2));
    println!("{}", utils::dummy_val::get_owned_string());
    println!("{}", utils::dummy_val::get_i64());
}
