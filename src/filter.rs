use std::net::IpAddr;

use ipnet::IpNet;

pub fn ip_addrs(ips: Vec<IpAddr>, allow_ipv6: bool) -> Vec<IpAddr> {
    ips.iter()
        .filter(|ip| ip.is_ipv4() || allow_ipv6 && ip.is_ipv6())
        .cloned()
        .collect::<Vec<IpAddr>>()
}

pub fn ip_nets(ips: Vec<IpNet>, allow_ipv6: bool) -> Vec<IpNet> {
    ips.iter()
        .filter(|ip_net| ip_net.addr().is_ipv4() || allow_ipv6 && ip_net.addr().is_ipv6())
        .cloned()
        .collect::<Vec<IpNet>>()
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use super::*;

    #[test]
    fn test_filter_ip_addrs_allow_ipv6() {
        let ips = vec![
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V6(Ipv6Addr::LOCALHOST),
        ];

        let expected_result = vec![
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V6(Ipv6Addr::LOCALHOST),
        ];

        assert_eq!(expected_result, ip_addrs(ips, true));
    }

    #[test]
    fn test_filter_ip_addrs_deny_ipv6() {
        let ips = vec![
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V6(Ipv6Addr::LOCALHOST),
        ];

        let expected_result = vec![IpAddr::V4(Ipv4Addr::LOCALHOST)];

        assert_eq!(expected_result, ip_addrs(ips, false));
    }

    #[test]
    fn test_filter_ip_nets_allow_ipv6() {
        let ips = vec![
            IpNet::new(Ipv4Addr::LOCALHOST.into(), 8).unwrap(),
            IpNet::new(Ipv6Addr::LOCALHOST.into(), 128).unwrap(),
        ];

        let expected_result = vec![
            IpNet::new(Ipv4Addr::LOCALHOST.into(), 8).unwrap(),
            IpNet::new(Ipv6Addr::LOCALHOST.into(), 128).unwrap(),
        ];

        assert_eq!(expected_result, ip_nets(ips, true));
    }

    #[test]
    fn test_filter_ip_nets_deny_ipv6() {
        let ips = vec![
            IpNet::new(Ipv4Addr::LOCALHOST.into(), 8).unwrap(),
            IpNet::new(Ipv6Addr::LOCALHOST.into(), 128).unwrap(),
        ];

        let expected_result = vec![IpNet::new(Ipv4Addr::LOCALHOST.into(), 8).unwrap()];

        assert_eq!(expected_result, ip_nets(ips, false));
    }
}
