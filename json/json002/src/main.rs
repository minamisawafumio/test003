use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    pub topic1: String,
    pub topic2: String,
    pub topic3:String,
    x: i32,
    y: i32,
}

fn main() {
    let point = Point {
        topic1: "My first topic1".to_string(),
        topic2: "My first topic2".to_string(),
        topic3: "My first topic3".to_string(),
             x: 11,
             y: 22
    };

    // let serialized = serde_json::to_string(&point).unwrap();     //未整形
    let serialized = serde_json::to_string_pretty(&point).unwrap(); //整形
    println!("serialized = {}", serialized);


    //
    let sd: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized - {:?}", sd);
    println!("deserialized x={}", sd.x);
    println!("deserialized x={}", sd.x);
    println!("deserialized y={}", sd.y);
}