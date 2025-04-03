#[cfg(all(test, feature = "module_test"))]
mod example_tests {
    use anchor_parser::parse_file;
    use std::path::{Path, PathBuf};

    // Helper function to get the path to an example file
    fn example_path(name: &str) -> PathBuf {
        // CARGO_MANIFEST_DIR points to the crate directory (where the crate's Cargo.toml is)
        let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

        // To get to the workspace/project root (assuming your crate is one level deep), go up two levels
        let project_root = Path::new(&crate_root)
            .parent()
            .and_then(|p| p.parent())
            .expect("Could not find project root directory");

        // The examples directory is typically in the workspace/project root
        let path = project_root.join("examples").join(name).join("lib.rs");

        // Verify the file exists
        if !path.exists() {
            panic!("Example file not found: {:?}", path);
        }

        path
    }

    // Simplified test case definition structure
    struct ExampleTest {
        name: &'static str,
        program_module: &'static str,
        instructions: Vec<&'static str>,
        account_structs: Vec<&'static str>,
        raw_accounts: Vec<&'static str>,
    }

    // Provide default values
    impl Default for ExampleTest {
        fn default() -> Self {
            Self {
                name: "",
                program_module: "",
                instructions: Vec::new(),
                account_structs: Vec::new(),
                raw_accounts: Vec::new(),
            }
        }
    }

    // Reusable test executor - only testing basic structure, no constraints
    fn run_example_test(test: &ExampleTest) {
        let path = example_path(&test.name);

        // Parse the example program
        let program = parse_file(&path)
            .unwrap_or_else(|e| panic!("Failed to parse {} example: {}", test.name, e));

        // Verify program module exists
        assert!(
            program.find_program_module(test.program_module).is_some(),
            "Program module '{}' not found in {}",
            test.program_module,
            test.name
        );

        // Verify instructions
        let module = program.find_program_module(test.program_module).unwrap();
        for instruction in &test.instructions {
            assert!(
                module.find_instruction(instruction).is_some(),
                "Instruction '{}' not found in {}",
                instruction,
                test.name
            );
        }

        // Verify account structs
        for account_struct in &test.account_structs {
            assert!(
                program.find_account_struct(account_struct).is_some(),
                "Account struct '{}' not found in {}",
                account_struct,
                test.name
            );
        }

        // Verify raw accounts
        for raw_account in &test.raw_accounts {
            assert!(
                program.find_raw_account(raw_account).is_some(),
                "Raw account '{}' not found in {}",
                raw_account,
                test.name
            );
        }
    }

    #[test]
    fn test_hello_world_example() {
        let test = ExampleTest {
            name: "hello_world",
            program_module: "hello_world",
            instructions: vec!["initialize"],
            account_structs: vec!["Initialize"],
            raw_accounts: vec![],
        };

        run_example_test(&test);
    }

    #[test]
    fn test_counter_example() {
        let test = ExampleTest {
            name: "counter",
            program_module: "counter",
            instructions: vec!["initialize", "increment"],
            account_structs: vec!["Initialize", "Increment"],
            raw_accounts: vec!["Counter"],
        };

        run_example_test(&test);
    }

    #[test]
    fn test_token_vault_example() {
        let test = ExampleTest {
            name: "token_vault",
            program_module: "token_vault",
            instructions: vec!["initialize", "deposit"],
            account_structs: vec!["Initialize", "Deposit"],
            raw_accounts: vec!["Vault"],
        };

        run_example_test(&test);
    }

    #[test]
    fn test_examples_directory_exists() {
        // Get path to examples directory
        let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let project_root = Path::new(&crate_root)
            .parent()
            .and_then(|p| p.parent())
            .expect("Could not find project root directory");

        let examples_dir = project_root.join("examples");

        // Verify examples directory exists
        assert!(
            examples_dir.exists() && examples_dir.is_dir(),
            "Examples directory not found at {:?}",
            examples_dir
        );

        // List available examples for debugging
        if let Ok(entries) = std::fs::read_dir(&examples_dir) {
            println!("Available examples:");
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        println!("  - {:?}", entry.file_name());
                    }
                }
            }
        }
    }
}
