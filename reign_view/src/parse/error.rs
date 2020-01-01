use std::fmt;

pub struct Error {
    pub cursor: usize,
    pub message: String,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&self.message).unwrap();
        Ok(())
    }
}
