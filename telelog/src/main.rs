use crate::constants::BASEURL;
use chrono::Local;
use hostname::get as get_hostname;
use reqwest::blocking::multipart;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use clap::Parser;
use errors::AppError;
use self::settings::Settings;

mod errors;
mod settings;
mod constants;

type Result<T> = std::result::Result<T, AppError>;

#[derive(Parser)]
struct AppArgs {
    /// Send a message to the channel
    #[clap(long, short, conflicts_with = "file")]
    message: Option<String>,
    /// Send a file to the channel
    #[clap(long, short, conflicts_with = "message")]
    file: Option<PathBuf>,
    /// Set chat_id in the configuration file
    #[clap(long, short)]
    chat_id: Option<i64>,
    /// Set token in the configuration file
    #[clap(long, short)]
    token: Option<String>,
}

fn get_settings_path() -> Result<String> {
    let path = format!("{}/.config/telelog/settings", env::var("HOME").unwrap());

    if !Path::new(&path).exists() {
        return Err(AppError::Settings);
    }

    Ok(path)
}

fn read_settings() -> Result<Settings> {
    let settings_path = get_settings_path()?;
    let contents = fs::read_to_string(settings_path)?;

    Settings::try_from(contents)
}

fn set_setting(settings: &Settings) -> Result<()> {
    let settings_path = get_settings_path()?;

    fs::write(&settings_path, settings.to_string())?;
    println!("Modified your configuration file.");
    Ok(())
}

fn send_message(
    client: &Client,
    chat_id: i64,
    token: &str,
    message: &str,
    server_name: &str,
) -> Result<()> {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let url = format!("{BASEURL}{token}/sendMessage");

    let body = json!({
        "chat_id": chat_id,
        "text": format!("Date: {date}\nMessage: {message}\nServer: {server_name}"),
        "disable_notification": true
    });

    client.post(&url).json(&body).send()?;
    Ok(())
}

fn send_file(
    client: &Client,
    chat_id: i64,
    token: &str,
    file_path: &PathBuf,
    server_name: &str,
) -> Result<()> {
    let date: String = Local::now().format("%Y-%m-%d").to_string();
    let url = format!("{BASEURL}{token}/sendDocument");

    let body = json!({
        "chat_id": chat_id,
        "caption": format!("Date: {date}\nServer: {server_name}"),
        "disable_notification": true
    });

    let file = fs::read(file_path)?;
    let form = multipart::Form::new().part("document", multipart::Part::bytes(file));
    client.post(&url).multipart(form).json(&body).send()?;
    Ok(())
}

fn main() -> Result<()> {
    let AppArgs {
        message,
        file,
        chat_id,
        token,
    } = AppArgs::parse();

    let mut settings = read_settings()?;
    let server_name = get_hostname()?.into_string().unwrap();
    let client = Client::new();

    if let Some(file) = file.as_ref() {
        send_file(
            &client,
            chat_id.unwrap_or(settings.channel_id),
            token.as_deref().unwrap_or(&settings.token),
            file,
            &server_name,
        )?;
        set_setting(&settings)?;
    }

    if let Some(message) = message.as_deref() {
        send_message(
            &client,
            chat_id.unwrap_or(settings.channel_id),
            token.as_deref().unwrap_or(&settings.token),
            message,
            &server_name,
        )?;
    }

    if let Some(chat_id) = chat_id {
        settings.channel_id = chat_id;
        set_setting(&settings)?;
    }

    if let Some(token) = token {
        settings.token = token;
        set_setting(&settings)?;
    }

    Ok(())
}
