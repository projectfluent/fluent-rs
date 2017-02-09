extern crate fluent;
extern crate getopts;

use std::fs::File;
use std::io::Read;
use std::io;
use std::env;

use getopts::Options;

use fluent::syntax::parser::parse;
use fluent::syntax::ast::Resource;
use fluent::syntax::errors::display::annotate_slice;
use fluent::syntax::errors;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn print_entries_resource(res: &Resource) {
    println!("{:#?}", res);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("s", "silence", "disable output");
    opts.optflag("r", "raw", "print raw result");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let source = read_file(&input).expect("Read file failed");

    let res = parse(&source);

    if matches.opt_present("s") {
        return;
    };

    let asl = annotate_slice(
        "key = Value\n\nkey2 = Value 2".to_owned(),
        Some("main.ftl".to_owned()),
        errors::get_item(errors::items::GenericError),
        0
    );
    println!("{}", asl);

    let asl = annotate_slice(
        "key = Value\n\nkey2 = Value 2".to_owned(),
        Some("main.ftl".to_owned()),
        errors::get_item(errors::items::UnusedVariable),
        56
    );
    println!("{}", asl);

    // match res {
    //     Ok(res) => print_entries_resource(&res),
    //     Err((res, errors)) => {
    //         print_entries_resource(&res);
    //         println!("==============================\n");
    //         if errors.len() == 1 {
    //             println!("Parser encountered one error:");
    //         } else {
    //             println!("Parser encountered {} errors:", errors.len());
    //         }
    //         println!("-----------------------------");
    //         for err in errors {
    //             println!("{}", err);
    //             println!("-----------------------------");
    //         }
    //     }
    // };
}
