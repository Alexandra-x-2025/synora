use std::process::Command;

use serde_json::Value;

use crate::domain::{SoftwareItem, UpdateItem};
use crate::security::{SecurityError, SecurityGuard};

#[derive(Debug)]
pub enum IntegrationError {
    Security(SecurityError),
    CommandFailed(String),
}

#[derive(Debug, Clone, Copy)]
pub enum ParsePath {
    Json,
    TextFallback,
    UnsupportedPlatform,
}

#[derive(Default, Clone, Copy)]
pub struct WingetClient;

impl WingetClient {
    pub fn list_installed(
        &self,
        guard: &SecurityGuard,
    ) -> Result<(Vec<SoftwareItem>, ParsePath), IntegrationError> {
        let cmd = vec![
            "winget".to_string(),
            "list".to_string(),
            "--output".to_string(),
            "json".to_string(),
        ];
        guard.validate_command(&cmd).map_err(IntegrationError::Security)?;

        if !cfg!(target_os = "windows") {
            return Ok((Vec::new(), ParsePath::UnsupportedPlatform));
        }

        match run_winget_json("list") {
            Ok(payload) => Ok((parse_software_items(&payload), ParsePath::Json)),
            Err(json_err) => match run_winget_text("list") {
                Ok(text) => Ok((parse_tabular_software_items(&text), ParsePath::TextFallback)),
                Err(text_err) => Err(IntegrationError::CommandFailed(format!(
                    "winget list json path failed ({json_err}); text fallback failed ({text_err})"
                ))),
            },
        }
    }

    pub fn list_upgrades(
        &self,
        guard: &SecurityGuard,
    ) -> Result<(Vec<UpdateItem>, ParsePath), IntegrationError> {
        let cmd = vec![
            "winget".to_string(),
            "upgrade".to_string(),
            "--output".to_string(),
            "json".to_string(),
        ];
        guard.validate_command(&cmd).map_err(IntegrationError::Security)?;

        if !cfg!(target_os = "windows") {
            return Ok((Vec::new(), ParsePath::UnsupportedPlatform));
        }

        match run_winget_json("upgrade") {
            Ok(payload) => Ok((parse_upgrade_items(&payload), ParsePath::Json)),
            Err(json_err) => match run_winget_text("upgrade") {
                Ok(text) => Ok((parse_tabular_upgrade_items(&text), ParsePath::TextFallback)),
                Err(text_err) => Err(IntegrationError::CommandFailed(format!(
                    "winget upgrade json path failed ({json_err}); text fallback failed ({text_err})"
                ))),
            },
        }
    }
}

fn run_winget_json(operation: &str) -> Result<Value, String> {
    let output = Command::new("winget")
        .arg(operation)
        .arg("--accept-source-agreements")
        .arg("--output")
        .arg("json")
        .output()
        .map_err(|err| format!("failed to invoke winget {operation}: {err}"))?;

    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!(
            "winget {operation} failed with code {code}: {}",
            if stderr.is_empty() { "no stderr" } else { &stderr }
        ));
    }

    serde_json::from_slice::<Value>(&output.stdout)
        .map_err(|err| format!("winget {operation} returned malformed JSON: {err}"))
}

fn run_winget_text(operation: &str) -> Result<String, String> {
    let output = Command::new("winget")
        .arg(operation)
        .arg("--accept-source-agreements")
        .output()
        .map_err(|err| format!("failed to invoke winget {operation} fallback: {err}"))?;

    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!(
            "winget {operation} fallback failed with code {code}: {}",
            if stderr.is_empty() { "no stderr" } else { &stderr }
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_software_items(payload: &Value) -> Vec<SoftwareItem> {
    let mut items = Vec::new();

    if let Some(sources) = payload.get("Sources").and_then(Value::as_array) {
        for source in sources {
            if let Some(packages) = source.get("Packages").and_then(Value::as_array) {
                for pkg in packages {
                    if let Some(item) = package_to_software_item(pkg) {
                        items.push(item);
                    }
                }
            }
        }
        return items;
    }

    if let Some(data) = payload.get("Data").and_then(Value::as_array) {
        for pkg in data {
            if let Some(item) = package_to_software_item(pkg) {
                items.push(item);
            }
        }
        return items;
    }

    if let Some(array) = payload.as_array() {
        for pkg in array {
            if let Some(item) = package_to_software_item(pkg) {
                items.push(item);
            }
        }
    }

    items
}

fn parse_upgrade_items(payload: &Value) -> Vec<UpdateItem> {
    let mut items = Vec::new();

    if let Some(sources) = payload.get("Sources").and_then(Value::as_array) {
        for source in sources {
            if let Some(packages) = source.get("Packages").and_then(Value::as_array) {
                for pkg in packages {
                    if let Some(item) = package_to_upgrade_item(pkg) {
                        items.push(item);
                    }
                }
            }
        }
        return items;
    }

    if let Some(data) = payload.get("Data").and_then(Value::as_array) {
        for pkg in data {
            if let Some(item) = package_to_upgrade_item(pkg) {
                items.push(item);
            }
        }
        return items;
    }

    if let Some(array) = payload.as_array() {
        for pkg in array {
            if let Some(item) = package_to_upgrade_item(pkg) {
                items.push(item);
            }
        }
    }

    items
}

fn parse_tabular_software_items(raw: &str) -> Vec<SoftwareItem> {
    let mut items = Vec::new();
    let lines: Vec<&str> = raw.lines().collect();

    let sep_idx = lines.iter().position(|line| is_separator_line(line));
    let Some(sep_idx) = sep_idx else {
        return items;
    };
    if sep_idx == 0 || sep_idx + 1 >= lines.len() {
        return items;
    }

    let header = lines[sep_idx - 1];
    let separator = lines[sep_idx];
    let ranges = extract_column_ranges(separator);
    if ranges.is_empty() {
        return items;
    }

    let columns: Vec<String> = ranges
        .iter()
        .map(|(start, end)| slice_field(header, *start, *end).to_ascii_lowercase())
        .collect();

    for line in lines.iter().skip(sep_idx + 1) {
        if line.trim().is_empty() {
            continue;
        }

        let mut name = String::new();
        let mut package_id = String::new();
        let mut version = String::new();
        let mut source = String::new();

        for (idx, (start, end)) in ranges.iter().enumerate() {
            let col = columns.get(idx).cloned().unwrap_or_default();
            let value = slice_field(line, *start, *end);
            match col.as_str() {
                "name" => name = value,
                "id" | "packageidentifier" => package_id = value,
                "version" | "installed" | "installedversion" => version = value,
                "source" => source = value,
                _ => {}
            }
        }

        if name.is_empty() && package_id.is_empty() {
            continue;
        }

        items.push(SoftwareItem {
            name,
            package_id,
            version,
            source,
        });
    }

    items
}

fn parse_tabular_upgrade_items(raw: &str) -> Vec<UpdateItem> {
    let mut items = Vec::new();
    let lines: Vec<&str> = raw.lines().collect();

    let sep_idx = lines.iter().position(|line| is_separator_line(line));
    let Some(sep_idx) = sep_idx else {
        return items;
    };
    if sep_idx == 0 || sep_idx + 1 >= lines.len() {
        return items;
    }

    let header = lines[sep_idx - 1];
    let separator = lines[sep_idx];
    let ranges = extract_column_ranges(separator);
    if ranges.is_empty() {
        return items;
    }

    let columns: Vec<String> = ranges
        .iter()
        .map(|(start, end)| slice_field(header, *start, *end).to_ascii_lowercase())
        .collect();

    for line in lines.iter().skip(sep_idx + 1) {
        if line.trim().is_empty() {
            continue;
        }

        let mut name = String::new();
        let mut package_id = String::new();
        let mut installed_version = String::new();
        let mut available_version = String::new();
        let mut source = String::new();

        for (idx, (start, end)) in ranges.iter().enumerate() {
            let col = columns.get(idx).cloned().unwrap_or_default();
            let value = slice_field(line, *start, *end);
            match col.as_str() {
                "name" => name = value,
                "id" | "packageidentifier" => package_id = value,
                "version" | "installed" | "installedversion" => installed_version = value,
                "available" | "availableversion" => available_version = value,
                "source" => source = value,
                _ => {}
            }
        }

        if name.is_empty() && package_id.is_empty() {
            continue;
        }

        items.push(UpdateItem {
            name,
            package_id,
            installed_version,
            available_version,
            source,
        });
    }

    items
}

fn is_separator_line(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty() && trimmed.chars().all(|c| c == '-' || c == ' ')
}

fn extract_column_ranges(separator: &str) -> Vec<(usize, usize)> {
    let bytes = separator.as_bytes();
    let mut ranges = Vec::new();
    let mut idx = 0usize;

    while idx < bytes.len() {
        while idx < bytes.len() && bytes[idx] == b' ' {
            idx += 1;
        }
        if idx >= bytes.len() {
            break;
        }

        let start = idx;
        while idx < bytes.len() && bytes[idx] == b'-' {
            idx += 1;
        }
        let end = idx;
        if end > start {
            ranges.push((start, end));
        }
    }

    ranges
}

fn slice_field(line: &str, start: usize, end: usize) -> String {
    let bytes = line.as_bytes();
    if start >= bytes.len() {
        return String::new();
    }
    let end_bound = end.min(bytes.len());
    if end_bound <= start {
        return String::new();
    }
    String::from_utf8_lossy(&bytes[start..end_bound])
        .trim()
        .to_string()
}

fn package_to_software_item(pkg: &Value) -> Option<SoftwareItem> {
    let name = pick_string(pkg, &["Name", "PackageName"]);
    let package_id = pick_string(pkg, &["Id", "PackageIdentifier"]);
    let version = pick_string(pkg, &["Version", "InstalledVersion"]);
    let source = pick_string(pkg, &["Source", "Repository", "SourceIdentifier"]);

    if name.is_empty() && package_id.is_empty() {
        return None;
    }

    Some(SoftwareItem {
        name,
        package_id,
        version,
        source,
    })
}

fn package_to_upgrade_item(pkg: &Value) -> Option<UpdateItem> {
    let name = pick_string(pkg, &["Name", "PackageName"]);
    let package_id = pick_string(pkg, &["Id", "PackageIdentifier"]);
    let installed = pick_string(pkg, &["Version", "InstalledVersion"]);
    let available = pick_string(pkg, &["Available", "AvailableVersion"]);
    let source = pick_string(pkg, &["Source", "Repository", "SourceIdentifier"]);

    if name.is_empty() && package_id.is_empty() {
        return None;
    }

    Some(UpdateItem {
        name,
        package_id,
        installed_version: installed,
        available_version: available,
        source,
    })
}

fn pick_string(value: &Value, keys: &[&str]) -> String {
    for key in keys {
        if let Some(v) = value.get(*key).and_then(Value::as_str) {
            return v.to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::{
        parse_software_items, parse_tabular_software_items, parse_tabular_upgrade_items,
        parse_upgrade_items,
    };

    #[test]
    fn parse_sources_packages_shape() {
        let payload = serde_json::json!({
            "Sources": [
                {
                    "Packages": [
                        {
                            "PackageName": "Git",
                            "PackageIdentifier": "Git.Git",
                            "InstalledVersion": "2.45.0",
                            "Source": "winget"
                        }
                    ]
                }
            ]
        });

        let items = parse_software_items(&payload);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Git");
        assert_eq!(items[0].package_id, "Git.Git");
        assert_eq!(items[0].version, "2.45.0");
        assert_eq!(items[0].source, "winget");
    }

    #[test]
    fn parse_data_shape() {
        let payload = serde_json::json!({
            "Data": [
                {
                    "Name": "Python",
                    "Id": "Python.Python.3.12",
                    "Version": "3.12.0",
                    "Source": "winget"
                }
            ]
        });

        let items = parse_software_items(&payload);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Python");
        assert_eq!(items[0].package_id, "Python.Python.3.12");
    }

    #[test]
    fn parse_tabular_shape() {
        let raw = "Name              Id                  Version   Source\n----------------  ------------------  --------  ------\nGit               Git.Git             2.45.0    winget\nPowerShell 7-x64  Microsoft.PowerShell 7.4.0    winget\n";

        let items = parse_tabular_software_items(raw);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "Git");
        assert_eq!(items[0].package_id, "Git.Git");
        assert_eq!(items[1].name, "PowerShell 7-x64");
    }

    #[test]
    fn parse_upgrade_data_shape() {
        let payload = serde_json::json!({
            "Data": [
                {
                    "Name": "Git",
                    "Id": "Git.Git",
                    "Version": "2.44.0",
                    "AvailableVersion": "2.45.0",
                    "Source": "winget"
                }
            ]
        });

        let items = parse_upgrade_items(&payload);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Git");
        assert_eq!(items[0].package_id, "Git.Git");
        assert_eq!(items[0].installed_version, "2.44.0");
        assert_eq!(items[0].available_version, "2.45.0");
    }

    #[test]
    fn parse_upgrade_tabular_shape() {
        let raw = "Name  Id       Version  Available  Source\n----  -------  -------  ---------  ------\nGit   Git.Git  2.44.0   2.45.0     winget\n";

        let items = parse_tabular_upgrade_items(raw);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Git");
        assert_eq!(items[0].package_id, "Git.Git");
        assert_eq!(items[0].installed_version, "2.44.0");
        assert_eq!(items[0].available_version, "2.45.0");
    }
}
