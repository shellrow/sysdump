use winreg::enums::*;
use winreg::RegKey;
use std::collections::HashMap;

pub fn get_network_adapters() -> HashMap<String, String> {
    let mut adapter_map: HashMap<String, String> = HashMap::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let clss = match hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class"){
        Ok(clss) => clss,
        Err(_) => return adapter_map,
    };
    let mut key_path: String = String::new();
    for key in clss.enum_keys().map(|x| x.unwrap()) {
        let target = format!("SYSTEM\\CurrentControlSet\\Control\\Class\\{}", key);
        let cls = match hklm.open_subkey(target.clone()){
            Ok(clss) => clss,
            Err(_) => return adapter_map,
        };
        if cls.get_value("Class").unwrap_or(String::new()) == String::from("Net") {
            key_path = target;
            break;
        }
    }
    if !key_path.is_empty() {
        let adapters = hklm.open_subkey(key_path.clone()).unwrap();
        for key in adapters.enum_keys().map(|x| x.unwrap()) {
            let target = format!("{}\\{}", key_path, key);
            match hklm.open_subkey(target.clone()) {
                Ok(adapter) => {
                    let guid: String = adapter.get_value("NetCfgInstanceId").unwrap_or(String::new());
                    let name: String = adapter.get_value("DriverDesc").unwrap_or(String::new());
                    adapter_map.insert(guid, name);
                },
                Err(_) => {},
            }
        }
    }
    return adapter_map;
}
