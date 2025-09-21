#!/bin/bash

set -e

DB_NAME="SEGURIDAD"
USERNAME="api"
HOST="localhost"
PORT="5432"

read -sp "Enter PostgreSQL password for $USERNAME: " PGPASSWORD
echo
export PGPASSWORD

cleanup() {
    unset PGPASSWORD
    echo "Password environment variable cleared."
}
trap cleanup EXIT

echo "Creating database $DB_NAME..."
if ! psql -h $HOST -p $PORT -U $USERNAME -d postgres -c "SELECT 'CREATE DATABASE $DB_NAME' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '$DB_NAME')\gexec"; then
    echo "ERROR: Failed to create database. Check credentials and permissions."
    exit 1
fi

echo "Setting up tables..."
if ! psql -h $HOST -p $PORT -U $USERNAME -d $DB_NAME -f up.sql; then
    echo "ERROR: Failed to execute up.sql. Check the SQL syntax."
    exit 1
fi

echo "Database setup complete!"
