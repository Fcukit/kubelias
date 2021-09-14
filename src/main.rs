mod writer;
mod get_pod;

mod custom_structs;
pub use crate::custom_structs::*;

pub use crate::writer::write_current_pod;
pub use crate::get_pod::process;

use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = custom_structs::Kubelias::from_args();
    println!("{:#?}", &args);

    match &args.cmd {
        custom_structs::KuberCommand::GetPod(x) => {
            println!("GetPod !!! {:#?}", &args.cmd);
            let get_pod_struct = custom_structs::GetPod { config: x.config.to_path_buf(), namespace: x.namespace.to_string() };
            let _x: Result<(), Box<dyn std::error::Error>> = get_pod::process(get_pod_struct);
        },
        custom_structs::KuberCommand::CurrentPod => { println!("CurrentPod !!!") },
        _ => {
            println!("Error !!!");
            ()
        }
    }

    Ok(())
}

