use std::cell::RefCell;

use super::IntoMd;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct CStructField {
    name: RefCell<String>,
    unit: RefCell<String>,
}

impl CStructField {
    pub(crate) fn new(name: &str, unit: &str) -> Self {
        CStructField {
            name: RefCell::new(String::from(name)),
            unit: RefCell::new(String::from(unit)),
        }
    }
}

impl IntoMd for CStructField {
    fn into_md(&self) -> String {
        format!("{} {};", self.unit.borrow(), self.name.borrow())
    }
}
