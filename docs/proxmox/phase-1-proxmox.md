# Proxmox Phase 1

## Host roles

One physical Proxmox host runs logical environments:

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

Use a consistent naming convention once the network is finalized.

## Future replication

A future `pve02` can receive selected VM replication from `pve01` using ZFS-backed storage, enabling a controlled recovery workflow.
