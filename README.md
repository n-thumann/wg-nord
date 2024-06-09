# wg-nord

Command-line tool to create Wireguard configuration files for NordVPN

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
3. Run `wg-nord config <server> <token>`
4. Copy & paste the output into your WireGuard client

## Building

1. Install [Rust](https://www.rust-lang.org/)
2. Run `cargo build`
3. Done
