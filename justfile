# Default: show available commands
default:
    @just --summary

# -------------------------------
# 🏗️ Project Structure
# -------------------------------

# List all crates in the workspace
list-crates:
    @cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.manifest_path | contains("crates/")) | .name'

# -------------------------------
# 🧱 Build & Run
# -------------------------------

# Build any crate by name
build-crate name:
    cargo build -p {{name}}

# Run a binary crate with optional args
run-crate name args="":
    cargo run -p {{name}} -- {{args}}

# Build the whole workspace
build:
    cargo build --workspace

# -------------------------------
# Run specific stages of the pipeline on some example
# -------------------------------

# Run normalization on a specific example
normalize-example name:
    cargo run -p stylusport -- normalize examples/{{name}}/lib.rs --format=yaml --output=output/{{name}}.norm.yaml


# -------------------------------
# 🧪 Testing
# -------------------------------


# Run all tests in the workspace
test:
    cargo test --workspace

# Run all tests for a specific crate
test-crate name:
    cargo test -p {{name}} --all-features

# Run only unit tests (with unit_test attribute)
test-unit:
    cargo test --workspace --features "unit_test"

# Run only module tests (with module_test attribute)
test-module:
    cargo test --workspace --features "module_test"

# Run only integration tests (tests in workspace-level tests/)
test-integration:
    cargo test --workspace --test '*'

# Run only doc tests
test-doc:
    cargo test --workspace --doc

# Run a specific unit test
test-unit-one name:
    cargo test {{name}} --features "unit_test" --no-default-features

# Run a specific module test
test-module-one name:
    cargo test {{name}} --features "module_test" --no-default-features

# -------------------------------
# 📊 Test Coverage
# -------------------------------

# Generate coverage report for workspace
coverage:
    cargo llvm-cov --workspace --lcov --output-path lcov.info
    cargo llvm-cov report

# Generate coverage for specific crate
coverage-crate name:
    cargo llvm-cov --package {{name}} --lcov --output-path lcov-{{name}}.info
    cargo llvm-cov report --package {{name}}

# Generate and open HTML coverage report
coverage-html:
    cargo llvm-cov --workspace --html
    @echo "Opening coverage report in browser..."
    @# Cross-platform browser opening - tries each method
    @open target/llvm-cov/html/index.html 2>/dev/null || xdg-open target/llvm-cov/html/index.html 2>/dev/null || start target/llvm-cov/html/index.html

# Generate and open HTML coverage report for specific crate
coverage-crate-html name:
    cargo llvm-cov --package {{name}} --html
    @echo "Opening coverage report in browser..."
    @open target/llvm-cov/html/index.html 2>/dev/null || xdg-open target/llvm-cov/html/index.html 2>/dev/null || start target/llvm-cov/html/index.html

# -------------------------------
# 🧪 Snapshot Testing
# -------------------------------

# Run snapshot tests for the whole workspace
snap:
    cargo insta test

# Review/accept/reject snapshot diffs
snap-review:
    cargo insta review

# Run snapshot tests for a single crate
snap-crate name:
    cargo insta test -p {{name}}

# -------------------------------
# 🔄 Pipeline Testing
# -------------------------------

# Run end-to-end pipeline tests with fixtures
test-pipeline:
    cargo test -p port_cli -- --test pipeline_tests

# Test IR generation from fixtures
test-ir:
    cargo test -p ir_builder -- --test ir_fixtures

# Test code generation from IR
test-codegen:
    cargo test -p stylus_generator -- --test codegen_fixtures

# -------------------------------
# 📝 IR Validation
# -------------------------------

# Validate IR schema on sample fixtures
validate-ir:
    @echo "Validating IR schema..."
    cargo run -p ir_schema -- validate fixtures/ir/*.yaml

# Generate IR from an Anchor example
gen-ir example="hello_world":
    @echo "Generating IR from {{example}} example..."
    cargo run -p port_cli -- --mode ir-only examples/anchor/{{example}} --output-ir target/debug/{{example}}.ir.yaml

# -------------------------------
# 📚 Documentation
# -------------------------------

# Generate and open documentation
docs:
    cargo doc --workspace --no-deps --open

# Check documentation coverage
docs-coverage:
    cargo rustdoc --workspace -- -Z unstable-options --show-coverage

# -------------------------------
# 🧹 Code Quality
# -------------------------------

# Clippy lint all crates and targets
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format all crates
fmt:
    cargo fmt --all

# Run lint, test, and fmt-check (standard check)
check:
    just fmt
    just lint
    just test
    cargo fmt --all -- --check

# Complete check with coverage and doc tests
check-full:
    just check
    just test-doc
    just coverage

# -------------------------------
# 🧠 CI Workflows
# -------------------------------

# Full CI test suite (with snapshots and coverage)
ci:
    just check
    just test-doc
    just snap
    just test-pipeline
    just validate-ir
    cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info

# -------------------------------
# 🧼 Misc
# -------------------------------

# Clean build artifacts
clean:
    cargo clean

# Show workspace dependency tree
tree:
    cargo tree --workspace

# Update Cargo.lock deps
update:
    cargo update

# Show outdated crates (requires cargo-outdated)
outdated:
    cargo outdated --workspace