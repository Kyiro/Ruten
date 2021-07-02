use std::thread::sleep;
use std::time::Duration;
use sysinfo::{ProcessExt, SystemExt};

pub fn find(system: &mut sysinfo::System, name: &str) -> usize {
    loop {
        match system
            .get_processes()
            .iter()
            .find(|(_, i)| i.name() == name)
        {
            Some((pid, _)) => break *pid,
            None => (),
        };
        sleep(Duration::from_secs(1));
        system.refresh_all();
    }
}
