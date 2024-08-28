use std::fmt::Display;
use crate::errors::AppError;

#[derive(Default)]
pub struct Settings {
    pub channel_id: i64,
    pub token: String,
}

impl TryFrom<String> for Settings {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut channel_id = -1;
        let mut token = String::default();

        for line in value.lines() {
            if line.starts_with("#") {
                continue;
            }

            let (key, value) = line
                .trim()
                .split_once("=")
                .ok_or(AppError::Parser("El formato es `Key = Value`".to_owned()))?;

            match key {
                "chat_id" => channel_id = value.parse()?,
                "token" => token = value.to_owned(),
                _ => continue,
            }
        }

        Ok(Self { channel_id, token })
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "chat_id = {}\ntoken = {}\n", self.channel_id, self.token)
    }
}
