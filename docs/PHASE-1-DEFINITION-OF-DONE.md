# Phase 1 Definition of Done

Phase 1 is complete when the lab can be rebuilt from documented configuration and passes these checks:

- network addressing and DNS are documented
- management interfaces are not publicly exposed
- NAS storage is healthy and monitored
- backup job completes successfully
- at least one restore has been tested
- Proxmox host is patched and documented
- Production and Acceptance workloads are logically separated
- Docker services use persistent, documented storage
- monitoring collects host/service signals
- Ansible can perform non-destructive health checks
- patch discovery is automated
- patch execution uses Acceptance before Production
- Rust API passes CI
- web UI builds in CI
- secrets are absent from Git history
