use crate::thiserror::Error as ThisError;
use std::io;
use std::net::AddrParseError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("client error")]
    ClientError(#[from] io::Error),
}
