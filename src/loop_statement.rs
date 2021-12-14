/// Demonstrate the use of loop statement.
/// `loop` is infnite loop

fn main() {
    // 0 with u32 type
    let mut count = 0u32;

    loop {
        count += 1;

        if count == 2 {
            println!("tee hee!");
            continue;
        }

        if count == 10 {
            break;
        }

        println!("{}", count);
    }
}
