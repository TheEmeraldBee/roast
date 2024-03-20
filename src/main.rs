use std::net::IpAddr;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use clap::Parser;
use command::{ServerCommand, ServerResult};
use config::Config;
use lazy_static::lazy_static;

mod command;
mod website;

mod auth;

mod config;

use rcgen::generate_simple_self_signed;
use rocket::config::{SecretKey, TlsConfig};
use rocket::{main, routes};

use website::*;

#[macro_export]
macro_rules! html_template {
    ($filepath:expr => [$($val:expr => $replace:expr),*]) => {{
        #[allow(unused_mut)]
        let mut value = include_str!($filepath).to_string();

        $(
            value = value.replace($val, $replace);
        )*

        RawHtml(value)
    }};
}

lazy_static! {
    static ref CONFIG: Config = {
        let file = std::fs::read_to_string("./roast-options.toml")
            .expect("roast-options.toml does not exist");

        toml::from_str::<Config>(&file).expect("roast-options.toml is incorrectly formatted")
    };
}

lazy_static! {
    static ref COMMAND: RwLock<ServerCommand> = {
        let command = ServerCommand::new(
            Command::new("sh")
                .arg(CONFIG.runnables[0].path.clone())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed!"),
            200,
        );

        RwLock::new(command)
    };
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    gen_tls: bool,

    #[arg(long)]
    gen_config: bool,
}

#[main]
async fn main() {
    let commands = Args::parse();

    if commands.gen_tls {
        let cert =
            generate_simple_self_signed(vec![]).expect("Self Signed Cert Failed to Generate");

        std::fs::write(
            "./cert.pem",
            cert.serialize_pem().expect("cert.pem failed to serialize"),
        )
        .expect("cert.pem failed to write");

        std::fs::write("./key.pem", cert.serialize_private_key_pem())
            .expect("key.pem failed to write");

        println!("Cert Files Generated. Run 'roast' to run your server!");
        return;
    }

    if commands.gen_config {
        std::fs::write(
            "./roast-options.toml",
            include_str!("../roast-options.toml"),
        )
        .expect("File should be able to write");

        println!("Roast Options Generated! Now Change Those Passwords!");
        return;
    }

    if !Path::new("./cert.pem").exists() || !Path::new("./key.pem").exists() {
        println!("Please run command with --gen_tls to generate the cert files required.");
    }

    ctrlc::set_handler(|| {
        COMMAND
            .write()
            .expect("COMMAND failed to lock")
            .command
            .kill()
            .expect("Command Failed To Kill");
    })
    .expect("CTRL-C Handler failed to set");

    fn handle_command() -> anyhow::Result<()> {
        loop {
            thread::sleep(Duration::from_millis(500));

            let mut command = COMMAND.write().expect("COMMAND failed to lock");

            match command.handle().unwrap() {
                ServerResult::Continue => {}
                ServerResult::Stopped => {
                    command.running = false;
                }
                ServerResult::Restart => {
                    *command = ServerCommand::new(
                        Command::new("./handle.sh")
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("Failed!"),
                        200,
                    )
                }
            }

            drop(command);
        }
    }

    thread::spawn(handle_command);

    let rocket_config = rocket::config::Config {
        address: IpAddr::from_str(&CONFIG.address).expect("Config's ip should be valid"),
        port: CONFIG.port,
        tls: Some(TlsConfig::from_paths("./cert.pem", "./key.pem")),
        secret_key: SecretKey::generate().expect("Failed to generate secret key"),
        ..Default::default()
    };

    rocket::custom(rocket_config)
        .mount("/", routes![index, log, login, logout, user])
        .mount("/action", routes![boot, kill, website::command])
        .mount("/data", routes![data_log])
        .launch()
        .await
        .expect("Rocket crashed");
}
