#[derive(Debug)]
pub enum Error {
    Unknown,
}

pub type Result<T> = core::result::Result<T, Error>;
