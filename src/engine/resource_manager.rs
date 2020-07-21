use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            path: details.path.clone(),
            size: details.size,
        }
    }
}
