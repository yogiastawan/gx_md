use std::{env, process::exit};

use file_reader::file_list;
use file_writer::{parse_into_file, GxFile};

mod file_reader;
mod file_writer;
mod page;
mod parser;
mod utils;

enum Command {
    Src,
    OutDir,
    Home,
    Name,
}

impl Command {
    pub(crate) fn into_str(&self) -> &str {
        match self {
            Self::Src => "-src",
            Self::OutDir => "-o",
            Self::Home => "-h",
            Self::Name => "gx_md",
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 5 {
        eprintln!("No argument provided.");
        println!("Usage: gx_md -src [source dir] -o [output directory] -h [home_src_file.h]");
        exit(1);
    }

    let mut prev: Command = Command::Name;

    let (src, out, home) = {
        let mut src = String::new();
        let mut out = String::new();
        let mut home = String::new();
        for arg in args {
            if arg == Command::Src.into_str() {
                prev = Command::Src;
                continue;
            } else if arg == Command::OutDir.into_str() {
                prev = Command::OutDir;
                continue;
            } else if arg == Command::Name.into_str() {
                prev = Command::Name;
                continue;
            } else if arg == Command::Home.into_str() {
                prev = Command::Home;
                continue;
            }
            match prev {
                Command::Src => {
                    src = arg;
                }
                Command::OutDir => {
                    out = arg;
                }
                Command::Name => {}
                Command::Home => {
                    home = arg;
                }
            }
        }
        (src, out, home)
    };

    let mut srcs: Vec<String> = vec![];
    file_list(&mut srcs, &src);

    for s in srcs {
        let indexed_file = GxFile {
            dir: src.clone(),
            file: s.clone(),
            out_dir: out.clone(),
            home_file: home.clone(),
        };
        match parse_into_file(&indexed_file) {
            Ok(_) => println!("Success parse file: {}", &s),
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(7)
            }
        }
    }
}
