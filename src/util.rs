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

/// 凯撒密码加密/解密
///
/// # 参数
/// * `text` - 输入文本
/// * `shift` - 偏移量（正数加密，负数解密）
///
/// # 返回
/// 加密或解密后的字符串
pub fn caesar_cipher(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let base = b'A' as i32;
                (((c as i32 - base + shift).rem_euclid(26)) + base) as u8 as char
            } else if c.is_ascii_lowercase() {
                let base = b'a' as i32;
                (((c as i32 - base + shift).rem_euclid(26)) + base) as u8 as char
            } else {
                c
            }
        })
        .collect()
}