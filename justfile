# Default: show available commands
default:
    @just --summary

# -------------------------------
# 🧱 Build & Run (Generic)
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
# 🧪 Tests (Generic + Common)
# -------------------------------

# Test entire workspace
test:
    cargo test --workspace

# Test any crate
test-crate name:
    cargo test -p {{name}}

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
# 🧹 Code Quality
# -------------------------------

# Clippy lint all crates and targets
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format all crates
fmt:
    cargo fmt --all

# Run lint, test, and fmt-check (CI-mode)
check:
    just fmt
    just lint
    just test
    cargo fmt --all -- --check

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