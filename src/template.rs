
use std::{fmt::format, io::{BufReader, Read, Write}};

use base64::prelude::*;
use flate2::{bufread::{GzDecoder}, Compression, write::{GzEncoder}};
use json::JsonValue;

pub struct Template {
    data: String
}

impl Template {
    pub fn new(data: String) -> Template {
        return Template {
            data
        };
    }

    pub fn from_json(data: String) -> Template {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data.as_bytes());
        let gzipped = encoder.finish().unwrap();
        let base64ed = BASE64_STANDARD.encode(gzipped);
        return Template {
            data: base64ed
        };
    }

    pub fn get_json(&self) -> String {
        let gzipped = BASE64_STANDARD.decode(&self.data).unwrap();
        let mut decoder = GzDecoder::new(gzipped.as_slice());
        let mut s = String::new();
        decoder.read_to_string(&mut s).unwrap();
        s = jsonformat::format(&s, jsonformat::Indentation::FourSpace); // Format it. This may seem unnessecary, but it greatly improves git's ability to merge the data.
        return s;
    }

    fn decode(&self) -> JsonValue {
        let s = self.get_json();
        let parsed = json::parse(&s).unwrap();
        return parsed;
    }

    pub fn get_filename(&self) -> String {
        let parsed = self.decode();
        // Get first block & find name
        let first_block = &parsed["blocks"][0];
        let btype = &first_block["block"];
        let bname: &str;
        if ["func", "process"].contains(&btype.as_str().unwrap()) {
            bname = first_block["data"].as_str().unwrap();
        } else {
            bname = first_block["action"].as_str().unwrap();
        }
        // Return
        return String::from(format!("{}.{}.df", btype, bname));
    }

    pub fn get_data(&self) -> &String {
        return &self.data;
    }
}