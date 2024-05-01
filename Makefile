#Server
dev-server:
	 cargo watch -x run
server:
	cargo run .

# Migrations
migration_source := src/lib/db/migrations

migrations.new:
	sqlx migrate add -r ${name} --source ${migration_source}
migrations.migrate:
	sqlx migrate run --source ${migration_source}
migrations.revert:
	sqlx migrate revert --source ${migration_source}
