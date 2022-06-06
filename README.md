# What is my ip

Work out what your external ip is. This is done by querying the
`o-o.myaddr.l.google.com` `TXT` record on the Google DNS servers
(`ns1.google.com`, `ns2.google.com`, `ns3.google.com`, or
`ns4.google.com`).

You can do the same thing yourself running using the [dig
tool](https://en.wikipedia.org/wiki/Dig_(command)):

``` shell,skip()
dig TXT +short o-o.myaddr.l.google.com @ns1.google.com
```

I made this tool as a convenient way to do the same thing, including
making multiple requests via only IPv4 and IPv6, so you can see both
your IPv4 and IPv6 addresses.

## Usage

``` shell,script(name="help",expected_exit_code=0)
whatismyip --help | sed 's/\.exe//'
```

``` text,verify(script_name="help",stream=stdout)
whatismyip 0.10.13
Billie Thompson <billie@billiecodes.com>
Work out what your external ip is

USAGE:
    whatismyip [OPTIONS]

OPTIONS:
    -4, --only-4     Only print IPv4 addresses
    -6, --only-6     Only print IPv6 addresses
    -h, --help       Print help information
    -r, --reverse    Print the reverse DNS entries for the IP addresses
    -V, --version    Print version information
```

When you run it you should get an IP back

``` shell,script(name="demo",expected_exit_code=0)
whatismyip
```

``` shell,skip()
207.105.7.192
```

It returns IPs and only IPs

``` shell,script(name="test",expected_exit_code=0)
whatismyip | grep -E '([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})|(([a-f0-9:]+:+)+[a-f0-9]+)'
```

``` shell,skip()
207.105.7.192
```

If you have an IPv4 address and an IPv6 address it'll list both

``` shell,script(name="v4-only-ip",expected_exit_code=0)
whatismyip
```

``` shell,skip()
207.105.7.192
2001:0db8:85a3:0000:0000:8a2e:0370:7334
```

And if you have only an IPv6 address it'll list that

``` shell,script(name="v6-only-ip",expected_exit_code=0)
whatismyip
```

``` shell,skip()
2001:0db8:85a3:0000:0000:8a2e:0370:7334
```

You can also force only v6 IPs

``` shell,skip()
whatismyip -6
```

``` shell,skip()
2001:0db8:85a3:0000:0000:8a2e:0370:7334
```

Or v4s

``` shell,script(name="v4-only",expected_exit_code=0)
whatismyip -4
```

``` shell,skip()
207.105.7.192
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
page](https://github.com/PurpleBooth/whatismyip/releases/latest) we
build for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/whatismyip
```
