// File        : idmrs/src/main.rs
// Author      : Hadi Cahyadi <cumulus13@gmail.com>
// Description : command line to download with Internet Download Manager (IDM) on Windows OS
// Repo        : https://github.com/cumulus13/idmrs
// License     : MIT
//
// Rust port of https://github.com/cumulus13/pyidm (idm/idm.py)

mod config;
#[cfg(windows)]
mod com;

use clap::Parser;
use colored::*;
use config::Config;
use std::process::ExitCode;

/// Command line downloader with/via Internet Download Manager (IDM)
#[derive(Parser, Debug)]
#[command(name = "idmrs", version, about, long_about = None)]
struct Args {
    /// URL(s) to download, or "c" to get url from clipboard
    urls: Vec<String>,

    /// Path to save
    #[arg(short = 'p', long)]
    path: Option<String>,

    /// Save with different name
    #[arg(short = 'o', long)]
    output: Option<String>,

    /// Confirm before download
    #[arg(short = 'c', long)]
    confirm: bool,

    /// Add link to IDM without starting download
    #[arg(short = 'a', long)]
    add: bool,

    /// Url referrer
    #[arg(short = 'r', long)]
    referrer: Option<String>,

    /// Cookie string
    #[arg(short = 'C', long)]
    cookie: Option<String>,

    /// Post data string
    #[arg(short = 'D', long = "post-data")]
    post_data: Option<String>,

    /// Username if required
    #[arg(short = 'U', long)]
    username: Option<String>,

    /// Password if required
    #[arg(short = 'P', long)]
    password: Option<String>,

    /// Send with custom User-Agent string
    #[arg(long = "user-agent", visible_alias = "ua")]
    user_agent: Option<String>,

    /// Set config: format section:option:value, or pass "doc" for a list of
    /// valid section/option names
    #[arg(long)]
    config: Option<String>,
}

fn get_from_clipboard() -> anyhow::Result<String> {
    use arboard::Clipboard;
    match Clipboard::new().and_then(|mut cb| cb.get_text()) {
        Ok(text) if !text.is_empty() => Ok(text),
        _ => {
            use std::io::{self, Write};
            print!("Please re-input url download to:");
            io::stdout().flush().ok();
            let mut q = String::new();
            io::stdin().read_line(&mut q)?;
            let q = q.trim().to_string();
            if q.is_empty() {
                anyhow::bail!("You not input URL Download !");
            }
            Ok(q)
        }
    }
}

fn docs() {
    println!("{}", "uppercase words is VALUE NAME".cyan());
    println!();
    println!("{}", "download:path:DIR_NAME".bright_yellow());
    println!("{}", "download:confirm:1 or 0".bright_green());
    println!("{}", "data:user_agent:STRING".bright_green());
    println!("{}", "debug:verbose:1 or 0".bright_green());
}

#[cfg(not(windows))]
fn run() -> anyhow::Result<()> {
    anyhow::bail!("{}", "This only for Windows OS !".bright_white().on_red());
}

#[cfg(windows)]
fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    // --config doc : print docs and exit, matching Python's early sys.argv inspection.
    if let Some(cfg_arg) = args.config.as_deref() {
        if cfg_arg == "doc" {
            docs();
            return Ok(());
        }
        let parts: Vec<&str> = cfg_arg.splitn(3, ':').collect();
        if parts.len() == 3 {
            let mut cfg = Config::load();
            cfg.write_config(parts[0], parts[1], parts[2])?;
            println!(
                "{}",
                format!("Config updated: {}:{} = {}", parts[0], parts[1], parts[2]).green()
            );
            return Ok(());
        } else {
            eprintln!("{}", "INVALID config parameter/argument !".on_red().white());
            return Ok(());
        }
    }

    if args.urls.is_empty() {
        com::bring_to_top();
        anyhow::bail!("No URL given. Run with --help for usage.");
    }

    let cfg = Config::load();
    let download_path = args
        .path
        .clone()
        .or_else(|| cfg.get("DOWNLOAD_PATH"))
        .unwrap_or_else(|| ".".to_string());
    let confirm = args.confirm || cfg.get_bool("DOWNLOAD_CONFIRM");
    let user_agent = args.user_agent.clone().or_else(|| cfg.get("DATA_USER_AGENT"));

    let lflag = if confirm {
        0
    } else if args.add {
        2
    } else {
        1
    };

    for url in &args.urls {
        let resolved = if url == "c" {
            get_from_clipboard()?
        } else {
            url.clone()
        };

        let send_args = com::SendLinkArgs {
            link: &resolved,
            referrer: args.referrer.as_deref(),
            cookie: args.cookie.as_deref(),
            post_data: args.post_data.as_deref(),
            user: args.username.as_deref(),
            password: args.password.as_deref(),
            path_to_save: Some(download_path.as_str()),
            output: args.output.as_deref(),
            lflag,
            user_agent: user_agent.as_deref(),
        };

        match com::send_link_to_idm(&send_args) {
            Ok(()) => {
                if !cfg.get_bool("DEBUG_VERBOSE") {
                    println!("\n{}", "Link sent to IDM successfully.".bright_yellow().on_blue());
                }
            }
            Err(e) => println!("Error: {}", e.to_string().white().on_red()),
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{}", e.to_string().white().on_red());
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
