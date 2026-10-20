# Application-Aware Patching

The patching system is designed around safety rather than “patch everything.”

## Workflow

```text
SCAN
  |
PLAN
  |
PRE-CHECKS
  |
ACCEPTANCE / CANARY
  |
HEALTH CHECKS
  |
APPROVAL
  |
PRODUCTION
  |
POST-CHECKS
  |
REPORT
```

## Pre-checks

- backup status
- disk space
- service state
- monitoring state
- network reachability
- application health

## Acceptance

Acceptance is patched first. The run stops on failed health checks.

## Production

Production changes require policy validation and explicit approval. The Rust API will not expose arbitrary shell execution.

## Recovery

Where technically appropriate, a pre-change snapshot may be used as a short-term safety measure. Snapshots are not backups.
