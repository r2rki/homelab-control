# Observability Compose

This is a starting stack, not a production configuration.

Before exposing it beyond the local lab:

- pin image versions/digests
- set strong credentials
- put Grafana behind the intended reverse proxy
- configure Alertmanager receivers
- add retention policies
- integrate Alloy for host/container logs
- define VictoriaMetrics scrape targets
