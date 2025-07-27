use insta::assert_debug_snapshot;
use llvm_parser::parse_module::parse_module;

#[test]
fn test_saxpy_ir() {
    let ll_path = "../../examples/saxpy.ll";
    let module = parse_module(ll_path).unwrap();
    let func = &module.functions[0];
    assert_debug_snapshot!(func, @r#"
    Function {
        name: "saxpy",
        parameters: [
            Parameter {
                name: Name(
                    "a",
                ),
                ty: TypeRef(
                    FPType(
                        Single,
                    ),
                ),
                attributes: [],
            },
            Parameter {
                name: Name(
                    "x",
                ),
                ty: TypeRef(
                    PointerType {
                        pointee_type: TypeRef(
                            FPType(
                                Single,
                            ),
                        ),
                        addr_space: 0,
                    },
                ),
                attributes: [],
            },
            Parameter {
                name: Name(
                    "y",
                ),
                ty: TypeRef(
                    PointerType {
                        pointee_type: TypeRef(
                            FPType(
                                Single,
                            ),
                        ),
                        addr_space: 0,
                    },
                ),
                attributes: [],
            },
            Parameter {
                name: Name(
                    "out",
                ),
                ty: TypeRef(
                    PointerType {
                        pointee_type: TypeRef(
                            FPType(
                                Single,
                            ),
                        ),
                        addr_space: 0,
                    },
                ),
                attributes: [],
            },
            Parameter {
                name: Name(
                    "n",
                ),
                ty: TypeRef(
                    IntegerType {
                        bits: 32,
                    },
                ),
                attributes: [],
            },
        ],
        is_var_arg: false,
        return_type: TypeRef(
            VoidType,
        ),
        basic_blocks: [
            BasicBlock {
                name: Name(
                    "entry",
                ),
                instrs: [
                    Alloca(
                        Alloca {
                            allocated_type: TypeRef(
                                IntegerType {
                                    bits: 32,
                                },
                            ),
                            num_elements: ConstantOperand(
                                ConstantRef(
                                    Int {
                                        bits: 32,
                                        value: 1,
                                    },
                                ),
                            ),
                            dest: Name(
                                "i",
                            ),
                            alignment: 4,
                            debugloc: None,
                        },
                    ),
                    Store(
                        Store {
                            address: LocalOperand {
                                name: Name(
                                    "i",
                                ),
                                ty: TypeRef(
                                    PointerType {
                                        pointee_type: TypeRef(
                                            IntegerType {
                                                bits: 32,
                                            },
                                        ),
                                        addr_space: 0,
                                    },
                                ),
                            },
                            value: ConstantOperand(
                                ConstantRef(
                                    Int {
                                        bits: 32,
                                        value: 0,
                                    },
                                ),
                            ),
                            volatile: false,
                            atomicity: None,
                            alignment: 4,
                            debugloc: None,
                        },
                    ),
                ],
                term: Br(
                    Br {
                        dest: Name(
                            "loop",
                        ),
                        debugloc: None,
                    },
                ),
            },
            BasicBlock {
                name: Name(
                    "loop",
                ),
                instrs: [
                    Load(
                        Load {
                            address: LocalOperand {
                                name: Name(
                                    "i",
                                ),
                                ty: TypeRef(
                                    PointerType {
                                        pointee_type: TypeRef(
                                            IntegerType {
                                                bits: 32,
                                            },
                                        ),
                                        addr_space: 0,
                                    },
                                ),
                            },
                            dest: Name(
                                "idx",
                            ),
                            volatile: false,
                            atomicity: None,
                            alignment: 4,
                            debugloc: None,
                        },
                    ),
                    ICmp(
                        ICmp {
                            predicate: SLT,
                            operand0: LocalOperand {
                                name: Name(
                                    "idx",
                                ),
                                ty: TypeRef(
                                    IntegerType {
                                        bits: 32,
                                    },
                                ),
                            },
                            operand1: LocalOperand {
                                name: Name(
                                    "n",
                                ),
                                ty: TypeRef(
                                    IntegerType {
                                        bits: 32,
                                    },
                                ),
                            },
                            dest: Name(
                                "cmp",
                            ),
                            debugloc: None,
                        },
                    ),
                ],
                term: CondBr(
                    CondBr {
                        condition: LocalOperand {
                            name: Name(
                                "cmp",
                            ),
                            ty: TypeRef(
                                IntegerType {
                                    bits: 1,
                                },
                            ),
                        },
                        true_dest: Name(
                            "body",
                        ),
                        false_dest: Name(
                            "exit",
                        ),
                        debugloc: None,
                    },
                ),
            },
            BasicBlock {
                name: Name(
                    "body",
                ),
                instrs: [
                    GetElementPtr(
                        GetElementPtr {
                            address: LocalOperand {
                                name: Name(
                                    "x",
                                ),
                                ty: TypeRef(
                                    PointerType {
                                        pointee_type: TypeRef(
                                            FPType(
                                                Single,
                                            ),
                                        ),
                                        addr_space: 0,
                                    },
                                ),
                            },
                            indices: [
                                LocalOperand {
                                    name: Name(
                                        "idx",
                                    ),
                                    ty: TypeRef(
                                        IntegerType {
                                            bits: 32,
                                        },
                                    ),
                                },
                            ],
                            dest: Name(
                                "x_ptr",
                            ),
                            in_bounds: false,
                            debugloc: None,
                            source_element_type: TypeRef(
                                FPType(
                                    Single,
                                ),
                            ),
                        },
                    ),
                    GetElementPtr(
                        GetElementPtr {
                            address: LocalOperand {
                                name: Name(
                                    "y",
                                ),
                                ty: TypeRef(
                                    PointerType {
                                        pointee_type: TypeRef(
                                            FPType(
                                                Single,
                                            ),
                                        ),
                                        addr_space: 0,
                                    },
                                ),
                            },
                            indices: [
                                LocalOperand {
                                    name: Name(
                                        "idx",
                                    ),
                                    ty: TypeRef(
                                        IntegerType {
                                            bits: 32,
                                        },
                                    ),
                                },
                            ],
                            dest: Name(
                                "y_ptr",
                            ),
                            in_bounds: false,
                            debugloc: None,
                            source_element_type: TypeRef(
                                FPType(
                                    Single,
                                ),
                            ),
                        },
                    ),
                    GetElementPtr(
                        GetElementPtr {
                            address: LocalOperand {
                                name: Name(
                                    "out",
                                ),
                                ty: TypeRef(
                                    PointerType {
                                        pointee_type: TypeRef(
                                            FPType(
                                                Single,
                                            ),
                                        ),
                                        addr_space: 0,
                                    },
                                ),
                            },
                            indices: [
                                LocalOperand {
                                    name: Name(
                                        "idx",
                                    ),
                                    ty: TypeRef(
                                        IntegerType {
                                            bits: 32,
                                        },
                                    ),
                                },
                            ],
                            dest: Name(
                                "out_ptr",
                            ),
                            in_bounds: false,
                            debugloc: None,
                            source_element_type: TypeRef(
                                FPType(
                                    Single,
                                ),
                            ),
                        },
                    ),
                    Load(
                        Load {
                            address: LocalOperand {
                                name: Name(
                                    "x_ptr",
                                ),
                                ty: TypeRef(
                                    PointerType {
                                        pointee_type: TypeRef(
                                            FPType(
                                                Single,
                                            ),
                                        ),
                                        addr_space: 0,
                                    },
                                ),
                            },
                            dest: Name(
                                "xval",
                            ),
                            volatile: false,
                            atomicity: None,
                            alignment: 4,
                            debugloc: None,
                        },
                    ),
                    Load(
                        Load {
                            address: LocalOperand {
                                name: Name(
                                    "y_ptr",
                                ),
                                ty: TypeRef(
                                    PointerType {
                                        pointee_type: TypeRef(
                                            FPType(
                                                Single,
                                            ),
                                        ),
                                        addr_space: 0,
                                    },
                                ),
                            },
                            dest: Name(
                                "yval",
                            ),
                            volatile: false,
                            atomicity: None,
                            alignment: 4,
                            debugloc: None,
                        },
                    ),
                    FMul(
                        FMul {
                            operand0: LocalOperand {
                                name: Name(
                                    "a",
                                ),
                                ty: TypeRef(
                                    FPType(
                                        Single,
                                    ),
                                ),
                            },
                            operand1: LocalOperand {
                                name: Name(
                                    "xval",
                                ),
                                ty: TypeRef(
                                    FPType(
                                        Single,
                                    ),
                                ),
                            },
                            dest: Name(
                                "ax",
                            ),
                            debugloc: None,
                        },
                    ),
                    FAdd(
                        FAdd {
                            operand0: LocalOperand {
                                name: Name(
                                    "ax",
                                ),
                                ty: TypeRef(
                                    FPType(
                                        Single,
                                    ),
                                ),
                            },
                            operand1: LocalOperand {
                                name: Name(
                                    "yval",
                                ),
                                ty: TypeRef(
                                    FPType(
                                        Single,
                                    ),
                                ),
                            },
                            dest: Name(
                                "res",
                            ),
                            debugloc: None,
                        },
                    ),
                    Store(
                        Store {
                            address: LocalOperand {
                                name: Name(
                                    "out_ptr",
                                ),
                                ty: TypeRef(
                                    PointerType {
                                        pointee_type: TypeRef(
                                            FPType(
                                                Single,
                                            ),
                                        ),
                                        addr_space: 0,
                                    },
                                ),
                            },
                            value: LocalOperand {
                                name: Name(
                                    "res",
                                ),
                                ty: TypeRef(
                                    FPType(
                                        Single,
                                    ),
                                ),
                            },
                            volatile: false,
                            atomicity: None,
                            alignment: 4,
                            debugloc: None,
                        },
                    ),
                    Add(
                        Add {
                            operand0: LocalOperand {
                                name: Name(
                                    "idx",
                                ),
                                ty: TypeRef(
                                    IntegerType {
                                        bits: 32,
                                    },
                                ),
                            },
                            operand1: ConstantOperand(
                                ConstantRef(
                                    Int {
                                        bits: 32,
                                        value: 1,
                                    },
                                ),
                            ),
                            dest: Name(
                                "next",
                            ),
                            debugloc: None,
                        },
                    ),
                    Store(
                        Store {
                            address: LocalOperand {
                                name: Name(
                                    "i",
                                ),
                                ty: TypeRef(
                                    PointerType {
                                        pointee_type: TypeRef(
                                            IntegerType {
                                                bits: 32,
                                            },
                                        ),
                                        addr_space: 0,
                                    },
                                ),
                            },
                            value: LocalOperand {
                                name: Name(
                                    "next",
                                ),
                                ty: TypeRef(
                                    IntegerType {
                                        bits: 32,
                                    },
                                ),
                            },
                            volatile: false,
                            atomicity: None,
                            alignment: 4,
                            debugloc: None,
                        },
                    ),
                ],
                term: Br(
                    Br {
                        dest: Name(
                            "loop",
                        ),
                        debugloc: None,
                    },
                ),
            },
            BasicBlock {
                name: Name(
                    "exit",
                ),
                instrs: [],
                term: Ret(
                    Ret {
                        return_operand: None,
                        debugloc: None,
                    },
                ),
            },
        ],
        function_attributes: [],
        return_attributes: [],
        linkage: External,
        visibility: Default,
        dll_storage_class: Default,
        calling_convention: C,
        section: None,
        comdat: None,
        alignment: 0,
        garbage_collector_name: None,
        personality_function: None,
        debugloc: None,
    }
    "#);
}
