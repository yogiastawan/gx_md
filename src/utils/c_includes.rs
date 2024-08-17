use std::cell::RefCell;

use crate::page::view::link::Link;

use super::{AnchorMd, IntoMd, TitleMd};

#[derive(Clone)]
pub(crate) struct CIncludes {
    name: RefCell<String>,
    url: RefCell<String>,
}

impl CIncludes {
    pub(crate) fn new() -> Self {
        CIncludes {
            name: RefCell::new(String::new()),
            url: RefCell::new(String::new()),
        }
    }

    pub(crate) fn set_name(&self, str: &str) {
        *self.name.borrow_mut() = String::from(str);
    }

    pub(crate) fn set_url(&self, str: &str) {
        *self.url.borrow_mut() = String::from(str);
    }
}

impl IntoMd for CIncludes {
    fn into_md(&self) -> String {
        format!("[{}]({})", self.name.borrow(), self.url.borrow())
    }
}

impl TitleMd for CIncludes {
    fn create_title(&self) -> String {
        self.name.borrow().to_owned()
    }
}

impl AnchorMd for CIncludes {
    fn create_anchor(&self) -> Option<Link> {
        let name = self.name.borrow();
        let url = self.url.borrow();
        Some(Link::new(name.as_ref(), url.as_ref(), true))
    }
}
