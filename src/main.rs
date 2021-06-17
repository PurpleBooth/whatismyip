use anyhow::anyhow;
use anyhow::Result;
use std::net::IpAddr;
use trust_dns_resolver::config::{NameServerConfigGroup, ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

mod cli;

fn main() -> Result<()> {
    cli::app().get_matches();

    let ns_ip = resolver_ip()?;
    let resolver = resolver(ns_ip)?;
    let user_ips = user_ips(&resolver)?;
    println!("{}", user_ips);

    Ok(())
}

fn user_ips(resolver: &Resolver) -> Result<String> {
    Ok(resolver
        .txt_lookup("o-o.myaddr.l.google.com")?
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n"))
}

fn resolver(ip: IpAddr) -> Result<Resolver> {
    Ok(Resolver::new(
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(&[ip], 53, true),
        ),
        ResolverOpts::default(),
    )?)
}

fn resolver_ip() -> Result<IpAddr> {
    Resolver::default()?
        .lookup_ip("ns1.google.com")?
        .iter()
        .next()
        .ok_or_else(|| anyhow!("Nameserver ip not found"))
}
