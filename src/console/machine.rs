use crate::{Info, Result};

pub fn print_machine(info: &Info) -> Result<()> {
    let machine = &info.machine;
    print!("{machine} ");
    Ok(())
}
