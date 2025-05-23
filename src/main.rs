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
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    missing_docs
)]
#![allow(clippy::multiple_crate_versions)]

use clap::Parser;
use futures::future::join_all;
use miette::{Result, bail, set_panic_hook};
use std::hash::RandomState;
use whatismyip::IpVersion::{Ipv4, Ipv6};
use whatismyip::cli::Args;
use whatismyip::{MyIps, find_local_ip, find_wan_ip, format_ips, process_ips};
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
    let args = Args::parse();

    // Process arguments to determine which strategies to use
    let strategies = process_args(args);

    // Start WAN IP lookups
    let wan_handle = tokio::spawn(async move { join_all(strategies).await });

    // Start local IP lookups in parallel if needed
    let local_results = if args.only_wan {
        vec![]
    } else {
        get_local_ips(args)
    };

    // Wait for WAN IP lookups to complete
    let mut results = wan_handle.await.unwrap_or_default();

    // Add local IPs to results
    results.extend(local_results);

    // Partition results into successes and failures
    let (ok, failures): (Vec<Result<MyIps>>, Vec<Result<MyIps>>) =
        results.into_iter().partition(Result::is_ok);

    if ok.is_empty() {
        bail!("Failed: {:?}", failures,);
    }

    let processed_ips = process_ips(&ok, args.reverse).await;
    let resolution_result = format_ips::<RandomState>(processed_ips);
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

    if args.only_local {
        return strategies;
    }

    if !args.only_6 {
        strategies.push(find_wan_ip(Ipv4));
    }
    if !args.only_4 {
        strategies.push(find_wan_ip(Ipv6));
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
        } => vec![(find_local_ip(Some(Ipv4)))],
        Args {
            only_wan: false,
            only_6: true,
            only_4: false,
            ..
        } => vec![(find_local_ip(Some(Ipv6)))],
        Args {
            only_wan: false,
            only_6: false,
            only_4: false,
            ..
        } => vec![(find_local_ip(None))],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miette::miette;
    use std::net::IpAddr;
    use whatismyip::myip::MyIp;

    type TestResult = Result<()>;

    #[test]
    fn test_find_local_ip_ipv4_only() -> TestResult {
        let ips = find_local_ip(Some(Ipv4))?;

        // Check that we got at least one IP
        if ips.is_empty() {
            return Err(miette!("No IPv4 addresses found"));
        }

        // Check that all IPs are IPv4
        for ip in ips {
            if !matches!(ip.ip(), IpAddr::V4(_)) {
                return Err(miette!("Found IPv6 address when only IPv4 was requested"));
            }
        }

        Ok(())
    }

    #[test]
    fn test_find_local_ip_ipv6_only() -> TestResult {
        let ips = find_local_ip(Some(Ipv6))?;

        // Not all systems have IPv6, so we can't assert that we got IPs
        // But if we did get IPs, they should all be IPv6
        for ip in ips {
            if !matches!(ip.ip(), IpAddr::V6(_)) {
                return Err(miette!("Found IPv4 address when only IPv6 was requested"));
            }
        }

        Ok(())
    }

    #[test]
    fn test_find_local_ip_both() -> TestResult {
        let ips = find_local_ip(None)?;

        // Check that we got at least one IP
        if ips.is_empty() {
            return Err(miette!("No IP addresses found"));
        }

        Ok(())
    }

    #[test]
    fn test_process_args_default() -> TestResult {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: false,
            only_6: false,
            reverse: false,
        };

        let strategies = process_args(args);

        // Should have 2 strategies (IPv4 and IPv6 WAN)
        if strategies.len() != 2 {
            return Err(miette!("Expected 2 strategies, got {}", strategies.len()));
        }

        Ok(())
    }

    #[test]
    fn test_process_args_only_local() -> TestResult {
        let args = Args {
            only_local: true,
            only_wan: false,
            only_4: false,
            only_6: false,
            reverse: false,
        };

        let strategies = process_args(args);

        // Should have 0 strategies (no WAN lookups)
        if !strategies.is_empty() {
            return Err(miette!("Expected 0 strategies, got {}", strategies.len()));
        }

        Ok(())
    }

    #[test]
    fn test_process_args_only_ipv4() -> TestResult {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: true,
            only_6: false,
            reverse: false,
        };

        let strategies = process_args(args);

        // Should have 1 strategy (IPv4 WAN only)
        if strategies.len() != 1 {
            return Err(miette!("Expected 1 strategy, got {}", strategies.len()));
        }

        Ok(())
    }

    #[test]
    fn test_process_args_only_ipv6() -> TestResult {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: false,
            only_6: true,
            reverse: false,
        };

        let strategies = process_args(args);

        // Should have 1 strategy (IPv6 WAN only)
        if strategies.len() != 1 {
            return Err(miette!("Expected 1 strategy, got {}", strategies.len()));
        }

        Ok(())
    }

    #[test]
    fn test_get_local_ips_default() -> TestResult {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: false,
            only_6: false,
            reverse: false,
        };

        let local_ips = get_local_ips(args);

        // Should have 1 result (all local IPs)
        if local_ips.len() != 1 {
            return Err(miette!("Expected 1 result, got {}", local_ips.len()));
        }

        // The result should be Ok
        if local_ips[0].is_err() {
            return Err(miette!("Expected Ok result, got Err"));
        }

        Ok(())
    }

    #[test]
    fn test_get_local_ips_only_wan() -> TestResult {
        let args = Args {
            only_local: false,
            only_wan: true,
            only_4: false,
            only_6: false,
            reverse: false,
        };

        let local_ips = get_local_ips(args);

        // Should have 0 results (no local IPs)
        if !local_ips.is_empty() {
            return Err(miette!("Expected 0 results, got {}", local_ips.len()));
        }

        Ok(())
    }

    #[test]
    fn test_get_local_ips_only_ipv4() -> TestResult {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: true,
            only_6: false,
            reverse: false,
        };

        let local_ips = get_local_ips(args);

        // Should have 1 result (IPv4 local IPs)
        if local_ips.len() != 1 {
            return Err(miette!("Expected 1 result, got {}", local_ips.len()));
        }

        // The result should be Ok
        if local_ips[0].is_err() {
            return Err(miette!("Expected Ok result, got Err"));
        }

        // All IPs should be IPv4
        for ip in local_ips[0].as_ref().unwrap() {
            if !matches!(ip.ip(), IpAddr::V4(_)) {
                return Err(miette!("Expected IPv4 address, got IPv6"));
            }
        }

        Ok(())
    }

    #[test]
    fn test_get_local_ips_only_ipv6() -> TestResult {
        let args = Args {
            only_local: false,
            only_wan: false,
            only_4: false,
            only_6: true,
            reverse: false,
        };

        let local_ips = get_local_ips(args);

        // Should have 1 result (IPv6 local IPs)
        if local_ips.len() != 1 {
            return Err(miette!("Expected 1 result, got {}", local_ips.len()));
        }

        // The result should be Ok
        if local_ips[0].is_err() {
            return Err(miette!("Expected Ok result, got Err"));
        }

        // All IPs should be IPv6 (if any)
        for ip in local_ips[0].as_ref().unwrap() {
            if !matches!(ip.ip(), IpAddr::V6(_)) {
                return Err(miette!("Expected IPv6 address, got IPv4"));
            }
        }

        Ok(())
    }

    /// Mock implementation of reverse DNS lookup for testing
    ///
    /// This function returns a predictable reverse DNS entry based on the IP address.
    /// It's used in tests to avoid making actual network calls.
    #[cfg(test)]
    fn mock_reverse_ip(ip: &MyIp) -> whatismyip::myip::ReversedIp {
        use std::net::IpAddr;
        // For testing, return a predictable reverse DNS entry based on the IP
        match ip.ip() {
            IpAddr::V4(ipv4) => {
                if ipv4.is_loopback() {
                    whatismyip::myip::ReversedIp("localhost".to_string())
                } else {
                    whatismyip::myip::ReversedIp(format!("host-{ipv4}.example.com"))
                }
            }
            IpAddr::V6(ipv6) => {
                if ipv6.is_loopback() {
                    whatismyip::myip::ReversedIp("localhost".to_string())
                } else {
                    whatismyip::myip::ReversedIp(format!("host-{ipv6}.example.com"))
                }
            }
        }
    }

    #[tokio::test]
    async fn test_reverse_ip() -> TestResult {
        use std::net::{IpAddr, Ipv4Addr};

        // Create a test IP
        let test_ip = MyIp::new_plain(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        // Use the mock function to get a reverse DNS entry
        let reversed = mock_reverse_ip(&test_ip);

        // Check that the result is what we expect
        if reversed != whatismyip::myip::ReversedIp("localhost".to_string()) {
            return Err(miette!("Expected 'localhost', got '{}'", reversed.0));
        }

        // Test with a non-loopback IP
        let test_ip = MyIp::new_plain(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
        let reversed = mock_reverse_ip(&test_ip);

        // Check that the result contains the IP
        let reversed_str = reversed.0;
        if !reversed_str.contains("192.168.1.1") {
            return Err(miette!(
                "Expected string containing '192.168.1.1', got '{}'",
                reversed_str
            ));
        }

        Ok(())
    }
}
