use std::fs::File;
use serde::{Serialize};
use std::error::Error;

use crate::collection::Collection;

struct TableRow {
    id: u32,
    name: String,
    num_entries: u32,
    path: String
}

impl TableRow
{
    fn new<T:Serialize>(&mut self, id : u32, name: String, entry: Collection<T>) -> Result<(), Box<dyn Error>>{
        self.id = id;
        self.name = name.clone();
        self.num_entries = u32::try_from(entry.entries.len()).unwrap();
        self.path = format!("{}.bson",name.clone());
        let mut file = File::create(self.path.clone()).unwrap();
        entry.save(file)?;
        Table::insert(self);
        Ok(())
    }
}

mod Table{
    use super::TableRow;


    pub fn insert(row :&mut TableRow){
        
    }
}