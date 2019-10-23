#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
    pub fn new_box(msg: &str) -> Box<dyn std::error::Error> {
        return Box::new(Error::new(msg));
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn new_box(msg: &str) -> Box<dyn std::error::Error> {
    Error::new_box(msg)
}
