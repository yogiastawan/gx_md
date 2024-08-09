use std::cell::RefCell;

use crate::{
    page::{view::link::Link, Renderer},
    utils::IntoMd,
};

pub(crate) struct SidePanel {
    objetcs_name: RefCell<Vec<Link>>,
    functions: RefCell<Vec<Link>>,
    includes: RefCell<Vec<Link>>,
}

impl SidePanel {
    pub(crate) fn new() -> Self {
        SidePanel {
            objetcs_name: RefCell::new(vec![]),
            functions: RefCell::new(vec![]),
            includes: RefCell::new(vec![]),
        }
    }

    pub(crate) fn add_obj(&self, l: Link) {
        self.objetcs_name.borrow_mut().push(l);
    }

    pub(crate) fn get_objs(&self) -> Vec<Link> {
        let a = self.objetcs_name.borrow();
        a.clone()
    }
    pub(crate) fn add_fun(&self, l: Link) {
        self.functions.borrow_mut().push(l);
    }

    pub(crate) fn get_func(&self) -> Vec<Link> {
        let a = self.functions.borrow();
        a.clone()
    }

    pub(crate) fn add_includes(&self, l: Link) {
        self.includes.borrow_mut().push(l);
    }

    pub(crate) fn get_incl(&self) -> Vec<Link> {
        let a = self.includes.borrow();
        a.clone()
    }
}

impl Renderer for SidePanel {
    fn render(&self) -> String {
        let obj = self.objetcs_name.borrow();
        let obj_len = obj.len();
        let obj = if obj_len > 0 {
            obj.iter()
                .map(|x| x.into_md())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            String::from("")
        };

        let func = self.functions.borrow();
        let func_len = func.len();
        let func = if func_len > 0 {
            func.iter()
                .map(|x| x.into_md())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            String::from("")
        };

        let incl = self.includes.borrow();
        let incl_len = incl.len();
        let incl = if incl_len > 0 {
            incl.iter()
                .map(|x| x.into_md())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            String::from("")
        };

        format!(
            "#### **Objects ({})**\n{}\n#### **Functions ({})**\n{}\n#### **Includes ({})**\n{}",
            obj_len, obj, func_len, func, incl_len, incl
        )
    }
}
