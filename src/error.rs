pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    Overconstrained,
    Underconstrained,
    Calculation,
}

#[derive(Copy, Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    description: &'static str,
}

impl Error {
    pub fn new(kind: ErrorKind, description: &'static str) -> Error {
        Error { kind, description }
    }
}
