use std::{fs::{File,remove_file}, io::{Seek, BufReader}};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use std::path::Path;
use serde_json;
use crate::collection::Collection;


#[derive(Serialize, Deserialize, Clone)]
struct TableRow {
    id: usize,
    name: Uuid,
    num_entries: usize,
    path: String,
    file_pos: u64
}

impl TableRow
{
    fn new<T:Serialize>(id: usize, name: Uuid, entry: Collection<T>, file_pos: u64) -> Result<TableRow, Box<dyn Error>>{
        let path = format!("{}.bson", name);
        let file = File::create(&path)?;
        entry.save(file)?;
        Ok(
            TableRow { id: id, name: name, num_entries: entry.entries.len(), path: path, file_pos: file_pos  }
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

    fn rewrite_table(&mut self) -> Result<(), Box<dyn Error>>{
        remove_file("metalize.table")?;
        let file = File::create("metalize.table")?;
        for row in self.rows.iter(){
            let x = serde_json::to_value(row)?;
            serde_json::to_writer(&file, &x)?;
        }
        Ok(())
    }

    pub fn insert<T:Serialize>(&mut self, entry: Collection<T>) -> Result<(), Box<dyn Error>> { 
        let path = Path::new("metalize.table");
        let y = if path.exists() {
            File::open(path)?
        } else {
            File::create(path)?
        };
        let new_pos = y.metadata()?.len();
        let table_row = TableRow::new(
            self.rows.len()+1,
            Uuid::new_v4(),
            entry,
            new_pos
        )?;
        let x = serde_json::to_value(&table_row)?;
        serde_json::to_writer(&y, &x)?;
        self.rows.push(table_row);
        Ok(())
    }

    pub fn add_to_collection<T:Serialize + DeserializeOwned>(&mut self, collection_id : usize, object: T) -> Result<(), Box<dyn Error>>{
        // TODO: this currently wipes out files and rewrites the entirety of them. Update to only change the sections that need to change.
        let selected_row = self.get_row(collection_id);
        let mut selected_collection: Collection<T> = Collection::load(File::open(&selected_row.path)?)?;
        remove_file(&selected_row.path)?;
        selected_collection.entries.push(object);
        selected_row.num_entries = selected_collection.entries.len();
        selected_collection.save(File::create(&selected_row.path)?)?;
        self.rewrite_table()?;
        Ok(())
    }

    pub fn update<T:Serialize>(&mut self, id: usize, entry: Collection<T>)-> Result<(), Box<dyn Error>>{
        // TODO: this currently wipes out files and rewrites the entirety of them. Update to only change the sections that need to change.
        let selected_row = self.get_row(id);
        remove_file(&selected_row.path)?;
        selected_row.num_entries = entry.entries.len();
        entry.save(File::create(&selected_row.path)?)?;
        self.rewrite_table()?;
        Ok(())
    }

    fn get_row(&mut self, id: usize) -> &mut TableRow{
        let row_index = self.rows.iter().position(move |row |{
            row.id == id
        }).expect("Failed to find that row in table.");
        &mut self.rows[row_index]
    }

    pub fn get<T:DeserializeOwned>(&mut self, id: usize)-> Result<Collection<T>, Box<dyn Error>>{
        let table_row = self.get_row(id);
        let file = File::open(&table_row.path)?;
        Collection::load(file)
    }
}