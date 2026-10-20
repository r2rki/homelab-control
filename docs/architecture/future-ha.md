# Future HA / Recovery Architecture

## Preferred expansion path

Do not optimize Phase 1 for a 3-node cluster. The first physical expansion should be a second standalone Proxmox host.

```text
             NAS / backup anchor
                 /       \
                /         \
        Proxmox #1      Proxmox #2
          PROD           DR / UAT
             \           /
              \-- ZFS --/
```

## Why

- lower cost and complexity
- practical disaster recovery learning
- fast recovery from a broken host
- a natural target for ZFS replication
- easy future migration to a cluster

## Important distinction

Snapshots and replication improve recovery time but do not replace independent backups. The NAS remains the backup anchor.

## Future 3-node option

A third physical host can later be introduced if there is a real requirement for Proxmox cluster quorum and HA. That is a future phase, not a Phase 1 dependency.
