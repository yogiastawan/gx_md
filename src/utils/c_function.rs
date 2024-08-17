use std::cell::RefCell;

use crate::page::view::link::Link;

use super::{c_function_param::CFunctionParams, AnchorMd, IntoMd, TitleMd};

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
        let params = params.join(", ");

        format!(
            "\t{} {}({});",
            self.unit.borrow(),
            self.name.borrow(),
            params
        )
    }
}

impl TitleMd for CFunction {
    fn create_title(&self) -> String {
        self.name.borrow().to_owned()
    }
}

impl AnchorMd for CFunction {
    fn create_anchor(&self) -> Option<crate::page::view::link::Link> {
        let title = self.create_title();
        let url = format!("#{}", title.to_lowercase().replace(" ", "-"));
        Some(Link::new(&title, &url, false))
    }
}
