use libc::c_char;
use std::ffi::CStr;
use std::fs::{File, read_to_string};
use std::io::BufWriter;
use serde_json::{Value, from_str, to_writer_pretty};
use jsonwebtoken::{encode, EncodingKey, Header};

unsafe fn extract_str(param: *const c_char) -> String{
  assert!(!param.is_null());

  CStr::from_ptr(param).to_str().expect("Bad format for input path").to_string()
}

#[no_mangle]
pub extern "C" fn tokenize(input_path: *const c_char, output_path: *const c_char) -> () {
  let input_str = unsafe {extract_str(input_path)};
  let output_str = unsafe {extract_str(output_path)};

  let file_contents = read_to_string(input_str).expect("Fail to read input file");
  let data: Vec<Value> = from_str(&file_contents).expect("Can't read data of input file");

  let mut tokens: Vec<String> = vec![];
  let header = Header::default();
  let secret = EncodingKey::from_secret(b"secret");

  for item in data {
    tokens.push(encode(&header, &item, &secret).expect("JWT encoding failed"))
  }

  let output_file = File::create(output_str).expect("Fail to create output file");
  let output_buffer = BufWriter::new(output_file);
  to_writer_pretty(output_buffer, &tokens).expect("Fail to write new data in output file");

  ()
}