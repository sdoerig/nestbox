use lazy_static::lazy_static;
use regex::Regex;

pub trait Validator {
    fn is_valid(&self) -> bool;
}

pub fn is_uuid(uuid: &str) -> bool {
    //
    // Checks it a given string is a valid UUID
    // e.g. 9496be03-8e94-48c9-ad08-0e6fa8b37c20
    //
    lazy_static! {
        static ref UUID_PATTERN: Regex =
            Regex::new("^[0-9a-f]{8}-([0-9a-f]{4}-){3}[0-9a-f]{12}$").unwrap();
    }
    UUID_PATTERN.is_match(uuid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[actix_rt::test]
    async fn test_uuid_validator() {
        for _i in 0..200 {
            assert!(is_uuid(&Uuid::new_v4().to_string()));
        }
        assert!(!is_uuid("invalid"));
        assert!(!is_uuid("9496be03-8e94-48c9-ad08-0e6fa8b3720"))
    }
}
