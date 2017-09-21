use part;
use part::Part;
use prettytable::{Table, format};

/// Stores a vec of `part::Part`s.
pub struct PartList {
    pub list: Vec<part::Part>
}

/// PartList operators.
impl PartList {

    /// Adds a part.
    pub fn add(&mut self, part: Part) {
        self.list.push(part);
    }

    /// Removes a part by number.
    pub fn remove_by_number(&mut self, part_number: &i64) {
            match self.get_index_by_number(part_number) {
                Some(s) => println!("Part: {} was removed.", (self.list.remove(s)).part_name()),
                None => println!("Cannot remove item!"),
            };
    }

    /// Reduces a `quantity` after a sale.
    pub fn decrement(&mut self, part_number: &i64) {
        let part: &mut Part = self.get_by_number_mut(part_number);
        if part.quantity_mut() > &mut 0 {
            *part.quantity_mut() -= 1;
        } else {
            panic!("Tried to decrement past zero!");
        }
    }

    /// Increses a `quantity`.
    pub fn increment(&mut self, part_number: &i64) {
        let part: &mut Part = self.get_by_number_mut(part_number);
        *part.quantity_mut() += 1;
    }

    /// Gets the indesx of a `Part` by part number.
    fn get_index_by_number(&self, part_number: &i64) -> Option<usize> {
        match self.list.binary_search_by(|part| part.part_number().cmp(part_number)) {
                Ok(o) => Some(o),
                Err(e) => {println!("Cannot locate the item!: {}",e); None},
        }
    }

    /// Gets a `Part` by part number.
    pub fn get_by_number(&self, part_number: &i64) -> &Part {
        let index: usize = self.get_index_by_number(part_number).unwrap();
        &self.list[index]
    }

    /// Gets a mutable `Part` by part number.
    pub fn get_by_number_mut(&mut self, part_number: &i64) -> &mut Part {
        let index: usize = self.get_index_by_number(part_number).unwrap();
        &mut self.list[index]
    }

    /// Gets the index of a `Part` by part name.
    fn get_index_by_name(&self, part_name: String) -> Result<usize, usize> {
        self.list.binary_search_by(|part| part.part_name().cmp(&part_name))
    }

    /// Gets a `Part` by part name.
    pub fn get_by_name(&self, part_name: String) -> &Part {
        let index: usize = self.get_index_by_name(part_name).unwrap();
        &self.list[index]
    }

    /// Sorts the `PartList` by part name.
    pub fn sort_by_name(&mut self) {
        self.list.sort_by(|origin, reference| origin.part_name().cmp(reference.part_name()))
    }

    /// Sorts the `PartList` by part number.
    pub fn sort_by_number(&mut self) {
        self.list.sort_by(|origin, reference| origin.part_number().cmp(reference.part_number()))
    }

    /// Deduplicates the `PartList`.
    pub fn dedup (&mut self) {
        self.list.dedup_by(|origin, reference| origin.part_number().eq(reference.part_number()))
    }

    /// Pretty-prints all the elements of a `PartList`.
    pub fn print(&self) {
        let mut table: Table = Table::new();
        table.add_row(row!["Part Name", "Part Number", "Price", "Sale Price", "On Sale", "Quantity"]);
        for part in self.list.iter() {
            table.add_row(row![part.part_name().as_str(), part.part_number().to_string().as_str(), String::from("$") + part.list_price().to_string().as_str(), String::from("$") + part.sale_price().to_string().as_str(), part.on_sale().to_string().as_str(), part.quantity().to_string().as_str()]);
        }
        table.set_format(*format::consts::FORMAT_NO_COLSEP);
        table.printstd();
        print!("\n")
    }

}
