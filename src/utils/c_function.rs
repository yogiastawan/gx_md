use std::cell::RefCell;

use super::{c_function_param::CFunctionParams, IntoMd};

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct CFunction {
    unit: RefCell<String>,
    name: RefCell<String>,
    parameters: RefCell<Vec<CFunctionParams>>,
}

impl CFunction {
    pub(crate) fn new() -> Self {
        CFunction {
            name: RefCell::new(String::new()),
            unit: RefCell::new(String::new()),
            parameters: RefCell::new(vec![]),
        }
    }

    pub(crate) fn set_unit(&self, str: &str) {
        *self.unit.borrow_mut() = String::from(str);
    }

    pub(crate) fn set_name(&self, str: &str) {
        *self.name.borrow_mut() = String::from(str);
    }

    pub(crate) fn add_param(&self, param: CFunctionParams) {
        self.parameters.borrow_mut().push(param);
    }
}

impl IntoMd for CFunction {
    fn into_md(&self) -> String {
        let params = self
            .parameters
            .borrow()
            .iter()
            .map(|x| x.into_md())
            .collect::<Vec<String>>();
        let params = params.join(",");

        format!("{} {}({})", self.unit.borrow(), self.name.borrow(), params)
    }
}
