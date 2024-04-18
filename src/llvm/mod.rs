use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar, "/llvm/grammar.rs");

#[cfg(test)]
pub mod test;

/// because lalrpop is broken
type DualValue = [std::sync::Arc<crate::ir::Value>; 2];

pub fn parse_escape_sequences(s: &str) -> String {
    // TODO
    s.to_string()
}

#[derive(Debug)]
pub enum Operation {
    Assignment { identifier: String, value: crate::ir::Instruction },
    NoAssignment { instruction: crate::ir::Instruction },
}

#[derive(Debug)]
pub struct BasicBlock {
    pub name: Option<String>,
    pub operations: Vec<Operation>,
    pub terminator: crate::ir::Terminator,
}

#[derive(Debug, Default)]
/// https://llvm.org/docs/LangRef.html#linkage
pub enum LinkageType {
    /// private
    Private,
    /// internal
    Internal,
    /// available_externally
    AvailableExternally,
    /// linkonce
    LinkOnce,
    /// weak
    Weak,
    /// common
    Common,
    /// appending
    Appending,
    /// extern_weak
    ExternalWeak,
    /// linkonce_odr
    LinkOnceODR,
    /// weak_odr
    WeakODR,
    /// external
    #[default]
    External,
}

#[derive(Debug, Default)]
/// https://llvm.org/docs/LangRef.html#runtime-preemption-model
pub enum PreemptionSpecifier {
    /// dso_preemptable
    #[default]
    Preemptable,
    /// dso_local
    Local,
}

#[derive(Debug, Default)]
pub enum Visibility {
    /// default
    #[default]
    Default,
    /// hidden
    Hidden,
    /// protected
    Protected,
}

#[derive(Debug)]
pub struct FunctionParameter {
    pub parameter_type: crate::types::Type,
    pub name: String,
}

#[derive(Debug)]
pub struct Function {
    pub linkage: LinkageType,
    pub preemption_specifier: PreemptionSpecifier,
    pub visibility: Visibility,
    // TODO: calling convention, DLL storage class, unnamed_addr
    pub return_type: crate::types::Type,
    pub return_type_parameter_attributes: Vec<crate::types::ParameterAttribute>,
    pub name: String,
    pub arguments: Vec<FunctionParameter>,
    pub address_space: Option<crate::types::AddressSpace>,
    // TODO: function attributes
    pub section_name: Option<String>,
    pub partition_name: Option<String>,
    // TODO: comdat
    /// align
    pub alignment: Option<usize>,
    /// gc
    pub is_garbage_collected: bool,
    // TODO: prefix, prologue, personality, metadata
    pub basic_blocks: Vec<BasicBlock>,
}
