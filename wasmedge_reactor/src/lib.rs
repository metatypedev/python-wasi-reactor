mod wasi_vm;

use deno_bindgen::deno_bindgen;
use std::{collections::HashMap};

#[deno_bindgen]
struct WasiReactorInp {
    preopens: Vec<String>,
}

#[deno_bindgen]
pub struct WithRecord {
    my_map: HashMap<String, String>,
}

#[deno_bindgen]
fn test_hashmap() -> WithRecord {
    let mut map = HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    WithRecord { my_map: map }
}