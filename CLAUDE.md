# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust project called "ruzzle" that appears to be focused on Advent of Code solutions. The project uses Rust 2024 edition and has a modular structure organized around different programming challenges.

## Architecture

The codebase follows a standard Rust library structure:

- `src/lib.rs` - Main library entry point with basic utilities and tests
- `src/aoc/` - Module containing Advent of Code solutions
  - `mod.rs` - Module declarations for AoC solutions
  - `yr2015_d01.rs` - Individual day solutions (currently empty template)

## Common Commands

**Build the project:**
```bash
cargo build
```

**Run tests:**
```bash
cargo test
```

**Run specific test:**
```bash
cargo test <test_name>
```

**Check code (lint equivalent):**
```bash
cargo check
```

**Format code:**
```bash
cargo fmt
```

**Run clippy (additional linting):**
```bash
cargo clippy
```

**Build and run:**
```bash
cargo run
```

## Development Workflow

When adding new Advent of Code solutions:
1. Create new files in `src/aoc/` following the pattern `yr{YEAR}_d{DAY}.rs`
2. Add module declarations to `src/aoc/mod.rs`
3. Structure solutions with clear function names and include unit tests
4. Use `cargo test` to verify solutions work correctly

The project appears to be in early development with template files in place for organizing AoC solutions by year and day.