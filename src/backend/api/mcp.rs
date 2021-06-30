use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;
use crate::backend::api::structs::mcp::*;
use crate::util::user_path;

fn get_profile(id: &str) -> std::io::Result<RProfile> {
    let user = user_path();
    create_dir_all([&user, "profiles"].join("\\"))?;
    
    let path = [&user, "\\profiles\\", id, ".json"].join("");
    if !Path::new(&path).is_file() {
        let data = RProfile::new(id);
        write(&path, serde_json::to_string(&data)?)?;
        Ok(data)
    } else {
        Ok(serde_json::from_str(
            &read_to_string(&path)?
        )?)
    }
}

fn update_profile(profile: RProfile) -> std::io::Result<()> {
    let user = user_path();
    create_dir_all([&user, "profiles"].join("\\"))?;
    
    let path = [&user, "\\profiles\\", &profile.id, ".json"].join("");
    write(&path, serde_json::to_string(
        &profile
    )?)?;
    
    Ok(())
}