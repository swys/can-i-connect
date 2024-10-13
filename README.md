can-i-connect
==============

a CLI tool written in Rust that takes a list of `http` and/or `tcp` hosts and tries to establish a connection. If the connection succeeds it will report success for each host. When finished it will print a summary to inform you how many hosts it was able to connect to out of the total number of hosts. If any hosts were unreachable, it will print a list of those hosts.

This might be useful if you need a quick and easy way to check if an app/services dependancies are reachable.

You can also run this in `server mode` where it becomes an api. You can send it `POST` requests and it will return a summary of the results.


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
      --http-hosts <https://example.com>
          comma seperated list of http hosts to attempt to connect to
      --tcp-hosts <example.com:80>
          comma seperated list of tcp hosts to attempt to connect to. Required format: <dns name or ip address>:<port>
      --timeout <5>
          how much time in seconds to wait while connecting to a host before giving up
      --log-level <debug>
          set the log level {info|error|debug} [default: info]
      --no-color
          remove color from log output
      --listen <127.0.0.1:8000>
          run in Server Mode by binding to <ip address>:<port> e.g. 127.0.0.1:8000 or [::1]:8000
  -h, --help
          Print help
  -V, --version
          Print version
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

#### --listen:
run in Server Mode by binding to <ip address>:<port> e.g. `127.0.0.1:8000` (ipv4) or `"[::1]:8000"` (ipv6)

__Note__: If you want to bind to an ipv6 interface you need to wrap the address in quotes.

When running in server mode, using `--listen`, it becomes an API. See below for the list of handlers that can be called, and all of the available options.

##### /health handler

`/health` accepts `GET` requests and returns a `200` with the health status and the app version.

##### GET /health
```
curl "http://[::1]:8000/health" 
```

##### Response
```
{
    "healthy": true,
    "version": "0.1.1"
}
```

##### /can-i-connect handler

`/can-i-connect` accepts `POST` requests and returns a `200` with a full report of the connection status of each host that was passed in.

##### POST /can-i-connect
```
curl -X POST "http://[::1]:8000/can-i-connect" \
-H "Content-Type: application/json" \
-d '{
  "http_hosts": [
    "https://duckduckgo.com",
    "https://rust-lang.org",
    "https://apple.com"
  ],
  "tcp_hosts": [
    "duckduckgo.com:443",
    "rust-lang.org:443"
  ]
  "timeout": "5"
}'
```

##### Response
```
{
    "connection_report": {
        "failures": {
            "failed_hosts_list": [],
            "hosts_unreachable": 0
        },
        "successful": {
            "hosts_reachable": 5,
            "successful_hosts_list": [
                "https://duckduckgo.com",
                "https://rust-lang.org",
                "https://apple.com",
                "duckduckgo.com:443",
                "rust-lang.org:443"
            ]
        }
    },
    "success": true
}
```

##### POST Options
| field name | type | required? | default | description |
|----------|----------|----------| -------| ------------|
| http_hosts | array | false | `[]` | list of http hosts to try to connect to: `["http://duckduckgo.com","https://rust-lang.org"]`  not required both `http_hosts` and `tcp_hosts` cannot be missing/empty
| tcp_hosts | array | false |`[]` | list of tcp hosts to try to connect to: `["duckduckgo.com:443", rust-lang.org:443"]` not required both `http_hosts` and `tcp_hosts` cannot be missing/empty
| timeout | number or string | false | how much time in seconds to wait while connecting to a host before giving up |

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

quick dev setup
===============

There is a `quickdev.rs` file located in the `examples` directory. The purpose of this file is to provide a quick and easy way to test the API without the need to keep running the binary in server mode and then sending requests to test its working as expected.

This can be used to "live reload" after any changes are made. Once there is a change the binary will automatically be rebuild, the new binary will be called and start to listen on a port, and the requests defined in the `examples/quickdev.rs` will automatically trigger and you will see the results after every change.

To set this up you'll need 3 different terminal windows:

#### Terminal 1 (build new binary)
`pwd`: root of this git repo
```
cargo watch -x "install --path ."
```

#### Terminal 2 (rerun binary in server mode after every change)
`pwd`: root of this git repo
```
cargo watch -x build -s './target/debug/can-i-connect --listen 127.0.0.1:8000 --log-level debug'
```

#### Terminal 3 (quickdev example)
`pwd`: root of this git repo
```
cargo watch -q -c -w /Users/swys/.cargo/bin -x "run --example quick_dev"
```

With these three terminals active, once you make any change to any file within the repo it will automatically kick off a sequence that will:
1. rebuild binary if any file in the git repo changes
2. terminate and rerun CLI command if the binary in step 1 changes
3. rerun the `--example quick_dev` whenever the binary from step 2 changes


license
========

MIT