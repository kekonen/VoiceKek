extern crate reqwest;
use serde::{Deserialize};

use std::io;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Result {
    file_id: String,
    file_size: i64,
    file_path: String,
}

#[derive(Debug, Deserialize)]
struct Answer {
  ok: bool,
  result: Result,
}

pub fn download_file(token: &str, file_id: &str, filename: &str) -> Option<i64> {
    println!("Downloading========> {}\n{}", token, file_id);
    let info_url = format!("https://api.telegram.org/bot{}/getFile?file_id={}", token, file_id);

    match reqwest::get(&info_url){
        Ok(mut response) => {
            println!("response========> {:?}", response);
            match response.json::<Answer>() {
                Ok(json) => {
                    println!("JSON========> {:?}", json);
                    //return Some(3.12);
                    // match json.ok.parse::<f32>() {
                    //     Ok(p) => return Some(p),
                    //     _ => return None,
                    // };
                    match json.ok {
                        true => {
                            let file_url = format!("https://api.telegram.org/file/bot{}/{}", token, json.result.file_path);
                            let file_path = format!("files/{}", filename);
                            let mut resp = reqwest::get(&file_url).expect("request failed");
                            let mut out = File::create(file_path).expect("failed to create file");
                            io::copy(&mut resp, &mut out).expect("failed to copy content");

                            return Some(json.result.file_size);
                        },
                        _ => return None,
                    }
                },
                _ => return None,
            };
        },
        Err(e) => {
            println!("Errr ====> {:?}", e);
            return None
        },
        _ => return None,
    };
}
// Download file :
// https://api.telegram.org/bot<bot token>/getFile?file_id=AwADBAADHQUAAje4yVP5rcuqXqA_7wI
// https://api.telegram.org/file/bot<token>/<file_path>