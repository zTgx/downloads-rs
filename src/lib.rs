static URL: &'static str = "https://crates.io/api/v1/crates?user_id=";

use curl::easy::{Easy};
use curl::easy::{List};
pub use serde_json::{Value, Result, json};

pub struct Header {}
impl Header {
    pub fn get() -> List {
        let mut list = List::new();
        list.append("User-Agent: downloads").unwrap();

        list
    }
}

pub fn fetch(user_id: &str) -> Result<Value> {
    let mut engine = Easy::new();

    engine.http_headers(Header::get()).unwrap();

    let url = URL.to_string() + user_id;
    engine.url(&url).unwrap();

    let mut buf = Vec::new();
    {
        let mut transfer = engine.transfer();
        transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let result: Value = serde_json::from_slice(buf.as_slice()).unwrap();
    let items = result["crates"].clone();
    
    Ok( items )
}
