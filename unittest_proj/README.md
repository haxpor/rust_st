# Brief

`unittest_proj` demonstrates a testing project to conduct

* Unit tests
* Integration test (\*requires nightly build)
* Binary test
* Benchmark test
* Demonstrate as a sample project layout that works for testing

# Note

The project contains the following

* `src/main.rs` - main executable crate, main driver of the program, it consumes APIs defined in `common` and `utils` modules
* `src/tests.rs` - consists all unit tests
* `src/lib.rs` - library crate, shared code used in this binary project
* `src/utils` - `utils` module
    * `src/utils/mod.rs` - module file
    * `src/utils/dummy_val.rs` - `dummy_val` sub-module
* `src/common.rs` - `common` module

# How to run nightly

There are two ways

1. `rustup run nightly <cargo-command>` - benefit of not switching to nightly, but required that you install nightly build which can be done via `rustup toolchain install nightly`.
2. `cargo default nightly` then `cargo bench` - benefit of taking care automatically install nightly for us
