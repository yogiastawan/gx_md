use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
    process::exit,
};

use page::{utils::content::Content, view::FieldView};
use parser::{parse_cstruct, parse_function, parse_inc, parse_ty_struct, parse_typedef};
use utils::{c_function::CFunction, c_includes::CIncludes, c_struct::CStruct, CommentMain};

mod page;
mod utils;

mod parser;

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum Type {
    MainComment,
    Desc,
    Typedef,
    TypedefStruct,
    Struct,
    Func,
    Inc,
    Unknown,
}

impl Type {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            Self::MainComment => "///!",
            Self::Desc => "//!",
            Self::Typedef => "typedef",
            Type::TypedefStruct => "typedef struct",
            Self::Struct => "struct",
            Self::Inc => "#include",
            _ => "",
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("No argument provided");
        exit(1);
    }

    let path = "path";
    let content = read_line(path);
    let content = match content {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2);
        }
    };

    for line in content.flatten() {
        println!("{}", line);
    }
}

fn read_line<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn line_parser(lines: Lines<BufReader<File>>) -> Content {
    let main_comment = CommentMain::new();
    let content: Content = Content::new();

    // let mut is_complete = false;

    let mut temp_func: CFunction = CFunction::new();
    let mut tem_str: CStruct = CStruct::new();
    let mut temp_inc: CIncludes = CIncludes::new();

    let mut desc: String = String::new();
    let mut prev: Type = Type::Unknown;

    let mut str: String = String::new();
    let mut title = String::new();

    for line in lines.flatten() {
        // if length is 0
        if line.len() <= 0 {
            prev = Type::Unknown;
            desc.clear();
            continue;
        }

        // if start with ///!
        if line.starts_with(Type::MainComment.to_str()) {
            main_comment.append(&line[5..]);
            prev = Type::MainComment;
            continue;
        }
        // if start with //!
        else if line.starts_with(Type::Desc.to_str()) {
            if prev != Type::Desc {
                desc.clear();
            }
            desc += &line[4..].trim();
            continue;
        }
        // other
        else {
            if line.starts_with(Type::Inc.to_str()) || prev == Type::Inc {
                // include header must be in one line.
                if !line.contains("\"") {
                    continue;
                }
                prev = Type::Inc;
                str += line.as_str();
                temp_inc = parse_inc(&str);
                str.clear();
            } else if line.starts_with(Type::Typedef.to_str()) || prev == Type::Typedef {
                prev = Type::Typedef;
                str += line.as_str();
                if !line.ends_with(";") {
                    continue;
                }
                (title, tem_str) = parse_typedef(&str);
                str.clear();
            } else if line.starts_with(Type::TypedefStruct.to_str()) || prev == Type::TypedefStruct
            {
                prev = Type::TypedefStruct;
                str += line.as_str();
                if !line.contains("}") {
                    continue;
                }
                if !line.ends_with(";") {
                    continue;
                }
                (title, tem_str) = parse_ty_struct(&str);
                str.clear()
            } else if line.starts_with(Type::Struct.to_str()) || prev == Type::Struct {
                // struct
                prev = Type::Struct;
                str += line.as_str();
                if !line.ends_with("};") {
                    continue;
                }
                (title, tem_str) = parse_cstruct(&str);
                str.clear();
            } else {
                // function
                prev = Type::Func;
                str += line.as_str();
                if !line.ends_with(");") {
                    continue;
                }
                temp_func = parse_function(&str);
                str.clear();
            }
        }

        // if is_complete {
        match prev {
            Type::Inc => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone())
                };
                temp_inc.set_desc(d);
                content.add_include(temp_inc.clone());
            }
            Type::Typedef => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone())
                };
                let fv = FieldView::new(d, Some(title.clone()), tem_str.clone());
                content.add_object(fv);
            }
            Type::TypedefStruct => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone())
                };
                let fv = FieldView::new(d, Some(title.clone()), tem_str.clone());
                content.add_object(fv);
            }
            Type::Struct => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone())
                };
                let fv = FieldView::new(d, Some(title.clone()), tem_str.clone());
                content.add_object(fv);
            }
            Type::Func => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone())
                };

                //TODO! use name funcction as title. and show function as code below title.
                let fv = FieldView::new(d, None, temp_func.clone());
                content.add_func(fv);
            }
            _ => {}
        }
        prev = Type::Unknown;
        desc.clear();
        // is_complete = false;
        // }
    }

    content
}
