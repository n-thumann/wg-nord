use std::{error::Error, net::IpAddr};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct IP {
    pub ip: IpAddr,
}

#[derive(Deserialize)]
pub struct IPContainer {
    pub ip: IP,
}

#[derive(Deserialize)]
pub struct Metadata {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct Technology {
    pub identifier: String,
    pub metadata: Vec<Metadata>,
}

#[derive(Deserialize)]
pub struct Server {
    pub hostname: String,
    pub ips: Vec<IPContainer>,
    pub technologies: Vec<Technology>,
}

pub fn get() -> Result<Vec<Server>, Box<dyn Error>> {
    let body: Vec<Server> = reqwest::blocking::get("https://api.nordvpn.com/v1/servers?fields[servers.technologies]&fields[servers.hostname]&fields[servers.ips]&limit=0")?
        .json()?;

    Ok(body)
}
