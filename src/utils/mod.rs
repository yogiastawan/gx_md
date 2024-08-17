use std::cell::RefCell;

use crate::page::view::link::Link;

pub(crate) mod c_function;
pub(crate) mod c_function_param;
pub(crate) mod c_includes;
pub(crate) mod c_struct;
pub(crate) mod c_struct_field;
pub(crate) mod c_typedef;

pub(crate) trait IntoMd {
    fn into_md(&self) -> String;
}

pub(crate) trait TitleMd {
    fn create_title(&self) -> String;
}

pub(crate) trait AnchorMd {
    fn create_anchor(&self) -> Option<Link>;
}

// when start with ///!
#[derive(Clone)]
pub(crate) struct CommentMain {
    content: RefCell<Vec<String>>,
}

impl CommentMain {
    pub(crate) fn new() -> Self {
        CommentMain {
            content: RefCell::new(vec![]),
        }
    }

    pub(crate) fn append(&self, str: &str) {
        self.content.borrow_mut().push(String::from(str));
    }
}

impl IntoMd for CommentMain {
    fn into_md(&self) -> String {
        self.content.borrow().join("\n")
    }
}
