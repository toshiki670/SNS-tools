#!/bin/bash
sqlx db create --database-url="sqlite://passwords.db"
sqlx mig run --database-url="sqlite://passwords.db"
cargo sqlx prepare --database-url="sqlite://passwords.db"

rm passwords.db
