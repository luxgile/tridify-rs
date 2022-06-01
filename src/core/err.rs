use core::fmt;
use std::fmt::Formatter;

#[derive(Default)]
pub struct LErr {
    msg: String,
}

impl LErr {
    pub fn new(msg: String) -> Self { Self { msg } }
}

impl fmt::Debug for LErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "{}", self.msg) }
}

impl From<std::io::Error> for LErr {
    fn from(e: std::io::Error) -> Self {
        Self {
            msg: format!("{}", e),
        }
    }
}
