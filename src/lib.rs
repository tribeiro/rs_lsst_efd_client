use std::error::Error;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EfdAuth {
    host: String,
    schema_registry: String,
    port: String,
    username: String,
    password: String,
    path: String,
}

impl EfdAuth {
    pub async fn new(efd_name: &str) -> Result<EfdAuth, Box<dyn Error>> {
        let url = format!("https://roundtable.lsst.codes/segwarides/creds/{efd_name}");
        let response = reqwest::get(&url).await?;

        let response_text = response.text().await?;
        println!("{response_text}");
        let efd_auth: EfdAuth = serde_json::from_str(&response_text)?;

        Ok(efd_auth)
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }
    pub fn get_schema_registry(&self) -> &str {
        &self.schema_registry
    }
    pub fn get_port(&self) -> &str {
        &self.port
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
}
