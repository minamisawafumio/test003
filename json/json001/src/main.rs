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

    let w_cities: Cities = serde_json::from_reader(reader).unwrap();


    print_json(w_cities);


    eprintln!("*** 終了 ***");



}

fn  print_json(target: Cities){
    eprintln!("★★★*** 終了 ***");
    for (key,value) in target.cities2.iter() {
        print!("{}\t{}\t{}\t{}\n",key,value["name"],value["population"],value["date_mod"]);
     }
}