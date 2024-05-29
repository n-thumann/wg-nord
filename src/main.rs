use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// NordVPN server
    #[arg(short, long)]
    server: String,
    /// NordVPN token
    #[arg(short, long)]
    token: String,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}, {:?}", args.server, args.token);

    let client = reqwest::blocking::Client::new();
    let body = client
        .get("https://api.nordvpn.com/v1/users/services/credentials")
        .header("Authorization", format!("token:{}", args.token))
        .json();

    println!("body = {:?}", body);

}
