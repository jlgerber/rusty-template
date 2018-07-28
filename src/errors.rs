
#[derive(Debug, Fail)]
pub enum RustyTemplateError {
    #[fail(display = "runtime error: {}", _0)]
    RuntimeError(String),
    #[fail(display = "{}", _0)]
    PestError(String),
}
