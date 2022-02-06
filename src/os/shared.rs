use sysinfo::{System, SystemExt, ProcessorExt, DiskExt, UserExt, PidExt, ProcessExt, DiskUsage, RefreshKind};
//use sysinfo::{NetworkExt, NetworksExt};
use default_net::Interface as NetworkInterface;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CpuInfo {
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
    pub cpu_usage: f32,
    pub physical_core_count: usize,
    pub logical_processor_count: usize,
}

#[derive(Clone, Debug)]
pub struct ProcessorStat {
    pub name: String,
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
    pub cpu_usage: f32,
}

#[derive(Clone, Debug)]
pub struct MemoryInfo {
    pub total_memory: u64,
    pub free_memory: u64,
    pub available_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub free_swap: u64,
    pub used_swap: u64,
}

#[derive(Clone, Debug)]
pub struct Diskinfo {
    pub name: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}

#[derive(Clone, Debug)]
pub struct NetworkUsageInfo {
    pub interface_name: String,
    pub received: u64,
    pub total_received: u64,
    pub transmitted: u64,
    pub total_transmitted: u64,
    pub packets_received: u64,
    pub total_packets_received: u64, 
    pub packets_transmitted: u64, 
    pub total_packets_transmitted: u64,
    pub errors_on_received: u64,
    pub total_errors_on_received: u64,
    pub errors_on_transmitted: u64,
    pub total_errors_on_transmitted: u64,
}

#[derive(Clone, Debug)]
pub struct NetworkInterfaceInfo {
    pub index: u32,
    pub name: String,
    pub mac: String,
    pub ipv4_addr: Vec<String>,
    pub ipv6_addr: Vec<String>,
    pub gateway_ip: String,
    pub gateway_mac: String,
}

#[derive(Clone, Debug)]
pub struct LoadAverage {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

#[derive(Clone, Debug)]
pub struct OsInfo {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub long_os_version: String,
    pub host_name: String,
    pub uptime: u64,
    pub boot_time: u64,
    pub load_average: LoadAverage,
}

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub user_id: String,
    pub user_name: String,
    pub group_id: String,
    pub groups: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SystemOverview {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disks: Vec<Diskinfo>,
    pub network_interfaces: Vec<NetworkInterface>,
    pub os: OsInfo,
    pub users: Vec<UserInfo>,
}

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cmd: String,
    pub full_path: String,
    pub environ: String,
    pub cw_dir: String,
    pub root_dir: String,
    pub memory: u64,
    pub virtual_memory: u64,
    pub parent: u32,
    pub status: String,
    pub start_time: u64,
    pub cpu_usage: f32,
    pub disk_usage: DiskUsage,
}

pub fn get_system_overview() -> SystemOverview {
    let sys_cpu = System::new_with_specifics(RefreshKind::new().with_cpu());
    // CPU
    let global_cpu = sys_cpu.global_processor_info();
    let processors = sys_cpu.processors();
    let cpu_info = CpuInfo {
        vendor_id: global_cpu.vendor_id().to_string(),
        brand: global_cpu.brand().to_string(),
        frequency: {
            if processors.len() > 0 {
                processors[0].frequency()
            }else{
                global_cpu.frequency()      
            }
        },
        cpu_usage: global_cpu.cpu_usage(),
        physical_core_count: {
            match sys_cpu.physical_core_count() {
                Some(cnt) => cnt,
                None => processors.len() / 2,
            }
        },
        logical_processor_count: processors.len(),
    };

    let sys_ram = System::new_with_specifics(RefreshKind::new().with_memory());
    // Memory
    let mem_info = MemoryInfo {
        total_memory: sys_ram.total_memory(),
        free_memory: sys_ram.free_memory(),
        available_memory: sys_ram.available_memory(),
        used_memory: sys_ram.used_memory(),
        total_swap: sys_ram.total_swap(),
        free_swap: sys_ram.free_swap(),
        used_swap: sys_ram.used_swap(),
    };

    let mut sys = System::new_all();
    sys.refresh_all();
    // Disk
    let mut disks: Vec<Diskinfo> = vec![];
    for disk in sys.disks() {
        let disk_info = Diskinfo {
            name: disk.name().to_str().unwrap_or("").to_string(),
            file_system: String::from_utf8(disk.file_system().to_vec()).unwrap_or(String::new()),
            mount_point: disk.mount_point().to_str().unwrap_or("").to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
        };
        disks.push(disk_info);
    }

    // Network
    let interfaces: Vec<NetworkInterface> = default_net::get_interfaces();

    // OS
    let load_avg = sys.load_average();
    let load_average = LoadAverage {
        one: load_avg.one,
        five: load_avg.five,
        fifteen: load_avg.fifteen,
    };
    let os_info = OsInfo {
        name: sys.name().unwrap_or(String::new()),
        kernel_version: sys.kernel_version().unwrap_or(String::new()),
        os_version: sys.os_version().unwrap_or(String::new()),
        long_os_version: sys.long_os_version().unwrap_or(String::new()),
        host_name: sys.host_name().unwrap_or(String::new()),
        uptime: sys.uptime(),
        boot_time: sys.boot_time(),
        load_average: load_average,
    };

    // User
    let mut user_list: Vec<UserInfo> = vec![];
    for user in sys.users() {
        let user_info = UserInfo {
            user_id: user.uid().to_string(),
            user_name: user.name().to_string(),
            group_id: user.gid().to_string(),
            groups: user.groups().to_vec(),
        };
        user_list.push(user_info);
    }

    // Overview
    let system_overview = SystemOverview {
        cpu: cpu_info,
        memory: mem_info,
        disks: disks,
        network_interfaces: interfaces,
        os: os_info,
        users: user_list,
    };
    return system_overview;
}

pub fn get_process_map() -> HashMap<u32, ProcessInfo> {
    let mut process_map: HashMap<u32, ProcessInfo> = HashMap::new();
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
        let mut cmd: String = String::new();
        for ele in process.cmd().to_vec() {
            cmd = format!("{} {}",cmd,ele);
        }
        let mut env: String = String::new();
        for ele in process.environ() {
            env = format!("{} {}",env,ele);
        }
        let process_info = ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            cmd: cmd,
            full_path: process.exe().to_str().unwrap_or("").to_string(),
            environ: env,
            cw_dir: process.cwd().to_str().unwrap_or("").to_string(),
            root_dir: process.root().to_str().unwrap_or("").to_string(),
            memory: process.memory(),
            virtual_memory: process.virtual_memory(),
            parent: match process.parent() {
                Some(p) => p.as_u32(),
                None => 0,
            },
            status: process.status().to_string(),
            start_time: process.start_time(),
            cpu_usage: process.cpu_usage(),
            disk_usage: process.disk_usage(),
        };
        process_map.insert(pid.as_u32(), process_info);
    }
    return process_map;
}
