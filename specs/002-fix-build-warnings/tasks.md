# Implementation Tasks: Fix Build Warnings

**Branch**: `002-fix-build-warnings` | **Date**: 2026-05-02 | **Plan**: [plan.md](./plan.md)

## Phase 2.1: Fix Unused Imports in batch.rs

- [ ] **T2.1.1** Remove unused `HashMap` import from `src/export/batch.rs`
  - Remove `use std::collections::HashMap;` from line 1
  - Verify build still compiles
  - Estimated: 5 min

- [ ] **T2.1.2** Remove unused `JobStatus` import from `src/export/batch.rs`
  - Change `use super::job::{JobStatus, MigrationJob};` to `use super::job::MigrationJob;`
  - Verify build still compiles
  - Estimated: 5 min

## Phase 2.2: Fix Unused Imports in single.rs

- [ ] **T2.2.1** Remove unused `io` and `Read` imports from `src/export/single.rs`
  - Change `use std::io::{self, Read, Write};` to `use std::io::Write;`
  - Verify build still compiles
  - Estimated: 5 min

## Phase 2.3: Fix Unused Import in discovery.rs

- [ ] **T2.3.1** Remove unused `MigrationError` import from `src/ollama/discovery.rs`
  - Change `use crate::error::{MigrationError, Result};` to `use crate::error::Result;`
  - Verify build still compiles  
  - Estimated: 5 min

## Phase 2.4: Fix Unused Variable in batch.rs

- [ ] **T2.4.1** Prefix unused `progress_callback` parameter with underscore
  - Change `progress_callback: Option<ProgressCallback>,` to `_progress_callback: Option<ProgressCallback>,`
  - This indicates intentional non-use (feature not yet implemented)
  - Verify build shows no unused variable warning
  - Estimated: 5 min

## Phase 2.5: Fix Unnecessary Mut in metadata.rs

- [ ] **T2.5.1** Remove unnecessary `mut` qualifier from `meta` variable
  - Change `let mut meta: GGUFMetadata = header.try_into()?;` to `let meta: GGUFMetadata = header.try_into()?;`
  - Verify build shows no "unused_mut" warning
  - Estimated: 5 min

## Phase 2.6: Fix Clippy Warnings in main.rs

- [ ] **T2.6.1** Change `&PathBuf` to `&Path` for function parameters
  - Update `cmd_list`, `cmd_export`, `cmd_export_all`, `cmd_info`, and `cmd_verify` signatures
  - Change import from `use std::path::PathBuf;` to `use std::path::{Path, PathBuf};`
  - Estimated: 10 min

- [ ] **T2.6.2** Fix `useless_asref` warning in `cmd_export`
  - Change `.as_ref().map(|p| p.clone())` to `.clone()`
  - Estimated: 5 min

## Phase 2.7: Fix Collapsible If in discovery.rs

- [ ] **T2.7.1** Collapse nested if-let into else-if chain
  - Combine `else if path.is_file() { if let Some... }` into `else if path.is_file() && let Some...`
  - Estimated: 5 min

## Phase 2.8: Verification & Regression Testing

- [ ] **T2.8.1** Run full build verification
  - Run `cargo build` and confirm zero warnings
  - Run `cargo clippy -- -D warnings` and confirm passes
  - Estimated: 5 min

- [ ] **T2.8.2** Run regression tests
  - Run `cargo test` and confirm all tests pass
  - Estimated: 10 min

- [ ] **T2.8.3** Run CI check
  - Run `just ci` and confirm passes
  - Estimated: 30 sec

## Summary

| Phase | Tasks | Est. Time | Status |
|-------|-------|-----------|--------|
| Fix batch.rs imports | 2 | 10 min | ✅ Complete |
| Fix single.rs imports | 1 | 5 min | ✅ Complete |
| Fix discovery.rs import | 1 | 5 min | ✅ Complete |
| Fix batch.rs parameter | 1 | 5 min | ✅ Complete |
| Fix metadata.rs mut | 1 | 5 min | ✅ Complete |
| Fix main.rs clippy | 2 | 15 min | ✅ Complete |
| Fix collapsible if | 1 | 5 min | ✅ Complete |
| Verification & Testing | 3 | 15 min 30 sec | ✅ Complete |

**Total**: 12 tasks, completed

**Final Verification Command**:
```bash
just ci  # Should pass with no warnings
```
