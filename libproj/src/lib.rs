/// To be compatible with `cdylib`, we need `export "C"` in order for the symbol
/// not to be stripped when produce a resulting shared library.
/// If you use `dylib`, it might be different with no stripping. Check
/// https://doc.rust-lang.org/reference/linkage.html, and
/// https://users.rust-lang.org/t/what-is-the-difference-between-dylib-and-cdylib/28847/3?u=haxpor.
#[no_mangle]
pub extern "C" fn dummy() -> u64 {
    1
}

#[cfg(test)]
mod tests {
    // NOTE: downside of not having a module for which `dummy()` would be defined in
    use crate::dummy;

    #[test]
    fn test_add() {
        assert_eq!(1, dummy());
    }
}
