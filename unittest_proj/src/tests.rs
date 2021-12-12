#[cfg(test)]
mod test {
    /// NOTE: importing modules inside `mod` needs to be inside its scope
    use crate::common;
    use crate::utils::dummy_val;

    #[test]
    fn test_add() {
        assert_eq!(2, common::dummy_add(1,1));
    }

    #[test]
    fn test_dummy_val_get_owned_string() {
        assert_eq!("Dummy String", dummy_val::get_owned_string());
    }

    #[test]
    fn test_dummy_val_get_i64() {
        assert_eq!(1024, dummy_val::get_i64());
    }
}
