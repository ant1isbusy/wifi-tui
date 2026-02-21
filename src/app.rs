use crate::network::{self, WifiNetwork};
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    // holds the state, what we have currently selected,
    // and the list of available networks
    pub wifi_list: Vec<WifiNetwork>,
    pub highlighted_index: usize,
    pub is_scanning: bool,
    pub selected_network: Option<WifiNetwork>,
    pub password_input: String,
    pub input_mode: InputMode,
    pub tx: Sender<Vec<WifiNetwork>>,
    pub rx: Receiver<Vec<WifiNetwork>>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            wifi_list: Vec::new(),
            highlighted_index: 0,
            is_scanning: false,
            input_mode: InputMode::Normal,
            selected_network: None,
            password_input: String::new(),
            tx,
            rx,
        }
    }

    pub fn start_scan(&mut self) {
        if self.is_scanning {
            return;
        }
        let tx = self.tx.clone();
        self.is_scanning = true;

        std::thread::spawn(move || {
            let networks = network::fetch_wifi_networks();
            let _ = tx.send(networks);
        });
    }

    pub fn connect(&mut self) {
        if let Some(net) = self.selected_network.as_ref() {
            let pass = if net.is_saved {
                None
            } else {
                Some(self.password_input.clone())
            };
            match network::connect_to_net(&net.ssid, pass) {
                Ok(_) => {
                    self.selected_network = None;
                    self.password_input.clear();
                    self.input_mode = InputMode::Normal;
                }
                Err(e) => {
                    // TODO: put error message into TUI
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }

    pub fn update(&mut self) {
        if let Ok(networks) = self.rx.try_recv() {
            self.wifi_list = networks;
            self.is_scanning = false;
        }
    }

    pub fn next(&mut self) {
        if self.highlighted_index < self.wifi_list.len() - 1 {
            self.highlighted_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.highlighted_index > 0 {
            self.highlighted_index -= 1;
        }
    }
}
