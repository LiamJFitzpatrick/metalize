use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use bson::{bson, Bson, doc, Document};
use std::fs::File;
use std::io::{BufReader, Seek, Write, Read};
use std::error::Error;


pub struct Collection<T> 
{
    pub entries: Vec<T>
}

impl<T> Collection<T>{
    fn get(&self) -> Option<&T>{
        self.entries.get(0)
    }
    fn new( entries: Vec<T>) -> Collection<T>{
        Collection{
            entries:entries
        }
    }
}

impl<T> Collection<T>
    where 
    T : Serialize
{
    pub fn save<W>(&self, mut writer : W) -> Result<(), Box<dyn Error>>
     where W : Write {
        for a in self.entries.iter(){
            let d = bson::to_document(a)?;
            d.to_writer(&mut writer)?;
        }
        Ok(())
    }

}

impl<T> Collection<T>
where
T: DeserializeOwned
{
    pub fn load(mut file: File) -> Result<Collection<T>, Box<dyn Error>>{
        let n = file.metadata()?.len();
        let mut reader = BufReader::new(file);
        let mut current_position = reader.stream_position()?;
        let mut entries = Vec::new();
        while current_position < n{
            let d = Document::from_reader(&mut reader)?;
            let h: T = bson::from_document(d)?;
            entries.push(h);
            current_position = reader.stream_position()?;
        }
        Ok(Collection::new( entries))
    }
}