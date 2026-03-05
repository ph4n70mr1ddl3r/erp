ALTER TABLE non_conformance_reports ADD COLUMN product_id TEXT;

CREATE INDEX IF NOT EXISTS idx_ncr_product ON non_conformance_reports(product_id);
