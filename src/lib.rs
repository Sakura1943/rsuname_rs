mod console;
pub use console::*;
pub mod cli;
pub use anyhow::{anyhow, Result};
use nix::sys::utsname::uname;

#[derive(Debug, Clone, Default)]
pub struct Info {
    pub kernel_name: String,
    pub node_name: String,
    pub release: String,
    pub machine: String,
    pub processor: String,
    pub kernel_version: String,
    pub hardware_platform: String,
    pub operating_system: String,
}

impl Info {
    pub fn new() -> Result<Self> {
        let uname = uname()?;
        let Some(sysname) = uname.sysname().to_str() else {
            return Err(anyhow!("Fetch sysname failed"));
        };
        let Some(nodename) = uname.nodename().to_str() else {
            return Err(anyhow!("Fetch nodename failed"))
        };
        let Some(release) = uname.release().to_str() else {
            return Err(anyhow!("Fetch release failed"))
        };
        let Some(machine) = uname.machine().to_str() else {
            return Err(anyhow!("Fetch machine failed"))
        };
        let Some(version) = uname.version().to_str() else {
            return Err(anyhow!("Fetch version failed"))
        };
        let processor = if cfg!(target_arch = "x86") {
            "x86".to_owned()
        } else if cfg!(target_arch = "x86_64") {
            "x86_64".to_owned()
        } else if cfg!(target_arch = "mips") {
            "mips".to_owned()
        } else if cfg!(target_arch = "powerpc") {
            "powerpc".to_owned()
        } else if cfg!(target_arch = "powerpc64") {
            "powerpc64".to_owned()
        } else if cfg!(target_arch = "arm") {
            "arm".to_owned()
        } else if cfg!(target_arch = "aarch64") {
            "aarch64".to_owned()
        } else {
            "unknown".to_owned()
        };

        let hardware_platform = if cfg!(target_vendor = "apple") {
            "apple".to_owned()
        } else if cfg!(target_vendor = "fortanix") {
            "fortanix".to_owned()
        } else if cfg!(target_vendor = "pc") {
            "pc".to_owned()
        } else {
            "unknown".to_owned()
        };
        Ok(Self {
            kernel_name: sysname.to_owned(),
            node_name: nodename.to_owned(),
            release: release.to_owned(),
            machine: machine.to_owned(),
            kernel_version: version.to_owned(),
            operating_system: format!("GNU/{sysname}"),
            hardware_platform,
            processor,
        })
    }
}
