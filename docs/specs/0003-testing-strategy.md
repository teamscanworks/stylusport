# RFC-0003: Testing Strategy

> Status: Draft
> Date: 2025-03-29
> Author: Scanworks Team

## Summary

This document outlines the testing strategy for `StylusPort`, a modular pipeline for translating Solana Anchor programs to Stylus-compatible Rust. The testing approach is designed around the central IR (Intermediate Representation) and emphasizes semantic fidelity, explainability, and reliability across all stages of the pipeline.

---

## Motivation

Given the complexity of translating between smart contract frameworks with different execution models, a comprehensive testing strategy is essential to:

- Ensure semantic fidelity between source and target programs
- Provide confidence in translation correctness
- Detect regressions early
- Document expected behavior
- Facilitate collaborative development

---

## Testing Principles

- **IR-centric:** Tests focus on correct transformation to and from the central IR
- **Snapshot-based:** Expected outputs are captured and versioned
- **Modular:** Each crate is tested independently
- **End-to-end:** Complete pipeline tests validate the full workflow
- **Explainable:** Tests document expected behavior and outcomes
- **Coverage-aware:** Test coverage is measured and monitored

---

## Test Types

### 1. Unit Tests

Unit tests verify the behavior of individual functions and components within each crate.

**Characteristics:**
- Focus on isolated components
- Mock dependencies when necessary
- Test edge cases and error conditions
- Verify correct internal behavior

**Examples:**
- Testing individual constraint normalization functions
- Verifying correct parsing of specific Anchor constructs
- Testing effect generation for specific instruction patterns

**Location:**
- Within each crate's `src` directory in `#[cfg(test)]` modules or in `tests/unit/` directories

### 2. Integration Tests

Integration tests verify the interaction between different components and stages of the pipeline.

**Characteristics:**
- Test interactions between crates
- Verify proper data flow through the pipeline
- Test with realistic inputs

**Examples:**
- Testing parsing and IR generation for complete programs
- Verifying IR to Stylus code generation
- Testing error propagation across boundaries

**Location:**
- In each crate's `tests/` directory
- Cross-crate tests in the `port_cli` crate

### 3. Snapshot Tests

Snapshot tests capture expected outputs to detect unintended changes and document expected behavior.

**Characteristics:**
- Generate deterministic outputs
- Compare against stored reference outputs
- Version controlled to track intentional changes

**Examples:**
- IR generation from Anchor source
- Stylus code generated from IR
- Explanation annotations for specific constructs

**Tools:**
- Uses `insta` crate for snapshot testing
- Snapshots stored in `tests/snapshots/` directories

### 4. Doc Tests

Doc tests provide executable examples in documentation comments that serve both as documentation and verification.

**Characteristics:**
- Demonstrate correct API usage
- Document expected behavior
- Serve as living examples
- Show common patterns and edge cases

**Examples:**
- API usage examples for each crate
- IR structure examples for common Anchor patterns
- Pipeline usage demonstrations
- Edge case handling illustrations

**Implementation:**
- Added to public API functions with `///` comments
- Runnable with `cargo test --doc`
- Integrated into CI pipeline

### 5. Coverage Testing

Coverage testing measures the extent to which the codebase is exercised by tests.

**Characteristics:**
- Identifies undertested components
- Helps ensure comprehensive test coverage
- Provides metrics for quality assessment

**Tools:**
- Uses `cargo-llvm-cov` for coverage reporting
- Generates both summary reports and detailed HTML views
- Includes line, branch, and function coverage metrics

---

## Test Fixtures

Fixtures provide standardized inputs for tests at various stages of the pipeline.

### Anchor Source Fixtures

- Located in `fixtures/anchor/`
- Includes minimal examples and complete programs
- Covers common patterns and edge cases
- Sourced from actual Solana programs when possible

### IR Fixtures

- Located in `fixtures/ir/`
- YAML representations of parsed and normalized programs
- Used for testing IR validation and code generation
- Includes examples of all supported constructs

### Expected Output Fixtures

- Located in `fixtures/stylus/`
- Reference Stylus code for comparison
- Used for validation of code generation
- Includes expected comments and explanations

---

## Testing Workflow

### Local Development

1. Run unit tests for the crate being modified: