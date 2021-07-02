#![allow(non_snake_case)]

use crate::options::{AuthType, AUTH_TYPE};
use crate::util::input;
use std::{fs::create_dir_all, thread::sleep, time::Duration};
mod backend;
pub mod epic;
mod launcher;
pub mod options;
pub mod util;

// the ammount of unwraps is too scary
// will have to be fixed later in case
fn main() {
    if let Err(_) = std::env::var("RUTEN_LOG") {
        std::env::set_var("RUTEN_LOG", "info");
    }
    pretty_env_logger::init_custom_env("RUTEN_LOG");

    for i in options::WELCOME.decrypt_str().unwrap().split("\n") {
        log::info!("{}", i.replace("{VER}", env!("CARGO_PKG_VERSION")));
    }

    create_dir_all(util::user_path()).unwrap();

    let thread = std::thread::spawn(|| {
        if options::BACKEND == true {
            backend::run().unwrap();
        }
    });

    if options::LAUNCHER == true {
        let config = util::config().unwrap();
        log::info!("Fortnite Directory: {}", config.path);

        // lazy ass fix to the backend logs overlaping
        sleep(Duration::from_secs(1));
        if AUTH_TYPE == AuthType::Exchange {
            let exchange = input("Exchange Code: ");
            launcher::launch(None, Some(&exchange), AuthType::Exchange, config.path);
        } else if AUTH_TYPE == AuthType::Username {
            let username = input("Username: ");
            launcher::launch(Some(&username), None, AuthType::Username, config.path);
        } else if AUTH_TYPE == AuthType::Password {
            let username = input("Username: ");
            let password = input("Password: ");
            launcher::launch(
                Some(&username),
                Some(&password),
                AuthType::Password,
                config.path,
            );
        }
    } else if options::BACKEND == true {
        thread.join().unwrap();
    } else {
        log::info!("No Launcher/Backend included...");
    }
}
