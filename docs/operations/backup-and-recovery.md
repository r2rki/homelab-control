# Backup and Recovery

## Principle

Replication and snapshots improve recovery speed. Independent backups provide a separate recovery path.

## Phase 1

- Proxmox workloads backed up to the NAS
- Nextcloud data stored on the NAS
- configuration and infrastructure definitions stored in Git
- recovery procedure tested periodically

## Future

When a second Proxmox host exists, selected VMs can be replicated using ZFS replication. Recovery tests should verify that a replicated workload can actually be started and pass application health checks.
