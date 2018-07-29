extern crate clap;
extern crate fluent;
extern crate fluent_syntax;

use std::fs::File;
use std::io;
use std::io::Read;

use clap::App;

use fluent_syntax::ast::Resource;
use fluent_syntax::parser::errors::display::annotate_error;
use fluent_syntax::parser::parse;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn print_entries_resource(res: &Resource) {
    println!("{:#?}", res);
}

fn main() {
    let matches = App::new("Fluent Parser")
        .version("1.0")
        .about("Parses FTL file into an AST")
        .args_from_usage(
            "-s, --silence 'disable output'
             <INPUT> 'Sets the input file to use'",
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();

    let source = read_file(&input).expect("Read file failed");

    let res = parse(&source);

    if matches.is_present("silence") {
        return;
    };

    match res {
        Ok(res) => print_entries_resource(&res),
        Err((res, errors)) => {
            print_entries_resource(&res);
            println!("==============================\n");
            if errors.len() == 1 {
                println!("Parser encountered one error:");
            } else {
                println!("Parser encountered {} errors:", errors.len());
            }
            println!("-----------------------------");
            for err in errors {
                let f = annotate_error(&err, &Some(input.to_string()), true);
                println!("{}", f);
                println!("-----------------------------");
            }
        }
    };
}
