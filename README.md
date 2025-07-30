# ptxgen-rs

**ptxgen-rs** is a pure Rust LLVM IR → PTX code generator.

## Goals

- Fully written in Rust (no C++ or LLVM backend)
- Embeddable, modular and lightweight
- Targeted at scientific computing, DSLs, and GPU runtime generation
- PTX output compatible with NVIDIA's driver runtime
- Designed to replace NVCC/NVPTX for specific use cases


## Motivation

### The Problem with the Traditional LLVM → PTX Toolchain

Generating PTX code from LLVM IR typically requires the full LLVM C++ toolchain, including:

- Compiling or installing `llc` with the `NVPTX` backend
- Managing large binaries and complex dependencies
- Navigating cryptic CLI flags and non-modular design
- Sacrificing portability and embeddability

This results in:

|                       | LLVM + NVPTX (`llc`) |
|-----------------------|----------------------|
| Disk space            | > 10 GB              |
| External dependencies | Heavy (C++ toolchain) |
| Portability           | Low (platform-specific) |
| Compile time          | Minutes to hours     |
| Embeddable            | No                  |
| Customizable backend  | Black-box           |

---

### What `ptxgen-rs` Solves

`ptxgen-rs` is a lightweight, fully Rust-based LLVM IR to PTX code generator designed to eliminate that complexity:

|                       | `ptxgen-rs`           |
|-----------------------|-----------------------|
| Disk space            | < 1 MB                |
| External dependencies | None (Rust-only)      |
| Portability           | High (cross-platform) |
| Compile time          | Seconds               |
| Embeddable            | Yes                  |
| Customizable backend  | Full control         |

It enables:

- Rapid prototyping for GPU DSLs and code generation
- PTX generation directly from `.ll` files
- Tooling for compiler devs, researchers, and GPU hackers without heavy LLVM baggage

No more fighting LLVM — just pipe `.ll` in, get `.ptx` out.


## Repository Structure

```
ptxgen-rs/
├── crates/
│   ├── llvm_parser/    # Parses LLVM IR
│   ├── ir_model/       # Internal IR representation
│   ├── ptx_backend/    # PTX generator
│   ├── ptx_runtime/    # (Optional) CUDA driver-based runtime
│   └── cli/            # CLI entry point
├── examples/           # Sample .ll files
├── LICENSE             # Apache 2.0
├── NOTICE              # Author and attribution
└── README.md           # This file
```


## Example

### Input: LLVM IR (`add.ll`)
```llvm
define i32 @add(i32 %a, i32 %b) {
entry:
  %sum = add i32 %a, %b
  ret i32 %sum
}
```

### Output: PTX (`add.ptx`)
```
.version 7.0
.target sm_70
.address_size 64

.entry add(
    .param .u32 a,
    .param .u32 b,
    .param .u32 ret
)
{
    .reg .u32 %r1;
    .reg .u32 %r2;
    .reg .u32 %r3;

    ld.param.u32 %r1, [a];
    ld.param.u32 %r2, [b];
    add.u32 %r3, %r1, %r2;
    st.param.u32 [ret], %r3;
    ret;
}
```

## Author

Raul Estrada ([@uurl](https://github.com/uurl))  
Founder, Treu Technologies — [https://treutech.io](https://treutech.io)

## License

Apache License 2.0 © 2025 Treu Technologies
