# What is my ip

Work out what your external ip is

## Usage

``` shell,script(name="help",expected_exit_code=0)
whatismyip --help
```

``` text,verify(script_name="help",stream=stdout)
whatismyip 0.7.0
Billie Thompson <billie@billiecodes.com>
Work out what your external ip is

USAGE:
    whatismyip

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
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

``` shell,skip()
whatismyip
```

``` shell,skip()
207.105.7.192
2001:0db8:85a3:0000:0000:8a2e:0370:7334
```

And if you have only an IPv6 address it'll list that

``` shell,skip()
whatismyip
```

``` shell,skip()
2001:0db8:85a3:0000:0000:8a2e:0370:7334
```

## Installing

See the [releases
page](https://github.com/PurpleBooth/whatismyip/releases/latest) we
build for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/whatismyip
```
