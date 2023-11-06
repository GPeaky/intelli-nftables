use serde::Deserialize;
use std::net::IpAddr;

#[derive(Clone, Copy)]
pub enum RuleType {
    Open,
    PartiallyOpen,
}

pub struct FirewallRule {
    pub port: u16,
    pub address: Option<IpAddr>,
    pub rule_type: RuleType,
}

#[derive(Deserialize)]
pub struct OpenPortForm {
    pub port: u16,
    pub championship_id: u32,
}

#[derive(Deserialize)]
pub struct OpenPortPartiallyForm {
    pub port: u16,
    pub address: String,
    pub championship_id: u32,
}

#[derive(Deserialize)]
pub struct ClosePortForm {
    pub championship_id: u32,
}
