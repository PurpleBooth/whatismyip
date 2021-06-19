use std::net::IpAddr;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use futures::future::join_all;
use trust_dns_resolver::config::{
    LookupIpStrategy, NameServerConfigGroup, ResolverConfig, ResolverOpts,
};
use trust_dns_resolver::{AsyncResolver, TokioAsyncResolver};

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::app().get_matches();

    let mut strategies = vec![];
    if !matches.is_present("only-6") {
        strategies.push(find_ip(LookupIpStrategy::Ipv4Only));
    }
    if !matches.is_present("only-4") {
        strategies.push(find_ip(LookupIpStrategy::Ipv6Only));
    }

    let (ok, failures): (Vec<Result<String>>, Vec<Result<String>>) = join_all(strategies)
        .await
        .into_iter()
        .partition(std::result::Result::is_ok);

    if ok.is_empty() {
        bail!("Failed: {:?}", failures,);
    }

    println!(
        "{}",
        ok.iter().flatten().cloned().collect::<Vec<_>>().join("\n")
    );

    Ok(())
}

async fn find_ip(strategy: LookupIpStrategy) -> Result<String> {
    let ns_ip = tokio::select! {
        ns_ip = async { resolver_ip("ns1.google.com", strategy).await } => ns_ip,
        ns_ip = async { resolver_ip("ns2.google.com", strategy).await } => ns_ip,
        ns_ip = async { resolver_ip("ns3.google.com", strategy).await } => ns_ip,
        ns_ip = async { resolver_ip("ns4.google.com", strategy).await } => ns_ip,
    }?;

    let dns_resolver = resolver(ns_ip, LookupIpStrategy::Ipv4Only)?;
    user_ips(&dns_resolver).await
}

async fn user_ips(resolver: &TokioAsyncResolver) -> Result<String> {
    Ok(resolver
        .txt_lookup("o-o.myaddr.l.google.com")
        .await?
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n"))
}

fn resolver(ip: IpAddr, ip_strategy: LookupIpStrategy) -> Result<TokioAsyncResolver> {
    Ok(AsyncResolver::tokio(
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(&[ip], 53, true),
        ),
        resolver_opts(ip_strategy),
    )?)
}

fn resolver_opts(ip_strategy: LookupIpStrategy) -> ResolverOpts {
    ResolverOpts {
        ndots: 1,
        timeout: Duration::from_secs(5),
        attempts: 2,
        rotate: false,
        check_names: true,
        edns0: false,
        validate: false,
        ip_strategy,
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

async fn resolver_ip(ns_host: &str, ip_strategy: LookupIpStrategy) -> Result<IpAddr> {
    AsyncResolver::tokio(ResolverConfig::default(), resolver_opts(ip_strategy))?
        .lookup_ip(ns_host)
        .await?
        .iter()
        .next()
        .ok_or_else(|| anyhow!("Nameserver ip not found"))
}
