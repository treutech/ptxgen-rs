# ptxgen-rs

**ptxgen-rs** is a pure Rust LLVM IR → PTX code generator.

## Goals

- Fully written in Rust (no C++ or LLVM backend)
- Embeddable, modular and lightweight
- Targeted at scientific computing, DSLs, and GPU runtime generation
- PTX output compatible with NVIDIA's driver runtime
- Designed to replace NVCC/NVPTX for specific use cases

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

## Author

Raul Estrada ([@uurl](https://github.com/uurl))  
Founder, Treu Technologies — [https://treutech.io](https://treutech.io)

## License

Apache License 2.0 © 2025 Treu Technologies
