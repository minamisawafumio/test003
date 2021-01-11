use serde_json::json;

fn main() {
    let json = json!({
        "foo": "bar",
        "hoge": 0,
        "fuga": true,
        "piyo": [0.1, 0.2, 0.3],
    });
    println!("{}", json);
}