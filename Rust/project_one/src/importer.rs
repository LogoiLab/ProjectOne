use database_handler::save_database;
use part::Part;
use part_list::PartList;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn import_line(mut part_list: PartList) -> PartList{
    let stdin = io::stdin();
    let buffer = stdin.lock().lines().next().unwrap().unwrap();
    let mut split_csv: Vec<&str> = buffer.split(",").collect::<Vec<&str>>();
    let part: Part = Part{
        part_name: split_csv.remove(0).trim().to_string(),
        part_number: split_csv.remove(0).trim().to_string().parse::<i64>().unwrap(),
        list_price: split_csv.remove(0).trim().to_string().parse::<f64>().unwrap(),
        sale_price: split_csv.remove(0).trim().to_string().parse::<f64>().unwrap(),
        on_sale: split_csv.remove(0).trim().to_string().parse::<bool>().unwrap(),
        quantity: split_csv.remove(0).trim().to_string().parse::<i64>().unwrap()
    };
    part_list.add(part);
    part_list.dedup();
    part_list
}
pub fn import_file(path: String, mut part_list: PartList) -> PartList {
    let db_file = match File::open(path.as_str()) {
        Ok(o) => o,
        Err(e) => {
            save_database(String::from("warehouseDB.txt"), part_list);
            panic!("File error! Additional info: {}", e);
        },
    };
    let file_buffer = BufReader::new(&db_file);
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
        part_list.add(part);
        part_list.dedup();
    }
    part_list
}
