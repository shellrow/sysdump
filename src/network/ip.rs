use reqwest::Url;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

const IP_INFO_URL: &str = "https://api.ipstruct.io/ip";

#[derive(Serialize, Deserialize, Debug)]
pub struct IpInfo {
    pub ip_version: String,
    pub ip_addr: String,
    pub host_name: String,
    pub ip_from_dec: String,
    pub ip_to_dec: String,
    pub ip_from: String,
    pub ip_to: String,
    pub cidr: String,
    pub asn: String,
    pub as_name: String,
    pub country_code: String,
    pub country_name: String,
    pub registry: String,
    pub status: String,
}

pub fn get_external_ip_info() -> Result<IpInfo, String> {
    let url = match Url::parse(IP_INFO_URL){
        Ok(url) => url,
        Err(e) => return Err(e.to_string()),
    };
    let client = match reqwest::blocking::Client::builder().build() {
        Ok(client) => client,
        Err(e) => return Err(e.to_string()),
    };
    let res: Response = match client.get(url).send() {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };
    if res.status().is_success() {
        let res_text = match res.text() {
            Ok(res_text) => res_text,
            Err(e) => return Err(e.to_string()),
        };
        let ip_json: IpInfo = match serde_json::from_str(res_text.as_str()) {
            Ok(ip_json) => ip_json,
            Err(e) => return Err(e.to_string()),
        };
        Ok(ip_json)
    }else{
        Err(res.status().to_string())
    }
}
