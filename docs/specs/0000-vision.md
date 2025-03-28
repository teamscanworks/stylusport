# RFC-0000: Project Vision

> Status: Final
> Date: 2025-03-25  
> Author: Scanworks Team  
---

## Summary

This document provides an overview of `StylusPort`, including its goals, objectives, scope, deliverables, and timeline.  
Note this is a working document which can be adapted as the project evolves.

---

## Goals

The general objective of `StylusPort` is to facilitate the translation between Solana programs and Stylus-compatible programs.

`StylusPort` pursues the two following goals:

- (Enable-migration) Enable seamless migration of Solana programs and protocols to Stylus WASM-compatible contracts.  
- (Enhance-security) Enhance the security and quality of Stylus contracts by promoting best practices and robust migration patterns through the `StylusPort` framework.

---

## Objectives

### For Goal 1 (Enable-migration)

The following objectives are meant to be achieved during the first phase of the project, which will last 3 months:

- Develop and release a production-ready toolkit in the form of a Command-Line Interface (CLI) or editor assistant.  
- Provide full automated support for core Solana programs, including:  
  - Basic Solana programs  
  - Token programs and token extensions  
  - Oracles  
- Focus exclusively on programs written in Anchor  
- Ensure the toolkit can handle all example programs in the solana-developers repository for the supported frameworks. It represents the minimum bar for releasing the beta.  
- Provide migration assistance, including:  
  - Automated code snippets for migration  
  - Project scaffolding for Stylus-compatible contracts  
  - Identification of migration pitfalls and potential issues  
  - User guidance for arbitrary programs, including edge cases and non-standard implementations  
- Create detailed documentation for the toolkit, including:  
  - Step-by-step usage guides for each of the solana-developers example programs  
  - Tutorials covering common migration scenarios and advanced use cases  

### For Goal 2 (Enhance-security)

- Identify code patterns that could lead to insecure contracts during migration by:  
  - Analyzing patterns of code that are not easily translatable from Solana to Stylus and providing safe alternatives, possibly leveraging static analysis tools  
  - Detecting security checks enforced by the Solana VM or compiler that need to be replicated or adapted for Stylus  
  - Establishing a set of test cases to evaluate the effectiveness of the code pattern analysis and migration guidance  
- Provide migration guidance that helps users avoid security pitfalls by:  
  - Highlighting risky migration scenarios if and where automatic translation might introduce security vulnerabilities  
  - Suggesting manual code adjustments or additional security measures in scenarios where Solana’s security checks might not directly translate to Stylus  
  - Defining benchmarks for security guidance effectiveness, aiming for a false positive rate below X% and a false negative rate below Y%  
- Implement proactive warnings and suggestions in the CLI tool, including:  
  - Flagging patterns that require developer attention for security alignment  
  - Providing migration-specific advice with explanatory context to help developers understand why a code pattern is risky  
- Document security-focused migration patterns and examples in the technical handbook, specifically addressing:  
  - How to handle non-translatable code patterns securely  
  - Best practices for replicating Solana’s security checks in Stylus  
  - Detailed examples demonstrating secure migration practices for high-risk scenarios  
- Measure the effectiveness of security guidance through:  
  - Continuously tracking security vulnerabilities in Stylus programs and assessing their impact / feasibility on the generated code  
  - Conducting periodic security assessments of migrated contracts to validate the CLI tool’s guidance  
  - Collecting community feedback on security suggestions and using it to fine-tune pattern recognition algorithms  

---

## Scope

**Inclusions**

- A production-ready toolkit to facilitate the migration of Solana programs to Stylus WASM-compatible contracts  
- Development of a toolkit as a CLI or editor assistant to automate and assist in the migration process  
- Full support for migration of the following program types:  
  - Basic Solana programs  
  - Token programs and token extensions  
  - Oracles  
  - Programs written in Anchor  
- Migration assistance includes:  
  - Project scaffolding for Stylus-compatible contracts  
  - Automated code snippets for migration with either translated code or code section to be filled  
  - Identification of migration pitfalls and potential issues  
  - User guidance  
  - Detailed documentation  
  - Step-by-step usage guides for each example program in the solana-developers repository  
  - Tutorials covering common migration scenarios and advanced use cases  
  - Security-focused migration patterns and examples  

**Exclusions**

- Support for Solana programs written outside the Anchor framework including those written in languages other than Rust, such as TypeScript or Python.  
- Automated migration support for highly custom or non-standard Solana programs beyond the provided examples.
- Post-migration support services, including maintenance or ongoing developer support.
- Support for third-party integrations not listed in the project objectives.
- Compatibility of the libraries used by programs.

---

## Assumptions

- Developers using the toolkit will have good knowledge of Solana and at least a basic knowledge of Stylus  
- Community contributions will support the expansion and enhancement of the toolkit post-release  
- The Arbitrum VM will provide comparable security features to those of the Solana VM, enabling the replication of security checks  