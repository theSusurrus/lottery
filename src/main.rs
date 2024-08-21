use names::Provider;
use rand;
use std::sync::{Arc, Mutex};

mod config;
mod names;

const CONFIG_PATH: &str = "config.toml";

slint::include_modules!();

fn names_to_string(names: &Vec<String>) -> String {
    names.join("\n").into()
}

fn generate_status(names: &Vec<String>) -> String {
    "Zostało ".to_owned() + &names.len().to_string()
}

fn main() -> Result<(), slint::PlatformError> {
    let config: config::LotteryConfig = config::LotteryConfig::new(CONFIG_PATH);

    let provider: names::html::HtmlProvider = crate::names::html::HtmlProvider::new(&config.name_source);
    let names =
        Arc::new(
            Mutex::new(
                provider.get_names().unwrap()));

    let ui = AppWindow::new()?;
    
    ui.set_status(generate_status(&names.lock().unwrap()).into());
    ui.set_list(names_to_string(&names.lock().unwrap()).into());
    ui.set_listFile(config.name_source.into());

    ui.on_draw_person({
        let ui_handle = ui.as_weak();
        let names = names.clone();
        move || {
            let ui = ui_handle.unwrap();

            let mut names = names.lock().unwrap();

            if names.len() > 1 {
                let random_index = rand::random::<usize>() % names.len();
                let name = names[random_index].clone();
                names.remove(random_index);
    
                ui.set_status(generate_status(&names).into());
                ui.set_list(names_to_string(&names).into());
                ui.set_winner(name.into());
            } else {
                let mut winner_text: String = "WINNER: ".to_string();
                winner_text += &names[0].clone();
                ui.set_winner(winner_text.into());
            }
        }
    });

    ui.on_restart({
        let ui_handle = ui.as_weak();
        let names = names.clone();
        move || {
            let ui = ui_handle.unwrap();

            let mut names = names.lock().unwrap();

            *names = provider.get_names().unwrap();

            ui.set_status(generate_status(&names).into());
            ui.set_list(names_to_string(&names).into());
            ui.set_winner("".into());
        }
    });

    ui.run()
}
