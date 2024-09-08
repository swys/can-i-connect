can-i-connect
==============

a CLI tool written in Rust that takes a list of `http` and/or `tcp` hosts and tries to establish a connection. If the connection succeeds it will report success for each host. When finished it will print a summary to inform you how many hosts it was able to connect to out of the total number of hosts. If any hosts were unreachable, it will print a list of those hosts.

This might be useful if you need a quick and easy way to check if an app/services dependancies are reachable.


usage
=====

```
can-i-connect \
  --http-hosts https://duckduckgo.com,https://www.rust-lang.org/ \
  --tcp-hosts duckduckgo.com:443,rust-lang.org:80 \
  --log-level debug
```

options
=======

Below is what you'll see if you use `-h` or `--help` switch:
```
tool to check connectivity to various hosts using HTTP or TCP

Usage: can-i-connect [OPTIONS]

Options:
      --http-hosts <http-hosts>  comma seperated list of http hosts to attempt to connect to
      --tcp-hosts <tcp-hosts>    comma seperated list of tcp hosts to attempt to connect to. Required format: <dns name or ip address>:<port>
      --timeout <timeout>        how much time in seconds to wait while connecting to a host before giving up
      --log-level <log-level>    set the log level {info|error|debug} [default: info]
      --no-color                 remove color from log output
  -h, --help                     Print help
  -V, --version                  Print version
  ```

#### --http-hosts:
comma seperated list of http hosts
expected format: `<protocol http|https>://<dns host name|ip address>/<path (optional)>`
example: `https://rust-lang.org | http://rust-lang.org | http://18.238.80.4 | https://www.rust-lang.org/learn`
default: ""

#### --tcp-hosts:
comma seperated list of tcp hosts
expected format: `<dns hostname|ip address>:<port>`
example: `rust-lang.org:443 | 18.238.80.4:443`
default: ""

__NOTE:__ there must be at least one host listed in either `--http-hosts` or `--tcp-hosts` arguments. If both of these args are not present or parse into an empty list you will receive the error shown below:
```
No hosts supplied. Must supply hosts through --http-hosts or --tcp-hosts args. Both cannot be empty!
```

#### --timeout:
how much time in seconds to wait while connecting to a host before giving up
example: `10`
default: `5`

#### --log-level:
comma seperated list of tcp hosts
expected format: `<dns hostname|ip address>:<port>`
example: `rust-lang.org:443 | 18.238.80.4:443`
default: info

#### --no-color:
remove color from log output. By default the logs display color
example: `can-i-connect --http-hosts https://rust-lang.org/ --no-color` # <== output will be printed without any color

#### -h | --help:
print help screen

#### -V | --version:
prints version

tests
=====

```
cargo test
```

license
========

MIT