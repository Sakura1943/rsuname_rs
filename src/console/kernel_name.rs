use crate::{Info, Result};

pub fn print_kernel_name(info: &Info) -> Result<()> {
    let kernel_name = &info.kernel_name;
    print!("{kernel_name} ");
    Ok(())
}
