# Proxmox Phase 1

## Host roles

One physical Proxmox host runs the following logical environments:

- Production
- Acceptance
- Shared services

## Storage

Preferred direction:

- fast local NVMe for Proxmox and VM/container workloads
- separate NAS for backups and shared storage needs

The exact storage design depends on the purchased hardware.

## Naming examples

```text
pve01
prod-app01
prod-db01
acc-app01
monitor01
control01
```
