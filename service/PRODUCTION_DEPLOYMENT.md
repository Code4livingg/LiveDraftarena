# LiveDraft Arena - Production Deployment Guide

## Overview

The LiveDraft Arena backend service has been prepared for production deployment with the following production-ready features:

## ✅ Production Features Implemented

### 1. Network Configuration
- **Bind Address**: Configurable via `BIND_ADDRESS` (defaults to `0.0.0.0`)
- **Port Configuration**: Configurable via `PORT` environment variable
- **Public Access**: Binds to all interfaces for VPS/cloud deployment

### 2. CORS Security
- **Configurable Origins**: Set via `CORS_ORIGINS` environment variable
- **Production Mode**: Restricts to specific frontend domains
- **Development Mode**: Allows all origins (`*`) for local testing

### 3. Logging Configuration
- **Production Logging**: Structured, compact format for monitoring
- **Configurable Levels**: Set via `RUST_LOG` environment variable
- **Performance Optimized**: Removes module paths and thread IDs in production

### 4. Deployment Modes
- **Development Mode**: Debug logging, open CORS, cargo run
- **Production Mode**: Info logging, restricted CORS, optimized binary

## 🚀 Quick Production Setup

### 1. Environment Configuration

Create `service/.env`:
```bash
# Required
LIVEDRAFT_APP_ID=your_deployed_app_id_here

# Production networking
BIND_ADDRESS=0.0.0.0
PORT=8080

# Production CORS (replace with your frontend domain)
CORS_ORIGINS=https://your-frontend-domain.com

# Production logging
RUST_LOG=info
DEPLOYMENT_MODE=production
```

### 2. Start Production Service

```bash
cd service
DEPLOYMENT_MODE=production ./start.sh
```

## 🔧 Configuration Options

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `LIVEDRAFT_APP_ID` | *required* | Deployed application ID from Conway testnet |
| `BIND_ADDRESS` | `0.0.0.0` | Network interface to bind to |
| `PORT` | `8080` | HTTP port for the service |
| `CORS_ORIGINS` | `*` (dev) / *required* (prod) | Comma-separated allowed origins |
| `RUST_LOG` | `debug` (dev) / `info` (prod) | Logging level |
| `DEPLOYMENT_MODE` | `development` | `development` or `production` |
| `LINERA_WALLET_PATH` | `~/.config/linera/wallet.json` | Path to Linera wallet |
| `LIVEDRAFT_CHAIN_ID` | *auto* | Override default chain ID |

### CORS Configuration Examples

```bash
# Single domain
CORS_ORIGINS=https://livedraft.example.com

# Multiple domains
CORS_ORIGINS=https://livedraft.example.com,https://www.livedraft.example.com

# Development (allow all)
CORS_ORIGINS=*
```

## 🏭 Production Deployment Steps

### 1. Server Preparation
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Linera CLI (follow official docs)
# Initialize wallet
linera wallet init
```

### 2. Application Deployment
```bash
# Clone repository
git clone your-repo
cd livedraft-arena

# Deploy contracts
./scripts/deploy_conway.sh
# Note the Application ID output
```

### 3. Service Configuration
```bash
cd service

# Create production config
cp .env.production.example .env
# Edit .env with your Application ID and frontend domain
```

### 4. Start Service
```bash
# Production mode
DEPLOYMENT_MODE=production ./start.sh
```

### 5. Verify Deployment
```bash
# Health check
curl http://your-server:8080/health

# GraphQL test
curl -X POST http://your-server:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "query { rooms { chainId roomName } }"}'
```

## 🔍 Production Monitoring

### Health Endpoints
- `GET /health` - Service health status
- `GET /graphql` - GraphQL endpoint
- `GET /playground` - GraphQL playground (development)

### Logging
- Structured JSON logs for monitoring tools
- Configurable log levels via `RUST_LOG`
- Request tracing with player identity context

### Error Handling
- Graceful error responses
- No sensitive information in error messages
- Proper HTTP status codes

## 🔒 Security Features

### Network Security
- Binds to `0.0.0.0` for public access
- Configurable CORS origins
- No hardcoded localhost assumptions

### Application Security
- Stateless architecture (no local database)
- Deterministic player identity system
- All operations authenticated via Linera Owner addresses

### Data Security
- All state stored on Linera blockchain
- No sensitive data in logs
- Environment-based configuration

## 📊 Scaling Considerations

### Horizontal Scaling
- Multiple service instances can run concurrently
- No session affinity required
- Load balancer compatible

### Performance
- Optimized production binary (`cargo build --release`)
- Efficient logging configuration
- Minimal memory footprint (stateless)

### Reliability
- All state persisted on Linera network
- Automatic reconnection to Conway testnet
- Graceful error handling

## 🛠️ Troubleshooting

### Common Issues

**Service won't start:**
```bash
# Check wallet access
linera wallet show

# Verify Conway testnet connectivity
linera query-validators

# Check environment variables
env | grep LIVEDRAFT
```

**CORS errors:**
```bash
# Verify CORS configuration
curl -H "Origin: https://your-domain.com" \
     -X OPTIONS http://your-server:8080/graphql
```

**Network binding issues:**
```bash
# Check if port is available
netstat -tlnp | grep :8080

# Verify binding address
ss -tlnp | grep :8080
```

## 📝 Production Checklist

- [ ] Linera wallet initialized on server
- [ ] Application deployed to Conway testnet
- [ ] Environment variables configured
- [ ] CORS origins set to frontend domain
- [ ] Service starts without errors
- [ ] Health endpoint responds
- [ ] GraphQL endpoint accessible
- [ ] Frontend can connect successfully
- [ ] Monitoring/logging configured
- [ ] Firewall rules configured (port 8080)

## 🔄 Updates and Maintenance

### Application Updates
```bash
# Redeploy contracts if needed
./scripts/deploy_conway.sh

# Update Application ID in .env
# Restart service
```

### Service Updates
```bash
# Pull latest code
git pull

# Rebuild and restart
DEPLOYMENT_MODE=production ./start.sh
```

The service is now production-ready for Conway testnet deployment with proper security, monitoring, and scalability features.