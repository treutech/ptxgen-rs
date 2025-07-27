use llvm_parser::parse_module::parse_module;
use ptx_backend::lower_to_ptx;

#[test]
fn test_saxpy_ptx_output() {

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("saxpy.ll");
    println!("Reading from path: {}", path.display());

    let module = parse_module(&path).expect("Failed to parse module");

    let mut actual = String::new();

    for func in module.functions {
        actual.push_str(&format!("// Function: {}\n", func.name));
        for block in func.basic_blocks {
            actual.push_str(&format!("// Block: {}\n", block.name));
            let instrs = block
                .instrs
                .iter()
                .map(llvm_parser::convert::lower)
                .collect::<Vec<_>>();
            for line in lower_to_ptx(&instrs) {
                actual.push_str(&format!("{}\n", line));
            }
        }
    }

    insta::assert_snapshot!("saxpy_ptx", actual);
}
