use crate::{Info, Result};

#[allow(unused_variables)]
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
    #[cfg(not(target_os = "linux"))]
    println!("{kernel_name} {node_name} {release} {kernel_version} {machine} {processor} {hardware_platform} {operating_system}");
    #[cfg(target_os = "linux")]
    println!("{kernel_name} {node_name} {release} {kernel_version} {machine} {operating_system}");
    Ok(())
}
