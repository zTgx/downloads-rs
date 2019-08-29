extern crate downloads;

use downloads::*;

fn main() {
    let v: Result<Value> = fetch();
    if let Ok(x) = v {
        if let Some(arr) = x.as_array() {
            for i in arr {
                println!("name: {}", i["name"]);
                println!("downloads: {}",i["downloads"]);
            }
        }
    }
}
