use super::grammar::TypeParser;
use crate::types::*;

/// simple test to ensure the examples given in the LLVM documentation are parsed correctly
#[test]
fn type_parsing() {
    assert!(TypeParser::new().parse("i1") == Ok(Type::Integer { bit_width: 1 }));
    assert!(TypeParser::new().parse("i32") == Ok(Type::Integer { bit_width: 32 }));
    assert!(TypeParser::new().parse("i1942652") == Ok(Type::Integer { bit_width: 1942652 }));

    assert!(TypeParser::new().parse("half") == Ok(Type::FloatingPoint { kind: FloatingPointKind::Binary16 }));
    assert!(TypeParser::new().parse("bfloat") == Ok(Type::FloatingPoint { kind: FloatingPointKind::Brain }));
    assert!(TypeParser::new().parse("float") == Ok(Type::FloatingPoint { kind: FloatingPointKind::Binary32 }));
    assert!(TypeParser::new().parse("double") == Ok(Type::FloatingPoint { kind: FloatingPointKind::Binary64 }));
    assert!(TypeParser::new().parse("fp128") == Ok(Type::FloatingPoint { kind: FloatingPointKind::Binary128 }));
    assert!(TypeParser::new().parse("x86_fp80") == Ok(Type::FloatingPoint { kind: FloatingPointKind::X86Fp80 }));
    assert!(TypeParser::new().parse("ppc_fp128") == Ok(Type::FloatingPoint { kind: FloatingPointKind::PpcFp128 }));

    assert!(TypeParser::new().parse("x86_amx") == Ok(Type::AMX));
    assert!(TypeParser::new().parse("x86_mmx") == Ok(Type::MMX));

    assert!(
        TypeParser::new().parse("ptr")
            == Ok(Type::Pointer {
                address_space: AddressSpace::Numbered(0),
            })
    );
    assert!(
        TypeParser::new().parse("ptr addrspace(621)")
            == Ok(Type::Pointer {
                address_space: AddressSpace::Numbered(621),
            })
    );
    assert!(
        TypeParser::new().parse(r#"ptr addrspace("UwU")"#)
            == Ok(Type::Pointer {
                address_space: AddressSpace::Named("UwU".to_string()),
            })
    );

    assert!(
        TypeParser::new().parse(r#"target("label")"#)
            == Ok(Type::TargetExtension {
                name: "label".to_string(),
                parameters: vec![],
            })
    );
    assert!(
        TypeParser::new().parse(r#"target("label", void)"#)
            == Ok(Type::TargetExtension {
                name: "label".to_string(),
                parameters: vec![TargetExtensionParameter::Type(Type::Void)],
            })
    );
    assert!(
        TypeParser::new().parse(r#"target("label", void, i32)"#)
            == Ok(Type::TargetExtension {
                name: "label".to_string(),
                parameters: vec![TargetExtensionParameter::Type(Type::Void), TargetExtensionParameter::Type(Type::Integer { bit_width: 32 })],
            })
    );
    assert!(
        TypeParser::new().parse(r#"target("label", 0, 1, 2)"#)
            == Ok(Type::TargetExtension {
                name: "label".to_string(),
                parameters: vec![TargetExtensionParameter::Integer(0), TargetExtensionParameter::Integer(1), TargetExtensionParameter::Integer(2)],
            })
    );
    assert!(
        TypeParser::new().parse(r#"target("label", void, i32, 0, 1, 2)"#)
            == Ok(Type::TargetExtension {
                name: "label".to_string(),
                parameters: vec![
                    TargetExtensionParameter::Type(Type::Void),
                    TargetExtensionParameter::Type(Type::Integer { bit_width: 32 }),
                    TargetExtensionParameter::Integer(0),
                    TargetExtensionParameter::Integer(1),
                    TargetExtensionParameter::Integer(2),
                ],
            })
    );

    assert!(
        TypeParser::new().parse("<4 x i32>")
            == Ok(Type::Vector {
                length: 4,
                element_type: Box::new(Type::Integer { bit_width: 32 }),
                is_scalable: false,
            })
    );
    assert!(
        TypeParser::new().parse("<8 x float>")
            == Ok(Type::Vector {
                length: 8,
                element_type: Box::new(Type::FloatingPoint { kind: FloatingPointKind::Binary32 }),
                is_scalable: false,
            })
    );
    assert!(
        TypeParser::new().parse("<2 x i64>")
            == Ok(Type::Vector {
                length: 2,
                element_type: Box::new(Type::Integer { bit_width: 64 }),
                is_scalable: false,
            })
    );
    assert!(
        TypeParser::new().parse("<4 x ptr>")
            == Ok(Type::Vector {
                length: 4,
                element_type: Box::new(Type::Pointer {
                    address_space: AddressSpace::Numbered(0),
                }),
                is_scalable: false,
            })
    );
    assert!(
        TypeParser::new().parse("<vscale x 4 x i32>")
            == Ok(Type::Vector {
                length: 4,
                element_type: Box::new(Type::Integer { bit_width: 32 }),
                is_scalable: true,
            })
    );

    //assert!(TypeParser::new().parse("label") == Ok(Type::Label));
    assert!(TypeParser::new().parse("token") == Ok(Type::Token));
    assert!(TypeParser::new().parse("metadata") == Ok(Type::Metadata));

    assert!(
        TypeParser::new().parse("[40 x i32]")
            == Ok(Type::Array {
                length: 40,
                element_type: Box::new(Type::Integer { bit_width: 32 }),
            })
    );
    assert!(
        TypeParser::new().parse("[41 x i32]")
            == Ok(Type::Array {
                length: 41,
                element_type: Box::new(Type::Integer { bit_width: 32 }),
            })
    );
    assert!(
        TypeParser::new().parse("[4 x i8]")
            == Ok(Type::Array {
                length: 4,
                element_type: Box::new(Type::Integer { bit_width: 8 }),
            })
    );
    assert!(
        TypeParser::new().parse("[3 x [4 x i32]]")
            == Ok(Type::Array {
                length: 3,
                element_type: Box::new(Type::Array {
                    length: 4,
                    element_type: Box::new(Type::Integer { bit_width: 32 }),
                }),
            })
    );
    assert!(
        TypeParser::new().parse("[12 x [10 x float]]")
            == Ok(Type::Array {
                length: 12,
                element_type: Box::new(Type::Array {
                    length: 10,
                    element_type: Box::new(Type::FloatingPoint { kind: FloatingPointKind::Binary32 }),
                }),
            })
    );
    assert!(
        TypeParser::new().parse("[2 x [3 x [4 x i16]]]")
            == Ok(Type::Array {
                length: 2,
                element_type: Box::new(Type::Array {
                    length: 3,
                    element_type: Box::new(Type::Array {
                        length: 4,
                        element_type: Box::new(Type::Integer { bit_width: 16 }),
                    }),
                }),
            })
    );

    assert!(
        TypeParser::new().parse("{ i32, i32, i32 }")
            == Ok(Type::Structure {
                types: vec![Type::Integer { bit_width: 32 }, Type::Integer { bit_width: 32 }, Type::Integer { bit_width: 32 }],
                is_packed: false,
            })
    );
    assert!(
        TypeParser::new().parse("{ float, ptr }")
            == Ok(Type::Structure {
                types: vec![Type::FloatingPoint { kind: FloatingPointKind::Binary32 }, Type::Pointer {
                    address_space: AddressSpace::Numbered(0),
                }],
                is_packed: false,
            })
    );
    assert!(
        TypeParser::new().parse("<{ i8, i32 }>")
            == Ok(Type::Structure {
                types: vec![Type::Integer { bit_width: 8 }, Type::Integer { bit_width: 32 }],
                is_packed: true,
            })
    );

    assert!(
        TypeParser::new().parse("i32 (i32)")
            == Ok(Type::Function {
                return_type: Box::new(Type::Integer { bit_width: 32 }),
                parameters: vec![Type::Integer { bit_width: 32 }],
                has_varargs: false,
            })
    );
    assert!(
        TypeParser::new().parse("i32 (ptr, ...)")
            == Ok(Type::Function {
                return_type: Box::new(Type::Integer { bit_width: 32 }),
                parameters: vec![Type::Pointer {
                    address_space: AddressSpace::Numbered(0),
                }],
                has_varargs: true,
            })
    );
    assert!(
        TypeParser::new().parse("{i32, i32} (i32)")
            == Ok(Type::Function {
                return_type: Box::new(Type::Structure {
                    types: vec![Type::Integer { bit_width: 32 }, Type::Integer { bit_width: 32 }],
                    is_packed: false,
                }),
                parameters: vec![Type::Integer { bit_width: 32 }],
                has_varargs: false,
            })
    );
}
