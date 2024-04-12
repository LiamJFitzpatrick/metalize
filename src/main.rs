use std::fs::File;

use metalize::table::Table;
use serde::{Deserialize, Serialize};
use metalize::collection::Collection;

#[derive(Serialize, Deserialize)]
struct Potato {
    bob: f64
}

fn main() {
    // let mut peeps: Collection<Potato> = Collection{entries: Vec::new()};

    // peeps.entries.push(Potato{bob:12.0});
    // peeps.entries.push(Potato{bob:42.0});
    

    // let mut table = Table::init().unwrap();

    // table.insert(peeps).unwrap();

    let mut table = Table::init().unwrap();

    table.addToCollection(1, Potato{bob: 66.0}).unwrap();

    println!("blah");
}