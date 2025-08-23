use sysinfo::{System};
use whoami::{self};

use crate::design::quad;

//Alles noch komplett unfertig

pub fn info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    //sysinfo
    let user = format!("{:?}", System::host_name().unwrap_or_else(|| "unbekannt".to_string()));
    let os = format!("{:?}", System::name().unwrap_or_else(|| "unbekannt".to_string()));
    let kv = format!("{:?}", System::kernel_version().unwrap_or_else(|| "unbekannt".to_string()));
    let os_v = format!("{:?}", System::os_version().unwrap_or_else(|| "unbekannt".to_string()));
    let uptime = format!("{:?}", updtime(System::uptime()));

    //sysinfo System usages

    //memory
    let total_memory = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_memory = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let free_memory = sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    //Hardware

    //CPU
    let cpu_brand = sys.cpus().get(0).unwrap().brand();
    let cpu = sys.cpus().get(0).unwrap();
    let cpu_usage = cpu.cpu_usage();

    //GPU

    //whoami
    let distro = format!("{:?}", whoami::distro());
    let real_name = format!("{:?}", whoami::realname());
    let user_name = format!("{:?}", whoami::username());
    let architektur = format!("{:?}", whoami::arch());
    quad::quad(&user, &os, &kv, &os_v, &uptime, &distro, &real_name, &user_name, &architektur, &cpu_brand, &cpu_usage, &total_memory, &used_memory, &free_memory);
}

fn updtime(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    return format!("{}h {}m {}s", hours, minutes, seconds);
}