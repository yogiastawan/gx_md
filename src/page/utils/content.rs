use std::cell::RefCell;

use crate::{
    page::{
        view::{FieldView, IntoViewAnchor},
        Renderer,
    },
    utils::{c_function::CFunction, c_includes::CIncludes, c_object::CObject, CommentMain, IntoMd},
};

use super::side_panel::SidePanel;

#[derive(Clone)]
pub(crate) struct Content {
    main: RefCell<Option<CommentMain>>,
    object: RefCell<Vec<FieldView<CObject>>>,
    func: RefCell<Vec<FieldView<CFunction>>>,
    incl: RefCell<Vec<FieldView<CIncludes>>>,
}

impl Content {
    pub(crate) fn new() -> Self {
        Content {
            main: RefCell::new(None),
            object: RefCell::new(vec![]),
            func: RefCell::new(vec![]),
            incl: RefCell::new(vec![]),
        }
    }

    pub(crate) fn set_main(&self, main: Option<CommentMain>) {
        *self.main.borrow_mut() = main;
    }

    // pub(crate) fn get_main(&self) -> Option<CommentMain> {
    //     let a = self.main.borrow();
    //     a.clone()
    // }

    pub(crate) fn add_object(&self, obj: FieldView<CObject>) {
        self.object.borrow_mut().push(obj);
    }

    // pub(crate) fn get_objects(&self) -> Vec<FieldView<CStruct>> {
    //     let a = self.object.borrow();
    //     a.clone()
    // }

    pub(crate) fn add_func(&self, fun: FieldView<CFunction>) {
        self.func.borrow_mut().push(fun);
    }

    // pub(crate) fn get_func(&self) -> Vec<FieldView<CFunction>> {
    //     let a = self.func.borrow();
    //     a.clone()
    // }

    pub(crate) fn add_include(&self, inc: FieldView<CIncludes>) {
        self.incl.borrow_mut().push(inc);
    }

    // pub(crate) fn get_include(&self) -> Vec<CIncludes> {
    //     let a = self.incl.borrow();
    //     a.clone()
    // }

    pub(crate) fn create_side_panel(&self) -> SidePanel {
        let sp = SidePanel::new();
        let obj = self.object.borrow();
        obj.iter().for_each(|o| {
            if let Some(x) = o.create_anchor() {
                sp.add_obj(x);
            }
        });

        let fnc = self.func.borrow();
        fnc.iter().for_each(|f| {
            if let Some(x) = f.create_anchor() {
                sp.add_fun(x);
            }
        });

        let incl = self.incl.borrow();
        incl.iter().for_each(|f| {
            if let Some(x) = f.create_anchor() {
                sp.add_includes(x);
            }
        });
        return sp;
    }
}

impl Renderer for Content {
    fn render(&self) -> String {
        let main = self.main.borrow();
        let main = match main.as_ref() {
            Some(x) => x.into_md(),
            None => String::new(),
        };

        let obj = self.object.borrow();
        let obj = if obj.len() > 0 {
            let s: Vec<String> = obj.iter().map(|o| o.into_view()).collect::<Vec<String>>();
            let s = s.join("\n");
            format!("\n### **Objects:**\n\n---\n\n{}", s)
        } else {
            String::new()
        };

        let fun = self.func.borrow();

        let fun = if fun.len() > 0 {
            let s: Vec<String> = fun.iter().map(|o| o.into_view()).collect::<Vec<String>>();
            let s = s.join("\n");
            format!("\n### **Functions:**\n\n---\n\n{}", s)
        } else {
            String::new()
        };

        let inc = self.incl.borrow();

        let inc = if inc.len() > 0 {
            let s: Vec<String> = inc.iter().map(|o| o.into_view()).collect::<Vec<String>>();
            let s = s.join("\n");
            format!("\n### **Includes:**\n\n---\n\n{}", s)
        } else {
            String::new()
        };

        format!("{}{}{}{}", main, obj, fun, inc)
    }
}
