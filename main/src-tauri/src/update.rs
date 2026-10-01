use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Serialize;
use sha2::{Digest, Sha256};
use tauri::{AppHandle, Emitter};

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/Frostfleee/fTools/releases/latest";
const DOWNLOAD_PREFIX: &str = "https://github.com/Frostfleee/fTools/releases/download/";
const ASSET_NAME: &str = "fTools.exe";
const UPDATED_FLAG: &str = "--updated";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    version: String,
    current_version: String,
}

struct Release {
    version: String,
    url: String,
    size: u64,
    sha256: Option<String>,
}

fn version_parts(version: &str) -> Vec<u64> {
    let mut parts: Vec<u64> = version
        .trim()
        .trim_start_matches(['v', 'V'])
        .split(['.', '-', '+'])
        .take(3)
        .map(|part| part.parse().unwrap_or(0))
        .collect();
    parts.resize(3, 0);
    parts
}

fn is_newer(latest: &str, current: &str) -> bool {
    version_parts(latest) > version_parts(current)
}

fn sibling(exe: &Path, suffix: &str) -> PathBuf {
    let mut name = exe.file_name().map(OsString::from).unwrap_or_else(|| OsString::from(ASSET_NAME));
    name.push(suffix);
    exe.with_file_name(name)
}

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent(concat!("fTools/", env!("CARGO_PKG_VERSION")))
        .connect_timeout(Duration::from_secs(10))
        .read_timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Couldn't start the updater: {e}"))
}

async fn latest_release(client: &reqwest::Client) -> Result<Release, String> {
    let body = client
        .get(LATEST_RELEASE_URL)
        .header("Accept", "application/vnd.github+json")
        .timeout(Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| format!("Couldn't reach GitHub: {e}"))?
        .error_for_status()
        .map_err(|e| format!("GitHub refused the update check: {e}"))?
        .text()
        .await
        .map_err(|e| format!("Couldn't read GitHub's reply: {e}"))?;
    let json: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("GitHub sent an unexpected reply: {e}"))?;

    let version = json["tag_name"]
        .as_str()
        .ok_or("The latest release has no version tag.")?
        .trim_start_matches(['v', 'V'])
        .to_string();
    let asset = json["assets"]
        .as_array()
        .and_then(|assets| {
            assets
                .iter()
                .find(|asset| asset["name"].as_str().is_some_and(|name| name.eq_ignore_ascii_case(ASSET_NAME)))
        })
        .ok_or_else(|| format!("The latest release has no {ASSET_NAME} to download."))?;
    let url = asset["browser_download_url"]
        .as_str()
        .filter(|url| url.starts_with(DOWNLOAD_PREFIX))
        .ok_or("The latest release has an invalid download link.")?
        .to_string();
    let size = asset["size"].as_u64().unwrap_or(0);
    let sha256 = asset["digest"]
        .as_str()
        .and_then(|digest| digest.strip_prefix("sha256:"))
        .map(|digest| digest.to_ascii_lowercase());

    Ok(Release { version, url, size, sha256 })
}

async fn download(
    app: &AppHandle,
    client: &reqwest::Client,
    release: &Release,
    expected_sha256: &str,
    path: &Path,
) -> Result<(), String> {
    let mut response = client
        .get(&release.url)
        .send()
        .await
        .and_then(|response| response.error_for_status())
        .map_err(|e| format!("Couldn't download the update: {e}"))?;
    let total = response.content_length().filter(|&length| length > 0).unwrap_or(release.size);

    let mut file = fs::File::create(path).map_err(|e| format!("Couldn't save the update next to fTools: {e}"))?;
    let mut hasher = Sha256::new();
    let mut received: u64 = 0;
    let mut last_percent = u64::MAX;

    while let Some(chunk) = response.chunk().await.map_err(|e| format!("The download was interrupted: {e}"))? {
        file.write_all(&chunk).map_err(|e| format!("Couldn't save the update: {e}"))?;
        hasher.update(&chunk);
        received += chunk.len() as u64;
        if total > 0 {
            let percent = (received * 100 / total).min(100);
            if percent != last_percent {
                last_percent = percent;
                let _ = app.emit("update-progress", percent);
            }
        }
    }
    file.sync_all().map_err(|e| format!("Couldn't save the update: {e}"))?;
    drop(file);

    if release.size > 0 && received != release.size {
        return Err("The download is incomplete. Try again.".to_string());
    }
    let actual: String = hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect();
    if actual != expected_sha256 {
        return Err("The downloaded update is damaged. Try again.".to_string());
    }
    Ok(())
}

async fn rename_with_retry(from: &Path, to: &Path) -> std::io::Result<()> {
    let mut attempt = 0;
    loop {
        match fs::rename(from, to) {
            Ok(()) => return Ok(()),
            Err(_) if attempt < 10 => {
                attempt += 1;
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
            Err(e) => return Err(e),
        }
    }
}

#[tauri::command]
pub async fn check_for_update() -> Result<Option<UpdateInfo>, String> {
    let release = latest_release(&client()?).await?;
    Ok(is_newer(&release.version, CURRENT_VERSION).then(|| UpdateInfo {
        version: release.version,
        current_version: CURRENT_VERSION.to_string(),
    }))
}

#[tauri::command]
pub async fn install_update(app: AppHandle) -> Result<(), String> {
    let client = client()?;
    let release = latest_release(&client).await?;
    if !is_newer(&release.version, CURRENT_VERSION) {
        return Err("fTools is already up to date.".to_string());
    }
    let expected_sha256 = release
        .sha256
        .clone()
        .ok_or("The latest release has no checksum, so it can't be installed safely.")?;

    let exe = std::env::current_exe().map_err(|e| format!("Couldn't find the running fTools: {e}"))?;
    let new_path = sibling(&exe, ".new");
    let old_path = sibling(&exe, ".old");

    if let Err(e) = download(&app, &client, &release, &expected_sha256, &new_path).await {
        let _ = fs::remove_file(&new_path);
        return Err(e);
    }

    let _ = fs::remove_file(&old_path);
    if let Err(e) = rename_with_retry(&exe, &old_path).await {
        let _ = fs::remove_file(&new_path);
        return Err(format!("Couldn't replace fTools in its folder: {e}"));
    }
    if let Err(e) = rename_with_retry(&new_path, &exe).await {
        let _ = fs::rename(&old_path, &exe);
        let _ = fs::remove_file(&new_path);
        return Err(format!("Couldn't move the new version into place: {e}"));
    }

    std::process::Command::new(&exe)
        .arg(UPDATED_FLAG)
        .spawn()
        .map_err(|e| format!("fTools was updated, but couldn't restart: {e}. Open it again to finish."))?;
    app.exit(0);
    Ok(())
}

pub fn remove_previous_version() {
    let Ok(exe) = std::env::current_exe() else { return };
    let old_path = sibling(&exe, ".old");
    let attempts = if std::env::args().any(|arg| arg == UPDATED_FLAG) { 100 } else { 1 };
    for _ in 0..attempts {
        if !old_path.exists() || fs::remove_file(&old_path).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
