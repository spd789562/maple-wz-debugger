use tokio::io::AsyncWriteExt;
use wz_reader::{node, property};

pub mod utils;

pub use utils::*;

pub type Result<T> = std::result::Result<T, Error>;

pub const LOG_FILE: &str = "wz_debugger.txt";
pub const WZ_BASE: &str = "Base.wz";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("init wz failed")]
    InitWzFailed,
    #[error("root wz not yet initialized, please use init command first")]
    NotInitialized,
    #[error("node error: {0}")]
    NodeError(#[from] node::Error),
    #[error("image parse error")]
    ImageParseError(#[from] property::png::WzPngParseError),
    #[error("image sending error")]
    ImageSendError,
    #[error("sound parse error")]
    SoundParseError(#[from] property::sound::WzSoundError),
    #[error("string parse error")]
    StringParseError(#[from] property::string::WzStringParseError),
    #[error("node not found")]
    NodeNotFound,
    #[error("node type mismatch, can only use on {0}")]
    NodeTypeMismatch(&'static str),
}

pub async fn append_to_log(msg: &str) -> Result<()> {
    let log_file = std::path::Path::new(LOG_FILE);
    let mut file = tokio::fs::OpenOptions::new()
        .append(true)
        .write(true)
        .open(log_file)
        .await?;

    file.write(msg.as_bytes()).await?;

    Ok(())
}
