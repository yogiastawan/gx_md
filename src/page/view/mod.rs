use std::cell::RefCell;

use link::Link;

use crate::utils::{AnchorMd, IntoMd, TitleMd};

pub(crate) mod link;

pub(crate) trait IntoViewAnchor {
    fn into_view(&self) -> String;
    fn create_anchor(&self) -> Option<Link>;
}

#[derive(Clone)]
pub(crate) struct FieldView<T>
where
    T: IntoMd + TitleMd + AnchorMd + Clone,
{
    desc: RefCell<Option<String>>,
    object: RefCell<T>,
}

impl<T> FieldView<T>
where
    T: IntoMd + TitleMd + AnchorMd + Clone,
{
    pub(crate) fn new(desc: Option<String>, obj: T) -> Self {
        FieldView {
            desc: RefCell::new(desc),
            object: RefCell::new(obj),
        }
    }

    // pub(crate) fn get_title(&self) -> Option<String> {
    //     let a = self.subtitle.borrow();
    //     a.clone()
    // }

    // pub(crate) fn get_desc(&self) -> Option<String> {
    //     let a = self.desc.borrow();
    //     a.clone()
    // }

    // pub(crate) fn get_field(&self) -> T {
    //     let a = self.field.borrow();
    //     a.clone()
    // }
}

impl<T> IntoViewAnchor for FieldView<T>
where
    T: IntoMd + TitleMd + AnchorMd + Clone,
{
    fn into_view(&self) -> String {
        let object = self.object.borrow().into_md();
        let code_obj = format!("\n\t```c\n{}\n\t```\n", &object);

        let heading = format!("#### **{}**", &self.object.borrow().create_title());

        let desc = self.desc.borrow();
        let desc = match desc.as_ref() {
            Some(x) => format!("\n\n\t{}", x),
            None => String::new(),
        };
        format!("* {}{}{}", heading, code_obj, desc)
    }

    fn create_anchor(&self) -> Option<Link> {
        self.object.borrow().create_anchor()
    }
}
