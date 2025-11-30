use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub nordlynx_private_key: Option<String>,
    errors: Option<Error>,
}

#[derive(Deserialize, Debug)]
struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NordVPN API error: {}", self.message)
    }
}
impl std::error::Error for Error {}

pub fn get(token: String) -> Result<Credentials, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let body: Credentials = client
        .get("https://api.nordvpn.com/v1/users/services/credentials")
        .header("Authorization", format!("Bearer token:{}", token))
        .send()?
        .json()?;

    match body.errors {
        Some(x) => Err(Error { message: x.message }.into()),
        None => Ok(body),
    }
}
