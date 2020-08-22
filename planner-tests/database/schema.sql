-- \i this file, psql lives in the folder you ran it in

CREATE TABLE appointments (
  id BIGSERIAL PRIMARY KEY,
  description TEXT default '' NOT NULL,
  stupid_id_to_convert BIGINT NOT NULL,
  timestamp timestamptz default now() NOT NULL
);
CREATE INDEX idx_appointments__id ON appointments(id);
CREATE INDEX idx_appointments__timestamp ON appointments(timestamp);

-- GRANT SELECT, INSERT, UPDATE, DELETE ON ALL SEQUENCES IN SCHEMA public TO olga;
-- GRANT SELECT, UPDATE ON ALL SEQUENCES IN SCHEMA public TO olga;
