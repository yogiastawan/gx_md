use std::cell::RefCell;

use c_variable_field::CVariableField;

use crate::page::view::link::Link;

use super::{AnchorMd, IntoMd, TitleMd};

pub(crate) mod c_variable_field;

#[derive(Clone, Copy)]
pub(crate) enum CObjectType {
    Struct,
    Union,
    Alias,
    Unknown,
}

impl CObjectType {
    pub(crate) fn into_str(&self) -> &str {
        match self {
            CObjectType::Struct => "struct",
            CObjectType::Union => "union",
            _ => "",
        }
    }
}

#[derive(Clone)]
pub(crate) struct CObject {
    obj_type: CObjectType,
    name: RefCell<String>,
    fields: RefCell<Vec<CVariableField>>,
    alias: RefCell<Option<String>>,
}

impl CObject {
    pub(crate) fn new() -> Self {
        CObject {
            obj_type: CObjectType::Unknown,
            name: RefCell::new(String::new()),
            fields: RefCell::new(vec![]),
            alias: RefCell::new(None),
        }
    }

    pub(crate) fn set_obj_type(&mut self, obj_type: CObjectType) {
        self.obj_type = obj_type;
    }

    pub(crate) fn set_name(&self, name: &str) {
        *self.name.borrow_mut() = String::from(name);
    }

    pub(crate) fn add_field(&self, field: CVariableField) {
        self.fields.borrow_mut().push(field);
    }

    pub(crate) fn set_alias(&self, alias: Option<String>) {
        *self.alias.borrow_mut() = alias;
    }

    fn struct_md(&self) -> String {
        let name = format!("struct {}", self.name.borrow());
        let fields = self
            .fields
            .borrow()
            .iter()
            .map(|x| x.into_md())
            .collect::<Vec<String>>();

        let fields = match fields.len() > 0 {
            true => fields.join("\n\t\t"),
            false => String::from("*NO FIELDS*"),
        };
        let alias = self.alias.borrow();
        let alias = match alias.as_ref() {
            Some(x) => format!("\n\ttypedef {} {};", name, x),
            None => String::new(),
        };

        format!("\t{}{{\n\t\t{}\n\t}};{}", name, fields, alias)
    }

    fn union_md(&self) -> String {
        let name = format!("union {}", self.name.borrow());
        let fields = self
            .fields
            .borrow()
            .iter()
            .map(|x| x.into_md())
            .collect::<Vec<String>>();

        let fields = match fields.len() > 0 {
            true => fields.join("\n\t\t"),
            false => String::from("*NO FIELDS*"),
        };
        let alias = self.alias.borrow();
        let alias = match alias.as_ref() {
            Some(x) => format!("\n\ttypedef {} {};", name, x),
            None => String::new(),
        };

        format!("\t{}{{\n\t\t{}\n\t}};{}", name, fields, alias)
    }

    fn alias_md(&self) -> String {
        let name = format!("union {}", self.name.borrow());

        let alias = self.alias.borrow();
        let alias = match alias.as_ref() {
            Some(x) => format!("\n\ttypedef {} {};", name, x),
            None => String::new(),
        };

        alias
    }
}

impl IntoMd for CObject {
    fn into_md(&self) -> String {
        match self.obj_type {
            CObjectType::Struct => self.struct_md(),
            CObjectType::Union => self.union_md(),
            CObjectType::Alias => self.alias_md(),
            CObjectType::Unknown => String::new(),
        }
    }
}

impl TitleMd for CObject {
    fn create_title(&self) -> String {
        let alias = self.alias.borrow();
        let title = match alias.as_ref() {
            Some(x) => x,
            None => {
                let pre = self.obj_type.into_str();
                let pre = if pre.len() == 0 {
                    ""
                } else {
                    &format!(" {}", pre)
                };
                &format!("{}{}", pre, self.name.borrow())
            }
        };
        title.to_owned()
    }
}

impl AnchorMd for CObject {
    fn create_anchor(&self) -> Option<Link> {
        let title = self.create_title();
        let url = format!("#{}", title.to_lowercase().replace(" ", "-"));
        Some(Link::new(&title, &url, false))
    }
}
