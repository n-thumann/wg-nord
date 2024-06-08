use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// https://github.com/NordSecurity/nordvpn-linux/blob/bd470d191525a2f39308ddd05eb23e3182ecdf76/daemon/dns/nameservers.go#L6-L13
pub const DEFAULT_DNS_IPV4: IpAddr = IpAddr::V4(Ipv4Addr::new(103, 86, 96, 100));
pub const DEFAULT_DNS_IPV6: IpAddr = IpAddr::V6(Ipv6Addr::new(
    0x2400, 0xbb40, 0x4444, 0x0, 0x0, 0x0, 0x0, 0x0100,
));
