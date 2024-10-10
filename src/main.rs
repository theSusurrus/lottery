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
    match  &names.len() {
        0 => "".to_string(),
        len => "Zostało ".to_owned() + &format!("{}", len)
    }
}

#[derive(Clone)]
struct LogContext {
    filename: String,
}

fn get_log_name() -> String {
    let time = Utc::now();
    format!("{}-{}-{}_{}_{}_{}.txt",
        time.year(),
        time.month(),
        time.day(),
        time.hour(),
        time.minute(),
        time.second())
}

fn open_log(ctx: &Arc<Mutex<LogContext>>) {
    let mut context = ctx.lock().unwrap();
    context.filename = get_log_name();
    File::create(context.filename.clone()).unwrap();
}

fn write_to_log(ctx: &Arc<Mutex<LogContext>>, buf: &[u8]) {
    let context = ctx.lock().unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(context.filename.clone())
        .unwrap();
    file.write(buf).unwrap();
}

fn refresh_ui(ui: &AppWindow, names: &Vec<String>, winner_text: &str) {
    ui.set_status(generate_status(&names).into());
    ui.set_list(names_to_string(&names).into());
    ui.set_winner(winner_text.into());
}

fn restart(ui: &AppWindow,
           provider: &names::html::HtmlProvider,
           names: &Arc<Mutex<Vec<String>>>,
           log_ctx: &Arc<Mutex<LogContext>>) {
    match provider.get_names() {
        Ok(provided) =>{
            refresh_ui(&ui, &provided, "");
            open_log(log_ctx);
            *names.lock().unwrap() = provided;
        },
        Err(error) => {
            ui.set_status(error.to_string().into());
            *names.lock().unwrap() = vec![];
        },
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let config: config::LotteryConfig = config::LotteryConfig::new(CONFIG_PATH);
    ui.set_listFile(config.name_source.clone().into());

    let names: Arc<Mutex<Vec<String>>> =
        Arc::new(
            Mutex::new(vec![]));

    let log_context =
        Arc::new(Mutex::new(
            LogContext {
                filename: "".to_string(),
            }));

    let provider: names::html::HtmlProvider =
        crate::names::html::HtmlProvider::new(&config.name_source.clone());
    restart(&ui, &provider, &names, &log_context);

    ui.on_draw_person({
        let ui_handle = ui.as_weak();
        let names = names.clone();
        let log_context = log_context.clone();
        move || {
            let ui = ui_handle.unwrap();

            let mut names = names.lock().unwrap();

            if names.len() > 1 {
                let random_index = rand::random::<usize>() % names.len();
                let name_drawn = names[random_index].clone();
                names.remove(random_index);

                write_to_log(&log_context, format!("{}\n", name_drawn).as_bytes());

                refresh_ui(&ui, &names, name_drawn.as_str());
            } else if names.len() == 1 {

                let mut winner_text: String = "WINNER: ".to_string();
                winner_text += &names[0].clone();

                write_to_log(&log_context, names[0].clone().as_bytes());

                *names = vec![];

                refresh_ui(&ui, &names, &winner_text);
            }
        }
    });

    ui.on_restart({
        let ui_handle = ui.as_weak();
        let names = names.clone();
        let log_context = log_context.clone();
        move || {
            let ui = ui_handle.unwrap();
            restart(&ui, &provider, &names, &log_context);
        }
    });

    ui.run()
}
