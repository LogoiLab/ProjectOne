use part_list;

pub fn get_price(part_name: String) -> f64 {
    part_list::get_price(String::from(part_name.trim()))
}
