# Contributing to latex2chin

## Development Setup

### Prerequisites

- Rust toolchain (rustc 1.80+, cargo)
- Python 3.10+ with [uv](https://docs.astral.sh/uv/)
- A virtual environment for PyO3 builds

### Install Dependencies

```bash
# Rust dependencies
cargo build

# Python dependencies (core only)
uv sync

# Python dependencies (with LLM support)
uv sync --extra llm
```

### Build Python Extension

```bash
uv run maturin develop
```

### Run Tests

```bash
# Rust core tests (no Python needed)
cargo test -p latex2chin-core

# Full workspace tests
cargo test --workspace

# Python import smoke test
uv run python -c "from latex2chin import parse_latex; print(parse_latex(r'\\frac{1}{2}'))"
```

## Architecture Overview

The project is organized as a Cargo workspace:

```
crates/
  core/          # latex2chin-core (rlib) - pure Rust, no Python dependency
    src/
      ast.rs         # AST type definitions (Expr, BinaryOpKind, etc.)
      latex.pest     # PEG grammar for LaTeX parsing
      latex_parser.rs # pest parser derive
      builder.rs     # pest Pair -> AST conversion
      translator.rs  # AST -> Chinese text
      error.rs       # ParseError type
      lib.rs         # Public API: parse_latex(), parse_to_ast(), translate()
  py/             # latex2chin (cdylib) - PyO3 Python binding
    src/
      lib.rs         # PyO3 module glue
python/
  latex2chin/     # Python package with LangChain agent
    __init__.py     # parse_chinese(), configure(), settings
```

## Adding a New LaTeX Syntax

To add support for a new LaTeX command (e.g., `\vec{a}`):

1. **Grammar** (`crates/core/src/latex.pest`): Add a new rule
2. **AST** (`crates/core/src/ast.rs`): Add a new `Expr` variant if needed, or extend an existing enum
3. **Builder** (`crates/core/src/builder.rs`): Add a handler to convert the pest Pair to the AST node
4. **Translator** (`crates/core/src/translator.rs`): Add a match arm in `translate()` and any helper functions
5. **Tests** (`crates/core/tests/`): Add tests in the appropriate test file:
   - `test_builder.rs` for AST construction tests
   - `test_translator.rs` for translation tests
   - `test_parse_latex.rs` for end-to-end tests
6. **README**: Update the supported syntax table

## Code Style

- Rust: 4-space indentation, follow `cargo fmt`
- Python: Follow PEP 8, 4-space indentation
- Code comments and documentation in English
- User-facing output in Chinese
- Use ASCII hyphen (`-`, U+002D), never Unicode hyphen (`‑`, U+2011)
