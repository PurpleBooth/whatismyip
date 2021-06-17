use anyhow::anyhow;
use anyhow::Result;
use std::net::IpAddr;
use trust_dns_resolver::config::{
    LookupIpStrategy, NameServerConfigGroup, ResolverConfig, ResolverOpts,
};
use trust_dns_resolver::Resolver;

use std::time::Duration;

mod cli;

fn main() -> Result<()> {
    cli::app().get_matches();

    let ns_ip = resolver_ip("ns1.google.com")?;
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
        resolver_opts(),
    )?)
}

fn resolver_opts() -> ResolverOpts {
    ResolverOpts {
        ndots: 1,
        timeout: Duration::from_secs(5),
        attempts: 2,
        rotate: false,
        check_names: true,
        edns0: false,
        validate: false,
        ip_strategy: LookupIpStrategy::Ipv4AndIpv6,
        cache_size: 32,
        use_hosts_file: true,
        positive_min_ttl: None,
        negative_min_ttl: None,
        positive_max_ttl: None,
        negative_max_ttl: None,
        num_concurrent_reqs: 2,
        preserve_intermediates: false,
    }
}

fn resolver_ip(ns_host: &str) -> Result<IpAddr> {
    Resolver::default()?
        .lookup_ip(ns_host)?
        .iter()
        .next()
        .ok_or_else(|| anyhow!("Nameserver ip not found"))
}
