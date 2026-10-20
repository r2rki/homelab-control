# Monitoring

Phase 1 observability target:

- Grafana for dashboards
- VictoriaMetrics for metrics
- Loki for logs
- Grafana Alloy for telemetry collection
- Alertmanager for alert routing
- optional Uptime Kuma for simple service-availability checks

The stack should answer three questions quickly:

1. Is the service up?
2. What changed?
3. Why did it fail?
