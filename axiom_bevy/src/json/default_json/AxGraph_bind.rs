
use serde_json::{Value, json};

pub fn get_axgraph_binds() ->Value{
    json!({
        "TEST":"Caps Lock + a"
    })
}