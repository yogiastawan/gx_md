use std::{borrow::Borrow, cell::RefCell};

use super::{c_struct_field::CStructField, IntoMd};

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct CStruct {
    name: RefCell<String>,
    fields: RefCell<Vec<CStructField>>,
    alias: RefCell<Option<String>>,
}

impl CStruct {
    pub(crate) fn new() -> Self {
        CStruct {
            name: RefCell::new(String::new()),
            fields: RefCell::new(vec![]),
            alias: RefCell::new(None),
        }
    }

    pub(crate) fn set_name(&self, name: &str) {
        *self.name.borrow_mut() = String::from(name);
    }

    pub(crate) fn add_field(&self, field: CStructField) {
        self.fields.borrow_mut().push(field);
    }

    pub(crate) fn set_alias(&self, alias: &str) {
        *self.alias.borrow_mut() = Some(String::from(alias));
    }
}

impl IntoMd for CStruct {
    fn into_md(&self) -> String {
        let name = format!("struct {}", self.name.borrow());
        let fields = self
            .fields
            .borrow()
            .iter()
            .map(|x| x.into_md())
            .collect::<Vec<String>>();

        let fields = match fields.len() > 0 {
            true => fields.join("\n\t"),
            false => String::from("\n\t*PRIVATE FIELD*"),
        };
        let alias = self.alias.borrow();
        let alias = match alias.as_ref() {
            Some(x) => format!("\ntypedef {} {};", name, x),
            None => String::new(),
        };

        format!("{}{{{}\n}}{}", name, fields, alias)
    }
}
