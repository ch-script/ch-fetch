// This module is a helper for system.rs, it uses the sysinfo library

// IMPORTS

// Sysinfo is the crate this app uses to get the system's info... Pretty self explanatory, here the docs: https://docs.rs/sysinfo/latest/sysinfo/
use sysinfo::{
    Disks, System, RefreshKind, CpuRefreshKind, MemoryRefreshKind,
};

use std::env;
use std::path::Path;
use std::process::Command;

// 


pub struct SysinfoManager {
    sys: System,
}

impl SysinfoManager {
    pub fn new() -> Self {

        let refresh = RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()).with_memory(MemoryRefreshKind::everything()); // just what we need for now... (optimization)

        SysinfoManager {
            sys: System::new_with_specifics(refresh),
        }
    }

    // pub fn refresh(&mut self) {
    //     self.sys.refresh_all();
    // }
    
    // GETTERS FROM SYSTEM THINGS
    pub fn get_kernel(&self) -> String {
        System::kernel_version().unwrap_or_else(|| String::from("Unknown"))
    }

    pub fn get_os(&self) -> String {
        System::name().unwrap_or_else(|| String::from("Unknown"))
    }

    pub fn get_uptime(&self) -> String {
        let uptime_time = System::uptime() / 60 / 60;
        format!("{}h", uptime_time)
    }

    pub fn get_hostname(&self) -> String {
        System::host_name().unwrap_or_else(|| String::from("Unknown"))
    }

    pub fn get_user(&self) -> String {
        env::var("USER").unwrap_or_else(|_| String::from("Unknown"))
    }

    // shell is an archive, so its only viable cleaned up
    pub fn get_shell(&self) -> String {
        let uncleaned_shell = env::var("SHELL").unwrap_or_else(|_| String::from("Unknown"));
        uncleaned_shell.split('/').last().unwrap_or("Unknown").to_string()
    }
    // tried with two, if one doesn't work maybe the other will... hopefully
    pub fn get_terminal(&self) -> String {
        env::var("TERM_PROGRAM").or_else(|_| env::var("TERM")).unwrap_or_else(|_| String::from("Unknown")).replace("xterm-","")
    }

    // MACHINE INFO

    pub fn get_cpu_inf(&self) -> String {
        let cpu_read = self.sys.cpus();

        if !cpu_read.is_empty() {
            cpu_read[0].brand().trim().to_string()
        } else {
            String::from("Unknown")
        }
    }

    pub fn get_ram(&self) -> String {
        let used_ram = self.sys.used_memory() / 1024 / 1024;
        let total_ram = self.sys.total_memory() / 1024 / 1024;

        format!("{} MB / {} MB", used_ram, total_ram)
    }
    
    pub fn get_disk(&self) -> String {
        let disks = Disks::new_with_refreshed_list();
        let mut disk_info_return = String::new();

        

        for disk in disks.list() {
            // conditional that analize wheter it is root or external devices
            let mount_path = disk.mount_point();
            let is_root = mount_path == Path::new("/");
            let is_external = mount_path.starts_with("/run/media/") || mount_path.starts_with("/media/");

            if is_root || is_external {

                let tag = if is_external {"[external]"} else {""};

                let total_mem = disk.total_space() / 1024/1024/1024;
                let available_mem = disk.available_space() / 1024/1024/1024;
                let used_mem = total_mem - available_mem; //xd
                let mount_point = mount_path.display();

                // string for each partition
                let temp_disk_data = format!("Disk({}): {} GB ({} GB / {} GB) {}\n", mount_point, available_mem, total_mem, used_mem, tag);

                disk_info_return.push_str(&temp_disk_data);
            }
            
        }

        if disk_info_return.is_empty() {
            String::from("That's quite strange... we couldn't find a disk, so how is this thing up?")
        } else {
            disk_info_return
        }
    }


    pub fn get_gpu(&self) -> String {
        if let Ok(out) = Command::new("nvidia-smi").args(["--query-gpu=name", "--format=csv,noheader"]).output()
        {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() { return s; }
        }

        if let Ok(out) = Command::new("lspci").output() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.contains("VGA") {
                    if let Some(idx) = line.find("Advanced Micro Devices, Inc.") {
                        return line[idx..].to_string();
                    }
                    if let Some(idx) = line.find("Intel Corporation") {
                        return line[idx..].to_string();
                    }
                }
            }
        }

        String::from("Unknown GPU")
    }
}