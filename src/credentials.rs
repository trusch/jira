use std::process::Command;

use crate::error;
use crate::types::Result;

pub fn load(pass: &String, key: &String) -> Result<(String, String)> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", pass, key))
        .output()?;
    if output.status.code().unwrap() != 0 {
        return Err(error::new_box("failed to get credentials"));
    }
    let str = String::from_utf8(output.stdout).unwrap();
    let parts: Vec<&str> = str.split("\n").collect();
    let pw = String::from(parts[0]);
    let name_line = String::from(parts[1]);
    let name_parts: Vec<&str> = name_line.split(" ").collect();
    let name = String::from(name_parts[1]);
    Ok((name, pw))
}
