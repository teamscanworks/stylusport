```
stylusport/
├── .github/
│   └── workflows/        # CI/CD workflows
├── crates/               # Main workspace crates
│   ├── anchor_parser/    # Parses Anchor source files
│   │   ├── src/
│   │   ├── tests/
│   │   │   ├── unit/
│   │   │   ├── fixtures/
│   │   │   └── snapshots/
│   │   └── Cargo.toml
│   ├── anchor_normalizer/ # Normalizes Anchor AST
│   │   ├── src/
│   │   ├── tests/
│   │   └── Cargo.toml
│   ├── ir_builder/       # Builds IR from normalized Anchor
│   │   ├── src/
│   │   ├── tests/
│   │   │   ├── ir_fixtures.rs
│   │   │   └── snapshots/
│   │   └── Cargo.toml
│   ├── ir_schema/        # IR schema definition and validation
│   │   ├── src/
│   │   ├── tests/
│   │   └── Cargo.toml
│   ├── explain_engine/   # Annotates IR with explanations
│   │   ├── src/
│   │   ├── tests/
│   │   └── Cargo.toml
│   ├── stylus_generator/ # Generates Stylus code from IR
│   │   ├── src/
│   │   ├── tests/
│   │   │   ├── codegen_fixtures.rs
│   │   │   └── snapshots/
│   │   └── Cargo.toml
│   └── port_cli/         # Command-line interface
│       ├── src/
│       ├── tests/
│       │   ├── pipeline_tests.rs   # End-to-end tests
│       │   └── snapshots/
│       └── Cargo.toml
├── docs/                 # Documentation
│   ├── specs/            # RFCs and specifications
│   │   ├── 0000-vision.md
│   │   ├── 0001-architecture-pipeline.md
│   │   ├── 0002-ir-schema.md
│   │   └── 0003-testing-strategy.md
│   ├── user/             # User documentation
│   │   ├── getting-started.md
│   │   └── examples/
│   └── developer/        # Developer documentation
│       ├── contributing.md
│       └── design-principles.md
├── examples/             # Example projects
│   ├── anchor/           # Input Anchor examples
│   │   ├── hello_world/
│   │   └── token_program/
│   └── stylus/           # Expected Stylus outputs
│       ├── hello_world/
│       └── token_program/
├── fixtures/             # Test fixtures
│   ├── anchor/           # Anchor source fixtures
│   ├── ir/               # IR fixtures in YAML format
│   └── stylus/           # Expected Stylus output
├── scripts/              # Development and utility scripts
├── .rustfmt.toml         # Formatting configuration
├── .gitignore
├── Cargo.toml            # Workspace manifest
├── justfile              # Command runner
├── LICENSE
└── README.md
```