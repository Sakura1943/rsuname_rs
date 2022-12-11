use crate::{Info, Result};

pub fn print_kernel_verison(info: &Info) -> Result<()> {
    let kernel_version = &info.kernel_version;
    print!("{kernel_version} ");
    Ok(())
}
