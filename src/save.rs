use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process,
};

use chrono::Utc;

use crate::{constants::FESI_DIR_NAME, request::Request};

pub fn save_response_to_file(response: String, action_name: String) -> Result<bool, anyhow::Error> {
    let time_of_output: String = Utc::now().timestamp().to_string();
    let file_name: String = format!("{time_of_output}_{action_name}.txt");
    //
    let mut file_path: PathBuf = PathBuf::from(format!("{FESI_DIR_NAME}/response"));
    file_path.push(file_name);
    if let Some(parent_dir) = file_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    let mut file: File = fs::File::create(&file_path)?;

    file.write_all(response.as_bytes())?;

    Ok(true)
}

pub fn _save(request: Request, res: String) {
    if request.save_response {
        save_response_to_file(res, request.name.clone().unwrap_or("response".to_string()))
            .unwrap_or_else(|error| {
                eprintln!("Could not save response: {error}");
                process::exit(1);
            });
    };
}
