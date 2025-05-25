mod my_module { // Replace with the actual module name
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }
    #[cfg(test)]
    mod tests {
        use super::my_module; // Replace with the actual module name
        #[test]
        fn it_adds_two_numbers() {
            assert_eq!(my_module::add(2, 3), 5);
        }
    }
    fn main() {
        println!("{}", my_module::add(2, 3)); // Call the function in main
    }
// command line
// cargo tarpaulin --ignore-tests --target-dir $PWD/target/tarpaulin --skip-clean --out Lco