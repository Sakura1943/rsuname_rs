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
    if !client.kernel_name
        && !client.all
        && !client.hardware_platform
        && !client.kernel_version
        && !client.machine
        && !client.node_name
        && !client.operating_system
        && !client.processor
    {
        print_kernel_name(&info)?;
    }
    match client.all {
        true => {
            print_all(&info)?;
            return Ok(());
        }
        false => {
            if !client.kernel_name
                && !client.kernel_release
                && !client.kernel_version
                && !client.machine
                && !client.node_name
                && !client.operating_system
                && !client.processor
                && !client.hardware_platform
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
    match (client.processor, client.hardware_platform) {
        (true, true) => {
            if info.processor != "unknown" && info.hardware_platform != "unknown" {
                print_processor(&info)?;
                print_hardware_platform(&info)?;
                print!("\n");
            } else {
                if !client.all
                    && !client.kernel_name
                    && !client.kernel_release
                    && !client.kernel_version
                    && !client.machine
                    && !client.node_name
                    && !client.operating_system
                {
                    print_processor(&info)?;
                    print_hardware_platform(&info)?;
                    return Ok(());
                }
            }
        }
        (true, false) => {
            if info.processor != "unknown" && info.hardware_platform != "unknown" {
                print_processor(&info)?;
                print!("\n");
            } else {
                if !client.all
                    && !client.kernel_name
                    && !client.kernel_release
                    && !client.kernel_version
                    && !client.machine
                    && !client.node_name
                    && !client.operating_system
                {
                    print_processor(&info)?;
                    return Ok(());
                }
            }
        }
        (false, true) => {
            if info.processor != "unknown" && info.hardware_platform != "unknown" {
                print_hardware_platform(&info)?;
                print!("\n");
            } else {
                if !client.all
                    && !client.kernel_name
                    && !client.kernel_release
                    && !client.kernel_version
                    && !client.machine
                    && !client.node_name
                    && !client.operating_system
                {
                    print_hardware_platform(&info)?;
                    return Ok(());
                }
            }
        }
        _ => {}
    }
    if client.operating_system {
        print_operating_system(&info)?;
    }
    println!("");
    Ok(())
}
