/// learn serde

use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
struct Point {
    id: i32,
    name: String,
}

fn main() {
    let point = Point {id: 1, name:"Nazmul".to_string()};

    let serialized = serde_json::to_string(&point).unwrap();
    println!("Serialized {}",serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("Deserialize: {:?}",deserialized);

}