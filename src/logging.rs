use log::error;
use simplelog::{CombinedLogger, ConfigBuilder, LevelFilter, SharedLogger, WriteLogger};
use std::env;
use std::fs::OpenOptions;
use std::net;
use std::path::PathBuf;

pub fn create_logger(logfile: &PathBuf) -> Result<(), std::io::Error> {
    let logconfig = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_time_offset_to_local()
        .unwrap_or_else(|v| v)
        .build();

    let loggers: Vec<Box<dyn SharedLogger>> = vec![WriteLogger::new(
        LevelFilter::Info,
        logconfig,
        OpenOptions::new().append(true).create(true).open(logfile)?,
    )];

    if CombinedLogger::init(loggers).is_err() {
        error!("Could not initialize logger.");
    };

    Ok(())
}

pub fn get_ssh_client() -> String {
    match env::var("SSH_CONNECTION") {
        Ok(con) => {
            let ssh_con = con.split(' ').collect::<Vec<&str>>();

            if ssh_con.len() != 4 {
                return con;
            }

            let mut ssh_client = String::new();
            let ssh_ip = String::from(ssh_con[0]);
            if ssh_ip.parse::<net::Ipv6Addr>().is_ok() {
                ssh_client.push('[');
                ssh_client.push_str(&ssh_ip);
                ssh_client.push(']');
            } else {
                ssh_client.push_str(&ssh_ip);
            }
            ssh_client.push(':');
            ssh_client.push_str(ssh_con[1]);
            ssh_client
        }
        _ => "local user".to_string(),
    }
}
