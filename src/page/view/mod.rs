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
    T: IntoMd,
{
    subtitle: RefCell<Option<String>>,
    desc: RefCell<Option<String>>,
    field: RefCell<T>,
}

impl<T> FieldView<T>
where
    T: IntoMd,
{
    pub(crate) fn new(desc: Option<String>, title: Option<String>, field: T) -> Self {
        FieldView {
            subtitle: RefCell::new(title),
            desc: RefCell::new(desc),
            field: RefCell::new(field),
        }
    }
}

impl<T: IntoMd> IntoViewAnchor for FieldView<T> {
    fn into_view(&self) -> String {
        let field = self.field.borrow().into_md();
        let title = self.subtitle.borrow();
        let (f_code, title) = match title.as_ref() {
            Some(x) => (&format!("\n```c\n{}\n```", &field), x),
            None => (&String::from(""), &field),
        };
        let url = format!("#### {}", &title);

        let desc = self.desc.borrow();
        let desc = match desc.as_ref() {
            Some(x) => format!("\n{}", x),
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
        Some(Link::new(title, &url))
    }
}
