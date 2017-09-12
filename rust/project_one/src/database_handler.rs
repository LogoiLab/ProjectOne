use part::Part;
use part_list::PartList;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use std::io::LineWriter;
use std::fs::{File, OpenOptions};
use std::path::Path;

fn open_database(path: String, part_list: PartList) -> (File, PartList) {
    match File::open(path.as_str()){
        Ok(o) => (o, part_list),
        Err(e) => {
            println!("Database not found! creating! Additional info:\n{}", e);
            match OpenOptions::new().create(true).write(true).open(&Path::new(path.as_str())) {
                Ok(_) => println!("Sucessfully created database."),
                Err(e) => {
                    save_database(String::from("warehouseDB.txt"), part_list);
                    panic!("FAILED TO CREATE DATABASE... EXITING. Additional info:\n{}",e)
                },
            }
            open_database(path, part_list)
        }
    }
}

pub fn load_database(path: String) -> PartList {
    let mut initial_database = open_database(path, PartList{list: vec!()});
    let file_buffer = BufReader::new(initial_database.0);
    for line in file_buffer.lines() {
        let csv_string = line.unwrap();
        let mut split_csv = csv_string.split(",").collect::<Vec<&str>>();
        let part: Part = Part{
            part_name: split_csv.remove(0).trim().to_string(),
            part_number: split_csv.remove(0).trim().to_string().parse::<i64>().unwrap(),
            list_price: split_csv.remove(0).trim().to_string().parse::<f64>().unwrap(),
            sale_price: split_csv.remove(0).trim().to_string().parse::<f64>().unwrap(),
            on_sale: split_csv.remove(0).trim().to_string().parse::<bool>().unwrap(),
            quantity: split_csv.remove(0).trim().to_string().parse::<i64>().unwrap()
        };
        initial_database.1.add(part);
    }
    initial_database.1.dedup();
    initial_database.1
}

pub fn save_database(path: String, part_list: PartList) -> PartList {
    let file = OpenOptions::new().write(true).append(false).truncate(true).open(path.as_str()).unwrap();
    let mut file = LineWriter::new(file);
    for part in &part_list.list {
        let csv_line: String = String::from("") + part.part_name().as_str() + "," + part.part_number().to_string().as_str() + "," + part.list_price().to_string().as_str() + "," + part.sale_price().to_string().as_str() + "," + part.on_sale().to_string().as_str() + "," + part.quantity().to_string().as_str() + "\n";
        for byte in csv_line.into_bytes() {
           file.write(&[byte]).unwrap();
        }
    }
    part_list
}
