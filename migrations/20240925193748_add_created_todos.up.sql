-- Add up migration script here
ALTER TABLE todos ADD created timestamptz NOT NULL DEFAULT now();