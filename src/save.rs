use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use chrono::Utc;

use crate::constants::FESI_DIR_NAME;

pub fn save_response_to_file(response: String, action_name: String) -> Result<bool, anyhow::Error> {
    let time_of_output: String = Utc::now().timestamp().to_string();
    let file_name: String = format!("{time_of_output}_{action_name}.txt");
    //
    let mut file_path: PathBuf = PathBuf::from(FESI_DIR_NAME);
    file_path.push(file_name);
    if let Some(parent_dir) = file_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    let mut file: File = fs::File::create(&file_path)?;

    file.write_all(response.as_bytes())?;

    Ok(true)
}
