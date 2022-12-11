use colored::*;
use rsuname::Result;
use rsuname::{
    all::print_all, cli::Cli, hardware_platform::print_hardware_platform,
    kernel_name::print_kernel_name, kernel_release::print_kernel_release,
    kernel_version::print_kernel_verison, machine::print_machine, nodename::print_nodename,
    operating_system::print_operating_system, processor::print_processor, Info,
};

#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "ios"))]
#[cfg(not(target_os = "macos"))]
#[cfg(not(target_os = "dragonfly"))]

fn main() -> Result<()> {
    let info = Info::new()?;

    let client = Cli::build();
    match client.all {
        true => print_all(&info)?,
        false => {
            if !client.kernel_name
                && !client.kernel_release
                && !client.kernel_version
                && !client.machine
                && !client.node_name
                && !client.operating_system
            {
                return Ok(());
            }
        }
    }
    if client.kernel_name {
        print_kernel_name(&info)?;
    }
    if client.node_name {
        print_nodename(&info)?;
    }
    if client.kernel_release {
        print_kernel_release(&info)?;
    }
    if client.kernel_version {
        print_kernel_verison(&info)?;
    }
    if client.machine {
        print_machine(&info)?;
    }
    if client.processor {
        if info.processor != "unknown".to_owned() {
            print_processor(&info)?;
        } else {
            eprintln!("{}", "Failed to get processor\n".bright_red());
        }
    }
    if client.hardware_platform {
        if info.hardware_platform != "unknown".to_owned() {
            print_hardware_platform(&info)?;
        } else {
            eprintln!("{}", "Failed to get hardware platform\n".bright_red());
        }
    }
    if client.operating_system {
        print_operating_system(&info)?;
    }
    println!("");
    Ok(())
}
