use serde::{Serialize, Deserialize};

// Image is the image resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    // color
    pub color: Option<String>,
    // Unique id of the image
    pub id: u64,
    // Language of the image
    pub lang: String,
    // filename
    pub name: Option<String>,
    // Type of the image (main, background, secondary)
    #[serde(alias = "type")]
    pub kind: String,
    // Publicly available URL of the image
    pub url: Option<String>,
    // Path to the file.
    pub value: Option<String>,
    // Versions of the image
    pub versions: Option<Versions>
}

// Versions is the image versions resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Versions {
    pub full: Option<Version>,
    pub large: Option<Version>,
    pub medium: Option<Version>,
    pub small: Option<Version>,
}

// Version is the image version resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    // Image height in pixel, null in case it is the original version of the file
    pub height: Option<u64>,
    // Image URL
    pub url: String,
    // Image width in pixel, null in case it is the original version of the file
    pub width: Option<u64>,
}
