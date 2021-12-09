fn main()
{
    // in-order argument placeholder
    println!("{} days", 31);
    println!("{} {} {} {}", 1, 2, 3, 4);

    // indexed-based argument used in printing
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");

    // placeholder names and use them in printing
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // align left with 5 characters
    println!("Hello {:5}!", "x");
    // align center with 5 characters but also has '-' filled for the alignment-slots
    println!("Hello {:-^5}!", "x");
}
