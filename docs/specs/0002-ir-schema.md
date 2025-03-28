# RFC-0002: Intermediate Representation

> Status: Draft
> Date: 2025-03-26
> Author: Scanworks Team  

## Summary

This document defines the structure, purpose, and design principles of the intermediate representation (IR) used in the first iteration of `Port` to translate Solana Anchor programs into Stylus-compatible Rust.  
The IR acts as the semantic contract between parsing, analysis, explanation, and code generation phases.  
It is serialized as YAML and designed to support traceability, explainability, snapshot testing, and eventually multi-backend support.

---

## Design Principles

- **Semantic-first.** The IR captures what the program means, not how it was written. Macros, derives, and sugar are desugared into explicit constructs. For example, `#[account(init)]` becomes an `AllocateAccount` operation with a `payer`.

- **Source-linked.** Each IR node carries source metadata (file, span, construct name). This supports diagnostics, traceability, and inline explainability across the pipeline.

- **Explicit.** The IR includes only explicitly derived or transformed constructs. It does not rewrite or correct programs. Ambiguities or non-portable patterns are preserved and surfaced.

- **Cross-phase contract.** The IR is the stable interface between parsing, normalization, explanation, and code generation. It enables fixture-based testing and independent validation per stage.

- **Serializable.** The IR is represented as structured YAML. This supports snapshot testing, external tooling, and manual inspection.

- **Versioned.** The IR format is versioned to allow non-breaking evolution. Tools consuming IR must check and validate version compatibility.

- **Target-neutral.** The IR is designed to support Stylus as a backend, but does not assume it. As much as possible, the structure is general enough to support other Rust-based smart contract platforms in the future.

---

## IR Structure Overview

```yaml
- version            # IR schema version (integer or string, e.g. 1 or "1.0")

- Program
  - name               # Program name (usually derived from crate or declared module)
  - instructions       # List of instruction identifiers
  - account_structs    # List of reusable account struct identifiers
  - constants          # List of constant identifiers (e.g., seeds, fees)
  - errors             # List of error identifiers
  - events             # List of event identifiers
  - source             # Optional traceability info (file + span)
  - metadata           # Optional non-semantic metadata (program_id, source_root, etc.)

- Instruction
  - name               # Instruction name
  - args               # Function arguments (name, type, optional doc)
  - returns            # Optional return type (e.g., "u64", "WithdrawResult", "()" by default)
  - account_struct     # Identifier of the account struct used as context
  - effects            # List of semantic operations describing the instruction body (see below)
  - doc                # Optional doc comment from the source
  - source             # Optional file and span of the original function declaration

 - AccountStruct
  - name               # Name of the account struct (e.g., "Initialize")
  - accounts           # Ordered list of account entries (see Account)
  - doc                # Optional doc comment attached to the struct
  - source             # Optional file and span of the original account struct declaration

- Account
  - name               # Field name within the struct
  - type               # Canonical account type (e.g., AccountInfo, TokenAccount, Program)
  - mutability         # Whether the account must be mutable (true/false)
  - signer             # Whether the account must sign the transaction (true/false)
  - optional           # Whether the account is optional (true/false)
  - bump               # Optional boolean flag (true if bump is expected)
  - space              # Optional integer or string (bytes allocated, if known)
  - constraints        # List of declarative constraints
  - doc                # Optional doc comment attached to the field

- Constraint
  - kind               # Constraint variant (e.g., RequireSigner, Init, HasOne)
  - target             # Account to which the constraint applies
  - arguments          # Variant-specific fields (defined per kind)
  - doc                # Optional explanation or source comment

    Constraint.kind Variants:
      - RequireSigner     # Account must be a transaction signer
      - CheckOwner        # Account must be owned by a specific program
      - HasOne            # Field in the account must match another account's pubkey
      - Seeds             # PDA derivation seeds for the account
      - Bump              # Optional bump seed in PDA derivation
      - Init              # Account creation parameters (payer, space, owner)
      - Close             # Account will be closed and lamports refunded

- Effect
  - kind               # Effect variant (e.g., Log, Return, Block)
  - explanation        # Optional human-readable description
  - source             # Optional file and span metadata for traceability
    
    Effect.kind Variants:
      - Log: message
      - Return: value
      - Require: condition, message
      - AllocateAccount: target, space, payer
      - Transfer: from, to, amount
      - Block: source (raw code block)

- Constant
  - name               # Constant name
  - type               # Literal type (e.g., u64, string)
  - value              # Value as string or expression

- Error
  - code               # Numeric error code
  - name               # Error variant name
  - message            # Optional explanation

- Event
  - name               # Event type name
  - fields             # Named fields with types and optional doc
  - discriminator      # Optional on-chain tag
  - doc                # Optional type-level documentation
```

---

## Sections of the Intermediate Representation

This section details and comments on each of the sections of the IR.

### Versioning

The IR format includes a top-level `version` field. Tools and test fixtures must explicitly check and validate version compatibility.

```yaml
version: 1
```

A future versioning scheme may adopt semantic versioning:

```yaml
version: 1.0
```

### Program

The Program node is the root of the IR and defines the public interface of the on-chain program. It contains only references to top-level definitions — the actual definitions appear in their own top-level sections.

The IR builder is responsible for ensuring referential integrity: each identifier listed in instructions, accounts, etc., must correspond to a uniquely named definition elsewhere in the IR document.

Ordering should reflect the original source order where possible (for traceability and diffing).
Metadata has no semantic effect — it may be omitted by consumers that don’t require traceability or diagnostics.

The `metadata` field is an optional map of non-semantic annotations. It may include:

- `program_id`: Declared or inferred address of the program (string)
- `entry_file`: Path to the file where the program entrypoint is defined
- `source_root`: Root of the source tree relative to the workspace
- `origin`: One of: `Declared`, `Extracted`, `Synthesized`
Additional keys may be present for custom tooling. Crates **must not fail** when encountering unknown metadata keys.

### AccountStruct and Account

Each `AccountStruct` represents a named group of accounts used as the context for an instruction. It corresponds to a `#[derive(Accounts)]` struct in the Anchor source. The struct contains an ordered list of account entries, each of which defines the required properties and constraints for one account passed to the instruction.

`Account` entries are local to the struct and are not globally identified. Each entry captures the account’s name, type, mutability, signer requirement, optionality, and constraint list.

`Account.type` is a normalized string drawn from the following domains:
- Canonical wrappers: `AccountInfo`, `UncheckedAccount`, `Program`, `TokenAccount`, `Mint`, etc.
- User-defined types: any struct defined in the program (e.g., `Vault`, `State`, `Config`)
- Program marker types: `System`, `Token2022`, etc.

The type should be desugared and normalized. Generic wrappers like `Account<'info, Vault>` become just `Vault`.

The `optional` field indicates whether an account may be absent at runtime.

This is a semantic property derived from the use of `Option<...>` in the source.  
It affects runtime validation and explainability — the account may be `None` and must be handled accordingly.

### Instruction

Each `Instruction` represents a single entrypoint into the program.  
It corresponds to a function in the original Anchor source and models both its interface and behavior.

Each item in `args` is a function parameter derived from the instruction signature.

- name: Name of the argument (string)
- type: Normalized type string (e.g., "u64", "Pubkey", "Option<bool>")
- doc: Optional documentation comment, if present in source

The semantics of an instruction is defined by its effects field — an ordered list of operations describing runtime behavior. See the Effect section for details.

Notes:

- `account_struct` must reference the name of an account struct defined in the `accounts` section.
- Argument ordering must match the source program.
- Constraints are attached to the account struct — not duplicated at the instruction level.
- If semantic lowering is incomplete, a `Block` effect should be used to preserve the original source code as a fallback.


### Effect

Each Effect represents a semantic operation executed within an instruction.  
Effects are modeled as tagged objects with a kind and variant-specific fields.  
They may originate from user-written code, derived constraints, or fallback lowering.

- kind: the variant name (e.g., Log, Return, Block)
- explanation: optional human-readable annotation
- source: optional source metadata (file and span)
- other fields: depend on the variant

The following variants are currently defined:

```yaml
- Effect
  - kind: Log                            # Emits a message to the on-chain log
  - message: <string>                    # Message string to log
  - explanation: <optional string>
  - source:
      file: <path>                       # Source file path
      span: [<start>, <end>]             # Byte or line span
```

```yaml
- Effect
  - kind: Return                         # Terminates execution with a return value
  - value: <any>                         # Return value (structured literal or object)
  - explanation: <optional string>
  - source:
      file: <path>
      span: [<start>, <end>]
```

```yaml
- Effect
  - kind: Require                        # Asserts that a condition holds at runtime
  - condition:                           # Logical condition to evaluate
      type: <expression type>            # e.g., EqualityCheck or RawExpression
      left_operand: <string>
      right_operand: <any>
  - message: <string>                    # Error message if assertion fails
  - explanation: <optional string>
  - source:
      file: <path>
      span: [<start>, <end>]
```

```yaml
- Effect
  - kind: AllocateAccount                # Allocates space and assigns payer for a new account
  - target: <account_name>               # The account to allocate
  - space: <u64>                         # Number of bytes to allocate
  - payer: <account_name>                # Account funding the allocation
  - explanation: <optional string>
  - source:
      file: <path>
      span: [<start>, <end>]
```

```yaml
- Effect
  - kind: Transfer                       # Transfers lamports between accounts
  - from: <account_name>                 # Sender account
  - to: <account_name>                   # Recipient account
  - amount: <literal or variable name>   # Amount to transfer
  - explanation: <optional string>
  - source:
      file: <path>
      span: [<start>, <end>]
```

```yaml
- Effect
  - kind: Block                          # Fallback: embeds raw source when semantic lowering is unavailable
  - source: |
      {
          if !ctx.accounts.flagged {
              msg!("safe to proceed");
          }
      }                                  # Raw source block as string
  - explanation: <optional string>
  - source:
      file: <path>
      span: [<start>, <end>]
```


### Constraint

Each `Constraint` represents a declarative rule attached to an account within an AccountStruct.
Constraints are desugared from Anchor attributes (e.g., `#[account(...)]`) and model checks or initializations such as signer requirements, ownership validation, PDA derivation, and space allocation. Each constraint is a variant with its own expected arguments.

Below, we give a breakdown of the constraints and the values of their respective fields:

```yaml
- Constraint
  - kind: RequireSigner         # Account must sign the transaction
  - target: <account_name>
  - arguments: {}
  - doc: <optional>
```

```yaml
- Constraint
  - kind: CheckOwner            # Account must be owned by another account (usually a program)
  - target: <account_name>
  - arguments:
      expected: <account_name>
  - doc: <optional>
```

```yaml
- Constraint
  - kind: HasOne                # Account field must point to the same pubkey as another account
  - target: <account_name>
  - arguments:
      field: <field_name>
  - doc: <optional>
```

```yaml
- Constraint
  - kind: Seeds                 # PDA derivation seeds
  - target: <account_name>
  - arguments:
      seeds: [<expr>, <expr>, ...]
  - doc: <optional>
```

```yaml
- Constraint
  - kind: Bump                  # Indicates use of bump seed in PDA derivation
  - target: <account_name>
  - arguments:
      value: <u8> (optional)
  - doc: <optional>
```

```yaml
- Constraint
  - kind: Init                  # Declares that the account will be created during the instruction
  - target: <account_name>
  - arguments:
      payer: <account_name>
      space: <u64>
      owner: <account_name> (optional)
  - doc: <optional>
```

```yaml
- Constraint
  - kind: Close                 # Declares that the account will be closed and refunded to another
  - target: <account_name>
  - arguments:
      refund_to: <account_name>
  - doc: <optional>
```

### Constant

Each `Constant` represents a named compile-time value declared in the source program.
These are typically used in seeds, instruction arguments, or configuration values. The value is preserved as a string and may represent a literal or a symbolic expression.

### Error

Each Error represents a program-specific error variant.
It is identified by a numeric code and a name, and may include an optional message for user-facing diagnostics. These are desugared from Anchor’s #[error_code] declarations or generated by convention.

### Event

Each `Event` represents a type that may be emitted by the program at runtime using `emit!`.
It contains an ordered list of named fields and may include a custom discriminator to support on-chain log decoding. Events are used for off-chain indexing, ABI generation, and runtime auditability.

Type must match a Rust-serializable type supported in logs.
Examples: `u64`, `Pubkey`, `bool`, `Option<u8>`.

---

## Source Metadata

Each IR node may include an optional `source` field:

```yaml
source:
  file: programs/token/src/lib.rs
  span: [120, 158]
```

The `span` typically encodes a source range as either `[line, column]` or `[start_byte, end_byte]`, depending on the implementation.

---

## IR Normalization and Canonicalization

This section defines how IR documents are normalized to ensure consistent snapshot testing, diffability, and reproducibility across tools.

- Field ordering: All YAML object fields must be ordered lexicographically unless otherwise specified.
- Optional fields: Fields that are unset should be omitted from the serialized output. Nulls are never emitted.
- List ordering:
  - accounts, args, fields, constraints, and effects must preserve original source order where known.
  - Top-level sections (instructions, account_structs, etc.) should be sorted by name.
- Empty collections: Empty lists may be omitted or serialized as []. Consumers must handle both.
- Version field: All IR documents must include a version key at the root.
- Key ordering in constraints and effects: Within arguments, keys must be ordered lexicographically.

This section ensures that two semantically equivalent IRs produce identical snapshots and simplifies tooling and diffing.

---

## Open Questions and Future Work

- Complete the list of effects.
- Model ReturnEffect.value as a structured expression, and validate that its type matches Instruction.returns.
- Support for preserving source module structure (e.g., file, mod path) for definitions to improve visibility scoping and multi-module output.
- Clarify the usage of relative names (`user`) versus absolute names (`account_struct.name`).
- Distinguish between user-authored and constraint-lowered effects (e.g., via an origin tag) to enhance traceability and documentation fidelity.