use clap::{crate_authors, crate_version, Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("only-4")
                .long("only-4")
                .short('4')
                .help("Only print IPv4 addresses")
                .conflicts_with("only-6"),
        )
        .arg(
            Arg::new("only-6")
                .long("only-6")
                .short('6')
                .help("Only print IPv6 addresses")
                .conflicts_with("only-4"),
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .short('r')
                .help("Print the reverse DNS entries for the IP addresses"),
        )
}
