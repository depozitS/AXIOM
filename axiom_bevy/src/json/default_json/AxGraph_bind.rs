
use serde_json::{Value, json};

impl default_json{

    fn get_axgraph_binds() ->Value{
        json!({
            "TEST":"Caps Lock + a"
        })
    }

}