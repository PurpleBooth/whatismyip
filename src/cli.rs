use clap::Parser;

#[allow(
    clippy::struct_excessive_bools,
    reason = "This is the interface for the cli"
)]
#[derive(Parser, Debug, Copy, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// Only print IP addresses local to this machine
    #[clap(short = 'l', long = "only-local", conflicts_with = "only_wan")]
    pub only_local: bool,
    /// Only print IP addresses as seen by a remote service
    #[clap(short = 'w', long = "only-wan", conflicts_with = "only_local")]
    pub only_wan: bool,
    /// Only print IPv4 addresses
    #[clap(short = '4', long = "only-4", conflicts_with = "only_6")]
    pub only_4: bool,
    /// Only print IPv6 addresses
    #[clap(short = '6', long = "only-6", conflicts_with = "only_4")]
    pub only_6: bool,
    /// Print the reverse DNS entries for the IP addresses
    #[clap(short = 'r', long = "reverse")]
    pub reverse: bool,
}
