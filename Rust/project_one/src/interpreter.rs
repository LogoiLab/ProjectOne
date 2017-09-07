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

    match &buffer.to_lowercase().as_str() {
        &"display" => manipulator::display(),
        &"enter" => manipulator::enter(),
        &"quit" => cont = false,
        &"read" => manipulator::read(),
        &"sell" => manipulator::sell(),
        &"sortname" => manipulator::sort_by_name(),
        &"sortnumber" => manipulator::sort_by_number(),
        _ => println!("Invalid input, try again."),
    };
    //temp
    Response{out: out, cont: cont}
}
