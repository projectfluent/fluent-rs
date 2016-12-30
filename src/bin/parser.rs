extern crate fluent;
extern crate getopts;

use std::fs::File;
use std::io::Read;
use std::io;
use std::env;

use getopts::Options;

use fluent::syntax::parse;
use fluent::syntax::ast;


fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

enum PrintingMode {
    Raw,

    #[cfg(any(feature = "json", feature = "entries-json"))]
    JSON,
}

fn print_resource(res: &ast::Resource, mode: PrintingMode) {
    match mode {
        PrintingMode::Raw => println!("{:?}", res),

        #[cfg(any(feature = "json", feature = "entries-json"))]
        PrintingMode::JSON => {
            let e = fluent::syntax::json::serialize_json(res);
            println!("{}", e);
        }
    }
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

    #[cfg(any(feature = "json", feature = "entries-json"))]
    opts.optflag("j", "json", "serialize to json");

    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            print_usage(&program, opts);
            return;
        }
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

    #[allow(unused_mut)]
    match res {
        Ok(res) => {
            let mut mode = PrintingMode::Raw;

            #[cfg(any(feature = "json", feature = "entries-json"))]
            {
                if matches.opt_present("j") {
                    mode = PrintingMode::JSON;
                }
            }

            print_resource(&res, mode);
        }
        Err(err) => println!("Error: {:?}", err),
    };
}
