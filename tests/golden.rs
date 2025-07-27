use ptx_backend::lower_to_ptx;
use llvm_parser::parse_module::parse_module;
use llvm_parser::convert::lower;

#[test]
fn test_saxpy_ptx() {
    let m = parse_module("examples/saxpy.ll").unwrap();
    let mut out = String::new();
    for f in m.functions {
        for b in f.basic_blocks {
            let instrs = b.instrs.iter().map(lower).collect::<Vec<_>>();
            for l in lower_to_ptx(&instrs) {
                out.push_str(&l);
                out.push('\n');
            }
        }
    }
    insta::assert_snapshot!(out);
}
