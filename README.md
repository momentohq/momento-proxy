# Momento Proxy

This product is a simple proxy which can allow existing applications which use
Memcached or RESP operations for caching to use [Momento](https://momentohq.com)
cache offering without any code changes.

## Features

- **Transparent**: allows existing applications which use Memcached or RESP to switch to
  Momento without code changes.
- **Stats**: get insight into runtime by using the Memcached `stats` command on
  the admin port.
- **Command Log**: enables logging of commands for audit and offline workload
  analysis.

## Limitations

- Only Memcached `get`, `set`, and `delete` operations are supported.
- Only a subset of RESP operations are supported; see the [protocol/resp](./src/protocol/resp/) folder.

## Building

Use the [Makefile](./Makefile).

## Configuration

### API Key

The Momento proxy requires that the `MOMENTO_API_KEY` environment
variable is set and contains a valid Momento API key, which you can 
obtain from the [Momento Console](https://console.gomomento.com/).

### Create Cache

After obtaining an API key, you should create one or more caches to
use with the proxy.

### Proxy Configuration

The Momento proxy uses a TOML configuration file. As there aren't any sensible
defaults, we require that you provide a configuration file when using the proxy.
See the [example config](./config/momento_proxy.toml) and modify it to suit
your requirements.

## Running

After completing the build and configuration, you are ready to run the Momento
proxy.

`cargo run --release --bin momento_proxy -- path/to/config.toml`

Your application can now connect to the proxy on the configured port(s) to send
requests to the corresponding Momento cache(s).

The resulting binary will be `target/release/momento_proxy` and it can be copied
to a standard system path (eg: `/usr/local/bin`).

### momento-proxy Docker image

You can run the `momento-proxy` container by pulling it from [Momento's](https://momentohq.com/) [docker hub registry](https://hub.docker.com/u/gomomento) with following commands.

```
docker pull gomomento/momento-proxy
docker run -d \
  -p 11211:11211 \
  -p 6379:6379 \
  -p 9999:9999 \
  -e MOMENTO_API_KEY=<YOUR_MOMENTO_API_KEY> \
  gomomento/momento-proxy
```

By default, [this configuration](./config/momento_proxy.toml) is used for the Momento proxy.
To set your own, please provide an env variable `CONFIG` as well as the directory where your config file is located to `-v` when running a container.

```
docker run -d \
  -p 11211:11211 \
  -p 6379:6379 \
  -p 9999:9999 \
  -e MOMENTO_API_KEY=<YOUR_MOMENTO_API_KEY> \
  -e CONFIG=my_custom_config.toml \
  -v /your/path/to/config/dir:/app/config gomomento/momento-proxy
```

### momento-proxy Docker image local development

- Building a new momento-proxy image:

```
docker build --tag momento-proxy .
```

- Running the newly built image with the default config:

```
docker run -d \
  -p 11211:11211 \
  -p 6379:6379 \
  -p 9999:9999 \
  -e MOMENTO_API_KEY=<YOUR_MOMENTO_API_KEY> \
  momento-proxy
```

- Running the newly built image with your custom config:

```
docker run -d \
  -p 11211:11211 \
  -p 6379:6379 \
  -p 9999:9999 \
  -e MOMENTO_API_KEY=<YOUR_MOMENTO_API_KEY> \
  -e CONFIG=<YOUR_CONFIG_FILE> \
  -v /your/path/to/config/dir:/app/config \
  momento-proxy
```

- Testing to see if a container with the `momento-proxy` is running properly with `telnet` for Memcache:

```
telnet 0.0.0.0 11211
Trying 0.0.0.0...
Connected to 0.0.0.0.
Escape character is '^]'
set foo 0 0  3
bar
STORED
```

- Or you can use `redis-cli` to test to see if `momento-proxy` is running properly for Redis:

```
redis-cli -h 0.0.0.0 -p 6379 SET FOO BAR
OK
redis-cli -h 0.0.0.0 -p 6379 GET FOO
"BAR"
```
