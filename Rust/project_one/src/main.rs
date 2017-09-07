#[macro_use]
extern crate prettytable;

pub mod part;
pub mod part_list;
pub mod database_handler;
pub mod interpreter;
pub mod importer;
pub mod manipulator;

pub fn main() {

    let mut cont: bool = true;
    print!("> ");
    while cont {
        let response: interpreter::Response = interpreter::call();
        cont = response.value().cont;
        println!("{}", response.value().out);
    }

}
