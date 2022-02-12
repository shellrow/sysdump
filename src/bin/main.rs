mod util;
use util::table;

fn main() {
    match sysinfo::get_current_pid() {
        Ok(pid) => {
            println!("current pid: {}", pid);
        }
        Err(e) => {
            eprintln!("failed to get current pid: {}", e);
        }
    }
    table::show_system_overview();
    table::show_process_simple();
    table::show_connection_simple();
    table::show_ip_info();
}
