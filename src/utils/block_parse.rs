use tokio::task::spawn_blocking;
use wz_reader::{util::node_util::parse_node, WzNodeArc, WzNodeCast};

use crate::{append_to_log, Error, Result};

pub async fn block_parse(node: &WzNodeArc) -> Result<()> {
    let node = node.clone();

    let path = node
        .read()
        .unwrap()
        .try_as_file()
        .map(|f| f.wz_file_meta.path.clone());
    if let Some(path) = path {
        append_to_log(&format!("try parsing wz file: {}\n", path)).await?;
    }

    spawn_blocking(move || parse_node(&node))
        .await
        .map_err(|_| Error::InitWzFailed)?
        .map_err(Error::from)
}

pub async fn block_parse_with_parent(node: &WzNodeArc, parent: &WzNodeArc) -> Result<()> {
    let node = node.clone();
    let parent = parent.clone();

    let path = node
        .read()
        .unwrap()
        .try_as_file()
        .map(|f| f.wz_file_meta.path.clone());
    if let Some(path) = path {
        append_to_log(&format!("try parsing wz file: {}\n", path)).await?;
    }

    spawn_blocking(move || node.write().unwrap().parse(&parent))
        .await
        .map_err(|_| Error::InitWzFailed)?
        .map_err(Error::from)
}
