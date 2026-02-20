pub struct App {
    pub wifi_list: Vec<String>,
    pub selected_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            wifi_list: vec![
                "Home_Wifi".to_string(), 
                "Guest Wifi 1".to_string(), 
                "Guest Wifi 2".to_string()
            ],
            selected_index: 0,
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
