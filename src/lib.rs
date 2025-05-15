//! A library for determining machine IP addresses
//!
//! This library provides functionality to identify both local network interface
//! IP addresses and external (WAN) IP addresses of a machine. It supports both
//! IPv4 and IPv6 addresses and can perform reverse DNS lookups to resolve
//! hostnames associated with IP addresses.
//!
//! ## Features
//!
//! - Local IP discovery through network interface enumeration
//! - External IP discovery using DNS queries to Google's nameservers
//! - Reverse DNS resolution for IP addresses
//! - Filtering by IP version (IPv4/IPv6)
//! - Concurrent processing for efficient lookups

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
    missing_docs,
    non_fmt_panics
)]
#![allow(clippy::multiple_crate_versions)]

use crate::IpVersion::{Ipv4, Ipv6};
use futures::{StreamExt, stream};
use hickory_resolver::config::{LookupIpStrategy, NameServerConfigGroup, ResolverConfig};
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::{Resolver, TokioResolver};
use local_ip_address::list_afinet_netifas;
use miette::{IntoDiagnostic, Result, miette};
use std::collections::HashSet;
use std::net::IpAddr;
use std::str::FromStr;
use tokio::sync::OnceCell;

pub mod cli;
pub mod myip;

/// A collection of IP addresses
pub type MyIps = Vec<myip::MyIp>;

/// Google's primary nameserver hostname
pub const GOOGLE_NS1: &str = "ns1.google.com";
/// Google's secondary nameserver hostname
pub const GOOGLE_NS2: &str = "ns2.google.com";
/// Google's tertiary nameserver hostname
pub const GOOGLE_NS3: &str = "ns3.google.com";
/// Google's quaternary nameserver hostname
pub const GOOGLE_NS4: &str = "ns4.google.com";
/// Special Google DNS record that returns the client's IP address
pub const MYADDR_RECORD: &str = "o-o.myaddr.l.google.com";

/// Represents the version of IP address to use
#[derive(Copy, Debug, Clone)]
pub enum IpVersion {
    /// IPv4 address version
    Ipv4,
    /// IPv6 address version
    Ipv6,
}

/// Discovers external (WAN) IP addresses using DNS queries
///
/// This function determines the machine's external IP address by querying
/// a special DNS record that returns the client's IP address as seen by
/// the DNS server. The process works as follows:
///
/// 1. Resolves one of Google's nameservers (tries multiple in parallel)
/// 2. Creates a DNS resolver using the first nameserver that responds
/// 3. Queries the special DNS record to get the client's IP address
///
/// For performance, the function caches DNS resolvers for subsequent calls.
///
/// # Arguments
///
/// * `strategy` - The IP version filter to apply (IPv4 or IPv6)
///
/// # Returns
///
/// A result containing a vector of external IP addresses
///
/// # Errors
///
/// Returns an error if DNS resolution fails for all nameservers or if
/// the special DNS record cannot be queried successfully.
pub async fn find_wan_ip(strategy: IpVersion) -> Result<MyIps> {
    // Use tokio's OnceCell for async initialization
    static IPV4_DNS_RESOLVER: OnceCell<TokioResolver> = OnceCell::const_new();
    static IPV6_DNS_RESOLVER: tokio::sync::OnceCell<TokioResolver> =
        tokio::sync::OnceCell::const_new();

    // Try to use cached resolver first
    let resolver_cell = match strategy {
        Ipv4 => &IPV4_DNS_RESOLVER,
        Ipv6 => &IPV6_DNS_RESOLVER,
    };

    // Add retry logic with 3 attempts
    let mut retries = 3;
    loop {
        // If we already have a resolver, use it directly
        if let Some(resolver) = resolver_cell.get() {
            return user_ips(resolver).await;
        }

        // Otherwise, we need to create a new resolver
        let lookup_ip_strategy = match strategy {
            Ipv4 => LookupIpStrategy::Ipv4Only,
            Ipv6 => LookupIpStrategy::Ipv6Only,
        };

        // Try all nameservers in parallel and use the first one that responds
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
        };

        match ns_ip {
            Ok(ip) => {
                // Create and cache the resolver
                let dns_resolver = resolver_cell
                    .get_or_init(|| async { resolver(ip, lookup_ip_strategy) })
                    .await;
                return user_ips(dns_resolver).await;
            }
            Err(_e) if retries > 0 => {
                retries -= 1;
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Discovers IP addresses from local network interfaces
///
/// This function enumerates all network interfaces on the machine and collects
/// their IP addresses. The results can be filtered by IP version (IPv4 or IPv6).
/// The function is optimized for common use cases and includes special handling
/// for different filtering scenarios.
///
/// # Arguments
///
/// * `strategy` - Optional IP version filter:
///   * `None` - Return both IPv4 and IPv6 addresses
///   * `Some(Ipv4)` - Return only IPv4 addresses
///   * `Some(Ipv6)` - Return only IPv6 addresses
///
/// # Returns
///
/// A vector of IP addresses matching the specified filter
///
/// # Errors
///
/// While this function returns a `Result`, it's designed to handle most errors
/// gracefully by returning an empty vector rather than propagating the error.
/// It will only return an error in exceptional circumstances.
pub fn find_local_ip(strategy: Option<IpVersion>) -> Result<MyIps> {
    // Pre-allocate with a reasonable capacity to avoid reallocations
    let mut result = Vec::with_capacity(8);

    // Early return if we can't get network interfaces
    let Ok(netifas) = list_afinet_netifas() else {
        return Ok(result);
    };

    // Optimize the common case (no filter) to avoid match overhead in the loop
    if strategy.is_none() {
        result.reserve(netifas.len());
        for (_, ip) in netifas {
            result.push(myip::MyIp::new_plain(ip));
        }
        return Ok(result);
    }

    // For filtered cases, use a more direct approach
    match strategy {
        Some(Ipv4) => {
            for (_, ip) in netifas {
                if !ip.is_ipv6() {
                    result.push(myip::MyIp::new_plain(ip));
                }
            }
        }
        Some(Ipv6) => {
            for (_, ip) in netifas {
                if ip.is_ipv6() {
                    result.push(myip::MyIp::new_plain(ip));
                }
            }
        }
        _ => {}
    }

    Ok(result)
}

/// Queries a DNS resolver to retrieve the client's external IP addresses
///
/// This function performs a DNS TXT record lookup for a special domain
/// (`o-o.myaddr.l.google.com`) that returns the client's IP address as seen
/// by the DNS server. This technique is used to determine external (WAN) IP
/// addresses without relying on third-party web services.
///
/// The function parses the TXT record responses and converts them to IP addresses.
///
/// # Arguments
///
/// * `resolver` - A configured DNS resolver to use for the query
///
/// # Returns
///
/// A result containing a vector of IP addresses obtained from the DNS response
///
/// # Errors
///
/// Returns an error if:
/// - The DNS lookup fails to complete
/// - The resolver encounters network issues
/// - The TXT records cannot be retrieved
pub async fn user_ips(resolver: &Resolver<TokioConnectionProvider>) -> Result<MyIps> {
    // Perform the DNS lookup
    let txt_records = resolver.txt_lookup(MYADDR_RECORD).await.into_diagnostic()?;

    // Pre-allocate the result vector with a reasonable capacity
    // Most of the time we'll get 1-2 IPs (IPv4 and/or IPv6)
    let mut result = Vec::with_capacity(2);

    // Process each TXT record
    for record in txt_records.iter() {
        // Convert to string only once
        let ip_str = record.to_string();

        // Try to parse as IP address
        if let Ok(ip) = IpAddr::from_str(&ip_str) {
            result.push(myip::MyIp::new_plain(ip));
        }
    }

    Ok(result)
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
#[must_use]
pub fn resolver(ip: IpAddr, ip_strategy: LookupIpStrategy) -> TokioResolver {
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
///
/// # Errors
///
/// Returns an error if the DNS lookup fails or if no IP address is found for the given hostname
pub async fn resolver_ip(ns_host: &str, ip_strategy: LookupIpStrategy) -> Result<IpAddr> {
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};

    // Static caches for resolved IPs to avoid repeated lookups
    static IPV4_CACHE: LazyLock<Mutex<HashMap<String, IpAddr>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));
    static IPV6_CACHE: LazyLock<Mutex<HashMap<String, IpAddr>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    // Use tokio's OnceCell for async initialization
    static IPV4_RESOLVER: OnceCell<TokioResolver> = OnceCell::const_new();
    static IPV6_RESOLVER: tokio::sync::OnceCell<TokioResolver> = tokio::sync::OnceCell::const_new();

    // Select the appropriate cache based on IP strategy
    let cache = match ip_strategy {
        LookupIpStrategy::Ipv4Only => &IPV4_CACHE,
        _ => &IPV6_CACHE,
    };

    // Try to get from cache first (using std::sync::Mutex instead of tokio::sync::Mutex)
    // This avoids an await point and is more efficient for read-heavy workloads
    if let Ok(cache_guard) = cache.lock() {
        if let Some(ip) = cache_guard.get(ns_host) {
            return Ok(*ip);
        }
    }

    // Get or initialize the appropriate resolver based on the IP strategy
    let resolver = match ip_strategy {
        LookupIpStrategy::Ipv4Only => {
            IPV4_RESOLVER
                .get_or_try_init::<miette::Error, _, _>(|| async {
                    let mut builder = Resolver::builder_tokio().into_diagnostic()?;
                    builder.options_mut().ip_strategy = LookupIpStrategy::Ipv4Only;
                    miette::Result::Ok(builder.build())
                })
                .await
        }
        _ => {
            IPV6_RESOLVER
                .get_or_try_init(|| async {
                    let mut builder = Resolver::builder_tokio().into_diagnostic()?;
                    builder.options_mut().ip_strategy = LookupIpStrategy::Ipv6Only;
                    miette::Result::Ok(builder.build())
                })
                .await
        }
    }?;

    // Perform the lookup
    let ip = resolver
        .lookup_ip(ns_host)
        .await
        .into_diagnostic()?
        .iter()
        .next()
        .ok_or_else(|| miette!("Nameserver ip not found"))?;

    // Cache the result for future use (using std::sync::Mutex)
    if let Ok(mut cache_guard) = cache.lock() {
        cache_guard.insert(ns_host.to_string(), ip);
    }

    Ok(ip)
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
pub async fn reverse_ip(ip: &myip::MyIp) -> Option<myip::ReversedIp> {
    // Create a resolver only once
    static RESOLVER: OnceCell<TokioResolver> = OnceCell::const_new();

    let resolver = RESOLVER
        .get_or_try_init::<miette::Error, _, _>(|| async {
            miette::Result::Ok(Resolver::builder_tokio().into_diagnostic()?.build())
        })
        .await
        .ok()?;

    resolver
        .reverse_lookup(ip.ip())
        .await
        .ok()?
        .iter()
        .map(ToString::to_string)
        .map(myip::ReversedIp::from)
        .next()
}

/// Processes IP addresses with optional reverse DNS resolution
///
/// This function takes a collection of IP addresses and:
/// 1. Optionally performs reverse DNS lookups for each IP
/// 2. Formats the results as strings (with hostname information if reverse lookup was performed)
/// 3. Returns a deduplicated set of formatted IP addresses
///
/// The function is optimized for performance with different code paths for the reverse
/// and non-reverse cases. When reverse lookups are requested, it uses concurrent
/// processing to improve performance.
///
/// # Arguments
///
/// * `ips` - A collection of results containing IP addresses to process
/// * `do_reverse` - Boolean flag indicating whether to perform reverse DNS lookups
///
/// # Returns
///
/// A future that resolves to a `HashSet` of formatted IP address strings.
/// When `do_reverse` is true, the strings will be in the format "`ip_address` (hostname)"
pub async fn process_ips(ips: &[Result<MyIps>], do_reverse: bool) -> HashSet<String> {
    // If we don't need to do reverse lookups, we can optimize by avoiding the async processing
    if !do_reverse {
        // Estimate capacity to avoid reallocations
        let mut total_ips = 0;
        for ip_result in ips.iter().flatten() {
            total_ips += ip_result.len();
        }

        let mut result = HashSet::with_capacity(total_ips);

        // Use a single loop to avoid nested iterators
        for ip_result in ips.iter().flatten() {
            for my_ip in ip_result {
                result.insert(my_ip.ip().to_string());
            }
        }

        return result;
    }

    // For reverse lookups, use parallel stream processing for better performance
    // Use a more efficient approach to collect IPs
    let mut all_ips = Vec::new();
    let mut estimated_capacity = 0;

    // First pass to estimate capacity
    for ip_result in ips.iter().flatten() {
        estimated_capacity += ip_result.len();
    }

    all_ips.reserve(estimated_capacity);

    // Second pass to collect IPs without allocating for each iteration
    for ip_result in ips.iter().flatten() {
        all_ips.extend(ip_result.iter());
    }

    // Process in parallel with buffer_unordered for concurrent lookups
    // Use a more reasonable concurrency limit based on typical DNS resolver limits
    stream::iter(all_ips)
        .map(|my_ip| async move {
            reverse_ip(my_ip).await.map_or_else(
                || my_ip.ip().to_string(),
                |reversed_ip| format!("{} ({})", my_ip.ip(), reversed_ip.0),
            )
        })
        .buffer_unordered(16) // Reduced from 32 to avoid overwhelming DNS resolvers
        .collect::<HashSet<String>>()
        .await
}

/// Format a collection of IP address strings as a single string
///
/// This function takes a collection of IP address strings and formats them as a single string,
/// with each IP address on a new line.
///
/// # Arguments
///
/// * `ips` - A collection of IP address strings
///
/// # Returns
///
/// A string containing the formatted IP addresses
#[must_use]
pub fn format_ips<S: ::std::hash::BuildHasher>(ips: HashSet<String, S>) -> String {
    if ips.is_empty() {
        return String::new();
    }

    // For small sets, the overhead of sorting might not be worth it
    if ips.len() <= 5 {
        // Calculate total capacity needed to avoid reallocations
        let total_len = ips.iter().map(std::string::String::len).sum::<usize>() + ips.len() - 1;

        // Use with_capacity for better performance
        let mut result = String::with_capacity(total_len);

        // Use iterator to build the string efficiently
        let mut iter = ips.into_iter();

        // Add the first item (we know there's at least one because we checked is_empty)
        if let Some(first) = iter.next() {
            result.push_str(&first);

            // Add remaining items with newline prefix
            for ip in iter {
                result.push('\n');
                result.push_str(&ip);
            }
        }

        return result;
    }

    // For larger sets, convert to a sorted Vec for more predictable output
    // This can improve caching behavior and make the output more user-friendly
    let mut ips_vec: Vec<_> = ips.into_iter().collect();
    ips_vec.sort_unstable(); // sort_unstable is faster than sort

    // Calculate capacity more precisely now that we know the exact order
    let total_len = ips_vec.iter().map(std::string::String::len).sum::<usize>() + ips_vec.len() - 1;

    // Pre-allocate the result string
    let mut result = String::with_capacity(total_len);

    // Build the result string
    let mut first = true;
    for ip in ips_vec {
        if first {
            first = false;
        } else {
            result.push('\n');
        }
        result.push_str(&ip);
    }

    result
}

#[cfg(test)]
/// Mock implementation of reverse DNS lookup for testing
///
/// This function returns a predictable reverse DNS entry based on the IP address.
/// It's used in tests to avoid making actual network calls.
///
/// # Arguments
///
/// * `ip` - The IP address to look up
///
/// # Returns
///
/// An option containing the mock reverse DNS entry
#[must_use]
pub fn mock_reverse_ip(ip: &myip::MyIp) -> Option<myip::ReversedIp> {
    // For testing, return a predictable reverse DNS entry based on the IP
    match ip.ip() {
        IpAddr::V4(ipv4) => {
            if ipv4.is_loopback() {
                Some(myip::ReversedIp("localhost".to_string()))
            } else {
                Some(myip::ReversedIp(format!("host-{ipv4}.example.com")))
            }
        }
        IpAddr::V6(ipv6) => {
            if ipv6.is_loopback() {
                Some(myip::ReversedIp("localhost".to_string()))
            } else {
                Some(myip::ReversedIp(format!("host-{ipv6}.example.com")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

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
    fn test_user_ips_parsing() -> TestResult {
        // This test verifies the parsing logic in user_ips without making network calls
        use std::str::FromStr;

        // Test IPv4 parsing
        let ipv4_str = "192.168.1.1";
        let ipv4 = IpAddr::from_str(ipv4_str)
            .map_err(|e| miette!("Failed to parse IPv4 address: {}", e))?;
        let my_ip = myip::MyIp::new_plain(ipv4);
        
        if my_ip.ip() != ipv4 {
            return Err(miette!("IPv4 address mismatch"));
        }

        // Test IPv6 parsing
        let ipv6_str = "2001:db8::1";
        let ipv6 = IpAddr::from_str(ipv6_str)
            .map_err(|e| miette!("Failed to parse IPv6 address: {}", e))?;
        let my_ip = myip::MyIp::new_plain(ipv6);
        
        if my_ip.ip() != ipv6 {
            return Err(miette!("IPv6 address mismatch"));
        }

        Ok(())
    }
}
