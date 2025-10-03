#!/bin/bash

set -e

DB_NAME="SEGURIDAD"
PG_USER="api"
PG_PORT="5432"
CLUSTER_NAME="main"
SQL_DIR=$(pwd)
TEMP_DIR="/tmp/pg_setup_$$"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                      WARNING!                                ║"
echo "║ This script will DESTROY ALL PostgreSQL databases            ║"
echo "║ and recreate the cluster. ALL DATA WILL BE LOST!             ║"
echo "║                                                              ║"
echo "║ Type 'DESTROY' to continue, or anything else to abort:       ║"
echo "╚══════════════════════════════════════════════════════════════╝"

read -r confirmation
if [ "$confirmation" != "DESTROY" ]; then
    echo "Aborted. No changes were made."
    exit 1
fi

read -sp "Enter PostgreSQL password for $PG_USER: " PGPASSWORD
echo
export PGPASSWORD

cleanup() {
    unset PGPASSWORD
    echo "Password environment variable cleared."
    if [ -d "$TEMP_DIR" ]; then
        echo "Cleaning up temporary directory..."
        sudo rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

echo "Force stopping PostgreSQL and cleaning stale processes..."
sudo systemctl stop postgresql || true

echo "Killing any remaining PostgreSQL processes..."
sudo pkill -9 postgres || true
sudo pkill -9 postmaster || true

echo "Cleaning up stale PID files..."
sudo find /var/run/postgresql -name "*.pid" -delete 2>/dev/null || true
sudo rm -f /var/run/postgresql/*.pid 2>/dev/null || true

echo "Checking existing clusters..."
CLUSTER_INFO=$(pg_lsclusters)

if [ -z "$CLUSTER_INFO" ]; then
    echo "No existing clusters found. Detecting PostgreSQL version..."
    PG_VERSION=$(psql --version | grep -oE '[0-9]+' | head -1)
    CLUSTER_NAME="main"
else
    PG_VERSION=$(echo "$CLUSTER_INFO" | awk 'NR==2 {print $1}')
    CLUSTER_NAME=$(echo "$CLUSTER_INFO" | awk 'NR==2 {print $2}')
fi
echo "Working with PostgreSQL version: $PG_VERSION, cluster: $CLUSTER_NAME"

echo "Removing existing cluster..."
sudo pg_dropcluster --stop "$PG_VERSION" "$CLUSTER_NAME" 2>/dev/null || echo "Cluster might not exist or already removed"

if pg_lsclusters | grep -q "$PG_VERSION.*$CLUSTER_NAME"; then
    echo "Force removing cluster directory..."
    sudo rm -rf "/var/lib/postgresql/$PG_VERSION/$CLUSTER_NAME"
fi

echo "Creating new cluster..."
sudo pg_createcluster "$PG_VERSION" "$CLUSTER_NAME" --start

echo "Waiting for PostgreSQL to start..."
sudo systemctl start postgresql
sleep 5

if ! sudo systemctl is-active --quiet postgresql; then
    echo "PostgreSQL failed to start. Checking logs..."
    sudo journalctl -u postgresql -n 20 --no-pager
    exit 1
fi

echo "Creating temporary directory and copying SQL files..."
mkdir -p "$TEMP_DIR"
cp "$SQL_DIR"/up.sql "$TEMP_DIR"/
sudo chown -R postgres:postgres "$TEMP_DIR"

echo "Recreating user and database..."

sudo -u postgres psql -c "DROP ROLE IF EXISTS \"$PG_USER\";"

sudo -u postgres psql -c "CREATE ROLE \"$PG_USER\" WITH PASSWORD '$PGPASSWORD' LOGIN;"

sudo -u postgres psql -c "DROP DATABASE IF EXISTS \"$DB_NAME\";"

sudo -u postgres psql -c "CREATE DATABASE \"$DB_NAME\" OWNER \"$PG_USER\";"


echo "Running database setup..."
if [ -f "$TEMP_DIR/up.sql" ]; then
    sudo -u postgres psql -h localhost -p "$PG_PORT" -U "$PG_USER" -d "$DB_NAME" -f "$TEMP_DIR/up.sql"
else
    echo "WARNING: up.sql file not found. Database is empty."
fi

echo "Verifying setup..."
if sudo -u postgres psql -h localhost -p "$PG_PORT" -U "$PG_USER" -d "$DB_NAME" -c "\dt" 2>/dev/null | grep -q "No relations found"; then
    echo "WARNING: No tables found in database $DB_NAME"
else
    echo "Tables created successfully:"
    sudo -u postgres psql -h localhost -p "$PG_PORT" -U "$PG_USER" -d "$DB_NAME" -c "\dt"
fi

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                   CLUSTER RESET COMPLETE!                    ║"
echo "║                                                              ║"
echo "║ PostgreSQL cluster has been completely destroyed and         ║"
echo "║ recreated. All previous data has been lost.                  ║"
echo "╚══════════════════════════════════════════════════════════════╝"
