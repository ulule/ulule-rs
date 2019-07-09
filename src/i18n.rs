// Some payloads and resources contain fields of type i18n object.
// An i18n object is a JSON object that contains i18n translations.
// Object keys are language codes (see supported languages),
// and object values are translation strings.

// For example, the description field of the user resource field may look like this:
// {
//  "en": "This is the description in english",
//  "fr": "Ceci est la description en français"
// }

use std::collections::HashMap;

pub type I18n = HashMap<String, String>;
