use crate::utils::{
    c_function::CFunction, c_function_param::CFunctionParams, c_includes::CIncludes,
    c_struct::CStruct, c_struct_field::CStructField,
};

pub(crate) fn parse_inc(str: &str) -> CIncludes {
    let str = str.trim();
    println!("STR inc: {}", str);
    let file = str.strip_prefix("#include").unwrap();
    let name = file.trim().replace("\"", "");
    let inc = CIncludes::new();
    inc.set_name(&name);
    inc
}

pub(crate) fn parse_cstruct(str: &str) -> (String, CStruct) {
    let c_struct = CStruct::new();
    let str = str.trim().strip_prefix("struct").unwrap().trim();
    println!("STR struct: {}", str);
    let name = &str[..str.find("{").unwrap()];
    let name = name.trim();
    println!("STR name: {}", name);
    c_struct.set_name(name);

    let fields = str.strip_prefix(name).unwrap().trim();
    println!("STR Fi: {}", fields);
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
    println!("STR Fis: {}", field[0]);

    field.into_iter().for_each(|f| {
        let f = f.trim();
        println!("f: {}", f);
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
    let str = str.strip_prefix(unit).unwrap().trim_start();
    let name = str[..str.find("(").unwrap()].trim_end();
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
        let unit = p[0];
        let name = if p.len() < 2 {
            None
        } else {
            Some(p[1].to_string())
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

    let alias = &str[str.find("}").unwrap()..]
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
    println!("TY str: {}", str);
    let str = str.strip_prefix("struct").unwrap().trim();
    let str = str.split(" ").collect::<Vec<&str>>();
    let name = str[0].trim();
    let alias = str[1].trim().strip_suffix(";").unwrap().trim_end();
    cs.set_name(name);
    cs.set_alias(alias);
    (alias.to_string(), cs)
}
