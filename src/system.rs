// Module that manages and returns system info, including actual user

// Use Modules

use crate::SysinfoManager;

// Data Structures

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

// Manager for such data

pub struct InformationManager {
    user_data : UsrData,
    os_data : OSData,
    machine_data : MachineData,
}

impl InformationManager {
    pub fn new(sys_mng: &SysinfoManager) -> Self {
        InformationManager {
            user_data: UsrData {
                host: sys_mng.get_hostname(),
                user: sys_mng.get_user(),
            },
            os_data: OSData {
                os: sys_mng.get_os(),
                kernel: sys_mng.get_kernel(),
                shell: sys_mng.get_shell(),
                uptime: sys_mng.get_uptime(),
                terminal: sys_mng.get_terminal(),
            },
            machine_data: MachineData {
                cpu: sys_mng.get_cpu_inf(),
                gpu: sys_mng.get_gpu(),
                memory: sys_mng.get_ram(),
                disk: sys_mng.get_disk(),
            }
        }
    }

    pub fn user(&self) -> &str {
        &self.user_data.user
    }

    pub fn host(&self) -> &str {
        &self.user_data.host
    }

    pub fn os(&self) -> &str {
        &self.os_data.os
    }

    pub fn kernel(&self) -> &str {
        &self.os_data.kernel
    }

    pub fn shell(&self) -> &str {
        &self.os_data.shell
    }

    pub fn uptime(&self) -> &str {
        &self.os_data.uptime
    }

    pub fn terminal(&self) -> &str {
        &self.os_data.terminal
    }

    pub fn cpu(&self) -> &str {
        &self.machine_data.cpu
    }

    pub fn gpu(&self) -> &str {
        &self.machine_data.gpu
    }

    pub fn ram(&self) -> &str {
        &self.machine_data.memory
    }

    pub fn memory(&self) -> &str {
        &self.machine_data.disk
    }

}
