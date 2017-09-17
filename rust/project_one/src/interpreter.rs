use manipulator;
use part_list::PartList;
use std::io;
use std::io::prelude::*;

pub struct Response {
    pub cont: bool,
    pub help: bool,
    pub list: PartList
}

impl Response {
    pub fn value(&self) -> &Response {
        &self
    }
}

pub fn call(part_list: PartList) -> Response {
    print!("VVV\n");
    let stdin = io::stdin();
    let mut cont: bool = true;
    let mut help: bool = false;
    let buffer = stdin.lock().lines().next().unwrap().unwrap();

    let part_list: PartList = match &buffer.to_lowercase().trim() {
        &"again" => {println!("Haha very funny."); part_list},
        &"dedup" => manipulator::dedup(part_list),
        &"display" => manipulator::display(part_list),
        &"enter" => manipulator::enter(part_list),
        &"help" => {help = true; part_list},
        &"quit" => {cont = false; manipulator::save(part_list)},
        &"read" => manipulator::read(part_list),
        &"sell" => manipulator::sell(part_list),
        &"sortname" => manipulator::sort_by_name(part_list),
        &"sortnumber" => manipulator::sort_by_number(part_list),
        _ => {println!("Invalid input, try again."); part_list},
    };
    Response{cont: cont, help: help, list: part_list}
}
