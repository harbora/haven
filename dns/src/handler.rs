use async_trait::async_trait;
use hickory_server::server::{Request, RequestHandler, ResponseHandler, ResponseInfo};

use crate::utils;

pub struct HavenDNSHandler {}

#[async_trait]
impl RequestHandler for HavenDNSHandler {
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        _response_handle: R,
    ) -> ResponseInfo {
        let query = request.query();

        let _domain = query.name();
        let _ty = query.query_type();

        utils::serve_failed()
    }
}
