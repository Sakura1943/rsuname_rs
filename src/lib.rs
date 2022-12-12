mod console;
pub use console::*;
pub mod cli;
pub use anyhow::{anyhow, Result};
use nix::sys::utsname::uname;

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

/* Target processor */
#[cfg(target_arch = "x86")]
const PROCESSOR: &str = "x86";
#[cfg(all(target_arch = "x86_64", not(target_os = "linux")))]
const PROCESSOR: &str = "x86_64";
#[cfg(target_arch = "mips")]
const PROCESSOR: &str = "mips";
#[cfg(target_arch = "powerpc")]
const PROCESSOR: &str = "powerpc";
#[cfg(target_arch = "powerpc64")]
const PROCESSOR: &str = "powerpc64";
#[cfg(target_arch = "arm")]
const PROCESSOR: &str = "arm";
#[cfg(target_arch = "aarch64")]
const PROCESSOR: &str = "aarch64";

/* Hardware platform */
#[cfg(target_vendor = "apple")]
const HARDWARE_PLATFORM: &str = "apple";
#[cfg(target_vendor = "fortanix")]
const HARDWARE_PLATFORM: &str = "fortanix";
#[cfg(target_vendor = "pc")]
const HARDWARE_PLATFORM: &str = "pc";
#[cfg(target_os = "linux")]
const HARDWARE_PLATFORM: &str = "unknown";
const PROCESSOR: &str = "unknown";

/* Operating system */
#[cfg(all(target_os = "linux", any(target_env = "gnu", target_env = "")))]
const OPERATING_SYSTEM: &str = "GNU/Linux";
#[cfg(all(target_os = "linux", not(any(target_env = "gnu", target_env = ""))))]
const OPERATING_SYSTEM: &str = "Linux";
#[cfg(target_os = "android")]
const OPERATING_SYSTEM: &str = "Android";
#[cfg(target_os = "freebsd")]
const OPERATING_SYSTEM: &str = "FreeBSD";
#[cfg(target_os = "netbsd")]
const OPERATING_SYSTEM: &str = "NetBSD";
#[cfg(target_os = "openbsd")]
const OPERATING_SYSTEM: &str = "OpenBSD";
#[cfg(target_os = "fuchsia")]
const OPERATING_SYSTEM: &str = "Fuchsia";
#[cfg(target_os = "redox")]
const OPERATING_SYSTEM: &str = "Redox";

impl Info {
    pub fn new() -> Result<Self> {
        let uname = uname()?;
        let Some(sysname) = uname.sysname().to_str() else {
            return Err(anyhow!("Failed to get sysname"));
        };
        let Some(nodename) = uname.nodename().to_str() else {
            return Err(anyhow!("Failed to get nodename"))
        };
        let Some(release) = uname.release().to_str() else {
            return Err(anyhow!("Failed to get release"))
        };
        let Some(machine) = uname.machine().to_str() else {
            return Err(anyhow!("Failed to get machine"))
        };
        let Some(version) = uname.version().to_str() else {
            return Err(anyhow!("Failed to get version"))
        };

        Ok(Self {
            kernel_name: sysname.to_owned(),
            node_name: nodename.to_owned(),
            release: release.to_owned(),
            machine: machine.to_owned(),
            kernel_version: version.to_owned(),
            operating_system: OPERATING_SYSTEM.to_owned(),
            hardware_platform: HARDWARE_PLATFORM.to_owned(),
            processor: PROCESSOR.to_owned(),
        })
    }
}
