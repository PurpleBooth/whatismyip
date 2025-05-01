//! IP address representation and formatting
//!
//! This module provides types for representing IP addresses and their
//! associated reverse DNS entries. It includes functionality for:
//! - Creating IP address representations
//! - Formatting IP addresses for display
//! - Handling reverse DNS lookups

use core::fmt;
use core::fmt::{Display, Formatter};
use std::net::IpAddr;

/// Represents an IP address, optionally with a reverse DNS entry
///
/// This enum can represent either a plain IP address or an IP address
/// with an associated reverse DNS entry.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MyIp {
    /// An IP address with an associated reverse DNS entry
    Reversed {
        /// The IP address
        ip: IpAddr,
        /// The reverse DNS entry for the IP address
        reversed: ReversedIp,
    },
    /// A plain IP address without a reverse DNS entry
    Plain {
        /// The IP address
        ip: IpAddr,
    },
}

impl MyIp {
    /// Create a new IP address with a reverse DNS entry
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address
    /// * `reversed` - The reverse DNS entry for the IP address
    ///
    /// # Returns
    ///
    /// A new `MyIp` instance with the IP and reverse DNS entry
    #[must_use]
    pub const fn new_reversed(ip: IpAddr, reversed: ReversedIp) -> Self {
        Self::Reversed { ip, reversed }
    }

    /// Create a new plain IP address without a reverse DNS entry
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address
    ///
    /// # Returns
    ///
    /// A new `MyIp` instance with just the IP address
    #[must_use]
    pub const fn new_plain(ip: IpAddr) -> Self {
        Self::Plain { ip }
    }

    /// Get the IP address
    ///
    /// # Returns
    ///
    /// The IP address, regardless of whether it has a reverse DNS entry
    #[must_use]
    pub const fn ip(&self) -> IpAddr {
        match self {
            Self::Reversed { ip, .. } | Self::Plain { ip } => *ip,
        }
    }
}

/// Represents a reverse DNS entry for an IP address
///
/// This is a wrapper around a String that contains the hostname
/// associated with an IP address.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReversedIp(pub String);

impl From<String> for ReversedIp {
    /// Create a new `ReversedIp` from a String
    ///
    /// # Arguments
    ///
    /// * `value` - The hostname string
    ///
    /// # Returns
    ///
    /// A new `ReversedIp` instance
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for MyIp {
    /// Format an IP address for display
    ///
    /// If the IP has a reverse DNS entry, it will be displayed as:
    /// "`ip_address` (hostname)"
    ///
    /// Otherwise, just the IP address will be displayed.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reversed { ip, reversed } => {
                write!(f, "{ip} ({})", reversed.0)
            }
            Self::Plain { ip } => {
                write!(f, "{ip}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use super::MyIp;
    use super::ReversedIp;

    #[test]
    fn can_format_reversed_ip() {
        let actual = format!(
            "{}",
            MyIp::new_reversed(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                "www.example.com".to_string().into(),
            )
        );
        assert_eq!(actual, String::from("127.0.0.1 (www.example.com)"));
    }

    #[test]
    fn can_create_a_reversed_ip_from_a_string() {
        let input = "Testing".to_string();
        let actual: ReversedIp = input.clone().into();
        assert_eq!(actual, ReversedIp(input));
    }

    #[test]
    fn can_format_plain_ip() {
        let actual = format!(
            "{}",
            MyIp::new_plain(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
        );
        assert_eq!(actual, "127.0.0.1".to_string());
    }
    #[test]
    fn can_get_the_ip_v4() {
        let actual = MyIp::new_plain(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(actual.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
    #[test]
    fn can_get_the_ip_v6() {
        let actual = MyIp::new_reversed(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            "www.example.com".to_string().into(),
        );
        assert_eq!(actual.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
}
