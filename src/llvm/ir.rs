use crate::types::Type;

#[derive(Debug)]
pub enum Value {
    Identifier(Type, String),
    Constant(Type, Constant),
}

impl Value {
    pub fn from_type_constant(t: Type, c: Constant) -> Self {
        if !c.is_compatible_with_type(&t) {
            panic!("constant {c:?} is invalid for type {t:?}");
        }

        Self::Constant(t, c)
    }

    pub fn get_type(&self) -> &Type {
        match self {
            Self::Identifier(t, _) => t,
            Self::Constant(t, _) => t,
        }
    }
}

#[derive(Debug)]
pub enum Constant {
    Void,
    Boolean(bool),
    Integer(usize),
    FloatingPoint(usize),
    NullPointer,
    NoneToken,
    Structure(Vec<Value>),
    Array(Vec<Value>),
    Vector(Vec<Value>),
    Zero,
    Metadata,
    Undefined,
    Poison,
}

impl Constant {
    pub fn is_compatible_with_type(&self, t: &Type) -> bool {
        match self {
            Constant::Void => t == &Type::Void,
            Constant::Boolean(_) => t == &Type::Integer { bit_width: 1 },
            Constant::Integer(_) => matches!(t, Type::Integer { .. }),
            Constant::FloatingPoint(_) => matches!(t, Type::FloatingPoint { .. }),
            Constant::NullPointer => matches!(t, Type::Pointer { .. }),
            Constant::NoneToken => t == &Type::Token,
            Constant::Structure(values) => match t {
                Type::Structure { types, .. } => !values.iter().map(Value::get_type).zip(types).any(|(a, b)| a != b),
                _ => false,
            },
            Constant::Array(values) => match t {
                Type::Array { length, element_type } => *length == values.len() && !values.iter().any(|v| v.get_type() != element_type.as_ref()),
                _ => false,
            },
            Constant::Vector(values) => match t {
                Type::Vector { length, element_type, .. } => *length == values.len() && !values.iter().any(|v| v.get_type() != element_type.as_ref()),
                _ => false,
            },
            Constant::Zero => true,
            Constant::Metadata => t == &Type::Metadata,
            Constant::Undefined => !matches!(t, Type::Label | Type::Void),
            Constant::Poison => true,
        }
    }
}

pub struct JumpDestination {
    pub value: Value,
    pub destination: Value,
}

pub enum Terminator {
    Return {
        value: Value,
    },
    ConditionalBranch {
        condition: Value,
        if_true: Value,
        if_false: Value,
    },
    Branch {
        destination: Value,
    },
    Switch {
        value: Value,
        default_destination: Value,
        destinations: Vec<JumpDestination>,
    },
    IndirectBranch {
        address: Value,
        valid_destinations: Vec<Value>,
    },
    Invoke {
        calling_convention: String,
        return_attributes: Vec<String>,
        address_space: crate::types::AddressSpace,
        return_type: Type,
        function_type: Type,
        function_pointer_value: Value,
        function_arguments: Vec<Value>,
        normal_label: Value,
        exception_label: Value,
        function_attributes: Vec<String>,
        operand_bundles: Vec<String>,
    },
    CallBranch {
        calling_convention: String,
        return_attributes: Vec<String>,
        address_space: crate::types::AddressSpace,
        return_type: Type,
        function_type: Type,
        function_pointer_value: Value,
        function_arguments: Vec<Value>,
        fallthrough_label: Value,
        indirect_labels: Vec<Value>,
        function_attributes: Vec<String>,
        operand_bundles: Vec<String>,
    },
    Resume {
        exception: Value,
    },
    CatchSwitch {
        parent_token: Value,
        handler_labels: Vec<Value>,
        unwind_label: Option<Value>,
    },
    CatchReturn {
        from_token: Value,
        to_label: Value,
    },
    CleanUpReturn {
        from_pad: String,
        to_label: Value,
    },
    Unreachable,
}
