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
                Some(s) => println!("Part: {} was removed.", self.list.remove(s).part_name()),
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
        let index: usize = match self.get_index_by_number(part_number) {
            Some(o) => o,
            None => panic!("Index mismatch!"),
        };
        &self.list[index]
    }

    /// Gets a mutable `Part` by part number.
    pub fn get_by_number_mut(&mut self, part_number: &i64) -> &mut Part {
        let index: usize = match self.get_index_by_number(part_number) {
            Some(o) => o,
            None => panic!("Index mismatch!"),
        };
        &mut self.list[index]
    }

    /// Gets the index of a `Part` by part name.
    fn get_index_by_name(&self, part_name: String) -> Result<usize, usize> {
        self.list.binary_search_by(|part| part.part_name().cmp(&part_name))
    }

    /// Gets a `Part` by part name.
    pub fn get_by_name(&self, part_name: String) -> &Part {
        let index: usize = match self.get_index_by_name(part_name) {
            Ok(o) => o,
            Err(e) => e,
        };
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
        self.sort_by_number();
        self.list.dedup_by(|origin, reference|
            if origin.part_number().eq(reference.part_number()) {
                *reference.quantity_mut() += *origin.quantity_mut();
                *reference.on_sale_mut() = *origin.on_sale_mut();
                *reference.sale_price_mut() = *origin.sale_price_mut();
                *reference.list_price_mut() = *origin.list_price_mut();
                return true;
            } else {
                return false;
            }
        )
    }

    /// Pretty-prints all the elements of a `PartList`.
    pub fn print(&self) {
        let mut quant: i64 = 0;
        let mut quant_table: Table = Table::new();
        let mut table: Table = Table::new();
        table.add_row(row!["Part Name", "Part Number", "Price", "Sale Price", "On Sale", "Quantity"]);
        for part in self.list.iter() {
            quant += part.quantity().abs();
            table.add_row(row![part.part_name().as_str(), part.part_number().to_string().as_str(), String::from("$") + part.list_price().to_string().as_str(), String::from("$") + part.sale_price().to_string().as_str(), part.on_sale().to_string().as_str(), part.quantity().to_string().as_str()]);
        }
        quant_table.add_row(row!["Total Parts", quant.to_string().as_str()]);
        quant_table.set_format(*format::consts::FORMAT_NO_COLSEP);
        quant_table.printstd();
        table.set_format(*format::consts::FORMAT_NO_COLSEP);
        table.printstd();
        print!("\n")
    }

}
