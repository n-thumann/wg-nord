# wg-nord

Command-line tool for generating WireGuard configuration files for NordVPN

```
# wg-nord --server us9591.nordvpn.com --token 818f804a2c24e89082a37d1486ac45b83f9452ea9b953678c7e115f354f96463
[Interface]
PrivateKey = IDQu32ovMxFQ3VSywgKTUBFGs/CN/S3ouWAmV6nsc1M=
Address = 10.5.0.2, 2a0d:5600:8:26a:0:11:5:2
DNS = 103.86.96.100, 2400:bb40:4444::100

[Peer]
PublicKey = V1WC7wt34kcSDyqPuUhN56NJ0v+GlqY9TwZR5WlzzB4=
AllowedIPs = 0.0.0.0/0, ::/0
Endpoint = us9591.nordvpn.com:51820
PersistentKeepalive = 25
```

## Installation

You can either
1. Install via `cargo install wg-nord --git https://github.com/n-thumann/wg-nord/`
2. Download a precompiled binary of the [latest release](https://github.com/n-thumann/wg-nord/releases/latest)

## Usage

1. Pick a server from https://nordvpn.com/servers/tools/. Make sure that is supports WireGuard.
2. Obtain a token from the NordVPN (see https://support.nordvpn.com/hc/en-us/articles/20286980309265-How-to-use-a-token-with-NordVPN-on-Linux).
3. Run `wg-nord --server <server> --token <token>` (see below)
4. Copy & paste the output into your WireGuard client

```
Command-line tool for generating WireGuard configuration files for NordVPN

Usage: wg-nord [OPTIONS] --server <SERVER> --token <TOKEN>

Options:
      --server <SERVER>            NordVPN server
      --token <TOKEN>              NordVPN token [env: TOKEN=]
      --dns-servers <DNS_SERVERS>  DNS servers [default: 103.86.96.100 2400:bb40:4444::100]
      --allowed-ips <ALLOWED_IPS>  Allowed IPs [default: 0.0.0.0/0 ::/0]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Examples

- Specify custom DNS servers
    ```
    # wg-nord --dns-servers 1.1.1.1 --dns-servers 2606:4700:4700::1111 --server us9591.nordvpn.com --token 818f804a2c24e89082a37d1486ac45b83f9452ea9b953678c7e115f354f96463
    [Interface]
    PrivateKey = IDQu32ovMxFQ3VSywgKTUBFGs/CN/S3ouWAmV6nsc1M=
    Address = 10.5.0.2, 2a0d:5600:8:26a:0:11:5:2
    DNS = 1.1.1.1, 2606:4700:4700::1111

    [Peer]
    PublicKey = V1WC7wt34kcSDyqPuUhN56NJ0v+GlqY9TwZR5WlzzB4=
    AllowedIPs = 0.0.0.0/0, ::/0
    Endpoint = us9591.nordvpn.com:51820
    PersistentKeepalive = 25
    ```

- Specify custom allowed IPs (only route specify prefixed via WireGuard)
    ```
    # wg-nord --allowed-ips 1.0.0.0/24 --allowed-ips 2606:4700::/36 --server us9591.nordvpn.com --token 818f804a2c24e89082a37d1486ac45b83f9452ea9b953678c7e115f354f96463
    [Interface]
    PrivateKey = IDQu32ovMxFQ3VSywgKTUBFGs/CN/S3ouWAmV6nsc1M=
    Address = 10.5.0.2, 2a0d:5600:8:26a:0:11:5:2
    DNS = 103.86.96.100, 2400:bb40:4444::100

    [Peer]
    PublicKey = V1WC7wt34kcSDyqPuUhN56NJ0v+GlqY9TwZR5WlzzB4=
    AllowedIPs = 1.0.0.0/24, 2606:4700::/36
    Endpoint = us9591.nordvpn.com:51820
    PersistentKeepalive = 25
    ```

- Specifying IPv6 DNS servers or allowed IPs on NordVPN server without IPv6 support will be ignored
    ```
    wg-nord --dns-servers 1.1.1.1 --dns-servers 2606:4700:4700::1111 --allowed-ips 1.0.0.0/24 --allowed-ips 2606:4700::/36 --server us9590.nordvpn.com --token 818f804a2c24e89082a37d1486ac45b83f9452ea9b953678c7e115f354f96463
    [Interface]
    PrivateKey = IDQu32ovMxFQ3VSywgKTUBFGs/CN/S3ouWAmV6nsc1M=
    Address = 10.5.0.2
    DNS = 1.1.1.1

    [Peer]
    PublicKey = V1WC7wt34kcSDyqPuUhN56NJ0v+GlqY9TwZR5WlzzB4=
    AllowedIPs = 1.0.0.0/24
    Endpoint = us9590.nordvpn.com:51820
    PersistentKeepalive = 25
    ```

## Building

Install [Rust](https://www.rust-lang.org/) and run `cargo build`.

## Background
NordLynx by NordVPN is based on WireGuard, but there's no official way to download a WireGuard configuration file directly, like with OpenVPN. WireGuard is only supported by using NordVPN's official apps. Therefore, I dug through the [code](https://github.com/NordSecurity/nordvpn-linux) of the NordVPN Linux client to figure out how their WireGuard configuration is created.
Compared to the official Linux client, wg-nord even supports IPv6 by default (enabled on `us9591.nordvpn.com` and `us9592.nordvpn.com` at the time of writing).
