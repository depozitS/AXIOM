use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
pub mod default_json;
// существует ли файл
// если нет создать
// если да проверить совпадает ли по параметрам
// если не совпадает дополнить и удалить нерабочие параметры
// если совпадает то вернуть парами "НАЗВАНИЕ ФУНКЦИИ":"бинд"

fn get_default_params(value :&Value) -> Vec<String>{
    match value{
        Value::Object(map) => map.keys().cloned().collect(),
        _ => Vec::new(),
    }
}

