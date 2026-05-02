# Implementation Plan: Fix Build Warnings

**Branch**: `002-fix-build-warnings` | **Date**: 2026-05-02 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/002-fix-build-warnings/spec.md`

## Summary

Remediate all compiler warnings in the ollama-migrator codebase to achieve a clean build. The current build emits 6 warnings: 4 unused imports, 1 unused variable, and 1 unnecessary `mut` qualifier. All fixes are straightforward code cleanup without functional changes.

## Technical Context

**Language/Version**: Rust 1.75+  
**Build System**: Cargo  
**CI Tool**: just (task runner)  
**Project Type**: CLI application  
**Constraints**: No functional changes; only warning remediation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Zero-Cost Abstractions | ✅ Pass | No runtime changes, compile-time only |
| II. Memory Safety Without GC | ✅ Pass | No unsafe code modifications |
| III. Performance-First Benchmarking | ✅ Pass | No performance impact; compile-time only |
| IV. Unsafe Code Audit Trail | ✅ Pass | No unsafe code involved |
| V. CLI-First Design | ✅ Pass | No CLI changes |

**Violations:** None

## Project Structure

### Documentation (this feature)

```text
specs/002-fix-build-warnings/
├── plan.md              # This file
├── spec.md              # Feature specification
├── tasks.md             # Implementation tasks
└── contracts/
    └── (none needed for this feature)
```

### Source Code Changes

```text
src/
├── export/
│   ├── batch.rs         # Remove unused imports, fix unused parameter
│   └── single.rs        # Remove unused imports
├── ollama/
│   └── discovery.rs     # Remove unused import
└── gguf/
    └── metadata.rs      # Remove unnecessary `mut`
```

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

N/A - No violations detected.

## Phase 0: Research

### Warning Analysis

Running `cargo build` reveals 6 warnings:

| File | Line | Warning Type | Message |
|------|------|--------------|---------|
| `src/export/batch.rs` | 1 | unused import | `std::collections::HashMap` |
| `src/export/batch.rs` | 9 | unused import | `JobStatus` |
| `src/export/batch.rs` | 47 | unused variable | `progress_callback` |
| `src/export/single.rs` | 2 | unused imports | `std::io::{self, Read}` |
| `src/ollama/discovery.rs` | 3 | unused import | `MigrationError` |
| `src/gguf/metadata.rs` | 31 | unused mut | `mut meta` |

### Root Cause Analysis

1. **Unused imports**: These were likely added during development but the code that used them was removed or refactored. The imports remained as artifacts.

2. **Unused variable**: `progress_callback` in `batch.rs` is passed to `export_all()` but never used because batch export doesn't implement progress feedback yet. The parameter should be prefixed with `_` to indicate intentional non-use.

3. **Unnecessary mut**: In `metadata.rs`, the `meta` variable is declared as `mut` but never mutated after assignment. The `mut` qualifier can be removed.

### Technology Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Fix Method | Manual edit | Compiler suggestions are straightforward; automated fix with `cargo fix` would work too |
| Verification | `cargo build` + `cargo clippy` | Confirm zero warnings after fixes |
| Regression Testing | `cargo test` | Ensure no functionality broken |

## Phase 1: Design

### Fix Strategy

1. **Unused imports**: Remove entirely or use `#[allow(unused_imports)]` if intentional
2. **Unused variable**: Prefix with underscore: `_progress_callback`
3. **Unnecessary mut**: Remove `mut` keyword

### Code Changes

#### Change Set 1: `src/export/batch.rs`

**Remove line 1:**
```rust
use std::collections::HashMap;
```

**Remove from line 9:**
```rust
use super::job::{JobStatus, MigrationJob};
```
→ 
```rust
use super::job::MigrationJob;
```

**Modify line 47:**
```rust
progress_callback: Option<ProgressCallback>,
```
→
```rust
_progress_callback: Option<ProgressCallback>,
```

#### Change Set 2: `src/export/single.rs`

**Modify line 2:**
```rust
use std::io::{self, Read, Write};
```
→
```rust
use std::io::Write;
```

#### Change Set 3: `src/ollama/discovery.rs`

**Modify line 3:**
```rust
use crate::error::{MigrationError, Result};
```
→
```rust
use crate::error::Result;
```

#### Change Set 4: `src/gguf/metadata.rs`

**Modify line 31:**
```rust
let mut meta: GGUFMetadata = header.try_into()?;
```
→
```rust
let meta: GGUFMetadata = header.try_into()?;
```

### Interface Contract

No interface changes. All changes are internal cleanup.

### Quick Start

After implementing:
```bash
just build      # Should show 0 warnings
just clippy     # Should pass with no warnings
just test       # Should pass all tests
```

## Phase 2: Task Planning

Tasks are documented in [tasks.md](./tasks.md).

## Re-evaluation Post-Design

**Constitution Check (Post-Design):**

All principles remain satisfied. No architectural changes needed.
