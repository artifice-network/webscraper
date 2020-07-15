use crate::traits::*;
pub struct DOM {
    raw_html: String,
    elements: Box<dyn HTMLTag>,
}

pub trait Document {
    fn get_element_by_id(&self, id: &str) -> Option<Box<dyn HTMLTag>>;
    fn get_elements_by_classname(&self, class: &str) -> Option<Vec<Box<dyn HTMLTag>>>;
}
