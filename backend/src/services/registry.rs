#[derive(Debug, Clone, Copy)]
pub enum ServiceKind {
    Proxmox,
    Docker,
    Ansible,
    Monitoring,
    Nextcloud,
}

impl ServiceKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Proxmox => "proxmox",
            Self::Docker => "docker",
            Self::Ansible => "ansible",
            Self::Monitoring => "monitoring",
            Self::Nextcloud => "nextcloud",
        }
    }
}
