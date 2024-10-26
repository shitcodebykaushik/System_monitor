use sysinfo::{System, SystemExt};

pub fn get_system_info() -> System {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys
}
