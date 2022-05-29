#[derive(Default, Debug)]
pub struct LErr {
    msg: String,
}

impl LErr {
    pub fn new(msg: String) -> Self { Self { msg } }
}

impl From<std::io::Error> for LErr {
    fn from(e: std::io::Error) -> Self {
        Self {
            msg: format!("{}", e),
        }
    }
}
