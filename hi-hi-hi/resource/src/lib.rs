use itertools;
use rust_embed::Embed;
use serde_json::Value;

#[derive(Embed)]
#[folder = "bh3_data"]
pub struct Bh3Data;

impl Bh3Data {
    pub fn print_json_test(file_name: &str) {
        let file_path = format!("ExcelOutputAsset/{}.json", file_name);

        match Bh3Data::get(&file_path) {
            Some(file_content) => match std::str::from_utf8(&file_content.data) {
                Ok(content_str) => match serde_json::from_str::<Value>(content_str) {
                    Ok(json_data) => {
                        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
                    }
                    Err(e) => {
                        eprintln!("json parse err: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("couldnt conv to string: {}", e);
                }
            },
            None => {
                eprintln!("didnt find: {}", file_name);
            }
        }
    }
}
