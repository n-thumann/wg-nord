use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use clap::Parser;
use ipnet::IpNet;

pub mod config;
mod filter;
mod nordvpn;

/// Create a WireGuard configuration file for NordVPN
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// NordVPN server
    #[arg(long)]
    server: String,
    /// NordVPN token
    #[arg(long, env)]
    token: String,
    /// DNS servers
    #[arg(long, default_values_t=vec![nordvpn::dns::DEFAULT_DNS_IPV4, nordvpn::dns::DEFAULT_DNS_IPV6])]
    dns_servers: Vec<IpAddr>,
    /// Allowed IPs
    #[arg(long, default_values_t=vec![IpNet::new(Ipv4Addr::UNSPECIFIED.into(), 0).unwrap(), IpNet::new(Ipv6Addr::UNSPECIFIED.into(), 0).unwrap()])]
    allowed_ips: Vec<IpNet>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let private_key = nordvpn::private_key::get(args.token)?;
    let server = nordvpn::server::get(args.server)?;

    let mut addresses = vec![nordvpn::ip::CLIENT_IPV4];
    if server.supports_ipv6() {
        for ip in server.ip_addresses.iter() {
            if let IpAddr::V6(server_ip) = ip {
                addresses.push(nordvpn::ip::client_ipv6(*server_ip));
            }
        }
    }

    let dns_servers = filter::ip_addrs(args.dns_servers, server.supports_ipv6());
    let allowed_ips = filter::ip_nets(args.allowed_ips, server.supports_ipv6());

    let config = config::Config {
        private_key,
        addresses,
        dns_servers,
        public_key: server.public_key.clone(),
        allowed_ips,
        endpoint: server.endpoint(),
        persistent_keepalive: 25,
    };
    println!("{}", config);

    Ok(())
}
