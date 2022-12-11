use crate::{Info, Result};

pub fn print_operating_system(info: &Info) -> Result<()> {
    let operating_system = &info.operating_system;
    print!("{operating_system}");
    Ok(())
}
