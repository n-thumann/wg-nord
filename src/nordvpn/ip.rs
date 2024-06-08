use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// https://github.com/NordSecurity/nordvpn-linux/blob/bd470d191525a2f39308ddd05eb23e3182ecdf76/daemon/vpn/nordlynx/nordlynx.go#L36
pub const CLIENT_IPV4: IpAddr = IpAddr::V4(Ipv4Addr::new(10, 5, 0, 2));

// https://github.com/NordSecurity/nordvpn-linux/blob/bd470d191525a2f39308ddd05eb23e3182ecdf76/daemon/vpn/network.go
pub fn client_ipv6(server_ip: Ipv6Addr) -> IpAddr {
    let prefix = server_ip.octets();
    let suffix: [u8; 8] = [0x0, 0x0, 0x0, 0x11, 0x0, 0x5, 0x0, 0x2];

    let mut result: [u8; 16] = [0x0; 16];
    result[..8].clone_from_slice(&prefix[..8]);
    result[8..].clone_from_slice(&suffix);

    IpAddr::V6(Ipv6Addr::from(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_address() {
        let server_ip = Ipv6Addr::new(0x2a0d, 0x5600, 0x8, 0x26a, 0x0, 0x0, 0x0, 0x3);
        let expected_result = Ipv6Addr::new(0x2a0d, 0x5600, 0x8, 0x26a, 0x0, 0x011, 0x5, 0x2);

        assert_eq!(expected_result, client_ipv6(server_ip));
    }
}
