use crate::options::{UserDir, USER_DIR};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, write};
use std::io::{stdin, stdout, Error, ErrorKind, Write};
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub path: String,
}

pub fn config() -> std::io::Result<Config> {
    let user = user_path();

    let path = &[&user, "config.json"].join("\\");

    if !Path::new(path).is_file() {
        write(
            path,
            serde_json::to_string(&Config {
                path: match fn_path() {
                    Ok(data) => data,
                    Err(err) => {
                        log::error!(
                            "{}, please input your path manually...",
                            err.into_inner().unwrap()
                        );
                        input("Fortnite Path: ")
                    }
                },
            })
            .unwrap(),
        )?;
    }

    Ok(serde_json::from_str::<Config>(&read_to_string(path)?).unwrap())
}

pub fn user_path() -> String {
    if USER_DIR == UserDir::Root {
        String::from(".\\user")
    } else {
        match std::env::var("LOCALAPPDATA") {
            Ok(data) => {
                let path = [&data, "Ruten"].join("\\");
                create_dir_all(&path).unwrap();
                path
            }
            Err(_) => String::from(".\\user"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LauncherInstalled {
    #[serde(rename = "InstallationList")]
    installation_list: Vec<InstallationEntry>,
}

#[derive(Deserialize, Serialize)]
pub struct InstallationEntry {
    #[serde(rename = "InstallLocation")]
    install_location: String,
    #[serde(rename = "AppName")]
    app_name: String,
}

pub fn fn_path() -> std::io::Result<String> {
    let program_data = match std::env::var("ProgramData") {
        Ok(data) => data,
        Err(_) => return Err(Error::new(ErrorKind::NotFound, "ProgramData: Not Found")),
    };

    let path = &[
        &program_data,
        "Epic\\UnrealEngineLauncher\\LauncherInstalled.dat",
    ]
    .join("\\");

    if !Path::new(path).is_file() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "LauncherInstalled.dat: Not Found",
        ));
    }

    let installed: LauncherInstalled = match serde_json::from_str(&read_to_string(path)?) {
        Ok(data) => data,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Couldn't parse LauncherInstalled.dat",
            ))
        }
    };

    match installed
        .installation_list
        .iter()
        .find(|i| i.app_name == "Fortnite")
    {
        Some(data) => Ok(data.install_location.clone()),
        None => {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Couldn't find Fortnite in LauncherInstalled.dat",
            ))
        }
    }
}

pub fn open(url: &str) -> std::io::Result<std::process::Child> {
    std::process::Command::new("cmd.exe")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(url)
        .spawn()
}

pub fn input(text: &'static str) -> String {
    let mut data = String::new();
    print!("{}", text);
    stdout().flush().unwrap();
    stdin().read_line(&mut data).unwrap();
    data
}
