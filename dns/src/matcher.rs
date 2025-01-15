use std::net::IpAddr;

use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagId(pub u64);

#[derive(Debug, Clone, Copy)]
pub struct OutgoingTag {
    pub outgoing: TagId,
    pub primary_dns: TagId,
    pub secondary_dns: TagId,
    pub is_direct: bool,
}

#[async_trait]
pub trait RuleMatcher: Send + Sync + 'static {
    async fn match_domain(&self, domain: &str) -> Result<OutgoingTag>;

    async fn match_ip(&self, ip: &IpAddr) -> Result<OutgoingTag>;
}

#[async_trait]
impl RuleMatcher for () {
    async fn match_domain(&self, _domain: &str) -> Result<OutgoingTag> {
        Ok(OutgoingTag {
            outgoing: TagId(0),
            primary_dns: TagId(0),
            secondary_dns: TagId(0),
            is_direct: true,
        })
    }

    async fn match_ip(&self, _ip: &IpAddr) -> Result<OutgoingTag> {
        Ok(OutgoingTag {
            outgoing: TagId(0),
            primary_dns: TagId(0),
            secondary_dns: TagId(0),
            is_direct: true,
        })
    }
}
