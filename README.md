# What is my ip

I made this tool as a convenient way to get my IPs, be they remote or
local.

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

When you run it you should get an IP back

``` shell,script(name="demo",expected_exit_code=0)
whatismyip
```

``` shell,skip()
207.105.7.192
192.168.1.56
```

It returns IPs and only IPs

``` shell,script(name="test",expected_exit_code=0)
whatismyip | grep -E '([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})|(([a-f0-9:]+:+)+[a-f0-9]+)'
```

``` shell,skip()
207.105.7.192
192.168.1.56
```

If you have an IPv4 address and an IPv6 address it'll list both

``` shell,script(name="v4-only-ip",expected_exit_code=0)
whatismyip
```

``` shell,skip()
207.105.7.192
192.168.1.56
2001:0db8:85a3:0000:0000:8a2e:0370:7334
fe80::4
```

And if you have only an IPv6 address it'll list that

``` shell,script(name="v6-only-ip",expected_exit_code=0)
whatismyip
```

``` shell,skip()
2001:0db8:85a3:0000:0000:8a2e:0370:7334
fe80::4
```

You can also force only v6 IPs

``` shell,skip()
whatismyip -6
```

``` shell,skip()
2001:0db8:85a3:0000:0000:8a2e:0370:7334
fe80::4
```

Or v4s

``` shell,script(name="v4-only",expected_exit_code=0)
whatismyip -4
```

``` shell,skip()
207.105.7.192
192.168.1.56
```

You can also get only IP Addresses local to your network interfaces

``` shell,script(name="local-only",expected_exit_code=0)
whatismyip -l
```

``` shell,skip()
192.168.1.56
fe80::4
```

Or only the WAN ones

``` shell,script(name="wan-only",expected_exit_code=0)
whatismyip -w
```

``` shell,skip()
207.105.7.192
2001:0db8:85a3:0000:0000:8a2e:0370:7334
```

You can also reverse those IPs, which is handy for checking VPNs and
similar where you want to identify your gateway exit point

``` shell,script(name="reverse",expected_exit_code=0)
whatismyip -r
```

``` shell,skip()
207.105.7.192 (5898c708dfaf.dip0.t-ipconnect.de.)
2001:0db8:85a3:0000:0000:8a2e:0370:7334 (c06aa6b6af6c4ad5b46473d8d70bc068.dip0.t-ipconnect.de.)
```

## Installing

See the [releases
page](https://codeberg.org/PurpleBooth/whatismyip/releases/latest) we
build for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/whatismyip
```

## How the WAN IP detection works

This is done by querying the `o-o.myaddr.l.google.com` `TXT` record on
the Google DNS servers (`ns1.google.com`, `ns2.google.com`,
`ns3.google.com`, or `ns4.google.com`).

You can do the same thing yourself running using the [dig
tool](https://en.wikipedia.org/wiki/Dig_(command)):

``` shell,skip()
dig TXT +short o-o.myaddr.l.google.com @ns1.google.com
```
