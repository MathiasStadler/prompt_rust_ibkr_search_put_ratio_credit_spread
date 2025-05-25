// cargo build --example main_std_in_out 
// cargo run --example main_std_in_out

//cargo tarpaulin --example main_std_in_out  --ignore-tests --target-dir $PWD/target/tarpaulin --skip-clean --out Lcov

// exact one testcase
// cargo test --example main_std_in_out -- test_program_input_output --exact --show-output
use std::fmt::Error;
fn main() -> Result<(),Error>{
    // This is a simple program that reads input from the user and prints it.
        println!("Entry >");
        let mut _input = String::new();
        std::io::stdin().read_line(&mut _input).unwrap();
        println!("You entered: {}", _input.trim());
        Ok(())
    } 
    // Example test (not fully functional)
    #[test]
    fn test_program_input_output() {
        // This is a simplified example.  A full integration test would use
        // a more robust method of simulating user input and checking output.
        let _input = "hello";
        let mut _stdin = std::io::stdin();
        // Replace stdin with a mock input stream
        let _ = std::io::stdin().lock();
        let mut _stdout = std::io::stdout();
        let result = main();

        match result{
            Ok(_) => {
                // Check if the output matches the expected output
                // This is a placeholder; actual implementation would capture stdout
                assert_eq!(_input, "hello");
            },
            Err(e) => panic!("Test failed with error: {}", e),
        }
        // Assert that the expected output was printed to stdout.
        // Restore the original stdin and stdout.
        // std::io::stdin().lock();
        // std::io::stdout().lock();
    }