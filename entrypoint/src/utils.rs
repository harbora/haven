use std::path::Path;

use anyhow::Result;
use tokio::fs;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub async fn load_certs(filename: &Path) -> Result<Vec<CertificateDer<'static>>> {
    let data = fs::read(filename).await?;

    let mut data = data.as_slice();

    // Load and return certificate.
    let res = rustls_pemfile::certs(&mut data);

    let mut certs = Vec::new();

    for r in res {
        certs.push(r?);
    }

    Ok(certs)
}

pub async fn load_private_key(filename: &Path) -> Result<PrivateKeyDer<'static>> {
    let data = fs::read(filename).await?;

    let mut data = data.as_slice();

    // Load and return certificate.
    let res = rustls_pemfile::private_key(&mut data)?;

    let cert = res.ok_or(anyhow::anyhow!("failed to load private key"))?;

    Ok(cert)
}
