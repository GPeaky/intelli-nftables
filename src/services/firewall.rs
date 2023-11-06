use crate::{dtos::FirewallRule, error::AppResult};
use rustc_hash::FxHashMap;
use tokio::sync::RwLock;

pub struct FirewallService {
    rules: RwLock<FxHashMap<u32, FirewallRule>>,
}

impl FirewallService {
    pub fn new() -> Self {
        Self {
            rules: RwLock::new(FxHashMap::default()),
        }
    }

    pub async fn open_port(&self, port: u16, championship_id: u32) -> AppResult<()> {
        let mut rules = self.rules.write().await;
        rules.insert(
            championship_id,
            FirewallRule {
                port,
                address: None,
                rule_type: crate::dtos::RuleType::Open,
            },
        );

        Ok(())
    }

    pub async fn open_port_partially(
        &self,
        port: u16,
        address: String,
        championship_id: u32,
    ) -> AppResult<()> {
        let mut rules = self.rules.write().await;
        rules.insert(
            championship_id,
            FirewallRule {
                port,
                address: Some(address.parse().unwrap()),
                rule_type: crate::dtos::RuleType::PartiallyOpen,
            },
        );

        Ok(())
    }

    pub async fn close_port(&self, championship_id: &u32) -> AppResult<()> {
        let mut rules = self.rules.write().await;
        rules.remove(championship_id);

        Ok(())
    }

    pub async fn close_all_ports(&self) -> AppResult<()> {
        let mut rules = self.rules.write().await;
        rules.clear();

        Ok(())
    }
}
