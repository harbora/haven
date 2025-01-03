use std::future::Future;

use anyhow::Result;

use crate::{MetadataStorage, Stream};

pub trait Inbound: Stream {
    fn create(
        stream: impl Stream,
        metadata: &mut impl MetadataStorage,
    ) -> impl Future<Output = Result<Self>>;
}
