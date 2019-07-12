use serde::{Serialize, Deserialize};
use crate::i18n::I18n;

// Tag is the tag and category resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    // Link to the category page on the Ulule website
    pub absolute_url: String,
    // Parent category, only present in the list-tags endpoint
    pub category: Option<Box<Tag>>,
    // Unique ID of the tag
    pub id: u64,
    // Name of the tag
    pub name: I18n,
    // Position
    pub position: u64,
    // Slug
    pub slug: String,
}
