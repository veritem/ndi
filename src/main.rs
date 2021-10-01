use clap::{App, AppSettings, Arg};

// struct Font {}

fn main() {
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

    if let Some(matches) = matches.subcommand_matches("install") {
        println!("Installing font {}", matches.value_of("font").unwrap());
    }
}
