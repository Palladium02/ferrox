# ferrox

ferrox is a very basic load balancer written in Rust that uses a round robin approach to distribute the load.
It continuously performs health checks on all servers in an given interval, removes unresponsive servers from
the pool and adds them once they are responsive again.

## Usage

```
ferrox <path/to/config>
```

## Config

The configuration is expected to be in JSON format.

```
{
  "addr": "a valid ip",
  "remote_addrs": ["a", "list", "of", "valid", "ips"],
  "health_check_interval": 5
}
```

## Planned features
