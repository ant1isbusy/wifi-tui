use std::collections::HashSet;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: String,
    pub security: String,
    pub is_saved: bool,
    pub is_connected: bool,
}

pub fn connect_to_net(ssid: &str, pass: Option<String>) -> std::io::Result<()> {
    let mut cmd = Command::new("nmcli");

    cmd.args(["device", "wifi", "connect", ssid]);

    if let Some(password) = pass {
        cmd.arg("password");
        cmd.arg(password);
    }

    let output = cmd.output()?;

    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(std::io::Error::other(error_msg))
    }
}

pub fn forget_net(ssid: &str) -> std::io::Result<()> {
    let output = Command::new("nmcli")
        .args(["connection", "delete", "id", ssid])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(std::io::Error::other(error_msg))
    }
}

fn parse_network_line(line: &str, saved_ssids: &HashSet<String>) -> Option<WifiNetwork> {
    let mut right = line.rsplitn(3, ':');
    let security = right.next()?;
    let signal = right.next()?;
    let left = right.next()?;

    let (in_use, ssid) = left.split_once(':')?;
    if ssid.is_empty() {
        return None;
    }

    Some(WifiNetwork {
        ssid: ssid.to_string(),
        signal: signal.to_string(),
        security: security.to_string(),
        is_saved: saved_ssids.contains(ssid),
        is_connected: in_use.trim() == "*",
    })
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
        .args(["-t", "-f", "IN-USE,SSID,SIGNAL,SECURITY", "dev", "wifi", "list"])
        .output();

    match output {
        Ok(out) => {
            let saved_ssids = get_saved_ssids();
            let stdout = String::from_utf8_lossy(&out.stdout);

            stdout
                .lines()
                .filter(|line| !line.is_empty())
                .filter_map(|line| parse_network_line(line, &saved_ssids))
                .collect()
        }
        Err(_) => Vec::new(),
    }
}
