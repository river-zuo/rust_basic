pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_parse {
        ($a: tt) => {
            assert_eq!($a("Hello, world!"), "Hello, world!")
        };
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        println!();
    }
}
