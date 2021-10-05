// use std::fmt::format;

use std::fs::{set_permissions, File, Permissions};
use std::io::copy;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;

// use tempfile::Builder;

use zip::ZipArchive;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Font {
    pub name: String,
    url: String,
    size: u32,
    browser_download_url: String,
}

pub struct FontHandler {
    client: reqwest::Client,
}

impl FontHandler {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn get_fonts(self) -> Result<Vec<Font>, reqwest::Error> {
        let res = self
            .client
            .get("https://api.github.com/repos/ryanoasis/nerd-fonts/releases/23316029/assets")
            .send()
            .await?;

        let body = res.text().await?;

        let fonts: Vec<Font> = serde_json::from_str(&body).unwrap();

        Ok(fonts)
    }

    pub async fn download_and_install(font: &Font) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Downloading {}...",
            font.name.split(".").next().unwrap_or("")
        );
        let response = reqwest::get(format!("{}", font.browser_download_url)).await?;

        // let temp_dir = Builder::new().prefix("fonts").tempdir()?;

        let file_output_path = format!("~/fonts/{}", font.name);

        let file_name = Path::new(&file_output_path);

        println!("file_output_path {}", file_output_path);
        println!("File_name {:?}", file_name);

        let mut destination = File::create(&file_output_path)?;

        let content = response.text().await?;

        copy(&mut content.as_bytes(), &mut destination)?;

        let zip_file = File::open(file_name).unwrap();

        let mut archive = ZipArchive::new(zip_file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            let output_path = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            println!("{}", file.name());

            let mut outfile = std::fs::File::create(&output_path).unwrap();

            std::io::copy(&mut file, &mut outfile).unwrap();

            #[cfg(unix)]
            {
                if let Some(mode) = file.unix_mode() {
                    set_permissions(&output_path, Permissions::from_mode(mode)).unwrap();
                }
            }

            if (&*file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, output_path.display());
                std::fs::create_dir_all(&output_path).unwrap();
            } else {
                println!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    output_path.display(),
                    file.size()
                );
                if let Some(p) = output_path.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p).unwrap();
                    }
                }
            }
        }

        Ok(())
    }
}
