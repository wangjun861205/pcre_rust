mod client;
mod error;
mod matcher;

extern crate thiserror;

use client::UDPClient;
use error::Error;
use matcher::*;

pub trait Sender {
    fn send(&self, data: &[String]) -> Result<usize, Error>;
}

pub trait Matcher {
    fn pcre_match(&self, pattern: &str, subject: &str) -> Result<Vec<String>, String>;
}

fn main() {
    let subject = "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    let pattern = r"\d{4}([^\d^\s]{3,11})(?!\s)";
    let matcher = PCRE::new();
    let client = UDPClient::new("127.0.0.1:8888");
    if let Ok(group) = matcher.pcre_match(pattern, subject) {
        if group.len() > 1 {
            client.send(&group[1..]).unwrap();
        }
    }
}
