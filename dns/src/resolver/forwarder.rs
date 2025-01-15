use anyhow::Result;
use hickory_server::{
    authority::{MessageRequest, MessageResponse},
    proto::{
        op::Message,
        rr::{Name, Record, RecordType},
        serialize::binary::BinEncodable,
    },
};

use crate::TagId;

pub struct ForwarderResolver {}

impl ForwarderResolver {
    pub async fn new() -> Self {
        Self {}
    }

    pub async fn lookup(
        &self,
        dns: TagId,
        name: Name,
        ty: RecordType,
        req: &MessageRequest,
    ) -> Result<()> {
        let bytes = req.to_bytes()?;

        Ok(())
    }
}
