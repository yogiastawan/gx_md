use std::cell::RefCell;

use crate::utils::IntoMd;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct CVariableField {
    name: RefCell<String>,
    unit: RefCell<String>,
}

impl CVariableField {
    pub(crate) fn new(name: &str, unit: &str) -> Self {
        CVariableField {
            name: RefCell::new(String::from(name)),
            unit: RefCell::new(String::from(unit)),
        }
    }
}

impl IntoMd for CVariableField {
    fn into_md(&self) -> String {
        format!("{} {};", self.unit.borrow(), self.name.borrow())
    }
}
