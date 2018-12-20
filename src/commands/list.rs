use finder::Finder;

pub const USAGE: &str = "";

#[derive(Deserialize, Debug)]
pub struct Args {
    flag_verbose: bool,
    flag_version: bool,
    flag_help: bool,
    flag_code_dir: String,
    flag_exclude: Option<String>,
    flag_include: Option<String>,
    arg_command: String,
    arg_args: Vec<String>,
}

pub fn run(finder: Finder, args: Args) {
    unimplemented!();
}
