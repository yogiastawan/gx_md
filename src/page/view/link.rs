use crate::utils::IntoMd;

#[derive(Clone)]
pub(crate) struct Link {
    name: String,
    url: String,
    go_page: bool,
}

impl Link {
    pub(crate) fn new(name: &str, url: &str, open_page: bool) -> Self {
        Link {
            name: String::from(name),
            url: String::from(url),
            go_page: open_page,
        }
    }
}

impl IntoMd for Link {
    fn into_md(&self) -> String {
        match self.go_page {
            true => format!("* [{}]({})", self.name, self.url),
            false => format!("* [{}](#{})", self.name, self.url),
        }
    }
}
