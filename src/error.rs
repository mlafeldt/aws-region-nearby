/// The errors returned by the library.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    /// An invalid AWS region name was provided.
    #[error("invalid AWS region")]
    InvalidRegion,

    /// An invalid Deno region name was provided.
    #[error("invalid Deno region")]
    InvalidDenoRegion,
}
