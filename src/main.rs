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
    let waf_name = plugin_manger.run_check(&resp.text()?, status);
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
