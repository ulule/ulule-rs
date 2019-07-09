use serde::{Serialize, Deserialize};
use crate::i18n::I18n;

// User is the user resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // Link to the profile page on the Ulule website
    pub absolute_url: String,
    // Two-letter ISO code of the country where the user lives.
    pub country: String,
    // Date at which the user created an account, with RFC 3339 format
    pub date_joined: String,
    // Short description of the user
    pub description: I18n,
    // First name of the user
    pub first_name: String,
    // True if user has an avatar. Otherwise a default avatar is assigned to the user
    pub has_avatar: bool,
    // Unique id of the user
    pub id: u64,
    // True if user is staff
    pub is_staff: bool,
    // Language of the user
    pub lang: String,
    // Date at which the user last logged in, with RFC 3339 format
    pub last_login: String,
    // Last name of the user
    pub last_name: String,
    // Location of the user
    pub location: String,
    // Concatenation of first name and last name if they exist, username otherwise
    pub name: String,
    // Longer presentation of the user
    pub presentation: I18n,
    // URL of the user resource
    pub resource_uri: String,
    // Unique screenname of the user
    pub screenname: String,
    // Timezone of the user
    pub timezone: String,
    // Unique username of the user
    pub username: String,
}
