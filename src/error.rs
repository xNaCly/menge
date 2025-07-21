#[derive(Debug)]
pub struct Error {
    message: String,
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            message: value.into(),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}
