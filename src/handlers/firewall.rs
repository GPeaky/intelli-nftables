#[inline(always)]
pub async fn open_port() -> &'static str {
    "Open Firewall"
}

#[inline(always)]
pub async fn open_port_partially() -> &'static str {
    "Open Firewall Partially"
}

#[inline(always)]
pub async fn close_port() -> &'static str {
    "Close Firewall"
}

#[inline(always)]
pub async fn close_all_ports() -> &'static str {
    "Close All Firewall"
}
