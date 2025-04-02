# Roadmap

## Anchor parser

### Current Status
The basic parser is functional for program structure, instructions, account structs, and raw accounts. However, attribute parsing (particularly for account constraints) is currently incomplete.

### Implementation Priorities

#### 1. Account Attribute Parsing
- Fix the parser to correctly handle `mut` and other keywords in account attributes
- Update the attribute parser to recognize special keywords vs value constraints
- Add comprehensive tests for attribute parsing

#### 2. Instruction Parameter Detection
- Add methods to extract and validate function parameters 
- Implement parameter type checking
- Support for checking parameter existence and types

#### 3. Event Support
- Implement event parsing for `#[event]` structs
- Add methods to find and query events
- Parse event fields and their types
- Track event emissions in instructions with `emit!` macro

#### 4. CPI Call Detection
- Add methods to detect cross-program invocations in instructions
- Parse `CpiContext` usage and target programs
- Support for analyzing CPI security patterns

#### 5. Account Field Analysis
- Add support for account field detection in raw accounts
- Implement field type resolution
- Add methods to query field properties

#### 6. Constants and Impl Blocks
- Parse impl blocks and associated constants
- Track constant values defined in impl blocks
- Support for analyzing constant usage in constraints

#### 7. Space Allocation Analysis
- Enhanced parsing for "space" attribute
- Analysis of account size calculations
- Add helpers for common constraint types

#### 8. Integration with IR Builder
- Connect parser output to intermediate representation
- Ensure traceability between source and IR
- Add metadata for explanation engine

### Testing Strategy
- Start with basic structure tests (current approach)
- Gradually add constraint testing as parser is enhanced
- Implement snapshot testing for complex structures
- Create specialized tests for edge cases

### Documentation
- Document parser API for each feature as implemented
- Add examples for common parsing patterns
- Create developer guide for extending the parser