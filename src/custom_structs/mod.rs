use structopt::StructOpt;
use serde::{Deserialize};

#[derive(StructOpt, Debug)]
pub struct Kubelias {
    #[structopt(name = "supervisor", default_value = "Puck", long = "supervisor")]
    supervising_faerie: String,
    tree: Option<String>,
    #[structopt(subcommand)]
    pub cmd: KuberCommand
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum KuberCommand {
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

#[derive(Deserialize)]
pub struct Record {
    pub name: String,
    pub ready: String,
    pub status: String,
    pub restarts: String,
    pub age: String
}
