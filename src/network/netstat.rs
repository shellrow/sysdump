use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ConnectionInfo {
    pub process_id: u32,
    pub protocol: String,
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
    pub status: String,
}

pub fn get_connection_map() -> HashMap<u32, ConnectionInfo> {
    let mut conn_map: HashMap<u32, ConnectionInfo> = HashMap::new();
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = match get_sockets_info(af_flags, proto_flags){
        Ok(si) => si,
        Err(_) => return conn_map,
    };
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                for pid in si.associated_pids {
                    let conn_info: ConnectionInfo = ConnectionInfo {
                        process_id: pid,
                        protocol: String::from("TCP"),
                        local_addr: tcp_si.local_addr.to_string(),
                        local_port: tcp_si.local_port,
                        remote_addr: tcp_si.remote_addr.to_string(),
                        remote_port: tcp_si.remote_port,
                        status: tcp_si.state.to_string(),
                    };
                    conn_map.insert(pid, conn_info);
                }
            },
            ProtocolSocketInfo::Udp(udp_si) => {
                for pid in si.associated_pids {
                    let conn_info: ConnectionInfo = ConnectionInfo {
                        process_id: pid,
                        protocol: String::from("UDP"),
                        local_addr: udp_si.local_addr.to_string(),
                        local_port: udp_si.local_port,
                        remote_addr: String::new(),
                        remote_port: 0,
                        status: String::new(),
                    };
                    conn_map.insert(pid, conn_info);
                }
            },
        }
    }
    return conn_map;
}

#[allow(dead_code)]
pub fn get_connection_list() -> Vec<ConnectionInfo> {
    let mut conn_list: Vec<ConnectionInfo> = vec![];
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = match get_sockets_info(af_flags, proto_flags){
        Ok(si) => si,
        Err(_) => return conn_list,
    };
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                for pid in si.associated_pids {
                    let conn_info: ConnectionInfo = ConnectionInfo {
                        process_id: pid,
                        protocol: String::from("TCP"),
                        local_addr: tcp_si.local_addr.to_string(),
                        local_port: tcp_si.local_port,
                        remote_addr: tcp_si.remote_addr.to_string(),
                        remote_port: tcp_si.remote_port,
                        status: tcp_si.state.to_string(),
                    };
                    conn_list.push(conn_info);
                }
            },
            ProtocolSocketInfo::Udp(udp_si) => {
                for pid in si.associated_pids {
                    let conn_info: ConnectionInfo = ConnectionInfo {
                        process_id: pid,
                        protocol: String::from("UDP"),
                        local_addr: udp_si.local_addr.to_string(),
                        local_port: udp_si.local_port,
                        remote_addr: String::new(),
                        remote_port: 0,
                        status: String::new(),
                    };
                    conn_list.push(conn_info);
                }
            },
        }
    }
    return conn_list;
}
