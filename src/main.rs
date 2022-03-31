mod client;
mod error;
mod pcre;

#[macro_use]
extern crate thiserror;

use pcre::*;

pub trait Sender {
    fn send(data: &[String]) -> Result<i32, String>;
}

fn main() {
    let subject = "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    let pattern = r"\d{4}([^\d^\s]{3,11})(?!\s)";
    if let Ok(group) = pcre_match(pattern, subject) {
        for g in group {
            println!("{g}");
        }
    }
}
