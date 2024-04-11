/// a specific kind of floating point type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FloatingPointKind {
    /// a 16 bit floating point value, corresponding to the LLVM `half` type and the IEE-754-2008 `binary16` type
    Binary16,
    /// a 16 bit float value with the same dynamic range as `Binary32`, but with much less precision. corresponds to the LLVM `bfloat` type
    Brain,
    /// a 32 bit floating point value, corresponding to the LLVM `float` type and the IEE-754-2008 `binary32` type
    Binary32,
    /// a 64 bit floating point value, corresponding to the LLVM `double` type and the IEE-754-2008 `binary64` type
    Binary64,
    /// a 128 bit floating point value, corresponding to the LLVM `fp128` type and the IEE-754-2008 `binary128` type
    Binary128,
    /// an 80 bit wide floating point value, as used on the x87 FPU
    X86Fp80,
    /// a 128 bit wide floating point value, as used on PPC
    PpcFp128,
}

/// an address space that a pointer can point to
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddressSpace {
    Numbered(usize),
    Named(String),
}

/// a parameter that can be passed to the target extension type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetExtensionParameter {
    Type(Type),
    Integer(usize),
}

/// a type (wow!). this directly maps to LLVM's types for now. for more information see https://llvm.org/docs/LangRef.html#type-system
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    /// the classic void type. doesn't represent any value.
    /// this type is neither first-class nor sized
    Void,
    /// a function type, representing the return type and arguments of a function
    /// this type is neither first-class nor sized
    Function {
        /// the return type of this function. must be a void type or first-class type, but not a label or metadata type
        return_type: Box<Type>,
        /// the parameters of this function
        parameters: Vec<Type>,
        /// whether the function has C varargs
        has_varargs: bool,
    },
    /// an integer type with an arbitrary bit width.
    /// this type is first-class and sized
    Integer {
        /// the width in bits of this integer type. must be above 1
        bit_width: usize,
    },
    /// a floating point type.
    /// this type is first-class and sized
    FloatingPoint {
        /// the kind of floating point type this is
        kind: FloatingPointKind,
    },
    /// represents a value stored in an x86 AMX register.
    /// this type is first-class but not sized
    AMX,
    /// represents a value stored in an x86 MMX register.
    /// this type is first-class but not sized
    MMX,
    /// a pointer type, representing an address of another value in memory.
    /// this type is first-class and sized
    Pointer {
        /// the address space of this pointer
        address_space: AddressSpace,
    },
    /// a type specific to the target architecture that the compiler is not aware of. for more information see https://llvm.org/docs/LangRef.html#target-extension-type.
    /// this type is presumably sized? and is first-class
    TargetExtension {
        /// the name of this target extension type
        name: String,
        /// the parameters of this target extension type
        parameters: Vec<TargetExtensionParameter>,
    },
    /// a vector type, like a packed array of primitive values.
    /// this type is first-class and sized
    Vector {
        /// the length of this vector. must be greater than 0
        length: usize,
        /// the type of the elements in this vector. must be a first-class type
        element_type: Box<Type>,
        /// whether this vector is scalable. if this is true, then the total number of elements in this vector will be a constant multiple of its length value
        is_scalable: bool,
    },
    /// a label type, representing labels in the program.
    /// this type is neither first-class nor sized
    Label,
    /// honestly idk what this does. go read the LLVM docs at https://llvm.org/docs/LangRef.html#token-type.
    /// this type is neither first-class nor sized
    Token,
    /// represents embedded metadata.
    /// this type is neither first-class nor sized
    Metadata,
    /// an array type, representing a constant length sequential list of elements of the same type in memory.
    /// this type is sized, but is not first-class
    Array {
        /// the length of this array. must be greater than 0
        length: usize,
        /// the type of all the elements in this array
        element_type: Box<Type>,
    },
    /// a structure type, representing a collection of values in memory that can have any combination of sized types.
    /// this type is sized, but is not first-class
    Structure {
        /// an ordered list of the types of values in this structure
        types: Vec<Type>,
        /// whether this structure type should be packed when stored in memory
        is_packed: bool,
    },
    /// an opaque structure type, which doesn't have its contents defined yet.
    /// this type is neither first-class nor sized
    OpaqueStructure,
}

impl Type {
    /// whether this type is first-class (able to be returned from instructions and stored in registers)
    pub fn is_first_class(&self) -> bool {
        matches!(
            self,
            Self::Integer { .. } | Self::FloatingPoint { .. } | Self::AMX | Self::MMX | Self::Pointer { .. } | Self::TargetExtension { .. } | Self::Vector { .. }
        )
    }

    /// whether this type has a size
    pub fn is_sized(&self) -> bool {
        matches!(
            self,
            Self::Integer { .. } | Self::FloatingPoint { .. } | Self::Pointer { .. } | Self::Vector { .. } | Self::Array { .. } | Self::Structure { .. }
        )
    }
}
