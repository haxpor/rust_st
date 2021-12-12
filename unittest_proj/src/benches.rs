extern crate test;

#[cfg(test)]
mod benches {
    // bring in both Bencher, and black_box in to the scope
    use super::test::{Bencher, black_box};

    /// First style to use `black_box` to hint compilers not to optimize
    /// our code. As tested, either removing `;` at the end of code statement
    /// is enough, and no need to use `black_box` at all.
    ///
    /// NOTE: `black_box` uses `std::hint::black_box` to hint compiler not to
    /// optimize.
    ///
    /// NOTE2: logic code used inside the benchmark taken from
    /// https://doc.rust-lang.org/1.2.0/book/benchmark-tests.html.
    #[bench]
    fn bench_dummy(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(1000);

            // not optimizer cannot optimize out our code thus avod it entirely
            (0..n).fold(0, |a, b| a ^ b)
        });
    }

    /// Alternative style to not use `black_box`.
    #[bench]
    fn bench_dummy2(b: &mut Bencher) {
        b.iter(|| {
            (0..1000).fold(0, |a, b| a ^ b)
        });
    }
}
