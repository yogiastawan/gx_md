use std::{
    env,
    fs::{create_dir_all, read_dir, File, ReadDir},
    io::{self, BufRead, BufReader, Lines, Result, Write},
    path::Path,
    process::exit,
    str::Split,
};

use page::{utils::content::Content, view::FieldView, Page};
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

struct GxFile {
    dir: String,
    file: String,
    out_dir: String,
    home_file: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 5 {
        eprintln!("No argument provided.");
        println!("Usage: gx_md -src [source dir] -o [output directory] -h my_lib.h");
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

fn parse_into_file(fo: &GxFile) -> Result<()> {
    let source_file = &fo.file;
    let source_dir = &fo.dir;
    let out_dir = &fo.out_dir;
    let home = &fo.home_file;
    let mut is_home = false;

    let content_file = read_line(source_file);
    let content = match content_file {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(2);
        }
    };

    let content = line_parser(content);

    let page = Page::new();
    page.set_content(Some(content));

    let out_file = source_file.strip_prefix(source_dir).unwrap();

    is_home = {
        let out_file = &out_file[1..];
        out_file == home
    };

    let (splits, file_source_name) = extract_source(&out_file);

    let file_name = match file_source_name {
        Some(x) => {
            page.set_title(x);
            create_file_name(x)
        }
        None => {
            println!("Error: Cannot change file name");
            exit(5);
        }
    };

    let (page_out_rel, side_out_rel) = match splits {
        Some(x) => create_relative_path(x, &file_name, is_home),
        None => {
            println!("Error: Construct file output");
            exit(5);
        }
    };

    let out_dir = if out_dir.ends_with("/") {
        out_dir.strip_suffix("/")
    } else if out_dir.ends_with("\\") {
        out_dir.strip_suffix("/")
    } else {
        Some(out_dir.as_str())
    };

    let (out_page, out_side) = match out_dir {
        Some(x) => {
            let a = String::from(x);
            (a.clone() + &page_out_rel, a + &side_out_rel)
        }
        None => {
            println!("Error: invalid output directory");
            exit(5);
        }
    };

    create_file(&page, &out_page, &out_side)
}

fn create_file_name(str: &str) -> String {
    let mut c = str.as_bytes().to_vec();
    let b = c[0].to_ascii_uppercase();
    c[0] = b;
    let n = match String::from_utf8(c) {
        Ok(x) => x.replace(".h", ".md"),
        Err(e) => {
            eprintln!("Error: Cannot change .h to .md.\n\t Reason: {}", e);
            exit(8);
        }
    };
    n
}

fn create_relative_path(split: Split<&str>, file_name: &str, is_home: bool) -> (String, String) {
    let s = split.collect::<Vec<&str>>();
    let mut f = s.clone();
    let mut file = file_name;
    if is_home {
        file = "Home.md";
        f.remove(&s.len() - 1);
    } else {
        f[&s.len() - 1] = match &file_name.strip_suffix(".md") {
            Some(x) => x,
            None => {
                eprintln!(
                    "Error: File extension is not .md\n Cannot create subdirectory for {}",
                    file
                );
                exit(8);
            }
        };
    }
    (
        format!("{}/{}", f.join("/"), file),
        format!("{}/_Sidebar", f.join("/")),
    )
}

fn extract_source(src: &str) -> (Option<Split<&str>>, Option<&str>) {
    if src.contains("/") {
        let a = src.split("/");
        (Some(a.clone()), a.last())
    } else if src.contains("\\") {
        let a = src.split("\\");
        (Some(a.clone()), a.last())
    } else {
        (None, Some(src))
    }
}

fn create_file(page: &Page, out_page: &str, out_sidebar: &str) -> Result<()> {
    let path = Path::new(&out_page);
    let dir = path.parent().unwrap();
    create_dir_all(dir)?;
    let mut file = File::create(&path)?;
    println!("Write to {}", &out_page);
    file.write_all(page.render_content().as_bytes())?;

    println!("Write to {}", &out_sidebar);
    let path = Path::new(&out_sidebar);
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
