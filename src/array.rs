/// Print input array using 'while let' statement
/// `while let` is a simplified way in writing similar to `match optional`.
/// Note that `Some()` is unwrapping from `Optional` that expects value from it.
fn print_array_iter(arr: &[i32])
{
    // TODO: use clippy on this to detect that we better call iter() to not consume
    // the slice.
    let mut iter = arr.into_iter().enumerate();

    // TODO: yep, with clippy, it suggests us to just use `for e in iter`
    // but this is for educational purpose.
    while let Some(e) = iter.next() {
        println!("elem[{}] = {}", e.0, e.1);
    }
}

/// iterate by value
/// this way, we don't get index value for each element
fn print_array(arr: &[i32])
{
    // this is equivalent to
    // `for e in arr.iter()`
    for e in arr {
        println!("{}", e);
    }
}

/// iterate by value
/// another option
fn print_array2(arr: &[i32])
{
    for e in IntoIterator::into_iter(arr) {
        println!("{}", e);
    }
}

fn main() {
    // a is an array [{integer}; 5]
    let a = [1, 2, 3, 4, 5];

    print_array_iter(&a);
    print_array(&a);
    print_array2(&a);

    // Both into_iter() and IntoIterator::into_iter() are not implemented
    // for [T; N] for edition <2021 so it won't move, but will borrow intead
    // thus needs us to change from `a` to `&a` when call `into_iter()`.
    //
    // Exception still although into_iter() not implemented, but `Copy` implemented
    // for such T then it will be copied instead of moved thus we can still
    // use such variable afterwards.
    //
    // Primitive integer (in this case), or primitive data types (not BigInt) implement `Copy`
    // function.
    //
    // See https://stackoverflow.com/a/34745885/571227.
    // See library/core/src/array/mod.rs for IntoIterator + into_iter in rust code.
    // See compiler/rustc_span/src/symbol.rs for integer symbol which depends on
    //   Copy.
    // See https://github.com/rust-num/num/issues/191 as well for discussion.
    //
    // option 1
    for e in a.into_iter().enumerate() {
        let (index, val): (usize, i32) = e;
        println!("elem[{}] = {}", index, val);
    }

    // option 2
    for e in IntoIterator::into_iter(a).enumerate() {
        let (index, val): (usize, i32) = e;
        println!("elem[{}] = {}", index, val);
    }


    println!("{:?}", a);
}
