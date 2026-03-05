use crate::network::{self, WifiNetwork};
use std::sync::mpsc::{self, Receiver, Sender};
use ratatui::widgets::TableState;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

// holds the state, what we have currently selected,
// and the list of available networks
pub struct App {
    pub wifi_list: Vec<WifiNetwork>,
    pub table_state: TableState,
    pub highlighted_index: usize,
    pub is_scanning: bool,
    pub is_connecting: bool,
    pub connection_error: Option<String>,
    pub selected_network: Option<WifiNetwork>,
    pub connected_network: Option<WifiNetwork>,
    pub password_input: String,
    pub input_mode: InputMode,
    pub tx: Sender<Vec<WifiNetwork>>,
    pub rx: Receiver<Vec<WifiNetwork>>,
    pub conn_tx: Sender<Result<String, String>>,
    pub conn_rx: Receiver<Result<String, String>>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let (conn_tx, conn_rx) = mpsc::channel();
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        Self {
            wifi_list: Vec::new(),
            table_state,
            highlighted_index: 0,
            is_scanning: false,
            is_connecting: false,
            connection_error: None,
            input_mode: InputMode::Normal,
            selected_network: None,
            connected_network: None,
            password_input: String::new(),
            tx,
            rx,
            conn_tx,
            conn_rx,
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
        if self.is_connecting {
            return;
        }

        if let Some(net) = self.selected_network.clone() {
            let pass = if net.is_saved { None } else { Some(self.password_input.clone()) };
            
            self.is_connecting = true;
            self.connection_error = None;
            
            let tx = self.conn_tx.clone();
            let ssid = net.ssid.clone();

            std::thread::spawn(move || {
                match network::connect_to_net(&ssid, pass) {
                    Ok(_) => { let _ = tx.send(Ok(ssid)); }
                    Err(e) => { let _ = tx.send(Err(e.to_string())); }
                }
            });
        }
    }

    pub fn forget(&mut self) {
        if self.wifi_list.is_empty() {
            return;
        }
        
        let ssid = self.selected_network
            .as_ref()
            .unwrap_or_else(|| &self.wifi_list[self.highlighted_index])
            .ssid
            .clone();

        std::thread::spawn(move || {
            let _ = network::forget_net(&ssid);
        });

        self.selected_network = None;
        self.input_mode = InputMode::Normal;
        self.start_scan();
    }

    pub fn update(&mut self) {
        if let Ok(networks) = self.rx.try_recv() {
            self.wifi_list = networks;
            let current_selection = self.table_state.selected().unwrap_or(0);
            if current_selection >= self.wifi_list.len() && !self.wifi_list.is_empty() {
                self.table_state.select(Some(self.wifi_list.len().saturating_sub(1)));
            }
            self.is_scanning = false;
        }

        if let Ok(conn_result) = self.conn_rx.try_recv() {
            self.is_connecting = false;
            match conn_result {
                Ok(ssid) => {
                    for n in &mut self.wifi_list {
                        n.is_connected = n.ssid == ssid;
                    }
                    self.connected_network = self.selected_network.take();
                    self.password_input.clear();
                    self.input_mode = InputMode::Normal;
                }
                Err(e) => {
                    self.connection_error = Some(e);
                }
            }
        }
    }

    pub fn next(&mut self) {
        if self.wifi_list.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.wifi_list.len().saturating_sub(1) {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.wifi_list.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}
