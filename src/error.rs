/// The errors returned by the library.
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// An invalid AWS region name was provided.
    #[deprecated(note = "please use `InvalidAwsRegion` instead")]
    #[error("invalid AWS region")]
    InvalidRegion,

    /// An invalid AWS region name was provided.
    #[error("invalid AWS region")]
    InvalidAwsRegion,

    /// An invalid Deno Deploy region name was provided.
    #[cfg(feature = "deno")]
    #[error("invalid Deno Deploy region")]
    InvalidDenoRegion,
}
