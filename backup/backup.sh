#!/bin/bash

# Define variables
BACKUP_DIR="/backup"
BACKUP_FILE="${BACKUP_DIR}/backup_$(date +%Y%m%d_%H%M%S).sql.gz"

# Ensure backup directory exists
mkdir -p "$BACKUP_DIR"

# Run pg_dump and compress the output
PGPASSWORD="$POSTGRES_PASSWORD" pg_dump -h postgres -U "$POSTGRES_USER" "$POSTGRES_DB" | gzip > "$BACKUP_FILE"

# (Optional) Remove backups older than 7 days
find "$BACKUP_DIR" -type f -name "*.sql.gz" -mtime +7 -exec rm {} \;
