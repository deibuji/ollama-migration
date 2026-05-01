# High Performance Tooling (Rust) Constitution
<!-- Project governance document for Rust-based high-performance tooling development -->

<!--
SYNC IMPACT REPORT
==================
Version Change: 0.0.0 → 1.0.0 (initial ratification)
Modified Principles: All principles initialized from template placeholders
Added Sections:
  - I. Zero-Cost Abstractions
  - II. Memory Safety Without GC
  - III. Performance-First Benchmarking
  - IV. Unsafe Code Audit Trail
  - V. CLI-First Design
  - Toolchain & Ecosystem Standards
  - Development Workflow
Governance: Full framework established
Templates Requiring Updates:
  ✅ .specify/templates/plan-template.md - No changes required (generic)
  ✅ .specify/templates/spec-template.md - No changes required (generic)
  ✅ .specify/templates/tasks-template.md - No changes required (generic)
  ✅ .specify/templates/commands/*.md - No changes required (generic)
Follow-up TODOs:
  - TODO(RATIFICATION_DATE): Confirm actual project start date when known
-->

## Core Principles

### I. Zero-Cost Abstractions
All abstractions must compile to equivalent or better machine code than hand-written implementations. No runtime overhead for generic code—monomorphization preferred over dynamic dispatch unless explicitly justified with benchmarks. Traits used for compile-time polymorphism; trait objects only when dynamic dispatch is measurably necessary.

**Rationale**: Rust's core promise is providing high-level ergonomics without sacrificing performance. Violating this undermines the language choice.

### II. Memory Safety Without GC
Leverage ownership, borrowing, and lifetimes exclusively. No garbage collection, reference counting only when ownership cannot be statically determined. Prefer stack allocation; heap allocation must be justified. Interior mutability via `Cell`/`RefCell` only in single-threaded contexts; `Mutex`/`RwLock` for shared state.

**Rationale**: Eliminating GC pauses is essential for predictable high-performance tooling where latency matters.

### III. Performance-First Benchmarking
Every performance-sensitive feature requires accompanying criterion.rs benchmarks. Regression thresholds enforced in CI: any change degrading throughput or increasing latency by >5% requires explicit approval. Profile-guided optimization (PGO) applied to release builds. Memory profiling via heaptrack/valgrind for allocations.

**Rationale**: What isn't measured degrades. Performance claims must be reproducible and continuously verified.

### IV. Unsafe Code Audit Trail
`unsafe` blocks permitted only for FFI, SIMD intrinsics (when stable SIMD insufficient), or proven-hot paths where safe Rust cannot match performance. Each `unsafe` block requires:
- Detailed safety comment explaining invariants maintained
- Unit test specifically exercising the unsafe path
- Review by second maintainer
- Tracking in UNSAFE_AUDIT.md log

**Rationale**: Unsafe code is a liability; its usage must be deliberate, minimal, and auditable.

### V. CLI-First Design
All tooling exposes functionality via command-line interface with structured output (JSON, TOML) and human-readable formats. CLI flags follow POSIX conventions where applicable. stdin/stdout for streaming, stderr for diagnostics. Exit codes meaningful (0 = success, 1 = error, 2-255 = specific error categories).

**Rationale**: High-performance tooling is consumed by scripts, CI/CD, and other tools. Programmatic interface is primary; human interface secondary.

## Toolchain & Ecosystem Standards

- **Rust Version**: Latest stable; MSRV (Minimum Supported Rust Version) documented in Cargo.toml, updated only with minor version bumps
- **Edition**: 2021 or later
- **Formatter**: rustfmt with project-specific rustfmt.toml
- **Linter**: clippy with `#![deny(warnings)]` in root; custom linting rules for unsafe detection
- **Dependencies**: Prefer stdlib; external crates require security audit (cargo-audit) and license compatibility check
- **Build System**: Cargo with workspace organization for multi-crate projects
- **Testing**: Built-in test framework + criterion.rs for benchmarks + cargo-fuzz for fuzzing

## Development Workflow

- **Code Review**: All PRs require one approval; unsafe code changes require two approvals
- **CI Gates**: `cargo build --release`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, benchmark regression detection
- **Documentation**: rustdoc for all public APIs; README with quickstart; ARCHITECTURE.md for design decisions
- **Releases**: Semantic versioning enforced; CHANGELOG.md maintained; GitHub releases with prebuilt binaries for major platforms

## Governance

This constitution governs all development decisions. When trade-offs arise, principles are prioritized in order listed (I-V). Amendments require:
1. Written proposal with rationale and impact analysis
2. Discussion period of 48 hours minimum
3. Approval by majority of active maintainers
4. Version bump and changelog entry

Violations discovered in existing code must be documented as technical debt with remediation plan.

**Version**: 1.0.0 | **Ratified**: TODO(RATIFICATION_DATE) | **Last Amended**: 2026-05-01
