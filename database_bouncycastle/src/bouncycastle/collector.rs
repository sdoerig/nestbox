use mongodb::{bson::Bson, sync::Collection};

use std::collections::{hash_map::RandomState, HashMap};
const STEP_SIZE: usize = 5000;
type VecDocType = Vec<mongodb::bson::Document>;

pub enum CollectorState {
    Flushed,
    Accumulating,
}
pub struct Collector {
    docs: VecDocType,
    pub uuids: Vec<String>,
    collection: Collection,
    pub result: HashMap<usize, Bson, RandomState>,
}

impl Collector {
    // Collects the generated records and if the STEPSIZE is reached
    // writes it to the mongodb collection.
    pub fn new(collection_store: Collection) -> Self {
        Collector {
            docs: Vec::new(),
            uuids: Vec::new(),
            collection: collection_store,
            result: HashMap::new(),
        }
    }

    pub fn append_doc(&mut self, doc: mongodb::bson::Document) -> CollectorState {
        if self.docs.is_empty() {
            // result is empty - so I'm in a new round -> flush all the uuids
            self.uuids.clear();
        }
        let opt_uuid = doc.get("uuid");
        let uuid = match opt_uuid {
            Some(u) => u.to_string(),
            None => String::from("")
        };

        if !uuid.is_empty() {
            self.uuids.push(uuid.replace('"', ""));
        }

        self.docs.push(doc);
        if self.docs.len() > STEP_SIZE {
            self.write_to_db();
            return CollectorState::Flushed;
        }
        CollectorState::Accumulating
    }

    fn write_to_db(&mut self) {
        self.result = match self.collection.insert_many(self.docs.drain(..), None) {
            Ok(s) => s.inserted_ids,
            _ => HashMap::new(),
        };
    }

    pub fn flush(&mut self) {
        if !self.docs.is_empty() {
            self.write_to_db();
        }
    }
}
