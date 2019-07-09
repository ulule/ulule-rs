use serde::{Serialize, Deserialize};

// User is the user resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // Link to the profile page on the Ulule website
    pub absolute_url: String,
    // Unique id of the user
    pub id: u64,
    // Unique username of the user
    pub username: String,
}
