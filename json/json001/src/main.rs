use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]

struct Cities {
    cities2: HashMap <String, HashMap<String,String>>
}

fn main() {
    eprintln!("*** 開始 ***");
    let file_name = "cities.json";

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let deserialized: Cities = serde_json::from_reader(reader).unwrap();


    for (key,value) in deserialized.cities2.iter() {
        print!("{}",key);
        print!("\t");
        print!("{}",value["name"]);
        print!("\t");
        print!("{}",value["population"]);
        print!("\t");
        println!("{}",value["date_mod"]);
        }


    eprintln!("*** 終了 ***");
}
