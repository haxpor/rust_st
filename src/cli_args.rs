/// Demonstrate how to accept command line arguments
/// After build the program, test it by supplying some command line arguments.

use std::env;

fn main() {
    // use collect() to turn things into collection
    // the returned type depends on the context e.g. Vec<String> in this case.
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}
