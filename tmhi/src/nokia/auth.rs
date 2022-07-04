use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{cell::RefCell, rc::Rc, time::Instant};
use ext_util::serde::{TryParse, TryParseResult};

use super::request::Request;

pub struct AuthClient {
    request: Request,
    username: String,
    password: String,
    token: RefCell<Token>,
}
impl AuthClient {
    pub fn new(request: &Request, username: &str, password: &str) -> Self {
        AuthClient {
            request: request.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            token: RefCell::new(Token::expired()),
        }
    }

    pub async fn refresh(&self) -> Result<Rc<TokenData>, reqwest::Error> {
        let token = if self.token.borrow().is_expired() {
            log::info!("Refreshing expired token...");
            self.login().await?
        } else {
            self.token.borrow().data()
        };

        Ok(token)
    }

    pub async fn login(&self) -> Result<Rc<TokenData>, reqwest::Error> {
        let token = self.get_token().await?;

        self.token.replace(token);

        log::info!("Logged in until {:?}!", &self.token.borrow().expiration);

        Ok(self.token.borrow().data())
    }

    async fn get_token(&self) -> Result<Token, reqwest::Error> {
        let nonce = self.get_nonce().await?;
        let form = AuthForm::new(&self.username, &self.password, &nonce);
        let token_data = self.get_token_data(&form).await?;
        let token_expiration = self.get_token_expiration(&token_data).await?;
        let token = Token::new(token_data, token_expiration);

        debug_assert!(!&token.is_expired());

        Ok(token)
    }

    async fn get_nonce(&self) -> Result<Nonce, reqwest::Error> {
        let response = self.request.login_nonce().send().await?;
        let body = response.json().await?;

        Ok(body)
    }

    // todo: translate error kinds
    async fn get_token_data(&self, form: &AuthForm) -> Result<TokenData, reqwest::Error> {
        let response = self.request.login(form).send().await?;
        let body: TryParse<TokenData> = response.json().await?;
        let result = match (body.value, body.raw.get("result")) {
            (TryParseResult::Parsed(token), Some(result)) if result == 0 => Ok(token),
            (value, _) => todo!("Invalid login value: {:?}, raw: {:?}", value, body.raw),
        };

        result
    }

    async fn get_token_expiration(
        &self,
        token_data: &TokenData,
    ) -> Result<TokenExpiration, reqwest::Error> {
        let response = self.request.check_expire(token_data).send().await?;
        let body: TokenExpiration = response.json().await?;

        Ok(body)
    }
}

struct Token {
    data: Option<Rc<TokenData>>,
    expiration: Instant,
}
impl Token {
    pub fn new(data: TokenData, expiration: TokenExpiration) -> Self {
        Token {
            data: Some(Rc::new(data)),
            expiration: expiration.expiration,
        }
    }

    pub fn expired() -> Self {
        Token {
            data: None,
            expiration: Instant::now(),
        }
    }

    pub fn data(&self) -> Rc<TokenData> {
        match &self.data {
            Some(data) => Rc::clone(data),
            None => panic!("called `Token::data()` on a Expired value"),
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.data {
            None => true,
            _ => self.expiration < Instant::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    pub sid: String,

    #[serde(rename = "token")]
    pub csrf_token: String,
}

#[derive(Deserialize, Debug)]
struct TokenExpiration {
    #[serde(
        rename = "expire",
        deserialize_with = "ext_util::serde::deserialize_secs_from_now"
    )]
    expiration: Instant,
}

#[derive(Serialize, Deserialize, Debug)]
struct Nonce {
    iterations: usize,
    nonce: String,

    #[serde(rename = "randomKey")]
    random_key: String,

    #[allow(dead_code)]
    #[serde(rename = "pubkey")]
    public_key: String,
}

#[derive(Debug)]
pub struct AuthForm(Vec<(&'static str, String)>);
impl AuthForm {
    fn new(username: &str, password: &str, nonce: &Nonce) -> AuthForm {
        let mut hasher = FormHasher::new();

        let password = hasher.pw_hash(nonce.iterations, password);
        let creds_hash = hasher.kv_hash(username, &password);
        let key: [u8; 16] = rand::random();
        let iv: [u8; 16] = rand::random();

        let form = [
            ("userhash", hasher.kv_hash(username, &nonce.nonce)),
            (
                "RandomKeyhash",
                hasher.kv_hash(&nonce.random_key, &nonce.nonce),
            ),
            ("response", hasher.kv_hash(&creds_hash, &nonce.nonce)),
            ("nonce", nonce.nonce.to_owned()),
            ("enckey", base64::encode(key)),
            ("enciv", base64::encode(iv)),
        ];

        let form = form
            .into_iter()
            .map(|(key, val)| (key, AuthForm::escape_url(&val)))
            .collect();

        AuthForm(form)
    }

    fn escape_url(url: &str) -> String {
        let escape = |c| match c {
            '+' => '-',
            '/' => '_',
            '=' => '.',
            c => c,
        };

        url.chars().map(escape).collect()
    }
}
impl AsRef<[(&'static str, String)]> for AuthForm {
    fn as_ref(&self) -> &[(&'static str, String)] {
        &self.0
    }
}

struct FormHasher(Sha256);
impl FormHasher {
    fn new() -> FormHasher {
        FormHasher(Sha256::new())
    }

    fn pw_hash(&mut self, iterations: usize, val: &str) -> String {
        if iterations >= 1 {
            let val_hash = (1..iterations).fold(val.as_bytes().to_owned(), |bytes, _| {
                self.0.update(bytes);
                self.0.finalize_reset().into_iter().collect()
            });

            // to hex string
            val_hash.into_iter().map(|v| format!("{v:x}")).collect()
        } else {
            val.to_lowercase()
        }
    }

    fn kv_hash(&mut self, key: &str, val: &str) -> String {
        let formatted_str = format!("{key}:{val}");

        self.0.update(formatted_str.as_bytes());
        base64::encode(self.0.finalize_reset())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn should_create_valid_form() {
        // given
        let nonce = Nonce {
            iterations: 0,
            nonce: "zIgvpmeliRQPXnKIjAgqjKmCLu9UiSSuUKGrgdj1r8I=".to_owned(),
            random_key: "865".to_owned(),
            public_key: "".to_owned(),
        };

        // when
        let AuthForm(form) = AuthForm::new("admin", "password", &nonce);
        let form_map: HashMap<&'static str, String> = form.into_iter().collect();

        // then
        assert_eq!(
            form_map.get("userhash").unwrap(),
            "oGDD6y7GtxYddeM9kT9su3d3PvEYTTq7tT17nMV8cd4."
        );
        assert_eq!(
            form_map.get("RandomKeyhash").unwrap(),
            "6OyJkKB5CVp8TJvzOi8UtoCrE3xoD6OGhb-1wXSuENE."
        );
        assert_eq!(
            form_map.get("response").unwrap(),
            "qpl2HDBuion6am4Is90U_nfVKk8J0p2cyyiLdDYHrAg."
        );
        assert_eq!(
            form_map.get("nonce").unwrap(),
            "zIgvpmeliRQPXnKIjAgqjKmCLu9UiSSuUKGrgdj1r8I."
        );
        assert!(form_map.get("enckey").is_some());
        assert!(form_map.get("enciv").is_some());
    }
}
