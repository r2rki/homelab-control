# Current State - Phase 1

## Physical topology

One physical Proxmox host plus a separate NAS located near the network equipment.

### Logical environments on the Proxmox host

- **Production**: services intended to be stable and usable
- **Acceptance**: test/canary environment used before production changes
- **Shared**: monitoring, control plane and utility workloads where appropriate

## Design goals

- deterministic operations before automation/AI
- easy recovery from failed changes
- no dependence on proprietary work data
- observable workloads
- repeatable configuration

## Future extension

The architecture must not assume that the current single physical host is permanent. A second Proxmox host can later be added for recovery and ZFS replication without changing the application-level API design.
