# NAS Plan

The NAS is a separate storage system located next to the router.

## Intended roles

- Nextcloud data storage
- Proxmox backups
- configuration backups
- selected application data
- disaster-recovery source

## Three-HDD design

A three-disk ZFS layout is a candidate, but the exact topology is not locked until disk sizes, models and health are known.

Before committing to a pool:

- record disk model, size, serial and SMART status
- confirm intended redundancy
- confirm backup target exists
- document recovery steps

## Rule

RAID/ZFS redundancy is not a backup strategy by itself. At least one independent recovery path must exist.

## Network

The NAS will use the dedicated Storage network when VLAN support is implemented.

Storage access should be limited to systems that require it, including Proxmox and selected application workloads.

## Future expansion

The NAS should remain independent of the Proxmox compute host and support future expansion to a second Proxmox host with ZFS replication.

The NAS is a backup and storage component, not a replacement for independent backups.
