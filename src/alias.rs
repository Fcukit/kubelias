use crate::custom_structs::*;

pub use crate::writer::write_alias;

pub fn process(args: Alias) -> Result<(), Box<dyn std::error::Error>>  {
  write_alias(args.name.into(), args.exec_cmd.into());

  Ok(())
}
