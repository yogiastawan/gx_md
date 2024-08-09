use std::{fs::read, path::PathBuf, process::exit};

use crate::{line_parser, page::view::IntoViewAnchor, read_line};

use super::parse_inc;

const PATH: &str = "test/test.h";

#[test]
fn parse_include() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test");
    let l = read_line(PATH);
    let l = match l {
        Ok(x) => x,
        Err(e) => {
            eprint!("Error: {e}");
            exit(1)
        }
    };

    let t = line_parser(l);
    let inc = t.get_include();

    for inc in inc {
        print!("{}", inc.into_view());
    }
}
#[test]

fn parse_struct() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test");

    let l = read_line(PATH);
    let l = match l {
        Ok(x) => x,
        Err(e) => {
            eprint!("Error: {e}");
            exit(1)
        }
    };

    let t = line_parser(l);

    let str = t.get_objects();

    for cstru in str {
        print!("{}", cstru.into_view());
    }
}
