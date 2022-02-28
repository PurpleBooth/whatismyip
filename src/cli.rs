use clap::Parser;

#[derive(Parser, Debug, Copy, Clone)]
#[clap(author, version, about)]
pub struct Args {
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
