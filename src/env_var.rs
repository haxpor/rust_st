/// Demonstrate getting value from environment variables.
/// Test the program with
/// On Linux:
///   rustc env_var.rs
///   export ENV_SECRET_KEY=mysecretkey && ./env_var

use std::env;

fn main() {
    const KEY: &str = "ENV_SECRET_KEY";

    // checking style 1
    {
        let env_val = env::var(KEY);
        match env_val {
            Ok(val) => println!("{} = {}", KEY, val),
            Err(e) => println!("couldn't read value from {} with error '{}'", KEY, e),
        }
    }

    // checking style 2
    {
        if env::var(KEY).is_ok() {
            // this is usually not reccommended to use
            // better use `match` pattern
            println!("{} = {}", KEY, env::var(KEY).unwrap());
        } else {
            println!("couldn't read value from {}", KEY);
        }
    }
}
