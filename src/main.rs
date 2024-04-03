use anyhow::Result;
use clap::arg;
use clap::Parser;

use comfy_table::Table;
use wafcheck::help::Help;
use wafcheck::init;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: Option<String>,
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    let plugin_manger = init();
    let args = Args::parse();
    let mut table = Table::new();
    table.set_header(vec!["current support check waf"]);
    let wafs = plugin_manger.support_check_wafs();
    for i in wafs.iter() {
        table.add_row(vec![i]);
    }

    if args.list {
        println!("{table}");
    }

    let help = Help::new();
    if let Some(url) = args.url {
        let mut table = Table::new();
        table.set_header(vec!["Check Waf Type"]);
        let resp = help.attack(&url)?;
        let status = resp.status();
        let headers = resp.headers().to_owned();
        let content = resp.text()?;
        let waf_name = plugin_manger.run_check(&content, status, &headers);
        match waf_name {
            Some(waf_name) => {
                table.add_row(vec![waf_name]);
                println!("{table}");
            }
            None => {
                table.add_row(vec!["None"]);
                println!("{table}");
            }
        }
    }

    Ok(())
}
