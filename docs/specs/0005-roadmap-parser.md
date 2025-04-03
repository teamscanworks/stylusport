# RFC-005: Anchor Parser Roadmap

## Current Status
The basic parser is functional for program structure, instructions, account structs, and raw accounts. However, attribute parsing (particularly for account constraints) is currently incomplete.

## Implementation Priorities

### 1. Account Constraint and Field Parsing (Highest Priority)
- Complete the attribute parser to properly handle all constraint types (`init`, `mut`, `seeds`, etc.)
- Capture all fields in account structs with their types and constraints
- Parse constraint expressions (e.g., `space = 8 + 8`) in structured form
- Extend the output format to include complete account constraints

### 2. Function Body Capture (Critical)
- Store function bodies as structured AST or at minimum as raw text
- Capture macro invocations within bodies (especially `msg!`, `require!`)
- Parse common patterns like field assignments, function calls, and returns
- Enable the normalizer to extract semantic effects

### 3. Source Location Tracking (High Importance)
- Add file paths and span information to all program elements
- Track source locations for nested elements (expressions, attributes)
- Ensure traceability from parser output back to source code
- Update the output schema to include location information

### 4. Data Account Processing
- Properly identify and parse `#[account]` struct definitions
- Capture field definitions within data accounts
- Track discriminator information
- Link data accounts to their usage in account structs

### 5. Event Support
- Implement event parsing for `#[event]` structs
- Add methods to find and query events
- Parse event fields and their types
- Track event emissions in instructions with `emit!` macro

### 6. CPI Call Detection
- Add methods to detect cross-program invocations in instructions
- Parse `CpiContext` usage and target programs
- Support for analyzing CPI security patterns

### 7. Constants and Impl Blocks
- Parse impl blocks and associated constants
- Track constant values defined in impl blocks
- Support for analyzing constant usage in constraints

### 8. Space Allocation Analysis
- Enhanced parsing for "space" attribute
- Analysis of account size calculations
- Add helpers for common constraint types

### 9. Integration with IR Builder
- Connect parser output to intermediate representation
- Ensure traceability between source and IR
- Add metadata for explanation engine

## Output Schema

The parser will produce a structured representation with the following enhanced schema:

```yaml
program_modules:
 - name: string
   visibility: string
   source: { file: string, span: [start, end] }
   instructions: [...]
   
instructions:
 - name: string
   visibility: string
   parameters: [...]
   return_type: string
   context_type: string
   body: {...}  # AST or structured representation
   source: { file: string, span: [start, end] }
   
account_structs:
 - name: string
   visibility: string
   fields:
     - name: string
       ty: string
       visibility: string
       constraints:
         - kind: string (e.g., "init", "mut")
           arguments: {...}  # Structured representation
       source: { file: string, span: [start, end] }
   source: { file: string, span: [start, end] }
   
raw_accounts:
 - name: string
   visibility: string
   fields:
     - name: string
       ty: string
       visibility: string
   discriminator: string|null
   source: { file: string, span: [start, end] }
   
events:
 - name: string
   visibility: string
   fields: [...]
   source: { file: string, span: [start, end] }
```

### Implementation Phases

The parser enhancements will be implemented in phases:

- Phase 1: Account constraints and fields, function bodies, source location tracking
- Phase 2: Data account processing, event support
- Phase 3: CPI detection, constants, impl blocks, space allocation analysis

Each phase will be backward compatible, adding information without breaking existing consumers.