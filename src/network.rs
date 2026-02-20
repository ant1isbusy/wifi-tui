
use std::process::Command;

pub fn fetch_wifi_networks() -> Vec<String> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "SSID", "dev", "wifi", "list"])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let mut networks: Vec<String> = stdout
                .lines()
                .filter(|line| !line.is_empty()) // Remove empty lines
                .map(|line| line.to_string())
                .collect();

            networks.sort();
            networks.dedup();
            networks
        }
        Err(_) => vec!["Error: NetworkManager not found".to_string()],
    }
}
