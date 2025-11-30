use std::{
    fmt::{self, Formatter},
    net::IpAddr,
};

use ini::{Ini, WriteOption};
use ipnet::IpNet;

#[derive(Debug)]
pub struct Config {
    pub private_key: String,
    pub addresses: Vec<IpAddr>,
    pub dns_servers: Vec<IpAddr>,
    pub public_key: String,
    pub allowed_ips: Vec<IpNet>,
    pub endpoint: String,
    pub persistent_keepalive: i32,
}

fn join_with_comma<T: ToString>(ips: &[T]) -> String {
    ips.iter()
        .map(|ip| ip.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut conf = Ini::new();
        conf.with_section(Some("Interface"))
            .set("PrivateKey", &self.private_key)
            .set("Address", join_with_comma(&self.addresses))
            .set("DNS", join_with_comma(&self.dns_servers));

        conf.with_section(Some("Peer"))
            .set("PublicKey", &self.public_key)
            .set("AllowedIPs", join_with_comma(&self.allowed_ips))
            .set("Endpoint", &self.endpoint)
            .set("PersistentKeepalive", self.persistent_keepalive.to_string());

        let mut buf = Vec::new();
        let options = WriteOption {
            kv_separator: " = ",
            ..WriteOption::default()
        };

        conf.write_to_opt(&mut buf, options).unwrap();
        let output = String::from_utf8(buf).unwrap();
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use super::*;

    #[test]
    fn join_ips_with_comma() {
        let ips = vec![
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V6(Ipv6Addr::LOCALHOST),
        ];

        assert_eq!("127.0.0.1, ::1", join_with_comma(&ips));
    }

    #[test]
    fn join_ip_networks_with_comma() {
        let ips = vec![
            IpNet::new(Ipv4Addr::LOCALHOST.into(), 8).unwrap(),
            IpNet::new(Ipv6Addr::LOCALHOST.into(), 128).unwrap(),
        ];

        assert_eq!("127.0.0.1/8, ::1/128", join_with_comma(&ips));
    }
}
