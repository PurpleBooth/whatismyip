//! A little utility to work out the IP address of a machine
#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::missing_assert_message,
    clippy::todo,
    clippy::allow_attributes_without_reason,
    clippy::panic,
    clippy::panicking_unwrap,
    clippy::panic_in_result_fn
)]
#![deny(warnings)]
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::multiple_crate_versions)]

use clap::Parser;
use local_ip_address::list_afinet_netifas;
use std::collections::HashSet;
use std::net::IpAddr;
use std::str::FromStr;

use crate::cli::Args;
use crate::myip::{MyIp, ReversedIp};
use crate::IpVersion::{Ipv4, Ipv6};
use futures::future::join_all;
use futures::{stream, StreamExt};
use hickory_resolver::config::{
    LookupIpStrategy, NameServerConfigGroup, ResolverConfig, ResolverOpts,
};
use hickory_resolver::{AsyncResolver, TokioAsyncResolver};
use miette::{bail, miette, set_panic_hook, IntoDiagnostic, Result};

mod cli;
mod myip;

type MyIps = Vec<MyIp>;

const GOOGLE_NS1: &str = "ns1.google.com";
const GOOGLE_NS2: &str = "ns2.google.com";
const GOOGLE_NS3: &str = "ns3.google.com";
const GOOGLE_NS4: &str = "ns4.google.com";
const MYADDR_RECORD: &str = "o-o.myaddr.l.google.com";

#[derive(Copy, Debug, Clone)]
enum IpVersion {
    Ipv4,
    Ipv6,
}
#[tokio::main]
async fn main() -> Result<()> {
    set_panic_hook();
    let args = cli::Args::parse();

    let mut strategies = vec![];

    if !args.only_local {
        if !args.only_6 && !args.only_local {
            strategies.push(find_wan_ip(IpVersion::Ipv4));
        }
        if !args.only_4 && !args.only_local {
            strategies.push(find_wan_ip(IpVersion::Ipv6));
        }
    }

    let (ok, failures): (Vec<Result<MyIps>>, Vec<Result<MyIps>>) = join_all(strategies)
        .await
        .into_iter()
        .chain(match args {
            Args {
                only_wan: false,
                only_6: false,
                only_4: true,
                ..
            } => vec![Ok(find_local_ip(Some(Ipv4)))],
            Args {
                only_wan: false,
                only_6: true,
                only_4: false,
                ..
            } => vec![Ok(find_local_ip(Some(Ipv6)))],
            Args {
                only_wan: false,
                only_6: false,
                only_4: false,
                ..
            } => vec![Ok(find_local_ip(None))],
            _ => vec![],
        })
        .partition(std::result::Result::is_ok);

    if ok.is_empty() {
        bail!("Failed: {:?}", failures,);
    }

    let resolution_result = stream::iter(ok.iter().flatten().flatten().cloned())
        .then(|my_ip| async move {
            if args.reverse {
                reverse_ip(&my_ip.clone()).await.map_or_else(
                    || my_ip.clone(),
                    |reversed_ip| MyIp::new_reversed(my_ip.ip(), reversed_ip),
                )
            } else {
                my_ip.clone()
            }
        })
        .map(|ip| format!("{ip}"))
        .collect::<HashSet<String>>()
        .await
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n");
    println!("{resolution_result}");

    Ok(())
}

#[allow(clippy::redundant_pub_crate)]
async fn find_wan_ip(strategy: IpVersion) -> Result<MyIps> {
    let lookup_ip_strategy = match strategy {
        Ipv4 => LookupIpStrategy::Ipv4Only,
        Ipv6 => LookupIpStrategy::Ipv6Only,
    };
    let ns_ip = tokio::select! {
        ns_ip = async {
            resolver_ip(
                GOOGLE_NS1,
                lookup_ip_strategy
            ).await
        } => ns_ip,
        ns_ip = async {
            resolver_ip(
                GOOGLE_NS2,
                lookup_ip_strategy
            ).await
        } => ns_ip,
        ns_ip = async {
            resolver_ip(
                GOOGLE_NS3,
                lookup_ip_strategy
            ).await
        } => ns_ip,
        ns_ip = async {
            resolver_ip(
                GOOGLE_NS4,
                lookup_ip_strategy
            ).await
        } => ns_ip,
    }?;

    let dns_resolver = resolver(ns_ip, LookupIpStrategy::Ipv4Only);
    user_ips(&dns_resolver).await
}

fn find_local_ip(strategy: Option<IpVersion>) -> MyIps {
    list_afinet_netifas()
        .into_iter()
        .flatten()
        .filter_map(match strategy {
            Some(Ipv4) => |(_, ip): (String, IpAddr)| if ip.is_ipv6() { None } else { Some(ip) },
            Some(Ipv6) => |(_, ip): (String, IpAddr)| if ip.is_ipv6() { Some(ip) } else { None },
            None => |(_, ip): (String, IpAddr)| Some(ip),
        })
        .map(MyIp::new_plain)
        .collect::<Vec<_>>()
}

async fn user_ips(resolver: &TokioAsyncResolver) -> Result<MyIps> {
    Ok(resolver
        .txt_lookup(MYADDR_RECORD)
        .await
        .into_diagnostic()?
        .iter()
        .map(ToString::to_string)
        .flat_map(|possible_ip| IpAddr::from_str(&possible_ip))
        .map(MyIp::new_plain)
        .collect())
}

fn resolver(ip: IpAddr, ip_strategy: LookupIpStrategy) -> TokioAsyncResolver {
    AsyncResolver::tokio(
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(&[ip], 53, true),
        ),
        resolver_opts(ip_strategy),
    )
}

fn resolver_opts(ip_strategy: LookupIpStrategy) -> ResolverOpts {
    let mut opts = ResolverOpts::default();
    opts.ip_strategy = ip_strategy;
    opts
}

async fn resolver_ip(ns_host: &str, ip_strategy: LookupIpStrategy) -> Result<IpAddr> {
    AsyncResolver::tokio(ResolverConfig::default(), resolver_opts(ip_strategy))
        .lookup_ip(ns_host)
        .await
        .into_diagnostic()?
        .iter()
        .next()
        .ok_or_else(|| miette!("Nameserver ip not found"))
}

async fn reverse_ip(ip: &MyIp) -> Option<ReversedIp> {
    AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
        .reverse_lookup(ip.ip())
        .await
        .ok()?
        .iter()
        .map(ToString::to_string)
        .map(ReversedIp::from)
        .next()
}
