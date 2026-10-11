use std::process::Command;
use regex::Regex;

/// Returns the volume of the default audio source (microphone)
pub fn get_input_volume() -> Result<f64, String> {
    let output = Command::new("pactl")
        .args(["get-source-volume", "@DEFAULT_SOURCE@"])
        .output()
        .map_err(|e| format!("Failed to execute pactl: {}", e))?;

    if !output.status.success() {
        return Err("Failed to get default source volume".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for part in stdout.split('/') {
        let part = part.trim();
        if part.ends_with('%') {
            let num_str = &part[..part.len() - 1];
            if let Ok(val) = num_str.parse::<f64>() {
                return Ok(val);
            }
        }
    }

    Err("Could not parse volume from pactl output".to_string())
}

/// Sets the volume of the default input source (expects 0 to 100)
pub fn set_input_volume(val: u8) -> Result<(), String> {
    let vol_arg = format!("{}%", val);
    let status = Command::new("pactl")
        .args(["set-source-volume", "@DEFAULT_SOURCE@", &vol_arg])
        .status()
        .map_err(|e| format!("Failed to execute pactl: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("pactl exited with non-zero status: {}", status))
    }
}

/// Returns the name of the currently selected default audio source
pub fn get_selected_input_device() -> Result<String, String> {
    let output = Command::new("pactl")
        .args(["get-default-source"])
        .output()
        .map_err(|e| format!("Failed to execute pactl: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err("Failed to get default source".to_string())
    }
}

/// Returns a list of available audio source devices (microphones)
pub fn get_input_device_list() -> Result<Vec<String>, String> {
    let output = Command::new("pactl")
        .args(["-f", "json", "list", "sources"])
        .output();

    let sources = if let Ok(out) = output {
        if out.status.success() {
            parse_pactl_sources(&String::from_utf8_lossy(&out.stdout))
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    Ok(sources)
}

/// Sets the default active input device by its name string
pub fn set_selected_input_device(name: &str) -> Result<(), String> {
    let status = Command::new("pactl")
        .arg("set-default-source")
        .arg(name)
        .status()
        .map_err(|e| format!("Failed to execute pactl: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("pactl exited with non-zero status: {}", status))
    }
}

fn parse_pactl_sources(raw: &str) -> Vec<String> {
    let mut names = Vec::new();
    let re = Regex::new(r#""name"\s*:\s*"([^"]+)""#).unwrap();
    for mat in re.captures_iter(raw) {
        if let Some(group) = mat.get(1) {
            let name = group.as_str();

            names.push(name.to_string());
        }
    }
    names
}