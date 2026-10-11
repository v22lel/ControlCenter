use std::process::Command;
use volumecontrol_core::AudioDevice;
use crate::utils::ErrAsString;

/// Returns the volume (0 to 1090) of the default audio sink
pub fn get_volume() -> Result<f64, String> {
    let dev = volumecontrol_linux::AudioDevice::from_default().e()?;
    dev.get_vol().e().map(|v| v as f64)

}

/// Sets the volume of the default sink (expects a value between 0 and 100)
pub fn set_volume(val: u8) -> Result<(), String> {
    let dev = volumecontrol_linux::AudioDevice::from_default().e()?;
    dev.set_vol((val as f64) as u8).e()
}

/// Returns the name of the currently selected default audio sink
pub fn get_selected_device() -> Result<String, String> {
    let dev = volumecontrol_linux::AudioDevice::from_default().e()?;
    Ok(dev.name().to_string())
}

/// Returns a list of available audio sink devices
pub fn get_device_list() -> Result<Vec<String>, String> {
    let devs = volumecontrol_linux::AudioDevice::list().e()?;
    let devs = devs.into_iter().map(|i| i.name).collect();
    Ok(devs)
}

pub fn set_selected_device(name: &str) -> Result<(), String> {
    let dev = volumecontrol_linux::AudioDevice::from_name(name).e()?;
    let id: &str = dev.id();

    let status = Command::new("pactl")
        .arg("set-default-sink")
        .arg(id)
        .status()
        .map_err(|e| format!("Failed to execute wpctl: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("wpctl exited with non-zero status: {}", status))
    }
}