// Error types for the Threads API - populated in Task 3

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("placeholder")]
    Placeholder,
}
