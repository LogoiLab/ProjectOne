pub struct Part {
    pub part_name: String,
    pub part_number: i64,
    pub list_price: f64,
    pub sale_price: f64,
    pub on_sale: bool,
    pub quantity: i64
}
impl Part {

    pub fn part_name(&self) -> &String {
        &self.part_name
    }
    pub fn part_number(&self) -> &i64 {
        &self.part_number
    }
    pub fn list_price(&self) -> &f64 {
        &self.list_price
    }
    pub fn sale_price(&self) -> &f64 {
        &self.sale_price
    }
    pub fn on_sale(&self) -> &bool {
        &self.on_sale
    }
    pub fn quantity(&self) -> &i64 {
        &self.quantity
    }


    pub fn part_name_mut(&mut self) -> &mut String {
        &mut self.part_name
    }
    pub fn part_number_mut(&mut self) -> &mut i64 {
        &mut self.part_number
    }
    pub fn list_price_mut(&mut self) -> &mut f64 {
        &mut self.list_price
    }
    pub fn sale_price_mut(&mut self) -> &mut f64 {
        &mut self.sale_price
    }
    pub fn on_sale_mut(&mut self) -> &mut bool {
        &mut self.on_sale
    }
    pub fn quantity_mut(&mut self) -> &mut i64 {
        &mut self.quantity
    }

}
