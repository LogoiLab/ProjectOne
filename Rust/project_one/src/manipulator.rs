use database_handler::save_database;
use importer::{import_line,import_file};
use part::Part;
use part_list::PartList;
use std::io;
use std::io::prelude::*;
use chrono::{Local, DateTime};
use prettytable::Table;

pub fn prompt(decorator: String) -> String {
    let stdin = io::stdin();
    println!("{}", decorator);
    let buffer = stdin.lock().lines().next().unwrap().unwrap();
    buffer
}

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
pub fn enter(part_list: PartList) -> PartList {
    import_line(part_list)
}
pub fn read(part_list: PartList) -> PartList {
    import_file(prompt(String::from("Filename to read:\n")), part_list)
}

pub fn save(part_list: PartList) -> PartList {
    save_database(String::from("warehouseDB.txt"), part_list)
}
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
pub fn sort_by_name(mut part_list: PartList) -> PartList {
    part_list.sort_by_name();
    part_list.print();
    part_list
}
pub fn sort_by_number(mut part_list: PartList) -> PartList {
    part_list.sort_by_number();
    part_list.print();
    part_list
}
