use std::{
    fs::{create_dir_all, File},
    io::{Result, Write},
    path::Path,
    process::exit,
};

use crate::{file_reader::read_line, page::Page, parser::str_parser};

pub(crate) struct GxFile {
    pub(crate) dir: String,
    pub(crate) file: String,
    pub(crate) out_dir: String,
    pub(crate) home_file: String,
}

pub(crate) fn parse_into_file(fo: &GxFile) -> Result<()> {
    let source_file = &fo.file;
    let source_dir = &fo.dir;
    let out_dir = &fo.out_dir;
    let home = &fo.home_file;
    let is_home: bool;

    let path_separator = if source_file.contains("/") { "/" } else { "\\" };

    println!("::> Reading file source.");
    let content_file = read_line(source_file);
    let content = match content_file {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Reading failed.\nError: {}.", e);
            exit(2);
        }
    };

    let content = str_parser(content, home);
    let page = Page::new();
    page.set_content(Some(content));

    let source_dir = {
        let s_o = if source_dir.contains(path_separator) {
            source_dir.strip_suffix(path_separator)
        } else {
            Some(source_dir.as_str())
        };

        match s_o {
            Some(x) => x,
            None => source_dir,
        }
    };

    let out_file = source_file.strip_prefix(source_dir).unwrap();

    page.set_path_src(out_file);

    is_home = {
        let home = if home.starts_with(path_separator) {
            &home[1..]
        } else {
            home
        };
        let out_file = &out_file[1..];
        out_file == home
    };

    let (splits, file_source_name) = extract_source(out_file, path_separator);

    let file_name = match file_source_name {
        Some(x) => {
            page.set_title(x);
            create_file_name(x)
        }
        None => {
            eprintln!("Error: Cannot change file name.");
            exit(5);
        }
    };

    let (page_out_rel, side_out_rel) = match splits {
        Some(x) => create_relative_path(x, &file_name, is_home),
        None => {
            eprintln!("Error: Construct file output.");
            exit(5);
        }
    };

    let out_dir = if out_dir.ends_with(path_separator) {
        out_dir.strip_suffix(path_separator)
    } else {
        Some(out_dir.as_str())
    };

    let (out_page, out_side) = match out_dir {
        Some(x) => {
            let a = String::from(x);
            (a.clone() + &page_out_rel, a + &side_out_rel)
        }
        None => {
            eprintln!("Error: invalid output directory.");
            exit(5);
        }
    };
    println!("::> Writing file documentation.");
    create_file(&page, &out_page, &out_side)
}

pub(crate) fn create_file_name(str: &str) -> String {
    let mut c = str.as_bytes().to_vec();
    let b = c[0].to_ascii_uppercase();
    c[0] = b;
    let n = match String::from_utf8(c) {
        Ok(x) => x.replace(".h", ".md"),
        Err(e) => {
            eprintln!("Error: Cannot change .h to .md.\n\t Reason: {}.", e);
            exit(8);
        }
    };
    n
}

pub(crate) fn create_relative_path(
    split: Vec<&str>,
    file_name: &str,
    is_home: bool,
) -> (String, String) {
    let mut f = split.clone();
    let mut file = file_name;
    if is_home {
        file = "Home.md";
        f.remove(&split.len() - 1);
    } else {
        f[&split.len() - 1] = match &file_name.strip_suffix(".md") {
            Some(x) => x,
            None => {
                eprintln!(
                    "Error: File extension is not .md\n Cannot create subdirectory for {}.",
                    file
                );
                exit(8);
            }
        };
    }
    (
        format!("{}/{}", f.join("/"), file),
        format!("{}/_Sidebar.md", f.join("/")),
    )
}

pub(crate) fn extract_source<'a>(
    src: &'a str,
    sep: &'a str,
) -> (Option<Vec<&'a str>>, Option<&'a str>) {
    if src.contains(sep) {
        let a = src.split(sep).collect::<Vec<&str>>();
        (Some(a.clone()), a.last().copied())
    } else {
        (None, Some(src))
    }
}

pub(crate) fn create_file(page: &Page, out_page: &str, out_sidebar: &str) -> Result<()> {
    let path = Path::new(&out_page);
    let dir = path.parent().unwrap();
    create_dir_all(dir)?;
    let mut file = File::create(&path)?;
    file.write_all(page.render_content().as_bytes())?;

    let path = Path::new(&out_sidebar);
    let mut file = File::create(&path)?;
    file.write_all(page.render_side_bar().unwrap().as_bytes())?;
    Ok(())
}
