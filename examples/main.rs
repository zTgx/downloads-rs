extern crate downloads;

use downloads::*;

fn main() {
    let mut totals = 0;
    let user_id = "56717";
    let v: Result<Value> = fetch(user_id);
    if let Ok(x) = v {
        if let Some(arr) = x.as_array() {
            for i in arr {

                println!("crate name: {}", i["name"]);
                println!("downloads: {}",i["downloads"]);
                println!("\n");
                totals += i["downloads"].as_u64().unwrap() as usize;
            }
        }
    }

    println!("Total Downloads: {}", totals);
}
