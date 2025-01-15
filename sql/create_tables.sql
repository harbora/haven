CREATE TABLE dns_records (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    domain_name TEXT NOT NULL,
    record_type TEXT NOT NULL,
    record_value TEXT NOT NULL,
    ttl INTEGER NOT NULL,
    priority INTEGER,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(domain_name, record_type, record_value)
);

CREATE INDEX idx_dns_records_domain_type ON dns_records (domain_name, record_type);
CREATE INDEX idx_dns_records_domain_name ON dns_records (domain_name);
CREATE INDEX idx_dns_records_updated_at ON dns_records (updated_at);

CREATE TABLE outgoing_dns (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    protocol TEXT NOT NULL,
    config TEXT NOT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_outgoing_dns_name ON outgoing_dns (name);
CREATE INDEX idx_outgoing_dns_updated_at ON outgoing_dns (updated_at);

CREATE TABLE outgoing (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    type TEXT NOT NULL,
    name TEXT NOT NULL,
    primary_dns_id INTEGER NOT NULL REFERENCES outgoing_dns(id),
    secondary_dns_id INTEGER NOT NULL REFERENCES outgoing_dns(id),
    is_direct BOOLEAN NOT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_outgoing_dns_records_name ON outgoing (name);
CREATE INDEX idx_outgoing_dns_records_updated_at ON outgoing (updated_at);
