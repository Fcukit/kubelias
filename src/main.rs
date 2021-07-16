use structopt::StructOpt;
use std::process::Command;

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
    println!("output - {:#?}", command);

    Ok(())
}
