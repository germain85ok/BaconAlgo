#!/bin/bash
# BaconAlgo — Cloudflare Tunnel Setup
# Run this in WSL2 or Git Bash on Windows

echo "🥓 BaconAlgo — Cloudflare Tunnel Setup"
echo "======================================="

# Install cloudflared
echo "📦 Installing cloudflared..."
# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    curl -L --output cloudflared.deb https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb
    sudo dpkg -i cloudflared.deb
    rm cloudflared.deb
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo "Download cloudflared from: https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-windows-amd64.exe"
    echo "Place it in your PATH"
fi

# Login to Cloudflare
echo "🔐 Login to Cloudflare..."
cloudflared tunnel login

# Create tunnel
echo "🔧 Creating tunnel 'baconalgo'..."
cloudflared tunnel create baconalgo

# Get tunnel ID
TUNNEL_ID=$(cloudflared tunnel list | grep baconalgo | awk '{print $1}')
echo "✅ Tunnel ID: $TUNNEL_ID"

# Create config
echo "📝 Creating tunnel config..."
mkdir -p ~/.cloudflared
cat > ~/.cloudflared/config.yml << EOF
tunnel: $TUNNEL_ID
credentials-file: /home/$USER/.cloudflared/$TUNNEL_ID.json

ingress:
  - hostname: baconalgo.com
    service: http://localhost:80
  - hostname: www.baconalgo.com
    service: http://localhost:80
  - hostname: api.baconalgo.com
    service: http://localhost:8080
  - service: http_status:404
EOF

echo "🌐 Configure DNS..."
cloudflared tunnel route dns baconalgo baconalgo.com
cloudflared tunnel route dns baconalgo www.baconalgo.com

echo ""
echo "✅ Setup complete!"
echo ""
echo "To start the tunnel:"
echo "  cloudflared tunnel run baconalgo"
echo ""
echo "To install as a service (Linux):"
echo "  sudo cloudflared service install"
echo ""
echo "🥓 BaconAlgo is ready to go LIVE!"
