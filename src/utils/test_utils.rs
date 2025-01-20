use std::{fs::{self, File}, io::Write, path::Path};
use uuid::Uuid; // Add the uuid crate for unique IDs

// Function to create a temporary file with unique name
pub fn create_temp_file(content: &str) -> String {
    // Generate a unique file name using UUID
    let file_name = format!("src/test/files/cheetah_{}.config.json", Uuid::new_v4());

    let path = Path::new(&file_name);
    let mut file = File::create(path).unwrap_or_else(|_e| {
        // Perform cleanup if the file creation fails
        remove_config_from_path(&file_name);
        panic!("Could not create file");
    });
    file.write_all(content.as_bytes()).unwrap_or_else(|_e| {
        // Perform cleanup if writing to the file fails
        remove_config_from_path(&file_name);
        panic!("Could not write to file");
    });
    file_name
}

// Function to remove a config file
pub fn remove_config_from_path(file_path: &str) {
    if Path::new(file_path).exists() {
        fs::remove_file(file_path).expect(&("Unable to remove file".to_owned() + file_path));
    }
}
