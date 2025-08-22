use sysinfo::{System};

//Alles noch komplett unfertig

pub fn info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let user = format!("{:?}", System::host_name().unwrap_or_else(|| "unbekannt".to_string()));
    let os = format!("{:?}", System::name().unwrap_or_else(|| "unbekannt".to_string()));
    let kv = format!("{:?}", System::kernel_version().unwrap_or_else(|| "unbekannt".to_string()));
    let os_v = format!("{:?}", System::os_version().unwrap_or_else(|| "unbekannt".to_string()));
    let boot_t = format!("{:?}", System::boot_time());
    let uptime = format!("{:?}", System::uptime());

    println!("{user}, {os}, {kv}, {os_v}, {boot_t}, {uptime}");

}