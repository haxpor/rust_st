fn main() {
    // compound element
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value as follows: {},{},{}", x, y, z);

    println!("x value: {}", tup.0);
    println!("y vaule: {}", tup.1);
    println!("z value: {}", tup.2);

    // this will also print out as ()
    let empty_tuple = ();
    println!("{:?}", empty_tuple);
}
