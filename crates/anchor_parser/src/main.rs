// main.rs
use std::path::Path;
use std::process;

// Extract the functionality into a function that returns a Result
pub fn run(file_path: &Path) -> Result<(), String> {
    match anchor_parser::parse_file(file_path) {
        Ok(ast) => {
            anchor_parser::print_ast(&ast);
            Ok(())
        }
        Err(err) => {
            Err(format!("Error parsing file: {}", err))
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path-to-anchor-program>", args[0]);
        process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    match run(file_path) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    const SAMPLE_ANCHOR_PROGRAM: &str = r#"
        use anchor_lang::prelude::*;
        
        #[program]
        mod hello_world {
            use super::*;
            
            pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
                msg!("Hello, world!");
                Ok(())
            }
        }
        
        #[derive(Accounts)]
        pub struct SayHello {}
    "#;

    #[test]
    fn test_run_valid_file() {
        // Setup
        let dir = tempdir().expect("Failed to create temp directory");
        let file_path = dir.path().join("test_program.rs");
        fs::write(&file_path, SAMPLE_ANCHOR_PROGRAM).expect("Failed to write test file");
        
        // Test directly by capturing stdout
        let result = run(&file_path);
        
        // Since print_ast writes to stdout, we're just verifying it doesn't error
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_run_invalid_file() {
        let result = run(Path::new("non_existent_file.rs"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Error parsing file"));
    }
}