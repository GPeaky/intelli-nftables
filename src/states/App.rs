use std::sync::Arc;

use crate::services::FirewallService;

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub firewall: FirewallService,
}

impl AppStateInner {
    pub fn new() -> Self {
        Self {
            firewall: FirewallService::new(),
        }
    }
}
