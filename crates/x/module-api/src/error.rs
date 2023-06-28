use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("not handled")]
    NotHandled,
}
