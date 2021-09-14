mod writer;
mod get_pod;

pub use crate::writer::write_current_pod;
pub use crate::get_pod::process;

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
use std::collections::HashMap;

use csv::StringRecord;

use serde::{Serialize, Deserialize};

use regex::Regex;
extern crate confy;
use std::io::Read;

#[derive(Deserialize)]
struct Record {
    name: String,
    ready: String,
    status: String,
    restarts: String,
    age: String
}

#[derive(StructOpt, Debug)]
struct Kubelias {
    #[structopt(name = "supervisor", default_value = "Puck", long = "supervisor")]
    supervising_faerie: String,
    tree: Option<String>,
    #[structopt(subcommand)]
    cmd: KuberCommand
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
enum KuberCommand {
    CurrentPod,
    GetPod(GetPod),
    Alias {
        #[structopt(short = "a", long = "alias")]
        alias: String,

        cmd: String
    }
}

#[derive(StructOpt, Debug)]
pub struct GetPod {
  /// kubectl config file
  #[structopt(parse(from_os_str), short = "c", long = "config")]
  pub config: std::path::PathBuf,

  /// kubectl namespace
  #[structopt(short = "n", long = "namespace")]
  pub namespace: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Kubelias::from_args();
    println!("{:#?}", &args);

    match &args.cmd {
        KuberCommand::GetPod(x) => {
            println!("GetPod !!! {:#?}", &args.cmd);
            let get_pod_struct = get_pod::GetPod { config: x.config.to_path_buf(), namespace: x.namespace.to_string() };
            let x: Result<(), Box<dyn std::error::Error>> = get_pod::process(get_pod_struct);
        },
        CurrentPod => { println!("CurrentPod !!!") },
        _ => {
            println!("Error !!!");
            ()
        }
    }

    Ok(())
}

