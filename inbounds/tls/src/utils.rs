use anyhow::Result;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub fn load_certs(data: &[u8]) -> Result<Vec<CertificateDer<'static>>> {
    let mut data = data;

    // Load and return certificate.
    let res = rustls_pemfile::certs(&mut data);

    let mut certs = Vec::new();

    for r in res {
        certs.push(r?);
    }

    Ok(certs)
}

pub fn load_private_key(data: &[u8]) -> Result<PrivateKeyDer<'static>> {
    let mut data = data;

    // Load and return certificate.
    let res = rustls_pemfile::private_key(&mut data)?;

    let cert = res.ok_or(anyhow::anyhow!("failed to load private key"))?;

    Ok(cert)
}
