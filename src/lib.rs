static URL: &'static str = "https://crates.io/api/v1/crates?user_id=56717";

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

pub fn fetch() -> Result<Value> {
    let mut engine = Easy::new();

    engine.http_headers(Header::get()).unwrap();

    engine.url(URL).unwrap();

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
    let total = result["meta"]["total"].as_u64().unwrap() as usize;
    let items = result["crates"].clone();
    let mut json = json!([]);
    let mut idx = 0usize;
    while idx < total {
        let items = items.clone();
        let item: Value = items.as_array().unwrap()[idx].clone();

        let mut i = json!({});

        i["name"] = item["name"].clone();
        i["downloads"] = item["downloads"].clone();

        json.as_array_mut().unwrap().push( i );

        idx += 1;
    }

    Ok( json )
}
