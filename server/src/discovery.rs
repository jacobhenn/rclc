use rocket::serde::Deserialize;
use std::net::IpAddr;

#[derive(FromForm, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DiscoveryRequest {
    pub ip: Option<IpAddr>,
    pub port: u16,
    pub requested_by: u32,
    pub looking_for: u32,
    pub public_key: String, // Lets get proper parsing on this value done at some point
}
