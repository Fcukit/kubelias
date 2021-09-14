use std::process::Command;
use std::str;

use crate::custom_structs::*;

use csv::StringRecord;
use regex::Regex;

pub use crate::writer::write_current_pod;

use std::collections::HashMap;
use std::io::{self};

pub fn process(args: GetPod) -> Result<(), Box<dyn std::error::Error>>  {
  let command = Command::new("kubectl")
                        .arg("get")
                        .arg("pods")
                        .arg("--kubeconfig")
                        .arg(&args.config)
                        .arg("-n")
                        .arg(&args.namespace)
                        .output()
                        .expect("fail");

  let s = match str::from_utf8(&command.stdout) {
      Ok(v) => v,
      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
  };

  let mut reader = csv::Reader::from_reader(s.as_bytes());

  let headers = reader.headers()?;

  let re = Regex::new(r"\s+").unwrap();
  let t = re.replace_all(&headers[0], ",");

  let v: Vec<&str> = t.split(',').collect();
  let lv: Vec<_> = v.iter().map(|s| s.to_lowercase()).collect();

  let header = StringRecord::from(lv);

  let mut reader = csv::Reader::from_reader(s.as_bytes());

  let mut hash = HashMap::new();

  for (i, record) in reader.records().enumerate() {
      let record = record?;

      let re = Regex::new(r"\s+").unwrap();
      let t = re.replace_all(&record[0], ",");

      let v: Vec<&str> = t.split(',').collect();
      let lv: Vec<_> = v.iter().map(|s| s.to_lowercase()).collect();
      let string_record = StringRecord::from(lv);
      let row: Record = string_record.deserialize(Some(&header))?;
      hash.insert(i + 1, String::from(row.name));
  }

  for (key, value) in &hash {
      println!("{} :{}", key, value);
  }

  println!("Press number of needed pod or 'q' to quit.");

  let mut input_text = String::new();
  io::stdin()
      .read_line(&mut input_text)
      .expect("failed to read from stdin");

  let trimmed = input_text.trim();

  match trimmed.parse::<u8>() {
      Ok(i) => {
          println!("your integer input: {}", i);
          let value = hash.get(&i.into()).unwrap();
          println!("Your choice is {:?}", value);
          write_current_pod(value.into());

          println!("{:#?}", args);

        },
      Err(..) => println!("this was not an integer: {}", trimmed),
  };

  Ok(())
}
