use bson::{doc, Document};

pub const NESTBOX_OF_OTHER_MANDANT: &str = "NESTBOX_OF_OTHER_MANDANT";
pub const NOT_FOUND: &str = "NOT_FOUND";

pub fn create_error_message(msg: &str) -> Document {
    match msg {
        NESTBOX_OF_OTHER_MANDANT => doc! {"error": 1, "error_message": NESTBOX_OF_OTHER_MANDANT},
        NOT_FOUND => doc! {"error": 1, "error_message": NOT_FOUND},
        _ => doc! {"error":255, "error_message": "UNKNOWN"},
    }
}
