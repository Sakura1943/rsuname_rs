use crate::{Info, Result};

pub fn print_nodename(info: &Info) -> Result<()> {
    let node_name = &info.node_name;
    print!("{node_name} ");
    Ok(())
}
