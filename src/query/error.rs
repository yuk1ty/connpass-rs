#[derive(Debug)]
pub enum ValidationError {
    OutOfRange { msg: String },
    InvalidToken { msg: String },
}
