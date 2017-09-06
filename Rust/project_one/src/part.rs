
pub struct Part {
    pub part_name: String,
    pub part_number: i64,
    pub list_price: f64,
    pub sale_price: f64,
    pub on_sale: bool,
    pub quantity: i64
}
impl Part {
    /*fn create (part_name: str, part_number: i64, list_price: f64, sale_price: f64, on_sale: bool, quantity: i64) -> Part {
        Part{partname: part_name, part_number: part_number, list_price: list_price, sale_price: sale_price, on_sale: on_sale, quantity: quantity}
    }*/
    /*fn as_string_record (&self) -> StringRecord {
        let mut record = StringRecord::new();
        record.push_field(&self.part_name);
        record.push_field(&self.part_number);
        record.push_field(&self.list_price);
        record.push_field(&self.sale_price);
        record.push_field(&self.on_sale);
        record.push_field(&self.quantity);
        return record;
    }*/
}
