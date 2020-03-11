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
    #[error("unknown data store error")]
    Unknown,
}
