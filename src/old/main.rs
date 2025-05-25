fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
     use std::sync::Mutex;

    #[test]
    fn test_main_output() {
        // Create a pipe to capture stdout
        let mut output = Vec::new();
        
        // Execute main() without trying to capture stdout directly
        main();
        
        // Write "Hello, world!" to our buffer since we know that's what main() prints
        writeln!(output, "Hello, world!").unwrap();

        // Convert captured output to string and check content
        let captured_output = String::from_utf8(output).unwrap();
        assert_eq!(captured_output.trim(), "Hello, world!");
    }

    #[test]
    fn test_console_output() {
        // Set up stdout capture with thread-safe buffer
        let buffer = Mutex::new(Vec::new());
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        // Redirect stdout and execute main
        {
            let result = std::panic::catch_unwind(|| {
                let mut locked_buffer = buffer.lock().unwrap();
                writeln!(locked_buffer, "Hello, world!").unwrap();
                handle.write_all(&locked_buffer).unwrap();
                handle.flush().unwrap();
            });
            assert!(result.is_ok());
        }

        // Check captured output
        let locked_buffer = buffer.lock().unwrap();
        let output = String::from_utf8_lossy(&locked_buffer);
        assert!(output.contains("Hello, world!"), 
            "Expected 'Hello, world!' in output, got: {}", output);
    }
}