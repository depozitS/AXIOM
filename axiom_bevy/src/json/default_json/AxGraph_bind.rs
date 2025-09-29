
use serde_json::{Value, json};

fn get_axgraph_binds() ->Value{
    json!({
        "TEST":"Caps Lock + a"
    })
}