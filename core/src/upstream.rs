#[derive(Debug, Clone)]
pub struct Upstream {
    pub id: u64,
    pub ty: UpstreamType,
}

#[derive(Debug, Clone)]
pub enum UpstreamType {
    Direct,
    Reject,
    UnixSocket(String),
}
