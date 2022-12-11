use crate::{Info, Result};

pub fn print_kernel_release(info: &Info) -> Result<()> {
    let kernel_release = &info.release;
    print!("{kernel_release} ");
    Ok(())
}
