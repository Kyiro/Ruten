#![allow(non_snake_case)]

use std::{fs::create_dir_all, time::Duration, thread::sleep};
use crate::util::input;
mod backend;
mod launcher;
pub mod epic;
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

    let thread = std::thread::spawn(|| backend::run().unwrap());

    if options::LAUNCHER == true {
        let config = util::config().unwrap();
        log::info!("Fortnite Directory: {}", config.path);
        
        // lazy ass fix to the backend logs overlaping
        sleep(Duration::from_secs(1));
        let exchange = input("Exchange Code: ");
        launcher::launch(
            None,
            Some(&exchange),
            launcher::AuthType::Exchange,
            config.path
        );
    } else if options::BACKEND == true {
        thread.join().unwrap();
    } else {
        log::info!("No Launcher/Backend included...");
    }
}
