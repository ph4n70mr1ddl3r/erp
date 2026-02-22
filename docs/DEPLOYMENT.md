# Deployment Guide

## Prerequisites

- Linux server (Ubuntu 22.04+ recommended)
- 1GB RAM minimum
- 10GB disk space
- Domain name (optional, for SSL)

## Option 1: Manual Deployment

### 1. Install Dependencies

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install SQLite
sudo apt-get install -y sqlite3
```

### 2. Build the Application

```bash
# Clone or copy the project
cd /opt/erp

# Build backend
cargo build --release

# Build frontend
cd frontend
npm install
npm run build
```

### 3. Configure Environment

Create `/opt/erp/.env`:
```env
DATABASE_URL=sqlite:/opt/erp/data/erp.db?mode=rwc
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
JWT_SECRET=your-very-long-random-secret-key-at-least-32-characters
RUST_LOG=info
```

### 4. Create Data Directory

```bash
mkdir -p /opt/erp/data
```

### 5. Create Systemd Service

Create `/etc/systemd/system/erp.service`:
```ini
[Unit]
Description=ERP System
After=network.target

[Service]
Type=simple
User=erp
WorkingDirectory=/opt/erp
EnvironmentFile=/opt/erp/.env
ExecStart=/opt/erp/target/release/erp-server
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable erp
sudo systemctl start erp
```

### 6. Serve Frontend with Nginx

Create `/etc/nginx/sites-available/erp`:
```nginx
server {
    listen 80;
    server_name your-domain.com;

    # Frontend
    location / {
        root /opt/erp/frontend/dist;
        try_files $uri $uri/ /index.html;
    }

    # API
    location /auth {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    location /api {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    location /health {
        proxy_pass http://127.0.0.1:3000;
    }
}
```

Enable:
```bash
sudo ln -s /etc/nginx/sites-available/erp /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 7. SSL with Let's Encrypt

```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

## Option 2: Docker Deployment (Recommended)

The project includes Docker support out of the box.

### 1. Quick Deploy

```bash
# Set required environment variable
export JWT_SECRET=your-very-long-random-secret-key

# Build and run
docker-compose up -d

# Access at http://localhost
```

### 2. Production Configuration

Edit `docker-compose.yml` or set environment variables:

```bash
export JWT_SECRET=your-secure-key-at-least-32-chars
export JWT_EXPIRATION=86400
export RUST_LOG=info
docker-compose up -d
```

### 3. Files Included

- `Dockerfile` - Multi-stage build for the Rust backend
- `docker-compose.yml` - Full stack (backend + nginx frontend)
- `nginx.conf` - Reverse proxy configuration
- `.dockerignore` - Excludes unnecessary files from build

### 4. Useful Commands

```bash
# View logs
docker-compose logs -f

# Rebuild after code changes
docker-compose up --build

# Stop services
docker-compose down

# Persistent data location
docker volume inspect erp_erp-data
```

## Security Checklist

- [ ] Change JWT_SECRET to a long random string
- [ ] Enable HTTPS with valid SSL certificate
- [ ] Set up firewall (ufw allow 80, 443, 22)
- [ ] Create non-root user for running the service
- [ ] Regular database backups
- [ ] Set up log rotation
- [ ] Keep system packages updated

## Database Backups

### Manual Backup
```bash
sqlite3 /opt/erp/data/erp.db ".backup /opt/erp/backups/erp-$(date +%Y%m%d).db"
```

### Automated Daily Backups

Create `/etc/cron.daily/erp-backup`:
```bash
#!/bin/bash
BACKUP_DIR=/opt/erp/backups
mkdir -p $BACKUP_DIR
sqlite3 /opt/erp/data/erp.db ".backup $BACKUP_DIR/erp-$(date +%Y%m%d).db"
# Keep only last 30 days
find $BACKUP_DIR -name "erp-*.db" -mtime +30 -delete
```

```bash
sudo chmod +x /etc/cron.daily/erp-backup
```

## Monitoring

### Health Check Endpoint
```bash
curl http://localhost:3000/health
# {"service":"erp-api","status":"healthy"}
```

### Logs
```bash
# Systemd logs
sudo journalctl -u erp -f

# Application logs (if RUST_LOG=debug)
# Check /var/log/syslog or journalctl
```

## Scaling

For high-availability deployments:

1. **Database**: Migrate from SQLite to PostgreSQL
2. **Load Balancer**: Use nginx or HAProxy
3. **Multiple Instances**: Run multiple API servers
4. **Redis**: For session management and caching
5. **CDN**: For static frontend assets

## Troubleshooting

### Server won't start
- Check logs: `journalctl -u erp -n 50`
- Verify .env file exists and has correct values
- Check database permissions

### Database locked
- SQLite doesn't handle concurrent writes well
- Consider migrating to PostgreSQL for production

### Frontend not connecting to API
- Check CORS settings (should allow your domain)
- Verify API URL in frontend/src/api/client.ts
- Check nginx proxy configuration
