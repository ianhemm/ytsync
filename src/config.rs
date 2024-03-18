use clap::{builder::OsStr, Parser};
use clap_serde_derive::ClapSerde;
use serde::Serialize;
use std::{env, error::Error, fs::File, io::Read};
use toml;
use tracing::info;

///
/// # The configuration module.
/// This module is what the program uses to configure itself.
/// All Data configured either through TOML or the command line
/// will be placed here.
///
/// When the program loads the configuration, it will
/// prioritize the values set in this order:
/// 1) Any values set through command line arguments
/// 2) Any values set in the configuration file.
/// 3) Any values set by default
///
/// If two values are set from different locations,
/// the program will pick the value with higher priority.

/**
 * The youtube playlist and subscription video synchronizer
 */
#[derive(ClapSerde, Parser, Serialize, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(long, default_value=None)]
    //A youtube API key
    yt_api: Option<String>,

    // (TODO: Unimplemented) A youtube OAuth token(To get this, launch the program, follow the link, then sign in with your account)
    #[arg(long, default_value = None)]
    yt_oauth_token: Option<String>,

    // The file used to set custom playlists(Currently not changeable)
    #[arg(
        long, short,
        default_value=
            OsStr::from(format!("{}/.config/ytsync/playlists",
                                env!("HOME"))))]
    playlist_file: String,

    // The config directory(Currently Not Changeable)
    #[arg(
        long, short,
        default_value=
            OsStr::from(format!("{}/.config/ytsync/ytsync.toml",
                                env!("HOME"))))]
    config_file: String,
}

impl Config {
    pub fn build() -> Result<Config, Box<dyn Error>> {
        info!("Loading configuration...");
        let conf = Config::parse();
        let file = match File::open(conf.config_file()) {
            Ok(x) => Some(x),
            Err(e) => {
                println!("Cannot open file in {}\n{}", conf.config_file(), e);
                None
            }
        };

        let mut buf = String::new();
        let conf;
        if let Some(mut file) = file {
            file.read_to_string(&mut buf)?;

            conf = Config::from(toml::from_str::<<Config as ClapSerde>::Opt>(&buf)?).merge_clap();
        } else {
            conf = Config::parse();
        };

        if conf.yt_api.is_none() && conf.yt_oauth_token.is_none() {
            panic!("\nPlease have a youtube api key or an OAuth2.0 token set in either\n\n{}\n\nor using the --yt-api flag.", conf.config_file())
        }

        Ok(conf)
    }

    pub fn youtube_api(&self) -> &String {
        // This is the only required field in the configuration,
        // The only reason its optional is to allow for the toml configuration
        // To set it without clap getting in the way
        self.yt_api.as_ref().unwrap()
    }

    pub fn youtube_oauth_token(&self) -> Option<&String> {
        self.yt_oauth_token.as_ref()
    }

    pub fn playlist_file(&self) -> &String {
        &self.playlist_file
    }

    pub fn config_file(&self) -> &String {
        &self.config_file
    }
}
