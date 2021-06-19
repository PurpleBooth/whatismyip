use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static> {
    App::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("only-4")
                .long("only-4")
                .short('4')
                .about("Only print IPv4 addresses"),
        )
        .arg(
            Arg::new("only-6")
                .long("only-6")
                .short('6')
                .about("Only print IPv6 addresses"),
        )
}
