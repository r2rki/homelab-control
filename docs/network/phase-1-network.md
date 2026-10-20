# Phase 1 Network Plan

Initial design should be simple but extensible.

## Logical segments (planned)

| Segment | Purpose | Example range |
|---|---|---|
| Management | Proxmox / admin | 10.10.10.0/24 |
| Production | production services | 10.10.20.0/24 |
| Acceptance | test/canary services | 10.10.30.0/24 |
| Storage | NAS / storage traffic | 10.10.40.0/24 |
| IoT | optional later | 10.10.50.0/24 |

These are example ranges only. Replace them after the actual router/switch is selected.

## Phase 1 priority

1. stable addressing and DNS
2. management access
3. firewall rules
4. NAS reachability
5. Proxmox reachability
6. VLANs when supported and useful

Do not expose Proxmox, Docker or the Rust API directly to the public internet.
