use std::fs::File;

use serde::{Deserialize, Serialize};

mod collection;

#[derive(Serialize, Deserialize)]
struct Potato {
    bob: f64
}

fn main() {
    // let mut peeps: collection::Collection<Potato> = collection::Collection{entries: Vec::new()};

    // peeps.entries.push(Potato{bob:12.0});
    // peeps.entries.push(Potato{bob:42.0});
    // let mut file = File::create("test.bson").unwrap();
    // peeps.save(file).unwrap();

    let mut pops: collection::Collection<Potato> = collection::Collection { entries: Vec::new() };

    let mut file = File::open("test.bson").unwrap();
    pops.load(file).unwrap();

    println!("blah");
}