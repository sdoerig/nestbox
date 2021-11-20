use mongodb::bson::{doc, Document};

pub const NESTBOX_OF_OTHER_MANDANT: &str = "NESTBOX_OF_OTHER_MANDANT";
pub const NOT_FOUND: &str = "NOT_FOUND";
pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
pub const INTERNAL_SERVER_ERROR: &str = "INTERNAL_SERVER_ERROR";
pub const BAD_REQUEST: &str = "BAD_REQUEST";

pub fn create_error_message(msg: &str) -> Document {
    match msg {
        NESTBOX_OF_OTHER_MANDANT => doc! {"error": 1, "error_message": NESTBOX_OF_OTHER_MANDANT},
        NOT_FOUND => doc! {"error": 2, "error_message": NOT_FOUND},
        UNAUTHORIZED => doc! {"error": 2, "error_message": UNAUTHORIZED},
        INTERNAL_SERVER_ERROR => doc! {"error": 2, "error_message": INTERNAL_SERVER_ERROR},
        BAD_REQUEST => doc! {"error": 2, "error_message": BAD_REQUEST},
        _ => doc! {"error":255, "error_message": "UNKNOWN"},
    }
}
