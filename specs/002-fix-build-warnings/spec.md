# Feature Specification: Fix Build Warnings

**Feature Branch**: `002-fix-build-warnings`  
**Created**: 2026-05-02  
**Status**: Draft  
**Input**: User description: "Review the warnings when building the application and remediate."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Clean Build Output (Priority: P1)

As a developer, I want to build the application without compiler warnings so that I can have confidence in the code quality and avoid warning fatigue.

**Why this priority**: Compiler warnings indicate potential code quality issues or dead code. A clean build ensures the codebase is maintainable and professional.

**Independent Test**: Can be fully tested by running `just build` or `cargo build` and verifying zero warnings are emitted.

**Acceptance Scenarios**:

1. **Given** the codebase has compiler warnings, **When** the developer runs `cargo build`, **Then** no warnings are emitted
2. **Given** CI runs the build, **When** `cargo build` executes, **Then** the build succeeds without warning annotations
3. **Given** new code is added that introduces warnings, **When** CI runs clippy, **Then** the lint check fails (preventing regression)

---

### User Story 2 - Maintain Code Quality Standards (Priority: P1)

As a maintainer, I want to remove unused code so that the codebase remains clean and maintainable.

**Why this priority**: Dead code increases cognitive load and makes refactoring more difficult. Removing it improves maintainability.

**Independent Test**: Can be tested by running `cargo clippy -- -Wunused` and verifying no unused code warnings.

**Acceptance Scenarios**:

1. **Given** the codebase has unused imports, **When** warnings are fixed, **Then** all imports are actively used
2. **Given** variables exist that don't need mutation, **When** warnings are fixed, **Then** unnecessary `mut` qualifiers are removed
3. **Given** code has unused parameters, **When** warnings are fixed, **Then** parameters are either used or prefixed with underscore

---

### Edge Cases

- What if fixing a warning reveals a deeper code issue?
- How do we prevent future warnings in CI?
- What happens if dependencies introduce warnings we can't control?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST emit zero warnings when running `cargo build`
- **FR-002**: System MUST pass `cargo clippy` without warnings
- **FR-003**: System MUST remove all unused imports identified by the compiler
- **FR-004**: System MUST remove unnecessary `mut` qualifiers from variables
- **FR-005**: System MUST prefix intentionally unused variables with underscore (e.g., `_progress_callback`)
- **FR-006**: System MUST preserve all existing functionality after warning remediation
- **FR-007**: System SHOULD run `just ci` successfully with no warnings

### Key Entities

- **Source Files**: Rust source files in `src/` directory that contain warnings
  - `src/export/batch.rs`: Contains unused imports and unused variable
  - `src/export/single.rs`: Contains unused imports from `std::io`
  - `src/ollama/discovery.rs`: Contains unused import `MigrationError`
  - `src/gguf/metadata.rs`: Contains unnecessary `mut` on variable

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Running `cargo build` produces exactly 0 warnings
- **SC-002**: Running `cargo clippy -- -D warnings` passes successfully  
- **SC-003**: All 6 current compiler warnings are resolved
- **SC-004**: All existing tests continue to pass (regression check)
- **SC-005**: `just ci` command completes successfully with no warnings

## Assumptions

- The warnings are all fixable without architectural changes
- No external dependencies are causing the warnings
- The code functionality is correct; only cleanup is needed
- Tests exist to verify behavior is preserved
