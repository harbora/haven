use hickory_server::{
    proto::op::{Header, ResponseCode},
    server::ResponseInfo,
};

pub fn serve_failed() -> ResponseInfo {
    let mut header = Header::new();
    header.set_response_code(ResponseCode::ServFail);
    header.into()
}
