use serde::Deserialize;

#[derive(Deserialize)]
pub struct RunPath {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,

    pub runnables: Vec<RunPath>,

    pub admin_user: String,
    pub admin_pass: String,

    pub main_user: String,
    pub main_pass: String,
}

/*

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,

    pub run_path: String,

    pub admin_user: String,
    pub admin_pass: String,

    pub main_user: String,
    pub main_pass: String,
}

*/
