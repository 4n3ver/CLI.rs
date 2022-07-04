use reqwest::{Client, RequestBuilder, Url};
use std::rc::Rc;

use super::auth::{AuthForm, TokenData};

#[derive(Clone)]
pub struct Request {
    host: Rc<Url>,
    client: Client,
}

impl Request {
    pub fn new(host: Url, client: Client) -> Self {
        Self {
            host: Rc::new(host),
            client,
        }
    }

    pub fn login_nonce(&self) -> RequestBuilder {
        self.client.get(self.url("/login_web_app.cgi?nonce"))
    }

    pub fn login(&self, form: &AuthForm) -> RequestBuilder {
        self.client
            .post(self.url("/login_web_app.cgi"))
            .form(form.as_ref())
    }

    pub fn check_expire(&self, token_data: &TokenData) -> RequestBuilder {
        self.client
            .get(self.url("/check_expire_web_app.cgi"))
            .header("Cookie", format!("sid={}", token_data.sid))
    }

    pub fn radio_status(&self) -> RequestBuilder {
        self.client
            .get(self.url("/fastmile_radio_status_web_app.cgi"))
    }

    pub fn reboot(&self, token_data: &TokenData) -> RequestBuilder {
        self.client
            .post(self.url("/reboot_web_app.cgi"))
            .header("Cookie", format!("sid={}", &token_data.sid))
            .form(&[("csrf_token", &token_data.csrf_token)])
    }

    fn url(&self, path: &'static str) -> Url {
        self.host.join(path).unwrap()
    }
}

impl Default for Request {
    fn default() -> Self {
        Self::new("http://192.168.12.1".parse().unwrap(), Client::new())
    }
}
