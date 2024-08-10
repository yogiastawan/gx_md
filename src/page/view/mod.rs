use std::cell::RefCell;

use link::Link;

use crate::utils::IntoMd;

pub(crate) mod link;

pub(crate) trait IntoViewAnchor {
    fn into_view(&self) -> String;
    fn create_anchor(&self) -> Option<Link>;
}

#[derive(Clone)]
pub(crate) struct FieldView<T>
where
    T: IntoMd + Clone,
{
    subtitle: RefCell<Option<String>>,
    desc: RefCell<Option<String>>,
    field: RefCell<T>,
}

impl<T> FieldView<T>
where
    T: IntoMd + Clone,
{
    pub(crate) fn new(desc: Option<String>, title: Option<String>, field: T) -> Self {
        FieldView {
            subtitle: RefCell::new(title),
            desc: RefCell::new(desc),
            field: RefCell::new(field),
        }
    }

    pub(crate) fn get_title(&self) -> Option<String> {
        let a = self.subtitle.borrow();
        a.clone()
    }

    pub(crate) fn get_desc(&self) -> Option<String> {
        let a = self.desc.borrow();
        a.clone()
    }

    pub(crate) fn get_field(&self) -> T {
        let a = self.field.borrow();
        a.clone()
    }
}

impl<T: IntoMd + Clone> IntoViewAnchor for FieldView<T> {
    fn into_view(&self) -> String {
        let field = self.field.borrow().into_md();
        let title = self.subtitle.borrow();
        let (f_code, title) = match title.as_ref() {
            Some(x) => (&format!("\n\t```c\n{}\n\t```", &field), x),
            None => (&String::from(""), &field),
        };
        let url = format!("#### **{}**", &title);

        let desc = self.desc.borrow();
        let desc = match desc.as_ref() {
            Some(x) => format!("\n\n\t{}", x),
            None => String::new(),
        };
        format!("* {}{}{}", url, &f_code, desc)
    }

    fn create_anchor(&self) -> Option<Link> {
        let field = self.field.borrow().into_md();
        let title = self.subtitle.borrow();
        let title = match title.as_ref() {
            Some(x) => x,
            None => &field,
        };
        let url = format!("{}", title.to_lowercase().replace(" ", "-"));
        Some(Link::new(title, &url, false))
    }
}
