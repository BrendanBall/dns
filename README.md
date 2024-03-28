# DNS
Currently implements a simple (incomplete) DNS stub resolver in Rust.

## Run
```bash
# Basic, defaults to querying 1.1.1.1 for an A record
cargo run -p dns_resolver -- example.com
```
```
;; ->>HEADER<<- opcode: QUERY, rcode: NOERROR, id: 47960
;; flags: qr rd ra ; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0
;; QUESTION SECTION:
;; example.com.		IN	A

;; ANSWER SECTION:
;; example.com.		80158	IN	A	93.184.216.34
```

```bash
# Full, call specified DNS server for an AAAA record
cargo run -p dns_resolver -- -s 1.1.1.1 -r AAAA example.com
```
```
;; ->>HEADER<<- opcode: QUERY, rcode: NOERROR, id: 38584
;; flags: qr rd ra ; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0
;; QUESTION SECTION:
;; example.com.		IN	AAAA

;; ANSWER SECTION:
;; example.com.		82169	IN	AAAA	2606:2800:220:1:248:1893:25c8:1946
```

## Resources
- https://howdns.works/
- https://www.statdns.com/rfc/
- https://github.com/jvns/dns-weekend
- https://www.zytrax.com/books/dns/