use actix_multipart::Multipart;
use actix_web::web;
use futures::{StreamExt, TryStreamExt};

use infer;
//use sha2::{Digest};
use std::{fs::File, io::Write};
use uuid::Uuid;

use std::io::{BufReader, Read};

use sha3::{Digest, Sha3_256};
pub struct ImageService {
    image_directory: String,
}

impl ImageService {
    pub fn new(image_directory: String) -> Self {
        ImageService { image_directory }
    }

    pub async fn save_file(&self, mut payload: Multipart) -> Option<Vec<String>> {
        // iterate over multipart stream

        let mut file_names: Vec<String> = Vec::new();

        while let Ok(Some(mut field)) = payload.try_next().await {
            let file_name_uuid = Uuid::new_v4().to_string();
            let filepath = format!("{}/{}", &self.image_directory, &file_name_uuid);
            let filepath_check_type = filepath.clone();
            // File::create is blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap()
                .unwrap();

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .unwrap();
            }
            let kind = infer::get_from_path(&filepath_check_type);
            let sha3_checksum = sha256_str(&filepath_check_type).await;
            if sha3_checksum.is_some() && kind.is_ok() {
                let file_name = format!(
                    "{}.{}",
                    sha3_checksum.unwrap(),
                    kind.unwrap().unwrap().extension()
                );
                let checksummed_path = format!("{}/{}", &self.image_directory, &file_name);
                let fm = std::fs::rename(&filepath_check_type, &checksummed_path);
                if fm.is_ok() {
                    file_names.push(file_name);
                }
            } else if std::fs::remove_file(&filepath_check_type).is_ok() {
            }
        }

        Some(file_names)
    }
}

async fn sha256_str(path: &str) -> Option<String> {
    if let Ok(inner) = File::open(path) {
        let mut reader = BufReader::new(inner);
        let mut hasher = Sha3_256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        Some(hex::encode(hasher.finalize()))
    } else {
        None
    }
}
