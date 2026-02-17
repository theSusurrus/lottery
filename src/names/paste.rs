use std::sync::{Arc, Mutex};

use slint::ComponentHandle;

use crate::names;
slint::include_modules!();

pub struct PasteProvider {
}

impl PasteProvider {
    pub fn new() -> PasteProvider {
        PasteProvider {}
    }
}

impl names::Provider for PasteProvider {
    fn get_names(&self) -> Result<Vec<String>, std::io::Error> {
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

        match paste_window.run() {
            Err(e) => return Err(std::io::Error::other(format!("paste window failed: {}", e.to_string()))),
            Ok(_) => ()
        };

        let mut names: Vec<String> = Vec::new();
        for name in names_string.lock().unwrap().split("\n") {
            names.push(name.trim().to_string());
        }

        Ok(names)
    }
}