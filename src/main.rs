use std::{
    env,
    fs::{create_dir_all, read_dir, File, ReadDir},
    io::{self, BufRead, BufReader, Lines, Result, Write},
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

enum Command {
    Src,
    OutDir,
    Name,
}

impl Command {
    pub(crate) fn into_str(&self) -> &str {
        match self {
            Self::Src => "-src",
            Self::OutDir => "-o",
            Self::Name => "gx_md",
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 4 {
        eprintln!("No argument provided.");
        println!("Usage: gx_md -src [source dir] -o [output directory]");
        exit(1);
    }

    let mut src = String::new();
    let mut out = String::new();
    let mut prev: Command = Command::Name;

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
        }
        match prev {
            Command::Src => {
                src = arg;
            }
            Command::OutDir => {
                out = arg;
            }
            Command::Name => {}
        }
    }

    let mut srcs: Vec<String> = vec![];
    file_list(&mut srcs, &src);

    for s in srcs {
        match parse_into_file(&src, &s, &out) {
            Ok(_) => println!("Success parse file: {}", &s),
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(7)
            }
        }
    }
}

fn file_list(v: &mut Vec<String>, src: &str) {
    let files = read_dir(src);

    let files = match files {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(3);
        }
    };

    file_extract(v, files, src)
}

fn file_extract(v: &mut Vec<String>, files: ReadDir, src: &str) {
    for f in files {
        let f = f.unwrap();
        let ty = f.file_type();
        let ty = match ty {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        if ty.is_dir() {
            let r = read_dir(f.path().to_str().unwrap()).unwrap();
            file_extract(v, r, src);
        } else if ty.is_file() {
            let file = f.path();
            let file = file.to_str().unwrap();
            if file.ends_with(".h") {
                v.push(String::from(file));
            }
        }
    }
}

fn parse_into_file(b: &str, p: &str, o: &str) -> Result<()> {
    let mut content = read_line(p);
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

    let out_file = p.strip_prefix(b).unwrap();

    let (s, out_file) = if out_file.contains("/") {
        let a = out_file.split("/");
        (Some(a.clone()), a.last())
    } else if out_file.contains("\\") {
        let a = out_file.split("\\");
        (Some(a.clone()), a.last())
    } else {
        (None, Some(out_file))
    };

    let (out_file, out_side_file) = match out_file {
        Some(x) => {
            page.set_title(x);
            let o = String::from("_");
            let mut c = x.as_bytes().to_vec();
            let b = c[0].to_ascii_uppercase();
            c[0] = b;
            let n = String::from_utf8(c).unwrap().replace(".h", ".md");
            (o.clone() + &n, o + "Sidebar" + &n)
        }
        None => {
            println!("Error: Cannot change file name");
            exit(5);
        }
    };

    let (out_file, out_side_file) = match s {
        Some(x) => {
            let s = x.collect::<Vec<&str>>();
            let mut f = s.clone();
            f[&s.len() - 1] = &out_file;
            let mut b = s.clone();
            b[&s.len() - 1] = &out_side_file;
            (f.join("/"), b.join("/"))
        }
        None => {
            println!("Error: Construct file output");
            exit(5);
        }
    };

    let out_dir = if o.ends_with("/") {
        o.strip_suffix("/")
    } else if o.ends_with("\\") {
        o.strip_suffix("/")
    } else {
        Some(o)
    };

    let (out_file, out_side_file) = match out_dir {
        Some(x) => (
            String::from(x) + &out_file,
            String::from(x) + &out_side_file,
        ),
        None => {
            println!("Error: invalid output directory");
            exit(5);
        }
    };

    let path = Path::new(&out_file);
    let dir = path.parent().unwrap();
    create_dir_all(dir)?;
    let mut file = File::create(&path)?;
    println!("Write to {}", &out_file);
    file.write_all(page.render_content().as_bytes())?;

    println!("Write to {}", &out_side_file);
    let path = Path::new(&out_side_file);
    let mut file = File::create(&path)?;
    file.write_all(page.render_side_bar().unwrap().as_bytes())?;
    Ok(())
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
