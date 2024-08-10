use std::{
    fs::File,
    io::{BufReader, Lines},
    process::exit,
};

use crate::{
    file_writer::create_file_name,
    page::{utils::content::Content, view::FieldView},
    utils::{
        c_function::CFunction, c_function_param::CFunctionParams, c_includes::CIncludes,
        c_struct::CStruct, c_struct_field::CStructField, CommentMain,
    },
};

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

pub(crate) fn str_parser(lines: Lines<BufReader<File>>, home: &str) -> Content {
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
                temp_inc = parse_inc(&str, home);
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

pub(crate) fn parse_inc(str: &str, home: &str) -> CIncludes {
    let str = str.trim();
    let file = str.strip_prefix("#include").unwrap();
    let name = file.trim().replace("\"", "");
    let inc = CIncludes::new();
    inc.set_name(&name);
    let is_home = {
        let sep = if home.contains("/") { "/" } else { "\\" };
        let home_file = match home.split(sep).last() {
            Some(x) => x,
            None => home,
        };
        home_file == name
    };
    let url = {
        if is_home {
            String::from("Home")
        } else {
            match create_file_name(&name).strip_suffix(".md") {
                Some(x) => x.to_owned(),
                None => {
                    eprintln!("Error: failed create url for file include {}", &name);
                    exit(9);
                }
            }
        }
    };
    inc.set_url(&url);
    inc
}

pub(crate) fn parse_cstruct(str: &str) -> (String, CStruct) {
    let c_struct = CStruct::new();
    let str = str.trim().strip_prefix("struct").unwrap().trim();
    let name = &str[..str.find("{").unwrap()];
    let name = name.trim();
    c_struct.set_name(name);

    let fields = str.strip_prefix(name).unwrap().trim();
    let fields = fields
        .strip_prefix("{")
        .unwrap()
        .strip_suffix("};")
        .unwrap()
        .trim();
    let field = fields
        .strip_suffix(";")
        .unwrap()
        .split(";")
        .collect::<Vec<&str>>();

    field.into_iter().for_each(|f| {
        let f = f.trim();
        let x = f.split(" ").collect::<Vec<&str>>();
        let csf = CStructField::new(x[0], x[1]);
        c_struct.add_field(csf);
    });

    (format!("struct {}", name), c_struct)
}

pub(crate) fn parse_function(str: &str) -> (String, CFunction) {
    let func = CFunction::new();

    let str = str.trim();
    let unit = &str[..str.find(" ").unwrap()];
    func.set_unit(unit);

    let str = str.strip_prefix(unit).unwrap().trim_start();
    let name = str[..str.find("(").unwrap()].trim_end();
    func.set_name(name);

    let params = str.strip_prefix(name).unwrap().trim_start();
    let params = params
        .strip_prefix("(")
        .unwrap()
        .strip_suffix(");")
        .unwrap()
        .trim();
    let params = params.split(",").collect::<Vec<&str>>();

    for p in params {
        let p = p.trim();
        let p = p.split(" ").collect::<Vec<&str>>();
        let unit = p[0].trim();
        let name = if p.len() < 2 {
            None
        } else {
            Some(p[1].trim().to_string())
        };
        let cp = CFunctionParams::new(unit, name);
        func.add_param(cp);
    }
    (String::from(name), func)
}

pub(crate) fn parse_ty_struct(str: &str) -> (String, CStruct) {
    let c_struct = CStruct::new();
    let str = str.trim();
    let str = str.strip_prefix("typedef").unwrap().trim_start();
    let str = str.strip_prefix("struct").unwrap().trim_start();
    let name = str[..str.find("{").unwrap()].trim();
    c_struct.set_name(name);
    let str = str.strip_prefix(name).unwrap().trim_start();
    let str = str.strip_prefix("{").unwrap();
    let fields = &str[..str.find("}").unwrap()].trim();

    let field = fields
        .strip_suffix(";")
        .unwrap()
        .split(";")
        .collect::<Vec<&str>>();

    field.into_iter().for_each(|f| {
        let f = f.trim();
        let x = f.split(" ").collect::<Vec<&str>>();
        let csf = CStructField::new(x[0], x[1]);
        c_struct.add_field(csf);
    });

    let alias = &str[1 + str.find("}").unwrap()..]
        .trim()
        .strip_suffix(";")
        .unwrap()
        .trim_end();
    c_struct.set_alias(alias);

    (alias.to_string(), c_struct)
}

pub(crate) fn parse_typedef(str: &str) -> (String, CStruct) {
    let cs = CStruct::new();
    let str = str.trim().strip_prefix("typedef").unwrap().trim();
    let str = str.strip_prefix("struct").unwrap().trim();
    let str = str.split(" ").collect::<Vec<&str>>();
    let name = str[0].trim();
    let alias = str[1].strip_suffix(";").unwrap().trim_end();
    cs.set_name(name);
    cs.set_alias(alias);
    (alias.to_string(), cs)
}
