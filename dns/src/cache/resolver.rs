use std::time::Duration;

use hickory_server::{
    proto::rr::{Name, RecordType},
    server::ResponseInfo,
};
use mini_moka::sync::Cache;

pub struct ResolverCache {
    cache: Cache<(Name, RecordType), ResponseInfo>,
}

impl ResolverCache {
    pub fn new(cache_size: u64, cache_ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(cache_size)
            .time_to_live(cache_ttl)
            .build();

        Self { cache }
    }

    pub fn get(&self, domain: Name, record_type: RecordType) -> Option<ResponseInfo> {
        self.cache.get(&(domain, record_type))
    }

    pub fn insert(&self, domain: Name, record_type: RecordType, response_info: ResponseInfo) {
        self.cache.insert((domain, record_type), response_info);
    }
}
