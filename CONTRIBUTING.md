# Contributing to ptxgen-rs

Thank you for your interest in contributing to **ptxgen-rs**!

## Commit Guidelines

We follow the **Conventional Commits** standard to ensure clarity and consistency.

### Format

```
<type>(<scope>): <short message>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Code style changes (formatting, missing semicolons, etc.)
- **refactor**: Code changes that neither fix a bug nor add a feature
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Maintenance tasks (build, tools, etc.)
- **build**: Changes to build system or dependencies

### Scopes

Use the crate or module name as scope. For example:

- `cli`
- `llvm_parser`
- `ir_model`
- `ptx_backend`
- `ptx_runtime`
- `meta` (for workspace-level or general repository structure)

### Examples

```
feat(llvm_parser): implement lowering of FMul/FAdd to IR
fix(cli): handle missing input file error gracefully
docs(ir_model): add JSON examples to instruction types
refactor(llvm_parser): simplify pattern matching on instructions
perf(ptx_backend): optimize PTX string emission loop
chore(meta): add CONTRIBUTING.md and commit guidelines
```

## Structure

This is a Rust monorepo with the following crates:

- `cli`: Command-line interface
- `llvm_parser`: Parses LLVM IR into a generic intermediate representation
- `ir_model`: Internal IR used for conversion to PTX
- `ptx_backend`: Emits PTX code from IR
- `ptx_runtime`: (optional) Manages runtime execution or bindings

## Testing

To run all tests:

```bash
cargo test --workspace
```

## Code of Conduct

Be respectful, constructive, and inclusive. We aim to foster an open and welcoming environment.

---

Maintained by [Raul Estrada](https://treutech.io).
