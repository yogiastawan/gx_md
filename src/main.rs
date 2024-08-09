use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
    process::exit,
};

use page::{utils::content::Content, view::FieldView, Page, Renderer};
use parser::{parse_cstruct, parse_function, parse_inc, parse_ty_struct, parse_typedef};
use utils::{c_function::CFunction, c_includes::CIncludes, c_struct::CStruct, CommentMain};

mod page;
mod parser;
mod utils;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum TypeC {
    MainComment,
    Desc,
    Typedef,
    TypedefStruct,
    Struct,
    Func,
    Inc,
    Unknown,
}

impl TypeC {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            Self::MainComment => "///!",
            Self::Desc => "//!",
            Self::Typedef => "typedef",
            Self::TypedefStruct => "typedef struct",
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

    let path = &args[1];
    let content = read_line(path);
    let content = match content {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2);
        }
    };

    let content = line_parser(content);

    let page = Page::new("title");
    page.set_content(Some(content));
    println!("Page:\n{}", page.render());
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

    let mut is_f_main = true;

    let mut temp_func: CFunction = CFunction::new();
    let mut tem_str: CStruct = CStruct::new();
    let mut temp_inc: CIncludes = CIncludes::new();

    let mut desc: String = String::new();
    let mut prev: TypeC = TypeC::Unknown;

    let mut str: String = String::new();
    let mut title = String::new();

    for line in lines.flatten() {
        // if length is 0
        if line.len() <= 0 {
            if prev == TypeC::MainComment {
                is_f_main = false;
            }
            prev = TypeC::Unknown;
            desc.clear();
            continue;
        }

        // if start with ///!
        if line.starts_with(TypeC::MainComment.to_str()) {
            if is_f_main == false {
                continue;
            }
            main_comment.append(&line[5..]);
            prev = TypeC::MainComment;
            continue;
        }
        // if start with //!
        else if line.starts_with(TypeC::Desc.to_str()) {
            is_f_main = false;
            if prev != TypeC::Desc {
                desc.clear();
            }
            prev = TypeC::Desc;
            desc += " ";
            desc += &line[4..].trim();
            continue;
        }
        // other
        else {
            is_f_main = false;
            if line.starts_with(TypeC::Inc.to_str()) {
                // include header must be in one line.
                prev = TypeC::Inc;

                if line.contains("<") {
                    desc.clear();
                    prev = TypeC::Unknown;
                    str.clear();
                    continue;
                }
                str += line.as_str();
                temp_inc = parse_inc(&str);
                str.clear();
            } else if line.starts_with(TypeC::Typedef.to_str()) || prev == TypeC::Typedef {
                prev = TypeC::Typedef;
                str += line.as_str();

                if line.contains("{") || line.contains("}") {
                    // Change to typedef struct
                    prev = TypeC::TypedefStruct;
                    continue;
                }

                if !line.ends_with(";") {
                    continue;
                }

                (title, tem_str) = parse_typedef(&str);
                str.clear();
            } else if line.starts_with(TypeC::TypedefStruct.to_str())
                || prev == TypeC::TypedefStruct
            {
                prev = TypeC::TypedefStruct;
                str += line.as_str();
                if !line.contains("}") {
                    continue;
                }
                if !line.ends_with(";") {
                    continue;
                }
                (title, tem_str) = parse_ty_struct(&str);
                str.clear()
            } else if line.starts_with(TypeC::Struct.to_str()) || prev == TypeC::Struct {
                // struct
                prev = TypeC::Struct;
                str += line.as_str();
                if !line.ends_with("};") {
                    continue;
                }
                (title, tem_str) = parse_cstruct(&str);
                str.clear();
            } else {
                // function
                // if not function run inside if
                if line.starts_with("//") || line.starts_with("#") {
                    str.clear();
                    prev = TypeC::Unknown;
                    continue;
                }
                prev = TypeC::Func;
                str += line.as_str();
                if !line.contains(");") {
                    continue;
                }
                (title, temp_func) = parse_function(&str);
                str.clear();
            }
        }

        // on complete
        match prev {
            TypeC::Inc => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone().trim().to_string())
                };
                temp_inc.set_desc(d);
                content.add_include(temp_inc.clone());
                prev = TypeC::Unknown;
            }
            TypeC::Typedef => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone().trim().to_string())
                };
                let fv = FieldView::new(d, Some(title.clone()), tem_str.clone());
                content.add_object(fv);
                prev = TypeC::Unknown;
            }
            TypeC::TypedefStruct => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone().trim().to_string())
                };
                let fv = FieldView::new(d, Some(title.clone()), tem_str.clone());
                content.add_object(fv);
                prev = TypeC::Unknown;
            }
            TypeC::Struct => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone().trim().to_string())
                };
                let fv = FieldView::new(d, Some(title.clone()), tem_str.clone());
                content.add_object(fv);
                prev = TypeC::Unknown;
            }
            TypeC::Func => {
                let d = if desc.is_empty() {
                    None
                } else {
                    Some(desc.clone().trim().to_string())
                };

                //TODO! use name funcction as title. and show function as code below title.
                let fv = FieldView::new(d, Some(title.clone()), temp_func.clone());
                content.add_func(fv);
                prev = TypeC::Unknown;
            }
            _ => {
                prev = TypeC::Unknown;
            }
        }

        desc.clear();
    }
    content.set_main(Some(main_comment));
    content
}
