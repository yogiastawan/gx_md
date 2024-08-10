use std::{
    fs::{read_dir, File, ReadDir},
    io::{self, BufRead},
    path::Path,
    process::exit,
};

pub(crate) fn file_list(v: &mut Vec<String>, src: &str) {
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

pub(crate) fn file_extract(v: &mut Vec<String>, files: ReadDir, src: &str) {
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

pub(crate) fn read_line<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
