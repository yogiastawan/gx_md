use std::cell::RefCell;

use super::IntoMd;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct CFunctionParams {
    unit: RefCell<String>,
    name: RefCell<Option<String>>,
}

impl CFunctionParams {
    pub(crate) fn new(unit: &str, name: Option<String>) -> Self {
        CFunctionParams {
            unit: RefCell::new(String::from(unit)),
            name: RefCell::new(name),
        }
    }
}

impl IntoMd for CFunctionParams {
    fn into_md(&self) -> String {
        let name = self.name.borrow();
        let name = match name.as_ref() {
            Some(x) => x,
            None => "",
        };

        format!("{} {}", self.unit.borrow(), name)
    }
}
