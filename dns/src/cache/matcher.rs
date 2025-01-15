use std::time::Duration;

use anyhow::Result;
use mini_moka::sync::Cache;

use crate::{OutgoingTag, RuleMatcher};

pub struct CachedMatcher {
    matcher: Box<dyn RuleMatcher>,
    cache: Cache<String, OutgoingTag>,
}

impl CachedMatcher {
    pub fn new(cache_size: u64, matcher: impl RuleMatcher, cache_ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(cache_size)
            .time_to_live(cache_ttl)
            .build();

        Self {
            matcher: Box::new(matcher),
            cache,
        }
    }

    pub async fn match_domain(&self, domain: String) -> Result<OutgoingTag> {
        if let Some(res) = self.cache.get(&domain) {
            return Ok(res);
        } else {
            let res = self.matcher.match_domain(&domain).await?;
            self.cache.insert(domain, res);
            return Ok(res);
        }
    }
}
