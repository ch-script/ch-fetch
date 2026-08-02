// Main Program for the CLI simple fetch CH-FETCH v0
// it has almost nothing done yet, it just shows things

// Imports

// Sysinfo is the crate this app uses to get the system's info... Pretty self explanatory, here the docs: https://docs.rs/sysinfo/latest/sysinfo/
use sysinfo::{
    Components, Disks, Networks, System,
};

use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;
use std::cmp::max;

// dictionaries
use std::collections::HashMap;


// Main Program
fn main() {

    let sys_mng = SysinfoManager::new();
    let mut data = InformationManager::new(&sys_mng);
    let ascii_mng = AsciiArt::new();


    terminal_output(&data,&ascii_mng);
}

// Useful structures
struct Color(u8, u8, u8);

struct AsciiArt {
    //custom: Option<String>, //this is for the route of a personalized one
    //distro: String,
    fallback: &'static str,
}

struct UsrData {
    host: String,
    user: String
}

struct OSData {
    os: String,
    kernel: String,
    shell: String,
    uptime: String,
    terminal: String
}

struct MachineData {
    cpu: String,
    gpu: String,
    memory: String,
    disk: String
}



// Re-usable data structure

struct InformationManager {
    user_data : UsrData,
    os_data : OSData,
    machine_data : MachineData,
}

impl InformationManager {
    fn new(sysMng: &SysinfoManager) -> Self {
        InformationManager {
            user_data: UsrData {
                host: sysMng.get_hostname(),
                user: sysMng.get_user(),
            },
            os_data: OSData {
                os: sysMng.get_os(),
                kernel: sysMng.get_kernel(),
                shell: sysMng.get_shell(),
                uptime: sysMng.get_uptime(),
                terminal: sysMng.get_terminal(),
            },
            machine_data: MachineData {
                cpu: sysMng.get_cpu_inf(),
                gpu: sysMng.get_gpu(),
                memory: sysMng.get_ram(),
                disk: sysMng.get_disk(),
            }
        }
    }
}

impl AsciiArt {
    fn new() -> Self {
        AsciiArt {
            fallback: include_str!("../assets/ascii/fallback.txt"),
        }
    }
    fn get(&self) -> &str {
        self.fallback
    }
}


fn terminal_output(data: &InformationManager, ascii: &AsciiArt) {
    let ascii_lines: Vec<&str> = ascii.get().lines().collect();
    
    let disk_raw = data.machine_data.disk.trim();
    let disk_lines: Vec<&str> = disk_raw.lines().collect();

    let mut info_lines = vec![
        format!("User:     {}", data.user_data.user),
        format!("Host:     {}", data.user_data.host),
        "-----------------------------".to_string(),
        format!("Distro:   {}", data.os_data.os),
        format!("Kernel:   {}", data.os_data.kernel),
        format!("Uptime:   {}", data.os_data.uptime),
        format!("Terminal: {}", data.os_data.terminal),
        format!("Shell:    {}", data.os_data.shell),
        "".to_string(),
        format!("CPU:      {}", data.machine_data.cpu),
        format!("GPU:      {}", data.machine_data.gpu),
        format!("RAM:      {}", data.machine_data.memory),
    ];

    for d in disk_lines {
        info_lines.push(d.to_string());
    }

    let total_lines = max(ascii_lines.len(), info_lines.len());

    let ascii_width = ascii_lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    for i in 0..total_lines {
        let ascii_part = if i < ascii_lines.len() {
            let line = ascii_lines[i];
            let char_count = line.chars().count();
            let pad = " ".repeat(ascii_width.saturating_sub(char_count));
            format!("{}{}", line, pad)
        } else {
            " ".repeat(ascii_width)
        };

        let info_part = if i < info_lines.len() {
            &info_lines[i]
        } else {
            ""
        };

        println!("{}   {}", ascii_part, info_part);
    }
}


// Helpers

struct SysinfoManager {
    sys: System,
}

impl SysinfoManager {
    fn new() -> Self {
        SysinfoManager {
            sys: System::new_all(), // this initialices the sysinfo crate
        }
    }

    fn refresh(&mut self) {
        self.sys.refresh_all();
    }
    
    // GETTERS FROM SYSTEM THINGS
    fn get_kernel(&self) -> String {
        System::kernel_version().unwrap_or_else(|| String::from("Unknown"))
    }

    fn get_os(&self) -> String {
        System::name().unwrap_or_else(|| String::from("Unknown"))
    }

    fn get_uptime(&self) -> String {
        let uptime_time = System::uptime() / 60 / 60;
        format!("{}h", uptime_time)
    }

    fn get_hostname(&self) -> String {
        System::host_name().unwrap_or_else(|| String::from("Unknown"))
    }

    fn get_user(&self) -> String {
        env::var("USER").unwrap_or_else(|_| String::from("Unknown"))
    }

    // shell is an archive, so its only viable cleaned up
    fn get_shell(&self) -> String {
        let uncleaned_shell = env::var("SHELL").unwrap_or_else(|_| String::from("Unknown"));
        uncleaned_shell.split('/').last().unwrap_or("Unknown").to_string()
    }
    // tried with two, if one doesn't work maybe the other will... hopefully
    fn get_terminal(&self) -> String {
        env::var("TERM_PROGRAM").or_else(|_| env::var("TERM")).unwrap_or_else(|_| String::from("Unknown")).replace("xterm-","")
    }

    // MACHINE INFO

    fn get_cpu_inf(&self) -> String {
        let cpu_read = self.sys.cpus();

        if !cpu_read.is_empty() {
            cpu_read[0].brand().trim().to_string()
        } else {
            String::from("Unknown")
        }
    }

    fn get_ram(&self) -> String {
        let used_ram = self.sys.used_memory() / 1024 / 1024;
        let total_ram = self.sys.total_memory() / 1024 / 1024;

        format!("{} MB / {} MB", used_ram, total_ram)
    }
    
    fn get_disk(&self) -> String {
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


    fn get_gpu(&self) -> String {
        let commands = [
            "nvidia-smi --query-gpu=name --format=csv,noheader",
            "lspci | grep -i 'VGA' | grep -oP 'Advanced Micro Devices, Inc. \\K.*'",
            "lspci | grep -i 'VGA' | grep -oP 'Intel Corporation \\K.*'",
        ];

        for cmd in commands {
            if let Ok(output) = Command::new("sh").args(&["-c", cmd]).output() {
                let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !result.is_empty() {
                    return result;
                }
            }
        }

        String::from("Unknown GPU")
    }
}


// Custom Conf Override

