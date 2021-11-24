use std::env;

use anyhow::{bail, Result};
use tokio::{fs, process};

pub mod auto_update;

pub async fn install_webview2() -> Result<()> {
    let should_install = rfd::AsyncMessageDialog::new()
        .set_title("Install WebView2 Runtime")
        .set_description("The WebView2 Runtime must be installed to use this program. Install now?")
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::YesNo)
        .show()
        .await;

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

pub fn clear_code_cache() {
    use std::fs;
    let execute = || -> Result<()> {
        let mut code_cache_dir = env::current_exe()?;
        code_cache_dir.set_extension("exe.WebView2");
        code_cache_dir.push("EBWebView\\Default\\Code Cache\\wasm");

        if code_cache_dir.is_dir() {
            const THRESHOLD: u64 = 1024 * 1024; // 1mo

            for entry in fs::read_dir(&code_cache_dir)? {
                let path = entry?.path();
                if path.is_file() && path.metadata()?.len() > THRESHOLD {
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    };
    let _ = execute();
}
