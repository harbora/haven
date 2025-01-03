use std::future::Future;

use anyhow::Result;

use crate::{MetadataStorage, Stream};

pub trait Inbound: Sized {
    type Config;

    fn new(config: Self::Config) -> impl Future<Output = Result<Self>>;

    fn warp(
        &mut self,
        stream: impl Stream,
        metadata: &mut impl MetadataStorage,
    ) -> impl Future<Output = Result<impl Stream>>;
}
