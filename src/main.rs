//! A little utility to work out the public IP address of a machine
#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
use clap::Parser;
use std::net::IpAddr;

use std::str::FromStr;

use crate::myip::{MyIp, ReversedIp};
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use futures::future::join_all;
use futures::{stream, StreamExt};
use trust_dns_resolver::config::{
    LookupIpStrategy, NameServerConfigGroup, ResolverConfig, ResolverOpts,
};
use trust_dns_resolver::{AsyncResolver, TokioAsyncResolver};

mod cli;
mod myip;

type MyIps = Vec<myip::MyIp>;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let mut strategies = vec![];
    if !args.only_6 {
        strategies.push(find_ip(LookupIpStrategy::Ipv4Only));
    }
    if !args.only_4 {
        strategies.push(find_ip(LookupIpStrategy::Ipv6Only));
    }

    let (ok, failures): (Vec<Result<MyIps>>, Vec<Result<MyIps>>) = join_all(strategies)
        .await
        .into_iter()
        .partition(std::result::Result::is_ok);

    if ok.is_empty() {
        bail!("Failed: {:?}", failures,);
    }

    println!(
        "{}",
        stream::iter(ok.iter().flatten().flatten().cloned().collect::<Vec<_>>())
            .then(|my_ip| {
                async move {
                    if args.reverse {
                        reverse_ip(&my_ip.clone()).await.map_or_else(
                            || my_ip.clone(),
                            |reversed_ip| MyIp::new_reversed(my_ip.ip(), ReversedIp(reversed_ip)),
                        )
                    } else {
                        my_ip.clone()
                    }
                }
            })
            .map(|ip| format!("{}", ip))
            .collect::<Vec<_>>()
            .await
            .join("\n")
    );

    Ok(())
}

async fn find_ip(strategy: LookupIpStrategy) -> Result<MyIps> {
    let ns_ip = tokio::select! {
        ns_ip = async { resolver_ip("ns1.google.com", strategy).await } => ns_ip,
        ns_ip = async { resolver_ip("ns2.google.com", strategy).await } => ns_ip,
        ns_ip = async { resolver_ip("ns3.google.com", strategy).await } => ns_ip,
        ns_ip = async { resolver_ip("ns4.google.com", strategy).await } => ns_ip,
    }?;

    let dns_resolver = resolver(ns_ip, LookupIpStrategy::Ipv4Only)?;
    user_ips(&dns_resolver).await
}

async fn user_ips(resolver: &TokioAsyncResolver) -> Result<MyIps> {
    Ok(resolver
        .txt_lookup("o-o.myaddr.l.google.com")
        .await?
        .iter()
        .map(std::string::ToString::to_string)
        .flat_map(|possible_ip| IpAddr::from_str(&possible_ip))
        .map(myip::MyIp::new_plain)
        .collect())
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
    let mut opts = ResolverOpts::default();
    opts.ip_strategy = ip_strategy;
    opts
}

async fn resolver_ip(ns_host: &str, ip_strategy: LookupIpStrategy) -> Result<IpAddr> {
    AsyncResolver::tokio(ResolverConfig::default(), resolver_opts(ip_strategy))?
        .lookup_ip(ns_host)
        .await?
        .iter()
        .next()
        .ok_or_else(|| anyhow!("Nameserver ip not found"))
}

async fn reverse_ip(ip: &MyIp) -> Option<String> {
    AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
        .ok()?
        .reverse_lookup(ip.ip())
        .await
        .ok()?
        .iter()
        .map(std::string::ToString::to_string)
        .next()
}
