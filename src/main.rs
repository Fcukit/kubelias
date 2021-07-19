use structopt::StructOpt;
use std::process::Command;

#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{self, Write};
use std::str;
use std::error::Error;
use csv;

use csv::StringRecord;
use serde::Deserialize;

use regex::Regex;

#[derive(Deserialize)]
struct Record {
    name: String,
    ready: String,
    status: String,
    restarts: String,
    age: String
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    /// kubectl config file
    #[structopt(short = "c", long = "config")]
    config: String,

    /// kubectl namespace
    #[structopt(short = "n", long = "namespace")]
    namespace: String,

    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    cmd: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();

    let command = Command::new("kubectl")
                           .arg("get")
                           .arg("pods")
                           .arg("--kubeconfig")
                           .arg(args.config)
                           .arg("-n")
                           .arg(args.namespace)
                           .output()
                           .expect("fail");

    // io::stdout().write_all(&command.stdout).unwrap();

    let s = match str::from_utf8(&command.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // println!("result: {}", s);

    // println!("Press number of needed pod or 'q' to quit.");

    let mut stdout = io::stdout();
    let no_modifiers = KeyModifiers::empty();

    // // //key detection
    // match read().unwrap() {
    //     Event::Key(KeyEvent {
    //             code: KeyCode::Char('h'),
    //             modifiers: no_modifiers,
    //     }) => execute!(stdout, Clear(ClearType::CurrentLine), Print("Hello world!")).unwrap(),
    //     Event::Key(KeyEvent {
    //             code: KeyCode::Char('q'),
    //             modifiers: no_modifiers,
    //     }) => return Ok(()),
    //     _ => ()
    // }

    let mut reader = csv::Reader::from_reader(s.as_bytes());

    let headers = reader.headers()?;

    let re = Regex::new(r"\s+").unwrap();
    let t = re.replace_all(&headers[0], ",");

    let v: Vec<&str> = t.split(',').collect();
    let lv: Vec<_> = v.iter().map(|s| s.to_lowercase()).collect();

    println!("Headers: {:?}", lv);

    let header = StringRecord::from(lv);

    let mut reader = csv::Reader::from_reader(s.as_bytes());
    for record in reader.records() {
        let record = record?;

        let re = Regex::new(r"\s+").unwrap();
        let t = re.replace_all(&record[0], ",");

        let v: Vec<&str> = t.split(',').collect();
        let lv: Vec<_> = v.iter().map(|s| s.to_lowercase()).collect();
        let string_record = StringRecord::from(lv);
        let row: Record = string_record.deserialize(Some(&header))?;
        // println!("{:?}", string_record);
        println!("{:?}", row.name);
    }

    Ok(())
}
