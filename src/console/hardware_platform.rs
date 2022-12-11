use crate::{Info, Result};

pub fn print_hardware_platform(info: &Info) -> Result<()> {
    let hardware_platform = &info.hardware_platform;
    print!("{hardware_platform} ");
    Ok(())
}
