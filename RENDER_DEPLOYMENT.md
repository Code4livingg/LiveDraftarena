# LiveDraft Arena - Render Deployment Guide

## 📦 What to Upload

**File**: `livedraft-arena-render.zip` (located in project root)

This ZIP contains:
- ✅ Rust backend service (`service/`)
- ✅ Smart contracts (`contracts/`)  
- ✅ Root-level `Dockerfile` optimized for Render
- ✅ `.dockerignore` for efficient cloud builds

## 🚀 Render Deployment Steps

### 1. Create New Web Service

1. Go to [Render Dashboard](https://dashboard.render.com)
2. Click **"New +"** → **"Web Service"**
3. Select **"Build and deploy from a Git repository"**
4. Click **"Upload from computer"** (at the bottom)
5. Upload `livedraft-arena-render.zip`
6. Click **"Continue"**

### 2. Configure Service Settings

**Basic Configuration:**
- **Name**: `livedraft-arena-api`
- **Region**: `Oregon (US West)` or closest to users
- **Branch**: `main` (auto-detected)
- **Root Directory**: Leave empty
- **Runtime**: `Docker` (auto-detected from Dockerfile)

**Build & Deploy:**
- **Build Command**: Leave empty (Docker handles build)
- **Start Command**: `./livedraft-arena-service`

**Advanced Settings:**
- **Port**: `8080`
- **Health Check Path**: `/health`
- **Auto-Deploy**: `Yes`

### 3. Environment Variables

Click **"Advanced"** → **"Environment Variables"**

**Required:**
```
LIVEDRAFT_APP_ID=placeholder_app_id_for_deployment
```

**Optional (recommended):**
```
RUST_LOG=info
CORS_ORIGINS=*
```

**Note**: Render automatically sets `PORT` and `BIND_ADDRESS=0.0.0.0`

### 4. Deploy

1. Click **"Create Web Service"**
2. Render will:
   - Extract the ZIP file
   - Build Docker image in the cloud
   - Deploy the service
3. Build time: ~5-10 minutes
4. Service URL: `https://livedraft-arena-api.onrender.com`

## 🔍 Verification Steps

### 1. Check Build Logs
Monitor the deployment in Render dashboard:
- Build logs show Docker compilation
- Deploy logs show service startup

### 2. Test Endpoints

**Health Check:**
```bash
curl https://livedraft-arena-api.onrender.com/health
# Expected: {"status":"ok"}
```

**GraphQL Introspection:**
```bash
curl -X POST https://livedraft-arena-api.onrender.com/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ __schema { types { name } } }"}'
# Expected: GraphQL schema response
```

**GraphQL Playground:**
Visit: `https://livedraft-arena-api.onrender.com/playground`

### 3. CORS Verification
```bash
curl -I -X OPTIONS https://livedraft-arena-api.onrender.com/graphql \
  -H "Origin: https://example.com"
# Expected: Access-Control-Allow-Origin header
```

## 📋 Configuration Summary

| Setting | Value |
|---------|-------|
| **Upload File** | `livedraft-arena-render.zip` |
| **Runtime** | Docker |
| **Start Command** | `./livedraft-arena-service` |
| **Port** | `8080` |
| **Health Check** | `/health` |
| **Required Env** | `LIVEDRAFT_APP_ID=placeholder_app_id_for_deployment` |

## 🌐 Frontend Integration

After successful deployment:

**Update Vercel Environment Variables:**
```
VITE_BACKEND_GRAPHQL_URL=https://livedraft-arena-api.onrender.com/graphql
```

**Update Local Development:**
```bash
# In frontend/.env.local
VITE_BACKEND_GRAPHQL_URL=https://livedraft-arena-api.onrender.com/graphql
```

## 🔧 Troubleshooting

### Build Failures
- Check build logs in Render dashboard
- Verify Dockerfile syntax
- Ensure all dependencies are included in ZIP

### Runtime Errors
- Check service logs in Render dashboard
- Verify environment variables are set
- Test health endpoint first

### CORS Issues
- Verify `CORS_ORIGINS=*` is set
- Check browser network tab for CORS headers

## ✅ Success Criteria

After deployment, these should work:

1. **Health Check**: `GET /health` returns `{"status":"ok"}`
2. **GraphQL Endpoint**: `POST /graphql` accepts queries
3. **CORS Headers**: Present in OPTIONS responses
4. **Frontend Integration**: Create Room button works

## 🎯 Expected Result

Opening `https://livedraft-arena-api.onrender.com/graphql` returns a valid GraphQL response, enabling the frontend "Create Room" feature to work immediately.

**Service Features:**
- ✅ POST GraphQL endpoint at `/graphql`
- ✅ Server listening on `0.0.0.0:8080`
- ✅ CORS enabled for all origins
- ✅ Health monitoring at `/health`
- ✅ GraphQL playground at `/playground`