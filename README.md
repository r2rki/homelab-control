# Homelab Control Hub

A personal, API-first homelab platform for application operations, DevOps, automation, observability and controlled service management.

> **Status: Phase 1 - Foundation / in development**

The project is deliberately built around a deterministic control plane first. AI agents may be added later, but they will use the same permissioned API/tool interfaces as the web application and will never receive unrestricted shell or root access.

## Project vision

Build a small, production-style operations platform that can:

- manage and observe a Proxmox-based homelab
- separate **Production** and **Acceptance** environments on one physical Proxmox host
- manage Docker workloads and application services
- automate routine operations with Ansible
- implement an application-aware patching workflow
- collect metrics, logs and alerts
- provide a responsive web control panel
- use Git + CI/CD as the source of truth for application changes
- remain ready for a future second Proxmox host with ZFS replication
- optionally gain agentic/AI capabilities only after the deterministic platform is reliable

## Current architecture

Phase 1 is intentionally small:

```text
                         INTERNET
                            |
                     Router / Firewall
                            |
                          Switch
                   _________|_________
                  |                   |
             Proxmox Host            NAS
                  |                3 x HDD
        __________|__________          |
       |          |         |          +-- Backups / snapshots
    PROD VM   ACCEPT VM   Shared VMs
       |          |         |
       +----------+---------+
                  |
             Docker services
                  |
        +---------+---------+
        |                   |
   Rust Control API      Monitoring
        |               Grafana / VM /
        |                Loki / Alloy /
        |                 Alertmanager
        |
     React Web UI
```

### Future recovery architecture

The next physical expansion is **a second standalone Proxmox host**, not a 3-node cluster by default:

```text
               NAS (backup anchor)
                 /            \
                /              \
        Proxmox #1          Proxmox #2
        Production          Recovery / DR
             |                    |
             +---- ZFS replication+
```

This gives us a practical progression:

1. One Proxmox host + NAS
2. Add a second Proxmox host + ZFS replication
3. Evaluate a 3-node HA cluster only if space, budget and workload justify it

## Environments

The words **Production** and **Acceptance** in this repository refer to logical environments hosted on the same physical Proxmox node during Phase 1. They are not physical Proxmox cluster nodes.

The intended promotion flow is:

```text
Git -> CI -> Acceptance -> automated health checks -> approval -> Production
```

## Technology direction

### Backend

- Rust
- Axum
- Tokio
- Serde
- Reqwest
- tracing
- SQLx / PostgreSQL or SQLite when persistence is needed

### Frontend

- React
- TypeScript
- Vite
- responsive UI

### Infrastructure

- Proxmox VE
- Docker / Compose
- Ansible
- GitHub Actions

### Observability

- Grafana
- VictoriaMetrics
- Loki
- Grafana Alloy
- Alertmanager
- optional Uptime Kuma for service availability

## Repository layout

```text
backend/       Rust API
web/           React web UI
ansible/       Infrastructure and application automation
infra/         Docker Compose and infrastructure definitions
monitoring/    Observability configuration
scripts/       Small operational helpers
docs/          Architecture, network, storage and runbooks
.github/       CI workflows
```

## Phase roadmap

- [x] Architecture baseline
- [ ] Sprint 1 - Network foundation
- [ ] Sprint 2 - NAS foundation
- [ ] Sprint 3 - Backup and recovery
- [ ] Sprint 4 - Proxmox foundation
- [ ] Sprint 5 - Production / Acceptance environments
- [ ] Sprint 6 - Docker platform
- [ ] Sprint 7 - Monitoring and observability
- [ ] Sprint 8 - Ansible automation
- [ ] Sprint 9 - Application-aware patching
- [ ] Sprint 10 - Rust API
- [ ] Sprint 11 - Responsive web UI
- [ ] Sprint 12 - CI/CD
- [ ] Sprint 13 - Acceptance -> Production promotion
- [ ] Sprint 14 - Security and resilience hardening
- [ ] Sprint 15 - Portfolio documentation and demos
- [ ] Future - second Proxmox host + ZFS replication
- [ ] Future - optional agents / AI-assisted operations
- [ ] Future - Android / iOS / desktop clients

## Security principles

1. No credentials in Git.
2. No unrestricted shell execution from the API.
3. Integrations are exposed through controlled service adapters.
4. Destructive operations require policy checks and, for production, explicit approval.
5. Backups are separate from replication and snapshots.
6. Acceptance is used as the canary environment before production changes.

## Non-goals for Phase 1

- Kubernetes
- microservice fragmentation
- Ceph
- 3-node HA cluster
- AI-controlled infrastructure
- direct public exposure of management APIs

These can be evaluated later based on actual need.

## Getting started

The repository is intentionally safe to clone before the lab hardware is final.

```bash
cp .env.example .env
```

Build the Rust API:

```bash
cd backend
cargo run
```

The starter API exposes:

- `GET /health`
- `GET /api/v1`

The web client is a scaffold for the responsive dashboard and will be expanded as the backend contract stabilizes.

## Git workflow

Recommended branches:

- `main` - releasable state
- `feature/*` - changes
- `fix/*` - fixes

Suggested commit style:

```text
feat: add proxmox health adapter
fix: handle docker API timeout
ops: update monitoring configuration
docs: document patch workflow
```

## Safety note

Examples in this repository are designed for a personal lab. Replace all placeholder addresses, IDs, credentials and storage paths before deploying anything.
