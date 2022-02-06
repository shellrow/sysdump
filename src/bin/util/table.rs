use term_table::{Table, TableStyle};
use term_table::table_cell::{TableCell,Alignment};
use term_table::row::Row;

use sysdump::os::shared::{self, SystemOverview};
use sysdump::network::netstat;

pub fn show_system_overview() {
    let sys_overview: SystemOverview = shared::get_system_overview();
    let mut table = Table::new();
    table.max_column_width = 60;
    table.separate_rows = false;
    table.style = TableStyle::blank();
    // CPU
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("CPU", 3, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Name", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.cpu.brand, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Frequency", 1, Alignment::Left),
        TableCell::new_with_alignment(format!("{} MHz", sys_overview.cpu.frequency), 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Physical Core", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.cpu.physical_core_count, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Logical Processor", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.cpu.logical_processor_count, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Current Usage", 1, Alignment::Left),
        TableCell::new_with_alignment(format!("{}%",sys_overview.cpu.cpu_usage), 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 3, Alignment::Left)
    ]));
    // Memory
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Memory", 3, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Total", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.memory.total_memory, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Available", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.memory.available_memory, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Free", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.memory.free_memory, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Used", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.memory.used_memory, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Total Swap", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.memory.total_swap, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Free Swap", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.memory.free_swap, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Used Swap", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.memory.used_swap, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 3, Alignment::Left)
    ]));
    // Disk
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Disk", 3, Alignment::Left)
    ]));
    for disk in sys_overview.disks {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Name", 1, Alignment::Left),
            TableCell::new_with_alignment(disk.name, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("File System", 1, Alignment::Left),
            TableCell::new_with_alignment(disk.file_system, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Mount Point", 1, Alignment::Left),
            TableCell::new_with_alignment(disk.mount_point, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Total Space", 1, Alignment::Left),
            TableCell::new_with_alignment(disk.total_space, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Available Space", 1, Alignment::Left),
            TableCell::new_with_alignment(disk.available_space, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Removable", 1, Alignment::Left),
            if disk.is_removable {
                TableCell::new_with_alignment("Yes", 1, Alignment::Left)
            }else{
                TableCell::new_with_alignment("No", 1, Alignment::Left)
            }
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 3, Alignment::Left)
        ]));
    }
    // Network
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Network", 3, Alignment::Left)
    ]));
    for interface in sys_overview.network_interfaces {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Index", 1, Alignment::Left),
            TableCell::new_with_alignment(interface.index, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Name", 1, Alignment::Left),
            TableCell::new_with_alignment(interface.name, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Mac Address", 1, Alignment::Left),
            TableCell::new_with_alignment(format!("{:?}",interface.mac_addr), 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("IPv4 Address", 1, Alignment::Left),
            TableCell::new_with_alignment(format!("{:?}",interface.ipv4), 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("IPv6 Address", 1, Alignment::Left),
            TableCell::new_with_alignment(format!("{:?}",interface.ipv6), 1, Alignment::Left)
        ]));
        if let Some(gateway) = interface.gateway {
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("", 1, Alignment::Left),
                TableCell::new_with_alignment("Gateway Mac", 1, Alignment::Left),
                TableCell::new_with_alignment(gateway.mac_addr, 1, Alignment::Left)
            ]));
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("", 1, Alignment::Left),
                TableCell::new_with_alignment("Gateway IP", 1, Alignment::Left),
                TableCell::new_with_alignment(gateway.ip_addr, 1, Alignment::Left)
            ]));
        }else {
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("", 1, Alignment::Left),
                TableCell::new_with_alignment("Gateway Mac", 1, Alignment::Left),
                TableCell::new_with_alignment("(Not found)", 1, Alignment::Left)
            ]));
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment("", 1, Alignment::Left),
                TableCell::new_with_alignment("Gateway IP", 1, Alignment::Left),
                TableCell::new_with_alignment("(Not found)", 1, Alignment::Left)
            ]));
        }
        
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 3, Alignment::Left)
        ]));
    }
    // OS
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("OS", 3, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Name", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.os.name, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Kernel Version", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.os.kernel_version, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Version", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.os.os_version, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Version Detail", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.os.long_os_version, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Host Name", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.os.host_name, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Up Time", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.os.uptime, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 1, Alignment::Left),
        TableCell::new_with_alignment("Boot Time", 1, Alignment::Left),
        TableCell::new_with_alignment(sys_overview.os.boot_time, 1, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 3, Alignment::Left)
    ]));
    // User
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("User", 3, Alignment::Left)
    ]));
    for user in sys_overview.users {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Id", 1, Alignment::Left),
            TableCell::new_with_alignment(user.user_id, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Name", 1, Alignment::Left),
            TableCell::new_with_alignment(user.user_name, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Group Id", 1, Alignment::Left),
            TableCell::new_with_alignment(user.group_id, 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 1, Alignment::Left),
            TableCell::new_with_alignment("Groups", 1, Alignment::Left),
            TableCell::new_with_alignment(format!("{:?}",user.groups), 1, Alignment::Left)
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("", 3, Alignment::Left)
        ]));
    }
    // render table
    println!("{}", table.render());
}

pub fn show_connection_simple() {
    let mut table = Table::new();
    table.max_column_width = 60;
    table.separate_rows = false;
    table.style = TableStyle::blank();
    let conn_map = netstat::get_connection_map();
    let process_map = shared::get_process_map();
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Connection", 8, Alignment::Left)
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("PID", 1, Alignment::Left),
        TableCell::new_with_alignment("Name", 1, Alignment::Left),
        TableCell::new_with_alignment("Protocol", 1, Alignment::Left),
        TableCell::new_with_alignment("Local Addr", 1, Alignment::Left),
        TableCell::new_with_alignment("Local Port", 1, Alignment::Left),
        TableCell::new_with_alignment("Remote Addr", 1, Alignment::Left),
        TableCell::new_with_alignment("Remote Port", 1, Alignment::Left),
        TableCell::new_with_alignment("Status", 1, Alignment::Left),
    ]));
    for (pid, conn) in conn_map {
        if let Some(process) = process_map.get(&pid) {
            //println!("[{}] [{}] {:?}", pid, process.name, conn); 
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(pid, 1, Alignment::Left),
                TableCell::new_with_alignment(process.name.to_string(), 1, Alignment::Left),
                TableCell::new_with_alignment(conn.protocol, 1, Alignment::Left),
                TableCell::new_with_alignment(conn.local_addr, 1, Alignment::Left),
                TableCell::new_with_alignment(conn.local_port, 1, Alignment::Left),
                TableCell::new_with_alignment(conn.remote_addr, 1, Alignment::Left),
                TableCell::new_with_alignment(conn.remote_port, 1, Alignment::Left),
                TableCell::new_with_alignment(conn.status, 1, Alignment::Left),
            ]));
        }
    }
    // render table
    println!("{}", table.render());
}
