# ADR 0002: Agents Stay Behind the Control Plane

## Decision

Future agents may propose and execute operations only through the same permissioned service/API layer used by the web client.

## Rules

- no raw root shell access
- tool allowlists
- role-based permissions
- validation before destructive actions
- explicit production approval
- audit log for state-changing operations

## Rationale

The deterministic control plane must remain reliable even when AI features are disabled or unavailable.
