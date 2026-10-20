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
