use std::{collections::HashMap, time};

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::header;

lazy_static! {
    static ref ATTACKVALUE: Vec<&'static str> = vec![
        "<script>alert(\"XSS\");</script>",
        "UNION SELECT ALL FROM information_schema AND ' or SLEEP(5) or '",
        "../../../../etc/passwd",
        "/bin/cat /etc/passwd; ping 127.0.0.1; curl google.com",
        "<!ENTITY xxe SYSTEM \"file:///etc/shadow\">]><pwn>&hack;</pwn>",
    ];
}

pub struct Help {
    pub http_client: reqwest::blocking::Client,
}

impl Default for Help {
    fn default() -> Self {
        Self::new()
    }
}

impl Help {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"));
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build()
            .unwrap();
        Help {
            http_client: client,
        }
    }
    pub fn random_key(&self) -> String {
        let mut rng = thread_rng();
        let chars: String = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect();
        chars
    }

    pub fn generate_attack_params(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for v in ATTACKVALUE.iter() {
            let key = self.random_key();
            map.insert(key, v.to_string());
        }
        map
    }

    pub fn normal_request(&self, url: &str) -> Result<reqwest::blocking::Response> {
        let resp = self
            .http_client
            .get(url)
            .timeout(time::Duration::from_secs(5))
            .send()
            .context("send normal request error")?;
        Ok(resp)
    }

    pub fn attack(&self, url: &str) -> Result<reqwest::blocking::Response> {
        let params = self.generate_attack_params();
        let resp = self
            .http_client
            .get(url)
            .query(&params)
            .timeout(time::Duration::from_secs(5))
            .send()
            .context("send attack request error")?;

        Ok(resp)
    }
}
