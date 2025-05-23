fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::.io;

    #[test]
    fn test_main_output() {
        // Create a pipe to capture stdout
        let mut output = Vec::new();
        
        // Execute main() and capture its output
        {
            let mut handle = io::Cursor::new(&mut output);
            // Temporarily redirect stdout to our buffer
            let result = std::panic::catch_unwind(|| {
                main();
            });
            assert!(result.is_ok(), "main() panicked!");
        }

        // Convert captured output to string and check content
        let captured_output = String::from_utf8(output).unwrap();
        assert_eq!(captured_output.trim(), "Hello, world!");
    }
}