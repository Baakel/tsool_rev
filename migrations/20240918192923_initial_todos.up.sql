-- Add up migration script here
CREATE TABLE IF NOT EXISTS todos(
	id bigserial primary key not null,
	value text not null,
	done timestamptz default null
);

CREATE TABLE IF NOT EXISTS goals(
	id bigserial primary key not null,
	value text not null,
	done timestamptz default null,
	goal_date timestamptz not null default CURRENT_TIMESTAMP
);
