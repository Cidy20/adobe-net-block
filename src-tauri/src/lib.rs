use std::fs;

const START_MARKER: &str = "# Cidy‘s Adobe Net Block Start";
const END_MARKER: &str = "# Cidy‘s Adobe Net Block End";

#[cfg(target_os = "windows")]
const HOSTS_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";
#[cfg(not(target_os = "windows"))]
const HOSTS_PATH: &str = "/etc/hosts";

#[tauri::command]
fn get_hosts_status() -> Result<(bool, String), String> {
    let content = fs::read_to_string(HOSTS_PATH).map_err(|e| e.to_string())?;
    let is_blocked = content.contains(START_MARKER);
    let mut update_date = String::new();

    if is_blocked {
        if let Some(start_idx) = content.find(START_MARKER) {
            let sub_content = &content[start_idx..];
            if let Some(end_idx) = sub_content.find(END_MARKER) {
                let block_content = &sub_content[..end_idx];
                for line in block_content.lines().take(10) {
                    if line.contains("Last update:") {
                        update_date = line.replace("#", "").trim().to_string();
                        break;
                    }
                }
            }
        }
    }

    Ok((is_blocked, update_date))
}

#[tauri::command]
async fn update_hosts(url: String) -> Result<(), String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch: {}", e))?;
    let new_content = response
        .text()
        .await
        .map_err(|e| format!("Failed to get text: {}", e))?;

    let mut hosts_content = fs::read_to_string(HOSTS_PATH).map_err(|e| e.to_string())?;

    // Remove existing block
    if let Some(start_idx) = hosts_content.find(START_MARKER) {
        if let Some(end_idx) = hosts_content.find(END_MARKER) {
            let before = &hosts_content[..start_idx];
            let after = &hosts_content[end_idx + END_MARKER.len()..];
            hosts_content = format!("{}{}", before.trim_end(), after);
        }
    }

    // Append new block
    let updated_hosts = format!(
        "{}\n\n{}\n{}\n{}\n",
        hosts_content.trim_end(),
        START_MARKER,
        new_content.trim(),
        END_MARKER
    );

    fs::write(HOSTS_PATH, updated_hosts).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            "Permission denied. Please run with administrator/sudo privileges.".to_string()
        } else {
            e.to_string()
        }
    })?;

    Ok(())
}

#[tauri::command]
async fn get_source_update_date(url: String) -> Result<String, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch: {}", e))?;
    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to get text: {}", e))?;

    for line in text.lines().take(20) {
        if line.contains("Last update:") {
            return Ok(line.replace("#", "").trim().to_string());
        }
    }
    Ok("-".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_hosts_status,
            update_hosts,
            get_source_update_date
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
