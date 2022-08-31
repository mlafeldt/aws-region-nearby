/// The errors returned by the library.
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// An invalid AWS region name was provided.
    #[error("invalid AWS region")]
    InvalidRegion,
}
