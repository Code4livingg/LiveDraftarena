# LiveDraft Arena - Fly.io Deployment Guide

## Overview

This guide covers deploying the LiveDraft Arena Rust GraphQL backend to Fly.io without using Git repositories.

## Prerequisites

1. **Fly.io Account**: Sign up at https://fly.io
2. **Flyctl CLI**: Install from https://fly.io/docs/hands-on/install-flyctl/
3. **Linera Application**: Deployed to Conway testnet with Application ID
4. **Environment Variables**: Required configuration values

## Quick Deployment

### 1. Install and Login to Fly.io

```bash
# Install flyctl (macOS)
brew install flyctl

# Or download from https://fly.io/docs/hands-on/install-flyctl/

# Login to Fly.io
flyctl auth login
```

### 2. Set Environment Variables

```bash
# Required: Your deployed Linera application ID
export LIVEDRAFT_APP_ID="your_deployed_app_id_here"

# Optional: Specific chain ID (uses wallet default if not set)
export LIVEDRAFT_CHAIN_ID="your_chain_id_here"

# Optional: Custom wallet path (uses default if not set)
export LINERA_WALLET_PATH="/path/to/wallet.json"
```

### 3. Deploy to Fly.io

```bash
cd service
./deploy-fly.sh
```

That's it! The script will:
- Build the Docker image locally
- Deploy to Fly.io
- Set environment variables as secrets
- Configure the service

## Manual Deployment Steps

If you prefer manual control:

### 1. Initialize Fly App

```bash
cd service
flyctl launch --no-deploy --name livedraft-arena-api --region iad
```

### 2. Set Secrets

```bash
flyctl secrets set LIVEDRAFT_APP_ID="your_app_id"
flyctl secrets set LIVEDRAFT_CHAIN_ID="your_chain_id"  # optional
```

### 3. Deploy

```bash
flyctl deploy --local-only
```

## Verification

### 1. Check Deployment Status

```bash
flyctl status
flyctl logs
```

### 2. Test GraphQL Endpoint

```bash
# Test the deployed endpoint
./test-graphql.sh https://livedraft-arena-api.fly.dev/graphql

# Or manually test
curl -X POST https://livedraft-arena-api.fly.dev/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ __schema { types { name } } }"}'
```

### 3. Verify Endpoints

- **GraphQL**: https://livedraft-arena-api.fly.dev/graphql
- **Playground**: https://livedraft-arena-api.fly.dev/playground  
- **Health**: https://livedraft-arena-api.fly.dev/health

## Configuration

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `LIVEDRAFT_APP_ID` | ✅ | Deployed Linera application ID |
| `LIVEDRAFT_CHAIN_ID` | ❌ | Specific chain ID (uses wallet default) |
| `LINERA_WALLET_PATH` | ❌ | Custom wallet path |
| `RUST_LOG` | ❌ | Log level (defaults to `info`) |
| `CORS_ORIGINS` | ❌ | CORS origins (defaults to `*`) |

### Fly.io Configuration

The `fly.toml` file configures:
- **App Name**: `livedraft-arena-api`
- **Region**: `iad` (Ashburn, VA)
- **Port**: 8080 (internal), 80/443 (external)
- **Health Checks**: HTTP checks on `/health`
- **Resources**: 1 CPU, 512MB RAM

## Scaling and Management

### Scale the Application

```bash
# Scale to 2 instances
flyctl scale count 2

# Scale to different regions
flyctl regions add lax sea
flyctl scale count 3
```

### Monitor the Application

```bash
# View logs
flyctl logs

# View metrics
flyctl dashboard

# SSH into instance
flyctl ssh console
```

### Update the Application

```bash
# Redeploy after changes
flyctl deploy --local-only

# Update secrets
flyctl secrets set LIVEDRAFT_APP_ID="new_app_id"
```

## Frontend Integration

After deployment, update your frontend configuration:

### Vercel Environment Variables

```bash
# In Vercel dashboard or CLI
VITE_BACKEND_GRAPHQL_URL=https://livedraft-arena-api.fly.dev/graphql
```

### Local Development

```bash
# In frontend/.env.local
VITE_BACKEND_GRAPHQL_URL=https://livedraft-arena-api.fly.dev/graphql
```

## Troubleshooting

### Common Issues

**Deployment Fails:**
```bash
# Check build logs
flyctl logs

# Verify Docker build locally
docker build -t livedraft-test .
docker run -p 8080:8080 livedraft-test
```

**GraphQL Errors:**
```bash
# Check if LIVEDRAFT_APP_ID is set
flyctl secrets list

# View application logs
flyctl logs --app livedraft-arena-api
```

**CORS Issues:**
```bash
# Test CORS headers
curl -I -X OPTIONS https://livedraft-arena-api.fly.dev/graphql \
  -H "Origin: https://your-frontend.vercel.app"
```

### Health Checks

The service includes comprehensive health monitoring:

- **HTTP Health Check**: `GET /health` every 10 seconds
- **TCP Check**: Port 8080 connectivity every 15 seconds
- **Startup Grace Period**: 1 second for service initialization

### Logs and Debugging

```bash
# Real-time logs
flyctl logs -f

# Filter logs by level
flyctl logs | grep ERROR

# Check specific time range
flyctl logs --since 1h
```

## Security Considerations

### Production Security

1. **CORS Configuration**: Set specific origins in production
2. **Environment Variables**: Use Fly.io secrets for sensitive data
3. **Network Security**: Fly.io provides automatic HTTPS
4. **Resource Limits**: Configure appropriate CPU/memory limits

### Recommended Production Settings

```bash
# Set specific CORS origins for production
flyctl secrets set CORS_ORIGINS="https://your-frontend.vercel.app,https://www.your-domain.com"

# Set production log level
flyctl secrets set RUST_LOG="info"
```

## Cost Optimization

### Resource Management

- **Shared CPU**: Sufficient for most workloads
- **512MB RAM**: Adequate for GraphQL service
- **Auto-scaling**: Fly.io scales based on demand

### Monitoring Costs

```bash
# View current usage
flyctl dashboard

# Check billing
flyctl billing
```

The deployment is now production-ready and will provide a stable GraphQL endpoint for the LiveDraft Arena frontend.