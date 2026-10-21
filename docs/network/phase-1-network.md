# Phase 1 Network Plan

The initial network should be simple, secure and extensible.

## Logical segments

| Segment    | Purpose                                        | Example range |
| ---------- | ---------------------------------------------- | ------------- |
| Management | Proxmox, NAS and infrastructure administration | 10.10.10.0/24 |
| Production | Production services                            | 10.10.20.0/24 |
| Acceptance | Test and canary services                       | 10.10.30.0/24 |
| Storage    | NAS and storage traffic                        | 10.10.40.0/24 |
| IoT        | Optional future segment                        | 10.10.50.0/24 |

These ranges are examples only and must be replaced after the router and switch are selected.

## VLAN strategy

VLANs are part of the target Phase 1 design.

The selected router and switch should support VLAN-aware networking. Actual VLAN implementation depends on the capabilities of the selected hardware.

## Phase 1 priorities

1. Stable addressing and centralized DNS/DHCP
2. Management access
3. Network segmentation
4. Firewall rules and inter-segment policy
5. NAS reachability
6. Proxmox reachability
7. Production and Acceptance isolation
8. Monitoring and network visibility

## Traffic policy

Inter-segment traffic should follow a default-deny model where practical.

Production and Acceptance must remain isolated by default.

Management access should be explicitly permitted to infrastructure and required services.

Storage access should be limited to systems that require it.

## Public exposure

Proxmox, Docker, NAS, monitoring systems and the Rust control API must not be directly exposed to the public internet.

External access, if required later, must use an explicitly designed secure access mechanism.

## Naming

Infrastructure should use predictable hostnames rather than relying on IP addresses alone.

Examples:

- `pve01`
- `nas01`
- `prod-app01`
- `acc-app01`
- `monitor01`
- `control01`

## Future expansion

The network design must support a future second Proxmox host and ZFS replication without redesigning the addressing or segmentation model.
