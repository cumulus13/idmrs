// File        : idmrs/src/lib.rs
// Author      : Hadi Cahyadi <cumulus13@gmail.com>
// Description : idmrs as a library — drive Internet Download Manager (IDM)
//               from any Rust program, not just the bundled CLI.
// Repo        : https://github.com/cumulus13/idmrs
// License     : MIT
//
//! # idmrs
//!
//! Drive [Internet Download Manager](https://www.internetdownloadmanager.com/)
//! (Windows only) from Rust, via the same COM automation interface the IDM
//! browser extensions use (`CIDMLinkTransmitter` / `SendLinkToIDM2`).
//!
//! This crate is both a CLI (`idmrs`, see the repo README) and a library you
//! can add as a dependency:
//!
//! ```toml
//! [dependencies]
//! idmrs = "0.1"
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use idmrs::{DownloadMode, SendLinkRequest};
//!
//! let req = SendLinkRequest::new("https://example.com/file.zip")
//!     .path_to_save(r"D:\Downloads")
//!     .mode(DownloadMode::AddOnly);
//!
//! idmrs::send_link(&req)?;
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! On non-Windows targets the crate still compiles (so downstream crates can
//! build cross-platform), but [`send_link`] and [`bring_to_top`] return an
//! error / no-op respectively, since IDM itself only exists on Windows.

pub mod config;

#[cfg(windows)]
pub mod com;

pub use config::Config;

/// How IDM should treat a submitted link, mirroring the `lflag` values used
/// by `idm.py` / `IDMan.download()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DownloadMode {
    /// Start downloading immediately (default).
    #[default]
    Download,
    /// Ask the user to confirm before starting the download.
    Confirm,
    /// Add the link to IDM's queue without starting the download.
    AddOnly,
}

impl DownloadMode {
    #[cfg_attr(not(windows), allow(dead_code))]
    fn lflag(self) -> i32 {
        match self {
            DownloadMode::Confirm => 0,
            DownloadMode::AddOnly => 2,
            DownloadMode::Download => 1,
        }
    }
}

/// A request to send a link to IDM, mirroring `IDMan.download()` in `idm.py`.
///
/// Build one with [`SendLinkRequest::new`] and the builder-style setters,
/// then pass it to [`send_link`].
#[derive(Debug, Clone, Default)]
pub struct SendLinkRequest {
    pub link: String,
    pub referrer: Option<String>,
    pub cookie: Option<String>,
    pub post_data: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub path_to_save: Option<String>,
    pub output: Option<String>,
    pub user_agent: Option<String>,
    pub mode: DownloadMode,
}

impl SendLinkRequest {
    /// Start building a request for the given URL.
    pub fn new(link: impl Into<String>) -> Self {
        Self {
            link: link.into(),
            ..Default::default()
        }
    }

    pub fn referrer(mut self, v: impl Into<String>) -> Self {
        self.referrer = Some(v.into());
        self
    }

    pub fn cookie(mut self, v: impl Into<String>) -> Self {
        self.cookie = Some(v.into());
        self
    }

    pub fn post_data(mut self, v: impl Into<String>) -> Self {
        self.post_data = Some(v.into());
        self
    }

    pub fn user(mut self, v: impl Into<String>) -> Self {
        self.user = Some(v.into());
        self
    }

    pub fn password(mut self, v: impl Into<String>) -> Self {
        self.password = Some(v.into());
        self
    }

    pub fn path_to_save(mut self, v: impl Into<String>) -> Self {
        self.path_to_save = Some(v.into());
        self
    }

    pub fn output(mut self, v: impl Into<String>) -> Self {
        self.output = Some(v.into());
        self
    }

    pub fn user_agent(mut self, v: impl Into<String>) -> Self {
        self.user_agent = Some(v.into());
        self
    }

    pub fn mode(mut self, v: DownloadMode) -> Self {
        self.mode = v;
        self
    }
}

/// Sends a link to IDM via COM automation (Windows) or returns an error on
/// any other platform.
#[cfg(windows)]
pub fn send_link(req: &SendLinkRequest) -> anyhow::Result<()> {
    let args = com::SendLinkArgs {
        link: &req.link,
        referrer: req.referrer.as_deref(),
        cookie: req.cookie.as_deref(),
        post_data: req.post_data.as_deref(),
        user: req.user.as_deref(),
        password: req.password.as_deref(),
        path_to_save: req.path_to_save.as_deref(),
        output: req.output.as_deref(),
        lflag: req.mode.lflag(),
        user_agent: req.user_agent.as_deref(),
    };
    com::send_link_to_idm(&args)
}

/// Non-Windows stub: IDM itself only exists on Windows.
#[cfg(not(windows))]
pub fn send_link(_req: &SendLinkRequest) -> anyhow::Result<()> {
    anyhow::bail!("This only for Windows OS !")
}

/// Brings IDM's main window to the foreground, if it's running.
/// No-op on non-Windows platforms.
#[cfg(windows)]
pub fn bring_to_top() {
    com::bring_to_top();
}

/// Non-Windows stub: no-op.
#[cfg(not(windows))]
pub fn bring_to_top() {}
