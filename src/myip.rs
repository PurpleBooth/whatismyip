use core::fmt;
use core::fmt::{Display, Formatter};
use std::net::IpAddr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MyIp {
    Reversed { ip: IpAddr, reversed: ReversedIp },
    Plain { ip: IpAddr },
}

impl MyIp {
    pub(crate) const fn new_reversed(ip: IpAddr, reversed: ReversedIp) -> Self {
        Self::Reversed { ip, reversed }
    }
    pub(crate) const fn new_plain(ip: IpAddr) -> Self {
        Self::Plain { ip }
    }
    pub(crate) const fn ip(&self) -> IpAddr {
        match self {
            MyIp::Reversed { ip, .. } | MyIp::Plain { ip } => *ip,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReversedIp(pub String);

impl From<String> for ReversedIp {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for MyIp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyIp::Reversed { ip, reversed } => {
                write!(f, "{} ({})", ip, reversed.0)
            }
            MyIp::Plain { ip } => {
                write!(f, "{}", ip)
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
                ReversedIp(String::from("www.example.com")),
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
        assert_eq!(actual, String::from("127.0.0.1"));
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
            ReversedIp(String::from("www.example.com")),
        );
        assert_eq!(actual.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
}
