use blight::Light;
use crate::utils::ErrAsString;

pub fn get_brightness() -> Result<(u32, u32), String> {
    let dev = blight::Device::new(None).e()?;
    let val = dev.current();
    let max = dev.max();
    Ok((val, max))
}

pub fn set_brightness(val: u32) -> Result<(), String> {
    let mut dev = blight::Device::new(None).e()?;
    dev.write_value(val).e()
}