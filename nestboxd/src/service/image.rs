use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{ web};
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

pub struct ImageService {
    image_directory: String,
}


impl ImageService {
    pub fn new(image_directory: String) -> Self {
        ImageService { image_directory }
    }

    pub async fn save_file(&self, mut payload: Multipart) -> Option<String> {
        // iterate over multipart stream
        let file_name = Uuid::new_v4().to_string();
        while let Ok(Some(mut field)) = payload.try_next().await {
            let content_type = field.content_disposition().unwrap();
            let filename = content_type.get_filename().unwrap();
            let filepath = format!("{}/{}.{}", &self.image_directory, 
            &file_name,
            "");

            // File::create is blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap();
            }
        }

        Some(file_name)
    }
}