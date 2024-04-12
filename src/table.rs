use std::{fs::File, io::{Seek, BufReader}};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use std::path::Path;
use serde_json;


use crate::collection::Collection;

#[derive(Serialize, Deserialize)]
struct TableRow {
    id: usize,
    name: Uuid,
    num_entries: usize,
    path: String
}

impl TableRow
{
    fn new<T:Serialize>(id: usize, name: Uuid, entry: Collection<T>) -> Result<TableRow, Box<dyn Error>>{
        let path = format!("{}.bson", name);
        let mut file = File::create(&path)?;
        entry.save(file)?;
        Ok(
            TableRow { id: id, name: name, num_entries: entry.entries.len(), path: path  }
        )
    }
}

pub struct Table{
    rows: Vec<TableRow>
}

impl Table {

    fn new() -> Table{
        Table{ rows:Vec::new()}
    }

    pub fn init() -> Result<Table, Box<dyn Error>>{
        let mut table = Table::new();

        // check for existing table entries
        let path = Path::new("metalize.table");
        if path.exists(){
            let file = File::open(path)?;
            let n = file.metadata()?.len();
            let mut file = BufReader::new(file);
            while file.stream_position()? < n {
                table.rows.push(serde_json::from_reader(&mut file)?);
            }
        }
        Ok(table)
    }

    pub fn insert<T:Serialize>(&mut self, entry: Collection<T>) -> Result<(), Box<dyn Error>> { 
        let table_row = TableRow::new(
            self.rows.len()+1,
            Uuid::new_v4(),
            entry
        )?;
        let path = Path::new("metalize.table");

        let y = if path.exists() {
            File::open(path)?
        } else {
            File::create(path)?
        };
        let x = serde_json::to_value(&table_row)?;
        serde_json::to_writer(&y, &x)?;
        self.rows.push(table_row);
        Ok(())
    }

    pub fn get<T:DeserializeOwned>(&self, id: usize)-> Result<Collection<T>, Box<dyn Error>>{
        let table_row = self.rows.get(id-1).expect("Could not access that id.");
        let mut file = File::open(&table_row.path)?;
        Collection::load(file)
    }
}