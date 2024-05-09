use clap::{Arg, ArgAction, Command};

use fluent_cli::parse_file;

fn main() {
    let matches = Command::new("Fluent Parser")
        .version("0.0.1")
        .about("Parses FTL file into an AST")
        .arg(
            Arg::new("silent")
                .short('s')
                .long("silent")
                .action(ArgAction::SetTrue)
                .help("Disables error reporting"),
        )
        .arg(Arg::new("FILE").required(true).help("FTL file to parse"))
        .get_matches();

    let input: &String = matches.get_one("FILE").unwrap();
    let silent: bool = *matches.get_one("silent").unwrap();
    parse_file(input, silent);
}
