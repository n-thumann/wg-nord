use std::error::Error;

use crate::nordvpn::api::credentials;

pub fn get(token: String) -> Result<String, Box<dyn Error>> {
    let credentials: credentials::Credentials = credentials::get(token)?;
    Ok(credentials.nordlynx_private_key.unwrap())
}
