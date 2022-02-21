use chrono::Utc;
use hmac::{Hmac, Mac};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};

use crate::error::Result;
use crate::responses::{ConnectionCheck, Operation, Operations, Tag, Tags};

pub struct Client {
    host: String,
    access_key: String,
    secret_key: Vec<u8>,
}

impl Client {
    pub fn new(host: String, access_key: String, secret_key: String) -> Result<Self> {
        let secret_key_decoded = base64::decode(secret_key)?;
        Ok(Client {
            host,
            access_key,
            secret_key: secret_key_decoded,
        })
    }

    pub fn get_operations(&self) -> Result<Operations> {
        self.get::<Operations>("/api/operations")
    }

    pub fn get_operation(&self, id: i64) -> Result<Operation> {
        self.get::<Operation>(&format!("/api/operations/{id}"))
    }

    pub fn create_operation(&self, slug: &str, name: &str) {
        unimplemented!()
    }

    pub fn check_connection(&self) -> Result<ConnectionCheck> {
        self.get::<ConnectionCheck>("/api/checkconnection")
    }

    pub fn submit_evidence(&self) {
        unimplemented!()
    }

    pub fn tags_for_operation(&self, slug: &str) -> Result<Tags> {
        self.get::<Tags>(&format!("/api/operations/{slug}/tags"))
    }

    pub fn create_tag(&self) {
        unimplemented!()
    }

    fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let client = reqwest::blocking::Client::new();
        let data = vec![];
        let sig = mac_request("GET", path, &date, &data, &self.secret_key)?;
        let encoded_sig = base64::encode(&sig);
        let auth = format!("{}:{}", self.access_key, encoded_sig);
        let resp = client
            .get(format!("{}{}", self.host, path))
            .header("Date", date)
            .header("Authorization", auth)
            .send()?;

        let t = resp.json::<T>()?;
        Ok(t)
    }

    fn post(&self) {}
}

fn mac_request(
    method: &str,
    uri: &str,
    date: &str,
    data: &[u8],
    secret_key: &[u8],
) -> Result<Vec<u8>> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret_key)?;
    mac.update(method.as_bytes());
    mac.update(b"\n");
    mac.update(uri.as_bytes());
    mac.update(b"\n");
    mac.update(date.as_bytes());
    mac.update(b"\n");
    mac.update(&Sha256::digest(data));
    Ok(mac.finalize().into_bytes().to_vec())
}
