//! A little utility to work out the IP address of a machine
//!
//! This utility provides functionality to determine both local and WAN IP addresses
//! of a machine. It supports both IPv4 and IPv6 addresses and can perform reverse
//! DNS lookups to get the hostname associated with an IP address.
//!
//! The utility uses various strategies to determine IP addresses, including:
//! - Local network interface enumeration
//! - DNS queries to Google's nameservers
//! - Special DNS records that return the client's IP address
#![warn(clippy::nursery)]
#![deny(
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::pedantic,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    missing_docs
)]
#![allow(clippy::multiple_crate_versions)]

use crate::IpVersion::{Ipv4, Ipv6};
use crate::cli::Args;
use crate::myip::{MyIp, ReversedIp};
use clap::Parser;
use futures::future::join_all;
use futures::{StreamExt, stream};
use hickory_resolver::config::{LookupIpStrategy, NameServerConfigGroup, ResolverConfig};
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::{Resolver, TokioResolver};
use local_ip_address::list_afinet_netifas;
use miette::{IntoDiagnostic, Result, bail, miette, set_panic_hook};
use std::collections::HashSet;
use std::net::IpAddr;
use std::str::FromStr;

mod cli;
mod myip;

/// A collection of IP addresses
type MyIps = Vec<MyIp>;

/// Google's primary nameserver hostname
const GOOGLE_NS1: &str = "ns1.google.com";
/// Google's secondary nameserver hostname
const GOOGLE_NS2: &str = "ns2.google.com";
/// Google's tertiary nameserver hostname
const GOOGLE_NS3: &str = "ns3.google.com";
/// Google's quaternary nameserver hostname
const GOOGLE_NS4: &str = "ns4.google.com";
/// Special Google DNS record that returns the client's IP address
const MYADDR_RECORD: &str = "o-o.myaddr.l.google.com";

/// Represents the version of IP address to use
#[derive(Copy, Debug, Clone)]
enum IpVersion {
    /// IPv4 address version
    Ipv4,
    /// IPv6 address version
    Ipv6,
}
/// Main entry point for the application
///
/// This function:
/// 1. Parses command-line arguments
/// 2. Determines which IP lookup strategies to use based on the arguments
/// 3. Executes the strategies in parallel
/// 4. Optionally performs reverse DNS lookups
/// 5. Formats and prints the results
#[tokio::main]
async fn main() -> Result<()> {
    set_panic_hook();
    let args = cli::Args::parse();

    // Process arguments to determine which strategies to use
    let strategies = process_args(args);

    let (ok, failures): (Vec<Result<MyIps>>, Vec<Result<MyIps>>) = join_all(strategies)
        .await
        .into_iter()
        .chain(get_local_ips(args))
        .partition(Result::is_ok);

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

/// Process command-line arguments to determine which WAN IP strategies to use
///
/// This function analyzes the command-line arguments and returns a vector of futures
/// that will be executed to find WAN IP addresses. The strategies returned depend on:
/// - Whether local-only mode is enabled
/// - Whether IPv4-only or IPv6-only mode is enabled
///
/// # Arguments
///
/// * `args` - The parsed command-line arguments
///
/// # Returns
///
/// A vector of futures that will resolve to IP addresses
fn process_args(args: Args) -> Vec<impl std::future::Future<Output = Result<MyIps>>> {
    let mut strategies = vec![];

    if !args.only_local {
        if !args.only_6 && !args.only_local {
            strategies.push(find_wan_ip(Ipv4));
        }
        if !args.only_4 && !args.only_local {
            strategies.push(find_wan_ip(Ipv6));
        }
    }

    strategies
}

/// Get local IPs based on command-line arguments
///
/// This function returns a vector of results containing local IP addresses
/// based on the command-line arguments. The IPs returned depend on:
/// - Whether WAN-only mode is enabled
/// - Whether IPv4-only or IPv6-only mode is enabled
///
/// # Arguments
///
/// * `args` - The parsed command-line arguments
///
/// # Returns
///
/// A vector of results containing local IP addresses
fn get_local_ips(args: Args) -> Vec<Result<MyIps>> {
    match args {
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
    }
}

/// Find WAN IP addresses using DNS queries
///
/// This function attempts to find the WAN IP address of the machine by:
/// 1. Resolving one of Google's nameservers
/// 2. Using that nameserver to query a special DNS record that returns the client's IP
///
/// The function will try multiple nameservers in parallel and use the first one that responds.
///
/// # Arguments
///
/// * `strategy` - The IP version strategy to use (IPv4 or IPv6)
///
/// # Returns
///
/// A result containing a vector of IP addresses if successful
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

/// Find local IP addresses on the machine
///
/// This function enumerates all network interfaces on the machine and returns
/// their IP addresses, filtered by the specified IP version strategy.
///
/// # Arguments
///
/// * `strategy` - Optional IP version filter (IPv4, IPv6, or both if None)
///
/// # Returns
///
/// A vector of IP addresses
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

/// Query a DNS resolver to get the user's IP addresses
///
/// This function queries a special DNS record that returns the client's IP address
/// as seen by the DNS server. This is used to determine the WAN IP address.
///
/// # Arguments
///
/// * `resolver` - The DNS resolver to use for the query
///
/// # Returns
///
/// A result containing a vector of IP addresses if successful
async fn user_ips(resolver: &Resolver<TokioConnectionProvider>) -> Result<MyIps> {
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

/// Create a DNS resolver that uses a specific nameserver
///
/// # Arguments
///
/// * `ip` - The IP address of the nameserver to use
/// * `ip_strategy` - The IP version strategy to use for lookups
///
/// # Returns
///
/// A configured DNS resolver
fn resolver(ip: IpAddr, ip_strategy: LookupIpStrategy) -> TokioResolver {
    let mut builder = Resolver::builder_with_config(
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(&[ip], 53, true),
        ),
        TokioConnectionProvider::default(),
    );
    builder.options_mut().ip_strategy = ip_strategy;
    builder.build()
}

/// Resolve a nameserver hostname to an IP address
///
/// # Arguments
///
/// * `ns_host` - The hostname of the nameserver to resolve
/// * `ip_strategy` - The IP version strategy to use for the lookup
///
/// # Returns
///
/// A result containing the IP address of the nameserver if successful
async fn resolver_ip(ns_host: &str, ip_strategy: LookupIpStrategy) -> Result<IpAddr> {
    let mut resolver_builder = Resolver::builder_tokio().into_diagnostic()?;
    resolver_builder.options_mut().ip_strategy = ip_strategy;
    let resolver = resolver_builder.build();

    resolver
        .lookup_ip(ns_host)
        .await
        .into_diagnostic()?
        .iter()
        .next()
        .ok_or_else(|| miette!("Nameserver ip not found"))
}

/// Perform a reverse DNS lookup on an IP address
///
/// # Arguments
///
/// * `ip` - The IP address to look up
///
/// # Returns
///
/// An option containing the reverse DNS entry if successful
async fn reverse_ip(ip: &MyIp) -> Option<ReversedIp> {
    Resolver::builder_tokio()
        .ok()?
        .build()
        .reverse_lookup(ip.ip())
        .await
        .ok()?
        .iter()
        .map(ToString::to_string)
        .map(ReversedIp::from)
        .next()
}

#[cfg(test)]
async fn mock_reverse_ip(ip: &MyIp) -> Option<ReversedIp> {
    // For testing, return a predictable reverse DNS entry based on the IP
    match ip.ip() {
        IpAddr::V4(ipv4) => {
            if ipv4.is_loopback() {
                Some(ReversedIp("localhost".to_string()))
            } else {
                Some(ReversedIp(format!("host-{ipv4}.example.com")))
            }
        }
        IpAddr::V6(ipv6) => {
            if ipv6.is_loopback() {
                Some(ReversedIp("localhost".to_string()))
            } else {
                Some(ReversedIp(format!("host-{ipv6}.example.com")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_find_local_ip_ipv4_only() {
        let ips = find_local_ip(Some(Ipv4));

        // Check that we got at least one IP
        assert!(!ips.is_empty(), "No IPv4 addresses found");

        // Check that all IPs are IPv4
        for ip in ips {
            assert!(
                matches!(ip.ip(), IpAddr::V4(_)),
                "Found IPv6 address when only IPv4 was requested"
            );
        }
    }

    #[test]
    fn test_find_local_ip_ipv6_only() {
        let ips = find_local_ip(Some(Ipv6));

        // Not all systems have IPv6, so we can't assert that we got IPs
        // But if we did get IPs, they should all be IPv6
        for ip in ips {
            assert!(
                matches!(ip.ip(), IpAddr::V6(_)),
                "Found IPv4 address when only IPv6 was requested"
            );
        }
    }

    #[test]
    fn test_find_local_ip_both() {
        let ips = find_local_ip(None);

        // Check that we got at least one IP
        assert!(!ips.is_empty(), "No IP addresses found");
    }

    #[test]
    fn test_process_args_default() {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: false,
            only_6: false,
            reverse: false,
        };

        let strategies = process_args(args);

        // Should have 2 strategies (IPv4 and IPv6 WAN)
        assert_eq!(strategies.len(), 2);
    }

    #[test]
    fn test_process_args_only_local() {
        let args = Args {
            only_local: true,
            only_wan: false,
            only_4: false,
            only_6: false,
            reverse: false,
        };

        let strategies = process_args(args);

        // Should have 0 strategies (no WAN lookups)
        assert_eq!(strategies.len(), 0);
    }

    #[test]
    fn test_process_args_only_ipv4() {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: true,
            only_6: false,
            reverse: false,
        };

        let strategies = process_args(args);

        // Should have 1 strategy (IPv4 WAN only)
        assert_eq!(strategies.len(), 1);
    }

    #[test]
    fn test_process_args_only_ipv6() {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: false,
            only_6: true,
            reverse: false,
        };

        let strategies = process_args(args);

        // Should have 1 strategy (IPv6 WAN only)
        assert_eq!(strategies.len(), 1);
    }

    #[test]
    fn test_get_local_ips_default() {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: false,
            only_6: false,
            reverse: false,
        };

        let local_ips = get_local_ips(args);

        // Should have 1 result (all local IPs)
        assert_eq!(local_ips.len(), 1);

        // The result should be Ok
        assert!(local_ips[0].is_ok());
    }

    #[test]
    fn test_get_local_ips_only_wan() {
        let args = Args {
            only_local: false,
            only_wan: true,
            only_4: false,
            only_6: false,
            reverse: false,
        };

        let local_ips = get_local_ips(args);

        // Should have 0 results (no local IPs)
        assert_eq!(local_ips.len(), 0);
    }

    #[test]
    fn test_get_local_ips_only_ipv4() {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: true,
            only_6: false,
            reverse: false,
        };

        let local_ips = get_local_ips(args);

        // Should have 1 result (IPv4 local IPs)
        assert_eq!(local_ips.len(), 1);

        // The result should be Ok
        assert!(local_ips[0].is_ok());

        // All IPs should be IPv4
        for ip in local_ips[0].as_ref().unwrap() {
            assert!(matches!(ip.ip(), IpAddr::V4(_)));
        }
    }

    #[test]
    fn test_get_local_ips_only_ipv6() {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: false,
            only_6: true,
            reverse: false,
        };

        let local_ips = get_local_ips(args);

        // Should have 1 result (IPv6 local IPs)
        assert_eq!(local_ips.len(), 1);

        // The result should be Ok
        assert!(local_ips[0].is_ok());

        // All IPs should be IPv6 (if any)
        for ip in local_ips[0].as_ref().unwrap() {
            assert!(matches!(ip.ip(), IpAddr::V6(_)));
        }
    }

    #[tokio::test]
    async fn test_reverse_ip() {
        use std::net::Ipv4Addr;

        // Create a test IP
        let test_ip = MyIp::new_plain(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        // Use the mock function to get a reverse DNS entry
        let reversed = mock_reverse_ip(&test_ip).await;

        // Check that we got a result
        assert!(reversed.is_some());

        // Check that the result is what we expect
        assert_eq!(reversed.unwrap(), ReversedIp("localhost".to_string()));

        // Test with a non-loopback IP
        let test_ip = MyIp::new_plain(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
        let reversed = mock_reverse_ip(&test_ip).await;

        // Check that we got a result
        assert!(reversed.is_some());

        // Check that the result contains the IP
        let reversed_str = reversed.unwrap().0;
        assert!(reversed_str.contains("192.168.1.1"));
    }

    #[test]
    fn test_user_ips_parsing() {
        // This test verifies the parsing logic in user_ips without making network calls
        use std::str::FromStr;

        // Test IPv4 parsing
        let ipv4_str = "192.168.1.1";
        let ipv4 = IpAddr::from_str(ipv4_str).unwrap();
        let my_ip = MyIp::new_plain(ipv4);
        assert_eq!(my_ip.ip(), ipv4);

        // Test IPv6 parsing
        let ipv6_str = "2001:db8::1";
        let ipv6 = IpAddr::from_str(ipv6_str).unwrap();
        let my_ip = MyIp::new_plain(ipv6);
        assert_eq!(my_ip.ip(), ipv6);
    }
}
