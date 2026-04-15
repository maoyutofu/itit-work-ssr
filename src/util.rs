use crate::page::{Item, Section};

// pub fn get_data(json_path: &str) -> Vec<Section> {
//     let json_str = match fs::read_to_string(json_path) {
//         Ok(json_str) => json_str,
//         Err(_err) => return Vec::new(),
//     };

//     match serde_json::from_str(&json_str) {
//         Ok(list) => list,
//         Err(_err) => return Vec::new(),
//     }
// }


pub fn get_data(json_str: &str) -> Vec<Section> {
    match serde_json::from_str(&json_str) {
        Ok(list) => list,
        Err(_err) => return Vec::new(),
    }
}

pub fn get_items(json_str: &str) -> Vec<Item> {
    match serde_json::from_str(&json_str) {
        Ok(list) => list,
        Err(_err) => return Vec::new(),
    }
}