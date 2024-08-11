use std::cell::RefCell;

use super::IntoMd;

#[derive(Clone)]
pub(crate) struct CTypedef {
    name: RefCell<String>,
    alias: RefCell<String>,
}

impl CTypedef {
    pub(crate) fn new() -> Self {
        CTypedef {
            name: RefCell::new(String::new()),
            alias: RefCell::new(String::new()),
        }
    }

    pub(crate) fn set_name(&self, name: &str) {
        *self.name.borrow_mut() = String::from(name);
    }

    pub(crate) fn set_alias(&self, alias: &str) {
        *self.alias.borrow_mut() = String::from(alias);
    }
}

impl IntoMd for CTypedef {
    fn into_md(&self) -> String {
        format!("typedef {} {};", self.name.borrow(), self.alias.borrow())
    }
}
