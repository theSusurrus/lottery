use std::sync::{Arc, Mutex};

use slint::ComponentHandle;

use crate::names;
slint::include_modules!();

pub struct PasteProvider {
    names: Vec<String>
}

impl PasteProvider {
    pub fn new() -> Result<PasteProvider, std::io::Error> {
        let paste_window = PasteWindow::new().unwrap();
        let names_string = Arc::new(Mutex::new(String::new()));

        paste_window.on_names_ready({
            let paste_window = paste_window.as_weak();
            let names_string = names_string.clone();
            move || {
                let shared_string = paste_window.unwrap().get_names();
                names_string.lock().unwrap().push_str(shared_string.as_str());
                paste_window.unwrap().hide().unwrap();
            }
        });

        paste_window.run().unwrap();

        let mut names: Vec<String> = Vec::new();
        for name in names_string.lock().unwrap().split("\n") {
            names.push(name.trim().to_string());
        }

        Ok(PasteProvider { 
            names
        })
    }
}

impl names::Provider for PasteProvider {
    fn get_names(&self) -> Vec<String> {
        self.names.clone()
    }
}
