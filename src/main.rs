#![windows_subsystem = "windows"]

use chrono::{Datelike, Timelike};
use names::Provider;
use rand;
use std::sync::{Arc, Mutex};
use chrono::prelude::Utc;
use std::fs::{OpenOptions, File};
use std::io::Write;

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

#[derive(Clone)]
struct LogContext {
    filename: String,
}

fn get_log_name() -> String {
    let time = Utc::now();
    let path = format!("{}-{}-{}_{}_{}_{}.txt",
        time.year(),
        time.month(),
        time.day(),
        time.hour(),
        time.minute(),
        time.second());
    
    path
}

fn open_log(filename: String) {
    File::create(filename).unwrap();
}

fn write_to_log(filename: String, buf: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();
    file.write(buf).unwrap();
}

fn main() -> Result<(), slint::PlatformError> {
    let config: config::LotteryConfig = config::LotteryConfig::new(CONFIG_PATH);

    let provider: names::html::HtmlProvider = crate::names::html::HtmlProvider::new(&config.name_source);
    let names =
        Arc::new(
            Mutex::new(
                provider.get_names().unwrap()));

    let log_context = 
        Arc::new(Mutex::new(
            LogContext {
                filename: get_log_name(),
            }));

    open_log(log_context.lock().unwrap().filename.clone());

    let ui = AppWindow::new()?;
    
    ui.set_status(generate_status(&names.lock().unwrap()).into());
    ui.set_list(names_to_string(&names.lock().unwrap()).into());
    ui.set_listFile(config.name_source.into());

    ui.on_draw_person({
        let ui_handle = ui.as_weak();
        let names = names.clone();
        let log_context = log_context.clone();
        move || {
            let ui = ui_handle.unwrap();

            let mut names = names.lock().unwrap();
            let log_context = log_context.lock().unwrap();

            if names.len() > 1 {
                let random_index = rand::random::<usize>() % names.len();
                let name = names[random_index].clone();
                names.remove(random_index);
                
                let name_line = format!("{}\n", name);
                write_to_log(log_context.filename.clone(), name_line.as_bytes());
    
                ui.set_status(generate_status(&names).into());
                ui.set_list(names_to_string(&names).into());
                ui.set_winner(name.into());
            } else if names.len() == 1 {

                let mut winner_text: String = "WINNER: ".to_string();
                winner_text += &names[0].clone();

                write_to_log(log_context.filename.clone(), names[0].clone().as_bytes());

                ui.set_winner(winner_text.into());

                names.remove(0);
            }
        }
    });

    ui.on_restart({
        let ui_handle = ui.as_weak();
        let names = names.clone();
        let log_context = log_context.clone();
        move || {
            let ui = ui_handle.unwrap();

            let mut names = names.lock().unwrap();
            let mut log_context = log_context.lock().unwrap();

            *names = provider.get_names().unwrap();
            log_context.filename = get_log_name();
            open_log(log_context.filename.clone());

            ui.set_status(generate_status(&names).into());
            ui.set_list(names_to_string(&names).into());
            ui.set_winner("".into());
        }
    });

    ui.run()
}
