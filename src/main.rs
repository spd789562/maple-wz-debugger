use maple_wz_dubegger::{append_to_log, resolve_base, LOG_FILE, WZ_BASE};
use std::path::Path;
use tokio::fs;

use wz_reader::WzNodeCast;

#[tokio::main]
async fn main() {
    // 1. check if base.wz exists in same directory
    let current = std::env::current_dir().unwrap();
    let base_wz = current.as_path().join(WZ_BASE);
    if !base_wz.exists() {
        println!("Base.wz 不再此資料，請將此程式放置於 Base.wz 所在資料夾");
        pause_cli();
        return;
    }
    let base_wz_str = base_wz.as_os_str().to_str().unwrap();
    let log_file = Path::new(LOG_FILE);
    fs::write(
        log_file,
        format!("start reading base.wz: {}\n", base_wz_str),
    )
    .await
    .unwrap();

    let base_node = resolve_base(base_wz_str, None).await;

    if base_node.is_err() {
        append_to_log(base_node.unwrap_err().to_string().as_str())
            .await
            .unwrap();
        println!("讀取失敗，請查看於同資料夾下的 {} 檔案", LOG_FILE);
        pause_cli();
        return;
    }

    let base_node = base_node.unwrap();

    let version = base_node
        .read()
        .unwrap()
        .try_as_file()
        .map(|f| f.wz_file_meta.patch_version)
        .unwrap_or(0);

    println!("Wz 讀取成功, 檢測到的版本: {}", version);

    append_to_log(&format!(
        "Read wz tree done, Base.wz version: {}\n",
        version
    ))
    .await
    .unwrap();
    pause_cli();
}

fn pause_cli() {
    println!("Press any key to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
