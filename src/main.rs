use anyhow::Result;
use clap::arg;
use clap::Parser;
use wafcheck::help::Help;
use wafcheck::init;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let plugin_manger = init();

    let help = Help::new();

    let resp = help.attack(&args.url)?;
    let status = resp.status();
    let headers = resp.headers().to_owned();
    println!("{:?}", headers);
    let content = resp.text()?;
    let waf_name = plugin_manger.run_check(&content, status, &headers);
    match waf_name {
        Some(waf_name) => {
            println!("Waf Name is {}", waf_name);
        }
        None => {
            println!("Waf Name is None");
        }
    }
    Ok(())
}
