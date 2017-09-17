extern crate chrono;
#[macro_use]
extern crate prettytable;

pub mod part;
pub mod part_list;
pub mod database_handler;
pub mod interpreter;
pub mod importer;
pub mod manipulator;

use prettytable::Table;

pub fn main() {
    let mut part_list: part_list::PartList = database_handler::load_database(String::from("warehouseDB.txt"));
    let mut cont: bool = true;
    let mut table: Table = Table::new();
    table.add_row(row!["dedup","Manually deduplicate the database."]);
    table.add_row(row!["display","Look up a part's price."]);
    table.add_row(row!["enter","Add a part to the database."]);
    table.add_row(row!["quit","Exit the application."]);
    table.add_row(row!["help","Displays this menu."]);
    table.add_row(row!["read","Provide the filename of an inventory list to import."]);
    table.add_row(row!["sell","Sell an item."]);
    table.add_row(row!["sortname","Display the full database sorted by name."]);
    table.add_row(row!["sortnumber","Display the full database sorted by part number."]);
    println!("Welcome to the bike part warehouse. What would you like to do?\n");
    table.printstd();

    while cont {
        let response: interpreter::Response = interpreter::call(part_list);
        cont = response.value().cont;
        if response.value().help {
            table.printstd();
        }
        part_list = response.list;
    }

}
