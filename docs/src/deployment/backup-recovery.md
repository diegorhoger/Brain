# Backup & Recovery

Comprehensive backup and recovery strategies for Brain AI production deployments, ensuring data safety and business continuity.

## Overview

Brain AI stores critical data across multiple components that require different backup strategies:

- **Memory System**: Semantic, episodic, and procedural memories
- **Concept Graph**: Dynamic knowledge relationships
- **Learning State**: Character prediction models and segment discovery
- **Configuration**: System settings and user preferences
- **Logs**: Operational and audit logs

## Backup Strategy

### Automated Daily Backups

Configure automated backups using the provided scripts:

```bash
# Set up daily backup cron job
crontab -e
# Add: 0 2 * * * /opt/brain-ai/scripts/backup.sh daily

# Backup script configuration
export BACKUP_RETENTION_DAYS=30
export BACKUP_LOCATION="/backups/brain-ai"
export S3_BUCKET="brain-ai-backups"  # Optional cloud storage
```

### Backup Types

#### 1. Full System Backup

Complete backup including all data and configuration:

```bash
# Manual full backup
./scripts/backup.sh full

# What's included:
# - Memory database (SQLite/PostgreSQL)
# - Concept graph data
# - Learning models and weights
# - Configuration files
# - User data and preferences
```

#### 2. Incremental Backup

Daily incremental backups for efficiency:

```bash
# Automated incremental backup
./scripts/backup.sh incremental

# Backs up only:
# - New memories since last backup
# - Updated concept relationships
# - Recent learning progress
# - Log files
```

#### 3. Critical Data Backup

Essential data only for emergency recovery:

```bash
# Critical data backup (fastest)
./scripts/backup.sh critical

# Includes:
# - Core memory database
# - Primary concept graph
# - Essential configuration
```

## Backup Configuration

### Local Backup Setup

```toml
# config/backup.toml
[backup]
enabled = true
schedule = "0 2 * * *"  # Daily at 2 AM
retention_days = 30
compression = "gzip"

[backup.local]
path = "/backups/brain-ai"
max_size_gb = 100
cleanup_old = true

[backup.verification]
verify_after_backup = true
test_restore_weekly = true
```

### Cloud Backup Integration

```bash
# AWS S3 configuration
export AWS_ACCESS_KEY_ID=your_access_key
export AWS_SECRET_ACCESS_KEY=your_secret_key
export AWS_DEFAULT_REGION=us-west-2

# Google Cloud Storage
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/service-account.json"
export GCS_BUCKET="brain-ai-backups"

# Azure Blob Storage
export AZURE_STORAGE_ACCOUNT=your_account
export AZURE_STORAGE_KEY=your_key
```

### Backup Verification

```bash
# Verify backup integrity
./scripts/verify-backup.sh /backups/brain-ai/backup-2024-01-01.tar.gz

# Test restore process (safe environment)
./scripts/test-restore.sh backup-2024-01-01.tar.gz
```

## Recovery Procedures

### Emergency Recovery

#### 1. Quick Recovery (Service Restart)

For minor issues or corruption:

```bash
# Stop Brain AI service
systemctl stop brain-ai

# Restore from latest backup
./scripts/restore.sh latest

# Verify system integrity
./scripts/health-check.sh

# Restart service
systemctl start brain-ai
```

#### 2. Full System Recovery

For complete system failure:

```bash
# 1. Prepare clean environment
sudo systemctl stop brain-ai
sudo rm -rf /opt/brain-ai/data/*

# 2. Restore from backup
./scripts/restore.sh full /backups/brain-ai/backup-2024-01-01.tar.gz

# 3. Verify configuration
./scripts/verify-config.sh

# 4. Test system functionality
./scripts/integration-test.sh

# 5. Start service
sudo systemctl start brain-ai
```

#### 3. Point-in-Time Recovery

Restore to specific timestamp:

```bash
# List available backups
./scripts/list-backups.sh

# Restore to specific date/time
./scripts/restore.sh point-in-time "2024-01-01 14:30:00"

# Verify data integrity
./scripts/verify-data.sh
```

### Recovery Validation

```bash
# Post-recovery validation checklist
./scripts/post-recovery-check.sh

# Checks performed:
# ✓ Memory system accessibility
# ✓ Concept graph integrity  
# ✓ Learning model functionality
# ✓ API endpoint responses
# ✓ Authentication system
# ✓ Performance benchmarks
```

## Disaster Recovery

### Multi-Site Recovery

For production environments with geographic redundancy:

```bash
# Primary site failure - activate secondary
./scripts/failover-to-secondary.sh

# Sync data from backup site
./scripts/sync-from-backup-site.sh

# Validate secondary site functionality
./scripts/validate-secondary.sh
```

### Recovery Time Objectives (RTO)

| Scenario | Target RTO | Procedure |
|----------|------------|-----------|
| Service restart | 2 minutes | Quick recovery |
| Data corruption | 15 minutes | Restore from latest backup |
| Full system failure | 1 hour | Complete system rebuild |
| Site disaster | 4 hours | Geographic failover |

### Recovery Point Objectives (RPO)

| Data Type | Target RPO | Backup Frequency |
|-----------|------------|------------------|
| Critical memories | 1 hour | Continuous replication |
| Concept relationships | 4 hours | Every 4 hours |
| Learning progress | 24 hours | Daily backup |
| Configuration | 24 hours | Daily backup |

## Monitoring and Alerting

### Backup Monitoring

```bash
# Monitor backup job status
./scripts/monitor-backups.sh

# Set up alerts for backup failures
crontab -e
# Add: 0 3 * * * /opt/brain-ai/scripts/check-backup-status.sh
```

### Health Checks

```bash
# Automated health monitoring
./scripts/health-check.sh --continuous

# Alerts configured for:
# - Backup job failures
# - Storage space issues
# - Data corruption detection
# - Recovery test failures
```

## Security Considerations

### Backup Encryption

```bash
# Encrypt backups at rest
export BACKUP_ENCRYPTION_KEY="your-encryption-key"
./scripts/backup.sh --encrypt

# Decrypt for recovery
./scripts/restore.sh --decrypt backup-encrypted.tar.gz.enc
```

### Access Control

```bash
# Secure backup storage permissions
chmod 600 /backups/brain-ai/*
chown brain-ai:brain-ai /backups/brain-ai/*

# Cloud storage IAM policies
# - Backup service: read/write access
# - Recovery team: read-only access
# - Auditors: list-only access
```

## Backup Scripts Reference

### Available Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| `backup.sh` | Create backups | `./backup.sh [full\|incremental\|critical]` |
| `restore.sh` | Restore from backup | `./restore.sh [latest\|full\|point-in-time]` |
| `verify-backup.sh` | Verify backup integrity | `./verify-backup.sh <backup-file>` |
| `health-check.sh` | System health validation | `./health-check.sh [--continuous]` |
| `monitor-backups.sh` | Backup monitoring | `./monitor-backups.sh` |

### Script Configuration

```bash
# scripts/backup-config.env
BACKUP_RETENTION_DAYS=30
BACKUP_COMPRESSION=gzip
BACKUP_ENCRYPTION=true
CLOUD_BACKUP_ENABLED=true
VERIFICATION_ENABLED=true
```

## Troubleshooting

### Common Issues

#### Backup Failures

```bash
# Check disk space
df -h /backups

# Check permissions
ls -la /backups/brain-ai/

# Check backup logs
tail -f /var/log/brain-ai/backup.log
```

#### Recovery Issues

```bash
# Verify backup integrity
./scripts/verify-backup.sh <backup-file>

# Check system dependencies
./scripts/check-dependencies.sh

# Validate configuration
./scripts/verify-config.sh
```

### Emergency Contacts

- **Primary DBA**: Contact for database recovery issues
- **System Administrator**: Infrastructure and storage issues  
- **DevOps Team**: Automation and deployment issues
- **Security Team**: Encryption and access control

## Best Practices

### Backup Best Practices

1. **Test Recovery Regularly**: Monthly recovery drills
2. **Multiple Backup Locations**: Local + cloud storage
3. **Encryption**: Always encrypt sensitive data
4. **Monitoring**: Automated backup success/failure alerts
5. **Documentation**: Keep recovery procedures updated

### Recovery Best Practices

1. **Validate Before Restore**: Always verify backup integrity
2. **Test Environment First**: Test recovery in staging
3. **Communicate**: Notify stakeholders of recovery operations
4. **Document**: Log all recovery actions and outcomes
5. **Post-Recovery Validation**: Comprehensive system testing

This backup and recovery system ensures Brain AI data safety and enables rapid recovery from any failure scenario.
