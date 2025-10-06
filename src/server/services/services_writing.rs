use uuid::Uuid;
use std::{env, fs};

use crate::server::{errors::writing_errors::writing_creation_error::WritingCreationError, repositories::repositories_writing::create_writing};

pub fn save_writing(author: Uuid, title: String, content: String, tags: Vec<Uuid>, cover: Vec<u8>, writing_type: Uuid, image_extension: String) -> Result<u64, WritingCreationError> {
    let image_filename = Uuid::new_v4().to_string();
    let mut image_dir = env::var("IMAGES_PATH_PRIVATE")?;
    let private_image_path = format!("{}/{}.{}", &image_dir, image_filename, image_extension);

    std::fs::write(&private_image_path, cover)?;
    image_dir = env::var("IMAGES_PATH_PUBLIC")?;

    let public_image_path = format!("{}/{}.{}", image_dir, image_filename, image_extension);

    let result = create_writing(author, title, content, tags, public_image_path, writing_type);
    let final_check: Result<u64, WritingCreationError> = match result {
        Ok(rows_affected) => Ok(rows_affected),
        Err(db_err) => {
            eprintln!("DB transaction failed, attempting file cleanup for: {}", private_image_path);
            
            if let Err(io_err) = fs::remove_file(&private_image_path) {
                 eprintln!("Warning: Failed to delete orphaned file {}: {}", private_image_path, io_err);
            }
            
            Err(WritingCreationError::Db(db_err))
        }
    };
    final_check
}