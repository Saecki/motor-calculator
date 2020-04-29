pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    Overconstrained,
    Underconstrained,
}

/// A structure able to represent errors resulting from solving equations.
#[derive(Copy, Clone, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub description: &'static str,
}

impl Error {
    pub fn new(kind: ErrorKind, description: &'static str) -> Self {
        Self { kind, description }
    }
}
