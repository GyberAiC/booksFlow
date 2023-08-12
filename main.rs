mod downloader;
mod config;
mod errors;

use downloader::BookDownloader;
use config::Config;
use log::{info, error};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    let config = Config::from_file("config.toml").unwrap();

    let downloader = BookDownloader::new(&config.api_key, &config.authors);
    if let Err(e) = downloader.download_books() {
        error!("Error: {}", e);
    }
}
