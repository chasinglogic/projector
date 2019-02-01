use crate::find::projects::Finder;
use std::io;
use std::io::Write;

pub const USAGE: &str = "
Usage: list [options]

Options:
  --verbose  Print verbose information while finding repositories

List code repositories found in your code directories.

See 'projector help' for more information.
";

#[derive(Deserialize, Debug)]
pub struct Args {
  flag_verbose: bool,
}

pub fn run(finder: Finder, _args: &Args) {
  for project in finder {
    match writeln!(
      io::stdout().lock(),
      "{}",
      project.as_os_str().to_string_lossy()
    ) {
      Ok(()) => (),
      Err(_) => (),
    };
  }
}
