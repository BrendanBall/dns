# DNS
Currently implements a simple (incomplete) DNS stub resolver in Rust.

## Run
```bash
# Basic, defaults to querying 1.1.1.1 for an A record
cargo run -p dns_resolver -- example.com
# Full, call specified DNS server for an AAAA record
cargo run -p dns_resolver -- -s 1.1.1.1 -r AAAA example.com
```

## Resources
- https://howdns.works/
- https://www.statdns.com/rfc/
- https://github.com/jvns/dns-weekend
- https://www.zytrax.com/books/dns/