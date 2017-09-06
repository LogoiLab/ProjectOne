use part;
use part::Part;
use prettytable::Table;

pub struct PartList {
    list: Vec<part::Part>
}

impl PartList {

    pub fn add(&mut self, part: Part) {
        self.list.push(part);
    }
    pub fn remove(&mut self, part: Part) {
        match self.get_index_by_number(part.part_number()) {
            Some(s) => println!("Part: {} was removed.", (self.list.remove(s)).part_name()),
            None => println!("Cannot remove item!"),
        };
    }
    pub fn get_index_by_number(&self, part_number: &i64) -> Option<usize> {
        match self.list.binary_search_by(|part| part.part_number().cmp(part_number)) {
                Ok(o) => Some(o),
                Err(e) => {println!("Cannot locate the item!"); None},
        }
    }
    pub fn get_index_by_name(&self, part_name: String) -> Result<usize, usize> {
        self.list.binary_search_by(|part| part.part_name().cmp(&part_name))
    }

    pub fn sort_by_name(&mut self) {
        self.list.sort_by(|origin, reference| origin.part_name().cmp(reference.part_name()))
    }

    pub fn sort_by_number(&mut self) {
        self.list.sort_by(|origin, reference| origin.part_number().cmp(reference.part_number()))
    }

    pub fn print(&self) {
        let mut table: Table = Table::new();
        table.add_row(row!["Part Name", "Part Number", "Price", "Sale Price", "On Sale", "Quantity"]);
        for part in self.list.iter() {
            table.add_row(row![part.part_name().as_str(), part.part_number().to_string().as_str(), part.list_price().to_string().as_str(), part.sale_price().to_string().as_str(), part.on_sale().to_string().as_str(), part.quantity().to_string().as_str()]);
        }
        table.printstd();
    }

}
