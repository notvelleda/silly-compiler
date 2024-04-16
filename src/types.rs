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

/// LLVM parameter attributes (https://llvm.org/docs/LangRef.html#paramattrs)
#[derive(Debug, Clone)]
pub enum ParameterAttribute {
    /// indicates that the parameter or return value should be zero-extended as required.
    /// corresponds to LLVM's `zeroext` parameter attribute
    ZeroExtend,
    /// indicates that the parameter or return value should be sign-extended as required.
    /// corresponds to LLVM's `signext` parameter attribute
    SignExtend,
    /// this depends on the target, but is apparently usually used to specify that a parameter or return value should be placed in a register instead of memory.
    /// corresponds to LLVM's `inreg` parameter attribute
    TargetDependent,
    /// indicates that a parameter should be copied in order to be effectively passed by value, and that changes made to this copy will not affect the original.
    /// corresponds to LLVM's `byval` parameter attribute
    PassByValue(Type),
    /// indicates that a parameter should be specifically passed by reference, instead of letting the code generator do whatever it wants.
    /// corresponds to LLVM's `byref` parameter attribute
    PassByReference(Type),
    /// indicates that a parameter has been copied in order to be effectively passed by value.
    /// corresponds to LLVM's `preallocated` parameter attribute
    PreAllocated(Type),
    /// indicates that a parameter was the last thing allocated on the stack, and can be deallocated by the callee.
    /// corresponds to LLVM's `inalloca` parameter attribute
    StackAllocated(Type),
    /// specifies that this parameter is a pointer to a structure that is the return value of this function.
    /// corresponds to LLVM's `sret` parameter attribute
    ReturnStructure(Type),
    // TODO: elementtype
    /// specifies the alignment of the parameter if it's a pointer or vector of pointers.
    /// corresponds to LLVM's `align` parameter attribute
    Alignment(usize),
    /// corresponds to LLVM's `noalias` parameter attribute
    NoAlias,
    /// corresponds to LLVM's `nocapture` parameter attribute
    NoCapture,
    /// indicates that the parameter will not be freed by the callee.
    /// corresponds to LLVM's `nofree` parameter attribute
    NoFree,
    /// corresponds to LLVM's `nest` parameter attribute
    Nest,
    /// indicates that this parameter will always be returned by the function.
    /// corresponds to LLVM's `returned` parameter attribute
    Returned,
    /// indicates that this parameter is not a null pointer, but is not checked or enforced.
    /// corresponds to LLVM's `nonnull` parameter attribute
    NonNull,
    /// indicates that this parameter can be dereferenced, and that the given number of bytes can be safely dereferenced without risk of exceptions.
    /// corresponds to LLVM's `dereferenceable` parameter attribute
    Dereferenceable(usize),
    /// like `Dereferenceable`, except this pointer can also be null.
    /// corresponds to LLVM's `dereferenceable_or_null` parameter attribute
    DereferenceableOrNull(usize),
    /// indicates that this parameter is the object or context that this function is associated with/
    /// corresponds to LLVM's `swiftself` parameter attribute
    Context,
    /// corresponds to LLVM's `swiftasync` parameter attribute
    SwiftAsync,
    /// corresponds to LLVM's `swifterror` parameter attribute
    SwiftError,
    /// specifies that this parameter must be an immediate value.
    /// corresponds to LLVM's `immarg` parameter attribute
    Immediate,
    /// specifies that this parameter shouldn't contain undefined or poison bits.
    /// corresponds to LLVM's `noundef` parameter attribute
    NoUndefined,
    // TODO: nofpclass
    /// specifies the preferred alignment for if this parameter is allocated stack space.
    /// corresponds to LLVM's `alignstack` parameter attribute
    StackAlignment(usize),
    /// specifies that this parameter refers to the minimum alignment of the pointer returned from a memory allocator function,
    /// where this function can return either a pointer aligned to at least the value of this argument or a null pointer.
    /// corresponds to LLVM's `allocalign` parameter attribute
    AllocationAlignment,
    // TODO: allocptr (what does it mean? does the pointer have to be invalidated, or does it just mean it'll be fucked with? how would this help with optimization at all?)
    /// specifies that this parameter will not be directly dereferenced (though the memory it points to could still be modified).
    /// corresponds to LLVM's `readnone` parameter attribute
    NoDereference,
    /// specifies that this parameter will not be directly dereferenced for write operations, though the memory it points to could still be modified by other pointers.
    /// corresponds to LLVM's `readonly` parameter attribute
    ReadOnly,
    // TODO: writeonly, writable
    /// specifies that this parameter will be poisoned (or the value pointed to by this parameter) if the function call unwinds.
    /// corresponds to LLVM's `dead_on_unwind` parameter attribute
    PoisonOnUnwind,
    /// specifies the possible range of this integer or integer vector parameter.
    /// corresponds to LLVM's `range` parameter attribute
    Range {
        range_type: Type,
        low_inclusive: Vec<usize>,
        high_exclusive: Vec<usize>,
    },
}
