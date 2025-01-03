use std::collections::BTreeMap;

pub struct CertKey {
    pub cert: Vec<u8>,
    pub key: Vec<u8>,
}

pub struct TLSInboundConfig {
    pub cert_key: BTreeMap<String, CertKey>,
    pub alpn_protocols: Vec<Vec<u8>>,
}
