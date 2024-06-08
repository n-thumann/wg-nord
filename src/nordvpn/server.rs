use std::{error::Error, net::IpAddr};

use crate::nordvpn::api::servers;

#[derive(Debug)]
pub struct Server {
    pub hostname: String,
    pub public_key: String,
    pub ip_addresses: Vec<IpAddr>,
}

impl Server {
    pub fn supports_ipv6(&self) -> bool {
        return self.ip_addresses.iter().any(IpAddr::is_ipv6);
    }

    pub fn endpoint(&self) -> String {
        format!("{}:51820", self.hostname).to_string()
    }
}

pub fn get(hostname: String) -> Result<Server, Box<dyn Error>> {
    let servers = servers::get()?;

    let server = match servers.iter().find(|server| server.hostname == hostname) {
        Some(server) => server,
        None => return Err(format!("Could not find server with hostname {}", hostname).into()),
    };

    let technology = match server
        .technologies
        .iter()
        .find(|technology| technology.identifier == "wireguard_udp")
    {
        Some(technology) => technology,
        None => return Err(format!("Server {} does not support Wireguard", hostname).into()),
    };

    let metadata = match technology
        .metadata
        .iter()
        .find(|metadata| metadata.name == "public_key")
    {
        Some(metadata) => metadata,
        None => return Err(format!("Server {} has no WireGuard public key", hostname).into()),
    };

    let ip_addresses = server
        .ips
        .iter()
        .map(|container| container.ip.ip)
        .collect::<Vec<_>>();

    let public_key = &metadata.value;

    let server = Server {
        hostname: server.hostname.to_string(),
        public_key: public_key.to_string(),
        ip_addresses,
    };
    Ok(server)
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use super::*;

    #[test]
    fn server_supports_ipv6() {
        let server = Server {
            hostname: "us9591.nordvpn.com".to_string(),
            public_key: "V1WC7wt34kcSDyqPuUhN56NJ0v+GlqY9TwZR5WlzzB4=".to_string(),
            ip_addresses: vec![
                IpAddr::V4(Ipv4Addr::new(195, 206, 104, 187)),
                IpAddr::V6(Ipv6Addr::new(
                    0x2a0d, 0x5600, 0x8, 0x26a, 0x0, 0x0, 0x0, 0x3,
                )),
            ],
        };

        assert!(server.supports_ipv6());
    }

    #[test]
    fn server_not_supports_ipv6() {
        let server = Server {
            hostname: "us9590.nordvpn.com".to_string(),
            public_key: "V1WC7wt34kcSDyqPuUhN56NJ0v+GlqY9TwZR5WlzzB4=".to_string(),
            ip_addresses: vec![IpAddr::V4(Ipv4Addr::new(195, 206, 104, 179))],
        };

        assert!(!server.supports_ipv6());
    }
}
