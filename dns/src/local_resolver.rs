use anyhow::Result;
use hickory_server::proto::{
    rr::{LowerName, RData, RecordType},
    serialize::txt::RDataParser,
};
use sqlx::{Pool, Postgres};

pub struct LocalResolver {
    pool: Pool<Postgres>,
}

impl LocalResolver {
    pub async fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn lookup(&self, domain: &LowerName, ty: RecordType) -> Result<Vec<RData>> {
        let domain = domain.to_string();
        let ty_str = ty.to_string();

        let result = sqlx::query!(
            "SELECT record_value FROM dns_records WHERE domain_name = $1 AND record_type = $2",
            domain,
            ty_str
        )
        .fetch_all(&self.pool)
        .await?;

        let mut records = Vec::with_capacity(result.len());

        for record in result {
            let value = record.record_value;
            let record = RData::try_from_str(ty, &value);

            if let Ok(record) = record {
                records.push(record);
            } else {
                tracing::warn!("Failed to parse record: {}", value);
            }
        }

        Ok(records)
    }
}
