use std::error::Error;

#[derive(Debug)]
pub enum RusteriaError {
    Unknown(String),
    NoTodoFile(String),
    InvalidUUID,
    NoSuchTodo,
}

impl std::fmt::Display for RusteriaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(err) => f.write_fmt(format_args!("Unknown: {}", err)),
            Self::NoTodoFile(filename) => f.write_fmt(format_args!(
                "No TODO file found. File named \'{}\' is expected",
                filename
            )),
            Self::InvalidUUID => f.write_str("Not a valid UUID"),
            Self::NoSuchTodo => f.write_str("No such Todo exists"),
        }
    }
}

impl RusteriaError {
    pub fn is_file_not_found_error(&self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(&Self::NoTodoFile("".to_owned()))
    }
}
impl Error for RusteriaError {}
