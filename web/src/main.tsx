import React from 'react';
import { createRoot } from 'react-dom/client';
import './styles.css';

function Card({ title, value, detail }: { title: string; value: string; detail: string }) {
  return (
    <section className="card">
      <div className="card-title">{title}</div>
      <div className="card-value">{value}</div>
      <div className="card-detail">{detail}</div>
    </section>
  );
}

function App() {
  return (
    <main className="shell">
      <header className="header">
        <div>
          <p className="eyebrow">HOMELAB CONTROL</p>
          <h1>Operations Dashboard</h1>
          <p className="muted">Phase 1 scaffold — deterministic control first.</p>
        </div>
        <div className="health-pill"><span className="dot" /> API online</div>
      </header>

      <div className="grid">
        <Card title="Production" value="—" detail="Environment status" />
        <Card title="Acceptance" value="—" detail="Canary environment" />
        <Card title="Hosts" value="—" detail="Proxmox inventory" />
        <Card title="Alerts" value="—" detail="Open incidents" />
      </div>

      <section className="panel">
        <h2>Roadmap</h2>
        <div className="roadmap">
          <span className="done">Network</span>
          <span className="done">NAS</span>
          <span>Proxmox</span>
          <span>Monitoring</span>
          <span>Ansible</span>
          <span>Patching</span>
          <span>Rust API</span>
          <span>CI/CD</span>
          <span>Agents</span>
        </div>
      </section>
    </main>
  );
}

createRoot(document.getElementById('root')!).render(
  <React.StrictMode><App /></React.StrictMode>,
);
