use crate::{Info, Result};

pub fn print_all(
    Info {
        kernel_name,
        node_name,
        kernel_version,
        release,
        machine,
        operating_system,
        processor,
        hardware_platform,
    }: &Info,
) -> Result<()> {
    if processor == &"unknown".to_owned() {
        println!("{kernel_name} {node_name} {release} {kernel_version} {machine} {hardware_platform} {operating_system}");
    } else if hardware_platform == &"unknown".to_owned() {
        println!("{kernel_name} {node_name} {release} {kernel_version} {machine} {processor} {operating_system}");
    } else if hardware_platform == &"unknown".to_owned() && processor == &"unknown".to_owned() {
        println!(
            "{kernel_name} {node_name} {release} {kernel_version} {machine} {operating_system}"
        );
    } else {
        println!("{kernel_name} {node_name} {release} {kernel_version} {machine} {processor} {hardware_platform} {operating_system}");
    }
    Ok(())
}
