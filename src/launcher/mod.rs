use fs_extra::dir::{copy as dir_copy, CopyOptions};
use jector::{inject_pid, InjectionMethod};
use std::fs::{create_dir_all, remove_dir_all, remove_file, write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::SystemExt;

use crate::options::{AC_EXECUTABLES, AuthType, DUMMY_AC, INJECTABLES};

mod process;

pub fn launch(login: Option<&str>, password: Option<&str>, auth_type: AuthType, path: String) {
    // if !crate::util::input("Are you sure you want to continue? (y/N): ")
    //     .to_lowercase()
    //     .starts_with("y")
    // {
    //     std::process::exit(0);
    // }

    let login = if let Some(auth) = login {
        auth
    } else {
        "unused"
    };

    let password = if let Some(data) = password {
        data
    } else {
        "unused"
    };

    let auth_type = match auth_type {
        AuthType::Exchange => "exchangecode",
        _ => "epic"
    };

    let binaries = [&path, "FortniteGame\\Binaries"].join("\\");

    let win64_dir = [&binaries, "Win64"].join("\\");
    let ruten_dir = [&binaries, "Ruten"].join("\\");

    let mut options = CopyOptions::new();
    options.content_only = true;
    options.copy_inside = true;
    options.overwrite = true;

    create_dir_all(&ruten_dir).unwrap();
    dir_copy(&win64_dir, &ruten_dir, &options).unwrap();

    let ac = DUMMY_AC.decrypt();

    for file in AC_EXECUTABLES.iter() {
        let path = [&ruten_dir, *file].join("\\");
        remove_file(&path).unwrap();
        write(&path, &ac).unwrap();
    }

    let launcher_path = [&ruten_dir, "FortniteLauncher.exe"].join("\\");

    let mut launcher = Command::new(launcher_path)
        .args(&[
            &["-AUTH_LOGIN=", login].join(""),
            &["-AUTH_PASSWORD=", password].join(""),
            &["-AUTH_TYPE=", auth_type].join(""),
            "-epicapp=Fortnite",
            "-epicenv=Prod",
            "-epicportal",
            "-skippatchcheck",
        ])
        .spawn()
        .map(|data| {
            log::info!("Successfully launched FortniteLauncher.exe");
            data
        })
        .unwrap();

    let mut system = sysinfo::System::new_all();

    let pid = process::find(&mut system, "FortniteClient-Win64-Shipping.exe");

    log::info!("FortniteClient-Win64-Shipping.exe PID is {}", &pid);

    sleep(Duration::from_secs(1));

    for (index, dll) in INJECTABLES.iter().enumerate() {
        let dll = dll.decrypt();
        log::info!("Injecting DLL(s) ({}/{})", index + 1, INJECTABLES.len());
        inject_pid(pid as u32, &dll, InjectionMethod::LoadLibrary).unwrap();
    }

    launcher.wait().unwrap();

    remove_dir_all(&ruten_dir).unwrap();
}
