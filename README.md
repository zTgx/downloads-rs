# [downloads-rs](https://github.com/zTgx/downloads-rs) [![Build Status](https://travis-ci.org/zTgx/downloads-rs.svg?branch=master)](https://travis-ci.org/zTgx/downloads-rs) [![crate](https://img.shields.io/crates/v/downloads.svg)](https://crates.io/crates/downloads)

WIP  
A lib for check [my-crates](https://crates.io/me/crates) all downloads.

# Usage
Add dependencies
```rust
[dependencies]
downloads = "0.0.2"
```

Example
```rust
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

```

### Find UserID
__Steps__
1. Open https://crates.io  
2. Login  
3. Open Chrome, More Tools -> Developer Tools, select Network Tab, then refresh  
4. Finally, find something like `crates?user_id=56717`, double click, will see all your user info.  
