mod auth;
pub mod radio;
mod request;

use auth::AuthClient;
use radio::RadioStatus;
use request::Request;

pub struct Client {
    request: Request,
    auth: AuthClient,
}

impl Client {
    pub fn new(username: &str, password: &str) -> Self {
        let request = Request::default();
        Client {
            auth: AuthClient::new(&request, username, password),
            request,
        }
    }

    pub async fn radio_status(&self) -> Result<RadioStatus, reqwest::Error> {
        let response = self.request.radio_status().send().await?;
        let body = response.json().await?;

        Ok(body)
    }

    pub async fn reboot(&self) -> Result<String, reqwest::Error> {
        let token_data = self.auth.refresh().await?;
        let response = self.request.reboot(&token_data).send().await?;
        let body = response.text().await?;

        Ok(body)
    }

    pub async fn login(&self) -> Result<(), reqwest::Error> {
        self.auth.login().await?;

        Ok(())
    }
}
