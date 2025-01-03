use std::future::Future;

use anyhow::Result;

use crate::{MetadataStorage, Stream};

pub trait Acceptor: Sized {
    type Config;

    fn new(config: Self::Config) -> impl Future<Output = Result<Self>>;

    fn accept(
        &mut self,
        metadata: &mut impl MetadataStorage,
    ) -> impl Future<Output = Result<impl Stream>>;
}
