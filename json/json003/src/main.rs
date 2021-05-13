use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    topic1: String,
    topic2: String,
    topic3:String,
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

    print_json(point);

    print_serialized_json(serialized);

}

//-----------------------------------------------------------------------
fn print_serialized_json(i_json: String){
    println!("★　★　★　★　★　★　★　★　★ print_serialized_json");
    let sd: Point = serde_json::from_str(&i_json).unwrap();
    println!("deserialized - {:?}", sd);
    println!("deserialized x={}", sd.x);
    println!("deserialized x={}", sd.x);
    println!("deserialized y={}", sd.y);
    println!("deserialized topic1={}", sd.topic1);
    println!("deserialized topic2={}", sd.topic2);
    println!("deserialized topic3={}", sd.topic3);
}

//-----------------------------------------------------------------------
fn  print_json(point: Point){
    println!("★　★　★　★　★　★　★　★　★ print_json");
    println!("deserialized - {:?}", point);
    println!("deserialized x={}", point.x);
    println!("deserialized x={}", point.x);
    println!("deserialized y={}", point.y);
}
