extern crate reqwest;
use reqwest::r#async::{Client, Response, Decoder};
use crate::futures::*;

use serde::{Deserialize};
extern crate serde_json;

use sha2::{Sha256, Digest};
use data_encoding::HEXUPPER;

use std::io;
use std::str;
use std::fs::File;
use bytes::IntoBuf;
use bytes::Buf;
use std::io::Read;


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

pub fn get_hash(filename: &str) -> Option<String> {
    let mut data = Vec::new();
    let mut file = File::open(&filename).unwrap();
    file.read_to_end(&mut data).expect("Unable to read data");

    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher);
    let output = hasher.result();
    let b2s = HEXUPPER.encode(output.as_ref());

    Some(b2s)
}


pub fn download_file(token: &str, file_id: &str, filename: &str) -> Option<(i64, String)> { // impl Future<Item=(), Error=()>
    let client = Client::new();

    let info_url = format!("https://api.telegram.org/bot{}/getFile?file_id={}", token, file_id);

    let result_info = |mut res : Response | {
        res.json::<Answer>()
    };
    
    let json = client.get(&info_url)
    .send()
    .and_then(result_info)
    .map_err(|e| format!("Error: {:?}", e))
    .wait()
    .unwrap();

    let file_url = format!("https://api.telegram.org/file/bot{}/{}", token, json.result.file_path);
    let file_path = format!("{}", filename);
    
    let b2s_hash = client.get(&file_url)
    .send()
    .and_then(|res : Response | {
        res.into_body().concat2()
    })
    .and_then(|chunks| {
        let mut out = File::create(&file_path).expect("failed to create file");
        io::copy(&mut chunks.into_buf().bytes(), &mut out).expect("failed to copy content");
        Ok(file_path)
    })
    .and_then(|x|{
        Ok(get_hash(&x).unwrap())
    })
    .map_err(|e| format!("Error: {:?}", e))
    .wait()
    .unwrap();

    Some((json.result.file_size, b2s_hash))
}
// Download file :
// https://api.telegram.org/bot<bot token>/getFile?file_id=AwADBAADHQUAAje4yVP5rcuqXqA_7wI
// https://api.telegram.org/file/bot<token>/<file_path>