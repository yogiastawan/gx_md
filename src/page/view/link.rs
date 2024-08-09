use crate::utils::IntoMd;

#[derive(Clone)]
pub(crate) struct Link {
    name: String,
    url: String,
}

impl Link {
    pub(crate) fn new(name: &str, url: &str) -> Self {
        Link {
            name: String::from(name),
            url: String::from(url),
        }
    }
}

impl IntoMd for Link {
    fn into_md(&self) -> String {
        format!("[{}]({})", self.name, self.url)
    }
}
