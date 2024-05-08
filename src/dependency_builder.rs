pub mod dependency_builder;


#[cfg(test)]
mod tests {
    use core::panic;

    #[test]
    fn should_work() {
        assert!(true)
    }

    #[test]
    fn should_panic()  {
        panic!("this test failed");
    }

}
