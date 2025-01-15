use anyhow::Result;
use hickory_server::proto::{
    rr::{Name, RData, Record, RecordType},
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

    pub async fn lookup(&self, name: Name, ty: RecordType) -> Result<Vec<Record>> {
        let domain = name.to_string();
        let ty_str = ty.to_string();

        let result = sqlx::query!(
            "SELECT record_value, ttl, priority FROM dns_records WHERE domain_name = $1 AND record_type = $2",
            domain,
            ty_str
        )
        .fetch_all(&self.pool)
        .await?;

        let mut records = Vec::with_capacity(result.len());

        for sql_record in result {
            let value = sql_record.record_value;
            let rdata = parse_rdata(name.clone(), ty, &value);

            if let Ok(rdata) = rdata {
                let ttl = sql_record.ttl as u32;

                let record = Record::from_rdata(name.clone(), ttl, rdata);

                records.push(record);
            } else {
                tracing::warn!("Failed to parse record: {}", value);
            }
        }

        Ok(records)
    }
}

fn parse_rdata(name: Name, ty: RecordType, value: &str) -> Result<RData> {
    let tokens = [value];

    let rdata = RData::parse(ty, tokens.iter().map(AsRef::as_ref), Some(&name));

    if let Ok(rdata) = rdata {
        Ok(rdata)
    } else {
        Err(anyhow::anyhow!("Failed to parse record: {}", value))
    }
}
