/// Demonstrate implementation for a custom iterator included iter(), and into_iter().
/// Inspired by
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ab043cf7449cb45a1b4785e39b4a85a5

struct Widget;

struct CustomIterator {
    cnt: i32,
}

/// Only next() that we need to implement, other we can just rely on default
/// implementation of Iterator.
impl Iterator for CustomIterator {
    // Item declared as part of upstream Iterator type
    // Here we specifically specify the type for Item to be used with this Iterator
    // as default implementation of Iterator refers to Self
    type Item = i32;

    // just copy the signature from library/core/src/iter/traits/iterator.rs
    fn next(&mut self) -> Option<Self::Item> {
        if self.cnt < 10 {
            self.cnt += 1;
            Some(self.cnt)
        } else {
            None
        }
    }
}

// implementation of iter()
impl Widget {
    fn iter(&self) -> CustomIterator {
        CustomIterator { cnt: 0 }
    }
}

// following twos section are for implementation of into_iter()
impl<'a> IntoIterator for &'a Widget {
    type Item = i32;
    type IntoIter = CustomIterator;

    fn into_iter(self) -> Self::IntoIter {
        CustomIterator { cnt: 0 }
    }
}

impl IntoIterator for Widget {
    type Item = i32;
    type IntoIter = CustomIterator;

    fn into_iter(self) -> Self::IntoIter {
        CustomIterator { cnt: 0 }
    }
}

fn main() {
    let w = Widget{};

    // note: count() will iterating the whole container
    // thus it depends on the logic we have implemented in `impl Iterator for CustomIterator`.

    // iter() doesn't consume (not move, but borrow)
    let _c2 = w.iter().count();
    // so we are able to call into_iter() (to move) again
    let _c1 = w.into_iter().count();

    println!("{}", _c2);
    println!("{}", _c1);
}
