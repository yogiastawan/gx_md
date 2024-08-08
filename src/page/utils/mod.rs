use std::cell::RefCell;

use content::Content;
use side_panel::SidePanel;

pub(crate) mod content;
pub(crate) mod side_panel;

pub(crate) struct Page {
    content: RefCell<Content>,
    side_panel: RefCell<SidePanel>,
}
