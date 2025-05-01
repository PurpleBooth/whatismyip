# What is my IP

A command-line utility for identifying both local and remote IP addresses of your machine.

## Usage

``` shell,script(name="help",expected_exit_code=0)
whatismyip --help | sed 's/\.exe//'
```

``` text,verify(script_name="help",stream=stdout)
Work out what your IP Address is

Usage: whatismyip [OPTIONS]

Options:
  -l, --only-local  Only print IP addresses local to this machine
  -w, --only-wan    Only print IP addresses as seen by a remote service
  -4, --only-4      Only print IPv4 addresses
  -6, --only-6      Only print IPv6 addresses
  -r, --reverse     Print the reverse DNS entries for the IP addresses
  -h, --help        Print help
  -V, --version     Print version
```

When executed, the tool displays your IP addresses:

``` shell,script(name="demo",expected_exit_code=0)
whatismyip
```

``` shell,skip()
207.105.7.192
192.168.1.56
```

The output consists solely of IP addresses, one per line

``` shell,script(name="test",expected_exit_code=0)
whatismyip | grep -E '([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})|(([a-f0-9:]+:+)+[a-f0-9]+)'
```

``` shell,skip()
207.105.7.192
192.168.1.56
```

### IP Version Support

By default, the tool displays both IPv4 and IPv6 addresses if available:

``` shell,script(name="v4-only-ip",expected_exit_code=0)
whatismyip
```

``` shell,skip()
207.105.7.192
192.168.1.56
2001:0db8:85a3:0000:0000:8a2e:0370:7334
fe80::4
```

#### Filtering by IP Version

You can filter results to show only IPv6 addresses:

``` shell,skip()
whatismyip -6
```

``` shell,skip()
2001:0db8:85a3:0000:0000:8a2e:0370:7334
fe80::4
```

Or only IPv4 addresses:

``` shell,script(name="v4-only",expected_exit_code=0)
whatismyip -4
```

``` shell,skip()
207.105.7.192
192.168.1.56
```

### Network Interface Filtering

#### Local Network Interfaces Only

To display only IP addresses from your local network interfaces:

``` shell,script(name="local-only",expected_exit_code=0)
whatismyip -l
```

``` shell,skip()
192.168.1.56
fe80::4
```

#### WAN (External) IP Addresses Only

To display only your external IP addresses as seen by remote services:

``` shell,script(name="wan-only",expected_exit_code=0)
whatismyip -w
```

``` shell,skip()
207.105.7.192
2001:0db8:85a3:0000:0000:8a2e:0370:7334
```

### Reverse DNS Lookup

The tool can perform reverse DNS lookups to display the hostname associated with each IP address. This is particularly useful for identifying VPN exit points or verifying network configurations:

``` shell,script(name="reverse",expected_exit_code=0)
whatismyip -r
```

``` shell,skip()
207.105.7.192 (5898c708dfaf.dip0.t-ipconnect.de.)
2001:0db8:85a3:0000:0000:8a2e:0370:7334 (c06aa6b6af6c4ad5b46473d8d70bc068.dip0.t-ipconnect.de.)
```

## Installation

### Pre-built Binaries

Pre-compiled binaries for Linux and macOS (x86_64) are available on the [releases page](https://codeberg.org/PurpleBooth/whatismyip/releases/latest).

### Using Homebrew

If you use Homebrew, you can install the tool with:

``` shell,skip()
brew install PurpleBooth/repo/whatismyip
```

## Technical Details

### WAN IP Detection Mechanism

The tool determines your external IP address by querying a special DNS record that returns the client's IP address as seen by the DNS server. Specifically, it queries the `o-o.myaddr.l.google.com` TXT record on Google's DNS servers:

- `ns1.google.com`
- `ns2.google.com`
- `ns3.google.com`
- `ns4.google.com`

You can replicate this functionality manually using the [dig tool](https://en.wikipedia.org/wiki/Dig_(command)):

``` shell,skip()
dig TXT +short o-o.myaddr.l.google.com @ns1.google.com
```
