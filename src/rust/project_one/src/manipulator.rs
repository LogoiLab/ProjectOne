use database_handler::save_database;
use importer::{import_line,import_file};
use part::Part;
use part_list::PartList;
use std::io;
use std::io::prelude::*;
use chrono::{Local, DateTime};
use prettytable::Table;

/// Decorator for user input.
/// # Arguments
/// * `decorator` - A String to prepend std::io with.
pub fn prompt(decorator: String) -> String {
    let stdin = io::stdin();
    print!("{}", decorator);
    let buffer = stdin.lock().lines().next().unwrap().unwrap();
    buffer
}

/// Displays a part.
/// # Arguments
/// * `part_list` - The current operable PartList.
pub fn display(part_list: PartList) -> PartList {
    {
        let mut table: Table = Table::new();
        table.add_row(row!["Part Name", "Price"]);
        let part: &Part = &mut part_list.get_by_name(prompt(String::from("Part name:\n")));
        let mut price = part.list_price();
        if part.on_sale().eq(&true) {
            price = part.sale_price();
        }
        table.add_row(row![part.part_name().as_str(), price.to_string().as_str()]);
        table.printstd();
    }
    part_list
}

/// Calls for an `importer::import_line()`.
/// # Arguments
/// * `part_list` - The current operable PartList.
pub fn enter(part_list: PartList) -> PartList {
    import_line(part_list)
}

/// Calls `importer::import_file()` to import a file.
/// # Arguments
/// * `part_list` - The current operable PartList.
pub fn read(part_list: PartList) -> PartList {
    import_file(prompt(String::from("Filename to read:\n")), part_list)
}

/// Calls the `database_handler::save_database()` function.
/// # Arguments
/// * `part_list` - The current operable PartList.
pub fn save(part_list: PartList) -> PartList {
    save_database(String::from("warehouseDB.txt"), part_list)
}

/// Sells a part, reducing its quantity.
/// # Arguments
/// * `part_list` - The current operable PartList.
pub fn sell(mut part_list: PartList) -> PartList {
    let mut sellable: bool = false;
    let part_number: &i64 = &prompt(String::from("Part#:\n")).parse::<i64>().unwrap();
    {
        let mut table: Table = Table::new();
        table.add_row(row!["Part Name", "Part Number", "Price", "Date"]);
        let part: &Part = &mut part_list.get_by_number(part_number);
        let mut price = part.list_price();
        if part.on_sale().eq(&true) {
            price = part.sale_price();
        }
        if part.quantity() > &0 {
            sellable = true;
            let dt: DateTime<Local> = Local::now();
            table.add_row(row![part.part_name().as_str(), part.part_number().to_string().as_str(), price.to_string().as_str(), dt.format("%b %-d, %-I:%M:%S").to_string().as_str()]);
            table.printstd();
        } else {
            println!("You don't have any to sell!");
        }
    }
    if sellable {
        part_list.decrement(part_number);
    }
    part_list
}

/// Calls the `part_list::sort_by_name()` function.
/// # Arguments
/// * `part_list` - The current operable PartList.
pub fn sort_by_name(mut part_list: PartList) -> PartList {
    part_list.sort_by_name();
    part_list.print();
    part_list
}

/// Calls the `part_list::sort_by_number()` function.
/// # Arguments
/// * `part_list` - The current operable PartList.
pub fn sort_by_number(mut part_list: PartList) -> PartList {
    part_list.sort_by_number();
    part_list.print();
    part_list
}

/// Calls the `part_list::dedup()` function.
/// # Arguments
/// * `part_list` - The current operable PartList.
pub fn dedup(mut part_list: PartList) -> PartList {
    print!("deduplicating...");
    part_list.dedup();
    println!("done!");
    part_list
}
