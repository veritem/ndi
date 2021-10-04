use clap::{App, AppSettings, Arg};
use reqwest::Client;

use crate::fonts::FontHandler;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

mod fonts;

#[tokio::main]
async fn main() {
    let matches = App::new("ndi")
        .version("0.0.1")
        .about("Nerd Fonts Installer")
        .author("Makuza Mugabo Verite <mugaboverite@gmail.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("install")
                .about("Install new font")
                .arg(Arg::new("font").required(true)),
        )
        .get_matches();

    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    if let Some(matches) = matches.subcommand_matches("install") {
        let name = matches.value_of("font").unwrap();

        let fonts_results = FontHandler::new(client).get_fonts().await;

        if let Ok(fonts) = fonts_results {
            let find_font = fonts.iter().find(|&s| {
                s.name.to_lowercase().split(".").next().unwrap_or("") == name.to_lowercase()
            });

            if let Some(font) = find_font {
                FontHandler::download_and_install(&font).await.unwrap();
            } else {
                print!("Font not found");
            }
        }
    }
}
