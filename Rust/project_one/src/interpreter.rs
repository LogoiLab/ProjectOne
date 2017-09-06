use manipulator;

use std::io;
use std::io::prelude::*;

pub struct Response {
    pub out: String,
    pub cont: bool
}

impl Response {
    pub fn value(&self) -> &Response {
        &self
    }
}

pub fn call() -> Response {
    let stdin = io::stdin();
    let mut cont: bool = true;
    let buffer = stdin.lock().lines().next().unwrap().unwrap();
    let mut out: String = String::new();

    match &buffer.as_str() {
        &"exit" => cont = false,
        &"price" => out = (manipulator::get_price(String::from("my part"))).to_string(),
        _ => panic!("invalid input"),
    };
    //temp
    Response{out: out, cont: cont}
}
