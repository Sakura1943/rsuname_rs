use clap::{Command, CommandFactory, Parser};

#[derive(Parser)]
#[command(
    version,
    author,
    about,
    long_about = "Print certain system information.  With no OPTION, same as -s."
)]
pub struct Cli {
    #[arg(
        short,
        long,
        help = "print all information, in the following order, except omit -p and -i if unknown:"
    )]
    pub all: bool,
    #[arg(short = 's', long, help = "print the kernel name")]
    pub kernel_name: bool,
    #[arg(short, long = "nodename", help = "print the network node hostname")]
    pub node_name: bool,
    #[arg(short = 'r', long, help = "print the kernel release")]
    pub kernel_release: bool,
    #[arg(short = 'v', long, help = "print the kernel version")]
    pub kernel_version: bool,
    #[arg(short, long, help = "print the machine hardware name")]
    pub machine: bool,
    #[arg(short, long, help = "print the processor type (non-portable)")]
    pub processor: bool,
    #[arg(short = 'i', long, help = "print the hardware platform (non-portable)")]
    pub hardware_platform: bool,
    #[arg(short, long, help = "print the operating system")]
    pub operating_system: bool,
}

impl Cli {
    #[allow(dead_code)]
    pub fn build() -> Self {
        Self::parse()
    }
    pub fn cmd() -> Command {
        Self::command()
    }
}
