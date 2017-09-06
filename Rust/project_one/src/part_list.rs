use part;
use part::Part;

pub struct PartList {
    list: Vec<part::Part>
}
fn create_test() -> Vec<Part> {
    let mut test_part: Part = Part{part_name: String::from("my part"), part_number: 12345678 as i64, list_price: 12.34, sale_price: 11.11, on_sale: false, quantity: 34 as i64};
    let mut test_part_list = vec!();
    test_part_list.push(test_part);
    test_part_list
}
pub fn get_price(part_name: String) -> f64 {
    create_test().pop().unwrap().list_price
}
