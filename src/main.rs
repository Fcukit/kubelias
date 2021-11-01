mod writer;
mod reader;
mod get_pod;
mod alias;

mod custom_structs;
pub use crate::custom_structs::*;

pub use crate::writer::{write_current_pod};
pub use crate::reader::{read_current_pod, read_current_access_command};
pub use crate::get_pod::process;
pub use crate::alias::process as other_process;

use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = custom_structs::Kubelias::from_args();

    match &args.cmd {
        custom_structs::KuberCommand::GetPod(x) => {
            let get_pod_struct = custom_structs::GetPod { config: x.config.to_path_buf(), namespace: x.namespace.to_string() };
            let _x: Result<(), Box<dyn std::error::Error>> = get_pod::process(get_pod_struct);
        },
        custom_structs::KuberCommand::CurrentPod => {
            read_current_pod();
        },
        custom_structs::KuberCommand::Alias(x) => {
            let alias_struct = custom_structs::Alias { exec_cmd: x.exec_cmd.to_string(), name: x.name.to_string() };
            let _x: Result<(), Box<dyn std::error::Error>> = alias::process(alias_struct);
            read_current_access_command()
        }
    }

    Ok(())
}
