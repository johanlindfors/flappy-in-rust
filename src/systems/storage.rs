use std::fs;

use crate::FILE_NAME;

pub fn write(high_score: i32) -> std::io::Result<()> {
    fs::write(FILE_NAME, high_score.to_string())?;
    Ok(())
}

pub fn read() -> std::io::Result<i32> {
    match fs::read_to_string(FILE_NAME) {
        Ok(content) => Ok(content.parse().unwrap()),
        Err(_err) => Ok(0),
    }
}
