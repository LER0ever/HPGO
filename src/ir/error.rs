use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeriveError {
    #[error("Option variable returns unexpected None...")]
    OptionNone(String),
    #[error("Meta key not found...")]
    MetaKeyNotFound(String),
    #[error("Meta value not found...")]
    MetaValueNotFound(String),
    #[error("derivation not yet implemented")]
    DerivationUnimplemented(String),
    #[error("HLO AST not present in struct, cannot perform cache")]
    ASTNotPresent(),
    #[error("Instruction cache miss even after a full AOT generation")]
    InstNotInCache(String),
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum PropagationError {
    #[error("Option variable returns unexpected None...")]
    OptionNone(String),
    #[error("Visiting an already visited node, with incompatible params")]
    AlreadyVisitedIncompatible(String),
    #[error("unknown propagation error")]
    Unknown,
}
