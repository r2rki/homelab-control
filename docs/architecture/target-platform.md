# Target Platform Architecture

```text
                       Responsive Web UI
                              |
                              v
                       Rust Control API
                              |
                +-------------+-------------+
                |             |             |
            Proxmox         Docker       Observability
            adapter         adapter        adapters
                |             |             |
                +-------------+-------------+
                              |
                         Policy layer
                              |
                       Ansible workflows
                              |
                   +----------+----------+
                   |                     |
              Acceptance            Production

NAS remains a separate backup/storage system.

Future second Proxmox host:
    pve01 <---- ZFS replication ----> pve02
```

The agent layer, if added later, sits above the policy layer and can request only approved tools.
