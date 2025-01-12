CREATE DATABASE ?;

CREATE TABLE haven.dns_records (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    domain_name TEXT NOT NULL,
    record_type TEXT NOT NULL,
    record_value TEXT NOT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(domain_name, record_type, record_value)
);

CREATE INDEX idx_domain_type ON haven.dns_records (domain_name, record_type);
CREATE INDEX idx_domain_name ON haven.dns_records (domain_name);
CREATE INDEX idx_updated_at ON haven.dns_records (updated_at);