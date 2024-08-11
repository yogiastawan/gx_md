use std::cell::RefCell;

use utils::{content::Content, side_panel::SidePanel};

pub(crate) mod utils;
pub(crate) mod view;

pub(crate) trait Renderer {
    fn render(&self) -> String;
}

pub(crate) struct Page {
    title: RefCell<String>,
    path_src: RefCell<String>,
    md: RefCell<Option<Content>>,
    left_side: RefCell<Option<SidePanel>>,
}

impl Page {
    pub(crate) fn new() -> Self {
        Page {
            title: RefCell::new(String::new()),
            path_src: RefCell::new(String::new()),
            md: RefCell::new(None),
            left_side: RefCell::new(None),
        }
    }

    pub(crate) fn set_title(&self, str: &str) {
        *self.title.borrow_mut() = String::from(str);
    }

    pub(crate) fn set_content(&self, content: Option<Content>) {
        *self.md.borrow_mut() = content;
    }

    pub(crate) fn set_path_src(&self, path: &str) {
        *self.path_src.borrow_mut() = String::from(path);
    }

    pub(crate) fn render_content(&self) -> String {
        let contn = self.md.borrow();

        let content = match contn.as_ref() {
            Some(x) => format!("\n{}", x.render()),
            None => String::new(),
        };
        format!(
            "# {}\n*{}*\n\n---\n{}",
            self.title.borrow(),
            self.path_src.borrow(),
            content
        )
    }

    pub(crate) fn render_side_bar(&self) -> Option<String> {
        let side_bar = self.left_side.borrow();
        let contn = self.md.borrow();

        let side_bar = match side_bar.as_ref() {
            Some(x) => x.render(),
            None => match contn.as_ref() {
                Some(x) => x.create_side_panel().render(),
                None => String::new(),
            },
        };

        Some(side_bar)
    }
}

impl Renderer for Page {
    fn render(&self) -> String {
        let side_bar = self.left_side.borrow();
        let contn = self.md.borrow();

        let side_bar = match side_bar.as_ref() {
            Some(x) => x.render(),
            None => match contn.as_ref() {
                Some(x) => x.create_side_panel().render(),
                None => String::new(),
            },
        };

        let content = match contn.as_ref() {
            Some(x) => x.render(),
            None => String::from(""),
        };
        format!("## {}\n{}\n{}", self.title.borrow(), side_bar, content)
    }
}
