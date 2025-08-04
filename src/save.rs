use std::{fs, io::Write, path::PathBuf};

use chrono::Utc;

use crate::constants::FESI_DIR_NAME;

pub fn save_response_to_file(response: String) -> Result<bool, anyhow::Error> {
    let time_of_output = Utc::now().timestamp().to_string();
    let file_name = format!("{time_of_output}_response.txt");
    //
    let mut file_path = PathBuf::from(FESI_DIR_NAME);
    file_path.push(file_name);
    if let Some(parent_dir) = file_path.parent() {
        fs::create_dir_all(parent_dir);
    }
    let mut file = fs::File::create(&file_path)?;

    file.write_all(response.as_bytes());

    println!("output genareated for file: {:?}", file_path);
    Ok(true)
}
