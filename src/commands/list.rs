use find::projects::Finder;

pub const USAGE: &str = "
Usage: list

Options:
  --verbose  Print verbose information while finding repositories

List code repositories found in your code directories.

See 'projector help' for more information.
";

#[derive(Deserialize, Debug)]
pub struct Args {
    flag_verbose: bool,
}

pub fn run(finder: Finder, _args: Args) {
    for project in finder {
        println!("{}", project.as_os_str().to_string_lossy());
    }
}
