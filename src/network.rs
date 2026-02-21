use std::collections::HashSet;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: String,
    pub security: String,
    pub is_saved: bool,
}

pub fn connect_to_net(net: WifiNetwork, pass: String) {
    let output =
        Command::new("nmcli").args(["device", "wifi", "connect", &net.ssid, "--password", &pass]);
}

pub fn get_saved_ssids() -> HashSet<String> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "NAME", "connection", "show"])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.to_string())
                .collect()
        }
        Err(_) => HashSet::new(),
    }
}

pub fn fetch_wifi_networks() -> Vec<WifiNetwork> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,SECURITY", "dev", "wifi", "list"])
        .output();

    match output {
        Ok(out) => {
            let saved_ssids = get_saved_ssids();
            let stdout = String::from_utf8_lossy(&out.stdout);
            let networks: Vec<WifiNetwork> = stdout
                .lines()
                .filter(|line| !line.is_empty())
                .filter_map(|line| {
                    let parsed: Vec<&str> = line.split(":").collect();
                    if parsed.len() >= 3 && !parsed[0].is_empty() {
                        Some(WifiNetwork {
                            ssid: parsed[0].to_string(),
                            signal: parsed[1].to_string(),
                            security: parsed[2].to_string(),
                            is_saved: saved_ssids.contains(parsed[0]),
                        })
                    } else {
                        None
                    }
                })
                .collect();

            networks
        }
        Err(_) => Vec::new(),
    }
}
