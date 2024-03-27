use clap::arg;
use clap::Parser;
use regex::Regex;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use reqwest::blocking::Client;
use serde_json::json;
use wafcheck::init;


const XSSSTRING: &str = r#"<script>alert("XSS");</script>"#;
// const SQLISTRING: &str = r#"UNION SELECT ALL FROM information_schema AND ' or SLEEP(5) or '"#;
// const LFISTRING: &str = r#"../../../../etc/passwd"#;
// const RCESTRING: &str = r#"/bin/cat /etc/passwd; ping 127.0.0.1; curl google.com"#;
// const XXESTRING: &str = r#"<!ENTITY xxe SYSTEM "file:///etc/shadow">]><pwn>&hack;</pwn>"#;

pub struct Feature {
    pub pattern: Regex,
    pub waf_type: String,
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short,long)]
    url: String
}

pub fn random_key() -> String {
    let mut rng = thread_rng();
    let chars: String = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect();
    chars
}


fn main() {
    let args = Args::parse();
    println!("url is {}", args.url);
    let key = random_key();
    let plugin_manger = init();
    let params = json!(
        {
            key: XSSSTRING,
        }
    );
    let client = Client::new();
    let body = client
        .get(args.url)
        .query(&params)
        .send()
        .unwrap()
        .text()
        .unwrap();
    let waf_name = plugin_manger.run_check(&body);
    match waf_name {
        Some(waf_name) => {
            println!("Waf Name is {}", waf_name);
        },
        None => {
            println!("Waf Name is None");
        },
    }
}
