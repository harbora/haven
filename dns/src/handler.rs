use anyhow::Result;
use async_trait::async_trait;
use hickory_server::{
    authority::MessageResponseBuilder,
    proto::{
        op::Header,
        rr::{Name, Record},
    },
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
};

use crate::{utils, CachedMatcher, LocalResolver};

pub struct HavenDNSHandler {
    matcher: CachedMatcher,
    local_resolver: LocalResolver,
}

impl HavenDNSHandler {
    pub fn new(matcher: CachedMatcher, local_resolver: LocalResolver) -> Self {
        Self {
            matcher,
            local_resolver,
        }
    }

    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        response_handle: R,
    ) -> Result<ResponseInfo> {
        let mut response_handle = response_handle;

        let query = request.query();
        let name = Name::from(query.name());

        let builder = MessageResponseBuilder::from_message_request(&request);
        let header = Header::response_from_request(request.header());

        // resolve local first.
        let res = self.local_resolver.lookup(name, query.query_type()).await?;

        if !res.is_empty() {
            let answers: Vec<&Record> = res.iter().map(|r| r).collect();

            let response = builder.build(header, answers, vec![], None, vec![]);
            let response_info = response_handle.send_response(response).await?;

            return Ok(response_info);
        }

        let response = builder.build_no_records(header);
        let response_info = response_handle.send_response(response).await?;

        // match rule
        let domain = Name::from(query.name()).to_lowercase().to_utf8();
        let _tag_id = self.matcher.match_domain(domain).await?;

        // resolve domain by outgoing dns

        Ok(response_info)
    }
}

#[async_trait]
impl RequestHandler for HavenDNSHandler {
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        _response_handle: R,
    ) -> ResponseInfo {
        let res = self.handle_request(request, _response_handle).await;

        match res {
            Ok(response_info) => response_info,
            Err(e) => {
                tracing::warn!("handle request failed: {e}");
                utils::serve_failed()
            }
        }
    }
}
