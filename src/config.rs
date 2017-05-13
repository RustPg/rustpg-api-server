use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use rustc_serialize::json;
use errors::{CliResult,CliError};

#[derive(Debug,RustcDecodable)]
struct ConfigVer1 {
    bind_address: Option<String>,
    bind_port: Option<String>,
    database_url: String,
}

#[derive(Debug,RustcDecodable)]
enum RawConfig {
    V1(ConfigVer1),
}

#[derive(Debug)]
pub struct Config {
    bind_address: String,
    bind_port: String,
    database_url: String,
}

const CONFIG_FILES: [&'static str; 1] = ["config.json"];

impl Config {
    pub fn from_file(path: Option<&str>) -> CliResult<Config> {
        let path_to_file: &Path = if let Some(path_str) = path {
            let path = Path::new(path_str);
            if !path.is_file() {
                return Err(CliError::new(format!("Configuration file `{}` not found", path_str)));
            }

            path
        } else {
            CONFIG_FILES.iter()
                .map(|&path| Path::new(path))
                .find(|path| path.is_file())
                .unwrap_or_else(|| panic!("Invalid config files"))
        };

        let RawConfig::V1(raw_conf) = match path_to_file.extension().and_then(|ext| ext.to_str()) {
            Some("json") => Config::parse_json(path_to_file),
            _ => Err(CliError::new(format!("Invalid file extension: `{:?}`", path_to_file))),
        }?;

        Ok(Config {
            bind_address: raw_conf.bind_address.unwrap_or("0.0.0.0".to_owned()),
            bind_port: raw_conf.bind_port.unwrap_or("".to_owned()),
            database_url: raw_conf.database_url,
        })
    }

    fn parse_json(path: &Path) -> CliResult<RawConfig> {
        let content = Config::load_content(path)?;
        Ok(json::decode(&content[..])?)
    }

    fn load_content(path: &Path) -> CliResult<String> {
        let mut buf = String::new();

        let mut src = File::open(path)?;
        let _ = src.read_to_string(&mut buf)?;

        Ok(buf)
    }
}
