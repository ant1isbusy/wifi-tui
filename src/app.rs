use crate::network;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct App {
    // holds the state, what we have currently selected,
    // and the list of available networks
    pub wifi_list: Vec<String>,
    pub selected_index: usize,
    pub is_scanning: bool,
    pub tx: Sender<Vec<String>>,
    pub rx: Receiver<Vec<String>>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            wifi_list: Vec::new(),
            selected_index: 0,
            is_scanning: false,
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

    pub fn update(&mut self) {
        if let Ok(networks) = self.rx.try_recv() {
            self.wifi_list = networks;
            self.is_scanning = false;
        }
    }

    pub fn next(&mut self) {
        if self.selected_index < self.wifi_list.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}
