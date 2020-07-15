use crate::css::Color;
use std::collections::HashMap;
pub trait HTMLTag {
    fn id(&self) -> Option<String>;
    fn name(&self) -> Option<String>;
    fn classes(&self) -> Option<Vec<String>>;
    fn tag_name(&self) -> String;
    fn supported_attributes(&self) -> Vec<String>;
    fn get_styler(&self) -> Box<dyn Styler<Box<dyn HTMLTag>>>;
    fn attribute_ast(&self) -> HashMap<String, String>;
}
pub trait Styler<T: HTMLTag> {
    fn width(&mut self, element: &T, width: u32);
    fn height(&mut self, element: &T, height: u32);
    fn color(&mut self, color: Color);
}
