# What is my ip

Work out what your external ip is

## Usage

``` shell,script(name="help",expected_exit_code=0)
whatismyip --help
```

``` text,verify(script_name="help",stream=stdout)
whatismyip 0.6.2
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

This should be your ip

``` shell,script(name="test",expected_exit_code=0)
whatismyip | grep -E '([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})|(([a-f0-9:]+:+)+[a-f0-9]+)'
```

``` shell,skip()
207.105.7.192
```

## Installing

See the [releases
page](https://github.com/PurpleBooth/whatismyip/releases/latest) we
build for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/whatismyip
```
