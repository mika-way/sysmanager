use sysinfo::{System, Disks};
use wgpu::{AdapterInfo, PowerPreference, RequestAdapterOptions};
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

    //CPU
    let cpu_brand = sys.cpus().get(0).unwrap().brand();
    let cpu_usage = chip_usage();

    //GPU
    let gpu_name = get_adapter_name(get_adapter());
    let gpu_backend = get_adapter_backend(get_adapter());

    //Disk
    /* 
    let disks = Disks::new_with_refreshed_list();
    let disk_count_ssd = disks.list()
        .iter()
        .filter(|d| matches!(d.kind(), DiskKind::SSD))
        .map(|d| d.name().to_string_lossy())
        .count() as i32;
    */

    /* 
    let disk_count_hhd = disks.list()
        .iter()
        .filter(|d| matches!(d.kind(), DiskKind::HDD))
        .map(|d| d.name().to_string_lossy())
        .count() as i32;
    */



    //whoami
    let distro = format!("{:?}", whoami::distro());
    let real_name = format!("{:?}", whoami::realname());
    let user_name = format!("{:?}", whoami::username());
    let architektur = format!("{:?}", whoami::arch());
    quad::quad(&user, &os, &kv, &os_v, &uptime, &distro, &real_name, &user_name, &architektur, &cpu_brand, &cpu_usage, &total_memory, &used_memory, &free_memory, &gpu_name, &gpu_backend);
}

//Unterfunktionen

fn updtime(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    return format!("{}h {}m {}s", hours, minutes, seconds);
}

fn get_adapter() -> AdapterInfo{
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

    let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .expect("Konnte keinen Adapter finden.");

    let info = adapter.get_info();

    return info;
}

fn get_adapter_name(info: AdapterInfo) -> String{
    let result = format!("{:?}", info.name);
    return result;
}

fn get_adapter_backend(info: AdapterInfo) -> String{
    let result = format!("{:?}", info.backend);
    return result;
}

fn chip_usage() -> String{
    let mut sys = System::new_all();
    sys.refresh_all();

    std::thread::sleep(std::time::Duration::from_millis(500)); 
    sys.refresh_cpu_all();

    // Gesamte CPU-Auslastung
    let total_cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    let result = format!("{:.2}", total_cpu_usage) as String;

    return result;
}

pub fn disk_vec() -> Vec<(String, String, u64, u64)> {
    let mut disks_vec: Vec<(String, String, u64, u64)> = Vec::new();
    let disk_list = Disks::new_with_refreshed_list();
    for disk in &disk_list {
        let mount_point = disk.mount_point();
        // Hier prüfen, welche Disks du hinzufügen möchtest
        if mount_point == std::path::Path::new("/") || mount_point.to_string_lossy().starts_with("/mnt/") {
            let total_space = disk.total_space();
            let free_space = disk.available_space();
            disks_vec.push((
                disk.kind().to_string(),
                mount_point.to_string_lossy().to_string(),
                total_space,
                free_space
            ));
        }
    }
    disks_vec
}