use std::process::Command;
use std::str;

use crate::custom_structs::*;

use csv::StringRecord;
use regex::Regex;

pub use crate::writer::write_alias;

use std::collections::HashMap;
use std::io::{self};

pub fn process(args: Alias) -> Result<(), Box<dyn std::error::Error>>  {
  write_alias(args.name.into(), args.exec_cmd.into());
  // println!("name: {}, command: {}", args.name, args.exec_cmd);

  Ok(())
}
