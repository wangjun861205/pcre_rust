use crate::error::Error;
use crate::Sender;
use std::net::UdpSocket;

#[derive(Debug, Clone)]
pub struct UDPClient {
    address: String,
}

impl UDPClient {
    pub fn new(address: &str) -> Self {
        Self { address: address.to_owned() }
    }
}

impl Sender for UDPClient {
    fn send(&self, data: &[String]) -> Result<usize, Error> {
        let sock = UdpSocket::bind("127.0.0.1:9999")?;
        sock.connect(&self.address)?;
        let mut total = 0;
        for s in data {
            total += sock.send(format!("{s}\n").as_bytes())?;
        }
        Ok(total)
    }
}
