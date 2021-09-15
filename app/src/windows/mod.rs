use std::{env, path::Path};

use anyhow::{bail, Result};
use native_dialog::MessageType;
use tokio::{fs, process, task};

pub mod auto_update;

pub async fn install_webview2() -> Result<()> {
    let should_install = native_dialog::MessageDialog::new()
        .set_title("Install WebView2 Runtime")
        .set_text("The WebView2 Runtime must be installed to use this program. Install now?")
        .set_type(MessageType::Warning)
        .show_confirm()?;

    if !should_install {
        bail!("WebView2 install cancelled by user");
    }

    let setup =
        reqwest::get("https://go.microsoft.com/fwlink/p/?LinkId=2124703").await?.bytes().await?;

    let temp_dir = env::temp_dir().join("trilogy-save-editor");
    let path = temp_dir.join("MicrosoftEdgeWebview2Setup.exe");

    // If not exists
    if fs::metadata(&temp_dir).await.is_err() {
        fs::create_dir(temp_dir).await?;
    }
    fs::write(&path, setup).await?;

    let status = process::Command::new(path)
        .arg("/install")
        .status()
        .await
        .expect("Failed to launch WebView2 install");

    if !status.success() {
        bail!("Failed to install WebView2");
    }
    Ok(())
}

pub async fn clear_code_cache_if_more_than_20mo() {
    let _ = task::spawn_blocking(|| -> Result<()> {
        let mut code_cache_dir = env::current_exe()?;
        code_cache_dir.set_extension("exe.WebView2");
        code_cache_dir.push("EBWebView\\Default\\Code Cache");

        const THRESHOLD: u64 = if cfg!(debug_assertions) {
            1024 * 1024 * 50 // 50mo
        } else {
            1024 * 1024 * 20 // 20mo
        };

        if dir_size(&code_cache_dir)? > THRESHOLD {
            std::fs::remove_dir_all(code_cache_dir)?;
        }
        Ok(())
    })
    .await;
}

fn dir_size(path: &Path) -> Result<u64> {
    use std::fs;

    let mut result = 0;

    if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let path = entry?.path();
            if path.is_file() {
                result += path.metadata()?.len();
            } else {
                result += dir_size(&path)?;
            }
        }
    } else {
        result = path.metadata()?.len();
    }
    Ok(result)
}
