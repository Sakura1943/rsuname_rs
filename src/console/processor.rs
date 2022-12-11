use crate::{Info, Result};

pub fn print_processor(info: &Info) -> Result<()> {
    let processor = &info.processor;
    print!("{processor} ");
    Ok(())
}
