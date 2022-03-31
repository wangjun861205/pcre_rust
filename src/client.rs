use crate::error::Error;
use crate::Sender;
use std::net::UdpSocket;

#[derive(Debug, Clone)]
pub struct Client {
    address: String,
}

impl Client {
    pub fn new(address: &str) -> Self {
        Self { address: address.to_owned() }
    }
}

impl Sender for Client {
    fn send(&self, data: &[String]) -> Result<i32, Error> {
        let mut sock = UdpSocket::bind(&self.address)?;
    }
}
