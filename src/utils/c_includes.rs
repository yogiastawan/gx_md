use std::cell::RefCell;

use crate::page::view::{link::Link, IntoViewAnchor};

#[derive(Clone)]
pub(crate) struct CIncludes {
    name: RefCell<String>,
    url: RefCell<String>,
    desc: RefCell<Option<String>>,
}

impl CIncludes {
    pub(crate) fn new() -> Self {
        CIncludes {
            name: RefCell::new(String::new()),
            url: RefCell::new(String::new()),
            desc: RefCell::new(None),
        }
    }

    pub(crate) fn set_name(&self, str: &str) {
        *self.name.borrow_mut() = String::from(str);
    }

    pub(crate) fn set_url(&self, str: &str) {
        *self.url.borrow_mut() = String::from(str);
    }
    pub(crate) fn set_desc(&self, str: Option<String>) {
        *self.desc.borrow_mut() = str;
    }
}

impl IntoViewAnchor for CIncludes {
    fn into_view(&self) -> String {
        let desc = self.desc.borrow();
        let desc = match desc.as_ref() {
            Some(x) => format!("\\\n\t{}", x),
            None => String::new(),
        };
        format!("* [{}]({}){}", self.name.borrow(), self.url.borrow(), desc)
    }

    fn create_anchor(&self) -> Option<Link> {
        let name = self.name.borrow();
        let url = self.url.borrow();
        Some(Link::new(name.as_ref(), url.as_ref(), true))
    }
}
