use crate::types::{AddressSpace, ParameterAttribute, Type};
use std::sync::Arc;

#[derive(Debug, Copy, Clone)]
pub struct AllowedWrapping {
    pub can_wrap_unsigned: bool,
    pub can_wrap_signed: bool,
}

impl Default for AllowedWrapping {
    fn default() -> Self {
        Self {
            can_wrap_unsigned: true,
            can_wrap_signed: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    /// add
    Add {
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
        allowed_wrapping: AllowedWrapping,
    },
    // TODO: fadd
    /// sub
    Subtract {
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
        allowed_wrapping: AllowedWrapping,
    },
    // TODO: fsub
    /// mul
    Multiply {
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
        allowed_wrapping: AllowedWrapping,
    },
    // TODO: fmul
    /// udiv
    UnsignedDivide {
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
        is_exact: bool,
    },
    /// sdiv
    SignedDivide {
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
        is_exact: bool,
    },
    // TODO: fdiv
    /// urem
    UnsignedRemainder { left_hand_side: Arc<Value>, right_hand_side: Arc<Value> },
    /// srem
    SignedRemainder { left_hand_side: Arc<Value>, right_hand_side: Arc<Value> },
    // TODO: frem
    /// shl
    ShiftLeft {
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
        allowed_wrapping: AllowedWrapping,
    },
    /// lshr
    LogicalShiftRight {
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
        is_exact: bool,
    },
    /// ashr
    ArithmeticShiftRight {
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
        is_exact: bool,
    },
    /// and
    And { left_hand_side: Arc<Value>, right_hand_side: Arc<Value> },
    /// or
    Or {
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
        disjoint: bool,
    },
    /// xor
    ExclusiveOr { left_hand_side: Arc<Value>, right_hand_side: Arc<Value> },
    // TODO: extractelement, insertelement, shufflevector
    /// extractvalue
    ExtractValue { aggregate: Arc<Value>, indices: Vec<usize> },
    /// insertvalue
    InsertValue { aggregate: Arc<Value>, value: Arc<Value>, indices: Vec<usize> },
    /// alloca
    StackAllocate {
        can_reuse: bool,
        value_type: Type,
        num_elements: Option<Arc<Value>>,
        alignment: Option<usize>,
        address_space: Option<AddressSpace>,
    },
    /// load
    Load {
        is_volatile: bool,
        result_type: Type,
        pointer: Arc<Value>,
        alignment: Option<usize>,
    },
    /// load atomic
    AtomicLoad {
        is_volatile: bool,
        result_type: Type,
        pointer: Arc<Value>,
        ordering: Ordering,
        sync_scope: Option<String>,
        alignment: usize,
    },
    /// store
    Store {
        is_volatile: bool,
        value: Arc<Value>,
        pointer: Arc<Value>,
        alignment: Option<usize>,
    },
    /// store atomic
    AtomicStore {
        is_volatile: bool,
        value: Arc<Value>,
        pointer: Arc<Value>,
        ordering: Ordering,
        sync_scope: Option<String>,
        alignment: usize,
    },
    /// fence
    Fence { ordering: Ordering, sync_scope: Option<String> },
    // todo: cmpxchg, atomicrmw
    /// getelementptr
    GetElementPointer {
        kind: GetPointerKind,
        pointer_type: Type,
        pointer: Arc<Value>,
        indices: Vec<Arc<Value>>,
    },
    /// trunc
    Truncate { allowed_wrapping: AllowedWrapping, value: Arc<Value>, new_type: Type },
    /// zext
    ZeroExtend { value: Arc<Value>, new_type: Type },
    /// sext (🤨)
    SignExtend { value: Arc<Value>, new_type: Type },
    // TODO: fptrunc, fpext, fptoui, uitofp, sitofp
    /// ptrtoint
    PointerToInteger { value: Arc<Value>, new_type: Type },
    /// inttoptr
    IntegerToPointer { value: Arc<Value>, new_type: Type },
    /// bitcast
    BitCast { value: Arc<Value>, new_type: Type },
    /// addrspacecast
    AddressSpaceCast { value: Arc<Value>, new_type: Type },
    /// icmp
    CompareIntegers {
        comparison: IntegerComparison,
        left_hand_side: Arc<Value>,
        right_hand_side: Arc<Value>,
    },
    // TODO: fcmp, phi
    /// select
    Select { condition: Arc<Value>, true_value: Arc<Value>, false_value: Arc<Value> },
    /// freeze
    Freeze { value: Arc<Value> },
    /// call
    Call {
        tail_call_hint: TailCallHint,
        calling_convention: Option<String>,
        return_value_attributes: Vec<ParameterAttribute>,
        address_space: Option<AddressSpace>,
        function_type: Type,
        function_name: String,
        function_arguments: Vec<Arc<Value>>,
        // TODO: function attributes, operand bundles
    },
    // TODO: va_arg, landingpad, catchpad, cleanuppad
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum GetPointerKind {
    #[default]
    Regular,
    InBounds,
    InRange(usize, usize),
}

/// https://llvm.org/docs/LangRef.html#ordering
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ordering {
    Unordered,
    Monotonic,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
}

/// https://llvm.org/docs/LangRef.html#id306
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IntegerComparison {
    Equal,
    NotEqual,
    UnsignedGreaterThan,
    UnsignedGreaterOrEqual,
    UnsignedLessThan,
    UnsignedLessOrEqual,
    SignedGreaterThan,
    SignedGreaterOrEqual,
    SignedLessThan,
    SignedLessOrEqual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum TailCallHint {
    #[default]
    Indifferent,
    ShouldTail,
    MustTail,
    NeverTail,
}

/// TODO
#[derive(Debug, Clone)]
pub enum Value {
    FromInstruction {
        /// the instruction that produced this value
        instruction: Instruction,
    },
    /// TODO
    FromConstant {
        constant_type: Type,
        constant: Constant,
    },
    /// TODO
    FromGlobal,
    /// TODO
    FromFunction,
    /// TODO
    FromLabel,
    FromIdentifier {
        value_type: Type,
        identifier: String,
    },
}

impl Value {
    pub fn from_type_constant(constant_type: Type, constant: Constant) -> Self {
        if !constant.is_compatible_with_type(&constant_type) {
            panic!("constant {constant:?} is incompatible with type {constant_type:?}");
        }

        Self::FromConstant { constant_type, constant }
    }

    pub fn get_type(&self) -> &Type {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum Constant {
    Void,
    Boolean(bool),
    Integer(usize),
    FloatingPoint(usize),
    NullPointer,
    NoneToken,
    Structure(Vec<Arc<Value>>),
    Array(Vec<Arc<Value>>),
    Vector(Vec<Arc<Value>>),
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
                Type::Structure { types, .. } => !values.iter().map(|v| v.get_type()).zip(types).any(|(a, b)| a != b),
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

#[derive(Debug, Clone)]
pub struct SwitchDestination {
    pub value: Arc<Value>,
    pub destination: Arc<Value>,
}

/// https://llvm.org/docs/LangRef.html#terminator-instructions
#[derive(Debug, Clone)]
pub enum Terminator {
    Return {
        value: Arc<Value>,
    },
    ConditionalBranch {
        condition: Arc<Value>,
        if_true: Arc<Value>,
        if_false: Arc<Value>,
    },
    Branch {
        destination: Arc<Value>,
    },
    Switch {
        value: Arc<Value>,
        default_destination: Arc<Value>,
        destinations: Vec<SwitchDestination>,
    },
    IndirectBranch {
        address: Arc<Value>,
        valid_destinations: Vec<Arc<Value>>,
    },
    // TODO: invoke, callbr, resume, catchswitch, catchret, cleanupret
    Unreachable,
}
