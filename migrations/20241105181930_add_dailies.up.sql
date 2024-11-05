-- Add up migration script here
CREATE TABLE IF NOT EXISTS dailies(
	id bigserial primary key not null,
	value text not null,
	done bool not null default false,
	created timestamptz not null default CURRENT_TIMESTAMP,
	streak int not null default 0
);
