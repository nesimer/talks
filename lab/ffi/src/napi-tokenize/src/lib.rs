#![deny(clippy::all)]
use napi::bindgen_prelude::*;
use std::fs::{File, read_to_string};
use std::io::BufWriter;
use serde_json::{Value, from_str, to_writer_pretty};
use jsonwebtoken::{encode, EncodingKey, Header};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn tokenize(input_path: String, output_path:String) -> Result<()> {
  let output_file = File::create(output_path)?;
  let output_buffer = BufWriter::new(output_file);

  let file_contents = read_to_string(input_path)?;
  let data: Vec<Value> = from_str(&file_contents).expect("Can't read data of input file");

  let mut tokens: Vec<String> = vec![];
  let header = Header::default();
  let secret = EncodingKey::from_secret(b"secret");

  for item in data {
    tokens.push(encode(&header, &item, &secret).expect("JWT encoding failed"))
  }

  to_writer_pretty(output_buffer, &tokens).expect("Fail to write new data in output file");

  Ok(())
}
